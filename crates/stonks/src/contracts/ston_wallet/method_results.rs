//! ston_wallet method_results records.
use fastnum::U256;
use ton::ton_core::FromTVMStack;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
/// Complete seven-field STON wallet getter at numeric selector74038.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksStonWalletGetExtendedDataResult {
    /// Spendable balance.
    pub balance: U256,
    /// Source-defined router constant.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub router: Option<TonAddress>,
    /// First context address.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub context_address0: Option<TonAddress>,
    /// Second context address.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub context_address1: Option<TonAddress>,
    /// Normalized source flag.
    pub flag: bool,
    /// Owner.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub owner: Option<TonAddress>,
    /// Master.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub master: Option<TonAddress>,
}
