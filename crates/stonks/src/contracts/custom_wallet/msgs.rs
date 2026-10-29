//! custom_wallet msgs records.
use crate::contracts::custom_wallet::storage::StonksCustomWalletJettonTransferContext;
use crate::contracts::ston_minter::msgs::{
    StonksStonMinterJettonBurnNotificationMsg, StonksStonMinterJettonExcessesMsg,
};
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
use tonabis_core::msgs::EmptyBodyMsg;
/// Owner transfer with a raw Either forward tail, validated by dispatch.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct StonksCustomWalletJettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub destination: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    #[tlb(adapter = "ForwardPayload")]
    pub forward_payload: TonCell,
}

/// Internal token transfer with its source-defined context and raw forward tail.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x178d4519, bits_len = 32)]
pub struct StonksCustomWalletJettonInternalTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub sender_owner: Option<TonAddress>,
    pub transfer_context: TLBRef<StonksCustomWalletJettonTransferContext>,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TonCell,
}

/// Owner burn request with optional custom payload.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x595f07bc, bits_len = 32)]
pub struct StonksCustomWalletJettonBurnMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
}

/// Tax distribution transfer with separate peer context addresses.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a97ab, bits_len = 32)]
pub struct StonksCustomWalletJettonTaxTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub sender_owner: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub context_address0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub context_address1: Option<TonAddress>,
}

/// Tax-wallet bounce with a checked original-opcode domain and preserved raw tail.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
#[tlb(prefix = 0xffffffff, bits_len = 32)]
pub struct StonksCustomWalletBouncedKnownBodyMsg {
    #[tlb(adapter = "OriginalOpcode")]
    pub original_opcode: u32,
    pub query_id: u64,
    pub amount: TLBCoins,
    pub tail: TonCell,
}

/// Forwarded transfer notification with its original raw payload tail.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7362d09c, bits_len = 32)]
pub struct StonksCustomWalletJettonTransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub sender_owner: Option<TonAddress>,
    pub forward_payload: TonCell,
}

/// Known typed bodies and losslessly retained unknown messages.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub enum StonksCustomWalletMessage {
    JettonTransfer(StonksCustomWalletJettonTransferMsg),
    JettonInternalTransfer(StonksCustomWalletJettonInternalTransferMsg),
    JettonBurn(StonksCustomWalletJettonBurnMsg),
    JettonTaxTransfer(StonksCustomWalletJettonTaxTransferMsg),
    CustomJettonWalletBouncedKnownBody(StonksCustomWalletBouncedKnownBodyMsg),
    JettonTransferNotification(StonksCustomWalletJettonTransferNotificationMsg),
    JettonExcess(StonksStonMinterJettonExcessesMsg),
    JettonBurnNotification(StonksStonMinterJettonBurnNotificationMsg),

    /// Source-defined value-only distribution body.
    Empty(EmptyBodyMsg),
    Unknown(TonCell),
}

/// Validate the required Either selector while retaining the caller's raw tail.
pub(crate) struct ForwardPayload;
impl ForwardPayload {
    pub(crate) fn read(&self, parser: &mut CellParser) -> TonCoreResult<TonCell> {
        let cell = TonCell::read(parser)?;
        crate::contracts::ston_minter::msgs::validate_tail(&cell)?;
        Ok(cell)
    }
    pub(crate) fn write(&self, builder: &mut CellBuilder, value: &TonCell) -> TonCoreResult<()> {
        value.write(builder)
    }
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
        if !matches!(value, 0x178d4519 | 0x7bdd97de) {
            ton::ton_core::bail_ton_core_data!("unsupported custom-wallet bounced opcode");
        }
        Ok(())
    }
}
