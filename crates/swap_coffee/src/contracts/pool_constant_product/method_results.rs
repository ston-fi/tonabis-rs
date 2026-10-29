//! Structured getter results.
use fastnum::I512;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq, ton::ton_core::FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct CoffeeConstantProductPoolEstimateSwapAmountResult {
    pub input_amount: I512,
    pub output_amount: I512,
}

/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq, ton::ton_core::FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct CoffeeConstantProductPoolEstimateLiquidityWithdrawAmountResult {
    pub amount1: I512,
    pub amount2: I512,
}

/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq, ton::ton_core::FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct CoffeeConstantProductPoolGetJettonDataResult {
    pub total_supply: I512,
    pub mintable: bool,
    #[from_tvm_stack(adapter = InternalAddress)]
    pub admin_address: TonAddress,
    pub jetton_content: TonCell,
    pub jetton_wallet_code: TonCell,
}

/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq, ton::ton_core::FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct CoffeeConstantProductPoolEstimateLiquidityDepositAmountResult {
    pub amount1_used: I512,
    pub amount2_used: I512,
    pub lp_amount: I512,
    pub lp_to_lock: I512,
}

/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq, ton::ton_core::FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct CoffeeConstantProductPoolGetPoolDataResult {
    pub pool_version: I512,
    pub asset1: TonCell,
    pub asset2: TonCell,
    pub amm: I512,
    pub amm_settings: Option<TonCell>,
    pub active: bool,
    pub reserve1: I512,
    pub reserve2: I512,
    pub total_supply: I512,
    pub protocol_fee: I512,
    pub lp_fee: I512,
}
