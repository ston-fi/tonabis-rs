//! Moon cpmm factory contract API; storage and structured results live in child modules.
use crate::contracts::cpmm::factory::storage::MoonCpmmFactoryStorage;
ton::ton_contract!(MoonCpmmFactoryContract<MoonCpmmFactoryStorage>);

pub mod storage;
