use crate::contracts::{
    address, booster, catalog_range, clmm, cpmm, legacy_pool, mempad, order_factory, order_new, order_old,
};
use fastnum::U256;
use std::collections::HashMap;
use ton::ton_core::cell::TonHash;
use ton::ton_core::types::tlb_core::MsgAddressIntStd;
use ton::ton_core::types::tlb_core::TLBEither;
use ton::{
    block_tlb::{FromTVMStack, TVMStack},
    ton_core::{
        cell::TonCell,
        traits::tlb::TLB,
        types::{
            TonAddress,
            tlb_core::{TLBCoins, TLBRef},
        },
    },
};
#[test]
fn test_catalog_synthetic_storage_and_version_separation() -> anyhow::Result<()> {
    let old = order_old::storage::MoonOrderOldStorage::from_boc(
        include_bytes!("fixtures/synthetic-MoonOrderOld.data.boc").to_vec(),
    )?;
    assert_eq!(old.status, 1);
    assert_eq!(old.assets.asset1, None);
    assert_eq!(old.wallets.asset1_wallet, None);
    let new = order_new::storage::MoonOrderNewStorage::from_boc(
        include_bytes!("fixtures/synthetic-MoonOrderNew.data.boc").to_vec(),
    )?;
    assert_eq!(new.status, 0);
    assert_eq!(new.remaining_amount, TLBCoins::ZERO);
    let pool = legacy_pool::storage::MoonLegacyPoolStorage::from_boc(
        include_bytes!("fixtures/synthetic-MoonPool.data.boc").to_vec(),
    )?;
    assert_eq!(pool.asset1_wallet, None);
    assert_ne!(pool.asset2_wallet, None);
    assert!(
        cpmm::pool::storage::MoonCpmmPoolStorage::from_boc(
            include_bytes!("fixtures/synthetic-MoonPool.data.boc").to_vec()
        )
        .is_err()
    );
    let _ = catalog_range::storage::MoonCatalogClmmRangeStorage::from_boc(
        include_bytes!("fixtures/synthetic-MoonClmmRange.data.boc").to_vec(),
    )?;
    let _ = order_factory::storage::MoonOrderFactoryStorage::from_boc(
        include_bytes!("fixtures/synthetic-MoonOrderFactory.data.boc").to_vec(),
    )?;
    let _ = booster::storage::MoonBoosterStorage::from_boc(
        include_bytes!("fixtures/synthetic-MoonBooster.data.boc").to_vec(),
    )?;
    Ok(())
}
#[test]
fn test_launch_synthetic_state_from_recorded_economics() -> anyhow::Result<()> {
    let buy = mempad::curve::storage::MoonMempadCurveStorage::from_boc(
        include_bytes!("fixtures/launch-after-buy.boc").to_vec(),
    )?;
    assert_eq!(buy.state.value(), 1);
    assert_eq!(buy.trading.gram_reserve.to_u128(), 99_400_000);
    assert_eq!(buy.trading.supply.to_u128(), 90_412_952_519);
    assert_eq!(buy.params.econ.creator_fee_bps, 50);
    assert_eq!(buy.grad.state.value(), 0);
    assert_eq!(buy.grad.pool, None);
    let grad = mempad::curve::storage::MoonMempadCurveStorage::from_boc(
        include_bytes!("fixtures/launch-after-graduation.boc").to_vec(),
    )?;
    assert_eq!(grad.state.value(), 3);
    assert_eq!(grad.trading.gram_reserve, TLBCoins::ZERO);
    assert_eq!(grad.trading.supply.to_u128(), 1_000_000_000_000);
    assert_eq!(grad.trading.platform_fees, TLBCoins::ZERO);
    assert_eq!(grad.grad.state.value(), 4);
    assert_ne!(grad.grad.pool, None);
    Ok(())
}
#[test]
fn test_known_malformed_unknown_and_ordinary_tails() -> anyhow::Result<()> {
    // Independently written wire bytes: opcode + query 1 + coins(2), no refs.
    let mut raw = TonCell::builder();
    raw.write_num(&0x29d1f1c1u32, 32)?;
    raw.write_num(&1u64, 64)?;
    raw.write_num(&1u8, 4)?;
    raw.write_num(&2u8, 8)?;
    let cell = raw.build()?;
    let decoded = mempad::msgs::MoonMempadBuyMsg::from_cell(&cell)?;
    assert_eq!(decoded.query_id, 1);
    assert_eq!(decoded.min_out.to_u128(), 2);
    let mut extra = TonCell::builder();
    extra.write_cell(&cell)?;
    extra.write_bit(false)?;
    assert!(mempad::msgs::MoonMempadMsg::from_cell(&extra.build()?).is_ok());
    let mut short = TonCell::builder();
    short.write_num(&0x29d1f1c1u32, 32)?;
    assert!(mempad::msgs::MoonMempadMsg::from_cell(&short.build()?).is_err());
    let mut unknown = TonCell::builder();
    unknown.write_num(&0xaabbccddu32, 32)?;
    unknown.write_bit(true)?;
    unknown.write_ref(TonCell::empty().clone())?;
    let unknown = unknown.build()?;
    assert_eq!(mempad::msgs::MoonMempadMsg::from_cell(&unknown)?.to_cell()?, unknown);
    // Referenced ordinary records follow TON tail tolerance.
    let mut content = TonCell::builder();
    TLBCoins::ONE.write(&mut content)?;
    content.write_num(&U256::ZERO, 256)?;
    content.write_num(&U256::ZERO, 256)?;
    content.write_bit(false)?;
    let mut message = TonCell::builder();
    message.write_num(&0x7b5ceb3fu32, 32)?;
    message.write_num(&0u64, 64)?;
    message.write_ref(content.build()?)?;
    message.write_bit(false)?;
    assert!(cpmm::msgs::MoonCpmmDeployPositionMsg::from_cell(&message.build()?).is_ok());
    Ok(())
}
#[test]
fn test_null_zero_addresses_and_signed_bin_keys() -> anyhow::Result<()> {
    let a = TonAddress::ZERO;
    let value = clmm::range::storage::MoonClmmRangeStorage {
        pool: a,
        range_num: -1,
        x_wallet: None,
        y_wallet: a,
        bin_step: 100,
        lp_fee: 20,
        protocol_fee: 10,
        position_index: 7,
        range_state: TLBRef::new(clmm::range::storage::MoonClmmRangeState {
            current_bin: -1025,
            current_sqrt_p: U256::ONE,
            x_protocol_fees: TLBCoins::ZERO,
            y_protocol_fees: TLBCoins::ZERO,
        }),
        bins: HashMap::from([(
            -1025,
            clmm::range::storage::MoonClmmBin {
                l: U256::from(17u8),
                x_fee: U256::ZERO,
                y_fee: U256::ZERO,
            },
        )]),
        lp_account_code: TLBRef::new(TonCell::empty().clone()),
        lp_position_code: TLBRef::new(TonCell::empty().clone()),
    };
    let cell = value.to_cell()?;
    let decoded = clmm::range::storage::MoonClmmRangeStorage::from_cell(&cell)?;
    assert_eq!(decoded.range_state.current_bin, -1025);
    assert!(decoded.bins.contains_key(&-1025));
    assert_eq!(decoded.x_wallet, None);
    assert_eq!(decoded.y_wallet, TonAddress::ZERO);
    Ok(())
}
#[test]
fn test_address_derivation_uses_distinct_wallet_layout() -> anyhow::Result<()> {
    let curve = mempad::curve::storage::MoonMempadCurveStorage::from_boc(
        include_bytes!("fixtures/launch-after-buy.boc").to_vec(),
    )?;
    let owner = TonAddress::new(0, TonHash::from([0xabu8; 32]));
    let init = mempad::curve::storage::MoonMempadCurveInitData {
        factory: curve.factory,
        creator: curve.creator,
        salt: curve.salt,
    };
    let ordinary = address::derive(TonCell::empty().clone(), &init)?;
    let wallet = address::mempad_wallet(TonCell::empty().clone(), &owner, &ordinary)?;
    assert_eq!(wallet.hash.as_slice().first(), Some(&0xab));
    Ok(())
}

