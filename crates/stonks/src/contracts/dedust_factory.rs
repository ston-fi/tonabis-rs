//! StonksDedustFactoryContract API. Providers and transaction context are caller-owned.
use crate::contracts::dedust_factory::method_results::StonksDedustFactoryGetFactoryDataResult;
use crate::contracts::dedust_factory::storage::StonksDedustFactoryStorage;
use ton::ton_core::ton_methods;
use ton::{contracts::TonContract, errors::TonResult};
ton::ton_contract!(StonksDedustFactoryContract<StonksDedustFactoryStorage>);
#[ton_methods]
impl StonksDedustFactoryContract {
    /// Read the complete four-field factory configuration.
    pub async fn get_factory_data(&self) -> TonResult<StonksDedustFactoryGetFactoryDataResult>;
}
pub mod method_results;
pub mod msgs;
pub mod storage;
