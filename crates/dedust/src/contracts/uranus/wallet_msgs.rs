//! Both supported wallet generations restore amounts for these three bounced
//! operations. Sender authorization is not inferred from a body. A preserved
//! unknown opcode is rejected by captured wallet execution with code63.
use ton::ton_core::{cell::TonCell, errors::TonCoreError, traits::tlb::TLB, types::tlb_core::TLBCoins};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DedustUranusWalletRestoredOperation {
    InternalTransfer,
    BurnNotification,
    SellTokens,
}
#[derive(Debug, Clone, PartialEq)]
pub enum DedustUranusWalletBounceMsg {
    Restore {
        /// Source LDU32/NIP discards this field, without marker validation.
        prefix: u32,
        operation: DedustUranusWalletRestoredOperation,
        query_id: u64,
        amount: TLBCoins,
        ignored_suffix: TonCell,
    },
    Unknown(TonCell),
}
impl DedustUranusWalletBounceMsg {
    pub fn from_cell(cell: &TonCell) -> Result<Self, TonCoreError> {
        let mut p = cell.parser();
        let prefix = p.read_num(32)?;
        let operation = match p.read_num::<u32>(32)? {
            0x178d4519 => DedustUranusWalletRestoredOperation::InternalTransfer,
            0x7bdd97de => DedustUranusWalletRestoredOperation::BurnNotification,
            0x646ad424 => DedustUranusWalletRestoredOperation::SellTokens,
            _ => return Ok(Self::Unknown(cell.clone())),
        };
        Ok(Self::Restore {
            prefix,
            operation,
            query_id: p.read_num(64)?,
            amount: TLBCoins::read(&mut p)?,
            ignored_suffix: p.read_remaining()?,
        })
    }
    pub fn to_cell(&self) -> Result<TonCell, TonCoreError> {
        match self {
            Self::Unknown(cell) => Ok(cell.clone()),
            Self::Restore {
                prefix,
                operation,
                query_id,
                amount,
                ignored_suffix,
            } => {
                let opcode = match operation {
                    DedustUranusWalletRestoredOperation::InternalTransfer => 0x178d4519u32,
                    DedustUranusWalletRestoredOperation::BurnNotification => 0x7bdd97de,
                    DedustUranusWalletRestoredOperation::SellTokens => 0x646ad424,
                };
                let mut b = TonCell::builder();
                b.write_num(prefix, 32)?;
                b.write_num(&opcode, 32)?;
                b.write_num(query_id, 64)?;
                amount.write(&mut b)?;
                b.write_cell(ignored_suffix)?;
                b.build()
            },
        }
    }
}
