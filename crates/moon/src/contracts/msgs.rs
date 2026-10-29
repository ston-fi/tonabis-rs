//! Shared TEP-74 transfer and bounce envelopes.
use ton::ton_core::{
    TLB,
    cell::TonCell,
    types::{
        TonAddress,
        tlb_core::{
            TLBCoins, TLBEither, TLBRef,
            adapters::{InternalAddress, NullableAddress},
        },
    },
};

/// TEP-74 transfer with an explicit inline/reference forward-payload choice.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct MoonJettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}

/// Legacy bounce. The original body may be truncated; its remaining graph is retained.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xffffffff, bits_len = 32)]
pub struct MoonBounceMsg {
    pub original_prefix: TonCell,
}

/// Current position bounce: complete original request plus opaque diagnostics.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xfffffffe, bits_len = 32)]
pub struct MoonReferencedBounceMsg {
    pub original: TLBRef<TonCell>,
    pub diagnostics: TonCell,
}
