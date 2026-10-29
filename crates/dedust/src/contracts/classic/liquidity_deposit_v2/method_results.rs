//! Getter result records and protocol-specific stack decoding.

use crate::contracts::classic::asset::DedustClassicAsset;
use ton::ton_core::FromTVMStack;
use ton::ton_core::types::Coins;

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustLiquidityDepositV2GetPoolParamsResult {
    pub is_stable: bool,
    pub asset0: DedustClassicAsset,
    pub asset1: DedustClassicAsset,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustLiquidityDepositV2GetTargetBalancesResult {
    pub asset0_target_balance: Coins,
    pub asset1_target_balance: Coins,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustLiquidityDepositV2GetBalancesResult {
    pub asset0_balance: Coins,
    pub asset1_balance: Coins,
}
