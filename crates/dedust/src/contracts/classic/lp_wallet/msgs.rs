//! Classic LP wallet surface, reconciled with the embedded V1 wallet code and
//! SDK0.8.7. Caller-owned forwarding tails preserve the exact source STSLICE
//! boundary, including forms the wallet forwards without decoding.
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::ton_core::{
    TLB,
    cell::{CellBuilder, CellParser, TonCell},
    errors::TonCoreError,
    traits::tlb::{TLB, TLBPrefix},
    types::tlb_core::{TLBCoins, TLBRef},
};

/// This wallet's burn branch consumes the common prefix and ignores the
/// remainder, including the SDK custom-payload bit/ref. Preserve that suffix.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x595f07bc, bits_len = 32)]
pub struct DedustClassicLpWalletBurnMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub suffix: TonCell,
}
/// Source transfer8 decodes this fixed prefix, discards the custom reference,
/// then copies the complete forward tail with STSLICE. Its only tail check is
/// SBITS>0 (708 otherwise); the wallet does not decode an Either-ref body.
#[derive(Debug, Clone, PartialEq)]
pub struct DedustClassicLpWalletTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub destination: Option<TonAddress>,
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TonCell,
}
impl TLB for DedustClassicLpWalletTransferMsg {
    const PREFIX: TLBPrefix = TLBPrefix::new(0x0f8a7ea5, 32);
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        let value = Self {
            query_id: p.read_num(64)?,
            amount: TLBCoins::read(p)?,
            destination: NullableAddress.read(p)?,
            response_destination: NullableAddress.read(p)?,
            custom_payload: Option::<TLBRef<TonCell>>::read(p)?,
            forward_ton_amount: TLBCoins::read(p)?,
            forward_payload: p.read_remaining()?,
        };
        value.validate_tail()?;
        Ok(value)
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        self.validate_tail()?;
        b.write_num(&self.query_id, 64)?;
        self.amount.write(b)?;
        NullableAddress.write(b, &self.destination)?;
        NullableAddress.write(b, &self.response_destination)?;
        self.custom_payload.write(b)?;
        self.forward_ton_amount.write(b)?;
        b.write_cell(&self.forward_payload)
    }
}
impl DedustClassicLpWalletTransferMsg {
    fn validate_tail(&self) -> Result<(), TonCoreError> {
        if self.forward_payload.data_len_bits() == 0 {
            return Err(TonCoreError::Custom(
                "LP transfer forward tail needs at least one bit (708)".into(),
            ));
        }
        Ok(())
    }
}
/// Source internal-transfer9 forwards its remainder without decoding it. The
/// amount-zero/excess/notification execution policy is separate from parsing.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x178d4519, bits_len = 32)]
pub struct DedustClassicLpWalletInternalTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub from_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TonCell,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustLpWalletIncomingMsg {
    Transfer(Box<DedustClassicLpWalletTransferMsg>),
    InternalTransfer(Box<DedustClassicLpWalletInternalTransferMsg>),
    Burn(DedustClassicLpWalletBurnMsg),
    Unknown(TonCell),
}

/// Restore-on-bounce data is a prefix, not a complete original transfer body.
#[derive(Debug, Clone, PartialEq)]
pub enum DedustLpWalletBounceMsg {
    InternalTransfer {
        /// The source skips these32bits; the message's bounced flag is separate.
        prefix: u32,
        query_id: u64,
        amount: TLBCoins,
        suffix: TonCell,
    },
    BurnNotification {
        prefix: u32,
        query_id: u64,
        amount: TLBCoins,
        suffix: TonCell,
    },
    Unknown(TonCell),
}
impl DedustLpWalletBounceMsg {
    pub fn from_cell(cell: &TonCell) -> Result<Self, TonCoreError> {
        let mut p = cell.parser();
        let prefix = p.read_num::<u32>(32)?;
        let tag = p.read_num::<u32>(32)?;
        Ok(match tag {
            0x178d4519 => Self::InternalTransfer {
                prefix,
                query_id: p.read_num(64)?,
                amount: TLBCoins::read(&mut p)?,
                suffix: p.read_remaining()?,
            },
            0x7bdd97de => Self::BurnNotification {
                prefix,
                query_id: p.read_num(64)?,
                amount: TLBCoins::read(&mut p)?,
                suffix: p.read_remaining()?,
            },
            _ => Self::Unknown(cell.clone()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct DedustClassicLpWalletExcessMsg {
    pub query_id: u64,
}
/// Notification uses a raw message-address form. The dependency notification
/// convenience type uses TonAddress and collapses literal standard zero to null.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7362d09c, bits_len = 32)]
pub struct DedustClassicLpWalletTransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub sender: Option<TonAddress>,
    /// Opaque source-copied tail, including its caller-selected leading bit.
    pub forward_payload: TonCell,
}
/// Outgoing wire records remain distinct from authenticated sender checks.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustLpWalletOutgoingMsg {
    InternalTransfer(Box<DedustClassicLpWalletInternalTransferMsg>),
    TransferNotification(Box<DedustClassicLpWalletTransferNotificationMsg>),
    BurnNotification(super::super::pool_v1::msgs::DedustPoolV1JettonBurnNotificationMsg),
    Excess(DedustClassicLpWalletExcessMsg),
    Unknown(TonCell),
}
