//! Shared TONCO native-proxy minter code. Deploy-wallet uses selector 0x4f5f4313.
use crate::contracts::native::minter::method_results::ToncoNativeMinterGetJettonDataResult;
use crate::contracts::native::minter::method_results::ToncoNativeMinterGetVersionResult;
use crate::contracts::native::minter::storage::ToncoNativeMinterStorage;
use ton::block_tlb::TVMStack;
use ton::ton_core::types::TonAddress;
pub mod method_results;
pub mod msgs;
pub mod storage;

use ton::ton_core::ton_methods;
use ton::{contracts::TonContract, errors::TonResult, ton_contract};

ton_contract!(ToncoNativeMinterContract<ToncoNativeMinterStorage>);

#[ton_methods]
impl ToncoNativeMinterContract {
    pub async fn get_jetton_data(&self) -> TonResult<ToncoNativeMinterGetJettonDataResult>;
    pub async fn get_wallet_address(&self, owner: &TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(owner)?);
        self.emulate_get_method("get_wallet_address", &stack).await
    }
    pub async fn get_version(&self) -> TonResult<ToncoNativeMinterGetVersionResult>;
}
