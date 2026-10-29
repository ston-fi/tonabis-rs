//! Moon clmm pool contract API; storage and structured results live in child modules.
use crate::contracts::clmm::pool::method_results::MoonClmmPoolRangesResult;
use crate::contracts::clmm::pool::storage::MoonClmmPoolStorage;
use ton::block_tlb::TVMStack;
use ton::block_tlb::ToTVMStack;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::ton_methods;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::MsgAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
ton::ton_contract!(MoonClmmPoolContract<MoonClmmPoolStorage>);
#[ton_methods]
impl MoonClmmPoolContract {
    #[ton_method(name = "getPool")]
    pub async fn get_pool(&self) -> TonResult<TonCell>;
    pub async fn get_range_by_num(&self, range_num: i64) -> TonResult<Option<TonAddress>> {
        let mut stack = TVMStack::default();
        range_num.to_stack(&mut stack)?;
        let value: MsgAddress = self.emulate_get_method("getRangeByNum", &stack).await?;
        Ok(NullableInternalAddress::from_address(value)?)
    }
    #[ton_method(name = "getRanges")]
    pub async fn get_ranges(&self) -> TonResult<MoonClmmPoolRangesResult>;
}

pub mod method_results;
pub mod storage;
