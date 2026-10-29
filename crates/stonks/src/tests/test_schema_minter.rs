use crate::contracts::ston_minter::msgs::StonksStonMinterAskToPresaleSellMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterBuyFromPresaleMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterChangeAdminMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterChangeContentMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterJettonBurnNotificationMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterJettonExcessesMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterJettonInternalTransferMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterJettonMintMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterPresaleSellNotificationForMinterMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterPresaleTradeEventMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterProvideWalletAddressMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterTakeWalletAddressMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterVirtualDevBuyMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterVirtualMinterInitMsg;
use crate::contracts::ston_minter::storage::StonksVirtualMinterBondingState;
use crate::contracts::ston_minter::storage::StonksVirtualMinterStorage;
use ton::ton_core::{
    cell::TonCell,
    traits::tlb::TLB,
    types::{TonAddress, tlb_core::MsgAddress},
};
#[test]
fn test_schema_virtual_minter_storage() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-VirtualMinterStorage.boc").to_vec())?;
    let decoded = StonksVirtualMinterStorage::from_cell(&cell)?;
    assert_eq!(decoded.total_supply.to_u128(), 123);
    assert_eq!(
        decoded.admin_address,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_virtual_minter_bonding_state() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-VirtualMinterBondingState.boc").to_vec())?;
    let decoded = StonksVirtualMinterBondingState::from_cell(&cell)?;
    assert!(decoded.presale_open);
    assert_eq!(decoded.migration_ton_target.to_u128(), 123);
    assert_eq!(decoded.tons_collected.to_u128(), 123);
    assert_eq!(decoded.base_jetton_reserve.to_u128(), 123);
    assert_eq!(decoded.additional_ton_reserve.to_u128(), 123);
    assert_eq!(decoded.additional_jetton_reserve.to_u128(), 123);
    assert_eq!(decoded.max_buy_percent, 123);
    assert_eq!(decoded.tokens_sold.to_u128(), 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_virtual_dev_buy() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-VirtualDevBuy.boc").to_vec())?;
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
fn test_schema_jetton_mint() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-JettonMint.boc").to_vec())?;
    let decoded = StonksStonMinterJettonMintMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(
        decoded.recipient,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(decoded.forward_ton_amount.to_u128(), 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_jetton_burn_notification() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-JettonBurnNotification.boc").to_vec())?;
    let decoded = StonksStonMinterJettonBurnNotificationMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.amount.to_u128(), 123);
    assert_eq!(
        decoded.owner,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(decoded.response_address, None);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_provide_wallet_address() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-ProvideWalletAddress.boc").to_vec())?;
    let decoded = StonksStonMinterProvideWalletAddressMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(
        decoded.owner_address,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert!(decoded.include_address);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_change_admin() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-ChangeAdmin.boc").to_vec())?;
    let decoded = StonksStonMinterChangeAdminMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(
        decoded.new_admin_address,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_change_content() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-ChangeContent.boc").to_vec())?;
    let decoded = StonksStonMinterChangeContentMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_virtual_minter_init() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-VirtualMinterInit.boc").to_vec())?;
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
#[test]
fn test_schema_buy_from_presale() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-BuyFromPresale.boc").to_vec())?;
    let decoded = StonksStonMinterBuyFromPresaleMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.ton_amount.to_u128(), 123);
    assert_eq!(decoded.min_jettons_out.to_u128(), 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_presale_sell_notification_for_minter() -> anyhow::Result<()> {
    let cell =
        TonCell::from_boc(include_bytes!("fixtures/schema-minter-PresaleSellNotificationForMinter.boc").to_vec())?;
    let decoded = StonksStonMinterPresaleSellNotificationForMinterMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.jetton_amount.to_u128(), 123);
    assert_eq!(
        decoded.sell_initiator,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(decoded.tons_recipient, None);
    assert_eq!(decoded.min_ton_out.to_u128(), 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_ask_to_presale_sell() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-AskToPresaleSell.boc").to_vec())?;
    let decoded = StonksStonMinterAskToPresaleSellMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.jetton_amount.to_u128(), 123);
    assert_eq!(decoded.tons_recipient, None);
    assert_eq!(decoded.min_ton_out.to_u128(), 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_jetton_internal_transfer() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-JettonInternalTransfer.boc").to_vec())?;
    let decoded = StonksStonMinterJettonInternalTransferMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.amount.to_u128(), 123);
    assert_eq!(decoded.from_address, None);
    assert_eq!(decoded.response_address, None);
    assert_eq!(decoded.forward_ton_amount.to_u128(), 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_jetton_excesses() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-JettonExcesses.boc").to_vec())?;
    let decoded = StonksStonMinterJettonExcessesMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_take_wallet_address() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-TakeWalletAddress.boc").to_vec())?;
    let decoded = StonksStonMinterTakeWalletAddressMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.wallet_address, None);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_presale_trade_event() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-minter-PresaleTradeEvent.boc").to_vec())?;
    let decoded = StonksStonMinterPresaleTradeEventMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert!(decoded.is_buy);
    assert_eq!(decoded.amount_in.to_u128(), 123);
    assert_eq!(decoded.amount_out.to_u128(), 123);
    assert_eq!(
        decoded.recipient,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(decoded.leftover.to_u128(), 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
