//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{
    DedustFactoryV2CreateLiquidityDepositExtra, DedustFactoryV2PoolParams, DedustFactoryV2VaultProof,
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
pub struct DedustFactoryV2InstallMsg {
    pub query_id: u64,
    pub blank_code: TLBRef<TonCell>,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
    pub extra_params: TonCell,
}
/// Asset deposit forwarded from Factory to the created Liquidity Deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1411649509, bits_len = 32)]
pub struct DedustFactoryV2DepositAssetMsg {
    pub query_id: u64,
    pub asset: DedustClassicAsset,
    pub amount: TLBCoins,
}
/// Request to transfer Factory ownership to `newOwnerAddress`.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3395376462, bits_len = 32)]
pub struct DedustFactoryV2TransferOwnershipMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub new_owner_address: Option<TonAddress>,
}
/// Pending owner request to accept ownership after the configured delay.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3739616260, bits_len = 32)]
pub struct DedustFactoryV2AcceptOwnershipMsg {
    pub query_id: u64,
}
/// Current owner request to cancel a pending ownership transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 382435266, bits_len = 32)]
pub struct DedustFactoryV2CancelOwnershipTransferMsg {
    pub query_id: u64,
}
/// Owner/operator request to install a Vault implementation for an asset type.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3158255350, bits_len = 32)]
pub struct DedustFactoryV2InstallVaultCodeMsg {
    pub query_id: u64,
    #[tlb(bits_len = 4)]
    pub asset_type: u8,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
}
/// Public request to deploy the Vault for an asset if it is missing.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 567271467, bits_len = 32)]
pub struct DedustFactoryV2CreateVaultMsg {
    pub query_id: u64,
    pub asset: DedustClassicAsset,
}
/// Request to deploy a legacy Jetton Vault through a resolver.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3383063853, bits_len = 32)]
pub struct DedustFactoryV2CreateLegacyJettonVaultMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub minter_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub resolver_address: TonAddress,
}
/// Request to upgrade the Vault for an existing asset.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 634808593, bits_len = 32)]
pub struct DedustFactoryV2UpgradeVaultMsg {
    pub query_id: u64,
    pub asset: DedustClassicAsset,
}
/// Owner request to replace Factory code.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3746178986, bits_len = 32)]
pub struct DedustFactoryV2UpgradeMsg {
    pub query_id: u64,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
}
/// Owner request to return Factory excess gas.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2671708471, bits_len = 32)]
pub struct DedustFactoryV2ResetGasMsg {
    pub query_id: u64,
}
/// Owner request to deploy an Operator contract for a role and user.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3117586839, bits_len = 32)]
pub struct DedustFactoryV2DeployOperatorMsg {
    pub query_id: u64,
    pub role: u8,
    #[tlb(adapter = "InternalAddress")]
    pub user_address: TonAddress,
}
/// Owner request to change the user address for an existing Operator role.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 468115347, bits_len = 32)]
pub struct DedustFactoryV2ChangeOperatorUserAddressMsg {
    pub query_id: u64,
    pub role: u8,
    #[tlb(adapter = "InternalAddress")]
    pub user_address: TonAddress,
}
/// Owner request to install Operator contract code.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3842363930, bits_len = 32)]
pub struct DedustFactoryV2InstallOperatorCodeMsg {
    pub query_id: u64,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
}
/// Owner request to upgrade an Operator contract for a role.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 8779171, bits_len = 32)]
pub struct DedustFactoryV2UpgradeOperatorMsg {
    pub query_id: u64,
    pub role: u8,
}
/// Owner/operator request to install Pool implementation code.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2749652465, bits_len = 32)]
pub struct DedustFactoryV2InstallPoolCodeMsg {
    pub query_id: u64,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
}
/// Public request to deploy a volatile Pool for two sorted assets.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2547326767, bits_len = 32)]
pub struct DedustFactoryV2CreateVolatilePoolMsg {
    pub query_id: u64,
    pub asset0: DedustClassicAsset,
    pub asset1: DedustClassicAsset,
}
/// Stable-pool-factory operator request to deploy a stable Pool.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2084613255, bits_len = 32)]
pub struct DedustFactoryV2CreateStablePoolMsg {
    pub query_id: u64,
    pub asset0: DedustClassicAsset,
    pub asset0_decimals: u8,
    pub asset1: DedustClassicAsset,
    pub asset1_decimals: u8,
}
/// Owner/operator request to upgrade an existing Pool implementation.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1407341230, bits_len = 32)]
pub struct DedustFactoryV2UpgradePoolMsg {
    pub query_id: u64,
    pub pool_params: DedustFactoryV2PoolParams,
}
/// Owner/operator request to install Liquidity Deposit implementation code.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2577941265, bits_len = 32)]
pub struct DedustFactoryV2InstallLiquidityDepositCodeMsg {
    pub query_id: u64,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
}
/// Vault request to create a Liquidity Deposit for an owner and pool.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 4031694118, bits_len = 32)]
pub struct DedustFactoryV2CreateLiquidityDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustFactoryV2VaultProof>,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    pub pool_params: DedustFactoryV2PoolParams,
    pub extra: TLBRef<DedustFactoryV2CreateLiquidityDepositExtra>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Owner request to remove a non-ready Vault for an asset.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2320600333, bits_len = 32)]
pub struct DedustFactoryV2DestroyNonReadyVaultMsg {
    pub query_id: u64,
    pub asset: DedustClassicAsset,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustFactoryV2IncomingMsg {
    TransferOwnership(Box<DedustFactoryV2TransferOwnershipMsg>),
    AcceptOwnership(Box<DedustFactoryV2AcceptOwnershipMsg>),
    CancelOwnershipTransfer(Box<DedustFactoryV2CancelOwnershipTransferMsg>),
    InstallVaultCode(Box<DedustFactoryV2InstallVaultCodeMsg>),
    CreateVault(Box<DedustFactoryV2CreateVaultMsg>),
    CreateLegacyJettonVault(Box<DedustFactoryV2CreateLegacyJettonVaultMsg>),
    UpgradeVault(Box<DedustFactoryV2UpgradeVaultMsg>),
    DestroyNonReadyVault(Box<DedustFactoryV2DestroyNonReadyVaultMsg>),
    Upgrade(Box<DedustFactoryV2UpgradeMsg>),
    ResetGas(Box<DedustFactoryV2ResetGasMsg>),
    DeployOperator(Box<DedustFactoryV2DeployOperatorMsg>),
    ChangeOperatorUserAddress(Box<DedustFactoryV2ChangeOperatorUserAddressMsg>),
    InstallOperatorCode(Box<DedustFactoryV2InstallOperatorCodeMsg>),
    UpgradeOperator(Box<DedustFactoryV2UpgradeOperatorMsg>),
    InstallPoolCode(Box<DedustFactoryV2InstallPoolCodeMsg>),
    CreateVolatilePool(Box<DedustFactoryV2CreateVolatilePoolMsg>),
    CreateStablePool(Box<DedustFactoryV2CreateStablePoolMsg>),
    UpgradePool(Box<DedustFactoryV2UpgradePoolMsg>),
    InstallLiquidityDepositCode(Box<DedustFactoryV2InstallLiquidityDepositCodeMsg>),
    CreateLiquidityDeposit(Box<DedustFactoryV2CreateLiquidityDepositMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustFactoryV2OutgoingMsg {
    Install(Box<DedustFactoryV2InstallMsg>),
    DepositAsset(Box<DedustFactoryV2DepositAssetMsg>),
    Unknown(TonCell),
}
