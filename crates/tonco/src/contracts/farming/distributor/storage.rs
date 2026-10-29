//! Persistent TL-B records and local caller-code StateInit construction. No deployment is performed.
use std::collections::HashMap;
use ton::block_tlb::StateInit;
use ton::tlb_adapters::DictKeyAdapterUint;
use ton::tlb_adapters::DictValAdapterTLB;
use ton::tlb_adapters::TLBHashMapE;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::TLBRef;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoDistributorStorage {
    #[tlb(adapter = "NullableAddress")]
    pub admin: Option<TonAddress>,
    pub user_distributor_code: TLBRef<TonCell>,
    pub merkle_roots: TLBRef<ToncoFarmingDistributorMerkleRoots>,
}

impl ToncoDistributorStorage {
    /// Encode this exact revision's data with caller-supplied code. Does not deploy.
    pub fn state_init(&self, code: TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(code, ton::ton_core::traits::tlb::TLB::to_cell(self)?))
    }
    /// Hash the standard state-init; caller owns code identity and workchain policy.
    pub fn derive_address(&self, code: TonCell, workchain: i32) -> TonCoreResult<TonAddress> {
        self.state_init(code)?.derive_address(workchain)
    }
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoFarmingDistributorMerkleRoots {
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u32>, DictValAdapterTLB<_>>::new(32)")]
    pub roots: HashMap<u32, U256>,
}
