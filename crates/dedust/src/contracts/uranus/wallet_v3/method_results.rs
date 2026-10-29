//! Getter result records with native stack conversion.
use ton::ton_core::types::Coins;
use ton::ton_core::{FromTVMStack, cell::TonCell, types::TonAddress};

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustUranusMemeWalletV3GetWalletDataResult {
    pub balance: Coins,
    pub owner_address: TonAddress,
    pub meme_address: TonAddress,
    pub wallet_code: TonCell,
}
