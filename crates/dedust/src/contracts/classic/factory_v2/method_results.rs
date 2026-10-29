//! Getter result records and complete stack decoding.
use ton::ton_core::FromTVMStack;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

/// Nullable ownership addresses encoded as Cell/Slice; TVM NULL is rejected.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustFactoryV2GetOwnershipResult {
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub owner_address: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub pending_owner_address: Option<TonAddress>,
    /// Unix timestamp after which the pending owner can accept.
    pub can_accept_after: u32,
}
