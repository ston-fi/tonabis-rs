#![deny(warnings)]
use tonabis::tonabis_core::arithmetic::mul_div_floor;

#[cfg(any(feature = "all", feature = "dedust"))]
fn dedust() -> Result<(), Box<dyn std::error::Error>> {
    use ton::ton_core::traits::tlb::TLB;
    use tonabis::tonabis_dedust::contracts::classic::asset::{DedustClassicAsset, DedustClassicNativeAsset};
    let asset = DedustClassicAsset::Native(DedustClassicNativeAsset {});
    let cell: ton::ton_core::cell::TonCell = asset.to_cell()?;
    assert_eq!(DedustClassicAsset::from_cell(&cell)?, asset);
    Ok(())
}

#[cfg(any(feature = "all", feature = "moon"))]
fn moon() -> Result<(), Box<dyn std::error::Error>> {
    use ton::ton_core::{traits::tlb::TLB, types::tlb_core::TLBCoins};
    use tonabis::tonabis_moon::contracts::mempad::msgs::MoonMempadBuyMsg;
    let request = MoonMempadBuyMsg {
        query_id: 42,
        min_out: TLBCoins::new(1_000),
    };
    let cell: ton::ton_core::cell::TonCell = request.to_cell()?;
    assert_eq!(MoonMempadBuyMsg::from_cell(&cell)?, request);
    async fn address_getters(
        client: &ton::contracts::ContractClient,
        address: &ton::ton_core::types::TonAddress,
    ) -> ton::errors::TonResult<()> {
        use ton::contracts::TonContract;
        let catalog_range =
            tonabis::tonabis_moon::contracts::catalog_range::MoonCatalogClmmRangeContract::new(client, address, None);
        let _: ton::ton_core::types::TonAddress = catalog_range.get_lp_account(address).await?;
        let legacy_pool =
            tonabis::tonabis_moon::contracts::legacy_pool::MoonLegacyPoolContract::new(client, address, None);
        let _: ton::ton_core::types::TonAddress = legacy_pool.get_lp_account(address).await?;
        let _: ton::ton_core::types::TonAddress = legacy_pool.get_wallet_address(address).await?;
        let clmm_range =
            tonabis::tonabis_moon::contracts::clmm::range::MoonClmmRangeContract::new(client, address, None);
        let _: ton::ton_core::types::TonAddress = clmm_range.get_account_address(address).await?;
        let mempad_curve =
            tonabis::tonabis_moon::contracts::mempad::curve::MoonMempadCurveContract::new(client, address, None);
        let _: ton::ton_core::types::TonAddress = mempad_curve.get_wallet_address(address).await?;
        let order_factory =
            tonabis::tonabis_moon::contracts::order_factory::MoonOrderFactoryContract::new(client, address, None);
        let _: ton::ton_core::types::TonAddress = order_factory.get_pipe(Some(*address)).await?;
        Ok(())
    }
    let _ = address_getters;
    Ok(())
}

