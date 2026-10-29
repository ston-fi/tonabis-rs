//! Typed signature-bound references retain the original cell graph until edited.
use std::ops::{Deref, DerefMut};
use ton::ton_core::{
    cell::{CellBuilder, CellParser, TonCell},
    errors::TonCoreError,
    traits::tlb::TLB,
};

/// A referenced record whose original encoding participates in a signature hash.
///
/// Decoding is strict. Unchanged values retain valid alternative wire encodings;
/// edited values use their normal encoder and require a new caller-owned signature.
/// Like `TLBRef`, standalone cell/BOC operations address the referenced value.
#[derive(Debug, Clone)]
pub struct DedustSignatureRef<T> {
    value: T,
    original: Option<(T, TonCell)>,
}

impl<T> DedustSignatureRef<T> {
    /// Construct a new value without an existing signed encoding.
    pub fn new(value: T) -> Self {
        Self { value, original: None }
    }

    /// Consume the reference and discard its original encoding.
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: TLB + Clone + PartialEq> TLB for DedustSignatureRef<T> {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonCoreError> {
        Self::from_cell(parser.read_next_ref()?)
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonCoreError> {
        builder.write_ref(self.to_cell()?)
    }

    fn from_cell(cell: &TonCell) -> Result<Self, TonCoreError> {
        let value: T = super::utils::read_signature_record(cell)?;
        Ok(Self {
            original: Some((value.clone(), cell.clone())),
            value,
        })
    }

    fn to_cell(&self) -> Result<TonCell, TonCoreError> {
        if let Some((original, cell)) = &self.original
            && original == &self.value
        {
            return Ok(cell.clone());
        }
        self.value.to_cell()
    }

    fn to_boc_extra(&self, add_crc32: bool) -> Result<Vec<u8>, TonCoreError> {
        self.to_cell()?.to_boc_extra(add_crc32)
    }
}

impl<T: PartialEq> PartialEq for DedustSignatureRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<T> Deref for DedustSignatureRef<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}
impl<T> DerefMut for DedustSignatureRef<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
impl<T> From<T> for DedustSignatureRef<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
