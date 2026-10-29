//! Narrow address/dictionary adapters for domains not represented by native stack traits.
use std::collections::HashMap;
use std::hash::Hash;
use ton::block_tlb::{FromTVMStack, TVMStack};
use ton::errors::TonResult;
use ton::tlb_adapters::{DictKeyAdapter, DictValAdapterTLB, TLBHashMapE};
use ton::ton_core::cell::{CellBuilder, CellParser, TonCell};
use ton::ton_core::errors::{TonCoreError, TonCoreResult};
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

pub(super) fn wire_error(message: &str) -> TonCoreError {
    TonCoreError::data("tonabis_swap_coffee", message)
}
pub(super) fn read_signature(parser: &mut CellParser) -> TonCoreResult<[u8; 64]> {
    let mut signature = [0; 64];
    parser.read_bits_to(512, &mut signature)?;
    Ok(signature)
}
pub(super) fn stack_nullable_address(stack: &mut TVMStack) -> TonResult<Option<TonAddress>> {
    Ok(NullableInternalAddress.read(&mut TonCell::from_stack(stack)?.parser())?)
}
pub(super) fn stack_optional_ref<T: TLB>(stack: &mut TVMStack) -> TonResult<Option<TLBRef<T>>> {
    Option::<TonCell>::from_stack(stack)?
        .map(|cell| T::from_cell(&cell).map(TLBRef::new))
        .transpose()
        .map_err(Into::into)
}
pub(super) fn stack_dictionary<KA, V>(stack: &mut TVMStack, width: u32) -> TonResult<HashMap<KA::KeyType, V>>
where
    KA: DictKeyAdapter,
    KA::KeyType: Eq + Hash + Ord,
    V: TLB,
{
    let Some(root) = Option::<TonCell>::from_stack(stack)? else {
        return Ok(Default::default());
    };
    let mut builder = TonCell::builder();
    builder.write_bit(true)?;
    builder.write_ref(root)?;
    let cell = builder.build()?;
    Ok(TLBHashMapE::<KA, DictValAdapterTLB<V>>::new(width).read(&mut cell.parser())?)
}
/// Untagged suffix: omitted or exactly one selector bit, with no Maybe marker.
pub(super) struct TrailingSelector;
impl TrailingSelector {
    pub fn read(&self, parser: &mut CellParser) -> TonCoreResult<Option<bool>> {
        match parser.data_bits_left()? {
            0 => Ok(None),
            1 => Ok(Some(bool::read(parser)?)),
            _ => Err(wire_error("more than one optional trailing selector bit")),
        }
    }
    pub fn write(&self, builder: &mut CellBuilder, value: &Option<bool>) -> TonCoreResult<()> {
        if let Some(value) = value {
            value.write(builder)?;
        }
        Ok(())
    }
}
