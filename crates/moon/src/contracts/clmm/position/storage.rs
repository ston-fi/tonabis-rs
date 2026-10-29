//! Generated Moon wire records. See tools/reference/moon/README.md.
use crate::contracts::clmm::range::storage::MoonClmmBin;
use std::collections::HashMap;
use ton::tlb_adapters::DictKeyAdapterInt;
use ton::tlb_adapters::DictValAdapterTLB;
use ton::tlb_adapters::TLBHashMapE;
use ton::ton_core::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmLpPositionInitData {
    #[tlb(adapter = "InternalAddress")]
    pub range: TonAddress,
    pub nft_index: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmLpPositionStorage {
    #[tlb(adapter = "InternalAddress")]
    pub range: TonAddress,
    pub nft_index: u64,
    #[tlb(adapter = "InternalAddress")]
    pub user: TonAddress,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterInt<64, i64>, DictValAdapterTLB<MoonClmmBin>>::new(64)")]
    pub bins: HashMap<i64, MoonClmmBin>,
    pub locked: bool,
    pub pending: bool,
}
