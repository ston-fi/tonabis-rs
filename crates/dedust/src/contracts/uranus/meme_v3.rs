//! DedustUranusMemeV3 contract API. Data records live in [`storage`].
use self::{
    method_results::{
        DedustUranusMemeV3GetBondingCurveDataResult, DedustUranusMemeV3GetJettonDataResult,
        DedustUranusMemeV3GetMemeDataResult, DedustUranusMemeV3GetPartnerDataResult,
    },
    storage::DedustUranusMemeV3Storage,
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
ton_contract!(DedustUranusMemeV3Contract<DedustUranusMemeV3Storage>);

pub mod msgs;

#[ton_methods]
impl DedustUranusMemeV3Contract {
    pub async fn get_meme_data(&self) -> TonResult<DedustUranusMemeV3GetMemeDataResult>;

    pub async fn get_bonding_curve_data(&self) -> TonResult<DedustUranusMemeV3GetBondingCurveDataResult>;

    pub async fn get_partner_data(&self) -> TonResult<DedustUranusMemeV3GetPartnerDataResult>;

    pub async fn get_wallet_address(&self, owner_address: TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(&owner_address)?);
        self.emulate_get_method(103289i32, &stack).await
    }

    pub async fn get_jetton_data(&self) -> TonResult<DedustUranusMemeV3GetJettonDataResult>;
}
