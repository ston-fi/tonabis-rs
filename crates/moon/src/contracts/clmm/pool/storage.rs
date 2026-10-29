//! Generated Moon wire records. See tools/reference/moon/README.md.
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmPoolInitData {
    #[tlb(adapter = "InternalAddress")]
    pub factory: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub x_master: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_master: TonAddress,
    pub bin_step: u16,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmPoolStorage {
    #[tlb(adapter = "InternalAddress")]
    pub factory: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub x_master: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_master: TonAddress,
    pub bin_step: u16,
    pub lp_fee: u8,
    pub protocol_fee: u8,
    pub range_code: TLBRef<TonCell>,
    pub lp_acc_code: TLBRef<TonCell>,
    pub lp_position_code: TLBRef<TonCell>,
}
