use crate::contracts::classic::vault_native_v2::msgs::{
    DedustVaultNativeV2DepositLiquidityMsg, DedustVaultNativeV2IncomingMsg,
};
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmAsset;
use crate::contracts::{classic::pool_v2, uranus::meme_v3};
use fastnum::U256;
use ton::ton_core::{cell::TonCell, traits::tlb::TLB, types::tlb_core::MsgAddress};

#[test]
fn test_known_malformed_and_unknown_bodies_are_distinct() -> anyhow::Result<()> {
    let mut malformed = TonCell::builder();
    malformed.write_num(&0x61ee542du32, 32)?;
    assert!(pool_v2::msgs::DedustPoolV2IncomingMsg::from_cell(&malformed.build()?).is_err());
    let mut unknown = TonCell::builder();
    unknown.write_num(&0x12345678u32, 32)?;
    unknown.write_bit(true)?;
    let body = unknown.build()?;
    let decoded = pool_v2::msgs::DedustPoolV2IncomingMsg::from_cell(&body)?;
    assert!(matches!(decoded, pool_v2::msgs::DedustPoolV2IncomingMsg::Unknown(_)));
    assert_eq!(decoded.to_cell()?, body);
    Ok(())
}

#[test]
fn test_fixed_message_accepts_trailing_bits_and_refs() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64("te6cckEBAQEADgAAGGbIrXKN2V4LAHrmkcs/bH4=")?;
    let expected = meme_v3::msgs::DedustUranusMemeV3Opcode66c8ad72Msg::from_cell(&body)?;
    let mut extended = TonCell::builder();
    extended.write_cell(&body)?;
    extended.write_bit(true)?;
    extended.write_ref(TonCell::empty().clone())?;
    let extended = extended.build()?;
    assert_eq!(
        meme_v3::msgs::DedustUranusMemeV3Opcode66c8ad72Msg::from_cell(&extended)?,
        expected
    );
    assert_eq!(
        meme_v3::msgs::DedustUranusMemeV3OutgoingMsg::from_cell(&extended)?.to_cell()?,
        body
    );
    Ok(())
}

#[test]
fn test_null_address_is_distinct_from_zero_address() -> anyhow::Result<()> {
    let none = MsgAddress::NONE.to_cell()?;
    let mut zero_builder = TonCell::builder();
    zero_builder.write_num(&0b100u8, 3)?;
    zero_builder.write_num(&0i8, 8)?;
    zero_builder.write_num(&U256::ZERO, 256)?;
    let zero = zero_builder.build()?;
    assert_ne!(none, zero);
    // Both address forms are carried by exact optional asset identifiers.
    let a: DedustV2CpmmAsset = TLB::from_cell(&none)?;
    let b: DedustV2CpmmAsset = TLB::from_cell(&zero)?;
    assert_ne!(a, b);
    assert_eq!(a.to_cell()?, none);
    assert_eq!(b.to_cell()?, zero);
    Ok(())
}

#[test]
fn test_dictionary_leaf_suffix_is_tolerated_recursively() -> anyhow::Result<()> {
    use crate::contracts::cpmm_v2::position::storage::DedustV2CpmmPositionRewards;

    fn leaf(key: bool, extra: bool) -> anyhow::Result<TonCell> {
        let mut b = TonCell::builder();
        b.write_num(&0b010u8, 3)?;
        b.write_bit(key)?;
        b.write_num(&0u8, 4)?;
        b.write_num(&0u8, 5)?;
        if extra {
            b.write_bit(true)?;
        }
        Ok(b.build()?)
    }
    fn tree(extra: bool) -> anyhow::Result<TonCell> {
        let mut root = TonCell::builder();
        root.write_num(&0u8, 2)?;
        root.write_ref(leaf(false, extra)?)?;
        root.write_ref(leaf(true, false)?)?;
        let mut b = TonCell::builder();
        b.write_bit(true)?;
        b.write_ref(root.build()?)?;
        Ok(b.build()?)
    }
    let valid: DedustV2CpmmPositionRewards = TLB::from_cell(&tree(false)?)?;
    assert_eq!(valid.entries.len(), 2);
    assert!(valid.entries.contains_key(&0) && valid.entries.contains_key(&3));
    assert_eq!(DedustV2CpmmPositionRewards::from_cell(&tree(true)?)?, valid);
    Ok(())
}

