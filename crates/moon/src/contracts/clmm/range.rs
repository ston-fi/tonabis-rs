//! Moon clmm range contract API; storage and structured results live in child modules.
use crate::contracts::clmm::range::storage::MoonClmmRangeStorage;
use ton::block_tlb::TVMStack;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::ton_methods;
use ton::ton_core::types::TonAddress;
ton::ton_contract!(MoonClmmRangeContract<MoonClmmRangeStorage>);
#[ton_methods]
impl MoonClmmRangeContract {
    #[ton_method(name = "getRange")]
    pub async fn get_range(&self) -> TonResult<TonCell>;

    /// Captured selector 112561 derives an account from one owner slice.
    /// Its source name is not available; the numerical selector is proven.
    pub async fn get_account_address(&self, owner: &TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(owner)?);
        self.emulate_get_method(112561, &stack).await
    }
}

pub mod storage;