#[test]
fn test_metadata_prefix_branching_and_jetton_either_payload() -> anyhow::Result<()> {
    use crate::contracts::{metadata::MoonPrefixedString, msgs::MoonJettonTransferMsg};
    let mut raw = TonCell::builder();
    raw.write_num(&0u8, 8)?;
    raw.write_num(&0x6162u16, 16)?;
    let value = MoonPrefixedString::from_cell(&raw.build()?)?;
    assert_eq!(value.chunks, vec![b"ab".to_vec()]);
    let mut bad = TonCell::builder();
    bad.write_num(&1u8, 8)?;
    assert!(MoonPrefixedString::from_cell(&bad.build()?).is_err());
    let mut branch = TonCell::builder();
    branch.write_num(&0u8, 8)?;
    branch.write_ref(TonCell::empty().clone())?;
    branch.write_ref(TonCell::empty().clone())?;
    assert!(MoonPrefixedString::from_cell(&branch.build()?).is_err());
    let dest = TonAddress::new(0, [1u8; 32].into());
    let msg = MoonJettonTransferMsg {
        query_id: 9,
        amount: TLBCoins::new(100),
        destination: dest,
        response_destination: None,
        custom_payload: None,
        forward_ton_amount: TLBCoins::ONE,
        forward_payload: TLBEither::Right(TLBRef::new(TonCell::empty().clone())),
    };
    let wire = msg.to_cell()?;
    assert_eq!(MoonJettonTransferMsg::from_cell(&wire)?, msg);
    let mut truncated = TonCell::builder();
    truncated.write_num(&0x0f8a7ea5u32, 32)?;
    assert!(cpmm::msgs::MoonCpmmMsg::from_cell(&truncated.build()?).is_err());
    Ok(())
}
#[test]
fn test_source_defaults_are_not_literal_wire_constraints() -> anyhow::Result<()> {
    let address = TonAddress::new(0, [2u8; 32].into());
    let request = cpmm::msgs::MoonCpmmProvideWalletAddressMsg {
        query_id: 1,
        request_address: address,
        include_address: true,
    };
    assert!(cpmm::msgs::MoonCpmmProvideWalletAddressMsg::from_cell(&request.to_cell()?)?.include_address);
    let address = TonAddress::new(0, [3u8; 32].into());
    let transfer = cpmm::msgs::MoonCpmmJettonTransferMsg {
        query_id: 1,
        amount: TLBCoins::ONE,
        destination: address,
        response_destination: TonAddress::ZERO,
        custom_payload: Some(TLBRef::new(TonCell::empty().clone())),
        forward_ton_amount: TLBCoins::ZERO,
        have_payload: false,
    };
    // responseDestination is mandatory in this particular SDK record; use a
    // literal internal zero-hash address rather than the addr_none convenience.
    let mut transfer = transfer;
    transfer.response_destination = TonAddress::new(0, [0u8; 32].into());
    assert!(
        cpmm::msgs::MoonCpmmJettonTransferMsg::from_cell(&transfer.to_cell()?)?
            .custom_payload
            .is_some()
    );
    Ok(())
}
#[test]
fn test_launch_wallet_complete_independent_state_init_hash() -> anyhow::Result<()> {
    use ton::ton_core::cell::TonHash;
    let v: serde_json::Value = serde_json::from_str(include_str!("fixtures/wallet-address.json"))?;
    let owner = TonAddress::new(0, TonHash::from([0xabu8; 32]));
    let curve = TonAddress::new(0, TonHash::from([0x12u8; 32]));
    let addr = address::mempad_wallet(TonCell::empty().clone(), &owner, &curve)?;
    assert_eq!(
        hex::encode(addr.hash.as_slice()),
        v["wallet_hash"].as_str().ok_or_else(|| anyhow::anyhow!("hash"))?
    );
    Ok(())
}
#[test]
fn test_legacy_sdk_factory_layout_is_explicit_and_strict() -> anyhow::Result<()> {
    let owner = MsgAddressIntStd {
        anycast: None,
        workchain: 0,
        address: [1u8; 32].into(),
    };
    let mut legacy = TonCell::builder();
    owner.write(&mut legacy)?;
    legacy.write_num(&20u8, 8)?;
    legacy.write_num(&10u8, 8)?;
    for _ in 0..3 {
        legacy.write_ref(TonCell::empty().clone())?;
    }
    match cpmm::factory::storage::MoonCpmmFactoryRevision::from_cell(&legacy.build()?)? {
        cpmm::factory::storage::MoonCpmmFactoryRevision::Legacy(data) => {
            assert_eq!((data.lp_fee, data.protocol_fee), (20, 10));
        },
        _ => anyhow::bail!("wrong factory revision"),
    }
    let mut malformed = TonCell::builder();
    owner.write(&mut malformed)?;
    malformed.write_num(&0u32, 17)?;
    for _ in 0..3 {
        malformed.write_ref(TonCell::empty().clone())?;
    }
    assert!(cpmm::factory::storage::MoonCpmmFactoryRevision::from_cell(&malformed.build()?).is_err());
    Ok(())
}
#[test]
fn test_current_curve_admin_is_tvm_null_not_addr_none_slice() -> anyhow::Result<()> {
    use ton::block_tlb::{TVMNull, TVMStackValue};
    let mut stack = TVMStack::default();
    stack.push_tiny_int(99);
    stack.push_tiny_int(-1);
    stack.push(TVMStackValue::Null(TVMNull));
    stack.push_cell(TonCell::empty().clone());
    stack.push_cell(TonCell::empty().clone());
    let result = mempad::curve::method_results::MoonMempadCurveJettonDataResult::from_stack(&mut stack)?;
    assert_eq!(result.total_supply.to_string(), "99");
    assert!(result.mintable);
    assert!(result.admin_address.is_none());
    Ok(())
}
#[test]
fn test_legacy_masters_suffix_and_init_only_position_are_distinct() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/cpmm-legacy-masters.boc").to_vec())?;
    assert!(cpmm::pool::storage::MoonCpmmPoolStorage::from_cell(&cell).is_err());
    match cpmm::pool::storage::MoonCpmmPoolRevision::from_cell(&cell)? {
        cpmm::pool::storage::MoonCpmmPoolRevision::Legacy(value) => {
            assert_eq!(value.reserves.x_reserve.to_u128(), 1_009_960);
        },
        _ => anyhow::bail!("wrong pool revision"),
    }
    let owner = MsgAddressIntStd {
        anycast: None,
        workchain: 0,
        address: [1u8; 32].into(),
    };
    let mut init = TonCell::builder();
    owner.write(&mut init)?;
    owner.write(&mut init)?;
    let cell = init.build()?;
    assert!(matches!(
        cpmm::position::storage::MoonCpmmPositionState::from_cell(&cell)?,
        cpmm::position::storage::MoonCpmmPositionState::Init(_)
    ));
    let mut malformed = TonCell::builder();
    malformed.write_cell(&cell)?;
    malformed.write_ref(TonCell::empty().clone())?;
    assert!(cpmm::position::storage::MoonCpmmPositionState::from_cell(&malformed.build()?).is_err());
    Ok(())
}
#[test]
fn test_observed_external_triggers_have_no_invented_opcode_or_query_id() -> anyhow::Result<()> {
    let old = order_old::msgs::MoonOrderOldExternalVestingReleaseMsg {
        ignored_body: TonCell::empty().clone(),
    }
    .to_cell()?;
    let new = order_new::msgs::MoonOrderNewExternalVestingReleaseMsg {
        ignored_body: TonCell::empty().clone(),
    }
    .to_cell()?;
    let boost = booster::msgs::MoonBoosterExternalBoostTriggerMsg {
        ignored_body: TonCell::empty().clone(),
    }
    .to_cell()?;
    for cell in [old, new, boost] {
        assert_eq!(cell.data_len_bits(), 0);
        assert!(cell.refs().is_empty());
    }
    Ok(())
}

