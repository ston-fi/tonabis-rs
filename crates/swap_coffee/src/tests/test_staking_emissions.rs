use crate::contracts::staking::CoffeeStakingPeriod;
use crate::contracts::staking_item::storage::CoffeeStakingAssetData;
use crate::contracts::staking_master::msgs::CoffeeStakingMasterDepositLogMsg as DepositLog;
use crate::contracts::staking_master::msgs::CoffeeStakingMasterDepositSummary;
use crate::contracts::staking_master::msgs::CoffeeStakingMasterIncomingMessage;
use crate::contracts::staking_master::msgs::CoffeeStakingMasterOutgoingMessage;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::TLBEither;

use std::collections::HashMap;
use std::str::FromStr;

use ton::ton_core::cell::TonHash;
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
#[test]
fn test_deposit_log_matches_captured_emission_hashes_and_role() -> anyhow::Result<()> {
    let data: serde_json::Value = serde_json::from_str(include_str!("fixtures/staking-observations.json"))?;
    let owner = TonAddress::from_str(
        data["deposit_log_owner"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing owner"))?,
    )?;
    let points = data["points"].as_array().ok_or_else(|| anyhow::anyhow!("missing points"))?;
    for (i, row) in points.iter().enumerate() {
        let number = |index: usize| -> anyhow::Result<u64> {
            row[index].as_u64().ok_or_else(|| anyhow::anyhow!("missing number"))
        };
        let mut wallet_hash = [0u8; 32];
        wallet_hash[31] = number(0)? as u8;
        let wallet = TonAddress::new(0, TonHash::from_slice(&wallet_hash)?);
        let log = DepositLog {
            query_id: number(0)?,
            nft_id: 0,
            owner,
            summary: TLBRef::new(CoffeeStakingMasterDepositSummary {
                total_points: TLBCoins::new(number(4)?.into()),
                end_timestamp: 1_700_012_345,
                period_id: 7,
            }),
            asset_data: Some(TLBRef::new(CoffeeStakingAssetData {
                wallet: Some(wallet),
                amount: TLBCoins::new(number(1)?.into()),
            })),
            periods: HashMap::from([(
                7,
                TLBRef::new(CoffeeStakingPeriod {
                    duration: 12345,
                    percentage: number(3)?,
                }),
            )]),
        };
        let cell = log.to_cell()?;
        assert_eq!(
            format!("{}", cell.hash()?).to_lowercase(),
            data["deposit_log_hashes"][i]["hash_hex"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("missing hash"))?
        );
        assert!(matches!(
            CoffeeStakingMasterOutgoingMessage::from_cell(&cell)?,
            CoffeeStakingMasterOutgoingMessage::DepositLog(_)
        ));
        // Same opcode incoming request has incompatible fields, never guessed by role.
        assert!(CoffeeStakingMasterIncomingMessage::from_cell(&cell).is_err());
        let mut bad = TonCell::builder();
        bad.write_cell(&cell)?;
        bad.write_bit(true)?;
        assert!(DepositLog::from_cell(&bad.build()?).is_ok());
    }
    Ok(())
}
#[test]
fn test_initial_nft_data_matches_master_created_state_init() -> anyhow::Result<()> {
    use crate::contracts::staking_item::storage::CoffeeStakingItemInitialData as InitialData;
    use crate::contracts::staking_item::storage::CoffeeStakingItemStorage;
    let data: serde_json::Value = serde_json::from_str(include_str!("fixtures/staking-initialization.json"))?;
    for row in data["vectors"].as_array().ok_or_else(|| anyhow::anyhow!("missing vectors"))? {
        let master = TonAddress::from_str(row["master"].as_str().ok_or_else(|| anyhow::anyhow!("missing master"))?)?;
        let initial = InitialData {
            nft_id: row["nft_id"].as_u64().ok_or_else(|| anyhow::anyhow!("missing id"))?,
            master,
        };
        let cell = initial.to_cell()?;
        assert_eq!(
            format!("{}", cell.hash()?).to_lowercase(),
            row["data_hash_hex"].as_str().ok_or_else(|| anyhow::anyhow!("missing hash"))?
        );
        assert_eq!(InitialData::from_cell(&cell)?, initial);
        assert!(CoffeeStakingItemStorage::from_cell(&cell).is_err());
    }
    Ok(())
}
#[test]
fn test_single_item_master_initial_identity_and_internal_emissions() -> anyhow::Result<()> {
    use crate::contracts::staking_master::msgs::CoffeeStakingMasterExcessesMsg as Excesses;
    use crate::contracts::staking_master::msgs::CoffeeStakingMasterInternalMessage;
    use crate::contracts::staking_master::msgs::CoffeeStakingMasterJettonTransferMsg as JettonTransfer;
    use crate::contracts::staking_master::msgs::CoffeeStakingMasterUnlockToVaultMsg as UnlockToVault;
    use crate::contracts::staking_master::single_item::storage::CoffeeStakingMasterSingleItemInitialData as InitialData;

    use crate::contracts::staking_vault::msgs::CoffeeStakingVaultIncomingMessage;

    use ton::ton_core::cell::TonCell;
    let data: serde_json::Value = serde_json::from_str(include_str!("fixtures/staking-flow-observations.json"))?;
    let address = |v: &serde_json::Value| -> anyhow::Result<TonAddress> {
        Ok(TonAddress::from_str(
            v.as_str().ok_or_else(|| anyhow::anyhow!("missing address"))?,
        )?)
    };
    let initial = InitialData {
        vault: address(&data["master_initial"]["vault"])?,
        item_code: TLBRef::new(TonCell::empty().clone()),
    };
    let cell = initial.to_cell()?;
    assert_eq!(
        format!("{}", cell.hash()?).to_lowercase(),
        data["master_initial"]["data_hash_hex"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing initial hash"))?
    );
    assert_eq!(InitialData::from_cell(&cell)?, initial);
    assert_eq!(
        initial.state_init(TonCell::empty())?.derive_address(0)?,
        TonAddress::from_str(
            data["master_initial"]["master_address"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("missing master address"))?
        )?
    );
    let owner = address(&data["flow_owner"])?;
    let wallet = TonAddress::new(0, TonHash::ZERO);
    let unlock = UnlockToVault {
        query_id: 77,
        wallet,
        amount: TLBCoins::new(13),
        owner,
    }
    .to_cell()?;
    assert!(matches!(
        CoffeeStakingVaultIncomingMessage::from_cell(&unlock)?,
        CoffeeStakingVaultIncomingMessage::MasterUnlock(_)
    ));
    let transfer = JettonTransfer {
        query_id: 77,
        amount: TLBCoins::new(13),
        destination: owner,
        response_destination: Some(owner),
        custom_payload: None,
        forward_ton_amount: TLBCoins::new(0),
        forward_payload: TLBEither::Left(TonCell::empty().clone()),
    }
    .to_cell()?;
    let excess = Excesses { query_id: 77 }.to_cell()?;
    for revision in data["internal_emissions"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("missing revisions"))?
    {
        for row in revision["hashes"].as_array().ok_or_else(|| anyhow::anyhow!("missing hashes"))? {
            let id = row["id"].as_u64().ok_or_else(|| anyhow::anyhow!("missing id"))?;
            let body = match id {
                0 => &unlock,
                1 if revision["log"] == "probe-single-item-master-final.log" => TonCell::empty(),
                1 => &excess,
                2 => &transfer,
                _ => anyhow::bail!("unknown flow"),
            };
            assert_eq!(
                format!("{}", body.hash()?).to_lowercase(),
                row["hash_hex"].as_str().ok_or_else(|| anyhow::anyhow!("missing hash"))?
            );
            assert_eq!(CoffeeStakingMasterInternalMessage::from_cell(body)?.to_cell()?, *body);
        }
    }
    Ok(())
}
