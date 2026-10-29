//! DedustPoolV2 contract API. Data records live in [`storage`].
use self::{
    method_results::{
        DedustPoolV2EstimateDepositOutResult, DedustPoolV2EstimateSwapOutResult, DedustPoolV2GetAssetsResult,
        DedustPoolV2GetDecimalsResult, DedustPoolV2GetJettonDataResult, DedustPoolV2GetProtocolFeesResult,
        DedustPoolV2GetQuoteProvidersResult, DedustPoolV2GetReservesResult, DedustPoolV2GetScalingFactorsResult,
        DedustPoolV2GetTradeFeeResult,
    },
    storage::DedustPoolV2Storage,
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

pub mod method_results;
pub mod storage;
ton_contract!(DedustPoolV2Contract<DedustPoolV2Storage>);

pub mod msgs;

#[ton_methods]
impl DedustPoolV2Contract {
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
}
