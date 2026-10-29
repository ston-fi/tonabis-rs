//! Farming distributor API, root dictionary and caller-code StateInit construction.
use crate::contracts::farming::distributor::storage::ToncoDistributorStorage;
use ton::block_tlb::TVMStack;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
pub mod storage;

use fastnum::I512;
use ton::ton_core::ton_methods;
use ton::ton_core::types::tlb_core::MsgAddress;
use ton::{contracts::TonContract, errors::TonResult, ton_contract};

ton_contract!(ToncoDistributorContract<ToncoDistributorStorage>);

#[ton_methods]
impl ToncoDistributorContract {
    pub async fn get_merkle_root(&self, farming_id: I512) -> TonResult<I512>;

    pub async fn get_admin(&self) -> TonResult<Option<TonAddress>> {
        let stack = TVMStack::default();
        let value: MsgAddress = self.emulate_get_method("get_admin", &stack).await?;
        Ok(NullableInternalAddress::from_address(value)?)
    }

    pub async fn get_user_distributor_address(
        &self,
        proof_hash: I512,
        index: I512,
        farming_id: I512,
        position_id: I512,
    ) -> TonResult<MsgAddress>;
}
