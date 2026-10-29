//! All 66 source getter capabilities exercised through a caller-owned client.

use crate::contracts::farming::distributor::ToncoDistributorContract;
use crate::contracts::farming::user_distributor::ToncoUserDistributorContract;
use crate::contracts::native::minter::ToncoNativeMinterContract;
use crate::contracts::native::wallet::ToncoNativeWalletV1Contract;
use crate::contracts::native::wallet::ToncoNativeWalletV2Contract;
use crate::contracts::v1::account::ToncoAccountV1Contract;
use crate::contracts::v1::factory::ToncoFactoryV1Contract;
use crate::contracts::v1::pool::ToncoPoolV1Contract;
use crate::contracts::v1::position_nft::ToncoPositionNftV1Contract;
use crate::contracts::v1::router::ToncoRouterV1Contract;
use crate::contracts::v2::account::ToncoAccountV2Contract;
use crate::contracts::v2::factory::ToncoFactoryV2Contract;
use crate::contracts::v2::pool::ToncoPoolV2Contract;
use crate::contracts::v2::position_nft::ToncoPositionNftV2Contract;
use crate::contracts::v2::router::ToncoRouterV2Contract;
use ton::contracts::TonContract;
use ton::errors::TonError;
use ton::ton_core::errors::TonCoreError;

use ton::ton_core::{cell::TonCell, traits::tlb::TLB, types::TonAddress};

use ton::block_tlb::TVMStack;

use fastnum::I512;

use super::test_getter_provider::{recording_client, stack};

#[test]
fn test_distributor_get_merkle_root() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][62]["stack"])?;
    let mut arguments = TVMStack::default();
    let farming_id = I512::from_i64(-11);
    arguments.push_tiny_int(-11);
    let (client, recorder) = recording_client(108652, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoDistributorContract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_merkle_root(farming_id))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_distributor_get_admin() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][63]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(113142, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoDistributorContract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_admin())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_distributor_get_user_distributor_address() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][64]["stack"])?;
    let mut arguments = TVMStack::default();
    let proof_hash = I512::from_i64(-11);
    let index = I512::from_i64(12);
    let farming_id = I512::from_i64(-13);
    let position_id = I512::from_i64(14);
    arguments.push_tiny_int(-11);
    arguments.push_tiny_int(12);
    arguments.push_tiny_int(-13);
    arguments.push_tiny_int(14);
    let (client, recorder) = recording_client(93775, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoDistributorContract::new(&client, &address, None);
    let _result =
        tokio_test::block_on(contract.get_user_distributor_address(proof_hash, index, farming_id, position_id))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_userdistributor_get_claimed() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][65]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(85099, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoUserDistributorContract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_claimed())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_nativeminter_get_jetton_data() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][59]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(106029, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoNativeMinterContract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_jetton_data())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_nativeminter_get_wallet_address() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
        let reply = stack(&rows["rows"][60]["stack"])?;
        let mut arguments = TVMStack::default();
        let owner = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::new(0, [1; 32].into())
        };
        arguments.push_cell_slice(owner.to_msg_address_int().to_cell()?);
        let (client, recorder) = recording_client(103289, arguments, reply)?;
        let address = TonAddress::new(0, [42; 32].into());
        let contract = ToncoNativeMinterContract::new(&client, &address, None);
        let _result = tokio_test::block_on(contract.get_wallet_address(&owner))?;
        recorder.ensure_consumed()?;
    }
    Ok(())
}

#[test]
fn test_nativeminter_get_version() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][61]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(82320, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoNativeMinterContract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_version())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_nativewalletv1_get_wallet_data() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][43]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(97026, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoNativeWalletV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_wallet_data())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_nativewalletv2_get_wallet_data() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][58]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(97026, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoNativeWalletV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_wallet_data())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_accountv1_get_account_data() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][40]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(86329, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoAccountV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_account_data())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_factoryv1_get_order_address() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
        let reply = stack(&rows["rows"][42]["stack"])?;
        let mut arguments = TVMStack::default();
        let minter0 = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::new(0, [1; 32].into())
        };
        let minter1 = TonAddress::new(1, [2; 32].into());
        arguments.push_cell_slice(minter0.to_msg_address_int().to_cell()?);
        arguments.push_cell_slice(minter1.to_msg_address_int().to_cell()?);
        let (client, recorder) = recording_client(96490, arguments, reply)?;
        let address = TonAddress::new(0, [42; 32].into());
        let contract = ToncoFactoryV1Contract::new(&client, &address, None);
        let _result = tokio_test::block_on(contract.get_order_address(&minter0, &minter1))?;
        recorder.ensure_consumed()?;
    }
    Ok(())
}

