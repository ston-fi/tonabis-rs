//! Generated Coffee msgs. See tools/reference/swap_coffee/README.md.
use crate::contracts::pool_creator::storage::CoffeePoolCreatorAsset;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorInitData;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::{InternalAddress, NullableAddress};
use ton::ton_core::types::tlb_core::{TLBCoins, TLBRef};
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolCreatorNotificationDataSingle {
    #[tlb(adapter = "NullableAddress")]
    pub receiver: Option<TonAddress>,
    pub fwd_gas: TLBCoins,
    pub payload: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolCreatorPublicPoolCreationParams {
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(bits_len = 1)]
    pub use_recipient_on_failure: i8,
    pub notification_data: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolCreatorPrivatePoolCreationParams {
    #[tlb(bits_len = 1)]
    pub is_active: u8,
    pub extra_settings: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolCreatorPoolCreationParams {
    pub public_params: CoffeePoolCreatorPublicPoolCreationParams,
    pub private_params: CoffeePoolCreatorPrivatePoolCreationParams,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeePoolCreatorContractUpdate {
    pub code: Option<TLBRef<TonCell>>,
    pub data: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee22, bits_len = 32)]
pub struct CoffeePoolCreatorDeployMsg {
    pub query_id: u64,
    pub code: TLBRef<TonCell>,
    pub data: TLBRef<TonCell>,
    pub action: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee24, bits_len = 32)]
pub struct CoffeePoolCreatorCreatePoolCreatorInternalMsg {
    pub query_id: u64,
    pub asset: CoffeePoolCreatorAsset,
    pub amount: TLBCoins,
    pub creation_params: TLBRef<CoffeePoolCreatorPoolCreationParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee07, bits_len = 32)]
pub struct CoffeePoolCreatorWithdrawDepositMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2e, bits_len = 32)]
pub struct CoffeePoolCreatorUpdateContractInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub excesses_receiver: TonAddress,
    pub update: CoffeePoolCreatorContractUpdate,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee25, bits_len = 32)]
pub struct CoffeePoolCreatorCreatePoolRequestMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub tx_initiator: TonAddress,
    pub creation_params: TLBRef<CoffeePoolCreatorPoolCreationParams>,
    pub proof: TLBRef<CoffeePoolCreatorInitData>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee21, bits_len = 32)]
pub struct CoffeePoolCreatorPayoutInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    pub amount: TLBCoins,
    pub notification_data: Option<TLBRef<CoffeePoolCreatorNotificationDataSingle>>,
    pub proof: Option<TLBRef<CoffeePoolCreatorInitData>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct CoffeePoolCreatorExcessesMsg {
    pub query_id: u64,
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeePoolCreatorIncomingMessage {
    CreatePoolCreatorInternal(CoffeePoolCreatorCreatePoolCreatorInternalMsg),
    Deploy(CoffeePoolCreatorDeployMsg),
    WithdrawDeposit(CoffeePoolCreatorWithdrawDepositMsg),
    UpdateContractInternal(CoffeePoolCreatorUpdateContractInternalMsg),
    Unknown(TonCell),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeePoolCreatorOutgoingMessage {
    CreatePoolRequest(CoffeePoolCreatorCreatePoolRequestMsg),
    PayoutInternal(CoffeePoolCreatorPayoutInternalMsg),
    Excesses(CoffeePoolCreatorExcessesMsg),
    Unknown(TonCell),
}