#[cfg(any(feature = "all", feature = "stonks"))]
fn stonks() -> Result<(), Box<dyn std::error::Error>> {
    use ton::{
        contracts::{ContractClient, TonContract},
        errors::TonResult,
        ton_core::{
            cell::TonCell,
            traits::tlb::TLB,
            types::{TonAddress, tlb_core::TLBCoins},
        },
    };
    use tonabis::tonabis_stonks::contracts::{
        dedust_minter_v12::{StonksDedustMinterV12Contract, method_results::StonksDedustMinterV12GetBondingDataResult},
        ston_minter::{msgs::StonksStonMinterBuyFromPresaleMsg, storage::StonksVirtualMinterStorage},
    };
    async fn read(
        client: &ContractClient,
        address: &TonAddress,
    ) -> TonResult<(u32, StonksDedustMinterV12GetBondingDataResult)> {
        let minter = StonksDedustMinterV12Contract::new(client, address, None);
        Ok((minter.get_version().await?, minter.get_bonding_data().await?))
    }
    let request = StonksStonMinterBuyFromPresaleMsg {
        query_id: 42,
        ton_amount: TLBCoins::new(1_000_000_000),
        min_jettons_out: TLBCoins::new(1_000),
    };
    assert_eq!(StonksStonMinterBuyFromPresaleMsg::from_cell(&request.to_cell()?)?, request);
    let state: ton::block_tlb::StateInit = StonksVirtualMinterStorage::initial_state(
        TonCell::empty().clone(),
        Some(TonAddress::ZERO),
        TonCell::empty().clone(),
        TonCell::empty().clone(),
    )?;
    let data = state.data.as_deref().ok_or("missing minter storage")?;
    assert!(StonksVirtualMinterStorage::from_cell(data)?.bonding_state.is_none());
    let _: TonAddress = state.derive_address(0)?;
    let _ = read;
    async fn address_getters(
        client: &ton::contracts::ContractClient,
        address: &ton::ton_core::types::TonAddress,
    ) -> ton::errors::TonResult<()> {
        use ton::contracts::TonContract;
        let ston_minter =
            tonabis::tonabis_stonks::contracts::ston_minter::StonksStonMinterContract::new(client, address, None);
        let _: Option<ton::ton_core::types::TonAddress> = ston_minter.get_wallet_address(Some(*address)).await?;
        let dedust_minter_v7 = tonabis::tonabis_stonks::contracts::dedust_minter_v7::StonksDedustMinterV7Contract::new(
            client, address, None,
        );
        let _: Option<ton::ton_core::types::TonAddress> = dedust_minter_v7.get_wallet_address(Some(*address)).await?;
        let dedust_minter_v12 =
            tonabis::tonabis_stonks::contracts::dedust_minter_v12::StonksDedustMinterV12Contract::new(
                client, address, None,
            );
        let _: Option<ton::ton_core::types::TonAddress> = dedust_minter_v12.get_wallet_address(Some(*address)).await?;
        Ok(())
    }
    let _ = address_getters;
    Ok(())
}

#[cfg(any(feature = "all", feature = "swap_coffee"))]
fn coffee() -> Result<(), Box<dyn std::error::Error>> {
    use ton::{
        contracts::{ContractClient, TonContract},
        errors::TonResult,
        ton_core::{
            cell::TonCell,
            traits::tlb::TLB,
            types::{TonAddress, tlb_core::TLBRef},
        },
    };
    use tonabis::tonabis_swap_coffee::contracts::{
        factory::{CoffeeFactoryContract, method_results::CoffeeFactoryGetCodeResult},
        lp_wallet::msgs::CoffeeLpWalletExcessesMsg,
        vault_extra::storage::CoffeeVaultExtraStorage,
        vault_jetton::storage::CoffeeVaultJettonInactiveStorage,
        vault_native::{
            CoffeeVaultNativeContract,
            storage::{CoffeeAsset, CoffeeNativeAsset, CoffeeVaultInitData},
        },
    };
    async fn read(client: &ContractClient, address: &TonAddress) -> TonResult<(CoffeeFactoryGetCodeResult, bool)> {
        Ok((
            CoffeeFactoryContract::new(client, address, None).get_code().await?,
            CoffeeVaultNativeContract::new(client, address, None).is_active().await?,
        ))
    }
    let request = CoffeeLpWalletExcessesMsg { query_id: 42 };
    assert_eq!(CoffeeLpWalletExcessesMsg::from_cell(&request.to_cell()?)?, request);
    let identity = TLBRef::new(CoffeeVaultInitData {
        factory_address: TonAddress::ZERO,
        contract_type: 0,
        asset: CoffeeAsset::CoffeeNativeAsset(Box::new(CoffeeNativeAsset {})),
    });
    let extra = CoffeeVaultExtraStorage {
        init_code: TLBRef::new(TonCell::empty().clone()),
        init_data: identity.clone(),
        factory_initialization_marker: None,
    };
    let jetton = CoffeeVaultJettonInactiveStorage {
        init_code: extra.init_code.clone(),
        init_data: identity,
        is_active: false,
    };
    assert_eq!(CoffeeVaultExtraStorage::from_cell(&extra.to_cell()?)?, extra);
    assert_eq!(CoffeeVaultJettonInactiveStorage::from_cell(&jetton.to_cell()?)?, jetton);
    assert_eq!(
        extra.init_data.state_init(TonCell::empty())?,
        jetton.init_data.state_init(TonCell::empty())?
    );
    let _ = read;
    async fn address_getters(
        client: &ton::contracts::ContractClient,
        address: &ton::ton_core::types::TonAddress,
    ) -> ton::errors::TonResult<()> {
        use ton::contracts::TonContract;
        let pool_constant_product =
            tonabis::tonabis_swap_coffee::contracts::pool_constant_product::CoffeeConstantProductPoolContract::new(
                client, address, None,
            );
        let _: ton::ton_core::types::TonAddress = pool_constant_product.get_wallet_address(address).await?;
        let pool_curve_fi_stable =
            tonabis::tonabis_swap_coffee::contracts::pool_curve_fi_stable::CoffeeCurveFiStablePoolContract::new(
                client, address, None,
            );
        let _: ton::ton_core::types::TonAddress = pool_curve_fi_stable.get_wallet_address(address).await?;
        Ok(())
    }
    let _ = address_getters;
    Ok(())
}

