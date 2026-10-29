//! Offline message codec and caller-owned-client example; no network requests.
use std::error::Error;
use ton::contracts::{ContractClient, TonContract};
use ton::errors::TonResult;
use ton::ton_core::{cell::TonCell, traits::tlb::TLB, types::TonAddress};
use tonabis_swap_coffee::contracts::{
    factory::{CoffeeFactoryContract, method_results::CoffeeFactoryGetCodeResult},
    lp_wallet::msgs::CoffeeLpWalletExcessesMsg,
};

/// Fetch code configuration once; the caller selects client, address and context.
async fn read_code(client: &ContractClient, address: &TonAddress) -> TonResult<CoffeeFactoryGetCodeResult> {
    CoffeeFactoryContract::new(client, address, None).get_code().await
}
fn main() -> Result<(), Box<dyn Error>> {
    let message = CoffeeLpWalletExcessesMsg { query_id: 7 };
    let encoded: TonCell = message.to_cell()?;
    assert_eq!(CoffeeLpWalletExcessesMsg::from_cell(&encoded)?, message);
    let _getter = read_code;
    Ok(())
}
