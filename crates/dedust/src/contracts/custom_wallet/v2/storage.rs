//! X1000 V2 persistent state, runtime dictionaries and continuation contexts.
//! Types and their codecs are authored together; the generator preserves this file.
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2CallbackPayloadMsg;
use fastnum::U256;
use std::collections::HashMap;
use ton::block_tlb::Msg;
use ton::{
    tlb_adapters::{DictKeyAdapterUint, DictValAdapterNum, DictValAdapterTLB, TLBHashMapE},
    ton_core::{
        TLB,
        cell::{CellBuilder, CellParser, TonCell},
        errors::TonCoreError,
        traits::tlb::TLB as Tlb,
        types::{
            TonAddress,
            tlb_core::{
                MsgAddressIntStd, TLBCoins, TLBRef,
                adapters::{ConstLen, InternalAddress, NullableAddress},
            },
        },
    },
};
/// Persistent wallet state with source-defined runtime dictionaries and action configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000WalletV2Storage {
    pub storage_version: u8,
    pub counter0: u32,
    /// Sequence number consumed and incremented by accepted signed requests.
    pub seqno: u32,
    pub public_key: U256,
    /// Typed runtime dictionary envelopes and source-selected continuation contexts; see the independent branch replay evidence.
    pub runtime_state: TLBRef<DedustX1000WalletV2RuntimeState>,
    /// Executable implementation dispatch code invoked with BLESS/CALLXARGS, preserved as code. Not application state or a substitute for a missing record.
    pub implementation_code: TLBRef<TonCell>,
}
/// DeDust swap direction that treats the supplied amount as input.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0, bits_len = 1)]
pub struct DedustX1000GivenIn {}
/// DeDust swap direction that treats the requested amount as output.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 1)]
pub struct DedustX1000GivenOut {}
/// DeDust swap direction variants.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustX1000SwapKind {
    GivenIn(DedustX1000GivenIn),
    GivenOut(DedustX1000GivenOut),
}
/// One DeDust swap route step.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000SwapStep {
    #[tlb(adapter = "InternalAddress")]
    pub pool_address: TonAddress,
    /// Exact-input or exact-output mode.
    pub kind: DedustX1000SwapKind,
    pub limit: TLBCoins,
    pub next: Option<TLBRef<Box<DedustX1000SwapStep>>>,
}
/// DeDust whole-swap routing options.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000SwapParams {
    /// Unix deadline, or zero when disabled.
    pub deadline: u32,
    #[tlb(adapter = "NullableAddress")]
    pub recipient_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub referral_address: Option<TonAddress>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// STON.fi V2 additional swap parameters.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000StonfiV2SwapBody {
    /// Minimum accepted output amount in token base units.
    pub min_out: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub receiver: Option<TonAddress>,
    /// Nanograms forwarded with a success notification.
    pub forward_gas: TLBCoins,
    pub custom_payload: Option<TLBRef<TonCell>>,
    /// Nanograms forwarded with a refund notification.
    pub refund_forward_gas: TLBCoins,
    pub refund_payload: Option<TLBRef<TonCell>>,
    /// Referral fee in STON.fi fee units.
    pub referral_fee: u16,
    #[tlb(adapter = "NullableAddress")]
    pub referral_address: Option<TonAddress>,
}
/// TONCO multi-hop callback configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000ToncoMultihop {
    #[tlb(adapter = "InternalAddress")]
    pub target_address: TonAddress,
    /// Nanograms forwarded after success.
    pub fulfill_forward_amount: TLBCoins,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    /// Nanograms forwarded after rejection.
    pub reject_forward_amount: TLBCoins,
    pub reject_payload: Option<TLBRef<TonCell>>,
    #[tlb(adapter = "InternalAddress")]
    pub excess_address: TonAddress,
}
/// Optional CPMM route continuation.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000CpmmNextStep {
    #[tlb(adapter = "InternalAddress")]
    pub pool_address: TonAddress,
    pub min_out: TLBCoins,
    /// Unix deadline for the next step.
    #[tlb(bits_len = 40)]
    pub deadline: u64,
    pub next: Option<TLBRef<Box<DedustX1000CpmmNextStep>>>,
}
/// CPMM partner fee attribution.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000CpmmPartnerConfig {
    pub partner_id: U256,
    /// Partner fee in basis points.
    pub fee_bps: u16,
}
/// CPMM referrer fee attribution.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000CpmmReferrerConfig {
    pub referrer_id: U256,
    /// Referrer fee in basis points.
    pub fee_bps: u16,
}
/// CPMM payout path options.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000CpmmPayoutOptions {
    #[tlb(adapter = "NullableAddress")]
    pub destination: Option<TonAddress>,
    /// Additional downstream gas in nanograms.
    pub extra_gas: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
    pub wrap_payload: bool,
}
/// CPMM fulfillment and rejection payout configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000CpmmPayoutConfig {
    pub fulfill: DedustX1000CpmmPayoutOptions,
    pub reject: DedustX1000CpmmPayoutOptions,
    #[tlb(adapter = "NullableAddress")]
    pub excess_address: Option<TonAddress>,
}
/// Bidask DAMM optional referral data.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000BidaskDammReferral {
    #[tlb(adapter = "InternalAddress")]
    pub referral_address: TonAddress,
    /// Referral fee in Bidask fee units.
    pub referral_fee: u16,
}
/// Normal Bidask execution with a minimum output.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0, bits_len = 1)]
pub struct DedustX1000BidaskNormalSlippage {
    pub min_out: TLBCoins,
}
/// Partial Bidask execution with a price boundary.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 1)]
pub struct DedustX1000BidaskPartialSlippage {
    /// Q256 square-root price boundary.
    pub sqrt_price_limit: U256,
}
/// Bidask slippage variants.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustX1000BidaskSlippage {
    DedustX1000BidaskNormalSlippage(Box<DedustX1000BidaskNormalSlippage>),
    DedustX1000BidaskPartialSlippage(Box<DedustX1000BidaskPartialSlippage>),
}
/// Bidask CLMM swap origin and referral data.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000BidaskAdditionalData {
    #[tlb(adapter = "InternalAddress")]
    pub from_address: TonAddress,
    /// Optional referral; captured builder7 explicitly writes addr_none. Preserve null versus standard zero.
    #[tlb(adapter = "NullableAddress")]
    pub referral_address: Option<TonAddress>,
}
/// Coffee swap route step.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000CoffeeSwapStep {
    pub pool_address_hash: U256,
    pub min_out: TLBCoins,
    pub next: Option<TLBRef<Box<DedustX1000CoffeeSwapStep>>>,
}
/// Coffee callback destination and payload.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000CoffeeNotification {
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    /// Nanograms forwarded to the callback.
    pub forward_amount: TLBCoins,
    pub payload: TLBRef<TonCell>,
}
/// Coffee success and failure callback configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000CoffeeNotifications {
    pub fulfill: Option<TLBRef<DedustX1000CoffeeNotification>>,
    pub reject: Option<TLBRef<DedustX1000CoffeeNotification>>,
}
/// Coffee whole-swap options.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000CoffeeSwapParams {
    /// Unix deadline.
    pub deadline: u32,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub referral: Option<TonAddress>,
    pub notifications: Option<TLBRef<DedustX1000CoffeeNotifications>>,
}
/// Moon callback destination and optional body.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000MoonCallback {
    #[tlb(adapter = "NullableAddress")]
    pub recipient: Option<TonAddress>,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Moon swap limits and callback routing.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000MoonSwapParams {
    pub min_out: TLBCoins,
    /// Unix deadline.
    pub deadline: u64,
    #[tlb(adapter = "NullableAddress")]
    pub excess_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub referral_address: Option<TonAddress>,
    pub fulfill: Option<TLBRef<DedustX1000MoonCallback>>,
    pub reject: Option<TLBRef<DedustX1000MoonCallback>>,
}
/// Callback destination used by Moon CLMM routes.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000MoonClmmCallback {
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    /// Preserve recipient-only compact form emitted by captured builder7 and explicitly marked optional payload. Peer acceptance of compact form is not assumed.
    pub payload: DedustX1000WalletV2CallbackPayloadMsg,
}
/// DeDust Uranus partner attribution.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000UranusPartnerConfig {
    pub partner_id: U256,
    /// Partner fee in basis points.
    pub fee_bps: u16,
}
/// DeDust Uranus referrer attribution.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000UranusReferrerConfig {
    pub referrer_id: U256,
    /// Referrer fee in basis points.
    pub fee_bps: u16,
}
/// Wallet-specific exit codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DedustX1000WalletV2Errors {
    /// Signed-request sequence check failed.
    InvalidSeqno = 102,
    /// Referenced pending operation was not found.
    PendingOperationNotFound = 103,
    /// Stored layout revision is not supported by this implementation.
    UnsupportedStorageVersion = 104,
    /// Authorization or request validation failed.
    AuthorizationOrRequestValidationFailed = 555,
}

