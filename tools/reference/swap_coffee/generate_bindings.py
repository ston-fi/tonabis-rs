#!/usr/bin/env python3
"""Offline Coffee record generation from pinned, enriched interface facts.

Only explicit generated_files are writable. Wrappers, getter DTOs and protocol
exceptions are handwritten. No network, executable cells or source checkout.
"""
import hashlib
import json
import pathlib
import re

import sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from rust_imports import shorten_type_paths

ROOT = pathlib.Path(__file__).resolve().parents[3]
REF = ROOT / 'tools/reference/swap_coffee'
OUT = ROOT / 'crates/swap_coffee/src/contracts'


# Keep native opcode discrimination; boxed TLB variants erase their prefixes.
LARGE_MESSAGE_ENUMS = {'CoffeeVaultNativeOutgoingMessage', 'CoffeeLiquidityDepositoryIncomingMessage', 'CoffeeCrossDexOutgoingMessage', 'CoffeeLiquidityDepositoryOutgoingMessage', 'CoffeeVaultJettonIncomingMessage', 'CoffeeMevProtectorLegacyOutgoingMessage', 'CoffeeVaultExtraOutgoingMessage', 'CoffeePoolConstantProductOutgoingMessage', 'CoffeeCrossDexBidaskOutgoingMessage', 'CoffeeCrossDexLegacyOutgoingMessage', 'CoffeeVaultJettonOutgoingMessage', 'CoffeeMevProtectorOutgoingMessage', 'CoffeeStakingItemIncomingMessage', 'CoffeePoolCurveFiStableOutgoingMessage'}


def snake(value):
    value = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', value)
    value = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', value).lower()
    return 'r#' + value if value in ('type', 'ref', 'pub', 'in', 'match') else value


def numtype(value):
    width = int(re.search(r'\d+', value)[0])
    if width > 128:
        return 'I512' if value.startswith('int') else 'U256'
    return ('i' if value.startswith('int') else 'u') + str(next(n for n in (8, 16, 32, 64, 128) if n >= width))


def type_path(module, name):
    dest = module['destinations'][name]
    if 'canonical_path' in dest:
        return dest['canonical_path']
    return 'crate::contracts::' + module['module'] + '::' + dest['owner'] + '::' + dest['name']


def rtype(value, module):
    special = {
        'TrueBool': 'bool', 'FalseBool': 'bool', 'OptionalTrailingBool': 'Option<bool>',
        'WalletOwnerResponse': 'crate::contracts::vault_jetton::msgs::CoffeeVaultJettonOwnerResponse',
        'LpBounceOpcode': 'crate::contracts::lp_wallet::msgs::CoffeeLpWalletBounceOpcode',
        'VaultSenderProof': 'crate::contracts::vault_proof::CoffeeVaultContractProof',
        'StakingAssetData': 'crate::contracts::staking_item::storage::CoffeeStakingAssetData',
        'StakingPositionData': 'crate::contracts::staking_item::storage::CoffeeStakingPositionData',
        'EitherPayload': 'TLBEither<TonCell, TLBRef<TonCell>>',
        'MevReplayMarker': 'crate::contracts::mev::CoffeeMevReplayMarker',
        'StakingPeriod': 'crate::contracts::staking::CoffeeStakingPeriod',
        'StakingJettonConfiguration': 'crate::contracts::staking::CoffeeStakingJettonConfiguration',
        'StakingRewardSchedule': 'crate::contracts::staking::CoffeeStakingRewardSchedule',
        'hash256': 'TonHash', 'int': 'I512', 'coins': 'TLBCoins', 'bool': 'bool',
        'address': 'TonAddress', 'address?': 'Option<TonAddress>',
        'cell': 'TLBRef<TonCell>', 'slice': 'TLBRef<TonCell>',
        'RemainingBitsAndRefs': 'TonCell', 'bits512': '[u8; 64]',
    }
    if value in special:
        return special[value]
    if re.fullmatch(r'u?int\d+', value):
        return numtype(value)
    if value.endswith('?'):
        return 'Option<' + rtype(value[:-1], module) + '>'
    if value.startswith('Cell<'):
        inner = rtype(value[5:-1], module)
        if inner.endswith('SwapStepParams'):
            inner = 'Box<' + inner + '>'
        return 'TLBRef<' + inner + '>'
    if value.startswith('map<'):
        key, val = value[4:-1].split(', ')
        return 'std::collections::HashMap<' + rtype(key, module) + ', ' + rtype(val, module) + '>'
    return type_path(module, value)


