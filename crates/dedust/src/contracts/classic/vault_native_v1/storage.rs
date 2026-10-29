//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use crate::contracts::classic::asset::DedustClassicAsset;
use ton::ton_core::{
    TLB,
    cell::TonCell,
    types::{
        TonAddress,
        tlb_core::{TLBCoins, TLBRef, adapters::InternalAddress},
    },
};
/// Runtime storage loaded by the Native Vault v1 root data cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultNativeV1Storage {
    pub cached_proof: TLBRef<DedustVaultNativeV1VaultProof>,
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    pub blank_code: TLBRef<TonCell>,
    pub version: u16,
}
/// Vault proof data committed into Native Vault storage.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultNativeV1VaultProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    pub contract_type: u8,
    pub asset: DedustClassicAsset,
}
/// Pool identity encoded in native deposit and proof bodies.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultNativeV1PoolParams {
    pub is_stable: bool,
    pub asset0: DedustClassicAsset,
    pub asset1: DedustClassicAsset,
}
/// Pool proof accepted by the pool payout callback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultNativeV1PoolProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    pub contract_type: u8,
    pub pool_params: DedustVaultNativeV1PoolParams,
}
/// Liquidity Deposit proof accepted by the deposit payout callback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultNativeV1LiquidityDepositProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    pub contract_type: u8,
    pub params: TLBRef<DedustVaultNativeV1LiquidityDepositProofParams>,
}
/// Referenced owner and Pool identity in a Liquidity Deposit proof.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultNativeV1LiquidityDepositProofParams {
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    pub pool_params: DedustVaultNativeV1PoolParams,
}
/// Swap kind flag encoded in native swap steps.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0, bits_len = 1)]
pub struct DedustVaultNativeV1GivenIn {}
/// Reserved swap kind flag for exact-out style steps.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 1)]
pub struct DedustVaultNativeV1GivenOut {}
/// One-bit swap direction encoded in a swap step.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustVaultNativeV1SwapKind {
    GivenIn(DedustVaultNativeV1GivenIn),
    GivenOut(DedustVaultNativeV1GivenOut),
}
/// Swap-step parameters forwarded to a Pool.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultNativeV1SwapStepParams {
    pub kind: DedustVaultNativeV1SwapKind,
    pub limit: TLBCoins,
    pub next: Option<TLBRef<Box<DedustVaultNativeV1SwapStep>>>,
}
/// First swap step selected by a user-originated native swap.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultNativeV1SwapStep {
    #[tlb(adapter = "InternalAddress")]
    pub pool_address: TonAddress,
    pub params: DedustVaultNativeV1SwapStepParams,
}
/// Ref-backed Native Vault liquidity-deposit constraints.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultNativeV1DepositLiquidityParams {
    pub min_lp_amount: TLBCoins,
    pub asset0_target_balance: TLBCoins,
    pub asset1_target_balance: TLBCoins,
}
/// Referenced amount cell inside `create_liquidity_deposit`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultNativeV1CreateLiquidityDepositExtra {
    pub asset0_target_balance: TLBCoins,
    pub asset1_target_balance: TLBCoins,
    pub deposit_asset: DedustClassicAsset,
    pub deposit_amount: TLBCoins,
    pub min_lp_amount: TLBCoins,
}
/// DeDust Native Vault v1 exit-code catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DedustVaultNativeV1Errors {
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
    /// Incoming internal message opcode is unsupported.
    UnsupportedOperation = 65535,
}
