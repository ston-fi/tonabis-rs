//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{DedustVaultJettonV2CreateLiquidityDepositParams, DedustVaultJettonV2ProofData};
use crate::contracts::classic::liquidity_deposit_v2::storage::DedustLiquidityDepositV2PoolProof;
use crate::contracts::classic::liquidity_deposit_v2::storage::DedustLiquidityDepositV2Proof;
use crate::contracts::classic::vault_native_v2::storage::DedustSwapParams;
use crate::contracts::classic::vault_native_v2::storage::DedustSwapStep;
use crate::contracts::classic::vault_native_v2::storage::DedustSwapStepParams;
use crate::contracts::classic::vault_native_v2::storage::DedustVaultNativeV2PoolParams;
use ton::ton_core::types::tlb_core::TLBEitherRef;
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
use tonabis_core::adapters::maybe_ref_address::MaybeRefAddress;
/// Jetton transfer notification accepted from the vault-owned Jetton wallet.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1935855772, bits_len = 32)]
pub struct DedustVaultJettonV2TransferNotificationMsg {
    pub query_id: u64,
    /// Received Jetton amount in base units.
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    /// Typed generation-specific swap/deposit dispatcher with exact Either inline/reference form and unknown-byte preservation.
    pub forward_payload: TLBEitherRef<crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2ForwardMsg>,
}
/// User Jetton forward payload for swapping the transferred Jetton amount.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3818968194, bits_len = 32)]
pub struct DedustVaultJettonV2SwapForwardPayloadMsg {
    pub step: DedustSwapStep,
    pub swap_params: TLBRef<DedustSwapParams>,
}
/// User Jetton forward payload for depositing the transferred Jetton amount as liquidity.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1088489686, bits_len = 32)]
pub struct DedustVaultJettonV2DepositLiquidityForwardPayloadMsg {
    pub pool_params: DedustVaultNativeV2PoolParams,
    pub min_lp_amount: TLBCoins,
    pub asset0_target_balance: TLBCoins,
    pub asset1_target_balance: TLBCoins,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Pool-authorized payout callback accepted by the Jetton Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2907617013, bits_len = 32)]
pub struct DedustVaultJettonV2PayOutFromPoolMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustLiquidityDepositV2PoolProof>,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub recipient_addr: Option<TonAddress>,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Liquidity Deposit-authorized payout callback accepted by the Jetton Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1795913855, bits_len = 32)]
pub struct DedustVaultJettonV2PayOutFromDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustLiquidityDepositV2Proof>,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}
/// State-probe request accepted by the Jetton Vault and answered with `take_vault_state`.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3912500665, bits_len = 32)]
pub struct DedustVaultJettonV2ProvideVaultStateMsg {
    pub query_id: u64,
}
/// Common reset-gas request accepted when sent by the authenticated Jetton wallet address.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 650525367, bits_len = 32)]
pub struct DedustVaultJettonV2ResetGasMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub response_destination: TonAddress,
}
/// Factory-authorized request to destroy a non-ready Jetton Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2640660523, bits_len = 32)]
pub struct DedustVaultJettonV2DestroyNonReadyMsg {
    pub query_id: u64,
}
/// Vault-to-Pool swap routing message constructed after a Jetton transfer notification.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1643009069, bits_len = 32)]
pub struct DedustVaultJettonV2SwapExternalMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustVaultJettonV2ProofData>,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender_addr: TonAddress,
    pub current: DedustSwapStepParams,
    pub swap_params: TLBRef<DedustSwapParams>,
}
/// Vault-to-Factory request constructed after a Jetton liquidity deposit payload.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 4031694118, bits_len = 32)]
pub struct DedustVaultJettonV2CreateLiquidityDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustVaultJettonV2ProofData>,
    #[tlb(adapter = "InternalAddress")]
    pub owner_addr: TonAddress,
    pub pool_params: DedustVaultNativeV2PoolParams,
    pub deposit_params: TLBRef<DedustVaultJettonV2CreateLiquidityDepositParams>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Jetton transfer request body sent by the vault to its owned Jetton wallet for payouts and refunds.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 260734629, bits_len = 32)]
pub struct DedustVaultJettonV2JettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub destination: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    /// Optional custom payload; payout paths store `null`.
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TonCell,
}
/// State response emitted by the vault to callers of `provide_vault_state`.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2836651021, bits_len = 32)]
pub struct DedustVaultJettonV2TakeVaultStateMsg {
    pub query_id: u64,
    pub is_ready: bool,
}
/// Jetton master wallet-discovery request emitted while syncing the vault wallet.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 745978227, bits_len = 32)]
pub struct DedustVaultJettonV2ProvideWalletAddressMsg {
    /// Query id copied into `take_wallet_address`; live code sends zero.
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    pub include_address: bool,
}
/// Jetton wallet-discovery response accepted from the Jetton minter during readiness sync.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3513996288, bits_len = 32)]
pub struct DedustVaultJettonV2TakeWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub wallet_address: TonAddress,
    pub include_address: bool,
    #[tlb(adapter = "MaybeRefAddress")]
    pub owner_address: Option<TonAddress>,
}
/// Discovery bounce prefix. Only original opcode2c76b973 destroys/refunds the vault; other opcodes are ignored before storage access. Remaining bits/refs are ignored.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultJettonV2BouncedProvideWalletAddressMsg {
    /// Source skips32bits without marker validation; bounced flag remains a separate envelope property.
    pub prefix: u32,
    pub original_opcode: u32,
    pub tail: TonCell,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustVaultJettonV2IncomingMsg {
    TransferNotification(Box<DedustVaultJettonV2TransferNotificationMsg>),
    PayOutFromPool(Box<DedustVaultJettonV2PayOutFromPoolMsg>),
    PayOutFromDeposit(Box<DedustVaultJettonV2PayOutFromDepositMsg>),
    ProvideVaultState(Box<DedustVaultJettonV2ProvideVaultStateMsg>),
    ResetGas(Box<DedustVaultJettonV2ResetGasMsg>),
    DestroyNonReady(Box<DedustVaultJettonV2DestroyNonReadyMsg>),
    TakeWalletAddress(Box<DedustVaultJettonV2TakeWalletAddressMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustVaultJettonV2OutgoingMsg {
    SwapExternal(Box<DedustVaultJettonV2SwapExternalMsg>),
    CreateLiquidityDeposit(Box<DedustVaultJettonV2CreateLiquidityDepositMsg>),
    JettonTransfer(Box<DedustVaultJettonV2JettonTransferMsg>),
    TakeVaultState(Box<DedustVaultJettonV2TakeVaultStateMsg>),
    ProvideWalletAddress(Box<DedustVaultJettonV2ProvideWalletAddressMsg>),
    Unknown(TonCell),
}
/// TEP-74 forward body; short and unknown bodies retain their raw cells.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustVaultJettonV2ForwardMsg {
    Swap(Box<DedustVaultJettonV2SwapForwardPayloadMsg>),
    DepositLiquidity(Box<DedustVaultJettonV2DepositLiquidityForwardPayloadMsg>),
    Unknown(TonCell),
}