#[test]
fn test_factoryv1_get_pool_factory_data() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][41]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(119679, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoFactoryV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_pool_factory_data())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv1_get_tick_infos_from() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][6]["stack"])?;
    let mut arguments = TVMStack::default();
    let start = I512::from_i64(-11);
    let count = I512::from_i64(12);
    let back = I512::from_i64(-13);
    let full = I512::from_i64(14);
    arguments.push_tiny_int(-11);
    arguments.push_tiny_int(12);
    arguments.push_tiny_int(-13);
    arguments.push_tiny_int(14);
    let (client, recorder) = recording_client(116046, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_tick_infos_from(start, count, back, full))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv1_get_is_active() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][2]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(82247, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_is_active())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv1_get_pool_state_and_configuration() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][0]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(118400, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_pool_state_and_configuration())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv1_get_child_contracts() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][1]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(124601, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_child_contracts())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv1_get_all_tick_infos() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][5]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(96585, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_all_tick_infos())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv1_get_max_liquidity_per_tick() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][3]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(66347, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_max_liquidity_per_tick())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv1_get_collected_fees() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][8]["stack"])?;
    let mut arguments = TVMStack::default();
    let tick_lower = I512::from_i64(-11);
    let tick_upper = I512::from_i64(12);
    let pos_liquidity_delta = I512::from_i64(-13);
    let pos_fee_growth_inside0_x128 = I512::from_i64(14);
    let pos_fee_growth_inside1_x128 = I512::from_i64(-15);
    arguments.push_tiny_int(-11);
    arguments.push_tiny_int(12);
    arguments.push_tiny_int(-13);
    arguments.push_tiny_int(14);
    arguments.push_tiny_int(-15);
    let (client, recorder) = recording_client(108894, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_collected_fees(
        tick_lower,
        tick_upper,
        pos_liquidity_delta,
        pos_fee_growth_inside0_x128,
        pos_fee_growth_inside1_x128,
    ))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv1_get_mint_estimate() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][7]["stack"])?;
    let mut arguments = TVMStack::default();
    let tick_lower = I512::from_i64(-11);
    let tick_upper = I512::from_i64(12);
    let liquidity = I512::from_i64(-13);
    arguments.push_tiny_int(-11);
    arguments.push_tiny_int(12);
    arguments.push_tiny_int(-13);
    let (client, recorder) = recording_client(108264, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_mint_estimate(tick_lower, tick_upper, liquidity))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv1_get_swap_estimate() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][10]["stack"])?;
    let mut arguments = TVMStack::default();
    let zero_for_one = I512::from_i64(-11);
    let amount = I512::from_i64(12);
    let sqrt_price_limit_x96 = I512::from_i64(-13);
    arguments.push_tiny_int(-11);
    arguments.push_tiny_int(12);
    arguments.push_tiny_int(-13);
    let (client, recorder) = recording_client(85303, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_swap_estimate(zero_for_one, amount, sqrt_price_limit_x96))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv1_get_swap_estimate_gas() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][11]["stack"])?;
    let mut arguments = TVMStack::default();
    let zero_for_one = I512::from_i64(-11);
    let amount = I512::from_i64(12);
    let sqrt_price_limit_x96 = I512::from_i64(-13);
    let min_out_amount = I512::from_i64(14);
    let gas_limit = I512::from_i64(-15);
    arguments.push_tiny_int(-11);
    arguments.push_tiny_int(12);
    arguments.push_tiny_int(-13);
    arguments.push_tiny_int(14);
    arguments.push_tiny_int(-15);
    let (client, recorder) = recording_client(77100, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_swap_estimate_gas(
        zero_for_one,
        amount,
        sqrt_price_limit_x96,
        min_out_amount,
        gas_limit,
    ))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv1_get_fee_growth_inside() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][9]["stack"])?;
    let mut arguments = TVMStack::default();
    let tick_lower = I512::from_i64(-11);
    let tick_upper = I512::from_i64(12);
    let tick_current = I512::from_i64(-13);
    let fee_growth_global0_x128 = I512::from_i64(14);
    let fee_growth_global1_x128 = I512::from_i64(-15);
    arguments.push_tiny_int(-11);
    arguments.push_tiny_int(12);
    arguments.push_tiny_int(-13);
    arguments.push_tiny_int(14);
    arguments.push_tiny_int(-15);
    let (client, recorder) = recording_client(130735, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_fee_growth_inside(
        tick_lower,
        tick_upper,
        tick_current,
        fee_growth_global0_x128,
        fee_growth_global1_x128,
    ))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv1_get_user_account_address() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
        let reply = stack(&rows["rows"][12]["stack"])?;
        let mut arguments = TVMStack::default();
        let user_address = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::new(0, [1; 32].into())
        };
        arguments.push_cell_slice(user_address.to_msg_address_int().to_cell()?);
        let (client, recorder) = recording_client(67666, arguments, reply)?;
        let address = TonAddress::new(0, [42; 32].into());
        let contract = ToncoPoolV1Contract::new(&client, &address, None);
        let _result = tokio_test::block_on(contract.get_user_account_address(&user_address))?;
        recorder.ensure_consumed()?;
    }
    Ok(())
}