#[cfg(any(feature = "all", feature = "tonco"))]
fn tonco() -> Result<(), Box<dyn std::error::Error>> {
    use ton::{
        contracts::{ContractClient, TonContract},
        errors::TonResult,
        ton_core::{traits::tlb::TLB, types::TonAddress},
    };
    use tonabis::tonabis_tonco::contracts::v2::pool::{
        ToncoPoolV2Contract, method_results::ToncoPoolV2GetPoolStateAndConfigurationResult,
        msgs::ToncoPoolV2ResetGasMsg,
    };
    async fn read(
        client: &ContractClient,
        address: &TonAddress,
    ) -> TonResult<ToncoPoolV2GetPoolStateAndConfigurationResult> {
        ToncoPoolV2Contract::new(client, address, None)
            .get_pool_state_and_configuration()
            .await
    }
    let request = ToncoPoolV2ResetGasMsg { query_id: 42 };
    let cell: ton::ton_core::cell::TonCell = request.to_cell()?;
    assert_eq!(ToncoPoolV2ResetGasMsg::from_cell(&cell)?, request);
    let _ = read;
    async fn address_getters(
        client: &ton::contracts::ContractClient,
        address: &ton::ton_core::types::TonAddress,
    ) -> ton::errors::TonResult<()> {
        use ton::contracts::TonContract;
        let v1_factory =
            tonabis::tonabis_tonco::contracts::v1::factory::ToncoFactoryV1Contract::new(client, address, None);
        let _: ton::ton_core::types::TonAddress = v1_factory.get_order_address(address, address).await?;
        let v1_pool = tonabis::tonabis_tonco::contracts::v1::pool::ToncoPoolV1Contract::new(client, address, None);
        let _: ton::ton_core::types::TonAddress = v1_pool.get_user_account_address(address).await?;
        let v1_router =
            tonabis::tonabis_tonco::contracts::v1::router::ToncoRouterV1Contract::new(client, address, None);
        let _: ton::ton_core::types::TonAddress = v1_router.get_pool_address(address, address).await?;
        let _: ton::ton_core::cell::TonCell = v1_router.get_pool_initial_data(address, address).await?;
        let _: ton::ton_core::cell::TonCell = v1_router.get_pool_state_init(address, address).await?;
        let v2_factory =
            tonabis::tonabis_tonco::contracts::v2::factory::ToncoFactoryV2Contract::new(client, address, None);
        let _: ton::ton_core::types::TonAddress = v2_factory.get_order_address(address, address).await?;
        let v2_pool = tonabis::tonabis_tonco::contracts::v2::pool::ToncoPoolV2Contract::new(client, address, None);
        let _: ton::ton_core::types::TonAddress = v2_pool.get_user_account_address(address).await?;
        let v2_router =
            tonabis::tonabis_tonco::contracts::v2::router::ToncoRouterV2Contract::new(client, address, None);
        let _: ton::ton_core::types::TonAddress = v2_router.get_pool_address(address, address).await?;
        let _: ton::ton_core::cell::TonCell = v2_router.get_pool_initial_data(address, address).await?;
        let _: ton::ton_core::cell::TonCell = v2_router.get_pool_state_init(address, address).await?;
        let native_minter =
            tonabis::tonabis_tonco::contracts::native::minter::ToncoNativeMinterContract::new(client, address, None);
        let _: ton::ton_core::types::TonAddress = native_minter.get_wallet_address(address).await?;
        Ok(())
    }
    let _ = address_getters;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(mul_div_floor(1_000u64.into(), 3u64.into(), 10u64.into())?, 300u64.into());
    #[cfg(any(feature = "all", feature = "dedust"))]
    dedust()?;
    #[cfg(any(feature = "all", feature = "moon"))]
    moon()?;
    #[cfg(any(feature = "all", feature = "stonks"))]
    stonks()?;
    #[cfg(any(feature = "all", feature = "swap_coffee"))]
    coffee()?;
    #[cfg(any(feature = "all", feature = "tonco"))]
    tonco()?;
    Ok(())
}
