use crate::contracts::farming::distributor::storage::ToncoDistributorStorage;
use crate::contracts::farming::distributor::storage::ToncoFarmingDistributorMerkleRoots;
use crate::contracts::farming::msgs::ToncoFarmingClaimMsg;
use crate::contracts::farming::user_distributor::storage::ToncoUserDistributorStorage;
use crate::contracts::native::minter::method_results::ToncoNativeMinterGetJettonDataResult;
use crate::contracts::native::minter::method_results::ToncoNativeMinterGetVersionResult;
use crate::contracts::native::minter::msgs::ToncoNativeMinterTakeWalletAddressMsg;
use crate::contracts::native::minter::storage::ToncoNativeMinterStorage;
use crate::contracts::native::wallet::method_results::ToncoNativeWalletGetWalletDataResult;
use crate::contracts::native::wallet::msgs::ToncoNativeWalletV2Message;
use crate::contracts::native::wallet::storage::ToncoNativeWalletStorage;
use crate::contracts::v1::account::storage::ToncoAccountV1Storage;
use crate::contracts::v1::factory::msgs::ToncoFactoryV1CreatePoolMsg;
use crate::contracts::v1::factory::storage::ToncoFactoryV1Storage;
use crate::contracts::v1::order::storage::ToncoOrderV1Storage;
use crate::contracts::v1::pool::method_results::ToncoPoolTickPage;
use crate::contracts::v1::pool::msgs::ToncoPoolV1CollectProtocolMsg;
use crate::contracts::v1::pool::msgs::ToncoPoolV1InitMsg;
use crate::contracts::v1::pool::msgs::ToncoPoolV1SetFeeMsg;
use crate::contracts::v1::pool::storage::ToncoPoolV1Storage;
use crate::contracts::v1::pool::storage::ToncoPoolV1Ticks;
use crate::contracts::v1::position_nft::storage::ToncoPositionNftV1Storage;
use crate::contracts::v1::router::msgs::ToncoRouterV1ChangeAdminCommitMsg;
use crate::contracts::v1::router::msgs::ToncoRouterV1ChangeAdminStartMsg;
use crate::contracts::v1::router::msgs::ToncoRouterV1CreatePoolMsg;
use crate::contracts::v1::router::msgs::ToncoRouterV1JettonForwardPayload;
use crate::contracts::v1::router::storage::ToncoRouterV1Storage;
use crate::contracts::v2;
use crate::contracts::v2::account::storage::ToncoAccountV2Storage;
use crate::contracts::v2::factory::msgs::ToncoFactoryV2AddWhitelistMsg;
use crate::contracts::v2::factory::msgs::ToncoFactoryV2CreatePoolMsg;
use crate::contracts::v2::factory::msgs::ToncoFactoryV2DeleteWhitelistMsg;
use crate::contracts::v2::factory::storage::ToncoFactoryV2Storage;
use crate::contracts::v2::msgs::ToncoPayToV2Msg;
use crate::contracts::v2::msgs::ToncoReforgeV2Msg;
use crate::contracts::v2::msgs::ToncoSetOrderV2Msg;
use crate::contracts::v2::order::storage::ToncoOrderV2Storage;
use crate::contracts::v2::pool::method_results::ToncoPoolV2AmountsResult;
use crate::contracts::v2::pool::method_results::ToncoPoolV2GetCollectionDataResult;
use crate::contracts::v2::pool::msgs::ToncoPoolV2CollectProtocolMsg;
use crate::contracts::v2::pool::msgs::ToncoPoolV2GetDataMsg;
use crate::contracts::v2::pool::msgs::ToncoPoolV2InitMsg;
use crate::contracts::v2::pool::msgs::ToncoPoolV2LockMsg;
use crate::contracts::v2::pool::msgs::ToncoPoolV2ReportDataMsg;
use crate::contracts::v2::pool::msgs::ToncoPoolV2ResetGasMsg;
use crate::contracts::v2::pool::msgs::ToncoPoolV2SetFeesMsg;
use crate::contracts::v2::pool::msgs::ToncoPoolV2StartBurnMsg;
use crate::contracts::v2::pool::msgs::ToncoPoolV2SwapMsg;
use crate::contracts::v2::pool::msgs::ToncoPoolV2UnlockMsg;
use crate::contracts::v2::pool::storage::ToncoPoolV2State;
use crate::contracts::v2::pool::storage::ToncoPoolV2Storage;
use crate::contracts::v2::position_nft::msgs::ToncoPositionNftV2BurnMsg;
use crate::contracts::v2::position_nft::msgs::ToncoPositionNftV2DepositMsg;
use crate::contracts::v2::position_nft::msgs::ToncoPositionNftV2GetDataMsg;
use crate::contracts::v2::position_nft::msgs::ToncoPositionNftV2InitMsg;
use crate::contracts::v2::position_nft::msgs::ToncoPositionNftV2ReportDataMsg;
use crate::contracts::v2::position_nft::storage::ToncoPositionNftV2Storage;
use crate::contracts::v2::router::msgs::ToncoRouterV2ChangeParamsMsg;
use crate::contracts::v2::router::msgs::ToncoRouterV2CreatePoolMsg;
use crate::contracts::v2::router::msgs::ToncoRouterV2FundRouterMsg;
use crate::contracts::v2::router::msgs::ToncoRouterV2FundSomeoneMsg;
use crate::contracts::v2::router::msgs::ToncoRouterV2SwapForwardMsg;
use crate::contracts::v2::router::storage::ToncoRouterV2Storage;
use fastnum::I512;
use std::collections::BTreeSet;
use ton::ton_core::types::tlb_core::MsgAddress;
use ton::ton_core::{cell::TonCell, traits::tlb::TLB};
fn check<T: TLB>(cell: &TonCell) -> anyhow::Result<()> {
    let decoded = T::from_cell(cell)?;
    assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_sdk_wire_vectors() -> anyhow::Result<()> {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("fixtures/sdk_wire.json"))?;
    let mut seen = BTreeSet::new();
    for v in super::fixture_rows(&vectors, 46, "SDK wire")? {
        let id = v["id"].as_str().ok_or_else(|| anyhow::anyhow!("id"))?;
        anyhow::ensure!(seen.insert(id), "duplicate SDK wire fixture: {id}");
        let cell = TonCell::from_boc_hex(v["boc"].as_str().ok_or_else(|| anyhow::anyhow!("boc"))?)?;
        match id {
            "v1_pool_initial" => check::<ToncoPoolV1Storage>(&cell)?,
            "v1_router_create" => check::<ToncoRouterV1CreatePoolMsg>(&cell)?,
            "v2_router_create" => check::<ToncoRouterV2CreatePoolMsg>(&cell)?,
            "v1_admin_start" => check::<ToncoRouterV1ChangeAdminStartMsg>(&cell)?,
            "v1_admin_commit" => check::<ToncoRouterV1ChangeAdminCommitMsg>(&cell)?,
            "v1_router_params_absent" => assert!(matches!(
                crate::contracts::v1::router::msgs::ToncoRouterV1IncomingMessage::from_cell(&cell)?,
                crate::contracts::v1::router::msgs::ToncoRouterV1IncomingMessage::Unknown(_)
            )),
            "v2_router_params" => check::<ToncoRouterV2ChangeParamsMsg>(&cell)?,
            "v2_swap_forward" => check::<ToncoRouterV2SwapForwardMsg>(&cell)?,
            "v2_fund_someone" => check::<ToncoRouterV2FundSomeoneMsg>(&cell)?,
            "v2_fund_router" => check::<ToncoRouterV2FundRouterMsg>(&cell)?,
            "v1_factory_create" => check::<ToncoFactoryV1CreatePoolMsg>(&cell)?,
            "v2_factory_create_minimal" | "v2_factory_create_tagged" => check::<ToncoFactoryV2CreatePoolMsg>(&cell)?,
            "v2_factory_add_whitelist" => check::<ToncoFactoryV2AddWhitelistMsg>(&cell)?,
            "v2_factory_del_whitelist" => check::<ToncoFactoryV2DeleteWhitelistMsg>(&cell)?,
            "v2_nft_init_stale" => {
                assert!(ToncoPositionNftV2InitMsg::from_cell(&cell).is_err())
            },
            "v2_nft_report_stale" => {
                assert!(ToncoPositionNftV2ReportDataMsg::from_cell(&cell).is_err())
            },
            "v2_nft_deposit" => check::<ToncoPositionNftV2DepositMsg>(&cell)?,
            "v2_nft_burn" => check::<ToncoPositionNftV2BurnMsg>(&cell)?,
            "v2_nft_get" => check::<ToncoPositionNftV2GetDataMsg>(&cell)?,
            "v1_reinit" => check::<ToncoPoolV1InitMsg>(&cell)?,
            "v2_pool_initial" => {
                check::<ToncoPoolV2Storage>(&cell)?;
                let s = ToncoPoolV2Storage::from_cell(&cell)?;
                assert_eq!(s.state.ticks[&-10].liquidity_net, fastnum::i128!(-111));
                assert_eq!(s.state.ticks[&-10].outer_fee_growth0, fastnum::i256!(-1));
                assert_eq!(s.state.ticks[&10].outer_fee_growth1, fastnum::i256!(-2));
                assert!(s.wallet0.is_some());
            },
            "v1_fees" => check::<ToncoPoolV1SetFeeMsg>(&cell)?,
            "v1_collect" => check::<ToncoPoolV1CollectProtocolMsg>(&cell)?,
            "v1_6_lock" => check::<ToncoPoolV2LockMsg>(&cell)?,
            "v1_6_unlock" => check::<ToncoPoolV2UnlockMsg>(&cell)?,
            "v1_6_reset" => check::<ToncoPoolV2ResetGasMsg>(&cell)?,
            "v1_6_fees" => check::<ToncoPoolV2SetFeesMsg>(&cell)?,
            "v1_6_swap" => assert!(
                ToncoPoolV2SwapMsg::from_cell(&cell).is_err(),
                "stale SDK swap body must not decode as current V2"
            ),
            "v1_6_burn" | "v2_burn_actions" => check::<ToncoPoolV2StartBurnMsg>(&cell)?,
            "v1_6_collect" | "v2_collect_target" => check::<ToncoPoolV2CollectProtocolMsg>(&cell)?,
            "v2_report" => check::<ToncoPoolV2ReportDataMsg>(&cell)?,
            "v2_get_data" => check::<ToncoPoolV2GetDataMsg>(&cell)?,
            "v2_reinit" => check::<ToncoPoolV2InitMsg>(&cell)?,
            "v2_router_initial" => check::<ToncoRouterV2Storage>(&cell)?,
            "v1_router_initial" => check::<ToncoRouterV1Storage>(&cell)?,
            "v2_factory_initial" => check::<ToncoFactoryV2Storage>(&cell)?,
            "v1_factory_initial" => {
                assert!(ToncoFactoryV1Storage::from_cell(&cell).is_err(), "stale SDK factory layout")
            },
            "v2_account_initial" => check::<ToncoAccountV2Storage>(&cell)?,
            "v2_nft_initial" => check::<ToncoPositionNftV2Storage>(&cell)?,
            "v2_reforge_order" => check::<ToncoSetOrderV2Msg>(&cell)?,
            "v2_reforge" => check::<ToncoReforgeV2Msg>(&cell)?,
            "distributor_initial" => check::<ToncoDistributorStorage>(&cell)?,
            "user_distributor_initial" => check::<ToncoUserDistributorStorage>(&cell)?,
            _ => anyhow::bail!("unmapped fixture {id}"),
        }
    }
    Ok(())
}
#[test]
fn test_unknown_malformed_and_ordinary_layout_tails() -> anyhow::Result<()> {
    use ton::ton_core::{
        cell::TonHash,
        types::{
            TonAddress,
            tlb_core::{MsgAddress, MsgAddressInt, MsgAddressIntStd},
        },
    };
    let literal_zero = MsgAddress::Int(MsgAddressInt::Std(MsgAddressIntStd {
        anycast: None,
        workchain: 0,
        address: TonHash::ZERO,
    }));
    let zero_cell = literal_zero.to_cell()?;
    let none_cell = MsgAddress::NONE.to_cell()?;
    assert_eq!(zero_cell.parser().data_bits_left()?, 267);
    assert_eq!(none_cell.parser().data_bits_left()?, 2);
    assert_eq!(v2::factory::whitelist_key(&TonAddress::ZERO)?, TonHash::ZERO);
    assert!(v2::factory::whitelist_key(&TonAddress::new(128, TonHash::ZERO)).is_err());
    assert_ne!(MsgAddress::from_cell(&zero_cell)?, MsgAddress::from_cell(&none_cell)?);
    let mut b = TonCell::builder();
    b.write_num(&0x42a0fb43u32, 32)?;
    assert!(crate::contracts::v1::pool::msgs::ToncoPoolV1IncomingMessage::from_cell(&b.build()?).is_err());
    let mut b = TonCell::builder();
    b.write_num(&0x42a0fb43u32, 32)?;
    b.write_num(&17u64, 64)?;
    let fixed = b.build()?;
    assert!(matches!(
        crate::contracts::v1::pool::msgs::ToncoPoolV1IncomingMessage::from_cell(&fixed)?,
        crate::contracts::v1::pool::msgs::ToncoPoolV1IncomingMessage::PoolResetGas(_)
    ));
    let mut b = TonCell::builder();
    b.write_cell(&fixed)?;
    b.write_bit(false)?;
    assert!(crate::contracts::v1::pool::msgs::ToncoPoolV1IncomingMessage::from_cell(&b.build()?).is_ok());
    let mut b = TonCell::builder();
    b.write_num(&0xdeadbeefu32, 32)?;
    b.write_bit(true)?;
    b.write_ref(TonCell::empty().clone())?;
    let raw = b.build()?;
    match crate::contracts::v1::pool::msgs::ToncoPoolV1IncomingMessage::from_cell(&raw)? {
        crate::contracts::v1::pool::msgs::ToncoPoolV1IncomingMessage::Unknown(x) => assert_eq!(x, raw),
        _ => anyhow::bail!("unknown body lost"),
    };
    Ok(())
}
#[test]
fn test_synthetic_native_proxy_storage_and_address_derivation() -> anyhow::Result<()> {
    use crate::contracts::native::{minter, wallet};
    use ton::ton_core::types::TonAddress;
    let m = minter::storage::ToncoNativeMinterStorage::from_boc(
        include_bytes!("fixtures/native/synthetic_minter.boc").to_vec(),
    )?;
    let w = wallet::storage::ToncoNativeWalletStorage::from_boc(
        include_bytes!("fixtures/native/synthetic_wallet.boc").to_vec(),
    )?;
    let expected: TonAddress = include_str!("fixtures/native/synthetic_wallet_address.txt").trim().parse()?;
    assert_eq!(w.balance.to_string(), "12345");
    assert_eq!(m.derive_wallet_address(w.owner, w.minter, 0)?, expected);
    Ok(())
}
#[test]
fn test_tick_tuple_outer_fees_and_scalar_tails() -> anyhow::Result<()> {
    use crate::contracts::v1::pool::method_results::ToncoPoolTickPage;
    use ton::block_tlb::{FromTVMStack, TVMStack, TVMTuple};
    let mut fields = TVMTuple::default();
    for v in [-100, 1000, -999, -1, 2] {
        fields.push_tiny_int(v);
    }
    let mut group = TVMTuple::default();
    group.push_tuple(fields);
    let mut groups = TVMTuple::default();
    groups.push_tuple(group);
    let mut stack = TVMStack::default();
    stack.push_tuple(groups);
    let page = ToncoPoolTickPage::from_stack(&mut stack)?;
    assert_eq!(page.entries[0].index, -100);
    assert_eq!(page.entries[0].outer_fees, Some((I512::from_i64(-1), I512::from_i64(2))));
    let mut stack = TVMStack::default();
    stack.push_tiny_int(1);
    stack.push_tiny_int(2);
    assert!(bool::from_stack(&mut stack)?);
    assert_eq!(stack.len(), 1);
    Ok(())
}
#[test]
fn test_observed_v2_mint_burn_and_refund_vectors() -> anyhow::Result<()> {
    use anyhow::Context;
    for (bytes, expected) in [
        (include_bytes!("fixtures/flow/minted_pool_data.boc").as_slice(), 20000000),
        (include_bytes!("fixtures/flow/swapped_pool_data.boc").as_slice(), 20000000),
        (include_bytes!("fixtures/flow/burned_pool_data.boc").as_slice(), 0),
    ] {
        let pool = ToncoPoolV2Storage::from_boc(bytes.to_vec())?;
        assert_eq!(pool.state.liquidity, expected);
    }
    for (bytes, expected) in [
        (include_bytes!("fixtures/flow/minted_nft_data.boc").as_slice(), 20000000),
        (include_bytes!("fixtures/flow/burned_nft_data.boc").as_slice(), 0),
    ] {
        let nft = ToncoPositionNftV2Storage::from_boc(bytes.to_vec())?;
        assert_eq!(nft.position.liquidity, expected);
    }
    for bytes in [
        include_bytes!("fixtures/flow/mint_reforge_body.boc").as_slice(),
        include_bytes!("fixtures/flow/burn_reforge_body.boc").as_slice(),
    ] {
        let body = TonCell::from_boc(bytes.to_vec())?;
        check::<ToncoReforgeV2Msg>(&body).context("reforge body")?;
    }
    for bytes in [
        include_bytes!("fixtures/flow/mint_payto_body.boc").as_slice(),
        include_bytes!("fixtures/flow/swap_payto_body.boc").as_slice(),
        include_bytes!("fixtures/flow/burn_payto_body.boc").as_slice(),
    ] {
        let cell = TonCell::from_boc(bytes.to_vec())?;
        check::<ToncoPayToV2Msg>(&cell).context("payto")?;
    }
    let init = TonCell::from_boc(include_bytes!("fixtures/flow/mint_init_body.boc").to_vec())?;
    check::<ToncoPositionNftV2InitMsg>(&init).context("position init body")?;
    let burn = TonCell::from_boc(include_bytes!("fixtures/flow/nft_burn_body.boc").to_vec())?;
    check::<ToncoPositionNftV2BurnMsg>(&burn).context("position burn body")?;
    Ok(())
}

