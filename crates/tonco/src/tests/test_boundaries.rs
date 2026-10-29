//! Native parsing policy and protocol-specific validation boundaries.
use crate::contracts::{native, v1, v2};
use fastnum::I512;
use ton::{
    block_tlb::{FromTVMStack, TVMNull, TVMStack, TVMStackValue, TVMTuple},
    ton_core::{
        cell::TonCell,
        traits::tlb::TLB,
        types::{TonAddress, tlb_core::MsgAddress},
    },
};

#[test]
fn test_router_optional_addresses_and_native_leaf_tails() -> anyhow::Result<()> {
    use v2::router::method_results::{ToncoRouterV2GetPendingTimelocksResult, ToncoRouterV2GetRouterStateResult};
    let zero = MsgAddress::Int(TonAddress::ZERO.to_msg_address_int());
    for expected in [None, Some(MsgAddress::NONE), Some(zero.clone())] {
        for as_slice in [false, true] {
            let mut stack = TVMStack::default();
            for _ in 0..3 {
                stack.push_cell_slice(zero.to_cell()?);
            }
            match &expected {
                None => stack.push(TVMStackValue::Null(TVMNull)),
                Some(address) => {
                    let mut b = TonCell::builder();
                    b.write_cell(&address.to_cell()?)?;
                    b.write_bit(true)?;
                    b.write_ref(TonCell::empty().clone())?;
                    let cell = b.build()?;
                    if as_slice {
                        stack.push_cell_slice(cell);
                    } else {
                        stack.push_cell(cell);
                    }
                },
            }
            stack.push_tiny_int(0);
            stack.push_tiny_int(0);
            stack.push_tiny_int(16000);
            assert_eq!(
                ToncoRouterV2GetRouterStateResult::from_stack(&mut stack)?.proxy,
                (expected.clone())
                    .map(tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address)
                    .transpose()?
            );
        }
    }
    let mut stack = TVMStack::default();
    for value in [
        Some(1000),
        Some(0),
        None,
        Some(0),
        None,
        Some(0),
        Some(0),
        Some(0),
        None,
        Some(0),
        None,
    ] {
        match value {
            Some(n) => stack.push_tiny_int(n),
            None => stack.push(TVMStackValue::Null(TVMNull)),
        }
    }
    let result = ToncoRouterV2GetPendingTimelocksResult::from_stack(&mut stack)?;
    assert!(result.admin.is_none() && result.proxy.is_none() && result.code.is_none());
    Ok(())
}

#[test]
fn test_getter_native_scalars_and_structured_arity() -> anyhow::Result<()> {
    // Mandatory raw address results use TON's NULL-to-addr_none conversion.
    let mut null = TVMStack::default();
    null.push(TVMStackValue::Null(TVMNull));
    assert_eq!(MsgAddress::from_stack(&mut null)?, MsgAddress::NONE);
    // The pool's scalar maximum getter consumes one value, permitting stack tails.
    let mut scalar = TVMStack::default();
    scalar.push_tiny_int(9);
    scalar.push_tiny_int(27);
    assert_eq!(I512::from_stack(&mut scalar)?, I512::from_i64(27));
    assert_eq!(scalar.len(), 1);
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let valid = super::test_getter_provider::stack(&fixtures["rows"][44]["stack"])?;
    for malformed in 0..3 {
        let mut stack = valid.clone();
        match malformed {
            0 => {
                stack.pop_checked()?;
            },
            1 => stack.push_tiny_int(99),
            _ => {
                stack.pop_checked()?;
                stack.push_cell(TonCell::empty().clone());
            },
        }
        assert!(v2::router::method_results::ToncoRouterV2GetRouterStateResult::from_stack(&mut stack).is_err());
    }
    Ok(())
}

#[test]
fn test_getter_cells_accept_cell_and_slice_preserve_null() -> anyhow::Result<()> {
    use native::minter::method_results::ToncoNativeMinterGetJettonDataResult;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    for as_slice in [false, true] {
        let mut fields = super::test_getter_provider::stack(&fixture["rows"][59]["stack"])?;
        fields.pop_checked()?;
        if as_slice {
            fields.push_cell_slice(TonCell::empty().clone());
        } else {
            fields.push_cell(TonCell::empty().clone());
        }
        assert_eq!(
            ToncoNativeMinterGetJettonDataResult::from_stack(&mut fields)?.wallet_code,
            *TonCell::empty()
        );
        let mut optional = TVMStack::default();
        if as_slice {
            optional.push_cell_slice(TonCell::empty().clone());
        } else {
            optional.push_cell(TonCell::empty().clone());
        }
        assert_eq!(Option::<TonCell>::from_stack(&mut optional)?, Some(TonCell::empty().clone()));
    }
    let mut stack = TVMStack::default();
    stack.push(TVMStackValue::Null(TVMNull));
    assert_eq!(Option::<TonCell>::from_stack(&mut stack)?, None);
    Ok(())
}

fn ticks(values: &[i64]) -> TVMStack {
    let mut fields = TVMTuple::default();
    for v in values {
        fields.push_tiny_int(*v);
    }
    let mut group = TVMTuple::default();
    group.push_tuple(fields);
    let mut groups = TVMTuple::default();
    groups.push_tuple(group);
    let mut stack = TVMStack::default();
    stack.push_tuple(groups);
    stack
}

#[test]
fn test_tick_page_enforces_nested_shape_width_domain_and_stack_arity() -> anyhow::Result<()> {
    use v1::pool::method_results::ToncoPoolTickPage;
    for values in [&[-887272, 100, -9][..], &[887272, 100, -9, 7, 8][..]] {
        assert_eq!(
            ToncoPoolTickPage::from_stack(&mut ticks(values))?.entries[0].index,
            values[0] as i32
        );
    }
    for values in [
        &[1, 2][..],
        &[1, 2, 3, 4][..],
        &[1, 2, 3, 4, 5, 6][..],
        &[887273, 1, 2][..],
        &[-887273, 1, 2][..],
    ] {
        assert!(ToncoPoolTickPage::from_stack(&mut ticks(values)).is_err());
    }
    let mut stack = ticks(&[1, 2, 3]);
    stack.push_tiny_int(0);
    assert!(ToncoPoolTickPage::from_stack(&mut stack).is_err());
    let mut malformed = TVMStack::default();
    malformed.push_tiny_int(1);
    assert!(ToncoPoolTickPage::from_stack(&mut malformed).is_err());
    let mut null = TVMStack::default();
    null.push(TVMStackValue::Null(TVMNull));
    assert!(ToncoPoolTickPage::from_stack(&mut null)?.entries.is_empty());
    Ok(())
}

#[test]
fn test_native_wallet_storage_requires_standard_internal_addresses() -> anyhow::Result<()> {
    use native::wallet::storage::ToncoNativeWalletStorage;
    let stored = ToncoNativeWalletStorage::from_boc(include_bytes!("fixtures/native/synthetic_wallet.boc").to_vec())?;
    let mut invalid = stored.clone();
    invalid.owner = TonAddress::new(128, [1; 32].into());
    assert!(invalid.to_cell().is_err());
    let mut none = TonCell::builder();
    none.write_num(&0u8, 4)?; // zero coins
    none.write_cell(&MsgAddress::NONE.to_cell()?)?;
    none.write_cell(&MsgAddress::Int(stored.minter.to_msg_address_int()).to_cell()?)?;
    assert!(ToncoNativeWalletStorage::from_cell(&none.build()?).is_err());
    Ok(())
}
