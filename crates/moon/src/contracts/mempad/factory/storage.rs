//! Generated Moon wire records. See tools/reference/moon/README.md.
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadEconBase {
    pub tokens_for_sale: TLBCoins,
    pub tokens_for_lp: TLBCoins,
    pub virtual_gram: TLBCoins,
    pub virtual_tokens: TLBCoins,
    pub target_gram: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadFactoryConfig {
    #[tlb(adapter = "InternalAddress")]
    pub cpmm_factory: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub protocol_recipient: TonAddress,
    pub econ: TLBRef<MoonMempadEconBase>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadFactoryStorage {
    #[tlb(adapter = "InternalAddress")]
    pub admin: TonAddress,
    pub launch_count: u64,
    pub launch_fee: TLBCoins,
    pub config: TLBRef<MoonMempadFactoryConfig>,
    pub curve_code: TLBRef<TonCell>,
    pub jw_code: TLBRef<TonCell>,
}
