//! Generated Coffee msgs. See tools/reference/swap_coffee/README.md.
use crate::contracts::cross_dex::msgs::CoffeeCrossDexJettonTransferNotificationMsg;
use crate::contracts::staking::CoffeeStakingJettonConfiguration;
use crate::contracts::staking::CoffeeStakingPeriod;
use crate::contracts::staking_item::storage::CoffeeStakingAssetData;
use crate::contracts::staking_item::storage::CoffeeStakingPositionData;
use crate::contracts::staking_master::msgs::CoffeeStakingMasterUnlockToVaultMsg;
use std::collections::HashMap;
use ton::tlb_adapters::{DictKeyAdapterTonHash, DictKeyAdapterUint, DictValAdapterTLB, TLBHashMapE};
use ton::ton_core::TLB;
use ton::ton_core::cell::{TonCell, TonHash};
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::{InternalAddress, NullableAddress};
use ton::ton_core::types::tlb_core::{TLBCoins, TLBEither, TLBRef};
use tonabis_core::msgs::EmptyBodyMsg;
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0c0ffede, bits_len = 32)]
pub struct CoffeeStakingLockForwardPayloadMsg {
    pub period_id: u32,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xbe5a7595, bits_len = 32)]
pub struct CoffeeStakingVaultInitMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB<TLBRef<CoffeeStakingJettonConfiguration>>>::new(256)"
    )]
    pub jettons: HashMap<TonHash, TLBRef<CoffeeStakingJettonConfiguration>>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u32>, DictValAdapterTLB<TLBRef<CoffeeStakingPeriod>>>::new(32)")]
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xf1eef33c, bits_len = 32)]
pub struct CoffeeStakingVaultSetOwnerMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xeec15c2a, bits_len = 32)]
pub struct CoffeeStakingVaultSetJettonsMsg {
    pub query_id: u64,
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB<TLBRef<CoffeeStakingJettonConfiguration>>>::new(256)"
    )]
    pub jettons: HashMap<TonHash, TLBRef<CoffeeStakingJettonConfiguration>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xfab30dc2, bits_len = 32)]
pub struct CoffeeStakingVaultSetPeriodsMsg {
    pub query_id: u64,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u32>, DictValAdapterTLB<TLBRef<CoffeeStakingPeriod>>>::new(32)")]
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xce4e2066, bits_len = 32)]
pub struct CoffeeStakingVaultUpdateContractMsg {
    pub query_id: u64,
    pub new_code: TLBRef<TonCell>,
    pub new_data: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xbe5a7595, bits_len = 32)]
pub struct CoffeeStakingVaultMasterInitMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub distributor: TonAddress,
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB<TLBRef<CoffeeStakingJettonConfiguration>>>::new(256)"
    )]
    pub jettons: HashMap<TonHash, TLBRef<CoffeeStakingJettonConfiguration>>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u32>, DictValAdapterTLB<TLBRef<CoffeeStakingPeriod>>>::new(32)")]
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xf9471134, bits_len = 32)]
pub struct CoffeeStakingVaultDepositToMasterMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    pub jetton_amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub from_user: TonAddress,
    pub period_id: u32,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xbe5a7595, bits_len = 32)]
pub struct CoffeeStakingVaultItemInitMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    pub jetton_data: TLBRef<CoffeeStakingAssetData>,
    pub position_data: TLBRef<CoffeeStakingPositionData>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u32>, DictValAdapterTLB<TLBRef<CoffeeStakingPeriod>>>::new(32)")]
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct CoffeeStakingVaultJettonRefundMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub response_destination: TonAddress,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeStakingVaultIncomingMessage {
    Init(CoffeeStakingVaultInitMsg),
    TransferNotification(CoffeeCrossDexJettonTransferNotificationMsg),
    SetOwner(CoffeeStakingVaultSetOwnerMsg),
    SetJettons(CoffeeStakingVaultSetJettonsMsg),
    SetPeriods(CoffeeStakingVaultSetPeriodsMsg),
    UpdateContract(CoffeeStakingVaultUpdateContractMsg),
    MasterUnlock(CoffeeStakingMasterUnlockToVaultMsg),
    Empty(EmptyBodyMsg),
    Unknown(TonCell),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeStakingVaultOutgoingMessage {
    DepositToMaster(CoffeeStakingVaultDepositToMasterMsg),
    JettonRefund(CoffeeStakingVaultJettonRefundMsg),
    Unknown(TonCell),
}
