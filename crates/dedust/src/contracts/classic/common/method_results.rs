//! Getter result records and protocol-specific stack decoding.
use ton::ton_core::{FromTVMStack, TLB, cell::TonCell, types::tlb_core::TLBRef};

/// Actual program code; parsing does not authenticate its identity or authority.
#[derive(Debug, Clone, PartialEq, TLB, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustClassicVersionedCode {
    pub version: u16,
    pub code: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustClassicNullableVersionedCode {
    pub version: u16,
    pub code: Option<TonCell>,
}
