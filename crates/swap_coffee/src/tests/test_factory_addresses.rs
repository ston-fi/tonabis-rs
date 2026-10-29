use crate::contracts::factory::msgs::CoffeeFactoryAsset;
use crate::contracts::factory::msgs::CoffeeFactoryExtraAsset;
use crate::contracts::factory::msgs::CoffeeFactoryJettonAsset;
use crate::contracts::factory::msgs::CoffeeFactoryNativeAsset;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryAmm;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryAmmConstantProduct;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryAmmCurveFiStable;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryAsset;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryAssetExtra;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryAssetNative;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryInitData;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryInitPoolParams;
use crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolProof;
use crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolProofBody;
use crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolSourcePoolProofBody;
use crate::contracts::pool_constant_product::storage::CoffeePoolAsset;
use crate::contracts::pool_constant_product::storage::CoffeePoolExtraAsset;
use crate::contracts::pool_constant_product::storage::CoffeePoolNativeAsset;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorAmm;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorAmmConstantProduct;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorAmmCurveFiStable;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorAsset;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorAssetExtra;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorAssetNative;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorInitData;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorInitPoolParams;
use fastnum::U256;

use std::str::FromStr;

use ton::ton_core::cell::TonCell;
use ton::ton_core::cell::TonHash;
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
#[test]
fn test_factory_canonical_asset_order_and_all_derived_child_identities() -> anyhow::Result<()> {
    let data: serde_json::Value = serde_json::from_str(include_str!("fixtures/factory-addresses.json"))?;
    let factory = TonAddress::from_str(data["factory"].as_str().ok_or_else(|| anyhow::anyhow!("missing factory"))?)?;
    let mut hash = [0u8; 32];
    hash[31] = 123;
    let owner = TonAddress::new(0, TonHash::from_slice(&hash)?);
    let native = || CoffeeFactoryAsset::NativeAsset(Box::new(CoffeeFactoryNativeAsset {}));
    let jetton = |chain, hash: u32| {
        CoffeeFactoryAsset::JettonAsset(Box::new(CoffeeFactoryJettonAsset {
            chain,
            hash: U256::from(hash),
        }))
    };
    let extra = |id| CoffeeFactoryAsset::ExtraAsset(Box::new(CoffeeFactoryExtraAsset { id }));
    let pairs = [
        [native(), extra(1)],
        [jetton(0, 999), jetton(255, 1)],
        [jetton(0, 999), jetton(0, 1)],
        [extra(u32::MAX), extra(0)],
        [jetton(255, 0), extra(0)],
    ];
    for (id, pair) in pairs.into_iter().enumerate() {
        let original = pair.clone();
        let ([first, second], reversed) = CoffeeFactoryAsset::canonical_pair(pair)?;
        assert_eq!(reversed, id == 2 || id == 3);
        let (reverse_order, reverse_flag) =
            CoffeeFactoryAsset::canonical_pair([original[1].clone(), original[0].clone()])?;
        assert_eq!(reverse_order, [first.clone(), second.clone()]);
        assert_ne!(reverse_flag, reversed);
        let asset1 = CoffeePoolAsset::from_cell(&first.to_cell()?)?;
        let asset2 = CoffeePoolAsset::from_cell(&second.to_cell()?)?;
        let proof = CoffeeConstantProductPoolProof {
            factory_address: factory,
            body: CoffeeConstantProductPoolProofBody::CoffeePoolPoolProofBody(Box::new(
                CoffeeConstantProductPoolSourcePoolProofBody {
                    asset1,
                    asset2,
                    amm: 0,
                    amm_settings: None,
                },
            )),
        };
        assert_eq!(
            proof.derive_basechain_address(TonCell::empty())?,
            TonAddress::from_str(
                data["pairs"][id]["pool"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("missing pool"))?
            )?
        );
        let params = CoffeePoolCreatorInitPoolParams {
            asset1: CoffeePoolCreatorAsset::from_cell(&first.to_cell()?)?,
            asset2: CoffeePoolCreatorAsset::from_cell(&second.to_cell()?)?,
            amm: CoffeePoolCreatorAmm::AmmConstantProduct(Box::new(CoffeePoolCreatorAmmConstantProduct {})),
            amm_settings: None,
        };
        let proof = CoffeePoolCreatorInitData {
            factory_address: factory,
            contract_type: 3,
            owner_address: owner,
            pool_params: TLBRef::new(params),
        };
        assert_eq!(
            proof.derive_basechain_address(TonCell::empty())?,
            TonAddress::from_str(
                data["pairs"][id]["creator"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("missing creator"))?
            )?
        );
        let params = CoffeeLiquidityDepositoryInitPoolParams {
            asset1: CoffeeLiquidityDepositoryAsset::from_cell(&first.to_cell()?)?,
            asset2: CoffeeLiquidityDepositoryAsset::from_cell(&second.to_cell()?)?,
            amm: CoffeeLiquidityDepositoryAmm::AmmConstantProduct(Box::new(
                CoffeeLiquidityDepositoryAmmConstantProduct {},
            )),
            amm_settings: None,
        };
        let proof = CoffeeLiquidityDepositoryInitData {
            factory_address: factory,
            contract_type: 2,
            owner_address: owner,
            pool_params: TLBRef::new(params),
        };
        assert_eq!(
            proof.derive_basechain_address(TonCell::empty())?,
            TonAddress::from_str(
                data["pairs"][id]["depository"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("missing depository"))?
            )?
        );
    }
    assert!(CoffeeFactoryAsset::canonical_pair([native(), native()]).is_err());
    Ok(())
}
#[test]
fn test_factory_explicit_stable_settings_child_identity_matches_getters() -> anyhow::Result<()> {
    use crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolAmmSettings;

    use ton::ton_core::types::tlb_core::TLBCoins;
    let data: serde_json::Value = serde_json::from_str(include_str!("fixtures/factory-settings-addresses.json"))?;
    let address = |field: &str| -> anyhow::Result<TonAddress> {
        Ok(TonAddress::from_str(
            data[field].as_str().ok_or_else(|| anyhow::anyhow!("missing {field}"))?,
        )?)
    };
    let factory = address("factory")?;
    let mut hash = [0u8; 32];
    hash[31] = 123;
    let owner: TonAddress = TonAddress::new(0, TonHash::from_slice(&hash)?);
    let settings = CoffeeCurveFiStablePoolAmmSettings {
        amplification: 100,
        asset1_weight: TLBCoins::new(3),
        asset2_weight: TLBCoins::new(7),
    }
    .to_cell()?;
    assert_eq!(
        format!("{}", settings.hash()?).to_lowercase(),
        data["settings_hash"].as_str().ok_or_else(|| anyhow::anyhow!("missing hash"))?
    );
    let proof = CoffeeConstantProductPoolProof {
        factory_address: factory,
        body: CoffeeConstantProductPoolProofBody::CoffeePoolPoolProofBody(Box::new(
            CoffeeConstantProductPoolSourcePoolProofBody {
                asset1: CoffeePoolAsset::CoffeePoolNativeAsset(Box::new(CoffeePoolNativeAsset {})),
                asset2: CoffeePoolAsset::CoffeePoolExtraAsset(Box::new(CoffeePoolExtraAsset { id: 17 })),
                amm: 1,
                amm_settings: Some(TLBRef::new(settings.clone())),
            },
        )),
    };
    assert_eq!(proof.derive_basechain_address(TonCell::empty())?, address("pool")?);
    let params = CoffeePoolCreatorInitPoolParams {
        asset1: CoffeePoolCreatorAsset::AssetNative(Box::new(CoffeePoolCreatorAssetNative {})),
        asset2: CoffeePoolCreatorAsset::AssetExtra(Box::new(CoffeePoolCreatorAssetExtra { id: 17 })),
        amm: CoffeePoolCreatorAmm::AmmCurveFiStable(Box::new(CoffeePoolCreatorAmmCurveFiStable {})),
        amm_settings: Some(TLBRef::new(settings.clone())),
    };
    let initial = CoffeePoolCreatorInitData {
        factory_address: factory,
        contract_type: 3,
        owner_address: owner,
        pool_params: TLBRef::new(params),
    };
    assert_eq!(initial.derive_basechain_address(TonCell::empty())?, address("creator")?);
    let params = CoffeeLiquidityDepositoryInitPoolParams {
        asset1: CoffeeLiquidityDepositoryAsset::AssetNative(Box::new(CoffeeLiquidityDepositoryAssetNative {})),
        asset2: CoffeeLiquidityDepositoryAsset::AssetExtra(Box::new(CoffeeLiquidityDepositoryAssetExtra { id: 17 })),
        amm: CoffeeLiquidityDepositoryAmm::AmmCurveFiStable(Box::new(CoffeeLiquidityDepositoryAmmCurveFiStable {})),
        amm_settings: Some(TLBRef::new(settings)),
    };
    let initial = CoffeeLiquidityDepositoryInitData {
        factory_address: factory,
        contract_type: 2,
        owner_address: owner,
        pool_params: TLBRef::new(params),
    };
    assert_eq!(initial.derive_basechain_address(TonCell::empty())?, address("depository")?);
    Ok(())
}
