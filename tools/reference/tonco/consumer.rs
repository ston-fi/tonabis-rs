//! Local encoding and caller-owned client workflows; no network I/O in main.
use fastnum::I512;
use ton::{
    block_tlb::StateInit,
    contracts::{ContractClient, TonContract},
    errors::TonResult,
    ton_core::{cell::TonCell, errors::TonCoreResult, traits::tlb::TLB, types::TonAddress},
};
use tonabis_tonco::contracts::{
    native::wallet::storage::ToncoNativeWalletStorage,
    v2::pool::{
        ToncoPoolV2Contract, method_results::ToncoPoolV2GetPoolStateAndConfigurationResult,
        msgs::ToncoPoolV2ResetGasMsg,
    },
};

pub async fn inspect(
    client: &ContractClient,
    address: &TonAddress,
) -> TonResult<(I512, ToncoPoolV2GetPoolStateAndConfigurationResult)> {
    let pool = ToncoPoolV2Contract::new(client, address, None);
    Ok((
        pool.get_max_liquidity_per_tick().await?,
        pool.get_pool_state_and_configuration().await?,
    ))
}

pub fn wallet_init(storage: &ToncoNativeWalletStorage, code: TonCell) -> TonCoreResult<StateInit> {
    storage.state_init(code)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    decode_results()?;
    let claim = tonabis_tonco::contracts::farming::msgs::ToncoFarmingClaimMsg {
        query_id: 5,
        proof: ton::ton_core::types::tlb_core::TLBRef::new(TonCell::empty().clone()),
        legacy_index: tonabis_tonco::contracts::farming::msgs::ToncoFarmingLegacyIndex(None),
    };
    let _: TonCell = claim.to_cell()?;
    let body = ToncoPoolV2ResetGasMsg { query_id: 7 }.to_cell()?;
    let parsed = ToncoPoolV2ResetGasMsg::from_cell(&body)?;
    assert_eq!(parsed.query_id, 7);
    Ok(())
}

#[allow(dead_code)]
fn direct_type_identity(
    storage: &ToncoNativeWalletStorage,
    result: &tonabis_tonco::contracts::native::minter::method_results::ToncoNativeMinterGetJettonDataResult,
) {
    let _: &TonAddress = &storage.owner;
    let _: &TonCell = &result.wallet_code;
    let _: &Option<TonAddress> = &result.admin;
}

#[allow(dead_code)]
fn all_wrappers(client: &ContractClient, address: &TonAddress) {
    let _ = tonabis_tonco::contracts::farming::distributor::ToncoDistributorContract::new(client, address, None);
    let _ =
        tonabis_tonco::contracts::farming::user_distributor::ToncoUserDistributorContract::new(client, address, None);
    let _ = tonabis_tonco::contracts::v1::factory::ToncoFactoryV1Contract::new(client, address, None);
    let _ = tonabis_tonco::contracts::v1::order::ToncoOrderV1Contract::new(client, address, None);
    let _ = tonabis_tonco::contracts::v1::account::ToncoAccountV1Contract::new(client, address, None);
    let _ = tonabis_tonco::contracts::v1::position_nft::ToncoPositionNftV1Contract::new(client, address, None);
    let _ = tonabis_tonco::contracts::v1::router::ToncoRouterV1Contract::new(client, address, None);
    let _ = tonabis_tonco::contracts::v1::pool::ToncoPoolV1Contract::new(client, address, None);
    let _ = tonabis_tonco::contracts::native::wallet::ToncoNativeWalletV1Contract::new(client, address, None);
    let _ = tonabis_tonco::contracts::native::wallet::ToncoNativeWalletV2Contract::new(client, address, None);
    let _ = tonabis_tonco::contracts::native::minter::ToncoNativeMinterContract::new(client, address, None);
    let _ = tonabis_tonco::contracts::v2::factory::ToncoFactoryV2Contract::new(client, address, None);
    let _ = tonabis_tonco::contracts::v2::order::ToncoOrderV2Contract::new(client, address, None);
    let _ = tonabis_tonco::contracts::v2::account::ToncoAccountV2Contract::new(client, address, None);
    let _ = tonabis_tonco::contracts::v2::position_nft::ToncoPositionNftV2Contract::new(client, address, None);
    let _ = tonabis_tonco::contracts::v2::router::ToncoRouterV2Contract::new(client, address, None);
    let _ = tonabis_tonco::contracts::v2::pool::ToncoPoolV2Contract::new(client, address, None);
}

fn decode_results() -> TonResult<()> {
    use ton::block_tlb::{FromTVMStack, TVMStack};
    let mut stack = TVMStack::default();
    stack.push_tiny_int(1);
    stack.push_tiny_int(6);
    stack.push_cell_slice(TonCell::empty().clone());
    let version =
        tonabis_tonco::contracts::native::minter::method_results::ToncoNativeMinterGetVersionResult::from_stack(
            &mut stack,
        )?;
    assert_eq!(version.major, I512::from_i64(1));
    let mut scalar = TVMStack::default();
    scalar.push_tiny_int(123);
    assert_eq!(I512::from_stack(&mut scalar)?, I512::from_i64(123));
    Ok(())
}
