//! Persistent TL-B records and local caller-code StateInit construction. No deployment is performed.
use ton::block_tlb::StateInit;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

use crate::contracts::v2::msgs::{ToncoV2BurnOrders, ToncoV2OrderBody};
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::{TLBCoins, TLBRef};

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoAccountV2Storage {
    #[tlb(adapter = "NullableAddress")]
    pub user: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub pool: Option<TonAddress>,
    pub stored0: TLBCoins,
    pub stored1: TLBCoins,
    pub flags: u32,
    pub positions: TLBRef<ToncoV2BurnOrders>,
    pub order: Option<TLBRef<ToncoV2OrderBody>>,
}

impl ToncoAccountV2Storage {
    /// Encode this exact revision's data with caller-supplied code. Does not deploy.
    pub fn state_init(&self, code: TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(code, ton::ton_core::traits::tlb::TLB::to_cell(self)?))
    }
    /// Hash the standard state-init; caller owns code identity and workchain policy.
    pub fn derive_address(&self, code: TonCell, workchain: i32) -> TonCoreResult<TonAddress> {
        self.state_init(code)?.derive_address(workchain)
    }
}
