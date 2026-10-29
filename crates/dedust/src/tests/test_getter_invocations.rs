//! Exercise wrapper calls, including macro selectors and ABI-specific Slice packing.
use super::test_getter_provider::RecordingProvider;
use crate::contracts::classic::asset::DedustClassicAsset;
use crate::contracts::classic::{factory_v1, operator_pool, pool_v2};
use fastnum::I512;
use fastnum::U256;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use ton::{
    block_tlb::TVMStack,
    contracts::{ContractClient, TonContract},
    ton_core::{cell::TonCell, traits::tlb::TLB, types::TonAddress},
};

fn numbers(values: &[i64]) -> TVMStack {
    let mut s = TVMStack::EMPTY;
    for value in values {
        s.push_int((*value).into());
    }
    s
}
fn address_reply() -> anyhow::Result<TVMStack> {
    let mut s = TVMStack::EMPTY;
    s.push_cell_slice(TonAddress::ZERO.to_msg_address_int().to_cell()?);
    Ok(s)
}

#[test]
fn test_wrappers_preserve_selectors_argument_order_slice_and_internal_zero() -> anyhow::Result<()> {
    let mut owner = TVMStack::EMPTY;
    owner.push_cell_slice(TonAddress::ZERO.to_msg_address_int().to_cell()?);
    let mut child = owner.clone();
    child.push_int(I512::ONE);
    // Independent compact DedustClassicAsset encodings: native tag0, jetton tag1/wc-1/hash7.
    let mut native = TonCell::builder();
    native.write_num(&0u8, 4)?;
    let native = native.build()?;
    let mut jetton = TonCell::builder();
    jetton.write_num(&1u8, 4)?;
    jetton.write_num(&-1i8, 8)?;
    jetton.write_num(&U256::from_u64(7), 256)?;
    let jetton = jetton.build()?;
    child.push_cell_slice(native.clone());
    child.push_cell_slice(jetton.clone());
    let provider = RecordingProvider(Arc::new(Mutex::new(VecDeque::from([
        (82320, TVMStack::EMPTY, numbers(&[7])),
        (119877, numbers(&[11, 29]), numbers(&[3, 5, 13])),
        (103289, owner, address_reply()?),
        (84481, child, address_reply()?),
        (82320, TVMStack::EMPTY, numbers(&[8])),
        (96780, TVMStack::EMPTY, numbers(&[30, 10000])),
        (112861, TVMStack::EMPTY, numbers(&[1100])),
    ]))));
    let client = ContractClient::builder(provider.clone(), provider.clone())?.build()?;
    let pool = pool_v2::DedustPoolV2Contract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(tokio_test::block_on(pool.get_version())?, 7);
    let deposit = tokio_test::block_on(pool.estimate_deposit_out(11.into(), 29.into()))?;
    assert_eq!(deposit.fair_supply, 13.into());
    assert_eq!(
        tokio_test::block_on(pool.get_wallet_address(TonAddress::ZERO))?,
        TonAddress::ZERO
    );
    let factory = factory_v1::DedustFactoryV1Contract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(
        tokio_test::block_on(factory.get_liquidity_deposit_address(
            TonAddress::ZERO,
            true,
            DedustClassicAsset::from_cell(&native)?,
            DedustClassicAsset::from_cell(&jetton)?
        ))?,
        TonAddress::ZERO
    );
    let managed = operator_pool::DedustClassicOperatorPoolContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(tokio_test::block_on(managed.get_version())?, 8);
    assert_eq!(tokio_test::block_on(managed.get_trade_fee())?.trade_fee_numerator, 30);
    assert_eq!(tokio_test::block_on(managed.get_start_at())?, 1100);
    assert!(provider.0.lock().map_err(|_| anyhow::anyhow!("recording lock"))?.is_empty());
    Ok(())
}

