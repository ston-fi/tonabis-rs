//! Getter result records and protocol-specific stack decoding.
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

use crate::contracts::classic::asset::DedustClassicAsset;
use fastnum::I512;
use ton::ton_core::types::Coins;
use ton::ton_core::{FromTVMStack, cell::TonCell};

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustPoolV1GetTradeFeeResult {
    pub trade_fee_numerator: u16,
    pub trade_fee_denominator: u16,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustPoolV1GetReservesResult {
    pub reserve0: Coins,
    pub reserve1: Coins,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustPoolV1GetProtocolFeesResult {
    pub asset0_protocol_fee: Coins,
    pub asset1_protocol_fee: Coins,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustPoolV1GetAssetsResult {
    pub asset0: DedustClassicAsset,
    pub asset1: DedustClassicAsset,
}

/// Estimate output amount and fee for a serialized input asset and amount.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustPoolV1EstimateSwapOutResult {
    pub asset_out: DedustClassicAsset,
    pub amount_out: I512,
    pub trade_fee: I512,
}

/// Estimate accepted deposit amounts and fair LP supply for proposed asset amounts.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustPoolV1EstimateDepositOutResult {
    pub asset0_deposit: I512,
    pub asset1_deposit: I512,
    pub fair_supply: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustPoolV1GetJettonDataResult {
    pub total_supply: Coins,
    pub mintable: bool,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub admin_address: Option<TonAddress>,
    pub jetton_content: TonCell,
    pub jetton_wallet_code: TonCell,
}

/// Captured numeric export for both stored asset decimal counts.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustPoolV1GetDecimalsResult {
    pub decimals_x: u8,
    pub decimals_y: u8,
}
