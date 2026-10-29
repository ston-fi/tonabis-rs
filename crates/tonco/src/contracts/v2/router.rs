//! V2 router API. Optional getter addresses preserve NULL independently of present addr_none.
use crate::contracts::v2::router::method_results::ToncoRouterV2GetChildContractsResult;
use crate::contracts::v2::router::method_results::ToncoRouterV2GetPendingTimelocksResult;
use crate::contracts::v2::router::method_results::ToncoRouterV2GetRouterStateResult;
use crate::contracts::v2::router::storage::ToncoRouterV2Storage;
use ton::block_tlb::TVMStack;
use ton::ton_core::types::TonAddress;
pub mod method_results;
pub mod msgs;
pub mod storage;

use ton::ton_core::cell::TonCell;
use ton::ton_core::ton_methods;
use ton::{contracts::TonContract, errors::TonResult, ton_contract};

ton_contract!(ToncoRouterV2Contract<ToncoRouterV2Storage>);

#[ton_methods]
impl ToncoRouterV2Contract {
    #[ton_method(name = "getChildContracts")]
    pub async fn get_child_contracts(&self) -> TonResult<ToncoRouterV2GetChildContractsResult>;

    pub async fn get_pending_timelocks(&self) -> TonResult<ToncoRouterV2GetPendingTimelocksResult>;

    #[ton_method(name = "getRouterState")]
    pub async fn get_router_state(&self) -> TonResult<ToncoRouterV2GetRouterStateResult>;
    pub async fn get_pool_address(&self, wallet0: &TonAddress, wallet1: &TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(wallet0)?);
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(wallet1)?);
        self.emulate_get_method("getPoolAddress", &stack).await
    }
    pub async fn get_pool_initial_data(&self, wallet0: &TonAddress, wallet1: &TonAddress) -> TonResult<TonCell> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(wallet0)?);
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(wallet1)?);
        self.emulate_get_method("getPoolInitialData", &stack).await
    }
    pub async fn get_pool_state_init(&self, wallet0: &TonAddress, wallet1: &TonAddress) -> TonResult<TonCell> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(wallet0)?);
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(wallet1)?);
        self.emulate_get_method("getPoolStateInit", &stack).await
    }
}