fn decode_pool_getter_rows(rows: &serde_json::Value) -> anyhow::Result<()> {
    use anyhow::{Context, bail};
    use ton::block_tlb::{FromTVMStack, TVMStack};
    fn decode<T: FromTVMStack>(s: &mut TVMStack) -> anyhow::Result<()> {
        T::from_stack(s)?;
        s.ensure_empty()?;
        Ok(())
    }
    for row in super::fixture_rows(rows, 66, "getter replies")? {
        let rev = row["revision"].as_str().context("revision")?;
        let method = row["method"].as_str().context("method")?;
        let mut s = super::test_getter_provider::stack(&row["stack"])?;
        if let Some((family, name)) = method.split_once('/') {
            let result = match (rev, family, name) {
                ("1", "router", "getRouterState") => {
                    decode::<crate::contracts::v1::router::method_results::ToncoRouterV1GetRouterStateResult>(&mut s)
                },
                ("2", "router", "getRouterState") => {
                    decode::<crate::contracts::v2::router::method_results::ToncoRouterV2GetRouterStateResult>(&mut s)
                },
                ("1", "router", "getChildContracts") => {
                    decode::<crate::contracts::v1::router::method_results::ToncoRouterV1GetChildContractsResult>(&mut s)
                },
                ("2", "router", "getChildContracts") => {
                    decode::<crate::contracts::v2::router::method_results::ToncoRouterV2GetChildContractsResult>(&mut s)
                },
                (_, "router", "getPoolAddress") => decode::<MsgAddress>(&mut s),
                (_, "router", "getPoolInitialData" | "getPoolStateInit") => decode::<TonCell>(&mut s),
                (_, "router", "get_pending_timelocks") => decode::<
                    crate::contracts::v2::router::method_results::ToncoRouterV2GetPendingTimelocksResult,
                >(&mut s),
                (_, "position", "getPoolAddress" | "getUserAddress") => decode::<MsgAddress>(&mut s),
                ("1", "position", "getPositionInfo") => decode::<
                    crate::contracts::v1::position_nft::method_results::ToncoPositionNftV1GetPositionInfoResult,
                >(&mut s),
                (_, "position", "getPositionInfo") => decode::<
                    crate::contracts::v2::position_nft::method_results::ToncoPositionNftV2GetPositionInfoResult,
                >(&mut s),
                ("1", "position", "get_nft_data") => decode::<
                    crate::contracts::v1::position_nft::method_results::ToncoPositionNftV1GetNftDataResult,
                >(&mut s),
                (_, "position", "get_nft_data") => decode::<
                    crate::contracts::v2::position_nft::method_results::ToncoPositionNftV2GetNftDataResult,
                >(&mut s),
                ("1", "account", _) => {
                    decode::<crate::contracts::v1::account::method_results::ToncoAccountV1GetAccountDataResult>(&mut s)
                },
                ("2", "account", _) => {
                    decode::<crate::contracts::v2::account::method_results::ToncoAccountV2GetAccountDataResult>(&mut s)
                },
                ("1", "factory", "getPoolFactoryData") => decode::<
                    crate::contracts::v1::factory::method_results::ToncoFactoryV1GetPoolFactoryDataResult,
                >(&mut s),
                (_, "factory", "getPoolFactoryData") => decode::<
                    crate::contracts::v2::factory::method_results::ToncoFactoryV2GetPoolFactoryDataResult,
                >(&mut s),
                (_, "factory", "getOrderAddress") => decode::<MsgAddress>(&mut s),
                (_, "factory", "getWhitelist") => decode::<Option<TonCell>>(&mut s),
                (_, "wallet", "get_wallet_data") => decode::<ToncoNativeWalletGetWalletDataResult>(&mut s),
                (_, "minter", "get_jetton_data") => decode::<ToncoNativeMinterGetJettonDataResult>(&mut s),
                (_, "minter", "get_wallet_address") => decode::<MsgAddress>(&mut s),
                (_, "minter", "get_version") => decode::<ToncoNativeMinterGetVersionResult>(&mut s),
                (_, "distributor", "get_merkle_root") => decode::<I512>(&mut s),
                (_, "distributor", "get_admin" | "get_user_distributor_address") => decode::<MsgAddress>(&mut s),
                (_, "user_distributor", "get_claimed") => decode::<bool>(&mut s),
                _ => bail!("unmapped peripheral getter {rev}/{method}"),
            };
            result.with_context(|| format!("{rev}/{method}"))?;
            continue;
        }
        let result = match (rev, method) {
            ("1", "getPoolStateAndConfiguration") => {
                let p =
                    crate::contracts::v1::pool::method_results::ToncoPoolV1GetPoolStateAndConfigurationResult::from_stack(&mut s)?;
                assert_eq!(p.current_tick, I512::ZERO);
                assert_eq!(p.current_price.to_string(), "79228162514264337593543950336");
                assert_eq!(p.current_liquidity, I512::from_i64(20000000));
                assert_eq!(p.protocol_fee, I512::from_i64(2000));
                s.ensure_empty()?;
                Ok(())
            },
            ("2", "getPoolStateAndConfiguration") => {
                let p =
                    crate::contracts::v2::pool::method_results::ToncoPoolV2GetPoolStateAndConfigurationResult::from_stack(&mut s)?;
                assert_eq!(p.current_tick, I512::ZERO);
                assert_eq!(p.current_price.to_string(), "79228162514264337593543950336");
                assert_eq!(p.current_liquidity, I512::from_i64(20000000));
                assert_eq!(p.protocol_fee, I512::from_i64(2000));
                assert_eq!(p.version, I512::from_i64(16000));
                s.ensure_empty()?;
                Ok(())
            },
            ("1", "getChildContracts") => {
                decode::<crate::contracts::v1::pool::method_results::ToncoPoolV1GetChildContractsResult>(&mut s)
            },
            (_, "getChildContracts") => {
                decode::<crate::contracts::v2::pool::method_results::ToncoPoolV2GetChildContractsResult>(&mut s)
            },
            (_, "getIsActive") => decode::<bool>(&mut s),
            (_, "getMaxLiquidityPerTick") => decode::<I512>(&mut s),
            (_, "get_collection_data") => decode::<ToncoPoolV2GetCollectionDataResult>(&mut s),
            ("1", "getAllTickInfos") => decode::<TonCell>(&mut s),
            ("2", "getAllTickInfos") => decode::<Option<TonCell>>(&mut s),
            (_, "getTickInfosFrom") => decode::<ToncoPoolTickPage>(&mut s),
            ("1", "getMintEstimate") => {
                decode::<crate::contracts::v1::pool::method_results::ToncoPoolV1GetMintEstimateResult>(&mut s)
            },
            (_, "getMintEstimate") => {
                decode::<crate::contracts::v2::pool::method_results::ToncoPoolV2GetMintEstimateResult>(&mut s)
            },
            (_, "getCollectedFees" | "getFeeGrowthInside" | "getSwapEstimate" | "getSwapEstimateGas") => {
                decode::<ToncoPoolV2AmountsResult>(&mut s)
            },
            (_, "getUserAccountAddress" | "get_nft_address_by_index") => decode::<MsgAddress>(&mut s),
            (_, "get_nft_content") => decode::<TonCell>(&mut s),
            (_, "get_pending_timelocks") => {
                decode::<crate::contracts::v2::pool::method_results::ToncoPoolV2GetPendingTimelocksResult>(&mut s)
            },
            _ => bail!("unmapped getter {rev}/{method}"),
        };
        result.with_context(|| format!("{rev}/{method}"))?;
    }
    Ok(())
}

