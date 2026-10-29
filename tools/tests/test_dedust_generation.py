"""Protect handwritten children from generator ownership regressions."""
import importlib.util
from pathlib import Path
import tempfile
import unittest

SOURCE = Path(__file__).resolve().parents[1] / "reference/dedust/generate_bindings.py"
SPEC = importlib.util.spec_from_file_location("dedust_bindings", SOURCE)
GENERATOR = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GENERATOR)

class GenerationOwnershipTests(unittest.TestCase):
    def test_generator_refuses_to_overwrite_handwritten_children(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for child in tuple(mod+".rs" for mod in GENERATOR.MODS.values()) + ("classic/operator_pool.rs", "classic/operator_pool/getters.rs",
                          "classic/asset.rs", "classic/factory_v1/method_results.rs", "classic/pool_v2/method_results.rs",
                          "classic/operator_pool/storage.rs", "classic/lp_wallet/storage.rs",
                          "cpmm_v2/position/storage.rs", "cpmm_v2/deposit/storage.rs",
                          "custom_wallet/v1/msgs.rs", "custom_wallet/v2/msgs.rs", "custom_wallet/v2/storage.rs",
                          "cpmm_v2/pool/msgs.rs"):
                path = root / "crates/dedust/src/contracts" / child
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("handwritten behavior")
                with self.assertRaisesRegex(ValueError, "handwritten"):
                    GENERATOR.write_generated(path, "generated", root)
                self.assertEqual(path.read_text(), "handwritten behavior")
            path = root / "crates/dedust/src/contracts/classic/factory_v1/storage.rs"
            GENERATOR.write_generated(path, "generated data", root)
            self.assertEqual(path.read_text(), "generated data")

    def test_type_names_preserve_variant_labels_and_wire_literals(self):
        source = '''// DedustPoolParams is the upstream schema identity.
#[tlb(adapter = "TLBRef<DedustPoolParams>")]
pub struct DedustPoolParams { pub value: u32 }
pub enum Holder {
DedustPoolParams(Box<DedustPoolParams>),
}
const SOURCE: &str = "DedustPoolParams";
'''
        renamed = GENERATOR.rename_source(source, 'classic/liquidity_deposit_v1/storage')
        self.assertIn('struct DedustLiquidityDepositV1PoolParams', renamed)
        self.assertIn('DedustPoolParams(Box<DedustLiquidityDepositV1PoolParams>)', renamed)
        self.assertIn('"TLBRef<DedustLiquidityDepositV1PoolParams>"', renamed)
        self.assertIn('// DedustPoolParams is the upstream schema identity.', renamed)
        self.assertIn('"DedustPoolParams";', renamed)
        other = GENERATOR.rename_source(source, 'classic/liquidity_deposit_v2/storage')
        self.assertIn('struct DedustLiquidityDepositV2PoolParams', other)