#[test]
fn test_factory_code_selectors_and_raw_operator_address() -> anyhow::Result<()> {
    use crate::contracts::classic::factory_v2;
    use ton::block_tlb::{TVMNull, TVMStackValue};

    let code = TonCell::empty().clone();
    let mut reply = numbers(&[23]);
    reply.push_cell(code.clone());
    for selector in [79554, 120195, 129291] {
        let client = super::test_getter_provider::recording_client(
            selector,
            if selector == 129291 {
                numbers(&[1])
            } else {
                TVMStack::EMPTY
            },
            reply.clone(),
        )?;
        let factory = factory_v1::DedustFactoryV1Contract::new(&client, &TonAddress::ZERO, None);
        let value = match selector {
            79554 => tokio_test::block_on(factory.get_pool_code())?,
            120195 => tokio_test::block_on(factory.get_liquidity_deposit_code())?,
            _ => tokio_test::block_on(factory.get_vault_code(1))?,
        };
        assert_eq!(value.version, 23);
        assert_eq!(*value.code, code);
    }
    let mut nullable = numbers(&[0]);
    nullable.push(TVMStackValue::Null(TVMNull));
    let client = super::test_getter_provider::recording_client(78248, TVMStack::EMPTY, nullable)?;
    let factory = factory_v2::DedustFactoryV2Contract::new(&client, &TonAddress::ZERO, None);
    let value = tokio_test::block_on(factory.get_operator_code())?;
    assert_eq!(value.version, 0);
    assert_eq!(value.code, None);
    let client = super::test_getter_provider::recording_client(126188, numbers(&[10]), address_reply()?)?;
    let factory = factory_v2::DedustFactoryV2Contract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(
        tokio_test::block_on(factory.get_operator_address(10))?,
        ton::ton_core::types::TonAddress::from_msg_address(TonAddress::ZERO.to_msg_address_int())?
    );
    Ok(())
}

#[test]
fn test_pending_presence_preserves_outside_uint64_key_arguments() -> anyhow::Result<()> {
    use crate::contracts::custom_wallet::v2::DedustX1000WalletV2Contract;
    use fastnum::I512;

    for key in [-I512::ONE, I512::ONE << 64u32] {
        let mut arguments = TVMStack::EMPTY;
        arguments.push_int(key);
        let client = super::test_getter_provider::recording_client(97376, arguments, numbers(&[0]))?;
        let contract = DedustX1000WalletV2Contract::new(&client, &TonAddress::ZERO, None);
        assert_eq!(tokio_test::block_on(contract.get_pending_operation_presence(key))?, 0);
    }
    Ok(())
}

#[test]
fn test_factory_pool_flags_encode_integer_zero_or_one() -> anyhow::Result<()> {
    use super::test_getter_provider::{fixture_arguments, recording_client};
    use crate::contracts::classic::factory_v2;
    for version in [1, 2] {
        for is_stable in [false, true] {
            for selector in [101789, 84481] {
                let flag = if is_stable { "1" } else { "0" };
                let arguments = if selector == 101789 {
                    fixture_arguments(&[flag, "native", "jetton"])?
                } else {
                    fixture_arguments(&["owner", flag, "native", "jetton"])?
                };
                let client = recording_client(selector, arguments, address_reply()?)?;
                let native = fixture_arguments(&["native"])?.pop_cell()?;
                let jetton = fixture_arguments(&["jetton"])?.pop_cell()?;
                let result = if version == 1 {
                    let factory = factory_v1::DedustFactoryV1Contract::new(&client, &TonAddress::ZERO, None);
                    let native = DedustClassicAsset::from_cell(&native)?;
                    let jetton = DedustClassicAsset::from_cell(&jetton)?;
                    if selector == 101789 {
                        tokio_test::block_on(factory.get_pool_address(is_stable, native, jetton))?
                    } else {
                        tokio_test::block_on(factory.get_liquidity_deposit_address(
                            TonAddress::ZERO,
                            is_stable,
                            native,
                            jetton,
                        ))?
                    }
                } else {
                    let factory = factory_v2::DedustFactoryV2Contract::new(&client, &TonAddress::ZERO, None);
                    let native = DedustClassicAsset::from_cell(&native)?;
                    let jetton = DedustClassicAsset::from_cell(&jetton)?;
                    if selector == 101789 {
                        tokio_test::block_on(factory.get_pool_address(is_stable, native, jetton))?
                    } else {
                        tokio_test::block_on(factory.get_liquidity_deposit_address(
                            TonAddress::ZERO,
                            is_stable,
                            native,
                            jetton,
                        ))?
                    }
                };
                assert_eq!(result, TonAddress::ZERO);
            }
        }
    }
    Ok(())
}
