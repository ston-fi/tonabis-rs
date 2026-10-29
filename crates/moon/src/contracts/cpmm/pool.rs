//! Moon cpmm pool contract API; storage and structured results live in child modules.
use crate::contracts::cpmm::pool::storage::MoonCpmmPoolStorage;
ton::ton_contract!(MoonCpmmPoolContract<MoonCpmmPoolStorage>);

pub mod storage;
