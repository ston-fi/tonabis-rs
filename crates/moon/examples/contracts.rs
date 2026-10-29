//! Caller-owned getter, storage and local message construction; no network in main.
use ton::{
    contracts::{ContractClient, TonContract},
    errors::TonResult,
    ton_core::{
        traits::tlb::TLB,
        types::{TonAddress, tlb_core::TLBCoins},
    },
};
use tonabis_moon::contracts::{
    clmm::range::{MoonClmmRangeContract, storage::MoonClmmRangeStorage},
    mempad::msgs::MoonMempadBuyMsg,
};

pub async fn load_range(client: &ContractClient, address: &TonAddress) -> TonResult<MoonClmmRangeStorage> {
    let range = MoonClmmRangeContract::new(client, address, None);
    Ok(MoonClmmRangeStorage::from_cell(&range.get_range().await?)?)
}

fn main() -> anyhow::Result<()> {
    let request = MoonMempadBuyMsg {
        query_id: 42,
        min_out: TLBCoins::new(1_000),
    };
    let body = request.to_cell()?;
    println!("buy body hash: {}", body.hash()?);
    Ok(())
}
