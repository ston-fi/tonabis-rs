//! Independent literal encoder vectors catch fixed V1 widths/reference order.
use crate::contracts::custom_wallet::v1::msgs::DedustX1000WalletV1Callback01f3835dMsg;
use crate::contracts::custom_wallet::v1::msgs::DedustX1000WalletV1Callback6edd65f0Msg;
use crate::contracts::custom_wallet::v1::msgs::DedustX1000WalletV1ExecuteMsg;
use crate::contracts::custom_wallet::v1::msgs::DedustX1000WalletV1IncomingMsg;
use crate::contracts::custom_wallet::v1::msgs::DedustX1000WalletV1RevokeKeyMsg;
use fastnum::{U256, U512};
use ton::ton_core::{cell::TonCell, traits::tlb::TLB, types::tlb_core::MsgAddress};
fn wire(name: &str) -> anyhow::Result<TonCell> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/x1000-v1-wire-vectors.json"))?;
    Ok(TonCell::from_boc_base64(
        rows[name]["boc_base64"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing V1 wire {name}"))?,
    )?)
}
#[test]
fn test_v1_signed_reference_order_sequence_width_and_expiry() -> anyhow::Result<()> {
    let original = wire("v1-synthetic-execute")?;
    let execute: DedustX1000WalletV1ExecuteMsg = TLB::from_cell(&original)?;
    assert_eq!(execute.signature, U512::from_u64(17));
    assert_eq!(execute.authorization.signature, U512::from_u64(33));
    assert_eq!(execute.command.network_id, -239);
    assert_eq!(execute.command.sequence, 7);
    assert_eq!(execute.command.valid_until, 1000);
    let envelope = &execute.authorization.message;
    assert_eq!(envelope.schema_hash, 2358434786);
    assert_eq!(envelope.timestamp, 10);
    assert_eq!(envelope.user_address, MsgAddress::NONE);
    assert_eq!(envelope.app_domain_cell.parser().read_num::<u32>(32)?, 0x11223344);
    assert_eq!(envelope.payload.valid_until, 1000);
    assert_eq!(envelope.payload.public_key, U256::from_u64(44));
    assert_eq!(execute.command.actions.actions.len(), 2);
    assert_eq!(execute.command.actions.actions[0].mode, 3);
    assert_eq!(execute.command.actions.actions[1].mode, 1);
    assert_eq!(
        execute.command.actions.actions[0]
            .message
            .body
            .value
            .parser()
            .read_num::<u32>(32)?,
        0xabcdef01
    );
    assert_eq!(execute.to_cell()?.hash()?, original.hash()?);
    let mut bad = (*execute.command.actions).clone();
    bad.actions[0].mode = 1;
    assert!(bad.to_cell().is_err());
    let mut too_many = (*execute.command.actions).clone();
    too_many.actions = vec![too_many.actions[0].clone(); 256];
    assert!(too_many.to_cell().is_err());
    Ok(())
}
#[test]
fn test_v1_callback_context_is_fixed_and_generation_specific() -> anyhow::Result<()> {
    let pton: DedustX1000WalletV1Callback01f3835dMsg = TLB::from_cell(&wire("v1-callback-pton")?)?;
    assert_eq!((pton.query_id, *pton.amount), (1, 99));
    assert_eq!(pton.address, MsgAddress::NONE);
    let context = pton
        .payload
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("missing fixed payout context"))?;
    assert_eq!((context.query_id, context.word0, context.word1), (9, 300, 4000));
    assert_ne!(context.address0, context.address1);
    let bidask: DedustX1000WalletV1Callback6edd65f0Msg = TLB::from_cell(&wire("v1-callback-bidask")?)?;
    assert_eq!((bidask.query_id, *bidask.amount), (2, 1234));
    for name in [
        "v1-callback-pton",
        "v1-callback-bidask",
        "v1-callback-classic",
        "v1-callback-coffee",
        "v1-callback-b39f",
        "v1-callback-context",
    ] {
        let original = wire(name)?;
        let message = DedustX1000WalletV1IncomingMsg::from_cell(&original)?;
        assert!(!matches!(message, DedustX1000WalletV1IncomingMsg::Unknown(_)));
        assert_eq!(message.to_cell()?.hash()?, original.hash()?);
    }
    // The known nested continuation cannot silently become caller-defined bytes.
    let mut invalid = TonCell::builder();
    invalid.write_num(&0x474f86cfu32, 32)?;
    invalid.write_num(&3u64, 64)?;
    invalid.write_bit(true)?;
    invalid.write_ref(TonCell::empty().clone())?;
    assert!(DedustX1000WalletV1IncomingMsg::from_cell(&invalid.build()?).is_err());
    Ok(())
}

#[test]
fn test_v1_revoke_signature_and_fixed_delegation_reference_are_not_execute_layout() -> anyhow::Result<()> {
    let original = wire("v1-synthetic-revoke")?;
    let revoke: DedustX1000WalletV1RevokeKeyMsg = TLB::from_cell(&original)?;
    assert_eq!(revoke.signature, U512::from_u64(55));
    assert_eq!(revoke.message.schema_hash, 2358434786);
    assert_eq!(revoke.message.payload.public_key, U256::from_u64(44));
    assert_eq!(revoke.message.payload.valid_until, 1000);
    assert_eq!(revoke.message.timestamp, 10);
    assert_eq!(revoke.message.user_address, MsgAddress::NONE);
    assert_eq!(revoke.to_cell()?.hash()?, original.hash()?);
    assert!(matches!(
        DedustX1000WalletV1IncomingMsg::from_cell(&original)?,
        DedustX1000WalletV1IncomingMsg::RevokeKey(_)
    ));
    let mut extra = TonCell::builder();
    extra.write_cell(&original)?;
    extra.write_bit(false)?;
    assert_eq!(DedustX1000WalletV1IncomingMsg::from_cell(&extra.build()?)?.to_cell()?, original);
    Ok(())
}