#[test]
fn test_poolv1_get_collection_data() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][4]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(102491, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_collection_data())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv1_get_nft_address_by_index() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][13]["stack"])?;
    let mut arguments = TVMStack::default();
    let index = I512::from_i64(-11);
    arguments.push_tiny_int(-11);
    let (client, recorder) = recording_client(92067, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_nft_address_by_index(index))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv1_get_nft_content() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][14]["stack"])?;
    let mut arguments = TVMStack::default();
    let index = I512::from_i64(-11);
    let mut builder = TonCell::builder();
    builder.write_num(&0x1234u32, 32)?;
    let nftv3item_content = builder.build()?;
    arguments.push_tiny_int(-11);
    arguments.push_cell(nftv3item_content.clone());
    let (client, recorder) = recording_client(68445, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_nft_content(index, nftv3item_content.clone()))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_positionnftv1_get_pool_address() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][36]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(125122, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPositionNftV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_pool_address())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_positionnftv1_get_user_address() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][37]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(89625, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPositionNftV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_user_address())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_positionnftv1_get_position_info() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][38]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(68152, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPositionNftV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_position_info())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_positionnftv1_get_nft_data() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][39]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(102351, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPositionNftV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_nft_data())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_routerv1_get_router_state() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][31]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(103471, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoRouterV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_router_state())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_routerv1_get_pool_address() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
        let reply = stack(&rows["rows"][33]["stack"])?;
        let mut arguments = TVMStack::default();
        let jetton_wallet0 = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::new(0, [1; 32].into())
        };
        let jetton_wallet1 = TonAddress::new(1, [2; 32].into());
        arguments.push_cell_slice(jetton_wallet0.to_msg_address_int().to_cell()?);
        arguments.push_cell_slice(jetton_wallet1.to_msg_address_int().to_cell()?);
        let (client, recorder) = recording_client(125122, arguments, reply)?;
        let address = TonAddress::new(0, [42; 32].into());
        let contract = ToncoRouterV1Contract::new(&client, &address, None);
        let _result = tokio_test::block_on(contract.get_pool_address(&jetton_wallet0, &jetton_wallet1))?;
        recorder.ensure_consumed()?;
    }
    Ok(())
}