#[test]
fn test_pool_getter_structural_observations() -> anyhow::Result<()> {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    decode_pool_getter_rows(&fixture["rows"])
}
#[test]
fn test_independently_authored_protocol_messages() -> anyhow::Result<()> {
    use anyhow::{Context, bail};
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/protocol_wire.json"))?;
    macro_rules! typed {
        ($ty:ty,$cell:expr,$unknown:path) => {{
            let value = <$ty>::from_cell($cell)?;
            assert!(!matches!(value, $unknown(_)));
            assert_eq!(value.to_cell()?.hash()?, $cell.hash()?);
            let opcode: u32 = $cell.parser().read_num(32)?;
            let mut truncated = TonCell::builder();
            truncated.write_num(&opcode, 32)?;
            assert!(<$ty>::from_cell(&truncated.build()?).is_err());
            let mut unrecognized = TonCell::builder();
            unrecognized.write_num(&0xdeadbeefu32, 32)?;
            unrecognized.write_bit(true)?;
            unrecognized.write_ref(TonCell::empty().clone())?;
            let raw = unrecognized.build()?;
            let decoded = <$ty>::from_cell(&raw)?;
            assert!(matches!(decoded, $unknown(_)));
            assert_eq!(decoded.to_cell()?.hash()?, raw.hash()?);
        }};
    }
    for row in super::fixture_rows(&rows, 70, "protocol wire")? {
        let cell = TonCell::from_boc_hex(row["boc"].as_str().context("boc")?)?;
        let family = row["family"].as_str().context("family")?;
        let result = (|| -> anyhow::Result<()> {
            match family {
                "v1/position-out" => typed!(
                    crate::contracts::v1::position_nft::msgs::ToncoPositionNftV1OutgoingMessage,
                    &cell,
                    crate::contracts::v1::position_nft::msgs::ToncoPositionNftV1OutgoingMessage::Unknown
                ),
                "native/out" => typed!(
                    crate::contracts::native::wallet::msgs::ToncoNativeWalletOutgoingMessage,
                    &cell,
                    crate::contracts::native::wallet::msgs::ToncoNativeWalletOutgoingMessage::Unknown
                ),
                "v1/forward" => {
                    let value = ToncoRouterV1JettonForwardPayload::from_cell(&cell)?;
                    assert!(!matches!(value, ToncoRouterV1JettonForwardPayload::Unknown(_)));
                    assert_eq!(value.to_cell()?.hash()?, cell.hash()?);
                },
                "v1/pool" => typed!(
                    crate::contracts::v1::pool::msgs::ToncoPoolV1IncomingMessage,
                    &cell,
                    crate::contracts::v1::pool::msgs::ToncoPoolV1IncomingMessage::Unknown
                ),
                "v2/pool" => typed!(
                    crate::contracts::v2::pool::msgs::ToncoPoolV2IncomingMessage,
                    &cell,
                    crate::contracts::v2::pool::msgs::ToncoPoolV2IncomingMessage::Unknown
                ),
                "v1/router" => typed!(
                    crate::contracts::v1::router::msgs::ToncoRouterV1IncomingMessage,
                    &cell,
                    crate::contracts::v1::router::msgs::ToncoRouterV1IncomingMessage::Unknown
                ),
                "v2/router" => typed!(
                    crate::contracts::v2::router::msgs::ToncoRouterV2IncomingMessage,
                    &cell,
                    crate::contracts::v2::router::msgs::ToncoRouterV2IncomingMessage::Unknown
                ),
                "v1/factory" => typed!(
                    crate::contracts::v1::factory::msgs::ToncoFactoryV1IncomingMessage,
                    &cell,
                    crate::contracts::v1::factory::msgs::ToncoFactoryV1IncomingMessage::Unknown
                ),
                "v2/factory" => typed!(
                    crate::contracts::v2::factory::msgs::ToncoFactoryV2IncomingMessage,
                    &cell,
                    crate::contracts::v2::factory::msgs::ToncoFactoryV2IncomingMessage::Unknown
                ),
                "v1/account" => typed!(
                    crate::contracts::v1::account::msgs::ToncoAccountV1IncomingMessage,
                    &cell,
                    crate::contracts::v1::account::msgs::ToncoAccountV1IncomingMessage::Unknown
                ),
                "v2/account" => typed!(
                    crate::contracts::v2::account::msgs::ToncoAccountV2IncomingMessage,
                    &cell,
                    crate::contracts::v2::account::msgs::ToncoAccountV2IncomingMessage::Unknown
                ),
                "v1/position" => typed!(
                    crate::contracts::v1::position_nft::msgs::ToncoPositionNftV1IncomingMessage,
                    &cell,
                    crate::contracts::v1::position_nft::msgs::ToncoPositionNftV1IncomingMessage::Unknown
                ),
                "v2/position" => typed!(
                    crate::contracts::v2::position_nft::msgs::ToncoPositionNftV2IncomingMessage,
                    &cell,
                    crate::contracts::v2::position_nft::msgs::ToncoPositionNftV2IncomingMessage::Unknown
                ),
                "v2/nft-report" => check::<ToncoPositionNftV2ReportDataMsg>(&cell)?,
                "native/minter" => typed!(
                    crate::contracts::native::minter::msgs::ToncoNativeMinterIncomingMessage,
                    &cell,
                    crate::contracts::native::minter::msgs::ToncoNativeMinterIncomingMessage::Unknown
                ),
                "native/take" => {
                    check::<ToncoNativeMinterTakeWalletAddressMsg>(&cell)?;
                    check::<ToncoNativeMinterTakeWalletAddressMsg>(&cell)?;
                    check::<ToncoNativeMinterTakeWalletAddressMsg>(&cell)?;
                },
                "native/wallet" => typed!(ToncoNativeWalletV2Message, &cell, ToncoNativeWalletV2Message::Unknown),
                "farming/incoming" => typed!(
                    crate::contracts::farming::msgs::ToncoFarmingIncomingMessage,
                    &cell,
                    crate::contracts::farming::msgs::ToncoFarmingIncomingMessage::Unknown
                ),
                "farming/claim" => check::<ToncoFarmingClaimMsg>(&cell)?,
                _ => bail!("unmapped protocol family {family}"),
            };
            Ok(())
        })();
        result.with_context(|| row["id"].as_str().unwrap_or("missing vector id").to_owned())?;
    }
    Ok(())
}

