//! Moon wire records. See tools/reference/moon/README.md.

use crate::contracts::cpmm::position::storage::MoonCpmmLpPositionContent;
use crate::contracts::msgs::MoonJettonTransferMsg;
use fastnum::U256;
use ton::ton_core::{
    TLB,
    cell::TonCell,
    types::{
        TonAddress,
        tlb_core::{
            TLBCoins, TLBRef,
            adapters::{InternalAddress, NullableAddress},
        },
    },
};
use tonabis_core::adapters::expected_bool::ExpectedBool;
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x50bbec04, bits_len = 32)]
pub struct MoonCpmmJettonBoostPayloadMsg {}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x90af0b63, bits_len = 32)]
pub struct MoonCpmmJettonDepositLiquidityPayloadMsg {
    pub min_lp_out: TLBCoins,
    pub locked: bool,
    pub split: Option<TLBRef<MoonCpmmLpSplit>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x9d142643, bits_len = 32)]
pub struct MoonCpmmJettonSwapFulfillPayloadMsg {
    pub swap_params: Option<TLBRef<MoonCpmmJettonSwapParams>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmJettonSwapParams {
    #[tlb(adapter = "InternalAddress")]
    pub excess: TonAddress,
    pub min_out: TLBCoins,
    pub next_fulfill: Option<TLBRef<TonCell>>,
    pub next_reject: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xfa45056b, bits_len = 32)]
pub struct MoonCpmmJettonSwapPayloadMsg {
    pub swap_params: MoonCpmmJettonSwapParams,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmLpSplit {
    #[tlb(adapter = "InternalAddress")]
    pub a_recipient: TonAddress,
    pub a_bps: u16,
    #[tlb(adapter = "InternalAddress")]
    pub b_recipient: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x50bbec04, bits_len = 32)]
pub struct MoonCpmmBoostPoolMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x1f911dd9, bits_len = 32)]
pub struct MoonCpmmCancelDepositMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub provider: TonAddress,
    pub x_amount: TLBCoins,
    pub y_amount: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc492cce9, bits_len = 32)]
pub struct MoonCpmmCollectProtocolFeesMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xec725c4f, bits_len = 32)]
pub struct MoonCpmmCreatePoolMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub x_master: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub y_master: Option<TonAddress>,
    pub fee_bps: u8,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6ca4cb66, bits_len = 32)]
pub struct MoonCpmmDeployPoolMsg {
    pub query_id: u64,
    pub lp_fee: u8,
    pub protocol_fee: u8,
    pub lp_acc_code: TLBRef<TonCell>,
    pub lp_position_code: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7b5ceb3f, bits_len = 32)]
pub struct MoonCpmmDeployPositionMsg {
    pub query_id: u64,
    pub content: TLBRef<MoonCpmmLpPositionContent>,
    pub locked: bool,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x90af0b63, bits_len = 32)]
pub struct MoonCpmmDepositLiquidityMsg {
    pub query_id: u64,
    pub min_lp_out: TLBCoins,
    pub locked: bool,
    pub split: Option<TLBRef<MoonCpmmLpSplit>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xfda9374a, bits_len = 32)]
pub struct MoonCpmmDepositLiquidityRecordMsg {
    pub query_id: u64,
    pub x_in: bool,
    pub amount: TLBCoins,
    pub min_lp_out: TLBCoins,
    pub locked: bool,
    pub split: Option<TLBRef<MoonCpmmLpSplit>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xa7335450, bits_len = 32)]
pub struct MoonCpmmInternalCollectProtocolFeesMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xbbfa6e0d, bits_len = 32)]
pub struct MoonCpmmInternalUpdatePoolFeesMsg {
    pub query_id: u64,
    pub new_lp_fee: u8,
    pub new_protocol_fee: u8,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc29e72ba, bits_len = 32)]
pub struct MoonCpmmInternalUpdatePoolWalletsMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub new_xwallet: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub new_ywallet: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x38599428, bits_len = 32)]
pub struct MoonCpmmInternalUpgradePoolMsg {
    pub query_id: u64,
    pub new_code: Option<TLBRef<TonCell>>,
    pub new_data: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd5c9f8c3, bits_len = 32)]
pub struct MoonCpmmInternalWithdrawFeesMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub provider: TonAddress,
    pub lp_pos: TLBCoins,
    pub x_fees_pos: U256,
    pub y_fees_pos: U256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x90ad0f98, bits_len = 32)]
