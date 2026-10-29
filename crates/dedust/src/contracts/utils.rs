//! Private contract codecs, stack adapters and state construction helpers.
use std::{collections::HashMap, hash::Hash};
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::{
    block_tlb::{FromTVMStack, StateInit, TVMStack},
    errors::TonResult,
    tlb_adapters::{DictKeyAdapter, DictValAdapter, TLBHashMap},
    ton_core::{
        cell::{CellBuilder, CellParser, CellType, TonCell, TonHash},
        errors::TonCoreError,
        traits::tlb::TLB,
    },
};

pub(crate) fn invalid_tag(name: &str, tag: u64) -> TonCoreError {
    TonCoreError::Custom(format!("invalid {name} tag {tag}"))
}

pub(crate) fn dictionary_stack<KA, VA>(stack: &mut TVMStack, bits: u32) -> TonResult<HashMap<KA::KeyType, VA::ValType>>
where
    KA: DictKeyAdapter,
    VA: DictValAdapter,
    KA::KeyType: Eq + Hash,
{
    match Option::<TonCell>::from_stack(stack)? {
        Some(cell) => Ok(TLBHashMap::<KA, VA>::new(bits).read(&mut cell.parser())?),
        None => Ok(HashMap::new()),
    }
}

pub(crate) fn library(hash: TonHash) -> Result<TonCell, TonCoreError> {
    let mut b = TonCell::builder_extra(CellType::LibraryRef, 34);
    b.write_num(&2u8, 8)?;
    hash.write(&mut b)?;
    b.build()
}
pub(crate) fn same_shard(
    mut init: StateInit,
    close_to: &TonAddress,
    prefix: u8,
) -> Result<(StateInit, TonAddress), TonCoreError> {
    if prefix > 31 {
        return Err(TonCoreError::Custom("shard prefix exceeds uint5".into()));
    }
    init.split_depth = Some(prefix);
    let hash = init.cell_hash()?;
    let mut bytes = hash.as_slice().to_vec();
    let close = close_to.hash.as_slice();
    let whole = usize::from(prefix / 8);
    let rest = prefix % 8;
    bytes[..whole].copy_from_slice(&close[..whole]);
    if rest != 0 {
        let mask = u8::MAX << (8 - rest);
        bytes[whole] = (bytes[whole] & !mask) | (close[whole] & mask);
    }
    let address = TonAddress::new(0, TonHash::from_slice(&bytes)?);
    Ok((init, address))
}
pub(crate) fn classic_pair<A: TLB>(stable: bool, a: &A, b: &A) -> Result<TonCell, TonCoreError> {
    let ac = a.to_cell()?;
    let bc = b.to_cell()?;
    // All supported classic assets begin with distinct uint4 tags, or are equal-
    // width jetton records. Byte order equals SDLEXCMP for these typed records.
    let ordering = ac
        .parser()
        .read_bits(ac.data_len_bits())?
        .cmp(&bc.parser().read_bits(bc.data_len_bits())?);
    if ordering.is_eq() {
        return Err(TonCoreError::Custom("classic pool assets are equal (factory exit262)".into()));
    }
    let (lo, hi) = if ordering.is_lt() { (ac, bc) } else { (bc, ac) };
    let mut c = TonCell::builder();
    c.write_bit(stable)?;
    c.write_cell(&lo)?;
    c.write_cell(&hi)?;
    c.build()
}
pub(crate) fn classic_initial(
    factory: &TonAddress,
    code: TonCell,
    kind: u8,
    tail: &TonCell,
) -> Result<StateInit, TonCoreError> {
    let mut data = TonCell::builder();
    InternalAddress.write(&mut data, factory)?;
    data.write_num(&kind, 8)?;
    data.write_cell(tail)?;
    Ok(StateInit::new(code, data.build()?))
}

/// Complete consumption is reserved for graphs whose original encoding is signed.
pub(crate) fn read_signature_record<T: TLB>(cell: &TonCell) -> Result<T, TonCoreError> {
    let mut parser = cell.parser();
    let value = T::read(&mut parser)?;
    parser.ensure_empty()?;
    Ok(value)
}
pub(crate) fn stack_nullable_address(stack: &mut TVMStack) -> TonResult<Option<TonAddress>> {
    // TVM NULL is not addr_none; only address cells/slices are accepted here.
    let cell = TonCell::from_stack(stack)?;
    Ok(NullableAddress.read(&mut cell.parser())?)
}

/// Dictionary values must retain internal zero instead of writing addr_none.
pub(crate) struct DictValueInternalAddress;
impl DictValAdapter for DictValueInternalAddress {
    type ValType = TonAddress;
    fn read(parser: &mut CellParser) -> Result<TonAddress, TonCoreError> {
        InternalAddress.read(parser)
    }
    fn write(builder: &mut CellBuilder, value: &TonAddress) -> Result<(), TonCoreError> {
        InternalAddress.write(builder, value)
    }
}
