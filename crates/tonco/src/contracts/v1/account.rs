//! V1 user-account API, funding callbacks and local storage construction.
use crate::contracts::v1::account::method_results::ToncoAccountV1GetAccountDataResult;
use crate::contracts::v1::account::storage::ToncoAccountV1Storage;
pub mod method_results;
pub mod msgs;
pub mod storage;

use ton::ton_core::ton_methods;
use ton::{contracts::TonContract, errors::TonResult, ton_contract};

// Adapted from ton-blockchain/abis 7dc57067, MIT; see reference provenance.

ton_contract!(ToncoAccountV1Contract<ToncoAccountV1Storage>);

#[ton_methods]
impl ToncoAccountV1Contract {
    pub async fn get_account_data(&self) -> TonResult<ToncoAccountV1GetAccountDataResult>;
}
