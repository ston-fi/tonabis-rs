//! Exact-arity structured getter results; integer units/widths follow the getter, not storage.
use fastnum::I512;
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoNativeMinterGetJettonDataResult {
    pub total_supply: I512,
    pub mintable: bool,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub admin: Option<TonAddress>,
    pub content: TonCell,
    pub wallet_code: TonCell,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoNativeMinterGetVersionResult {
    pub major: I512,
    pub minor: I512,
    pub label: TonCell,
}
