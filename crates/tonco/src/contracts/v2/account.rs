//! V2 user-account API. Orders and position holdings use the shared V2 reforge records.
use crate::contracts::v2::account::method_results::ToncoAccountV2GetAccountDataResult;
use crate::contracts::v2::account::storage::ToncoAccountV2Storage;
pub mod method_results;
pub mod msgs;
pub mod storage;

use ton::ton_core::ton_methods;
use ton::{contracts::TonContract, errors::TonResult, ton_contract};

ton_contract!(ToncoAccountV2Contract<ToncoAccountV2Storage>);

#[ton_methods]
impl ToncoAccountV2Contract {
    pub async fn get_account_data(&self) -> TonResult<ToncoAccountV2GetAccountDataResult>;
}
