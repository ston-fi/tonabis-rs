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
/// Runtime storage written by the installed Liquidity Deposit code after the Factory install message.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustLiquidityDepositV2Storage {
    pub proof: TLBRef<DedustLiquidityDepositV2Proof>,
    #[tlb(adapter = "InternalAddress")]
    pub factory_addr: TonAddress,
    pub proof_code: TLBRef<TonCell>,
    #[tlb(adapter = "InternalAddress")]
    pub owner_addr: TonAddress,
    pub asset0_balance: TLBCoins,
    pub asset1_balance: TLBCoins,
    pub is_processing: bool,
    pub min_lp_amount: TLBCoins,
    pub details: TLBRef<DedustLiquidityDepositV2StorageDetails>,
}
/// Referenced Liquidity Deposit runtime tail cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustLiquidityDepositV2StorageDetails {
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
    pub pool_params: DedustLiquidityDepositV2PoolParams,
    pub asset0_target_balance: TLBCoins,
    pub asset1_target_balance: TLBCoins,
}
/// Pool identity embedded in proof and liquidity messages.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustLiquidityDepositV2PoolParams {
    pub is_stable: bool,
    pub asset0: DedustClassicAsset,
    pub asset1: DedustClassicAsset,
}
/// Referenced params proving the Liquidity Deposit address owner and target pool.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustLiquidityDepositV2ProofParams {
    #[tlb(adapter = "InternalAddress")]
    pub owner_addr: TonAddress,
    pub pool_params: DedustLiquidityDepositV2PoolParams,
}
/// Generic DeDust proof specialized to Liquidity Deposit params.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustLiquidityDepositV2Proof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_addr: TonAddress,
    pub contract_type: u8,
    pub params: TLBRef<DedustLiquidityDepositV2ProofParams>,
}
/// Generic DeDust proof specialized to Pool params.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustLiquidityDepositV2PoolProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_addr: TonAddress,
    pub contract_type: u8,
    pub params: DedustLiquidityDepositV2PoolParams,
}
/// Referenced asset/amount pair sent from Liquidity Deposit to Pool.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustLiquidityDepositV2DepositLiquidityAllAssets {
    pub asset0: DedustClassicAsset,
    pub asset0_amount: TLBCoins,
    pub asset1: DedustClassicAsset,
    pub asset1_amount: TLBCoins,
}
/// DeDust Liquidity Deposit v2 exit-code catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DedustLiquidityDepositV2Errors {
    /// Cancellation sender is not the stored deposit owner.
    InvalidOwnerSender = 257,
    /// Asset-deposit sender is not the Factory.
    InvalidFactorySender = 259,
    /// Asset tag or proof shape is invalid for this Liquidity Deposit.
    InvalidAssetOrProof = 261,
    /// Pool callback sender is not the authenticated Pool.
    InvalidPoolSender = 265,
    /// Deposit lifecycle state is invalid for the requested operation.
    InvalidProcessingState = 276,
    /// Incoming internal message opcode is unsupported.
    UnsupportedOperation = 65535,
}
