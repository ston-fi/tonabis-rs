//! Exact-arity structured getter results; integer units/widths follow the getter, not storage.
use fastnum::I512;
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPositionNftV2GetPositionInfoResult {
    pub liquidity: I512,
    pub lower: I512,
    pub upper: I512,
    pub growth0: I512,
    pub growth1: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPositionNftV2GetNftDataResult {
    pub initialized: bool,
    pub index: I512,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub pool: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub owner: Option<TonAddress>,
    pub content: Option<TonCell>,
}
