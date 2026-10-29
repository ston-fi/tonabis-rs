//! Generated Coffee msgs. See tools/reference/swap_coffee/README.md.
use crate::contracts::vault_native::storage::CoffeeAsset;
use crate::contracts::vault_native::storage::CoffeeVaultInitData;
use crate::contracts::vault_proof::CoffeeVaultContractProof;
use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::{InternalAddress, NullableAddress};
use ton::ton_core::types::tlb_core::{TLBCoins, TLBRef};
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b0, bits_len = 1)]
pub struct CoffeeNoPreviousAssetHint {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b1, bits_len = 1)]
pub struct CoffeeSomePreviousAssetHint {
    pub asset: CoffeeAsset,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b000, bits_len = 3)]
pub struct CoffeeConstantProductAmm {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b001, bits_len = 3)]
pub struct CoffeeCurveFiStableAmm {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b00, bits_len = 2)]
pub struct CoffeeNoDepositLiquidityCondition {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeeLpQuantityDepositLiquidityCondition {
    pub min_lp_amount: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeeReservesRatioDepositLiquidityCondition {
    pub denominator: u16,
    pub min_nominator: u16,
    pub max_nominator: u16,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b11, bits_len = 2)]
pub struct CoffeeComplexDepositLiquidityCondition {
    pub min_lp_amount: TLBCoins,
    pub denominator: u16,
    pub min_nominator: u16,
    pub max_nominator: u16,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolParams {
    pub first: CoffeeAsset,
    pub second: CoffeeAsset,
    pub amm: CoffeeAmm,
    pub amm_settings: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePublicPoolCreationParams {
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(bits_len = 1)]
    pub use_recipient_on_failure: i8,
    pub notification_data: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePrivatePoolCreationParams {
    #[tlb(bits_len = 1)]
    pub is_active: u8,
    pub extra_settings: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolCreationParams {
    pub public_value: CoffeePublicPoolCreationParams,
    pub private_value: CoffeePrivatePoolCreationParams,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeNotificationDataSingle {
    #[tlb(adapter = "NullableAddress")]
    pub receiver: Option<TonAddress>,
    pub fwd_gas: TLBCoins,
    pub payload: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeNotificationData {
    pub on_success: Option<TLBRef<CoffeeNotificationDataSingle>>,
    pub on_failure: Option<TLBRef<CoffeeNotificationDataSingle>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeSwapParams {
    pub deadline: u32,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub referral: Option<TonAddress>,
    pub notification_data: Option<TLBRef<CoffeeNotificationData>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeSwapStepParams {
    pub pool_address_hash: U256,
    pub min_output_amount: TLBCoins,
    pub next: Option<TLBRef<Box<CoffeeSwapStepParams>>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeSwapStepInternalParams {
    pub previous_amount: TLBCoins,
    pub previous_asset_hint: CoffeePreviousAssetHint,
    pub min_output_amount: TLBCoins,
    pub next: Option<TLBRef<Box<CoffeeSwapStepParams>>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeDepositLiquidityParamsTrimmed {
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(bits_len = 1)]
    pub use_recipient_on_failure: i8,
    #[tlb(adapter = "InternalAddress")]
    pub referral: TonAddress,
    pub deadline: u32,
    pub condition: CoffeeDepositLiquidityCondition,
    pub extra_settings: Option<TLBRef<TonCell>>,
    pub notification_data: Option<TLBRef<CoffeeNotificationData>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeDepositLiquidityParams {
    pub params: TLBRef<CoffeeDepositLiquidityParamsTrimmed>,
    pub pool_params: TLBRef<CoffeePoolParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeContractUpdate {
    pub code: Option<TLBRef<TonCell>>,
    pub data: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee21, bits_len = 32)]
pub struct CoffeePayoutInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    pub amount: TLBCoins,
    pub notification_data: Option<TLBRef<CoffeeNotificationDataSingle>>,
    pub proof: Option<TLBRef<CoffeeVaultContractProof>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee00, bits_len = 32)]
pub struct CoffeeSwapNativeMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub step: CoffeeSwapStepParams,
    pub params: TLBRef<CoffeeSwapParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee04, bits_len = 32)]
pub struct CoffeeDepositLiquidityNativeMsg {
    pub query_id: u64,
    pub input_amount: TLBCoins,
    pub params: CoffeeDepositLiquidityParams,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee02, bits_len = 32)]
pub struct CoffeeCreatePoolNativeMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub params: CoffeePoolParams,
    pub creation_params: CoffeePoolCreationParams,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2e, bits_len = 32)]
pub struct CoffeeUpdateContractInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub excesses_receiver: TonAddress,
    pub update: CoffeeContractUpdate,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee36, bits_len = 32)]
pub struct CoffeeNotificationMsg {
    pub query_id: u64,
    pub body: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee32, bits_len = 32)]
