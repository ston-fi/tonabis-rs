use crate::contracts::factory::msgs::CoffeeFactoryAsset;
use crate::contracts::factory::msgs::CoffeeFactoryExtraAsset;
use crate::contracts::factory::msgs::CoffeeFactoryNativeAsset;
use crate::contracts::vault_proof::CoffeeVaultContractProof;
use crate::contracts::vault_proof::CoffeeVaultPoolIdentity;
use crate::contracts::vault_proof::CoffeeVaultSenderIdentity;

use std::str::FromStr;

use ton::block_tlb::StateInit;
use ton::ton_core::cell::TonCell;
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::MsgAddressInt;
#[test]
fn test_typed_vault_sender_proof_matches_factory_derived_pool_address() -> anyhow::Result<()> {
    let data: serde_json::Value = serde_json::from_str(include_str!("fixtures/factory-addresses.json"))?;
    let address = TonAddress::from_str(data["factory"].as_str().ok_or_else(|| anyhow::anyhow!("missing factory"))?)?;
    let proof = CoffeeVaultContractProof {
        factory: address,
        identity: CoffeeVaultSenderIdentity::Pool(CoffeeVaultPoolIdentity {
            asset1: CoffeeFactoryAsset::NativeAsset(Box::new(CoffeeFactoryNativeAsset {})),
            asset2: CoffeeFactoryAsset::ExtraAsset(Box::new(CoffeeFactoryExtraAsset { id: 1 })),
            amm: 0,
            amm_settings: None,
        }),
    };
    let cell = proof.to_cell()?;
    let expected = TonAddress::from_str(
        data["pairs"][0]["pool"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing pool"))?,
    )?;
    assert_eq!(
        StateInit::new(TonCell::empty().clone(), cell.clone()).derive_address(0)?,
        expected
    );
    assert_eq!(CoffeeVaultContractProof::from_cell(&cell)?, proof);
    // AMM widths remain checked; ordinary referenced identity suffixes are tolerated.
    let CoffeeVaultSenderIdentity::Pool(mut pool) = proof.identity else {
        anyhow::bail!("wrong identity")
    };
    pool.amm = 8;
    assert!(pool.to_cell().is_err());
    let mut pair = TonCell::builder();
    pool.amm = 0;
    pair.write_cell(&pool.to_cell()?)?;
    pair.write_bit(true)?;
    let owner = MsgAddressInt::from_cell(&address.to_cell()?)?;
    let mut bad = TonCell::builder();
    owner.write(&mut bad)?;
    bad.write_num(&2u8, 2)?;
    owner.write(&mut bad)?;
    bad.write_ref(pair.build()?)?;
    assert!(CoffeeVaultContractProof::from_cell(&bad.build()?).is_ok());
    // Vault identity form has a direct tagged asset, not a nested pool reference.
    let vault = CoffeeVaultContractProof {
        factory: address,
        identity: CoffeeVaultSenderIdentity::Vault {
            asset: CoffeeFactoryAsset::NativeAsset(Box::new(CoffeeFactoryNativeAsset {})),
        },
    };
    assert_eq!(CoffeeVaultContractProof::from_cell(&vault.to_cell()?)?, vault);
    Ok(())
}
