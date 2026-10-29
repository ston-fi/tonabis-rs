//! Structured getter results.
use crate::contracts::staking::CoffeeStakingJettonConfiguration;
use crate::contracts::staking::CoffeeStakingPeriod;
use crate::contracts::wire::{stack_dictionary, stack_nullable_address};
use std::collections::HashMap;
use ton::block_tlb::{FromTVMStack, TVMStack};
use ton::errors::TonResult;
use ton::tlb_adapters::{DictKeyAdapterTonHash, DictKeyAdapterUint};
use ton::ton_core::cell::{TonCell, TonHash};
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq)]
pub struct CoffeeStakingVaultGetStoredDataResult {
    pub initialized: bool,
    pub owner: Option<TonAddress>,
    pub jettons: HashMap<TonHash, TLBRef<CoffeeStakingJettonConfiguration>>,
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
    pub master_code: TonCell,
    pub item_code: TonCell,
}
impl FromTVMStack for CoffeeStakingVaultGetStoredDataResult {
    fn from_stack(s: &mut TVMStack) -> TonResult<Self> {
        let item_code = TonCell::from_stack(s)?;
        let master_code = TonCell::from_stack(s)?;
        let periods = stack_dictionary::<DictKeyAdapterUint<u32>, TLBRef<CoffeeStakingPeriod>>(s, 32)?;
        let jettons = stack_dictionary::<DictKeyAdapterTonHash, TLBRef<CoffeeStakingJettonConfiguration>>(s, 256)?;
        let owner = stack_nullable_address(s)?;
        let initialized = <bool>::from_stack(s)?;
        s.ensure_empty()?;
        Ok(Self {
            initialized,
            owner,
            jettons,
            periods,
            master_code,
            item_code,
        })
    }
}
