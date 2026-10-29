//! Metadata byte strings preserve source chunk boundaries without lossy UTF-8.
use ton::ton_core::{
    cell::{CellBuilder, CellParser, TonCell},
    errors::{TonCoreError, TonCoreResult},
    traits::tlb::{TLB, TLBPrefix},
};
/// TEP-64 zero-prefixed, byte-aligned linear snake. Each element holds one
/// cell's bytes, excluding the prefix byte in the first cell.
/// Unlike native `SnakeData`, this validates the prefix and linear byte layout,
/// and retains empty continuation cells when re-encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoonPrefixedString {
    pub chunks: Vec<Vec<u8>>,
}
fn chunk(parser: &mut CellParser) -> TonCoreResult<(Vec<u8>, Option<TonCell>)> {
    let bits = parser.data_bits_left()?;
    if bits % 8 != 0 || parser.refs_left() > 1 {
        ton::ton_core::bail_ton_core_data!("metadata must be a byte-aligned linear snake");
    }
    let bytes = parser.read_bits(bits)?;
    let next = if parser.refs_left() == 0 {
        None
    } else {
        Some(parser.read_next_ref()?.clone())
    };
    parser.ensure_empty()?;
    Ok((bytes, next))
}
impl TLB for MoonPrefixedString {
    const PREFIX: TLBPrefix = TLBPrefix::new(0, 8);
    fn read_definition(parser: &mut CellParser) -> TonCoreResult<Self> {
        let (head, mut next) = chunk(parser)?;
        let mut chunks = vec![head];
        while let Some(cell) = next {
            let (bytes, rest) = chunk(&mut cell.parser())?;
            chunks.push(bytes);
            next = rest;
        }
        Ok(Self { chunks })
    }
    fn write_definition(&self, builder: &mut CellBuilder) -> TonCoreResult<()> {
        let head = self
            .chunks
            .first()
            .ok_or_else(|| TonCoreError::data(module_path!(), "metadata snake needs a root chunk"))?;
        let mut next = None;
        for bytes in self.chunks.iter().skip(1).rev() {
            let mut b = TonCell::builder();
            b.write_bits(bytes, bytes.len() * 8)?;
            if let Some(c) = next {
                b.write_ref(c)?;
            }
            next = Some(b.build()?);
        }
        builder.write_bits(head, head.len() * 8)?;
        if let Some(c) = next {
            builder.write_ref(c)?;
        }
        Ok(())
    }
}
