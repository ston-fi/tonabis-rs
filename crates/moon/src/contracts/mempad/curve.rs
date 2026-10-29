//! Moon mempad curve contract API; storage and structured results live in child modules.
use crate::contracts::mempad::curve::method_results::MoonMempadCurveJettonDataResult;
use crate::contracts::mempad::curve::method_results::MoonMempadCurveNullResult;
use crate::contracts::mempad::curve::storage::MoonMempadCurveStorage;
use ton::block_tlb::TVMStack;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::ton_methods;
use ton::ton_core::types::TonAddress;
ton::ton_contract!(MoonMempadCurveContract<MoonMempadCurveStorage>);
#[ton_methods]
impl MoonMempadCurveContract {
    pub async fn get_wallet_address(&self, owner: &TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(owner)?);
        self.emulate_get_method("get_wallet_address", &stack).await
    }
    pub async fn get_jetton_data(&self) -> TonResult<MoonMempadCurveJettonDataResult>;

    /// The captured selector returns exactly one TVM NULL.
    #[ton_method(id = 78683)]
    pub async fn get_null(&self) -> TonResult<MoonMempadCurveNullResult>;
}

pub mod method_results;
pub mod storage;
