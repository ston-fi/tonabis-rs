//! dedust_wallet method_results records.
use fastnum::U256;
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
/// Complete standard getter, with executable code supplied by MYCODE.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksDedustWalletGetWalletDataResult {
    /// Spendable token balance.
    pub balance: U256,
    /// Owner.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub owner: Option<TonAddress>,
    /// Minter.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub master: Option<TonAddress>,
    /// Executable wallet code.
    pub code: TonCell,
}

/// Complete seven-field wallet getter at numeric selector84724.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksDedustWalletGetExtendedDataResult {
    /// Spendable balance.
    pub balance: U256,
    /// Source-defined constant-zero numeric field.
    pub reserved_amount: U256,
    /// First emitted copy of the context authority.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub context_address0: Option<TonAddress>,
    /// Second emitted copy of the context authority.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub context_address1: Option<TonAddress>,
    /// Normalized context flag.
    pub flag: bool,
    /// Owner.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub owner: Option<TonAddress>,
    /// Master.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub master: Option<TonAddress>,
}
