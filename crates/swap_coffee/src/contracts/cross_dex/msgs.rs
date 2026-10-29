//! Generated Coffee msgs. See tools/reference/swap_coffee/README.md.
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::{InternalAddress, NullableAddress};
use ton::ton_core::types::tlb_core::{TLBCoins, TLBEither, TLBRef};
use tonabis_core::msgs::EmptyBodyMsg;
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7362d09c, bits_len = 32)]
pub struct CoffeeCrossDexJettonTransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub sender: Option<TonAddress>,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x474f86cf, bits_len = 32)]
pub struct CoffeeCrossDexDedustPayoutMsg {
    pub query_id: u64,
    pub payload: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x01f3835d, bits_len = 32)]
pub struct CoffeeCrossDexPtonTonTransferMsg {
    pub query_id: u64,
    pub ton_amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub refund_address: Option<TonAddress>,
    pub forward_payload: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee36, bits_len = 32)]
pub struct CoffeeCrossDexCoffeeNotificationMsg {
    pub query_id: u64,
    pub body: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6edd65f0, bits_len = 32)]
pub struct CoffeeCrossDexBidaskNativeTransferNotificationMsg {
    pub query_id: u64,
    pub native_amount: TLBCoins,
    pub forward_payload: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x9d142643, bits_len = 32)]
pub struct CoffeeCrossDexWrappedPayloadMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x200f9086, bits_len = 32)]
pub struct CoffeeCrossDexResendMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub next_recipient: TonAddress,
    pub next: TLBRef<TonCell>,
    pub total_gas: TLBCoins,
    pub next_gas: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x4ee9b106, bits_len = 32)]
pub struct CoffeeCrossDexResendJettonPayloadMsg {
    pub next: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xb902e61a, bits_len = 32)]
pub struct CoffeeCrossDexFailurePayloadMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x57861aa7, bits_len = 32)]
pub struct CoffeeCrossDexSetAdminMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub admin_address: Option<TonAddress>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6f80872c, bits_len = 32)]
pub struct CoffeeCrossDexUpdateContractMsg {
    pub query_id: u64,
    pub code: Option<TLBRef<TonCell>>,
    pub data: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct CoffeeCrossDexExcessesMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct CoffeeCrossDexJettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x2bf71be6, bits_len = 32)]
pub struct CoffeeCrossDexNativeRescueMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    pub amount: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7b4b82fe, bits_len = 32)]
pub struct CoffeeCrossDexJettonRescueMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub jetton_wallet: TonAddress,
    pub amount: TLBCoins,
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeCrossDexIncomingMessage {
    JettonTransferNotification(CoffeeCrossDexJettonTransferNotificationMsg),
    DedustPayout(CoffeeCrossDexDedustPayoutMsg),
    PtonTonTransfer(CoffeeCrossDexPtonTonTransferMsg),
    CoffeeNotification(CoffeeCrossDexCoffeeNotificationMsg),
    BidaskNativeTransferNotification(CoffeeCrossDexBidaskNativeTransferNotificationMsg),
    WrappedPayload(CoffeeCrossDexWrappedPayloadMsg),
    Resend(CoffeeCrossDexResendMsg),
    SetAdmin(CoffeeCrossDexSetAdminMsg),
    UpdateContract(CoffeeCrossDexUpdateContractMsg),
    NativeRescue(CoffeeCrossDexNativeRescueMsg),
    JettonRescue(CoffeeCrossDexJettonRescueMsg),
    Excesses(CoffeeCrossDexExcessesMsg),
    Empty(EmptyBodyMsg),
    Unknown(TonCell),
}
/// Typed dispatch; unknown bodies retain bits and references.
#[derive(Debug, Clone, PartialEq, TLB)]
// Keep opcode prefixes visible: Box<T> erases them in locked TON dispatch.
#[allow(clippy::large_enum_variant)]
pub enum CoffeeCrossDexOutgoingMessage {
    JettonTransfer(CoffeeCrossDexJettonTransferMsg),
    Excesses(CoffeeCrossDexExcessesMsg),
    Unknown(TonCell),
}
