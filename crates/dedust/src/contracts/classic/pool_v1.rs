//! DedustPoolV1 contract API. Data records live in [`storage`].
use self::{
    method_results::{
        DedustPoolV1EstimateDepositOutResult, DedustPoolV1EstimateSwapOutResult, DedustPoolV1GetAssetsResult,
        DedustPoolV1GetDecimalsResult, DedustPoolV1GetJettonDataResult, DedustPoolV1GetProtocolFeesResult,
        DedustPoolV1GetReservesResult, DedustPoolV1GetTradeFeeResult,
    },
    storage::DedustPoolV1Storage,
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
ton_contract!(DedustPoolV1Contract<DedustPoolV1Storage>);

pub mod msgs;

#[ton_methods]
impl DedustPoolV1Contract {
    pub async fn get_version(&self) -> TonResult<u16>;

    pub async fn is_stable(&self) -> TonResult<bool>;

    pub async fn get_trade_fee(&self) -> TonResult<DedustPoolV1GetTradeFeeResult>;

    pub async fn get_reserves(&self) -> TonResult<DedustPoolV1GetReservesResult>;

    pub async fn get_protocol_fees(&self) -> TonResult<DedustPoolV1GetProtocolFeesResult>;

    pub async fn get_assets(&self) -> TonResult<DedustPoolV1GetAssetsResult>;
    pub async fn estimate_swap_out(
        &self,
        asset_in: DedustClassicAsset,
        amount_in: I512,
    ) -> TonResult<DedustPoolV1EstimateSwapOutResult>;
    pub async fn estimate_deposit_out(
        &self,
        amount0: I512,
        amount1: I512,
    ) -> TonResult<DedustPoolV1EstimateDepositOutResult>;

    pub async fn get_jetton_data(&self) -> TonResult<DedustPoolV1GetJettonDataResult>;

    pub async fn get_wallet_address(&self, owner_address: TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(&owner_address)?);
        self.emulate_get_method(103289i32, &stack).await
    }
    pub async fn get_decimals(&self) -> TonResult<DedustPoolV1GetDecimalsResult>;
}
