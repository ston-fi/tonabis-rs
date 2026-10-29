use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2SignedRequestMsg as DedustX1000WalletV2SignedRequest;
use std::collections::HashMap;
use ton::ton_core::{cell::TonCell, traits::tlb::TLB};

fn replace_reference(cell: &TonCell, index: usize, replacement: TonCell) -> anyhow::Result<TonCell> {
    let mut builder = TonCell::builder();
    builder.write_bits(cell.parser().read_bits(cell.data_len_bits())?, cell.data_len_bits())?;
    for (i, reference) in cell.refs().iter().enumerate() {
        builder.write_ref(if i == index {
            replacement.clone()
        } else {
            reference.clone()
        })?;
    }
    Ok(builder.build()?)
}

#[test]
fn test_signed_v2_preserves_alternate_cache_labels_and_tracks_mutation() -> anyhow::Result<()> {
    // Original schema fixture is synthetic; no signing or executable input is used.
    let fixtures: HashMap<String, String> = serde_json::from_str(include_str!("fixtures/schema-vectors.json"))?;
    let raw = TonCell::from_boc_base64(
        fixtures
            .get("schema_DedustX1000WalletV2_DedustX1000WalletV2SignedRequest")
            .ok_or_else(|| anyhow::anyhow!("missing signed fixture"))?,
    )?;
    let mut request: DedustX1000WalletV2SignedRequest = TLB::from_cell(&raw)?;
    request.payload.action.cache = HashMap::from([(0, 17)]);
    let canonical = request.to_cell()?;
    for long in [true, false] {
        let mut cache = TonCell::builder();
        if long {
            cache.write_num(&2u8, 2)?; // hml_long$10
            cache.write_num(&28u8, 5)?;
        } else {
            cache.write_bit(false)?; // hml_short$0, unary length28
            for _ in 0..28 {
                cache.write_bit(true)?;
            }
            cache.write_bit(false)?;
        }
        cache.write_num(&0u32, 28)?;
        cache.write_num(&17u64, 64)?;
        let action = replace_reference(&canonical.refs()[0], 1, cache.build()?)?;
        let mut malformed = TonCell::builder();
        malformed.write_cell(&action)?;
        malformed.write_bit(true)?;
        let malformed = replace_reference(&canonical, 0, malformed.build()?)?;
        assert!(<DedustX1000WalletV2SignedRequest as TLB>::from_cell(&malformed).is_err());
        let packet = replace_reference(&canonical, 0, action)?;
        let mut decoded: DedustX1000WalletV2SignedRequest = TLB::from_cell(&packet)?;
        assert_eq!(decoded.payload.action.cache, HashMap::from([(0, 17)]));
        assert_eq!(
            decoded.to_cell()?.hash()?,
            packet.hash()?,
            "an unchanged signed request must preserve the signature-bound graph"
        );
        decoded.payload.action.request_id += 1;
        let changed = decoded.to_cell()?;
        assert_ne!(changed.hash()?, packet.hash()?);
        let changed: DedustX1000WalletV2SignedRequest = TLB::from_cell(&changed)?;
        assert_eq!(changed.payload.action.request_id, request.payload.action.request_id + 1);
    }
    Ok(())
}

#[test]
fn test_signed_v1_preserves_alternate_message_dictionary_and_tracks_mutation() -> anyhow::Result<()> {
    use crate::contracts::custom_wallet::v1::msgs::DedustX1000WalletV1ExecuteMsg as Execute;
    use ton::block_tlb::Msg;
    use ton::ton_core::types::tlb_core::MsgAddress;
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("fixtures/x1000-v1-wire-vectors.json"))?;
    let packet = TonCell::from_boc_base64(
        fixtures["v1-synthetic-execute"]["boc_base64"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing V1 execute"))?,
    )?;
    let mut message = TonCell::builder();
    message.write_num(&0b0100u8, 4)?; // Internal, IHR disabled, no bounce.
    MsgAddress::NONE.write(&mut message)?;
    MsgAddress::NONE.write(&mut message)?;
    message.write_num(&1u8, 4)?; // Valid length-one zero Coins, not canonical length-zero.
    message.write_num(&0u8, 8)?;
    let mut extra = TonCell::builder();
    extra.write_num(&2u8, 2)?; // hml_long$10 rather than the canonical repeated-bit label.
    extra.write_num(&32u8, 6)?;
    extra.write_num(&0u32, 32)?;
    extra.write_num(&0u8, 5)?; // VarUInteger32 zero.
    message.write_bit(true)?;
    message.write_ref(extra.build()?)?;
    message.write_num(&0u8, 4)?; // IHR fee.
    message.write_num(&0u8, 4)?; // Forward fee.
    message.write_num(&0u64, 64)?;
    message.write_num(&0u32, 32)?;
    message.write_bit(false)?; // No StateInit.
    message.write_bit(true)?; // Referenced opaque body.
    message.write_ref(TonCell::empty().clone())?;
    let message = message.build()?;
    let decoded: Msg = TLB::from_cell(&message)?;
    assert_ne!(
        decoded.to_cell()?.hash()?,
        message.hash()?,
        "the test must exercise a noncanonical but accepted message encoding"
    );
    let command = &packet.refs()[1];
    let actions = replace_reference(&command.refs()[0], 1, message)?;
    let command = replace_reference(command, 0, actions)?;
    let packet = replace_reference(&packet, 1, command)?;
    let mut decoded: Execute = TLB::from_cell(&packet)?;
    assert_eq!(decoded.to_cell()?.hash()?, packet.hash()?);
    assert_eq!(decoded.command.actions.to_cell()?.hash()?, packet.refs()[1].refs()[0].hash()?);
    assert_eq!(
        TonCell::from_boc(decoded.command.actions.to_boc()?)?.hash()?,
        packet.refs()[1].refs()[0].hash()?
    );
    decoded.command.actions.actions[1].mode = 3;
    let changed = decoded.to_cell()?;
    assert_ne!(changed.hash()?, packet.hash()?);
    let changed: Execute = TLB::from_cell(&changed)?;
    assert_eq!(changed.command.actions.actions[1].mode, 3);
    Ok(())
}
