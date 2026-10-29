//! Getter result records and protocol-specific stack decoding.
use ton::ton_core::{FromTVMStack, types::TonAddress};

/// Exact numeric selector79987 returns factory, role, user and execution activation time; no synthetic-name hashing.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustOperatorV2GetOperatorDataResult {
    pub factory: TonAddress,
    pub role: u8,
    pub user: TonAddress,
    pub can_execute_after: u32,
}
