//! Moon mempad factory contract API; storage and structured results live in child modules.
use crate::contracts::mempad::factory::storage::MoonMempadFactoryStorage;
ton::ton_contract!(MoonMempadFactoryContract<MoonMempadFactoryStorage>);

pub mod storage;
