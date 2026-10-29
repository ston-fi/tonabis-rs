//! Structured getter results.
use crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolAmmSettings;
use crate::contracts::wire::stack_optional_ref;
use fastnum::I512;
use ton::block_tlb::{FromTVMStack, TVMStack};
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq, ton::ton_core::FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct CoffeeCurveFiStablePoolEstimateSwapAmountResult {
    pub input_amount: I512,
    pub output_amount: I512,
}

/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq, ton::ton_core::FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct CoffeeCurveFiStablePoolEstimateLiquidityWithdrawAmountResult {
    pub amount1: I512,
    pub amount2: I512,
}

/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq, ton::ton_core::FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct CoffeeCurveFiStablePoolEstimateLiquidityDepositAmountResult {
    pub amount1_used: I512,
    pub amount2_used: I512,
    pub lp_amount: I512,
    pub lp_to_lock: I512,
}

/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq, ton::ton_core::FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct CoffeeCurveFiStablePoolGetJettonDataResult {
    pub total_supply: I512,
    pub mintable: bool,
    #[from_tvm_stack(adapter = InternalAddress)]
    pub admin_address: TonAddress,
    pub jetton_content: TonCell,
    pub jetton_wallet_code: TonCell,
}

/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq)]
pub struct CoffeeCurveFiStablePoolGetPoolDataResult {
    pub pool_version: I512,
    pub asset1: TonCell,
    pub asset2: TonCell,
    pub amm: I512,
    pub amm_settings: Option<TLBRef<CoffeeCurveFiStablePoolAmmSettings>>,
    pub active: bool,
    pub reserve1: I512,
    pub reserve2: I512,
    pub total_supply: I512,
    pub protocol_fee: I512,
    pub lp_fee: I512,
}
impl FromTVMStack for CoffeeCurveFiStablePoolGetPoolDataResult {
    fn from_stack(s: &mut TVMStack) -> TonResult<Self> {
        let lp_fee = <I512>::from_stack(s)?;
        let protocol_fee = <I512>::from_stack(s)?;
        let total_supply = <I512>::from_stack(s)?;
        let reserve2 = <I512>::from_stack(s)?;
        let reserve1 = <I512>::from_stack(s)?;
        let active = <bool>::from_stack(s)?;
        let amm_settings = stack_optional_ref::<CoffeeCurveFiStablePoolAmmSettings>(s)?;
        let amm = <I512>::from_stack(s)?;
        let asset2 = TonCell::from_stack(s)?;
        let asset1 = TonCell::from_stack(s)?;
        let pool_version = <I512>::from_stack(s)?;
        s.ensure_empty()?;
        Ok(Self {
            pool_version,
            asset1,
            asset2,
            amm,
            amm_settings,
            active,
            reserve1,
            reserve2,
            total_supply,
            protocol_fee,
            lp_fee,
        })
    }
}