#[test]
fn test_all_storage_state_init_addresses_against_independent_encoder() -> anyhow::Result<()> {
    use anyhow::{Context, bail};
    use ton::ton_core::types::TonAddress;
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/state_init.json"))?;
    for row in super::fixture_rows(&rows, 16, "state init")? {
        let data = TonCell::from_boc_hex(row["data"].as_str().context("data")?)?;
        let code = TonCell::from_boc_hex(row["code"].as_str().context("code")?)?;
        let expected: TonAddress = row["address"].as_str().context("address")?.parse()?;
        macro_rules! init {
            ($ty:ty) => {
                <$ty>::from_cell(&data)?.state_init(code.clone())?.derive_address(0)?
            };
        }
        let actual = match row["family"].as_str().context("family")? {
            "v1/pool" => init!(ToncoPoolV1Storage),
            "v2/pool" => init!(ToncoPoolV2Storage),
            "v1/router" => init!(ToncoRouterV1Storage),
            "v2/router" => init!(ToncoRouterV2Storage),
            "v1/account" => init!(ToncoAccountV1Storage),
            "v2/account" => init!(ToncoAccountV2Storage),
            "v1/position" => init!(ToncoPositionNftV1Storage),
            "v2/position" => init!(ToncoPositionNftV2Storage),
            "v1/factory" => init!(ToncoFactoryV1Storage),
            "v2/factory" => init!(ToncoFactoryV2Storage),
            "v1/order" => init!(ToncoOrderV1Storage),
            "v2/order" => init!(ToncoOrderV2Storage),
            "native/minter" => init!(ToncoNativeMinterStorage),
            "native/wallet" => init!(ToncoNativeWalletStorage),
            "farming/distributor" => init!(ToncoDistributorStorage),
            "farming/user" => init!(ToncoUserDistributorStorage),
            f => bail!("unmapped storage {f}"),
        };
        assert_eq!(actual, expected, "{}", row["family"]);
    }
    Ok(())
}

