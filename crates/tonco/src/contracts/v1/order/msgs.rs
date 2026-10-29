//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use crate::contracts::native::minter::msgs::ToncoNativeMinterProvideWalletAddressMsg;
use crate::contracts::native::minter::msgs::ToncoNativeMinterTakeWalletAddressMsg;
use crate::contracts::v1::factory::msgs::ToncoFactoryV1ExecuteOrderMsg;
use crate::contracts::v1::factory::msgs::ToncoFactoryV1InitOrderMsg;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoOrderV1IncomingMessage {
    Init(Box<ToncoFactoryV1InitOrderMsg>),
    TakeWalletAddress(Box<ToncoNativeMinterTakeWalletAddressMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoOrderV1OutgoingMessage {
    ProvideWalletAddress(Box<ToncoNativeMinterProvideWalletAddressMsg>),
    ExecuteOrder(Box<ToncoFactoryV1ExecuteOrderMsg>),
    Unknown(TonCell),
}
