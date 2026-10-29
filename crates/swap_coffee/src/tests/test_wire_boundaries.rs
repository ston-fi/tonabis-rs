use super::test_schema_vectors::cell_for;
use crate::contracts::factory::storage::CoffeeFactoryCodeDictionaries;
use crate::contracts::init::msgs::CoffeeInitDeployMsg;
use crate::contracts::init::storage::CoffeeInitStorage;
use crate::contracts::lp_wallet::msgs::CoffeeLpWalletBurnMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolTakeWalletAddressMsg;
use crate::contracts::pool_constant_product::storage::CoffeePoolAsset;
use crate::contracts::pool_constant_product::storage::CoffeePoolExtraAsset;
use crate::contracts::pool_constant_product::storage::CoffeePoolJettonAsset;
use crate::contracts::staking_item::msgs::CoffeeStakingItemIncomingMessage;
use crate::contracts::staking_item::msgs::CoffeeStakingItemOutgoingMessage;
use ton::ton_core::types::TonAddress;

use fastnum::I512;
use fastnum::U256;

use ton::block_tlb::FromTVMStack;
use ton::block_tlb::TVMStack;

use ton::ton_core::cell::TonCell;

use ton::ton_core::cell::TonHash;

use ton::ton_core::traits::tlb::TLB;

use ton::ton_core::types::tlb_core::MsgAddressIntStd;
use ton::ton_core::types::tlb_core::TLBCoins;

#[test]
fn test_nullable_address_is_not_maybe_or_literal_zero() -> anyhow::Result<()> {
    let empty = CoffeeLpWalletBurnMsg {
        query_id: 7,
        amount: TLBCoins::new(256),
        response_address: None,
        custom_payload: None,
    }
    .to_cell()?;
    // Header 32+64, Coins length 4+16, then addr_none 00 and Maybe-ref 0.
    assert_eq!(empty.data_len_bits(), 32 + 64 + 4 + 16 + 2 + 1);
    let mut zero = CoffeeLpWalletBurnMsg::from_cell(&empty)?;
    zero.response_address = Some(TonAddress::ZERO);
    let zero_cell = zero.to_cell()?;
    assert_ne!(empty.hash()?, zero_cell.hash()?);
    assert!(CoffeeLpWalletBurnMsg::from_cell(&empty)?.response_address.is_none());
    assert!(CoffeeLpWalletBurnMsg::from_cell(&zero_cell)?.response_address.is_some());
    Ok(())
}

#[test]
fn test_asset_tags_and_unsigned_chain_byte_preserve_boundaries() -> anyhow::Result<()> {
    // Source-defined unsigned byte preserves -1 workchain's encoded bits as 255.
    let mut b = TonCell::builder();
    b.write_num(&1u8, 2)?;
    b.write_num(&255u8, 8)?;
    b.write_num(&U256::MAX, 256)?;
    let jetton = b.build()?;
    let decoded = CoffeePoolJettonAsset::from_cell(&jetton)?;
    assert_eq!(decoded.chain, 255);
    assert_eq!(decoded.hash, U256::MAX);
    assert_eq!(decoded.to_cell()?, jetton);
    let mut b = TonCell::builder();
    b.write_num(&2u8, 2)?;
    b.write_num(&u32::MAX, 32)?;
    let extra = b.build()?;
    let decoded = CoffeePoolExtraAsset::from_cell(&extra)?;
    assert_eq!(decoded.id, u32::MAX);
    let mut invalid = TonCell::builder();
    invalid.write_num(&3u8, 2)?;
    assert!(CoffeePoolAsset::from_cell(&invalid.build()?).is_err());
    Ok(())
}

#[test]
fn test_init_tail_and_fixed_message_reference_suffix() -> anyhow::Result<()> {
    // Init identity is intentionally a tail; installing a contract has fixed refs.
    let mut b = TonCell::builder();
    b.write_cell(
        &MsgAddressIntStd {
            anycast: None,
            workchain: 0,
            address: TonHash::ZERO,
        }
        .to_cell()?,
    )?;
    b.write_num(&5u8, 3)?;
    b.write_ref(TonCell::empty().clone())?;
    let data = b.build()?;
    let parsed = CoffeeInitStorage::from_cell(&data)?;
    assert_eq!(parsed.child_identity_tail.data_len_bits(), 3);
    assert_eq!(parsed.child_identity_tail.refs().len(), 1);
    assert_eq!(parsed.to_cell()?, data);
    let original = cell_for("init/CoffeeInitDeploy/0")?;
    let mut bad = TonCell::builder();
    bad.write_cell(&original)?;
    bad.write_ref(TonCell::empty().clone())?;
    assert!(CoffeeInitDeployMsg::from_cell(&bad.build()?).is_ok());
    Ok(())
}

