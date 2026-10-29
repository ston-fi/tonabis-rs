"""Coffee generation cannot replace handwritten APIs or independent expectations."""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import shutil
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'tools/reference/swap_coffee/generate_bindings.py'
SPEC = importlib.util.spec_from_file_location('coffee_bindings', SOURCE)
GENERATOR = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GENERATOR)


class CoffeeGenerationTests(unittest.TestCase):
    def run_in_copy(self, edit=None):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for relative in ('tools/reference/swap_coffee', 'crates/swap_coffee'):
                shutil.copytree(ROOT / relative, root / relative)
            ref = root / 'tools/reference/swap_coffee'
            manifest = json.loads((ref / 'bindings-generation.json').read_text())
            generated = set(manifest['generated_files'])
            authored = {str(p.relative_to(root)): p.read_bytes()
                        for p in (root / 'crates/swap_coffee').rglob('*')
                        if p.is_file() and str(p.relative_to(root)) not in generated}
            original = GENERATOR.ROOT, GENERATOR.REF, GENERATOR.OUT
            GENERATOR.ROOT, GENERATOR.REF = root, ref
            GENERATOR.OUT = root / 'crates/swap_coffee/src/contracts'
            try:
                if edit:
                    edit(root, ref, manifest)
                else:
                    GENERATOR.main()
                    first = {p: (root / p).read_bytes() for p in generated}
                    GENERATOR.main()
                    self.assertEqual(first, {p: (root / p).read_bytes() for p in generated})
                    self.assertEqual(authored, {p: (root / p).read_bytes() for p in authored})
            finally:
                GENERATOR.ROOT, GENERATOR.REF, GENERATOR.OUT = original

    def test_determinism_and_protected_files(self):
        self.run_in_copy()

    def test_changed_input_fails_before_any_output(self):
        def edit(root, ref, manifest):
            paths = manifest['generated_files']
            before = {p: (root / p).read_bytes() for p in paths}
            with (ref / 'schema-inventory.json').open('a') as output:
                output.write(' ')
            with self.assertRaisesRegex(ValueError, 'pinned input changed'):
                GENERATOR.main()
            self.assertEqual(before, {p: (root / p).read_bytes() for p in paths})
        self.run_in_copy(edit)

    def test_wrapper_cannot_enter_output_allowlist(self):
        def edit(root, ref, manifest):
            wrapper = root / 'crates/swap_coffee/src/contracts/factory.rs'
            before = wrapper.read_bytes()
            manifest['generated_files'].append(str(wrapper.relative_to(root)))
            (ref / 'bindings-generation.json').write_text(json.dumps(manifest))
            with self.assertRaisesRegex(ValueError, 'protected allowlist'):
                GENERATOR.main()
            self.assertEqual(before, wrapper.read_bytes())
        self.run_in_copy(edit)

    def test_normalized_family_pins_match_bundled_interfaces(self):
        schema = json.loads((SOURCE.parent / 'schema-inventory.json').read_text())
        for family in schema:
            interface = SOURCE.parent / 'interfaces' / Path(family['source']).name
            self.assertEqual(hashlib.sha256(interface.read_bytes()).hexdigest(), family['sha256'])

    def test_vault_records_resolve_to_one_owner(self):
        schema = {m['module']: m for m in json.loads((SOURCE.parent / 'schema-inventory.json').read_text())}
        records = ('CoffeeVaultInitData', 'CoffeeAsset', 'CoffeeNativeAsset', 'CoffeeJettonAsset', 'CoffeeExtraAsset')
        for name in records:
            kind = 'unions' if name == 'CoffeeAsset' else 'structs'
            canonical = schema['vault_native'][kind][name]
            path = 'crate::contracts::vault_native::storage::' + name
            for family in ('vault_native', 'vault_extra', 'vault_jetton'):
                self.assertEqual(canonical, schema[family][kind][name])
                self.assertEqual(GENERATOR.type_path(schema[family], name), path)
                body = GENERATOR.records(schema[family], 'storage')
                declaration = ('pub enum ' if kind == 'unions' else 'pub struct ') + name + ' '
                self.assertEqual(declaration in body, family == 'vault_native')

    def test_independent_fixture_bytes_stay_pinned(self):
        inventory = json.loads((SOURCE.parent / 'bindings.json').read_text())
        for fixture in inventory['fixtures']:
            if fixture['destination']:
                digest = hashlib.sha256((ROOT / fixture['destination']).read_bytes()).hexdigest()
                self.assertEqual(digest, fixture['sha256'], fixture['destination'])

    def test_canonical_records_share_one_declaration(self):
        modules = {m['module']: m for m in json.loads((SOURCE.parent / 'schema-inventory.json').read_text())}
        records = {family: dict(module['structs'], **module['unions']) for family, module in modules.items()}

        def compare_nested(first, second, name, seen, target=None):
            if name in seen:
                return
            seen.add(name)
            self.assertEqual(records[first][name], records[second][target or name])
            for dependency in set(re.findall(r'\b[A-Z]\w+', json.dumps(records[first][name]))):
                if dependency in records[first] or dependency in records[second]:
                    compare_nested(first, second, dependency, seen)

        for module in modules.values():
            for name, destination in module['destinations'].items():
                if 'canonical_path' not in destination:
                    continue
                path = destination['canonical_path']
                _, _, family, owner, target = path.split('::')
                kind = 'structs' if name in module['structs'] else 'unions'
                target_source = next(n for n, d in modules[family]['destinations'].items()
                                     if d['name'] == target and d['owner'] == owner
                                     and 'canonical_path' not in d)
                self.assertEqual(module[kind][name], modules[family][kind][target_source])
                compare_nested(module['module'], family, name, set(), target_source)
                self.assertEqual(GENERATOR.type_path(module, name), path)
                declaration = ('pub struct ' if kind == 'structs' else 'pub enum ') + destination['name'] + ' '
                manifest = json.loads((SOURCE.parent / 'bindings-generation.json').read_text())
                relative = 'crates/swap_coffee/src/contracts/' + module['module'] + '/' + destination['owner'] + '.rs'
                body = ((ROOT / relative).read_text() if relative in manifest['handwritten_files']
                        else GENERATOR.records(module, destination['owner']))
                self.assertNotIn(declaration, body)
                canonical_declaration = ('pub struct ' if kind == 'structs' else 'pub enum ') + target + ' '
                self.assertIn(canonical_declaration, GENERATOR.records(modules[family], owner))

    def test_callable_getters_reconcile_with_recording_cases(self):
        inventory = json.loads((SOURCE.parent / 'bindings.json').read_text())
        source = (ROOT / 'crates/swap_coffee/src/tests/test_getters.rs').read_text()
        cases = set(re.findall(r'fn (test_\w+)', source))
        rows = inventory['expanded_getters']
        self.assertEqual(len(inventory['wrappers']), 26)
        self.assertEqual(len(rows), 60)
        self.assertEqual(len({r['source'] for r in rows}), len(rows))
        self.assertEqual(cases, {r['recording_test'].split('::')[-1] for r in rows})
        for row in rows:
            wrapper = row['destination'].split('::')[-2]
            getter = row['destination'].split('::')[-1]
            test = row['recording_test'].split('::')[-1]
            block = source.split('fn ' + test + '(', 1)[1].split('#[test]', 1)[0]
            self.assertIn(wrapper + '::new(', block)
            self.assertIn('wrapper.' + getter + '(', block)
            self.assertRegex(block, r'client\(\s*' + str(row['selector']) + ',')