// X1000 V2 runtime dictionaries, reconciled from captured code 4609b587...
// and independently replayed callback/controller transitions. The third map
// holds empty markers or held outgoing messages. Captured callback/controller
// execution independently distinguishes the two fixed forms.

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000WalletV2RuntimeState {
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterUint<u64>, DictValAdapterTLB<DedustX1000WalletV2PendingContinuation>>::new(64)"
    )]
    pub pending_operations: HashMap<u64, DedustX1000WalletV2PendingContinuation>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u64>, DictValAdapterTLB<TLBCoins>>::new(64)")]
    pub reserved_values: HashMap<u64, TLBCoins>,
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterUint<u64>, DictValAdapterTLB<DedustX1000WalletV2ExecutionEntry>>::new(64)"
    )]
    pub execution_entries: HashMap<u64, DedustX1000WalletV2ExecutionEntry>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u64>, DictValAdapterNum<u64, 64>>::new(64)")]
    pub request_links: HashMap<u64, u64>,
}

/// Fixed dictionary3 value. Function8 replaces an empty marker with a held
/// canonical TON message. Controller opcode01010202 removes that message and
/// SENDRAWMSG forwards it. No signer/authority inference is made by decoding.
#[derive(Debug, Clone, PartialEq)]
pub enum DedustX1000WalletV2ExecutionEntry {
    Marker,
    StoredMessage(Box<TLBRef<Msg>>),
}
impl ton::ton_core::traits::tlb::TLB for DedustX1000WalletV2ExecutionEntry {
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        if p.data_bits_left()? != 0 {
            return Err(TonCoreError::Custom("X1000 execution entry has unexpected bits".into()));
        }
        let value = match p.refs_left() {
            0 => Self::Marker,
            1 => Self::StoredMessage(Box::new(TLBRef::read(p)?)),
            refs => {
                return Err(TonCoreError::Custom(format!(
                    "invalid X1000 execution entry reference count {refs}"
                )));
            },
        };

