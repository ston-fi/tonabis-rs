//! Structured getter results.
use fastnum::I512;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
/// Structured getter result in declared TVM order.
#[derive(Debug, Clone, PartialEq, ton::ton_core::FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct CoffeeLpWalletGetWalletDataResult {
    pub balance: I512,
    #[from_tvm_stack(adapter = InternalAddress)]
    pub owner_address: TonAddress,
    #[from_tvm_stack(adapter = InternalAddress)]
    pub jetton_master_address: TonAddress,
    pub jetton_wallet_code: TonCell,
}
