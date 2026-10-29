//! CoffeeCrossDexBidask wrapper; callers own the client and code provenance.
use crate::contracts::cross_dex_bidask::storage::CoffeeCrossDexBidaskDeploymentStorage;
use crate::contracts::cross_dex_bidask::storage::CoffeeCrossDexBidaskStorage;
use ton::block_tlb::TVMStack;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::MsgAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
ton::ton_contract! { CoffeeCrossDexBidaskContract<CoffeeCrossDexBidaskStorage> }
#[ton::ton_core::ton_methods]
impl CoffeeCrossDexBidaskContract {
    pub async fn get_admin(&self) -> TonResult<Option<TonAddress>> {
        let stack = TVMStack::default();
        let value: MsgAddress = self.emulate_get_method("get_admin", &stack).await?;
        NullableInternalAddress::from_address(value).map_err(Into::into)
    }
}
ton::ton_contract! { CoffeeCrossDexBidaskDeploymentContract< CoffeeCrossDexBidaskDeploymentStorage> }
#[ton::ton_core::ton_methods]
impl CoffeeCrossDexBidaskDeploymentContract {
    pub async fn get_admin(&self) -> TonResult<Option<TonAddress>> {
        let stack = TVMStack::default();
        let value: MsgAddress = self.emulate_get_method("get_admin", &stack).await?;
        NullableInternalAddress::from_address(value).map_err(Into::into)
    }
}
pub mod msgs;
pub mod storage;

/// Exit codes declared by the pinned interface; provider errors retain their type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CoffeeCrossDexBidaskErrors {
    InvalidPayloadToResend = 100,
    DedustPayloadIsEmpty = 101,
    PtonPayloadIsEmpty = 102,
    BidaskPayloadIsEmpty = 103,
    NotAnAdmin = 200,
    WrongOperation = 65535,
}
