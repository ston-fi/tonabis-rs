//! V1 pool API: referenced signed-key tick storage and the catalog mint/burn bodies.
use crate::contracts::v1::pool::method_results::ToncoPoolTickPage;
use crate::contracts::v1::pool::method_results::ToncoPoolV1GetChildContractsResult;
use crate::contracts::v1::pool::method_results::ToncoPoolV1GetCollectedFeesResult;
use crate::contracts::v1::pool::method_results::ToncoPoolV1GetCollectionDataResult;
use crate::contracts::v1::pool::method_results::ToncoPoolV1GetFeeGrowthInsideResult;
use crate::contracts::v1::pool::method_results::ToncoPoolV1GetMintEstimateResult;
use crate::contracts::v1::pool::method_results::ToncoPoolV1GetPoolStateAndConfigurationResult;
use crate::contracts::v1::pool::method_results::ToncoPoolV1GetSwapEstimateResult;
use crate::contracts::v1::pool::storage::ToncoPoolV1Storage;
use ton::block_tlb::TVMStack;
use ton::block_tlb::ToTVMStack;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
pub mod method_results;
pub mod msgs;
pub mod storage;

use fastnum::I512;
use ton::ton_core::cell::TonCell;
use ton::ton_core::ton_methods;
use ton::ton_core::types::tlb_core::MsgAddress;
use ton::{contracts::TonContract, errors::TonResult, ton_contract};

// Adapted from ton-blockchain/abis 7dc57067, MIT; see reference provenance.

ton_contract!(ToncoPoolV1Contract<ToncoPoolV1Storage>);

#[ton_methods]
impl ToncoPoolV1Contract {
    #[ton_method(name = "getTickInfosFrom")]
    pub async fn get_tick_infos_from(
        &self,
        start: I512,
        count: I512,
        back: I512,
        full: I512,
    ) -> TonResult<ToncoPoolTickPage>;

    #[ton_method(name = "getIsActive")]
    pub async fn get_is_active(&self) -> TonResult<bool>;
    #[ton_method(name = "getPoolStateAndConfiguration")]
    pub async fn get_pool_state_and_configuration(&self) -> TonResult<ToncoPoolV1GetPoolStateAndConfigurationResult>;
    #[ton_method(name = "getChildContracts")]
    pub async fn get_child_contracts(&self) -> TonResult<ToncoPoolV1GetChildContractsResult>;
    #[ton_method(name = "getAllTickInfos")]
    pub async fn get_all_tick_infos(&self) -> TonResult<TonCell>;
    #[ton_method(name = "getMaxLiquidityPerTick")]
    pub async fn get_max_liquidity_per_tick(&self) -> TonResult<I512>;
    #[ton_method(name = "getCollectedFees")]
    pub async fn get_collected_fees(
        &self,
        tick_lower: I512,
        tick_upper: I512,
        pos_liquidity_delta: I512,
        pos_fee_growth_inside0_x128: I512,
        pos_fee_growth_inside1_x128: I512,
    ) -> TonResult<ToncoPoolV1GetCollectedFeesResult>;
    #[ton_method(name = "getMintEstimate")]
    pub async fn get_mint_estimate(
        &self,
        tick_lower: I512,
        tick_upper: I512,
        liquidity: I512,
    ) -> TonResult<ToncoPoolV1GetMintEstimateResult>;
    #[ton_method(name = "getSwapEstimate")]
    pub async fn get_swap_estimate(
        &self,
        zero_for_one: I512,
        amount: I512,
        sqrt_price_limit_x96: I512,
    ) -> TonResult<ToncoPoolV1GetSwapEstimateResult>;
    #[ton_method(name = "getSwapEstimateGas")]
    pub async fn get_swap_estimate_gas(
        &self,
        zero_for_one: I512,
        amount: I512,
        sqrt_price_limit_x96: I512,
        min_out_amount: I512,
        gas_limit: I512,
    ) -> TonResult<ToncoPoolV1GetSwapEstimateResult>;
    #[ton_method(name = "getFeeGrowthInside")]
    pub async fn get_fee_growth_inside(
        &self,
        tick_lower: I512,
        tick_upper: I512,
        tick_current: I512,
        fee_growth_global0_x128: I512,
        fee_growth_global1_x128: I512,
    ) -> TonResult<ToncoPoolV1GetFeeGrowthInsideResult>;
    pub async fn get_user_account_address(&self, user_address: &TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(user_address)?);
        self.emulate_get_method("getUserAccountAddress", &stack).await
    }

    pub async fn get_collection_data(&self) -> TonResult<ToncoPoolV1GetCollectionDataResult>;

    pub async fn get_nft_address_by_index(&self, index: I512) -> TonResult<Option<TonAddress>> {
        let mut stack = TVMStack::default();
        index.to_stack(&mut stack)?;
        let value: MsgAddress = self.emulate_get_method("get_nft_address_by_index", &stack).await?;
        Ok(NullableInternalAddress::from_address(value)?)
    }

    pub async fn get_nft_content(&self, index: I512, nftv3item_content: TonCell) -> TonResult<TonCell>;
}
