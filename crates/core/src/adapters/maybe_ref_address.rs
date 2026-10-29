//! A Maybe reference to a standard internal address.
use ton_core::{
    cell::{CellBuilder, CellParser, TonCell},
    errors::TonCoreResult,
    types::{TonAddress, tlb_core::adapters::InternalAddress},
};

/// A Maybe reference to a standard internal address.
/// Unlike NullableAddress, absence consumes a Maybe bit. Internal zero stays internal.
pub struct MaybeRefAddress;
impl MaybeRefAddress {
    pub fn read(&self, parser: &mut CellParser) -> TonCoreResult<Option<TonAddress>> {
        if !parser.read_bit()? {
            return Ok(None);
        }
        Ok(Some(InternalAddress.read(&mut parser.read_next_ref()?.parser())?))
    }
    pub fn write(&self, builder: &mut CellBuilder, value: &Option<TonAddress>) -> TonCoreResult<()> {
        builder.write_bit(value.is_some())?;
        if let Some(value) = value {
            let mut child = TonCell::builder();
            InternalAddress.write(&mut child, value)?;
            builder.write_ref(child.build()?)?;
        }
        Ok(())
    }
}
