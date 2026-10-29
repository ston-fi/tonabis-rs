//! DedustUranusMemeWalletV3 contract API. Data records live in [`storage`].
use self::{method_results::DedustUranusMemeWalletV3GetWalletDataResult, storage::DedustUranusMemeWalletV3Storage};
use ton::{contracts::TonContract, errors::TonResult, ton_contract, ton_core::ton_methods};

pub mod method_results;
pub mod storage;
ton_contract!(DedustUranusMemeWalletV3Contract<DedustUranusMemeWalletV3Storage>);

pub mod msgs;

#[ton_methods]
impl DedustUranusMemeWalletV3Contract {
    pub async fn get_wallet_data(&self) -> TonResult<DedustUranusMemeWalletV3GetWalletDataResult>;
}
