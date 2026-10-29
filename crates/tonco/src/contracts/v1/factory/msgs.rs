//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use crate::contracts::native::wallet::msgs::ToncoNativeWalletExcessesMsg;
use crate::contracts::v1::router::msgs::ToncoRouterV1CreatePoolMsg;
use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoFactoryV1Wallets {
    #[tlb(adapter = "NullableAddress")]
    pub token0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub token1: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x9e9a8f7f, bits_len = 32)]
pub struct ToncoFactoryV1CreatePoolMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub minter0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub minter1: Option<TonAddress>,
    #[tlb(bits_len = 160)]
    pub price: U256,
    pub settings: u16,
    pub wallets: TLBRef<ToncoFactoryV1Wallets>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xaa0db3d7, bits_len = 32)]
pub struct ToncoFactoryV1ChangeParamsMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub router: Option<TonAddress>,
    pub collection_content: TLBRef<TonCell>,
    pub item_content: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x42a0fb43, bits_len = 32)]
pub struct ToncoFactoryV1ResetGasMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoFactoryV1IncomingMessage {
    ChangeAdmin(Box<ToncoFactoryV1ChangeAdminMsg>),
    Excesses(Box<ToncoNativeWalletExcessesMsg>),
    ExecuteOrder(Box<ToncoFactoryV1ExecuteOrderMsg>),
    CreatePool(Box<ToncoFactoryV1CreatePoolMsg>),
    ChangeParams(Box<ToncoFactoryV1ChangeParamsMsg>),
    ResetGas(Box<ToncoFactoryV1ResetGasMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd4b33b0a, bits_len = 32)]
pub struct ToncoFactoryV1ExecuteOrderMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub minter0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub minter1: Option<TonAddress>,
    #[tlb(bits_len = 160)]
    pub price: U256,
    pub settings: u16,
    pub wallets: TLBRef<ToncoFactoryV1Wallets>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoFactoryV1Minters {
    #[tlb(adapter = "NullableAddress")]
    pub token0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub token1: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x8b85de63, bits_len = 32)]
pub struct ToncoFactoryV1InitOrderMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub router: Option<TonAddress>,
    #[tlb(bits_len = 160)]
    pub price: U256,
    pub settings: u16,
    pub minters: TLBRef<ToncoFactoryV1Minters>,
}

/// Executable factory admin transfer, absent from the SDK's opcode declarations.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x246189c4, bits_len = 32)]
pub struct ToncoFactoryV1ChangeAdminMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub admin: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoFactoryV1OutgoingMessage {
    InitOrder(Box<ToncoFactoryV1InitOrderMsg>),
    CreatePool(Box<ToncoRouterV1CreatePoolMsg>),
    Unknown(TonCell),
}
