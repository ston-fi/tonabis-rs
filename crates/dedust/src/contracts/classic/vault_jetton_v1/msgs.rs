//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{
    DedustVaultJettonV1CreateLiquidityDepositParams, DedustVaultJettonV1PoolParams, DedustVaultJettonV1ProofData,
    DedustVaultJettonV1SwapParams, DedustVaultJettonV1SwapStep, DedustVaultJettonV1SwapStepParams,
};
use crate::contracts::classic::liquidity_deposit_v1::storage::DedustLiquidityDepositV1PoolProof;
use crate::contracts::classic::liquidity_deposit_v1::storage::DedustLiquidityDepositV1Proof;
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
pub struct DedustVaultJettonV1TransferNotificationMsg {
    pub query_id: u64,
    /// Received Jetton amount in base units.
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    /// Typed generation-specific swap/deposit dispatcher with exact Either inline/reference form and unknown-byte preservation.
    pub forward_payload: TLBEitherRef<crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1ForwardMsg>,
}
/// User Jetton forward payload for swapping the transferred Jetton amount.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3818968194, bits_len = 32)]
pub struct DedustVaultJettonV1SwapForwardPayloadMsg {
    pub step: DedustVaultJettonV1SwapStep,
    pub swap_params: TLBRef<DedustVaultJettonV1SwapParams>,
}
/// User Jetton forward payload for depositing the transferred Jetton amount as liquidity.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1088489686, bits_len = 32)]
pub struct DedustVaultJettonV1DepositLiquidityForwardPayloadMsg {
    pub pool_params: DedustVaultJettonV1PoolParams,
    pub min_lp_amount: TLBCoins,
    pub asset0_target_balance: TLBCoins,
    pub asset1_target_balance: TLBCoins,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Pool-authorized payout callback accepted by the Jetton Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2907617013, bits_len = 32)]
pub struct DedustVaultJettonV1PayOutFromPoolMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustLiquidityDepositV1PoolProof>,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub recipient_addr: Option<TonAddress>,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Liquidity Deposit-authorized payout callback accepted by the Jetton Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1795913855, bits_len = 32)]
pub struct DedustVaultJettonV1PayOutFromDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustLiquidityDepositV1Proof>,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Factory-authorized request to install a newer child code cell.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 444, bits_len = 32)]
pub struct DedustVaultJettonV1UpgradeChildCodeMsg {
    pub query_id: u64,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
}
/// Jetton wallet-discovery response accepted from the Jetton minter.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3513996288, bits_len = 32)]
pub struct DedustVaultJettonV1TakeWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub wallet_address: TonAddress,
    pub include_address: bool,
    #[tlb(adapter = "MaybeRefAddress")]
    pub owner_address: Option<TonAddress>,
}
/// Factory-authorized request to destroy a non-ready Jetton Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2640660523, bits_len = 32)]
pub struct DedustVaultJettonV1DestroyNonReadyMsg {
    /// Body tail ignored by the v1 handler.
    pub tail: TonCell,
}
/// Vault-to-Pool swap routing message constructed after a Jetton transfer notification.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1643009069, bits_len = 32)]
pub struct DedustVaultJettonV1SwapExternalMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustVaultJettonV1ProofData>,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender_addr: TonAddress,
    pub current: DedustVaultJettonV1SwapStepParams,
    pub swap_params: TLBRef<DedustVaultJettonV1SwapParams>,
}
/// Vault-to-Factory request constructed after a Jetton liquidity deposit payload.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 4031694118, bits_len = 32)]
pub struct DedustVaultJettonV1CreateLiquidityDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustVaultJettonV1ProofData>,
    #[tlb(adapter = "InternalAddress")]
    pub owner_addr: TonAddress,
    pub pool_params: DedustVaultJettonV1PoolParams,
    pub deposit_params: TLBRef<DedustVaultJettonV1CreateLiquidityDepositParams>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Jetton transfer request body sent by the vault to its owned Jetton wallet.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 260734629, bits_len = 32)]
pub struct DedustVaultJettonV1JettonTransferMsg {
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
/// Jetton master wallet-discovery request emitted while syncing the vault wallet.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 745978227, bits_len = 32)]
pub struct DedustVaultJettonV1ProvideWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    pub include_address: bool,
}
/// Discovery bounce prefix. Only original opcode2c76b973 destroys/refunds the vault; other opcodes are ignored before storage access. Remaining bits/refs are ignored.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultJettonV1BouncedProvideWalletAddressMsg {
    /// Source skips32bits without marker validation; bounced flag remains a separate envelope property.
    pub prefix: u32,
    pub original_opcode: u32,
    /// Remaining bounced body bits/refs are ignored.
    pub tail: TonCell,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustVaultJettonV1IncomingMsg {
    TransferNotification(Box<DedustVaultJettonV1TransferNotificationMsg>),
    PayOutFromPool(Box<DedustVaultJettonV1PayOutFromPoolMsg>),
    PayOutFromDeposit(Box<DedustVaultJettonV1PayOutFromDepositMsg>),
    UpgradeChildCode(Box<DedustVaultJettonV1UpgradeChildCodeMsg>),
    TakeWalletAddress(Box<DedustVaultJettonV1TakeWalletAddressMsg>),
    DestroyNonReady(Box<DedustVaultJettonV1DestroyNonReadyMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustVaultJettonV1OutgoingMsg {
    SwapExternal(Box<DedustVaultJettonV1SwapExternalMsg>),
    CreateLiquidityDeposit(Box<DedustVaultJettonV1CreateLiquidityDepositMsg>),
    JettonTransfer(Box<DedustVaultJettonV1JettonTransferMsg>),
    ProvideWalletAddress(Box<DedustVaultJettonV1ProvideWalletAddressMsg>),
    Unknown(TonCell),
}
/// TEP-74 forward body; short and unknown bodies retain their raw cells.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustVaultJettonV1ForwardMsg {
    Swap(Box<DedustVaultJettonV1SwapForwardPayloadMsg>),
    DepositLiquidity(Box<DedustVaultJettonV1DepositLiquidityForwardPayloadMsg>),
    Unknown(TonCell),
}
