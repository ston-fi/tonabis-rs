//! JettonWalletCoffeeLp wrapper; callers own the client and code provenance.
use crate::contracts::lp_wallet::method_results::CoffeeLpWalletGetWalletDataResult;
use crate::contracts::lp_wallet::storage::CoffeeLpWalletStorage;
use ton::contracts::TonContract;
use ton::errors::TonResult;
ton::ton_contract! { CoffeeLpWalletContract<CoffeeLpWalletStorage> }
#[ton::ton_core::ton_methods]
impl CoffeeLpWalletContract {
    pub async fn get_wallet_data(&self) -> TonResult<CoffeeLpWalletGetWalletDataResult>;
}
pub mod method_results;
pub mod msgs;
pub mod storage;

/// Exit codes declared by the pinned interface; provider errors retain their type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CoffeeLpWalletErrors {
    UnknownOpcode = 6,
    WrongChain = 255,
    UnauthorizedOwner = 705,
    InsufficientJettonBalance = 706,
    InvalidSenderOrBurnFunding = 707,
    MissingForwardPayload = 708,
    TransferFundingOrBounce = 709,
}
