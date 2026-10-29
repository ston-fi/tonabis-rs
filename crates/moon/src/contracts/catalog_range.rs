//! Moon catalog_range contract API; storage and structured results live in child modules.
use crate::contracts::catalog_range::storage::MoonCatalogClmmRangeStorage;
use fastnum::I512;
use ton::block_tlb::TVMStack;
use ton::block_tlb::ToTVMStack;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::ton_methods;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::MsgAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
ton::ton_contract!(MoonCatalogClmmRangeContract<MoonCatalogClmmRangeStorage>);
#[ton_methods]
impl MoonCatalogClmmRangeContract {
    pub async fn get_range(&self) -> TonResult<TonCell>;
    pub async fn get_bins(&self) -> TonResult<Option<TonCell>>;
    pub async fn get_lp_account(&self, owner_address: &TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(owner_address)?);
        self.emulate_get_method("get_lp_account", &stack).await
    }
    pub async fn get_current_bin(&self) -> TonResult<I512>;
    pub async fn get_nft_address_by_index(&self, index: I512) -> TonResult<Option<TonAddress>> {
        let mut stack = TVMStack::default();
        index.to_stack(&mut stack)?;
        let value: MsgAddress = self.emulate_get_method("get_nft_address_by_index", &stack).await?;
        Ok(NullableInternalAddress::from_address(value)?)
    }
    pub async fn get_current_sqrt_price(&self) -> TonResult<I512>;
}

pub mod msgs;
pub mod storage;
