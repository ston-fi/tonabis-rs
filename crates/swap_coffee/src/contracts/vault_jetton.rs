//! CoffeeVaultJetton wrapper; callers own the client and code provenance.
use crate::contracts::vault_jetton::storage::CoffeeVaultJettonActiveStorage;
use crate::contracts::vault_jetton::storage::CoffeeVaultJettonInactiveStorage;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
ton::ton_contract! { CoffeeVaultJettonContract<CoffeeVaultJettonActiveStorage> }
#[ton::ton_core::ton_methods]
impl CoffeeVaultJettonContract {
    pub async fn get_asset(&self) -> TonResult<TonCell>;

    pub async fn is_active(&self) -> TonResult<bool>;
}
ton::ton_contract! { CoffeeVaultJettonDeploymentContract<CoffeeVaultJettonInactiveStorage> }
#[ton::ton_core::ton_methods]
impl CoffeeVaultJettonDeploymentContract {
    pub async fn get_asset(&self) -> TonResult<TonCell>;

    pub async fn is_active(&self) -> TonResult<bool>;
}
pub mod msgs;
pub mod storage;

/// Exit codes declared by the pinned interface; provider errors retain their type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CoffeeVaultJettonErrors {
    UnknownOpcode = 6,
    NotEnoughGas = 40,
    Unexpected = 250,
    WrongSender = 252,
    NotAFactory = 253,
    WrongChain = 255,
    SameAssets = 257,
    WrongSignature = 258,
    InvalidBalance = 259,
    InvalidJettonMaster = 260,
    InvalidAssetType = 267,
    AlreadyActive = 272,
    NotActive = 273,
}
