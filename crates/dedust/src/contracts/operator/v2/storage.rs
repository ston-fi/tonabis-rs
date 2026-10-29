//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use ton::ton_core::{
    TLB,
    types::{TonAddress, tlb_core::adapters::InternalAddress},
};
/// Exact storage read by captured Operator V2 code.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustOperatorV2Storage {
    #[tlb(adapter = "InternalAddress")]
    pub factory: TonAddress,
    pub version: u16,
    pub role: u8,
    #[tlb(adapter = "InternalAddress")]
    pub user: TonAddress,
    pub can_execute_after: u32,
}
