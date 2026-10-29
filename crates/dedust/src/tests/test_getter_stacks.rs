//! Captured results qualify real getter selectors, packing, values and exact arity.
//! Expected stacks are independent; program-code substitutions stay explicit.
#![allow(non_snake_case)]
use super::test_getter_provider::{cell, fixture_arguments, fixture_stack, fixture_values, integer, recording_client};
use crate::contracts::classic::asset::DedustClassicAsset;
use crate::contracts::classic::factory_v1::DedustFactoryV1Contract;
use crate::contracts::classic::factory_v2::DedustFactoryV2Contract;
use crate::contracts::classic::liquidity_deposit_v1::DedustLiquidityDepositV1Contract;
use crate::contracts::classic::liquidity_deposit_v2::DedustLiquidityDepositV2Contract;
use crate::contracts::classic::lp_wallet::DedustClassicLpWalletContract;
use crate::contracts::classic::operator_pool::DedustClassicOperatorPoolContract;
use crate::contracts::classic::pool_v1::DedustPoolV1Contract;
use crate::contracts::classic::pool_v2::DedustPoolV2Contract;
use crate::contracts::classic::vault_jetton_v1::DedustVaultJettonV1Contract;
use crate::contracts::classic::vault_jetton_v2::DedustVaultJettonV2Contract;
use crate::contracts::classic::vault_native_v1::DedustVaultNativeV1Contract;
use crate::contracts::classic::vault_native_v2::DedustVaultNativeV2Contract;
use crate::contracts::cpmm_v2::deposit::DedustV2CpmmDepositContract;
use crate::contracts::cpmm_v2::pool::DedustV2CpmmContract;
use crate::contracts::cpmm_v2::position::DedustV2CpmmPositionContract;
use crate::contracts::custom_wallet::v2::DedustX1000WalletV2Contract;
use crate::contracts::operator::v2::DedustOperatorV2Contract;
use crate::contracts::uranus::meme_v2::DedustUranusMemeV2Contract;
use crate::contracts::uranus::meme_v3::DedustUranusMemeV3Contract;
use crate::contracts::uranus::wallet_v2::DedustUranusMemeWalletV2Contract;
use crate::contracts::uranus::wallet_v3::DedustUranusMemeWalletV3Contract;
use fastnum::{I512, U256};
use ton::ton_core::types::Coins;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::{
    block_tlb::{TVMInt, TVMNull, TVMStackValue},
    contracts::TonContract,
    ton_core::{traits::tlb::TLB, types::TonAddress},
};
#[test]
fn test_raw_getter_DedustFactoryV1_110004() -> anyhow::Result<()> {
    let values = fixture_values("DedustFactoryV1", 110004)?;
    let client = recording_client(110004, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustFactoryV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_ownership())?;
    assert_eq!(value.owner_address, NullableAddress.read(&mut cell(&values[0])?.parser())?);
    assert_eq!(
        value.pending_owner_address,
        NullableAddress.read(&mut cell(&values[1])?.parser())?
    );
    assert_eq!(
        value.can_accept_after,
        <u32>::try_from(integer(&values[2])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(110004, fixture_arguments(&[])?, extra)?;
    let contract = DedustFactoryV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_ownership()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(110004, fixture_arguments(&[])?, missing)?;
    let contract = DedustFactoryV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_ownership()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustFactoryV1_82320() -> anyhow::Result<()> {
    let values = fixture_values("DedustFactoryV1", 82320)?;
    let client = recording_client(82320, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustFactoryV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_version())?;
    assert_eq!(
        value,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustFactoryV1_76695() -> anyhow::Result<()> {
    let values = fixture_values("DedustFactoryV1", 76695)?;
    let client = recording_client(76695, fixture_arguments(&["native"])?, fixture_stack(values)?)?;
    let contract = DedustFactoryV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(
        contract.get_vault_address(DedustClassicAsset::from_cell(&fixture_arguments(&["native"])?.pop_cell()?)?),
    )?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustFactoryV1_101789() -> anyhow::Result<()> {
    let values = fixture_values("DedustFactoryV1", 101789)?;
    let client = recording_client(101789, fixture_arguments(&["1", "native", "jetton"])?, fixture_stack(values)?)?;
    let contract = DedustFactoryV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_pool_address(
        true,
        DedustClassicAsset::from_cell(&fixture_arguments(&["native"])?.pop_cell()?)?,
        DedustClassicAsset::from_cell(&fixture_arguments(&["jetton"])?.pop_cell()?)?,
    ))?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustFactoryV1_84481() -> anyhow::Result<()> {
    let values = fixture_values("DedustFactoryV1", 84481)?;
    let client = recording_client(
        84481,
        fixture_arguments(&["owner", "1", "native", "jetton"])?,
        fixture_stack(values)?,
    )?;
    let contract = DedustFactoryV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_liquidity_deposit_address(
        TonAddress::ZERO,
        true,
        DedustClassicAsset::from_cell(&fixture_arguments(&["native"])?.pop_cell()?)?,
        DedustClassicAsset::from_cell(&fixture_arguments(&["jetton"])?.pop_cell()?)?,
    ))?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustFactoryV2_110004() -> anyhow::Result<()> {
    let values = fixture_values("DedustFactoryV2", 110004)?;
    let client = recording_client(110004, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustFactoryV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_ownership())?;
    assert_eq!(value.owner_address, NullableAddress.read(&mut cell(&values[0])?.parser())?);
    assert_eq!(
        value.pending_owner_address,
        NullableAddress.read(&mut cell(&values[1])?.parser())?
    );
    assert_eq!(
        value.can_accept_after,
        <u32>::try_from(integer(&values[2])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(110004, fixture_arguments(&[])?, extra)?;
    let contract = DedustFactoryV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_ownership()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(110004, fixture_arguments(&[])?, missing)?;
    let contract = DedustFactoryV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_ownership()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustFactoryV2_82320() -> anyhow::Result<()> {
    let values = fixture_values("DedustFactoryV2", 82320)?;
    let client = recording_client(82320, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustFactoryV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_version())?;
    assert_eq!(
        value,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustFactoryV2_76695() -> anyhow::Result<()> {
    let values = fixture_values("DedustFactoryV2", 76695)?;
    let client = recording_client(76695, fixture_arguments(&["native"])?, fixture_stack(values)?)?;
    let contract = DedustFactoryV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(
        contract.get_vault_address(DedustClassicAsset::from_cell(&fixture_arguments(&["native"])?.pop_cell()?)?),
    )?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustFactoryV2_101789() -> anyhow::Result<()> {
    let values = fixture_values("DedustFactoryV2", 101789)?;
    let client = recording_client(101789, fixture_arguments(&["1", "native", "jetton"])?, fixture_stack(values)?)?;
    let contract = DedustFactoryV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_pool_address(
        true,
        DedustClassicAsset::from_cell(&fixture_arguments(&["native"])?.pop_cell()?)?,
        DedustClassicAsset::from_cell(&fixture_arguments(&["jetton"])?.pop_cell()?)?,
    ))?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustFactoryV2_84481() -> anyhow::Result<()> {
    let values = fixture_values("DedustFactoryV2", 84481)?;
    let client = recording_client(
        84481,
        fixture_arguments(&["owner", "1", "native", "jetton"])?,
        fixture_stack(values)?,
    )?;
    let contract = DedustFactoryV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_liquidity_deposit_address(
        TonAddress::ZERO,
        true,
        DedustClassicAsset::from_cell(&fixture_arguments(&["native"])?.pop_cell()?)?,
        DedustClassicAsset::from_cell(&fixture_arguments(&["jetton"])?.pop_cell()?)?,
    ))?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV1_82320() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV1", 82320)?;
    let client = recording_client(82320, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_version())?;
    assert_eq!(
        value,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV1_103723() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV1", 103723)?;
    let client = recording_client(103723, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.is_stable())?;
    assert_eq!(value, integer(&values[0])? != I512::ZERO);
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV1_96780() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV1", 96780)?;
    let client = recording_client(96780, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_trade_fee())?;
    assert_eq!(
        value.trade_fee_numerator,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(
        value.trade_fee_denominator,
        <u16>::try_from(integer(&values[1])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(96780, fixture_arguments(&[])?, extra)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_trade_fee()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(96780, fixture_arguments(&[])?, missing)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_trade_fee()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV1_65971() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV1", 65971)?;
    let client = recording_client(65971, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_reserves())?;
    assert_eq!(value.reserve0, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.reserve1, Coins::try_from(integer(&values[1])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(65971, fixture_arguments(&[])?, extra)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_reserves()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(65971, fixture_arguments(&[])?, missing)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_reserves()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV1_112792() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV1", 112792)?;
    let client = recording_client(112792, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_protocol_fees())?;
    assert_eq!(value.asset0_protocol_fee, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.asset1_protocol_fee, Coins::try_from(integer(&values[1])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(112792, fixture_arguments(&[])?, extra)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_protocol_fees()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(112792, fixture_arguments(&[])?, missing)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_protocol_fees()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV1_118188() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV1", 118188)?;
    let client = recording_client(118188, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_assets())?;
    assert_eq!(value.asset0, DedustClassicAsset::from_cell(&cell(&values[0])?)?);
    assert_eq!(value.asset1, DedustClassicAsset::from_cell(&cell(&values[1])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(118188, fixture_arguments(&[])?, extra)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_assets()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(118188, fixture_arguments(&[])?, missing)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_assets()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV1_70754() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV1", 70754)?;
    let client = recording_client(70754, fixture_arguments(&["native", "1000000"])?, fixture_stack(values)?)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_swap_out(
        DedustClassicAsset::from_cell(&fixture_arguments(&["native"])?.pop_cell()?)?,
        1000000.into(),
    ))?;
    assert_eq!(value.asset_out, DedustClassicAsset::from_cell(&cell(&values[0])?)?);
    assert_eq!(value.amount_out, integer(&values[1])?);
    assert_eq!(value.trade_fee, integer(&values[2])?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(70754, fixture_arguments(&["native", "1000000"])?, extra)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(
        tokio_test::block_on(contract.estimate_swap_out(
            DedustClassicAsset::from_cell(&fixture_arguments(&["native"])?.pop_cell()?)?,
            1000000.into()
        ))
        .is_err()
    );
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(70754, fixture_arguments(&["native", "1000000"])?, missing)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(
        tokio_test::block_on(contract.estimate_swap_out(
            DedustClassicAsset::from_cell(&fixture_arguments(&["native"])?.pop_cell()?)?,
            1000000.into()
        ))
        .is_err()
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV1_119877() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV1", 119877)?;
    let client = recording_client(119877, fixture_arguments(&["1000000", "1000000"])?, fixture_stack(values)?)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_deposit_out(1000000.into(), 1000000.into()))?;
    assert_eq!(value.asset0_deposit, integer(&values[0])?);
    assert_eq!(value.asset1_deposit, integer(&values[1])?);
    assert_eq!(value.fair_supply, integer(&values[2])?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(119877, fixture_arguments(&["1000000", "1000000"])?, extra)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.estimate_deposit_out(1000000.into(), 1000000.into())).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(119877, fixture_arguments(&["1000000", "1000000"])?, missing)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.estimate_deposit_out(1000000.into(), 1000000.into())).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV1_106029() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV1", 106029)?;
    let client = recording_client(106029, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_jetton_data())?;
    assert_eq!(value.total_supply, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.mintable, integer(&values[1])? != I512::ZERO);
    assert_eq!(value.admin_address, NullableAddress.read(&mut cell(&values[2])?.parser())?);
    assert_eq!(value.jetton_content, cell(&values[3])?);
    assert_eq!(value.jetton_wallet_code, cell(&values[4])?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(106029, fixture_arguments(&[])?, extra)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_jetton_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(106029, fixture_arguments(&[])?, missing)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_jetton_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV1_103289() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV1", 103289)?;
    let client = recording_client(103289, fixture_arguments(&["owner"])?, fixture_stack(values)?)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_wallet_address(TonAddress::ZERO))?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV1_106049() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV1", 106049)?;
    let client = recording_client(106049, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_decimals())?;
    assert_eq!(
        value.decimals_x,
        <u8>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(
        value.decimals_y,
        <u8>::try_from(integer(&values[1])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(106049, fixture_arguments(&[])?, extra)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_decimals()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(106049, fixture_arguments(&[])?, missing)?;
    let contract = DedustPoolV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_decimals()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV2_82320() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV2", 82320)?;
    let client = recording_client(82320, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_version())?;
    assert_eq!(
        value,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV2_103723() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV2", 103723)?;
    let client = recording_client(103723, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.is_stable())?;
    assert_eq!(value, integer(&values[0])? != I512::ZERO);
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV2_96780() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV2", 96780)?;
    let client = recording_client(96780, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_trade_fee())?;
    assert_eq!(
        value.trade_fee_numerator,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(
        value.trade_fee_denominator,
        <u16>::try_from(integer(&values[1])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(96780, fixture_arguments(&[])?, extra)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_trade_fee()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(96780, fixture_arguments(&[])?, missing)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_trade_fee()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV2_65971() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV2", 65971)?;
    let client = recording_client(65971, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_reserves())?;
    assert_eq!(value.reserve0, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.reserve1, Coins::try_from(integer(&values[1])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(65971, fixture_arguments(&[])?, extra)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_reserves()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(65971, fixture_arguments(&[])?, missing)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_reserves()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV2_112792() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV2", 112792)?;
    let client = recording_client(112792, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_protocol_fees())?;
    assert_eq!(value.asset0_protocol_fee, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.asset1_protocol_fee, Coins::try_from(integer(&values[1])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(112792, fixture_arguments(&[])?, extra)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_protocol_fees()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(112792, fixture_arguments(&[])?, missing)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_protocol_fees()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV2_118188() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV2", 118188)?;
    let client = recording_client(118188, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_assets())?;
    assert_eq!(value.asset0, DedustClassicAsset::from_cell(&cell(&values[0])?)?);
    assert_eq!(value.asset1, DedustClassicAsset::from_cell(&cell(&values[1])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(118188, fixture_arguments(&[])?, extra)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_assets()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(118188, fixture_arguments(&[])?, missing)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_assets()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV2_70754() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV2", 70754)?;
    let client = recording_client(70754, fixture_arguments(&["native", "1000000"])?, fixture_stack(values)?)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_swap_out(
        DedustClassicAsset::from_cell(&fixture_arguments(&["native"])?.pop_cell()?)?,
        1000000.into(),
    ))?;
    assert_eq!(value.asset_out, DedustClassicAsset::from_cell(&cell(&values[0])?)?);
    assert_eq!(value.amount_out, integer(&values[1])?);
    assert_eq!(value.trade_fee, integer(&values[2])?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(70754, fixture_arguments(&["native", "1000000"])?, extra)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(
        tokio_test::block_on(contract.estimate_swap_out(
            DedustClassicAsset::from_cell(&fixture_arguments(&["native"])?.pop_cell()?)?,
            1000000.into()
        ))
        .is_err()
    );
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(70754, fixture_arguments(&["native", "1000000"])?, missing)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(
        tokio_test::block_on(contract.estimate_swap_out(
            DedustClassicAsset::from_cell(&fixture_arguments(&["native"])?.pop_cell()?)?,
            1000000.into()
        ))
        .is_err()
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV2_119877() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV2", 119877)?;
    let client = recording_client(119877, fixture_arguments(&["1000000", "1000000"])?, fixture_stack(values)?)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_deposit_out(1000000.into(), 1000000.into()))?;
    assert_eq!(value.asset0_deposit, integer(&values[0])?);
    assert_eq!(value.asset1_deposit, integer(&values[1])?);
    assert_eq!(value.fair_supply, integer(&values[2])?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(119877, fixture_arguments(&["1000000", "1000000"])?, extra)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.estimate_deposit_out(1000000.into(), 1000000.into())).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(119877, fixture_arguments(&["1000000", "1000000"])?, missing)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.estimate_deposit_out(1000000.into(), 1000000.into())).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV2_106029() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV2", 106029)?;
    let client = recording_client(106029, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_jetton_data())?;
    assert_eq!(value.total_supply, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.mintable, integer(&values[1])? != I512::ZERO);
    assert_eq!(value.admin_address, NullableAddress.read(&mut cell(&values[2])?.parser())?);
    assert_eq!(value.jetton_content, cell(&values[3])?);
    assert_eq!(value.jetton_wallet_code, cell(&values[4])?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(106029, fixture_arguments(&[])?, extra)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_jetton_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(106029, fixture_arguments(&[])?, missing)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_jetton_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV2_103289() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV2", 103289)?;
    let client = recording_client(103289, fixture_arguments(&["owner"])?, fixture_stack(values)?)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_wallet_address(TonAddress::ZERO))?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV2_66722() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV2", 66722)?;
    let client = recording_client(66722, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_quote_providers())?;
    assert_eq!(value.provider0, NullableAddress.read(&mut cell(&values[0])?.parser())?);
    assert_eq!(value.provider1, NullableAddress.read(&mut cell(&values[1])?.parser())?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(66722, fixture_arguments(&[])?, extra)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_quote_providers()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(66722, fixture_arguments(&[])?, missing)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_quote_providers()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV2_106049() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV2", 106049)?;
    let client = recording_client(106049, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_decimals())?;
    assert_eq!(
        value.decimals_x,
        <u8>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(
        value.decimals_y,
        <u8>::try_from(integer(&values[1])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(106049, fixture_arguments(&[])?, extra)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_decimals()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(106049, fixture_arguments(&[])?, missing)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_decimals()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustPoolV2_111021() -> anyhow::Result<()> {
    let values = fixture_values("DedustPoolV2", 111021)?;
    let client = recording_client(111021, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_scaling_factors())?;
    assert_eq!(
        value.factor_x,
        <u128>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(
        value.factor_y,
        <u128>::try_from(integer(&values[1])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(111021, fixture_arguments(&[])?, extra)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_scaling_factors()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(111021, fixture_arguments(&[])?, missing)?;
    let contract = DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_scaling_factors()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultNativeV1_82320() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultNativeV1", 82320)?;
    let client = recording_client(82320, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultNativeV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_version())?;
    assert_eq!(
        value,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultNativeV1_89995() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultNativeV1", 89995)?;
    let client = recording_client(89995, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultNativeV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_factory_addr())?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultNativeV1_89352() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultNativeV1", 89352)?;
    let client = recording_client(89352, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultNativeV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_asset())?;
    assert_eq!(value, DedustClassicAsset::from_cell(&cell(&values[0])?)?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultNativeV1_109505() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultNativeV1", 109505)?;
    let client = recording_client(109505, fixture_arguments(&["17"])?, fixture_stack(values)?)?;
    let contract = DedustVaultNativeV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_gas_one_arg(17.into()))?;
    assert_eq!(value, integer(&values[0])?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultNativeV1_117300() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultNativeV1", 117300)?;
    let client = recording_client(117300, fixture_arguments(&["17", "17"])?, fixture_stack(values)?)?;
    let contract = DedustVaultNativeV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_gas_two_args(17.into(), 17.into()))?;
    assert_eq!(value, integer(&values[0])?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultNativeV2_82320() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultNativeV2", 82320)?;
    let client = recording_client(82320, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultNativeV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_version())?;
    assert_eq!(
        value,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultNativeV2_89995() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultNativeV2", 89995)?;
    let client = recording_client(89995, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultNativeV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_factory_addr())?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultNativeV2_89352() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultNativeV2", 89352)?;
    let client = recording_client(89352, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultNativeV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_asset())?;
    assert_eq!(value, DedustClassicAsset::from_cell(&cell(&values[0])?)?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultNativeV2_109505() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultNativeV2", 109505)?;
    let client = recording_client(109505, fixture_arguments(&["17"])?, fixture_stack(values)?)?;
    let contract = DedustVaultNativeV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_gas_one_arg(17.into()))?;
    assert_eq!(value, integer(&values[0])?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultNativeV2_117300() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultNativeV2", 117300)?;
    let client = recording_client(117300, fixture_arguments(&["17", "17"])?, fixture_stack(values)?)?;
    let contract = DedustVaultNativeV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_gas_two_args(17.into(), 17.into()))?;
    assert_eq!(value, integer(&values[0])?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultNativeV2_109867() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultNativeV2", 109867)?;
    let client = recording_client(109867, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultNativeV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_tracked_native_balance())?;
    assert_eq!(value, Coins::try_from(integer(&values[0])?)?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultJettonV1_82320() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultJettonV1", 82320)?;
    let client = recording_client(82320, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultJettonV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_version())?;
    assert_eq!(
        value,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultJettonV1_66908() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultJettonV1", 66908)?;
    let client = recording_client(66908, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultJettonV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.is_ready())?;
    assert_eq!(value, integer(&values[0])? != I512::ZERO);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultJettonV1_89995() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultJettonV1", 89995)?;
    let client = recording_client(89995, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultJettonV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_factory_addr())?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultJettonV1_69014() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultJettonV1", 69014)?;
    let client = recording_client(69014, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultJettonV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_wallet_addr())?;
    assert_eq!(value, NullableAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultJettonV1_89352() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultJettonV1", 89352)?;
    let client = recording_client(89352, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultJettonV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_asset())?;
    assert_eq!(value, DedustClassicAsset::from_cell(&cell(&values[0])?)?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultJettonV1_109505() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultJettonV1", 109505)?;
    let client = recording_client(109505, fixture_arguments(&["17"])?, fixture_stack(values)?)?;
    let contract = DedustVaultJettonV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_gas_one_arg(17.into()))?;
    assert_eq!(value, integer(&values[0])?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultJettonV1_117300() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultJettonV1", 117300)?;
    let client = recording_client(117300, fixture_arguments(&["17", "17"])?, fixture_stack(values)?)?;
    let contract = DedustVaultJettonV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_gas_two_args(17.into(), 17.into()))?;
    assert_eq!(value, integer(&values[0])?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultJettonV2_82320() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultJettonV2", 82320)?;
    let client = recording_client(82320, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultJettonV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_version())?;
    assert_eq!(
        value,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultJettonV2_66908() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultJettonV2", 66908)?;
    let client = recording_client(66908, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultJettonV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.is_ready())?;
    assert_eq!(value, integer(&values[0])? != I512::ZERO);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultJettonV2_89995() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultJettonV2", 89995)?;
    let client = recording_client(89995, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultJettonV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_factory_addr())?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultJettonV2_69014() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultJettonV2", 69014)?;
    let client = recording_client(69014, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultJettonV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_wallet_addr())?;
    assert_eq!(value, NullableAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultJettonV2_89352() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultJettonV2", 89352)?;
    let client = recording_client(89352, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustVaultJettonV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_asset())?;
    assert_eq!(value, DedustClassicAsset::from_cell(&cell(&values[0])?)?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultJettonV2_109505() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultJettonV2", 109505)?;
    let client = recording_client(109505, fixture_arguments(&["17"])?, fixture_stack(values)?)?;
    let contract = DedustVaultJettonV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_gas_one_arg(17.into()))?;
    assert_eq!(value, integer(&values[0])?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustVaultJettonV2_117300() -> anyhow::Result<()> {
    let values = fixture_values("DedustVaultJettonV2", 117300)?;
    let client = recording_client(117300, fixture_arguments(&["17", "17"])?, fixture_stack(values)?)?;
    let contract = DedustVaultJettonV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_gas_two_args(17.into(), 17.into()))?;
    assert_eq!(value, integer(&values[0])?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV1_89995() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV1", 89995)?;
    let client = recording_client(89995, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_factory_addr())?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV1_95132() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV1", 95132)?;
    let client = recording_client(95132, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_owner_addr())?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV1_84365() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV1", 84365)?;
    let client = recording_client(84365, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_pool_addr())?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV1_106125() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV1", 106125)?;
    let client = recording_client(106125, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_pool_params())?;
    assert_eq!(value.is_stable, integer(&values[0])? != I512::ZERO);
    assert_eq!(value.asset0, DedustClassicAsset::from_cell(&cell(&values[1])?)?);
    assert_eq!(value.asset1, DedustClassicAsset::from_cell(&cell(&values[2])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(106125, fixture_arguments(&[])?, extra)?;
    let contract = DedustLiquidityDepositV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_pool_params()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(106125, fixture_arguments(&[])?, missing)?;
    let contract = DedustLiquidityDepositV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_pool_params()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV1_84232() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV1", 84232)?;
    let client = recording_client(84232, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_target_balances())?;
    assert_eq!(value.asset0_target_balance, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.asset1_target_balance, Coins::try_from(integer(&values[1])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(84232, fixture_arguments(&[])?, extra)?;
    let contract = DedustLiquidityDepositV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_target_balances()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(84232, fixture_arguments(&[])?, missing)?;
    let contract = DedustLiquidityDepositV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_target_balances()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV1_87878() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV1", 87878)?;
    let client = recording_client(87878, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_balances())?;
    assert_eq!(value.asset0_balance, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.asset1_balance, Coins::try_from(integer(&values[1])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(87878, fixture_arguments(&[])?, extra)?;
    let contract = DedustLiquidityDepositV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_balances()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(87878, fixture_arguments(&[])?, missing)?;
    let contract = DedustLiquidityDepositV1Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_balances()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV1_108752() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV1", 108752)?;
    let client = recording_client(108752, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.is_processing())?;
    assert_eq!(value, integer(&values[0])? != I512::ZERO);
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV1_85296() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV1", 85296)?;
    let client = recording_client(85296, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV1Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_min_lp_amount())?;
    assert_eq!(value, Coins::try_from(integer(&values[0])?)?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV2_89995() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV2", 89995)?;
    let client = recording_client(89995, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_factory_addr())?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV2_95132() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV2", 95132)?;
    let client = recording_client(95132, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_owner_addr())?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV2_84365() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV2", 84365)?;
    let client = recording_client(84365, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_pool_addr())?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV2_106125() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV2", 106125)?;
    let client = recording_client(106125, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_pool_params())?;
    assert_eq!(value.is_stable, integer(&values[0])? != I512::ZERO);
    assert_eq!(value.asset0, DedustClassicAsset::from_cell(&cell(&values[1])?)?);
    assert_eq!(value.asset1, DedustClassicAsset::from_cell(&cell(&values[2])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(106125, fixture_arguments(&[])?, extra)?;
    let contract = DedustLiquidityDepositV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_pool_params()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(106125, fixture_arguments(&[])?, missing)?;
    let contract = DedustLiquidityDepositV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_pool_params()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV2_84232() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV2", 84232)?;
    let client = recording_client(84232, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_target_balances())?;
    assert_eq!(value.asset0_target_balance, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.asset1_target_balance, Coins::try_from(integer(&values[1])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(84232, fixture_arguments(&[])?, extra)?;
    let contract = DedustLiquidityDepositV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_target_balances()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(84232, fixture_arguments(&[])?, missing)?;
    let contract = DedustLiquidityDepositV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_target_balances()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV2_87878() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV2", 87878)?;
    let client = recording_client(87878, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_balances())?;
    assert_eq!(value.asset0_balance, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.asset1_balance, Coins::try_from(integer(&values[1])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(87878, fixture_arguments(&[])?, extra)?;
    let contract = DedustLiquidityDepositV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_balances()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(87878, fixture_arguments(&[])?, missing)?;
    let contract = DedustLiquidityDepositV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_balances()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV2_108752() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV2", 108752)?;
    let client = recording_client(108752, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.is_processing())?;
    assert_eq!(value, integer(&values[0])? != I512::ZERO);
    Ok(())
}
#[test]
fn test_raw_getter_DedustLiquidityDepositV2_85296() -> anyhow::Result<()> {
    let values = fixture_values("DedustLiquidityDepositV2", 85296)?;
    let client = recording_client(85296, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustLiquidityDepositV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_min_lp_amount())?;
    assert_eq!(value, Coins::try_from(integer(&values[0])?)?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustV2Cpmm_72157() -> anyhow::Result<()> {
    let values = fixture_values("DedustV2Cpmm", 72157)?;
    let client = recording_client(72157, fixture_arguments(&["owner"])?, fixture_stack(values)?)?;
    let contract = DedustV2CpmmContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_position_address(TonAddress::ZERO))?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustV2Cpmm_81689() -> anyhow::Result<()> {
    let values = fixture_values("DedustV2Cpmm", 81689)?;
    let client = recording_client(81689, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustV2CpmmContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_pool_data())?;
    assert_eq!(
        value.status,
        <i8>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(value.deposit_active, integer(&values[1])? != I512::ZERO);
    assert_eq!(value.swap_active, integer(&values[2])? != I512::ZERO);
    assert_eq!(value.asset_x, NullableAddress.read(&mut cell(&values[3])?.parser())?);
    assert_eq!(value.asset_y, NullableAddress.read(&mut cell(&values[4])?.parser())?);
    assert!(value.wallets_by_assets.is_empty());
    assert!(value.assets_by_wallets.is_empty());
    assert!(value.wallets_resolutions.is_empty());
    assert_eq!(
        value.base_fee_bps,
        <i32>::try_from(integer(&values[8])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(value.reserve_x, Coins::try_from(integer(&values[9])?)?);
    assert_eq!(value.reserve_y, Coins::try_from(integer(&values[10])?)?);
    assert_eq!(value.liquidity, Coins::try_from(integer(&values[11])?)?);
    assert_eq!(value.protocol_fee_x, Coins::try_from(integer(&values[12])?)?);
    assert_eq!(value.protocol_fee_y, Coins::try_from(integer(&values[13])?)?);
    assert_eq!(value.creator_fee_x, Coins::try_from(integer(&values[14])?)?);
    assert_eq!(value.creator_fee_y, Coins::try_from(integer(&values[15])?)?);
    assert_eq!(value.x_fee_per_liquidity, U256::from_str(&integer(&values[16])?.to_string())?);
    assert_eq!(value.y_fee_per_liquidity, U256::from_str(&integer(&values[17])?.to_string())?);
    assert!(value.rewards.is_empty());
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(81689, fixture_arguments(&[])?, extra)?;
    let contract = DedustV2CpmmContract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_pool_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(81689, fixture_arguments(&[])?, missing)?;
    let contract = DedustV2CpmmContract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_pool_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustV2Cpmm_112421() -> anyhow::Result<()> {
    let values = fixture_values("DedustV2Cpmm", 112421)?;
    let client = recording_client(112421, fixture_arguments(&["1000000"])?, fixture_stack(values)?)?;
    let contract = DedustV2CpmmContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_liquidity_amounts(1000000.into()))?;
    assert_eq!(value.amount_x, integer(&values[0])?);
    assert_eq!(value.amount_y, integer(&values[1])?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(112421, fixture_arguments(&["1000000"])?, extra)?;
    let contract = DedustV2CpmmContract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_liquidity_amounts(1000000.into())).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(112421, fixture_arguments(&["1000000"])?, missing)?;
    let contract = DedustV2CpmmContract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_liquidity_amounts(1000000.into())).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustUranusMemeV2_101289() -> anyhow::Result<()> {
    let values = fixture_values("DedustUranusMemeV2", 101289)?;
    let client = recording_client(101289, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustUranusMemeV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_meme_data())?;
    assert_eq!(value.initialized, integer(&values[0])? != I512::ZERO);
    assert_eq!(value.migrated, integer(&values[1])? != I512::ZERO);
    assert_eq!(value.controller_address, InternalAddress.read(&mut cell(&values[2])?.parser())?);
    assert_eq!(value.creator_address, InternalAddress.read(&mut cell(&values[3])?.parser())?);
    assert_eq!(value.creator_fee, Coins::try_from(integer(&values[4])?)?);
    assert_eq!(
        value.seed,
        <u128>::try_from(integer(&values[5])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(value.is_graduated, integer(&values[6])? != I512::ZERO);
    assert_eq!(value.alpha, Coins::try_from(integer(&values[7])?)?);
    assert_eq!(value.beta, Coins::try_from(integer(&values[8])?)?);
    assert_eq!(value.on_sell_supply, Coins::try_from(integer(&values[9])?)?);
    assert_eq!(
        value.trade_fee_bps,
        <u16>::try_from(integer(&values[10])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(value.raised_funds, Coins::try_from(integer(&values[11])?)?);
    assert_eq!(value.current_supply, Coins::try_from(integer(&values[12])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(101289, fixture_arguments(&[])?, extra)?;
    let contract = DedustUranusMemeV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_meme_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(101289, fixture_arguments(&[])?, missing)?;
    let contract = DedustUranusMemeV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_meme_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustUranusMemeV2_103289() -> anyhow::Result<()> {
    let values = fixture_values("DedustUranusMemeV2", 103289)?;
    let client = recording_client(103289, fixture_arguments(&["owner"])?, fixture_stack(values)?)?;
    let contract = DedustUranusMemeV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_wallet_address(TonAddress::ZERO))?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustUranusMemeV2_106029() -> anyhow::Result<()> {
    let values = fixture_values("DedustUranusMemeV2", 106029)?;
    let client = recording_client(106029, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustUranusMemeV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_jetton_data())?;
    assert_eq!(value.total_supply, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.mintable, integer(&values[1])? != I512::ZERO);
    assert_eq!(value.admin_address, NullableAddress.read(&mut cell(&values[2])?.parser())?);
    assert_eq!(value.jetton_content, cell(&values[3])?);
    assert_eq!(value.jetton_wallet_code, cell(&values[4])?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(106029, fixture_arguments(&[])?, extra)?;
    let contract = DedustUranusMemeV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_jetton_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(106029, fixture_arguments(&[])?, missing)?;
    let contract = DedustUranusMemeV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_jetton_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustUranusMemeV3_101289() -> anyhow::Result<()> {
    let values = fixture_values("DedustUranusMemeV3", 101289)?;
    let client = recording_client(101289, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustUranusMemeV3Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_meme_data())?;
    assert_eq!(value.initialized, integer(&values[0])? != I512::ZERO);
    assert_eq!(value.migrated, integer(&values[1])? != I512::ZERO);
    assert_eq!(value.controller_address, InternalAddress.read(&mut cell(&values[2])?.parser())?);
    assert_eq!(value.creator_address, InternalAddress.read(&mut cell(&values[3])?.parser())?);
    assert_eq!(value.creator_fee, Coins::try_from(integer(&values[4])?)?);
    assert_eq!(
        value.seed,
        <u128>::try_from(integer(&values[5])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(value.is_graduated, integer(&values[6])? != I512::ZERO);
    assert_eq!(value.alpha, Coins::try_from(integer(&values[7])?)?);
    assert_eq!(value.beta, Coins::try_from(integer(&values[8])?)?);
    assert_eq!(value.on_sell_supply, Coins::try_from(integer(&values[9])?)?);
    assert_eq!(
        value.trade_fee_bps,
        <u16>::try_from(integer(&values[10])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(value.raised_funds, Coins::try_from(integer(&values[11])?)?);
    assert_eq!(value.current_supply, Coins::try_from(integer(&values[12])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(101289, fixture_arguments(&[])?, extra)?;
    let contract = DedustUranusMemeV3Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_meme_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(101289, fixture_arguments(&[])?, missing)?;
    let contract = DedustUranusMemeV3Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_meme_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustUranusMemeV3_121862() -> anyhow::Result<()> {
    let values = fixture_values("DedustUranusMemeV3", 121862)?;
    let client = recording_client(121862, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustUranusMemeV3Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_bonding_curve_data())?;
    assert_eq!(value.is_graduated, integer(&values[0])? != I512::ZERO);
    assert_eq!(value.max_supply, Coins::try_from(integer(&values[1])?)?);
    assert_eq!(value.current_supply, Coins::try_from(integer(&values[2])?)?);
    assert_eq!(value.bonding_curve_supply, Coins::try_from(integer(&values[3])?)?);
    assert_eq!(value.liquidity_supply, Coins::try_from(integer(&values[4])?)?);
    assert_eq!(value.raising_funds, Coins::try_from(integer(&values[5])?)?);
    assert_eq!(value.migration_fee, Coins::try_from(integer(&values[6])?)?);
    assert_eq!(value.raised_funds, Coins::try_from(integer(&values[7])?)?);
    assert_eq!(value.alpha, Coins::try_from(integer(&values[8])?)?);
    assert_eq!(value.beta, Coins::try_from(integer(&values[9])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(121862, fixture_arguments(&[])?, extra)?;
    let contract = DedustUranusMemeV3Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_bonding_curve_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(121862, fixture_arguments(&[])?, missing)?;
    let contract = DedustUranusMemeV3Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_bonding_curve_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustUranusMemeV3_83180() -> anyhow::Result<()> {
    let values = fixture_values("DedustUranusMemeV3", 83180)?;
    let client = recording_client(83180, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustUranusMemeV3Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_partner_data())?;
    assert_eq!(value.partner_fee, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(
        value.partner_fee_bps,
        <u16>::try_from(integer(&values[1])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(value.partner_address, InternalAddress.read(&mut cell(&values[2])?.parser())?);
    assert_eq!(
        value.pool_partner_fee_bps,
        <u16>::try_from(integer(&values[3])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(83180, fixture_arguments(&[])?, extra)?;
    let contract = DedustUranusMemeV3Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_partner_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(83180, fixture_arguments(&[])?, missing)?;
    let contract = DedustUranusMemeV3Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_partner_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustUranusMemeV3_103289() -> anyhow::Result<()> {
    let values = fixture_values("DedustUranusMemeV3", 103289)?;
    let client = recording_client(103289, fixture_arguments(&["owner"])?, fixture_stack(values)?)?;
    let contract = DedustUranusMemeV3Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_wallet_address(TonAddress::ZERO))?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustUranusMemeV3_106029() -> anyhow::Result<()> {
    let values = fixture_values("DedustUranusMemeV3", 106029)?;
    let client = recording_client(106029, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustUranusMemeV3Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_jetton_data())?;
    assert_eq!(value.total_supply, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.mintable, integer(&values[1])? != I512::ZERO);
    assert_eq!(value.admin_address, NullableAddress.read(&mut cell(&values[2])?.parser())?);
    assert_eq!(value.jetton_content, cell(&values[3])?);
    assert_eq!(value.jetton_wallet_code, cell(&values[4])?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(106029, fixture_arguments(&[])?, extra)?;
    let contract = DedustUranusMemeV3Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_jetton_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(106029, fixture_arguments(&[])?, missing)?;
    let contract = DedustUranusMemeV3Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_jetton_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustUranusMemeWalletV2_97026() -> anyhow::Result<()> {
    let values = fixture_values("DedustUranusMemeWalletV2", 97026)?;
    let client = recording_client(97026, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustUranusMemeWalletV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_wallet_data())?;
    assert_eq!(value.balance, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.owner_address, NullableAddress.read(&mut cell(&values[1])?.parser())?);
    assert_eq!(value.meme_address, NullableAddress.read(&mut cell(&values[2])?.parser())?);
    assert_eq!(value.wallet_code, cell(&values[3])?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(97026, fixture_arguments(&[])?, extra)?;
    let contract = DedustUranusMemeWalletV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_wallet_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(97026, fixture_arguments(&[])?, missing)?;
    let contract = DedustUranusMemeWalletV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_wallet_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustUranusMemeWalletV3_97026() -> anyhow::Result<()> {
    let values = fixture_values("DedustUranusMemeWalletV3", 97026)?;
    let client = recording_client(97026, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustUranusMemeWalletV3Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_wallet_data())?;
    assert_eq!(value.balance, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.owner_address, InternalAddress.read(&mut cell(&values[1])?.parser())?);
    assert_eq!(value.meme_address, InternalAddress.read(&mut cell(&values[2])?.parser())?);
    assert_eq!(value.wallet_code, cell(&values[3])?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(97026, fixture_arguments(&[])?, extra)?;
    let contract = DedustUranusMemeWalletV3Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_wallet_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(97026, fixture_arguments(&[])?, missing)?;
    let contract = DedustUranusMemeWalletV3Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_wallet_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustOperatorV2_79987() -> anyhow::Result<()> {
    let values = fixture_values("DedustOperatorV2", 79987)?;
    let client = recording_client(79987, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustOperatorV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_operator_data())?;
    assert_eq!(value.factory, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    assert_eq!(
        value.role,
        <u8>::try_from(integer(&values[1])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(value.user, InternalAddress.read(&mut cell(&values[2])?.parser())?);
    assert_eq!(
        value.can_execute_after,
        <u32>::try_from(integer(&values[3])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(79987, fixture_arguments(&[])?, extra)?;
    let contract = DedustOperatorV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_operator_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(79987, fixture_arguments(&[])?, missing)?;
    let contract = DedustOperatorV2Contract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_operator_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_DedustOperatorV2_82320() -> anyhow::Result<()> {
    let values = fixture_values("DedustOperatorV2", 82320)?;
    let client = recording_client(82320, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustOperatorV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_version())?;
    assert_eq!(
        value,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustX1000WalletV2_85143() -> anyhow::Result<()> {
    let values = fixture_values("DedustX1000WalletV2", 85143)?;
    let client = recording_client(85143, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustX1000WalletV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.seqno())?;
    assert_eq!(
        value,
        <u32>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustX1000WalletV2_83648() -> anyhow::Result<()> {
    let values = fixture_values("DedustX1000WalletV2", 83648)?;
    let client = recording_client(83648, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustX1000WalletV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_balance())?;
    assert_eq!(value, Coins::try_from(integer(&values[0])?)?);
    Ok(())
}
#[test]
fn test_raw_getter_DedustX1000WalletV2_85802() -> anyhow::Result<()> {
    let values = fixture_values("DedustX1000WalletV2", 85802)?;
    let client = recording_client(85802, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustX1000WalletV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.is_pending_operations_empty())?;
    assert_eq!(value, integer(&values[0])? != I512::ZERO);
    Ok(())
}
#[test]
fn test_raw_getter_DedustX1000WalletV2_89828() -> anyhow::Result<()> {
    let values = fixture_values("DedustX1000WalletV2", 89828)?;
    let client = recording_client(89828, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustX1000WalletV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_implementation_code_hash())?;
    assert_eq!(value, U256::from_str(&integer(&values[0])?.to_string())?);
    {
        let bad = -I512::ONE;
        let mut response = fixture_stack(values)?;
        response[0] = TVMStackValue::Int(TVMInt { value: bad });
        let client = recording_client(89828, fixture_arguments(&[])?, response)?;
        let contract = DedustX1000WalletV2Contract::new(&client, &TonAddress::ZERO, None);
        assert!(
            tokio_test::block_on(contract.get_implementation_code_hash()).is_err(),
            "DedustX1000WalletV2/89828/value"
        );
    }
    Ok(())
}
#[test]
fn test_raw_getter_DedustX1000WalletV2_91459() -> anyhow::Result<()> {
    let values = fixture_values("DedustX1000WalletV2", 91459)?;
    let client = recording_client(91459, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustX1000WalletV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_stored_seqno())?;
    assert_eq!(
        value,
        <u32>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustX1000WalletV2_94056() -> anyhow::Result<()> {
    let values = fixture_values("DedustX1000WalletV2", 94056)?;
    let client = recording_client(94056, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustX1000WalletV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.is_execution_entries_empty())?;
    assert_eq!(value, integer(&values[0])? != I512::ZERO);
    Ok(())
}
#[test]
fn test_raw_getter_DedustX1000WalletV2_95507() -> anyhow::Result<()> {
    let values = fixture_values("DedustX1000WalletV2", 95507)?;
    let client = recording_client(95507, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustX1000WalletV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.storage_version())?;
    assert_eq!(
        value,
        <u8>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustX1000WalletV2_97027() -> anyhow::Result<()> {
    let values = fixture_values("DedustX1000WalletV2", 97027)?;
    let client = recording_client(97027, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustX1000WalletV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_counter())?;
    assert_eq!(
        value,
        <u32>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustX1000WalletV2_97376() -> anyhow::Result<()> {
    let values = fixture_values("DedustX1000WalletV2", 97376)?;
    let client = recording_client(97376, fixture_arguments(&["17"])?, fixture_stack(values)?)?;
    let contract = DedustX1000WalletV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_pending_operation_presence(17.into()))?;
    assert_eq!(
        value,
        <u8>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_DedustX1000WalletV2_98121() -> anyhow::Result<()> {
    let values = fixture_values("DedustX1000WalletV2", 98121)?;
    let client = recording_client(98121, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustX1000WalletV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.is_reserved_values_empty())?;
    assert_eq!(value, integer(&values[0])? != I512::ZERO);
    Ok(())
}
#[test]
fn test_raw_getter_DedustX1000WalletV2_118999() -> anyhow::Result<()> {
    let values = fixture_values("DedustX1000WalletV2", 118999)?;
    let client = recording_client(118999, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustX1000WalletV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.has_no_pending_operations())?;
    assert_eq!(value, integer(&values[0])? != I512::ZERO);
    Ok(())
}
#[test]
fn test_raw_getter_Position_89720() -> anyhow::Result<()> {
    let values = fixture_values("Position", 89720)?;
    let client = recording_client(89720, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustV2CpmmPositionContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_position_data())?;
    assert_eq!(value.pool, NullableAddress.read(&mut cell(&values[0])?.parser())?);
    assert_eq!(value.owner, NullableAddress.read(&mut cell(&values[1])?.parser())?);
    assert_eq!(value.total_liquidity, Coins::try_from(integer(&values[2])?)?);
    assert_eq!(value.locked_liquidity, Coins::try_from(integer(&values[3])?)?);
    assert!(value.rewards.is_empty());
    assert_eq!(value.x_pending, Coins::try_from(integer(&values[5])?)?);
    assert_eq!(value.y_pending, Coins::try_from(integer(&values[6])?)?);
    assert_eq!(value.x_checkpoint, U256::from_str(&integer(&values[7])?.to_string())?);
    assert_eq!(value.y_checkpoint, U256::from_str(&integer(&values[8])?.to_string())?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(89720, fixture_arguments(&[])?, extra)?;
    let contract = DedustV2CpmmPositionContract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_position_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(89720, fixture_arguments(&[])?, missing)?;
    let contract = DedustV2CpmmPositionContract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_position_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_Position_80110() -> anyhow::Result<()> {
    let values = fixture_values("Position", 80110)?;
    let client = recording_client(80110, fixture_arguments(&["17", "17"])?, fixture_stack(values)?)?;
    let contract = DedustV2CpmmPositionContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_available_fees(17.into(), 17.into()))?;
    assert_eq!(value.x_fee, integer(&values[0])?);
    assert_eq!(value.y_fee, integer(&values[1])?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(80110, fixture_arguments(&["17", "17"])?, extra)?;
    let contract = DedustV2CpmmPositionContract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_available_fees(17.into(), 17.into())).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(80110, fixture_arguments(&["17", "17"])?, missing)?;
    let contract = DedustV2CpmmPositionContract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_available_fees(17.into(), 17.into())).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_Deposit_70330() -> anyhow::Result<()> {
    let values = fixture_values("Deposit", 70330)?;
    let client = recording_client(70330, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustV2CpmmDepositContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_deposit_data())?;
    assert_eq!(value.pool, NullableAddress.read(&mut cell(&values[0])?.parser())?);
    assert_eq!(
        value.fulfill_destination,
        NullableAddress.read(&mut cell(&values[1])?.parser())?
    );
    assert_eq!(value.reject_destination, NullableAddress.read(&mut cell(&values[2])?.parser())?);
    assert_eq!(value.target_x, Coins::try_from(integer(&values[3])?)?);
    assert_eq!(value.target_y, Coins::try_from(integer(&values[4])?)?);
    assert_eq!(
        value.locked_liquidity_share,
        <u16>::try_from(integer(&values[5])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(value.minimal_liquidity, Coins::try_from(integer(&values[6])?)?);
    assert_eq!(value.amount_x, Coins::try_from(integer(&values[7])?)?);
    assert_eq!(value.amount_y, Coins::try_from(integer(&values[8])?)?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(70330, fixture_arguments(&[])?, extra)?;
    let contract = DedustV2CpmmDepositContract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_deposit_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(70330, fixture_arguments(&[])?, missing)?;
    let contract = DedustV2CpmmDepositContract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_deposit_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_ClassicLPWallet_97026() -> anyhow::Result<()> {
    let values = fixture_values("ClassicLPWallet", 97026)?;
    let client = recording_client(97026, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicLpWalletContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_wallet_data())?;
    assert_eq!(value.balance, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.owner, NullableAddress.read(&mut cell(&values[1])?.parser())?);
    assert_eq!(value.pool, NullableAddress.read(&mut cell(&values[2])?.parser())?);
    assert_eq!(value.wallet_code, cell(&values[3])?);
    let mut extra = fixture_stack(values)?;
    extra.insert(0, TVMStackValue::Null(TVMNull));
    let client = recording_client(97026, fixture_arguments(&[])?, extra)?;
    let contract = DedustClassicLpWalletContract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_wallet_data()).is_err());
    let mut missing = fixture_stack(values)?;
    missing.pop();
    let client = recording_client(97026, fixture_arguments(&[])?, missing)?;
    let contract = DedustClassicLpWalletContract::new(&client, &TonAddress::ZERO, None);
    assert!(tokio_test::block_on(contract.get_wallet_data()).is_err());
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPool778f_82320() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPool778f", 82320)?;
    let client = recording_client(82320, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_version())?;
    assert_eq!(
        value,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPool778f_103723() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPool778f", 103723)?;
    let client = recording_client(103723, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.is_stable())?;
    assert_eq!(value, integer(&values[0])? != I512::ZERO);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPool778f_96780() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPool778f", 96780)?;
    let client = recording_client(96780, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_trade_fee())?;
    assert_eq!(
        value.trade_fee_numerator,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(
        value.trade_fee_denominator,
        <u16>::try_from(integer(&values[1])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPool778f_65971() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPool778f", 65971)?;
    let client = recording_client(65971, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_reserves())?;
    assert_eq!(value.reserve0, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.reserve1, Coins::try_from(integer(&values[1])?)?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPool778f_112792() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPool778f", 112792)?;
    let client = recording_client(112792, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_protocol_fees())?;
    assert_eq!(value.asset0_protocol_fee, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.asset1_protocol_fee, Coins::try_from(integer(&values[1])?)?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPool778f_118188() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPool778f", 118188)?;
    let client = recording_client(118188, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_assets())?;
    assert_eq!(value.asset0, DedustClassicAsset::from_cell(&cell(&values[0])?)?);
    assert_eq!(value.asset1, DedustClassicAsset::from_cell(&cell(&values[1])?)?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPool778f_70754() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPool778f", 70754)?;
    let client = recording_client(70754, fixture_arguments(&["native", "1000000"])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_swap_out(
        DedustClassicAsset::from_cell(&fixture_arguments(&["native"])?.pop_cell()?)?,
        1000000.into(),
    ))?;
    assert_eq!(value.asset_out, DedustClassicAsset::from_cell(&cell(&values[0])?)?);
    assert_eq!(value.amount_out, integer(&values[1])?);
    assert_eq!(value.trade_fee, integer(&values[2])?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPool778f_119877() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPool778f", 119877)?;
    let client = recording_client(119877, fixture_arguments(&["1000000", "1000000"])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_deposit_out(1000000.into(), 1000000.into()))?;
    assert_eq!(value.asset0_deposit, integer(&values[0])?);
    assert_eq!(value.asset1_deposit, integer(&values[1])?);
    assert_eq!(value.fair_supply, integer(&values[2])?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPool778f_106029() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPool778f", 106029)?;
    let client = recording_client(106029, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_jetton_data())?;
    assert_eq!(value.total_supply, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.mintable, integer(&values[1])? != I512::ZERO);
    assert_eq!(value.admin_address, NullableAddress.read(&mut cell(&values[2])?.parser())?);
    assert_eq!(value.jetton_content, cell(&values[3])?);
    assert_eq!(value.jetton_wallet_code, cell(&values[4])?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPool778f_103289() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPool778f", 103289)?;
    let client = recording_client(103289, fixture_arguments(&["owner"])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_wallet_address(TonAddress::ZERO))?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPool778f_66722() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPool778f", 66722)?;
    let client = recording_client(66722, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_quote_providers())?;
    assert_eq!(value.provider0, NullableAddress.read(&mut cell(&values[0])?.parser())?);
    assert_eq!(value.provider1, NullableAddress.read(&mut cell(&values[1])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPool778f_106049() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPool778f", 106049)?;
    let client = recording_client(106049, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_decimals())?;
    assert_eq!(
        value.decimals_x,
        <u8>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(
        value.decimals_y,
        <u8>::try_from(integer(&values[1])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPool778f_111021() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPool778f", 111021)?;
    let client = recording_client(111021, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_scaling_factors())?;
    assert_eq!(
        value.factor_x,
        <u128>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(
        value.factor_y,
        <u128>::try_from(integer(&values[1])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPoolC0f9_82320() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPoolC0f9", 82320)?;
    let client = recording_client(82320, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_version())?;
    assert_eq!(
        value,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPoolC0f9_103723() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPoolC0f9", 103723)?;
    let client = recording_client(103723, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.is_stable())?;
    assert_eq!(value, integer(&values[0])? != I512::ZERO);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPoolC0f9_96780() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPoolC0f9", 96780)?;
    let client = recording_client(96780, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_trade_fee())?;
    assert_eq!(
        value.trade_fee_numerator,
        <u16>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(
        value.trade_fee_denominator,
        <u16>::try_from(integer(&values[1])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPoolC0f9_65971() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPoolC0f9", 65971)?;
    let client = recording_client(65971, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_reserves())?;
    assert_eq!(value.reserve0, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.reserve1, Coins::try_from(integer(&values[1])?)?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPoolC0f9_112792() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPoolC0f9", 112792)?;
    let client = recording_client(112792, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_protocol_fees())?;
    assert_eq!(value.asset0_protocol_fee, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.asset1_protocol_fee, Coins::try_from(integer(&values[1])?)?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPoolC0f9_118188() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPoolC0f9", 118188)?;
    let client = recording_client(118188, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_assets())?;
    assert_eq!(value.asset0, DedustClassicAsset::from_cell(&cell(&values[0])?)?);
    assert_eq!(value.asset1, DedustClassicAsset::from_cell(&cell(&values[1])?)?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPoolC0f9_70754() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPoolC0f9", 70754)?;
    let client = recording_client(70754, fixture_arguments(&["native", "1000000"])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_swap_out(
        DedustClassicAsset::from_cell(&fixture_arguments(&["native"])?.pop_cell()?)?,
        1000000.into(),
    ))?;
    assert_eq!(value.asset_out, DedustClassicAsset::from_cell(&cell(&values[0])?)?);
    assert_eq!(value.amount_out, integer(&values[1])?);
    assert_eq!(value.trade_fee, integer(&values[2])?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPoolC0f9_119877() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPoolC0f9", 119877)?;
    let client = recording_client(119877, fixture_arguments(&["1000000", "1000000"])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.estimate_deposit_out(1000000.into(), 1000000.into()))?;
    assert_eq!(value.asset0_deposit, integer(&values[0])?);
    assert_eq!(value.asset1_deposit, integer(&values[1])?);
    assert_eq!(value.fair_supply, integer(&values[2])?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPoolC0f9_106029() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPoolC0f9", 106029)?;
    let client = recording_client(106029, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_jetton_data())?;
    assert_eq!(value.total_supply, Coins::try_from(integer(&values[0])?)?);
    assert_eq!(value.mintable, integer(&values[1])? != I512::ZERO);
    assert_eq!(value.admin_address, NullableAddress.read(&mut cell(&values[2])?.parser())?);
    assert_eq!(value.jetton_content, cell(&values[3])?);
    assert_eq!(value.jetton_wallet_code, cell(&values[4])?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPoolC0f9_103289() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPoolC0f9", 103289)?;
    let client = recording_client(103289, fixture_arguments(&["owner"])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_wallet_address(TonAddress::ZERO))?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPoolC0f9_66722() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPoolC0f9", 66722)?;
    let client = recording_client(66722, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_quote_providers())?;
    assert_eq!(value.provider0, NullableAddress.read(&mut cell(&values[0])?.parser())?);
    assert_eq!(value.provider1, NullableAddress.read(&mut cell(&values[1])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPoolC0f9_106049() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPoolC0f9", 106049)?;
    let client = recording_client(106049, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_decimals())?;
    assert_eq!(
        value.decimals_x,
        <u8>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(
        value.decimals_y,
        <u8>::try_from(integer(&values[1])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_OperatorManagedPoolC0f9_111021() -> anyhow::Result<()> {
    let values = fixture_values("OperatorManagedPoolC0f9", 111021)?;
    let client = recording_client(111021, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_scaling_factors())?;
    assert_eq!(
        value.factor_x,
        <u128>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(
        value.factor_y,
        <u128>::try_from(integer(&values[1])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    Ok(())
}
#[test]
fn test_raw_getter_CpmmPoolV1_72157() -> anyhow::Result<()> {
    let values = fixture_values("CpmmPoolV1", 72157)?;
    let client = recording_client(72157, fixture_arguments(&["owner"])?, fixture_stack(values)?)?;
    let contract = DedustV2CpmmContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_position_address(TonAddress::ZERO))?;
    assert_eq!(value, InternalAddress.read(&mut cell(&values[0])?.parser())?);
    Ok(())
}
#[test]
fn test_raw_getter_CpmmPoolV1_81689() -> anyhow::Result<()> {
    let values = fixture_values("CpmmPoolV1", 81689)?;
    let client = recording_client(81689, fixture_arguments(&[])?, fixture_stack(values)?)?;
    let contract = DedustV2CpmmContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_pool_data())?;
    assert_eq!(
        value.status,
        <i8>::try_from(integer(&values[0])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(value.deposit_active, integer(&values[1])? != I512::ZERO);
    assert_eq!(value.swap_active, integer(&values[2])? != I512::ZERO);
    assert_eq!(value.asset_x, NullableAddress.read(&mut cell(&values[3])?.parser())?);
    assert_eq!(value.asset_y, NullableAddress.read(&mut cell(&values[4])?.parser())?);
    assert!(value.wallets_by_assets.is_empty());
    assert!(value.assets_by_wallets.is_empty());
    assert!(value.wallets_resolutions.is_empty());
    assert_eq!(
        value.base_fee_bps,
        <i32>::try_from(integer(&values[8])?).map_err(|_| anyhow::anyhow!("narrow"))?
    );
    assert_eq!(value.reserve_x, Coins::try_from(integer(&values[9])?)?);
    assert_eq!(value.reserve_y, Coins::try_from(integer(&values[10])?)?);
    assert_eq!(value.liquidity, Coins::try_from(integer(&values[11])?)?);
    assert_eq!(value.protocol_fee_x, Coins::try_from(integer(&values[12])?)?);
    assert_eq!(value.protocol_fee_y, Coins::try_from(integer(&values[13])?)?);
    assert_eq!(value.creator_fee_x, Coins::try_from(integer(&values[14])?)?);
    assert_eq!(value.creator_fee_y, Coins::try_from(integer(&values[15])?)?);
    assert_eq!(value.x_fee_per_liquidity, U256::from_str(&integer(&values[16])?.to_string())?);
    assert_eq!(value.y_fee_per_liquidity, U256::from_str(&integer(&values[17])?.to_string())?);
    assert!(value.rewards.is_empty());
    Ok(())
}
#[test]
fn test_raw_getter_CpmmPoolV1_112421() -> anyhow::Result<()> {
    let values = fixture_values("CpmmPoolV1", 112421)?;
    let client = recording_client(112421, fixture_arguments(&["1000000"])?, fixture_stack(values)?)?;
    let contract = DedustV2CpmmContract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(contract.get_liquidity_amounts(1000000.into()))?;
    assert_eq!(value.amount_x, integer(&values[0])?);
    assert_eq!(value.amount_y, integer(&values[1])?);
    Ok(())
}
