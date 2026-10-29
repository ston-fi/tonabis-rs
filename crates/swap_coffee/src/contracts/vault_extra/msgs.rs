//! Generated Coffee msgs. See tools/reference/swap_coffee/README.md.
use crate::contracts::vault_native::msgs::CoffeeCreateLiquidityDepositoryRequestMsg;
use crate::contracts::vault_native::msgs::CoffeeCreatePoolCreatorRequestMsg;
use crate::contracts::vault_native::msgs::CoffeeDepositLiquidityParams;
use crate::contracts::vault_native::msgs::CoffeeExcessesMsg;
use crate::contracts::vault_native::msgs::CoffeeNotificationMsg;
use crate::contracts::vault_native::msgs::CoffeePayoutInternalMsg;
use crate::contracts::vault_native::msgs::CoffeePayoutMsg;
use crate::contracts::vault_native::msgs::CoffeePoolCreationParams;
use crate::contracts::vault_native::msgs::CoffeePoolParams;
use crate::contracts::vault_native::msgs::CoffeeSwapInternalMsg;
use crate::contracts::vault_native::msgs::CoffeeSwapParams;
use crate::contracts::vault_native::msgs::CoffeeSwapStepParams;
use crate::contracts::vault_native::msgs::CoffeeUpdateContractInternalMsg;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::TLBRef;
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee01, bits_len = 32)]
pub struct CoffeeSwapExtraMsg {
    pub query_id: u64,
    pub step: CoffeeSwapStepParams,
    pub params: TLBRef<CoffeeSwapParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee05, bits_len = 32)]
pub struct CoffeeDepositLiquidityExtraMsg {
    pub query_id: u64,
    pub params: CoffeeDepositLiquidityParams,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee03, bits_len = 32)]
pub struct CoffeeCreatePoolExtraMsg {
    pub query_id: u64,
    pub params: CoffeePoolParams,
    pub creation_params: CoffeePoolCreationParams,
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeVaultExtraIncomingMessage {
    CoffeePayoutInternal(CoffeePayoutInternalMsg),
    CoffeeSwapExtra(CoffeeSwapExtraMsg),
    CoffeeDepositLiquidityExtra(CoffeeDepositLiquidityExtraMsg),
    CoffeeCreatePoolExtra(CoffeeCreatePoolExtraMsg),
    CoffeeUpdateContractInternal(CoffeeUpdateContractInternalMsg),
    Unknown(TonCell),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
// Keep opcode prefixes visible: Box<T> erases them in locked TON dispatch.
#[allow(clippy::large_enum_variant)]
pub enum CoffeeVaultExtraOutgoingMessage {
    CoffeeNotification(CoffeeNotificationMsg),
    CoffeePayout(CoffeePayoutMsg),
    CoffeeSwapInternal(CoffeeSwapInternalMsg),
    CoffeeCreateLiquidityDepositoryRequest(CoffeeCreateLiquidityDepositoryRequestMsg),
    CoffeeCreatePoolCreatorRequest(CoffeeCreatePoolCreatorRequestMsg),
    CoffeeExcesses(CoffeeExcessesMsg),
    Unknown(TonCell),
}
