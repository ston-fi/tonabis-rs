//! DedustOperatorV2 contract API. Data records live in [`storage`].
use self::{method_results::DedustOperatorV2GetOperatorDataResult, storage::DedustOperatorV2Storage};
use ton::ton_core::ton_methods;
use ton::{contracts::TonContract, errors::TonResult, ton_contract};

pub mod method_results;
pub mod storage;
ton_contract!(DedustOperatorV2Contract<DedustOperatorV2Storage>);

pub mod msgs;

#[ton_methods]
impl DedustOperatorV2Contract {
    pub async fn get_operator_data(&self) -> TonResult<DedustOperatorV2GetOperatorDataResult>;
    pub async fn get_version(&self) -> TonResult<u16>;
}
