//! dedust_factory method_results records.
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
/// Complete factory getter stack in source field order.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksDedustFactoryGetFactoryDataResult {
    pub virtual_minter_code: TonCell,
    pub jetton_wallet_code: TonCell,
    pub dex_config: TonCell,
    pub disabled: bool,
}
