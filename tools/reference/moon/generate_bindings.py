#!/usr/bin/env python3
"""Render the pinned Moon wire schema; never write handwritten APIs or exceptions."""
import hashlib
import json
from pathlib import Path
import re

import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from rust_imports import shorten_type_paths

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[2]

IMPORTS = {
    'U256': 'fastnum::U256', 'I512': 'fastnum::I512',
    'HashMap': 'std::collections::HashMap',
    'TonCell': 'ton::ton_core::cell::TonCell', 'TonHash': 'ton::ton_core::cell::TonHash',
    'TonAddress': 'ton::ton_core::types::TonAddress',
    'TLBCoins': 'ton::ton_core::types::tlb_core::TLBCoins',
    'TLBRef': 'ton::ton_core::types::tlb_core::TLBRef',
    'MsgAddress': 'ton::ton_core::types::tlb_core::MsgAddress',
    'InternalAddress': 'ton::ton_core::types::tlb_core::adapters::InternalAddress',
    'NullableAddress': 'ton::ton_core::types::tlb_core::adapters::NullableAddress',
    'ConstLen': 'ton::ton_core::types::tlb_core::adapters::ConstLen',
    'DictKeyAdapterInt': 'ton::tlb_adapters::DictKeyAdapterInt',
    'DictKeyAdapterUint': 'ton::tlb_adapters::DictKeyAdapterUint',
    'DictKeyAdapterTonHash': 'ton::tlb_adapters::DictKeyAdapterTonHash',
    'DictValAdapterTLB': 'ton::tlb_adapters::DictValAdapterTLB',
    'TLBHashMapE': 'ton::tlb_adapters::TLBHashMapE',
}


def load_schema():
    data = (BASE / 'bindings.json').read_bytes()
    expected = (BASE / 'bindings.sha256').read_text().strip()
    if hashlib.sha256(data).hexdigest() != expected:
        raise ValueError('Moon schema hash mismatch; review schema changes before updating its pin')
    return json.loads(data)


def write_generated(path, text, root=ROOT):
    allowed = {root / 'crates/moon/src/contracts' / p for p in load_schema()['generated_files']}
    if path not in allowed:
        raise ValueError(f'refusing handwritten or unowned path: {path}')
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(shorten_type_paths(text))


def render(schema):
    output = {p[:-3]: [] for p in schema['generated_files']}
    imports = {p: {'use ton::ton_core::TLB;'} for p in output}

    def resolve(text, family, target):
        def replace(match):
            name = match[0]
            info = schema['paths'].get(f'{family}::{name}')
            if info:
                if info['path'] != target:
                    imports[target].add('use crate::contracts::' + info['path'].replace('/', '::') + '::' + info['name'] + ';')
                return info['name']
            return name
        return re.sub(r'\b\w+\b', replace, text)

    for record in schema['records']:
        path = record['path']; family = record['source_module']
        if path not in output:
            continue
        lines = ['/// ' + line for line in record.get('description', '').splitlines()]
        lines.append('#[derive(Debug, Clone, PartialEq, TLB)]')
        if record['prefix']:
            prefix, bits = record['prefix']
            lines.append(f'#[tlb(prefix = {prefix}, bits_len = {bits})]')
        lines.append(f'pub struct {record["name"]} {{')
        for field in record['fields']:
            for attr in field['attributes']:
                native_width = re.fullmatch(r'[uiUI](8|16|32|64|128|256|512|1024)', field['type'])
                if native_width and attr == 'bits_len = ' + native_width[1]:
                    continue
                attr = resolve(attr, family, path)
                lines.append(f'    #[tlb({attr})]')
            ty = resolve(field['type'], family, path)
            lines.append(f'    pub {field["name"]}: {ty},')
        lines.append('}')
        output[path].append('\n'.join(lines))

    for dispatch in schema['dispatch']:
        path = dispatch['path']; family = path.split('/')[0]
        if path not in output:
            continue
        variants = []
        # Type names in dispatch were already renamed by the one-time migration.
        for name, ty in dispatch['variants']:
            for old, info in schema['paths'].items():
                if old.startswith(family + '::') and re.search(r'\b'+info['name']+r'\b', ty) and info['path'] != path:
                    imports[path].add('use crate::contracts::'+info['path'].replace('/', '::')+'::'+info['name']+';')
            variants.append((name, ty))
        # Preserve exact dispatch in mempad: nested metadata prefixes must not
        # turn a malformed known message into an Unknown top-level body.
        manual = family == 'mempad'
        lines = ['#[derive(Debug, Clone, PartialEq' + ('' if manual else ', TLB') + ')]', f'pub enum {dispatch["name"]} {{']
        lines += [f'    {name}({ty}),' for name, ty in variants]
        lines.append('}')
        if manual:
            imports[path].add('use ton::ton_core::traits::tlb::TLB;')
            lines += [f'impl {dispatch["name"]} {{', '    pub fn from_cell(cell: &TonCell) -> ton::ton_core::errors::TonCoreResult<Self> {', '        let mut parser = cell.parser();', '        if parser.data_bits_left()? < 32 { return Ok(Self::Unknown(cell.clone())); }', '        match parser.read_num::<u32>(32)? {']
            for name,ty in variants:
                if name=='Unknown':continue
                inner=ty.removeprefix('Box<').removesuffix('>')
                lines.append(f'            {dispatch["opcodes"][name]} => Ok(Self::{name}(Box::new({inner}::from_cell(cell)?))),')
            lines += ['            _ => Ok(Self::Unknown(cell.clone())),', '        }', '    }', '    pub fn to_cell(&self) -> ton::ton_core::errors::TonCoreResult<TonCell> {', '        match self {']
            lines += [f'            Self::{name}(value) => '+('Ok(value.clone()),' if name=='Unknown' else 'value.to_cell(),') for name,_ in variants]
            lines += ['        }', '    }', '}']
        output[path].append('\n'.join(lines))

    for path, parts in output.items():
        for child in schema['children'].get(path, []):
            parts.append(f'pub mod {child};')
        body = '\n\n'.join(parts)+'\n'
        for name, source in IMPORTS.items():
            if re.search(r'\b'+name+r'\b', body):
                imports[path].add(f'use {source};')
        if '#[tlb(bits_len' in body:
            imports[path].add('use ton::ton_core::types::tlb_core::adapters::ConstLen;')
        output[path] = '//! Generated Moon wire records. See tools/reference/moon/README.md.\n'+'\n'.join(sorted(imports[path]))+'\n\n'+body
    return output


def main():
    schema = load_schema()
    for path, text in render(schema).items():
        write_generated(ROOT / 'crates/moon/src/contracts' / (path+'.rs'), text)


if __name__ == '__main__':
    main()
