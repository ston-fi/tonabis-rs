//! DedustVaultJettonV1 contract API. Data records live in [`storage`].
use self::storage::DedustVaultJettonV1Storage;
use crate::contracts::classic::asset::DedustClassicAsset;
use fastnum::I512;
use ton::ton_core::ton_methods;
use ton::{block_tlb::TVMStack, contracts::TonContract, errors::TonResult, ton_contract, ton_core::types::TonAddress};

pub mod storage;
ton_contract!(DedustVaultJettonV1Contract<DedustVaultJettonV1Storage>);

pub mod msgs;

#[ton_methods]
impl DedustVaultJettonV1Contract {
    pub async fn get_version(&self) -> TonResult<u16>;

    pub async fn is_ready(&self) -> TonResult<bool>;

    pub async fn get_factory_addr(&self) -> TonResult<TonAddress>;

    pub async fn get_wallet_addr(&self) -> TonResult<Option<TonAddress>> {
        let mut stack = self.emulate_get_method::<_, TVMStack>(69014i32, &TVMStack::EMPTY).await?;
        crate::contracts::utils::stack_nullable_address(&mut stack)
    }

    pub async fn get_asset(&self) -> TonResult<DedustClassicAsset>;
    /// Captured numeric gas-estimation export with one integer argument. V2 depends on the emulation blockchain configuration; no fixed gas policy is inferred.
    #[ton_method(id = 109505)]
    pub async fn estimate_gas_one_arg(&self, argument0: I512) -> TonResult<I512>;
    /// Captured numeric gas-estimation export with two integer arguments. Source order and signed result are preserved; application parameter labels remain unproven.
    #[ton_method(id = 117300)]
    pub async fn estimate_gas_two_args(&self, argument0: I512, argument1: I512) -> TonResult<I512>;
}
