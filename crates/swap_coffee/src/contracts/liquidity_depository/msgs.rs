//! Generated Coffee msgs. See tools/reference/swap_coffee/README.md.
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryAsset;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryInitData;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::{InternalAddress, NullableAddress};
use ton::ton_core::types::tlb_core::{TLBCoins, TLBRef};
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b00, bits_len = 2)]
pub struct CoffeeLiquidityDepositoryDepositConditionNone {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeeLiquidityDepositoryDepositConditionLpQuantity {
    pub min_lp_amount: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeeLiquidityDepositoryDepositConditionReservesRatio {
    pub denominator: u16,
    pub min_nominator: u16,
    pub max_nominator: u16,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b11, bits_len = 2)]
pub struct CoffeeLiquidityDepositoryDepositConditionComplex {
    pub min_lp_amount: TLBCoins,
    pub denominator: u16,
    pub min_nominator: u16,
    pub max_nominator: u16,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeLiquidityDepositoryNotificationDataSingle {
    #[tlb(adapter = "NullableAddress")]
    pub receiver: Option<TonAddress>,
    pub fwd_gas: TLBCoins,
    pub payload: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeLiquidityDepositoryNotificationData {
    pub on_success: Option<TLBRef<CoffeeLiquidityDepositoryNotificationDataSingle>>,
    pub on_failure: Option<TLBRef<CoffeeLiquidityDepositoryNotificationDataSingle>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeLiquidityDepositoryDepositLiquidityParamsTrimmed {
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(bits_len = 1)]
    pub use_recipient_on_failure: i8,
    #[tlb(adapter = "InternalAddress")]
    pub referral: TonAddress,
    pub deadline: u32,
    pub condition: CoffeeLiquidityDepositoryDepositLiquidityCondition,
    pub extra_settings: Option<TLBRef<TonCell>>,
    pub notification_data: Option<TLBRef<CoffeeLiquidityDepositoryNotificationData>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeLiquidityDepositoryContractUpdate {
    pub code: Option<TLBRef<TonCell>>,
    pub data: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee22, bits_len = 32)]
pub struct CoffeeLiquidityDepositoryDeployMsg {
    pub query_id: u64,
    pub code: TLBRef<TonCell>,
    pub data: TLBRef<TonCell>,
    pub action: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee28, bits_len = 32)]
pub struct CoffeeLiquidityDepositoryCreateLiquidityDepositoryInternalMsg {
    pub query_id: u64,
    pub asset: CoffeeLiquidityDepositoryAsset,
    pub amount: TLBCoins,
    pub params: TLBRef<CoffeeLiquidityDepositoryDepositLiquidityParamsTrimmed>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee07, bits_len = 32)]
pub struct CoffeeLiquidityDepositoryWithdrawDepositMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2e, bits_len = 32)]
pub struct CoffeeLiquidityDepositoryUpdateContractInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub excesses_receiver: TonAddress,
    pub update: CoffeeLiquidityDepositoryContractUpdate,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee29, bits_len = 32)]
pub struct CoffeeLiquidityDepositoryDepositLiquidityInternalMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub tx_initiator: TonAddress,
    pub params: TLBRef<CoffeeLiquidityDepositoryDepositLiquidityParamsTrimmed>,
    pub proof: TLBRef<CoffeeLiquidityDepositoryInitData>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee21, bits_len = 32)]
pub struct CoffeeLiquidityDepositoryPayoutInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    pub amount: TLBCoins,
    pub notification_data: Option<TLBRef<CoffeeLiquidityDepositoryNotificationDataSingle>>,
    pub proof: Option<TLBRef<CoffeeLiquidityDepositoryInitData>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct CoffeeLiquidityDepositoryExcessesMsg {
    pub query_id: u64,
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeLiquidityDepositoryDepositLiquidityCondition {
    DepositConditionNone(CoffeeLiquidityDepositoryDepositConditionNone),
    DepositConditionLpQuantity(CoffeeLiquidityDepositoryDepositConditionLpQuantity),
    DepositConditionReservesRatio(CoffeeLiquidityDepositoryDepositConditionReservesRatio),
    DepositConditionComplex(CoffeeLiquidityDepositoryDepositConditionComplex),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
// Keep opcode prefixes visible: Box<T> erases them in locked TON dispatch.
#[allow(clippy::large_enum_variant)]
pub enum CoffeeLiquidityDepositoryIncomingMessage {
    CreateLiquidityDepositoryInternal(CoffeeLiquidityDepositoryCreateLiquidityDepositoryInternalMsg),
    Deploy(CoffeeLiquidityDepositoryDeployMsg),
    WithdrawDeposit(CoffeeLiquidityDepositoryWithdrawDepositMsg),
    UpdateContractInternal(CoffeeLiquidityDepositoryUpdateContractInternalMsg),
    Unknown(TonCell),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
// Keep opcode prefixes visible: Box<T> erases them in locked TON dispatch.
#[allow(clippy::large_enum_variant)]
pub enum CoffeeLiquidityDepositoryOutgoingMessage {
    DepositLiquidityInternal(CoffeeLiquidityDepositoryDepositLiquidityInternalMsg),
    PayoutInternal(CoffeeLiquidityDepositoryPayoutInternalMsg),
    Excesses(CoffeeLiquidityDepositoryExcessesMsg),
    Unknown(TonCell),
}
