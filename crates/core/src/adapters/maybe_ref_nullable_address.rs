//! A Maybe reference to a nullable standard address.
use ton_core::{
    cell::{CellBuilder, CellParser, TonCell},
    errors::TonCoreResult,
    types::{TonAddress, tlb_core::adapters::NullableAddress},
};

/// A Maybe reference to a nullable standard address.
/// Unlike NullableAddress, absence consumes a Maybe bit. Internal zero stays internal.
pub struct MaybeRefNullableAddress;
impl MaybeRefNullableAddress {
    pub fn read(&self, parser: &mut CellParser) -> TonCoreResult<Option<Option<TonAddress>>> {
        if !parser.read_bit()? {
            return Ok(None);
        }
        Ok(Some(NullableAddress.read(&mut parser.read_next_ref()?.parser())?))
    }
    pub fn write(&self, builder: &mut CellBuilder, value: &Option<Option<TonAddress>>) -> TonCoreResult<()> {
        builder.write_bit(value.is_some())?;
        if let Some(value) = value {
            let mut child = TonCell::builder();
            NullableAddress.write(&mut child, value)?;
            builder.write_ref(child.build()?)?;
        }
        Ok(())
    }
}
