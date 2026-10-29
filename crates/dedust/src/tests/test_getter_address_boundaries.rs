//! Address tails and NULL handling follow each getter's address contract.
use super::test_getter_provider::{fixture_arguments, recording_client};
use crate::contracts::classic::factory_v1::DedustFactoryV1Contract;
use crate::contracts::classic::factory_v2::DedustFactoryV2Contract;
use crate::contracts::uranus::meme_v3::DedustUranusMemeV3Contract;
use ton::{
    block_tlb::{TVMNull, TVMStack, TVMStackValue},
    contracts::TonContract,
    ton_core::{
        cell::TonCell,
        traits::tlb::TLB,
        types::{TonAddress, tlb_core::MsgAddress},
    },
};

#[test]
fn test_stack_address_getter_preserves_zero_with_trailing_bits_and_refs() -> anyhow::Result<()> {
    let address = MsgAddress::Int(TonAddress::ZERO.to_msg_address_int()).to_cell()?;
    for as_slice in [true, false] {
        let mut stack = TVMStack::EMPTY;
        if as_slice {
            stack.push_cell_slice(address.clone());
        } else {
            stack.push_cell(address.clone());
        }
        let _ = tokio_test::block_on(
            DedustUranusMemeV3Contract::new(
                &recording_client(103289, fixture_arguments(&["owner"])?, stack)?,
                &TonAddress::ZERO,
                None,
            )
            .get_wallet_address(TonAddress::ZERO),
        )?;
        for extra_ref in [false, true] {
            let mut malformed = TonCell::builder();
            malformed.write_cell(&address)?;
            if extra_ref {
                malformed.write_ref(TonCell::empty().clone())?;
            } else {
                malformed.write_bit(true)?;
            }
            let mut stack = TVMStack::EMPTY;
            if as_slice {
                stack.push_cell_slice(malformed.build()?);
            } else {
                stack.push_cell(malformed.build()?);
            }
            let decoded = tokio_test::block_on(
                DedustUranusMemeV3Contract::new(
                    &recording_client(103289, fixture_arguments(&["owner"])?, stack)?,
                    &TonAddress::ZERO,
                    None,
                )
                .get_wallet_address(TonAddress::ZERO),
            )?;
            assert_eq!(decoded, TonAddress::ZERO);
        }
    }
    Ok(())
}

#[test]
fn test_native_vault_getters_use_native_result_decoding() -> anyhow::Result<()> {
    use crate::contracts::classic::vault_native_v1::DedustVaultNativeV1Contract;
    for null in [false, true] {
        let mut stack = TVMStack::EMPTY;
        if null {
            stack.push(TVMStackValue::Null(TVMNull));
        } else {
            stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
        }
        let client = recording_client(89995, TVMStack::EMPTY, stack)?;
        let contract = DedustVaultNativeV1Contract::new(&client, &TonAddress::ZERO, None);
        assert_eq!(tokio_test::block_on(contract.get_factory_addr())?, TonAddress::ZERO);
    }
    // The named scalar getter consumes its result without whole-stack exhaustion.
    let mut stack = TVMStack::EMPTY;
    stack.push(TVMStackValue::Null(TVMNull));
    stack.push_tiny_int(17);
    let client = recording_client(82320, TVMStack::EMPTY, stack)?;
    let contract = DedustVaultNativeV1Contract::new(&client, &TonAddress::ZERO, None);
    assert_eq!(tokio_test::block_on(contract.get_version())?, 17);
    Ok(())
}

#[test]
fn test_factory_ownership_uses_nullable_cell_adapter() -> anyhow::Result<()> {
    use crate::contracts::classic::{factory_v1, factory_v2};
    use ton::block_tlb::FromTVMStack;
    let none = MsgAddress::NONE.to_cell()?;
    let zero = MsgAddress::Int(TonAddress::ZERO.to_msg_address_int()).to_cell()?;
    for as_slice in [false, true] {
        for owner_absent in [false, true] {
            let mut stack = TVMStack::EMPTY;
            for absent in [owner_absent, !owner_absent] {
                let mut cell = TonCell::builder();
                cell.write_cell(if absent { &none } else { &zero })?;
                cell.write_bit(true)?;
                cell.write_ref(TonCell::empty().clone())?;
                if as_slice {
                    stack.push_cell_slice(cell.build()?);
                } else {
                    stack.push_cell(cell.build()?);
                }
            }
            stack.push_int(123u32.into());
            let client = recording_client(110004, TVMStack::EMPTY, stack.clone())?;
            let v1 =
                tokio_test::block_on(DedustFactoryV1Contract::new(&client, &TonAddress::ZERO, None).get_ownership())?;
            let client = recording_client(110004, TVMStack::EMPTY, stack)?;
            let v2 =
                tokio_test::block_on(DedustFactoryV2Contract::new(&client, &TonAddress::ZERO, None).get_ownership())?;
            for (owner, pending, timestamp) in [
                (v1.owner_address, v1.pending_owner_address, v1.can_accept_after),
                (v2.owner_address, v2.pending_owner_address, v2.can_accept_after),
            ] {
                assert_eq!(owner, (!owner_absent).then_some(TonAddress::ZERO));
                assert_eq!(pending, owner_absent.then_some(TonAddress::ZERO));
                assert_eq!(timestamp, 123);
            }
        }
    }
    for null_index in 0..2 {
        let mut stack = TVMStack::EMPTY;
        for index in 0..2 {
            if index == null_index {
                stack.push(TVMStackValue::Null(TVMNull));
            } else {
                stack.push_cell_slice(none.clone());
            }
        }
        stack.push_tiny_int(123);
        assert!(factory_v1::method_results::DedustFactoryV1GetOwnershipResult::from_stack(&mut stack.clone()).is_err());
        assert!(factory_v2::method_results::DedustFactoryV2GetOwnershipResult::from_stack(&mut stack).is_err());
    }
    Ok(())
}
