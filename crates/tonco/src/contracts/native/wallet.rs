//! Distinct native-proxy wallet generations sharing storage. V2 adds jetton-transfer dispatch.
use crate::contracts::native::wallet::method_results::ToncoNativeWalletGetWalletDataResult;
use crate::contracts::native::wallet::storage::ToncoNativeWalletStorage;
pub mod method_results;
pub mod msgs;
pub mod storage;

use ton::ton_core::ton_methods;
use ton::{contracts::TonContract, errors::TonResult, ton_contract};

ton_contract!(ToncoNativeWalletV1Contract<ToncoNativeWalletStorage>);

ton_contract!(ToncoNativeWalletV2Contract<ToncoNativeWalletStorage>);

#[ton_methods]
impl ToncoNativeWalletV1Contract {
    pub async fn get_wallet_data(&self) -> TonResult<ToncoNativeWalletGetWalletDataResult>;
}

#[ton_methods]
impl ToncoNativeWalletV2Contract {
    pub async fn get_wallet_data(&self) -> TonResult<ToncoNativeWalletGetWalletDataResult>;
}
