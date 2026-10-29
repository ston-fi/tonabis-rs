//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use crate::contracts::native::minter::msgs::ToncoNativeMinterProvideWalletAddressMsg;
use crate::contracts::native::minter::msgs::ToncoNativeMinterTakeWalletAddressMsg;
use crate::contracts::v2::factory::msgs::ToncoFactoryV2ExecuteOrderMsg;
use crate::contracts::v2::factory::msgs::ToncoFactoryV2InitOrderMsg;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoOrderV2IncomingMessage {
    Init(Box<ToncoFactoryV2InitOrderMsg>),
    TakeWalletAddress(Box<ToncoNativeMinterTakeWalletAddressMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoOrderV2OutgoingMessage {
    ProvideWalletAddress(Box<ToncoNativeMinterProvideWalletAddressMsg>),
    ExecuteOrder(Box<ToncoFactoryV2ExecuteOrderMsg>),
    Unknown(TonCell),
}
