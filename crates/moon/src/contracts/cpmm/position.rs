//! Moon cpmm position contract API; storage and structured results live in child modules.
use crate::contracts::cpmm::position::storage::MoonCpmmLpPositionStorage;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::ton_methods;
ton::ton_contract!(MoonCpmmPositionContract<MoonCpmmLpPositionStorage>);
#[ton_methods]
impl MoonCpmmPositionContract {
    #[ton_method(name = "getLocked")]
    pub async fn get_locked(&self) -> TonResult<bool>;
    #[ton_method(name = "getPosition")]
    pub async fn get_position(&self) -> TonResult<TonCell>;
}

pub mod storage;
