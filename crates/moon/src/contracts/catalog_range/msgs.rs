//! Generated Moon wire records. See tools/reference/moon/README.md.
use crate::contracts::catalog_range::storage::MoonCatalogClmmRangeCallback;
use crate::contracts::catalog_range::storage::MoonCatalogClmmRangeDepositBin;
use crate::contracts::catalog_range::storage::MoonCatalogClmmRangeSwapPayload;
use fastnum::U256;
use std::collections::HashMap;
use ton::tlb_adapters::DictKeyAdapterInt;
use ton::tlb_adapters::DictValAdapterTLB;
use ton::tlb_adapters::TLBHashMapE;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

/// Direct native GRAM swap accepted by a Moon.cx CLMM range.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3511755023, bits_len = 32)]
pub struct MoonCatalogClmmRangeSwapMsg {
    pub query_id: u64,
    pub swap_amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub excess_address: TonAddress,
    pub sqrt_price_limit_x128: U256,
    pub next_fulfill: Option<TLBRef<MoonCatalogClmmRangeCallback>>,
    pub next_reject: Option<TLBRef<MoonCatalogClmmRangeCallback>>,
}

/// Compatibility wrapper that carries a referenced native swap payload.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2635343427, bits_len = 32)]
pub struct MoonCatalogClmmRangeWrappedSwapMsg {
    pub query_id: u64,
    pub swap_amount: TLBCoins,
    pub swap: TLBRef<MoonCatalogClmmRangeSwapPayload>,
}

/// Direct native GRAM liquidity deposit into selected bins.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3963622466, bits_len = 32)]
pub struct MoonCatalogClmmRangeDepositLiquidityMsg {
    pub query_id: u64,
    pub deposit_amount: TLBCoins,
    #[tlb(bits_len = 1)]
    pub is_single: i8,
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterInt<64, i64>, DictValAdapterTLB<MoonCatalogClmmRangeDepositBin>>::new(64)"
    )]
    pub bins: HashMap<i64, MoonCatalogClmmRangeDepositBin>,
}

/// Standard Jetton transfer notification accepted by the range.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1935855772, bits_len = 32)]
pub struct MoonCatalogClmmRangeJettonTransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub sender: Option<TonAddress>,
    pub forward_payload: TonCell,
}

/// Swap payload carried by a Jetton transfer notification.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3511755023, bits_len = 32)]
pub struct MoonCatalogClmmRangeSwapForwardPayloadMsg {
    #[tlb(adapter = "InternalAddress")]
    pub excess_address: TonAddress,
    pub sqrt_price_limit_x128: U256,
    pub next_fulfill: Option<TLBRef<MoonCatalogClmmRangeCallback>>,
    pub next_reject: Option<TLBRef<MoonCatalogClmmRangeCallback>>,
}

/// Liquidity-deposit payload carried by a Jetton transfer notification.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3963622466, bits_len = 32)]
pub struct MoonCatalogClmmRangeDepositLiquidityForwardPayloadMsg {
    #[tlb(bits_len = 1)]
    pub is_single: i8,
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterInt<64, i64>, DictValAdapterTLB<MoonCatalogClmmRangeDepositBin>>::new(64)"
    )]
    pub bins: HashMap<i64, MoonCatalogClmmRangeDepositBin>,
}

/// Native swap result returned to a caller-selected callback address.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2635343427, bits_len = 32)]
pub struct MoonCatalogClmmRangeNativeSwapResultMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}

/// Swap result carried as the forward payload of a Jetton transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2635343427, bits_len = 32)]
pub struct MoonCatalogClmmRangeJettonSwapResultPayloadMsg {
    pub payload: Option<TLBRef<TonCell>>,
}

/// Native swap rejection or refund returned to a callback address.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 4281522841, bits_len = 32)]
pub struct MoonCatalogClmmRangeNativeSwapRefundMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}

/// Standard Jetton transfer emitted for swap output or Jetton refunds.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 260734629, bits_len = 32)]
pub struct MoonCatalogClmmRangeJettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub response_destination: TonAddress,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum MoonCatalogClmmRangeIncomingMsg {
    MoonClmmRangeSwapMsg(Box<MoonCatalogClmmRangeSwapMsg>),
    MoonClmmRangeWrappedSwapMsg(Box<MoonCatalogClmmRangeWrappedSwapMsg>),
    MoonClmmRangeDepositLiquidityMsg(Box<MoonCatalogClmmRangeDepositLiquidityMsg>),
    MoonClmmRangeJettonTransferNotificationMsg(Box<MoonCatalogClmmRangeJettonTransferNotificationMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum MoonCatalogClmmRangeOutgoingMsg {
    MoonClmmRangeNativeSwapResultMsg(Box<MoonCatalogClmmRangeNativeSwapResultMsg>),
    MoonClmmRangeNativeSwapRefundMsg(Box<MoonCatalogClmmRangeNativeSwapRefundMsg>),
    MoonClmmRangeJettonTransferMsg(Box<MoonCatalogClmmRangeJettonTransferMsg>),
    Unknown(TonCell),
}
