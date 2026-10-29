//! Generated Moon wire records. See tools/reference/moon/README.md.
use fastnum::U256;
use std::collections::HashMap;
use ton::tlb_adapters::DictKeyAdapterInt;
use ton::tlb_adapters::DictValAdapterTLB;
use ton::tlb_adapters::TLBHashMapE;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmBin {
    pub l: U256,
    pub x_fee: U256,
    pub y_fee: U256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmRangeInitData {
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    pub range_num: i16,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmRangeInitPrice {
    pub init_bin: i64,
    pub init_sqrt_price: U256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmRangeState {
    pub current_bin: i64,
    pub current_sqrt_p: U256,
    pub x_protocol_fees: TLBCoins,
    pub y_protocol_fees: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmRangeStorage {
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    pub range_num: i16,
    #[tlb(adapter = "NullableAddress")]
    pub x_wallet: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_wallet: TonAddress,
    pub bin_step: u16,
    pub lp_fee: u8,
    pub protocol_fee: u8,
    pub position_index: u64,
    pub range_state: TLBRef<MoonClmmRangeState>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterInt<64, i64>, DictValAdapterTLB<MoonClmmBin>>::new(64)")]
    pub bins: HashMap<i64, MoonClmmBin>,
    pub lp_account_code: TLBRef<TonCell>,
    pub lp_position_code: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmRangeUninitMasters {
    #[tlb(adapter = "NullableAddress")]
    pub x_master: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_master: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmRangeUninitStorage {
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    pub range_num: i16,
    #[tlb(adapter = "NullableAddress")]
    pub x_wallet: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub y_wallet: Option<TonAddress>,
    pub bin_step: u16,
    pub lp_fee: u8,
    pub protocol_fee: u8,
    pub nft_index: u64,
    pub range_state: TLBRef<MoonClmmRangeState>,
    pub masters: TLBRef<MoonClmmRangeUninitMasters>,
    pub lp_account_code: TLBRef<TonCell>,
    pub lp_position_code: TLBRef<TonCell>,
}
