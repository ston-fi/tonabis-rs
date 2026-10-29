//! Structured getter results with exact TVM stack arity.
use fastnum::I512;
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct MoonLegacyPoolGetReservesResult {
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub asset1_id: Option<TonAddress>,
    pub asset1_reserve: I512,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub asset2_id: Option<TonAddress>,
    pub asset2_reserve: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct MoonLegacyPoolGetAssetsResult {
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub asset1_id: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub asset2_id: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct MoonLegacyPoolGetWalletsResult {
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub asset1_wallet: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub asset2_wallet: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct MoonLegacyPoolGetFeesResult {
    pub swap_fee_bps: I512,
    pub protocol_fee_bps: I512,
    pub referral_fee_bps: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct MoonLegacyPoolGetCollectedFeesResult {
    pub asset1_fees: I512,
    pub asset2_fees: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct MoonLegacyPoolGetEstimatedLpBurnResult {
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub asset1_id: Option<TonAddress>,
    pub asset1_amount: I512,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub asset2_id: Option<TonAddress>,
    pub asset2_amount: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct MoonLegacyPoolGetEstimatedSwapOutResult {
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub output_asset_id: Option<TonAddress>,
    pub output_amount: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct MoonLegacyPoolGetJettonDataResult {
    pub total_supply: I512,
    pub mintable: bool,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub admin_address: Option<TonAddress>,
    pub jetton_content: TonCell,
    pub jetton_wallet_code: TonCell,
}
