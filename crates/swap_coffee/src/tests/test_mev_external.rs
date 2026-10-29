use crate::contracts::mev_protector::msgs::CoffeeMevProtectorIncomingExternalMessage;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorSendMessageAction;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorSignedBatchMsg;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacyIncomingExternalMessage;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacySendMessageAction;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacySignedBatchMsg;
use fastnum::U256;

use std::collections::HashMap;
use std::str::FromStr;

use ton::ton_core::cell::TonCell;
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::MsgAddressInt;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
fn message(recipient: &MsgAddressInt, key: i16) -> anyhow::Result<TonCell> {
    let mut b = TonCell::builder();
    b.write_num(&0x10u8, 6)?;
    recipient.write(&mut b)?;
    TLBCoins::new(1).write(&mut b)?;
    b.write_num(&U256::ZERO, 107)?;
    b.write_num(&(key as u16 as u32), 32)?;
    Ok(b.build()?)
}
#[test]
fn test_original_current_and_legacy_external_vectors_preserve_signed_keys() -> anyhow::Result<()> {
    let data: serde_json::Value = serde_json::from_str(include_str!("fixtures/mev-external-observations.json"))?;
    let recipient = MsgAddressInt::from_cell(
        &TonAddress::from_str(data["recipient"].as_str().ok_or_else(|| anyhow::anyhow!("missing recipient"))?)?
            .to_cell()?,
    )?;
    for row in data["cases"].as_array().ok_or_else(|| anyhow::anyhow!("missing cases"))? {
        let id = row["id"].as_u64().ok_or_else(|| anyhow::anyhow!("missing id"))? as usize;
        let keys = data["keys"][id].as_array().ok_or_else(|| anyhow::anyhow!("missing keys"))?;
        let mut payload = HashMap::new();
        let mut old_payload = HashMap::new();
        for key in keys {
            let key: i16 = key.as_i64().ok_or_else(|| anyhow::anyhow!("invalid key"))?.try_into()?;
            let cell = message(&recipient, key)?;
            payload.insert(
                key,
                CoffeeMevProtectorSendMessageAction {
                    mode: 1,
                    message: TLBRef::new(cell.clone()),
                },
            );
            old_payload.insert(
                key,
                CoffeeMevProtectorLegacySendMessageAction {
                    mode: 1,
                    message: TLBRef::new(cell),
                },
            );
        }
        let wallet = if id == 4 { 124 } else { 123 };
        let query = if id == 5 {
            (1_700_000_000u64 << 32) - 1
        } else {
            (1_700_000_100u64 << 32) + 1
        };
        let batch = CoffeeMevProtectorSignedBatchMsg::new([0; 64], wallet, query, payload);
        let old = CoffeeMevProtectorLegacySignedBatchMsg::new([0; 64], wallet, query, old_payload);
        let cell = batch.to_cell()?;
        assert_eq!(old.to_cell()?, cell);
        assert_eq!(
            format!("{}", cell.hash()?).to_lowercase(),
            row["hash_hex"].as_str().ok_or_else(|| anyhow::anyhow!("missing hash"))?
        );
        let decoded = CoffeeMevProtectorSignedBatchMsg::from_cell(&cell)?;
        assert_eq!(decoded, batch);
        assert_eq!(CoffeeMevProtectorIncomingExternalMessage::from_cell(&cell)?.to_cell()?, cell);
        assert_eq!(
            CoffeeMevProtectorLegacyIncomingExternalMessage::from_cell(&cell)?.to_cell()?,
            cell
        );
        for key in keys {
            let key: i16 = key.as_i64().ok_or_else(|| anyhow::anyhow!("invalid key"))?.try_into()?;
            assert!(decoded.payload.contains_key(&key));
        }
    }
    Ok(())
}
#[test]
fn test_signed_body_preserves_valid_noncanonical_dictionary_labels() -> anyhow::Result<()> {
    // A short all-zero16-bit label is valid but longer than the canonical same
    // label. Re-encoding it canonically would change the signed body cell hash.
    let recipient = super::getter_provider::address_cell(1)?;
    let recipient = MsgAddressInt::from_cell(&recipient)?;
    let mut leaf = TonCell::builder();
    leaf.write_bit(false)?;
    for _ in 0..16 {
        leaf.write_bit(true)?;
    }
    leaf.write_bit(false)?;
    leaf.write_num(&0u16, 16)?;
    leaf.write_num(&1u8, 8)?;
    leaf.write_ref(message(&recipient, 0)?)?;
    let mut unsigned = TonCell::builder();
    unsigned.write_num(&123u32, 32)?;
    unsigned.write_num(&77u64, 64)?;
    unsigned.write_bit(true)?;
    unsigned.write_ref(leaf.build()?)?;
    let unsigned = unsigned.build()?;
    let mut full = TonCell::builder();
    full.write_bits([0u8; 64], 512)?;
    full.write_cell(&unsigned)?;
    let full = full.build()?;
    let mut batch = CoffeeMevProtectorSignedBatchMsg::from_cell(&full)?;
    assert_eq!(batch.unsigned_body_cell()?, unsigned);
    assert_eq!(batch.to_cell()?, full);
    let canonical = CoffeeMevProtectorSignedBatchMsg::new(
        batch.signature,
        batch.subwallet_id,
        batch.query_id,
        batch.payload.clone(),
    )
    .unsigned_body_cell()?;
    assert_ne!(canonical.hash()?, unsigned.hash()?);
    batch.signature = [9; 64];
    assert_eq!(batch.unsigned_body_cell()?, unsigned);
    batch.query_id = 78;
    assert_ne!(batch.unsigned_body_cell()?.hash()?, unsigned.hash()?);
    // A signature is intentionally never repaired or silently generated.
    assert_eq!(batch.signature, [9; 64]);
    Ok(())
}

