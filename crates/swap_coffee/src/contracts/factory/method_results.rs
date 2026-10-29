use crate::contracts::factory::storage::CoffeeFactoryCodeCells;
use crate::contracts::factory::storage::CoffeeFactoryCodeDictionaries;
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
// Structured getter results.
use fastnum::I512;
use ton::block_tlb::{FromTVMStack, TVMStack};
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::TLBRef;
/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq, ton::ton_core::FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct CoffeeFactoryAddressWithHashResult {
    #[from_tvm_stack(adapter = InternalAddress)]
    pub address: TonAddress,
    pub hash: I512,
}

/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq)]
pub struct CoffeeFactoryGetCodeResult {
    pub first: TLBRef<CoffeeFactoryCodeCells>,
    pub second: TLBRef<CoffeeFactoryCodeDictionaries>,
}
impl FromTVMStack for CoffeeFactoryGetCodeResult {
    fn from_stack(s: &mut TVMStack) -> TonResult<Self> {
        let second = TLBRef::new(CoffeeFactoryCodeDictionaries::from_cell(&TonCell::from_stack(s)?)?);
        let first = TLBRef::new(CoffeeFactoryCodeCells::from_cell(&TonCell::from_stack(s)?)?);
        s.ensure_empty()?;
        Ok(Self { first, second })
    }
}
