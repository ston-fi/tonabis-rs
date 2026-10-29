//! Schema records and authored protocol codecs. See the crate guide for evidence
//! and the explicit generator ownership boundary.
use crate::contracts::cpmm_v2::deposit::storage::DedustV2CpmmDepositConfig;
use crate::contracts::cpmm_v2::pool::{
    self,
    storage::{
        DedustV2CpmmBasicPayoutConfig, DedustV2CpmmExtendedPayoutConfig, DedustV2CpmmPartnerConfig,
        DedustV2CpmmPoolConfig, DedustV2CpmmPoolFees, DedustV2CpmmPoolReward, DedustV2CpmmPoolRewards,
        DedustV2CpmmPoolUpdatedState, DedustV2CpmmReferrerConfig, DedustV2CpmmStatus, DedustV2CpmmSwapFees,
        DedustV2CpmmSwapStep,
    },
};
use fastnum::U256;
use ton::ton_core::types::tlb_core::TLBEitherRef;
use ton::ton_core::{
    TLB,
    cell::TonCell,
    types::{
        TonAddress,
        tlb_core::{
            TLBCoins, TLBRef, VarLenBytes,
            adapters::{ConstLen, InternalAddress, NullableAddress},
        },
    },
};
use tonabis_core::adapters::maybe_ref_address::MaybeRefAddress;
/// Payment payload requesting a swap.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3292680207, bits_len = 32)]
pub struct DedustV2CpmmSwapPayloadMsg {
    pub minimal_amount_out: TLBCoins,
    /// Unix timestamp after which the swap must not execute.
    #[tlb(bits_len = 40)]
    pub deadline: u64,
    pub next: Option<TLBRef<Box<DedustV2CpmmSwapStep>>>,
    pub partner_config: Option<DedustV2CpmmPartnerConfig>,
    pub referrer_config: Option<DedustV2CpmmReferrerConfig>,
}
/// Payment payload requesting a liquidity deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3382711770, bits_len = 32)]
pub struct DedustV2CpmmDepositPayloadMsg {
    pub amount_x: TLBCoins,
    pub amount_y: TLBCoins,
    pub minimal_liquidity: TLBCoins,
    pub locked_liquidity_share: u16,
}
/// Payment payload funding a reward distribution.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 783842809, bits_len = 32)]
pub struct DedustV2CpmmFundRewardPayloadMsg {
    #[tlb(bits_len = 40)]
    pub till: u64,
}
/// Forward payload used by jetton transfer notifications.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3418569033, bits_len = 32)]
pub struct DedustV2CpmmPayJettonMsg {
    /// Referenced finite swap/deposit/reward operation. Known malformed bodies fail; unknown bytes remain preserved.
    pub payment_payload: TLBRef<crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmPoolPayloadMsg>,
    pub payout_config: TLBRef<DedustV2CpmmExtendedPayoutConfig>,
}
/// Incoming native-asset payment body.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2779237368, bits_len = 32)]
pub struct DedustV2CpmmPayNativeMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    /// Referenced finite swap/deposit/reward operation. Known malformed bodies fail; unknown bytes remain preserved.
    pub payment_payload: TLBRef<crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmPoolPayloadMsg>,
    pub payout_config: TLBRef<DedustV2CpmmExtendedPayoutConfig>,
}
/// Starts pool initialization.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3733193422, bits_len = 32)]
pub struct DedustV2CpmmInitPoolMsg {
    pub query_id: u64,
    pub custom_payload: Option<TLBRef<TonCell>>,
}
/// Wallet-resolution callback used during pool initialization.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3513996288, bits_len = 32)]
pub struct DedustV2CpmmTakeWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub wallet_address: TonAddress,
    #[tlb(adapter = "MaybeRefAddress")]
    pub owner_address: Option<TonAddress>,
}
/// TEP-74 transfer notification received from a pool-owned jetton wallet.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1935855772, bits_len = 32)]
pub struct DedustV2CpmmTransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    /// Typed Either inline/reference CBC33949 payment frame; caller payloads inside its payout config remain opaque.
    pub forward_payload: TLBEitherRef<crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmPoolJettonPayloadMsg>,
}
/// Finalizes a two-asset liquidity deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 623189728, bits_len = 32)]
pub struct DedustV2CpmmJoinLiquidityMsg {
    pub query_id: u64,
    /// Fixed deposit configuration loaded by the pool/child path, not caller-defined payload. See position/deposit captured-code replay.
    pub config: TLBRef<DedustV2CpmmDepositConfig>,
    pub amount_x: TLBCoins,
    pub amount_y: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub initiator_address: TonAddress,
}
/// Refunds an incomplete two-asset liquidity deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 220831250, bits_len = 32)]
pub struct DedustV2CpmmRefundDepositMsg {
    pub query_id: u64,
    /// Fixed deposit configuration loaded by the pool/child path, not caller-defined payload. See position/deposit captured-code replay.
    pub config: TLBRef<DedustV2CpmmDepositConfig>,
    pub amount_x: TLBCoins,
    pub amount_y: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub initiator_address: TonAddress,
}
/// Requests synchronization and payout of position fees.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1448276447, bits_len = 32)]
pub struct DedustV2CpmmClaimPositionFeesMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub excesses_to: TonAddress,
}
/// Position callback requesting actual fee payout.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 704584655, bits_len = 32)]
pub struct DedustV2CpmmPayoutPositionFeesMsg {
    pub query_id: u64,
    pub x_fees: TLBCoins,
    pub y_fees: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub excesses_to: TonAddress,
}
/// Requests protocol fee payout.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1100617651, bits_len = 32)]
pub struct DedustV2CpmmClaimProtocolFeesMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub to: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub excesses_to: TonAddress,
}
/// Requests creator fee payout.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3191746937, bits_len = 32)]
pub struct DedustV2CpmmClaimCreatorFeesMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub to: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub excesses_to: TonAddress,
}
/// Requests liquidity withdrawal.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 548794249, bits_len = 32)]
pub struct DedustV2CpmmWithdrawMsg {
    pub query_id: u64,
    pub liquidity: TLBCoins,
    pub minimal_x_out: TLBCoins,
    pub minimal_y_out: TLBCoins,
    pub auto_claim_fees: bool,
    pub payout_config: TLBRef<DedustV2CpmmBasicPayoutConfig>,
}
/// Position callback finalizing liquidity withdrawal.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 4143358890, bits_len = 32)]
pub struct DedustV2CpmmExitLiquidityMsg {
    pub query_id: u64,
    pub liquidity: TLBCoins,
    pub x_fees: TLBCoins,
    pub y_fees: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    pub minimal_amount_x: TLBCoins,
    pub minimal_amount_y: TLBCoins,
    pub payout_config: TLBRef<DedustV2CpmmBasicPayoutConfig>,
}
/// Requests a pool state snapshot.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 14209913, bits_len = 32)]
pub struct DedustV2CpmmProvidePoolStateMsg {
    pub query_id: u64,
    pub include_config: bool,
    pub include_fees: bool,
    pub include_code_hash: bool,
    pub include_rewards: bool,
}
/// Requests reward payout synchronization.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2426395493, bits_len = 32)]
pub struct DedustV2CpmmClaimRewardMsg {
    pub query_id: u64,
    #[tlb(bits_len = 2)]
    pub reward_index: u8,
    #[tlb(adapter = "InternalAddress")]
    pub excesses_to: TonAddress,
}
/// Position callback requesting actual reward payout.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2652371902, bits_len = 32)]
pub struct DedustV2CpmmPayoutRewardMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(bits_len = 2)]
    pub reward_index: u8,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub excesses_to: TonAddress,
}
/// Administrative request that replaces the pool implementation code.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 4068991113, bits_len = 32)]
pub struct DedustV2CpmmUpgradePoolCodeMsg {
    pub query_id: u64,
    pub next_code: TLBRef<TonCell>,
}
/// Wallet-address request emitted to an asset resolver.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 745978227, bits_len = 32)]
pub struct DedustV2CpmmProvideWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub owner_address: Option<TonAddress>,
    pub include_owner_address: bool,
}
/// Standard excess-value response.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3576854235, bits_len = 32)]
pub struct DedustV2CpmmExcessMsg {
    pub query_id: u64,
}
/// Initialization result callback emitted by the pool.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3457702871, bits_len = 32)]
pub struct DedustV2CpmmInitPoolResultMsg {
    pub query_id: u64,
    pub exit_code: i32,
    pub custom_payload: Option<TLBRef<TonCell>>,
}
/// Payout callback carrying amount, status, and an optional payload.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 840354313, bits_len = 32)]
pub struct DedustV2CpmmCallback3216ca09Msg {
    pub query_id: u64,
    pub amount0: TLBCoins,
    pub code: i32,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Position request to withdraw liquidity and optionally claim fees.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3155505577, bits_len = 32)]
