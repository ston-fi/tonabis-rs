//! CPMM position storage reconciled from captured library dd82f24d...,
//! kit 0.0.4 getter records and the pool's deterministic child-state construction.
use super::super::pool;
use crate::contracts::classic::lp_wallet::msgs::DedustClassicLpWalletExcessMsg;
use ton::ton_core::types::TonAddress;
use ton::ton_core::{
    TLB,
    cell::TonCell,
    errors::TonCoreError,
    traits::tlb::TLB,
    types::tlb_core::{TLBCoins, TLBRef},
};
use tonabis_core::adapters::maybe_address::MaybeAddress;

/// Complete persistent position state. Locked liquidity is retained separately.
use super::storage::{DedustV2CpmmPositionFees, DedustV2CpmmPositionRewards};
/// Recognized position commands retain complete typed bodies; unknowns retain raw cells.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustV2PositionIncomingMsg {
    CreditLiquidity(Box<pool::msgs::DedustV2CpmmCreditLiquidityMsg>),
    ClaimAvailableFees(Box<pool::msgs::DedustV2CpmmClaimAvailableFeesMsg>),
    WithdrawLiquidity(Box<pool::msgs::DedustV2CpmmWithdrawLiquidityMsg>),
    ClaimReward(Box<pool::msgs::DedustV2CpmmClaimPositionRewardMsg>),
    ProvidePositionState(DedustV2CpmmPositionProvidePositionStateMsg),
    Unknown(TonCell),
}

/// Captured request for position addresses, fee state and reward state.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x9e0c2428, bits_len = 32)]
pub struct DedustV2CpmmPositionProvidePositionStateMsg {
    pub query_id: u64,
    pub include_pool_address: bool,
    pub include_owner_address: bool,
    pub include_fees: bool,
    pub include_rewards: bool,
}

/// Bounces return only the first 256 data bits of the original message body.
/// Parse only recognized prefixes. Source LDU32/NIP skips the marker without
/// validation; the bounced envelope flag is checked separately. Parsing does
/// not authenticate a sender or establish that an original payout occurred.
#[derive(Debug, Clone, PartialEq)]
pub enum DedustV2PositionBounceMsg {
    PositionFees {
        prefix: u32,
        query_id: u64,
        x_fees: TLBCoins,
        y_fees: TLBCoins,
        tail: TonCell,
    },
    ExitLiquidity {
        prefix: u32,
        query_id: u64,
        liquidity: TLBCoins,
        x_fees: TLBCoins,
        y_fees: TLBCoins,
        tail: TonCell,
    },
    Reward {
        prefix: u32,
        query_id: u64,
        amount: TLBCoins,
        reward_index: u8,
        tail: TonCell,
    },
    Unknown(TonCell),
}
impl DedustV2PositionBounceMsg {
    pub fn from_cell(cell: &TonCell) -> Result<Self, TonCoreError> {
        let mut p = cell.parser();
        let prefix = p.read_num::<u32>(32)?;
        let tag = p.read_num::<u32>(32)?;
        Ok(match tag {
            0x29ff1bcf => Self::PositionFees {
                prefix,
                query_id: p.read_num(64)?,
                x_fees: TLBCoins::read(&mut p)?,
                y_fees: TLBCoins::read(&mut p)?,
                tail: p.read_remaining()?,
            },
            0xf6f6a3aa => Self::ExitLiquidity {
                prefix,
                query_id: p.read_num(64)?,
                liquidity: TLBCoins::read(&mut p)?,
                x_fees: TLBCoins::read(&mut p)?,
                y_fees: TLBCoins::read(&mut p)?,
                tail: p.read_remaining()?,
            },
            0x9e17fbbe => Self::Reward {
                prefix,
                query_id: p.read_num(64)?,
                amount: TLBCoins::read(&mut p)?,
                reward_index: p.read_num(2)?,
                tail: p.read_remaining()?,
            },
            _ => Self::Unknown(cell.clone()),
        })
    }
}

/// Position state response layout emitted by opcode 0x9e0c2428.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x52e659c0, bits_len = 32)]
pub struct DedustV2CpmmPositionStateReplyMsg {
    pub query_id: u64,
    pub total_liquidity: TLBCoins,
    pub locked_liquidity: TLBCoins,
    #[tlb(adapter = "MaybeAddress")]
    pub pool: Option<TonAddress>,
    #[tlb(adapter = "MaybeAddress")]
    pub owner: Option<TonAddress>,
    pub fees: Option<TLBRef<DedustV2CpmmPositionFees>>,
    pub rewards: Option<DedustV2CpmmPositionRewards>,
}
/// Optional notification sent to the owner after a credited deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x63554160, bits_len = 32)]
pub struct DedustV2CpmmPositionDepositNotificationMsg {
    pub query_id: u64,
    pub liquidity: TLBCoins,
    pub locked_liquidity: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}

/// Complete position responses and pool callbacks. A malformed recognized body
/// remains an error; preservation of an unknown body does not confer authority.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustV2PositionOutgoingMsg {
    DepositNotification(DedustV2CpmmPositionDepositNotificationMsg),
    StateReply(Box<DedustV2CpmmPositionStateReplyMsg>),
    PayoutFees(Box<pool::msgs::DedustV2CpmmPayoutPositionFeesMsg>),
    ExitLiquidity(Box<pool::msgs::DedustV2CpmmExitLiquidityMsg>),
    PayoutReward(Box<pool::msgs::DedustV2CpmmPayoutRewardMsg>),
    Excess(DedustClassicLpWalletExcessMsg),
    Unknown(TonCell),
}
