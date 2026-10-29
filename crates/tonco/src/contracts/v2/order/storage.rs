//! Persistent TL-B records and local caller-code StateInit construction. No deployment is performed.
use ton::block_tlb::StateInit;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::ConstLen;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoOrderV2Pair {
    #[tlb(adapter = "NullableAddress")]
    pub token0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub token1: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoOrderV2Storage {
    #[tlb(adapter = "NullableAddress")]
    pub factory: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub router: Option<TonAddress>,
    #[tlb(bits_len = 160)]
    pub price: U256,
    pub settings: u16,
    pub minters: TLBRef<ToncoOrderV2Pair>,
    pub wallets: TLBRef<ToncoOrderV2Pair>,
}

impl ToncoOrderV2Storage {
    /// Code and complete data are supplied explicitly; nothing is sent.
    pub fn state_init(&self, code: TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(code, self.to_cell()?))
    }
}
