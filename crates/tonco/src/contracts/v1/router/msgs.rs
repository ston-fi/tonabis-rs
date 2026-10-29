//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use crate::contracts::native::wallet::msgs::ToncoNativeWalletExcessesMsg;
use crate::contracts::v1::pool::msgs::ToncoPoolV1FundAccountMsg;
use crate::contracts::v1::pool::msgs::ToncoPoolV1InitMsg;
use crate::contracts::v1::pool::msgs::ToncoPoolV1SwapMsg;
use fastnum::{I256, U256};
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
pub struct ToncoRouterV1CreatePoolMinterCell {
    #[tlb(adapter = "NullableAddress")]
    pub jetton0_minter: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub jetton1_minter: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub controller_address: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x2e3034ef, bits_len = 32)]
pub struct ToncoRouterV1CreatePoolMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub jetton_wallet0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub jetton_wallet1: Option<TonAddress>,
    #[tlb(bits_len = 24)]
    pub tick_spacing: u32,
    #[tlb(bits_len = 160)]
    pub initial_price_x96: U256,
    pub activate_pool: bool,
    pub protocol_fee: u16,
    pub lp_fee_base: u16,
    pub lp_fee_current: u16,
    pub nftv3_content: TLBRef<TonCell>,
    pub nftv3item_content: TLBRef<TonCell>,
    pub minter_cell: TLBRef<ToncoRouterV1CreatePoolMinterCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7362d09c, bits_len = 32)]
pub struct ToncoRouterV1JettonTransferNotificationMsg {
    pub query_id: u64,
    pub jetton_amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub from_user: Option<TonAddress>,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xa1daa96d, bits_len = 32)]
pub struct ToncoRouterV1PayToMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub receiver0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub receiver1: Option<TonAddress>,
    pub pay_to: ToncoPayToPayload,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x42a0fb43, bits_len = 32)]
pub struct ToncoRouterV1ResetGasMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd4deb03b, bits_len = 32)]
pub struct ToncoRouterV1ChangeAdminStartMsg {
    pub query_id: u64,
    pub has_new_admin: bool,
    #[tlb(adapter = "NullableAddress")]
    pub new_admin: Option<TonAddress>,
    pub has_new_flags: bool,
    pub new_flags: u64,
    pub new_code: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x98bac405, bits_len = 32)]
pub struct ToncoRouterV1ChangeAdminCommitMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x00000000, bits_len = 32)]
pub struct ToncoRouterV1PayToCode0Msg {
    pub seqno: u64,
    pub coinsinfo_cell: Option<TLBRef<ToncoRouterV1PayToCoinsinfoCell>>,
    /// Emitted as an absent second dictionary by the pinned refund/mint path.
    /// Preserve any future indexer extension without interpreting it.
    pub indexer: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x000000c8, bits_len = 32)]
pub struct ToncoRouterV1PayToCode200Msg {
    pub seqno: u64,
    pub coinsinfo_cell: Option<TLBRef<ToncoRouterV1PayToCoinsinfoCell>>,
    pub indexer_swap_info_cell: Option<TLBRef<ToncoRouterV1PayToIndexerSwapInfoCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x000000c9, bits_len = 32)]
pub struct ToncoRouterV1PayToCode201Msg {
    pub seqno: u64,
    pub coinsinfo_cell: Option<TLBRef<ToncoRouterV1PayToCoinsinfoCell>>,
    pub indexer_burn_info_cell: Option<TLBRef<ToncoRouterV1PayToIndexerBurnInfoCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x000000e6, bits_len = 32)]
