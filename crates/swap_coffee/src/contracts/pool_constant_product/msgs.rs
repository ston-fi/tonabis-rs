//! Generated Coffee msgs. See tools/reference/swap_coffee/README.md.
use crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolProof;
use crate::contracts::pool_constant_product::storage::CoffeePoolAsset;
use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::{InternalAddress, NullableAddress};
use ton::ton_core::types::tlb_core::{TLBCoins, TLBEither, TLBRef};
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b0, bits_len = 1)]
pub struct CoffeePoolPreviousAssetHintAbsent {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b100, bits_len = 3)]
pub struct CoffeePoolPreviousNativeAssetHint {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b101, bits_len = 3)]
pub struct CoffeePoolPreviousJettonAssetHint {
    pub chain: u8,
    pub hash: U256,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b110, bits_len = 3)]
pub struct CoffeePoolPreviousExtraAssetHint {
    pub id: u32,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolNotificationDataSingle {
    #[tlb(adapter = "NullableAddress")]
    pub receiver: Option<TonAddress>,
    pub fwd_gas: TLBCoins,
    pub payload: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolNotificationData {
    pub on_success: Option<TLBRef<CoffeePoolNotificationDataSingle>>,
    pub on_failure: Option<TLBRef<CoffeePoolNotificationDataSingle>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolSwapParams {
    pub deadline: u32,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub referral: Option<TonAddress>,
    pub notification_data: Option<TLBRef<CoffeePoolNotificationData>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolSwapStepParams {
    pub pool_address_hash: U256,
    pub min_output_amount: TLBCoins,
    pub next: Option<TLBRef<Box<CoffeePoolSwapStepParams>>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolSwapStepInternalParams {
    pub previous_amount: TLBCoins,
    pub previous_asset_hint: CoffeePoolPreviousAssetHint,
    pub min_output_amount: TLBCoins,
    pub next: Option<TLBRef<Box<CoffeePoolSwapStepParams>>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b00, bits_len = 2)]
pub struct CoffeePoolDepositConditionNone {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeePoolDepositConditionLpQuantity {
    pub min_lp_amount: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeePoolDepositConditionReservesRatio {
    pub denominator: u16,
    pub min_nominator: u16,
    pub max_nominator: u16,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b11, bits_len = 2)]
pub struct CoffeePoolDepositConditionComplex {
    pub min_lp_amount: TLBCoins,
    pub denominator: u16,
    pub min_nominator: u16,
    pub max_nominator: u16,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolDepositLiquidityParamsTrimmed {
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(bits_len = 1)]
    pub use_recipient_on_failure: i8,
    #[tlb(adapter = "NullableAddress")]
    pub referral: Option<TonAddress>,
    pub deadline: u32,
    pub condition: CoffeePoolDepositLiquidityCondition,
    pub extra_settings: Option<TLBRef<TonCell>>,
    pub notification_data: Option<TLBRef<CoffeePoolNotificationData>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b00, bits_len = 2)]
pub struct CoffeePoolWithdrawConditionNone {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeePoolWithdrawConditionAssetsQuantity {
    pub min_first_amount: TLBCoins,
    pub min_second_amount: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeePoolWithdrawConditionReservesRatio {
    pub denominator: u16,
    pub min_nominator: u16,
    pub max_nominator: u16,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b11, bits_len = 2)]
pub struct CoffeePoolWithdrawConditionComplex {
    pub min_first_amount: TLBCoins,
    pub min_second_amount: TLBCoins,
    pub denominator: u16,
    pub min_nominator: u16,
    pub max_nominator: u16,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolWithdrawLiquidityParams {
    #[tlb(bits_len = 1)]
    pub use_recipient_on_failure: i8,
    pub deadline: u32,
    pub condition: CoffeePoolWithdrawLiquidityCondition,
    pub extra_settings: Option<TLBRef<TonCell>>,
    pub on_success: Option<TLBRef<CoffeePoolNotificationDataSingle>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolContractUpdate {
    pub code: Option<TLBRef<TonCell>>,
    pub data: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b00, bits_len = 2)]
pub struct CoffeePoolUpdateNoop {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeePoolUpdateFees {
    pub protocol_fee: u16,
    pub lp_fee: u16,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeePoolUpdateActive {
    pub is_active: bool,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b11, bits_len = 2)]
pub struct CoffeePoolUpdateFeesAndActive {
    pub protocol_fee: u16,
    pub lp_fee: u16,
    pub is_active: bool,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolValuePair {
    pub first: TLBCoins,
    pub second: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee20, bits_len = 32)]
pub struct CoffeeConstantProductPoolSwapInternalMsg {
    pub query_id: u64,
    pub step: CoffeePoolSwapStepInternalParams,
    pub params: TLBRef<CoffeePoolSwapParams>,
    pub proof: TLBRef<CoffeeConstantProductPoolProof>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee29, bits_len = 32)]
pub struct CoffeeConstantProductPoolDepositLiquidityInternalMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub tx_initiator: TonAddress,
    pub params: TLBRef<CoffeePoolDepositLiquidityParamsTrimmed>,
    pub proof: TLBRef<CoffeeConstantProductPoolProof>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7bdd97de, bits_len = 32)]
pub struct CoffeeConstantProductPoolJettonBurnNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub from_address: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<CoffeePoolWithdrawLiquidityParams>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x2c76b973, bits_len = 32)]
pub struct CoffeeConstantProductPoolProvideWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    pub include_address: bool,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2d, bits_len = 32)]
pub struct CoffeeConstantProductPoolWithdrawInternalMsg {
    pub query_id: u64,
    pub asset: CoffeePoolAsset,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub receiver: TonAddress,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2b, bits_len = 32)]