pub struct CoffeePayoutMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee20, bits_len = 32)]
pub struct CoffeeSwapInternalMsg {
    pub query_id: u64,
    pub step: CoffeeSwapStepInternalParams,
    pub params: TLBRef<CoffeeSwapParams>,
    pub proof: TLBRef<CoffeeVaultInitData>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee27, bits_len = 32)]
pub struct CoffeeCreateLiquidityDepositoryRequestMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub params: CoffeeDepositLiquidityParams,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    pub proof: TLBRef<CoffeeVaultInitData>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee23, bits_len = 32)]
pub struct CoffeeCreatePoolCreatorRequestMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub params: CoffeePoolParams,
    pub creation_params: TLBRef<CoffeePoolCreationParams>,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    pub proof: TLBRef<CoffeeVaultInitData>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct CoffeeExcessesMsg {
    pub query_id: u64,
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeePreviousAssetHint {
    CoffeeNoPreviousAssetHint(CoffeeNoPreviousAssetHint),
    CoffeeSomePreviousAssetHint(CoffeeSomePreviousAssetHint),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeAmm {
    CoffeeConstantProductAmm(CoffeeConstantProductAmm),
    CoffeeCurveFiStableAmm(CoffeeCurveFiStableAmm),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeDepositLiquidityCondition {
    CoffeeNoDepositLiquidityCondition(CoffeeNoDepositLiquidityCondition),
    CoffeeLpQuantityDepositLiquidityCondition(CoffeeLpQuantityDepositLiquidityCondition),
    CoffeeReservesRatioDepositLiquidityCondition(CoffeeReservesRatioDepositLiquidityCondition),
    CoffeeComplexDepositLiquidityCondition(CoffeeComplexDepositLiquidityCondition),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeVaultNativeIncomingMessage {
    CoffeePayoutInternal(CoffeePayoutInternalMsg),
    CoffeeSwapNative(CoffeeSwapNativeMsg),
    CoffeeDepositLiquidityNative(CoffeeDepositLiquidityNativeMsg),
    CoffeeCreatePoolNative(CoffeeCreatePoolNativeMsg),
    CoffeeUpdateContractInternal(CoffeeUpdateContractInternalMsg),
    Unknown(TonCell),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
// Keep opcode prefixes visible: Box<T> erases them in locked TON dispatch.
#[allow(clippy::large_enum_variant)]
pub enum CoffeeVaultNativeOutgoingMessage {
    CoffeeNotification(CoffeeNotificationMsg),
    CoffeePayout(CoffeePayoutMsg),
    CoffeeSwapInternal(CoffeeSwapInternalMsg),
    CoffeeCreateLiquidityDepositoryRequest(CoffeeCreateLiquidityDepositoryRequestMsg),
    CoffeeCreatePoolCreatorRequest(CoffeeCreatePoolCreatorRequestMsg),
    CoffeeExcesses(CoffeeExcessesMsg),
    Unknown(TonCell),
}
