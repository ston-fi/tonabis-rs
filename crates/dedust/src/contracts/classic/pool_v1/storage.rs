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
/// Runtime Pool storage loaded from the v1 root data cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV1Storage {
    pub proof: TLBRef<DedustPoolV1PoolProof>,
    pub proof_code: TLBRef<TonCell>,
    pub lp_wallet_code: TLBRef<TonCell>,
    pub config: TLBRef<DedustPoolV1Config>,
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
}
/// Referenced Pool configuration cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV1Config {
    pub asset0_decimals: u8,
    pub asset1_decimals: u8,
    #[tlb(adapter = "InternalAddress")]
    pub asset0_vault_addr: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub asset1_vault_addr: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub fee_collector_addr: Option<TonAddress>,
}
/// Volatile curve tag used in pool parameters.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0, bits_len = 1)]
pub struct DedustPoolV1VolatilePool {}
/// Stable curve tag used in pool parameters.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 1)]
pub struct DedustPoolV1StablePool {}
/// One-bit pool type represented by getters as `is_stable`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustPoolV1PoolType {
    Volatile(DedustPoolV1VolatilePool),
    Stable(DedustPoolV1StablePool),
}
/// Pool identity tuple embedded in Pool and Liquidity Deposit proofs.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV1PoolParams {
    pub pool_type: DedustPoolV1PoolType,
    pub asset0: DedustClassicAsset,
    pub asset1: DedustClassicAsset,
}
/// Contract-type tag proving a sender is a DeDust Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 8)]
pub struct DedustPoolV1VaultContractType {}
/// Contract-type tag proving a sender is a DeDust Pool.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2, bits_len = 8)]
pub struct DedustPoolV1PoolContractType {}
/// Contract-type tag proving a sender is a DeDust Liquidity Deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3, bits_len = 8)]
pub struct DedustPoolV1LiquidityDepositContractType {}
/// Vault proof body referenced by swap-external messages.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV1VaultProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_addr: TonAddress,
    pub contract_type: DedustPoolV1VaultContractType,
    pub params: DedustClassicAsset,
}
/// Pool proof body referenced by peer swaps, pool payouts, and callbacks.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV1PoolProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_addr: TonAddress,
    pub contract_type: DedustPoolV1PoolContractType,
    pub params: DedustPoolV1PoolParams,
}
/// Referenced proof parameters for a Liquidity Deposit contract.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV1LiquidityDepositProofParams {
    #[tlb(adapter = "InternalAddress")]
    pub owner_addr: TonAddress,
    pub pool_params: DedustPoolV1PoolParams,
}
/// Liquidity Deposit proof body referenced by deposit-liquidity-all messages.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV1LiquidityDepositProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_addr: TonAddress,
    pub contract_type: DedustPoolV1LiquidityDepositContractType,
    pub params: TLBRef<DedustPoolV1LiquidityDepositProofParams>,
}
/// Exact-input swap kind.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0, bits_len = 1)]
pub struct DedustPoolV1GivenIn {}
/// Exact-output swap kind.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 1)]
pub struct DedustPoolV1GivenOut {}
/// One-bit swap kind embedded in each swap step.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustPoolV1SwapKind {
    GivenIn(DedustPoolV1GivenIn),
    GivenOut(DedustPoolV1GivenOut),
}
/// Parameters for the current step of a routed swap.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV1SwapStepParams {
    pub kind: DedustPoolV1SwapKind,
    pub limit: TLBCoins,
    pub next: Option<TLBRef<Box<DedustPoolV1SwapStep>>>,
}
/// Full next-hop swap step referenced from step parameters.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV1SwapStep {
    #[tlb(adapter = "InternalAddress")]
    pub pool_addr: TonAddress,
    pub params: DedustPoolV1SwapStepParams,
}
/// Whole-swap options stored in the required referenced swap-params cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV1SwapParams {
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
pub struct DedustPoolV1DepositLiquidityAllAmounts {
    pub asset0: DedustClassicAsset,
    pub asset0_amount: TLBCoins,
    pub asset1: DedustClassicAsset,
    pub asset1_amount: TLBCoins,
}
/// Referenced tail for swap external event details.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustPoolV1SwapEventTail {
    #[tlb(adapter = "InternalAddress")]
    pub sender_addr: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub referral_addr: Option<TonAddress>,
    pub reserve0: TLBCoins,
    pub reserve1: TLBCoins,
}
/// DeDust Pool v1 exit codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DedustPoolV1Errors {
    /// LP burn notification or protocol-fee sender is not authorized.
    InvalidLpOrFeeSender = 257,
    /// Control message sender is not the stored Factory/proof address.
    InvalidControlSender = 259,
    /// Asset tag or proof shape is invalid for this Pool.
    InvalidAssetOrProof = 261,
    /// Swap-external sender is not the authenticated Vault.
    InvalidVaultSender = 264,
    /// Swap-peer sender is not the authenticated Pool.
    InvalidPoolSender = 265,
    /// Liquidity-deposit callback sender is not the authenticated deposit contract.
    InvalidLiquidityDepositSender = 272,
    /// Incoming internal message opcode is unsupported.
    UnsupportedOperation = 65535,
}
