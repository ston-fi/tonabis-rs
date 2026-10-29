//! DedustUranusMemeWalletV2 contract API. Data records live in [`storage`].
use self::{method_results::DedustUranusMemeWalletV2GetWalletDataResult, storage::DedustUranusMemeWalletV2Storage};
use ton::{contracts::TonContract, errors::TonResult, ton_contract, ton_core::ton_methods};

pub mod method_results;
pub mod storage;
ton_contract!(DedustUranusMemeWalletV2Contract<DedustUranusMemeWalletV2Storage>);

pub mod msgs;

#[ton_methods]
impl DedustUranusMemeWalletV2Contract {
    pub async fn get_wallet_data(&self) -> TonResult<DedustUranusMemeWalletV2GetWalletDataResult>;
}
