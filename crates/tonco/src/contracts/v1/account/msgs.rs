//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use crate::contracts::v1::pool::msgs::ToncoPoolV1MintMsg;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::adapters::ConstLen;

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x3ebe5431, bits_len = 32)]
pub struct ToncoAccountV1AddLiquidityMsg {
    pub query_id: u64,
    pub new_amount0: TLBCoins,
    pub new_amount1: TLBCoins,
    pub new_enough0: TLBCoins,
    pub new_enough1: TLBCoins,
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub tick_lower: i32,
    #[tlb(bits_len = 24)]
    pub tick_upper: i32,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x42a0fb43, bits_len = 32)]
pub struct ToncoAccountV1ResetGasMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0bf3f447, bits_len = 32)]
pub struct ToncoAccountV1RefundMeMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoAccountV1IncomingMessage {
    AccountAddLiquidity(Box<ToncoAccountV1AddLiquidityMsg>),
    AccountResetGas(Box<ToncoAccountV1ResetGasMsg>),
    AccountRefundMe(Box<ToncoAccountV1RefundMeMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoAccountV1OutgoingMessage {
    PoolMint(Box<ToncoPoolV1MintMsg>),
    Unknown(TonCell),
}
