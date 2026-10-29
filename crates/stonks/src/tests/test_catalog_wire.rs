use crate::contracts::custom_wallet::msgs::StonksCustomWalletBouncedKnownBodyMsg;
use crate::contracts::custom_wallet::msgs::StonksCustomWalletJettonTaxTransferMsg;
use crate::contracts::custom_wallet::msgs::StonksCustomWalletJettonTransferMsg;
use crate::contracts::custom_wallet::msgs::StonksCustomWalletJettonTransferNotificationMsg;
use crate::contracts::ston_factory::msgs::StonksStonFactoryCreateVirtualLiquidityJettonMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterBuyFromPresaleMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterPresaleSellNotificationForMinterMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterPresaleTradeEventMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterVirtualMinterInitMsg;
use ton::ton_core::{cell::TonCell, traits::tlb::TLB};
#[test]
fn test_virtual_factory_0() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/virtual_factory-0.boc").to_vec())?;
    let decoded = StonksStonFactoryCreateVirtualLiquidityJettonMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 0);
    assert_eq!(decoded.dev_buy_ton_amount.to_u128(), 20000000000);
    assert_eq!(decoded.max_buy_percent, 100);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_virtual_factory_1() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/virtual_factory-1.boc").to_vec())?;
    let decoded = StonksStonMinterVirtualMinterInitMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 0);
    assert!(decoded.presale_open);
    assert_eq!(decoded.migration_ton_target.to_u128(), 2000000000000);
    assert_eq!(decoded.tons_collected.to_u128(), 0);
    assert_eq!(decoded.base_jetton_reserve.to_u128(), 100000000000000000);
    assert_eq!(decoded.additional_ton_reserve.to_u128(), 1992015968062);
    assert_eq!(decoded.additional_jetton_reserve.to_u128(), 49700598802325000);
    assert_eq!(decoded.max_buy_percent, 100);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_virtual_minter_0() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/virtual_minter-0.boc").to_vec())?;
    let decoded = StonksStonMinterVirtualMinterInitMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 0);
    assert!(decoded.presale_open);
    assert_eq!(decoded.migration_ton_target.to_u128(), 2000000000000);
    assert_eq!(decoded.tons_collected.to_u128(), 0);
    assert_eq!(decoded.base_jetton_reserve.to_u128(), 100000000000000000);
    assert_eq!(decoded.additional_ton_reserve.to_u128(), 997750562359);
    assert_eq!(decoded.additional_jetton_reserve.to_u128(), 12471882029487500);
    assert_eq!(decoded.max_buy_percent, 100);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_virtual_minter_1() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/virtual_minter-1.boc").to_vec())?;
    let decoded = StonksStonMinterBuyFromPresaleMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 5790209817526438628);
    assert_eq!(decoded.ton_amount.to_u128(), 10000000000);
    assert_eq!(decoded.min_jettons_out.to_u128(), 995023146122475);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_virtual_minter_2() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/virtual_minter-2.boc").to_vec())?;
    let decoded = StonksStonMinterPresaleSellNotificationForMinterMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 4932221010697934584);
    assert_eq!(decoded.jetton_amount.to_u128(), 1004405849249668);
    assert_eq!(decoded.min_ton_out.to_u128(), 8733041382);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_virtual_minter_3() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/virtual_minter-3.boc").to_vec())?;
    let decoded = StonksStonMinterPresaleTradeEventMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 4932221010697934584);
    assert!(!decoded.is_buy);
    assert_eq!(decoded.amount_in.to_u128(), 1004405849249668);
    assert_eq!(decoded.amount_out.to_u128(), 9003135446);
    assert_eq!(decoded.leftover.to_u128(), 0);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_virtual_minter_4() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/virtual_minter-4.boc").to_vec())?;
    let decoded = crate::contracts::ston_minter::msgs::StonksStonMinterJettonInternalTransferMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 5790209817526438628);
    assert_eq!(decoded.amount.to_u128(), 996035525638513);
    assert_eq!(decoded.forward_ton_amount.to_u128(), 10000000);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_custom_jetton_wallet_0() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/custom_jetton_wallet-0.boc").to_vec())?;
    let decoded = StonksCustomWalletJettonTransferMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 4245818663139325215);
    assert_eq!(decoded.amount.to_u128(), 27537513875642);
    assert_eq!(decoded.forward_ton_amount.to_u128(), 0);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_custom_jetton_wallet_1() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/custom_jetton_wallet-1.boc").to_vec())?;
    let decoded = crate::contracts::custom_wallet::msgs::StonksCustomWalletJettonInternalTransferMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 10221304187694590303);
    assert_eq!(decoded.amount.to_u128(), 776351605663938);
    assert_eq!(decoded.forward_ton_amount.to_u128(), 200000000);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_custom_jetton_wallet_2() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/custom_jetton_wallet-2.boc").to_vec())?;
    let decoded = StonksCustomWalletJettonTransferNotificationMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 10221304187694590303);
    assert_eq!(decoded.amount.to_u128(), 776351605663938);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_custom_jetton_wallet_3() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/custom_jetton_wallet-3.boc").to_vec())?;
    let decoded = StonksCustomWalletJettonTaxTransferMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 4245818663139325215);
    assert_eq!(decoded.amount.to_u128(), 7985879023936);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_custom_jetton_wallet_4() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/custom_jetton_wallet-4.boc").to_vec())?;
    let decoded = StonksCustomWalletBouncedKnownBodyMsg::from_cell(&cell)?;
    assert_eq!(decoded.original_opcode, 0x178d4519);
    assert_eq!(decoded.query_id, 0);
    assert_eq!(decoded.amount.to_u128(), 28509452102784);
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
