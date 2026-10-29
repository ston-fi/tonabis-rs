//! Moon wire records. See tools/reference/moon/README.md.

use fastnum::U256;
use ton::ton_core::{
    TLB,
    cell::TonCell,
    errors::TonCoreResult,
    traits::tlb::TLB,
    types::{
        TonAddress,
        tlb_core::{TLBCoins, TLBRef, adapters::InternalAddress},
    },
};
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmLpPositionContent {
    pub lp: TLBCoins,
    pub x_fees_lp: U256,
    pub y_fees_lp: U256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmLpPositionInitData {
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub user: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmLpPositionStorage {
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub user: TonAddress,
    pub data: TLBRef<MoonCpmmLpPositionContent>,
    pub frozen: bool,
    pub locked: bool,
}

// Explicit wire-layout recognition; this does not authenticate deployed code.
#[derive(Debug, Clone, PartialEq)]
pub enum MoonCpmmPositionState {
    Init(MoonCpmmLpPositionInitData),
    Live(MoonCpmmLpPositionStorage),
}
impl MoonCpmmPositionState {
    pub fn from_cell(cell: &TonCell) -> TonCoreResult<Self> {
        if cell.data_len_bits() == 534 && cell.refs().is_empty() {
            Ok(Self::Init(MoonCpmmLpPositionInitData::from_cell(cell)?))
        } else {
            Ok(Self::Live(MoonCpmmLpPositionStorage::from_cell(cell)?))
        }
    }
}
