//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use fastnum::U256;
use std::collections::HashMap;
use ton::{
    tlb_adapters::{DictKeyAdapterTonHash, DictValAdapterNum, TLBHashMapE},
    ton_core::{TLB, cell::TonHash},
};
/// Persistent replay protection and signer-revocation state.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustX1000WalletV1Storage {
    /// Base sequence number for the active 256-message replay window.
    pub seqno_offset: u32,
    pub seqno_bitmap: TonHash,
    pub public_key: U256,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterNum<u64, 64>>::new(256)")]
    pub revoked: HashMap<TonHash, u64>,
}
