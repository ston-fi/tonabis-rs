//! Moon wire records. See tools/reference/moon/README.md.

use crate::contracts::booster::storage::MoonBoosterBoostData;
use ton::ton_core::{
    TLB,
    cell::{CellBuilder, CellParser, TonCell},
    errors::{TonCoreError, TonCoreResult},
    traits::tlb::{TLB, TLBPrefix},
    types::{
        TonAddress,
        tlb_core::{TLBCoins, TLBRef, adapters::NullableAddress},
    },
};
/// Extra inline fields supplied when configuration requests Jetton funding.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonBoosterFundingTail {
    #[tlb(adapter = "NullableAddress")]
    pub funding_address: Option<TonAddress>,
    pub amount: TLBCoins,
}

/// Standard Jetton transfer notification received during Jetton funding.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1935855772, bits_len = 32)]
pub struct MoonBoosterJettonTransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub sender: Option<TonAddress>,
    pub forward_payload: TonCell,
}

/// Signals that an expired boost can return its remaining balance.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2850284919, bits_len = 32)]
pub struct MoonBoosterCloseExpiredMsg {
    pub query_id: u64,
}

/// Requests protocol-side Jetton funding for a configured boost.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3009695497, bits_len = 32)]
pub struct MoonBoosterFundingRequestMsg {
    pub query_id: u64,
    pub booster_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub pool: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub funding_address: Option<TonAddress>,
    pub amount: TLBCoins,
}

/// Standard Jetton transfer emitted for a boost or a rejected notification refund.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 260734629, bits_len = 32)]
pub struct MoonBoosterJettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub destination: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TonCell,
}

/// Direct GRAM boost request sent to the MoonCX pool.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2527729030, bits_len = 32)]
pub struct MoonBoosterPoolBoostMsg {
    pub query_id: u64,
}

/// Payload forwarded through a Jetton wallet when boosting a pool with Jettons.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2527729030, bits_len = 32)]
pub struct MoonBoosterJettonBoostPayloadMsg {
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
}

/// Standard excess-value response emitted when a boost closes.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3576854235, bits_len = 32)]
pub struct MoonBoosterExcessesMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonBoosterExternalBoostTriggerMsg {
    pub ignored_body: TonCell,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum MoonBoosterIncomingMsg {
    MoonBoosterConfigureMsg(Box<MoonBoosterConfigureMsg>),
    MoonBoosterJettonTransferNotificationMsg(Box<MoonBoosterJettonTransferNotificationMsg>),
    MoonBoosterCloseExpiredMsg(Box<MoonBoosterCloseExpiredMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum MoonBoosterOutgoingMsg {
    MoonBoosterFundingRequestMsg(Box<MoonBoosterFundingRequestMsg>),
    MoonBoosterJettonTransferMsg(Box<MoonBoosterJettonTransferMsg>),
    MoonBoosterPoolBoostMsg(Box<MoonBoosterPoolBoostMsg>),
    MoonBoosterExcessesMsg(Box<MoonBoosterExcessesMsg>),
    Unknown(TonCell),
}

// Booster configuration with mode-dependent funding-tail validation.
#[derive(Debug, Clone, PartialEq)]
pub struct MoonBoosterConfigureMsg {
    pub query_id: u64,
    pub mode: u8,
    pub boost: TLBRef<MoonBoosterBoostData>,
    pub funding: MoonBoosterConfigurationTail,
}
impl TLB for MoonBoosterConfigureMsg {
    const PREFIX: TLBPrefix = TLBPrefix::new(2608328882, 32);
    fn read_definition(parser: &mut CellParser) -> TonCoreResult<Self> {
        let query_id: u64 = parser.read_num(64)?;
        let mode: u8 = parser.read_num(8)?;
        let boost: TLBRef<MoonBoosterBoostData> = TLB::read(parser)?;
        let funding: MoonBoosterConfigurationTail = TLB::read(parser)?;
        if !matches!(
            (&funding, mode),
            (MoonBoosterConfigurationTail::Empty, 1) | (MoonBoosterConfigurationTail::Funding(_), 2)
        ) {
            ton::ton_core::bail_ton_core_data!("booster mode/tail mismatch");
        }
        Ok(Self {
            query_id,
            mode,
            boost,
            funding,
        })
    }
    fn write_definition(&self, builder: &mut CellBuilder) -> TonCoreResult<()> {
        if !matches!(
            (&self.funding, self.mode),
            (MoonBoosterConfigurationTail::Empty, 1) | (MoonBoosterConfigurationTail::Funding(_), 2)
        ) {
            ton::ton_core::bail_ton_core_data!("booster mode/tail mismatch");
        }
        builder.write_num(&self.query_id, 64)?;
        builder.write_num(&self.mode, 8)?;
        self.boost.write(builder)?;
        self.funding.write(builder)?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum MoonBoosterConfigurationTail {
    Empty,
    Funding(MoonBoosterFundingTail),
}
impl TLB for MoonBoosterConfigurationTail {
    fn read_definition(parser: &mut CellParser) -> TonCoreResult<Self> {
        if parser.data_bits_left()? == 0 && parser.refs_left() == 0 {
            Ok(Self::Empty)
        } else {
            Ok(Self::Funding(MoonBoosterFundingTail::read(parser)?))
        }
    }
    fn write_definition(&self, builder: &mut CellBuilder) -> TonCoreResult<()> {
        match self {
            Self::Empty => Ok(()),
            Self::Funding(v) => v.write(builder),
        }
    }
}
