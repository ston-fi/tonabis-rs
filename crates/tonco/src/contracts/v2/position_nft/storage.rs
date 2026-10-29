//! Persistent TL-B records and local caller-code StateInit construction. No deployment is performed.
use crate::contracts::v2::msgs::ToncoV2Fees;
use fastnum::I256;
use ton::block_tlb::StateInit;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPositionNftV2PositionState {
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub lower: i32,
    #[tlb(bits_len = 24)]
    pub upper: i32,
    pub growth0: I256,
    pub growth1: I256,
    pub new_fees: Option<TLBRef<ToncoV2Fees>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPositionNftV2Storage {
    pub index: u64,
    #[tlb(adapter = "NullableAddress")]
    pub pool: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub user: Option<TonAddress>,
    pub content: TLBRef<TonCell>,
    pub position: TLBRef<ToncoPositionNftV2PositionState>,
}

impl ToncoPositionNftV2Storage {
    /// Encode this exact revision's data with caller-supplied code. Does not deploy.
    pub fn state_init(&self, code: TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(code, ton::ton_core::traits::tlb::TLB::to_cell(self)?))
    }
    /// Hash the standard state-init; caller owns code identity and workchain policy.
    pub fn derive_address(&self, code: TonCell, workchain: i32) -> TonCoreResult<TonAddress> {
        self.state_init(code)?.derive_address(workchain)
    }
}
