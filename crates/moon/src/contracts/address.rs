//! Address derivation from caller-supplied, revision-correct code and typed init data.
use crate::contracts::mempad::wallet::storage::MoonMempadWalletStorage;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::{
    block_tlb::StateInit,
    ton_core::{cell::TonCell, errors::TonCoreResult, traits::tlb::TLB, types::TonAddress},
};
/// Ordinary child StateInit uses workchain zero; the caller orders assets as the factory does.
/// Launch wallets require `mempad_wallet`, which has a different hash construction.
pub fn state_init<T: TLB>(code: TonCell, data: &T) -> TonCoreResult<StateInit> {
    Ok(StateInit::new(code, data.to_cell()?))
}
pub fn derive<T: TLB>(code: TonCell, data: &T) -> TonCoreResult<TonAddress> {
    state_init(code, data)?.derive_address(0)
}
/// Launch jetton wallets use split depth eight and splice the owner's first byte.
/// This differs from an ordinary state-init hash. The curve's stored wallet code
/// must be supplied, rather than a possibly upgraded factory template.
pub fn mempad_wallet(code: TonCell, owner: &TonAddress, curve: &TonAddress) -> TonCoreResult<TonAddress> {
    let data = MoonMempadWalletStorage {
        jetton_balance: TLBCoins::ZERO,
        owner_address: *owner,
        minter_address: *curve,
    };
    let mut init = StateInit::new(code, data.to_cell()?);
    init.split_depth = Some(8);
    let mut hash = init.cell_hash()?;
    hash.as_slice_sized_mut()[0] = owner.hash.as_slice_sized()[0];
    Ok(TonAddress::new(0, hash))
}

/// Standard internal address held in a referenced field or dictionary value.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub struct MoonReferencedAddress {
    #[tlb(adapter = "InternalAddress")]
    pub value: TonAddress,
}
