use crate::contracts::staking::CoffeeStakingPeriod;
use crate::contracts::staking_item::storage::CoffeeStakingAssetData;
use crate::contracts::staking_item::storage::CoffeeStakingPositionData;
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
use ton::tlb_adapters::DictKeyAdapterUint;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::TLBRef;
/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq, ton::ton_core::FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct CoffeeStakingItemGetNftDataResult {
    pub is_initialized: bool,
    pub item_index: I512,
    #[from_tvm_stack(adapter = InternalAddress)]
    pub collection_address: TonAddress,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub owner_address: Option<TonAddress>,
    pub content: TonCell,
}

/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq)]
pub struct CoffeeStakingItemGetStoredDataResult {
    pub initialized: bool,
    pub nft_id: u64,
    pub master: TonAddress,
    pub owner: Option<TonAddress>,
    pub asset: TLBRef<CoffeeStakingAssetData>,
    pub info: TLBRef<CoffeeStakingPositionData>,
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
}
impl FromTVMStack for CoffeeStakingItemGetStoredDataResult {
    fn from_stack(s: &mut TVMStack) -> TonResult<Self> {
        let periods = stack_dictionary::<DictKeyAdapterUint<u32>, TLBRef<CoffeeStakingPeriod>>(s, 32)?;
        let info = TLBRef::new(CoffeeStakingPositionData::from_cell(&TonCell::from_stack(s)?)?);
        let asset = TLBRef::new(CoffeeStakingAssetData::from_cell(&TonCell::from_stack(s)?)?);
        let owner = stack_nullable_address(s)?;
        let master = InternalAddress.read(&mut TonCell::from_stack(s)?.parser())?;
        let nft_id = <u64>::from_stack(s)?;
        let initialized = <bool>::from_stack(s)?;
        s.ensure_empty()?;
        Ok(Self {
            initialized,
            nft_id,
            master,
            owner,
            asset,
            info,
            periods,
        })
    }
}
