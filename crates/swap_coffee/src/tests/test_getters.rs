//! Strict getter tuples and exact selector/argument encoding; no actual TVM execution.

use super::getter_provider::*;

use super::test_schema_vectors::cell_for;

use crate::contracts::*;

use fastnum::I512;

use std::sync::atomic::Ordering;

use ton::block_tlb::FromTVMStack;
use ton::block_tlb::TVMNull;
use ton::block_tlb::TVMStack;

use ton::contracts::TonContract;

use ton::ton_core::cell::TonCell;

use ton::ton_core::traits::tlb::TLB;

use ton::ton_core::types::TonAddress;

use ton::ton_core::types::tlb_core::MsgAddressInt;
#[test]
fn test_cross_dex_get_admin_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(none_address_cell()?);
    let decoded = nullable_address(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(nullable_address(&mut missing).is_err());
    }
    assert!(decoded.is_none());
    assert!(nullable_address(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(nullable_address(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(113142, input, output)?;
    let wrapper = cross_dex::CoffeeCrossDexContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_admin())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_cross_dex_bidask_get_admin_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(none_address_cell()?);
    let decoded = nullable_address(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(nullable_address(&mut missing).is_err());
    }
    assert!(decoded.is_none());
    assert!(nullable_address(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(nullable_address(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(113142, input, output)?;
    let wrapper = cross_dex_bidask::CoffeeCrossDexBidaskContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_admin())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_factory_get_vault_address_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(address_cell(1)?);
    output.push_tiny_int(2);
    let decoded =
        crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut missing)
                .is_err()
        );
    }
    assert_eq!(
        decoded.address,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(1)?)?)?
    );
    assert_eq!(decoded.hash, I512::from_i64(2));
    assert!(
        crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut extra_stack(
            &output
        ))
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut wrong)
                .is_err()
        );
    }
    let mut input = TVMStack::default();
    let asset: factory::msgs::CoffeeFactoryAsset =
        factory::msgs::CoffeeFactoryAsset::ExtraAsset(Box::new(factory::msgs::CoffeeFactoryExtraAsset {
            id: 0xf1234567,
        }));
    let mut expected_asset = TonCell::builder();
    expected_asset.write_num(&2u8, 2)?;
    expected_asset.write_num(&0xf1234567u32, 32)?;
    input.push_cell_slice(expected_asset.build()?);
    let (client, calls) = client(76695, input, output)?;
    let wrapper = factory::CoffeeFactoryContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_vault_address(&asset))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_factory_get_pool_address_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(address_cell(1)?);
    output.push_tiny_int(2);
    let decoded =
        crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut missing)
                .is_err()
        );
    }
    assert_eq!(
        decoded.address,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(1)?)?)?
    );
    assert_eq!(decoded.hash, I512::from_i64(2));
    assert!(
        crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut extra_stack(
            &output
        ))
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut wrong)
                .is_err()
        );
    }
    let mut input = TVMStack::default();
    let asset1: factory::msgs::CoffeeFactoryAsset =
        factory::msgs::CoffeeFactoryAsset::ExtraAsset(Box::new(factory::msgs::CoffeeFactoryExtraAsset {
            id: 0xf1234567,
        }));
    let mut expected_asset = TonCell::builder();
    expected_asset.write_num(&2u8, 2)?;
    expected_asset.write_num(&0xf1234567u32, 32)?;
    input.push_cell_slice(expected_asset.build()?);
    let asset2: factory::msgs::CoffeeFactoryAsset =
        factory::msgs::CoffeeFactoryAsset::ExtraAsset(Box::new(factory::msgs::CoffeeFactoryExtraAsset {
            id: 0xf1234567,
        }));
    let mut expected_asset = TonCell::builder();
    expected_asset.write_num(&2u8, 2)?;
    expected_asset.write_num(&0xf1234567u32, 32)?;
    input.push_cell_slice(expected_asset.build()?);
    let amm: I512 = I512::from_i64(3);
    input.push_int(amm);
    let amm_settings: TonCell = opaque_cell()?;
    input.push_cell(amm_settings.clone());
    let (client, calls) = client(101789, input, output)?;
    let wrapper = factory::CoffeeFactoryContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_pool_address(&asset1, &asset2, &amm, &amm_settings))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_factory_get_pool_address_no_settings_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(address_cell(1)?);
    output.push_tiny_int(2);
    let decoded =
        crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut missing)
                .is_err()
        );
    }
    assert_eq!(
        decoded.address,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(1)?)?)?
    );
    assert_eq!(decoded.hash, I512::from_i64(2));
    assert!(
        crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut extra_stack(
            &output
        ))
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut wrong)
                .is_err()
        );
    }
    let mut input = TVMStack::default();
    let asset1: factory::msgs::CoffeeFactoryAsset =
        factory::msgs::CoffeeFactoryAsset::ExtraAsset(Box::new(factory::msgs::CoffeeFactoryExtraAsset {
            id: 0xf1234567,
        }));
    let mut expected_asset = TonCell::builder();
    expected_asset.write_num(&2u8, 2)?;
    expected_asset.write_num(&0xf1234567u32, 32)?;
    input.push_cell_slice(expected_asset.build()?);
    let asset2: factory::msgs::CoffeeFactoryAsset =
        factory::msgs::CoffeeFactoryAsset::ExtraAsset(Box::new(factory::msgs::CoffeeFactoryExtraAsset {
            id: 0xf1234567,
        }));
    let mut expected_asset = TonCell::builder();
    expected_asset.write_num(&2u8, 2)?;
    expected_asset.write_num(&0xf1234567u32, 32)?;
    input.push_cell_slice(expected_asset.build()?);
    let amm: I512 = I512::from_i64(3);
    input.push_int(amm);
    let (client, calls) = client(71347, input, output)?;
    let wrapper = factory::CoffeeFactoryContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_pool_address_no_settings(&asset1, &asset2, &amm))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_factory_get_pool_creator_address_stack_and_request() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let mut output = TVMStack::default();
        output.push_cell_slice(address_cell(1)?);
        output.push_tiny_int(2);
        let decoded = crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(
            &mut output.clone(),
        )?;
        for remaining in 0..output.len() {
            let mut missing = output.clone();
            missing.truncate(remaining);
            assert!(
                crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut missing)
                    .is_err()
            );
        }
        assert_eq!(
            decoded.address,
            ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(1)?)?)?
        );
        assert_eq!(decoded.hash, I512::from_i64(2));
        assert!(
            crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(
                &mut extra_stack(&output)
            )
            .is_err()
        );
        for index in 0..output.len() {
            let mut wrong = output.clone();
            wrong[index] = wrong_type(&output[index])?;
            assert!(
                crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut wrong)
                    .is_err()
            );
        }
        let mut input = TVMStack::default();
        let owner = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::from_cell(&address_cell(1)?)?
        };
        input.push_cell_slice(owner.to_msg_address_int().to_cell()?);
        let asset1: factory::msgs::CoffeeFactoryAsset =
            factory::msgs::CoffeeFactoryAsset::ExtraAsset(Box::new(factory::msgs::CoffeeFactoryExtraAsset {
                id: 0xf1234567,
            }));
        let mut expected_asset = TonCell::builder();
        expected_asset.write_num(&2u8, 2)?;
        expected_asset.write_num(&0xf1234567u32, 32)?;
        input.push_cell_slice(expected_asset.build()?);
        let asset2: factory::msgs::CoffeeFactoryAsset =
            factory::msgs::CoffeeFactoryAsset::ExtraAsset(Box::new(factory::msgs::CoffeeFactoryExtraAsset {
                id: 0xf1234567,
            }));
        let mut expected_asset = TonCell::builder();
        expected_asset.write_num(&2u8, 2)?;
        expected_asset.write_num(&0xf1234567u32, 32)?;
        input.push_cell_slice(expected_asset.build()?);
        let amm: I512 = I512::from_i64(4);
        input.push_int(amm);
        let amm_settings: TonCell = opaque_cell()?;
        input.push_cell(amm_settings.clone());
        let (client, calls) = client(74737, input, output)?;
        let wrapper = factory::CoffeeFactoryContract::new(&client, &TonAddress::ZERO, None);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        let result =
            tokio_test::block_on(wrapper.get_pool_creator_address(&owner, &asset1, &asset2, &amm, &amm_settings))?;
        assert_eq!(result, decoded);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    Ok(())
}
#[test]
fn test_factory_get_pool_creator_address_no_settings_stack_and_request() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let mut output = TVMStack::default();
        output.push_cell_slice(address_cell(1)?);
        output.push_tiny_int(2);
        let decoded = crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(
            &mut output.clone(),
        )?;
        for remaining in 0..output.len() {
            let mut missing = output.clone();
            missing.truncate(remaining);
            assert!(
                crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut missing)
                    .is_err()
            );
        }
        assert_eq!(
            decoded.address,
            ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(1)?)?)?
        );
        assert_eq!(decoded.hash, I512::from_i64(2));
        assert!(
            crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(
                &mut extra_stack(&output)
            )
            .is_err()
        );
        for index in 0..output.len() {
            let mut wrong = output.clone();
            wrong[index] = wrong_type(&output[index])?;
            assert!(
                crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut wrong)
                    .is_err()
            );
        }
        let mut input = TVMStack::default();
        let owner = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::from_cell(&address_cell(1)?)?
        };
        input.push_cell_slice(owner.to_msg_address_int().to_cell()?);
        let asset1: factory::msgs::CoffeeFactoryAsset =
            factory::msgs::CoffeeFactoryAsset::ExtraAsset(Box::new(factory::msgs::CoffeeFactoryExtraAsset {
                id: 0xf1234567,
            }));
        let mut expected_asset = TonCell::builder();
        expected_asset.write_num(&2u8, 2)?;
        expected_asset.write_num(&0xf1234567u32, 32)?;
        input.push_cell_slice(expected_asset.build()?);
        let asset2: factory::msgs::CoffeeFactoryAsset =
            factory::msgs::CoffeeFactoryAsset::ExtraAsset(Box::new(factory::msgs::CoffeeFactoryExtraAsset {
                id: 0xf1234567,
            }));
        let mut expected_asset = TonCell::builder();
        expected_asset.write_num(&2u8, 2)?;
        expected_asset.write_num(&0xf1234567u32, 32)?;
        input.push_cell_slice(expected_asset.build()?);
        let amm: I512 = I512::from_i64(4);
        input.push_int(amm);
        let (client, calls) = client(122690, input, output)?;
        let wrapper = factory::CoffeeFactoryContract::new(&client, &TonAddress::ZERO, None);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        let result =
            tokio_test::block_on(wrapper.get_pool_creator_address_no_settings(&owner, &asset1, &asset2, &amm))?;
        assert_eq!(result, decoded);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    Ok(())
}
#[test]
fn test_factory_get_liquidity_depository_address_stack_and_request() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let mut output = TVMStack::default();
        output.push_cell_slice(address_cell(1)?);
        output.push_tiny_int(2);
        let decoded = crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(
            &mut output.clone(),
        )?;
        for remaining in 0..output.len() {
            let mut missing = output.clone();
            missing.truncate(remaining);
            assert!(
                crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut missing)
                    .is_err()
            );
        }
        assert_eq!(
            decoded.address,
            ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(1)?)?)?
        );
        assert_eq!(decoded.hash, I512::from_i64(2));
        assert!(
            crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(
                &mut extra_stack(&output)
            )
            .is_err()
        );
        for index in 0..output.len() {
            let mut wrong = output.clone();
            wrong[index] = wrong_type(&output[index])?;
            assert!(
                crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut wrong)
                    .is_err()
            );
        }
        let mut input = TVMStack::default();
        let owner = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::from_cell(&address_cell(1)?)?
        };
        input.push_cell_slice(owner.to_msg_address_int().to_cell()?);
        let asset1: factory::msgs::CoffeeFactoryAsset =
            factory::msgs::CoffeeFactoryAsset::ExtraAsset(Box::new(factory::msgs::CoffeeFactoryExtraAsset {
                id: 0xf1234567,
            }));
        let mut expected_asset = TonCell::builder();
        expected_asset.write_num(&2u8, 2)?;
        expected_asset.write_num(&0xf1234567u32, 32)?;
        input.push_cell_slice(expected_asset.build()?);
        let asset2: factory::msgs::CoffeeFactoryAsset =
            factory::msgs::CoffeeFactoryAsset::ExtraAsset(Box::new(factory::msgs::CoffeeFactoryExtraAsset {
                id: 0xf1234567,
            }));
        let mut expected_asset = TonCell::builder();
        expected_asset.write_num(&2u8, 2)?;
        expected_asset.write_num(&0xf1234567u32, 32)?;
        input.push_cell_slice(expected_asset.build()?);
        let amm: I512 = I512::from_i64(4);
        input.push_int(amm);
        let amm_settings: TonCell = opaque_cell()?;
        input.push_cell(amm_settings.clone());
        let (client, calls) = client(94971, input, output)?;
        let wrapper = factory::CoffeeFactoryContract::new(&client, &TonAddress::ZERO, None);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        let result = tokio_test::block_on(wrapper.get_liquidity_depository_address(
            &owner,
            &asset1,
            &asset2,
            &amm,
            &amm_settings,
        ))?;
        assert_eq!(result, decoded);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    Ok(())
}
#[test]
fn test_factory_get_liquidity_depository_address_no_settings_stack_and_request() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let mut output = TVMStack::default();
        output.push_cell_slice(address_cell(1)?);
        output.push_tiny_int(2);
        let decoded = crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(
            &mut output.clone(),
        )?;
        for remaining in 0..output.len() {
            let mut missing = output.clone();
            missing.truncate(remaining);
            assert!(
                crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut missing)
                    .is_err()
            );
        }
        assert_eq!(
            decoded.address,
            ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(1)?)?)?
        );
        assert_eq!(decoded.hash, I512::from_i64(2));
        assert!(
            crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(
                &mut extra_stack(&output)
            )
            .is_err()
        );
        for index in 0..output.len() {
            let mut wrong = output.clone();
            wrong[index] = wrong_type(&output[index])?;
            assert!(
                crate::contracts::factory::method_results::CoffeeFactoryAddressWithHashResult::from_stack(&mut wrong)
                    .is_err()
            );
        }
        let mut input = TVMStack::default();
        let owner = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::from_cell(&address_cell(1)?)?
        };
        input.push_cell_slice(owner.to_msg_address_int().to_cell()?);
        let asset1: factory::msgs::CoffeeFactoryAsset =
            factory::msgs::CoffeeFactoryAsset::ExtraAsset(Box::new(factory::msgs::CoffeeFactoryExtraAsset {
                id: 0xf1234567,
            }));
        let mut expected_asset = TonCell::builder();
        expected_asset.write_num(&2u8, 2)?;
        expected_asset.write_num(&0xf1234567u32, 32)?;
        input.push_cell_slice(expected_asset.build()?);
        let asset2: factory::msgs::CoffeeFactoryAsset =
            factory::msgs::CoffeeFactoryAsset::ExtraAsset(Box::new(factory::msgs::CoffeeFactoryExtraAsset {
                id: 0xf1234567,
            }));
        let mut expected_asset = TonCell::builder();
        expected_asset.write_num(&2u8, 2)?;
        expected_asset.write_num(&0xf1234567u32, 32)?;
        input.push_cell_slice(expected_asset.build()?);
        let amm: I512 = I512::from_i64(4);
        input.push_int(amm);
        let (client, calls) = client(78874, input, output)?;
        let wrapper = factory::CoffeeFactoryContract::new(&client, &TonAddress::ZERO, None);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        let result =
            tokio_test::block_on(wrapper.get_liquidity_depository_address_no_settings(&owner, &asset1, &asset2, &amm))?;
        assert_eq!(result, decoded);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    Ok(())
}
#[test]
fn test_factory_get_admin_address_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(none_address_cell()?);
    let decoded = nullable_address(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(nullable_address(&mut missing).is_err());
    }
    assert!(decoded.is_none());
    assert!(nullable_address(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(nullable_address(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(94255, input, output)?;
    let wrapper = factory::CoffeeFactoryContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_admin_address())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_factory_get_code_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell(cell_for("factory/CoffeeFactoryCodeCells/0")?);
    output.push_cell(cell_for("factory/CoffeeFactoryCodeDictionaries/0")?);
    let decoded =
        crate::contracts::factory::method_results::CoffeeFactoryGetCodeResult::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::factory::method_results::CoffeeFactoryGetCodeResult::from_stack(&mut missing).is_err()
        );
    }
    assert!(
        crate::contracts::factory::method_results::CoffeeFactoryGetCodeResult::from_stack(&mut extra_stack(&output))
            .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(crate::contracts::factory::method_results::CoffeeFactoryGetCodeResult::from_stack(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(81234, input, output)?;
    let wrapper = factory::CoffeeFactoryContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_code())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_lp_wallet_get_wallet_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_cell_slice(address_cell(2)?);
    output.push_cell_slice(address_cell(3)?);
    output.push_cell(opaque_cell()?);
    let decoded = crate::contracts::lp_wallet::method_results::CoffeeLpWalletGetWalletDataResult::from_stack(
        &mut output.clone(),
    )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::lp_wallet::method_results::CoffeeLpWalletGetWalletDataResult::from_stack(&mut missing)
                .is_err()
        );
    }
    assert_eq!(decoded.balance, I512::from_i64(1));
    assert_eq!(
        decoded.owner_address,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(2)?)?)?
    );
    assert_eq!(
        decoded.jetton_master_address,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(3)?)?)?
    );
    assert_eq!(decoded.jetton_wallet_code, opaque_cell()?);
    assert!(
        crate::contracts::lp_wallet::method_results::CoffeeLpWalletGetWalletDataResult::from_stack(&mut extra_stack(
            &output
        ))
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::lp_wallet::method_results::CoffeeLpWalletGetWalletDataResult::from_stack(&mut wrong)
                .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(97026, input, output)?;
    let wrapper = lp_wallet::CoffeeLpWalletContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_wallet_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_mev_protector_get_public_key_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    let decoded = <fastnum::U256>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<fastnum::U256>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, fastnum::U256::from(1u8));
    assert!(<fastnum::U256>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<fastnum::U256>::from_stack(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(78748, input, output)?;
    let wrapper = mev_protector::CoffeeMevProtectorContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_public_key())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_mev_protector_processed_query_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    let decoded = <bool>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<bool>::from_stack(&mut missing).is_err());
    }
    assert!(decoded);
    assert!(<bool>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<bool>::from_stack(&mut wrong).is_err());
    }
    let mut input = TVMStack::default();
    let query_id: u64 = 1;
    input.push_int(I512::from_i64(1));
    let (client, calls) = client(117746, input, output)?;
    let wrapper = mev_protector::CoffeeMevProtectorContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.processed_query(&query_id))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_mev_protector_legacy_get_public_key_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    let decoded = <fastnum::U256>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<fastnum::U256>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, fastnum::U256::from(1u8));
    assert!(<fastnum::U256>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<fastnum::U256>::from_stack(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(78748, input, output)?;
    let wrapper = mev_protector_legacy::CoffeeMevProtectorLegacyContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_public_key())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_mev_protector_legacy_processed_query_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    let decoded = <bool>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<bool>::from_stack(&mut missing).is_err());
    }
    assert!(decoded);
    assert!(<bool>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<bool>::from_stack(&mut wrong).is_err());
    }
    let mut input = TVMStack::default();
    let query_id: u64 = 1;
    input.push_int(I512::from_i64(1));
    let (client, calls) = client(117746, input, output)?;
    let wrapper = mev_protector_legacy::CoffeeMevProtectorLegacyContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.processed_query(&query_id))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_pool_constant_product_get_jetton_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_tiny_int(-1);
    output.push_cell_slice(address_cell(3)?);
    output.push_cell(opaque_cell()?);
    output.push_cell(opaque_cell()?);
    let decoded =
        crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolGetJettonDataResult::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolGetJettonDataResult::from_stack(&mut missing)
                .is_err()
        );
    }
    assert_eq!(decoded.total_supply, I512::from_i64(1));
    assert!(decoded.mintable);
    assert_eq!(
        decoded.admin_address,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(3)?)?)?
    );
    assert_eq!(decoded.jetton_content, opaque_cell()?);
    assert_eq!(decoded.jetton_wallet_code, opaque_cell()?);
    assert!(
        crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolGetJettonDataResult::from_stack(&mut extra_stack(
            &output
        ))
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolGetJettonDataResult::from_stack(&mut wrong)
                .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(106029, input, output)?;
    let wrapper = pool_constant_product::CoffeeConstantProductPoolContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_jetton_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_pool_constant_product_get_wallet_address_stack_and_request() -> anyhow::Result<()> {
    for owner_address in [TonAddress::from_cell(&address_cell(1)?)?, TonAddress::ZERO] {
        let mut input = TVMStack::default();
        // Independent expected encoding: zero must stay addr_std, not addr_none.
        input.push_cell_slice(owner_address.to_msg_address_int().to_cell()?);
        let mut output = TVMStack::default();
        output.push_cell_slice(address_cell(2)?);
        let (client, calls) = client(103289, input, output)?;
        let wrapper = pool_constant_product::CoffeeConstantProductPoolContract::new(&client, &TonAddress::ZERO, None);
        let result: TonAddress = tokio_test::block_on(wrapper.get_wallet_address(&owner_address))?;
        assert_eq!(result, TonAddress::from_cell(&address_cell(2)?)?);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    let (client, calls) = client(103289, TVMStack::default(), TVMStack::default())?;
    let wrapper = pool_constant_product::CoffeeConstantProductPoolContract::new(&client, &TonAddress::ZERO, None);
    let invalid_owner = TonAddress::new(128, TonAddress::ZERO.hash);
    assert!(tokio_test::block_on(wrapper.get_wallet_address(&invalid_owner)).is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    Ok(())
}
#[test]
fn test_pool_constant_product_get_pool_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_cell_slice(opaque_cell()?);
    output.push_cell_slice(opaque_cell()?);
    output.push_tiny_int(4);
    output.push(TVMNull {}.into());
    output.push_tiny_int(-1);
    output.push_tiny_int(7);
    output.push_tiny_int(8);
    output.push_tiny_int(9);
    output.push_tiny_int(10);
    output.push_tiny_int(11);
    let decoded =
        crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolGetPoolDataResult::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolGetPoolDataResult::from_stack(&mut missing)
                .is_err()
        );
    }
    assert_eq!(decoded.pool_version, I512::from_i64(1));
    assert_eq!(decoded.asset1, opaque_cell()?);
    assert_eq!(decoded.asset2, opaque_cell()?);
    assert_eq!(decoded.amm, I512::from_i64(4));
    assert!(decoded.amm_settings.is_none());
    assert!(decoded.active);
    assert_eq!(decoded.reserve1, I512::from_i64(7));
    assert_eq!(decoded.reserve2, I512::from_i64(8));
    assert_eq!(decoded.total_supply, I512::from_i64(9));
    assert_eq!(decoded.protocol_fee, I512::from_i64(10));
    assert_eq!(decoded.lp_fee, I512::from_i64(11));
    assert!(
        crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolGetPoolDataResult::from_stack(&mut extra_stack(
            &output
        ))
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolGetPoolDataResult::from_stack(&mut wrong).is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(81689, input, output)?;
    let wrapper = pool_constant_product::CoffeeConstantProductPoolContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_pool_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_pool_constant_product_estimate_swap_amount_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_tiny_int(2);
    let decoded = crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolEstimateSwapAmountResult::from_stack(
        &mut output.clone(),
    )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolEstimateSwapAmountResult::from_stack(&mut missing)
                .is_err()
        );
    }
    assert_eq!(decoded.input_amount, I512::from_i64(1));
    assert_eq!(decoded.output_amount, I512::from_i64(2));
    assert!(
        crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolEstimateSwapAmountResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolEstimateSwapAmountResult::from_stack(&mut wrong)
                .is_err()
        );
    }
    let mut input = TVMStack::default();
    let asset: pool_constant_product::storage::CoffeePoolAsset =
        pool_constant_product::storage::CoffeePoolAsset::CoffeePoolExtraAsset(Box::new(
            pool_constant_product::storage::CoffeePoolExtraAsset { id: 0xf1234567 },
        ));
    let mut expected_asset = TonCell::builder();
    expected_asset.write_num(&2u8, 2)?;
    expected_asset.write_num(&0xf1234567u32, 32)?;
    input.push_cell_slice(expected_asset.build()?);
    let amount: I512 = I512::from_i64(2);
    input.push_int(amount);
    let (client, calls) = client(87113, input, output)?;
    let wrapper = pool_constant_product::CoffeeConstantProductPoolContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.estimate_swap_amount(&asset, &amount))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_pool_constant_product_estimate_liquidity_withdraw_amount_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_tiny_int(2);
    let decoded =
        crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolEstimateLiquidityWithdrawAmountResult::from_stack(
            &mut output.clone(),
        )?;
    assert_eq!(decoded.amount1, I512::from_i64(1));
    assert_eq!(decoded.amount2, I512::from_i64(2));
    assert!(
        crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolEstimateLiquidityWithdrawAmountResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolEstimateLiquidityWithdrawAmountResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let mut input = TVMStack::default();
    let lp_amount: I512 = I512::from_i64(1);
    input.push_int(lp_amount);
    let (client, calls) = client(114037, input, output)?;
    let wrapper = pool_constant_product::CoffeeConstantProductPoolContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.estimate_liquidity_withdraw_amount(&lp_amount))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_pool_constant_product_estimate_liquidity_deposit_amount_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_tiny_int(2);
    output.push_tiny_int(3);
    output.push_tiny_int(4);
    let decoded =
        crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolEstimateLiquidityDepositAmountResult::from_stack(
            &mut output.clone(),
        )?;
    assert_eq!(decoded.amount1_used, I512::from_i64(1));
    assert_eq!(decoded.amount2_used, I512::from_i64(2));
    assert_eq!(decoded.lp_amount, I512::from_i64(3));
    assert_eq!(decoded.lp_to_lock, I512::from_i64(4));
    assert!(
        crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolEstimateLiquidityDepositAmountResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::pool_constant_product::method_results::CoffeeConstantProductPoolEstimateLiquidityDepositAmountResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let mut input = TVMStack::default();
    let amount1: I512 = I512::from_i64(1);
    input.push_int(amount1);
    let amount2: I512 = I512::from_i64(2);
    input.push_int(amount2);
    let (client, calls) = client(113529, input, output)?;
    let wrapper = pool_constant_product::CoffeeConstantProductPoolContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.estimate_liquidity_deposit_amount(&amount1, &amount2))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_pool_curve_fi_stable_get_jetton_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_tiny_int(-1);
    output.push_cell_slice(address_cell(3)?);
    output.push_cell(opaque_cell()?);
    output.push_cell(opaque_cell()?);
    let decoded =
        crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolGetJettonDataResult::from_stack(
            &mut output.clone(),
        )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolGetJettonDataResult::from_stack(&mut missing)
                .is_err()
        );
    }
    assert_eq!(decoded.total_supply, I512::from_i64(1));
    assert!(decoded.mintable);
    assert_eq!(
        decoded.admin_address,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(3)?)?)?
    );
    assert_eq!(decoded.jetton_content, opaque_cell()?);
    assert_eq!(decoded.jetton_wallet_code, opaque_cell()?);
    assert!(
        crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolGetJettonDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolGetJettonDataResult::from_stack(&mut wrong)
                .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(106029, input, output)?;
    let wrapper = pool_curve_fi_stable::CoffeeCurveFiStablePoolContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_jetton_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_pool_curve_fi_stable_get_wallet_address_stack_and_request() -> anyhow::Result<()> {
    for owner_address in [TonAddress::from_cell(&address_cell(1)?)?, TonAddress::ZERO] {
        let mut input = TVMStack::default();
        // Independent expected encoding: zero must stay addr_std, not addr_none.
        input.push_cell_slice(owner_address.to_msg_address_int().to_cell()?);
        let mut output = TVMStack::default();
        output.push_cell_slice(address_cell(2)?);
        let (client, calls) = client(103289, input, output)?;
        let wrapper = pool_curve_fi_stable::CoffeeCurveFiStablePoolContract::new(&client, &TonAddress::ZERO, None);
        let result: TonAddress = tokio_test::block_on(wrapper.get_wallet_address(&owner_address))?;
        assert_eq!(result, TonAddress::from_cell(&address_cell(2)?)?);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    let (client, calls) = client(103289, TVMStack::default(), TVMStack::default())?;
    let wrapper = pool_curve_fi_stable::CoffeeCurveFiStablePoolContract::new(&client, &TonAddress::ZERO, None);
    let invalid_owner = TonAddress::new(128, TonAddress::ZERO.hash);
    assert!(tokio_test::block_on(wrapper.get_wallet_address(&invalid_owner)).is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    Ok(())
}
#[test]
fn test_pool_curve_fi_stable_get_pool_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_cell_slice(opaque_cell()?);
    output.push_cell_slice(opaque_cell()?);
    output.push_tiny_int(4);
    output.push(TVMNull {}.into());
    output.push_tiny_int(-1);
    output.push_tiny_int(7);
    output.push_tiny_int(8);
    output.push_tiny_int(9);
    output.push_tiny_int(10);
    output.push_tiny_int(11);
    let decoded =
        crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolGetPoolDataResult::from_stack(
            &mut output.clone(),
        )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolGetPoolDataResult::from_stack(&mut missing)
                .is_err()
        );
    }
    assert_eq!(decoded.pool_version, I512::from_i64(1));
    assert_eq!(decoded.asset1, opaque_cell()?);
    assert_eq!(decoded.asset2, opaque_cell()?);
    assert_eq!(decoded.amm, I512::from_i64(4));
    assert!(decoded.amm_settings.is_none());
    assert!(decoded.active);
    assert_eq!(decoded.reserve1, I512::from_i64(7));
    assert_eq!(decoded.reserve2, I512::from_i64(8));
    assert_eq!(decoded.total_supply, I512::from_i64(9));
    assert_eq!(decoded.protocol_fee, I512::from_i64(10));
    assert_eq!(decoded.lp_fee, I512::from_i64(11));
    assert!(
        crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolGetPoolDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolGetPoolDataResult::from_stack(&mut wrong).is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(81689, input, output)?;
    let wrapper = pool_curve_fi_stable::CoffeeCurveFiStablePoolContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_pool_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_pool_curve_fi_stable_estimate_swap_amount_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_tiny_int(2);
    let decoded = crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolEstimateSwapAmountResult::from_stack(
        &mut output.clone(),
    )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolEstimateSwapAmountResult::from_stack(&mut missing)
                .is_err()
        );
    }
    assert_eq!(decoded.input_amount, I512::from_i64(1));
    assert_eq!(decoded.output_amount, I512::from_i64(2));
    assert!(
        crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolEstimateSwapAmountResult::from_stack(&mut extra_stack(
            &output
        ))
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolEstimateSwapAmountResult::from_stack(&mut wrong)
                .is_err()
        );
    }
    let mut input = TVMStack::default();
    let asset: pool_constant_product::storage::CoffeePoolAsset =
        pool_constant_product::storage::CoffeePoolAsset::CoffeePoolExtraAsset(Box::new(
            pool_constant_product::storage::CoffeePoolExtraAsset { id: 0xf1234567 },
        ));
    let mut expected_asset = TonCell::builder();
    expected_asset.write_num(&2u8, 2)?;
    expected_asset.write_num(&0xf1234567u32, 32)?;
    input.push_cell_slice(expected_asset.build()?);
    let amount: I512 = I512::from_i64(2);
    input.push_int(amount);
    let (client, calls) = client(87113, input, output)?;
    let wrapper = pool_curve_fi_stable::CoffeeCurveFiStablePoolContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.estimate_swap_amount(&asset, &amount))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_pool_curve_fi_stable_estimate_liquidity_withdraw_amount_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_tiny_int(2);
    let decoded =
        crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolEstimateLiquidityWithdrawAmountResult::from_stack(
            &mut output.clone(),
        )?;
    assert_eq!(decoded.amount1, I512::from_i64(1));
    assert_eq!(decoded.amount2, I512::from_i64(2));
    assert!(
        crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolEstimateLiquidityWithdrawAmountResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolEstimateLiquidityWithdrawAmountResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let mut input = TVMStack::default();
    let lp_amount: I512 = I512::from_i64(1);
    input.push_int(lp_amount);
    let (client, calls) = client(114037, input, output)?;
    let wrapper = pool_curve_fi_stable::CoffeeCurveFiStablePoolContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.estimate_liquidity_withdraw_amount(&lp_amount))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_pool_curve_fi_stable_estimate_liquidity_deposit_amount_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_tiny_int(2);
    output.push_tiny_int(3);
    output.push_tiny_int(4);
    let decoded =
        crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolEstimateLiquidityDepositAmountResult::from_stack(
            &mut output.clone(),
        )?;
    assert_eq!(decoded.amount1_used, I512::from_i64(1));
    assert_eq!(decoded.amount2_used, I512::from_i64(2));
    assert_eq!(decoded.lp_amount, I512::from_i64(3));
    assert_eq!(decoded.lp_to_lock, I512::from_i64(4));
    assert!(
        crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolEstimateLiquidityDepositAmountResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::pool_curve_fi_stable::method_results::CoffeeCurveFiStablePoolEstimateLiquidityDepositAmountResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let mut input = TVMStack::default();
    let amount1: I512 = I512::from_i64(1);
    input.push_int(amount1);
    let amount2: I512 = I512::from_i64(2);
    input.push_int(amount2);
    let (client, calls) = client(113529, input, output)?;
    let wrapper = pool_curve_fi_stable::CoffeeCurveFiStablePoolContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.estimate_liquidity_deposit_amount(&amount1, &amount2))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_staking_item_get_stored_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    output.push_tiny_int(2);
    output.push_cell_slice(address_cell(3)?);
    output.push_cell_slice(none_address_cell()?);
    output.push_cell(cell_for("staking_item/CoffeeStakingAssetData/0")?);
    output.push_cell(cell_for("staking_item/CoffeeStakingPositionData/0")?);
    output.push(TVMNull {}.into());
    let decoded = crate::contracts::staking_item::method_results::CoffeeStakingItemGetStoredDataResult::from_stack(
        &mut output.clone(),
    )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::staking_item::method_results::CoffeeStakingItemGetStoredDataResult::from_stack(
                &mut missing
            )
            .is_err()
        );
    }
    assert!(decoded.initialized);
    assert_eq!(decoded.nft_id, 2);
    assert_eq!(
        decoded.master,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(3)?)?)?
    );
    assert!(decoded.owner.is_none());
    assert!(decoded.periods.is_empty());
    assert!(
        crate::contracts::staking_item::method_results::CoffeeStakingItemGetStoredDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::staking_item::method_results::CoffeeStakingItemGetStoredDataResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(125126, input, output)?;
    let wrapper = staking_item::CoffeeStakingItemContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_stored_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_staking_item_get_nft_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    output.push_tiny_int(2);
    output.push_cell_slice(address_cell(3)?);
    output.push_cell_slice(none_address_cell()?);
    output.push_cell(opaque_cell()?);
    let decoded = crate::contracts::staking_item::method_results::CoffeeStakingItemGetNftDataResult::from_stack(
        &mut output.clone(),
    )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::staking_item::method_results::CoffeeStakingItemGetNftDataResult::from_stack(&mut missing)
                .is_err()
        );
    }
    assert!(decoded.is_initialized);
    assert_eq!(decoded.item_index, I512::from_i64(2));
    assert_eq!(
        decoded.collection_address,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(3)?)?)?
    );
    assert!(decoded.owner_address.is_none());
    assert_eq!(decoded.content, opaque_cell()?);
    assert!(
        crate::contracts::staking_item::method_results::CoffeeStakingItemGetNftDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::staking_item::method_results::CoffeeStakingItemGetNftDataResult::from_stack(&mut wrong)
                .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(102351, input, output)?;
    let wrapper = staking_item::CoffeeStakingItemContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_nft_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_staking_master_get_stored_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    output.push_cell_slice(none_address_cell()?);
    output.push_cell_slice(none_address_cell()?);
    output.push_cell_slice(address_cell(4)?);
    output.push_cell(cell_for("staking_master/CoffeeStakingMasterState/0")?);
    output.push(TVMNull {}.into());
    output.push(TVMNull {}.into());
    output.push(TVMNull {}.into());
    let decoded = crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
        &mut output.clone(),
    )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
                &mut missing
            )
            .is_err()
        );
    }
    assert!(decoded.initialized);
    assert!(decoded.owner.is_none());
    assert!(decoded.distributor.is_none());
    assert_eq!(
        decoded.vault,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(4)?)?)?
    );
    assert!(decoded.rewards.is_empty());
    assert!(decoded.jettons.is_empty());
    assert!(decoded.periods.is_empty());
    assert!(
        crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(125126, input, output)?;
    let wrapper = staking_master::CoffeeStakingMasterContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_stored_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_staking_master_get_collection_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_cell(opaque_cell()?);
    output.push_cell_slice(none_address_cell()?);
    let decoded =
        crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
            &mut output.clone(),
        )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
                &mut missing
            )
            .is_err()
        );
    }
    assert_eq!(decoded.next_item_index, I512::from_i64(1));
    assert_eq!(decoded.collection_metadata, opaque_cell()?);
    assert!(decoded.owner_address.is_none());
    assert!(
        crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(102491, input, output)?;
    let wrapper = staking_master::CoffeeStakingMasterContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_collection_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_staking_master_get_nft_address_by_index_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(address_cell(1)?);
    let decoded = <MsgAddressInt>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<MsgAddressInt>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, MsgAddressInt::from_cell(&address_cell(1)?)?);
    assert!(<MsgAddressInt>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<MsgAddressInt>::from_stack(&mut wrong).is_err());
    }
    let mut input = TVMStack::default();
    let item_index: I512 = I512::from_i64(1);
    input.push_int(item_index);
    let (client, calls) = client(92067, input, output)?;
    let wrapper = staking_master::CoffeeStakingMasterContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_nft_address_by_index(&item_index))?;
    assert_eq!(result, ton::ton_core::types::TonAddress::from_msg_address(decoded)?);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_staking_master_get_nft_content_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell(opaque_cell()?);
    let decoded = <TonCell>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<TonCell>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, opaque_cell()?);
    assert!(<TonCell>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<TonCell>::from_stack(&mut wrong).is_err());
    }
    let mut input = TVMStack::default();
    let item_index: I512 = I512::from_i64(1);
    input.push_int(item_index);
    let individual_content: TonCell = opaque_cell()?;
    input.push_cell(individual_content.clone());
    let (client, calls) = client(68445, input, output)?;
    let wrapper = staking_master::CoffeeStakingMasterContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_nft_content(&item_index, &individual_content))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_staking_vault_get_stored_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    output.push_cell_slice(none_address_cell()?);
    output.push(TVMNull {}.into());
    output.push(TVMNull {}.into());
    output.push_cell(opaque_cell()?);
    output.push_cell(opaque_cell()?);
    let decoded = crate::contracts::staking_vault::method_results::CoffeeStakingVaultGetStoredDataResult::from_stack(
        &mut output.clone(),
    )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::staking_vault::method_results::CoffeeStakingVaultGetStoredDataResult::from_stack(
                &mut missing
            )
            .is_err()
        );
    }
    assert!(decoded.initialized);
    assert!(decoded.owner.is_none());
    assert!(decoded.jettons.is_empty());
    assert!(decoded.periods.is_empty());
    assert_eq!(decoded.master_code, opaque_cell()?);
    assert_eq!(decoded.item_code, opaque_cell()?);
    assert!(
        crate::contracts::staking_vault::method_results::CoffeeStakingVaultGetStoredDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::staking_vault::method_results::CoffeeStakingVaultGetStoredDataResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(125126, input, output)?;
    let wrapper = staking_vault::CoffeeStakingVaultContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_stored_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_staking_vault_get_master_address_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(address_cell(1)?);
    let decoded = <MsgAddressInt>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<MsgAddressInt>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, MsgAddressInt::from_cell(&address_cell(1)?)?);
    assert!(<MsgAddressInt>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<MsgAddressInt>::from_stack(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(115119, input, output)?;
    let wrapper = staking_vault::CoffeeStakingVaultContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_master_address())?;
    assert_eq!(result, ton::ton_core::types::TonAddress::from_msg_address(decoded)?);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_vault_extra_get_asset_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(opaque_cell()?);
    let decoded = <TonCell>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<TonCell>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, opaque_cell()?);
    assert!(<TonCell>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<TonCell>::from_stack(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(89352, input, output)?;
    let wrapper = vault_extra::CoffeeVaultExtraContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_asset())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_vault_extra_is_active_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    let decoded = <bool>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<bool>::from_stack(&mut missing).is_err());
    }
    assert!(decoded);
    assert!(<bool>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<bool>::from_stack(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(122058, input, output)?;
    let wrapper = vault_extra::CoffeeVaultExtraContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.is_active())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_vault_jetton_get_asset_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(opaque_cell()?);
    let decoded = <TonCell>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<TonCell>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, opaque_cell()?);
    assert!(<TonCell>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<TonCell>::from_stack(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(89352, input, output)?;
    let wrapper = vault_jetton::CoffeeVaultJettonContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_asset())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_vault_jetton_is_active_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    let decoded = <bool>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<bool>::from_stack(&mut missing).is_err());
    }
    assert!(decoded);
    assert!(<bool>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<bool>::from_stack(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(122058, input, output)?;
    let wrapper = vault_jetton::CoffeeVaultJettonContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.is_active())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_vault_native_get_asset_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(opaque_cell()?);
    let decoded = <TonCell>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<TonCell>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, opaque_cell()?);
    assert!(<TonCell>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<TonCell>::from_stack(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(89352, input, output)?;
    let wrapper = vault_native::CoffeeVaultNativeContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_asset())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_vault_native_is_active_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    let decoded = <bool>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<bool>::from_stack(&mut missing).is_err());
    }
    assert!(decoded);
    assert!(<bool>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<bool>::from_stack(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(122058, input, output)?;
    let wrapper = vault_native::CoffeeVaultNativeContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.is_active())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}

