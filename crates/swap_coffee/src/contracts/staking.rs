//! Typed staking dictionary values from pinned MIT Coffee documentation, with
//! structural layout confirmed against separately captured public accounts.
use ton::ton_core::TLB;
use ton::ton_core::types::{
    TonAddress,
    tlb_core::{TLBCoins, adapters::InternalAddress},
};

/// A configured lock duration and additional point percentage.
/// Period IDs are unsigned32 dictionary keys and need not start at zero or one.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeStakingPeriod {
    pub duration: u64,
    pub percentage: u64,
}

/// A configured staking wallet and its point normalizer.
/// The dictionary key is the hash of the serialized wallet address slice.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeStakingJettonConfiguration {
    #[tlb(adapter = "InternalAddress")]
    pub wallet_address: TonAddress,
    pub normalizer: TLBCoins,
}

/// A single on-chain reward distribution schedule, keyed by wallet-slice hash.
/// This is not the proprietary off-chain reward calculation/checkpoint backend.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeStakingRewardSchedule {
    pub duration: u64,
    pub finish_at: u64,
    pub reward_rate: TLBCoins,
}