#[test]
fn test_dictionary_leaf_tails_are_tolerated_and_out_of_width_key_fails() -> anyhow::Result<()> {
    // HashmapE<2,^Cell>, one key 00 with hml_short label 0,110,00.
    // Ordinary leaf tails are permitted by native dictionary decoding.
    let mut leaf = TonCell::builder();
    leaf.write_num(&0b011000u8, 6)?;
    leaf.write_ref(TonCell::empty().clone())?;
    leaf.write_bit(true)?;
    let mut root = TonCell::builder();
    root.write_bit(true)?;
    root.write_ref(leaf.build()?)?;
    root.write_bit(false)?;
    assert!(CoffeeFactoryCodeDictionaries::from_cell(&root.build()?).is_ok());
    let mut maps = CoffeeFactoryCodeDictionaries::from_cell(&cell_for("factory/CoffeeFactoryCodeDictionaries/1")?)?;
    let value = maps
        .vault_code_by_asset_type
        .get(&0)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("fixture key absent"))?;
    maps.vault_code_by_asset_type.insert(4, value);
    assert!(maps.to_cell().is_err());
    Ok(())
}

#[test]
fn test_staking_same_opcode_has_distinct_request_and_callback_layouts() -> anyhow::Result<()> {
    let request = cell_for("staking_item/CoffeeStakingItemUnlock/0")?;
    let callback = cell_for("staking_item/CoffeeStakingItemUnlockNotification/0")?;
    assert!(CoffeeStakingItemIncomingMessage::from_cell(&request).is_ok());
    assert!(CoffeeStakingItemOutgoingMessage::from_cell(&request).is_err());
    assert!(CoffeeStakingItemOutgoingMessage::from_cell(&callback).is_ok());
    assert!(CoffeeStakingItemIncomingMessage::from_cell(&callback).is_ok());
    Ok(())
}

