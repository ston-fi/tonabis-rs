use crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult;
use crate::contracts::factory::method_results::CoffeeFactoryGetCodeResult;
use crate::contracts::factory::msgs::CoffeeFactoryAsset;
use crate::contracts::factory::storage::CoffeeFactoryStorage;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::MsgAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
// CoffeeFactory wrapper; callers own the client and code provenance.
use fastnum::I512;
use ton::block_tlb::{TVMStack, ToTVMStack};
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
ton::ton_contract! { CoffeeFactoryContract<CoffeeFactoryStorage> }
#[ton::ton_core::ton_methods]
impl CoffeeFactoryContract {
    pub async fn get_vault_address(&self, asset: &CoffeeFactoryAsset) -> TonResult<CoffeeFactoryAddressWithHashResult>;
    pub async fn get_pool_address(
        &self,
        asset1: &CoffeeFactoryAsset,
        asset2: &CoffeeFactoryAsset,
        amm: &I512,
        amm_settings: &TonCell,
    ) -> TonResult<CoffeeFactoryAddressWithHashResult>;
    pub async fn get_pool_address_no_settings(
        &self,
        asset1: &CoffeeFactoryAsset,
        asset2: &CoffeeFactoryAsset,
        amm: &I512,
    ) -> TonResult<CoffeeFactoryAddressWithHashResult>;
    /// Owners are standard internal addresses, including literal zero.
    pub async fn get_pool_creator_address(
        &self,
        owner: &TonAddress,
        asset1: &CoffeeFactoryAsset,
        asset2: &CoffeeFactoryAsset,
        amm: &I512,
        amm_settings: &TonCell,
    ) -> TonResult<CoffeeFactoryAddressWithHashResult> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(owner)?);
        asset1.to_stack(&mut stack)?;
        asset2.to_stack(&mut stack)?;
        amm.to_stack(&mut stack)?;
        amm_settings.to_stack(&mut stack)?;
        self.emulate_get_method("get_pool_creator_address", &stack).await
    }
    pub async fn get_pool_creator_address_no_settings(
        &self,
        owner: &TonAddress,
        asset1: &CoffeeFactoryAsset,
        asset2: &CoffeeFactoryAsset,
        amm: &I512,
    ) -> TonResult<CoffeeFactoryAddressWithHashResult> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(owner)?);
        asset1.to_stack(&mut stack)?;
        asset2.to_stack(&mut stack)?;
        amm.to_stack(&mut stack)?;
        self.emulate_get_method("get_pool_creator_address_no_settings", &stack).await
    }
    pub async fn get_liquidity_depository_address(
        &self,
        owner: &TonAddress,
        asset1: &CoffeeFactoryAsset,
        asset2: &CoffeeFactoryAsset,
        amm: &I512,
        amm_settings: &TonCell,
    ) -> TonResult<CoffeeFactoryAddressWithHashResult> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(owner)?);
        asset1.to_stack(&mut stack)?;
        asset2.to_stack(&mut stack)?;
        amm.to_stack(&mut stack)?;
        amm_settings.to_stack(&mut stack)?;
        self.emulate_get_method("get_liquidity_depository_address", &stack).await
    }
    pub async fn get_liquidity_depository_address_no_settings(
        &self,
        owner: &TonAddress,
        asset1: &CoffeeFactoryAsset,
        asset2: &CoffeeFactoryAsset,
        amm: &I512,
    ) -> TonResult<CoffeeFactoryAddressWithHashResult> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(owner)?);
        asset1.to_stack(&mut stack)?;
        asset2.to_stack(&mut stack)?;
        amm.to_stack(&mut stack)?;
        self.emulate_get_method("get_liquidity_depository_address_no_settings", &stack)
            .await
    }
    pub async fn get_admin_address(&self) -> TonResult<Option<TonAddress>> {
        let stack = TVMStack::default();
        let value: MsgAddress = self.emulate_get_method("get_admin_address", &stack).await?;
        NullableInternalAddress::from_address(value).map_err(Into::into)
    }

    pub async fn get_code(&self) -> TonResult<CoffeeFactoryGetCodeResult>;
}
pub mod method_results;
pub mod msgs;
pub mod storage;

/// Exit codes declared by the pinned interface; provider errors retain their type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CoffeeFactoryErrors {
    UnknownOpcode = 6,
    NotEnoughGas = 40,
    Unexpected = 250,
    WrongSender = 252,
    NotAnAdmin = 254,
    WrongChain = 255,
    SameAssets = 257,
    WrongSignature = 258,
    UnknownAmm = 261,
    InvalidAssetType = 267,
}
