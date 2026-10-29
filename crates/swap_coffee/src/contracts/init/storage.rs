//! Generated Coffee storage. See tools/reference/swap_coffee/README.md.
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeInitStorage {
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    pub child_identity_tail: TonCell,
}
