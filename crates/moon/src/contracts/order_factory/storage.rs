//! Generated Moon wire records. See tools/reference/moon/README.md.
use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

/// Persistent factory state recovered from the exact deployed code and live data cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderFactoryStorage {
    #[tlb(adapter = "InternalAddress")]
    pub admin_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub pipe_deployment_authority: TonAddress,
    pub last_order_id: u64,
    pub order_fee_bps: u8,
    pub pipe_code: TLBRef<TonCell>,
    pub order_code: TLBRef<TonCell>,
}

/// Immutable price, lock mode, and vesting duration supplied when an order is created.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderFactoryOrderParams {
    pub rate: U256,
    #[tlb(bits_len = 2)]
    pub lock_mode: u8,
    pub vesting_duration: u64,
}

/// Asset master identifiers stored in the deployed order.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderFactoryAssets {
    #[tlb(adapter = "NullableAddress")]
    pub asset1: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub asset2: Option<TonAddress>,
}

/// Asset wallet addresses stored in the deployed order.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderFactoryWallets {
    #[tlb(adapter = "NullableAddress")]
    pub asset1_wallet: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub asset2_wallet: Option<TonAddress>,
}

/// Optional Jetton funding record attached to a Jetton-backed order deployment.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderFactoryFunding {
    #[tlb(adapter = "InternalAddress")]
    pub source_wallet: TonAddress,
    pub amount: TLBCoins,
}

/// Replacement factory configuration installed together with upgraded code.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderFactoryUpgradeStorage {
    #[tlb(adapter = "InternalAddress")]
    pub admin_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub pipe_deployment_authority: TonAddress,
    pub order_fee_bps: u8,
    pub pipe_code: TLBRef<TonCell>,
    pub order_code: TLBRef<TonCell>,
}
