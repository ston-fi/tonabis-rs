use crate::contracts::staking::CoffeeStakingJettonConfiguration;
use crate::contracts::staking::CoffeeStakingPeriod;
use crate::contracts::staking::CoffeeStakingRewardSchedule;
use crate::contracts::staking_master::storage::CoffeeStakingMasterState;
use std::collections::HashMap;
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
// Structured getter results.
use crate::contracts::wire::{stack_dictionary, stack_nullable_address};
use fastnum::I512;
use ton::block_tlb::{FromTVMStack, TVMStack};
use ton::errors::TonResult;
use ton::tlb_adapters::{DictKeyAdapterTonHash, DictKeyAdapterUint};
use ton::ton_core::cell::{TonCell, TonHash};
use ton::ton_core::types::tlb_core::TLBRef;
/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq)]
pub struct CoffeeStakingMasterGetStoredDataResult {
    pub initialized: bool,
    pub owner: Option<TonAddress>,
    pub distributor: Option<TonAddress>,
    pub vault: TonAddress,
    pub state: TLBRef<CoffeeStakingMasterState>,
    pub rewards: HashMap<TonHash, TLBRef<CoffeeStakingRewardSchedule>>,
    pub jettons: HashMap<TonHash, TLBRef<CoffeeStakingJettonConfiguration>>,
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
}
impl FromTVMStack for CoffeeStakingMasterGetStoredDataResult {
    fn from_stack(s: &mut TVMStack) -> TonResult<Self> {
        let periods = stack_dictionary::<DictKeyAdapterUint<u32>, TLBRef<CoffeeStakingPeriod>>(s, 32)?;
        let jettons = stack_dictionary::<DictKeyAdapterTonHash, TLBRef<CoffeeStakingJettonConfiguration>>(s, 256)?;
        let rewards = stack_dictionary::<DictKeyAdapterTonHash, TLBRef<CoffeeStakingRewardSchedule>>(s, 256)?;
        let state = TLBRef::new(CoffeeStakingMasterState::from_cell(&TonCell::from_stack(s)?)?);
        let vault = InternalAddress.read(&mut TonCell::from_stack(s)?.parser())?;
        let distributor = stack_nullable_address(s)?;
        let owner = stack_nullable_address(s)?;
        let initialized = <bool>::from_stack(s)?;
        s.ensure_empty()?;
        Ok(Self {
            initialized,
            owner,
            distributor,
            vault,
            state,
            rewards,
            jettons,
            periods,
        })
    }
}

/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq, ton::ton_core::FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct CoffeeStakingMasterGetCollectionDataResult {
    pub next_item_index: I512,
    pub collection_metadata: TonCell,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub owner_address: Option<TonAddress>,
}
