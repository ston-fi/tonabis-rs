//! DedustLiquidityDepositV1 contract API. Data records live in [`storage`].
use self::{
    method_results::{
        DedustLiquidityDepositV1GetBalancesResult, DedustLiquidityDepositV1GetPoolParamsResult,
        DedustLiquidityDepositV1GetTargetBalancesResult,
    },
    storage::DedustLiquidityDepositV1Storage,
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
ton_contract!(DedustLiquidityDepositV1Contract<DedustLiquidityDepositV1Storage>);

pub mod msgs;

#[ton_methods]
impl DedustLiquidityDepositV1Contract {
    pub async fn get_factory_addr(&self) -> TonResult<TonAddress>;

    pub async fn get_owner_addr(&self) -> TonResult<TonAddress>;

    pub async fn get_pool_addr(&self) -> TonResult<TonAddress>;

    pub async fn get_pool_params(&self) -> TonResult<DedustLiquidityDepositV1GetPoolParamsResult>;

    pub async fn get_target_balances(&self) -> TonResult<DedustLiquidityDepositV1GetTargetBalancesResult>;

    pub async fn get_balances(&self) -> TonResult<DedustLiquidityDepositV1GetBalancesResult>;

    pub async fn is_processing(&self) -> TonResult<bool>;

    pub async fn get_min_lp_amount(&self) -> TonResult<Coins>;
}
