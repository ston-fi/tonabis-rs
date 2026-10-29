use crate::contracts::cross_dex::msgs::CoffeeCrossDexIncomingMessage;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexJettonTransferMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexPtonTonTransferMsg;
use crate::contracts::cross_dex::storage::CoffeeCrossDexStorage;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskIncomingMessage;
use crate::contracts::cross_dex_bidask::storage::CoffeeCrossDexBidaskStorage;
use crate::contracts::cross_dex_legacy::msgs::CoffeeCrossDexLegacyIncomingMessage;
use crate::contracts::cross_dex_legacy::storage::CoffeeCrossDexLegacyStorage;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryIncomingMessage;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryOutgoingMessage;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorExcessesMsg;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorIncomingMessage;
use crate::contracts::mev_protector::storage::CoffeeMevProtectorStorage;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacyIncomingMessage;
use crate::contracts::mev_protector_legacy::storage::CoffeeMevProtectorLegacyStorage;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorIncomingMessage;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorOutgoingMessage;
use crate::contracts::staking_master::single_item::storage::CoffeeStakingMasterSingleItemStorage;
use crate::contracts::staking_master::storage::CoffeeStakingMasterStorage;
use crate::contracts::staking_vault::storage::CoffeeStakingVaultStorage;
use crate::contracts::vault_jetton;
use crate::contracts::vault_jetton::msgs::CoffeeTakeWalletAddressMsg;
use crate::contracts::vault_jetton::msgs::CoffeeTransferNotificationMsg;
use crate::contracts::vault_jetton::msgs::CoffeeVaultJettonForwardPayload;
use crate::contracts::vault_native::msgs::CoffeePayoutInternalMsg;
use crate::contracts::vault_native::msgs::CoffeeSwapNativeMsg;
use crate::contracts::vault_proof::CoffeeVaultSenderIdentity;
use fastnum::U256;
use std::collections::BTreeSet;
use ton::ton_core::types::tlb_core::TLBCoins;

