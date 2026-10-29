//! Schema records and authored protocol codecs. See the crate guide for evidence
//! and the explicit generator ownership boundary.
use crate::contracts::{
    custom_wallet::v2::storage::{
        DedustX1000BidaskAdditionalData, DedustX1000BidaskDammReferral, DedustX1000BidaskSlippage,
        DedustX1000CoffeeSwapParams, DedustX1000CoffeeSwapStep, DedustX1000CpmmNextStep, DedustX1000CpmmPartnerConfig,
        DedustX1000CpmmPayoutConfig, DedustX1000CpmmReferrerConfig, DedustX1000MoonClmmCallback,
        DedustX1000MoonSwapParams, DedustX1000StonfiV2SwapBody, DedustX1000SwapParams, DedustX1000SwapStep,
        DedustX1000ToncoMultihop, DedustX1000UranusPartnerConfig, DedustX1000UranusReferrerConfig,
        DedustX1000WalletV2PendingContinuation,
    },
    signature_ref::DedustSignatureRef,
};
use fastnum::{U256, U512};
use std::collections::HashMap;
use ton::{
    tlb_adapters::{DictKeyAdapterUint, DictValAdapterNum, TLBHashMapE},
    ton_core::{
        TLB,
        cell::{CellBuilder, CellParser, TonCell},
        errors::TonCoreError,
        traits::tlb::{TLB, TLBPrefix},
        types::{
            TonAddress,
            tlb_core::{
                MsgAddressInt, TLBCoins, TLBEitherRef, TLBRef,
                adapters::{ConstLen, InternalAddress, NullableAddress},
            },
        },
    },
};
/// Signed external wallet request with a session and batched-action tail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000WalletV2SignedRequestMsg {
    /// Ed25519 signature checked against the public key in storage.
    pub signature: U512,
    pub seqno: u32,
    /// Fixed typed signed envelope and action; unresolved scalar meanings remain explicitly positional.
    pub payload: crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2SignedPayloadMsg,
}
/// Generic correlation envelope only. Use IncomingMessage for known fixed callbacks; parsing this envelope alone does not establish their schema or dispatch acceptance.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000WalletV2InternalCallbackMsg {
    pub opcode: u32,
    pub request_id: u64,
    pub payload: TonCell,
}
/// Controller callback that releases a stored outgoing message for execution.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 16843266, bits_len = 32)]
pub struct DedustX1000WalletV2ExecuteStoredMsg {
    pub request_id: u64,
    /// Source controller branch ignores all remaining bits/refs after request id; retained byte-exact, not an unresolved callback record.
    pub ignored_suffix: TonCell,
}
/// Outbound controller request for execution of a stored message.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 16843265, bits_len = 32)]
pub struct DedustX1000WalletV2ExecutionRequestedMsg {
    pub request_id: u64,
}
/// TEP-74 Jetton transfer emitted to a Jetton wallet.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 260734629, bits_len = 32)]
pub struct DedustX1000WalletV2JettonTransferMsg {
    pub query_id: u64,
    /// Jetton quantity in token base units.
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    /// GRAM amount forwarded with the recipient notification, in nanograms.
    pub forward_amount: TLBCoins,
    pub forward_payload: Option<TLBRef<TonCell>>,
}
/// DeDust Vault swap payload embedded in a Jetton transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3818968194, bits_len = 32)]
pub struct DedustX1000DedustSwapPayloadMsg {
    pub step: DedustX1000SwapStep,
    pub params: TLBRef<DedustX1000SwapParams>,
}
/// STON.fi V2 swap payload embedded in a Jetton transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1717886506, bits_len = 32)]
pub struct DedustX1000StonfiV2SwapPayloadMsg {
    #[tlb(adapter = "InternalAddress")]
    pub token_wallet: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub refund_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub excess_address: TonAddress,
    /// Unix deadline for execution.
    pub deadline: u64,
    pub swap_body: TLBRef<DedustX1000StonfiV2SwapBody>,
}
/// TONCO swap payload embedded in a Jetton transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2818267384, bits_len = 32)]
pub struct DedustX1000ToncoSwapPayloadMsg {
    #[tlb(adapter = "InternalAddress")]
    pub target_jetton_wallet: TonAddress,
    #[tlb(bits_len = 160)]
    pub sqrt_price_limit_x96: U256,
    pub min_out: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    pub multihop: Option<TLBRef<DedustX1000ToncoMultihop>>,
}
/// DeDust CPMM swap operation payload.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3292680207, bits_len = 32)]
pub struct DedustX1000CpmmSwapPayloadMsg {
    pub min_out: TLBCoins,
    /// Unix deadline encoded in 40 bits.
    #[tlb(bits_len = 40)]
    pub deadline: u64,
    pub next: Option<TLBRef<Box<DedustX1000CpmmNextStep>>>,
    pub partner: Option<DedustX1000CpmmPartnerConfig>,
    pub referrer: Option<DedustX1000CpmmReferrerConfig>,
}
/// DeDust CPMM Jetton payment payload.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3418569033, bits_len = 32)]
pub struct DedustX1000CpmmPayJettonPayloadMsg {
    pub operation: TLBRef<DedustX1000CpmmSwapPayloadMsg>,
    pub payout: TLBRef<DedustX1000CpmmPayoutConfig>,
}
/// Bidask DAMM swap payload embedded in a Jetton transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3715724076, bits_len = 32)]
pub struct DedustX1000BidaskDammSwapPayloadMsg {
    #[tlb(adapter = "InternalAddress")]
    pub to_address: TonAddress,
    pub slippage: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub from_address: TonAddress,
    /// Exact-output target, or zero for exact-input mode.
    pub exact_out: TLBCoins,
    pub referral: Option<TLBRef<DedustX1000BidaskDammReferral>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
}
/// Bidask CLMM swap payload embedded in a Jetton transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3231557710, bits_len = 32)]
pub struct DedustX1000BidaskClmmSwapPayloadMsg {
    #[tlb(adapter = "InternalAddress")]
    pub to_address: TonAddress,
    pub slippage: DedustX1000BidaskSlippage,
    /// Exact-output target, or zero for exact-input mode.
    pub exact_out: TLBCoins,
    pub additional_data: Option<TLBRef<DedustX1000BidaskAdditionalData>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
}
/// Coffee swap payload embedded in a Jetton transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3237998096, bits_len = 32)]
pub struct DedustX1000CoffeeSwapPayloadMsg {
    pub step: DedustX1000CoffeeSwapStep,
    pub params: Option<TLBRef<DedustX1000CoffeeSwapParams>>,
}
/// Moon swap payload embedded in a Jetton transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3011153931, bits_len = 32)]
pub struct DedustX1000MoonSwapPayloadMsg {
    pub params: DedustX1000MoonSwapParams,
}
/// Moon success callback body produced by the x1000 route.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3414112470, bits_len = 32)]
pub struct DedustX1000MoonSwapSucceededMsg {
    pub request_id: u64,
}
/// Moon failure callback body produced by the x1000 route.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3296468823, bits_len = 32)]
pub struct DedustX1000MoonSwapFailedMsg {
    pub request_id: u64,
}
/// Moon CLMM swap payload embedded in a Jetton transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3511755023, bits_len = 32)]
pub struct DedustX1000MoonClmmSwapPayloadMsg {
    #[tlb(adapter = "InternalAddress")]
    pub excess_address: TonAddress,
    pub sqrt_price_limit_x128: U256,
    pub fulfill: Option<TLBRef<DedustX1000MoonClmmCallback>>,
    pub reject: Option<TLBRef<DedustX1000MoonClmmCallback>>,
}
/// Application payload `0xfa45056b` with a coin value and two optional addresses.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 4198827371, bits_len = 32)]
pub struct DedustX1000PayloadFa45056bMsg {
    #[tlb(adapter = "InternalAddress")]
    pub address0: TonAddress,
    pub unknown0: TLBCoins,
    pub unknown1: Option<TLBRef<MsgAddressInt>>,
    pub unknown2: Option<TLBRef<MsgAddressInt>>,
}
/// STON.fi V1 swap payload embedded in a Jetton transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 630424929, bits_len = 32)]
pub struct DedustX1000StonfiV1SwapPayloadMsg {
    #[tlb(adapter = "InternalAddress")]
    pub token_wallet: TonAddress,
    pub min_out: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub referral_address: Option<TonAddress>,
}
/// StonksPump presale sell request emitted by the wallet.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3424294831, bits_len = 32)]
pub struct DedustX1000StonksPumpSellMsg {
    pub query_id: u64,
    /// Jetton quantity sold in token base units.
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub proceeds_recipient: Option<TonAddress>,
    pub min_gram_out: TLBCoins,
}
/// DeDust Uranus token sell request emitted by the wallet.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3074793004, bits_len = 32)]
pub struct DedustX1000UranusSellMsg {
    pub query_id: u64,
    /// Token quantity sold in base units.
    pub amount: TLBCoins,
    pub min_gram_out: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub excess_address: Option<TonAddress>,
    pub partner: Option<TLBRef<DedustX1000UranusPartnerConfig>>,
    pub referrer: Option<TLBRef<DedustX1000UranusReferrerConfig>>,
}
/// TEP-74 Jetton burn request emitted by the wallet.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1499400124, bits_len = 32)]
pub struct DedustX1000JettonBurnMsg {
    pub query_id: u64,
    /// Jetton quantity burned in token base units.
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
}
/// Source-defined outbound event `0x10937847`.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 278100039, bits_len = 32)]
pub struct DedustX1000WalletV2Event10937847Msg {
    pub unknown0: u64,
    pub unknown1: u32,
}
/// Source-defined outbound event `0xbeaf9617`.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3199178263, bits_len = 32)]
pub struct DedustX1000WalletV2EventBeaf9617Msg {
    pub unknown0: u64,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustX1000WalletV2IncomingMsg {
    ExecuteStored(Box<DedustX1000WalletV2ExecuteStoredMsg>),
    JettonNotification(Box<DedustX1000WalletV2JettonNotificationMsg>),
    JettonExcesses(Box<DedustX1000WalletV2JettonExcessesMsg>),
    ClassicNativePayout(Box<DedustX1000WalletV2ClassicNativePayoutMsg>),
    CoffeePayout(Box<DedustX1000WalletV2CoffeePayoutMsg>),
    PtonTransfer(Box<DedustX1000WalletV2PtonTransferMsg>),
    BidaskNotification(Box<DedustX1000WalletV2BidaskNotificationMsg>),
    MoonClmmNativeResult(Box<DedustX1000WalletV2MoonClmmNativeResultMsg>),
    MoonClmmNativeRefund(Box<DedustX1000WalletV2MoonClmmNativeRefundMsg>),
    X1000MoonSwapSucceeded(Box<DedustX1000MoonSwapSucceededMsg>),
    X1000MoonSwapFailed(Box<DedustX1000MoonSwapFailedMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustX1000WalletV2OutgoingMsg {
    ExecutionRequested(Box<DedustX1000WalletV2ExecutionRequestedMsg>),
    JettonTransfer(Box<DedustX1000WalletV2JettonTransferMsg>),
    X1000StonksPumpSell(Box<DedustX1000StonksPumpSellMsg>),
    X1000UranusSell(Box<DedustX1000UranusSellMsg>),
    X1000JettonBurn(Box<DedustX1000JettonBurnMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustX1000WalletV2EventMsg {
    Event10937847(Box<DedustX1000WalletV2Event10937847Msg>),
    EventBeaf9617(Box<DedustX1000WalletV2EventBeaf9617Msg>),
    Unknown(TonCell),
}

// Fixed callback records and byte-preserving callback-tail variants.

/// Captured X1000 builder7 emits recipient-only Moon CLMM callback references.
/// The catalog also defines explicitly marked optional payloads. Both encodings
/// are retained; this does not assert a peer revision accepts the compact form.
#[derive(Debug, Clone, PartialEq)]
pub enum DedustX1000WalletV2CallbackPayloadMsg {
    Omitted,
    Explicit(Option<TonCell>),
}
impl TLB for DedustX1000WalletV2CallbackPayloadMsg {
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        if p.data_bits_left()? == 0 && p.refs_left() == 0 {
            return Ok(Self::Omitted);
        }
        let payload = if p.read_bit()? {
            Some(p.read_next_ref()?.clone())
        } else {
            None
        };

        Ok(Self::Explicit(payload))
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        match self {
            Self::Omitted => Ok(()),
            Self::Explicit(None) => b.write_bit(false),
            Self::Explicit(Some(v)) => {
                b.write_bit(true)?;
                b.write_ref(v.clone())
            },
        }
    }
}

/// TEP-74 notification; nullable encoding preserves absence versus internal zero.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7362d09c, bits_len = 32)]
pub struct DedustX1000WalletV2JettonNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub sender: Option<TonAddress>,
    pub forward_payload: TLBEitherRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct DedustX1000WalletV2JettonExcessesMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x474f86cf, bits_len = 32)]
pub struct DedustX1000WalletV2ClassicNativePayoutMsg {
    pub query_id: u64,
    pub payload: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee32, bits_len = 32)]
pub struct DedustX1000WalletV2CoffeePayoutMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x01f3835d, bits_len = 32)]
pub struct DedustX1000WalletV2PtonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub refund_address: TonAddress,
    pub forward_payload: TLBEitherRef<TonCell>,
}

