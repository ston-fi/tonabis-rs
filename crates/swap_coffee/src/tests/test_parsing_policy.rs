use super::getter_provider::{client, failure_client};
use crate::contracts::cross_dex::CoffeeCrossDexContract;
use std::sync::atomic::Ordering;
use ton::block_tlb::{TVMNull, TVMStack};
use ton::contracts::TonContract;
use ton::errors::TonError;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreError;
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::{MsgAddress, MsgAddressInt, MsgAddressIntVar};

#[test]
fn test_nullable_getter_preserves_none_zero_and_native_stack_forms() -> anyhow::Result<()> {
    let wider: MsgAddressInt = MsgAddressIntVar {
        anycast: None,
        addr_bits_len: 8,
        workchain: 256,
        address: vec![42],
    }
    .into();
    let zero = MsgAddressInt::from_cell(&TonAddress::ZERO.to_msg_address_int().to_cell()?)?;
    for address in [None, Some(zero)] {
        let cell = address.as_ref().map_or_else(|| MsgAddress::NONE.to_cell(), TLB::to_cell)?;
        for slice in [false, true] {
            let mut output = TVMStack::default();
            output.push_tiny_int(19); // permitted older scalar stack value
            if slice {
                output.push_cell_slice(cell.clone());
            } else {
                output.push_cell(cell.clone());
            }
            let (client, calls) = client(113142, TVMStack::default(), output)?;
            let wrapper = CoffeeCrossDexContract::new(&client, &TonAddress::ZERO, None);
            assert_eq!(
                tokio_test::block_on(wrapper.get_admin())?,
                address.clone().map(TonAddress::from_msg_address).transpose()?
            );
            assert_eq!(calls.load(Ordering::SeqCst), 1);
        }
    }
    let mut output = TVMStack::default();
    output.push_cell(wider.to_cell()?);
    let (client, _) = super::getter_provider::client(113142, TVMStack::default(), output)?;
    assert!(tokio_test::block_on(CoffeeCrossDexContract::new(&client, &TonAddress::ZERO, None).get_admin()).is_err());
    let mut output = TVMStack::default();
    output.push(TVMNull.into());
    let (client, _) = super::getter_provider::client(113142, TVMStack::default(), output)?;
    assert!(tokio_test::block_on(CoffeeCrossDexContract::new(&client, &TonAddress::ZERO, None).get_admin())?.is_none());
    Ok(())
}

#[test]
fn test_provider_failure_is_typed_and_not_retried() -> anyhow::Result<()> {
    let (client, calls) = failure_client()?;
    let error = tokio_test::block_on(CoffeeCrossDexContract::new(&client, &TonAddress::ZERO, None).get_admin())
        .err()
        .ok_or_else(|| anyhow::anyhow!("provider failure unexpectedly succeeded"))?;
    assert!(
        matches!(error, TonError::TLCoreError(TonCoreError::DataError { producer, msg }) if producer == "coffee-test" && msg == "provider failed")
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}

#[test]
fn test_native_cell_scalar_getter_accepts_cell_slice_and_older_stack_values() -> anyhow::Result<()> {
    use crate::contracts::vault_native::CoffeeVaultNativeContract;
    for slice in [false, true] {
        let mut output = TVMStack::default();
        output.push_tiny_int(7);
        if slice {
            output.push_cell_slice(TonCell::empty().clone());
        } else {
            output.push_cell(TonCell::empty().clone());
        }
        let (client, calls) = client(89352, TVMStack::default(), output)?;
        let wrapper = CoffeeVaultNativeContract::new(&client, &TonAddress::ZERO, None);
        assert_eq!(tokio_test::block_on(wrapper.get_asset())?, *TonCell::empty());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    Ok(())
}