#[test]
fn test_deployment_cross_dex_get_admin_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(none_address_cell()?);
    let decoded = nullable_address(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(nullable_address(&mut missing).is_err());
    }
    assert!(decoded.is_none());
    assert!(nullable_address(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(nullable_address(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(113142, input, output)?;
    let wrapper = cross_dex::CoffeeCrossDexDeploymentContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_admin())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_deployment_cross_dex_bidask_get_admin_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(none_address_cell()?);
    let decoded = nullable_address(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(nullable_address(&mut missing).is_err());
    }
    assert!(decoded.is_none());
    assert!(nullable_address(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(nullable_address(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(113142, input, output)?;
    let wrapper = cross_dex_bidask::CoffeeCrossDexBidaskDeploymentContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_admin())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_deployment_staking_item_get_stored_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    output.push_tiny_int(2);
    output.push_cell_slice(address_cell(3)?);
    output.push_cell_slice(none_address_cell()?);
    output.push_cell(cell_for("staking_item/CoffeeStakingAssetData/0")?);
    output.push_cell(cell_for("staking_item/CoffeeStakingPositionData/0")?);
    output.push(TVMNull {}.into());
    let decoded = crate::contracts::staking_item::method_results::CoffeeStakingItemGetStoredDataResult::from_stack(
        &mut output.clone(),
    )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::staking_item::method_results::CoffeeStakingItemGetStoredDataResult::from_stack(
                &mut missing
            )
            .is_err()
        );
    }
    assert!(decoded.initialized);
    assert_eq!(decoded.nft_id, 2);
    assert_eq!(
        decoded.master,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(3)?)?)?
    );
    assert!(decoded.owner.is_none());
    assert!(decoded.periods.is_empty());
    assert!(
        crate::contracts::staking_item::method_results::CoffeeStakingItemGetStoredDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::staking_item::method_results::CoffeeStakingItemGetStoredDataResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(125126, input, output)?;
    let wrapper = staking_item::CoffeeStakingItemDeploymentContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_stored_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_deployment_staking_item_get_nft_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    output.push_tiny_int(2);
    output.push_cell_slice(address_cell(3)?);
    output.push_cell_slice(none_address_cell()?);
    output.push_cell(opaque_cell()?);
    let decoded = crate::contracts::staking_item::method_results::CoffeeStakingItemGetNftDataResult::from_stack(
        &mut output.clone(),
    )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::staking_item::method_results::CoffeeStakingItemGetNftDataResult::from_stack(&mut missing)
                .is_err()
        );
    }
    assert!(decoded.is_initialized);
    assert_eq!(decoded.item_index, I512::from_i64(2));
    assert_eq!(
        decoded.collection_address,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(3)?)?)?
    );
    assert!(decoded.owner_address.is_none());
    assert_eq!(decoded.content, opaque_cell()?);
    assert!(
        crate::contracts::staking_item::method_results::CoffeeStakingItemGetNftDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::staking_item::method_results::CoffeeStakingItemGetNftDataResult::from_stack(&mut wrong)
                .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(102351, input, output)?;
    let wrapper = staking_item::CoffeeStakingItemDeploymentContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_nft_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_deployment_staking_master_get_stored_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    output.push_cell_slice(none_address_cell()?);
    output.push_cell_slice(none_address_cell()?);
    output.push_cell_slice(address_cell(4)?);
    output.push_cell(cell_for("staking_master/CoffeeStakingMasterState/0")?);
    output.push(TVMNull {}.into());
    output.push(TVMNull {}.into());
    output.push(TVMNull {}.into());
    let decoded = crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
        &mut output.clone(),
    )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
                &mut missing
            )
            .is_err()
        );
    }
    assert!(decoded.initialized);
    assert!(decoded.owner.is_none());
    assert!(decoded.distributor.is_none());
    assert_eq!(
        decoded.vault,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(4)?)?)?
    );
    assert!(decoded.rewards.is_empty());
    assert!(decoded.jettons.is_empty());
    assert!(decoded.periods.is_empty());
    assert!(
        crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(125126, input, output)?;
    let wrapper = staking_master::CoffeeStakingMasterDeploymentContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_stored_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_deployment_staking_master_get_collection_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_cell(opaque_cell()?);
    output.push_cell_slice(none_address_cell()?);
    let decoded =
        crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
            &mut output.clone(),
        )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
                &mut missing
            )
            .is_err()
        );
    }
    assert_eq!(decoded.next_item_index, I512::from_i64(1));
    assert_eq!(decoded.collection_metadata, opaque_cell()?);
    assert!(decoded.owner_address.is_none());
    assert!(
        crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(102491, input, output)?;
    let wrapper = staking_master::CoffeeStakingMasterDeploymentContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_collection_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_deployment_staking_master_get_nft_address_by_index_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(address_cell(1)?);
    let decoded = <MsgAddressInt>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<MsgAddressInt>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, MsgAddressInt::from_cell(&address_cell(1)?)?);
    assert!(<MsgAddressInt>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<MsgAddressInt>::from_stack(&mut wrong).is_err());
    }
    let mut input = TVMStack::default();
    let item_index: I512 = I512::from_i64(1);
    input.push_int(item_index);
    let (client, calls) = client(92067, input, output)?;
    let wrapper = staking_master::CoffeeStakingMasterDeploymentContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_nft_address_by_index(&item_index))?;
    assert_eq!(result, ton::ton_core::types::TonAddress::from_msg_address(decoded)?);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_deployment_staking_master_get_nft_content_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell(opaque_cell()?);
    let decoded = <TonCell>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<TonCell>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, opaque_cell()?);
    assert!(<TonCell>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<TonCell>::from_stack(&mut wrong).is_err());
    }
    let mut input = TVMStack::default();
    let item_index: I512 = I512::from_i64(1);
    input.push_int(item_index);
    let individual_content: TonCell = opaque_cell()?;
    input.push_cell(individual_content.clone());
    let (client, calls) = client(68445, input, output)?;
    let wrapper = staking_master::CoffeeStakingMasterDeploymentContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_nft_content(&item_index, &individual_content))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_deployment_vault_jetton_get_asset_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(opaque_cell()?);
    let decoded = <TonCell>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<TonCell>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, opaque_cell()?);
    assert!(<TonCell>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<TonCell>::from_stack(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(89352, input, output)?;
    let wrapper = vault_jetton::CoffeeVaultJettonDeploymentContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_asset())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_deployment_vault_jetton_is_active_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    let decoded = <bool>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<bool>::from_stack(&mut missing).is_err());
    }
    assert!(decoded);
    assert!(<bool>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<bool>::from_stack(&mut wrong).is_err());
    }
    let input = TVMStack::default();
    let (client, calls) = client(122058, input, output)?;
    let wrapper = vault_jetton::CoffeeVaultJettonDeploymentContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.is_active())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_single_item_master_get_stored_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    output.push_cell_slice(none_address_cell()?);
    output.push_cell_slice(none_address_cell()?);
    output.push_cell_slice(address_cell(4)?);
    output.push_cell(cell_for("staking_master/CoffeeStakingMasterState/0")?);
    output.push(TVMNull {}.into());
    output.push(TVMNull {}.into());
    output.push(TVMNull {}.into());
    let decoded = crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
        &mut output.clone(),
    )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
                &mut missing
            )
            .is_err()
        );
    }
    assert!(decoded.initialized);
    assert!(decoded.owner.is_none());
    assert!(decoded.distributor.is_none());
    assert_eq!(
        decoded.vault,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(4)?)?)?
    );
    assert!(decoded.rewards.is_empty());
    assert!(decoded.jettons.is_empty());
    assert!(decoded.periods.is_empty());
    assert!(
        crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(125126, input, output)?;
    let wrapper =
        staking_master::single_item::CoffeeStakingMasterSingleItemContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_stored_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_single_item_master_get_collection_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_cell(opaque_cell()?);
    output.push_cell_slice(none_address_cell()?);
    let decoded =
        crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
            &mut output.clone(),
        )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
                &mut missing
            )
            .is_err()
        );
    }
    assert_eq!(decoded.next_item_index, I512::from_i64(1));
    assert_eq!(decoded.collection_metadata, opaque_cell()?);
    assert!(decoded.owner_address.is_none());
    assert!(
        crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(102491, input, output)?;
    let wrapper =
        staking_master::single_item::CoffeeStakingMasterSingleItemContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_collection_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_single_item_master_get_nft_address_by_index_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(address_cell(1)?);
    let decoded = <MsgAddressInt>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<MsgAddressInt>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, MsgAddressInt::from_cell(&address_cell(1)?)?);
    assert!(<MsgAddressInt>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<MsgAddressInt>::from_stack(&mut wrong).is_err());
    }
    let mut input = TVMStack::default();
    let item_index: I512 = I512::from_i64(1);
    input.push_int(item_index);
    let (client, calls) = client(92067, input, output)?;
    let wrapper =
        staking_master::single_item::CoffeeStakingMasterSingleItemContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_nft_address_by_index(&item_index))?;
    assert_eq!(result, ton::ton_core::types::TonAddress::from_msg_address(decoded)?);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_single_item_master_get_nft_content_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell(opaque_cell()?);
    let decoded = <TonCell>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<TonCell>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, opaque_cell()?);
    assert!(<TonCell>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<TonCell>::from_stack(&mut wrong).is_err());
    }
    let mut input = TVMStack::default();
    let item_index: I512 = I512::from_i64(1);
    input.push_int(item_index);
    let individual_content: TonCell = opaque_cell()?;
    input.push_cell(individual_content.clone());
    let (client, calls) = client(68445, input, output)?;
    let wrapper =
        staking_master::single_item::CoffeeStakingMasterSingleItemContract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_nft_content(&item_index, &individual_content))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_single_item_deployment_master_get_stored_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(-1);
    output.push_cell_slice(none_address_cell()?);
    output.push_cell_slice(none_address_cell()?);
    output.push_cell_slice(address_cell(4)?);
    output.push_cell(cell_for("staking_master/CoffeeStakingMasterState/0")?);
    output.push(TVMNull {}.into());
    output.push(TVMNull {}.into());
    output.push(TVMNull {}.into());
    let decoded = crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
        &mut output.clone(),
    )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
                &mut missing
            )
            .is_err()
        );
    }
    assert!(decoded.initialized);
    assert!(decoded.owner.is_none());
    assert!(decoded.distributor.is_none());
    assert_eq!(
        decoded.vault,
        ton::ton_core::types::TonAddress::from_msg_address(MsgAddressInt::from_cell(&address_cell(4)?)?)?
    );
    assert!(decoded.rewards.is_empty());
    assert!(decoded.jettons.is_empty());
    assert!(decoded.periods.is_empty());
    assert!(
        crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(125126, input, output)?;
    let wrapper = staking_master::single_item::CoffeeStakingMasterSingleItemDeploymentContract::new(
        &client,
        &TonAddress::ZERO,
        None,
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_stored_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_single_item_deployment_master_get_collection_data_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_tiny_int(1);
    output.push_cell(opaque_cell()?);
    output.push_cell_slice(none_address_cell()?);
    let decoded =
        crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
            &mut output.clone(),
        )?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
                &mut missing
            )
            .is_err()
        );
    }
    assert_eq!(decoded.next_item_index, I512::from_i64(1));
    assert_eq!(decoded.collection_metadata, opaque_cell()?);
    assert!(decoded.owner_address.is_none());
    assert!(
        crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
            &mut extra_stack(&output)
        )
        .is_err()
    );
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(
            crate::contracts::staking_master::method_results::CoffeeStakingMasterGetCollectionDataResult::from_stack(
                &mut wrong
            )
            .is_err()
        );
    }
    let input = TVMStack::default();
    let (client, calls) = client(102491, input, output)?;
    let wrapper = staking_master::single_item::CoffeeStakingMasterSingleItemDeploymentContract::new(
        &client,
        &TonAddress::ZERO,
        None,
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_collection_data())?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_single_item_deployment_master_get_nft_address_by_index_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell_slice(address_cell(1)?);
    let decoded = <MsgAddressInt>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<MsgAddressInt>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, MsgAddressInt::from_cell(&address_cell(1)?)?);
    assert!(<MsgAddressInt>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<MsgAddressInt>::from_stack(&mut wrong).is_err());
    }
    let mut input = TVMStack::default();
    let item_index: I512 = I512::from_i64(1);
    input.push_int(item_index);
    let (client, calls) = client(92067, input, output)?;
    let wrapper = staking_master::single_item::CoffeeStakingMasterSingleItemDeploymentContract::new(
        &client,
        &TonAddress::ZERO,
        None,
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_nft_address_by_index(&item_index))?;
    assert_eq!(result, ton::ton_core::types::TonAddress::from_msg_address(decoded)?);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
