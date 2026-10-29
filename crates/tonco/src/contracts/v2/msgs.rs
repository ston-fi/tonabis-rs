//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

use fastnum::{I128, I256, U256};
use ton::ton_core::{
    TLB,
    cell::TonCell,
    types::tlb_core::{TLBCoins, TLBRef, adapters::ConstLen},
};
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoV2PayoutPayloads {
    pub forward0: TLBCoins,
    pub payload0: Option<TLBRef<TonCell>>,
    pub forward1: TLBCoins,
    pub payload1: Option<TLBRef<TonCell>>,
    #[tlb(adapter = "NullableAddress")]
    pub excess: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoV2Amounts {
    #[tlb(adapter = "NullableAddress")]
    pub wallet0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub wallet1: Option<TonAddress>,
    pub amount0: TLBCoins,
    pub amount1: TLBCoins,
    pub payloads: Option<TLBRef<ToncoV2PayoutPayloads>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoV2SwapLog {
    pub liquidity: u128,
    #[tlb(bits_len = 160)]
    pub price: U256,
    #[tlb(bits_len = 24)]
    pub tick: i32,
    pub fee_growth0: I256,
    pub fee_growth1: I256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xa1daa96d, bits_len = 32)]
pub struct ToncoPayToV2Msg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub receiver0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub receiver1: Option<TonAddress>,
    pub exit_code: u32,
    pub seqno: u64,
    pub amounts: Option<TLBRef<ToncoV2Amounts>>,
    pub swap_log: Option<TLBRef<ToncoV2SwapLog>>,
    pub positions: Option<TLBRef<ToncoV2BurnOrders>>,
}

/// Signed Q128 fee-growth checkpoints for token0/token1, not token fee amounts.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoV2Fees {
    pub token0: I256,
    pub token1: I256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoV2BurnOrder {
    pub index: u64,
    #[tlb(bits_len = 4)]
    pub subindex: u8,
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub lower: i32,
    #[tlb(bits_len = 24)]
    pub upper: i32,
    pub growth0: I256,
    pub growth1: I256,
    pub new_fees: Option<TLBRef<ToncoV2Fees>>,
    pub liquidity_to_burn: I128,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoV2MintOrder {
    pub operation: u32,
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub lower: i32,
    #[tlb(bits_len = 24)]
    pub upper: i32,
    #[tlb(adapter = "NullableAddress")]
    pub recipient: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoV2BurnOrders {
    pub slot0: Option<TLBRef<ToncoV2BurnOrder>>,
    pub slot1: Option<TLBRef<ToncoV2BurnOrder>>,
    pub slot2: Option<TLBRef<ToncoV2BurnOrder>>,
    pub slot3: Option<TLBRef<ToncoV2BurnOrder>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoV2MintOrders {
    pub slot0: Option<TLBRef<ToncoV2MintOrder>>,
    pub slot1: Option<TLBRef<ToncoV2MintOrder>>,
    pub slot2: Option<TLBRef<ToncoV2MintOrder>>,
    pub slot3: Option<TLBRef<ToncoV2MintOrder>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoV2ReforgePayloads {
    pub amount0: TLBCoins,
    pub payload0: TLBRef<TonCell>,
    pub amount1: TLBCoins,
    pub payload1: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoV2Actions {
    #[tlb(adapter = "NullableAddress")]
    pub target0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub target1: Option<TonAddress>,
    pub payloads: TLBRef<ToncoV2ReforgePayloads>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0eb9e061, bits_len = 32)]
pub struct ToncoReforgeV2Msg {
    pub query_id: u64,
    #[tlb(bits_len = 4)]
    pub source: u8,
    pub source_id: u64,
    pub amount0: TLBCoins,
    pub amount1: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub provider: Option<TonAddress>,
    pub positions: Option<TLBRef<ToncoV2BurnOrders>>,
    #[tlb(bits_len = 4)]
    pub passthrough: u8,
    pub orders: Option<TLBRef<ToncoV2MintOrders>>,
    pub action_target: u32,
    pub actions: Option<TLBRef<ToncoV2Actions>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoV2OrderBody {
    pub enough0: TLBCoins,
    pub enough1: TLBCoins,
    pub positions_needed: u64,
    #[tlb(bits_len = 4)]
    pub passthrough: u8,
    pub action: u32,
    pub orders: ToncoV2MintOrders,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x61d86ffe, bits_len = 32)]
pub struct ToncoSetOrderV2Msg {
    pub query_id: u64,
    pub order: ToncoV2OrderBody,
}