use ton::ton_core::cell::TonCell;
use ton::ton_core::traits::tlb::TLB;
#[test]
fn test_original_executable_flow_message_layouts() -> anyhow::Result<()> {
    let data: serde_json::Value = serde_json::from_str(include_str!("fixtures/original-flow-messages.json"))?;
    let vectors = data["vectors"].as_array().ok_or_else(|| anyhow::anyhow!("missing vectors"))?;
    assert_eq!(
        vectors.len(),
        98,
        "captured flow fixtures must not be empty or omit a required scenario"
    );
    for (prefix, count) in [
        ("escrow-", 22),
        ("aux-", 53),
        ("jetton-envelope-", 8),
        ("nullable-aux-data-", 5),
        ("nullable-staking-data-", 4),
        ("wallet-response-", 6),
    ] {
        assert_eq!(
            vectors
                .iter()
                .filter(|row| row["id"].as_str().is_some_and(|id| id.starts_with(prefix)))
                .count(),
            count,
            "missing captured scenario group {prefix}"
        );
    }
    let mut seen = BTreeSet::new();
    for row in vectors {
        let id = row["id"].as_str().ok_or_else(|| anyhow::anyhow!("missing id"))?;
        anyhow::ensure!(seen.insert(id), "duplicate original flow fixture: {id}");
        let cell = TonCell::from_boc_hex(row["boc_hex"].as_str().ok_or_else(|| anyhow::anyhow!("missing boc"))?)?;
        if let Some(rest) = id.strip_prefix("nullable-aux-data-") {
            let index: u8 = rest.parse()?;
            match index {
                0 => {
                    let v = CoffeeCrossDexStorage::from_cell(&cell)?;
                    assert!(v.admin_address.is_none());
                    assert_eq!(v.to_cell()?, cell);
                },
                1 => {
                    let v = CoffeeCrossDexLegacyStorage::from_cell(&cell)?;
                    assert!(v.admin_address.is_none());
                    assert_eq!(v.to_cell()?, cell);
                },
                2 => {
                    let v = CoffeeCrossDexBidaskStorage::from_cell(&cell)?;
                    assert!(v.admin_address.is_none());
                    assert_eq!(v.to_cell()?, cell);
                },
                3 => {
                    let v = CoffeeMevProtectorStorage::from_cell(&cell)?;
                    assert!(v.admin_address.is_none());
                    assert_eq!(v.to_cell()?, cell);
                },
                4 => {
                    let v = CoffeeMevProtectorLegacyStorage::from_cell(&cell)?;
                    assert!(v.admin_address.is_none());
                    assert_eq!(v.to_cell()?, cell);
                },
                _ => anyhow::bail!("unknown owner revision"),
            }
            continue;
        }
        if let Some(rest) = id.strip_prefix("nullable-staking-data-") {
            match rest {
                "0" | "1" => {
                    let v = CoffeeStakingMasterStorage::from_cell(&cell)?;
                    assert!(v.initialized);
                    assert!(v.owner.is_none() && v.distributor.is_none());
                    assert_eq!(v.to_cell()?, cell);
                },
                "2" => {
                    let v = CoffeeStakingMasterSingleItemStorage::from_cell(&cell)?;
                    assert!(v.initialized);
                    assert!(v.owner.is_none() && v.distributor.is_none());
                    assert_eq!(v.to_cell()?, cell);
                },
                "3" => {
                    let v = CoffeeStakingVaultStorage::from_cell(&cell)?;
                    assert!(v.initialized);
                    assert!(v.owner.is_none());
                    assert_eq!(v.to_cell()?, cell);
                },
                _ => anyhow::bail!("unknown staking owner revision"),
            }
            continue;
        }
        if let Some(rest) = id.strip_prefix("wallet-response-") {
            use vault_jetton::msgs::CoffeeVaultJettonOwnerResponse;
            let v = CoffeeTakeWalletAddressMsg::from_cell(&cell)?;
            assert_eq!(v.query_id, 77);
            match rest.rsplit('-').next() {
                Some("0") => assert!(matches!(v.owner_response, CoffeeVaultJettonOwnerResponse::Omitted)),
                Some("1") => assert!(matches!(v.owner_response, CoffeeVaultJettonOwnerResponse::NotIncluded)),
                Some("2") => assert!(matches!(v.owner_response, CoffeeVaultJettonOwnerResponse::Included(_))),
                _ => anyhow::bail!("unknown wallet response"),
            }
            assert_eq!(v.to_cell()?, cell);
            continue;
        }
        if let Some(rest) = id.strip_prefix("jetton-envelope-") {
            let mode = rest.rsplit('-').next().ok_or_else(|| anyhow::anyhow!("missing mode"))?;
            if mode == "2" {
                assert!(CoffeeTransferNotificationMsg::from_cell(&cell).is_err());
                continue;
            }
            let decoded = CoffeeTransferNotificationMsg::from_cell(&cell)?;
            assert_eq!(decoded.query_id, 77);
            assert_eq!(
                decoded.forward_payload_selector,
                match mode {
                    "0" => Some(true),
                    "1" => None,
                    "3" => Some(false),
                    _ => anyhow::bail!("unknown mode"),
                }
            );
            assert!(matches!(
                &*decoded.forward_payload,
                CoffeeVaultJettonForwardPayload::CoffeeSwapJetton(_)
            ));
            assert_eq!(decoded.to_cell()?, cell);
            continue;
        }
        let parts: Vec<_> = id.split('-').collect();
        let number: u32 = parts[1].parse()?;
        if parts[0] == "escrow" {
            let output = id.ends_with("output");
            if output && number % 10 != 2 {
                let payout = CoffeePayoutInternalMsg::from_cell(&cell)?;
                let proof = payout.proof.as_ref().ok_or_else(|| anyhow::anyhow!("missing payout proof"))?;
                match (&proof.identity, number < 10) {
                    (CoffeeVaultSenderIdentity::Creator { .. }, true)
                    | (CoffeeVaultSenderIdentity::Depository { .. }, false) => {},
                    _ => anyhow::bail!("wrong escrow identity tag"),
                }
                assert_eq!(payout.to_cell()?, cell);
            }
            if number < 10 {
                if output {
                    assert_eq!(CoffeePoolCreatorOutgoingMessage::from_cell(&cell)?.to_cell()?, cell);
                } else {
                    assert_eq!(CoffeePoolCreatorIncomingMessage::from_cell(&cell)?.to_cell()?, cell);
                }
            } else if output {
                assert_eq!(CoffeeLiquidityDepositoryOutgoingMessage::from_cell(&cell)?.to_cell()?, cell);
            } else {
                assert_eq!(CoffeeLiquidityDepositoryIncomingMessage::from_cell(&cell)?.to_cell()?, cell);
            }
            continue;
        }
        if parts[0] != "aux" {
            anyhow::bail!("unknown fixture {id}");
        }
        if id.ends_with("output") {
            let mut p = cell.parser();
            let opcode = p.read_num::<u32>(32)?;
            match opcode {
                0xc0ffee00 => {
                    let decoded = CoffeeSwapNativeMsg::from_cell(&cell)?;
                    assert_eq!(decoded.query_id, 66);
                    assert_eq!(decoded.step.pool_address_hash, U256::from(123u8));
                    assert_eq!(decoded.to_cell()?, cell);
                },
                0x0f8a7ea5 => {
                    let decoded = CoffeeCrossDexJettonTransferMsg::from_cell(&cell)?;
                    assert_eq!(decoded.query_id, 77);
                    assert_eq!(decoded.amount, TLBCoins::new(13));
                    assert_eq!(decoded.to_cell()?, cell);
                },
                0xd53276db => {
                    assert_eq!(CoffeeMevProtectorExcessesMsg::from_cell(&cell)?.query_id, 77);
                },
                _ => anyhow::bail!("unclassified original response opcode {opcode:x}"),
            }
            continue;
        }
        // pTON false selector is an absent Maybe-ref; ordinary remaining tail is tolerated.
        if number % 100 == 7 {
            assert!(CoffeeCrossDexPtonTonTransferMsg::from_cell(&cell)?.forward_payload.is_none());
            continue;
        }
        match number / 100 {
            0 => assert_eq!(CoffeeCrossDexIncomingMessage::from_cell(&cell)?.to_cell()?, cell),
            1 => assert_eq!(CoffeeCrossDexLegacyIncomingMessage::from_cell(&cell)?.to_cell()?, cell),
            2 => assert_eq!(CoffeeCrossDexBidaskIncomingMessage::from_cell(&cell)?.to_cell()?, cell),
            4 => assert_eq!(CoffeeMevProtectorIncomingMessage::from_cell(&cell)?.to_cell()?, cell),
            5 => assert_eq!(CoffeeMevProtectorLegacyIncomingMessage::from_cell(&cell)?.to_cell()?, cell),
            _ => anyhow::bail!("unknown original flow"),
        }
    }
    Ok(())
}
