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
/// Runtime storage loaded by the v1 Jetton Vault root data cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultJettonV1Storage {
    pub cached_proof: TLBRef<DedustVaultJettonV1ProofData>,
    pub blank_code: TLBRef<TonCell>,
    pub is_ready: bool,
    #[tlb(adapter = "InternalAddress")]
    pub jetton_minter_addr: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub jetton_wallet_addr: Option<TonAddress>,
    pub version: u16,
}
/// DeDust proof data stored by the vault and used for child-address authentication.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultJettonV1ProofData {
    #[tlb(adapter = "InternalAddress")]
    pub factory_addr: TonAddress,
    pub contract_type: u8,
    pub asset: DedustClassicAsset,
}
/// Volatile pool type tag used in PoolParams.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0, bits_len = 1)]
pub struct DedustVaultJettonV1VolatilePool {}
/// Stable pool type tag used in PoolParams.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 1)]
pub struct DedustVaultJettonV1StablePool {}
/// Pool type variants encoded as one bit in DeDust pool parameters.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustVaultJettonV1PoolType {
    Volatile(DedustVaultJettonV1VolatilePool),
    Stable(DedustVaultJettonV1StablePool),
}
/// Pool identity tuple used by Jetton forward payloads and Factory routing.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultJettonV1PoolParams {
    pub pool_type: DedustVaultJettonV1PoolType,
    pub asset0: DedustClassicAsset,
    pub asset1: DedustClassicAsset,
}
/// Swap kind tag for exact-in swaps.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0, bits_len = 1)]
pub struct DedustVaultJettonV1GivenIn {}
/// Swap kind tag reserved for exact-out swaps.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 1)]
pub struct DedustVaultJettonV1GivenOut {}
/// One-bit swap direction encoded inside a swap step.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustVaultJettonV1SwapKind {
    GivenIn(DedustVaultJettonV1GivenIn),
    GivenOut(DedustVaultJettonV1GivenOut),
}
/// Per-step swap parameters embedded after the Pool address.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultJettonV1SwapStepParams {
    pub kind: DedustVaultJettonV1SwapKind,
    pub limit: TLBCoins,
    pub next: Option<TLBRef<Box<DedustVaultJettonV1SwapStep>>>,
}
/// Swap step routed by the Jetton Vault to a Pool.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultJettonV1SwapStep {
    #[tlb(adapter = "InternalAddress")]
    pub pool_addr: TonAddress,
    pub params: DedustVaultJettonV1SwapStepParams,
}
/// Whole-swap options stored in a required referenced cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultJettonV1SwapParams {
    /// Unix timestamp deadline, or zero when disabled.
    pub deadline: u32,
    #[tlb(adapter = "NullableAddress")]
    pub recipient_addr: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub referral_addr: Option<TonAddress>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Referenced amount cell inside `create_liquidity_deposit`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultJettonV1CreateLiquidityDepositParams {
    pub asset0_target_balance: TLBCoins,
    pub asset1_target_balance: TLBCoins,
    pub deposit_asset: DedustClassicAsset,
    pub deposit_amount: TLBCoins,
    pub min_lp_amount: TLBCoins,
}
/// DeDust Jetton Vault v1 exit-code catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DedustVaultJettonV1Errors {
    /// Wallet-discovery response sender is not the stored Jetton minter.
    InvalidJettonMinterSender = 257,
    /// Factory message sender is not the stored Factory.
    InvalidFactorySender = 259,
    /// Asset tag, forward payload, or proof shape is invalid for this Vault.
    InvalidAssetOrPayload = 261,
    /// Pool payout sender is not the authenticated Pool.
    InvalidPoolSender = 265,
    /// Liquidity-deposit payout sender is not the authenticated Liquidity Deposit.
    InvalidLiquidityDepositSender = 272,
    /// Vault is not ready for code upgrade.
    VaultNotReady = 273,
    /// Vault readiness state does not match the requested operation.
    InvalidReadinessState = 274,
    /// Incoming internal opcode is unsupported.
    UnsupportedOperation = 65535,
}
