//! Exact-arity structured getter results; integer units/widths follow the getter, not storage.
use fastnum::I512;
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoAccountV2GetAccountDataResult {
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub user: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub pool: Option<TonAddress>,
    pub amount0: I512,
    pub amount1: I512,
    pub position_count: I512,
    pub order: Option<TonCell>,
    pub positions: TonCell,
}
