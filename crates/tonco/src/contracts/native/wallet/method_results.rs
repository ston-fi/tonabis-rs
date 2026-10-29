//! Exact-arity structured getter results; integer units/widths follow the getter, not storage.
use fastnum::I512;
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoNativeWalletGetWalletDataResult {
    pub balance: I512,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub owner: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub minter: Option<TonAddress>,
    pub code: TonCell,
}
