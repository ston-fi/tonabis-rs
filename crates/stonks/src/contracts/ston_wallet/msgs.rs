//! STON presale wallet messages.
use crate::contracts::custom_wallet::msgs::StonksCustomWalletJettonBurnMsg;
use crate::contracts::custom_wallet::msgs::StonksCustomWalletJettonTransferMsg;
use crate::contracts::dedust_wallet::msgs::StonksDedustWalletBouncedTransferMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterSellMsg;
use crate::contracts::ston_wallet::storage::StonksStonWalletContext;
use ton::ton_core::TLB;
use ton::ton_core::cell::CellBuilder;
use ton::ton_core::cell::CellParser;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreError;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::traits::tlb::TLB as _;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
/// STON wallet-to-wallet transfer preserves its two-address context separately
/// from the required referenced notification payload.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x178d4519, bits_len = 32)]
pub struct StonksStonWalletInternalTransferMsg {
    /// Correlation id.
    pub query_id: u64,
    /// Tokens delivered.
    pub amount: TLBCoins,
    /// Previous owner or mint origin.
    #[tlb(adapter = "NullableAddress")]
    pub from_address: Option<TonAddress>,
    /// Excess recipient.
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
    /// Forwarded notification value.
    pub forward_ton_amount: TLBCoins,
    /// Referenced notification payload; source requires the true selector.
    #[tlb(adapter = "ReferencedPayload")]
    pub forward_payload: TLBRef<TonCell>,
    /// STON-specific wallet context.
    pub context: TLBRef<StonksStonWalletContext>,
}

/// Current STON v10/v11 presale-wallet incoming dispatch. Older fixed-layout
/// messages remain available in minter; forwarding semantics are revision-specific.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub enum StonksStonWalletMessage {
    /// Standard owner transfer.
    Transfer(StonksCustomWalletJettonTransferMsg),
    /// Two-address context-bearing internal transfer.
    InternalTransfer(StonksStonWalletInternalTransferMsg),
    /// Standard owner burn.
    Burn(StonksCustomWalletJettonBurnMsg),
    /// Owner sell with source-defined optional payout tail.
    Sell(StonksStonMinterSellMsg),
    /// Restore a failed transfer, burn or sell amount.
    Bounce(StonksDedustWalletBouncedTransferMsg),
    /// Unrecognized raw body.
    Unknown(TonCell),
}

/// The payload selector must be true; the following context is a separate reference.
pub(crate) struct ReferencedPayload;
impl ReferencedPayload {
    pub(crate) fn read(&self, p: &mut CellParser) -> TonCoreResult<TLBRef<TonCell>> {
        if !p.read_bit()? {
            ton::ton_core::bail_ton_core_data!("presale transfer requires a referenced payload");
        }
        TLBRef::read(p)
    }
    pub(crate) fn write(&self, b: &mut CellBuilder, value: &TLBRef<TonCell>) -> TonCoreResult<()> {
        b.write_bit(true)?;
        value.write(b)
    }
}
