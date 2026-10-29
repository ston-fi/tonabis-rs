//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{
    DedustLiquidityDepositV1DepositLiquidityAllAssets, DedustLiquidityDepositV1PoolProof, DedustLiquidityDepositV1Proof,
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
/// Blank-installer deployment body received before Liquidity Deposit code is installed.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2604311546, bits_len = 32)]
pub struct DedustLiquidityDepositV1InstallMsg {
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
pub struct DedustLiquidityDepositV1DepositAssetMsg {
    pub query_id: u64,
    pub asset: DedustClassicAsset,
    /// Deposited amount in basic asset units.
    pub amount: TLBCoins,
}
/// Pool-originated success callback after all pending liquidity was accepted.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2867302998, bits_len = 32)]
pub struct DedustLiquidityDepositV1FulfillDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustLiquidityDepositV1PoolProof>,
    pub asset0_deposit: TLBCoins,
    pub asset1_deposit: TLBCoins,
}
/// Pool-originated rejection callback when pending liquidity cannot be accepted.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3785583828, bits_len = 32)]
pub struct DedustLiquidityDepositV1RejectDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustLiquidityDepositV1PoolProof>,
    pub payload: Option<TLBRef<TonCell>>,
}
/// User-originated cancellation request.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 376237550, bits_len = 32)]
pub struct DedustLiquidityDepositV1CancelDepositMsg {
    pub query_id: u64,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Liquidity Deposit to Pool message that attempts to mint LP tokens with all stored balances.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3043726744, bits_len = 32)]
pub struct DedustLiquidityDepositV1DepositLiquidityAllMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustLiquidityDepositV1Proof>,
    #[tlb(adapter = "InternalAddress")]
    pub owner_addr: TonAddress,
    pub min_lp_amount: TLBCoins,
    pub assets: TLBRef<DedustLiquidityDepositV1DepositLiquidityAllAssets>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// Liquidity Deposit to Vault payout request for returning unconsumed funds.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1795913855, bits_len = 32)]
pub struct DedustLiquidityDepositV1PayOutFromDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustLiquidityDepositV1Proof>,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustLiquidityDepositV1IncomingMsg {
    LiquidityDepositInstall(Box<DedustLiquidityDepositV1InstallMsg>),
    DepositAsset(Box<DedustLiquidityDepositV1DepositAssetMsg>),
    FulfillDeposit(Box<DedustLiquidityDepositV1FulfillDepositMsg>),
    RejectDeposit(Box<DedustLiquidityDepositV1RejectDepositMsg>),
    CancelDeposit(Box<DedustLiquidityDepositV1CancelDepositMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustLiquidityDepositV1OutgoingMsg {
    DepositLiquidityAll(Box<DedustLiquidityDepositV1DepositLiquidityAllMsg>),
    PayOutFromDeposit(Box<DedustLiquidityDepositV1PayOutFromDepositMsg>),
    Unknown(TonCell),
}
