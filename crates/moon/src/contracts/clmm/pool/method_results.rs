//! Structured getter results with exact TVM stack arity.
use crate::contracts::address::MoonReferencedAddress;
use std::collections::HashMap;
use ton::block_tlb::FromTVMStack;
use ton::block_tlb::TVMStack;
use ton::errors::TonResult;
use ton::tlb_adapters::DictKeyAdapterInt;
use ton::tlb_adapters::DictValAdapterTLB;
use ton::tlb_adapters::TLBHashMap;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
#[derive(Debug, Clone, PartialEq)]
pub struct MoonClmmPoolRangesResult {
    pub ranges: HashMap<i16, TonAddress>,
}
impl FromTVMStack for MoonClmmPoolRangesResult {
    fn from_stack(stack: &mut TVMStack) -> TonResult<Self> {
        let cell = Option::<TonCell>::from_stack(stack)?;
        stack.ensure_empty()?;
        let mut ranges = HashMap::new();
        if let Some(cell) = cell {
            let mut parser = cell.parser();
            let values = TLBHashMap::<DictKeyAdapterInt<16, i16>, DictValAdapterTLB<MoonReferencedAddress>>::new(16)
                .read(&mut parser)?;
            ranges = values.into_iter().map(|(k, v)| (k, v.value)).collect();
        }
        Ok(Self { ranges })
    }
}
