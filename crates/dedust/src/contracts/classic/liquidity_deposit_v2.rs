//! DedustLiquidityDepositV2 contract API. Data records live in [`storage`].
use self::{
    method_results::{
        DedustLiquidityDepositV2GetBalancesResult, DedustLiquidityDepositV2GetPoolParamsResult,
        DedustLiquidityDepositV2GetTargetBalancesResult,
    },
    storage::DedustLiquidityDepositV2Storage,
};
use ton::ton_core::types::Coins;
use ton::{
    contracts::TonContract,
    errors::TonResult,
    ton_contract,
    ton_core::{ton_methods, types::TonAddress},
};

pub mod method_results;
pub mod storage;
ton_contract!(DedustLiquidityDepositV2Contract<DedustLiquidityDepositV2Storage>);

pub mod msgs;

#[ton_methods]
impl DedustLiquidityDepositV2Contract {
    pub async fn get_factory_addr(&self) -> TonResult<TonAddress>;

    pub async fn get_owner_addr(&self) -> TonResult<TonAddress>;

    pub async fn get_pool_addr(&self) -> TonResult<TonAddress>;

    pub async fn get_pool_params(&self) -> TonResult<DedustLiquidityDepositV2GetPoolParamsResult>;

    pub async fn get_target_balances(&self) -> TonResult<DedustLiquidityDepositV2GetTargetBalancesResult>;

    pub async fn get_balances(&self) -> TonResult<DedustLiquidityDepositV2GetBalancesResult>;

    pub async fn is_processing(&self) -> TonResult<bool>;

    pub async fn get_min_lp_amount(&self) -> TonResult<Coins>;
}
