//! CoffeeMevProtectorLegacy wrapper; callers own the client and code provenance.
use crate::contracts::mev_protector_legacy::storage::CoffeeMevProtectorLegacyStorage;
use fastnum::U256;
use ton::contracts::TonContract;
use ton::errors::TonResult;
ton::ton_contract! { CoffeeMevProtectorLegacyContract<CoffeeMevProtectorLegacyStorage> }
#[ton::ton_core::ton_methods]
impl CoffeeMevProtectorLegacyContract {
    pub async fn get_public_key(&self) -> TonResult<U256>;
    #[ton_method(name = "processed?")]
    pub async fn processed_query(&self, query_id: &u64) -> TonResult<bool>;
}
pub mod msgs;
pub mod storage;

/// Exit codes declared by the pinned interface; provider errors retain their type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CoffeeMevProtectorLegacyErrors {
    QueryAlreadyExecuted = 32,
    InvalidSubwalletId = 34,
    InvalidQueryIdOrSignature = 35,
    EmptyTxToResend = 100,
    DedustPayloadIsEmpty = 101,
    InvalidJettonForwardPayload = 200,
    InvalidDedustPayload = 201,
    NotAnAdmin = 300,
    WrongOperation = 65535,
}
