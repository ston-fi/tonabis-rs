//! CPMM deposit library 2cac3fdd... storage and deposit/refund commands.
use super::super::pool;
use ton::ton_core::{TLB, cell::TonCell, types::tlb_core::TLBCoins};

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xdc5ddba1, bits_len = 32)]
pub struct DedustV2CpmmDepositCreditMsg {
    pub query_id: u64,
    pub is_x: bool,
    pub amount: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x3db5f13a, bits_len = 32)]
pub struct DedustV2CpmmDepositRefundMsg {
    pub query_id: u64,
}

/// CreditMsg and refund dispatch retain unknown raw messages.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustV2DepositIncomingMsg {
    Credit(DedustV2CpmmDepositCreditMsg),
    Refund(DedustV2CpmmDepositRefundMsg),
    Unknown(TonCell),
}

/// Fixed requests emitted to the parent pool, or the partial-credit excess.
/// Known deposit configuration references retain their concrete schema.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustV2DepositOutgoingMsg {
    JoinLiquidity(Box<pool::msgs::DedustV2CpmmJoinLiquidityMsg>),
    RefundDeposit(Box<pool::msgs::DedustV2CpmmRefundDepositMsg>),
    Excess(pool::msgs::DedustV2CpmmExcessMsg),
    Unknown(TonCell),
}
