//! Every expanded public getter invokes the caller-owned provider with exact arguments.
use super::test_getter_provider::{bonding_stack, fixture_stack, recording_client};
use crate::contracts;
use fastnum::I512;
use ton::ton_core::cell::TonCell;
use ton::{
    block_tlb::TVMStack,
    contracts::TonContract,
    ton_core::{
        traits::tlb::TLB,
        types::{TonAddress, tlb_core::MsgAddress},
    },
};

#[test]
fn test_ston_factory_get_factory_data() -> anyhow::Result<()> {
    let client = recording_client(76063, TVMStack::EMPTY, Some(fixture_stack(9)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::ston_factory::StonksStonFactoryContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_factory_data())?;
    assert!(!result.disabled);
    Ok(())
}

#[test]
fn test_dedust_factory_get_factory_data() -> anyhow::Result<()> {
    let client = recording_client(76063, TVMStack::EMPTY, Some(fixture_stack(10)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::dedust_factory::StonksDedustFactoryContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_factory_data())?;
    assert!(!result.disabled);
    Ok(())
}

#[test]
fn test_custom_wallet_get_tax() -> anyhow::Result<()> {
    let client = recording_client(82014, TVMStack::EMPTY, Some(fixture_stack(11)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::custom_wallet::StonksCustomWalletContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_tax())?;
    assert_eq!(result.buy_tax_permille, I512::from(290));
    Ok(())
}

#[test]
fn test_custom_wallet_get_wallet_data() -> anyhow::Result<()> {
    let client = recording_client(97026, TVMStack::EMPTY, Some(fixture_stack(12)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::custom_wallet::StonksCustomWalletContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_wallet_data())?;
    assert_eq!(result.jetton_balance.to_string(), "85207471148995122");
    Ok(())
}

#[test]
fn test_dedust_wallet_get_wallet_data() -> anyhow::Result<()> {
    let client = recording_client(97026, TVMStack::EMPTY, Some(fixture_stack(6)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::dedust_wallet::StonksDedustWalletContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_wallet_data())?;
    assert_eq!(result.balance.to_string(), "0");
    Ok(())
}

#[test]
fn test_dedust_wallet_get_extended_data() -> anyhow::Result<()> {
    let client = recording_client(84724, TVMStack::EMPTY, Some(fixture_stack(7)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::dedust_wallet::StonksDedustWalletContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_extended_data())?;
    assert!(!result.flag);
    Ok(())
}

#[test]
fn test_ston_wallet_get_wallet_data() -> anyhow::Result<()> {
    let client = recording_client(97026, TVMStack::EMPTY, Some(fixture_stack(6)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::ston_wallet::StonksStonWalletContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_wallet_data())?;
    assert_eq!(result.balance.to_string(), "0");
    Ok(())
}

#[test]
fn test_ston_wallet_get_extended_data() -> anyhow::Result<()> {
    let client = recording_client(74038, TVMStack::EMPTY, Some(fixture_stack(8)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::ston_wallet::StonksStonWalletContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_extended_data())?;
    assert!(result.flag);
    Ok(())
}

#[test]
fn test_ston_minter_get_bonding_data() -> anyhow::Result<()> {
    let client = recording_client(119688, TVMStack::EMPTY, Some(bonding_stack(false)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::ston_minter::StonksStonMinterContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_bonding_data())?;
    assert_eq!(result.max_buy_percent, I512::from(100));
    Ok(())
}

#[test]
fn test_ston_minter_get_jetton_data() -> anyhow::Result<()> {
    let client = recording_client(106029, TVMStack::EMPTY, Some(fixture_stack(5)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::ston_minter::StonksStonMinterContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_jetton_data())?;
    assert!(!result.mintable);
    Ok(())
}

#[test]
fn test_ston_minter_get_version() -> anyhow::Result<()> {
    let client = recording_client(82320, TVMStack::EMPTY, Some(fixture_stack(13)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::ston_minter::StonksStonMinterContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_version())?;
    assert_eq!(result, 11);
    Ok(())
}

#[test]
fn test_ston_minter_get_wallet_address() -> anyhow::Result<()> {
    let owner = MsgAddress::Int(TonAddress::new(0, [0x42; 32].into()).to_msg_address_int());
    let client = recording_client(
        103289,
        {
            let mut stack = TVMStack::EMPTY;
            stack.push_cell_slice(owner.to_cell()?);
            stack
        },
        Some({
            let mut stack = TVMStack::EMPTY;
            stack.push_cell_slice(owner.to_cell()?);
            stack
        }),
    )?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::ston_minter::StonksStonMinterContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_wallet_address(Some(TonAddress::from_cell(&owner.to_cell()?)?)))?;
    assert_eq!(result, Some(TonAddress::from_cell(&owner.to_cell()?)?));
    Ok(())
}

#[test]
fn test_ston_minter_get_extended_data() -> anyhow::Result<()> {
    let client = recording_client(82057, TVMStack::EMPTY, Some(fixture_stack(0)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::ston_minter::StonksStonMinterContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_extended_data())?;
    assert_eq!(result.max_buy_percent, 100);
    Ok(())
}

#[test]
fn test_ston_minter_get_ston_data() -> anyhow::Result<()> {
    let client = recording_client(68631, TVMStack::EMPTY, Some(fixture_stack(1)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::ston_minter::StonksStonMinterContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_ston_data())?;
    assert!(result.presale_open);
    Ok(())
}

#[test]
fn test_ston_minter_get_protocol_addresses() -> anyhow::Result<()> {
    let client = recording_client(95602, TVMStack::EMPTY, Some(fixture_stack(2)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::ston_minter::StonksStonMinterContract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_protocol_addresses())?;
    assert_eq!(
        tonabis_core::address::nullable_address_cell(&result.controller)?,
        TonCell::from_boc_hex(
            "b5ee9c72010101010024000043800f07c63b93028b35508ec5309407d1aeb864086b80158b6cd67b6c967766e38e30"
        )?
    );
    Ok(())
}

#[test]
fn test_dedust_minter_v7_get_bonding_data() -> anyhow::Result<()> {
    let client = recording_client(119688, TVMStack::EMPTY, Some(bonding_stack(false)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::dedust_minter_v7::StonksDedustMinterV7Contract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_bonding_data())?;
    assert_eq!(result.max_buy_percent, I512::from(100));
    Ok(())
}

#[test]
fn test_dedust_minter_v7_get_jetton_data() -> anyhow::Result<()> {
    let client = recording_client(106029, TVMStack::EMPTY, Some(fixture_stack(5)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::dedust_minter_v7::StonksDedustMinterV7Contract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_jetton_data())?;
    assert!(!result.mintable);
    Ok(())
}

#[test]
fn test_dedust_minter_v7_get_version() -> anyhow::Result<()> {
    let client = recording_client(82320, TVMStack::EMPTY, Some(fixture_stack(13)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::dedust_minter_v7::StonksDedustMinterV7Contract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_version())?;
    assert_eq!(result, 11);
    Ok(())
}

#[test]
fn test_dedust_minter_v7_get_wallet_address() -> anyhow::Result<()> {
    let owner = MsgAddress::Int(TonAddress::new(0, [0x42; 32].into()).to_msg_address_int());
    let client = recording_client(
        103289,
        {
            let mut stack = TVMStack::EMPTY;
            stack.push_cell_slice(owner.to_cell()?);
            stack
        },
        Some({
            let mut stack = TVMStack::EMPTY;
            stack.push_cell_slice(owner.to_cell()?);
            stack
        }),
    )?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::dedust_minter_v7::StonksDedustMinterV7Contract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_wallet_address(Some(TonAddress::from_cell(&owner.to_cell()?)?)))?;
    assert_eq!(result, Some(TonAddress::from_cell(&owner.to_cell()?)?));
    Ok(())
}

#[test]
fn test_dedust_minter_v7_get_extended_data() -> anyhow::Result<()> {
    let client = recording_client(82057, TVMStack::EMPTY, Some(fixture_stack(0)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::dedust_minter_v7::StonksDedustMinterV7Contract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_extended_data())?;
    assert_eq!(result.max_buy_percent, 100);
    Ok(())
}

#[test]
fn test_dedust_minter_v12_get_bonding_data() -> anyhow::Result<()> {
    let client = recording_client(119688, TVMStack::EMPTY, Some(bonding_stack(true)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::dedust_minter_v12::StonksDedustMinterV12Contract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_bonding_data())?;
    assert_eq!(result.max_buy_percent, 100);
    Ok(())
}

#[test]
fn test_dedust_minter_v12_get_jetton_data() -> anyhow::Result<()> {
    let client = recording_client(106029, TVMStack::EMPTY, Some(fixture_stack(5)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::dedust_minter_v12::StonksDedustMinterV12Contract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_jetton_data())?;
    assert!(!result.mintable);
    Ok(())
}

#[test]
fn test_dedust_minter_v12_get_version() -> anyhow::Result<()> {
    let client = recording_client(82320, TVMStack::EMPTY, Some(fixture_stack(13)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::dedust_minter_v12::StonksDedustMinterV12Contract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_version())?;
    assert_eq!(result, 11);
    Ok(())
}

#[test]
fn test_dedust_minter_v12_get_wallet_address() -> anyhow::Result<()> {
    let owner = MsgAddress::Int(TonAddress::new(0, [0x42; 32].into()).to_msg_address_int());
    let client = recording_client(
        103289,
        {
            let mut stack = TVMStack::EMPTY;
            stack.push_cell_slice(owner.to_cell()?);
            stack
        },
        Some({
            let mut stack = TVMStack::EMPTY;
            stack.push_cell_slice(owner.to_cell()?);
            stack
        }),
    )?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::dedust_minter_v12::StonksDedustMinterV12Contract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_wallet_address(Some(TonAddress::from_cell(&owner.to_cell()?)?)))?;
    assert_eq!(result, Some(TonAddress::from_cell(&owner.to_cell()?)?));
    Ok(())
}

#[test]
fn test_dedust_minter_v12_get_extended_data() -> anyhow::Result<()> {
    let client = recording_client(82057, TVMStack::EMPTY, Some(fixture_stack(4)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::dedust_minter_v12::StonksDedustMinterV12Contract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_extended_data())?;
    assert_eq!(result.max_buy_percent, 100);
    Ok(())
}

#[test]
fn test_dedust_minter_v12_get_fee_data() -> anyhow::Result<()> {
    let client = recording_client(71772, TVMStack::EMPTY, Some(fixture_stack(3)?))?;
    let address = TonAddress::new(0, [0x24; 32].into());
    let wrapper = contracts::dedust_minter_v12::StonksDedustMinterV12Contract::new(&client, &address, None);
    let result = tokio_test::block_on(wrapper.get_fee_data())?;
    assert_eq!(
        (result.cpmm_fee_bps, result.presale_tax_bps, result.creator_fee_bps),
        (100, 140, 5000)
    );
    Ok(())
}

#[test]
fn test_provider_error_propagates_without_retry() -> anyhow::Result<()> {
    let client = recording_client(82057, TVMStack::EMPTY, None)?;
    let address = TonAddress::ZERO;
    let wrapper = contracts::ston_minter::StonksStonMinterContract::new(&client, &address, None);
    let error = tokio_test::block_on(wrapper.get_extended_data())
        .err()
        .ok_or_else(|| anyhow::anyhow!("expected provider failure"))?;
    assert!(error.to_string().contains("stonks provider sentinel"), "{error}");
    Ok(())
}

#[test]
fn test_nullable_wallet_arguments_remain_slices_on_all_minters() -> anyhow::Result<()> {
    for owner in [None, Some(TonAddress::ZERO)] {
        let raw_owner = match owner {
            Some(address) => MsgAddress::Int(address.to_msg_address_int()),
            None => MsgAddress::NONE,
        };
        let mut arguments = TVMStack::EMPTY;
        arguments.push_cell_slice(raw_owner.to_cell()?);
        let address = TonAddress::ZERO;
        let client = recording_client(103289, arguments.clone(), Some(arguments.clone()))?;
        assert_eq!(
            tokio_test::block_on(
                contracts::ston_minter::StonksStonMinterContract::new(&client, &address, None)
                    .get_wallet_address(owner)
            )?,
            owner
        );
        let client = recording_client(103289, arguments.clone(), Some(arguments.clone()))?;
        assert_eq!(
            tokio_test::block_on(
                contracts::dedust_minter_v7::StonksDedustMinterV7Contract::new(&client, &address, None)
                    .get_wallet_address(owner)
            )?,
            owner
        );
        let client = recording_client(103289, arguments.clone(), Some(arguments))?;
        assert_eq!(
            tokio_test::block_on(
                contracts::dedust_minter_v12::StonksDedustMinterV12Contract::new(&client, &address, None)
                    .get_wallet_address(owner)
            )?,
            owner
        );
    }
    Ok(())
}
