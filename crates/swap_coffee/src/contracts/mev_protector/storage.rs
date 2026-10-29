//! Generated Coffee storage. See tools/reference/swap_coffee/README.md.
use crate::contracts::mev::CoffeeMevReplayMarker;
use fastnum::U256;
use std::collections::HashMap;
use ton::tlb_adapters::{DictKeyAdapterUint, DictValAdapterTLB, TLBHashMapE};
use ton::ton_core::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeMevProtectorStorage {
    #[tlb(adapter = "NullableAddress")]
    pub admin_address: Option<TonAddress>,
    pub config: u32,
    pub processed_until: u64,
    pub public_key: U256,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u64>, DictValAdapterTLB<CoffeeMevReplayMarker>>::new(64)")]
    pub processed_queries: HashMap<u64, CoffeeMevReplayMarker>,
}
