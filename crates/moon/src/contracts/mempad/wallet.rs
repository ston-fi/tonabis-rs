//! Moon mempad wallet contract API; storage and structured results live in child modules.
use crate::contracts::mempad::wallet::method_results::MoonMempadWalletWalletDataResult;
use crate::contracts::mempad::wallet::storage::MoonMempadWalletStorage;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::ton_methods;
ton::ton_contract!(MoonMempadWalletContract<MoonMempadWalletStorage>);
#[ton_methods]
impl MoonMempadWalletContract {
    pub async fn get_wallet_data(&self) -> TonResult<MoonMempadWalletWalletDataResult>;
}

pub mod method_results;
pub mod storage;
