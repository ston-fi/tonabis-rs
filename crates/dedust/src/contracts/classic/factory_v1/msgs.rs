//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{
    DedustFactoryV1CreateLiquidityDepositExtra, DedustFactoryV1PoolParams, DedustFactoryV1VaultProof,
};
use crate::contracts::classic::asset::DedustClassicAsset;
use ton::ton_core::{
    TLB,
    cell::TonCell,
    types::{
        TonAddress,
        tlb_core::{
            TLBCoins, TLBRef,
            adapters::{ConstLen, InternalAddress, NullableAddress},
        },
    },
};
/// Install payload sent by the Factory to a Blank contract during child deployment.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2604311546, bits_len = 32)]
pub struct DedustFactoryV1InstallMsg {
    pub query_id: u64,
    pub blank_code: TLBRef<TonCell>,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
    pub extra_params: TonCell,
}
/// Child upgrade payload sent from Factory to an existing child contract.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 444, bits_len = 32)]
pub struct DedustFactoryV1UpgradeChildCodeMsg {
    pub query_id: u64,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
}
/// Command sent by the Factory to destroy a non-ready Vault.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2640660523, bits_len = 32)]
pub struct DedustFactoryV1DestroyNonReadyVaultCommandMsg {
    pub query_id: u64,
}
/// Asset deposit forwarded from Factory to the created Liquidity Deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1411649509, bits_len = 32)]
pub struct DedustFactoryV1DepositAssetMsg {
    pub query_id: u64,
    pub asset: DedustClassicAsset,
    pub amount: TLBCoins,
}
/// Request to transfer Factory ownership to `newOwnerAddress`.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3395376462, bits_len = 32)]
pub struct DedustFactoryV1TransferOwnershipMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub new_owner_address: Option<TonAddress>,
}
/// Pending owner request to accept ownership.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3739616260, bits_len = 32)]
pub struct DedustFactoryV1AcceptOwnershipMsg {
    /// Body tail ignored by this opcode handler.
    pub tail: TonCell,
}
/// Current owner request to cancel a pending ownership transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 382435266, bits_len = 32)]
pub struct DedustFactoryV1CancelOwnershipTransferMsg {
    /// Body tail ignored by this opcode handler.
    pub tail: TonCell,
}
/// Owner request to install a Vault implementation for an asset type.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3158255350, bits_len = 32)]
pub struct DedustFactoryV1InstallVaultCodeMsg {
    pub query_id: u64,
    #[tlb(bits_len = 4)]
    pub asset_type: u8,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
}
/// Public request to deploy the Vault for an asset if it is missing.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 567271467, bits_len = 32)]
pub struct DedustFactoryV1CreateVaultMsg {
    pub query_id: u64,
    pub asset: DedustClassicAsset,
}
/// Request to deploy a legacy Jetton Vault through a resolver.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3383063853, bits_len = 32)]
pub struct DedustFactoryV1CreateLegacyJettonVaultMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub minter_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub resolver_address: TonAddress,
}
/// Request to upgrade the Vault for an existing asset.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 634808593, bits_len = 32)]
pub struct DedustFactoryV1UpgradeVaultMsg {
    pub query_id: u64,
    pub asset: DedustClassicAsset,
}
/// Owner request to remove a non-ready Vault for an asset.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2320600333, bits_len = 32)]
pub struct DedustFactoryV1DestroyNonReadyVaultMsg {
    pub query_id: u64,
    pub asset: DedustClassicAsset,
}
/// Owner request to replace Factory code.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3746178986, bits_len = 32)]
pub struct DedustFactoryV1UpgradeMsg {
    pub query_id: u64,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
}
/// Owner request to return Factory excess gas.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2671708471, bits_len = 32)]
pub struct DedustFactoryV1ResetGasMsg {
    /// Body tail ignored by this opcode handler.
    pub tail: TonCell,
}
/// Owner request to install Pool implementation code.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2749652465, bits_len = 32)]
pub struct DedustFactoryV1InstallPoolCodeMsg {
    pub query_id: u64,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
}
/// Public request to deploy a volatile Pool for two sorted assets.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2547326767, bits_len = 32)]
pub struct DedustFactoryV1CreateVolatilePoolMsg {
    pub query_id: u64,
    pub asset0: DedustClassicAsset,
    pub asset1: DedustClassicAsset,
}
/// Owner request to deploy a stable Pool.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2084613255, bits_len = 32)]
pub struct DedustFactoryV1CreateStablePoolMsg {
    pub query_id: u64,
    pub asset0: DedustClassicAsset,
    pub asset0_decimals: u8,
    pub asset1: DedustClassicAsset,
    pub asset1_decimals: u8,
}
/// Owner request to upgrade an existing Pool implementation.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1407341230, bits_len = 32)]
pub struct DedustFactoryV1UpgradePoolMsg {
    pub query_id: u64,
    pub pool_params: DedustFactoryV1PoolParams,
}
/// Owner request to install Liquidity Deposit implementation code.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2577941265, bits_len = 32)]
pub struct DedustFactoryV1InstallLiquidityDepositCodeMsg {
    pub query_id: u64,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
}
/// Vault request to create a Liquidity Deposit for an owner and pool.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 4031694118, bits_len = 32)]
pub struct DedustFactoryV1CreateLiquidityDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustFactoryV1VaultProof>,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    pub pool_params: DedustFactoryV1PoolParams,
    pub extra: TLBRef<DedustFactoryV1CreateLiquidityDepositExtra>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustFactoryV1IncomingMsg {
    TransferOwnership(Box<DedustFactoryV1TransferOwnershipMsg>),
    AcceptOwnership(Box<DedustFactoryV1AcceptOwnershipMsg>),
    CancelOwnershipTransfer(Box<DedustFactoryV1CancelOwnershipTransferMsg>),
    InstallVaultCode(Box<DedustFactoryV1InstallVaultCodeMsg>),
    CreateVault(Box<DedustFactoryV1CreateVaultMsg>),
    CreateLegacyJettonVault(Box<DedustFactoryV1CreateLegacyJettonVaultMsg>),
    UpgradeVault(Box<DedustFactoryV1UpgradeVaultMsg>),
    DestroyNonReadyVault(Box<DedustFactoryV1DestroyNonReadyVaultMsg>),
    Upgrade(Box<DedustFactoryV1UpgradeMsg>),
    ResetGas(Box<DedustFactoryV1ResetGasMsg>),
    InstallPoolCode(Box<DedustFactoryV1InstallPoolCodeMsg>),
    CreateVolatilePool(Box<DedustFactoryV1CreateVolatilePoolMsg>),
    CreateStablePool(Box<DedustFactoryV1CreateStablePoolMsg>),
    UpgradePool(Box<DedustFactoryV1UpgradePoolMsg>),
    InstallLiquidityDepositCode(Box<DedustFactoryV1InstallLiquidityDepositCodeMsg>),
    CreateLiquidityDeposit(Box<DedustFactoryV1CreateLiquidityDepositMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustFactoryV1OutgoingMsg {
    Install(Box<DedustFactoryV1InstallMsg>),
    UpgradeChildCode(Box<DedustFactoryV1UpgradeChildCodeMsg>),
    DestroyNonReadyVaultCommand(Box<DedustFactoryV1DestroyNonReadyVaultCommandMsg>),
    DepositAsset(Box<DedustFactoryV1DepositAssetMsg>),
    Unknown(TonCell),
}
