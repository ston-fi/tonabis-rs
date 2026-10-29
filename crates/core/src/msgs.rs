//! Message bodies with protocol-independent wire layouts.
use ton_core::{
    cell::{CellBuilder, CellParser},
    errors::{TonCoreError, TonCoreResult},
    traits::tlb::TLB,
};

/// Value-only funding body containing neither bits nor references.
///
/// Nonempty input reports a variant mismatch so derived dispatch can reach its
/// `Unknown` variant. A derived `ensure_empty` failure would stop dispatch.
#[derive(Debug, Clone, PartialEq)]
pub struct EmptyBodyMsg {}

impl TLB for EmptyBodyMsg {
    fn read_definition(parser: &mut CellParser) -> TonCoreResult<Self> {
        if parser.data_bits_left()? == 0 && parser.refs_left() == 0 {
            return Ok(Self {});
        }
        Err(TonCoreError::TLBEnumOutOfOptions {
            message: "expected empty funding body".into(),
            cell_boc_hex: parser.clone().read_remaining()?.to_boc_hex()?,
        })
    }
    fn write_definition(&self, _: &mut CellBuilder) -> TonCoreResult<()> {
        Ok(())
    }
}
