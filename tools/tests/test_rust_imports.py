"""Generated type imports must preserve names, local scopes and function calls."""
import sys
from pathlib import Path
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'reference'))
from rust_imports import shorten_type_paths


class RustImportTests(unittest.TestCase):
    def test_literals_and_nested_comments_are_preserved(self):
        literals = [
            '"ton::ton_core::types::TonAddress"',
            '"escaped \\" ton::ton_core::types::TonAddress"',
            'r#"ton::ton_core::types::TonAddress"#',
            'br##"embedded \"# ton::ton_core::types::TonAddress"##',
            'cr#"ton::ton_core::types::TonAddress"#',
        ]
        comments = [
            '// ton::ton_core::types::TonAddress',
            '/* outer /* nested */ ton::ton_core::types::TonAddress */',
        ]
        source = '\n'.join('const LABEL: &str = ' + value + ';' for value in literals)
        source += '\n' + '\n'.join(comments) + '\n'
        self.assertEqual(shorten_type_paths(source), source)
        source += "fn owner<'a>(value: &'a ton::ton_core::types::TonAddress) {}\n"
        result = shorten_type_paths(source)
        for text in literals + comments:
            self.assertIn(text, result)
        self.assertIn("fn owner<'a>(value: &'a TonAddress)", result)
        self.assertEqual(shorten_type_paths(result), result)

    def test_nested_alias_collision_keeps_qualified_type(self):
        source = '''fn first() {
    use crate::other::Address as TonAddress;
}
fn second(value: ton::ton_core::types::TonAddress) {}
'''
        self.assertEqual(shorten_type_paths(source), source)

    def test_grouped_imports_collisions_and_function_paths(self):
        source = '''//! API documentation.
use ton::ton_core::{cell::TonCell, types::TonAddress};
use crate::first::Record;
pub struct Message {
    pub owner: ton::ton_core::types::TonAddress,
    pub body: ton::ton_core::cell::TonCell,
    pub other: crate::second::Record,
}
fn value() { crate::helper::build(); }
'''
        result = shorten_type_paths(source)
        self.assertIn('pub owner: TonAddress,', result)
        self.assertIn('pub body: TonCell,', result)
        self.assertIn('pub other: crate::second::Record,', result)
        self.assertIn('crate::helper::build()', result)
        self.assertEqual(result.count('use ton::ton_core'), 1)
        self.assertEqual(shorten_type_paths(result), result)

    def test_function_local_import_does_not_hide_required_module_import(self):
        source = '''fn first() {
    use ton::ton_core::types::TonAddress;
    let _ = TonAddress::ZERO;
}
fn second(value: ton::ton_core::types::TonAddress) {}
'''
        result = shorten_type_paths(source)
        self.assertTrue(result.startswith('use ton::ton_core::types::TonAddress;\n'))
        self.assertIn('fn second(value: TonAddress)', result)
        self.assertEqual(shorten_type_paths(result), result)
