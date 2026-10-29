//! V1 pool factory API and order-child initialization. Caller supplies executable code.
use crate::contracts::v1::factory::method_results::ToncoFactoryV1GetPoolFactoryDataResult;
use crate::contracts::v1::factory::storage::ToncoFactoryV1Storage;
use ton::block_tlb::TVMStack;
use ton::ton_core::types::TonAddress;
pub mod method_results;
pub mod msgs;
pub mod storage;

use ton::ton_core::ton_methods;
use ton::{contracts::TonContract, errors::TonResult, ton_contract};

ton_contract!(ToncoFactoryV1Contract<ToncoFactoryV1Storage>);

#[ton_methods]
impl ToncoFactoryV1Contract {
    pub async fn get_order_address(&self, minter0: &TonAddress, minter1: &TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(minter0)?);
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(minter1)?);
        self.emulate_get_method("getOrderAddress", &stack).await
    }

    #[ton_method(name = "getPoolFactoryData")]
    pub async fn get_pool_factory_data(&self) -> TonResult<ToncoFactoryV1GetPoolFactoryDataResult>;
}
