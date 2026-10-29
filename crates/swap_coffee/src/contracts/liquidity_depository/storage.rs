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
pub struct CoffeeLiquidityDepositoryStorage {
    pub init_code_cell: TLBRef<TonCell>,
    pub init_data_cell: TLBRef<CoffeeLiquidityDepositoryInitData>,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeLiquidityDepositoryInitData {
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    #[tlb(bits_len = 2)]
    pub contract_type: u8,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    pub pool_params: TLBRef<CoffeeLiquidityDepositoryInitPoolParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeLiquidityDepositoryInitPoolParams {
    pub asset1: CoffeeLiquidityDepositoryAsset,
    pub asset2: CoffeeLiquidityDepositoryAsset,
    pub amm: CoffeeLiquidityDepositoryAmm,
    pub amm_settings: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b00, bits_len = 2)]
pub struct CoffeeLiquidityDepositoryAssetNative {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeeLiquidityDepositoryAssetJetton {
    pub chain: u8,
    pub hash: U256,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeeLiquidityDepositoryAssetExtra {
    pub id: u32,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b000, bits_len = 3)]
pub struct CoffeeLiquidityDepositoryAmmConstantProduct {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b001, bits_len = 3)]
pub struct CoffeeLiquidityDepositoryAmmCurveFiStable {}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeLiquidityDepositoryAsset {
    AssetNative(Box<CoffeeLiquidityDepositoryAssetNative>),
    AssetJetton(Box<CoffeeLiquidityDepositoryAssetJetton>),
    AssetExtra(Box<CoffeeLiquidityDepositoryAssetExtra>),
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeLiquidityDepositoryAmm {
    AmmConstantProduct(Box<CoffeeLiquidityDepositoryAmmConstantProduct>),
    AmmCurveFiStable(Box<CoffeeLiquidityDepositoryAmmCurveFiStable>),
}

// Caller-code identity construction; parsing is not authentication.
impl CoffeeLiquidityDepositoryInitData {
    /// Exact code/data StateInit; caller supplies the factory's Init code.
    pub fn state_init(&self, init_code: &TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(init_code.clone(), self.to_cell()?))
    }
    /// Basechain identity address; this does not prove factory admission.
    pub fn derive_basechain_address(&self, init_code: &TonCell) -> TonCoreResult<TonAddress> {
        self.state_init(init_code)?.derive_address(0)
    }
}
