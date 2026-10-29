//! DedustFactoryV2 contract API. Data records live in [`storage`].
use self::{method_results::DedustFactoryV2GetOwnershipResult, storage::DedustFactoryV2Storage};
use crate::contracts::classic::asset::DedustClassicAsset;
use crate::contracts::classic::common::DedustClassicOperatorProof;
use crate::contracts::classic::common::method_results::{
    DedustClassicNullableVersionedCode, DedustClassicVersionedCode,
};
use ton::{
    block_tlb::{StateInit, TVMStack, ToTVMStack},
    contracts::TonContract,
    errors::TonResult,
    ton_contract,
    ton_core::{
        cell::TonCell,
        errors::TonCoreError,
        ton_methods,
        traits::tlb::TLB,
        types::{TonAddress, tlb_core::adapters::InternalAddress},
    },
};

pub mod method_results;
pub mod storage;
ton_contract!(DedustFactoryV2Contract<DedustFactoryV2Storage>);

pub mod msgs;

#[ton_methods]
impl DedustFactoryV2Contract {
    /// Blank vault state. Native and Jetton vaults differ by typed asset.
    pub fn vault_state_init(
        factory: &TonAddress,
        blank_code: TonCell,
        asset: &DedustClassicAsset,
    ) -> Result<StateInit, TonCoreError> {
        crate::contracts::utils::classic_initial(factory, blank_code, 1, &asset.to_cell()?)
    }
    /// Canonical pair order is SDLEXCMP; equal assets return a typed error.
    pub fn pool_state_init(
        factory: &TonAddress,
        blank_code: TonCell,
        stable: bool,
        asset_a: &DedustClassicAsset,
        asset_b: &DedustClassicAsset,
    ) -> Result<StateInit, TonCoreError> {
        crate::contracts::utils::classic_initial(
            factory,
            blank_code,
            2,
            &crate::contracts::utils::classic_pair(stable, asset_a, asset_b)?,
        )
    }
    /// One referenced record holds the owner and inline canonical pair identity.
    pub fn liquidity_deposit_state_init(
        factory: &TonAddress,
        blank_code: TonCell,
        owner: &TonAddress,
        stable: bool,
        asset_a: &DedustClassicAsset,
        asset_b: &DedustClassicAsset,
    ) -> Result<StateInit, TonCoreError> {
        let mut params = TonCell::builder();
        InternalAddress.write(&mut params, owner)?;
        params.write_cell(&crate::contracts::utils::classic_pair(stable, asset_a, asset_b)?)?;
        let mut tail = TonCell::builder();
        tail.write_ref(params.build()?)?;
        crate::contracts::utils::classic_initial(factory, blank_code, 3, &tail.build()?)
    }
    pub fn operator_state_init(factory: &TonAddress, blank_code: TonCell, role: u8) -> Result<StateInit, TonCoreError> {
        let proof = DedustClassicOperatorProof {
            factory: *factory,
            role,
        };
        Ok(StateInit::new(blank_code, proof.to_cell()?))
    }

    pub async fn get_ownership(&self) -> TonResult<DedustFactoryV2GetOwnershipResult>;

    pub async fn get_version(&self) -> TonResult<u16>;

    pub async fn get_vault_address(&self, asset: DedustClassicAsset) -> TonResult<TonAddress>;

    pub async fn get_pool_address(
        &self,
        is_stable: bool,
        asset0: DedustClassicAsset,
        asset1: DedustClassicAsset,
    ) -> TonResult<TonAddress>;

    pub async fn get_liquidity_deposit_address(
        &self,
        owner_address: TonAddress,
        is_stable: bool,
        asset0: DedustClassicAsset,
        asset1: DedustClassicAsset,
    ) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(&owner_address)?);
        is_stable.to_stack(&mut stack)?;
        asset0.to_stack(&mut stack)?;
        asset1.to_stack(&mut stack)?;
        self.emulate_get_method(84481i32, &stack).await
    }
    pub async fn get_pool_code(&self) -> TonResult<DedustClassicVersionedCode>;
    pub async fn get_liquidity_deposit_code(&self) -> TonResult<DedustClassicVersionedCode>;
    pub async fn get_vault_code(&self, asset_type: u8) -> TonResult<DedustClassicVersionedCode>;
    /// Optional Operator code/version, selector78248. Absent old configuration
    /// returns version0/null without inventing an installed implementation.
    pub async fn get_operator_code(&self) -> TonResult<DedustClassicNullableVersionedCode>;
    /// Deterministic Operator address for the uint8 role, selector126188.
    pub async fn get_operator_address(&self, role: u8) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        role.to_stack(&mut stack)?;
        let cell: TonCell = self.emulate_get_method("get_operator_address", &stack).await?;
        Ok(InternalAddress.read(&mut cell.parser())?)
    }
}
