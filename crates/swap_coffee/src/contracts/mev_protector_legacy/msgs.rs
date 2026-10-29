//! Coffee msgs. See tools/reference/swap_coffee/README.md.
use crate::contracts::cross_dex::msgs::CoffeeCrossDexJettonTransferNotificationMsg;
use fastnum::U256;
use std::collections::HashMap;
use ton::ton_core::types::{
    TonAddress,
    tlb_core::adapters::{InternalAddress, NullableAddress},
};
use ton::{
    tlb_adapters::{DictKeyAdapterInt, DictValAdapterTLB, TLBHashMapE},
    ton_core::{
        TLB,
        cell::{CellBuilder, CellParser, TonCell, TonHash},
        errors::TonCoreResult,
        traits::tlb::TLB,
        types::tlb_core::{TLBCoins, TLBEither, TLBRef},
    },
};
use tonabis_core::msgs::EmptyBodyMsg;
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6bc79e7e, bits_len = 32)]
pub struct CoffeeMevProtectorLegacyHoldFundsMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xee51ce51, bits_len = 32)]
pub struct CoffeeMevProtectorLegacyFailedSwapPayloadMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x474f86cf, bits_len = 32)]
pub struct CoffeeMevProtectorLegacyDedustPayoutMsg {
    pub query_id: u64,
    pub payload: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x57861aa7, bits_len = 32)]
pub struct CoffeeMevProtectorLegacySetAdminMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub admin_address: Option<TonAddress>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xb19e2647, bits_len = 32)]
pub struct CoffeeMevProtectorLegacySetPublicKeyMsg {
    pub query_id: u64,
    pub public_key: U256,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6f80872c, bits_len = 32)]
pub struct CoffeeMevProtectorLegacyUpdateContractMsg {
    pub query_id: u64,
    pub code: Option<TLBRef<TonCell>>,
    pub data: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x2bf71be6, bits_len = 32)]
pub struct CoffeeMevProtectorLegacyAdmin2bf71be6Msg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    pub amount: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7b4b82fe, bits_len = 32)]
pub struct CoffeeMevProtectorLegacyAdmin7b4b82feMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub jetton_wallet: TonAddress,
    pub amount: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct CoffeeMevProtectorLegacyExcessesMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct CoffeeMevProtectorLegacyJettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeMevProtectorLegacySendMessageAction {
    pub mode: u8,
    pub message: TLBRef<TonCell>,
}
// Handwritten protocol constraint or signed-body preservation.
#[derive(Debug, Clone)]
pub struct CoffeeMevProtectorLegacySignedBatchMsg {
    pub signature: [u8; 64],
    pub subwallet_id: u32,
    pub query_id: u64,
    /// Signed16 traversal starts after -1; negative keys are retained but skipped
    /// by the observed external execution loop.
    pub payload: HashMap<i16, CoffeeMevProtectorLegacySendMessageAction>,
    original_body: Option<(TonHash, TonCell)>,
}
impl PartialEq for CoffeeMevProtectorLegacySignedBatchMsg {
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature
            && self.subwallet_id == other.subwallet_id
            && self.query_id == other.query_id
            && self.payload == other.payload
    }
}
impl CoffeeMevProtectorLegacySignedBatchMsg {
    /// Construct a canonical body for a caller-supplied signature. The caller must
    /// ensure that signature belongs to unsigned_body_cell(); mutation needs re-signing.
    pub fn new(
        signature: [u8; 64],
        subwallet_id: u32,
        query_id: u64,
        payload: HashMap<i16, CoffeeMevProtectorLegacySendMessageAction>,
    ) -> Self {
        Self {
            signature,
            subwallet_id,
            query_id,
            payload,
            original_body: None,
        }
    }
    /// Exact body after the signature. Preserves a parsed body's original dictionary
    /// encoding when unchanged, and canonicalizes newly constructed/modified fields.
    pub fn unsigned_body_cell(&self) -> TonCoreResult<TonCell> {
        let mut b = TonCell::builder();
        self.subwallet_id.write(&mut b)?;
        self.query_id.write(&mut b)?;
        TLBHashMapE::<DictKeyAdapterInt<16, i16>, DictValAdapterTLB<CoffeeMevProtectorLegacySendMessageAction>>::new(
            16,
        )
        .write(&mut b, &self.payload)?;
        let canonical = b.build()?;
        if let Some((logical_hash, original)) = &self.original_body
            && canonical.hash()? == logical_hash
        {
            return Ok(original.clone());
        }
        Ok(canonical)
    }
}
impl TLB for CoffeeMevProtectorLegacySignedBatchMsg {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        let signature = crate::contracts::wire::read_signature(p)?;
        let body = p.read_remaining()?;
        let mut data = body.parser();
        let subwallet_id = u32::read(&mut data)?;
        let query_id = u64::read(&mut data)?;
        let payload = TLBHashMapE::<
            DictKeyAdapterInt<16, i16>,
            DictValAdapterTLB<CoffeeMevProtectorLegacySendMessageAction>,
        >::new(16)
        .read(&mut data)?;
        data.ensure_empty()?;
        let mut value = Self::new(signature, subwallet_id, query_id, payload);
        value.original_body = Some((*value.unsigned_body_cell()?.hash()?, body));
        Ok(value)
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        b.write_bits(self.signature, 512)?;
        self.unsigned_body_cell()?.write(b)
    }
}

/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub enum CoffeeMevProtectorLegacyIncomingMessage {
    JettonTransferNotification(CoffeeCrossDexJettonTransferNotificationMsg),
    HoldFunds(CoffeeMevProtectorLegacyHoldFundsMsg),
    DedustPayout(CoffeeMevProtectorLegacyDedustPayoutMsg),
    SetAdmin(CoffeeMevProtectorLegacySetAdminMsg),
    SetPublicKey(CoffeeMevProtectorLegacySetPublicKeyMsg),
    UpdateContract(CoffeeMevProtectorLegacyUpdateContractMsg),
    Admin2bf71be6(CoffeeMevProtectorLegacyAdmin2bf71be6Msg),
    Admin7b4b82fe(CoffeeMevProtectorLegacyAdmin7b4b82feMsg),
    Excesses(CoffeeMevProtectorLegacyExcessesMsg),
    Empty(EmptyBodyMsg),

    Unknown(TonCell),
}

/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeMevProtectorLegacyIncomingExternalMessage {
    SignedBatch(CoffeeMevProtectorLegacySignedBatchMsg),
    Unknown(TonCell),
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
// Keep opcode prefixes visible: Box<T> erases them in locked TON dispatch.
#[allow(clippy::large_enum_variant)]
pub enum CoffeeMevProtectorLegacyOutgoingMessage {
    JettonTransfer(CoffeeMevProtectorLegacyJettonTransferMsg),
    Excesses(CoffeeMevProtectorLegacyExcessesMsg),

    Unknown(TonCell),
}
