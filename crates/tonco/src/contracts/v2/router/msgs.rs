//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use crate::contracts::native::wallet::msgs::ToncoNativeWalletExcessesMsg;
use crate::contracts::native::wallet::msgs::ToncoNativeWalletJettonTransferMsg;
use crate::contracts::native::wallet::msgs::ToncoNativeWalletTransferMsg;
use crate::contracts::v2::msgs::ToncoPayToV2Msg;
use crate::contracts::v2::msgs::ToncoV2OrderBody;
use crate::contracts::v2::pool::msgs::ToncoPoolV2FundAccountMsg;
use crate::contracts::v2::pool::msgs::ToncoPoolV2InitMsg;
use crate::contracts::v2::pool::msgs::ToncoPoolV2StartAdminMsg;
use crate::contracts::v2::pool::msgs::ToncoPoolV2SwapMsg;
use crate::contracts::v2::position_nft::msgs::ToncoPositionNftV2RichBounceMsg;
use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::CellBuilder;
use ton::ton_core::cell::CellParser;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBEither;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::ton_core::types::tlb_core::{TLBCoins, TLBRef};

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x87ffe363, bits_len = 32)]
pub struct ToncoRouterV2ChangeParamsMsg {
    pub query_id: u64,
    pub has_factory: bool,
    #[tlb(adapter = "NullableAddress")]
    pub factory: Option<TonAddress>,
    pub has_admin: bool,
    #[tlb(adapter = "NullableAddress")]
    pub admin: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV2SecondaryRoles {
    pub has_arbiter: bool,
    #[tlb(adapter = "NullableAddress")]
    pub arbiter: Option<TonAddress>,
    pub has_alm: bool,
    #[tlb(adapter = "NullableAddress")]
    pub alm: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV2OracleRole {
    pub has_oracle: bool,
    #[tlb(adapter = "NullableAddress")]
    pub oracle: Option<TonAddress>,
}

/// Router create roles start with controller and creator, unlike Pool Init's
/// admin/controller/creator record. Factory messages may omit privilege refs.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV2Roles {
    pub has_controller: bool,
    #[tlb(adapter = "NullableAddress")]
    pub controller: Option<TonAddress>,
    pub has_creator: bool,
    #[tlb(adapter = "NullableAddress")]
    pub creator: Option<TonAddress>,
    pub privileges: ToncoRouterV2RoleRefs,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToncoRouterV2RoleRefs {
    None,
    Secondary(TLBRef<ToncoRouterV2SecondaryRoles>),
    Both {
        secondary: TLBRef<ToncoRouterV2SecondaryRoles>,
        oracle: TLBRef<ToncoRouterV2OracleRole>,
    },
}

impl ton::ton_core::traits::tlb::TLB for ToncoRouterV2RoleRefs {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        if p.ensure_empty().is_ok() {
            return Ok(Self::None);
        }
        let secondary = TLBRef::<ToncoRouterV2SecondaryRoles>::read(p)?;
        if p.ensure_empty().is_ok() {
            return Ok(Self::Secondary(secondary));
        }
        let oracle = TLBRef::<ToncoRouterV2OracleRole>::read(p)?;
        p.ensure_empty()?;
        Ok(Self::Both { secondary, oracle })
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        match self {
            Self::None => {},
            Self::Secondary(v) => v.write(b)?,
            Self::Both { secondary, oracle } => {
                secondary.write(b)?;
                oracle.write(b)?;
            },
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV2Contents {
    pub collection: TLBRef<TonCell>,
    pub item: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV2Minters {
    #[tlb(adapter = "NullableAddress")]
    pub token0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub token1: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x2e3034ef, bits_len = 32)]
pub struct ToncoRouterV2CreatePoolMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub wallet0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub wallet1: Option<TonAddress>,
    #[tlb(bits_len = 24)]
    pub spacing: u32,
    #[tlb(bits_len = 160)]
    pub price: U256,
    pub active: bool,
    pub protocol_fee: u16,
    pub base_fee: u16,
    pub current_fee: u16,
    pub roles: TLBRef<ToncoRouterV2Roles>,
    pub contents: TLBRef<ToncoRouterV2Contents>,
    pub minters: TLBRef<ToncoRouterV2Minters>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV2SwapPayloads {
    #[tlb(adapter = "NullableAddress")]
    pub target: Option<TonAddress>,
    pub success_amount: TLBCoins,
    pub success_payload: Option<TLBRef<TonCell>>,
    pub refund_amount: TLBCoins,
    pub refund_payload: Option<TLBRef<TonCell>>,
    #[tlb(adapter = "NullableAddress")]
    pub excess: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV2Referral {
    pub code: u32,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xa7fb58f8, bits_len = 32)]
pub struct ToncoRouterV2SwapForwardMsg {
    #[tlb(adapter = "NullableAddress")]
    pub target_wallet: Option<TonAddress>,
    #[tlb(bits_len = 160)]
    pub limit: U256,
    pub minimum_output: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub origin: Option<TonAddress>,
    pub payloads: Option<TLBRef<ToncoRouterV2SwapPayloads>>,
    pub referral: Option<TLBRef<ToncoRouterV2Referral>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x134b6d30, bits_len = 32)]
pub struct ToncoRouterV2FundSomeoneMsg {
    #[tlb(adapter = "NullableAddress")]
    pub target_wallet: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub user: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xb811eb6b, bits_len = 32)]
pub struct ToncoRouterV2FundRouterMsg {}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoRouterV2IncomingMessage {
    RichBounce(Box<ToncoPositionNftV2RichBounceMsg>),
    Excesses(Box<ToncoNativeWalletExcessesMsg>),
    StartAdmin(Box<crate::contracts::v2::router::msgs::ToncoRouterV2StartAdminMsg>),
    CommitAdmin(Box<ToncoRouterV2CommitAdminMsg>),
    ResetGas(Box<ToncoRouterV2ResetGasMsg>),
    TransferNotification(Box<ToncoRouterV2TransferNotificationMsg>),
    ProxyToChild(Box<ToncoRouterV2ProxyToChildMsg>),
    PayTo(Box<ToncoPayToV2Msg>),
    ChangeParams(Box<ToncoRouterV2ChangeParamsMsg>),
    CreatePool(Box<ToncoRouterV2CreatePoolMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd4deb03b, bits_len = 32)]
pub struct ToncoRouterV2StartAdminMsg {
    pub query_id: u64,
    pub admin_change: bool,
    pub admin_reset: bool,
    #[tlb(adapter = "NullableAddress")]
    pub admin: Option<TonAddress>,
    pub proxy_change: bool,
    pub proxy_reset: bool,
    #[tlb(adapter = "NullableAddress")]
    pub proxy: Option<TonAddress>,
    pub flags_change: bool,
    pub flags_reset: bool,
    pub flags: u64,
    pub code: Option<TLBRef<TonCell>>,
    pub pool_code: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x98bac405, bits_len = 32)]
pub struct ToncoRouterV2CommitAdminMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x42a0fb43, bits_len = 32)]
pub struct ToncoRouterV2ResetGasMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7362d09c, bits_len = 32)]
pub struct ToncoRouterV2TransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub sender: Option<TonAddress>,
    pub payload: TLBEither<TonCell, TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoRouterV2ChildCommand {
    StartAdmin(Box<ToncoPoolV2StartAdminMsg>),
    CommitAdmin(Box<ToncoRouterV2CommitAdminMsg>),
    JettonTransfer(Box<ToncoNativeWalletJettonTransferMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x1441d907, bits_len = 32)]
pub struct ToncoRouterV2ProxyToChildMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub child: Option<TonAddress>,
    pub command: TLBRef<ToncoRouterV2ChildCommand>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x4468de77, bits_len = 32)]
pub struct ToncoRouterV2FundAccountMsg {
    #[tlb(adapter = "NullableAddress")]
    pub target_wallet: Option<TonAddress>,
    pub order: ToncoV2OrderBody,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoRouterV2ForwardPayload {
    Swap(Box<ToncoRouterV2SwapForwardMsg>),
    FundAccount(Box<crate::contracts::v2::router::msgs::ToncoRouterV2FundAccountMsg>),
    FundSomeone(Box<ToncoRouterV2FundSomeoneMsg>),
    FundRouter(Box<ToncoRouterV2FundRouterMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoRouterV2OutgoingMessage {
    Init(Box<ToncoPoolV2InitMsg>),
    Swap(Box<ToncoPoolV2SwapMsg>),
    FundAccount(Box<ToncoPoolV2FundAccountMsg>),
    Transfer(Box<ToncoNativeWalletTransferMsg>),
    Unknown(TonCell),
}
