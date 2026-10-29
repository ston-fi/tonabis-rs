//! CoffeeInit wrapper; callers own the client and code provenance.
use crate::contracts::init::storage::CoffeeInitStorage;
ton::ton_contract! { CoffeeInitContract<CoffeeInitStorage> }
pub mod msgs;
pub mod storage;

/// Exit codes declared by the pinned interface; provider errors retain their type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CoffeeInitErrors {
    UnknownOpcode = 6,
    NotAFactory = 253,
}
