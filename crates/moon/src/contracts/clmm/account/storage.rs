//! Generated Moon wire records. See tools/reference/moon/README.md.
use ton::ton_core::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmLpAccInitData {
    #[tlb(adapter = "InternalAddress")]
    pub range: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub user: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmLpAccountInitData {
    #[tlb(adapter = "InternalAddress")]
    pub range: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub user: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmLpAccountStorage {
    #[tlb(adapter = "InternalAddress")]
    pub range: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub user: TonAddress,
    pub x_amount: TLBCoins,
    pub y_amount: TLBCoins,
}
