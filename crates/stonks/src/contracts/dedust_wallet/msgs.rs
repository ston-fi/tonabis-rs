//! DeDust presale wallet messages.
use crate::contracts::custom_wallet::msgs::StonksCustomWalletJettonBurnMsg;
use crate::contracts::custom_wallet::msgs::StonksCustomWalletJettonTransferMsg;
use crate::contracts::dedust_wallet::storage::StonksDedustWalletContext;
use crate::contracts::ston_minter::msgs::StonksStonMinterSellMsg;
use crate::contracts::ston_wallet::msgs::ReferencedPayload;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::cell::{CellBuilder, CellParser};
use ton::ton_core::errors::TonCoreError;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
/// DeDust v7/v12 internal transfer: required referenced payload, then context reference.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x178d4519, bits_len = 32)]
pub struct StonksDedustWalletInternalTransferMsg {
    /// Correlation id.
    pub query_id: u64,
    /// Tokens delivered.
    pub amount: TLBCoins,
    /// Previous owner, or addr_none for minting.
    #[tlb(adapter = "NullableAddress")]
    pub from_address: Option<TonAddress>,
    /// Excess value recipient.
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
    /// Notification value.
    pub forward_ton_amount: TLBCoins,
    /// Referenced notification payload; source requires the true selector.
    #[tlb(adapter = "ReferencedPayload")]
    pub forward_payload: TLBRef<TonCell>,
    /// Master/peer wallet context.
    pub context: TLBRef<StonksDedustWalletContext>,
}

/// Minter-controlled wallet context flag update.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a97b2, bits_len = 32)]
pub struct StonksDedustWalletSetFlagMsg {
    /// Correlation id.
    pub query_id: u64,
    /// New source-defined flag.
    pub flag: bool,
}

/// Truncated bounced transfers restore token amounts for the three presale-wallet
/// operations recognized in captured STON and DeDust wallet dispatch.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
#[tlb(prefix = 0xffffffff, bits_len = 32)]
pub struct StonksDedustWalletBouncedTransferMsg {
    /// Transfer, burn notification or presale sell notification opcode.
    #[tlb(adapter = "OriginalOpcode")]
    pub original_opcode: u32,
    /// Original query id.
    pub query_id: u64,
    /// Tokens restored by the wallet.
    pub amount: TLBCoins,
    /// Uninterpreted truncated suffix.
    pub tail: TonCell,
}

/// Current DeDust presale-wallet incoming dispatch; unknown bodies remain raw.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub enum StonksDedustWalletMessage {
    /// Standard owner transfer (wire layout is shared with the catalog record).
    Transfer(StonksCustomWalletJettonTransferMsg),
    /// Current context-bearing internal transfer.
    InternalTransfer(StonksDedustWalletInternalTransferMsg),
    /// Standard owner burn.
    Burn(StonksCustomWalletJettonBurnMsg),
    /// Owner sell with optional payout tail.
    Sell(StonksStonMinterSellMsg),
    /// Master context update.
    SetFlag(StonksDedustWalletSetFlagMsg),
    /// Token restoration on bounce.
    Bounce(StonksDedustWalletBouncedTransferMsg),
    /// Unrecognized raw body.
    Unknown(TonCell),
}

/// Only these original operations restore tokens for this wallet generation.
struct OriginalOpcode;
impl OriginalOpcode {
    fn read(&self, parser: &mut CellParser) -> TonCoreResult<u32> {
        let value = u32::read(parser)?;
        Self::validate(value)?;
        Ok(value)
    }
    fn write(&self, builder: &mut CellBuilder, value: &u32) -> TonCoreResult<()> {
        Self::validate(*value)?;
        value.write(builder)
    }
    fn validate(value: u32) -> TonCoreResult<()> {
        if !matches!(value, 0x178d4519 | 0x7bdd97de | 0xcc1a97b0) {
            ton::ton_core::bail_ton_core_data!("unsupported dedust-wallet bounced opcode");
        }
        Ok(())
    }
}
