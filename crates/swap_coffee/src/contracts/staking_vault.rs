//! CoffeeStakingVault wrapper; callers own the client and code provenance.
use crate::contracts::staking_vault::method_results::CoffeeStakingVaultGetStoredDataResult;
use crate::contracts::staking_vault::storage::CoffeeStakingVaultStorage;
use ton::block_tlb::TVMStack;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
ton::ton_contract! { CoffeeStakingVaultContract<CoffeeStakingVaultStorage> }
#[ton::ton_core::ton_methods]
impl CoffeeStakingVaultContract {
    pub async fn get_stored_data(&self) -> TonResult<CoffeeStakingVaultGetStoredDataResult>;

    pub async fn get_master_address(&self) -> TonResult<TonAddress> {
        let stack = TVMStack::default();
        let cell: TonCell = self.emulate_get_method("get_master_address", &stack).await?;
        Ok(InternalAddress.read(&mut cell.parser())?)
    }
}
pub mod method_results;
pub mod msgs;
pub mod storage;

/// Exit codes declared by the pinned interface; provider errors retain their type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CoffeeStakingVaultErrors {
    InvalidAmount = 80,
    InvalidDictionaryValue = 82,
    NotInitialized = 99,
    AlreadyInitialized = 100,
    UnauthorizedAccess = 403,
    WrongPeriod = 503,
    UnexpectedForwardOpcode = 504,
    UnsupportedOperation = 65535,
}
