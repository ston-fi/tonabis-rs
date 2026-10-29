//! ston_factory storage records.
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
/// STON factory templates and router configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksStonFactoryStorage {
    pub virtual_minter_code: TLBRef<TonCell>,
    pub jetton_wallet_code: TLBRef<TonCell>,
    #[tlb(adapter = "NullableAddress")]
    pub router_address: Option<TonAddress>,
    pub disabled: bool,
}
