//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use crate::contracts::v2::account::msgs::ToncoAccountV2AddLiquidityMsg;
use crate::contracts::v2::msgs::ToncoPayToV2Msg;
use crate::contracts::v2::msgs::ToncoReforgeV2Msg;
use crate::contracts::v2::msgs::ToncoV2OrderBody;
use crate::contracts::v2::pool::storage::ToncoPoolV2Minters;
use crate::contracts::v2::position_nft::msgs::ToncoPositionNftV2BurnMsg;
use crate::contracts::v2::position_nft::msgs::ToncoPositionNftV2DepositCallbackMsg;
use crate::contracts::v2::position_nft::msgs::ToncoPositionNftV2InitMsg;
use crate::contracts::v2::router::msgs::ToncoRouterV2Contents;
use crate::contracts::v2::router::msgs::ToncoRouterV2OracleRole;
use crate::contracts::v2::router::msgs::ToncoRouterV2SecondaryRoles;
use crate::contracts::v2::router::storage::ToncoRouterV2OptionalU64;
use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::CellBuilder;
use ton::ton_core::cell::CellParser;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::ton_core::types::tlb_core::{TLBCoins, TLBRef};

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6bdcbeb8, bits_len = 32)]
pub struct ToncoPoolV2SetFeesMsg {
    pub query_id: u64,
    pub protocol_fee: u16,
    pub base_fee: u16,
    pub current_fee: u16,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x05e74697, bits_len = 32)]
pub struct ToncoPoolV2LockMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x3205adbd, bits_len = 32)]
pub struct ToncoPoolV2UnlockMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x42a0fb43, bits_len = 32)]
pub struct ToncoPoolV2ResetGasMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x98bac405, bits_len = 32)]
pub struct ToncoPoolV2CommitAdminMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2SwapParams {
    pub amount: TLBCoins,
    #[tlb(bits_len = 160)]
    pub price_limit: U256,
    pub minimum_output: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2SwapPayloads {
    #[tlb(adapter = "NullableAddress")]
    pub target: Option<TonAddress>,
    pub success_forward: TLBCoins,
    pub success_payload: Option<TLBRef<TonCell>>,
    pub refund_forward: TLBCoins,
    pub refund_payload: Option<TLBRef<TonCell>>,
    #[tlb(adapter = "NullableAddress")]
    pub excess: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xa7fb58f8, bits_len = 32)]
pub struct ToncoPoolV2SwapMsg {
    pub query_id: u64,
    #[tlb(bits_len = 4)]
    pub source: u8,
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub origin: Option<TonAddress>,
    pub zero_for_one: bool,
    #[tlb(adapter = "NullableAddress")]
    pub refund_wallet: Option<TonAddress>,
    pub params: TLBRef<ToncoPoolV2SwapParams>,
    pub payloads: TLBRef<ToncoPoolV2SwapPayloads>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2BurnPayloads {
    pub forward0: TLBCoins,
    pub payload0: Option<TLBRef<TonCell>>,
    pub forward1: TLBCoins,
    pub payload1: Option<TLBRef<TonCell>>,
    #[tlb(adapter = "NullableAddress")]
    pub excess: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2BurnActions {
    #[tlb(adapter = "NullableAddress")]
    pub target0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub target1: Option<TonAddress>,
    pub payloads: Option<TLBRef<ToncoPoolV2BurnPayloads>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x530b5f2c, bits_len = 32)]
pub struct ToncoPoolV2StartBurnMsg {
    pub query_id: u64,
    pub index: u64,
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub lower: i32,
    #[tlb(bits_len = 24)]
    pub upper: i32,
    pub actions: Option<TLBRef<ToncoPoolV2BurnActions>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2CollectProtocolTarget {
    #[tlb(adapter = "NullableAddress")]
    pub target: Option<TonAddress>,
    pub amount0: TLBCoins,
    pub amount1: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd3f8a538, bits_len = 32)]
pub struct ToncoPoolV2CollectProtocolMsg {
    pub query_id: u64,
    pub target: ToncoPoolV2CollectProtocolSuffix,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0c52e3b, bits_len = 32)]
pub struct ToncoPoolV2GetDataMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub target: Option<TonAddress>,
    pub payload: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xdedf1e98, bits_len = 32)]
