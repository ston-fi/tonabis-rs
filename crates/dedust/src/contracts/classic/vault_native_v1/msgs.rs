//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{
    DedustVaultNativeV1CreateLiquidityDepositExtra, DedustVaultNativeV1DepositLiquidityParams,
    DedustVaultNativeV1LiquidityDepositProof, DedustVaultNativeV1PoolParams, DedustVaultNativeV1PoolProof,
    DedustVaultNativeV1SwapStep, DedustVaultNativeV1SwapStepParams, DedustVaultNativeV1VaultProof,
};
use crate::contracts::classic::pool_v1::storage::DedustPoolV1SwapParams;
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
/// User-originated native GRAM swap accepted by the Native Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3926267997, bits_len = 32)]
pub struct DedustVaultNativeV1SwapMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub step: DedustVaultNativeV1SwapStep,
    pub swap_params: TLBRef<DedustPoolV1SwapParams>,
}
/// User-originated native GRAM liquidity deposit accepted by the Native Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3579725446, bits_len = 32)]
pub struct DedustVaultNativeV1DepositLiquidityMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub pool_params: DedustVaultNativeV1PoolParams,
    pub params: TLBRef<DedustVaultNativeV1DepositLiquidityParams>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Pool-authorized payout callback accepted by the Native Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2907617013, bits_len = 32)]
pub struct DedustVaultNativeV1PayOutFromPoolMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustVaultNativeV1PoolProof>,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub recipient_address: Option<TonAddress>,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Liquidity Deposit-authorized payout callback accepted by the Native Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1795913855, bits_len = 32)]
pub struct DedustVaultNativeV1PayOutFromDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustVaultNativeV1LiquidityDepositProof>,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Factory-authorized Vault code upgrade message.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 444, bits_len = 32)]
pub struct DedustVaultNativeV1UpgradeChildCodeMsg {
    pub query_id: u64,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
}
/// Vault-to-Pool swap routing message constructed after a native swap.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1643009069, bits_len = 32)]
pub struct DedustVaultNativeV1SwapExternalMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustVaultNativeV1VaultProof>,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender_address: TonAddress,
    pub current: DedustVaultNativeV1SwapStepParams,
    pub swap_params: TLBRef<DedustPoolV1SwapParams>,
}
/// Vault-to-Factory request constructed after a native liquidity deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 4031694118, bits_len = 32)]
pub struct DedustVaultNativeV1CreateLiquidityDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustVaultNativeV1VaultProof>,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    pub pool_params: DedustVaultNativeV1PoolParams,
    pub extra: TLBRef<DedustVaultNativeV1CreateLiquidityDepositExtra>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Final native payout body emitted by the Vault to recipients.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1196394191, bits_len = 32)]
pub struct DedustVaultNativeV1PayoutMsg {
    pub query_id: u64,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustVaultNativeV1IncomingMsg {
    UpgradeChildCode(Box<DedustVaultNativeV1UpgradeChildCodeMsg>),
    DepositLiquidity(Box<DedustVaultNativeV1DepositLiquidityMsg>),
    PayOutFromDeposit(Box<DedustVaultNativeV1PayOutFromDepositMsg>),
    PayOutFromPool(Box<DedustVaultNativeV1PayOutFromPoolMsg>),
    Swap(Box<DedustVaultNativeV1SwapMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustVaultNativeV1OutgoingMsg {
    SwapExternal(Box<DedustVaultNativeV1SwapExternalMsg>),
    CreateLiquidityDeposit(Box<DedustVaultNativeV1CreateLiquidityDepositMsg>),
    Payout(Box<DedustVaultNativeV1PayoutMsg>),
    Unknown(TonCell),
}