#[test]
fn test_trait_cell_and_boc_boundaries_accept_storage_suffixes() -> anyhow::Result<()> {
    use crate::contracts::classic::factory_v1::storage::DedustFactoryV1Storage;

    // These public entry points also serve TonContract::load_parsed_data.
    let original = TonCell::from_boc(include_bytes!("fixtures/factory-1-data.boc").to_vec())?;
    let expected = DedustFactoryV1Storage::from_cell(&original)?;
    let mut suffix = TonCell::builder();
    suffix.write_cell(&original)?;
    suffix.write_bit(false)?;
    let suffix = suffix.build()?;
    assert_eq!(DedustFactoryV1Storage::from_cell(&suffix)?, expected);
    assert_eq!(DedustFactoryV1Storage::from_boc(suffix.to_boc()?)?, expected);
    let mut reference = TonCell::builder();
    reference.write_cell(&original)?;
    reference.write_ref(TonCell::empty().clone())?;
    assert_eq!(DedustFactoryV1Storage::from_cell(&reference.build()?)?, expected);
    Ok(())
}

#[test]
fn test_operator_forward_opcode_internal_zero_and_required_body_reference() -> anyhow::Result<()> {
    use crate::contracts::operator::v2::msgs::{DedustOperatorV2ForwardMsg, DedustOperatorV2IncomingMsg};
    use ton::ton_core::types::TonAddress;

    let mut body = TonCell::builder();
    body.write_num(&0x1234u16, 16)?;
    let body = body.build()?;
    let mut wire = TonCell::builder();
    wire.write_num(&0xf1989255u32, 32)?;
    wire.write_num(&7u64, 64)?;
    TonAddress::ZERO.to_msg_address_int().write(&mut wire)?;
    let missing = wire.build()?;
    let mut wire = TonCell::builder();
    wire.write_cell(&missing)?;
    wire.write_ref(body.clone())?;
    let wire = wire.build()?;
    let parsed = DedustOperatorV2ForwardMsg::from_cell(&wire)?;
    assert_eq!(parsed.destination, TonAddress::ZERO);
    assert_eq!(*parsed.body, body);
    assert_eq!(parsed.to_cell()?, wire);
    assert!(matches!(
        DedustOperatorV2IncomingMsg::from_cell(&wire)?,
        DedustOperatorV2IncomingMsg::Forward(_)
    ));
    assert!(DedustOperatorV2IncomingMsg::from_cell(&missing).is_err());
    Ok(())
}

#[test]
fn test_factory_required_zero_nullable_zero_and_referenced_config_tails() -> anyhow::Result<()> {
    use crate::contracts::classic::factory_v1::storage::DedustFactoryV1Storage;
    use ton::ton_core::types::{TonAddress, tlb_core::MsgAddressInt};

    let original = TonCell::from_boc(include_bytes!("fixtures/factory-1-data.boc").to_vec())?;
    let mut storage = DedustFactoryV1Storage::from_cell(&original)?;
    storage.owner_address = TonAddress::ZERO;
    for pending in [None, Some(TonAddress::ZERO)] {
        storage.pending_owner_address = pending;
        let wire = storage.to_cell()?;
        let mut p = wire.parser();
        assert_eq!(MsgAddressInt::read(&mut p)?, TonAddress::ZERO.to_msg_address_int());
        let raw = MsgAddress::read(&mut p)?;
        assert_eq!(
            raw,
            if pending.is_some() {
                MsgAddress::Int(TonAddress::ZERO.to_msg_address_int())
            } else {
                MsgAddress::NONE
            }
        );
        assert_eq!(DedustFactoryV1Storage::from_cell(&wire)?, storage);
    }
    let mut config = TonCell::builder();
    config.write_cell(&original.refs()[1])?;
    config.write_bit(true)?;
    let mut root = TonCell::builder();
    root.write_bits(original.parser().read_bits(original.data_len_bits())?, original.data_len_bits())?;
    root.write_ref(original.refs()[0].clone())?;
    root.write_ref(config.build()?)?;
    assert_eq!(
        DedustFactoryV1Storage::from_cell(&root.build()?)?,
        DedustFactoryV1Storage::from_cell(&original)?
    );
    Ok(())
}