pub struct CoffeeConstantProductPoolUpdatePoolInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_receiver: Option<TonAddress>,
    pub params: TLBRef<CoffeePoolUpdateParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2e, bits_len = 32)]
pub struct CoffeeConstantProductPoolUpdateContractInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_receiver: Option<TonAddress>,
    pub update: CoffeePoolContractUpdate,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee26, bits_len = 32)]
pub struct CoffeeConstantProductPoolCreatePoolInternalMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub tx_initiator: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(bits_len = 1)]
    pub use_recipient_on_failure: i8,
    pub extra_settings: Option<TLBRef<TonCell>>,
    pub notification_data: Option<TLBRef<CoffeePoolNotificationData>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee22, bits_len = 32)]
pub struct CoffeeConstantProductPoolDeployMsg {
    pub code: TLBRef<TonCell>,
    pub data: TLBRef<TonCell>,
    pub action: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee21, bits_len = 32)]
pub struct CoffeeConstantProductPoolPayoutInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    pub amount: TLBCoins,
    pub notification_data: Option<TLBRef<CoffeePoolNotificationDataSingle>>,
    pub proof: Option<TLBRef<CoffeeConstantProductPoolProof>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee20, bits_len = 32)]
pub struct CoffeeConstantProductPoolSwapInternalOutMsg {
    pub query_id: u64,
    pub step: CoffeePoolSwapStepInternalParams,
    pub params: TLBRef<CoffeePoolSwapParams>,
    pub proof: TLBRef<CoffeeConstantProductPoolProof>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee36, bits_len = 32)]
pub struct CoffeeConstantProductPoolNotificationMsg {
    pub query_id: u64,
    pub body: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee30, bits_len = 32)]
pub struct CoffeeConstantProductPoolSwapSucceededMsg {
    pub query_id: u64,
    pub input: CoffeePoolAsset,
    pub input_amount: TLBCoins,
    pub output_amount: TLBCoins,
    pub reserves: CoffeePoolValuePair,
    pub protocol_fee: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee31, bits_len = 32)]
pub struct CoffeeConstantProductPoolSwapFailedMsg {
    pub query_id: u64,
    pub input: CoffeePoolAsset,
    pub input_amount: TLBCoins,
    pub reserves: Option<TLBRef<CoffeePoolValuePair>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee33, bits_len = 32)]
pub struct CoffeeConstantProductPoolDepositLiquiditySucceededMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    pub lp_amount: TLBCoins,
    pub total_supply: CoffeePoolValuePair,
    pub reserves: CoffeePoolValuePair,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee34, bits_len = 32)]
pub struct CoffeeConstantProductPoolDepositLiquidityFailedMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    pub min_lp_amount: TLBCoins,
    pub total_supply: TLBCoins,
    pub reserves: CoffeePoolValuePair,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee35, bits_len = 32)]
pub struct CoffeeConstantProductPoolWithdrawLiquiditySucceededMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    pub lp_amount: TLBCoins,
    pub total_supply: CoffeePoolValuePair,
    pub reserves: CoffeePoolValuePair,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd1735400, bits_len = 32)]
pub struct CoffeeConstantProductPoolTakeWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub wallet_address: TonAddress,
    pub owner_address: Option<TLBRef<TonAddress>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct CoffeeConstantProductPoolExcessesMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x178d4519, bits_len = 32)]
pub struct CoffeeConstantProductPoolLpWalletInternalTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub from_address: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub response_address: TonAddress,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeePoolPreviousAssetHint {
    CoffeePoolPreviousAssetHintAbsent(CoffeePoolPreviousAssetHintAbsent),
    CoffeePoolPreviousNativeAssetHint(CoffeePoolPreviousNativeAssetHint),
    CoffeePoolPreviousJettonAssetHint(CoffeePoolPreviousJettonAssetHint),
    CoffeePoolPreviousExtraAssetHint(CoffeePoolPreviousExtraAssetHint),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeePoolDepositLiquidityCondition {
    CoffeePoolDepositConditionNone(CoffeePoolDepositConditionNone),
    CoffeePoolDepositConditionLpQuantity(CoffeePoolDepositConditionLpQuantity),
    CoffeePoolDepositConditionReservesRatio(CoffeePoolDepositConditionReservesRatio),
    CoffeePoolDepositConditionComplex(CoffeePoolDepositConditionComplex),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeePoolWithdrawLiquidityCondition {
    CoffeePoolWithdrawConditionNone(CoffeePoolWithdrawConditionNone),
    CoffeePoolWithdrawConditionAssetsQuantity(CoffeePoolWithdrawConditionAssetsQuantity),
    CoffeePoolWithdrawConditionReservesRatio(CoffeePoolWithdrawConditionReservesRatio),
    CoffeePoolWithdrawConditionComplex(CoffeePoolWithdrawConditionComplex),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeePoolUpdateParams {
    CoffeePoolUpdateNoop(CoffeePoolUpdateNoop),
    CoffeePoolUpdateFees(CoffeePoolUpdateFees),
    CoffeePoolUpdateActive(CoffeePoolUpdateActive),
    CoffeePoolUpdateFeesAndActive(CoffeePoolUpdateFeesAndActive),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeConstantProductPoolIncomingMessage {
    SwapInternal(CoffeeConstantProductPoolSwapInternalMsg),
    DepositLiquidityInternal(CoffeeConstantProductPoolDepositLiquidityInternalMsg),
    JettonBurnNotification(CoffeeConstantProductPoolJettonBurnNotificationMsg),
    ProvideWalletAddress(CoffeeConstantProductPoolProvideWalletAddressMsg),
    WithdrawInternal(CoffeeConstantProductPoolWithdrawInternalMsg),
    UpdatePoolInternal(CoffeeConstantProductPoolUpdatePoolInternalMsg),
    UpdateContractInternal(CoffeeConstantProductPoolUpdateContractInternalMsg),
    CreatePoolInternal(CoffeeConstantProductPoolCreatePoolInternalMsg),
    Deploy(CoffeeConstantProductPoolDeployMsg),
    Unknown(TonCell),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
// Keep opcode prefixes visible: Box<T> erases them in locked TON dispatch.
#[allow(clippy::large_enum_variant)]
pub enum CoffeeConstantProductPoolOutgoingMessage {
    PayoutInternal(CoffeeConstantProductPoolPayoutInternalMsg),
    SwapInternalOut(CoffeeConstantProductPoolSwapInternalOutMsg),
    Notification(CoffeeConstantProductPoolNotificationMsg),
    SwapSucceeded(CoffeeConstantProductPoolSwapSucceededMsg),
    SwapFailed(CoffeeConstantProductPoolSwapFailedMsg),
    DepositLiquiditySucceeded(CoffeeConstantProductPoolDepositLiquiditySucceededMsg),
    DepositLiquidityFailed(CoffeeConstantProductPoolDepositLiquidityFailedMsg),
    WithdrawLiquiditySucceeded(CoffeeConstantProductPoolWithdrawLiquiditySucceededMsg),
    TakeWalletAddress(CoffeeConstantProductPoolTakeWalletAddressMsg),
    Excesses(CoffeeConstantProductPoolExcessesMsg),
    LpWalletInternalTransfer(CoffeeConstantProductPoolLpWalletInternalTransferMsg),
    Unknown(TonCell),
}
