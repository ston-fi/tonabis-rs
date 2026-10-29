use crate::contracts::custom_wallet::method_results::StonksCustomWalletGetTaxResult;
use crate::contracts::dedust_minter_v12::method_results::StonksDedustMinterV12GetFeeDataResult;
use crate::contracts::ston_minter::method_results::StonksStonMinterGetJettonDataResult;
use crate::contracts::ston_minter::method_results::StonksStonMinterGetProtocolAddressesResult;
use crate::contracts::ston_minter::method_results::StonksStonMinterGetStonDataResult;
use fastnum::I512;
use ton::block_tlb::{FromTVMStack, TVMStack};
use ton::ton_core::{cell::TonCell, traits::tlb::TLB};
fn stack(id: usize) -> anyhow::Result<TVMStack> {
    let values: Vec<serde_json::Value> = serde_json::from_str(include_str!("fixtures/getter-stacks.json"))?;
    stack_from_rows(&values, id)
}
fn stack_from_rows(values: &[serde_json::Value], id: usize) -> anyhow::Result<TVMStack> {
    let mut stack = TVMStack::new(vec![]);
    for v in values[id]["stack"].as_array().ok_or_else(|| anyhow::anyhow!("stack"))? {
        let value = v["value"].as_str().ok_or_else(|| anyhow::anyhow!("value"))?;
        match v["type"].as_str() {
            Some("int") => stack.push_tiny_int(value.parse()?),
            Some("slice") => stack.push_cell_slice(TonCell::from_boc_hex(value)?),
            Some("cell") => stack.push_cell(TonCell::from_boc_hex(value)?),
            _ => anyhow::bail!("stack type"),
        };
    }
    Ok(stack)
}
#[test]
fn test_getter_minter_extendedbondingdata() -> anyhow::Result<()> {
    let original = stack(0)?;
    let decoded = crate::contracts::ston_minter::method_results::StonksStonMinterGetExtendedDataResult::from_stack(
        &mut original.clone(),
    )?;
    let _ = &decoded;
    assert_eq!(decoded.max_buy_percent, 100);
    let mut extra = original.clone();
    extra.push_tiny_int(9);
    assert!(
        crate::contracts::ston_minter::method_results::StonksStonMinterGetExtendedDataResult::from_stack(&mut extra)
            .is_err()
    );
    Ok(())
}
#[test]
fn test_getter_ston_extendeddata() -> anyhow::Result<()> {
    let original = stack(1)?;
    let decoded = StonksStonMinterGetStonDataResult::from_stack(&mut original.clone())?;
    let _ = &decoded;
    assert!(decoded.presale_open);
    let mut extra = original.clone();
    extra.push_tiny_int(9);
    assert!(StonksStonMinterGetStonDataResult::from_stack(&mut extra).is_err());
    Ok(())
}
#[test]
fn test_getter_ston_protocoladdresses() -> anyhow::Result<()> {
    let original = stack(2)?;
    let decoded = StonksStonMinterGetProtocolAddressesResult::from_stack(&mut original.clone())?;
    let _ = &decoded;
    let mut extra = original.clone();
    extra.push_tiny_int(9);
    assert!(StonksStonMinterGetProtocolAddressesResult::from_stack(&mut extra).is_err());
    Ok(())
}
#[test]
fn test_getter_dedust_v12_feedata() -> anyhow::Result<()> {
    let original = stack(3)?;
    let decoded = StonksDedustMinterV12GetFeeDataResult::from_stack(&mut original.clone())?;
    let _ = &decoded;
    assert_eq!(decoded.cpmm_fee_bps, 100);
    assert_eq!(decoded.presale_tax_bps, 140);
    let mut extra = original.clone();
    extra.push_tiny_int(9);
    assert!(StonksDedustMinterV12GetFeeDataResult::from_stack(&mut extra).is_err());
    Ok(())
}
#[test]
fn test_getter_dedust_v12_extendedbondingdata() -> anyhow::Result<()> {
    let original = stack(4)?;
    let decoded =
        crate::contracts::dedust_minter_v12::method_results::StonksDedustMinterV12GetExtendedDataResult::from_stack(
            &mut original.clone(),
        )?;
    let _ = &decoded;
    assert_eq!(decoded.creator_fee_bps, 5000);
    assert_eq!(decoded.presale_tax_bps, 140);
    let mut extra = original.clone();
    extra.push_tiny_int(9);
    assert!(
        crate::contracts::dedust_minter_v12::method_results::StonksDedustMinterV12GetExtendedDataResult::from_stack(
            &mut extra
        )
        .is_err()
    );
    Ok(())
}
#[test]
fn test_getter_minter_jettondata() -> anyhow::Result<()> {
    let original = stack(5)?;
    let decoded = StonksStonMinterGetJettonDataResult::from_stack(&mut original.clone())?;
    let _ = &decoded;
    assert!(!decoded.mintable);
    let mut extra = original.clone();
    extra.push_tiny_int(9);
    assert!(StonksStonMinterGetJettonDataResult::from_stack(&mut extra).is_err());
    Ok(())
}
#[test]
fn test_getter_launch_wallet_walletdata() -> anyhow::Result<()> {
    let original = stack(6)?;
    let decoded = crate::contracts::dedust_wallet::method_results::StonksDedustWalletGetWalletDataResult::from_stack(
        &mut original.clone(),
    )?;
    let _ = &decoded;
    let mut extra = original.clone();
    extra.push_tiny_int(9);
    assert!(
        crate::contracts::dedust_wallet::method_results::StonksDedustWalletGetWalletDataResult::from_stack(&mut extra)
            .is_err()
    );
    Ok(())
}
#[test]
fn test_getter_launch_wallet_extendedwalletdata() -> anyhow::Result<()> {
    let original = stack(7)?;
    let decoded = crate::contracts::dedust_wallet::method_results::StonksDedustWalletGetExtendedDataResult::from_stack(
        &mut original.clone(),
    )?;
    let _ = &decoded;
    assert!(!decoded.flag);
    let mut extra = original.clone();
    extra.push_tiny_int(9);
    assert!(
        crate::contracts::dedust_wallet::method_results::StonksDedustWalletGetExtendedDataResult::from_stack(
            &mut extra
        )
        .is_err()
    );
    Ok(())
}
#[test]
fn test_getter_ston_extendedwalletdata() -> anyhow::Result<()> {
    let original = stack(8)?;
    let decoded = crate::contracts::ston_wallet::method_results::StonksStonWalletGetExtendedDataResult::from_stack(
        &mut original.clone(),
    )?;
    let _ = &decoded;
    assert!(decoded.flag);
    let mut extra = original.clone();
    extra.push_tiny_int(9);
    assert!(
        crate::contracts::ston_wallet::method_results::StonksStonWalletGetExtendedDataResult::from_stack(&mut extra)
            .is_err()
    );
    Ok(())
}
#[test]
fn test_getter_factory_virtualfactorystonfidata() -> anyhow::Result<()> {
    let original = stack(9)?;
    let decoded = crate::contracts::ston_factory::method_results::StonksStonFactoryGetFactoryDataResult::from_stack(
        &mut original.clone(),
    )?;
    let _ = &decoded;
    assert!(!decoded.disabled);
    let mut extra = original.clone();
    extra.push_tiny_int(9);
    assert!(
        crate::contracts::ston_factory::method_results::StonksStonFactoryGetFactoryDataResult::from_stack(&mut extra)
            .is_err()
    );
    Ok(())
}
#[test]
fn test_getter_factory_virtualfactorydedustdata() -> anyhow::Result<()> {
    let original = stack(10)?;
    let decoded =
        crate::contracts::dedust_factory::method_results::StonksDedustFactoryGetFactoryDataResult::from_stack(
            &mut original.clone(),
        )?;
    let _ = &decoded;
    assert!(!decoded.disabled);
    let mut extra = original.clone();
    extra.push_tiny_int(9);
    assert!(
        crate::contracts::dedust_factory::method_results::StonksDedustFactoryGetFactoryDataResult::from_stack(
            &mut extra
        )
        .is_err()
    );
    Ok(())
}
#[test]
fn test_getter_custom_wallet_jettontaxrates() -> anyhow::Result<()> {
    let original = stack(11)?;
    let decoded = StonksCustomWalletGetTaxResult::from_stack(&mut original.clone())?;
    let _ = &decoded;
    assert_eq!(decoded.buy_tax_permille, I512::from(290));
    let mut extra = original.clone();
    extra.push_tiny_int(9);
    assert!(StonksCustomWalletGetTaxResult::from_stack(&mut extra).is_err());
    Ok(())
}
#[test]
fn test_getter_custom_wallet_customjettonwalletdata() -> anyhow::Result<()> {
    let original = stack(12)?;
    let decoded = crate::contracts::custom_wallet::method_results::StonksCustomWalletGetWalletDataResult::from_stack(
        &mut original.clone(),
    )?;
    let _ = &decoded;
    let mut extra = original.clone();
    extra.push_tiny_int(9);
    assert!(
        crate::contracts::custom_wallet::method_results::StonksCustomWalletGetWalletDataResult::from_stack(&mut extra)
            .is_err()
    );
    Ok(())
}
#[test]
fn test_getter_generation_version() -> anyhow::Result<()> {
    let original = stack(13)?;
    let decoded = u32::from_stack(&mut original.clone())?;
    let _ = &decoded;
    assert_eq!(decoded, 11);
    let mut extra = original.clone();
    extra.push_tiny_int(9);
    assert_eq!(u32::from_stack(&mut extra)?, 9);
    assert!(!extra.is_empty());
    Ok(())
}