#[test]
fn test_single_item_deployment_master_get_nft_content_stack_and_request() -> anyhow::Result<()> {
    let mut output = TVMStack::default();
    output.push_cell(opaque_cell()?);
    let decoded = <TonCell>::from_stack(&mut output.clone())?;
    for remaining in 0..output.len() {
        let mut missing = output.clone();
        missing.truncate(remaining);
        assert!(<TonCell>::from_stack(&mut missing).is_err());
    }
    assert_eq!(decoded, opaque_cell()?);
    assert!(<TonCell>::from_stack(&mut extra_stack(&output)).is_ok());
    for index in 0..output.len() {
        let mut wrong = output.clone();
        wrong[index] = wrong_type(&output[index])?;
        assert!(<TonCell>::from_stack(&mut wrong).is_err());
    }
    let mut input = TVMStack::default();
    let item_index: I512 = I512::from_i64(1);
    input.push_int(item_index);
    let individual_content: TonCell = opaque_cell()?;
    input.push_cell(individual_content.clone());
    let (client, calls) = client(68445, input, output)?;
    let wrapper = staking_master::single_item::CoffeeStakingMasterSingleItemDeploymentContract::new(
        &client,
        &TonAddress::ZERO,
        None,
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = tokio_test::block_on(wrapper.get_nft_content(&item_index, &individual_content))?;
    assert_eq!(result, decoded);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}
