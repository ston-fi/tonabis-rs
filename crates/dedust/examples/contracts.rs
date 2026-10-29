//! Caller-owned client, scalar getters and canonical storage paths.
use tonabis_dedust::contracts::classic::asset::DedustClassicAsset;
use tonabis_dedust::contracts::classic::{
    common::method_results::DedustClassicVersionedCode,
    factory_v2::{DedustFactoryV2Contract, storage::DedustFactoryV2Storage},
    pool_v2::DedustPoolV2Contract,
};

use tonabis_dedust::contracts::custom_wallet::v2::DedustX1000WalletV2Contract;

use ton::{
    contracts::{ContractClient, TonContract},
    errors::TonResult,
    ton_core::{
        cell::TonCell,
        errors::TonCoreResult,
        traits::tlb::TLB,
        types::{Coins, TonAddress},
    },
};

pub async fn read_factory(
    client: &ContractClient,
    address: &TonAddress,
) -> TonResult<(u16, DedustClassicVersionedCode, TonAddress)> {
    let factory = DedustFactoryV2Contract::new(client, address, None);
    let version = factory.get_version().await?;
    let code = factory.get_vault_code(0).await?;
    let operator = factory.get_operator_address(10).await?;
    Ok((version, code, operator))
}

pub fn read_storage(data: &TonCell) -> TonCoreResult<DedustFactoryV2Storage> {
    DedustFactoryV2Storage::from_cell(data)
}

pub fn main() {}

pub async fn read_pool_address(
    client: &ContractClient,
    factory_address: &TonAddress,
    is_stable: bool,
    asset0: DedustClassicAsset,
    asset1: DedustClassicAsset,
) -> TonResult<TonAddress> {
    let factory = DedustFactoryV2Contract::new(client, factory_address, None);
    factory.get_pool_address(is_stable, asset0, asset1).await
}

/// Amounts use Coins; call inner() for integer base units at a consumer boundary.
pub async fn read_pool_reserves(client: &ContractClient, address: &TonAddress) -> TonResult<(Coins, Coins)> {
    let pool = DedustPoolV2Contract::new(client, address, None);
    let reserves = pool.get_reserves().await?;
    Ok((reserves.reserve0, reserves.reserve1))
}

/// Native counter/amount types from the selected wallet snapshot.
pub async fn read_wallet_status(client: &ContractClient, address: &TonAddress) -> TonResult<(u32, u8, Coins)> {
    let wallet = DedustX1000WalletV2Contract::new(client, address, None);
    Ok((
        wallet.seqno().await?,
        wallet.storage_version().await?,
        wallet.get_balance().await?,
    ))
}
