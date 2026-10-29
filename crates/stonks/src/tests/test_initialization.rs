//! Independent code-free data and StateInit hashes, including literal-zero contexts.
use crate::contracts::{
    custom_wallet::StonksCustomWalletContract, dedust_wallet::StonksDedustWalletContract,
    ston_minter::storage::StonksVirtualMinterStorage, ston_wallet::StonksStonWalletContract,
};
use ton::ton_core::{cell::TonCell, traits::tlb::TLB, types::TonAddress};
#[test]
fn test_independent_initial_data_state_init_and_addresses() -> anyhow::Result<()> {
    let owner = Some(TonAddress::new(0, [0x11; 32].into()));
    let master = Some(TonAddress::new(0, [0x22; 32].into()));
    let empty = TonCell::empty();
    let states = [
        StonksVirtualMinterStorage::initial_state(empty.clone(), owner, empty.clone(), empty.clone())?,
        StonksStonWalletContract::initial_state(empty.clone(), owner, master)?,
        StonksDedustWalletContract::initial_state(empty.clone(), owner, master)?,
        StonksCustomWalletContract::initial_state(empty.clone(), owner, master)?,
    ];
    let fixture: serde_json::Value = serde_json::from_str(include_str!("fixtures/initialization.json"))?;
    for (state, row) in states
        .iter()
        .zip(fixture["vectors"].as_array().ok_or_else(|| anyhow::anyhow!("vectors"))?)
    {
        let data = state.data.as_deref().ok_or_else(|| anyhow::anyhow!("data"))?;
        assert_eq!(
            hex::encode(data.hash()?),
            row["data_hash"].as_str().ok_or_else(|| anyhow::anyhow!("data hash"))?
        );
        assert_eq!(
            hex::encode(state.cell_hash()?),
            row["state_init_hash"].as_str().ok_or_else(|| anyhow::anyhow!("state hash"))?
        );
        let address = state.derive_address(-1)?;
        assert_eq!(address.workchain, -1);
        assert_eq!(
            hex::encode(address.hash),
            row["address_hash"].as_str().ok_or_else(|| anyhow::anyhow!("address hash"))?
        );
    }
    Ok(())
}
