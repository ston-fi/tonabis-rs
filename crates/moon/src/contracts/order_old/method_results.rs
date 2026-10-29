//! Structured getter results with exact TVM stack arity.
use ton::ton_core::FromTVMStack;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct MoonOrderOldGetVestingDataResult {
    pub vesting_start_time: u64,
    pub vesting_duration: u64,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub recipient: Option<TonAddress>,
}
