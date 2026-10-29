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
/// Runtime storage loaded by the verified Jetton Vault code root data cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultJettonV2Storage {
    pub cached_proof: TLBRef<DedustVaultJettonV2ProofData>,
    pub blank_code: TLBRef<TonCell>,
    pub is_ready: bool,
    #[tlb(adapter = "InternalAddress")]
    pub jetton_minter_addr: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub jetton_wallet_addr: Option<TonAddress>,
    pub version: u16,
}
/// DeDust proof data stored by the vault and used in pool/factory authentication.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultJettonV2ProofData {
    #[tlb(adapter = "InternalAddress")]
    pub factory_addr: TonAddress,
    pub contract_type: u8,
    pub asset: DedustClassicAsset,
}
/// Referenced amount cell inside `create_liquidity_deposit`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustVaultJettonV2CreateLiquidityDepositParams {
    pub asset0_target_balance: TLBCoins,
    pub asset1_target_balance: TLBCoins,
    pub deposit_asset: DedustClassicAsset,
    pub deposit_amount: TLBCoins,
    pub min_lp_amount: TLBCoins,
}
/// DeDust Jetton Vault v2 exit-code catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DedustVaultJettonV2Errors {
    /// Jetton notification sender is not the stored vault wallet.
    InvalidJettonWalletSender = 257,
    /// Factory message sender is not the stored Factory.
    InvalidFactorySender = 259,
    /// Asset tag, forward payload, or proof shape is invalid for this Vault.
    InvalidAssetOrPayload = 261,
    /// Attached value cannot cover the required downstream fee budget.
    InsufficientValue = 263,
    /// Pool payout sender is not the authenticated Pool.
    InvalidPoolSender = 265,
    /// Liquidity-deposit payout sender is not the authenticated Liquidity Deposit.
    InvalidLiquidityDepositSender = 272,
    /// Vault is not ready for code upgrade.
    VaultNotReady = 273,
    /// Vault readiness state does not match the requested operation.
    InvalidReadinessState = 274,
    /// Liquidity-deposit target amounts must be positive.
    InvalidDepositAmounts = 276,
    /// Liquidity-deposit asset does not match the vault asset side.
    InvalidDepositAsset = 291,
    /// Reset-gas sender is not the deterministic Operator contract.
    InvalidOperatorSender = 296,
    /// Incoming internal or forward-payload opcode is unsupported.
    UnsupportedOperation = 65535,
}
