//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use crate::contracts::native::wallet::msgs::ToncoNativeWalletExcessesMsg;
use crate::contracts::v1::position_nft::msgs::ToncoPositionNftV1OwnershipAssignedMsg;
use crate::contracts::v1::position_nft::msgs::ToncoPositionNftV1OwnershipNotifyMsg;
use crate::contracts::v2::msgs::ToncoReforgeV2Msg;
use crate::contracts::v2::msgs::ToncoV2BurnOrders;
use crate::contracts::v2::msgs::ToncoV2Fees;
use crate::contracts::v2::pool::msgs::ToncoPoolV2BurnActions;
use crate::contracts::v2::position_nft::storage::ToncoPositionNftV2PositionState;
use fastnum::U256;
use ton::block_tlb::CurrencyCollection;
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
pub struct ToncoPositionNftV2OriginalInfo {
    pub value: CurrencyCollection,
    pub created_lt: u64,
    pub created_at: u32,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPositionNftV2ComputeInfo {
    pub gas_used: u32,
    pub vm_steps: u32,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xfffffffe, bits_len = 32)]
pub struct ToncoPositionNftV2RichBounceMsg {
    pub original_body: TLBRef<TonCell>,
    pub original_info: TLBRef<ToncoPositionNftV2OriginalInfo>,
    pub phase: u8,
    pub exit_code: i32,
    pub compute: Option<ToncoPositionNftV2ComputeInfo>,
}

/// Executable V2 code reads a referenced position cell, unlike the stale SDK builder.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd5ecca2a, bits_len = 32)]
pub struct ToncoPositionNftV2InitMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub user: Option<TonAddress>,
    pub position: TLBRef<ToncoPositionNftV2PositionState>,
    pub indexer: ToncoPositionNftV2InitLogSuffix,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x46ca335a, bits_len = 32)]
pub struct ToncoPositionNftV2BurnMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub lower: i32,
    #[tlb(bits_len = 24)]
    pub upper: i32,
    pub fees: TLBRef<ToncoV2Fees>,
    pub actions: Option<TLBRef<ToncoPoolV2BurnActions>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0c52e3b, bits_len = 32)]
pub struct ToncoPositionNftV2GetDataMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub target: Option<TonAddress>,
    pub include_fee: bool,
    pub payload: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xdedf1e98, bits_len = 32)]
pub struct ToncoPositionNftV2ReportDataMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub pool: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub user: Option<TonAddress>,
    pub index: u64,
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub lower: i32,
    #[tlb(bits_len = 24)]
    pub upper: i32,
    pub fees: Option<TLBRef<ToncoV2Fees>>,
    pub payload: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x5fcc3d14, bits_len = 32)]
pub struct ToncoPositionNftV2TransferMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub new_owner: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response: Option<TonAddress>,
    pub custom: Option<TLBRef<TonCell>>,
    pub forward_amount: TLBCoins,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoPositionNftV2IncomingMessage {
    RichBounce(Box<ToncoPositionNftV2RichBounceMsg>),
    Deposit(Box<ToncoPositionNftV2DepositMsg>),
    PositionInit(Box<ToncoPositionNftV2InitMsg>),
    PositionBurn(Box<ToncoPositionNftV2BurnMsg>),
    GetData(Box<ToncoPositionNftV2GetDataMsg>),
    Transfer(Box<ToncoPositionNftV2TransferMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x062a8cca, bits_len = 32)]
pub struct ToncoPositionNftV2DepositMsg {
    pub query_id: u64,
    pub liquidity: u128,
}

/// Pool-generated source-defined indexer suffix. A direct initializer may omit it.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPositionNftV2InitLog {
    pub seqno: u64,
    pub index: u64,
    pub amount0: TLBCoins,
    pub amount1: TLBCoins,
    #[tlb(bits_len = 24)]
    pub tick: i32,
    #[tlb(bits_len = 160)]
    pub price: U256,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToncoPositionNftV2InitLogSuffix(pub Option<TLBRef<ToncoPositionNftV2InitLog>>);

impl ton::ton_core::traits::tlb::TLB for ToncoPositionNftV2InitLogSuffix {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        if p.ensure_empty().is_ok() {
            return Ok(Self(None));
        }
        let log = TLBRef::<ToncoPositionNftV2InitLog>::read(p)?;
        p.ensure_empty()?;
        Ok(Self(Some(log)))
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        if let Some(v) = &self.0 {
            v.write(b)?;
        }
        Ok(())
    }
}

/// NFT-to-pool callback; deposit does not withdraw principal. The pool forwards
/// the position to its derived account through PayTo's position list.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x062a8cca, bits_len = 32)]
pub struct ToncoPositionNftV2DepositCallbackMsg {
    pub query_id: u64,
    pub index: u64,
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
    pub positions: TLBRef<ToncoV2BurnOrders>,
    pub new_fees: Option<TLBRef<ToncoV2Fees>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoPositionNftV2OutgoingMessage {
    OwnershipNotify(Box<ToncoPositionNftV1OwnershipNotifyMsg>),
    Deposit(Box<ToncoPositionNftV2DepositCallbackMsg>),
    Reforge(Box<ToncoReforgeV2Msg>),
    ReportData(Box<ToncoPositionNftV2ReportDataMsg>),
    OwnershipAssigned(Box<ToncoPositionNftV1OwnershipAssignedMsg>),
    Excesses(Box<ToncoNativeWalletExcessesMsg>),
    Unknown(TonCell),
}
