//! A Maybe bit followed by a standard internal address.
use ton_core::{
    cell::{CellBuilder, CellParser},
    errors::TonCoreResult,
    types::{TonAddress, tlb_core::adapters::InternalAddress},
};

/// A Maybe bit followed by a standard internal address.
/// Unlike NullableAddress, absence consumes a Maybe bit. Internal zero stays internal.
pub struct MaybeAddress;
impl MaybeAddress {
    pub fn read(&self, parser: &mut CellParser) -> TonCoreResult<Option<TonAddress>> {
        if !parser.read_bit()? {
            return Ok(None);
        }
        Ok(Some(InternalAddress.read(parser)?))
    }
    pub fn write(&self, builder: &mut CellBuilder, value: &Option<TonAddress>) -> TonCoreResult<()> {
        builder.write_bit(value.is_some())?;
        if let Some(value) = value {
            InternalAddress.write(builder, value)?;
        }
        Ok(())
    }
}