#[test]
fn test_routerv1_get_child_contracts() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][32]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(124601, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoRouterV1Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_child_contracts())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_routerv1_get_pool_initial_data() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
        let reply = stack(&rows["rows"][34]["stack"])?;
        let mut arguments = TVMStack::default();
        let jetton_wallet0 = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::new(0, [1; 32].into())
        };
        let jetton_wallet1 = TonAddress::new(1, [2; 32].into());
        arguments.push_cell_slice(jetton_wallet0.to_msg_address_int().to_cell()?);
        arguments.push_cell_slice(jetton_wallet1.to_msg_address_int().to_cell()?);
        let (client, recorder) = recording_client(111401, arguments, reply)?;
        let address = TonAddress::new(0, [42; 32].into());
        let contract = ToncoRouterV1Contract::new(&client, &address, None);
        let _result = tokio_test::block_on(contract.get_pool_initial_data(&jetton_wallet0, &jetton_wallet1))?;
        recorder.ensure_consumed()?;
    }
    Ok(())
}

#[test]
fn test_routerv1_get_pool_state_init() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
        let reply = stack(&rows["rows"][35]["stack"])?;
        let mut arguments = TVMStack::default();
        let jetton_wallet0 = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::new(0, [1; 32].into())
        };
        let jetton_wallet1 = TonAddress::new(1, [2; 32].into());
        arguments.push_cell_slice(jetton_wallet0.to_msg_address_int().to_cell()?);
        arguments.push_cell_slice(jetton_wallet1.to_msg_address_int().to_cell()?);
        let (client, recorder) = recording_client(128687, arguments, reply)?;
        let address = TonAddress::new(0, [42; 32].into());
        let contract = ToncoRouterV1Contract::new(&client, &address, None);
        let _result = tokio_test::block_on(contract.get_pool_state_init(&jetton_wallet0, &jetton_wallet1))?;
        recorder.ensure_consumed()?;
    }
    Ok(())
}

#[test]
fn test_accountv2_get_account_data() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][54]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(86329, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoAccountV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_account_data())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_factoryv2_get_pool_factory_data() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][55]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(119679, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoFactoryV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_pool_factory_data())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_factoryv2_get_order_address() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
        let reply = stack(&rows["rows"][56]["stack"])?;
        let mut arguments = TVMStack::default();
        let minter0 = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::new(0, [1; 32].into())
        };
        let minter1 = TonAddress::new(1, [2; 32].into());
        arguments.push_cell_slice(minter0.to_msg_address_int().to_cell()?);
        arguments.push_cell_slice(minter1.to_msg_address_int().to_cell()?);
        let (client, recorder) = recording_client(96490, arguments, reply)?;
        let address = TonAddress::new(0, [42; 32].into());
        let contract = ToncoFactoryV2Contract::new(&client, &address, None);
        let _result = tokio_test::block_on(contract.get_order_address(&minter0, &minter1))?;
        recorder.ensure_consumed()?;
    }
    Ok(())
}