#[test]
fn test_legacy_signed_body_preserves_valid_noncanonical_dictionary_labels() -> anyhow::Result<()> {
    // A short all-zero16-bit label is valid but longer than the canonical same
    // label. Re-encoding it canonically would change the signed body cell hash.
    let recipient = super::getter_provider::address_cell(1)?;
    let recipient = MsgAddressInt::from_cell(&recipient)?;
    let mut leaf = TonCell::builder();
    leaf.write_bit(false)?;
    for _ in 0..16 {
        leaf.write_bit(true)?;
    }
    leaf.write_bit(false)?;
    leaf.write_num(&0u16, 16)?;
    leaf.write_num(&1u8, 8)?;
    leaf.write_ref(message(&recipient, 0)?)?;
    let mut unsigned = TonCell::builder();
    unsigned.write_num(&123u32, 32)?;
    unsigned.write_num(&77u64, 64)?;
    unsigned.write_bit(true)?;
    unsigned.write_ref(leaf.build()?)?;
    let unsigned = unsigned.build()?;
    let mut full = TonCell::builder();
    full.write_bits([0u8; 64], 512)?;
    full.write_cell(&unsigned)?;
    let full = full.build()?;
    let mut batch = CoffeeMevProtectorLegacySignedBatchMsg::from_cell(&full)?;
    assert_eq!(batch.unsigned_body_cell()?, unsigned);
    assert_eq!(batch.to_cell()?, full);
    let canonical = CoffeeMevProtectorLegacySignedBatchMsg::new(
        batch.signature,
        batch.subwallet_id,
        batch.query_id,
        batch.payload.clone(),
    )
    .unsigned_body_cell()?;
    assert_ne!(canonical.hash()?, unsigned.hash()?);
    batch.signature = [9; 64];
    assert_eq!(batch.unsigned_body_cell()?, unsigned);
    batch.query_id = 78;
    assert_ne!(batch.unsigned_body_cell()?.hash()?, unsigned.hash()?);
    // A signature is intentionally never repaired or silently generated.
    assert_eq!(batch.signature, [9; 64]);
    Ok(())
}