pub struct DedustV2CpmmWithdrawLiquidityMsg {
    pub query_id: u64,
    pub liquidity: TLBCoins,
    pub minimal_x_out: TLBCoins,
    pub minimal_y_out: TLBCoins,
    /// Q120.120 numerator, encoded as varuint32.
    pub x_fee_per_token: VarLenBytes<U256, 5>,
    /// Q120.120 numerator, encoded as varuint32.
    pub y_fee_per_token: VarLenBytes<U256, 5>,
    pub rewards: DedustV2CpmmPoolRewards,
    pub auto_claim_fees: bool,
    pub payout_config: TLBRef<DedustV2CpmmBasicPayoutConfig>,
}
/// Position request crediting newly minted liquidity.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2237332669, bits_len = 32)]
pub struct DedustV2CpmmCreditLiquidityMsg {
    pub query_id: u64,
    pub liquidity: TLBCoins,
    pub locked_liquidity_share: u16,
    pub notify_extra_gas: TLBCoins,
    pub notify_payload: Option<TLBRef<TonCell>>,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_to: Option<TonAddress>,
    /// Q120.120 numerator, encoded as varuint32.
    pub x_fee_per_token: VarLenBytes<U256, 5>,
    /// Q120.120 numerator, encoded as varuint32.
    pub y_fee_per_token: VarLenBytes<U256, 5>,
    pub rewards: DedustV2CpmmPoolRewards,
}
/// Position request to claim all currently available fees.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 636589906, bits_len = 32)]
pub struct DedustV2CpmmClaimAvailableFeesMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_to: Option<TonAddress>,
    /// Q120.120 numerator, encoded as varuint32.
    pub x_fee_per_token: VarLenBytes<U256, 5>,
    /// Q120.120 numerator, encoded as varuint32.
    pub y_fee_per_token: VarLenBytes<U256, 5>,
}
/// Requested snapshot of pool state and selected optional sections.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2265617785, bits_len = 32)]
pub struct DedustV2CpmmTakePoolStateMsg {
    pub query_id: u64,
    pub status: DedustV2CpmmStatus,
    pub deposit_active: bool,
    pub swap_active: bool,
    pub liquidity: TLBCoins,
    pub reserve_x: TLBCoins,
    pub reserve_y: TLBCoins,
    pub config: Option<TLBRef<DedustV2CpmmPoolConfig>>,
    pub fees: Option<TLBRef<DedustV2CpmmPoolFees>>,
    pub rewards: Option<DedustV2CpmmPoolRewards>,
    pub code_hash: Option<U256>,
}
/// Position request to claim one reward program.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 772590778, bits_len = 32)]
pub struct DedustV2CpmmClaimPositionRewardMsg {
    pub query_id: u64,
    #[tlb(bits_len = 2)]
    pub reward_index: u8,
    pub reward: DedustV2CpmmPoolReward,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_to: Option<TonAddress>,
}
/// Result envelope emitted according to payout options.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3906684566, bits_len = 32)]
pub struct DedustV2CpmmPayoutPayloadMsg {
    pub exit_code: i32,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Outgoing swap event.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2028444580, bits_len = 32)]
