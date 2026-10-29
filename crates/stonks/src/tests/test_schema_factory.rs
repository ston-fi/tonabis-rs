use crate::contracts::dedust_factory::storage::StonksDedustFactoryStorage;
use crate::contracts::ston_factory::msgs::StonksStonFactoryCreateVirtualLiquidityJettonMsg;
use crate::contracts::ston_factory::msgs::StonksStonFactorySetDisabledMsg;
use crate::contracts::ston_factory::msgs::StonksStonFactoryWithdrawTonMsg;
use crate::contracts::ston_factory::storage::StonksStonFactoryStorage;
use crate::contracts::ston_minter::msgs::StonksStonMinterVirtualDevBuyMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterVirtualMinterInitMsg;
use ton::ton_core::{
    cell::TonCell,
    traits::tlb::TLB,
    types::{TonAddress, tlb_core::MsgAddress},
};
#[test]
fn test_schema_virtual_factory_stonfi_storage() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-factory-VirtualFactoryStonfiStorage.boc").to_vec())?;
    let decoded = StonksStonFactoryStorage::from_cell(&cell)?;
    assert_eq!(
        decoded.router_address,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert!(decoded.disabled);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_virtual_factory_dedust_storage() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-factory-VirtualFactoryDedustStorage.boc").to_vec())?;
    let decoded = StonksDedustFactoryStorage::from_cell(&cell)?;
    assert!(decoded.disabled);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_create_virtual_liquidity_jetton() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-factory-CreateVirtualLiquidityJetton.boc").to_vec())?;
    let decoded = StonksStonFactoryCreateVirtualLiquidityJettonMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.dev_buy_ton_amount.to_u128(), 123);
    assert_eq!(decoded.max_buy_percent, 123);
    assert_eq!(
        decoded.dev_wallet,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_virtual_factory_withdraw_ton() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-factory-VirtualFactoryWithdrawTon.boc").to_vec())?;
    let decoded = StonksStonFactoryWithdrawTonMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.amount.to_u128(), 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_virtual_factory_set_disabled() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-factory-VirtualFactorySetDisabled.boc").to_vec())?;
    let decoded = StonksStonFactorySetDisabledMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert!(decoded.disabled);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_virtual_factory_dev_buy() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-factory-VirtualFactoryDevBuy.boc").to_vec())?;
    let decoded = StonksStonMinterVirtualDevBuyMsg::from_cell(&cell)?;
    assert_eq!(decoded.ton_amount.to_u128(), 123);
    assert_eq!(
        decoded.recipient,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(decoded.min_jettons_out.to_u128(), 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_virtual_minter_factory_init() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-factory-VirtualMinterFactoryInit.boc").to_vec())?;
    let decoded = StonksStonMinterVirtualMinterInitMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert!(decoded.presale_open);
    assert_eq!(decoded.migration_ton_target.to_u128(), 123);
    assert_eq!(decoded.tons_collected.to_u128(), 123);
    assert_eq!(decoded.base_jetton_reserve.to_u128(), 123);
    assert_eq!(decoded.additional_ton_reserve.to_u128(), 123);
    assert_eq!(decoded.additional_jetton_reserve.to_u128(), 123);
    assert_eq!(decoded.max_buy_percent, 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
