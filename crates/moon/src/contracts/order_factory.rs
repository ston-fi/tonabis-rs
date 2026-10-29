//! Moon order_factory contract API; storage and structured results live in child modules.
use crate::contracts::order_factory::storage::MoonOrderFactoryStorage;
use ton::block_tlb::TVMStack;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::ton_methods;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
ton::ton_contract!(MoonOrderFactoryContract<MoonOrderFactoryStorage>);
#[ton_methods]
impl MoonOrderFactoryContract {
    pub async fn get_id(&self) -> TonResult<u64>;
    /// None is the native/absent asset identifier, encoded as an addr_none Slice.
    pub async fn get_pipe(&self, asset: Option<TonAddress>) -> TonResult<TonAddress> {
        let mut builder = TonCell::builder();
        NullableAddress.write(&mut builder, &asset)?;
        let mut stack = TVMStack::default();
        stack.push_cell_slice(builder.build()?);
        self.emulate_get_method("get_pipe", &stack).await
    }
}

pub mod msgs;
pub mod storage;
