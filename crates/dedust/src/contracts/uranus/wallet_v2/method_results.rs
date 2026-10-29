//! Getter result records with native stack conversion.
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::Coins;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustUranusMemeWalletV2GetWalletDataResult {
    pub balance: Coins,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub owner_address: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub meme_address: Option<TonAddress>,
    pub wallet_code: TonCell,
}