        Ok(value)
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        match self {
            Self::Marker => Ok(()),
            Self::StoredMessage(v) => v.write(b),
        }
    }
}

/// Source-fixed recursive continuation. Context selection depends on all three
/// header nibbles; fixed prefixes are typed separately from source-ignored tails.
#[derive(Debug, Clone, PartialEq)]
pub struct DedustX1000WalletV2PendingContinuation {
    pub operation_kind: u8,
    pub field4_0: u8,
    pub field4_1: u8,
    pub context: Option<DedustX1000WalletV2ContextRecord>,
    pub continuation: Option<TLBRef<Box<DedustX1000WalletV2PendingContinuation>>>,
    pub fallback: Option<TLBRef<Box<DedustX1000WalletV2PendingContinuation>>>,
}
impl ton::ton_core::traits::tlb::TLB for DedustX1000WalletV2PendingContinuation {
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        let operation_kind = p.read_num(4)?;
        let field4_0 = p.read_num(4)?;
        let field4_1 = p.read_num(4)?;
        let context = if p.read_bit()? {
            Some(DedustX1000WalletV2ContextRecord::from_cell(
                operation_kind,
                field4_0,
                field4_1,
                p.read_next_ref()?,
            )?)
        } else {
            None
        };
        let continuation = Option::<TLBRef<Box<Self>>>::read(p)?;
        let fallback = Option::<TLBRef<Box<Self>>>::read(p)?;

