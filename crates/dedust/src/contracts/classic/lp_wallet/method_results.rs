//! Getter result records with native stack conversion.
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::Coins;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustClassicLpWalletWalletData {
    pub balance: Coins,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub owner: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub pool: Option<TonAddress>,
    pub wallet_code: TonCell,
}
