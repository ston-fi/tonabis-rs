//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{
    DedustLiquidityDepositV2DepositLiquidityAllAssets, DedustLiquidityDepositV2PoolProof, DedustLiquidityDepositV2Proof,
};
use crate::contracts::classic::asset::DedustClassicAsset;
use crate::contracts::classic::common::DedustClassicDepositParameters;
use ton::ton_core::{
    TLB,
    cell::TonCell,
    types::{
        TonAddress,
        tlb_core::{TLBCoins, TLBRef, adapters::InternalAddress},
    },
};
/// Blank-installer deployment body received by the address before Liquidity Deposit code is installed.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2604311546, bits_len = 32)]
pub struct DedustLiquidityDepositV2InstallMsg {
    pub query_id: u64,
    pub blank_code: TLBRef<TonCell>,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
    /// Fixed initialized-program arguments: minLP, targets, fulfillment/rejection refs, then source-ignored suffix. See classic_initializers.test.tolk.
    pub extra_params: DedustClassicDepositParameters,
}
/// Factory-originated notice that one side of the pending deposit was received.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1411649509, bits_len = 32)]
pub struct DedustLiquidityDepositV2DepositAssetMsg {
    pub query_id: u64,
    pub asset: DedustClassicAsset,
    /// Deposited amount in basic asset units.
    pub amount: TLBCoins,
}
/// Pool-originated success callback after all pending liquidity was accepted.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2867302998, bits_len = 32)]
pub struct DedustLiquidityDepositV2FulfillDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustLiquidityDepositV2PoolProof>,
    pub asset0_deposit: TLBCoins,
    pub asset1_deposit: TLBCoins,
}
/// Pool-originated rejection callback when pending liquidity cannot be accepted.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3785583828, bits_len = 32)]
pub struct DedustLiquidityDepositV2RejectDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustLiquidityDepositV2PoolProof>,
    pub payload: Option<TLBRef<TonCell>>,
}
/// User-originated cancellation request documented by DeDust docs and SDK.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 376237550, bits_len = 32)]
pub struct DedustLiquidityDepositV2CancelDepositMsg {
    pub query_id: u64,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Liquidity Deposit to Pool message that attempts to mint LP tokens with all stored balances.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3043726744, bits_len = 32)]
pub struct DedustLiquidityDepositV2DepositLiquidityAllMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustLiquidityDepositV2Proof>,
    #[tlb(adapter = "InternalAddress")]
    pub owner_addr: TonAddress,
    pub min_lp_amount: TLBCoins,
    pub assets: TLBRef<DedustLiquidityDepositV2DepositLiquidityAllAssets>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Liquidity Deposit to Vault payout request for returning unconsumed funds.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1795913855, bits_len = 32)]
pub struct DedustLiquidityDepositV2PayOutFromDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustLiquidityDepositV2Proof>,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustLiquidityDepositV2IncomingMsg {
    LiquidityDepositInstall(Box<DedustLiquidityDepositV2InstallMsg>),
    DepositAsset(Box<DedustLiquidityDepositV2DepositAssetMsg>),
    FulfillDeposit(Box<DedustLiquidityDepositV2FulfillDepositMsg>),
    RejectDeposit(Box<DedustLiquidityDepositV2RejectDepositMsg>),
    CancelDeposit(Box<DedustLiquidityDepositV2CancelDepositMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustLiquidityDepositV2OutgoingMsg {
    DepositLiquidityAll(Box<DedustLiquidityDepositV2DepositLiquidityAllMsg>),
    PayOutFromDeposit(Box<DedustLiquidityDepositV2PayOutFromDepositMsg>),
    Unknown(TonCell),
}