pub struct ToncoPoolV2ReportDataMsg {
    pub query_id: u64,
    #[tlb(bits_len = 160)]
    pub price: U256,
    #[tlb(bits_len = 24)]
    pub tick: i32,
    pub liquidity: u128,
    pub payload: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoPoolV2IncomingMessage {
    Deposit(Box<ToncoPositionNftV2DepositCallbackMsg>),
    StartAdmin(Box<ToncoPoolV2StartAdminMsg>),
    FundAccount(Box<ToncoPoolV2FundAccountMsg>),
    Init(Box<ToncoPoolV2InitMsg>),
    Reforge(Box<ToncoReforgeV2Msg>),
    SetFees(Box<ToncoPoolV2SetFeesMsg>),
    Lock(Box<ToncoPoolV2LockMsg>),
    Unlock(Box<ToncoPoolV2UnlockMsg>),
    ResetGas(Box<ToncoPoolV2ResetGasMsg>),
    StartBurn(Box<ToncoPoolV2StartBurnMsg>),
    Swap(Box<ToncoPoolV2SwapMsg>),
    CollectProtocol(Box<ToncoPoolV2CollectProtocolMsg>),
    GetData(Box<ToncoPoolV2GetDataMsg>),
    CommitAdmin(Box<ToncoPoolV2CommitAdminMsg>),
    Unknown(TonCell),
}

/// Optional source-defined suffix, selected by remaining length rather than a tag bit.
#[derive(Debug, Clone, PartialEq)]
pub struct ToncoPoolV2CollectProtocolSuffix(pub Option<ToncoPoolV2CollectProtocolTarget>);

impl ton::ton_core::traits::tlb::TLB for ToncoPoolV2CollectProtocolSuffix {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        let v = if p.data_bits_left()? == 0 {
            None
        } else {
            Some(ToncoPoolV2CollectProtocolTarget::read(p)?)
        };
        p.ensure_empty()?;
        Ok(Self(v))
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        if let Some(t) = &self.0 {
            t.write(b)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2ReplacementCode {
    pub pool: TLBRef<TonCell>,
    pub account: TLBRef<TonCell>,
    pub position: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x441c39ed, bits_len = 32)]
pub struct ToncoPoolV2InitMsg {
    pub query_id: u64,
    pub from_admin: bool,
    pub roles: TLBRef<ToncoPoolV2RolesUpdate>,
    pub has_spacing: bool,
    #[tlb(bits_len = 24)]
    pub spacing: u32,
    pub has_price: bool,
    #[tlb(bits_len = 160)]
    pub price: U256,
    pub has_active: bool,
    pub active: bool,
    pub protocol_fee: u16,
    pub base_fee: u16,
    pub current_fee: u16,
    pub has_flags: bool,
    pub flags: u32,
    pub code: TLBRef<ToncoPoolV2ReplacementCode>,
    pub content: TLBRef<ToncoRouterV2Contents>,
    pub minters: Option<TLBRef<ToncoPoolV2Minters>>,
    pub router_seqno: ToncoRouterV2OptionalU64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoPoolV2OutgoingMessage {
    ReportData(Box<ToncoPoolV2ReportDataMsg>),
    PayTo(Box<ToncoPayToV2Msg>),
    AddLiquidity(Box<ToncoAccountV2AddLiquidityMsg>),
    PositionInit(Box<ToncoPositionNftV2InitMsg>),
    PositionBurn(Box<ToncoPositionNftV2BurnMsg>),
    Unknown(TonCell),
}

/// Router-proven funding amount and original wallet used for bounced refunds.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2FundingParams {
    pub token0: bool,
    pub amount: TLBCoins,
    pub additional_amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub refund_wallet: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x4468de77, bits_len = 32)]
pub struct ToncoPoolV2FundAccountMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub from_user: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
    pub params: TLBRef<ToncoPoolV2FundingParams>,
    pub order: Option<TLBRef<ToncoV2OrderBody>>,
}

/// Pool timelock request: ALM first, arbiter second. Captured-code replay
/// distinguishes these roles from the router's admin/proxy update fields.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd4deb03b, bits_len = 32)]
pub struct ToncoPoolV2StartAdminMsg {
    pub query_id: u64,
    pub alm_change: bool,
    pub alm_reset: bool,
    #[tlb(adapter = "NullableAddress")]
    pub alm: Option<TonAddress>,
    pub arbiter_change: bool,
    pub arbiter_reset: bool,
    #[tlb(adapter = "NullableAddress")]
    pub arbiter: Option<TonAddress>,
    pub flags_change: bool,
    pub flags_reset: bool,
    pub flags: u64,
    pub code: Option<TLBRef<TonCell>>,
}

/// Pool ToncoPoolV2InitMsg roles include admin before controller and creator; Router CreatePool
/// omits that field and has a different record.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2RolesUpdate {
    pub has_admin: bool,
    #[tlb(adapter = "NullableAddress")]
    pub admin: Option<TonAddress>,
    pub has_controller: bool,
    #[tlb(adapter = "NullableAddress")]
    pub controller: Option<TonAddress>,
    pub has_creator: bool,
    #[tlb(adapter = "NullableAddress")]
    pub creator: Option<TonAddress>,
    pub secondary: TLBRef<ToncoRouterV2SecondaryRoles>,
    pub oracle: TLBRef<ToncoRouterV2OracleRole>,
}