/// The older wrapper schema and flagged pool notification are separate fixed
/// wire forms. Their complete bit lengths differ modulo8, so the decoder does
/// not guess by probing malformed known messages or dropping a trailing bit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6edd65f0, bits_len = 32)]
pub struct DedustX1000WalletV2BidaskLegacyNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6edd65f0, bits_len = 32)]
pub struct DedustX1000WalletV2BidaskFlaggedNotificationMsg {
    pub query_id: u64,
    pub notification_bit: bool,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}
/// These peer flavors overlap on opcode and use body width as their discriminator.
/// Unmodeled bits can change that discriminator; select a concrete flavor when
/// decoding an extended body. Extra references do not change the flavor.
#[derive(Debug, Clone, PartialEq)]
// Closed body layouts of one known opcode, not an open message dispatcher.
// An Unknown arm here would consume unrelated opcodes inside the outer dispatch.
pub enum DedustX1000WalletV2BidaskNotificationMsg {
    Legacy(DedustX1000WalletV2BidaskLegacyNotificationMsg),
    Flagged(DedustX1000WalletV2BidaskFlaggedNotificationMsg),
}
impl TLB for DedustX1000WalletV2BidaskNotificationMsg {
    const PREFIX: TLBPrefix = TLBPrefix::new(0x6edd65f0, 32);
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        match p.data_bits_left()? % 8 {
            5 => Ok(Self::Legacy(DedustX1000WalletV2BidaskLegacyNotificationMsg::read_definition(
                p,
            )?)),
            6 => Ok(Self::Flagged(DedustX1000WalletV2BidaskFlaggedNotificationMsg::read_definition(
                p,
            )?)),
            _ => Err(TonCoreError::Custom("malformed fixed Bidask notification width".into())),
        }
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        match self {
            Self::Legacy(v) => v.write_definition(b),
            Self::Flagged(v) => v.write_definition(b),
        }
    }
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x9d142643, bits_len = 32)]
pub struct DedustX1000WalletV2MoonClmmNativeResultMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xff32da99, bits_len = 32)]
pub struct DedustX1000WalletV2MoonClmmNativeRefundMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Same peer tag inside a Jetton forward payload has a different fixed body;
/// the enclosing native message carries query/amount, the Jetton wrapper does not.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x9d142643, bits_len = 32)]
pub struct DedustX1000WalletV2MoonClmmJettonResultPayloadMsg {
    pub payload: Option<TLBRef<TonCell>>,
}

