"""Keep catalog identities separate from contract-specific public Rust names."""
import json
import re
from collections import defaultdict
from pathlib import Path

from rust_imports import code_mask

NAMES = json.loads(Path(__file__).with_name('rust-type-names.json').read_text())


def names_for(module):
    candidates = defaultdict(set)
    for names in NAMES.values():
        for old, new in names.items():
            candidates[old].add(new)
    names = {old: next(iter(values)) for old, values in candidates.items() if len(values) == 1}
    family = module.rsplit('/', 1)[0]
    for path, local in NAMES.items():
        if path.rsplit('/', 1)[0] == family:
            names.update(local)
    names.update(NAMES.get(module, {}))
    return names


def rename_type(text, module):
    for path, names in NAMES.items():
        for old, new in names.items():
            text = text.replace(path.replace('/', '::') + '::' + old,
                                path.replace('/', '::') + '::' + new)
    names = names_for(module)
    return re.sub(r'\b\w+\b', lambda match: names.get(match[0], match[0]), text)


def rename_source(source, module):
    """Rename type tokens and adapter expressions, preserving variant labels and literals."""
    code = code_mask(source)
    variants = set()
    for enum in re.finditer(r'\benum\s+\w+\s*\{([^}]*)\}', code):
        for variant in re.finditer(r'(?m)^\s*(\w+)\s*(?:\(|,|=)', enum[1]):
            variants.add(enum.start(1) + variant.start(1))
    edits = []
    for token in re.finditer(r'\b\w+(?:::\w+)*\b', code):
        if token.start() in variants or token[0].startswith('Self::'):
            continue
        replacement = rename_type(token[0], module)
        if replacement != token[0]:
            edits.append((token.start(), token.end(), replacement))
    for attr in re.finditer(r'#\[(?:tlb|from_tvm_stack)\([^\n]*', source):
        for literal in re.finditer(r'"([^"\n]*)"', attr[0]):
            replacement = rename_type(literal[1], module)
            if replacement != literal[1]:
                edits.append((attr.start() + literal.start(1), attr.start() + literal.end(1), replacement))
    for start, end, replacement in sorted(edits, reverse=True):
        source = source[:start] + replacement + source[end:]
    return source


def rename_metadata(records):
    for record in records:
        for symbol in record['symbols']:
            module = symbol['module']
            symbol['rust_name'] = rename_type(symbol['rust_name'], module)
            if 'rust_type' in symbol:
                symbol['rust_type'] = rename_type(symbol['rust_type'], module)
            for field in symbol.get('fields', []):
                field['rust_type'] = rename_type(field['rust_type'], module)
            if 'coverage_symbol' in symbol:
                coverage = symbol['coverage_symbol']
                coverage['rust_name'] = rename_type(coverage['rust_name'], coverage['module'])
        for getter in record['getters']:
            module = getter.get('result_module') or getter['module']
            for key in ('result', 'return_type'):
                if getter.get(key):
                    getter[key] = rename_type(getter[key], module)
            for field in getter['fields'] + getter['parameters']:
                field['rust_type'] = rename_type(field['rust_type'], module)
