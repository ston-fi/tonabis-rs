//! Generated Coffee storage. See tools/reference/swap_coffee/README.md.
use std::collections::HashMap;
use ton::tlb_adapters::{DictKeyAdapterUint, DictValAdapterTLB, TLBHashMapE};
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeFactoryStorage {
    #[tlb(adapter = "NullableAddress")]
    pub admin_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub withdrawer_address: Option<TonAddress>,
    pub code_cells: TLBRef<CoffeeFactoryCodeCells>,
    pub code_dictionaries: TLBRef<CoffeeFactoryCodeDictionaries>,
    pub compatibility_tail: TonCell,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeFactoryCodeCells {
    pub lp_wallet_code: TLBRef<TonCell>,
    pub init_code: TLBRef<TonCell>,
    pub liquidity_depository_code: TLBRef<TonCell>,
    pub pool_creator_code: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeFactoryCodeDictionaries {
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u8>, DictValAdapterTLB<TLBRef<TonCell>>>::new(2)")]
    pub vault_code_by_asset_type: HashMap<u8, TLBRef<TonCell>>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u8>, DictValAdapterTLB<TLBRef<TonCell>>>::new(3)")]
    pub pool_code_by_amm: HashMap<u8, TLBRef<TonCell>>,
}
