//! DedustUranusMemeV2 contract API. Data records live in [`storage`].
use self::{
    method_results::{DedustUranusMemeV2GetJettonDataResult, DedustUranusMemeV2GetMemeDataResult},
    storage::DedustUranusMemeV2Storage,
};
use ton::{
    block_tlb::TVMStack,
    contracts::TonContract,
    errors::TonResult,
    ton_contract,
    ton_core::{ton_methods, types::TonAddress},
};

pub mod method_results;
pub mod storage;
ton_contract!(DedustUranusMemeV2Contract<DedustUranusMemeV2Storage>);

pub mod msgs;

#[ton_methods]
impl DedustUranusMemeV2Contract {
    pub async fn get_meme_data(&self) -> TonResult<DedustUranusMemeV2GetMemeDataResult>;

    pub async fn get_wallet_address(&self, owner_address: TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(&owner_address)?);
        self.emulate_get_method(103289i32, &stack).await
    }

    pub async fn get_jetton_data(&self) -> TonResult<DedustUranusMemeV2GetJettonDataResult>;
}
