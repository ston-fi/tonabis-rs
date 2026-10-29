//! Getter result records and protocol-specific stack decoding.
use ton::ton_core::FromTVMStack;
use ton::ton_core::types::Coins;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

/// Exact flattened stack returned by library getter 70330 (`get_deposit_data`).
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustV2CpmmDepositDataResult {
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub pool: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub fulfill_destination: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub reject_destination: Option<TonAddress>,
    pub target_x: Coins,
    pub target_y: Coins,
    pub locked_liquidity_share: u16,
    pub minimal_liquidity: Coins,
    pub amount_x: Coins,
    pub amount_y: Coins,
}
