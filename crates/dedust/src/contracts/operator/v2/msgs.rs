//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
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
/// Command from the operator's user: the body is forwarded verbatim to the destination.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 4053307989, bits_len = 32)]
pub struct DedustOperatorV2ForwardMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    pub body: TLBRef<TonCell>,
}
/// Factory command replacing the user allowed to forward through the operator.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1223010653, bits_len = 32)]
pub struct DedustOperatorV2SetUserAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub user_address: TonAddress,
}
/// Fee collection forwarded to a pool.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 188915538, bits_len = 32)]
pub struct DedustOperatorV2CollectFeesMsg {
    pub query_id: u64,
    pub asset0_amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub asset0_destination: TonAddress,
    pub asset0_payload: Option<TLBRef<TonCell>>,
    pub asset1_amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub asset1_destination: Option<TonAddress>,
    pub asset1_payload: Option<TLBRef<TonCell>>,
}
/// Trading start time forwarded to a pool.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2128082638, bits_len = 32)]
pub struct DedustOperatorV2ConfigureStartTimeMsg {
    pub query_id: u64,
    pub start_at: u32,
}
/// Factory-authorized implementation upgrade found in captured dispatch.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 444, bits_len = 32)]
pub struct DedustOperatorV2UpgradeMsg {
    pub query_id: u64,
    pub version: u16,
    pub code: TLBRef<TonCell>,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustOperatorV2IncomingMsg {
    Forward(Box<DedustOperatorV2ForwardMsg>),
    SetUserAddress(Box<DedustOperatorV2SetUserAddressMsg>),
    Upgrade(Box<DedustOperatorV2UpgradeMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustOperatorV2OutgoingMsg {
    CollectFees(Box<DedustOperatorV2CollectFeesMsg>),
    ConfigureStartTime(Box<DedustOperatorV2ConfigureStartTimeMsg>),
    Unknown(TonCell),
}
