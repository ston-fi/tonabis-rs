//! CoffeeVaultNative wrapper; callers own the client and code provenance.
use crate::contracts::vault_native::storage::CoffeeVaultNativeStorage;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
ton::ton_contract! { CoffeeVaultNativeContract<CoffeeVaultNativeStorage> }
#[ton::ton_core::ton_methods]
impl CoffeeVaultNativeContract {
    pub async fn get_asset(&self) -> TonResult<TonCell>;

    pub async fn is_active(&self) -> TonResult<bool>;
}
pub mod msgs;
pub mod storage;

/// Exit codes declared by the pinned interface; provider errors retain their type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CoffeeVaultNativeErrors {
    UnknownOpcode = 6,
    NotEnoughGas = 40,
    Unexpected = 250,
    NotAFactory = 253,
    WrongChain = 255,
    SameAssets = 257,
    WrongSignature = 258,
    InvalidAssetType = 267,
}
