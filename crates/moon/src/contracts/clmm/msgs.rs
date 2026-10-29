//! Moon wire records. See tools/reference/moon/README.md.

use crate::contracts::clmm::range::storage::{MoonClmmBin, MoonClmmRangeInitPrice, MoonClmmRangeState};
use crate::contracts::msgs::MoonJettonTransferMsg;
use fastnum::U256;
use std::collections::HashMap;
use ton::{
    tlb_adapters::{DictKeyAdapterInt, DictValAdapterTLB, TLBHashMapE},
    ton_core::{
        TLB,
        cell::TonCell,
        types::{
            TonAddress,
            tlb_core::{
                TLBCoins, TLBRef,
                adapters::{InternalAddress, NullableAddress},
            },
        },
    },
};
use tonabis_core::adapters::expected_bool::ExpectedBool;
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmDepositBin {
    pub x: TLBCoins,
    pub y: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xec401442, bits_len = 32)]
pub struct MoonClmmJettonDepositLiquidityPayloadMsg {
    pub is_single: bool,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterInt<64, i64>, DictValAdapterTLB<MoonClmmDepositBin>>::new(64)")]
    pub deposit_bins: HashMap<i64, MoonClmmDepositBin>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x9d142643, bits_len = 32)]
pub struct MoonClmmJettonSwapFulfillPayloadMsg {
    pub swap_params: Option<TLBRef<MoonClmmJettonSwapParams>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmJettonSwapParams {
    #[tlb(adapter = "InternalAddress")]
    pub excess: TonAddress,
    pub sqrt_price_limit: U256,
    pub next_fulfill: Option<TLBRef<TonCell>>,
    pub next_reject: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd151210f, bits_len = 32)]
pub struct MoonClmmJettonSwapPayloadMsg {
    pub swap_params: MoonClmmJettonSwapParams,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xdbc809b5, bits_len = 32)]
pub struct MoonClmmApplyCollectProtocolFeesMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x90be29eb, bits_len = 32)]
pub struct MoonClmmApplyUpdateRangeFeesMsg {
    pub query_id: u64,
    pub lp_fee: u8,
    pub protocol_fee: u8,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xa17882c8, bits_len = 32)]
pub struct MoonClmmApplyUpdateRangeWalletsMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub x_wallet: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_wallet: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xf0262313, bits_len = 32)]
pub struct MoonClmmApplyUpgradeRangeMsg {
    pub query_id: u64,
    pub new_code: Option<TLBRef<TonCell>>,
    pub new_data: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x2dcb38cb, bits_len = 32)]
pub struct MoonClmmCancelDepositMsg {
    pub query_id: u64,
    pub x_amount: TLBCoins,
    pub y_amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub provider: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x4b3332cb, bits_len = 32)]
pub struct MoonClmmCollectProtocolFeesMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    pub range_num: i16,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xede56ae4, bits_len = 32)]
pub struct MoonClmmCreatePoolMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub x_master: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub y_master: Option<TonAddress>,
    pub bin_step: u16,
    pub init_bin: i64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x4e5504a9, bits_len = 32)]
pub struct MoonClmmDeployLpPositionMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub provider: TonAddress,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterInt<64, i64>, DictValAdapterTLB<MoonClmmBin>>::new(64)")]
    pub position_bins: HashMap<i64, MoonClmmBin>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6d33fdcd, bits_len = 32)]
pub struct MoonClmmDeployPoolMsg {
    pub query_id: u64,
    pub init_bin: i64,
    pub lp_fee: u8,
    pub protocol_fee: u8,
    pub payload_code: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x724b6005, bits_len = 32)]
pub struct MoonClmmDeployRangeMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub x_master: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_master: TonAddress,
    pub bin_step: u16,
    pub lp_fee: u8,
    pub protocol_fee: u8,
    pub init_price: TLBRef<MoonClmmRangeInitPrice>,
    pub lp_acc_code: TLBRef<TonCell>,
    pub lp_position_code: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xec401442, bits_len = 32)]
pub struct MoonClmmDepositLiquidityMsg {
    pub query_id: u64,
    pub deposit_amount: TLBCoins,
    pub is_single: bool,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterInt<64, i64>, DictValAdapterTLB<MoonClmmDepositBin>>::new(64)")]
    pub deposit_bins: HashMap<i64, MoonClmmDepositBin>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xed8efa06, bits_len = 32)]
pub struct MoonClmmDepositLiquidityRecordMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub x_in: bool,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterInt<64, i64>, DictValAdapterTLB<MoonClmmDepositBin>>::new(64)")]
    pub deposit_bins: HashMap<i64, MoonClmmDepositBin>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xefef86da, bits_len = 32)]
pub struct MoonClmmInternalCollectProtocolFeesMsg {
    pub query_id: u64,
    pub range_num: i16,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd97a5c35, bits_len = 32)]
pub struct MoonClmmInternalUpdateRangeFeesMsg {
    pub query_id: u64,
    pub range_num: i16,
    pub lp_fee: u8,
    pub protocol_fee: u8,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xe82e11a1, bits_len = 32)]
