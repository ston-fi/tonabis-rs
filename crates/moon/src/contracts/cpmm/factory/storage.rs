//! Moon wire records. See tools/reference/moon/README.md.

use ton::ton_core::{
    TLB,
    cell::TonCell,
    errors::{TonCoreError, TonCoreResult},
    traits::tlb::TLB,
    types::{
        TonAddress,
        tlb_core::{TLBRef, adapters::InternalAddress},
    },
};
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmFactoryStorage {
    #[tlb(adapter = "InternalAddress")]
    pub admin: TonAddress,
    pub pool_code: TLBRef<TonCell>,
    pub lp_acc_code: TLBRef<TonCell>,
    pub lp_position_code: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmFactoryStorageLegacy {
    #[tlb(adapter = "InternalAddress")]
    pub admin: TonAddress,
    pub lp_fee: u8,
    pub protocol_fee: u8,
    pub pool_code: TLBRef<TonCell>,
    pub lp_acc_code: TLBRef<TonCell>,
    pub lp_position_code: TLBRef<TonCell>,
}

// Explicit wire-layout recognition; this does not authenticate deployed code.
#[derive(Debug, Clone, PartialEq)]
pub enum MoonCpmmFactoryRevision {
    Current(MoonCpmmFactoryStorage),
    Legacy(MoonCpmmFactoryStorageLegacy),
}
impl MoonCpmmFactoryRevision {
    pub fn from_cell(cell: &TonCell) -> TonCoreResult<Self> {
        let mut parser = cell.parser();
        InternalAddress.read(&mut parser)?;
        match parser.data_bits_left()? {
            0 => Ok(Self::Current(MoonCpmmFactoryStorage::from_cell(cell)?)),
            16 => Ok(Self::Legacy(MoonCpmmFactoryStorageLegacy::from_cell(cell)?)),
            _ => ton::ton_core::bail_ton_core_data!("unknown CPMM factory layout"),
        }
    }
}