#[test]
fn test_current_ranges_signed_dictionary_and_null_stack() -> anyhow::Result<()> {
    use ton::block_tlb::{TVMNull, TVMStackValue};
    let root = TonCell::from_boc(include_bytes!("fixtures/current-ranges-dictionary.boc").to_vec())?;
    let mut stack = TVMStack::default();
    stack.push_cell(root);
    let ranges = clmm::pool::method_results::MoonClmmPoolRangesResult::from_stack(&mut stack)?.ranges;
    assert_eq!(ranges.len(), 2);
    let first = ranges.get(&-1).ok_or_else(|| anyhow::anyhow!("signed range missing"))?;
    let expected = TonAddress::new(0, TonHash::from([0x55; 32]));
    assert_eq!(first, &expected);
    let mut stack = TVMStack::default();
    stack.push(TVMStackValue::Null(TVMNull));
    assert!(
        clmm::pool::method_results::MoonClmmPoolRangesResult::from_stack(&mut stack)?
            .ranges
            .is_empty()
    );
    Ok(())
}

#[test]
fn test_bounce_envelopes_preserve_source_defined_remainders() -> anyhow::Result<()> {
    use crate::contracts::msgs::{MoonBounceMsg, MoonReferencedBounceMsg};
    let mut original = TonCell::builder();
    original.write_num(&0x84548afau32, 32)?;
    original.write_num(&123u64, 64)?;
    let original = original.build()?;
    let mut legacy = TonCell::builder();
    legacy.write_num(&0xffffffffu32, 32)?;
    legacy.write_cell(&original)?;
    assert_eq!(MoonBounceMsg::from_cell(&legacy.build()?)?.original_prefix, original);
    let mut referenced = TonCell::builder();
    referenced.write_num(&0xfffffffeu32, 32)?;
    referenced.write_num(&7u8, 3)?;
    referenced.write_ref(original.clone())?;
    referenced.write_ref(TonCell::empty().clone())?;
    let cell = referenced.build()?;
    let value = MoonReferencedBounceMsg::from_cell(&cell)?;
    assert_eq!(*value.original, original);
    assert_eq!(value.diagnostics.data_len_bits(), 3);
    assert_eq!(value.diagnostics.refs().len(), 1);
    assert_eq!(value.to_cell()?, cell);
    let mut malformed = TonCell::builder();
    malformed.write_num(&0xfffffffeu32, 32)?;
    assert!(MoonReferencedBounceMsg::from_cell(&malformed.build()?).is_err());
    Ok(())
}

