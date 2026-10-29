//! V2 factory API, referenced whitelist permissions and exact standard-address key construction.
use crate::contracts::v2::factory::method_results::ToncoFactoryV2GetPoolFactoryDataResult;
use crate::contracts::v2::factory::storage::ToncoFactoryV2Storage;
use ton::block_tlb::TVMStack;
use ton::ton_core::cell::TonHash;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::TonAddress;
pub mod method_results;
pub mod msgs;
pub mod storage;

use fastnum::U256;
use ton::ton_core::cell::TonCell;
use ton::ton_core::ton_methods;
use ton::{contracts::TonContract, errors::TonResult, ton_contract};

ton_contract!(ToncoFactoryV2Contract<ToncoFactoryV2Storage>);

#[ton_methods]
impl ToncoFactoryV2Contract {
    #[ton_method(name = "getPoolFactoryData")]
    pub async fn get_pool_factory_data(&self) -> TonResult<ToncoFactoryV2GetPoolFactoryDataResult>;
    pub async fn get_order_address(&self, minter0: &TonAddress, minter1: &TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(minter0)?);
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(minter1)?);
        self.emulate_get_method("getOrderAddress", &stack).await
    }
    #[ton_method(name = "getWhitelist")]
    pub async fn get_whitelist(&self) -> TonResult<Option<TonCell>>;
}

/// Exact SDK `addr2hash`: first 255 bits after the three-bit standard-address tag.
/// This includes workchain bits; it is not a hash of the address text.
pub fn whitelist_key(address: &TonAddress) -> TonCoreResult<TonHash> {
    let cell = tonabis_core::address::internal_address_cell(address)?;
    let mut p = cell.parser();
    let _: u8 = p.read_num(3)?;
    let key: U256 = p.read_num(255)?;
    TonHash::from_u256(&key)
}
