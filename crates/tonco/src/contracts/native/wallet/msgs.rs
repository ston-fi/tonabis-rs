//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use crate::contracts::v2::router::msgs::ToncoRouterV2TransferNotificationMsg;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBEither;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::ton_core::types::tlb_core::{TLBCoins, TLBRef};

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x01f3835d, bits_len = 32)]
pub struct ToncoNativeWalletTonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub refund: Option<TonAddress>,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x29d22935, bits_len = 32)]
pub struct ToncoNativeWalletResetGasMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct ToncoNativeWalletTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub destination: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_amount: TLBCoins,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xb1d8b6c3, bits_len = 32)]
pub struct ToncoNativeWalletJettonTransferMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub jetton_wallet: Option<TonAddress>,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub recipient: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6540cf85, bits_len = 32)]
pub struct ToncoNativeWalletInternalDeployMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub excess: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct ToncoNativeWalletExcessesMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xae25d79e, bits_len = 32)]
pub struct ToncoNativeWalletTonRefundMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoNativeWalletV1Message {
    Transfer(Box<ToncoNativeWalletTransferMsg>),
    TonTransfer(Box<ToncoNativeWalletTonTransferMsg>),
    InternalDeploy(Box<ToncoNativeWalletInternalDeployMsg>),
    ResetGas(Box<ToncoNativeWalletResetGasMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoNativeWalletV2Message {
    Transfer(Box<ToncoNativeWalletTransferMsg>),
    TonTransfer(Box<ToncoNativeWalletTonTransferMsg>),
    InternalDeploy(Box<ToncoNativeWalletInternalDeployMsg>),
    ResetGas(Box<ToncoNativeWalletResetGasMsg>),
    JettonTransfer(Box<ToncoNativeWalletJettonTransferMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoNativeWalletOutgoingMessage {
    TonTransfer(Box<ToncoNativeWalletTonTransferMsg>),
    Transfer(Box<ToncoNativeWalletTransferMsg>),
    TransferNotification(Box<ToncoRouterV2TransferNotificationMsg>),
    Excesses(Box<ToncoNativeWalletExcessesMsg>),
    Refund(Box<ToncoNativeWalletTonRefundMsg>),
    Unknown(TonCell),
}
