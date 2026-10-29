#!/usr/bin/env python3
"""Qualify public getter calls against independent captured result stacks.

Fixture identities, original API spellings and bytes stay immutable. Current Rust
names/types/ownership come from bindings-generation.json.
"""
import json
from pathlib import Path
from bindings_api import MANIFEST

import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from rust_imports import shorten_type_paths

ROOT = Path(__file__).resolve().parents[3]
bindings = {r['module']: r for r in json.loads(MANIFEST.read_text())}
rows = json.loads((ROOT/'crates/dedust/src/tests/fixtures/getter-stack-vectors.json').read_text())['cases']


def binding(case):
    module = 'classic/operator_pool' if case['contract'].startswith('OperatorManaged') else case['module']
    return bindings[module]


def api(case):
    return next(g for g in binding(case)['getters'] if g['selector'] == case['selector'])


def field_type(value, metadata):
    return next(f['rust_type'] for f in metadata['fields'] if f['name'] == value['field'])


def invocation(case):
    metadata = api(case)
    module = 'crate::contracts::'+case['module'].replace('/', '::')
    row = binding(case)
    wrapper = 'crate::contracts::'+row['module'].replace('/', '::')+'::'+row['wrapper']
    method = metadata['rust_name']
    parameters = metadata['parameters']
    args = []
    for i, value in enumerate(case['arguments']):
        if value in ('native', 'jetton'):
            args.append(f'DedustClassicAsset::from_cell(&fixture_arguments(&["{value}"])?.pop_cell()?)?')
        elif value == 'owner':
            args.append('TonAddress::ZERO')
        else:
            typ = parameters[i]['rust_type']
            args.append(('true' if int(value) else 'false') if typ=='bool' else f'{value}.into()' if typ == 'I512' else value)
    return module, wrapper, method, ', '.join(args)


def setup(case, reply):
    module, wrapper, method, args = invocation(case)
    arguments = ', '.join(json.dumps(v) for v in case['arguments'])
    wrapper_path = wrapper if '::' in wrapper else module+'::'+wrapper
    return (f'let client=recording_client({case["selector"]},fixture_arguments(&[{arguments}])?,{reply})?;\n'
            f'let contract={wrapper_path}::new(&client,&TonAddress::ZERO,None);\n',
            f'tokio_test::block_on(contract.{method}({args}))')


s = '''//! Captured results qualify real getter selectors, packing, values and exact arity.
//! Expected stacks are independent; program-code substitutions stay explicit.
#![allow(non_snake_case)]
use super::test_getter_provider::{cell,integer,fixture_values,fixture_stack,fixture_arguments,recording_client};
use crate::contracts::classic::asset::DedustClassicAsset;
use fastnum::{I512,U256};
use ton::{block_tlb::{TVMInt,TVMStackValue,TVMNull},contracts::TonContract,ton_core::{traits::tlb::TLB,types::TonAddress}};
'''
arity_types=set()
domain_fields=set()
for case in rows:
    metadata = api(case)
    scalar = metadata['result'] is None
    module, _, _, _ = invocation(case)
    s += (f'#[test]\nfn test_raw_getter_{case["contract"]}_{case["selector"]}()->anyhow::Result<()>{{\n'
          f'let values=fixture_values("{case["contract"]}", {case["selector"]})?;\n')
    setup_code, call = setup(case, 'fixture_stack(values)?')
    s += setup_code+f'let value={call}?;\n'
    for j, value in enumerate(case['values']):
        typ = field_type(value, metadata)
        typ=typ.rsplit('::',1)[-1] if '<' not in typ else typ
        target = 'value' if scalar else 'value.'+value['field']
        v = f'&values[{j}]'
        if typ.startswith('HashMap'):
            if value['kind'] != 'null':
                raise ValueError('Nonempty map needs independent entries')
            s += f'assert!({target}.is_empty());\n'
            continue
        if value['kind'] == 'null':
            if typ != 'Option<TonCell>':
                raise ValueError('Unexpected null '+typ)
            expected = 'None'
        elif typ == 'bool':
            expected = f'integer({v})? != I512::ZERO'
        elif typ == 'Coins':
            expected = f'ton::ton_core::types::Coins::try_from(integer({v})?)?'
        elif typ == 'U256':
            expected = f'U256::from_str(&integer({v})?.to_string())?'
        elif typ == 'I512':
            expected = f'integer({v})?'
        elif typ in ('u8','u16','u32','u64','u128','i8','i16','i32'):
            expected = f'<{typ}>::try_from(integer({v})?).map_err(|_|anyhow::anyhow!("narrow"))?'
        elif typ == 'TonCell':
            expected = f'cell({v})?'
        elif typ == 'MsgAddressInt':
            expected = f'ton::ton_core::types::tlb_core::MsgAddressInt::from_cell(&cell({v})?)?'
        elif typ in ('TonAddress','Option<TonAddress>'):
            adapter = 'NullableAddress' if typ.startswith('Option') else 'InternalAddress'
            expected = f'ton::ton_core::types::tlb_core::adapters::{adapter}.read(&mut cell({v})?.parser())?'
        elif typ == 'MsgAddress':
            expected = f'ton::ton_core::types::tlb_core::MsgAddress::from_cell(&cell({v})?)?'
        elif typ == 'DedustClassicAsset':
            expected = f'DedustClassicAsset::from_cell(&cell({v})?)?'
        elif typ.startswith('Dedust'):
            expected = f'{module}::storage::{typ}::from_cell(&cell({v})?)?'
        else:
            raise ValueError('Need field comparison '+typ)
        s += f'assert_eq!({target},{expected});\n'
    # Exact arity is project-owned behavior, but common managed/revision paths
    # share result decoders. Exercise each complete DTO once. Native scalar
    # methods intentionally tolerate additional results.
    arity_key=(case['module'], metadata['result'])
    check_arity = not scalar and arity_key not in arity_types
    if check_arity:
        s += 'let mut extra=fixture_stack(values)?;extra.insert(0,TVMStackValue::Null(TVMNull));\n'
        code, call = setup(case, 'extra')
        s += code+f'assert!({call}.is_err());\n'
        s += 'let mut missing=fixture_stack(values)?;missing.pop();\n'
        code, call = setup(case, 'missing')
        s += code+f'assert!({call}.is_err());\n'
        arity_types.add(arity_key)
    # Keep unsigned-hash result coverage at the public method boundary.
    for j, value in enumerate(case['values']):
        if value['kind'] != 'int':
            continue
        field = value['field']
        typ = field_type(value, metadata)
        bits = None
        if typ == 'U256' and case['module'] == 'custom_wallet/v2':
            bits = 256
        if bits is None:
            continue
        domain_key=(arity_key if not scalar else (case['module'], metadata['rust_name']),field,bits)
        if domain_key in domain_fields:continue
        domain_fields.add(domain_key)
        opening = '{ let bad=-I512::ONE;' if bits==256 else f'for bad in [-I512::ONE,I512::ONE<<{bits}u32] {{'
        s += opening+f' let mut response=fixture_stack(values)?;response[{j}]=TVMStackValue::Int(TVMInt{{value:bad}});\n'
        code, call = setup(case, 'response')
        s += code+f'assert!({call}.is_err(),"{case["contract"]}/{case["selector"]}/{field}");\n}}\n'
    s += 'Ok(())\n}\n'
(ROOT/'crates/dedust/src/tests/test_getter_stacks.rs').write_text(shorten_type_paths(s))