// Fixed V2 external envelope after signature/sequence. Widths and field order
// follow captured dispatch524287 (code4609b587...), not a guessed session ABI.
// Several signed scalar meanings are unresolved and retain positional names.
// Construction neither signs nor validates authority. Arbitrary outgoing body
// bytes are forwarded by LDREF/STREF/SENDRAWMSG; extension code owns that ABI.

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000WalletV2SignedPayloadMsg {
    /// Signed word discarded by the fixed loader before signature checking.
    pub ignored_word: u32,
    pub field32_0: u32,
    pub flag: bool,
    pub field16: u16,
    /// Passed to SENDRAWMSG by the fixed dispatch.
    pub send_mode: u8,
    /// Signed word discarded immediately after loading.
    pub ignored_word1: u32,
    pub field32_1: u32,
    pub action: DedustSignatureRef<DedustX1000WalletV2Action>,
    /// The fixed loader uses PLDREFIDX0 and does not consume this suffix.
    /// It remains signature-bound, even though fixed dispatch ignores it.
    pub ignored_suffix: TonCell,
}
/// One outgoing action. The mandatory body-reference bit is checked against1
/// by THROWIFNOT555. Cache keys use DICTUGET28, values PLDU64. The final Maybe
/// reference is a continuation record, not a second dictionary.
#[derive(Debug, Clone, PartialEq, TLB)]
// SignatureRef compares this complete signed action before preserving its graph.
#[tlb(ensure_empty = true)]
pub struct DedustX1000WalletV2Action {
    pub destination: MsgAddressInt,
    pub amount: TLBCoins,
    pub reserved_amount: TLBCoins,
    pub body: DedustX1000WalletV2BodyReference,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u32>,DictValAdapterNum<u64,64>>::new(28)")]
    pub cache: HashMap<u32, u64>,
    pub request_id: u64,
    pub pending: Option<TLBRef<DedustX1000WalletV2PendingContinuation>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 1)]