pub struct ToncoRouterV1PayToCode230Msg {
    pub seqno: u64,
    pub coinsinfo_cell: Option<TLBRef<ToncoRouterV1PayToCoinsinfoCell>>,
    pub indexer_swap_info_cell: Option<TLBRef<ToncoRouterV1PayToIndexerSwapInfoCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV1PayToCoinsinfoCell {
    pub amount0: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub jetton0_address: Option<TonAddress>,
    pub amount1: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub jetton1_address: Option<TonAddress>,
    pub payload: ToncoRouterV1PayloadSuffix,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV1PayToPayloadCell {
    pub payload_amount0: TLBCoins,
    pub payload_cell0: Option<TLBRef<TonCell>>,
    pub payload_amount1: TLBCoins,
    pub payload_cell1: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV1PayToIndexerSwapInfoCell {
    pub liquidity: u128,
    #[tlb(bits_len = 160)]
    pub price_sqrt: U256,
    #[tlb(bits_len = 24)]
    pub tick: i32,
    pub fee_growth_global0_x128: I256,
    pub fee_growth_global1_x128: I256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV1PayToIndexerBurnInfoCell {
    pub nft_index: u64,
    pub liquidity_burned: u128,
    #[tlb(bits_len = 24)]
    pub tick_lower: i32,
    #[tlb(bits_len = 24)]
    pub tick_upper: i32,
    #[tlb(bits_len = 24)]
    pub tick: i32,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV1SwapForwardPayloadMultihopCell {
    #[tlb(adapter = "NullableAddress")]
    pub target_address: Option<TonAddress>,
    pub ok_forward_amount: TLBCoins,
    pub ok_forward_payload: Option<TLBRef<TonCell>>,
    pub ret_forward_amount: TLBCoins,
    pub ret_forward_payload: Option<TLBRef<TonCell>>,
    #[tlb(adapter = "NullableAddress")]
    pub excess_address: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xa7fb58f8, bits_len = 32)]
pub struct ToncoRouterV1SwapForwardPayloadMsg {
    #[tlb(adapter = "NullableAddress")]
    pub target_jetton_wallet: Option<TonAddress>,
    #[tlb(bits_len = 160)]
    pub sqrt_price_limit_x96: U256,
    pub min_out_amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub owner_address: Option<TonAddress>,
    pub multihop_cell: Option<TLBRef<ToncoRouterV1SwapForwardPayloadMultihopCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x4468de77, bits_len = 32)]
pub struct ToncoRouterV1FundAccountPayloadMsg {
    #[tlb(adapter = "NullableAddress")]
    pub jetton_target_wallet: Option<TonAddress>,
    pub enough0: TLBCoins,
    pub enough1: TLBCoins,
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub tick_lower: i32,
    #[tlb(bits_len = 24)]
    pub tick_upper: i32,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoPayToPayload {
    PayToCode0(Box<ToncoRouterV1PayToCode0Msg>),
    PayToCode200(Box<ToncoRouterV1PayToCode200Msg>),
    PayToCode201(Box<ToncoRouterV1PayToCode201Msg>),
    PayToCode230(Box<ToncoRouterV1PayToCode230Msg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoRouterV1JettonForwardPayload {
    SwapForwardPayload(Box<ToncoRouterV1SwapForwardPayloadMsg>),
    FundAccountPayload(Box<ToncoRouterV1FundAccountPayloadMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoRouterV1IncomingMessage {
    Excesses(Box<ToncoNativeWalletExcessesMsg>),
    RouterCreatePool(Box<ToncoRouterV1CreatePoolMsg>),
    JettonTransferNotification(Box<ToncoRouterV1JettonTransferNotificationMsg>),
    RouterPayTo(Box<ToncoRouterV1PayToMsg>),
    RouterResetGas(Box<ToncoRouterV1ResetGasMsg>),
    RouterChangeAdminStart(Box<ToncoRouterV1ChangeAdminStartMsg>),
    RouterChangeAdminCommit(Box<ToncoRouterV1ChangeAdminCommitMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoRouterV1OutgoingMessage {
    PoolInit(Box<ToncoPoolV1InitMsg>),
    PoolSwap(Box<ToncoPoolV1SwapMsg>),
    PoolFundAccount(Box<ToncoPoolV1FundAccountMsg>),
    Unknown(TonCell),
}

/// V1 payout coin information optionally appends an untagged forwarding ref.
#[derive(Debug, Clone, PartialEq)]
pub struct ToncoRouterV1PayloadSuffix(pub Option<TLBRef<ToncoRouterV1PayToPayloadCell>>);

impl ton::ton_core::traits::tlb::TLB for ToncoRouterV1PayloadSuffix {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        let v = if p.ensure_empty().is_ok() {
            None
        } else {
            Some(TLBRef::<ToncoRouterV1PayToPayloadCell>::read(p)?)
        };
        p.ensure_empty()?;
        Ok(Self(v))
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        if let Some(v) = &self.0 {
            v.write(b)?;
        }
        Ok(())
    }
}
