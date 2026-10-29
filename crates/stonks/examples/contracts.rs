//! Caller-owned getters and code-free local construction; main performs no network I/O.
use ton::{
    contracts::{ContractClient, TonContract},
    errors::TonResult,
    ton_core::{
        cell::TonCell,
        traits::tlb::TLB,
        types::{TonAddress, tlb_core::TLBCoins},
    },
};
use tonabis_stonks::contracts::{
    dedust_minter_v12::{StonksDedustMinterV12Contract, method_results::StonksDedustMinterV12GetBondingDataResult},
    ston_minter::{msgs::StonksStonMinterBuyFromPresaleMsg, storage::StonksVirtualMinterStorage},
    ston_wallet::StonksStonWalletContract,
};

pub async fn read_minter(
    client: &ContractClient,
    address: &TonAddress,
) -> TonResult<(u32, Option<TonAddress>, StonksDedustMinterV12GetBondingDataResult)> {
    let minter = StonksDedustMinterV12Contract::new(client, address, None);
    Ok((
        minter.get_version().await?,
        minter.get_wallet_address(Some(*address)).await?,
        minter.get_bonding_data().await?,
    ))
}

fn main() -> anyhow::Result<()> {
    let body = StonksStonMinterBuyFromPresaleMsg {
        query_id: 42,
        ton_amount: TLBCoins::new(1_000_000_000),
        min_jettons_out: TLBCoins::new(1_000),
    }
    .to_cell()?;
    let decoded = StonksStonMinterBuyFromPresaleMsg::from_cell(&body)?;
    assert_eq!(decoded.query_id, 42);
    let zero = Some(TonAddress::ZERO);
    // Empty code is a structural example, not a deployable program.
    let state = StonksStonWalletContract::initial_state(TonCell::empty().clone(), zero, zero)?;
    let minter = StonksVirtualMinterStorage::initial_state(
        TonCell::empty().clone(),
        zero,
        TonCell::empty().clone(),
        TonCell::empty().clone(),
    )?;
    let data = minter.data.as_deref().ok_or_else(|| anyhow::anyhow!("missing minter data"))?;
    let storage = StonksVirtualMinterStorage::from_cell(data)?;
    assert!(storage.bonding_state.is_none());
    println!("structural wallet address: {}", state.derive_address(0)?);
    Ok(())
}
