//! Native parsing at STONks-owned structured and scalar boundaries.
use super::test_getter_provider::{bonding_stack, fixture_stack};
use crate::contracts::dedust_wallet::method_results::StonksDedustWalletGetWalletDataResult;
use ton::{
    block_tlb::{FromTVMStack, TVMNull, TVMStack, TVMStackValue},
    ton_core::{
        cell::TonCell,
        traits::tlb::TLB,
        types::{TonAddress, tlb_core::MsgAddress},
    },
};

#[test]
fn test_wallet_result_accepts_cell_slice_and_address_tails() -> anyhow::Result<()> {
    let zero = MsgAddress::Int(TonAddress::ZERO.to_msg_address_int());
    for as_slice in [false, true] {
        for extra_ref in [false, true] {
            let mut address = TonCell::builder();
            address.write_cell(&zero.to_cell()?)?;
            if extra_ref {
                address.write_ref(TonCell::empty().clone())?;
            } else {
                address.write_bit(true)?;
            }
            let address = address.build()?;
            let mut stack = TVMStack::EMPTY;
            stack.push_tiny_int(123);
            stack.push_cell_slice(address.clone());
            stack.push_cell(address);
            if as_slice {
                stack.push_cell_slice(TonCell::empty().clone());
            } else {
                stack.push_cell(TonCell::empty().clone());
            }
            let result = StonksDedustWalletGetWalletDataResult::from_stack(&mut stack)?;
            assert_eq!(
                result.owner,
                tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(zero.clone())?
            );
            assert_eq!(
                result.master,
                tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(zero.clone())?
            );
            assert_eq!(result.code, *TonCell::empty());
        }
    }
    Ok(())
}

#[test]
fn test_wallet_result_rejects_null_and_distinguishes_addr_none_from_zero() -> anyhow::Result<()> {
    for (value, expected) in [
        (TVMStackValue::Null(TVMNull), MsgAddress::NONE),
        (
            {
                let mut s = TVMStack::EMPTY;
                s.push_cell_slice(MsgAddress::NONE.to_cell()?);
                s.pop_checked()?
            },
            MsgAddress::NONE,
        ),
        (
            {
                let mut s = TVMStack::EMPTY;
                s.push_cell_slice(MsgAddress::Int(TonAddress::ZERO.to_msg_address_int()).to_cell()?);
                s.pop_checked()?
            },
            MsgAddress::Int(TonAddress::ZERO.to_msg_address_int()),
        ),
    ] {
        let mut stack = TVMStack::EMPTY;
        stack.push_tiny_int(123);
        stack.push(value.clone());
        let is_null = matches!(&value, TVMStackValue::Null(_));
        stack.push(value);
        stack.push_cell(TonCell::empty().clone());
        let result = StonksDedustWalletGetWalletDataResult::from_stack(&mut stack);
        if is_null {
            assert!(result.is_err());
            continue;
        }
        let result = result?;
        assert_eq!(
            result.owner,
            tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(expected.clone())?
        );
        assert_eq!(
            result.master,
            tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(expected.clone())?
        );
    }
    // Raw code accepts Cell/Slice but has no optional-NULL contract.
    let mut stack = fixture_stack(6)?;
    stack.pop_checked()?;
    stack.push(TVMStackValue::Null(TVMNull));
    assert!(StonksDedustWalletGetWalletDataResult::from_stack(&mut stack).is_err());
    Ok(())
}

fn invalid_shapes<T: FromTVMStack>(original: TVMStack) -> anyhow::Result<()> {
    let mut missing = original.clone();
    missing.pop_checked()?;
    assert!(T::from_stack(&mut missing).is_err());
    let mut extra = TVMStack::EMPTY;
    extra.push_tiny_int(999);
    for v in original.iter() {
        extra.push(v.clone());
    }
    assert!(T::from_stack(&mut extra).is_err());
    // A tuple is invalid in each primitive/address/cell position in these DTOs.
    for index in 0..original.iter().count() {
        let mut wrong = original.clone();
        *wrong.get_mut(index).ok_or_else(|| anyhow::anyhow!("field"))? = TVMStackValue::Tuple(Default::default());
        assert!(T::from_stack(&mut wrong).is_err());
    }
    Ok(())
}

#[test]
fn test_every_structured_result_enforces_arity_and_field_types() -> anyhow::Result<()> {
    use crate::contracts;
    invalid_shapes::<contracts::ston_minter::method_results::StonksStonMinterGetExtendedDataResult>(fixture_stack(0)?)?;
    invalid_shapes::<contracts::ston_minter::method_results::StonksStonMinterGetStonDataResult>(fixture_stack(1)?)?;
    invalid_shapes::<contracts::ston_minter::method_results::StonksStonMinterGetProtocolAddressesResult>(
        fixture_stack(2)?,
    )?;
    invalid_shapes::<contracts::dedust_minter_v12::method_results::StonksDedustMinterV12GetFeeDataResult>(
        fixture_stack(3)?,
    )?;
    invalid_shapes::<contracts::dedust_minter_v12::method_results::StonksDedustMinterV12GetExtendedDataResult>(
        fixture_stack(4)?,
    )?;
    invalid_shapes::<contracts::ston_minter::method_results::StonksStonMinterGetJettonDataResult>(fixture_stack(5)?)?;
    invalid_shapes::<StonksDedustWalletGetWalletDataResult>(fixture_stack(6)?)?;
    invalid_shapes::<contracts::dedust_wallet::method_results::StonksDedustWalletGetExtendedDataResult>(
        fixture_stack(7)?,
    )?;
    invalid_shapes::<contracts::ston_wallet::method_results::StonksStonWalletGetExtendedDataResult>(fixture_stack(8)?)?;
    invalid_shapes::<contracts::ston_factory::method_results::StonksStonFactoryGetFactoryDataResult>(fixture_stack(
        9,
    )?)?;
    invalid_shapes::<contracts::dedust_factory::method_results::StonksDedustFactoryGetFactoryDataResult>(
        fixture_stack(10)?,
    )?;
    invalid_shapes::<contracts::custom_wallet::method_results::StonksCustomWalletGetTaxResult>(fixture_stack(11)?)?;
    invalid_shapes::<contracts::custom_wallet::method_results::StonksCustomWalletGetWalletDataResult>(fixture_stack(
        12,
    )?)?;
    invalid_shapes::<contracts::ston_minter::method_results::StonksStonMinterGetBondingDataResult>(bonding_stack(
        false,
    )?)?;
    invalid_shapes::<contracts::dedust_minter_v12::method_results::StonksDedustMinterV12GetBondingDataResult>(
        bonding_stack(true)?,
    )?;
    Ok(())
}

#[test]
fn test_external_address_is_rejected_by_standard_wallet_getter() -> anyhow::Result<()> {
    // addr_extern$01, length 8, value 0xab: independently specified raw bits.
    let mut b = TonCell::builder();
    b.write_num(&1_u8, 2)?;
    b.write_num(&8_u16, 9)?;
    b.write_num(&0xab_u8, 8)?;
    let address = b.build()?;
    let mut stack = TVMStack::EMPTY;
    stack.push_tiny_int(1);
    stack.push_cell_slice(address.clone());
    stack.push_cell(address.clone());
    stack.push_cell(TonCell::empty().clone());
    assert!(StonksDedustWalletGetWalletDataResult::from_stack(&mut stack).is_err());
    Ok(())
}
