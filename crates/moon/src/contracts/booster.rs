//! Moon booster contract API; storage and structured results live in child modules.
use crate::contracts::booster::storage::MoonBoosterStorage;
use ton::block_tlb::TVMStack;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::ton_methods;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::MsgAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
ton::ton_contract!(MoonBoosterContract<MoonBoosterStorage>);
#[ton_methods]
impl MoonBoosterContract {
    pub async fn get_booster(&self) -> TonResult<TonCell>;
    pub async fn get_status(&self) -> TonResult<u8>;
    pub async fn get_boost_data(&self) -> TonResult<TonCell>;
    pub async fn get_pool(&self) -> TonResult<Option<TonAddress>> {
        let stack = TVMStack::default();
        let value: MsgAddress = self.emulate_get_method("get_pool", &stack).await?;
        Ok(NullableInternalAddress::from_address(value)?)
    }
}

pub mod msgs;
pub mod storage;
