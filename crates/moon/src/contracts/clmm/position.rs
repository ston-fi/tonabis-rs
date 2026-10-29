//! Moon clmm position contract API; storage and structured results live in child modules.
use crate::contracts::clmm::position::storage::MoonClmmLpPositionStorage;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::ton_methods;
ton::ton_contract!(MoonClmmPositionContract<MoonClmmLpPositionStorage>);
#[ton_methods]
impl MoonClmmPositionContract {
    #[ton_method(name = "getPosition")]
    pub async fn get_position(&self) -> TonResult<TonCell>;
}

pub mod storage;