pub struct MoonCpmmInternalWithdrawFeesUpdateMsg {
    pub query_id: u64,
    pub x_fees_lp: U256,
    pub y_fees_lp: U256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcbd7f377, bits_len = 32)]
pub struct MoonCpmmInternalWithdrawLiquidityMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub provider: TonAddress,
    pub lp_pos: TLBCoins,
    pub x_fees_pos: U256,
    pub y_fees_pos: U256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xb751f838, bits_len = 32)]
pub struct MoonCpmmLockLiquidityMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x48a936dd, bits_len = 32)]
pub struct MoonCpmmProvideLiquidityMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub provider: TonAddress,
    pub x_amount: TLBCoins,
    pub y_amount: TLBCoins,
    pub min_lp_out: TLBCoins,
    pub locked: bool,
    pub split: Option<TLBRef<MoonCpmmLpSplit>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x1d42af37, bits_len = 32)]
pub struct MoonCpmmProvidePoolAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub x_master: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_master: TonAddress,
    pub fee_bps: u8,
    #[tlb(adapter = "InternalAddress")]
    pub initiator: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x44daf70f, bits_len = 32)]
pub struct MoonCpmmProvidePoolReadyMsg {
    pub query_id: u64,
    pub attempt: u8,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x2c76b973, bits_len = 32)]
pub struct MoonCpmmProvideWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub request_address: TonAddress,
    pub include_address: bool,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xfa45056b, bits_len = 32)]
pub struct MoonCpmmSwapMsg {
    pub query_id: u64,
    pub swap_params: MoonCpmmTonSwapParams,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x9d142643, bits_len = 32)]
pub struct MoonCpmmSwapFulfillMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub swap_params: Option<TLBRef<MoonCpmmTonSwapParams>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x81a58422, bits_len = 32)]
pub struct MoonCpmmTakePoolAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub x_master: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_master: TonAddress,
    pub fee_bps: u8,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x67071f0d, bits_len = 32)]
pub struct MoonCpmmTakePoolReadyMsg {
    pub query_id: u64,
    pub attempt: u8,
    pub is_ready: bool,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd1735400, bits_len = 32)]
pub struct MoonCpmmTakeWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub wallet: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7362d09c, bits_len = 32)]
pub struct MoonCpmmTransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    pub forward_payload: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0f3a90ad, bits_len = 32)]
pub struct MoonCpmmUpdateFactoryFeesMsg {
    pub query_id: u64,
    pub new_lp_fee: u8,
    pub new_protocol_fee: u8,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x834fc9b6, bits_len = 32)]
pub struct MoonCpmmUpdatePoolFeesMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    pub new_lp_fee: u8,
    pub new_protocol_fee: u8,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x51d80e5e, bits_len = 32)]
pub struct MoonCpmmUpdatePoolWalletsMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub x_wallet: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_wallet: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xbae2139a, bits_len = 32)]
pub struct MoonCpmmUpgradeFactoryMsg {
    pub query_id: u64,
    pub new_code: Option<TLBRef<TonCell>>,
    pub new_data: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0d730f18, bits_len = 32)]
pub struct MoonCpmmUpgradePoolMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    pub new_code: Option<TLBRef<TonCell>>,
    pub new_data: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xf325e2f4, bits_len = 32)]
pub struct MoonCpmmWithdrawFeesMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x84548afa, bits_len = 32)]
pub struct MoonCpmmWithdrawLiquidityMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmSwapResultJettonBodyMsg {
    pub op: u32,
    pub payload: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmSwapResultTonBodyMsg {
    pub op: u32,
    pub query_id: u64,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonCpmmTonSwapParams {
    pub swap_amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub excess: TonAddress,
    pub min_out: TLBCoins,
    pub next_fulfill: Option<TLBRef<TonCell>>,
    pub next_reject: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x13925ff2, bits_len = 32)]
pub struct MoonCpmmLiquidityResponseMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x9d9f7bad, bits_len = 32)]
pub struct MoonCpmmDepositResponseMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum MoonCpmmMsg {
    MoonCpmmBoostPoolMsg(Box<MoonCpmmBoostPoolMsg>),
    MoonCpmmCancelDepositMsg(Box<MoonCpmmCancelDepositMsg>),
    MoonCpmmCollectProtocolFeesMsg(Box<MoonCpmmCollectProtocolFeesMsg>),
    MoonCpmmCreatePoolMsg(Box<MoonCpmmCreatePoolMsg>),
    MoonCpmmDeployPoolMsg(Box<MoonCpmmDeployPoolMsg>),
    MoonCpmmDeployPositionMsg(Box<MoonCpmmDeployPositionMsg>),
    MoonCpmmDepositLiquidityMsg(Box<MoonCpmmDepositLiquidityMsg>),
    MoonCpmmDepositLiquidityRecordMsg(Box<MoonCpmmDepositLiquidityRecordMsg>),
    MoonCpmmInternalCollectProtocolFeesMsg(Box<MoonCpmmInternalCollectProtocolFeesMsg>),
    MoonCpmmInternalUpdatePoolFeesMsg(Box<MoonCpmmInternalUpdatePoolFeesMsg>),
    MoonCpmmInternalUpdatePoolWalletsMsg(Box<MoonCpmmInternalUpdatePoolWalletsMsg>),
    MoonCpmmInternalUpgradePoolMsg(Box<MoonCpmmInternalUpgradePoolMsg>),
    MoonCpmmInternalWithdrawFeesMsg(Box<MoonCpmmInternalWithdrawFeesMsg>),
    MoonCpmmInternalWithdrawFeesUpdateMsg(Box<MoonCpmmInternalWithdrawFeesUpdateMsg>),
    MoonCpmmInternalWithdrawLiquidityMsg(Box<MoonCpmmInternalWithdrawLiquidityMsg>),
    MoonCpmmLockLiquidityMsg(Box<MoonCpmmLockLiquidityMsg>),
    MoonCpmmProvideLiquidityMsg(Box<MoonCpmmProvideLiquidityMsg>),
    MoonCpmmProvidePoolAddressMsg(Box<MoonCpmmProvidePoolAddressMsg>),
    MoonCpmmProvidePoolReadyMsg(Box<MoonCpmmProvidePoolReadyMsg>),
    MoonCpmmProvideWalletAddressMsg(Box<MoonCpmmProvideWalletAddressMsg>),
    MoonCpmmSwapMsg(Box<MoonCpmmSwapMsg>),
    MoonCpmmSwapFulfillMsg(Box<MoonCpmmSwapFulfillMsg>),
    MoonCpmmTakePoolAddressMsg(Box<MoonCpmmTakePoolAddressMsg>),
    MoonCpmmTakePoolReadyMsg(Box<MoonCpmmTakePoolReadyMsg>),
    MoonCpmmTakeWalletAddressMsg(Box<MoonCpmmTakeWalletAddressMsg>),
    MoonCpmmTransferNotificationMsg(Box<MoonCpmmTransferNotificationMsg>),
    MoonCpmmUpdateFactoryFeesMsg(Box<MoonCpmmUpdateFactoryFeesMsg>),
    MoonCpmmUpdatePoolFeesMsg(Box<MoonCpmmUpdatePoolFeesMsg>),
    MoonCpmmUpdatePoolWalletsMsg(Box<MoonCpmmUpdatePoolWalletsMsg>),
    MoonCpmmUpgradeFactoryMsg(Box<MoonCpmmUpgradeFactoryMsg>),
    MoonCpmmUpgradePoolMsg(Box<MoonCpmmUpgradePoolMsg>),
    MoonCpmmWithdrawFeesMsg(Box<MoonCpmmWithdrawFeesMsg>),
    MoonCpmmWithdrawLiquidityMsg(Box<MoonCpmmWithdrawLiquidityMsg>),
    MoonCpmmLiquidityResponseMsg(Box<MoonCpmmLiquidityResponseMsg>),
    MoonCpmmDepositResponseMsg(Box<MoonCpmmDepositResponseMsg>),
    JettonTransfer(Box<MoonJettonTransferMsg>),
    Unknown(TonCell),
}

// SDK transfer variants with literal payload flags checked on read and write.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
#[tlb(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct MoonCpmmJettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub response_destination: TonAddress,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    /// Literal Either selector for this layout; changing it selects a different message.
    #[tlb(adapter = "ExpectedBool::<false>")]
    pub have_payload: bool,
}

#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
#[tlb(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct MoonCpmmJettonTransferWithPayloadMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub response_destination: TonAddress,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    /// Literal Either selector for this layout; changing it selects a different message.
    #[tlb(adapter = "ExpectedBool::<true>")]
    pub have_payload: bool,
    pub forward_payload: TLBRef<TonCell>,
}
