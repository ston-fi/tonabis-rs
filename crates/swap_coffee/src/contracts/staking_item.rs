//! CoffeeStakingItem wrapper; callers own the client and code provenance.
use crate::contracts::staking_item::method_results::CoffeeStakingItemGetNftDataResult;
use crate::contracts::staking_item::method_results::CoffeeStakingItemGetStoredDataResult;
use crate::contracts::staking_item::storage::CoffeeStakingItemInitialData;
use crate::contracts::staking_item::storage::CoffeeStakingItemStorage;
use ton::contracts::TonContract;
use ton::errors::TonResult;
ton::ton_contract! { CoffeeStakingItemContract<CoffeeStakingItemStorage> }
#[ton::ton_core::ton_methods]
impl CoffeeStakingItemContract {
    pub async fn get_stored_data(&self) -> TonResult<CoffeeStakingItemGetStoredDataResult>;

    pub async fn get_nft_data(&self) -> TonResult<CoffeeStakingItemGetNftDataResult>;
}
ton::ton_contract! { CoffeeStakingItemDeploymentContract< CoffeeStakingItemInitialData> }
#[ton::ton_core::ton_methods]
impl CoffeeStakingItemDeploymentContract {
    pub async fn get_stored_data(&self) -> TonResult<CoffeeStakingItemGetStoredDataResult>;

    pub async fn get_nft_data(&self) -> TonResult<CoffeeStakingItemGetNftDataResult>;
}
pub mod method_results;
pub mod msgs;
pub mod storage;

/// Exit codes declared by the pinned interface; provider errors retain their type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CoffeeStakingItemErrors {
    NotInitialized = 99,
    AlreadyInitialized = 100,
    TooEarlyToUnlock = 500,
    TooLateToExtend = 501,
    ZeroPoints = 502,
    WrongPeriod = 503,
    UnauthorizedAccess = 403,
    UnsupportedOperation = 65535,
}
