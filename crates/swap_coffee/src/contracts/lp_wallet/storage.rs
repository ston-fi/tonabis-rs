//! Coffee storage. See tools/reference/swap_coffee/README.md.
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::{
    block_tlb::StateInit,
    ton_core::{
        TLB,
        cell::TonCell,
        errors::TonCoreResult,
        traits::tlb::TLB,
        types::tlb_core::{TLBCoins, TLBRef},
    },
};
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeLpWalletStorage {
    pub balance: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub jetton_master_address: TonAddress,
    pub jetton_wallet_code: TLBRef<TonCell>,
}

// Zero-balance LP wallet initialization over caller-owned code.
impl CoffeeLpWalletStorage {
    /// Uses the supplied code in StateInit and the wallet's referenced code field.
    pub fn initial_state_init(
        owner: TonAddress,
        master: TonAddress,
        wallet_code: &TonCell,
    ) -> TonCoreResult<StateInit> {
        let data = Self {
            balance: TLBCoins::new(0),
            owner_address: owner,
            jetton_master_address: master,
            jetton_wallet_code: TLBRef::new(wallet_code.clone()),
        };
        Ok(StateInit::new(wallet_code.clone(), data.to_cell()?))
    }
}
