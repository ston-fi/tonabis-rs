//! Generated Coffee msgs. See tools/reference/swap_coffee/README.md.
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::TLBRef;
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee22, bits_len = 32)]
pub struct CoffeeInitDeployMsg {
    pub query_id: u64,
    pub code: TLBRef<TonCell>,
    pub data: TLBRef<TonCell>,
    pub action: Option<TLBRef<TonCell>>,
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeInitIncomingMessage {
    Deploy(CoffeeInitDeployMsg),
    Unknown(TonCell),
}
