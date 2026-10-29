//! CPMM position API for captured library dd82f24d... and kit 0.0.4 getters.
use self::{
    method_results::{DedustV2CpmmPositionAvailableFees, DedustV2CpmmPositionDataResult},
    storage::DedustV2CpmmPositionStorage,
};
use fastnum::I512;
use ton::{contracts::TonContract, errors::TonResult, ton_contract, ton_core::ton_methods};

ton_contract!(DedustV2CpmmPositionContract<DedustV2CpmmPositionStorage>);

#[ton_methods]
impl DedustV2CpmmPositionContract {
    pub async fn get_position_data(&self) -> TonResult<DedustV2CpmmPositionDataResult>;
    pub async fn get_available_fees(
        &self,
        x_checkpoint: I512,
        y_checkpoint: I512,
    ) -> TonResult<DedustV2CpmmPositionAvailableFees>;
}

pub mod method_results;
pub mod msgs;

pub mod storage;
