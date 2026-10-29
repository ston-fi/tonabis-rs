//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{
    DedustDepositLiquidityParams, DedustSwapParams, DedustSwapStep, DedustSwapStepParams,
    DedustVaultNativeV2CreateLiquidityDepositParams, DedustVaultNativeV2PoolParams, DedustVaultNativeV2ProofData,
};
use crate::contracts::classic::liquidity_deposit_v2::storage::DedustLiquidityDepositV2PoolProof;
use crate::contracts::classic::liquidity_deposit_v2::storage::DedustLiquidityDepositV2Proof;
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
/// User-originated native GRAM swap accepted by the live Native Vault code.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3926267997, bits_len = 32)]
pub struct DedustVaultNativeV2SwapMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub step: DedustSwapStep,
    pub swap_params: TLBRef<DedustSwapParams>,
}
/// User-originated native GRAM liquidity deposit using the ref-backed params cell.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3579725446, bits_len = 32)]
pub struct DedustVaultNativeV2DepositLiquidityMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub pool_params: DedustVaultNativeV2PoolParams,
    pub params: TLBRef<DedustDepositLiquidityParams>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Pool-authorized payout callback accepted by the Native Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2907617013, bits_len = 32)]
pub struct DedustVaultNativeV2PayOutFromPoolMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustLiquidityDepositV2PoolProof>,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub recipient_addr: Option<TonAddress>,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Liquidity Deposit-authorized payout callback accepted by the Native Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1795913855, bits_len = 32)]
pub struct DedustVaultNativeV2PayOutFromDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustLiquidityDepositV2Proof>,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}
/// State-probe request accepted by the Native Vault and answered with `take_vault_state`.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3912500665, bits_len = 32)]
pub struct DedustVaultNativeV2ProvideVaultStateMsg {
    pub query_id: u64,
}
/// Common reset-gas request accepted when sent by the authenticated operator address.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 650525367, bits_len = 32)]
pub struct DedustVaultNativeV2ResetGasMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub response_destination: TonAddress,
}
/// Vault-to-Pool swap routing message constructed after a native swap.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1643009069, bits_len = 32)]
pub struct DedustVaultNativeV2SwapExternalMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustVaultNativeV2ProofData>,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender_addr: TonAddress,
    pub current: DedustSwapStepParams,
    pub swap_params: TLBRef<DedustSwapParams>,
}
/// Vault-to-Factory request constructed after a native liquidity deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 4031694118, bits_len = 32)]
pub struct DedustVaultNativeV2CreateLiquidityDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustVaultNativeV2ProofData>,
    #[tlb(adapter = "InternalAddress")]
    pub owner_addr: TonAddress,
    pub pool_params: DedustVaultNativeV2PoolParams,
    pub deposit_params: TLBRef<DedustVaultNativeV2CreateLiquidityDepositParams>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Final native payout body emitted by the vault to recipients.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1196394191, bits_len = 32)]
pub struct DedustVaultNativeV2PayoutMsg {
    pub query_id: u64,
    pub payload: Option<TLBRef<TonCell>>,
}
/// State response emitted by the vault to callers of `provide_vault_state`.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2836651021, bits_len = 32)]
pub struct DedustVaultNativeV2TakeVaultStateMsg {
    pub query_id: u64,
    pub is_ready: bool,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustVaultNativeV2IncomingMsg {
    Swap(Box<DedustVaultNativeV2SwapMsg>),
    DepositLiquidity(Box<DedustVaultNativeV2DepositLiquidityMsg>),
    PayOutFromPool(Box<DedustVaultNativeV2PayOutFromPoolMsg>),
    PayOutFromDeposit(Box<DedustVaultNativeV2PayOutFromDepositMsg>),
    ProvideVaultState(Box<DedustVaultNativeV2ProvideVaultStateMsg>),
    ResetGas(Box<DedustVaultNativeV2ResetGasMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustVaultNativeV2OutgoingMsg {
    SwapExternal(Box<DedustVaultNativeV2SwapExternalMsg>),
    CreateLiquidityDeposit(Box<DedustVaultNativeV2CreateLiquidityDepositMsg>),
    Payout(Box<DedustVaultNativeV2PayoutMsg>),
    TakeVaultState(Box<DedustVaultNativeV2TakeVaultStateMsg>),
    Unknown(TonCell),
}
