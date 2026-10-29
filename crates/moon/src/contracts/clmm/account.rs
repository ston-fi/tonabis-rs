//! Moon clmm account contract API; storage and structured results live in child modules.
use crate::contracts::clmm::account::storage::MoonClmmLpAccountStorage;
ton::ton_contract!(MoonClmmAccountContract<MoonClmmLpAccountStorage>);

pub mod storage;
