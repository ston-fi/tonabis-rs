//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use fastnum::U256;
use std::collections::HashMap;
use ton::{
    tlb_adapters::{DictKeyAdapterTonHash, DictKeyAdapterUint, DictValAdapterTLB, TLBHashMapE},
    ton_core::{
        TLB,
        cell::{CellBuilder, CellParser, TonCell, TonHash},
        errors::TonCoreError,
        traits::tlb::TLB,
        types::{
            TonAddress,
            tlb_core::{
                TLBCoins, TLBRef, VarLenBytes,
                adapters::{ConstLen, InternalAddress, NullableAddress},
            },
        },
    },
};
/// Pool has not started initialization.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0, bits_len = 2)]
pub struct DedustV2CpmmStatusUninitialized {}
/// Pool is initializing under the specified initiator.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 2)]
pub struct DedustV2CpmmStatusInitializing {
    #[tlb(adapter = "InternalAddress")]
    pub initiator_address: TonAddress,
}
/// Pool initialization is complete.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2, bits_len = 2)]
pub struct DedustV2CpmmStatusInitialized {}
/// Persistent pool lifecycle state.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustV2CpmmStatus {
    Uninitialized(DedustV2CpmmStatusUninitialized),
    Initializing(DedustV2CpmmStatusInitializing),
    Initialized(DedustV2CpmmStatusInitialized),
}
/// Root persistent storage shared by CPMM v2 PoolV1 and PoolV2.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmStorage {
    pub config: TLBRef<DedustV2CpmmPoolConfig>,
    pub fees: TLBRef<DedustV2CpmmPoolFees>,
    pub extra: TLBRef<DedustV2CpmmPoolExtra>,
    pub rewards: DedustV2CpmmPoolRewards,
    pub status: DedustV2CpmmStatus,
    pub deposit_active: bool,
    pub swap_active: bool,
    pub liquidity: TLBCoins,
    pub reserve_x: TLBCoins,
    pub reserve_y: TLBCoins,
}
/// Single multi-hop swap step.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmSwapStep {
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    pub minimal_amount_out: TLBCoins,
    /// Unix timestamp after which this step must not execute.
    #[tlb(bits_len = 40)]
    pub deadline: u64,
    pub next: Option<TLBRef<Box<DedustV2CpmmSwapStep>>>,
}
/// Partner attribution configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmPartnerConfig {
    pub partner_id: U256,
    /// Partner fee in basis points.
    pub partner_fee_bps: u16,
}
/// Referrer attribution configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmReferrerConfig {
    pub referrer_id: U256,
    /// Referrer fee in basis points.
    pub referrer_fee_bps: u16,
}
/// Strategy for swap fee collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DedustV2CpmmFeeIn {
    /// Fees are collected in the input asset.
    Both,
    /// Fees are collected in asset X.
    AssetX,
    /// Fees are collected in asset Y.
    AssetY,
}
impl TLB for DedustV2CpmmFeeIn {
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        match p.read_num::<u64>(2)? {
            0 => Ok(Self::Both),
            1 => Ok(Self::AssetX),
            2 => Ok(Self::AssetY),
            tag => Err(crate::contracts::utils::invalid_tag("DedustV2CpmmFeeIn", tag)),
        }
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        let tag = match self {
            Self::Both => 0u64,
            Self::AssetX => 1u64,
            Self::AssetY => 2u64,
        };
        b.write_num(&tag, 2)
    }
}
/// Asset identifier used by CPMM v2; `addr_none` denotes native GRAM.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmAsset {
    #[tlb(adapter = "NullableAddress")]
    pub value: Option<TonAddress>,
}
/// Pool immutable configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmPoolConfig {
    pub asset_x: DedustV2CpmmAsset,
    pub asset_y: DedustV2CpmmAsset,
    #[tlb(adapter = "NullableAddress")]
    pub creator_address: Option<TonAddress>,
    /// Base swap fee in basis points.
    pub base_fee_bps: u16,
    /// Creator fee in basis points.
    pub creator_fee_bps: u16,
    pub deposit_activation: Option<TLBRef<DedustV2CpmmActivation>>,
    pub swap_activation: Option<TLBRef<DedustV2CpmmActivation>>,
    pub fee_in: DedustV2CpmmFeeIn,
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterTonHash, crate::contracts::utils::DictValueInternalAddress>::new(256)"
    )]
    pub custom_resolvers: HashMap<TonHash, TonAddress>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u8>, DictValAdapterTLB<DedustV2CpmmRewardConfig>>::new(2)")]
    pub allowed_rewards: HashMap<u8, DedustV2CpmmRewardConfig>,
}
/// Time-based activation rule.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 8)]
pub struct DedustV2CpmmActivationTime {
    /// Unix timestamp from which the operation is active.
    #[tlb(bits_len = 40)]
    pub active_since: u64,
}
/// Initiator-based activation rule.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2, bits_len = 8)]
pub struct DedustV2CpmmActivationInitiator {
    #[tlb(adapter = "InternalAddress")]
    pub initiator_address: TonAddress,
}
/// Pool activation rule.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustV2CpmmActivation {
    Time(DedustV2CpmmActivationTime),
    Initiator(DedustV2CpmmActivationInitiator),
}
/// Reward program configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmRewardConfig {
    pub asset: DedustV2CpmmAsset,
    #[tlb(bits_len = 40)]
    pub max_duration: u64,
}
/// Active reward programs by reward index.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmPoolRewards {
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u8>, DictValAdapterTLB<DedustV2CpmmPoolReward>>::new(2)")]
    pub entries: HashMap<u8, DedustV2CpmmPoolReward>,
}
/// Active reward state.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmPoolReward {
    #[tlb(bits_len = 40)]
    pub remaining_time: u64,
    pub remaining_budget: TLBCoins,
    /// Q120.120 numerator, encoded as varuint32.
    pub rewards_per_token: VarLenBytes<U256, 5>,
    #[tlb(bits_len = 40)]
    pub last_update: u64,
}
/// Accumulated pool fee state.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmPoolFees {
    pub protocol_fee_x: TLBCoins,
    pub protocol_fee_y: TLBCoins,
    pub creator_fee_x: TLBCoins,
    pub creator_fee_y: TLBCoins,
    /// Q120.120 numerator, encoded as varuint32.
    pub x_fee_per_token: VarLenBytes<U256, 5>,
    /// Q120.120 numerator, encoded as varuint32.
    pub y_fee_per_token: VarLenBytes<U256, 5>,
}
/// Wallet and asset lookup dictionaries maintained by the pool.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmPoolWallets {
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterTonHash, crate::contracts::utils::DictValueInternalAddress>::new(256)"
    )]
    pub wallets_by_assets: HashMap<TonHash, TonAddress>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB<DedustV2CpmmAsset>>::new(256)")]
    pub assets_by_wallets: HashMap<TonHash, DedustV2CpmmAsset>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB<DedustV2CpmmAsset>>::new(256)")]
    pub resolutions: HashMap<TonHash, DedustV2CpmmAsset>,
}
/// Controller and wallet-resolution state referenced by root storage.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmPoolExtra {
    #[tlb(adapter = "InternalAddress")]
    pub controller: TonAddress,
    pub wallets: DedustV2CpmmPoolWallets,
}
/// Compact post-operation pool state used in events.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmPoolUpdatedState {
    pub reserve_x: TLBCoins,
    pub reserve_y: TLBCoins,
    pub liquidity: TLBCoins,
    pub fees: TLBRef<DedustV2CpmmPoolFees>,
}
/// Fee breakdown emitted with swap events.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmSwapFees {
    /// Whether fees are denominated in asset X.
    pub is_x: bool,
    pub lp_fee: TLBCoins,
    pub creator_fee: TLBCoins,
    pub protocol_fee: TLBCoins,
    pub partner_fee: TLBCoins,
    pub referrer_fee: TLBCoins,
}
/// Full payout configuration for fulfillment and rejection paths.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmExtendedPayoutConfig {
    pub fulfill: DedustV2CpmmPayoutOptions,
    pub reject: DedustV2CpmmPayoutOptions,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_to: Option<TonAddress>,
}
/// Single-path payout configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmBasicPayoutConfig {
    pub options: DedustV2CpmmPayoutOptions,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_to: Option<TonAddress>,
}
/// Payout destination and notification options.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmPayoutOptions {
    #[tlb(adapter = "NullableAddress")]
    pub destination: Option<TonAddress>,
    /// Extra gas attached to the downstream notification.
    pub extra_gas: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
    pub wrap_payload: bool,
}
/// CPMM v2 pool exit codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DedustV2CpmmPoolErrors {
    /// Pool is not in the uninitialized state.
    NotUninitialized = 20,
    /// Pool is not in the initializing state.
    NotInitializing = 21,
    /// Pool has not completed initialization.
    NotInitialized = 22,
    /// Sender is not authorized for the requested operation.
    SenderUnauthorized = 23,
    /// Wallet resolver returned an unsupported address.
    WalletCannotBeResolved = 24,
    /// Attached message value is too low for gas and forwarding.
    MessageValueTooLow = 25,
    /// Pool initialization used duplicate assets.
    DuplicateOfAssets = 26,
    /// Payment asset is unrelated to this pool.
    UnrelatedAsset = 28,
    /// Input value is lower than required.
    ValueTooLow = 29,
    /// Slippage limit was exceeded.
    SlippageExceeded = 30,
    /// Asset type is not supported.
    AssetNotSupported = 31,
    /// Address is in an unexpected workchain.
    WrongWorkchain = 32,
    /// Operation deadline has passed.
    DeadlineExceeded = 33,
    /// Pool has zero liquidity.
    ZeroLiquidity = 34,
    /// Creator fee exceeds the allowed maximum.
    MaxCreatorFeeExceeded = 35,
    /// Position or pool liquidity is insufficient.
    NotEnoughLiquidity = 36,
    /// Swaps are not active.
    SwapNotActive = 37,
    /// Deposits are not active.
    DepositNotActive = 38,
    /// Jetton wallet is already registered.
    WalletAlreadyRegistered = 39,
    /// Jetton wallet is not registered.
    WalletNotRegistered = 40,
    /// Wallet-resolution callback came from an unknown resolver.
    UnknownResolver = 41,
    /// Reward asset is not allowed.
    RewardIsNotAllowed = 42,
    /// Reward duration exceeds the configured maximum.
    MaxRewardDurationExceeded = 43,
    /// Reward duration is zero.
    ZeroDurationReward = 44,
    /// Reward top-up would dilute the current reward rate.
    RewardBudgetDilution = 45,
    /// Reward program is not active.
    RewardIsNotActive = 46,
    /// Swap or deposit input amount is zero.
    ZeroInputAmount = 47,
    /// Tagged-union or forwarded-payload layout check failed.
    InvalidTag = 63,
    /// Fallback rejection for unsupported operation codes.
    UnknownOperation = 65535,
}
