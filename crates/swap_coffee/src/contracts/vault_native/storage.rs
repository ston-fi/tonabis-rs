//! Coffee storage. See tools/reference/swap_coffee/README.md.
use crate::contracts::wire::TrailingSelector;
use fastnum::U256;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::{
    block_tlb::StateInit,
    ton_core::{
        TLB,
        cell::TonCell,
        errors::TonCoreResult,
        traits::tlb::TLB,
        types::{
            TonAddress,
            tlb_core::{TLBRef, adapters::ConstLen},
        },
    },
};
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeVaultNativeStorage {
    pub init_code: TLBRef<TonCell>,
    pub init_data: TLBRef<CoffeeVaultInitData>,
    #[tlb(adapter = "TrailingSelector")]
    pub factory_initialization_marker: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeVaultInitData {
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    #[tlb(bits_len = 2)]
    pub contract_type: u8,
    pub asset: CoffeeAsset,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b00, bits_len = 2)]
pub struct CoffeeNativeAsset {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeeJettonAsset {
    pub chain: u8,
    pub hash: U256,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeeExtraAsset {
    pub id: u32,
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeAsset {
    CoffeeNativeAsset(Box<CoffeeNativeAsset>),
    CoffeeJettonAsset(Box<CoffeeJettonAsset>),
    CoffeeExtraAsset(Box<CoffeeExtraAsset>),
}

// Caller-code identity construction; parsing is not authentication.
impl CoffeeVaultInitData {
    /// Exact code/data StateInit; caller supplies the factory's Init code.
    pub fn state_init(&self, init_code: &TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(init_code.clone(), self.to_cell()?))
    }
    /// Basechain identity address; this does not prove factory admission.
    pub fn derive_basechain_address(&self, init_code: &TonCell) -> TonCoreResult<TonAddress> {
        self.state_init(init_code)?.derive_address(0)
    }
}
