//! Persistent classic LP wallet state.
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::{
    TLB,
    cell::TonCell,
    types::tlb_core::{TLBCoins, TLBRef},
};

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustClassicLpWalletStorage {
    pub balance: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    pub wallet_code: TLBRef<TonCell>,
}
