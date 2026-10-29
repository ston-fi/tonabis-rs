//! Classic operator-managed PoolV2 code778f0d3f/c0f9d14f. These are distinct
//! from ordinary code1275095b and from CPMM v2. Both add a swap activation time.
use self::storage::DedustClassicOperatorPoolStorage;
use super::pool_v2::method_results::{
    DedustPoolV2EstimateDepositOutResult, DedustPoolV2EstimateSwapOutResult, DedustPoolV2GetAssetsResult,
    DedustPoolV2GetDecimalsResult, DedustPoolV2GetJettonDataResult, DedustPoolV2GetProtocolFeesResult,
    DedustPoolV2GetQuoteProvidersResult, DedustPoolV2GetReservesResult, DedustPoolV2GetScalingFactorsResult,
    DedustPoolV2GetTradeFeeResult,
};
use crate::contracts::classic::asset::DedustClassicAsset;
use fastnum::I512;
use ton::{
    block_tlb::TVMStack,
    contracts::TonContract,
    errors::TonResult,
    ton_contract,
    ton_core::{ton_methods, types::TonAddress},
};

ton_contract!(DedustClassicOperatorPoolContract<DedustClassicOperatorPoolStorage>);

pub mod msgs;
pub mod storage;
#[ton_methods]
impl DedustClassicOperatorPoolContract {
    pub async fn get_version(&self) -> TonResult<u16>;

    pub async fn is_stable(&self) -> TonResult<bool>;

    pub async fn get_trade_fee(&self) -> TonResult<DedustPoolV2GetTradeFeeResult>;

    pub async fn get_reserves(&self) -> TonResult<DedustPoolV2GetReservesResult>;

    pub async fn get_protocol_fees(&self) -> TonResult<DedustPoolV2GetProtocolFeesResult>;

    pub async fn get_assets(&self) -> TonResult<DedustPoolV2GetAssetsResult>;
    pub async fn estimate_swap_out(
        &self,
        asset_in: DedustClassicAsset,
        amount_in: I512,
    ) -> TonResult<DedustPoolV2EstimateSwapOutResult>;
    pub async fn estimate_deposit_out(
        &self,
        amount0: I512,
        amount1: I512,
    ) -> TonResult<DedustPoolV2EstimateDepositOutResult>;

    pub async fn get_jetton_data(&self) -> TonResult<DedustPoolV2GetJettonDataResult>;

    pub async fn get_wallet_address(&self, owner_address: TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(&owner_address)?);
        self.emulate_get_method(103289i32, &stack).await
    }
    pub async fn get_quote_providers(&self) -> TonResult<DedustPoolV2GetQuoteProvidersResult>;
    pub async fn get_decimals(&self) -> TonResult<DedustPoolV2GetDecimalsResult>;
    pub async fn get_scaling_factors(&self) -> TonResult<DedustPoolV2GetScalingFactorsResult>;
    pub async fn get_start_at(&self) -> TonResult<u32>;
}
