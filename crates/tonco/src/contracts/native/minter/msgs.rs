//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x2c76b973, bits_len = 32)]
pub struct ToncoNativeMinterProvideWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
    pub include_owner: bool,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd1735400, bits_len = 32)]
pub struct ToncoNativeMinterTakeWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub wallet: Option<TonAddress>,
    pub owner: Option<TLBRef<ToncoNativeMinterOwner>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoNativeMinterOwner {
    #[tlb(adapter = "NullableAddress")]
    pub address: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x4f5f4313, bits_len = 32)]
pub struct ToncoNativeMinterDeployWalletMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub excess: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoNativeMinterIncomingMessage {
    ProvideWalletAddress(Box<ToncoNativeMinterProvideWalletAddressMsg>),
    DeployWallet(Box<ToncoNativeMinterDeployWalletMsg>),
    Unknown(TonCell),
}
