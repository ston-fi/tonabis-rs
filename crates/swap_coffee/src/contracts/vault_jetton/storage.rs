//! Generated Coffee storage. See tools/reference/swap_coffee/README.md.
use crate::contracts::vault_native::storage::CoffeeVaultInitData;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use tonabis_core::adapters::expected_bool::ExpectedBool;
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeVaultJettonActiveStorage {
    pub init_code: TLBRef<TonCell>,
    pub init_data: TLBRef<CoffeeVaultInitData>,
    #[tlb(adapter = "ExpectedBool::<true>")]
    pub is_active: bool,
    #[tlb(adapter = "InternalAddress")]
    pub jetton_wallet_address: TonAddress,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeVaultJettonInactiveStorage {
    pub init_code: TLBRef<TonCell>,
    pub init_data: TLBRef<CoffeeVaultInitData>,
    #[tlb(adapter = "ExpectedBool::<false>")]
    pub is_active: bool,
}