def field_attribute(value, module):
    if value in ('TrueBool', 'FalseBool'):
        return '#[tlb(adapter = "ExpectedBool::<' + ('true' if value == 'TrueBool' else 'false') + '>")]\n'
    if value == 'OptionalTrailingBool':
        return '#[tlb(adapter = "crate::contracts::wire::TrailingSelector")]\n'
    if value in ('address', 'address?'):
        return '#[tlb(adapter = "' + ('InternalAddress' if value == 'address' else 'NullableAddress') + '")]\n'
    if value == 'int' or re.fullmatch(r'u?int\d+', value):
        width = '257' if value == 'int' else re.search(r'\d+', value)[0]
        native_width = re.fullmatch(r'[uiUI](\d+)', rtype(value, module))
        return '' if native_width and native_width[1] == width else '#[tlb(bits_len = ' + width + ')]\n'
    if value.startswith('map<'):
        key, val = value[4:-1].split(', ')
        width = re.search(r'\d+', key)[0]
        adapter = 'DictKeyAdapterTonHash' if key == 'hash256' else ('DictKeyAdapterInt<' + width + ', ' + rtype(key, module) + '>' if key.startswith('int') else 'DictKeyAdapterUint<' + rtype(key, module) + '>')
        return '#[tlb(adapter = "TLBHashMapE::<'+adapter+', DictValAdapterTLB<'+rtype(val, module)+'>>::new('+width+')")]\n'
    return ''


def imports(text):
    groups = {
        'ton::ton_core::cell': ['CellBuilder', 'CellParser', 'TonCell', 'TonHash'],
        'ton::ton_core::errors': ['TonCoreResult'],
        'ton::ton_core::types::tlb_core': ['MsgAddressInt', 'TLBCoins', 'TLBRef', 'TLBEither'],
        'ton::tlb_adapters': ['TLBHashMapE', 'DictKeyAdapterInt', 'DictKeyAdapterUint', 'DictKeyAdapterTonHash', 'DictValAdapterTLB'],
        'fastnum': ['I512', 'U256'],
        'crate::contracts::wire': ['wire_error'],
        'ton::ton_core::types': ['TonAddress'],
        'ton::ton_core::types::tlb_core::adapters': ['InternalAddress', 'NullableAddress'],
        'tonabis_core::msgs': ['EmptyBodyMsg'],
        'tonabis_core::adapters::expected_bool': ['ExpectedBool'],
    }
    result = 'use ton::ton_core::TLB;\n'
    for path, names in groups.items():
        used = [name for name in names if re.search(r'(?<![\w:])' + name + r'\b', text)]
        if used:
            result += 'use ' + path + '::{' + ', '.join(used) + '};\n'
    return result