#[test]
fn test_factoryv2_get_whitelist() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][57]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(69939, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoFactoryV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_whitelist())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_max_liquidity_per_tick() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][18]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(66347, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_max_liquidity_per_tick())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_child_contracts() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][16]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(124601, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_child_contracts())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_collection_data() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][19]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(102491, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_collection_data())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_nft_content() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][29]["stack"])?;
    let mut arguments = TVMStack::default();
    let index = I512::from_i64(-11);
    let mut builder = TonCell::builder();
    builder.write_num(&0x1234u32, 32)?;
    let content = builder.build()?;
    arguments.push_tiny_int(-11);
    arguments.push_cell(content.clone());
    let (client, recorder) = recording_client(68445, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_nft_content(index, content.clone()))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_pool_state_and_configuration() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][15]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(118400, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_pool_state_and_configuration())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_tick_infos_from() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][21]["stack"])?;
    let mut arguments = TVMStack::default();
    let start = I512::from_i64(-11);
    let count = I512::from_i64(12);
    let back = I512::from_i64(-13);
    let full = I512::from_i64(14);
    arguments.push_tiny_int(-11);
    arguments.push_tiny_int(12);
    arguments.push_tiny_int(-13);
    arguments.push_tiny_int(14);
    let (client, recorder) = recording_client(116046, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_tick_infos_from(start, count, back, full))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_all_tick_infos() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][20]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(96585, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_all_tick_infos())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_swap_estimate() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][25]["stack"])?;
    let mut arguments = TVMStack::default();
    let direction = I512::from_i64(-11);
    let amount = I512::from_i64(12);
    let limit = I512::from_i64(-13);
    arguments.push_tiny_int(-11);
    arguments.push_tiny_int(12);
    arguments.push_tiny_int(-13);
    let (client, recorder) = recording_client(85303, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_swap_estimate(direction, amount, limit))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_swap_estimate_gas() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][26]["stack"])?;
    let mut arguments = TVMStack::default();
    let direction = I512::from_i64(-11);
    let amount = I512::from_i64(12);
    let limit = I512::from_i64(-13);
    let minimum = I512::from_i64(14);
    let gas_limit = I512::from_i64(-15);
    arguments.push_tiny_int(-11);
    arguments.push_tiny_int(12);
    arguments.push_tiny_int(-13);
    arguments.push_tiny_int(14);
    arguments.push_tiny_int(-15);
    let (client, recorder) = recording_client(77100, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_swap_estimate_gas(direction, amount, limit, minimum, gas_limit))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_is_active() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][17]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(82247, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_is_active())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_pending_timelocks() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][30]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(130052, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_pending_timelocks())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_mint_estimate() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][22]["stack"])?;
    let mut arguments = TVMStack::default();
    let lower = I512::from_i64(-11);
    let upper = I512::from_i64(12);
    let liquidity = I512::from_i64(-13);
    arguments.push_tiny_int(-11);
    arguments.push_tiny_int(12);
    arguments.push_tiny_int(-13);
    let (client, recorder) = recording_client(108264, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_mint_estimate(lower, upper, liquidity))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_collected_fees() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][23]["stack"])?;
    let mut arguments = TVMStack::default();
    let lower = I512::from_i64(-11);
    let upper = I512::from_i64(12);
    let liquidity = I512::from_i64(-13);
    let checkpoint0 = I512::from_i64(14);
    let checkpoint1 = I512::from_i64(-15);
    arguments.push_tiny_int(-11);
    arguments.push_tiny_int(12);
    arguments.push_tiny_int(-13);
    arguments.push_tiny_int(14);
    arguments.push_tiny_int(-15);
    let (client, recorder) = recording_client(108894, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_collected_fees(lower, upper, liquidity, checkpoint0, checkpoint1))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_fee_growth_inside() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][24]["stack"])?;
    let mut arguments = TVMStack::default();
    let lower = I512::from_i64(-11);
    let upper = I512::from_i64(12);
    let current = I512::from_i64(-13);
    let global0 = I512::from_i64(14);
    let global1 = I512::from_i64(-15);
    arguments.push_tiny_int(-11);
    arguments.push_tiny_int(12);
    arguments.push_tiny_int(-13);
    arguments.push_tiny_int(14);
    arguments.push_tiny_int(-15);
    let (client, recorder) = recording_client(130735, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_fee_growth_inside(lower, upper, current, global0, global1))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_poolv2_get_user_account_address() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
        let reply = stack(&rows["rows"][27]["stack"])?;
        let mut arguments = TVMStack::default();
        let owner = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::new(0, [1; 32].into())
        };
        arguments.push_cell_slice(owner.to_msg_address_int().to_cell()?);
        let (client, recorder) = recording_client(67666, arguments, reply)?;
        let address = TonAddress::new(0, [42; 32].into());
        let contract = ToncoPoolV2Contract::new(&client, &address, None);
        let _result = tokio_test::block_on(contract.get_user_account_address(&owner))?;
        recorder.ensure_consumed()?;
    }
    Ok(())
}

#[test]
fn test_poolv2_get_nft_address_by_index() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][28]["stack"])?;
    let mut arguments = TVMStack::default();
    let index = I512::from_i64(-11);
    arguments.push_tiny_int(-11);
    let (client, recorder) = recording_client(92067, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPoolV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_nft_address_by_index(index))?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_positionnftv2_get_pool_address() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][50]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(125122, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPositionNftV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_pool_address())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_positionnftv2_get_user_address() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][51]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(89625, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPositionNftV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_user_address())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_positionnftv2_get_position_info() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][52]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(68152, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPositionNftV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_position_info())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_positionnftv2_get_nft_data() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][53]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(102351, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoPositionNftV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_nft_data())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_routerv2_get_child_contracts() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][45]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(124601, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoRouterV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_child_contracts())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_routerv2_get_pending_timelocks() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][49]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(130052, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoRouterV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_pending_timelocks())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_routerv2_get_router_state() -> anyhow::Result<()> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
    let reply = stack(&rows["rows"][44]["stack"])?;
    let arguments = TVMStack::default();

    let (client, recorder) = recording_client(103471, arguments, reply)?;
    let address = TonAddress::new(0, [42; 32].into());
    let contract = ToncoRouterV2Contract::new(&client, &address, None);
    let _result = tokio_test::block_on(contract.get_router_state())?;
    recorder.ensure_consumed()?;
    Ok(())
}

