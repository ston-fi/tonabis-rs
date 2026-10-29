//! CoffeeCrossDexLegacy wrapper; callers own the client and code provenance.
use crate::contracts::cross_dex_legacy::storage::CoffeeCrossDexLegacyDeploymentStorage;
use crate::contracts::cross_dex_legacy::storage::CoffeeCrossDexLegacyStorage;
ton::ton_contract! { CoffeeCrossDexLegacyContract<CoffeeCrossDexLegacyStorage> }
ton::ton_contract! { CoffeeCrossDexLegacyDeploymentContract< CoffeeCrossDexLegacyDeploymentStorage> }
pub mod msgs;
pub mod storage;

/// Exit codes declared by the pinned interface; provider errors retain their type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CoffeeCrossDexLegacyErrors {
    InvalidPayloadToResend = 100,
    DedustPayloadIsEmpty = 101,
    PtonPayloadIsEmpty = 102,
    NotAnAdmin = 200,
    WrongOperation = 65535,
}