def records(module, owner):
    text = ''
    for name, record in module['structs'].items():
        dest = module['destinations'][name]
        if dest['owner'] != owner or 'canonical_path' in dest:
            continue
        if dest['handwritten']:
            raise ValueError("handwritten record cannot enter generated output: " + name)
        text += '#[derive(Debug, Clone, PartialEq, TLB)]\n'
        if record['prefix']:
            prefix = record['prefix']
            width = len(prefix[2:]) * (4 if prefix.startswith('0x') else 1)
            text += '#[tlb(prefix = '+prefix+', bits_len = '+str(width)+')]\n'
        text += 'pub struct '+dest['name']+' {\n'
        for field, typ in record['fields']:
            text += field_attribute(typ, module)+'pub '+snake(field)+': '+rtype(typ, module)+',\n'
        text += '}\n'
    for name, variants in module['unions'].items():
        if variants == ['void'] or module['destinations'][name]['owner'] != owner or 'canonical_path' in module['destinations'][name]:
            continue
        common = module['contract']
        def variant(v):
            return v[len(common):] if v.startswith(common) and v != common else v
        # Empty transport funding is a separate protocol role, not an untagged catch-all.
        empty = name.endswith('IncomingMessage') and module['module'] in ('cross_dex', 'cross_dex_bidask', 'cross_dex_legacy', 'mev_protector', 'mev_protector_legacy', 'lp_wallet', 'staking_item', 'staking_master', 'staking_vault')
        unknown = 'Message' in name and all(v in module['structs'] and module['structs'][v]['prefix'] and module['structs'][v]['prefix'].startswith('0x') for v in variants)
        extras = []
        if name == 'CoffeeStakingMasterOutgoingMessage':
            extras.append(('DepositLog', 'crate::contracts::staking_master::msgs::CoffeeStakingMasterDepositLogMsg', '0xf9471134'))
        if name == 'CoffeeStakingVaultIncomingMessage':
            extras.append(('MasterUnlock', 'crate::contracts::staking_master::msgs::CoffeeStakingMasterUnlockToVaultMsg', '0xcb03bfaf'))
        if name != 'CoffeeFactoryIncomingMessage':
            text += '/// Typed dispatch; unknown bodies retain bits and references.\n#[derive(Debug, Clone, PartialEq, TLB)]\n'
            if name in LARGE_MESSAGE_ENUMS:
                text += '// Keep opcode prefixes visible: Box<T> erases them in locked TON dispatch.\n#[allow(clippy::large_enum_variant)]\n'
            text += 'pub enum '+module['destinations'][name]['name']+' {\n'
            for value in variants:
                text += variant(value)+'('+type_path(module, value)+'),\n'
            for label, typ, _ in extras:
                text += label+'('+typ+'),\n'
            if empty:
                text += 'Empty(tonabis_core::msgs::EmptyBodyMsg),\n'
            if 'Message' in name:
                text += 'Unknown(TonCell),\n'
            text += '}\n'
            continue
        text += '/// Distinct dispatch role; malformed recognized bodies fail.\n#[derive(Debug, Clone, PartialEq'+('' if (empty or unknown) else ', TLB')+')]\npub enum '+module['destinations'][name]['name']+' {\n'
        for value in variants:
            text += variant(value)+'(Box<'+type_path(module, value)+'>),\n'
        for label, typ, _ in extras:
            text += label+'(Box<'+typ+'>),\n'
        if empty:
            text += 'Empty,\n'
        if unknown:
            text += 'Unknown(TonCell),\n'
        text += '}\n'
        if empty or unknown:
            text += 'impl ton::ton_core::traits::tlb::TLB for '+module['destinations'][name]['name']+' {\nfn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {\n'
            if empty:
                text += 'if p.data_bits_left()? == 0 && p.refs_left() == 0 {return Ok(Self::Empty);}\n'
            for label, typ, prefix in extras + [(variant(v), type_path(module, v), module['structs'][v]['prefix']) for v in variants]:
                width = len(prefix[2:]) * (4 if prefix.startswith('0x') else 1)
                text += 'if p.data_bits_left()? >= '+str(width)+' && p.lookup_bits('+str(width)+')? == '+prefix+' {return Ok(Self::'+label+'(Box::new('+typ+'::read(p)?)));}\n'
            text += 'Ok(Self::Unknown(p.read_remaining()?))\n}\nfn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {match self {\n'
            for label in [variant(v) for v in variants]+[e[0] for e in extras]:
                text += 'Self::'+label+'(value) => value.write(b),\n'
            if empty:
                text += 'Self::Empty => Ok(()),\n'
            text += 'Self::Unknown(value) => value.write(b),\n}}}\n'
    return text


def compact_paths(body, module, owner):
    pattern = r'(crate::contracts::[a-z_]+(?:::[a-z_]+)*::([A-Z]\w*))'
    found = re.findall(pattern, body)
    by_name = {}
    for path, name in found:
        by_name.setdefault(name, set()).add(path)
    local = 'crate::contracts::' + module['module'] + '::' + owner + '::'
    imports = []
    for name, paths in sorted(by_name.items()):
        if len(paths) != 1:
            continue
        path = next(iter(paths))
        if path != local + name:
            imports.append('use ' + path + ';\n')
        body = re.sub(r'\b'+re.escape(path)+r'\b', name, body)
    return ''.join(imports), body


def main():
    manifest = json.loads((REF / 'bindings-generation.json').read_text())
    for relative, expected in manifest['inputs'].items():
        actual = hashlib.sha256((REF / relative).read_bytes()).hexdigest()
        if actual != expected:
            raise ValueError('pinned input changed: ' + relative)
    schema = json.loads((REF / 'schema-inventory.json').read_text())
    outputs = {}
    for module in schema:
        for owner in ('storage', 'msgs'):
            relative = str(pathlib.Path('crates/swap_coffee/src/contracts') / module['module'] / (owner+'.rs'))
            if relative in manifest['handwritten_files']:
                continue
            body = records(module, owner)
            all_text = body
            extra_imports, body = compact_paths(body, module, owner)
            header = imports(all_text) + extra_imports
            if 'impl TLB for' in all_text:
                header += 'use ton::ton_core::traits::tlb::TLB;\n'
            if '#[tlb(bits_len' in body:
                header += 'use ton::ton_core::types::tlb_core::adapters::ConstLen;\n'
            relative = str(pathlib.Path('crates/swap_coffee/src/contracts') / module['module'] / (owner+'.rs'))
            outputs[relative] = '//! Generated Coffee '+owner+'. See tools/reference/swap_coffee/README.md.\n'+header+body
    if set(outputs) != set(manifest['generated_files']):
        raise ValueError('output ownership differs from protected allowlist')
    for relative, content in outputs.items():
        path = ROOT / relative
        if path.resolve().parent.parent != OUT.resolve():
            raise ValueError('output outside Coffee family children: ' + relative)
        path.write_text(shorten_type_paths(content))
    print(str(len(outputs)) + ' allowlisted Coffee storage/message files')


if __name__ == '__main__':
    main()
