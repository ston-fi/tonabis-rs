//! Current template selectors are independent capture observations, distinct from catalog names.
use super::test_getter_provider::recording_client;
use crate::contracts::{catalog_range, clmm, cpmm, mempad, order_new, order_old};
use ton::errors::TonError;
use ton::ton_core::errors::TonCoreError;
use ton::{
    block_tlb::{TVMNull, TVMStack, TVMStackValue},
    contracts::TonContract,
    ton_core::{
        cell::TonCell,
        traits::tlb::TLB,
        types::{TonAddress, tlb_core::MsgAddress},
    },
};

fn address_reply() -> anyhow::Result<TVMStack> {
    let mut stack = TVMStack::EMPTY;
    stack.push_cell_slice(TonAddress::ZERO.to_msg_address_int().to_cell()?);
    Ok(stack)
}
fn cell_reply() -> TVMStack {
    let mut stack = TVMStack::EMPTY;
    stack.push_cell(TonCell::empty().clone());
    stack
}
fn null_reply() -> TVMStack {
    let mut stack = TVMStack::EMPTY;
    stack.push(TVMStackValue::Null(TVMNull));
    stack
}

#[test]
fn test_current_cpmm_position_getters() -> anyhow::Result<()> {
    let mut reply = TVMStack::EMPTY;
    reply.push_tiny_int(-1);
    let client = recording_client(107920, TVMStack::EMPTY, reply)?;
    let position = cpmm::position::MoonCpmmPositionContract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(position.get_locked())?);
    let client = recording_client(109064, TVMStack::EMPTY, cell_reply())?;
    let position = cpmm::position::MoonCpmmPositionContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(tokio_test::block_on(position.get_position())?, *TonCell::empty());
    Ok(())
}

#[test]
fn test_clmm_factory_native_null_and_internal_zero_arguments() -> anyhow::Result<()> {
    let mut arguments = null_reply();
    arguments.push_cell_slice(TonAddress::ZERO.to_msg_address_int().to_cell()?);
    arguments.push_int(100.into());
    let client = recording_client(125122, arguments, address_reply()?)?;
    let factory = clmm::factory::MoonClmmFactoryContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(
        tokio_test::block_on(factory.get_pool_address(None, Some(TonAddress::ZERO), 100))?,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    for workchain in [-129, 128] {
        let invalid = TonAddress::new(workchain, TonAddress::ZERO.hash);
        assert!(matches!(
            tokio_test::block_on(factory.get_pool_address(Some(invalid), None, 100)),
            Err(TonError::TLCoreError(TonCoreError::DataError { .. }))
        ));
    }
    Ok(())
}

#[test]
fn test_clmm_current_selectors_and_signed_range_argument() -> anyhow::Result<()> {
    let client = recording_client(87704, TVMStack::EMPTY, cell_reply())?;
    let pool = clmm::pool::MoonClmmPoolContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(tokio_test::block_on(pool.get_pool())?, *TonCell::empty());
    let mut arguments = TVMStack::EMPTY;
    arguments.push_int((-7).into());
    let client = recording_client(117800, arguments, address_reply()?)?;
    let pool = clmm::pool::MoonClmmPoolContract::new(&client, &TonAddress::ZERO, None);
    tokio_test::block_on(pool.get_range_by_num(-7))?;
    let client = recording_client(122414, TVMStack::EMPTY, null_reply())?;
    let pool = clmm::pool::MoonClmmPoolContract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(pool.get_ranges())?.ranges.is_empty());
    let client = recording_client(103203, TVMStack::EMPTY, cell_reply())?;
    let range = clmm::range::MoonClmmRangeContract::new(&client, &TonAddress::ZERO, None);
    tokio_test::block_on(range.get_range())?;
    let client = recording_client(112561, address_reply()?, address_reply()?)?;
    let range = clmm::range::MoonClmmRangeContract::new(&client, &TonAddress::ZERO, None);
    tokio_test::block_on(range.get_account_address(&TonAddress::ZERO))?;
    let client = recording_client(109064, TVMStack::EMPTY, cell_reply())?;
    let position = clmm::position::MoonClmmPositionContract::new(&client, &TonAddress::ZERO, None);
    tokio_test::block_on(position.get_position())?;
    Ok(())
}