#[test]
fn test_dispatch_tlb_preserves_short_unknown_tails_and_reads_at_current_offset() -> anyhow::Result<()> {
    use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1IncomingMsg;
    use crate::contracts::operator::v2::msgs::{DedustOperatorV2ForwardMsg, DedustOperatorV2IncomingMsg};
    use ton::ton_core::types::{TonAddress, tlb_core::TLBRef};

    for bits in [0, 1, 31, 32] {
        let mut body = TonCell::builder();
        body.write_num(&0u32, bits)?;
        body.write_ref(TonCell::empty().clone())?;
        let body = body.build()?;
        let unknown = DedustOperatorV2IncomingMsg::from_cell(&body)?;
        assert!(matches!(unknown, DedustOperatorV2IncomingMsg::Unknown(_)));
        assert_eq!(unknown.to_cell()?, body);
        let factory = DedustFactoryV1IncomingMsg::from_cell(&body)?;
        assert!(matches!(factory, DedustFactoryV1IncomingMsg::Unknown(_)));
        assert_eq!(factory.to_cell()?, body);
    }

    let forward = DedustOperatorV2ForwardMsg {
        query_id: 17,
        destination: TonAddress::ZERO,
        body: TLBRef::new(TonCell::empty().clone()),
    };
    let mut embedded = TonCell::builder();
    embedded.write_num(&5u8, 3)?;
    forward.write(&mut embedded)?;
    let embedded = embedded.build()?;
    let mut parser = embedded.parser();
    assert_eq!(parser.read_num::<u8>(3)?, 5);
    let decoded = DedustOperatorV2IncomingMsg::read(&mut parser)?;
    assert!(matches!(&decoded, DedustOperatorV2IncomingMsg::Forward(value) if **value == forward));
    parser.ensure_empty()?;
    assert_eq!(decoded.to_cell()?, forward.to_cell()?);
    Ok(())
}

#[test]
fn test_known_pool_message_rejects_invalid_nested_proof_prefix() -> anyhow::Result<()> {
    use crate::contracts::classic::pool_v1;
    use ton::ton_core::{errors::TonCoreError, types::TonAddress};

    // SwapPeer's referenced proof starts with factory address then fixed type2.
    // A wrong nested tag must not be mistaken for an unknown outer opcode.
    let mut proof = TonCell::builder();
    TonAddress::ZERO.to_msg_address_int().write(&mut proof)?;
    proof.write_num(&7u8, 8)?;
    let mut body = TonCell::builder();
    body.write_num(&0x72aca8aau32, 32)?;
    body.write_num(&0u64, 64)?;
    body.write_ref(proof.build()?)?;
    let body = body.build()?;
    assert!(matches!(
        pool_v1::msgs::DedustPoolV1IncomingMsg::from_cell(&body),
        Err(TonCoreError::TLBWrongPrefix { .. })
    ));
    assert!(matches!(
        pool_v1::msgs::DedustPoolV1OutgoingMsg::from_cell(&body),
        Err(TonCoreError::TLBWrongPrefix { .. })
    ));
    assert!(matches!(
        pool_v2::msgs::DedustPoolV2IncomingMsg::from_cell(&body),
        Err(TonCoreError::TLBWrongPrefix { .. })
    ));
    assert!(matches!(
        pool_v2::msgs::DedustPoolV2OutgoingMsg::from_cell(&body),
        Err(TonCoreError::TLBWrongPrefix { .. })
    ));
    Ok(())
}

#[test]
fn test_native_vault_nested_prefix_mismatch_preserves_raw_body() -> anyhow::Result<()> {
    let mut body = TonCell::builder();
    body.write_num(&0xd55e4686u32, 32)?; // deposit_liquidity
    body.write_num(&0u64, 64)?;
    body.write_num(&0u8, 4)?; // zero Coins
    body.write_bit(false)?; // volatile pool
    body.write_num(&2u8, 4)?; // invalid asset tag: only native=0 and jetton=1 exist
    body.write_num(&0u8, 4)?; // native second asset
    let mut amounts = TonCell::builder();
    amounts.write_num(&0u16, 12)?; // three zero Coins
    body.write_ref(amounts.build()?)?;
    body.write_bit(false)?;
    body.write_bit(false)?;
    let body = body.build()?;

    assert!(DedustVaultNativeV2DepositLiquidityMsg::from_cell(&body).is_err());
    let decoded = DedustVaultNativeV2IncomingMsg::from_cell(&body)?;
    assert!(matches!(&decoded, DedustVaultNativeV2IncomingMsg::Unknown(raw) if raw == &body));
    assert_eq!(decoded.to_cell()?, body);

    // Prefix fallback does not turn every decoding error into Unknown.
    let mut truncated = TonCell::builder();
    truncated.write_num(&0xd55e4686u32, 32)?;
    assert!(DedustVaultNativeV2IncomingMsg::from_cell(&truncated.build()?).is_err());
    Ok(())
}