#[test]
fn test_unsigned_public_key_and_native_cell_slice_results() -> anyhow::Result<()> {
    let mut negative = TVMStack::default();
    negative.push_int(I512::from_i64(-1));
    assert!(U256::from_stack(&mut negative).is_err());
    // Native get_asset results accept Cell and Slice, while arguments retain exact kinds.
    let mut wrong = TVMStack::default();
    wrong.push_cell(TonCell::empty().clone());
    assert!(TonCell::from_stack(&mut wrong).is_ok());
    Ok(())
}
#[test]
fn test_dictionary_fork_tails_and_alternate_labels_remain_valid() -> anyhow::Result<()> {
    // Two 2-bit keys 00 and10: root label length0, fork branches consume one bit,
    // leaves each have suffix0 and a referenced code value. All bits are original.
    let mut leaf = TonCell::builder();
    leaf.write_num(&0b0100u8, 4)?;
    leaf.write_ref(TonCell::empty().clone())?;
    let leaf = leaf.build()?;
    for (label, bits) in [(0u8, 2usize), (0b1000u8, 4usize), (0b11000u8, 5usize)] {
        for suffix in 0..3 {
            let mut fork = TonCell::builder();
            fork.write_num(&label, bits)?;
            fork.write_ref(leaf.clone())?;
            fork.write_ref(leaf.clone())?;
            if suffix == 1 {
                fork.write_bit(true)?;
            }
            if suffix == 2 {
                fork.write_ref(TonCell::empty().clone())?;
            }
            let mut root = TonCell::builder();
            root.write_bit(true)?;
            root.write_ref(fork.build()?)?;
            root.write_bit(false)?;
            let parsed = CoffeeFactoryCodeDictionaries::from_cell(&root.build()?);
            if suffix == 0 {
                let parsed = parsed?;
                assert_eq!(parsed.vault_code_by_asset_type.len(), 2);
                assert!(parsed.vault_code_by_asset_type.contains_key(&0));
                assert!(parsed.vault_code_by_asset_type.contains_key(&2));
            } else {
                assert_eq!(parsed?.vault_code_by_asset_type.len(), 2, "fork suffix {suffix}, label {label}");
            }
        }
    }
    Ok(())
}
#[test]
fn test_jetton_vault_storage_phase_flag_is_required_and_consistent() -> anyhow::Result<()> {
    use crate::contracts::vault_jetton::storage::CoffeeVaultJettonActiveStorage as Active;
    use crate::contracts::vault_jetton::storage::CoffeeVaultJettonInactiveStorage as Inactive;
    let inactive = cell_for("vault_jetton/CoffeeVaultJettonInactiveStorage/0")?;
    assert!(!Inactive::from_cell(&inactive)?.is_active);
    assert!(Active::from_cell(&inactive).is_err());
    let mut invalid = Inactive::from_cell(&inactive)?;
    invalid.is_active = true;
    assert!(invalid.to_cell().is_err());
    let mut bad = TonCell::builder();
    bad.write_bit(true)?;
    for cell in inactive.refs() {
        bad.write_ref(cell.clone())?;
    }
    assert!(Inactive::from_cell(&bad.build()?).is_err());
    let active = cell_for("vault_jetton/CoffeeVaultJettonActiveStorage/0")?;
    let mut p = active.parser();
    let _ = p.read_bit()?;
    let mut bad = TonCell::builder();
    bad.write_bit(false)?;
    bad.write_cell(&p.read_remaining()?)?;
    let invalid = bad.build()?;
    assert!(Active::from_cell(&invalid).is_err());
    assert!(Inactive::from_cell(&invalid).is_ok());
    Ok(())
}
#[test]
fn test_tep74_known_message_requires_either_bit_and_reference() -> anyhow::Result<()> {
    use crate::contracts::cross_dex::msgs::CoffeeCrossDexIncomingMessage as Incoming;
    use crate::contracts::cross_dex::msgs::CoffeeCrossDexJettonTransferNotificationMsg as Notification;

    use ton::ton_core::types::tlb_core::MsgAddressInt;
    use ton::ton_core::types::tlb_core::TLBEither;
    use ton::ton_core::types::tlb_core::TLBRef;
    let sender: MsgAddressInt = MsgAddressIntStd {
        anycast: None,
        workchain: 0,
        address: TonHash::ZERO,
    }
    .into();
    let header = || -> anyhow::Result<_> {
        let mut b = TonCell::builder();
        b.write_num(&0x7362d09cu32, 32)?;
        b.write_num(&7u64, 64)?;
        TLBCoins::new(13).write(&mut b)?;
        sender.write(&mut b)?;
        Ok(b)
    };
    assert!(Incoming::from_cell(&header()?.build()?).is_err());
    let mut missing_ref = header()?;
    missing_ref.write_bit(true)?;
    assert!(Incoming::from_cell(&missing_ref.build()?).is_err());
    let reference = Notification {
        query_id: 7,
        amount: TLBCoins::new(13),
        sender: Some(TonAddress::from_msg_address(sender.clone())?),
        forward_payload: TLBEither::Right(TLBRef::new(TonCell::empty().clone())),
    }
    .to_cell()?;
    let mut surplus = TonCell::builder();
    surplus.write_cell(&reference)?;
    surplus.write_bit(false)?;
    assert!(Incoming::from_cell(&surplus.build()?).is_ok());
    let inline = Notification {
        query_id: 7,
        amount: TLBCoins::new(13),
        sender: Some(TonAddress::from_msg_address(sender)?),
        forward_payload: TLBEither::Left(TonCell::empty().clone()),
    }
    .to_cell()?;
    assert!(matches!(Notification::from_cell(&inline)?.forward_payload, TLBEither::Left(_)));
    assert_eq!(Incoming::from_cell(&reference)?.to_cell()?, reference);
    Ok(())
}

#[test]
fn test_wallet_discovery_owner_tails_and_distinct_absence_forms() -> anyhow::Result<()> {
    use crate::contracts::vault_jetton::msgs::CoffeeVaultJettonOwnerResponse;

    use ton::ton_core::types::tlb_core::MsgAddressInt;
    let address: MsgAddressInt = MsgAddressIntStd {
        anycast: None,
        workchain: 0,
        address: TonHash::ZERO,
    }
    .into();
    let legacy = CoffeeVaultJettonOwnerResponse::Omitted.to_cell()?;
    let excluded = CoffeeVaultJettonOwnerResponse::NotIncluded.to_cell()?;
    let included =
        CoffeeVaultJettonOwnerResponse::Included(TonAddress::from_msg_address(address.clone())?).to_cell()?;
    assert_eq!(legacy.data_len_bits(), 0);
    assert_eq!(excluded.data_len_bits(), 1);
    assert_eq!(included.data_len_bits(), 1);
    assert_eq!(included.refs()[0].data_len_bits(), 267);
    assert_ne!(legacy.hash()?, excluded.hash()?);
    let mut owner = TonCell::builder();
    owner.write_cell(&address.to_cell()?)?;
    owner.write_bit(true)?;
    let bad_owner = owner.build()?;
    let mut response = TonCell::builder();
    response.write_bit(true)?;
    response.write_ref(bad_owner.clone())?;
    assert!(CoffeeVaultJettonOwnerResponse::from_cell(&response.build()?).is_ok());
    let mut response = TonCell::builder();
    response.write_bit(true)?;
    assert!(CoffeeVaultJettonOwnerResponse::from_cell(&response.build()?).is_err());
    // Ordinary referenced address tails are also tolerated in pool responses.
    let mut body = TonCell::builder();
    body.write_num(&0xd1735400u32, 32)?;
    body.write_num(&77u64, 64)?;
    address.write(&mut body)?;
    body.write_bit(true)?;
    body.write_ref(bad_owner)?;
    assert!(CoffeeConstantProductPoolTakeWalletAddressMsg::from_cell(&body.build()?).is_ok());
    Ok(())
}

