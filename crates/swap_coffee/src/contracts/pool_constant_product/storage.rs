//! Coffee storage. See tools/reference/swap_coffee/README.md.
use fastnum::U256;
use ton::block_tlb::TVMStack;
use ton::errors::TonResult;
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
pub struct CoffeeConstantProductPoolStorage {
    pub init_code: TLBRef<TonCell>,
    pub init_data: TLBRef<CoffeeConstantProductPoolProof>,
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
#[tlb(prefix = 0b00, bits_len = 2)]
pub struct CoffeePoolNativeAsset {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeePoolJettonAsset {
    pub chain: u8,
    pub hash: U256,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeePoolExtraAsset {
    pub id: u32,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeConstantProductPoolDoubleAssetParams {
    pub asset1: CoffeePoolAsset,
    pub asset2: CoffeePoolAsset,
    #[tlb(bits_len = 3)]
    pub amm: u8,
    pub amm_settings: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b00, bits_len = 2)]
pub struct CoffeePoolVaultProofBody {
    pub asset: CoffeePoolAsset,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeeConstantProductPoolSourcePoolProofBody {
    pub asset1: CoffeePoolAsset,
    pub asset2: CoffeePoolAsset,
    #[tlb(bits_len = 3)]
    pub amm: u8,
    pub amm_settings: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeeConstantProductPoolLiquidityDepositoryProofBody {
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    pub params: TLBRef<CoffeeConstantProductPoolDoubleAssetParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b11, bits_len = 2)]
pub struct CoffeeConstantProductPoolCreatorProofBody {
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    pub params: TLBRef<CoffeeConstantProductPoolDoubleAssetParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeConstantProductPoolProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    pub body: CoffeeConstantProductPoolProofBody,
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeePoolAsset {
    CoffeePoolNativeAsset(Box<CoffeePoolNativeAsset>),
    CoffeePoolJettonAsset(Box<CoffeePoolJettonAsset>),
    CoffeePoolExtraAsset(Box<CoffeePoolExtraAsset>),
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeConstantProductPoolProofBody {
    CoffeePoolVaultProofBody(Box<CoffeePoolVaultProofBody>),
    CoffeePoolPoolProofBody(Box<CoffeeConstantProductPoolSourcePoolProofBody>),
    CoffeePoolLiquidityDepositoryProofBody(Box<CoffeeConstantProductPoolLiquidityDepositoryProofBody>),
    CoffeePoolCreatorProofBody(Box<CoffeeConstantProductPoolCreatorProofBody>),
}

// Caller-code identity construction; parsing is not authentication.
impl CoffeeConstantProductPoolProof {
    /// Exact code/data StateInit; caller supplies the factory's Init code.
    pub fn state_init(&self, init_code: &TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(init_code.clone(), self.to_cell()?))
    }
    /// Basechain identity address; this does not prove factory admission.
    pub fn derive_basechain_address(&self, init_code: &TonCell) -> TonCoreResult<TonAddress> {
        self.state_init(init_code)?.derive_address(0)
    }
}

impl ton::block_tlb::ToTVMStack for CoffeePoolAsset {
    fn to_stack(&self, stack: &mut TVMStack) -> TonResult<()> {
        stack.push_cell_slice(self.to_cell()?);
        Ok(())
    }
}