// Ordinary dictionary forks and leaves accept unmodeled suffixes.
#[test]
fn test_dictionary_accepts_fork_and_leaf_suffixes() -> anyhow::Result<()> {
    use crate::contracts::clmm::msgs::MoonClmmWithdrawLiquidityMsg;
    use ton::ton_core::{cell::TonCell, traits::tlb::TLB};
    for malformed in 0..5 {
        let leaf = |one: bool| -> anyhow::Result<TonCell> {
            let mut b = TonCell::builder();
            b.write_num(&3u8, 2)?; // hml_same$11
            b.write_bit(one)?;
            b.write_num(&63u8, 6)?;
            b.write_bit(true)?; // bool dictionary value
            if one && malformed == 3 {
                b.write_bit(false)?;
            }
            if one && malformed == 4 {
                b.write_ref(TonCell::empty().clone())?;
            }
            Ok(b.build()?)
        };
        let mut fork = TonCell::builder();
        fork.write_num(&0u8, 2)?; // hml_short$0, unary length zero
        if malformed == 1 {
            fork.write_bit(true)?;
        }
        fork.write_ref(leaf(false)?)?;
        fork.write_ref(leaf(true)?)?;
        if malformed == 2 {
            fork.write_ref(TonCell::empty().clone())?;
        }
        let mut body = TonCell::builder();
        body.write_num(&0xc84175bbu32, 32)?;
        body.write_num(&7u64, 64)?;
        body.write_bit(true)?;
        body.write_ref(fork.build()?)?;
        let result = MoonClmmWithdrawLiquidityMsg::from_cell(&body.build()?);
        if malformed == 0 {
            let value = result?;
            assert_eq!(value.withdraw_bins.len(), 2);
            assert_eq!(value.withdraw_bins.get(&0), Some(&true));
            assert_eq!(value.withdraw_bins.get(&-1), Some(&true));
        } else {
            assert!(result.is_ok(), "rejected dictionary tail case {malformed}");
        }
    }
    Ok(())
}
