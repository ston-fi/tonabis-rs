//! Shared asset identity for classic factories, pools, vaults and deposits.
use ton::{
    block_tlb::{FromTVMStack, TVMStack, ToTVMStack},
    errors::TonResult,
    ton_core::{
        TLB,
        cell::{TonCell, TonHash},
        traits::tlb::TLB,
    },
};

/// Native GRAM, encoded as the four-bit tag zero.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0, bits_len = 4)]
pub struct DedustClassicNativeAsset {}

/// Jetton minter identity, encoded after the four-bit tag one.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 4)]
pub struct DedustClassicJettonAsset {
    pub workchain_id: i8,
    pub address: TonHash,
}

/// Native currency or jetton identity shared by every classic generation.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustClassicAsset {
    Native(DedustClassicNativeAsset),
    Jetton(DedustClassicJettonAsset),
}

impl FromTVMStack for DedustClassicAsset {
    fn from_stack(stack: &mut TVMStack) -> TonResult<Self> {
        Ok(Self::from_cell(&TonCell::from_stack(stack)?)?)
    }
}

impl ToTVMStack for DedustClassicAsset {
    fn to_stack(&self, stack: &mut TVMStack) -> TonResult<()> {
        stack.push_cell_slice(self.to_cell()?);
        Ok(())
    }
}
