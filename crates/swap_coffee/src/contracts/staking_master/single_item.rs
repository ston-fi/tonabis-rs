use crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult;
use crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult;
use ton::block_tlb::TVMStack;
use ton::block_tlb::ToTVMStack;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
// Two-reference master generation, selected explicitly by callers.
use fastnum::I512;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
pub mod storage;
ton::ton_contract! { CoffeeStakingMasterSingleItemContract<storage::CoffeeStakingMasterSingleItemStorage> }
ton::ton_contract! { CoffeeStakingMasterSingleItemDeploymentContract<storage::CoffeeStakingMasterSingleItemInitialData> }
#[ton::ton_core::ton_methods]
impl CoffeeStakingMasterSingleItemContract {
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
#[ton::ton_core::ton_methods]
impl CoffeeStakingMasterSingleItemDeploymentContract {
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
