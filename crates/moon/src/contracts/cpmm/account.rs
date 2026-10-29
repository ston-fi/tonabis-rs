//! Moon cpmm account contract API; storage and structured results live in child modules.
use crate::contracts::cpmm::account::storage::MoonCpmmLpAccountStorage;
ton::ton_contract!(MoonCpmmAccountContract<MoonCpmmLpAccountStorage>);

pub mod storage;
