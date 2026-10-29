//! Moon legacy_pool contract API; storage and structured results live in child modules.
use crate::contracts::legacy_pool::method_results::MoonLegacyPoolGetAssetsResult;
use crate::contracts::legacy_pool::method_results::MoonLegacyPoolGetCollectedFeesResult;
use crate::contracts::legacy_pool::method_results::MoonLegacyPoolGetEstimatedLpBurnResult;
use crate::contracts::legacy_pool::method_results::MoonLegacyPoolGetEstimatedSwapOutResult;
use crate::contracts::legacy_pool::method_results::MoonLegacyPoolGetFeesResult;
use crate::contracts::legacy_pool::method_results::MoonLegacyPoolGetJettonDataResult;
use crate::contracts::legacy_pool::method_results::MoonLegacyPoolGetReservesResult;
use crate::contracts::legacy_pool::method_results::MoonLegacyPoolGetWalletsResult;
use crate::contracts::legacy_pool::storage::MoonLegacyPoolStorage;
use fastnum::I512;
use ton::block_tlb::TVMStack;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::ton_methods;
use ton::ton_core::types::TonAddress;
ton::ton_contract!(MoonLegacyPoolContract<MoonLegacyPoolStorage>);
#[ton_methods]
impl MoonLegacyPoolContract {
    pub async fn get_reserves(&self) -> TonResult<MoonLegacyPoolGetReservesResult>;
    pub async fn get_assets(&self) -> TonResult<MoonLegacyPoolGetAssetsResult>;
    pub async fn get_wallets(&self) -> TonResult<MoonLegacyPoolGetWalletsResult>;
    pub async fn get_fees(&self) -> TonResult<MoonLegacyPoolGetFeesResult>;
    pub async fn get_collected_fees(&self) -> TonResult<MoonLegacyPoolGetCollectedFeesResult>;
    pub async fn get_lp_total(&self) -> TonResult<I512>;
    pub async fn get_lp_account(&self, user_address: &TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(user_address)?);
        self.emulate_get_method("get_lp_account", &stack).await
    }
    pub async fn get_wallet_address(&self, owner_address: &TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(owner_address)?);
        self.emulate_get_method("get_wallet_address", &stack).await
    }
    pub async fn get_estimated_lp_burn(&self, lp_amount: I512) -> TonResult<MoonLegacyPoolGetEstimatedLpBurnResult>;
    pub async fn get_estimated_lp(&self, asset1_amount: TonCell, asset2_amount: TonCell) -> TonResult<I512>;
    pub async fn get_estimated_swap_out(
        &self,
        asset_and_amount: TonCell,
    ) -> TonResult<MoonLegacyPoolGetEstimatedSwapOutResult>;
    pub async fn get_jetton_data(&self) -> TonResult<MoonLegacyPoolGetJettonDataResult>;
    pub async fn get_pool(&self) -> TonResult<TonCell>;
}

pub mod method_results;
pub mod msgs;
pub mod storage;
