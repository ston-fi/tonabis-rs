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
/// Runtime storage loaded by the verified native vault code root data cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultNativeV2Storage {
    pub cached_proof: TLBRef<DedustVaultNativeV2ProofData>,
    #[tlb(adapter = "InternalAddress")]
    pub factory_addr: TonAddress,
    pub blank_code: TLBRef<TonCell>,
    pub version: u16,
    pub native_balance: TLBCoins,
}
/// Cached DeDust proof data stored by the Native Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultNativeV2ProofData {
    #[tlb(adapter = "InternalAddress")]
    pub factory_addr: TonAddress,
    pub contract_type: u8,
    pub asset: DedustClassicAsset,
}
/// Volatile pool type tag used in PoolParams.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0, bits_len = 1)]
pub struct DedustVolatilePool {}
/// Stable pool type tag used in PoolParams.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 1)]
pub struct DedustStablePool {}
/// Pool type variants encoded as one bit in DeDust pool parameters.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustPoolType {
    Volatile(DedustVolatilePool),
    Stable(DedustStablePool),
}
/// Pool identity tuple used by native deposit liquidity and factory routing.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultNativeV2PoolParams {
    pub pool_type: DedustPoolType,
    pub asset0: DedustClassicAsset,
    pub asset1: DedustClassicAsset,
}
/// Swap kind tag for exact-in swaps; SDK currently stores only this value.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0, bits_len = 1)]
pub struct DedustGivenIn {}
/// Swap kind tag for exact-out swaps; docs say this variant is not implemented.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 1)]
pub struct DedustGivenOut {}
/// One-bit swap direction encoded inside a swap step.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustSwapKind {
    GivenIn(DedustGivenIn),
    GivenOut(DedustGivenOut),
}
/// Per-step swap parameters embedded after the Pool address.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustSwapStepParams {
    pub kind: DedustSwapKind,
    pub limit: TLBCoins,
    pub next: Option<TLBRef<Box<DedustSwapStep>>>,
}
/// Swap step routed by the Native Vault to a Pool.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustSwapStep {
    #[tlb(adapter = "InternalAddress")]
    pub pool_addr: TonAddress,
    pub params: DedustSwapStepParams,
}
/// Whole-swap options stored in a required referenced cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustSwapParams {
    /// Unix timestamp deadline, or zero when disabled.
    pub deadline: u32,
    #[tlb(adapter = "NullableAddress")]
    pub recipient_addr: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub referral_addr: Option<TonAddress>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Ref-backed Native Vault liquidity-deposit parameters proven by SDK and live code.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustDepositLiquidityParams {
    pub min_lp_amount: TLBCoins,
    pub asset0_target_balance: TLBCoins,
    pub asset1_target_balance: TLBCoins,
}
/// Referenced amount cell inside `create_liquidity_deposit`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultNativeV2CreateLiquidityDepositParams {
    pub asset0_target_balance: TLBCoins,
    pub asset1_target_balance: TLBCoins,
    pub deposit_asset: DedustClassicAsset,
    pub deposit_amount: TLBCoins,
    pub min_lp_amount: TLBCoins,
}
/// DeDust Native Vault v2 exit-code catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DedustVaultNativeV2Errors {
    /// Factory message sender is not the stored Factory.
    InvalidFactorySender = 259,
    /// Asset tag, user payload, or proof shape is invalid for this Vault.
    InvalidAssetOrPayload = 261,
    /// Attached value cannot cover the required downstream fee budget.
    InsufficientValue = 263,
    /// Pool payout sender is not the authenticated Pool.
    InvalidPoolSender = 265,
    /// Liquidity-deposit payout sender is not the authenticated Liquidity Deposit.
    InvalidLiquidityDepositSender = 272,
    /// Liquidity-deposit target amounts must be positive.
    InvalidDepositAmounts = 276,
    /// Routed swap path exceeds the accepted depth.
    SwapPathTooLong = 278,
    /// Liquidity-deposit asset does not match the vault asset side.
    InvalidDepositAsset = 291,
    /// Reset-gas sender is not the deterministic Operator contract.
    InvalidOperatorSender = 296,
    /// Incoming internal message opcode is unsupported.
    UnsupportedOperation = 65535,
}
