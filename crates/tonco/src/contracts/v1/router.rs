//! V1 router API and its generation-specific creation, forwarding and payout records.
use crate::contracts::v1::router::method_results::ToncoRouterV1GetChildContractsResult;
use crate::contracts::v1::router::method_results::ToncoRouterV1GetRouterStateResult;
use crate::contracts::v1::router::storage::ToncoRouterV1Storage;
use ton::block_tlb::TVMStack;
use ton::ton_core::types::TonAddress;
pub mod method_results;
pub mod msgs;
pub mod storage;

use ton::ton_core::cell::TonCell;
use ton::ton_core::ton_methods;
use ton::{contracts::TonContract, errors::TonResult, ton_contract};

// Adapted from ton-blockchain/abis 7dc57067, MIT; see reference provenance.

ton_contract!(ToncoRouterV1Contract<ToncoRouterV1Storage>);

#[ton_methods]
impl ToncoRouterV1Contract {
    #[ton_method(name = "getRouterState")]
    pub async fn get_router_state(&self) -> TonResult<ToncoRouterV1GetRouterStateResult>;
    pub async fn get_pool_address(
        &self,
        jetton_wallet0: &TonAddress,
        jetton_wallet1: &TonAddress,
    ) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(jetton_wallet0)?);
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(jetton_wallet1)?);
        self.emulate_get_method("getPoolAddress", &stack).await
    }
    #[ton_method(name = "getChildContracts")]
    pub async fn get_child_contracts(&self) -> TonResult<ToncoRouterV1GetChildContractsResult>;
    pub async fn get_pool_initial_data(
        &self,
        jetton_wallet0: &TonAddress,
        jetton_wallet1: &TonAddress,
    ) -> TonResult<TonCell> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(jetton_wallet0)?);
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(jetton_wallet1)?);
        self.emulate_get_method("getPoolInitialData", &stack).await
    }
    pub async fn get_pool_state_init(
        &self,
        jetton_wallet0: &TonAddress,
        jetton_wallet1: &TonAddress,
    ) -> TonResult<TonCell> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(jetton_wallet0)?);
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(jetton_wallet1)?);
        self.emulate_get_method("getPoolStateInit", &stack).await
    }
}
