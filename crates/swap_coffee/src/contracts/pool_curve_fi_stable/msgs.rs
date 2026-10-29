//! Generated Coffee msgs. See tools/reference/swap_coffee/README.md.
use crate::contracts::pool_constant_product::msgs::CoffeePoolContractUpdate;
use crate::contracts::pool_constant_product::msgs::CoffeePoolDepositLiquidityParamsTrimmed;
use crate::contracts::pool_constant_product::msgs::CoffeePoolNotificationData;
use crate::contracts::pool_constant_product::msgs::CoffeePoolNotificationDataSingle;
use crate::contracts::pool_constant_product::msgs::CoffeePoolSwapParams;
use crate::contracts::pool_constant_product::msgs::CoffeePoolSwapStepInternalParams;
use crate::contracts::pool_constant_product::msgs::CoffeePoolUpdateParams;
use crate::contracts::pool_constant_product::msgs::CoffeePoolValuePair;
use crate::contracts::pool_constant_product::msgs::CoffeePoolWithdrawLiquidityParams;
use crate::contracts::pool_constant_product::storage::CoffeePoolAsset;
use crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolProof;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::{InternalAddress, NullableAddress};
use ton::ton_core::types::tlb_core::{TLBCoins, TLBEither, TLBRef};
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee20, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolSwapInternalMsg {
    pub query_id: u64,
    pub step: CoffeePoolSwapStepInternalParams,
    pub params: TLBRef<CoffeePoolSwapParams>,
    pub proof: TLBRef<CoffeeCurveFiStablePoolProof>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee29, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolDepositLiquidityInternalMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub tx_initiator: TonAddress,
    pub params: TLBRef<CoffeePoolDepositLiquidityParamsTrimmed>,
    pub proof: TLBRef<CoffeeCurveFiStablePoolProof>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7bdd97de, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolJettonBurnNotificationMsg {
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
pub struct CoffeeCurveFiStablePoolProvideWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    pub include_address: bool,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2d, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolWithdrawInternalMsg {
    pub query_id: u64,
    pub asset: CoffeePoolAsset,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub receiver: TonAddress,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2b, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolUpdatePoolInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_receiver: Option<TonAddress>,
    pub params: TLBRef<CoffeePoolUpdateParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2e, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolUpdateContractInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_receiver: Option<TonAddress>,
    pub update: CoffeePoolContractUpdate,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee26, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolCreatePoolInternalMsg {
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
pub struct CoffeeCurveFiStablePoolDeployMsg {
    pub code: TLBRef<TonCell>,
    pub data: TLBRef<TonCell>,
    pub action: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee21, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolPayoutInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    pub amount: TLBCoins,
    pub notification_data: Option<TLBRef<CoffeePoolNotificationDataSingle>>,
    pub proof: Option<TLBRef<CoffeeCurveFiStablePoolProof>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee20, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolSwapInternalOutMsg {
    pub query_id: u64,
    pub step: CoffeePoolSwapStepInternalParams,
    pub params: TLBRef<CoffeePoolSwapParams>,
    pub proof: TLBRef<CoffeeCurveFiStablePoolProof>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee36, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolNotificationMsg {
    pub query_id: u64,
    pub body: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee30, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolSwapSucceededMsg {
    pub query_id: u64,
    pub input: CoffeePoolAsset,
    pub input_amount: TLBCoins,
    pub output_amount: TLBCoins,
    pub reserves: CoffeePoolValuePair,
    pub protocol_fee: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee31, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolSwapFailedMsg {
    pub query_id: u64,
    pub input: CoffeePoolAsset,
    pub input_amount: TLBCoins,
    pub reserves: Option<TLBRef<CoffeePoolValuePair>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee33, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolDepositLiquiditySucceededMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    pub lp_amount: TLBCoins,
    pub total_supply: CoffeePoolValuePair,
    pub reserves: CoffeePoolValuePair,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee34, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolDepositLiquidityFailedMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    pub min_lp_amount: TLBCoins,
    pub total_supply: TLBCoins,
    pub reserves: CoffeePoolValuePair,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee35, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolWithdrawLiquiditySucceededMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    pub lp_amount: TLBCoins,
    pub total_supply: CoffeePoolValuePair,
    pub reserves: CoffeePoolValuePair,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd1735400, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolTakeWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub wallet_address: TonAddress,
    pub owner_address: Option<TLBRef<TonAddress>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolExcessesMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x178d4519, bits_len = 32)]
pub struct CoffeeCurveFiStablePoolLpWalletInternalTransferMsg {
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
pub enum CoffeeCurveFiStablePoolIncomingMessage {
    SwapInternal(CoffeeCurveFiStablePoolSwapInternalMsg),
    DepositLiquidityInternal(CoffeeCurveFiStablePoolDepositLiquidityInternalMsg),
    JettonBurnNotification(CoffeeCurveFiStablePoolJettonBurnNotificationMsg),
    ProvideWalletAddress(CoffeeCurveFiStablePoolProvideWalletAddressMsg),
    WithdrawInternal(CoffeeCurveFiStablePoolWithdrawInternalMsg),
    UpdatePoolInternal(CoffeeCurveFiStablePoolUpdatePoolInternalMsg),
    UpdateContractInternal(CoffeeCurveFiStablePoolUpdateContractInternalMsg),
    CreatePoolInternal(CoffeeCurveFiStablePoolCreatePoolInternalMsg),
    Deploy(CoffeeCurveFiStablePoolDeployMsg),
    Unknown(TonCell),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
// Keep opcode prefixes visible: Box<T> erases them in locked TON dispatch.
#[allow(clippy::large_enum_variant)]
pub enum CoffeeCurveFiStablePoolOutgoingMessage {
    PayoutInternal(CoffeeCurveFiStablePoolPayoutInternalMsg),
    SwapInternalOut(CoffeeCurveFiStablePoolSwapInternalOutMsg),
    Notification(CoffeeCurveFiStablePoolNotificationMsg),
    SwapSucceeded(CoffeeCurveFiStablePoolSwapSucceededMsg),
    SwapFailed(CoffeeCurveFiStablePoolSwapFailedMsg),
    DepositLiquiditySucceeded(CoffeeCurveFiStablePoolDepositLiquiditySucceededMsg),
    DepositLiquidityFailed(CoffeeCurveFiStablePoolDepositLiquidityFailedMsg),
    WithdrawLiquiditySucceeded(CoffeeCurveFiStablePoolWithdrawLiquiditySucceededMsg),
    TakeWalletAddress(CoffeeCurveFiStablePoolTakeWalletAddressMsg),
    Excesses(CoffeeCurveFiStablePoolExcessesMsg),
    LpWalletInternalTransfer(CoffeeCurveFiStablePoolLpWalletInternalTransferMsg),
    Unknown(TonCell),
}
