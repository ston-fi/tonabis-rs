//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use crate::contracts::v1::account::msgs::ToncoAccountV1AddLiquidityMsg;
use crate::contracts::v1::position_nft::msgs::ToncoPositionNftV1InitMsg;
use crate::contracts::v1::router::msgs::ToncoRouterV1PayToMsg;
use crate::contracts::v2::router::storage::ToncoRouterV2OptionalU64;
use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::ton_core::types::tlb_core::{TLBCoins, TLBRef};

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV1InitMinterCell {
    #[tlb(adapter = "NullableAddress")]
    pub jetton0_minter: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub jetton1_minter: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x441c39ed, bits_len = 32)]
pub struct ToncoPoolV1InitMsg {
    pub query_id: u64,
    pub from_admin: bool,
    pub has_admin: bool,
    #[tlb(adapter = "NullableAddress")]
    pub admin_addr: Option<TonAddress>,
    pub has_controller: bool,
    #[tlb(adapter = "NullableAddress")]
    pub controller_addr: Option<TonAddress>,
    pub set_spacing: bool,
    #[tlb(bits_len = 24)]
    pub tick_spacing: u32,
    pub set_price: bool,
    #[tlb(bits_len = 160)]
    pub initial_price_x96: U256,
    pub set_active: bool,
    pub pool_active: bool,
    pub protocol_fee: u16,
    pub lp_fee_base: u16,
    pub lp_fee_current: u16,
    pub nftv3_content: TLBRef<TonCell>,
    pub nftv3item_content: TLBRef<TonCell>,
    pub minter_cell: Option<TLBRef<ToncoPoolV1InitMinterCell>>,
    pub router_seqno: ToncoRouterV2OptionalU64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x05e74697, bits_len = 32)]
pub struct ToncoPoolV1LockMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x3205adbd, bits_len = 32)]
pub struct ToncoPoolV1UnlockMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x42a0fb43, bits_len = 32)]
pub struct ToncoPoolV1ResetGasMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6bdcbeb8, bits_len = 32)]
pub struct ToncoPoolV1SetFeeMsg {
    pub query_id: u64,
    pub protocol_fee: u16,
    pub lp_fee_base: u16,
    pub lp_fee_current: u16,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd3f8a538, bits_len = 32)]
pub struct ToncoPoolV1CollectProtocolMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x530b5f2c, bits_len = 32)]
pub struct ToncoPoolV1StartBurnMsg {
    pub query_id: u64,
    pub burned_index: u64,
    pub liquidity2_burn: u128,
    #[tlb(bits_len = 24)]
    pub tick_lower: i32,
    #[tlb(bits_len = 24)]
    pub tick_upper: i32,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV1BurnOldFeeCell {
    pub fee_growth_inside0_last_x128: U256,
    pub fee_growth_inside1_last_x128: U256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV1BurnNewFeeCell {
    pub fee_growth_inside0_current_x128: U256,
    pub fee_growth_inside1_current_x128: U256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd73ac09d, bits_len = 32)]
pub struct ToncoPoolV1BurnMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub recipient: Option<TonAddress>,
    pub burned_index: u64,
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub tick_lower: i32,
    #[tlb(bits_len = 24)]
    pub tick_upper: i32,
    pub liquidity2_burn: u128,
    pub old_fee_cell: TLBRef<ToncoPoolV1BurnOldFeeCell>,
    pub new_fee_cell: TLBRef<ToncoPoolV1BurnNewFeeCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV1SwapParamsCell {
    pub amount: TLBCoins,
    #[tlb(bits_len = 160)]
    pub sqrt_price_limit_x96: U256,
    pub min_out_amount: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV1SwapPayloadsCell {
    #[tlb(adapter = "NullableAddress")]
    pub target_address: Option<TonAddress>,
    pub ok_forward_amount: TLBCoins,
    pub ok_forward_payload: Option<TLBRef<TonCell>>,
    pub ret_forward_amount: TLBCoins,
    pub ret_forward_payload: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xa7fb58f8, bits_len = 32)]
pub struct ToncoPoolV1SwapMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub owner_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub source_wallet: Option<TonAddress>,
    pub params_cell: TLBRef<ToncoPoolV1SwapParamsCell>,
    pub payloads_cell: TLBRef<ToncoPoolV1SwapPayloadsCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x4468de77, bits_len = 32)]
pub struct ToncoPoolV1FundAccountMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub owner_addr: Option<TonAddress>,
    pub amount0: TLBCoins,
    pub amount1: TLBCoins,
    pub enough0: TLBCoins,
    pub enough1: TLBCoins,
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub tick_lower: i32,
    #[tlb(bits_len = 24)]
    pub tick_upper: i32,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x81702ef8, bits_len = 32)]
pub struct ToncoPoolV1MintMsg {
    pub query_id: u64,
    pub amount0_funded: TLBCoins,
    pub amount1_funded: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub recipient: Option<TonAddress>,
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub tick_lower: i32,
    #[tlb(bits_len = 24)]
    pub tick_upper: i32,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoPoolV1IncomingMessage {
    PoolInit(Box<ToncoPoolV1InitMsg>),
    PoolLock(Box<ToncoPoolV1LockMsg>),
    PoolUnlock(Box<ToncoPoolV1UnlockMsg>),
    PoolResetGas(Box<ToncoPoolV1ResetGasMsg>),
    PoolSetFee(Box<ToncoPoolV1SetFeeMsg>),
    PoolCollectProtocol(Box<ToncoPoolV1CollectProtocolMsg>),
    PoolStartBurn(Box<ToncoPoolV1StartBurnMsg>),
    PoolBurn(Box<ToncoPoolV1BurnMsg>),
    PoolSwap(Box<ToncoPoolV1SwapMsg>),
    PoolFundAccount(Box<ToncoPoolV1FundAccountMsg>),
    PoolMint(Box<ToncoPoolV1MintMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoPoolV1OutgoingMessage {
    RouterPayTo(Box<ToncoRouterV1PayToMsg>),
    AccountAddLiquidity(Box<ToncoAccountV1AddLiquidityMsg>),
    PositionNftPositionInit(Box<ToncoPositionNftV1InitMsg>),
    Unknown(TonCell),
}
