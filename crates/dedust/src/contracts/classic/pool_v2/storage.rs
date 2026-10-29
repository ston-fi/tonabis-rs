//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use crate::contracts::classic::asset::DedustClassicAsset;
use ton::ton_core::{
    TLB,
    cell::TonCell,
    types::{
        TonAddress,
        tlb_core::{
            TLBCoins, TLBRef,
            adapters::{InternalAddress, NullableAddress},
        },
    },
};
/// Runtime Pool storage encoded in the root persistent-data cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV2Storage {
    pub proof: TLBRef<DedustPoolV2PoolProof>,
    pub proof_code: TLBRef<TonCell>,
    pub lp_wallet_code: TLBRef<TonCell>,
    pub config: TLBRef<DedustPoolV2Config>,
    pub version: u16,
    /// Trade fee numerator returned by `get_trade_fee`; denominator is fixed at 10000.
    pub trade_fee_numerator: u16,
    pub total_supply: TLBCoins,
    pub reserve0: TLBCoins,
    pub reserve1: TLBCoins,
    /// Accumulated protocol fee in asset0 units.
    pub asset0_protocol_fee: TLBCoins,
    /// Accumulated protocol fee in asset1 units.
    pub asset1_protocol_fee: TLBCoins,
    /// Stored stable normalization factor0; source numeric getter111021 and staged swap math, not a cumulative price.
    pub normalization_factor0: u128,
    /// Stored stable normalization factor1; source numeric getter111021 and staged swap math, not a cumulative price.
    pub normalization_factor1: u128,
}
/// Referenced Pool configuration cell loaded by safe getters.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV2Config {
    pub asset0_decimals: u8,
    pub asset1_decimals: u8,
    #[tlb(adapter = "InternalAddress")]
    pub asset0_vault_addr: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub asset1_vault_addr: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub fee_collector_addr: Option<TonAddress>,
    /// Optional fixed provider pair, not an arbitrary dictionary; selector66722.
    pub quote_provider_data: Option<TLBRef<DedustPoolV2QuoteProviders>>,
}
/// Volatile curve tag used in pool parameters and proofs.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0, bits_len = 1)]
pub struct DedustPoolV2VolatilePool {}
/// Stable curve tag used in pool parameters and proofs.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 1)]
pub struct DedustPoolV2StablePool {}
/// One-bit pool type, represented by docs as volatile/stable and by Toncenter as `is_stable`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustPoolV2PoolType {
    Volatile(DedustPoolV2VolatilePool),
    Stable(DedustPoolV2StablePool),
}
/// Pool identity tuple embedded in pool proofs and liquidity-deposit requests.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV2PoolParams {
    pub pool_type: DedustPoolV2PoolType,
    pub asset0: DedustClassicAsset,
    pub asset1: DedustClassicAsset,
}
/// Contract-type tag proving a sender is a DeDust Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 8)]
pub struct DedustPoolV2VaultContractType {}
/// Contract-type tag proving a sender is a DeDust Pool.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2, bits_len = 8)]
pub struct DedustPoolV2PoolContractType {}
/// Contract-type tag proving a sender is a DeDust Liquidity Deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3, bits_len = 8)]
pub struct DedustPoolV2LiquidityDepositContractType {}
/// Vault proof body referenced by swap-external messages.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV2VaultProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_addr: TonAddress,
    pub contract_type: DedustPoolV2VaultContractType,
    pub params: DedustClassicAsset,
}
/// Pool proof body referenced by peer swaps, pool payouts, and liquidity callbacks.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV2PoolProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_addr: TonAddress,
    pub contract_type: DedustPoolV2PoolContractType,
    pub params: DedustPoolV2PoolParams,
}
/// Referenced proof parameters for a Liquidity Deposit contract.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV2LiquidityDepositProofParams {
    #[tlb(adapter = "InternalAddress")]
    pub owner_addr: TonAddress,
    pub pool_params: DedustPoolV2PoolParams,
}
/// Liquidity Deposit proof body referenced by deposit-liquidity-all messages.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV2LiquidityDepositProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_addr: TonAddress,
    pub contract_type: DedustPoolV2LiquidityDepositContractType,
    pub params: TLBRef<DedustPoolV2LiquidityDepositProofParams>,
}
/// Exact-input swap kind that fixes the supplied input amount.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0, bits_len = 1)]
pub struct DedustPoolV2GivenIn {}
/// Exact-output swap kind listed in docs but not used by the SDK examples.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 1)]
pub struct DedustPoolV2GivenOut {}
/// One-bit swap kind embedded in each swap step.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustPoolV2SwapKind {
    GivenIn(DedustPoolV2GivenIn),
    GivenOut(DedustPoolV2GivenOut),
}
/// Parameters for the current step of a routed swap.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV2SwapStepParams {
    pub kind: DedustPoolV2SwapKind,
    pub limit: TLBCoins,
    pub next: Option<TLBRef<Box<DedustPoolV2SwapStep>>>,
}
/// Full next-hop swap step referenced from step parameters.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV2SwapStep {
    #[tlb(adapter = "InternalAddress")]
    pub pool_addr: TonAddress,
    pub params: DedustPoolV2SwapStepParams,
}
/// Whole-swap options stored in the required referenced swap-params cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV2SwapParams {
    /// Unix timestamp deadline, or zero when disabled.
    pub deadline: u32,
    #[tlb(adapter = "NullableAddress")]
    pub recipient_addr: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub referral_addr: Option<TonAddress>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Referenced amounts cell inside `deposit_liquidity_all`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV2DepositLiquidityAllAmounts {
    pub asset0: DedustClassicAsset,
    pub asset0_amount: TLBCoins,
    pub asset1: DedustClassicAsset,
    pub asset1_amount: TLBCoins,
}
/// Optional Pool asset pair included in a state reply.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV2PoolStateAssets {
    pub asset0: DedustClassicAsset,
    pub asset1: DedustClassicAsset,
}
/// Referenced tail for swap external event details.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV2SwapEventTail {
    #[tlb(adapter = "InternalAddress")]
    pub sender_addr: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub referral_addr: Option<TonAddress>,
    pub reserve0: TLBCoins,
    pub reserve1: TLBCoins,
}
/// DeDust Pool v2 exit-code catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DedustPoolV2Errors {
    /// LP burn notification sender is not the deterministic LP wallet.
    InvalidLpWalletSender = 257,
    /// Control message sender is not the stored Factory/proof address.
    InvalidControlSender = 259,
    /// Asset tag or proof shape is invalid for this Pool.
    InvalidAssetOrProof = 261,
    /// Attached value cannot cover the required downstream fee budget.
    InsufficientValue = 263,
    /// Swap-external sender is not the authenticated Vault.
    InvalidVaultSender = 264,
    /// Swap-peer sender is not the authenticated Pool.
    InvalidPoolSender = 265,
    /// Liquidity-deposit callback sender is not the authenticated deposit contract.
    InvalidLiquidityDepositSender = 272,
    /// Trade fee is outside the accepted range.
    InvalidTradeFee = 279,
    /// Message token wallet does not match either Pool asset wallet.
    InvalidTokenWallet = 288,
    /// Payout target address is absent.
    MissingPayoutTarget = 289,
    /// Withdrawal branch does not match the requested asset side.
    InvalidWithdrawalAsset = 290,
    /// Requested asset does not match either Pool asset.
    InvalidRequestAsset = 291,
    /// Requested amount must be positive.
    InvalidAmount = 292,
    /// Protocol-fee withdrawal exceeds accumulated fees.
    ProtocolFeeExceeded = 293,
    /// Pool has no LP supply for swap pricing.
    EmptyLiquidity = 294,
    /// Deterministic child-contract sender does not match the expected address.
    InvalidChildSender = 296,
    /// Swap output is below the requested minimum.
    SlippageExceeded = 304,
    /// Swap deadline has passed.
    DeadlineExceeded = 305,
    /// Incoming internal message opcode is unsupported.
    UnsupportedOperation = 65535,
}
/// Fixed optional provider pair loaded by numeric getter66722 and quote configuration paths; not an arbitrary dictionary.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV2QuoteProviders {
    #[tlb(adapter = "NullableAddress")]
    pub provider0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub provider1: Option<TonAddress>,
}
