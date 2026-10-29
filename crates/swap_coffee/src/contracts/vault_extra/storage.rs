//! Generated Coffee storage. See tools/reference/swap_coffee/README.md.
use crate::contracts::vault_native::storage::CoffeeVaultInitData;
use crate::contracts::wire::TrailingSelector;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::TLBRef;
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeVaultExtraStorage {
    pub init_code: TLBRef<TonCell>,
    pub init_data: TLBRef<CoffeeVaultInitData>,
    #[tlb(adapter = "TrailingSelector")]
    pub factory_initialization_marker: Option<bool>,
}
