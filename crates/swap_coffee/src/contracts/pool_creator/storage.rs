//! Coffee storage. See tools/reference/swap_coffee/README.md.
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
            tlb_core::{TLBCoins, TLBRef, adapters::ConstLen},
        },
    },
};
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolCreatorStorage {
    pub init_code_cell: TLBRef<TonCell>,
    pub init_data_cell: TLBRef<CoffeePoolCreatorInitData>,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolCreatorInitData {
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    #[tlb(bits_len = 2)]
    pub contract_type: u8,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    pub pool_params: TLBRef<CoffeePoolCreatorInitPoolParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolCreatorInitPoolParams {
    pub asset1: CoffeePoolCreatorAsset,
    pub asset2: CoffeePoolCreatorAsset,
    pub amm: CoffeePoolCreatorAmm,
    pub amm_settings: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b00, bits_len = 2)]
pub struct CoffeePoolCreatorAssetNative {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeePoolCreatorAssetJetton {
    pub chain: u8,
    pub hash: U256,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeePoolCreatorAssetExtra {
    pub id: u32,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b000, bits_len = 3)]
pub struct CoffeePoolCreatorAmmConstantProduct {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b001, bits_len = 3)]
pub struct CoffeePoolCreatorAmmCurveFiStable {}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeePoolCreatorAsset {
    AssetNative(Box<CoffeePoolCreatorAssetNative>),
    AssetJetton(Box<CoffeePoolCreatorAssetJetton>),
    AssetExtra(Box<CoffeePoolCreatorAssetExtra>),
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeePoolCreatorAmm {
    AmmConstantProduct(Box<CoffeePoolCreatorAmmConstantProduct>),
    AmmCurveFiStable(Box<CoffeePoolCreatorAmmCurveFiStable>),
}

// Caller-code identity construction; parsing is not authentication.
impl CoffeePoolCreatorInitData {
    /// Exact code/data StateInit; caller supplies the factory's Init code.
    pub fn state_init(&self, init_code: &TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(init_code.clone(), self.to_cell()?))
    }
    /// Basechain identity address; this does not prove factory admission.
    pub fn derive_basechain_address(&self, init_code: &TonCell) -> TonCoreResult<TonAddress> {
        self.state_init(init_code)?.derive_address(0)
    }
}
