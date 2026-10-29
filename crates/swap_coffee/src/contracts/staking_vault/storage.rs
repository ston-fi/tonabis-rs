//! Generated Coffee storage. See tools/reference/swap_coffee/README.md.
use crate::contracts::staking::CoffeeStakingJettonConfiguration;
use crate::contracts::staking::CoffeeStakingPeriod;
use std::collections::HashMap;
use ton::tlb_adapters::{DictKeyAdapterTonHash, DictKeyAdapterUint, DictValAdapterTLB, TLBHashMapE};
use ton::ton_core::TLB;
use ton::ton_core::cell::{TonCell, TonHash};
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeStakingVaultStorage {
    pub initialized: bool,
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB<TLBRef<CoffeeStakingJettonConfiguration>>>::new(256)"
    )]
    pub jettons: HashMap<TonHash, TLBRef<CoffeeStakingJettonConfiguration>>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u32>, DictValAdapterTLB<TLBRef<CoffeeStakingPeriod>>>::new(32)")]
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
    pub master_code: TLBRef<TonCell>,
    pub item_code: TLBRef<TonCell>,
}
