//! dedust_minter_v12 storage records.
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
/// Migration configuration as stored by current DeDust. Referenced destination
/// state/code remain cells owned by the destination protocol.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksDedustMinterV12MigrationConfig {
    /// Executable destination code template.
    pub destination_code: TLBRef<TonCell>,
    /// Destination data template.
    pub destination_data: TLBRef<TonCell>,
    /// Creator share of post-graduation pool fees.
    pub creator_fee_bps: u16,
    /// Creator recipient.
    #[tlb(adapter = "NullableAddress")]
    pub dev_wallet: Option<TonAddress>,
    /// Additional creator presale fee, before the base 40 bps.
    pub creator_presale_bps: u16,
}
