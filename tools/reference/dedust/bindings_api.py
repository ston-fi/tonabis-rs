"""Resolve Rust ownership without changing upstream declaration/fixture identities."""
import json
from functools import lru_cache
from pathlib import Path

MANIFEST = Path(__file__).with_name("bindings-generation.json")


@lru_cache(maxsize=1)
def declarations():
    return {row["contract"]: {symbol["declaration"]: symbol for symbol in row["symbols"]}
            for row in json.loads(MANIFEST.read_text())}


def rust_declaration(contract, name):
    return declarations()[contract][name]


def rust_path(contract, name):
    symbol = rust_declaration(contract, name)
    return symbol.get("rust_type") or "crate::contracts::" + symbol["module"].replace("/", "::") + "::" + symbol["rust_name"]

def getter_api(module, selector):
    """Resolve selector identity independently of catalog/Rust method spelling."""
    for row in json.loads(MANIFEST.read_text()):
        if module == row['module']:
            return next((g for g in row['getters'] if g['selector'] == selector), None)
    return None
