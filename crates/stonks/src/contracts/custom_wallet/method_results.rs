//! custom_wallet method_results records.
use fastnum::I512;
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
/// Complete buy/sell tax stack in per-mille units.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksCustomWalletGetTaxResult {
    pub buy_tax_permille: I512,
    pub sell_tax_permille: I512,
}

/// Complete four-field wallet getter stack; cell and address conversion is native TON.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksCustomWalletGetWalletDataResult {
    pub jetton_balance: I512,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub owner_address: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub jetton_master_address: Option<TonAddress>,
    pub jetton_wallet_code: TonCell,
}
