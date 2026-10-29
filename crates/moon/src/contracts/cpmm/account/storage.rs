//! Generated Moon wire records. See tools/reference/moon/README.md.
use ton::ton_core::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmLpAccountInitData {
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub user: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmLpAccountStorage {
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub user: TonAddress,
    pub x_amount: TLBCoins,
    pub y_amount: TLBCoins,
}
