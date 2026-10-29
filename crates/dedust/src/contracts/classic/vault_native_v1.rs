//! DedustVaultNativeV1 contract API. Data records live in [`storage`].
use self::storage::DedustVaultNativeV1Storage;
use crate::contracts::classic::asset::DedustClassicAsset;
use fastnum::I512;
use ton::ton_core::ton_methods;
use ton::{contracts::TonContract, errors::TonResult, ton_contract, ton_core::types::TonAddress};

pub mod storage;
ton_contract!(DedustVaultNativeV1Contract<DedustVaultNativeV1Storage>);

pub mod msgs;

#[ton_methods]
impl DedustVaultNativeV1Contract {
    pub async fn get_version(&self) -> TonResult<u16>;

    pub async fn get_factory_addr(&self) -> TonResult<TonAddress>;

    pub async fn get_asset(&self) -> TonResult<DedustClassicAsset>;
    /// Captured numeric gas-estimation export with one integer argument. V2 depends on the emulation blockchain configuration; no fixed gas policy is inferred.
    #[ton_method(id = 109505)]
    pub async fn estimate_gas_one_arg(&self, argument0: I512) -> TonResult<I512>;
    /// Captured numeric gas-estimation export with two integer arguments. Source order and signed result are preserved; application parameter labels remain unproven.
    #[ton_method(id = 117300)]
    pub async fn estimate_gas_two_args(&self, argument0: I512, argument1: I512) -> TonResult<I512>;
}
