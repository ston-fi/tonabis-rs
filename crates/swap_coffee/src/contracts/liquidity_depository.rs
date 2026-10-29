//! CoffeeLiquidityDepository wrapper; callers own the client and code provenance.
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryStorage;
ton::ton_contract! { CoffeeLiquidityDepositoryContract< CoffeeLiquidityDepositoryStorage> }
pub mod msgs;
pub mod storage;

/// Exit codes declared by the pinned interface; provider errors retain their type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CoffeeLiquidityDepositoryErrors {
    UnknownOpcode = 6,
    InvalidRequest = 251,
    WrongSender = 252,
    NotAFactory = 253,
    InvalidLiquidity = 264,
    InvalidAssetType = 267,
}
