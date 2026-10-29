"""Moon regeneration must never overwrite handwritten APIs, results or exceptions."""
import importlib.util
from pathlib import Path
import tempfile
import unittest

SOURCE = Path(__file__).resolve().parents[1] / 'reference/moon/generate_bindings.py'
SPEC = importlib.util.spec_from_file_location('moon_bindings', SOURCE)
GENERATOR = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GENERATOR)


class MoonGenerationOwnershipTests(unittest.TestCase):
    def test_refuses_handwritten_files(self):
        schema = GENERATOR.load_schema()
        root = SOURCE.parents[3]
        contract_root = root / 'crates/moon/src/contracts'
        generated = set(schema['generated_files'])
        handwritten = [p.relative_to(contract_root) for p in contract_root.rglob('*.rs')
                       if str(p.relative_to(contract_root)) not in generated]
        with tempfile.TemporaryDirectory() as directory:
            target = Path(directory)
            for relative in handwritten:
                path = target / 'crates/moon/src/contracts' / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text('handwritten behavior')
                with self.assertRaisesRegex(ValueError, 'handwritten'):
                    GENERATOR.write_generated(path, 'replacement', target)
                self.assertEqual(path.read_text(), 'handwritten behavior')
            path = target / 'crates/moon/src/contracts/cpmm/account/storage.rs'
            GENERATOR.write_generated(path, 'generated', target)
            self.assertEqual(path.read_text(), 'generated')

    def test_schema_pin_rejects_changed_input(self):
        with tempfile.TemporaryDirectory() as directory:
            base = Path(directory)
            for name in ('bindings.json', 'bindings.sha256'):
                (base / name).write_bytes((SOURCE.parent / name).read_bytes())
            with (base / 'bindings.json').open('a') as output:
                output.write(' ')
            original = GENERATOR.BASE
            try:
                GENERATOR.BASE = base
                with self.assertRaisesRegex(ValueError, 'hash mismatch'):
                    GENERATOR.load_schema()
            finally:
                GENERATOR.BASE = original
