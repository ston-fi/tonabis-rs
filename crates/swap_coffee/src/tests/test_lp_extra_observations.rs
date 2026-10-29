use crate::contracts::factory::msgs::CoffeeFactoryUpdateAdminMsg;
use crate::contracts::factory::msgs::CoffeeFactoryUpdateWithdrawerMsg;
use crate::contracts::factory::storage::CoffeeFactoryStorage;
use crate::contracts::lp_wallet::msgs as bounce;
use crate::contracts::lp_wallet::msgs::CoffeeLpWalletIncomingMessage;
use crate::contracts::lp_wallet::msgs::CoffeeLpWalletInternalTransferMsg;
use crate::contracts::lp_wallet::msgs::CoffeeLpWalletTransferNotificationMsg;
use crate::contracts::vault_native::storage::CoffeeExtraAsset;

use std::collections::HashMap;

use ton::block_tlb::CurrencyCollection;
use ton::ton_core::cell::TonCell;
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonExtraCurrencyId;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBEither;
use ton::ton_core::types::tlb_core::VarLenBytes;
fn assert_hash(cell: &TonCell, expected: &serde_json::Value) -> anyhow::Result<()> {
    assert_eq!(
        format!("{}", cell.hash()?).to_lowercase(),
        expected.as_str().ok_or_else(|| anyhow::anyhow!("missing hash"))?
    );
    Ok(())
}
#[test]
fn test_lp_nullable_mint_and_bounce_match_two_executable_revisions() -> anyhow::Result<()> {
    let data: serde_json::Value = serde_json::from_str(include_str!("fixtures/lp-wallet-observations.json"))?;
    let empty = TonCell::empty().clone();
    let mint = CoffeeLpWalletInternalTransferMsg {
        query_id: 77,
        amount: TLBCoins::new(13),
        from_address: None,
        response_address: None,
        forward_ton_amount: TLBCoins::new(1_000_000),
        forward_payload: TLBEither::Left(empty.clone()),
    }
    .to_cell()?;
    let notification = CoffeeLpWalletTransferNotificationMsg {
        query_id: 77,
        amount: TLBCoins::new(13),
        from_address: None,
        forward_payload: TLBEither::Left(empty.clone()),
    }
    .to_cell()?;
    assert_hash(&notification, &data["notification_hash_hex"])?;
    assert!(
        CoffeeLpWalletTransferNotificationMsg::from_cell(&notification)?
            .from_address
            .is_none()
    );
    for row in data["vectors"].as_array().ok_or_else(|| anyhow::anyhow!("missing vectors"))? {
        if row["bounced"] == false {
            assert_hash(&mint, &row["input_hash_hex"])?;
            continue;
        }
        let op = row["original_opcode"]
            .as_u64()
            .ok_or_else(|| anyhow::anyhow!("missing opcode"))? as u32;
        let mut b = TonCell::builder();
        b.write_num(&u32::MAX, 32)?;
        b.write_num(&op, 32)?;
        b.write_num(&77u64, 64)?;
        TLBCoins::new(13).write(&mut b)?;
        let cell = b.build()?;
        assert_hash(&cell, &row["input_hash_hex"])?;
        if op == 0xdeadbeef {
            assert!(bounce::CoffeeLpWalletBounceMessage::from_cell(&cell).is_err());
            continue;
        }
        let expected = if op == 0x178d4519 {
            bounce::CoffeeLpWalletBounceOpcode::InternalTransfer
        } else {
            bounce::CoffeeLpWalletBounceOpcode::BurnNotification
        };
        let bounce::CoffeeLpWalletBounceMessage::Known(body) = bounce::CoffeeLpWalletBounceMessage::from_cell(&cell)?
        else {
            anyhow::bail!("known bounce lost")
        };
        assert_eq!(body.original_opcode, expected);
        assert_eq!(body.amount, TLBCoins::new(13));
        // Transport role matters: ordinary messages preserve this body as unknown.
        assert!(matches!(
            CoffeeLpWalletIncomingMessage::from_cell(&cell)?,
            CoffeeLpWalletIncomingMessage::Unknown(_)
        ));
        let mut short = TonCell::builder();
        short.write_num(&u32::MAX, 32)?;
        short.write_num(&op, 32)?;
        assert!(bounce::CoffeeLpWalletBounceMessage::from_cell(&short.build()?).is_err());
    }
    let mut b = TonCell::builder();
    b.write_num(&0xdeadbeefu32, 32)?;
    b.write_bit(true)?;
    b.write_ref(empty)?;
    let unknown = b.build()?;
    assert!(matches!(
        bounce::CoffeeLpWalletBounceMessage::from_cell(&unknown)?,
        bounce::CoffeeLpWalletBounceMessage::Unknown(_)
    ));
    assert_eq!(bounce::CoffeeLpWalletBounceMessage::from_cell(&unknown)?.to_cell()?, unknown);
    Ok(())
}
#[test]
fn test_extra_currency_unsigned_ids_match_observed_vault_action_encoding() -> anyhow::Result<()> {
    let data: serde_json::Value = serde_json::from_str(include_str!("fixtures/extra-currency-observations.json"))?;
    for row in data["vectors"].as_array().ok_or_else(|| anyhow::anyhow!("missing vectors"))? {
        let id = u32::try_from(row["id"].as_u64().ok_or_else(|| anyhow::anyhow!("missing id"))?)?;
        let currency = CurrencyCollection {
            coins: TLBCoins::new(0),
            other: HashMap::from([(TonExtraCurrencyId::new(id), VarLenBytes::new(num_bigint::BigUint::from(5u8), 8))]),
        };
        assert_hash(&currency.to_cell()?, &row["currency_hash_hex"])?;
        // Coffee's asset tag and the message currency dictionary share the exact u32 ID.
        let asset = CoffeeExtraAsset { id };
        let cell = asset.to_cell()?;
        let mut p = cell.parser();
        assert_eq!(p.read_num::<u8>(2)?, 2);
        assert_eq!(p.read_num::<u32>(32)?, id);
        p.ensure_empty()?;
    }
    Ok(())
}
#[test]
fn test_factory_nullable_ownership_matches_executed_updates() -> anyhow::Result<()> {
    use super::test_schema_vectors::cell_for;
    let mut storage = CoffeeFactoryStorage::from_cell(&cell_for("factory/CoffeeFactoryStorage/0")?)?;
    assert!(storage.admin_address.is_none());
    assert!(storage.withdrawer_address.is_none());
    storage.admin_address = None;
    storage.withdrawer_address = None;
    let cell = storage.to_cell()?;
    let mut p = cell.parser();
    assert_eq!(p.read_num::<u8>(4)?, 0);
    let updates = [
        (
            CoffeeFactoryUpdateWithdrawerMsg {
                query_id: 77,
                address: None,
            }
            .to_cell()?,
            "96ae210b18312a9c68c0cab338d8a92912e98aac856c0e43c420a8807cf20617",
        ),
        (
            CoffeeFactoryUpdateAdminMsg {
                query_id: 78,
                address: None,
            }
            .to_cell()?,
            "1b87d39564c3bc35ec1f9f9be6c9f2b6a0a196fa3a2f0c58ffcd850b0b61febf",
        ),
    ];
    // Values below are frozen from original VM request hashes, not roundtrip output.
    for (body, hash) in updates {
        assert_eq!(format!("{}", body.hash()?).to_lowercase(), hash);
    }
    Ok(())
}
