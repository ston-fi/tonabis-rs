use std::collections::BTreeSet;
// Layout vectors originate in the independent Python bit/BOC writer.
use crate::contracts::{booster, catalog_range, clmm, cpmm, legacy_pool, mempad, order_factory, order_new, order_old};
use ton::ton_core::{cell::TonCell, traits::tlb::TLB};
#[test]
fn test_independent_sdk_wire_layouts() -> anyhow::Result<()> {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("fixtures/wire-vectors.json"))?;
    let rows = fixture["vectors"].as_array().ok_or_else(|| anyhow::anyhow!("vectors"))?;
    anyhow::ensure!(rows.len() == 315, "wire corpus must cover all 315 records");
    let mut seen = BTreeSet::new();
    for row in rows {
        let module = row["module"].as_str().ok_or_else(|| anyhow::anyhow!("module"))?;
        let name = row["name"].as_str().ok_or_else(|| anyhow::anyhow!("name"))?;
        anyhow::ensure!(seen.insert((module, name)), "duplicate wire fixture: {module}/{name}");
        let cell = TonCell::from_boc_hex(row["boc"].as_str().ok_or_else(|| anyhow::anyhow!("boc"))?)?;
        match (module, name) {
            ("cpmm", "FactoryStorage") => {
                let value = cpmm::factory::storage::MoonCpmmFactoryStorage::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "JettonBoostPayload") => {
                let value = cpmm::msgs::MoonCpmmJettonBoostPayloadMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "JettonDepositLiquidityPayload") => {
                let value = cpmm::msgs::MoonCpmmJettonDepositLiquidityPayloadMsg::from_cell(&cell)?;
                assert_eq!(
                    value.min_lp_out.to_u128().to_string(),
                    "1007",
                    "cpmm::msgs::MoonCpmmJettonDepositLiquidityPayloadMsg::min_lp_out"
                );
                assert!(!value.locked);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "JettonSwapFulfillPayload") => {
                let value = cpmm::msgs::MoonCpmmJettonSwapFulfillPayloadMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "JettonSwapParams") => {
                let value = cpmm::msgs::MoonCpmmJettonSwapParams::from_cell(&cell)?;
                assert_eq!(
                    value.min_out.to_u128().to_string(),
                    "2007",
                    "cpmm::msgs::MoonCpmmJettonSwapParams::min_out"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "JettonSwapPayload") => {
                let value = cpmm::msgs::MoonCpmmJettonSwapPayloadMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "LpAccountInitData") => {
                let value = cpmm::account::storage::MoonCpmmLpAccountInitData::from_cell(&cell)?;
                assert_eq!(
                    hex::encode(
                        crate::contracts::address::derive(TonCell::empty().clone(), &value)?
                            .hash
                            .as_slice()
                    ),
                    row["init_hash"].as_str().ok_or_else(|| anyhow::anyhow!("init hash"))?
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "LpAccountStorage") => {
                let value = cpmm::account::storage::MoonCpmmLpAccountStorage::from_cell(&cell)?;
                assert_eq!(
                    value.x_amount.to_u128().to_string(),
                    "3007",
                    "cpmm::account::storage::MoonCpmmLpAccountStorage::x_amount"
                );
                assert_eq!(
                    value.y_amount.to_u128().to_string(),
                    "4007",
                    "cpmm::account::storage::MoonCpmmLpAccountStorage::y_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "LpPositionContent") => {
                let value = cpmm::position::storage::MoonCpmmLpPositionContent::from_cell(&cell)?;
                assert_eq!(
                    value.lp.to_u128().to_string(),
                    "1007",
                    "cpmm::position::storage::MoonCpmmLpPositionContent::lp"
                );
                assert_eq!(
                    value.x_fees_lp.to_string(),
                    "5",
                    "cpmm::position::storage::MoonCpmmLpPositionContent::x_fees_lp"
                );
                assert_eq!(
                    value.y_fees_lp.to_string(),
                    "6",
                    "cpmm::position::storage::MoonCpmmLpPositionContent::y_fees_lp"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "LpPositionInitData") => {
                let value = cpmm::position::storage::MoonCpmmLpPositionInitData::from_cell(&cell)?;
                assert_eq!(
                    hex::encode(
                        crate::contracts::address::derive(TonCell::empty().clone(), &value)?
                            .hash
                            .as_slice()
                    ),
                    row["init_hash"].as_str().ok_or_else(|| anyhow::anyhow!("init hash"))?
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "LpPositionStorage") => {
                let value = cpmm::position::storage::MoonCpmmLpPositionStorage::from_cell(&cell)?;
                assert!(!value.frozen);
                assert!(value.locked);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "LpSplit") => {
                let value = cpmm::msgs::MoonCpmmLpSplit::from_cell(&cell)?;
                assert_eq!(value.a_bps.to_string(), "5", "cpmm::msgs::MoonCpmmLpSplit::a_bps");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgBoostPool") => {
                let value = cpmm::msgs::MoonCpmmBoostPoolMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "cpmm::msgs::MoonCpmmBoostPoolMsg::query_id");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgCancelDeposit") => {
                let value = cpmm::msgs::MoonCpmmCancelDepositMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmCancelDepositMsg::query_id"
                );
                assert_eq!(
                    value.x_amount.to_u128().to_string(),
                    "3007",
                    "cpmm::msgs::MoonCpmmCancelDepositMsg::x_amount"
                );
                assert_eq!(
                    value.y_amount.to_u128().to_string(),
                    "4007",
                    "cpmm::msgs::MoonCpmmCancelDepositMsg::y_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgCollectProtocolFees") => {
                let value = cpmm::msgs::MoonCpmmCollectProtocolFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmCollectProtocolFeesMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgCreatePool") => {
                let value = cpmm::msgs::MoonCpmmCreatePoolMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "cpmm::msgs::MoonCpmmCreatePoolMsg::query_id");
                assert_eq!(value.fee_bps.to_string(), "7", "cpmm::msgs::MoonCpmmCreatePoolMsg::fee_bps");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgDeployPool") => {
                let value = cpmm::msgs::MoonCpmmDeployPoolMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "cpmm::msgs::MoonCpmmDeployPoolMsg::query_id");
                assert_eq!(value.lp_fee.to_string(), "5", "cpmm::msgs::MoonCpmmDeployPoolMsg::lp_fee");
                assert_eq!(
                    value.protocol_fee.to_string(),
                    "6",
                    "cpmm::msgs::MoonCpmmDeployPoolMsg::protocol_fee"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgDeployPosition") => {
                let value = cpmm::msgs::MoonCpmmDeployPositionMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmDeployPositionMsg::query_id"
                );
                assert!(value.locked);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgDepositLiquidity") => {
                let value = cpmm::msgs::MoonCpmmDepositLiquidityMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmDepositLiquidityMsg::query_id"
                );
                assert_eq!(
                    value.min_lp_out.to_u128().to_string(),
                    "2007",
                    "cpmm::msgs::MoonCpmmDepositLiquidityMsg::min_lp_out"
                );
                assert!(value.locked);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgDepositLiquidityRecord") => {
                let value = cpmm::msgs::MoonCpmmDepositLiquidityRecordMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmDepositLiquidityRecordMsg::query_id"
                );
                assert!(!value.x_in);
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "3007",
                    "cpmm::msgs::MoonCpmmDepositLiquidityRecordMsg::amount"
                );
                assert_eq!(
                    value.min_lp_out.to_u128().to_string(),
                    "4007",
                    "cpmm::msgs::MoonCpmmDepositLiquidityRecordMsg::min_lp_out"
                );
                assert!(value.locked);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgInternalCollectProtocolFees") => {
                let value = cpmm::msgs::MoonCpmmInternalCollectProtocolFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmInternalCollectProtocolFeesMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgInternalUpdatePoolFees") => {
                let value = cpmm::msgs::MoonCpmmInternalUpdatePoolFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmInternalUpdatePoolFeesMsg::query_id"
                );
                assert_eq!(
                    value.new_lp_fee.to_string(),
                    "5",
                    "cpmm::msgs::MoonCpmmInternalUpdatePoolFeesMsg::new_lp_fee"
                );
                assert_eq!(
                    value.new_protocol_fee.to_string(),
                    "6",
                    "cpmm::msgs::MoonCpmmInternalUpdatePoolFeesMsg::new_protocol_fee"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgInternalUpdatePoolWallets") => {
                let value = cpmm::msgs::MoonCpmmInternalUpdatePoolWalletsMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmInternalUpdatePoolWalletsMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgInternalUpgradePool") => {
                let value = cpmm::msgs::MoonCpmmInternalUpgradePoolMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmInternalUpgradePoolMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgInternalWithdrawFees") => {
                let value = cpmm::msgs::MoonCpmmInternalWithdrawFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmInternalWithdrawFeesMsg::query_id"
                );
                assert_eq!(
                    value.lp_pos.to_u128().to_string(),
                    "3007",
                    "cpmm::msgs::MoonCpmmInternalWithdrawFeesMsg::lp_pos"
                );
                assert_eq!(
                    value.x_fees_pos.to_string(),
                    "7",
                    "cpmm::msgs::MoonCpmmInternalWithdrawFeesMsg::x_fees_pos"
                );
                assert_eq!(
                    value.y_fees_pos.to_string(),
                    "8",
                    "cpmm::msgs::MoonCpmmInternalWithdrawFeesMsg::y_fees_pos"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgInternalWithdrawFeesUpdate") => {
                let value = cpmm::msgs::MoonCpmmInternalWithdrawFeesUpdateMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmInternalWithdrawFeesUpdateMsg::query_id"
                );
                assert_eq!(
                    value.x_fees_lp.to_string(),
                    "5",
                    "cpmm::msgs::MoonCpmmInternalWithdrawFeesUpdateMsg::x_fees_lp"
                );
                assert_eq!(
                    value.y_fees_lp.to_string(),
                    "6",
                    "cpmm::msgs::MoonCpmmInternalWithdrawFeesUpdateMsg::y_fees_lp"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgInternalWithdrawLiquidity") => {
                let value = cpmm::msgs::MoonCpmmInternalWithdrawLiquidityMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmInternalWithdrawLiquidityMsg::query_id"
                );
                assert_eq!(
                    value.lp_pos.to_u128().to_string(),
                    "3007",
                    "cpmm::msgs::MoonCpmmInternalWithdrawLiquidityMsg::lp_pos"
                );
                assert_eq!(
                    value.x_fees_pos.to_string(),
                    "7",
                    "cpmm::msgs::MoonCpmmInternalWithdrawLiquidityMsg::x_fees_pos"
                );
                assert_eq!(
                    value.y_fees_pos.to_string(),
                    "8",
                    "cpmm::msgs::MoonCpmmInternalWithdrawLiquidityMsg::y_fees_pos"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgJettonTransfer") => {
                let value = cpmm::msgs::MoonCpmmJettonTransferMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmJettonTransferMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "cpmm::msgs::MoonCpmmJettonTransferMsg::amount"
                );
                assert_eq!(
                    value.forward_ton_amount.to_u128().to_string(),
                    "6007",
                    "cpmm::msgs::MoonCpmmJettonTransferMsg::forward_ton_amount"
                );
                assert!(!value.have_payload);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgJettonTransferWithPayload") => {
                let value = cpmm::msgs::MoonCpmmJettonTransferWithPayloadMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmJettonTransferWithPayloadMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "cpmm::msgs::MoonCpmmJettonTransferWithPayloadMsg::amount"
                );
                assert_eq!(
                    value.forward_ton_amount.to_u128().to_string(),
                    "6007",
                    "cpmm::msgs::MoonCpmmJettonTransferWithPayloadMsg::forward_ton_amount"
                );
                assert!(value.have_payload);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgLockLiquidity") => {
                let value = cpmm::msgs::MoonCpmmLockLiquidityMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmLockLiquidityMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgProvideLiquidity") => {
                let value = cpmm::msgs::MoonCpmmProvideLiquidityMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmProvideLiquidityMsg::query_id"
                );
                assert_eq!(
                    value.x_amount.to_u128().to_string(),
                    "3007",
                    "cpmm::msgs::MoonCpmmProvideLiquidityMsg::x_amount"
                );
                assert_eq!(
                    value.y_amount.to_u128().to_string(),
                    "4007",
                    "cpmm::msgs::MoonCpmmProvideLiquidityMsg::y_amount"
                );
                assert_eq!(
                    value.min_lp_out.to_u128().to_string(),
                    "5007",
                    "cpmm::msgs::MoonCpmmProvideLiquidityMsg::min_lp_out"
                );
                assert!(!value.locked);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgProvidePoolAddress") => {
                let value = cpmm::msgs::MoonCpmmProvidePoolAddressMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmProvidePoolAddressMsg::query_id"
                );
                assert_eq!(
                    value.fee_bps.to_string(),
                    "7",
                    "cpmm::msgs::MoonCpmmProvidePoolAddressMsg::fee_bps"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgProvidePoolReady") => {
                let value = cpmm::msgs::MoonCpmmProvidePoolReadyMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmProvidePoolReadyMsg::query_id"
                );
                assert_eq!(
                    value.attempt.to_string(),
                    "5",
                    "cpmm::msgs::MoonCpmmProvidePoolReadyMsg::attempt"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgProvideWalletAddress") => {
                let value = cpmm::msgs::MoonCpmmProvideWalletAddressMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmProvideWalletAddressMsg::query_id"
                );
                assert!(!value.include_address);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgSwap") => {
                let value = cpmm::msgs::MoonCpmmSwapMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "cpmm::msgs::MoonCpmmSwapMsg::query_id");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgSwapFulfill") => {
                let value = cpmm::msgs::MoonCpmmSwapFulfillMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "cpmm::msgs::MoonCpmmSwapFulfillMsg::query_id");
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "cpmm::msgs::MoonCpmmSwapFulfillMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgTakePoolAddress") => {
                let value = cpmm::msgs::MoonCpmmTakePoolAddressMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmTakePoolAddressMsg::query_id"
                );
                assert_eq!(
                    value.fee_bps.to_string(),
                    "8",
                    "cpmm::msgs::MoonCpmmTakePoolAddressMsg::fee_bps"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgTakePoolReady") => {
                let value = cpmm::msgs::MoonCpmmTakePoolReadyMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmTakePoolReadyMsg::query_id"
                );
                assert_eq!(value.attempt.to_string(), "5", "cpmm::msgs::MoonCpmmTakePoolReadyMsg::attempt");
                assert!(value.is_ready);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgTakeWalletAddress") => {
                let value = cpmm::msgs::MoonCpmmTakeWalletAddressMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmTakeWalletAddressMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgTransferNotification") => {
                let value = cpmm::msgs::MoonCpmmTransferNotificationMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmTransferNotificationMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "cpmm::msgs::MoonCpmmTransferNotificationMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgUpdateFactoryFees") => {
                let value = cpmm::msgs::MoonCpmmUpdateFactoryFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmUpdateFactoryFeesMsg::query_id"
                );
                assert_eq!(
                    value.new_lp_fee.to_string(),
                    "5",
                    "cpmm::msgs::MoonCpmmUpdateFactoryFeesMsg::new_lp_fee"
                );
                assert_eq!(
                    value.new_protocol_fee.to_string(),
                    "6",
                    "cpmm::msgs::MoonCpmmUpdateFactoryFeesMsg::new_protocol_fee"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgUpdatePoolFees") => {
                let value = cpmm::msgs::MoonCpmmUpdatePoolFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmUpdatePoolFeesMsg::query_id"
                );
                assert_eq!(
                    value.new_lp_fee.to_string(),
                    "6",
                    "cpmm::msgs::MoonCpmmUpdatePoolFeesMsg::new_lp_fee"
                );
                assert_eq!(
                    value.new_protocol_fee.to_string(),
                    "7",
                    "cpmm::msgs::MoonCpmmUpdatePoolFeesMsg::new_protocol_fee"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgUpdatePoolWallets") => {
                let value = cpmm::msgs::MoonCpmmUpdatePoolWalletsMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmUpdatePoolWalletsMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgUpgradeFactory") => {
                let value = cpmm::msgs::MoonCpmmUpgradeFactoryMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmUpgradeFactoryMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgUpgradePool") => {
                let value = cpmm::msgs::MoonCpmmUpgradePoolMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "cpmm::msgs::MoonCpmmUpgradePoolMsg::query_id");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgWithdrawFees") => {
                let value = cpmm::msgs::MoonCpmmWithdrawFeesMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "cpmm::msgs::MoonCpmmWithdrawFeesMsg::query_id");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgWithdrawLiquidity") => {
                let value = cpmm::msgs::MoonCpmmWithdrawLiquidityMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmWithdrawLiquidityMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "cpmm::msgs::MoonCpmmWithdrawLiquidityMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "PoolInitData") => {
                let value = cpmm::pool::storage::MoonCpmmPoolInitData::from_cell(&cell)?;
                assert_eq!(
                    value.fee_bps.to_string(),
                    "6",
                    "cpmm::pool::storage::MoonCpmmPoolInitData::fee_bps"
                );
                assert_eq!(
                    hex::encode(
                        crate::contracts::address::derive(TonCell::empty().clone(), &value)?
                            .hash
                            .as_slice()
                    ),
                    row["init_hash"].as_str().ok_or_else(|| anyhow::anyhow!("init hash"))?
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "PoolMasters") => {
                let value = cpmm::pool::storage::MoonCpmmPoolMasters::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "PoolReserves") => {
                let value = cpmm::pool::storage::MoonCpmmPoolReserves::from_cell(&cell)?;
                assert_eq!(
                    value.x_reserve.to_u128().to_string(),
                    "1007",
                    "cpmm::pool::storage::MoonCpmmPoolReserves::x_reserve"
                );
                assert_eq!(
                    value.y_reserve.to_u128().to_string(),
                    "2007",
                    "cpmm::pool::storage::MoonCpmmPoolReserves::y_reserve"
                );
                assert_eq!(
                    value.x_fees_lp.to_string(),
                    "6",
                    "cpmm::pool::storage::MoonCpmmPoolReserves::x_fees_lp"
                );
                assert_eq!(
                    value.y_fees_lp.to_string(),
                    "7",
                    "cpmm::pool::storage::MoonCpmmPoolReserves::y_fees_lp"
                );
                assert_eq!(
                    value.x_fees_protocol.to_u128().to_string(),
                    "5007",
                    "cpmm::pool::storage::MoonCpmmPoolReserves::x_fees_protocol"
                );
                assert_eq!(
                    value.y_fees_protocol.to_u128().to_string(),
                    "6007",
                    "cpmm::pool::storage::MoonCpmmPoolReserves::y_fees_protocol"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "PoolStorage") => {
                let value = cpmm::pool::storage::MoonCpmmPoolStorage::from_cell(&cell)?;
                assert_eq!(
                    value.lp_fee.to_string(),
                    "7",
                    "cpmm::pool::storage::MoonCpmmPoolStorage::lp_fee"
                );
                assert_eq!(
                    value.protocol_fee.to_string(),
                    "8",
                    "cpmm::pool::storage::MoonCpmmPoolStorage::protocol_fee"
                );
                assert_eq!(
                    value.lp_total.to_u128().to_string(),
                    "6007",
                    "cpmm::pool::storage::MoonCpmmPoolStorage::lp_total"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "SwapResultJettonBody") => {
                let value = cpmm::msgs::MoonCpmmSwapResultJettonBodyMsg::from_cell(&cell)?;
                assert_eq!(value.op.to_string(), "4", "cpmm::msgs::MoonCpmmSwapResultJettonBodyMsg::op");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "SwapResultTonBody") => {
                let value = cpmm::msgs::MoonCpmmSwapResultTonBodyMsg::from_cell(&cell)?;
                assert_eq!(value.op.to_string(), "4", "cpmm::msgs::MoonCpmmSwapResultTonBodyMsg::op");
                assert_eq!(
                    value.query_id.to_string(),
                    "5",
                    "cpmm::msgs::MoonCpmmSwapResultTonBodyMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "3007",
                    "cpmm::msgs::MoonCpmmSwapResultTonBodyMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "TonSwapParams") => {
                let value = cpmm::msgs::MoonCpmmTonSwapParams::from_cell(&cell)?;
                assert_eq!(
                    value.swap_amount.to_u128().to_string(),
                    "1007",
                    "cpmm::msgs::MoonCpmmTonSwapParams::swap_amount"
                );
                assert_eq!(
                    value.min_out.to_u128().to_string(),
                    "3007",
                    "cpmm::msgs::MoonCpmmTonSwapParams::min_out"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgLiquidityResponse") => {
                let value = cpmm::msgs::MoonCpmmLiquidityResponseMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmLiquidityResponseMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("cpmm", "MsgDepositResponse") => {
                let value = cpmm::msgs::MoonCpmmDepositResponseMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "cpmm::msgs::MoonCpmmDepositResponseMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "Bin") => {
                let value = clmm::range::storage::MoonClmmBin::from_cell(&cell)?;
                assert_eq!(value.l.to_string(), "4", "clmm::range::storage::MoonClmmBin::l");
                assert_eq!(value.x_fee.to_string(), "5", "clmm::range::storage::MoonClmmBin::x_fee");
                assert_eq!(value.y_fee.to_string(), "6", "clmm::range::storage::MoonClmmBin::y_fee");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "DepositBin") => {
                let value = clmm::msgs::MoonClmmDepositBin::from_cell(&cell)?;
                assert_eq!(value.x.to_u128().to_string(), "1007", "clmm::msgs::MoonClmmDepositBin::x");
                assert_eq!(value.y.to_u128().to_string(), "2007", "clmm::msgs::MoonClmmDepositBin::y");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "FactoryStorage") => {
                let value = clmm::factory::storage::MoonClmmFactoryStorage::from_cell(&cell)?;
                assert_eq!(
                    value.lp_fee.to_string(),
                    "5",
                    "clmm::factory::storage::MoonClmmFactoryStorage::lp_fee"
                );
                assert_eq!(
                    value.protocol_fee.to_string(),
                    "6",
                    "clmm::factory::storage::MoonClmmFactoryStorage::protocol_fee"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "JettonDepositLiquidityPayload") => {
                let value = clmm::msgs::MoonClmmJettonDepositLiquidityPayloadMsg::from_cell(&cell)?;
                assert!(value.is_single);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "JettonSwapFulfillPayload") => {
                let value = clmm::msgs::MoonClmmJettonSwapFulfillPayloadMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "JettonSwapParams") => {
                let value = clmm::msgs::MoonClmmJettonSwapParams::from_cell(&cell)?;
                assert_eq!(
                    value.sqrt_price_limit.to_string(),
                    "5",
                    "clmm::msgs::MoonClmmJettonSwapParams::sqrt_price_limit"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "JettonSwapPayload") => {
                let value = clmm::msgs::MoonClmmJettonSwapPayloadMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "LpAccInitData") => {
                let value = clmm::account::storage::MoonClmmLpAccInitData::from_cell(&cell)?;
                assert_eq!(
                    hex::encode(
                        crate::contracts::address::derive(TonCell::empty().clone(), &value)?
                            .hash
                            .as_slice()
                    ),
                    row["init_hash"].as_str().ok_or_else(|| anyhow::anyhow!("init hash"))?
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "LpAccountInitData") => {
                let value = clmm::account::storage::MoonClmmLpAccountInitData::from_cell(&cell)?;
                assert_eq!(
                    hex::encode(
                        crate::contracts::address::derive(TonCell::empty().clone(), &value)?
                            .hash
                            .as_slice()
                    ),
                    row["init_hash"].as_str().ok_or_else(|| anyhow::anyhow!("init hash"))?
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "LpAccountStorage") => {
                let value = clmm::account::storage::MoonClmmLpAccountStorage::from_cell(&cell)?;
                assert_eq!(
                    value.x_amount.to_u128().to_string(),
                    "3007",
                    "clmm::account::storage::MoonClmmLpAccountStorage::x_amount"
                );
                assert_eq!(
                    value.y_amount.to_u128().to_string(),
                    "4007",
                    "clmm::account::storage::MoonClmmLpAccountStorage::y_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "LpPositionInitData") => {
                let value = clmm::position::storage::MoonClmmLpPositionInitData::from_cell(&cell)?;
                assert_eq!(
                    value.nft_index.to_string(),
                    "5",
                    "clmm::position::storage::MoonClmmLpPositionInitData::nft_index"
                );
                assert_eq!(
                    hex::encode(
                        crate::contracts::address::derive(TonCell::empty().clone(), &value)?
                            .hash
                            .as_slice()
                    ),
                    row["init_hash"].as_str().ok_or_else(|| anyhow::anyhow!("init hash"))?
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "LpPositionStorage") => {
                let value = clmm::position::storage::MoonClmmLpPositionStorage::from_cell(&cell)?;
                assert_eq!(
                    value.nft_index.to_string(),
                    "5",
                    "clmm::position::storage::MoonClmmLpPositionStorage::nft_index"
                );
                assert!(value.locked);
                assert!(!value.pending);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgApplyCollectProtocolFees") => {
                let value = clmm::msgs::MoonClmmApplyCollectProtocolFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmApplyCollectProtocolFeesMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgApplyUpdateRangeFees") => {
                let value = clmm::msgs::MoonClmmApplyUpdateRangeFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmApplyUpdateRangeFeesMsg::query_id"
                );
                assert_eq!(
                    value.lp_fee.to_string(),
                    "5",
                    "clmm::msgs::MoonClmmApplyUpdateRangeFeesMsg::lp_fee"
                );
                assert_eq!(
                    value.protocol_fee.to_string(),
                    "6",
                    "clmm::msgs::MoonClmmApplyUpdateRangeFeesMsg::protocol_fee"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgApplyUpdateRangeWallets") => {
                let value = clmm::msgs::MoonClmmApplyUpdateRangeWalletsMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmApplyUpdateRangeWalletsMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgApplyUpgradeRange") => {
                let value = clmm::msgs::MoonClmmApplyUpgradeRangeMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmApplyUpgradeRangeMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgCancelDeposit") => {
                let value = clmm::msgs::MoonClmmCancelDepositMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmCancelDepositMsg::query_id"
                );
                assert_eq!(
                    value.x_amount.to_u128().to_string(),
                    "2007",
                    "clmm::msgs::MoonClmmCancelDepositMsg::x_amount"
                );
                assert_eq!(
                    value.y_amount.to_u128().to_string(),
                    "3007",
                    "clmm::msgs::MoonClmmCancelDepositMsg::y_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgCollectProtocolFees") => {
                let value = clmm::msgs::MoonClmmCollectProtocolFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmCollectProtocolFeesMsg::query_id"
                );
                assert_eq!(
                    value.range_num.to_string(),
                    "-6",
                    "clmm::msgs::MoonClmmCollectProtocolFeesMsg::range_num"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgCreatePool") => {
                let value = clmm::msgs::MoonClmmCreatePoolMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "clmm::msgs::MoonClmmCreatePoolMsg::query_id");
                assert_eq!(value.bin_step.to_string(), "7", "clmm::msgs::MoonClmmCreatePoolMsg::bin_step");
                assert_eq!(value.init_bin.to_string(), "-8", "clmm::msgs::MoonClmmCreatePoolMsg::init_bin");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgDeployLpPosition") => {
                let value = clmm::msgs::MoonClmmDeployLpPositionMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmDeployLpPositionMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgDeployPool") => {
                let value = clmm::msgs::MoonClmmDeployPoolMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "clmm::msgs::MoonClmmDeployPoolMsg::query_id");
                assert_eq!(value.init_bin.to_string(), "-5", "clmm::msgs::MoonClmmDeployPoolMsg::init_bin");
                assert_eq!(value.lp_fee.to_string(), "6", "clmm::msgs::MoonClmmDeployPoolMsg::lp_fee");
                assert_eq!(
                    value.protocol_fee.to_string(),
                    "7",
                    "clmm::msgs::MoonClmmDeployPoolMsg::protocol_fee"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgDeployRange") => {
                let value = clmm::msgs::MoonClmmDeployRangeMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "clmm::msgs::MoonClmmDeployRangeMsg::query_id");
                assert_eq!(value.bin_step.to_string(), "7", "clmm::msgs::MoonClmmDeployRangeMsg::bin_step");
                assert_eq!(value.lp_fee.to_string(), "8", "clmm::msgs::MoonClmmDeployRangeMsg::lp_fee");
                assert_eq!(
                    value.protocol_fee.to_string(),
                    "9",
                    "clmm::msgs::MoonClmmDeployRangeMsg::protocol_fee"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgDepositLiquidity") => {
                let value = clmm::msgs::MoonClmmDepositLiquidityMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmDepositLiquidityMsg::query_id"
                );
                assert_eq!(
                    value.deposit_amount.to_u128().to_string(),
                    "2007",
                    "clmm::msgs::MoonClmmDepositLiquidityMsg::deposit_amount"
                );
                assert!(value.is_single);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgDepositLiquidityRecord") => {
                let value = clmm::msgs::MoonClmmDepositLiquidityRecordMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmDepositLiquidityRecordMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "clmm::msgs::MoonClmmDepositLiquidityRecordMsg::amount"
                );
                assert!(value.x_in);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgInternalCollectProtocolFees") => {
                let value = clmm::msgs::MoonClmmInternalCollectProtocolFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmInternalCollectProtocolFeesMsg::query_id"
                );
                assert_eq!(
                    value.range_num.to_string(),
                    "-5",
                    "clmm::msgs::MoonClmmInternalCollectProtocolFeesMsg::range_num"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgInternalUpdateRangeFees") => {
                let value = clmm::msgs::MoonClmmInternalUpdateRangeFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmInternalUpdateRangeFeesMsg::query_id"
                );
                assert_eq!(
                    value.range_num.to_string(),
                    "-5",
                    "clmm::msgs::MoonClmmInternalUpdateRangeFeesMsg::range_num"
                );
                assert_eq!(
                    value.lp_fee.to_string(),
                    "6",
                    "clmm::msgs::MoonClmmInternalUpdateRangeFeesMsg::lp_fee"
                );
                assert_eq!(
                    value.protocol_fee.to_string(),
                    "7",
                    "clmm::msgs::MoonClmmInternalUpdateRangeFeesMsg::protocol_fee"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgInternalUpdateRangeWallets") => {
                let value = clmm::msgs::MoonClmmInternalUpdateRangeWalletsMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmInternalUpdateRangeWalletsMsg::query_id"
                );
                assert_eq!(
                    value.range_num.to_string(),
                    "-5",
                    "clmm::msgs::MoonClmmInternalUpdateRangeWalletsMsg::range_num"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgInternalUpgradePool") => {
                let value = clmm::msgs::MoonClmmInternalUpgradePoolMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmInternalUpgradePoolMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgInternalUpgradeRange") => {
                let value = clmm::msgs::MoonClmmInternalUpgradeRangeMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmInternalUpgradeRangeMsg::query_id"
                );
                assert_eq!(
                    value.range_num.to_string(),
                    "-5",
                    "clmm::msgs::MoonClmmInternalUpgradeRangeMsg::range_num"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgInternalWithdrawFees") => {
                let value = clmm::msgs::MoonClmmInternalWithdrawFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmInternalWithdrawFeesMsg::query_id"
                );
                assert_eq!(
                    value.nft_index.to_string(),
                    "6",
                    "clmm::msgs::MoonClmmInternalWithdrawFeesMsg::nft_index"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgInternalWithdrawFeesUpdate") => {
                let value = clmm::msgs::MoonClmmInternalWithdrawFeesUpdateMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmInternalWithdrawFeesUpdateMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgInternalWithdrawLiquidity") => {
                let value = clmm::msgs::MoonClmmInternalWithdrawLiquidityMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmInternalWithdrawLiquidityMsg::query_id"
                );
                assert_eq!(
                    value.nft_index.to_string(),
                    "6",
                    "clmm::msgs::MoonClmmInternalWithdrawLiquidityMsg::nft_index"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgInternalWithdrawLiquidityUpdate") => {
                let value = clmm::msgs::MoonClmmInternalWithdrawLiquidityUpdateMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmInternalWithdrawLiquidityUpdateMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgJettonTransfer") => {
                let value = clmm::msgs::MoonClmmJettonTransferMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmJettonTransferMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "clmm::msgs::MoonClmmJettonTransferMsg::amount"
                );
                assert_eq!(
                    value.forward_ton_amount.to_u128().to_string(),
                    "6007",
                    "clmm::msgs::MoonClmmJettonTransferMsg::forward_ton_amount"
                );
                assert!(!value.have_payload);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgJettonTransferWithPayload") => {
                let value = clmm::msgs::MoonClmmJettonTransferWithPayloadMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmJettonTransferWithPayloadMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "clmm::msgs::MoonClmmJettonTransferWithPayloadMsg::amount"
                );
                assert_eq!(
                    value.forward_ton_amount.to_u128().to_string(),
                    "6007",
                    "clmm::msgs::MoonClmmJettonTransferWithPayloadMsg::forward_ton_amount"
                );
                assert!(value.have_payload);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgLockLiquidity") => {
                let value = clmm::msgs::MoonClmmLockLiquidityMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmLockLiquidityMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgProvideLiquidity") => {
                let value = clmm::msgs::MoonClmmProvideLiquidityMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmProvideLiquidityMsg::query_id"
                );
                assert_eq!(
                    value.x_amount.to_u128().to_string(),
                    "3007",
                    "clmm::msgs::MoonClmmProvideLiquidityMsg::x_amount"
                );
                assert_eq!(
                    value.y_amount.to_u128().to_string(),
                    "4007",
                    "clmm::msgs::MoonClmmProvideLiquidityMsg::y_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgProvideWalletAddress") => {
                let value = clmm::msgs::MoonClmmProvideWalletAddressMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmProvideWalletAddressMsg::query_id"
                );
                assert!(!value.include_address);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgSwap") => {
                let value = clmm::msgs::MoonClmmSwapMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "clmm::msgs::MoonClmmSwapMsg::query_id");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgSwapFulfill") => {
                let value = clmm::msgs::MoonClmmSwapFulfillMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "clmm::msgs::MoonClmmSwapFulfillMsg::query_id");
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "clmm::msgs::MoonClmmSwapFulfillMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgTakeWalletAddress") => {
                let value = clmm::msgs::MoonClmmTakeWalletAddressMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmTakeWalletAddressMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgTransferNotification") => {
                let value = clmm::msgs::MoonClmmTransferNotificationMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmTransferNotificationMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "clmm::msgs::MoonClmmTransferNotificationMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgUpdateRangeFees") => {
                let value = clmm::msgs::MoonClmmUpdateRangeFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmUpdateRangeFeesMsg::query_id"
                );
                assert_eq!(
                    value.range_num.to_string(),
                    "-6",
                    "clmm::msgs::MoonClmmUpdateRangeFeesMsg::range_num"
                );
                assert_eq!(value.lp_fee.to_string(), "7", "clmm::msgs::MoonClmmUpdateRangeFeesMsg::lp_fee");
                assert_eq!(
                    value.protocol_fee.to_string(),
                    "8",
                    "clmm::msgs::MoonClmmUpdateRangeFeesMsg::protocol_fee"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgUpdateRangeWallets") => {
                let value = clmm::msgs::MoonClmmUpdateRangeWalletsMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmUpdateRangeWalletsMsg::query_id"
                );
                assert_eq!(
                    value.range_num.to_string(),
                    "-6",
                    "clmm::msgs::MoonClmmUpdateRangeWalletsMsg::range_num"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgUpgradeFactory") => {
                let value = clmm::msgs::MoonClmmUpgradeFactoryMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmUpgradeFactoryMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgUpgradePool") => {
                let value = clmm::msgs::MoonClmmUpgradePoolMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "clmm::msgs::MoonClmmUpgradePoolMsg::query_id");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgUpgradeRange") => {
                let value = clmm::msgs::MoonClmmUpgradeRangeMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "clmm::msgs::MoonClmmUpgradeRangeMsg::query_id");
                assert_eq!(
                    value.range_num.to_string(),
                    "-6",
                    "clmm::msgs::MoonClmmUpgradeRangeMsg::range_num"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgWithdrawFees") => {
                let value = clmm::msgs::MoonClmmWithdrawFeesMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "clmm::msgs::MoonClmmWithdrawFeesMsg::query_id");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgWithdrawLiqudity") => {
                let value = clmm::msgs::MoonClmmWithdrawLiquidityMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmWithdrawLiquidityMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "PayloadCode") => {
                let value = clmm::msgs::MoonClmmPayloadCode::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "PoolInitData") => {
                let value = clmm::pool::storage::MoonClmmPoolInitData::from_cell(&cell)?;
                assert_eq!(
                    value.bin_step.to_string(),
                    "7",
                    "clmm::pool::storage::MoonClmmPoolInitData::bin_step"
                );
                assert_eq!(
                    hex::encode(
                        crate::contracts::address::derive(TonCell::empty().clone(), &value)?
                            .hash
                            .as_slice()
                    ),
                    row["init_hash"].as_str().ok_or_else(|| anyhow::anyhow!("init hash"))?
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "PoolStorage") => {
                let value = clmm::pool::storage::MoonClmmPoolStorage::from_cell(&cell)?;
                assert_eq!(
                    value.bin_step.to_string(),
                    "7",
                    "clmm::pool::storage::MoonClmmPoolStorage::bin_step"
                );
                assert_eq!(
                    value.lp_fee.to_string(),
                    "8",
                    "clmm::pool::storage::MoonClmmPoolStorage::lp_fee"
                );
                assert_eq!(
                    value.protocol_fee.to_string(),
                    "9",
                    "clmm::pool::storage::MoonClmmPoolStorage::protocol_fee"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "RangeInitData") => {
                let value = clmm::range::storage::MoonClmmRangeInitData::from_cell(&cell)?;
                assert_eq!(
                    value.range_num.to_string(),
                    "-5",
                    "clmm::range::storage::MoonClmmRangeInitData::range_num"
                );
                assert_eq!(
                    hex::encode(
                        crate::contracts::address::derive(TonCell::empty().clone(), &value)?
                            .hash
                            .as_slice()
                    ),
                    row["init_hash"].as_str().ok_or_else(|| anyhow::anyhow!("init hash"))?
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "RangeInitPrice") => {
                let value = clmm::range::storage::MoonClmmRangeInitPrice::from_cell(&cell)?;
                assert_eq!(
                    value.init_bin.to_string(),
                    "-4",
                    "clmm::range::storage::MoonClmmRangeInitPrice::init_bin"
                );
                assert_eq!(
                    value.init_sqrt_price.to_string(),
                    "5",
                    "clmm::range::storage::MoonClmmRangeInitPrice::init_sqrt_price"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "RangeState") => {
                let value = clmm::range::storage::MoonClmmRangeState::from_cell(&cell)?;
                assert_eq!(
                    value.current_bin.to_string(),
                    "-4",
                    "clmm::range::storage::MoonClmmRangeState::current_bin"
                );
                assert_eq!(
                    value.current_sqrt_p.to_string(),
                    "5",
                    "clmm::range::storage::MoonClmmRangeState::current_sqrt_p"
                );
                assert_eq!(
                    value.x_protocol_fees.to_u128().to_string(),
                    "3007",
                    "clmm::range::storage::MoonClmmRangeState::x_protocol_fees"
                );
                assert_eq!(
                    value.y_protocol_fees.to_u128().to_string(),
                    "4007",
                    "clmm::range::storage::MoonClmmRangeState::y_protocol_fees"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "RangeStorage") => {
                let value = clmm::range::storage::MoonClmmRangeStorage::from_cell(&cell)?;
                assert_eq!(
                    value.range_num.to_string(),
                    "-5",
                    "clmm::range::storage::MoonClmmRangeStorage::range_num"
                );
                assert_eq!(
                    value.bin_step.to_string(),
                    "8",
                    "clmm::range::storage::MoonClmmRangeStorage::bin_step"
                );
                assert_eq!(
                    value.lp_fee.to_string(),
                    "9",
                    "clmm::range::storage::MoonClmmRangeStorage::lp_fee"
                );
                assert_eq!(
                    value.protocol_fee.to_string(),
                    "10",
                    "clmm::range::storage::MoonClmmRangeStorage::protocol_fee"
                );
                assert_eq!(
                    value.position_index.to_string(),
                    "11",
                    "clmm::range::storage::MoonClmmRangeStorage::position_index"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "RangeUninitMasters") => {
                let value = clmm::range::storage::MoonClmmRangeUninitMasters::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "RangeUninitStorage") => {
                let value = clmm::range::storage::MoonClmmRangeUninitStorage::from_cell(&cell)?;
                assert_eq!(
                    value.range_num.to_string(),
                    "-5",
                    "clmm::range::storage::MoonClmmRangeUninitStorage::range_num"
                );
                assert_eq!(
                    value.bin_step.to_string(),
                    "8",
                    "clmm::range::storage::MoonClmmRangeUninitStorage::bin_step"
                );
                assert_eq!(
                    value.lp_fee.to_string(),
                    "9",
                    "clmm::range::storage::MoonClmmRangeUninitStorage::lp_fee"
                );
                assert_eq!(
                    value.protocol_fee.to_string(),
                    "10",
                    "clmm::range::storage::MoonClmmRangeUninitStorage::protocol_fee"
                );
                assert_eq!(
                    value.nft_index.to_string(),
                    "11",
                    "clmm::range::storage::MoonClmmRangeUninitStorage::nft_index"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "SwapOut") => {
                let value = clmm::msgs::MoonClmmSwapOut::from_cell(&cell)?;
                assert_eq!(
                    value.total_out.to_u128().to_string(),
                    "1007",
                    "clmm::msgs::MoonClmmSwapOut::total_out"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "SwapResultJettonBody") => {
                let value = clmm::msgs::MoonClmmSwapResultJettonBodyMsg::from_cell(&cell)?;
                assert_eq!(value.op.to_string(), "4", "clmm::msgs::MoonClmmSwapResultJettonBodyMsg::op");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "SwapResultTonBody") => {
                let value = clmm::msgs::MoonClmmSwapResultTonBodyMsg::from_cell(&cell)?;
                assert_eq!(value.op.to_string(), "4", "clmm::msgs::MoonClmmSwapResultTonBodyMsg::op");
                assert_eq!(
                    value.query_id.to_string(),
                    "5",
                    "clmm::msgs::MoonClmmSwapResultTonBodyMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "3007",
                    "clmm::msgs::MoonClmmSwapResultTonBodyMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "TonSwapParams") => {
                let value = clmm::msgs::MoonClmmTonSwapParams::from_cell(&cell)?;
                assert_eq!(
                    value.swap_amount.to_u128().to_string(),
                    "1007",
                    "clmm::msgs::MoonClmmTonSwapParams::swap_amount"
                );
                assert_eq!(
                    value.sqrt_price_limit.to_string(),
                    "6",
                    "clmm::msgs::MoonClmmTonSwapParams::sqrt_price_limit"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgLiquidityResponse") => {
                let value = clmm::msgs::MoonClmmLiquidityResponseMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmLiquidityResponseMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgPositionResponse") => {
                let value = clmm::msgs::MoonClmmPositionResponseMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmPositionResponseMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgRangeResponse") => {
                let value = clmm::msgs::MoonClmmRangeResponseMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmRangeResponseMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("clmm", "MsgNativeSurplusResponse") => {
                let value = clmm::msgs::MoonClmmNativeSurplusResponseMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "clmm::msgs::MoonClmmNativeSurplusResponseMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "AskToBurn") => {
                let value = mempad::msgs::MoonMempadAskToBurnMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadAskToBurnMsg::query_id"
                );
                assert_eq!(
                    value.jetton_amount.to_u128().to_string(),
                    "2007",
                    "mempad::msgs::MoonMempadAskToBurnMsg::jetton_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "AskToTransfer") => {
                let value = mempad::msgs::MoonMempadAskToTransferMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadAskToTransferMsg::query_id"
                );
                assert_eq!(
                    value.jetton_amount.to_u128().to_string(),
                    "2007",
                    "mempad::msgs::MoonMempadAskToTransferMsg::jetton_amount"
                );
                assert_eq!(
                    value.forward_ton_amount.to_u128().to_string(),
                    "6007",
                    "mempad::msgs::MoonMempadAskToTransferMsg::forward_ton_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "BurnNotificationForMinter") => {
                let value = mempad::msgs::MoonMempadBurnNotificationForMinterMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadBurnNotificationForMinterMsg::query_id"
                );
                assert_eq!(
                    value.jetton_amount.to_u128().to_string(),
                    "2007",
                    "mempad::msgs::MoonMempadBurnNotificationForMinterMsg::jetton_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "ChangeMinterAdmin") => {
                let value = mempad::msgs::MoonMempadChangeMinterAdminMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadChangeMinterAdminMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "ChangeMinterMetadata") => {
                let value = mempad::msgs::MoonMempadChangeMinterMetadataMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadChangeMinterMetadataMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "ClaimMinterAdmin") => {
                let value = mempad::msgs::MoonMempadClaimMinterAdminMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadClaimMinterAdminMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "CurveInitData") => {
                let value = mempad::curve::storage::MoonMempadCurveInitData::from_cell(&cell)?;
                assert_eq!(
                    value.salt.to_string(),
                    "6",
                    "mempad::curve::storage::MoonMempadCurveInitData::salt"
                );
                assert_eq!(
                    hex::encode(
                        crate::contracts::address::derive(TonCell::empty().clone(), &value)?
                            .hash
                            .as_slice()
                    ),
                    row["init_hash"].as_str().ok_or_else(|| anyhow::anyhow!("init hash"))?
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "CurveStorage") => {
                let value = mempad::curve::storage::MoonMempadCurveStorage::from_cell(&cell)?;
                assert_eq!(
                    value.salt.to_string(),
                    "6",
                    "mempad::curve::storage::MoonMempadCurveStorage::salt"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "DropMinterAdmin") => {
                let value = mempad::msgs::MoonMempadDropMinterAdminMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadDropMinterAdminMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "EconBase") => {
                let value = mempad::factory::storage::MoonMempadEconBase::from_cell(&cell)?;
                assert_eq!(
                    value.tokens_for_sale.to_u128().to_string(),
                    "1007",
                    "mempad::factory::storage::MoonMempadEconBase::tokens_for_sale"
                );
                assert_eq!(
                    value.tokens_for_lp.to_u128().to_string(),
                    "2007",
                    "mempad::factory::storage::MoonMempadEconBase::tokens_for_lp"
                );
                assert_eq!(
                    value.virtual_gram.to_u128().to_string(),
                    "3007",
                    "mempad::factory::storage::MoonMempadEconBase::virtual_gram"
                );
                assert_eq!(
                    value.virtual_tokens.to_u128().to_string(),
                    "4007",
                    "mempad::factory::storage::MoonMempadEconBase::virtual_tokens"
                );
                assert_eq!(
                    value.target_gram.to_u128().to_string(),
                    "5007",
                    "mempad::factory::storage::MoonMempadEconBase::target_gram"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "EconState") => {
                let value = mempad::curve::storage::MoonMempadEconState::from_cell(&cell)?;
                assert_eq!(
                    value.tokens_for_sale.to_u128().to_string(),
                    "1007",
                    "mempad::curve::storage::MoonMempadEconState::tokens_for_sale"
                );
                assert_eq!(
                    value.tokens_for_lp.to_u128().to_string(),
                    "2007",
                    "mempad::curve::storage::MoonMempadEconState::tokens_for_lp"
                );
                assert_eq!(
                    value.virtual_gram.to_u128().to_string(),
                    "3007",
                    "mempad::curve::storage::MoonMempadEconState::virtual_gram"
                );
                assert_eq!(
                    value.virtual_tokens.to_u128().to_string(),
                    "4007",
                    "mempad::curve::storage::MoonMempadEconState::virtual_tokens"
                );
                assert_eq!(
                    value.target_gram.to_u128().to_string(),
                    "5007",
                    "mempad::curve::storage::MoonMempadEconState::target_gram"
                );
                assert_eq!(
                    value.trade_fee_bps.to_string(),
                    "9",
                    "mempad::curve::storage::MoonMempadEconState::trade_fee_bps"
                );
                assert_eq!(
                    value.creator_fee_bps.to_string(),
                    "10",
                    "mempad::curve::storage::MoonMempadEconState::creator_fee_bps"
                );
                assert_eq!(
                    value.pool_fee_bps.to_string(),
                    "11",
                    "mempad::curve::storage::MoonMempadEconState::pool_fee_bps"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "FactoryConfig") => {
                let value = mempad::factory::storage::MoonMempadFactoryConfig::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "FactoryStorage") => {
                let value = mempad::factory::storage::MoonMempadFactoryStorage::from_cell(&cell)?;
                assert_eq!(
                    value.launch_count.to_string(),
                    "5",
                    "mempad::factory::storage::MoonMempadFactoryStorage::launch_count"
                );
                assert_eq!(
                    value.launch_fee.to_u128().to_string(),
                    "3007",
                    "mempad::factory::storage::MoonMempadFactoryStorage::launch_fee"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "GradState") => {
                let value = mempad::curve::storage::MoonMempadGradState::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "InternalTransferStep") => {
                let value = mempad::msgs::MoonMempadInternalTransferStepMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadInternalTransferStepMsg::query_id"
                );
                assert_eq!(
                    value.jetton_amount.to_u128().to_string(),
                    "2007",
                    "mempad::msgs::MoonMempadInternalTransferStepMsg::jetton_amount"
                );
                assert_eq!(
                    value.forward_ton_amount.to_u128().to_string(),
                    "5007",
                    "mempad::msgs::MoonMempadInternalTransferStepMsg::forward_ton_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "JettonDataReply") => {
                let value = mempad::curve::storage::MoonMempadJettonDataReply::from_cell(&cell)?;
                assert_eq!(
                    value.total_supply.to_string(),
                    "1",
                    "mempad::curve::storage::MoonMempadJettonDataReply::total_supply"
                );
                assert!(!value.mintable);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "JettonDepositLiquidityPayload") => {
                let value = mempad::msgs::MoonMempadJettonDepositLiquidityPayloadMsg::from_cell(&cell)?;
                assert_eq!(
                    value.min_lp_out.to_u128().to_string(),
                    "1007",
                    "mempad::msgs::MoonMempadJettonDepositLiquidityPayloadMsg::min_lp_out"
                );
                assert!(!value.locked);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "JettonWalletDataReply") => {
                let value = mempad::wallet::storage::MoonMempadJettonWalletDataReply::from_cell(&cell)?;
                assert_eq!(
                    value.jetton_balance.to_u128().to_string(),
                    "1007",
                    "mempad::wallet::storage::MoonMempadJettonWalletDataReply::jetton_balance"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "LaunchContent") => {
                let value = mempad::msgs::MoonMempadLaunchContent::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "LaunchExtra") => {
                let value = mempad::msgs::MoonMempadLaunchExtra::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "LaunchLinks") => {
                let value = mempad::msgs::MoonMempadLaunchLinks::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "LpSplit") => {
                let value = mempad::msgs::MoonMempadLpSplit::from_cell(&cell)?;
                assert_eq!(value.a_bps.to_string(), "5", "mempad::msgs::MoonMempadLpSplit::a_bps");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MintNewJettons") => {
                let value = mempad::msgs::MoonMempadMintNewJettonsMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadMintNewJettonsMsg::query_id"
                );
                assert_eq!(
                    value.ton_amount.to_u128().to_string(),
                    "3007",
                    "mempad::msgs::MoonMempadMintNewJettonsMsg::ton_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MinterStorage") => {
                let value = mempad::curve::storage::MoonMempadMinterStorage::from_cell(&cell)?;
                assert_eq!(
                    value.total_supply.to_u128().to_string(),
                    "1007",
                    "mempad::curve::storage::MoonMempadMinterStorage::total_supply"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgBuy") => {
                let value = mempad::msgs::MoonMempadBuyMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "mempad::msgs::MoonMempadBuyMsg::query_id");
                assert_eq!(
                    value.min_out.to_u128().to_string(),
                    "2007",
                    "mempad::msgs::MoonMempadBuyMsg::min_out"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgCreateLaunch") => {
                let value = mempad::msgs::MoonMempadCreateLaunchMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadCreateLaunchMsg::query_id"
                );
                assert_eq!(value.salt.to_string(), "5", "mempad::msgs::MoonMempadCreateLaunchMsg::salt");
                assert_eq!(
                    value.fee_bps.to_string(),
                    "6",
                    "mempad::msgs::MoonMempadCreateLaunchMsg::fee_bps"
                );
                assert_eq!(
                    value.initial_buy.to_u128().to_string(),
                    "4007",
                    "mempad::msgs::MoonMempadCreateLaunchMsg::initial_buy"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgCreatePool") => {
                let value = mempad::msgs::MoonMempadCreatePoolMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadCreatePoolMsg::query_id"
                );
                assert_eq!(value.fee_bps.to_string(), "7", "mempad::msgs::MoonMempadCreatePoolMsg::fee_bps");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgCreatorClaimFees") => {
                let value = mempad::msgs::MoonMempadCreatorClaimFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadCreatorClaimFeesMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgDepositLiquidity") => {
                let value = mempad::msgs::MoonMempadDepositLiquidityMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadDepositLiquidityMsg::query_id"
                );
                assert_eq!(
                    value.min_lp_out.to_u128().to_string(),
                    "2007",
                    "mempad::msgs::MoonMempadDepositLiquidityMsg::min_lp_out"
                );
                assert!(value.locked);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgFactoryWithdraw") => {
                let value = mempad::msgs::MoonMempadFactoryWithdrawMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadFactoryWithdrawMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "mempad::msgs::MoonMempadFactoryWithdrawMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgInitLaunch") => {
                let value = mempad::msgs::MoonMempadInitLaunchMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadInitLaunchMsg::query_id"
                );
                assert_eq!(
                    value.initial_buy.to_u128().to_string(),
                    "4007",
                    "mempad::msgs::MoonMempadInitLaunchMsg::initial_buy"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgInternalUpgradeCurve") => {
                let value = mempad::msgs::MoonMempadInternalUpgradeCurveMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadInternalUpgradeCurveMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgProvidePoolAddress") => {
                let value = mempad::msgs::MoonMempadProvidePoolAddressMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadProvidePoolAddressMsg::query_id"
                );
                assert_eq!(
                    value.fee_bps.to_string(),
                    "7",
                    "mempad::msgs::MoonMempadProvidePoolAddressMsg::fee_bps"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgProvidePoolReady") => {
                let value = mempad::msgs::MoonMempadProvidePoolReadyMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadProvidePoolReadyMsg::query_id"
                );
                assert_eq!(
                    value.attempt.to_string(),
                    "5",
                    "mempad::msgs::MoonMempadProvidePoolReadyMsg::attempt"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgTakePoolAddress") => {
                let value = mempad::msgs::MoonMempadTakePoolAddressMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadTakePoolAddressMsg::query_id"
                );
                assert_eq!(
                    value.fee_bps.to_string(),
                    "8",
                    "mempad::msgs::MoonMempadTakePoolAddressMsg::fee_bps"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgTakePoolReady") => {
                let value = mempad::msgs::MoonMempadTakePoolReadyMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadTakePoolReadyMsg::query_id"
                );
                assert_eq!(
                    value.attempt.to_string(),
                    "5",
                    "mempad::msgs::MoonMempadTakePoolReadyMsg::attempt"
                );
                assert!(value.is_ready);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgTokenSalePayout") => {
                let value = mempad::msgs::MoonMempadTokenSalePayoutMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadTokenSalePayoutMsg::query_id"
                );
                assert_eq!(
                    value.gram_out.to_u128().to_string(),
                    "2007",
                    "mempad::msgs::MoonMempadTokenSalePayoutMsg::gram_out"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgUpgradeCurve") => {
                let value = mempad::msgs::MoonMempadUpgradeCurveMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadUpgradeCurveMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgUpgradeFactory") => {
                let value = mempad::msgs::MoonMempadUpgradeFactoryMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadUpgradeFactoryMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "OnchainMetadataReply") => {
                let value = mempad::curve::storage::MoonMempadOnchainMetadataReply::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "Params") => {
                let value = mempad::curve::storage::MoonMempadParams::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "RequestWalletAddress") => {
                let value = mempad::msgs::MoonMempadRequestWalletAddressMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadRequestWalletAddressMsg::query_id"
                );
                assert!(value.include_owner_address);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "ResponseWalletAddress") => {
                let value = mempad::msgs::MoonMempadResponseWalletAddressMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadResponseWalletAddressMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "ReturnExcessesBack") => {
                let value = mempad::msgs::MoonMempadReturnExcessesBackMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadReturnExcessesBackMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "TokenSalePayload") => {
                let value = mempad::msgs::MoonMempadTokenSalePayload::from_cell(&cell)?;
                assert_eq!(value.op.to_string(), "4", "mempad::msgs::MoonMempadTokenSalePayload::op");
                assert_eq!(
                    value.min_out.to_u128().to_string(),
                    "2007",
                    "mempad::msgs::MoonMempadTokenSalePayload::min_out"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "TopUpTons") => {
                let value = mempad::msgs::MoonMempadTopUpTonsMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "TradingState") => {
                let value = mempad::curve::storage::MoonMempadTradingState::from_cell(&cell)?;
                assert_eq!(
                    value.gram_reserve.to_u128().to_string(),
                    "1007",
                    "mempad::curve::storage::MoonMempadTradingState::gram_reserve"
                );
                assert_eq!(
                    value.supply.to_u128().to_string(),
                    "2007",
                    "mempad::curve::storage::MoonMempadTradingState::supply"
                );
                assert_eq!(
                    value.creator_fees.to_u128().to_string(),
                    "3007",
                    "mempad::curve::storage::MoonMempadTradingState::creator_fees"
                );
                assert_eq!(
                    value.platform_fees.to_u128().to_string(),
                    "4007",
                    "mempad::curve::storage::MoonMempadTradingState::platform_fees"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "TransferNotificationForRecipient") => {
                let value = mempad::msgs::MoonMempadTransferNotificationForRecipientMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadTransferNotificationForRecipientMsg::query_id"
                );
                assert_eq!(
                    value.jetton_amount.to_u128().to_string(),
                    "2007",
                    "mempad::msgs::MoonMempadTransferNotificationForRecipientMsg::jetton_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "UpgradeMinterCode") => {
                let value = mempad::msgs::MoonMempadUpgradeMinterCodeMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadUpgradeMinterCodeMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "WalletStorage") => {
                let value = mempad::wallet::storage::MoonMempadWalletStorage::from_cell(&cell)?;
                assert_eq!(
                    value.jetton_balance.to_u128().to_string(),
                    "1007",
                    "mempad::wallet::storage::MoonMempadWalletStorage::jetton_balance"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgFactoryPayoutResponse") => {
                let value = mempad::msgs::MoonMempadFactoryPayoutResponseMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadFactoryPayoutResponseMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgCurveResponse") => {
                let value = mempad::msgs::MoonMempadCurveResponseMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadCurveResponseMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgPoolDeploymentResponse") => {
                let value = mempad::msgs::MoonMempadPoolDeploymentResponseMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadPoolDeploymentResponseMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("mempad", "MsgCurvePayoutResponse") => {
                let value = mempad::msgs::MoonMempadCurvePayoutResponseMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "mempad::msgs::MoonMempadCurvePayoutResponseMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("booster", "MoonBoosterBoostData") => {
                let value = booster::storage::MoonBoosterBoostData::from_cell(&cell)?;
                assert_eq!(
                    value.amount_per_round.to_u128().to_string(),
                    "1007",
                    "booster::storage::MoonBoosterBoostData::amount_per_round"
                );
                assert_eq!(
                    value.start_time.to_string(),
                    "5",
                    "booster::storage::MoonBoosterBoostData::start_time"
                );
                assert_eq!(
                    value.current_round.to_string(),
                    "6",
                    "booster::storage::MoonBoosterBoostData::current_round"
                );
                assert_eq!(
                    value.round_interval.to_string(),
                    "7",
                    "booster::storage::MoonBoosterBoostData::round_interval"
                );
                assert_eq!(
                    value.round_count.to_string(),
                    "8",
                    "booster::storage::MoonBoosterBoostData::round_count"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("booster", "MoonBoosterDeploymentStorage") => {
                let value = booster::storage::MoonBoosterDeploymentStorage::from_cell(&cell)?;
                assert_eq!(
                    value.status.to_string(),
                    "4",
                    "booster::storage::MoonBoosterDeploymentStorage::status"
                );
                assert_eq!(
                    value.booster_id.to_string(),
                    "5",
                    "booster::storage::MoonBoosterDeploymentStorage::booster_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("booster", "MoonBoosterStorage") => {
                let value = booster::storage::MoonBoosterStorage::from_cell(&cell)?;
                assert_eq!(value.status.to_string(), "4", "booster::storage::MoonBoosterStorage::status");
                assert_eq!(
                    value.booster_id.to_string(),
                    "5",
                    "booster::storage::MoonBoosterStorage::booster_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("booster", "MoonBoosterFundingTail") => {
                let value = booster::msgs::MoonBoosterFundingTail::from_cell(&cell)?;
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "booster::msgs::MoonBoosterFundingTail::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("booster", "MoonBoosterConfigure") => {
                let value = booster::msgs::MoonBoosterConfigureMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "booster::msgs::MoonBoosterConfigureMsg::query_id"
                );
                assert_eq!(value.mode.to_string(), "1", "booster::msgs::MoonBoosterConfigureMsg::mode");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("booster", "MoonBoosterJettonTransferNotification") => {
                let value = booster::msgs::MoonBoosterJettonTransferNotificationMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "booster::msgs::MoonBoosterJettonTransferNotificationMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "booster::msgs::MoonBoosterJettonTransferNotificationMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("booster", "MoonBoosterCloseExpired") => {
                let value = booster::msgs::MoonBoosterCloseExpiredMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "booster::msgs::MoonBoosterCloseExpiredMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("booster", "MoonBoosterFundingRequest") => {
                let value = booster::msgs::MoonBoosterFundingRequestMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "booster::msgs::MoonBoosterFundingRequestMsg::query_id"
                );
                assert_eq!(
                    value.booster_id.to_string(),
                    "5",
                    "booster::msgs::MoonBoosterFundingRequestMsg::booster_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "6007",
                    "booster::msgs::MoonBoosterFundingRequestMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("booster", "MoonBoosterJettonTransfer") => {
                let value = booster::msgs::MoonBoosterJettonTransferMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "booster::msgs::MoonBoosterJettonTransferMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "booster::msgs::MoonBoosterJettonTransferMsg::amount"
                );
                assert_eq!(
                    value.forward_ton_amount.to_u128().to_string(),
                    "6007",
                    "booster::msgs::MoonBoosterJettonTransferMsg::forward_ton_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("booster", "MoonBoosterPoolBoost") => {
                let value = booster::msgs::MoonBoosterPoolBoostMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "booster::msgs::MoonBoosterPoolBoostMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("booster", "MoonBoosterJettonBoostPayload") => {
                let value = booster::msgs::MoonBoosterJettonBoostPayloadMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("booster", "MoonBoosterExcesses") => {
                let value = booster::msgs::MoonBoosterExcessesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "booster::msgs::MoonBoosterExcessesMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeStorage") => {
                let value = catalog_range::storage::MoonCatalogClmmRangeStorage::from_cell(&cell)?;
                assert_eq!(
                    value.range_number.to_string(),
                    "-5",
                    "catalog_range::storage::MoonClmmRangeStorage::range_number"
                );
                assert_eq!(
                    value.bin_step.to_string(),
                    "8",
                    "catalog_range::storage::MoonClmmRangeStorage::bin_step"
                );
                assert_eq!(
                    value.lp_fee.to_string(),
                    "9",
                    "catalog_range::storage::MoonClmmRangeStorage::lp_fee"
                );
                assert_eq!(
                    value.protocol_fee.to_string(),
                    "10",
                    "catalog_range::storage::MoonClmmRangeStorage::protocol_fee"
                );
                assert_eq!(
                    value.next_position_index.to_string(),
                    "11",
                    "catalog_range::storage::MoonClmmRangeStorage::next_position_index"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeState") => {
                let value = catalog_range::storage::MoonCatalogClmmRangeState::from_cell(&cell)?;
                assert_eq!(
                    value.current_bin.to_string(),
                    "-4",
                    "catalog_range::storage::MoonClmmRangeState::current_bin"
                );
                assert_eq!(
                    value.current_sqrt_price_x128.to_string(),
                    "5",
                    "catalog_range::storage::MoonClmmRangeState::current_sqrt_price_x128"
                );
                assert_eq!(
                    value.x_protocol_fees.to_u128().to_string(),
                    "3007",
                    "catalog_range::storage::MoonClmmRangeState::x_protocol_fees"
                );
                assert_eq!(
                    value.y_protocol_fees.to_u128().to_string(),
                    "4007",
                    "catalog_range::storage::MoonClmmRangeState::y_protocol_fees"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeCallback") => {
                let value = catalog_range::storage::MoonCatalogClmmRangeCallback::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeSwapPayload") => {
                let value = catalog_range::storage::MoonCatalogClmmRangeSwapPayload::from_cell(&cell)?;
                assert_eq!(
                    value.swap_amount.to_u128().to_string(),
                    "1007",
                    "catalog_range::storage::MoonClmmRangeSwapPayload::swap_amount"
                );
                assert_eq!(
                    value.sqrt_price_limit_x128.to_string(),
                    "6",
                    "catalog_range::storage::MoonClmmRangeSwapPayload::sqrt_price_limit_x128"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeSwap") => {
                let value = catalog_range::msgs::MoonCatalogClmmRangeSwapMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "catalog_range::msgs::MoonClmmRangeSwapMsg::query_id"
                );
                assert_eq!(
                    value.swap_amount.to_u128().to_string(),
                    "2007",
                    "catalog_range::msgs::MoonClmmRangeSwapMsg::swap_amount"
                );
                assert_eq!(
                    value.sqrt_price_limit_x128.to_string(),
                    "7",
                    "catalog_range::msgs::MoonClmmRangeSwapMsg::sqrt_price_limit_x128"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeWrappedSwap") => {
                let value = catalog_range::msgs::MoonCatalogClmmRangeWrappedSwapMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "catalog_range::msgs::MoonClmmRangeWrappedSwapMsg::query_id"
                );
                assert_eq!(
                    value.swap_amount.to_u128().to_string(),
                    "2007",
                    "catalog_range::msgs::MoonClmmRangeWrappedSwapMsg::swap_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeDepositBin") => {
                let value = catalog_range::storage::MoonCatalogClmmRangeDepositBin::from_cell(&cell)?;
                assert_eq!(
                    value.x_amount.to_u128().to_string(),
                    "1007",
                    "catalog_range::storage::MoonClmmRangeDepositBin::x_amount"
                );
                assert_eq!(
                    value.y_amount.to_u128().to_string(),
                    "2007",
                    "catalog_range::storage::MoonClmmRangeDepositBin::y_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeDepositLiquidity") => {
                let value = catalog_range::msgs::MoonCatalogClmmRangeDepositLiquidityMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "catalog_range::msgs::MoonClmmRangeDepositLiquidityMsg::query_id"
                );
                assert_eq!(
                    value.deposit_amount.to_u128().to_string(),
                    "2007",
                    "catalog_range::msgs::MoonClmmRangeDepositLiquidityMsg::deposit_amount"
                );
                assert_eq!(
                    value.is_single.to_string(),
                    "0",
                    "catalog_range::msgs::MoonClmmRangeDepositLiquidityMsg::is_single"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeJettonTransferNotification") => {
                let value = catalog_range::msgs::MoonCatalogClmmRangeJettonTransferNotificationMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "catalog_range::msgs::MoonClmmRangeJettonTransferNotificationMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "catalog_range::msgs::MoonClmmRangeJettonTransferNotificationMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeSwapForwardPayload") => {
                let value = catalog_range::msgs::MoonCatalogClmmRangeSwapForwardPayloadMsg::from_cell(&cell)?;
                assert_eq!(
                    value.sqrt_price_limit_x128.to_string(),
                    "5",
                    "catalog_range::msgs::MoonClmmRangeSwapForwardPayloadMsg::sqrt_price_limit_x128"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeDepositLiquidityForwardPayload") => {
                let value =
                    catalog_range::msgs::MoonCatalogClmmRangeDepositLiquidityForwardPayloadMsg::from_cell(&cell)?;
                assert_eq!(
                    value.is_single.to_string(),
                    "0",
                    "catalog_range::msgs::MoonClmmRangeDepositLiquidityForwardPayloadMsg::is_single"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeNativeSwapResult") => {
                let value = catalog_range::msgs::MoonCatalogClmmRangeNativeSwapResultMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "catalog_range::msgs::MoonClmmRangeNativeSwapResultMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "catalog_range::msgs::MoonClmmRangeNativeSwapResultMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeJettonSwapResultPayload") => {
                let value = catalog_range::msgs::MoonCatalogClmmRangeJettonSwapResultPayloadMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeNativeSwapRefund") => {
                let value = catalog_range::msgs::MoonCatalogClmmRangeNativeSwapRefundMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "catalog_range::msgs::MoonClmmRangeNativeSwapRefundMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "catalog_range::msgs::MoonClmmRangeNativeSwapRefundMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("catalog_range", "MoonClmmRangeJettonTransfer") => {
                let value = catalog_range::msgs::MoonCatalogClmmRangeJettonTransferMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "catalog_range::msgs::MoonClmmRangeJettonTransferMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "catalog_range::msgs::MoonClmmRangeJettonTransferMsg::amount"
                );
                assert_eq!(
                    value.forward_ton_amount.to_u128().to_string(),
                    "6007",
                    "catalog_range::msgs::MoonClmmRangeJettonTransferMsg::forward_ton_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryStorage") => {
                let value = order_factory::storage::MoonOrderFactoryStorage::from_cell(&cell)?;
                assert_eq!(
                    value.last_order_id.to_string(),
                    "6",
                    "order_factory::storage::MoonOrderFactoryStorage::last_order_id"
                );
                assert_eq!(
                    value.order_fee_bps.to_string(),
                    "7",
                    "order_factory::storage::MoonOrderFactoryStorage::order_fee_bps"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryOrderParams") => {
                let value = order_factory::storage::MoonOrderFactoryOrderParams::from_cell(&cell)?;
                assert_eq!(
                    value.rate.to_string(),
                    "4",
                    "order_factory::storage::MoonOrderFactoryOrderParams::rate"
                );
                assert_eq!(
                    value.lock_mode.to_string(),
                    "1",
                    "order_factory::storage::MoonOrderFactoryOrderParams::lock_mode"
                );
                assert_eq!(
                    value.vesting_duration.to_string(),
                    "6",
                    "order_factory::storage::MoonOrderFactoryOrderParams::vesting_duration"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryAssets") => {
                let value = order_factory::storage::MoonOrderFactoryAssets::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryWallets") => {
                let value = order_factory::storage::MoonOrderFactoryWallets::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryFunding") => {
                let value = order_factory::storage::MoonOrderFactoryFunding::from_cell(&cell)?;
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "order_factory::storage::MoonOrderFactoryFunding::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryUpgradeStorage") => {
                let value = order_factory::storage::MoonOrderFactoryUpgradeStorage::from_cell(&cell)?;
                assert_eq!(
                    value.order_fee_bps.to_string(),
                    "6",
                    "order_factory::storage::MoonOrderFactoryUpgradeStorage::order_fee_bps"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryFeeNotification") => {
                let value = order_factory::msgs::MoonOrderFactoryFeeNotificationMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_factory::msgs::MoonOrderFactoryFeeNotificationMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryDeployPipeRequest") => {
                let value = order_factory::msgs::MoonOrderFactoryDeployPipeRequestMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_factory::msgs::MoonOrderFactoryDeployPipeRequestMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryRegisterOrder") => {
                let value = order_factory::msgs::MoonOrderFactoryRegisterOrderMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_factory::msgs::MoonOrderFactoryRegisterOrderMsg::query_id"
                );
                assert_eq!(
                    value.order_id.to_string(),
                    "6",
                    "order_factory::msgs::MoonOrderFactoryRegisterOrderMsg::order_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "6007",
                    "order_factory::msgs::MoonOrderFactoryRegisterOrderMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryForwardRequest") => {
                let value = order_factory::msgs::MoonOrderFactoryForwardRequestMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_factory::msgs::MoonOrderFactoryForwardRequestMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryUpgrade") => {
                let value = order_factory::msgs::MoonOrderFactoryUpgradeMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_factory::msgs::MoonOrderFactoryUpgradeMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryCreateOrder") => {
                let value = order_factory::msgs::MoonOrderFactoryCreateOrderMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_factory::msgs::MoonOrderFactoryCreateOrderMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryJettonCreatePayload") => {
                let value = order_factory::msgs::MoonOrderFactoryJettonCreatePayloadMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryJettonTransferNotification") => {
                let value = order_factory::msgs::MoonOrderFactoryJettonTransferNotificationMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_factory::msgs::MoonOrderFactoryJettonTransferNotificationMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "order_factory::msgs::MoonOrderFactoryJettonTransferNotificationMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryDeployPipe") => {
                let value = order_factory::msgs::MoonOrderFactoryDeployPipeMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_factory::msgs::MoonOrderFactoryDeployPipeMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryForwardedMessage") => {
                let value = order_factory::msgs::MoonOrderFactoryForwardedMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_factory::msgs::MoonOrderFactoryForwardedMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryDeployOrder") => {
                let value = order_factory::msgs::MoonOrderFactoryDeployOrderMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_factory::msgs::MoonOrderFactoryDeployOrderMsg::query_id"
                );
                assert_eq!(
                    value.order_fee_bps.to_string(),
                    "5",
                    "order_factory::msgs::MoonOrderFactoryDeployOrderMsg::order_fee_bps"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryJettonTransfer") => {
                let value = order_factory::msgs::MoonOrderFactoryJettonTransferMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_factory::msgs::MoonOrderFactoryJettonTransferMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "order_factory::msgs::MoonOrderFactoryJettonTransferMsg::amount"
                );
                assert_eq!(
                    value.forward_ton_amount.to_u128().to_string(),
                    "6007",
                    "order_factory::msgs::MoonOrderFactoryJettonTransferMsg::forward_ton_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_factory", "MoonOrderFactoryExcess") => {
                let value = order_factory::msgs::MoonOrderFactoryExcessMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_factory::msgs::MoonOrderFactoryExcessMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewDeploymentStorage") => {
                let value = order_new::storage::MoonOrderNewDeploymentStorage::from_cell(&cell)?;
                assert_eq!(
                    value.order_id.to_string(),
                    "6",
                    "order_new::storage::MoonOrderNewDeploymentStorage::order_id"
                );
                assert_eq!(
                    value.status.to_string(),
                    "7",
                    "order_new::storage::MoonOrderNewDeploymentStorage::status"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewStorage") => {
                let value = order_new::storage::MoonOrderNewStorage::from_cell(&cell)?;
                assert_eq!(
                    value.order_id.to_string(),
                    "6",
                    "order_new::storage::MoonOrderNewStorage::order_id"
                );
                assert_eq!(value.status.to_string(), "0", "order_new::storage::MoonOrderNewStorage::status");
                assert_eq!(
                    value.remaining_amount.to_u128().to_string(),
                    "0",
                    "order_new::storage::MoonOrderNewStorage::remaining_amount"
                );
                assert_eq!(
                    value.order_fee_bps.to_string(),
                    "9",
                    "order_new::storage::MoonOrderNewStorage::order_fee_bps"
                );
                assert_eq!(
                    value.accumulated_fee.to_u128().to_string(),
                    "7007",
                    "order_new::storage::MoonOrderNewStorage::accumulated_fee"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewOrderParams") => {
                let value = order_new::storage::MoonOrderNewOrderParams::from_cell(&cell)?;
                assert_eq!(value.rate.to_string(), "4", "order_new::storage::MoonOrderNewOrderParams::rate");
                assert_eq!(
                    value.lock_mode.to_string(),
                    "1",
                    "order_new::storage::MoonOrderNewOrderParams::lock_mode"
                );
                assert_eq!(
                    value.vesting_duration.to_string(),
                    "6",
                    "order_new::storage::MoonOrderNewOrderParams::vesting_duration"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewVestingState") => {
                let value = order_new::storage::MoonOrderNewVestingState::from_cell(&cell)?;
                assert_eq!(
                    value.start_time.to_string(),
                    "4",
                    "order_new::storage::MoonOrderNewVestingState::start_time"
                );
                assert_eq!(
                    value.duration.to_string(),
                    "5",
                    "order_new::storage::MoonOrderNewVestingState::duration"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewAssets") => {
                let value = order_new::storage::MoonOrderNewAssets::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewWallets") => {
                let value = order_new::storage::MoonOrderNewWallets::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewFunding") => {
                let value = order_new::storage::MoonOrderNewFunding::from_cell(&cell)?;
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "order_new::storage::MoonOrderNewFunding::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewCancel") => {
                let value = order_new::msgs::MoonOrderNewCancelMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewCancelMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewFill") => {
                let value = order_new::msgs::MoonOrderNewFillMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewFillMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewJettonFillPayload") => {
                let value = order_new::msgs::MoonOrderNewJettonFillPayloadMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewJettonTransferNotification") => {
                let value = order_new::msgs::MoonOrderNewJettonTransferNotificationMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewJettonTransferNotificationMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "order_new::msgs::MoonOrderNewJettonTransferNotificationMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewPipeWalletResolved") => {
                let value = order_new::msgs::MoonOrderNewPipeWalletResolvedMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewPipeWalletResolvedMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewDeploy") => {
                let value = order_new::msgs::MoonOrderNewDeployMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewDeployMsg::query_id"
                );
                assert_eq!(
                    value.order_fee_bps.to_string(),
                    "5",
                    "order_new::msgs::MoonOrderNewDeployMsg::order_fee_bps"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewFeeNotification") => {
                let value = order_new::msgs::MoonOrderNewFeeNotificationMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewFeeNotificationMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewFactoryRegistration") => {
                let value = order_new::msgs::MoonOrderNewFactoryRegistrationMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewFactoryRegistrationMsg::query_id"
                );
                assert_eq!(
                    value.order_id.to_string(),
                    "6",
                    "order_new::msgs::MoonOrderNewFactoryRegistrationMsg::order_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "6007",
                    "order_new::msgs::MoonOrderNewFactoryRegistrationMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewPipeWalletRequest") => {
                let value = order_new::msgs::MoonOrderNewPipeWalletRequestMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewPipeWalletRequestMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewResponse510") => {
                let value = order_new::msgs::MoonOrderNewResponse510Msg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewResponse510Msg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewResponse702") => {
                let value = order_new::msgs::MoonOrderNewResponse702Msg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewResponse702Msg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewCancelled") => {
                let value = order_new::msgs::MoonOrderNewCancelledMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewCancelledMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewPayout") => {
                let value = order_new::msgs::MoonOrderNewPayoutMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewPayoutMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewVestingPayout") => {
                let value = order_new::msgs::MoonOrderNewVestingPayoutMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewVestingPayoutMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewJettonTransfer") => {
                let value = order_new::msgs::MoonOrderNewJettonTransferMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewJettonTransferMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "order_new::msgs::MoonOrderNewJettonTransferMsg::amount"
                );
                assert_eq!(
                    value.forward_ton_amount.to_u128().to_string(),
                    "6007",
                    "order_new::msgs::MoonOrderNewJettonTransferMsg::forward_ton_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewExcess") => {
                let value = order_new::msgs::MoonOrderNewExcessMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_new::msgs::MoonOrderNewExcessMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_new", "MoonOrderNewVestingDataReply") => {
                let value = order_new::storage::MoonOrderNewVestingDataReply::from_cell(&cell)?;
                assert_eq!(
                    value.vesting_start_time.to_string(),
                    "4",
                    "order_new::storage::MoonOrderNewVestingDataReply::vesting_start_time"
                );
                assert_eq!(
                    value.vesting_duration.to_string(),
                    "5",
                    "order_new::storage::MoonOrderNewVestingDataReply::vesting_duration"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldDeploymentStorage") => {
                let value = order_old::storage::MoonOrderOldDeploymentStorage::from_cell(&cell)?;
                assert_eq!(
                    value.order_id.to_string(),
                    "6",
                    "order_old::storage::MoonOrderOldDeploymentStorage::order_id"
                );
                assert_eq!(
                    value.status.to_string(),
                    "7",
                    "order_old::storage::MoonOrderOldDeploymentStorage::status"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldStorage") => {
                let value = order_old::storage::MoonOrderOldStorage::from_cell(&cell)?;
                assert_eq!(
                    value.order_id.to_string(),
                    "6",
                    "order_old::storage::MoonOrderOldStorage::order_id"
                );
                assert_eq!(value.status.to_string(), "1", "order_old::storage::MoonOrderOldStorage::status");
                assert_eq!(
                    value.remaining_amount.to_u128().to_string(),
                    "5007",
                    "order_old::storage::MoonOrderOldStorage::remaining_amount"
                );
                assert_eq!(
                    value.order_fee_bps.to_string(),
                    "9",
                    "order_old::storage::MoonOrderOldStorage::order_fee_bps"
                );
                assert_eq!(
                    value.accumulated_fee.to_u128().to_string(),
                    "7007",
                    "order_old::storage::MoonOrderOldStorage::accumulated_fee"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldOrderParams") => {
                let value = order_old::storage::MoonOrderOldOrderParams::from_cell(&cell)?;
                assert_eq!(value.rate.to_string(), "4", "order_old::storage::MoonOrderOldOrderParams::rate");
                assert_eq!(
                    value.lock_mode.to_string(),
                    "1",
                    "order_old::storage::MoonOrderOldOrderParams::lock_mode"
                );
                assert_eq!(
                    value.vesting_duration.to_string(),
                    "6",
                    "order_old::storage::MoonOrderOldOrderParams::vesting_duration"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldVestingState") => {
                let value = order_old::storage::MoonOrderOldVestingState::from_cell(&cell)?;
                assert_eq!(
                    value.start_time.to_string(),
                    "4",
                    "order_old::storage::MoonOrderOldVestingState::start_time"
                );
                assert_eq!(
                    value.duration.to_string(),
                    "5",
                    "order_old::storage::MoonOrderOldVestingState::duration"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldAssets") => {
                let value = order_old::storage::MoonOrderOldAssets::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldWallets") => {
                let value = order_old::storage::MoonOrderOldWallets::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldFunding") => {
                let value = order_old::storage::MoonOrderOldFunding::from_cell(&cell)?;
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "order_old::storage::MoonOrderOldFunding::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldCancel") => {
                let value = order_old::msgs::MoonOrderOldCancelMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldCancelMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldFill") => {
                let value = order_old::msgs::MoonOrderOldFillMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldFillMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldJettonFillPayload") => {
                let value = order_old::msgs::MoonOrderOldJettonFillPayloadMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldJettonTransferNotification") => {
                let value = order_old::msgs::MoonOrderOldJettonTransferNotificationMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldJettonTransferNotificationMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "order_old::msgs::MoonOrderOldJettonTransferNotificationMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldPipeWalletResolved") => {
                let value = order_old::msgs::MoonOrderOldPipeWalletResolvedMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldPipeWalletResolvedMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldDeploy") => {
                let value = order_old::msgs::MoonOrderOldDeployMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldDeployMsg::query_id"
                );
                assert_eq!(
                    value.order_fee_bps.to_string(),
                    "5",
                    "order_old::msgs::MoonOrderOldDeployMsg::order_fee_bps"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldFeeNotification") => {
                let value = order_old::msgs::MoonOrderOldFeeNotificationMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldFeeNotificationMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldFactoryRegistration") => {
                let value = order_old::msgs::MoonOrderOldFactoryRegistrationMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldFactoryRegistrationMsg::query_id"
                );
                assert_eq!(
                    value.order_id.to_string(),
                    "6",
                    "order_old::msgs::MoonOrderOldFactoryRegistrationMsg::order_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "6007",
                    "order_old::msgs::MoonOrderOldFactoryRegistrationMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldPipeWalletRequest") => {
                let value = order_old::msgs::MoonOrderOldPipeWalletRequestMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldPipeWalletRequestMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldResponse510") => {
                let value = order_old::msgs::MoonOrderOldResponse510Msg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldResponse510Msg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldResponse702") => {
                let value = order_old::msgs::MoonOrderOldResponse702Msg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldResponse702Msg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldCancelled") => {
                let value = order_old::msgs::MoonOrderOldCancelledMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldCancelledMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldPayout") => {
                let value = order_old::msgs::MoonOrderOldPayoutMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldPayoutMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldVestingPayout") => {
                let value = order_old::msgs::MoonOrderOldVestingPayoutMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldVestingPayoutMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldJettonTransfer") => {
                let value = order_old::msgs::MoonOrderOldJettonTransferMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldJettonTransferMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "order_old::msgs::MoonOrderOldJettonTransferMsg::amount"
                );
                assert_eq!(
                    value.forward_ton_amount.to_u128().to_string(),
                    "6007",
                    "order_old::msgs::MoonOrderOldJettonTransferMsg::forward_ton_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldExcess") => {
                let value = order_old::msgs::MoonOrderOldExcessMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "order_old::msgs::MoonOrderOldExcessMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("order_old", "MoonOrderOldVestingDataReply") => {
                let value = order_old::storage::MoonOrderOldVestingDataReply::from_cell(&cell)?;
                assert_eq!(
                    value.vesting_start_time.to_string(),
                    "4",
                    "order_old::storage::MoonOrderOldVestingDataReply::vesting_start_time"
                );
                assert_eq!(
                    value.vesting_duration.to_string(),
                    "5",
                    "order_old::storage::MoonOrderOldVestingDataReply::vesting_duration"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolDeploymentStorage") => {
                let value = legacy_pool::storage::MoonLegacyPoolDeploymentStorage::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolStorage") => {
                let value = legacy_pool::storage::MoonLegacyPoolStorage::from_cell(&cell)?;
                assert_eq!(
                    value.swap_fee_bps.to_string(),
                    "7",
                    "legacy_pool::storage::MoonPoolStorage::swap_fee_bps"
                );
                assert_eq!(
                    value.protocol_fee_bps.to_string(),
                    "8",
                    "legacy_pool::storage::MoonPoolStorage::protocol_fee_bps"
                );
                assert_eq!(
                    value.referral_fee_bps.to_string(),
                    "9",
                    "legacy_pool::storage::MoonPoolStorage::referral_fee_bps"
                );
                assert_eq!(
                    value.lp_total_supply.to_u128().to_string(),
                    "7007",
                    "legacy_pool::storage::MoonPoolStorage::lp_total_supply"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolBalances") => {
                let value = legacy_pool::storage::MoonLegacyPoolBalances::from_cell(&cell)?;
                assert_eq!(
                    value.asset1_reserve.to_u128().to_string(),
                    "1007",
                    "legacy_pool::storage::MoonPoolBalances::asset1_reserve"
                );
                assert_eq!(
                    value.asset2_reserve.to_u128().to_string(),
                    "2007",
                    "legacy_pool::storage::MoonPoolBalances::asset2_reserve"
                );
                assert_eq!(
                    value.asset1_protocol_fees.to_u128().to_string(),
                    "3007",
                    "legacy_pool::storage::MoonPoolBalances::asset1_protocol_fees"
                );
                assert_eq!(
                    value.asset2_protocol_fees.to_u128().to_string(),
                    "4007",
                    "legacy_pool::storage::MoonPoolBalances::asset2_protocol_fees"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolAssetsAndContent") => {
                let value = legacy_pool::storage::MoonLegacyPoolAssetsAndContent::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolAssetAmount") => {
                let value = legacy_pool::storage::MoonLegacyPoolAssetAmount::from_cell(&cell)?;
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "legacy_pool::storage::MoonPoolAssetAmount::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolNextPayload") => {
                let value = legacy_pool::storage::MoonLegacyPoolNextPayload::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolSwapParams") => {
                let value = legacy_pool::storage::MoonLegacyPoolSwapParams::from_cell(&cell)?;
                assert_eq!(
                    value.min_out.to_u128().to_string(),
                    "1007",
                    "legacy_pool::storage::MoonPoolSwapParams::min_out"
                );
                assert_eq!(
                    value.deadline.to_string(),
                    "5",
                    "legacy_pool::storage::MoonPoolSwapParams::deadline"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolJettonForwardPayloadInline") => {
                let value = legacy_pool::msgs::MoonLegacyPoolJettonForwardPayloadInlineMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolJettonForwardPayloadInRef") => {
                let value = legacy_pool::msgs::MoonLegacyPoolJettonForwardPayloadInRefMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolInitialize") => {
                let value = legacy_pool::msgs::MoonLegacyPoolInitializeMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolInitializeMsg::query_id"
                );
                assert_eq!(
                    value.swap_fee_bps.to_string(),
                    "7",
                    "legacy_pool::msgs::MoonPoolInitializeMsg::swap_fee_bps"
                );
                assert_eq!(
                    value.protocol_fee_bps.to_string(),
                    "8",
                    "legacy_pool::msgs::MoonPoolInitializeMsg::protocol_fee_bps"
                );
                assert_eq!(
                    value.referral_fee_bps.to_string(),
                    "9",
                    "legacy_pool::msgs::MoonPoolInitializeMsg::referral_fee_bps"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolSwap") => {
                let value = legacy_pool::msgs::MoonLegacyPoolSwapMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "legacy_pool::msgs::MoonPoolSwapMsg::query_id");
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "legacy_pool::msgs::MoonPoolSwapMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolSwapForwardPayload") => {
                let value = legacy_pool::msgs::MoonLegacyPoolSwapForwardPayloadMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolDepositLiquidity") => {
                let value = legacy_pool::msgs::MoonLegacyPoolDepositLiquidityMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolDepositLiquidityMsg::query_id"
                );
                assert_eq!(
                    value.min_lp_out.to_u128().to_string(),
                    "2007",
                    "legacy_pool::msgs::MoonPoolDepositLiquidityMsg::min_lp_out"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolDepositLiquidityForwardPayload") => {
                let value = legacy_pool::msgs::MoonLegacyPoolDepositLiquidityForwardPayloadMsg::from_cell(&cell)?;
                assert_eq!(
                    value.min_lp_out.to_u128().to_string(),
                    "1007",
                    "legacy_pool::msgs::MoonPoolDepositLiquidityForwardPayloadMsg::min_lp_out"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolBoost") => {
                let value = legacy_pool::msgs::MoonLegacyPoolBoostMsg::from_cell(&cell)?;
                assert_eq!(value.query_id.to_string(), "4", "legacy_pool::msgs::MoonPoolBoostMsg::query_id");
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolBoostForwardPayload") => {
                let value = legacy_pool::msgs::MoonLegacyPoolBoostForwardPayloadMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolProvideLiquidity") => {
                let value = legacy_pool::msgs::MoonLegacyPoolProvideLiquidityMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolProvideLiquidityMsg::query_id"
                );
                assert_eq!(
                    value.amount1.to_u128().to_string(),
                    "2007",
                    "legacy_pool::msgs::MoonPoolProvideLiquidityMsg::amount1"
                );
                assert_eq!(
                    value.amount2.to_u128().to_string(),
                    "3007",
                    "legacy_pool::msgs::MoonPoolProvideLiquidityMsg::amount2"
                );
                assert_eq!(
                    value.min_lp_amount.to_u128().to_string(),
                    "5007",
                    "legacy_pool::msgs::MoonPoolProvideLiquidityMsg::min_lp_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolCancelDeposit") => {
                let value = legacy_pool::msgs::MoonLegacyPoolCancelDepositMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolCancelDepositMsg::query_id"
                );
                assert_eq!(
                    value.amount1.to_u128().to_string(),
                    "2007",
                    "legacy_pool::msgs::MoonPoolCancelDepositMsg::amount1"
                );
                assert_eq!(
                    value.amount2.to_u128().to_string(),
                    "3007",
                    "legacy_pool::msgs::MoonPoolCancelDepositMsg::amount2"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolLpBurnNotification") => {
                let value = legacy_pool::msgs::MoonLegacyPoolLpBurnNotificationMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolLpBurnNotificationMsg::query_id"
                );
                assert_eq!(
                    value.lp_amount.to_u128().to_string(),
                    "2007",
                    "legacy_pool::msgs::MoonPoolLpBurnNotificationMsg::lp_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolJettonTransferNotification") => {
                let value = legacy_pool::msgs::MoonLegacyPoolJettonTransferNotificationMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolJettonTransferNotificationMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "legacy_pool::msgs::MoonPoolJettonTransferNotificationMsg::amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolCollectFees") => {
                let value = legacy_pool::msgs::MoonLegacyPoolCollectFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolCollectFeesMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolCollectFeesPayout") => {
                let value = legacy_pool::msgs::MoonLegacyPoolCollectFeesPayoutMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolCollectFeesPayoutMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolSetFees") => {
                let value = legacy_pool::msgs::MoonLegacyPoolSetFeesMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolSetFeesMsg::query_id"
                );
                assert_eq!(
                    value.swap_fee_bps.to_string(),
                    "5",
                    "legacy_pool::msgs::MoonPoolSetFeesMsg::swap_fee_bps"
                );
                assert_eq!(
                    value.protocol_fee_bps.to_string(),
                    "6",
                    "legacy_pool::msgs::MoonPoolSetFeesMsg::protocol_fee_bps"
                );
                assert_eq!(
                    value.referral_fee_bps.to_string(),
                    "7",
                    "legacy_pool::msgs::MoonPoolSetFeesMsg::referral_fee_bps"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolSetContent") => {
                let value = legacy_pool::msgs::MoonLegacyPoolSetContentMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolSetContentMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolUpgradeCode") => {
                let value = legacy_pool::msgs::MoonLegacyPoolUpgradeCodeMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolUpgradeCodeMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolExcess") => {
                let value = legacy_pool::msgs::MoonLegacyPoolExcessMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolExcessMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolJettonInternalTransfer") => {
                let value = legacy_pool::msgs::MoonLegacyPoolJettonInternalTransferMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolJettonInternalTransferMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "legacy_pool::msgs::MoonPoolJettonInternalTransferMsg::amount"
                );
                assert_eq!(
                    value.forward_ton_amount.to_u128().to_string(),
                    "5007",
                    "legacy_pool::msgs::MoonPoolJettonInternalTransferMsg::forward_ton_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolJettonTransfer") => {
                let value = legacy_pool::msgs::MoonLegacyPoolJettonTransferMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolJettonTransferMsg::query_id"
                );
                assert_eq!(
                    value.amount.to_u128().to_string(),
                    "2007",
                    "legacy_pool::msgs::MoonPoolJettonTransferMsg::amount"
                );
                assert_eq!(
                    value.forward_ton_amount.to_u128().to_string(),
                    "6007",
                    "legacy_pool::msgs::MoonPoolJettonTransferMsg::forward_ton_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolSwapSucceed") => {
                let value = legacy_pool::msgs::MoonLegacyPoolSwapSucceedMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolSwapFailed") => {
                let value = legacy_pool::msgs::MoonLegacyPoolSwapFailedMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolProvideLiquiditySucceed") => {
                let value = legacy_pool::msgs::MoonLegacyPoolProvideLiquiditySucceedMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolProvideLiquidityFailed") => {
                let value = legacy_pool::msgs::MoonLegacyPoolProvideLiquidityFailedMsg::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolDepositRecord") => {
                let value = legacy_pool::msgs::MoonLegacyPoolDepositRecordMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolDepositRecordMsg::query_id"
                );
                assert_eq!(
                    value.asset_number.to_string(),
                    "5",
                    "legacy_pool::msgs::MoonPoolDepositRecordMsg::asset_number"
                );
                assert_eq!(
                    value.deposit_amount.to_u128().to_string(),
                    "3007",
                    "legacy_pool::msgs::MoonPoolDepositRecordMsg::deposit_amount"
                );
                assert_eq!(
                    value.min_lp_amount.to_u128().to_string(),
                    "4007",
                    "legacy_pool::msgs::MoonPoolDepositRecordMsg::min_lp_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolWithdrawLiquidityPayout") => {
                let value = legacy_pool::msgs::MoonLegacyPoolWithdrawLiquidityPayoutMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolWithdrawLiquidityPayoutMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolWithdrawLiquidityNotify") => {
                let value = legacy_pool::msgs::MoonLegacyPoolWithdrawLiquidityNotifyMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolWithdrawLiquidityNotifyMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolCancelDepositPayout") => {
                let value = legacy_pool::msgs::MoonLegacyPoolCancelDepositPayoutMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolCancelDepositPayoutMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolReferralPayout") => {
                let value = legacy_pool::msgs::MoonLegacyPoolReferralPayoutMsg::from_cell(&cell)?;
                assert_eq!(
                    value.query_id.to_string(),
                    "4",
                    "legacy_pool::msgs::MoonPoolReferralPayoutMsg::query_id"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolReservesReply") => {
                let value = legacy_pool::storage::MoonLegacyPoolReservesReply::from_cell(&cell)?;
                assert_eq!(
                    value.asset1_reserve.to_string(),
                    "2",
                    "legacy_pool::storage::MoonPoolReservesReply::asset1_reserve"
                );
                assert_eq!(
                    value.asset2_reserve.to_string(),
                    "4",
                    "legacy_pool::storage::MoonPoolReservesReply::asset2_reserve"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolAssetsReply") => {
                let value = legacy_pool::storage::MoonLegacyPoolAssetsReply::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolWalletsReply") => {
                let value = legacy_pool::storage::MoonLegacyPoolWalletsReply::from_cell(&cell)?;
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolFeesReply") => {
                let value = legacy_pool::storage::MoonLegacyPoolFeesReply::from_cell(&cell)?;
                assert_eq!(
                    value.swap_fee_bps.to_string(),
                    "1",
                    "legacy_pool::storage::MoonPoolFeesReply::swap_fee_bps"
                );
                assert_eq!(
                    value.protocol_fee_bps.to_string(),
                    "2",
                    "legacy_pool::storage::MoonPoolFeesReply::protocol_fee_bps"
                );
                assert_eq!(
                    value.referral_fee_bps.to_string(),
                    "3",
                    "legacy_pool::storage::MoonPoolFeesReply::referral_fee_bps"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolCollectedFeesReply") => {
                let value = legacy_pool::storage::MoonLegacyPoolCollectedFeesReply::from_cell(&cell)?;
                assert_eq!(
                    value.asset1_fees.to_string(),
                    "1",
                    "legacy_pool::storage::MoonPoolCollectedFeesReply::asset1_fees"
                );
                assert_eq!(
                    value.asset2_fees.to_string(),
                    "2",
                    "legacy_pool::storage::MoonPoolCollectedFeesReply::asset2_fees"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolEstimatedLpBurnReply") => {
                let value = legacy_pool::storage::MoonLegacyPoolEstimatedLpBurnReply::from_cell(&cell)?;
                assert_eq!(
                    value.asset1_amount.to_string(),
                    "2",
                    "legacy_pool::storage::MoonPoolEstimatedLpBurnReply::asset1_amount"
                );
                assert_eq!(
                    value.asset2_amount.to_string(),
                    "4",
                    "legacy_pool::storage::MoonPoolEstimatedLpBurnReply::asset2_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolEstimatedSwapOutReply") => {
                let value = legacy_pool::storage::MoonLegacyPoolEstimatedSwapOutReply::from_cell(&cell)?;
                assert_eq!(
                    value.output_amount.to_string(),
                    "2",
                    "legacy_pool::storage::MoonPoolEstimatedSwapOutReply::output_amount"
                );
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            ("legacy_pool", "MoonPoolJettonDataReply") => {
                let value = legacy_pool::storage::MoonLegacyPoolJettonDataReply::from_cell(&cell)?;
                assert_eq!(
                    value.total_supply.to_string(),
                    "1",
                    "legacy_pool::storage::MoonPoolJettonDataReply::total_supply"
                );
                assert!(!value.mintable);
                assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
            },
            _ => anyhow::bail!("unknown fixture"),
        }
    }
    Ok(())
}
