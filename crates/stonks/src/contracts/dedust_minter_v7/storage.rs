//! dedust_minter_v7 storage records.
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
/// Exact DeDust v7/v12 launch state stored behind the curve's first reference.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksDedustMinterV7LaunchState {
    /// Migration has been requested.
    pub active: bool,
    /// Minter has emitted destination funding; settlement is not proven.
    pub completed: bool,
    /// Query id of the pending migration.
    pub query_id: u64,
    /// Wallet authenticated by the migration callback.
    #[tlb(adapter = "NullableAddress")]
    pub wallet: Option<TonAddress>,
    /// Physical TON committed to graduation.
    pub ton_amount: TLBCoins,
    /// Unsold physical tokens committed to graduation.
    pub token_amount: TLBCoins,
}

/// Legacy migration configuration, distinct from the current v12 layout.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksDedustMinterV7MigrationConfig {
    /// Referenced executable destination code template.
    pub destination_code: TLBRef<TonCell>,
    /// Referenced destination data template.
    pub destination_data: TLBRef<TonCell>,
    /// Destination creator share, denominator10000.
    pub creator_fee_bps: u16,
    /// Source-defined fee recipient.
    #[tlb(adapter = "NullableAddress")]
    pub dev_wallet: Option<TonAddress>,
}