pub struct DedustV2CpmmSwapEventMsg {
    pub x_to_y: bool,
    pub amount_in: TLBCoins,
    pub amount_out: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub initiator_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub recipient_address: TonAddress,
    pub next_state: TLBRef<DedustV2CpmmPoolUpdatedState>,
    pub fees: TLBRef<DedustV2CpmmSwapFees>,
}
/// Outgoing liquidity deposit event emitted by the pool.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 903818770, bits_len = 32)]
pub struct DedustV2CpmmDepositEventMsg {
    pub amount_x_in: TLBCoins,
    pub amount_y_in: TLBCoins,
    pub liquidity: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub initiator_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub recipient_address: TonAddress,
    pub next_state: TLBRef<DedustV2CpmmPoolUpdatedState>,
}
/// Outgoing liquidity withdrawal event emitted by the pool.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3235347284, bits_len = 32)]
pub struct DedustV2CpmmWithdrawalEventMsg {
    pub liquidity: TLBCoins,
    pub amount_x_out: TLBCoins,
    pub amount_y_out: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub initiator_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub recipient_address: TonAddress,
    pub next_state: TLBRef<DedustV2CpmmPoolUpdatedState>,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustV2CpmmIncomingMsg {
    InitPool(Box<DedustV2CpmmInitPoolMsg>),
    TakeWalletAddress(Box<DedustV2CpmmTakeWalletAddressMsg>),
    PayNative(Box<DedustV2CpmmPayNativeMsg>),
    TransferNotification(Box<DedustV2CpmmTransferNotificationMsg>),
    JoinLiquidity(Box<DedustV2CpmmJoinLiquidityMsg>),
    RefundDeposit(Box<DedustV2CpmmRefundDepositMsg>),
    ClaimPositionFees(Box<DedustV2CpmmClaimPositionFeesMsg>),
    PayoutPositionFees(Box<DedustV2CpmmPayoutPositionFeesMsg>),
    ClaimProtocolFees(Box<DedustV2CpmmClaimProtocolFeesMsg>),
    ClaimCreatorFees(Box<DedustV2CpmmClaimCreatorFeesMsg>),
    Withdraw(Box<DedustV2CpmmWithdrawMsg>),
    ExitLiquidity(Box<DedustV2CpmmExitLiquidityMsg>),
    ProvidePoolState(Box<DedustV2CpmmProvidePoolStateMsg>),
    ClaimReward(Box<DedustV2CpmmClaimRewardMsg>),
    PayoutReward(Box<DedustV2CpmmPayoutRewardMsg>),
    UpgradePoolCode(Box<DedustV2CpmmUpgradePoolCodeMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustV2CpmmOutgoingMsg {
    ProvideWalletAddress(Box<DedustV2CpmmProvideWalletAddressMsg>),
    Excess(Box<DedustV2CpmmExcessMsg>),
    InitPoolResult(Box<DedustV2CpmmInitPoolResultMsg>),
    Callback3216ca09(Box<DedustV2CpmmCallback3216ca09Msg>),
    PayoutPayload(Box<DedustV2CpmmPayoutPayloadMsg>),
    CreditLiquidity(Box<DedustV2CpmmCreditLiquidityMsg>),
    ClaimAvailableFees(Box<DedustV2CpmmClaimAvailableFeesMsg>),
    WithdrawLiquidity(Box<DedustV2CpmmWithdrawLiquidityMsg>),
    TakePoolState(Box<DedustV2CpmmTakePoolStateMsg>),
    ClaimPositionReward(Box<DedustV2CpmmClaimPositionRewardMsg>),
    SwapEvent(Box<DedustV2CpmmSwapEventMsg>),
    DepositEvent(Box<DedustV2CpmmDepositEventMsg>),
    WithdrawalEvent(Box<DedustV2CpmmWithdrawalEventMsg>),
    Unknown(TonCell),
}

// Fixed payment discriminators omitted as unreachable aliases from the catalog
// ABI JSON. Known malformed records fail; unknown bodies remain byte-exact.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustV2CpmmPoolPayloadMsg {
    Swap(Box<pool::msgs::DedustV2CpmmSwapPayloadMsg>),
    Deposit(Box<pool::msgs::DedustV2CpmmDepositPayloadMsg>),
    FundReward(Box<pool::msgs::DedustV2CpmmFundRewardPayloadMsg>),
    Unknown(TonCell),
}

/// TEP-74 notification's Either-body discriminator. Its known protocol frame is
/// CBC33949; preserving an unknown body does not assert protocol acceptance.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustV2CpmmPoolJettonPayloadMsg {
    Payment(Box<pool::msgs::DedustV2CpmmPayJettonMsg>),
    Unknown(TonCell),
}
