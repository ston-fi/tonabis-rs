//! Numeric selectors come from the pinned catalog, not the wrapper macros.
use super::test_getter_provider::recording_client;
use crate::contracts::{booster, catalog_range, legacy_pool, order_factory, order_new, order_old};
use ton::{
    block_tlb::{TVMNull, TVMStack, TVMStackValue},
    contracts::TonContract,
    ton_core::{
        cell::TonCell,
        traits::tlb::TLB,
        types::{TonAddress, tlb_core::MsgAddress},
    },
};
#[test]
fn test_legacy_pool_getters_preserve_selectors_and_arguments() -> anyhow::Result<()> {
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        reply.push_tiny_int(4);
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        reply.push_tiny_int(6);
        let client = recording_client(65971, arguments, reply)?;
        let contract = legacy_pool::MoonLegacyPoolContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_reserves())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        let client = recording_client(118188, arguments, reply)?;
        let contract = legacy_pool::MoonLegacyPoolContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_assets())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        let client = recording_client(103071, arguments, reply)?;
        let contract = legacy_pool::MoonLegacyPoolContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_wallets())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        reply.push_tiny_int(4);
        reply.push_tiny_int(5);
        let client = recording_client(67344, arguments, reply)?;
        let contract = legacy_pool::MoonLegacyPoolContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_fees())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        reply.push_tiny_int(4);
        let client = recording_client(80604, arguments, reply)?;
        let contract = legacy_pool::MoonLegacyPoolContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_collected_fees())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        let client = recording_client(73585, arguments, reply)?;
        let contract = legacy_pool::MoonLegacyPoolContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_lp_total())?;
    }
    {
        let mut arguments = TVMStack::EMPTY;
        arguments.push_cell_slice(TonAddress::ZERO.to_msg_address_int().to_cell()?);
        let mut reply = TVMStack::EMPTY;
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        let client = recording_client(78548, arguments, reply)?;
        let contract = legacy_pool::MoonLegacyPoolContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_lp_account(&TonAddress::ZERO))?;
    }
    {
        let mut arguments = TVMStack::EMPTY;
        arguments.push_cell_slice(TonAddress::ZERO.to_msg_address_int().to_cell()?);
        let mut reply = TVMStack::EMPTY;
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        let client = recording_client(103289, arguments, reply)?;
        let contract = legacy_pool::MoonLegacyPoolContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_wallet_address(&TonAddress::ZERO))?;
    }
    {
        let mut arguments = TVMStack::EMPTY;
        arguments.push_int(7.into());
        let mut reply = TVMStack::EMPTY;
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        reply.push_tiny_int(4);
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        reply.push_tiny_int(6);
        let client = recording_client(81335, arguments, reply)?;
        let contract = legacy_pool::MoonLegacyPoolContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_estimated_lp_burn(7.into()))?;
    }
    {
        let mut arguments = TVMStack::EMPTY;
        arguments.push_cell(TonCell::empty().clone());
        arguments.push_cell(TonCell::empty().clone());
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        let client = recording_client(93853, arguments, reply)?;
        let contract = legacy_pool::MoonLegacyPoolContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_estimated_lp(TonCell::empty().clone(), TonCell::empty().clone()))?;
    }
    {
        let mut arguments = TVMStack::EMPTY;
        arguments.push_cell(TonCell::empty().clone());
        let mut reply = TVMStack::EMPTY;
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        reply.push_tiny_int(4);
        let client = recording_client(130423, arguments, reply)?;
        let contract = legacy_pool::MoonLegacyPoolContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_estimated_swap_out(TonCell::empty().clone()))?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        reply.push_tiny_int(4);
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        reply.push_cell(TonCell::empty().clone());
        reply.push_cell(TonCell::empty().clone());
        let client = recording_client(106029, arguments, reply)?;
        let contract = legacy_pool::MoonLegacyPoolContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_jetton_data())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_cell(TonCell::empty().clone());
        let client = recording_client(127226, arguments, reply)?;
        let contract = legacy_pool::MoonLegacyPoolContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_pool())?;
    }
    Ok(())
}
#[test]
fn test_catalog_range_getters_preserve_selectors_and_arguments() -> anyhow::Result<()> {
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_cell(TonCell::empty().clone());
        let client = recording_client(74767, arguments, reply)?;
        let contract = catalog_range::MoonCatalogClmmRangeContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_range())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push(TVMStackValue::Null(TVMNull));
        let client = recording_client(91258, arguments, reply)?;
        let contract = catalog_range::MoonCatalogClmmRangeContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_bins())?;
    }
    {
        let mut arguments = TVMStack::EMPTY;
        arguments.push_cell_slice(TonAddress::ZERO.to_msg_address_int().to_cell()?);
        let mut reply = TVMStack::EMPTY;
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        let client = recording_client(78548, arguments, reply)?;
        let contract = catalog_range::MoonCatalogClmmRangeContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_lp_account(&TonAddress::ZERO))?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        let client = recording_client(86620, arguments, reply)?;
        let contract = catalog_range::MoonCatalogClmmRangeContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_current_bin())?;
    }
    {
        let mut arguments = TVMStack::EMPTY;
        arguments.push_int(7.into());
        let mut reply = TVMStack::EMPTY;
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        let client = recording_client(92067, arguments, reply)?;
        let contract = catalog_range::MoonCatalogClmmRangeContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_nft_address_by_index(7.into()))?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        let client = recording_client(96761, arguments, reply)?;
        let contract = catalog_range::MoonCatalogClmmRangeContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_current_sqrt_price())?;
    }
    Ok(())
}
#[test]
fn test_order_factory_getters_preserve_selectors_and_arguments() -> anyhow::Result<()> {
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        let client = recording_client(117305, arguments, reply)?;
        let contract = order_factory::MoonOrderFactoryContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_id())?;
    }
    {
        let mut arguments = TVMStack::EMPTY;
        arguments.push_cell_slice(TonAddress::ZERO.to_msg_address_int().to_cell()?);
        let mut reply = TVMStack::EMPTY;
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        let client = recording_client(114750, arguments, reply)?;
        let contract = order_factory::MoonOrderFactoryContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_pipe(Some(TonAddress::ZERO)))?;
    }
    Ok(())
}
#[test]
fn test_order_old_getters_preserve_selectors_and_arguments() -> anyhow::Result<()> {
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        let client = recording_client(100881, arguments, reply)?;
        let contract = order_old::MoonOrderOldContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_status())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        let client = recording_client(75573, arguments, reply)?;
        let contract = order_old::MoonOrderOldContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_order_amount())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_cell(TonCell::empty().clone());
        let client = recording_client(82465, arguments, reply)?;
        let contract = order_old::MoonOrderOldContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_order())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_cell(TonCell::empty().clone());
        let client = recording_client(103071, arguments, reply)?;
        let contract = order_old::MoonOrderOldContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_wallets())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        reply.push_tiny_int(4);
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        let client = recording_client(107618, arguments, reply)?;
        let contract = order_old::MoonOrderOldContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_vesting_data())?;
    }
    {
        let mut arguments = TVMStack::EMPTY;
        arguments.push_int(7.into());
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        let client = recording_client(112212, arguments, reply)?;
        let contract = order_old::MoonOrderOldContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_fill_out(7.into()))?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_cell(TonCell::empty().clone());
        let client = recording_client(118188, arguments, reply)?;
        let contract = order_old::MoonOrderOldContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_assets())?;
    }
    Ok(())
}
#[test]
fn test_order_new_getters_preserve_selectors_and_arguments() -> anyhow::Result<()> {
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        let client = recording_client(100881, arguments, reply)?;
        let contract = order_new::MoonOrderNewContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_status())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        let client = recording_client(75573, arguments, reply)?;
        let contract = order_new::MoonOrderNewContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_order_amount())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_cell(TonCell::empty().clone());
        let client = recording_client(82465, arguments, reply)?;
        let contract = order_new::MoonOrderNewContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_order())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_cell(TonCell::empty().clone());
        let client = recording_client(103071, arguments, reply)?;
        let contract = order_new::MoonOrderNewContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_wallets())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        reply.push_tiny_int(4);
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        let client = recording_client(107618, arguments, reply)?;
        let contract = order_new::MoonOrderNewContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_vesting_data())?;
    }
    {
        let mut arguments = TVMStack::EMPTY;
        arguments.push_int(7.into());
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        let client = recording_client(112212, arguments, reply)?;
        let contract = order_new::MoonOrderNewContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_fill_out(7.into()))?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_cell(TonCell::empty().clone());
        let client = recording_client(118188, arguments, reply)?;
        let contract = order_new::MoonOrderNewContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_assets())?;
    }
    Ok(())
}
#[test]
fn test_booster_getters_preserve_selectors_and_arguments() -> anyhow::Result<()> {
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_cell(TonCell::empty().clone());
        let client = recording_client(76107, arguments, reply)?;
        let contract = booster::MoonBoosterContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_booster())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_tiny_int(3);
        let client = recording_client(100881, arguments, reply)?;
        let contract = booster::MoonBoosterContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_status())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_cell(TonCell::empty().clone());
        let client = recording_client(109406, arguments, reply)?;
        let contract = booster::MoonBoosterContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_boost_data())?;
    }
    {
        let arguments = TVMStack::EMPTY;
        let mut reply = TVMStack::EMPTY;
        reply.push_cell_slice(MsgAddress::NONE.to_cell()?);
        let client = recording_client(127226, arguments, reply)?;
        let contract = booster::MoonBoosterContract::new(&client, &TonAddress::ZERO, None);
        tokio_test::block_on(contract.get_pool())?;
    }
    Ok(())
}

#[test]
fn test_order_factory_native_asset_is_an_addr_none_slice() -> anyhow::Result<()> {
    let mut input = TVMStack::EMPTY;
    input.push_cell_slice(MsgAddress::NONE.to_cell()?);
    let mut reply = TVMStack::EMPTY;
    reply.push_cell_slice(TonAddress::ZERO.to_msg_address_int().to_cell()?);
    let client = recording_client(114750, input, reply)?;
    let factory = order_factory::MoonOrderFactoryContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(tokio_test::block_on(factory.get_pipe(None))?, TonAddress::ZERO);
    Ok(())
}
