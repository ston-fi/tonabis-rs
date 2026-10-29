//! dedust_factory msgs records.
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
/// Current DeDust factory creation extends the legacy body with a second wallet
/// and a 16-bit configured fee. The body is incompatible with the legacy decoder.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a9801, bits_len = 32)]
pub struct StonksDedustFactoryCreateVirtualLiquidityJettonMsg {
    /// Caller query id.
    pub query_id: u64,
    /// Referenced metadata.
    pub content: TLBRef<TonCell>,
    /// Optional developer buy TON.
    pub dev_buy_ton_amount: TLBCoins,
    /// Per-buy percent cap.
    pub max_buy_percent: u16,
    /// Recipient of the developer buy.
    #[tlb(adapter = "NullableAddress")]
    pub dev_buy_wallet: Option<TonAddress>,
    /// Creator fee recipient.
    #[tlb(adapter = "NullableAddress")]
    pub creator_wallet: Option<TonAddress>,
    /// Additional presale creator fee; factory derives destination config from it.
    pub creator_presale_bps: u16,
}