#[test]
fn test_native_dictionary_tails_and_alternate_labels() -> anyhow::Result<()> {
    use anyhow::{Context, bail};
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/dictionary_boundaries.json"))?;
    for row in super::fixture_rows(&rows, 32, "dictionary boundaries")? {
        let cell = TonCell::from_boc_hex(row["boc"].as_str().context("BOC")?)?;
        let accepted = match row["family"].as_str().context("family")? {
            "v1/ticks" => ToncoPoolV1Ticks::from_cell(&cell).is_ok(),
            "v2/state" => ToncoPoolV2State::from_cell(&cell).is_ok(),
            "farming/roots" => ToncoFarmingDistributorMerkleRoots::from_cell(&cell).is_ok(),
            "v2/whitelist" => ToncoFactoryV2Storage::from_cell(&cell).is_ok(),
            f => bail!("unmapped dictionary {f}"),
        };
        assert!(accepted, "native dictionary policy rejected {}", row["id"]);
    }
    Ok(())
}

#[test]
fn test_wallet_ordering_uses_complete_address_cell_hash() -> anyhow::Result<()> {
    use crate::contracts::addresses::ordered_wallets;
    use anyhow::Context;

    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/wallet_ordering.json"))?;
    for row in super::fixture_rows(&rows, 10, "wallet ordering")? {
        let first = ton::ton_core::types::tlb_core::adapters::NullableAddress
            .read(&mut TonCell::from_boc_hex(row["first"].as_str().context("first")?)?.parser())?;
        let second = ton::ton_core::types::tlb_core::adapters::NullableAddress
            .read(&mut TonCell::from_boc_hex(row["second"].as_str().context("second")?)?.parser())?;
        let expected = if row["first_precedes"].as_bool().context("direction")? {
            (first, second)
        } else {
            (second, first)
        };
        assert_eq!(ordered_wallets(first, second)?, expected);
        assert_eq!(ordered_wallets(second, first)?, expected);
        assert!(ordered_wallets(first, first).is_err());
    }
    Ok(())
}
