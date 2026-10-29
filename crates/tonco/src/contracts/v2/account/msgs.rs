//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use crate::contracts::v2::msgs::ToncoPayToV2Msg;
use crate::contracts::v2::msgs::ToncoReforgeV2Msg;
use crate::contracts::v2::msgs::{ToncoSetOrderV2Msg, ToncoV2OrderBody};
use ton::ton_core::TLB;
use ton::ton_core::cell::CellBuilder;
use ton::ton_core::cell::CellParser;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::tlb_core::TLBCoins;

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x42a0fb43, bits_len = 32)]
pub struct ToncoAccountV2ResetGasMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoAccountV2IncomingMessage {
    PayTo(Box<ToncoPayToV2Msg>),
    AddLiquidity(Box<ToncoAccountV2AddLiquidityMsg>),
    ResetGas(Box<ToncoAccountV2ResetGasMsg>),
    SetOrder(Box<ToncoSetOrderV2Msg>),
    Unknown(TonCell),
}

/// Optional untagged new order following a funding callback's two amounts.
#[derive(Debug, Clone, PartialEq)]
pub struct ToncoAccountV2FundingOrderSuffix(pub Option<ToncoV2OrderBody>);

impl ton::ton_core::traits::tlb::TLB for ToncoAccountV2FundingOrderSuffix {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        let order = if p.data_bits_left()? == 0 {
            None
        } else {
            Some(ToncoV2OrderBody::read(p)?)
        };
        p.ensure_empty()?;
        Ok(Self(order))
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        if let Some(v) = &self.0 {
            v.write(b)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x3ebe5431, bits_len = 32)]
pub struct ToncoAccountV2AddLiquidityMsg {
    pub query_id: u64,
    pub amount0: TLBCoins,
    pub amount1: TLBCoins,
    pub order: ToncoAccountV2FundingOrderSuffix,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoAccountV2OutgoingMessage {
    Reforge(Box<ToncoReforgeV2Msg>),
    Unknown(TonCell),
}
