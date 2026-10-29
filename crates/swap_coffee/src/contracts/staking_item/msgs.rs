//! Generated Coffee msgs. See tools/reference/swap_coffee/README.md.
use crate::contracts::staking::CoffeeStakingPeriod;
use crate::contracts::staking_item::storage::CoffeeStakingAssetData;
use crate::contracts::staking_item::storage::CoffeeStakingPositionData;
use std::collections::HashMap;
use ton::tlb_adapters::{DictKeyAdapterUint, DictValAdapterTLB, TLBHashMapE};
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::{TLBCoins, TLBRef};
use tonabis_core::msgs::EmptyBodyMsg;
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xbe5a7595, bits_len = 32)]
pub struct CoffeeStakingItemInitMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    pub jetton_data: TLBRef<CoffeeStakingAssetData>,
    pub position_data: TLBRef<CoffeeStakingPositionData>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u32>, DictValAdapterTLB<TLBRef<CoffeeStakingPeriod>>>::new(32)")]
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcb03bfaf, bits_len = 32)]
pub struct CoffeeStakingItemUnlockMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xeb666ce0, bits_len = 32)]
pub struct CoffeeStakingItemExtendMsg {
    pub query_id: u64,
    pub period_id: u32,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcb03bfaf, bits_len = 32)]
pub struct CoffeeStakingItemUnlockNotificationMsg {
    pub query_id: u64,
    pub nft_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    pub amount: TLBCoins,
    pub asset_data: TLBRef<CoffeeStakingAssetData>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xeb666ce0, bits_len = 32)]
pub struct CoffeeStakingItemExtendNotificationMsg {
    pub query_id: u64,
    pub nft_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    pub amount: TLBCoins,
    pub end_timestamp: u64,
    pub period_id: u32,
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
// Keep opcode prefixes visible: Box<T> erases them in locked TON dispatch.
#[allow(clippy::large_enum_variant)]
pub enum CoffeeStakingItemIncomingMessage {
    Init(CoffeeStakingItemInitMsg),
    Unlock(CoffeeStakingItemUnlockMsg),
    Extend(CoffeeStakingItemExtendMsg),
    Empty(EmptyBodyMsg),
    Unknown(TonCell),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeStakingItemOutgoingMessage {
    UnlockNotification(CoffeeStakingItemUnlockNotificationMsg),
    ExtendNotification(CoffeeStakingItemExtendNotificationMsg),
    Unknown(TonCell),
}
