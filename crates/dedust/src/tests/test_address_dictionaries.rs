use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmPoolWallets;
use std::collections::HashMap;
use ton::{
    tlb_adapters::{DictKeyAdapterTonHash, DictValAdapterTLB, TLBHashMapE},
    ton_core::{
        cell::TonHash,
        traits::tlb::TLB,
        types::{TonAddress, tlb_core::MsgAddressInt},
    },
};

#[test]
fn test_pool_wallet_dictionary_preserves_internal_zero() -> anyhow::Result<()> {
    let key = TonHash::from_slice_sized(&[17; 32]);
    let wallets = DedustV2CpmmPoolWallets {
        wallets_by_assets: HashMap::from([(key, TonAddress::ZERO)]),
        assets_by_wallets: HashMap::new(),
        resolutions: HashMap::new(),
    };
    let cell = wallets.to_cell()?;
    // Decode independently as required raw addresses: addr_none would fail here.
    let raw =
        TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB<MsgAddressInt>>::new(256).read(&mut cell.parser())?;
    assert_eq!(raw.get(&key), Some(&TonAddress::ZERO.to_msg_address_int()));
    assert_eq!(DedustV2CpmmPoolWallets::from_cell(&cell)?, wallets);
    Ok(())
}
