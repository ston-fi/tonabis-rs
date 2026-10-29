//! StonksStonFactoryContract API. Providers and transaction context are caller-owned.
use crate::contracts::ston_factory::method_results::StonksStonFactoryGetFactoryDataResult;
use crate::contracts::ston_factory::storage::StonksStonFactoryStorage;
use ton::ton_core::ton_methods;
use ton::{contracts::TonContract, errors::TonResult};
ton::ton_contract!(StonksStonFactoryContract<StonksStonFactoryStorage>);
#[ton_methods]
impl StonksStonFactoryContract {
    /// Read the complete four-field factory configuration.
    pub async fn get_factory_data(&self) -> TonResult<StonksStonFactoryGetFactoryDataResult>;
}
pub mod method_results;
pub mod msgs;
pub mod storage;
