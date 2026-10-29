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

fn main() -> TonCoreResult<()> {
    let body = ToncoPoolV2ResetGasMsg { query_id: 7 }.to_cell()?;
    let parsed = ToncoPoolV2ResetGasMsg::from_cell(&body)?;
    assert_eq!(parsed.query_id, 7);
    Ok(())
}
