use crate::contracts::lp_wallet::storage::CoffeeLpWalletStorage;
use crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolProof;
use crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolProofBody;
use crate::contracts::pool_constant_product::storage::CoffeePoolAsset;
use crate::contracts::pool_constant_product::storage::CoffeePoolExtraAsset;
use crate::contracts::pool_constant_product::storage::CoffeePoolNativeAsset;
use crate::contracts::pool_constant_product::storage::CoffeePoolVaultProofBody;
use std::str::FromStr;
use ton::ton_core::cell::{TonCell, TonHash};
use ton::ton_core::types::TonAddress;

#[derive(serde::Deserialize)]
struct Vectors {
    vectors: Vec<Vector>,
}
#[derive(serde::Deserialize)]
struct Vector {
    kind: String,
    id: u8,
    addresses: Vec<String>,
}
#[test]
fn test_independent_getter_and_proof_address_vectors() -> anyhow::Result<()> {
    let data: Vectors = serde_json::from_str(include_str!("fixtures/identity-vectors.json"))?;
    for vector in data.vectors {
        if vector.kind == "LPADDR" {
            let owner = TonAddress::from_str(&vector.addresses[0])?;
            let master = TonAddress::from_str(&vector.addresses[1])?;
            let state = CoffeeLpWalletStorage::initial_state_init(owner, master, TonCell::empty())?;
            assert_eq!(state.derive_address(0)?, TonAddress::from_str(&vector.addresses[2])?);
        } else {
            let mut hash = [0u8; 32];
            hash[31] = vector.id;
            let factory = TonAddress::new(0, TonHash::from_slice(&hash)?);
            let asset = if vector.id == 0 {
                CoffeePoolAsset::CoffeePoolNativeAsset(Box::new(CoffeePoolNativeAsset {}))
            } else {
                CoffeePoolAsset::CoffeePoolExtraAsset(Box::new(CoffeePoolExtraAsset { id: 1 }))
            };
            let proof = CoffeeConstantProductPoolProof {
                factory_address: factory,
                body: CoffeeConstantProductPoolProofBody::CoffeePoolVaultProofBody(Box::new(
                    CoffeePoolVaultProofBody { asset },
                )),
            };
            assert_eq!(
                proof.derive_basechain_address(TonCell::empty())?,
                TonAddress::from_str(&vector.addresses[1])?
            );
        }
    }
    Ok(())
}
