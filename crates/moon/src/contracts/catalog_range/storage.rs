//! Generated Moon wire records. See tools/reference/moon/README.md.
use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

/// Persistent state of one initialized CLMM price range.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCatalogClmmRangeStorage {
    #[tlb(adapter = "InternalAddress")]
    pub pool_address: TonAddress,
    pub range_number: i16,
    #[tlb(adapter = "NullableAddress")]
    pub jetton_wallet_x: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub jetton_wallet_y: Option<TonAddress>,
    pub bin_step: u16,
    pub lp_fee: u8,
    pub protocol_fee: u8,
    pub next_position_index: u64,
    pub range_state: TLBRef<MoonCatalogClmmRangeState>,
    pub position_collection_content: Option<TLBRef<TonCell>>,
    pub lp_account_code: TLBRef<TonCell>,
    pub position_nft_code: TLBRef<TonCell>,
}

/// Mutable values stored by reference in the range state.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCatalogClmmRangeState {
    pub current_bin: i64,
    pub current_sqrt_price_x128: U256,
    pub x_protocol_fees: TLBCoins,
    pub y_protocol_fees: TLBCoins,
}

/// Success or rejection routing selected by a swap caller.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCatalogClmmRangeCallback {
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    pub payload: Option<TLBRef<TonCell>>,
}

/// Swap fields stored inside the compatibility wrapper payload.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCatalogClmmRangeSwapPayload {
    pub swap_amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub excess_address: TonAddress,
    pub sqrt_price_limit_x128: U256,
    pub next_fulfill: Option<TLBRef<MoonCatalogClmmRangeCallback>>,
    pub next_reject: Option<TLBRef<MoonCatalogClmmRangeCallback>>,
}

/// Amounts assigned to one signed liquidity-bin index.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCatalogClmmRangeDepositBin {
    pub x_amount: TLBCoins,
    pub y_amount: TLBCoins,
}
