//! V2 pool API: inline signed-key ticks, account/reforge flows and version-16000 getters.
use crate::contracts::v1::pool::method_results::ToncoPoolTickPage;
use crate::contracts::v2::pool::method_results::ToncoPoolV2AmountsResult;
use crate::contracts::v2::pool::method_results::ToncoPoolV2FeeGrowthResult;
use crate::contracts::v2::pool::method_results::ToncoPoolV2GetChildContractsResult;
use crate::contracts::v2::pool::method_results::ToncoPoolV2GetCollectionDataResult;
use crate::contracts::v2::pool::method_results::ToncoPoolV2GetMintEstimateResult;
use crate::contracts::v2::pool::method_results::ToncoPoolV2GetPendingTimelocksResult;
use crate::contracts::v2::pool::method_results::ToncoPoolV2GetPoolStateAndConfigurationResult;
use crate::contracts::v2::pool::storage::ToncoPoolV2Storage;
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

ton_contract!(ToncoPoolV2Contract<ToncoPoolV2Storage>);

#[ton_methods]
impl ToncoPoolV2Contract {
    #[ton_method(name = "getMaxLiquidityPerTick")]
    pub async fn get_max_liquidity_per_tick(&self) -> TonResult<I512>;
    #[ton_method(name = "getChildContracts")]
    pub async fn get_child_contracts(&self) -> TonResult<ToncoPoolV2GetChildContractsResult>;

    pub async fn get_collection_data(&self) -> TonResult<ToncoPoolV2GetCollectionDataResult>;

    pub async fn get_nft_content(&self, index: I512, content: TonCell) -> TonResult<TonCell>;

    #[ton_method(name = "getPoolStateAndConfiguration")]
    pub async fn get_pool_state_and_configuration(&self) -> TonResult<ToncoPoolV2GetPoolStateAndConfigurationResult>;
    #[ton_method(name = "getTickInfosFrom")]
    pub async fn get_tick_infos_from(
        &self,
        start: I512,
        count: I512,
        back: I512,
        full: I512,
    ) -> TonResult<ToncoPoolTickPage>;
    #[ton_method(name = "getAllTickInfos")]
    pub async fn get_all_tick_infos(&self) -> TonResult<Option<TonCell>>;
    #[ton_method(name = "getSwapEstimate")]
    pub async fn get_swap_estimate(
        &self,
        direction: I512,
        amount: I512,
        limit: I512,
    ) -> TonResult<ToncoPoolV2AmountsResult>;
    #[ton_method(name = "getSwapEstimateGas")]
    pub async fn get_swap_estimate_gas(
        &self,
        direction: I512,
        amount: I512,
        limit: I512,
        minimum: I512,
        gas_limit: I512,
    ) -> TonResult<ToncoPoolV2AmountsResult>;

    #[ton_method(name = "getIsActive")]
    pub async fn get_is_active(&self) -> TonResult<bool>;

    pub async fn get_pending_timelocks(&self) -> TonResult<ToncoPoolV2GetPendingTimelocksResult>;
    #[ton_method(name = "getMintEstimate")]
    pub async fn get_mint_estimate(
        &self,
        lower: I512,
        upper: I512,
        liquidity: I512,
    ) -> TonResult<ToncoPoolV2GetMintEstimateResult>;
    #[ton_method(name = "getCollectedFees")]
    pub async fn get_collected_fees(
        &self,
        lower: I512,
        upper: I512,
        liquidity: I512,
        checkpoint0: I512,
        checkpoint1: I512,
    ) -> TonResult<ToncoPoolV2AmountsResult>;
    #[ton_method(name = "getFeeGrowthInside")]
    pub async fn get_fee_growth_inside(
        &self,
        lower: I512,
        upper: I512,
        current: I512,
        global0: I512,
        global1: I512,
    ) -> TonResult<ToncoPoolV2FeeGrowthResult>;
    pub async fn get_user_account_address(&self, owner: &TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(owner)?);
        self.emulate_get_method("getUserAccountAddress", &stack).await
    }

    pub async fn get_nft_address_by_index(&self, index: I512) -> TonResult<Option<TonAddress>> {
        let mut stack = TVMStack::default();
        index.to_stack(&mut stack)?;
        let value: MsgAddress = self.emulate_get_method("get_nft_address_by_index", &stack).await?;
        Ok(NullableInternalAddress::from_address(value)?)
    }
}