pub struct DedustX1000WalletV2BodyReference {
    pub body: TLBRef<TonCell>,
}

/// Function5's bounced-request envelope. Invoke only for a message whose bounced
/// flag was checked. A recognized request removes its outgoing-id link and emits
/// BEAF9617 with the linked parent id; its original opcode is not filtered.
#[derive(Debug, Clone, PartialEq)]
pub enum DedustX1000WalletV2BounceMsg {
    FailedRequest {
        original_opcode: u32,
        outgoing_request_id: u64,
        ignored_suffix: TonCell,
    },
    /// Fewer than128bits or a marker other thanFFFFFFFF: function5 ignores it.
    Unknown(TonCell),
}
impl DedustX1000WalletV2BounceMsg {
    pub fn from_cell(cell: &TonCell) -> Result<Self, TonCoreError> {
        if cell.data_len_bits() < 128 {
            return Ok(Self::Unknown(cell.clone()));
        }
        let mut p = cell.parser();
        if p.read_num::<u32>(32)? != u32::MAX {
            return Ok(Self::Unknown(cell.clone()));
        }
        Ok(Self::FailedRequest {
            original_opcode: p.read_num(32)?,
            outgoing_request_id: p.read_num(64)?,
            ignored_suffix: p.read_remaining()?,
        })
    }
    pub fn to_cell(&self) -> Result<TonCell, TonCoreError> {
        match self {
            Self::Unknown(cell) => Ok(cell.clone()),
            Self::FailedRequest {
                original_opcode,
                outgoing_request_id,
                ignored_suffix,
            } => {
                let mut b = TonCell::builder();
                b.write_num(&u32::MAX, 32)?;
                b.write_num(original_opcode, 32)?;
                b.write_num(outgoing_request_id, 64)?;
                b.write_cell(ignored_suffix)?;
                b.build()
            },
        }
    }
}
