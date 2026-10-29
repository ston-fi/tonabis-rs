//! Persistent TL-B records and local caller-code StateInit construction. No deployment is performed.
use crate::contracts::v2::router::storage::ToncoRouterV2OptionalU64;
use std::collections::HashMap;
use ton::block_tlb::StateInit;
use ton::tlb_adapters::DictKeyAdapterTonHash;
use ton::tlb_adapters::DictValAdapterTLB;
use ton::tlb_adapters::TLBHashMapE;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::cell::TonHash;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::ton_core::types::tlb_core::{TLBCoins, TLBRef};

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoFactoryV2WhitelistEntry {
    #[tlb(adapter = "NullableAddress")]
    pub address: Option<TonAddress>,
    pub allow_controller: bool,
    pub allow_creator: bool,
    pub allow_active: bool,
    pub allow_settings: bool,
    pub allow_content: bool,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoFactoryV2Storage {
    #[tlb(adapter = "NullableAddress")]
    pub admin: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub router: Option<TonAddress>,
    pub ton_price: TLBCoins,
    pub order_code: TLBRef<TonCell>,
    pub collection: TLBRef<TonCell>,
    pub item: TLBRef<TonCell>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB<_>>::new(255)")]
    pub whitelist: HashMap<TonHash, TLBRef<ToncoFactoryV2WhitelistEntry>>,
    pub state_nonce: ToncoRouterV2OptionalU64,
}

impl ToncoFactoryV2Storage {
    /// Encode this exact revision's data with caller-supplied code. Does not deploy.
    pub fn state_init(&self, code: TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(code, ton::ton_core::traits::tlb::TLB::to_cell(self)?))
    }
    /// Hash the standard state-init; caller owns code identity and workchain policy.
    pub fn derive_address(&self, code: TonCell, workchain: i32) -> TonCoreResult<TonAddress> {
        self.state_init(code)?.derive_address(workchain)
    }
}