#[test]
fn test_routerv2_get_pool_address() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
        let reply = stack(&rows["rows"][46]["stack"])?;
        let mut arguments = TVMStack::default();
        let wallet0 = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::new(0, [1; 32].into())
        };
        let wallet1 = TonAddress::new(1, [2; 32].into());
        arguments.push_cell_slice(wallet0.to_msg_address_int().to_cell()?);
        arguments.push_cell_slice(wallet1.to_msg_address_int().to_cell()?);
        let (client, recorder) = recording_client(125122, arguments, reply)?;
        let address = TonAddress::new(0, [42; 32].into());
        let contract = ToncoRouterV2Contract::new(&client, &address, None);
        let _result = tokio_test::block_on(contract.get_pool_address(&wallet0, &wallet1))?;
        recorder.ensure_consumed()?;
    }
    Ok(())
}

#[test]
fn test_routerv2_get_pool_initial_data() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
        let reply = stack(&rows["rows"][47]["stack"])?;
        let mut arguments = TVMStack::default();
        let wallet0 = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::new(0, [1; 32].into())
        };
        let wallet1 = TonAddress::new(1, [2; 32].into());
        arguments.push_cell_slice(wallet0.to_msg_address_int().to_cell()?);
        arguments.push_cell_slice(wallet1.to_msg_address_int().to_cell()?);
        let (client, recorder) = recording_client(111401, arguments, reply)?;
        let address = TonAddress::new(0, [42; 32].into());
        let contract = ToncoRouterV2Contract::new(&client, &address, None);
        let _result = tokio_test::block_on(contract.get_pool_initial_data(&wallet0, &wallet1))?;
        recorder.ensure_consumed()?;
    }
    Ok(())
}

#[test]
fn test_routerv2_get_pool_state_init() -> anyhow::Result<()> {
    for zero_owner in [false, true] {
        let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/getter_structural.json"))?;
        let reply = stack(&rows["rows"][48]["stack"])?;
        let mut arguments = TVMStack::default();
        let wallet0 = if zero_owner {
            TonAddress::ZERO
        } else {
            TonAddress::new(0, [1; 32].into())
        };
        let wallet1 = TonAddress::new(1, [2; 32].into());
        arguments.push_cell_slice(wallet0.to_msg_address_int().to_cell()?);
        arguments.push_cell_slice(wallet1.to_msg_address_int().to_cell()?);
        let (client, recorder) = recording_client(128687, arguments, reply)?;
        let address = TonAddress::new(0, [42; 32].into());
        let contract = ToncoRouterV2Contract::new(&client, &address, None);
        let _result = tokio_test::block_on(contract.get_pool_state_init(&wallet0, &wallet1))?;
        recorder.ensure_consumed()?;
    }
    Ok(())
}

#[test]
fn test_wallet_owner_workchain_is_checked_before_provider_access() -> anyhow::Result<()> {
    let mut input = TVMStack::EMPTY;
    input.push_cell_slice(TonAddress::ZERO.to_msg_address_int().to_cell()?);
    let (client, _) = recording_client(103289, input, TVMStack::EMPTY)?;
    let minter = ToncoNativeMinterContract::new(&client, &TonAddress::ZERO, None);
    let invalid = TonAddress::new(128, TonAddress::ZERO.hash);
    let error = tokio_test::block_on(minter.get_wallet_address(&invalid))
        .err()
        .ok_or_else(|| anyhow::anyhow!("expected invalid workchain"))?;
    assert!(matches!(error, TonError::TLCoreError(TonCoreError::DataError { .. })));
    Ok(())
}
