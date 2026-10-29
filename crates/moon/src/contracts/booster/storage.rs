//! Generated Moon wire records. See tools/reference/moon/README.md.
use ton::ton_core::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

/// Per-round boost schedule stored by an active booster.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonBoosterBoostData {
    pub amount_per_round: TLBCoins,
    pub start_time: u64,
    pub current_round: u16,
    pub round_interval: u64,
    pub round_count: u16,
    #[tlb(adapter = "NullableAddress")]
    pub jetton_wallet: Option<TonAddress>,
}

/// Persistent root stored before a boost schedule is installed.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonBoosterDeploymentStorage {
    pub status: u8,
    pub booster_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub admin: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub pool: Option<TonAddress>,
}

/// Persistent root stored while the booster has a boost schedule.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonBoosterStorage {
    pub status: u8,
    pub booster_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub admin: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub pool: Option<TonAddress>,
    pub boost: TLBRef<MoonBoosterBoostData>,
}
