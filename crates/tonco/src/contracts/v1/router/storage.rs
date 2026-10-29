//! Persistent TL-B records and local caller-code StateInit construction. No deployment is performed.
use crate::contracts::v2::router::storage::ToncoRouterV2OptionalU64;
use ton::block_tlb::StateInit;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV1Storage {
    #[tlb(adapter = "NullableAddress")]
    pub admin_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub pool_admin_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub pool_factory_address: Option<TonAddress>,
    pub flags: u64,
    pub pool_seqno: u64,
    pub child_contracts: TLBRef<ToncoRouterV1ChildContracts>,
    pub timelocks: TLBRef<ToncoRouterV1Timelocks>,
    pub nonce: ToncoRouterV2OptionalU64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV1ChildContracts {
    pub poolv3_code: TLBRef<TonCell>,
    pub accountv3_code: TLBRef<TonCell>,
    pub position_nftv3_code: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV1Timelocks {
    pub delay: u64,
    pub code_update: Option<TLBRef<ToncoRouterV1CodeUpdate>>,
    pub address_update: Option<TLBRef<ToncoRouterV1AddressUpdate>>,
    pub flags_update: Option<TLBRef<ToncoRouterV1FlagsUpdate>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV1CodeUpdate {
    pub code: TLBRef<TonCell>,
    pub unlock_time: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV1AddressUpdate {
    #[tlb(adapter = "NullableAddress")]
    pub address: Option<TonAddress>,
    pub unlock_time: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV1FlagsUpdate {
    pub flags: u64,
    pub unlock_time: u64,
}

impl ToncoRouterV1Storage {
    /// Construct data and StateInit with explicit caller-selected code; sends nothing.
    pub fn state_init(&self, code: TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(code, self.to_cell()?))
    }
    /// Caller owns code identity and workchain policy.
    pub fn derive_address(&self, code: TonCell, workchain: i32) -> TonCoreResult<TonAddress> {
        self.state_init(code)?.derive_address(workchain)
    }
}