#[test]
fn test_mempad_getters_preserve_null_admin_and_stack_order() -> anyhow::Result<()> {
    let client = recording_client(103289, address_reply()?, address_reply()?)?;
    let curve = mempad::curve::MoonMempadCurveContract::new(&client, &TonAddress::ZERO, None);
    tokio_test::block_on(curve.get_wallet_address(&TonAddress::ZERO))?;
    let mut reply = TVMStack::EMPTY;
    reply.push_int(123.into());
    reply.push_tiny_int(-1);
    reply.push(TVMStackValue::Null(TVMNull));
    reply.push_cell(TonCell::empty().clone());
    reply.push_cell(TonCell::empty().clone());
    let client = recording_client(106029, TVMStack::EMPTY, reply)?;
    let curve = mempad::curve::MoonMempadCurveContract::new(&client, &TonAddress::ZERO, None);
    let data = tokio_test::block_on(curve.get_jetton_data())?;
    assert_eq!(data.total_supply, 123.into());
    assert!(data.mintable);
    assert_eq!(data.admin_address, None);
    let client = recording_client(78683, TVMStack::EMPTY, null_reply())?;
    let curve = mempad::curve::MoonMempadCurveContract::new(&client, &TonAddress::ZERO, None);
    tokio_test::block_on(curve.get_null())?;
    let mut reply = TVMStack::EMPTY;
    reply.push_int(456.into());
    reply.push_cell_slice(TonAddress::ZERO.to_msg_address_int().to_cell()?);
    reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
    reply.push_cell(TonCell::empty().clone());
    let client = recording_client(97026, TVMStack::EMPTY, reply)?;
    let wallet = mempad::wallet::MoonMempadWalletContract::new(&client, &TonAddress::ZERO, None);
    let data = tokio_test::block_on(wallet.get_wallet_data())?;
    assert_eq!(data.balance, 456.into());
    assert_eq!(
        data.owner,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(data.minter, None);
    Ok(())
}

#[test]
fn test_observed_order_fee_selectors() -> anyhow::Result<()> {
    let mut reply = TVMStack::EMPTY;
    reply.push_int(777.into());
    let client = recording_client(80604, TVMStack::EMPTY, reply.clone())?;
    let old = order_old::MoonOrderOldContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(tokio_test::block_on(old.get_collected_fees())?, 777.into());
    let client = recording_client(80604, TVMStack::EMPTY, reply)?;
    let new = order_new::MoonOrderNewContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(tokio_test::block_on(new.get_collected_fees())?, 777.into());
    Ok(())
}

#[test]
fn test_getter_native_cells_and_nullable_values() -> anyhow::Result<()> {
    for kind in 0..3 {
        let mut reply = TVMStack::EMPTY;
        match kind {
            0 => reply.push(TVMStackValue::Null(TVMNull)),
            1 => reply.push_cell(TonCell::empty().clone()),
            _ => reply.push_cell_slice(TonCell::empty().clone()),
        };
        let client = recording_client(91258, TVMStack::EMPTY, reply.clone())?;
        let range = catalog_range::MoonCatalogClmmRangeContract::new(&client, &TonAddress::ZERO, None);
        let bins = tokio_test::block_on(range.get_bins())?;
        assert_eq!(bins.is_some(), kind != 0);
        let client = recording_client(109064, TVMStack::EMPTY, reply)?;
        let position = cpmm::position::MoonCpmmPositionContract::new(&client, &TonAddress::ZERO, None);
        let result = tokio_test::block_on(position.get_position());
        assert_eq!(result.is_ok(), kind != 0);
    }
    Ok(())
}