        Ok(Self {
            operation_kind,
            field4_0,
            field4_1,
            context,
            continuation,
            fallback,
        })
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        b.write_num(&self.operation_kind, 4)?;
        b.write_num(&self.field4_0, 4)?;
        b.write_num(&self.field4_1, 4)?;
        if let Some(context) = &self.context {
            context
                .body
                .validate_header(self.operation_kind, self.field4_0, self.field4_1)?;
            b.write_bit(true)?;
            b.write_ref(context.to_cell()?)?;
        } else {
            b.write_bit(false)?;
        }
        self.continuation.write(b)?;
        self.fallback.write(b)
    }
}

// Fixed continuation contexts consumed by captured X1000 V2 function7.
// Source7 loads one Coins/uint256, PLDREFIDX/LDREF, and fixed standard-address
// prefixes. Explicit DROP/PLD instructions ignore each remaining suffix; those
// bytes are preserved, not relabeled as an unknown fixed ABI. Local producer
// probes independently execute every supported branch and shorten each input.

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000WalletV2AddressPair {
    pub address0: MsgAddressIntStd,
    pub address1: MsgAddressIntStd,
    /// LDSTDADDR twice then DROP, rather than ENDS, in the pinned loader.
    pub ignored_suffix: TonCell,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000WalletV2Address {
    pub address: MsgAddressIntStd,
    /// LDSTDADDR then DROP in the pinned loader.
    pub ignored_suffix: TonCell,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000WalletV2AddressAndWord {
    pub address: MsgAddressIntStd,
    pub word: U256,
    /// PLDU256 reads the word without authenticating or consuming the suffix.
    pub ignored_suffix: TonCell,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000WalletV2CoinAndPair {
    pub amount: TLBCoins,
    pub addresses: TLBRef<DedustX1000WalletV2AddressPair>,
    /// PLDREFIDX0 leaves this source-ignored suffix intact.
    pub ignored_suffix: TonCell,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000WalletV2CoinAndTwoPairs {
    pub amount: TLBCoins,
    pub addresses: TLBRef<DedustX1000WalletV2AddressPair>,
    pub second_addresses: TLBRef<DedustX1000WalletV2AddressPair>,
    pub ignored_suffix: TonCell,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000WalletV2CoinAndPairAndAddress {
    pub amount: TLBCoins,
    pub addresses: TLBRef<DedustX1000WalletV2AddressPair>,
    pub second_address: TLBRef<DedustX1000WalletV2Address>,
    pub ignored_suffix: TonCell,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000WalletV2CoinAndPairAndWord {
    pub amount: TLBCoins,
    pub addresses: TLBRef<DedustX1000WalletV2AddressPair>,
    pub second_address: TLBRef<DedustX1000WalletV2AddressAndWord>,
    pub ignored_suffix: TonCell,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000WalletV2WordAndPairAndAddress {
    pub word: U256,
    pub addresses: TLBRef<DedustX1000WalletV2AddressPair>,
    pub second_address: TLBRef<DedustX1000WalletV2Address>,
    pub ignored_suffix: TonCell,
}
/// Concrete protocol branches are identified by the emitted fixed payload tags.
/// Positional header/scalar names remain explicit where meanings are unresolved. Every active fixed branch has a typed prefix. Contexts
/// on the source's no-dispatch paths are never loaded by function7; exact bytes
/// remain available under IgnoredByFixedBuilder, not a claim of extension support.
#[derive(Debug, Clone, PartialEq)]
pub enum DedustX1000WalletV2Context {
    Classic(DedustX1000WalletV2CoinAndTwoPairs),
    StonfiV2(DedustX1000WalletV2CoinAndTwoPairs),
    ToncoMode0(DedustX1000WalletV2CoinAndPair),
    ToncoMode1(DedustX1000WalletV2CoinAndPairAndAddress),
    Cpmm(DedustX1000WalletV2CoinAndPairAndAddress),
    BidaskDamm(DedustX1000WalletV2CoinAndPairAndAddress),
    Coffee(DedustX1000WalletV2CoinAndPairAndWord),
    Moon(DedustX1000WalletV2CoinAndPairAndAddress),
    PayloadFa45056b(DedustX1000WalletV2CoinAndPairAndAddress),
    MoonClmm(DedustX1000WalletV2WordAndPairAndAddress),
    BidaskClmm(DedustX1000WalletV2CoinAndPairAndAddress),
    StonksSell(DedustX1000WalletV2CoinAndPair),
    UranusSell(DedustX1000WalletV2CoinAndPair),
    JettonBurn(DedustX1000WalletV2CoinAndPair),
    IgnoredByFixedBuilder(TonCell),
}
// Header interpretation is shared by reading and writing continuations.
#[derive(PartialEq, Eq)]
enum ContextKind {
    Classic,
    StonfiV2,
    ToncoMode0,
    ToncoMode1,
    Cpmm,
    BidaskDamm,
    Coffee,
    Moon,
    PayloadFa45056b,
    MoonClmm,
    BidaskClmm,
    StonksSell,
    UranusSell,
    JettonBurn,
    IgnoredByFixedBuilder,
}
impl ContextKind {
    fn from_header(kind: u8, field4_0: u8, field4_1: u8) -> Self {
        if field4_0 != 1 || field4_1 > 1 {
            return Self::IgnoredByFixedBuilder;
        }
        match (kind, field4_1) {
            (1, _) => Self::Classic,
            (2, _) => Self::StonfiV2,
            (3, 0) => Self::ToncoMode0,
            (3, 1) => Self::ToncoMode1,
            (4, _) => Self::Cpmm,
            (5, _) => Self::BidaskDamm,
            (6, _) => Self::Coffee,
            (7, 0) => Self::Moon,
            (8, _) => Self::PayloadFa45056b,
            (9, _) => Self::MoonClmm,
            (10, _) => Self::BidaskClmm,
            (11, 0) => Self::StonksSell,
            (12, 0) => Self::UranusSell,
            (13, 0) => Self::JettonBurn,
            _ => Self::IgnoredByFixedBuilder,
        }
    }
}
impl DedustX1000WalletV2Context {
    /// Decode according to all three source header nibbles. The operation kind
    /// does not occur inside the referenced context cell itself.
    pub fn from_cell(kind: u8, field4_0: u8, field4_1: u8, cell: &TonCell) -> Result<Self, TonCoreError> {
        Ok(match ContextKind::from_header(kind, field4_0, field4_1) {
            ContextKind::Classic => Self::Classic(Tlb::from_cell(cell)?),
            ContextKind::StonfiV2 => Self::StonfiV2(Tlb::from_cell(cell)?),
            ContextKind::ToncoMode0 => Self::ToncoMode0(Tlb::from_cell(cell)?),
            ContextKind::ToncoMode1 => Self::ToncoMode1(Tlb::from_cell(cell)?),
            ContextKind::Cpmm => Self::Cpmm(Tlb::from_cell(cell)?),
            ContextKind::BidaskDamm => Self::BidaskDamm(Tlb::from_cell(cell)?),
            ContextKind::Coffee => Self::Coffee(Tlb::from_cell(cell)?),
            ContextKind::Moon => Self::Moon(Tlb::from_cell(cell)?),
            ContextKind::PayloadFa45056b => Self::PayloadFa45056b(Tlb::from_cell(cell)?),
            ContextKind::MoonClmm => Self::MoonClmm(Tlb::from_cell(cell)?),
            ContextKind::BidaskClmm => Self::BidaskClmm(Tlb::from_cell(cell)?),
            ContextKind::StonksSell => Self::StonksSell(Tlb::from_cell(cell)?),
            ContextKind::UranusSell => Self::UranusSell(Tlb::from_cell(cell)?),
            ContextKind::JettonBurn => Self::JettonBurn(Tlb::from_cell(cell)?),
            ContextKind::IgnoredByFixedBuilder => Self::IgnoredByFixedBuilder(cell.clone()),
        })
    }
    pub(crate) fn validate_header(&self, kind: u8, a: u8, b: u8) -> Result<(), TonCoreError> {
        let actual = match self {
            Self::Classic(_) => ContextKind::Classic,
            Self::StonfiV2(_) => ContextKind::StonfiV2,
            Self::ToncoMode0(_) => ContextKind::ToncoMode0,
            Self::ToncoMode1(_) => ContextKind::ToncoMode1,
            Self::Cpmm(_) => ContextKind::Cpmm,
            Self::BidaskDamm(_) => ContextKind::BidaskDamm,
            Self::Coffee(_) => ContextKind::Coffee,
            Self::Moon(_) => ContextKind::Moon,
            Self::PayloadFa45056b(_) => ContextKind::PayloadFa45056b,
            Self::MoonClmm(_) => ContextKind::MoonClmm,
            Self::BidaskClmm(_) => ContextKind::BidaskClmm,
            Self::StonksSell(_) => ContextKind::StonksSell,
            Self::UranusSell(_) => ContextKind::UranusSell,
            Self::JettonBurn(_) => ContextKind::JettonBurn,
            Self::IgnoredByFixedBuilder(_) => ContextKind::IgnoredByFixedBuilder,
        };
        if actual == ContextKind::from_header(kind, a, b) {
            Ok(())
        } else {
            Err(TonCoreError::Custom(
                "X1000 continuation context disagrees with fixed header".into(),
            ))
        }
    }
    pub fn to_cell(&self) -> Result<TonCell, TonCoreError> {
        match self {
            Self::Classic(v) | Self::StonfiV2(v) => v.to_cell(),
            Self::ToncoMode0(v) | Self::StonksSell(v) | Self::UranusSell(v) | Self::JettonBurn(v) => v.to_cell(),
            Self::ToncoMode1(v)
            | Self::Cpmm(v)
            | Self::BidaskDamm(v)
            | Self::Moon(v)
            | Self::PayloadFa45056b(v)
            | Self::BidaskClmm(v) => v.to_cell(),
            Self::Coffee(v) => v.to_cell(),
            Self::MoonClmm(v) => v.to_cell(),
            Self::IgnoredByFixedBuilder(v) => Ok(v.clone()),
        }
    }
}

/// Persistent context envelope. The fixed callback callers CTOS/LDGRAMS/NIP
/// before invoking source builder7. The leading Coins field is therefore part
/// of the stored ABI even though builder7 does not receive it. Its broader
/// application meaning is not inferred from the current account captures.
#[derive(Debug, Clone, PartialEq)]
pub struct DedustX1000WalletV2ContextRecord {
    pub prefix_amount: TLBCoins,
    pub body: DedustX1000WalletV2Context,
}
impl DedustX1000WalletV2ContextRecord {
    pub fn from_cell(kind: u8, a: u8, b: u8, cell: &TonCell) -> Result<Self, TonCoreError> {
        let mut p = cell.parser();
        let prefix_amount = TLBCoins::read(&mut p)?;
        let body = DedustX1000WalletV2Context::from_cell(kind, a, b, &p.read_remaining()?)?;
        Ok(Self { prefix_amount, body })
    }
    pub fn to_cell(&self) -> Result<TonCell, TonCoreError> {
        let mut b = TonCell::builder();
        self.prefix_amount.write(&mut b)?;
        b.write_cell(&self.body.to_cell()?)?;
        b.build()
    }
}
