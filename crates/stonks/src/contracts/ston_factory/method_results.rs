//! ston_factory method_results records.
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
/// Complete factory getter stack in source field order.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksStonFactoryGetFactoryDataResult {
    pub virtual_minter_code: TonCell,
    pub jetton_wallet_code: TonCell,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub router_address: Option<TonAddress>,
    pub disabled: bool,
}
