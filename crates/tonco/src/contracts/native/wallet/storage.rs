//! Persistent TL-B records and local caller-code StateInit construction. No deployment is performed.
use ton::block_tlb::StateInit;
use ton::ton_core::errors::TonCoreResult;

use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::{TonAddress, tlb_core::adapters::InternalAddress};

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoNativeWalletStorage {
    pub balance: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub minter: TonAddress,
}

impl ToncoNativeWalletStorage {
    /// Encode this exact revision's data with caller-supplied code. Does not deploy.
    pub fn state_init(&self, code: TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(code, ton::ton_core::traits::tlb::TLB::to_cell(self)?))
    }
    /// Hash the standard state-init; caller owns code identity and workchain policy.
    pub fn derive_address(&self, code: TonCell, workchain: i32) -> TonCoreResult<TonAddress> {
        self.state_init(code)?.derive_address(workchain)
    }
}
