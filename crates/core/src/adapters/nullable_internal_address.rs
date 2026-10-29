use ton_core::{
    cell::{CellBuilder, CellParser},
    errors::{TonCoreError, TonCoreResult},
    types::{
        TonAddress,
        tlb_core::{MsgAddress, MsgAddressExt, MsgAddressInt, adapters::NullableAddress},
    },
};

/// Nullable standard internal address without a `Maybe` bit.
///
/// `None` is `addr_none`; `Some(TonAddress::ZERO)` remains internal zero.
/// External, variable-length and anycast addresses are rejected. Ordinary tails
/// remain for the enclosing record. Wire encoding delegates to TON's adapter.
pub struct NullableInternalAddress;

impl NullableInternalAddress {
    /// Reads one nullable standard internal address.
    pub fn read(&self, parser: &mut CellParser) -> TonCoreResult<Option<TonAddress>> {
        NullableAddress.read(parser)
    }

    /// Writes `addr_none` or a standard internal address, preserving internal zero.
    pub fn write(&self, builder: &mut CellBuilder, value: &Option<TonAddress>) -> TonCoreResult<()> {
        NullableAddress.write(builder, value)
    }

    /// Converts a decoded address without normalizing internal zero or anycast.
    /// TVM NULL handling belongs to the stack decoder, not this wire adapter.
    pub fn from_address(value: MsgAddress) -> TonCoreResult<Option<TonAddress>> {
        match value {
            MsgAddress::Int(MsgAddressInt::Std(address)) if address.anycast.is_none() => {
                Ok(Some(TonAddress::new(i32::from(address.workchain), address.address)))
            },
            MsgAddress::Ext(MsgAddressExt::None(_)) => Ok(None),
            _ => Err(TonCoreError::data(
                module_path!(),
                "expected standard internal address or addr_none",
            )),
        }
    }
}
