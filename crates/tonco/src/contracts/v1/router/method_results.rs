//! Exact-arity structured getter results; integer units/widths follow the getter, not storage.
use fastnum::I512;
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoRouterV1GetRouterStateResult {
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub router_admin_address: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub pool_admin_address: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub pool_factory_address: Option<TonAddress>,
    pub active_flags: I512,
    pub pool_seqno: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoRouterV1GetChildContractsResult {
    pub pool_code: TonCell,
    pub account_code: TonCell,
    pub position_nft_code: TonCell,
}
