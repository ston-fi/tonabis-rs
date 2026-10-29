#![deny(warnings)]
use ton::contracts::{ContractClient, TonContract};
use ton::errors::TonResult;
use ton::ton_core::{
    cell::TonCell,
    traits::tlb::TLB,
    types::{
        TonAddress,
        tlb_core::{TLBCoins, TLBRef},
    },
};
use tonabis_swap_coffee::contracts::{
    factory::{
        CoffeeFactoryContract,
        method_results::{CoffeeFactoryAddressWithHashResult, CoffeeFactoryGetCodeResult},
        msgs::CoffeeFactoryAsset,
    },
    lp_wallet::{msgs::CoffeeLpWalletExcessesMsg, storage::CoffeeLpWalletStorage},
    vault_native::CoffeeVaultNativeContract,
};
async fn getters(client: &ContractClient, address: &TonAddress, asset: &CoffeeFactoryAsset) -> TonResult<()> {
    let factory = CoffeeFactoryContract::new(client, address, None);
    let _: CoffeeFactoryGetCodeResult = factory.get_code().await?;
    let _: CoffeeFactoryAddressWithHashResult = factory.get_vault_address(asset).await?;
    let _: Option<TonAddress> = factory.get_admin_address().await?;
    let amm = 0i64.into();
    let settings = TonCell::empty();
    let _: CoffeeFactoryAddressWithHashResult =
        factory.get_pool_creator_address(address, asset, asset, &amm, settings).await?;
    let _: CoffeeFactoryAddressWithHashResult = factory
        .get_pool_creator_address_no_settings(address, asset, asset, &amm)
        .await?;
    let _: CoffeeFactoryAddressWithHashResult = factory
        .get_liquidity_depository_address(address, asset, asset, &amm, settings)
        .await?;
    let _: CoffeeFactoryAddressWithHashResult = factory
        .get_liquidity_depository_address_no_settings(address, asset, asset, &amm)
        .await?;

    let _: TonAddress = tonabis_swap_coffee::contracts::pool_constant_product::CoffeeConstantProductPoolContract::new(
        client, address, None,
    )
    .get_wallet_address(address)
    .await?;
    let _: TonAddress = tonabis_swap_coffee::contracts::pool_curve_fi_stable::CoffeeCurveFiStablePoolContract::new(
        client, address, None,
    )
    .get_wallet_address(address)
    .await?;
    let _: bool = CoffeeVaultNativeContract::new(client, address, None).is_active().await?;
    Ok(())
}
fn wrappers(client: &ContractClient, address: &TonAddress) {
    let _ = tonabis_swap_coffee::contracts::cross_dex::CoffeeCrossDexContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::cross_dex::CoffeeCrossDexDeploymentContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::cross_dex_bidask::CoffeeCrossDexBidaskContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::cross_dex_bidask::CoffeeCrossDexBidaskDeploymentContract::new(
        client, address, None,
    );
    let _ = tonabis_swap_coffee::contracts::cross_dex_legacy::CoffeeCrossDexLegacyContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::cross_dex_legacy::CoffeeCrossDexLegacyDeploymentContract::new(
        client, address, None,
    );
    let _ = tonabis_swap_coffee::contracts::factory::CoffeeFactoryContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::init::CoffeeInitContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::liquidity_depository::CoffeeLiquidityDepositoryContract::new(
        client, address, None,
    );
    let _ = tonabis_swap_coffee::contracts::lp_wallet::CoffeeLpWalletContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::mev_protector::CoffeeMevProtectorContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::mev_protector_legacy::CoffeeMevProtectorLegacyContract::new(
        client, address, None,
    );
    let _ = tonabis_swap_coffee::contracts::pool_constant_product::CoffeeConstantProductPoolContract::new(
        client, address, None,
    );
    let _ = tonabis_swap_coffee::contracts::pool_creator::CoffeePoolCreatorContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::pool_curve_fi_stable::CoffeeCurveFiStablePoolContract::new(
        client, address, None,
    );
    let _ = tonabis_swap_coffee::contracts::staking_item::CoffeeStakingItemContract::new(client, address, None);
    let _ =
        tonabis_swap_coffee::contracts::staking_item::CoffeeStakingItemDeploymentContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::staking_master::CoffeeStakingMasterContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::staking_master::CoffeeStakingMasterDeploymentContract::new(
        client, address, None,
    );
    let _ = tonabis_swap_coffee::contracts::staking_master::single_item::CoffeeStakingMasterSingleItemContract::new(
        client, address, None,
    );
    let _ = tonabis_swap_coffee::contracts::staking_master::single_item::CoffeeStakingMasterSingleItemDeploymentContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::staking_vault::CoffeeStakingVaultContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::vault_extra::CoffeeVaultExtraContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::vault_jetton::CoffeeVaultJettonContract::new(client, address, None);
    let _ =
        tonabis_swap_coffee::contracts::vault_jetton::CoffeeVaultJettonDeploymentContract::new(client, address, None);
    let _ = tonabis_swap_coffee::contracts::vault_native::CoffeeVaultNativeContract::new(client, address, None);
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message = CoffeeLpWalletExcessesMsg { query_id: 17 };
    let cell: TonCell = message.to_cell()?;
    assert_eq!(CoffeeLpWalletExcessesMsg::from_cell(&cell)?, message);
    let address = TonAddress::ZERO;
    let storage = CoffeeLpWalletStorage {
        balance: TLBCoins::new(5),
        owner_address: address,
        jetton_master_address: address,
        jetton_wallet_code: TLBRef::new(TonCell::empty().clone()),
    };
    assert_eq!(CoffeeLpWalletStorage::from_cell(&storage.to_cell()?)?, storage);
    let state: ton::block_tlb::StateInit =
        CoffeeLpWalletStorage::initial_state_init(address, address, TonCell::empty())?;
    let _: TonAddress = state.derive_address(0)?;
    // The same identity reference moves across vault APIs without rebuilding it.
    use tonabis_swap_coffee::contracts::vault_extra::storage::CoffeeVaultExtraStorage;
    use tonabis_swap_coffee::contracts::vault_jetton::storage::CoffeeVaultJettonInactiveStorage;
    use tonabis_swap_coffee::contracts::vault_native::storage::{
        CoffeeAsset, CoffeeNativeAsset, CoffeeVaultInitData, CoffeeVaultNativeStorage,
    };
    let identity = TLBRef::new(CoffeeVaultInitData {
        factory_address: TonAddress::ZERO,
        contract_type: 0,
        asset: CoffeeAsset::CoffeeNativeAsset(Box::new(CoffeeNativeAsset {})),
    });
    let native = CoffeeVaultNativeStorage {
        init_code: TLBRef::new(TonCell::empty().clone()),
        init_data: identity,
        factory_initialization_marker: None,
    };
    let extra = CoffeeVaultExtraStorage {
        init_code: native.init_code.clone(),
        init_data: native.init_data.clone(),
        factory_initialization_marker: None,
    };
    let jetton = CoffeeVaultJettonInactiveStorage {
        init_code: extra.init_code.clone(),
        init_data: extra.init_data.clone(),
        is_active: false,
    };
    assert_eq!(
        native.init_data.state_init(TonCell::empty())?,
        jetton.init_data.state_init(TonCell::empty())?
    );
    assert_eq!(
        native.init_data.derive_basechain_address(TonCell::empty())?,
        jetton.init_data.state_init(TonCell::empty())?.derive_address(0)?
    );
    assert_eq!(CoffeeVaultExtraStorage::from_cell(&extra.to_cell()?)?, extra);
    assert_eq!(CoffeeVaultJettonInactiveStorage::from_cell(&jetton.to_cell()?)?, jetton);
    let _ = getters;
    let _ = wrappers;
    Ok(())
}
