use crate::contracts::custom_wallet::msgs::StonksCustomWalletBouncedKnownBodyMsg;
use crate::contracts::custom_wallet::msgs::StonksCustomWalletJettonBurnMsg;
use crate::contracts::custom_wallet::msgs::StonksCustomWalletJettonInternalTransferMsg;
use crate::contracts::custom_wallet::msgs::StonksCustomWalletJettonTaxTransferMsg;
use crate::contracts::custom_wallet::msgs::StonksCustomWalletJettonTransferMsg;
use crate::contracts::custom_wallet::msgs::StonksCustomWalletJettonTransferNotificationMsg;
use crate::contracts::custom_wallet::storage::StonksCustomWalletJettonTransferContext;
use crate::contracts::custom_wallet::storage::StonksCustomWalletStorage;
use crate::contracts::ston_minter::msgs::StonksStonMinterJettonBurnNotificationMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterJettonExcessesMsg;
use ton::ton_core::{
    cell::TonCell,
    traits::tlb::TLB,
    types::{TonAddress, tlb_core::MsgAddress},
};
use tonabis_core::msgs::EmptyBodyMsg;
#[test]
fn test_schema_jetton_transfer_context() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-custom_wallet-JettonTransferContext.boc").to_vec())?;
    let decoded = StonksCustomWalletJettonTransferContext::from_cell(&cell)?;
    assert_eq!(
        decoded.context_address0,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(
        decoded.context_address1,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_custom_jetton_wallet_storage() -> anyhow::Result<()> {
    let cell =
        TonCell::from_boc(include_bytes!("fixtures/schema-custom_wallet-CustomJettonWalletStorage.boc").to_vec())?;
    let decoded = StonksCustomWalletStorage::from_cell(&cell)?;
    assert_eq!(decoded.jetton_balance.to_u128(), 123);
    assert_eq!(decoded.accounting_balance.to_u128(), 123);
    assert_eq!(
        decoded.owner_address,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(
        decoded.jetton_master_address,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_jetton_transfer() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-custom_wallet-JettonTransfer.boc").to_vec())?;
    let decoded = StonksCustomWalletJettonTransferMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.amount.to_u128(), 123);
    assert_eq!(
        decoded.destination,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(decoded.response_destination, None);
    assert_eq!(decoded.forward_ton_amount.to_u128(), 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_jetton_internal_transfer() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-custom_wallet-JettonInternalTransfer.boc").to_vec())?;
    let decoded = StonksCustomWalletJettonInternalTransferMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.amount.to_u128(), 123);
    assert_eq!(decoded.sender_owner, None);
    assert_eq!(decoded.response_destination, None);
    assert_eq!(decoded.forward_ton_amount.to_u128(), 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_jetton_burn() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-custom_wallet-JettonBurn.boc").to_vec())?;
    let decoded = StonksCustomWalletJettonBurnMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.amount.to_u128(), 123);
    assert_eq!(decoded.response_destination, None);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_jetton_tax_transfer() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-custom_wallet-JettonTaxTransfer.boc").to_vec())?;
    let decoded = StonksCustomWalletJettonTaxTransferMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.amount.to_u128(), 123);
    assert_eq!(
        decoded.sender_owner,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(
        decoded.context_address0,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(
        decoded.context_address1,
        tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
            TonAddress::ZERO.to_msg_address_int()
        ))?
    );
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_custom_jetton_wallet_empty_body() -> anyhow::Result<()> {
    let cell =
        TonCell::from_boc(include_bytes!("fixtures/schema-custom_wallet-CustomJettonWalletEmptyBody.boc").to_vec())?;
    let decoded = EmptyBodyMsg::from_cell(&cell)?;
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_custom_jetton_wallet_bounced_known_body() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(
        include_bytes!("fixtures/schema-custom_wallet-CustomJettonWalletBouncedKnownBody.boc").to_vec(),
    )?;
    let decoded = StonksCustomWalletBouncedKnownBodyMsg::from_cell(&cell)?;
    assert_eq!(decoded.original_opcode, 0x178d4519);
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.amount.to_u128(), 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_jetton_transfer_notification() -> anyhow::Result<()> {
    let cell =
        TonCell::from_boc(include_bytes!("fixtures/schema-custom_wallet-JettonTransferNotification.boc").to_vec())?;
    let decoded = StonksCustomWalletJettonTransferNotificationMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.amount.to_u128(), 123);
    assert_eq!(decoded.sender_owner, None);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_jetton_excess() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-custom_wallet-JettonExcess.boc").to_vec())?;
    let decoded = StonksStonMinterJettonExcessesMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 123);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_schema_jetton_burn_notification() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/schema-custom_wallet-JettonBurnNotification.boc").to_vec())?;
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
