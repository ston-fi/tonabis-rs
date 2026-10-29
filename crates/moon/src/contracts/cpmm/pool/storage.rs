//! Moon wire records. See tools/reference/moon/README.md.

use fastnum::U256;
use ton::ton_core::{
    TLB,
    cell::TonCell,
    errors::{TonCoreError, TonCoreResult},
    traits::tlb::TLB,
    types::{
        TonAddress,
        tlb_core::{
            TLBCoins, TLBRef,
            adapters::{InternalAddress, NullableAddress},
        },
    },
};
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmPoolInitData {
    #[tlb(adapter = "InternalAddress")]
    pub factory: TonAddress,
    pub masters: TLBRef<MoonCpmmPoolMasters>,
    pub fee_bps: u8,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmPoolMasters {
    #[tlb(adapter = "NullableAddress")]
    pub x_master: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_master: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub initiator: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmPoolReserves {
    pub x_reserve: TLBCoins,
    pub y_reserve: TLBCoins,
    pub x_fees_lp: U256,
    pub y_fees_lp: U256,
    pub x_fees_protocol: TLBCoins,
    pub y_fees_protocol: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmPoolStorage {
    #[tlb(adapter = "InternalAddress")]
    pub factory: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub x_wallet: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_wallet: TonAddress,
    pub lp_fee: u8,
    pub protocol_fee: u8,
    pub lp_total: TLBCoins,
    pub reserves: TLBRef<MoonCpmmPoolReserves>,
    pub lp_acc_code: TLBRef<TonCell>,
    pub lp_position_code: TLBRef<TonCell>,
    pub masters: TLBRef<MoonCpmmPoolMasters>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmPoolMastersLegacy {
    #[tlb(adapter = "NullableAddress")]
    pub x_master: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_master: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmPoolStorageLegacy {
    #[tlb(adapter = "InternalAddress")]
    pub factory: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub x_wallet: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_wallet: TonAddress,
    pub lp_fee: u8,
    pub protocol_fee: u8,
    pub lp_total: TLBCoins,
    pub reserves: TLBRef<MoonCpmmPoolReserves>,
    pub lp_acc_code: TLBRef<TonCell>,
    pub lp_position_code: TLBRef<TonCell>,
    pub masters: TLBRef<MoonCpmmPoolMastersLegacy>,
}

// Explicit wire-layout recognition; this does not authenticate deployed code.
#[derive(Debug, Clone, PartialEq)]
pub enum MoonCpmmPoolRevision {
    Current(MoonCpmmPoolStorage),
    Legacy(MoonCpmmPoolStorageLegacy),
}
impl MoonCpmmPoolRevision {
    pub fn from_cell(cell: &TonCell) -> TonCoreResult<Self> {
        let master_cell = cell
            .refs()
            .get(3)
            .ok_or_else(|| TonCoreError::data(module_path!(), "missing pool masters"))?;
        let mut parser = master_cell.parser();
        NullableAddress.read(&mut parser)?;
        InternalAddress.read(&mut parser)?;
        match parser.data_bits_left()? {
            0 => Ok(Self::Legacy(MoonCpmmPoolStorageLegacy::from_cell(cell)?)),
            267 => Ok(Self::Current(MoonCpmmPoolStorage::from_cell(cell)?)),
            _ => ton::ton_core::bail_ton_core_data!("unknown CPMM masters suffix"),
        }
    }
}
