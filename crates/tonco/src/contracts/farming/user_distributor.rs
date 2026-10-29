//! Farming user helper API and claim identity storage; decoding does not verify a proof.
use crate::contracts::farming::user_distributor::storage::ToncoUserDistributorStorage;
pub mod storage;

use ton::ton_core::ton_methods;
use ton::{contracts::TonContract, errors::TonResult, ton_contract};

ton_contract!(ToncoUserDistributorContract<ToncoUserDistributorStorage>);

#[ton_methods]
impl ToncoUserDistributorContract {
    pub async fn get_claimed(&self) -> TonResult<bool>;
}
