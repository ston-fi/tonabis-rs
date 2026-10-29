//! CoffeeStakingMaster wrapper; callers own the client and code provenance.
use crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult;
use crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult;
use crate::contracts::staking_master::storage::CoffeeStakingMasterInitialData;
use crate::contracts::staking_master::storage::CoffeeStakingMasterStorage;
use fastnum::I512;
use ton::block_tlb::TVMStack;
use ton::block_tlb::ToTVMStack;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
ton::ton_contract! { CoffeeStakingMasterContract<CoffeeStakingMasterStorage> }
#[ton::ton_core::ton_methods]
impl CoffeeStakingMasterContract {
    pub async fn get_stored_data(&self) -> TonResult<CoffeeStakingMasterGetStoredDataResult>;

    pub async fn get_collection_data(&self) -> TonResult<CoffeeStakingMasterGetCollectionDataResult>;

    pub async fn get_nft_address_by_index(&self, item_index: &I512) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        item_index.to_stack(&mut stack)?;
        let cell: TonCell = self.emulate_get_method("get_nft_address_by_index", &stack).await?;
        Ok(InternalAddress.read(&mut cell.parser())?)
    }

    pub async fn get_nft_content(&self, item_index: &I512, individual_content: &TonCell) -> TonResult<TonCell>;
}
ton::ton_contract! { CoffeeStakingMasterDeploymentContract< CoffeeStakingMasterInitialData> }
#[ton::ton_core::ton_methods]
impl CoffeeStakingMasterDeploymentContract {
    pub async fn get_stored_data(&self) -> TonResult<CoffeeStakingMasterGetStoredDataResult>;

    pub async fn get_collection_data(&self) -> TonResult<CoffeeStakingMasterGetCollectionDataResult>;

    pub async fn get_nft_address_by_index(&self, item_index: &I512) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        item_index.to_stack(&mut stack)?;
        let cell: TonCell = self.emulate_get_method("get_nft_address_by_index", &stack).await?;
        Ok(InternalAddress.read(&mut cell.parser())?)
    }

    pub async fn get_nft_content(&self, item_index: &I512, individual_content: &TonCell) -> TonResult<TonCell>;
}
pub mod method_results;
pub mod msgs;
pub mod single_item;
pub mod storage;

/// Exit codes declared by the pinned interface; provider errors retain their type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CoffeeStakingMasterErrors {
    UnknownJettonWallet = 81,
    NotInitialized = 99,
    AlreadyInitialized = 100,
    MismatchingDuration = 110,
    ZeroRewardRate = 111,
    TokenNotDistributed = 200,
    UnauthorizedAccess = 403,
    WrongPeriod = 503,
    UnsupportedOperation = 65535,
}
