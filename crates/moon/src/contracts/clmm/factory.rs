//! Moon clmm factory contract API; storage and structured results live in child modules.
use crate::contracts::clmm::factory::storage::MoonClmmFactoryStorage;
use ton::block_tlb::{TVMNull, TVMStack, TVMStackValue, ToTVMStack};
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::ton_methods;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::MsgAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
ton::ton_contract!(MoonClmmFactoryContract<MoonClmmFactoryStorage>);
#[ton_methods]
impl MoonClmmFactoryContract {
    /// Native masters are TVM nulls, distinct from an addr_none slice argument.
    pub async fn get_pool_address(
        &self,
        x_master: Option<TonAddress>,
        y_master: Option<TonAddress>,
        bin_step: i64,
    ) -> TonResult<Option<TonAddress>> {
        let mut stack = TVMStack::default();
        for value in [x_master, y_master] {
            if let Some(value) = value {
                let mut cell = TonCell::builder();
                InternalAddress.write(&mut cell, &value)?;
                stack.push_cell_slice(cell.build()?);
            } else {
                stack.push(TVMStackValue::Null(TVMNull));
            }
        }
        bin_step.to_stack(&mut stack)?;
        let value: MsgAddress = self.emulate_get_method("getPoolAddress", &stack).await?;
        Ok(tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(value)?)
    }
}

pub mod storage;
