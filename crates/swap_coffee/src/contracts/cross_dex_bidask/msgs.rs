//! Generated Coffee msgs. See tools/reference/swap_coffee/README.md.
use crate::contracts::cross_dex::msgs::CoffeeCrossDexJettonTransferNotificationMsg;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::{InternalAddress, NullableAddress};
use ton::ton_core::types::tlb_core::{TLBCoins, TLBEither, TLBRef};
use tonabis_core::msgs::EmptyBodyMsg;
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x474f86cf, bits_len = 32)]
pub struct CoffeeCrossDexBidaskDedustPayoutMsg {
    pub query_id: u64,
    pub payload: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x01f3835d, bits_len = 32)]
pub struct CoffeeCrossDexBidaskPtonTonTransferMsg {
    pub query_id: u64,
    pub ton_amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub refund_address: Option<TonAddress>,
    pub forward_payload: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee36, bits_len = 32)]
pub struct CoffeeCrossDexBidaskCoffeeNotificationMsg {
    pub query_id: u64,
    pub body: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6edd65f0, bits_len = 32)]
pub struct CoffeeCrossDexBidaskBidaskNativeTransferNotificationMsg {
    pub query_id: u64,
    pub native_amount: TLBCoins,
    pub forward_payload: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x200f9086, bits_len = 32)]
pub struct CoffeeCrossDexBidaskResendMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub next_recipient: TonAddress,
    pub next: TLBRef<TonCell>,
    pub total_gas: TLBCoins,
    pub next_gas: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x4ee9b106, bits_len = 32)]
pub struct CoffeeCrossDexBidaskResendJettonPayloadMsg {
    pub next: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xb902e61a, bits_len = 32)]
pub struct CoffeeCrossDexBidaskFailurePayloadMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x57861aa7, bits_len = 32)]
pub struct CoffeeCrossDexBidaskSetAdminMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub admin_address: Option<TonAddress>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6f80872c, bits_len = 32)]
pub struct CoffeeCrossDexBidaskUpdateContractMsg {
    pub query_id: u64,
    pub code: Option<TLBRef<TonCell>>,
    pub data: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x2bf71be6, bits_len = 32)]
pub struct CoffeeCrossDexBidaskWithdrawTonMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    pub amount: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7b4b82fe, bits_len = 32)]
pub struct CoffeeCrossDexBidaskWithdrawJettonsMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub jetton_wallet: TonAddress,
    pub amount: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct CoffeeCrossDexBidaskExcessesMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct CoffeeCrossDexBidaskJettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub destination: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeCrossDexBidaskIncomingMessage {
    JettonTransferNotification(CoffeeCrossDexJettonTransferNotificationMsg),
    DedustPayout(CoffeeCrossDexBidaskDedustPayoutMsg),
    PtonTonTransfer(CoffeeCrossDexBidaskPtonTonTransferMsg),
    CoffeeNotification(CoffeeCrossDexBidaskCoffeeNotificationMsg),
    BidaskNativeTransferNotification(CoffeeCrossDexBidaskBidaskNativeTransferNotificationMsg),
    Resend(CoffeeCrossDexBidaskResendMsg),
    SetAdmin(CoffeeCrossDexBidaskSetAdminMsg),
    UpdateContract(CoffeeCrossDexBidaskUpdateContractMsg),
    WithdrawTon(CoffeeCrossDexBidaskWithdrawTonMsg),
    WithdrawJettons(CoffeeCrossDexBidaskWithdrawJettonsMsg),
    Excesses(CoffeeCrossDexBidaskExcessesMsg),
    Empty(EmptyBodyMsg),
    Unknown(TonCell),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
// Keep opcode prefixes visible: Box<T> erases them in locked TON dispatch.
#[allow(clippy::large_enum_variant)]
pub enum CoffeeCrossDexBidaskOutgoingMessage {
    JettonTransfer(CoffeeCrossDexBidaskJettonTransferMsg),
    Excesses(CoffeeCrossDexBidaskExcessesMsg),
    Unknown(TonCell),
}
