//! Classic operator-managed PoolV2 code778f0d3f/c0f9d14f. These are distinct
//! from ordinary code1275095b and from CPMM v2. Both add a swap activation time.
use crate::contracts::classic::pool_v2;
use ton::ton_core::{
    TLB,
    cell::{CellBuilder, CellParser, TonCell},
    errors::TonCoreResult,
    traits::tlb::TLB,
};

/// Fixed role10 operator request. Authority is checked by the contract, not parsing.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7ed7f6ce, bits_len = 32)]
pub struct DedustOperatorPoolConfigureSwapActivationMsg {
    pub query_id: u64,
    pub active_since: u32,
}
#[derive(Debug, Clone, PartialEq)]
pub enum DedustOperatorPoolIncomingMsg {
    ConfigureSwapActivation(DedustOperatorPoolConfigureSwapActivationMsg),
    Common(pool_v2::msgs::DedustPoolV2IncomingMsg),
    Unknown(TonCell),
}

// Common is itself a dispatcher with Unknown. Native sequential derive would
// consume every raw cell there, making the outer Unknown unreachable. Keep its
// nested-proof errors and route only recognized common bodies into Common.
impl TLB for DedustOperatorPoolIncomingMsg {
    fn read_definition(parser: &mut CellParser) -> TonCoreResult<Self> {
        if parser.data_bits_left()? >= 32 && parser.lookup_bits(32)? == 0x7ed7f6ce {
            return Ok(Self::ConfigureSwapActivation(
                DedustOperatorPoolConfigureSwapActivationMsg::read(parser)?,
            ));
        }
        match pool_v2::msgs::DedustPoolV2IncomingMsg::read(parser)? {
            pool_v2::msgs::DedustPoolV2IncomingMsg::Unknown(cell) => Ok(Self::Unknown(cell)),
            value => Ok(Self::Common(value)),
        }
    }
    fn write_definition(&self, builder: &mut CellBuilder) -> TonCoreResult<()> {
        match self {
            Self::ConfigureSwapActivation(value) => value.write(builder),
            Self::Common(value) => value.write(builder),
            Self::Unknown(cell) => cell.write(builder),
        }
    }
}
