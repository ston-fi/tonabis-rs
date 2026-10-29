use crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolEstimateLiquidityDepositAmountResult;
use crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolEstimateLiquidityWithdrawAmountResult;
use crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolEstimateSwapAmountResult;
use crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolGetJettonDataResult;
use crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolGetPoolDataResult;
use crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolStorage;
use crate::contracts::pool_constant_product::storage::CoffeePoolAsset;
// CoffeePoolConstantProduct wrapper; callers own the client and code provenance.
use fastnum::I512;
use ton::block_tlb::TVMStack;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::{
    cell::TonCell,
    types::{TonAddress, tlb_core::adapters::InternalAddress},
};
ton::ton_contract! { CoffeeConstantProductPoolContract< CoffeeConstantProductPoolStorage> }
#[ton::ton_core::ton_methods]
impl CoffeeConstantProductPoolContract {
    pub async fn get_jetton_data(&self) -> TonResult<CoffeeConstantProductPoolGetJettonDataResult>;
    /// Packs a standard internal owner, including internal zero; results use native TonAddress conversion.
    pub async fn get_wallet_address(&self, owner_address: &TonAddress) -> TonResult<TonAddress> {
        let mut cell = TonCell::builder();
        InternalAddress.write(&mut cell, owner_address)?;
        let mut stack = TVMStack::default();
        stack.push_cell_slice(cell.build()?);
        self.emulate_get_method("get_wallet_address", &stack).await
    }

    pub async fn get_pool_data(&self) -> TonResult<CoffeeConstantProductPoolGetPoolDataResult>;
    pub async fn estimate_swap_amount(
        &self,
        asset: &CoffeePoolAsset,
        amount: &I512,
    ) -> TonResult<CoffeeConstantProductPoolEstimateSwapAmountResult>;

    pub async fn estimate_liquidity_withdraw_amount(
        &self,
        lp_amount: &I512,
    ) -> TonResult<CoffeeConstantProductPoolEstimateLiquidityWithdrawAmountResult>;

    pub async fn estimate_liquidity_deposit_amount(
        &self,
        amount1: &I512,
        amount2: &I512,
    ) -> TonResult<CoffeeConstantProductPoolEstimateLiquidityDepositAmountResult>;
}
pub mod method_results;
pub mod msgs;
pub mod storage;

/// Exit codes declared by the pinned interface; provider errors retain their type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CoffeeConstantProductPoolErrors {
    UnknownOpcode = 6,
    NotEnoughGas = 40,
    Unexpected = 250,
    WrongSender = 252,
    NotAFactory = 253,
    WrongChain = 255,
    WrongAssets = 256,
    WrongSignature = 258,
    AmmFailed = 262,
    InvalidAmmSettings = 263,
    InvalidLiquidity = 264,
    InvalidFee = 265,
    InvalidCondition = 266,
    InvalidAssetType = 267,
    DeadlineExceeded = 268,
    SlippageTolerated = 269,
    PoolAlreadyExists = 270,
    ReservesRatioFailed = 271,
    AlreadyActive = 272,
    NotActive = 273,
}
