"""Public type search must identify one protocol entity without module context."""
from collections import defaultdict
from pathlib import Path
import re
import sys
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools/reference'))
from rust_imports import code_mask


class PublicTypeNamesTests(unittest.TestCase):
    def test_public_types_and_wrappers_have_unique_names(self):
        owners = defaultdict(list)
        for path in (ROOT / 'crates').glob('*/src/**/*.rs'):
            if 'tests' in path.parts:
                continue
            source = code_mask(path.read_text())
            declarations = re.findall(r'\bpub\s+(?:struct|enum|type|trait)\s+(\w+)', source)
            declarations += re.findall(r'ton_contract!\s*[({]\s*(\w+)', source)
            for name in declarations:
                owners[name].append(str(path.relative_to(ROOT)))
        duplicates = {name: paths for name, paths in owners.items() if len(paths) > 1}
        self.assertFalse(duplicates, f'Use protocol/contract/generation-specific names: {duplicates}')