pub struct MoonClmmInternalUpdateRangeWalletsMsg {
    pub query_id: u64,
    pub range_num: i16,
    #[tlb(adapter = "NullableAddress")]
    pub x_wallet: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_wallet: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xb7f86a0a, bits_len = 32)]
pub struct MoonClmmInternalUpgradePoolMsg {
    pub query_id: u64,
    pub new_code: Option<TLBRef<TonCell>>,
    pub new_data: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7791fc81, bits_len = 32)]
pub struct MoonClmmInternalUpgradeRangeMsg {
    pub query_id: u64,
    pub range_num: i16,
    pub new_code: Option<TLBRef<TonCell>>,
    pub new_data: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x002618d9, bits_len = 32)]
pub struct MoonClmmInternalWithdrawFeesMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub provider: TonAddress,
    pub nft_index: u64,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterInt<64, i64>, DictValAdapterTLB<MoonClmmBin>>::new(64)")]
    pub withdraw_bins: HashMap<i64, MoonClmmBin>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xfe49ce3f, bits_len = 32)]
pub struct MoonClmmInternalWithdrawFeesUpdateMsg {
    pub query_id: u64,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterInt<64, i64>, DictValAdapterTLB<MoonClmmBin>>::new(64)")]
    pub updated_bins: HashMap<i64, MoonClmmBin>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x69099bab, bits_len = 32)]
pub struct MoonClmmInternalWithdrawLiquidityMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub provider: TonAddress,
    pub nft_index: u64,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterInt<64, i64>, DictValAdapterTLB<MoonClmmBin>>::new(64)")]
    pub withdraw_bins: HashMap<i64, MoonClmmBin>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x18256917, bits_len = 32)]
pub struct MoonClmmInternalWithdrawLiquidityUpdateMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x850bdd2a, bits_len = 32)]
pub struct MoonClmmLockLiquidityMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x344629fc, bits_len = 32)]
pub struct MoonClmmProvideLiquidityMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub provider: TonAddress,
    pub x_amount: TLBCoins,
    pub y_amount: TLBCoins,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterInt<64, i64>, DictValAdapterTLB<MoonClmmDepositBin>>::new(64)")]
    pub deposit_bins: HashMap<i64, MoonClmmDepositBin>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x2c76b973, bits_len = 32)]
pub struct MoonClmmProvideWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub request_address: TonAddress,
    pub include_address: bool,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd151210f, bits_len = 32)]
pub struct MoonClmmSwapMsg {
    pub query_id: u64,
    pub swap_params: MoonClmmTonSwapParams,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x9d142643, bits_len = 32)]
pub struct MoonClmmSwapFulfillMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub swap_params: Option<TLBRef<MoonClmmTonSwapParams>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd1735400, bits_len = 32)]
pub struct MoonClmmTakeWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub wallet: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7362d09c, bits_len = 32)]
pub struct MoonClmmTransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    pub forward_payload: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x40073294, bits_len = 32)]
pub struct MoonClmmUpdateRangeFeesMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    pub range_num: i16,
    pub lp_fee: u8,
    pub protocol_fee: u8,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xe55e6dc7, bits_len = 32)]
pub struct MoonClmmUpdateRangeWalletsMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    pub range_num: i16,
    #[tlb(adapter = "NullableAddress")]
    pub x_wallet: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_wallet: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x496938f7, bits_len = 32)]
pub struct MoonClmmUpgradeFactoryMsg {
    pub query_id: u64,
    pub new_code: Option<TLBRef<TonCell>>,
    pub new_data: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x4c76e24e, bits_len = 32)]
pub struct MoonClmmUpgradePoolMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    pub new_code: Option<TLBRef<TonCell>>,
    pub new_data: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x06dbf780, bits_len = 32)]
pub struct MoonClmmUpgradeRangeMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    pub range_num: i16,
    pub new_code: Option<TLBRef<TonCell>>,
    pub new_data: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x716c13d8, bits_len = 32)]
pub struct MoonClmmWithdrawFeesMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc84175bb, bits_len = 32)]
pub struct MoonClmmWithdrawLiquidityMsg {
    pub query_id: u64,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterInt<64, i64>, DictValAdapterTLB<bool>>::new(64)")]
    pub withdraw_bins: HashMap<i64, bool>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmPayloadCode {
    pub range_code: TLBRef<TonCell>,
    pub lp_acc_code: TLBRef<TonCell>,
    pub lp_position_code: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmSwapOut {
    pub total_out: TLBCoins,
    pub new_range_state: MoonClmmRangeState,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterInt<64, i64>, DictValAdapterTLB<MoonClmmBin>>::new(64)")]
    pub new_range_bins: HashMap<i64, MoonClmmBin>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmSwapResultJettonBodyMsg {
    pub op: u32,
    pub payload: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmSwapResultTonBodyMsg {
    pub op: u32,
    pub query_id: u64,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonClmmTonSwapParams {
    pub swap_amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub excess: TonAddress,
    pub sqrt_price_limit: U256,
    pub next_fulfill: Option<TLBRef<TonCell>>,
    pub next_reject: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x5fd5f2b3, bits_len = 32)]
pub struct MoonClmmLiquidityResponseMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x80381933, bits_len = 32)]
pub struct MoonClmmPositionResponseMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x77714776, bits_len = 32)]
pub struct MoonClmmRangeResponseMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x89983597, bits_len = 32)]
pub struct MoonClmmNativeSurplusResponseMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum MoonClmmMsg {
    MoonClmmApplyCollectProtocolFeesMsg(Box<MoonClmmApplyCollectProtocolFeesMsg>),
    MoonClmmApplyUpdateRangeFeesMsg(Box<MoonClmmApplyUpdateRangeFeesMsg>),
    MoonClmmApplyUpdateRangeWalletsMsg(Box<MoonClmmApplyUpdateRangeWalletsMsg>),
    MoonClmmApplyUpgradeRangeMsg(Box<MoonClmmApplyUpgradeRangeMsg>),
    MoonClmmCancelDepositMsg(Box<MoonClmmCancelDepositMsg>),
    MoonClmmCollectProtocolFeesMsg(Box<MoonClmmCollectProtocolFeesMsg>),
    MoonClmmCreatePoolMsg(Box<MoonClmmCreatePoolMsg>),
    MoonClmmDeployLpPositionMsg(Box<MoonClmmDeployLpPositionMsg>),
    MoonClmmDeployPoolMsg(Box<MoonClmmDeployPoolMsg>),
    MoonClmmDeployRangeMsg(Box<MoonClmmDeployRangeMsg>),
    MoonClmmDepositLiquidityMsg(Box<MoonClmmDepositLiquidityMsg>),
    MoonClmmDepositLiquidityRecordMsg(Box<MoonClmmDepositLiquidityRecordMsg>),
    MoonClmmInternalCollectProtocolFeesMsg(Box<MoonClmmInternalCollectProtocolFeesMsg>),
    MoonClmmInternalUpdateRangeFeesMsg(Box<MoonClmmInternalUpdateRangeFeesMsg>),
    MoonClmmInternalUpdateRangeWalletsMsg(Box<MoonClmmInternalUpdateRangeWalletsMsg>),
    MoonClmmInternalUpgradePoolMsg(Box<MoonClmmInternalUpgradePoolMsg>),
    MoonClmmInternalUpgradeRangeMsg(Box<MoonClmmInternalUpgradeRangeMsg>),
    MoonClmmInternalWithdrawFeesMsg(Box<MoonClmmInternalWithdrawFeesMsg>),
    MoonClmmInternalWithdrawFeesUpdateMsg(Box<MoonClmmInternalWithdrawFeesUpdateMsg>),
    MoonClmmInternalWithdrawLiquidityMsg(Box<MoonClmmInternalWithdrawLiquidityMsg>),
    MoonClmmInternalWithdrawLiquidityUpdateMsg(Box<MoonClmmInternalWithdrawLiquidityUpdateMsg>),
    MoonClmmLockLiquidityMsg(Box<MoonClmmLockLiquidityMsg>),
    MoonClmmProvideLiquidityMsg(Box<MoonClmmProvideLiquidityMsg>),
    MoonClmmProvideWalletAddressMsg(Box<MoonClmmProvideWalletAddressMsg>),
    MoonClmmSwapMsg(Box<MoonClmmSwapMsg>),
    MoonClmmSwapFulfillMsg(Box<MoonClmmSwapFulfillMsg>),
    MoonClmmTakeWalletAddressMsg(Box<MoonClmmTakeWalletAddressMsg>),
    MoonClmmTransferNotificationMsg(Box<MoonClmmTransferNotificationMsg>),
    MoonClmmUpdateRangeFeesMsg(Box<MoonClmmUpdateRangeFeesMsg>),
    MoonClmmUpdateRangeWalletsMsg(Box<MoonClmmUpdateRangeWalletsMsg>),
    MoonClmmUpgradeFactoryMsg(Box<MoonClmmUpgradeFactoryMsg>),
    MoonClmmUpgradePoolMsg(Box<MoonClmmUpgradePoolMsg>),
    MoonClmmUpgradeRangeMsg(Box<MoonClmmUpgradeRangeMsg>),
    MoonClmmWithdrawFeesMsg(Box<MoonClmmWithdrawFeesMsg>),
    MoonClmmWithdrawLiquidityMsg(Box<MoonClmmWithdrawLiquidityMsg>),
    MoonClmmLiquidityResponseMsg(Box<MoonClmmLiquidityResponseMsg>),
    MoonClmmPositionResponseMsg(Box<MoonClmmPositionResponseMsg>),
    MoonClmmRangeResponseMsg(Box<MoonClmmRangeResponseMsg>),
    MoonClmmNativeSurplusResponseMsg(Box<MoonClmmNativeSurplusResponseMsg>),
    JettonTransfer(Box<MoonJettonTransferMsg>),
    Unknown(TonCell),
}

// SDK transfer variants with literal payload flags checked on read and write.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
#[tlb(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct MoonClmmJettonTransferMsg {
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
pub struct MoonClmmJettonTransferWithPayloadMsg {
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
