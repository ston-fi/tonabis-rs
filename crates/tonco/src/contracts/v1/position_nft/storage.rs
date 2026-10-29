//! Persistent TL-B records and local caller-code StateInit construction. No deployment is performed.
use ton::block_tlb::StateInit;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::ConstLen;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPositionNftV1Storage {
    pub index: u64,
    #[tlb(adapter = "NullableAddress")]
    pub pool_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub user_address: Option<TonAddress>,
    pub content: TLBRef<TonCell>,
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub tick_lower: i32,
    #[tlb(bits_len = 24)]
    pub tick_upper: i32,
    pub fee_growth_inside: TLBRef<ToncoPositionNftV1FeeGrowth>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPositionNftV1FeeGrowth {
    pub fee_growth_inside0_last_x128: U256,
    pub fee_growth_inside1_last_x128: U256,
}

impl ToncoPositionNftV1Storage {
    /// Encode this exact revision's data with caller-supplied code. Does not deploy.
    pub fn state_init(&self, code: TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(code, ton::ton_core::traits::tlb::TLB::to_cell(self)?))
    }
    /// Hash the standard state-init; caller owns code identity and workchain policy.
    pub fn derive_address(&self, code: TonCell, workchain: i32) -> TonCoreResult<TonAddress> {
        self.state_init(code)?.derive_address(workchain)
    }
}
