//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use fastnum::U256;
use ton::{
    contracts::tep::snake_data::SnakeData,
    ton_core::{
        TLB,
        types::{
            TonAddress,
            tlb_core::{TLBCoins, TLBRef, adapters::InternalAddress},
        },
    },
};
/// Metadata URI referenced by the immutable Meme configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV2Metadata {
    pub uri: TLBRef<SnakeData>,
}
/// Affiliate partner configuration carried by initialization and trades.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV2PartnerConfig {
    pub partner_id: U256,
    /// Partner fee share in basis points.
    pub partner_fee_bps: u16,
}
/// Affiliate referrer configuration carried by initialization and trades.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV2ReferrerConfig {
    pub referrer_id: U256,
    /// Referrer fee share in basis points.
    pub referrer_fee_bps: u16,
}
/// Referenced immutable configuration of one Uranus Meme V2 instance.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV2Config {
    #[tlb(adapter = "InternalAddress")]
    pub creator_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub controller_address: TonAddress,
    pub metadata: DedustUranusMemeV2Metadata,
    pub raising_funds: TLBCoins,
    pub liquidity_tokens: TLBCoins,
    pub seed: u128,
}
/// Mutable bonding-curve state stored in the Meme V2 root cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV2BondingCurve {
    pub graduated: bool,
    pub alpha: TLBCoins,
    pub beta: TLBCoins,
    pub on_sell_supply: TLBCoins,
    pub raised_funds: TLBCoins,
    pub current_supply: TLBCoins,
}
/// Complete runtime storage of the Uranus Meme V2 revision.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV2Storage {
    pub initialized: bool,
    pub migrated: bool,
    pub config: TLBRef<DedustUranusMemeV2Config>,
    /// Bonding-curve base trading fee in basis points.
    pub base_fee_bps: u16,
    pub curve: DedustUranusMemeV2BondingCurve,
    pub total_supply: TLBCoins,
    pub creator_fee: TLBCoins,
}
/// Fee breakdown emitted by Uranus Meme V2 trade events.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV2TradeFees {
    pub creator_fee: TLBCoins,
    pub protocol_fee: TLBCoins,
    pub partner_fee: TLBCoins,
    pub referrer_fee: TLBCoins,
}
/// Exit codes used by the Uranus Meme V2 implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DedustUranusMemeV2Errors {
    /// Output amount is below the supplied slippage limit.
    SlippageExceeded = 21,
    /// Attached GRAM value is not enough for the requested operation.
    MessageValueTooLow = 23,
    /// Meme state was already initialized.
    AlreadyInitialized = 24,
    /// Sender is not authorized for the operation.
    SenderUnauthorized = 25,
    /// Meme state has not been initialized yet.
    NotInitialized = 28,
    /// Meme already graduated and no longer accepts curve trading.
    AlreadyGraduated = 29,
    /// Required migration-state data is absent or invalid.
    MigrationStateRequired = 32,
    /// Incoming message body tag is unknown.
    UnknownOperation = 65535,
}
