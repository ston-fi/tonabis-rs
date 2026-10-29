//! CPMM deposit library 2cac3fdd... API and deposit/refund commands.
use self::{method_results::DedustV2CpmmDepositDataResult, storage::DedustV2CpmmDepositStorage};
use ton::{contracts::TonContract, errors::TonResult, ton_contract, ton_core::ton_methods};

ton_contract!(DedustV2CpmmDepositContract<DedustV2CpmmDepositStorage>);

#[ton_methods]
impl DedustV2CpmmDepositContract {
    pub async fn get_deposit_data(&self) -> TonResult<DedustV2CpmmDepositDataResult>;
}

pub mod method_results;
pub mod msgs;

pub mod storage;
