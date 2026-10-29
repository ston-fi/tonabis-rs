//! Coffee storage. See tools/reference/swap_coffee/README.md.
use crate::contracts::pool_constant_product::storage::{CoffeePoolAsset, CoffeePoolVaultProofBody};
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
pub struct CoffeeCurveFiStablePoolStorage {
    pub init_code: TLBRef<TonCell>,
    pub init_data: TLBRef<CoffeeCurveFiStablePoolProof>,
    pub is_active: bool,
    pub reserve1: TLBCoins,
    pub reserve2: TLBCoins,
    pub total_supply: TLBCoins,
    pub protocol_fee: u16,
    pub lp_fee: u16,
    pub protocol_fee_collected_asset1: TLBCoins,
    pub protocol_fee_collected_asset2: TLBCoins,
    pub lp_wallet_code: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeCurveFiStablePoolAmmSettings {
    pub amplification: u16,
    pub asset1_weight: TLBCoins,
    pub asset2_weight: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeCurveFiStablePoolDoubleAssetParams {
    pub asset1: CoffeePoolAsset,
    pub asset2: CoffeePoolAsset,
    #[tlb(bits_len = 3)]
    pub amm: u8,
    pub amm_settings: Option<TLBRef<CoffeeCurveFiStablePoolAmmSettings>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeeCurveFiStablePoolSourcePoolProofBody {
    pub asset1: CoffeePoolAsset,
    pub asset2: CoffeePoolAsset,
    #[tlb(bits_len = 3)]
    pub amm: u8,
    pub amm_settings: Option<TLBRef<CoffeeCurveFiStablePoolAmmSettings>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeeCurveFiStablePoolLiquidityDepositoryProofBody {
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    pub params: TLBRef<CoffeeCurveFiStablePoolDoubleAssetParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b11, bits_len = 2)]
pub struct CoffeeCurveFiStablePoolCreatorProofBody {
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    pub params: TLBRef<CoffeeCurveFiStablePoolDoubleAssetParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeCurveFiStablePoolProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    pub body: CoffeeCurveFiStablePoolProofBody,
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeCurveFiStablePoolProofBody {
    CoffeePoolVaultProofBody(Box<CoffeePoolVaultProofBody>),
    CoffeePoolPoolProofBody(Box<CoffeeCurveFiStablePoolSourcePoolProofBody>),
    CoffeePoolLiquidityDepositoryProofBody(Box<CoffeeCurveFiStablePoolLiquidityDepositoryProofBody>),
    CoffeePoolCreatorProofBody(Box<CoffeeCurveFiStablePoolCreatorProofBody>),
}

// Caller-code identity construction; parsing is not authentication.
impl CoffeeCurveFiStablePoolProof {
    /// Exact code/data StateInit; caller supplies the factory's Init code.
    pub fn state_init(&self, init_code: &TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(init_code.clone(), self.to_cell()?))
    }
    /// Basechain identity address; this does not prove factory admission.
    pub fn derive_basechain_address(&self, init_code: &TonCell) -> TonCoreResult<TonAddress> {
        self.state_init(init_code)?.derive_address(0)
    }
}
