//! Exact-arity structured getter results; integer units/widths follow the getter, not storage.
use fastnum::I512;
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoFactoryV2GetPoolFactoryDataResult {
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub admin: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub router: Option<TonAddress>,
    pub ton_price: I512,
    pub collection: TonCell,
    pub item: TonCell,
}
