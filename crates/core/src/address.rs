//! Standard address encoding helpers for contract getter arguments.
use ton_core::{
    cell::TonCell,
    errors::TonCoreResult,
    types::{TonAddress, tlb_core::adapters::InternalAddress},
};

/// Encodes a standard internal address as a cell suitable for a TVM Slice.
///
/// Zero remains internal `0:0`; workchains outside int8 fail. The caller decides
/// whether to place the resulting cell on the stack as a Cell or a Slice.
pub fn internal_address_cell(address: &TonAddress) -> TonCoreResult<TonCell> {
    let mut builder = TonCell::builder();
    InternalAddress.write(&mut builder, address)?;
    builder.build()
}

/// Encodes addr_none or a standard internal address without a Maybe bit.
/// Some(internal zero) stays distinct from None.
pub fn nullable_address_cell(address: &Option<TonAddress>) -> TonCoreResult<TonCell> {
    let mut builder = TonCell::builder();
    ton_core::types::tlb_core::adapters::NullableAddress.write(&mut builder, address)?;
    builder.build()
}
