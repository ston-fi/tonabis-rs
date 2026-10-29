//! dedust_factory storage records.
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::TLBRef;
/// DeDust factory templates and referenced DEX configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksDedustFactoryStorage {
    pub virtual_minter_code: TLBRef<TonCell>,
    pub jetton_wallet_code: TLBRef<TonCell>,
    pub dex_config: TLBRef<TonCell>,
    pub disabled: bool,
}
