//! Persistent CPMM deposit state and configuration.
use crate::contracts::cpmm_v2::pool;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::{
    TLB,
    types::tlb_core::{TLBCoins, TLBRef},
};

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmDepositStorage {
    pub config: TLBRef<DedustV2CpmmDepositConfig>,
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub initiator: TonAddress,
    pub amount_x: TLBCoins,
    pub amount_y: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmDepositConfig {
    pub target_x: TLBCoins,
    pub target_y: TLBCoins,
    pub minimal_liquidity: TLBCoins,
    pub locked_liquidity_share: u16,
    pub payout_config: TLBRef<pool::storage::DedustV2CpmmExtendedPayoutConfig>,
}
