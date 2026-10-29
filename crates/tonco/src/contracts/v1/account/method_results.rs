//! Exact-arity structured getter results; integer units/widths follow the getter, not storage.
use fastnum::I512;
use ton::ton_core::FromTVMStack;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoAccountV1GetAccountDataResult {
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub user_address: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub pool_address: Option<TonAddress>,
    pub amount0: I512,
    pub amount1: I512,
    pub enough0: I512,
    pub enough1: I512,
}
