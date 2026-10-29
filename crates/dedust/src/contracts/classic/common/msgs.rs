//! Blank-program installer envelope and finite supported initializer records.
//! The blank program invokes installed code58662. Program CODE is caller-owned;
//! its arbitrary initializer ABI is not inferred. Known classic programs below
//! have explicit typed arguments, selected by the caller rather than a classifier.
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::{cell::TonCell, errors::TonCoreError, traits::tlb::TLB as Tlb};

#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
#[tlb(prefix = 0x9b3aa3fa, bits_len = 32)]
pub struct DedustClassicInstallMsg<P: Tlb> {
    pub query_id: u64,
    pub blank_code: TLBRef<TonCell>,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
    pub parameters: P,
}
/// Both classic deposit generations' function103 bounced-pool body.
/// Callers first check the message's bounced flag and authenticated pool sender.
/// The contract ignores a wrong sender before reading this body; a matching
/// opcode triggers refund only while the deposit is processing. Neither check
/// can be established by parsing a body alone.

#[derive(Debug, Clone, PartialEq)]
pub enum DedustLiquidityDepositBounceMsg {
    /// Function103 skips the first32bits rather than authenticating a marker,
    /// reads the original opcode/query id, and ignores the remaining suffix.
    DepositLiquidityAll {
        prefix: u32,
        query_id: u64,
        ignored_suffix: TonCell,
    },
    /// An original opcode for which function103 performs no action.
    Unknown(TonCell),
}
impl DedustLiquidityDepositBounceMsg {
    pub fn from_cell(cell: &TonCell) -> Result<Self, TonCoreError> {
        let mut p = cell.parser();
        let prefix = p.read_num(32)?;
        let opcode = p.read_num::<u32>(32)?;
        if opcode != 0xb56b9598 {
            return Ok(Self::Unknown(cell.clone()));
        }
        Ok(Self::DepositLiquidityAll {
            prefix,
            query_id: p.read_num(64)?,
            ignored_suffix: p.read_remaining()?,
        })
    }

    pub fn to_cell(&self) -> Result<TonCell, TonCoreError> {
        match self {
            Self::Unknown(cell) => Ok(cell.clone()),
            Self::DepositLiquidityAll {
                prefix,
                query_id,
                ignored_suffix,
            } => {
                let mut b = TonCell::builder();
                b.write_num(prefix, 32)?;
                b.write_num(&0xb56b9598u32, 32)?;
                b.write_num(query_id, 64)?;
                b.write_cell(ignored_suffix)?;
                b.build()
            },
        }
    }
}
