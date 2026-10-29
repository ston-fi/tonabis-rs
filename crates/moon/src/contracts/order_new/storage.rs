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

/// Header installed by the factory before the order accepts its deployment message.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderNewDeploymentStorage {
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub creator_address: TonAddress,
    pub order_id: u64,
    pub status: u8,
}

/// Runtime state used after the factory's deployment message is accepted.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderNewStorage {
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub creator_address: TonAddress,
    pub order_id: u64,
    pub status: u8,
    pub remaining_amount: TLBCoins,
    pub order_fee_bps: u8,
    pub accumulated_fee: TLBCoins,
    pub state_data: TLBRef<TonCell>,
    pub assets: TLBRef<MoonOrderNewAssets>,
    pub wallets: TLBRef<MoonOrderNewWallets>,
    pub funding: Option<TLBRef<MoonOrderNewFunding>>,
}

/// Fixed order terms stored in `stateData` while the order remains fillable.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderNewOrderParams {
    pub rate: U256,
    #[tlb(bits_len = 2)]
    pub lock_mode: u8,
    pub vesting_duration: u64,
}

/// Vesting data stored in `stateData` after a vested fill.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderNewVestingState {
    pub start_time: u64,
    pub duration: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
}

/// Asset master identifiers retained by the order.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderNewAssets {
    #[tlb(adapter = "NullableAddress")]
    pub asset1: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub asset2: Option<TonAddress>,
}

/// Asset wallet addresses retained by the order.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderNewWallets {
    #[tlb(adapter = "NullableAddress")]
    pub asset1_wallet: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub asset2_wallet: Option<TonAddress>,
}

/// Optional Jetton funding record supplied by the factory.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderNewFunding {
    #[tlb(adapter = "InternalAddress")]
    pub source_wallet: TonAddress,
    pub amount: TLBCoins,
}

/// Structured result returned by `get_vesting_data` after a vested fill.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderNewVestingDataReply {
    pub vesting_start_time: u64,
    pub vesting_duration: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
}
