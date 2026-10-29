//! Exact-arity structured getter results; integer units/widths follow the getter, not storage.
use fastnum::I512;
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPositionNftV1GetPositionInfoResult {
    pub liquidity: I512,
    pub tick_lower: I512,
    pub tick_upper: I512,
    pub fee_growth_inside0_last_x128: I512,
    pub fee_growth_inside1_last_x128: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPositionNftV1GetNftDataResult {
    pub is_initialized: bool,
    pub item_index: I512,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub collection_address: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub owner_address: Option<TonAddress>,
    pub content: Option<TonCell>,
}
