//! Persistent TL-B records and local caller-code StateInit construction. No deployment is performed.
use ton::block_tlb::StateInit;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoUserDistributorStorage {
    pub claimed: bool,
    #[tlb(adapter = "NullableAddress")]
    pub distributor: Option<TonAddress>,
    pub proof_hash: U256,
    pub index: U256,
    pub farming_id: u32,
    pub position_id: u128,
}

impl ToncoUserDistributorStorage {
    /// Encode this exact revision's data with caller-supplied code. Does not deploy.
    pub fn state_init(&self, code: TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(code, ton::ton_core::traits::tlb::TLB::to_cell(self)?))
    }
    /// Hash the standard state-init; caller owns code identity and workchain policy.
    pub fn derive_address(&self, code: TonCell, workchain: i32) -> TonCoreResult<TonAddress> {
        self.state_init(code)?.derive_address(workchain)
    }
}