#[test]
fn test_standard_deployment_admin_preserves_none_and_zero_and_rejects_wider_addresses() -> anyhow::Result<()> {
    use crate::contracts::cross_dex::storage::CoffeeCrossDexDeploymentStorage;
    use ton::ton_core::types::TonAddress;
    use ton::ton_core::types::tlb_core::MsgAddressIntVar;
    for admin in [None, Some(TonAddress::ZERO)] {
        let record = CoffeeCrossDexDeploymentStorage {
            admin_address: admin,
            deployment_discriminator: 0x11223344,
        };
        let cell = record.to_cell()?;
        assert_eq!(cell.data_len_bits(), if admin.is_none() { 2 + 32 } else { 267 + 32 });
        assert_eq!(CoffeeCrossDexDeploymentStorage::from_cell(&cell)?, record);
        let mut parser = cell.parser();
        assert_eq!(parser.read_num::<u8>(2)?, if admin.is_none() { 0 } else { 2 });
    }
    let mut wide = TonCell::builder();
    MsgAddressIntVar {
        anycast: None,
        addr_bits_len: 8,
        workchain: 256,
        address: vec![42],
    }
    .write(&mut wide)?;
    wide.write_num(&0u32, 32)?;
    assert!(CoffeeCrossDexDeploymentStorage::from_cell(&wide.build()?).is_err());
    let mut anycast = TonCell::builder();
    // addr_std$10, present Anycast of depth one, workchain zero and zero hash.
    anycast.write_num(&5u8, 3)?;
    anycast.write_num(&1u8, 5)?;
    anycast.write_bit(true)?;
    anycast.write_num(&0i8, 8)?;
    anycast.write_bits([0u8; 32], 256)?;
    anycast.write_num(&0u32, 32)?;
    assert!(CoffeeCrossDexDeploymentStorage::from_cell(&anycast.build()?).is_err());
    let invalid = CoffeeCrossDexDeploymentStorage {
        admin_address: Some(TonAddress::new(128, TonHash::ZERO)),
        deployment_discriminator: 0,
    };
    assert!(invalid.to_cell().is_err());
    Ok(())
}

#[test]
fn test_lp_getter_requires_internal_identities_and_preserves_zero() -> anyhow::Result<()> {
    use crate::contracts::lp_wallet::method_results::CoffeeLpWalletGetWalletDataResult;
    use ton::block_tlb::{TVMNull, TVMStackValue};
    let mut raw = TonCell::builder();
    raw.write_num(&4u8, 3)?;
    raw.write_num(&0i8, 8)?;
    raw.write_bits([0u8; 32], 256)?;
    let zero = raw.build()?;
    let mut none = TonCell::builder();
    none.write_num(&0u8, 2)?;
    let none = none.build()?;
    for as_slice in [false, true] {
        let mut stack = TVMStack::default();
        stack.push_tiny_int(17);
        for _ in 0..2 {
            if as_slice {
                stack.push_cell_slice(zero.clone());
            } else {
                stack.push_cell(zero.clone());
            }
        }
        stack.push_cell(TonCell::empty().clone());
        let decoded = CoffeeLpWalletGetWalletDataResult::from_stack(&mut stack)?;
        assert_eq!(decoded.owner_address, TonAddress::ZERO);
        assert_eq!(decoded.jetton_master_address, TonAddress::ZERO);
        assert_eq!(decoded.balance, I512::from_i64(17));
    }
    for invalid in [None, Some(none)] {
        let mut stack = TVMStack::default();
        stack.push_tiny_int(17);
        match invalid {
            None => stack.push(TVMStackValue::Null(TVMNull)),
            Some(cell) => stack.push_cell(cell),
        }
        stack.push_cell_slice(zero.clone());
        stack.push_cell(TonCell::empty().clone());
        assert!(CoffeeLpWalletGetWalletDataResult::from_stack(&mut stack).is_err());
    }
    Ok(())
}
