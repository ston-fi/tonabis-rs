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
/// Metadata URI wrapper used by Uranus meme config and deploy messages.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV3Metadata {
    pub uri: TLBRef<SnakeData>,
}
/// Affiliate partner configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV3PartnerConfig {
    pub partner_id: U256,
    /// Partner fee share in basis points.
    pub partner_fee_bps: u16,
}
/// Affiliate referrer configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV3ReferrerConfig {
    pub referrer_id: U256,
    /// Referrer fee share in basis points.
    pub referrer_fee_bps: u16,
}
/// Referenced immutable meme configuration.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV3Config {
    #[tlb(adapter = "InternalAddress")]
    pub creator_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub controller_address: TonAddress,
    pub metadata: DedustUranusMemeV3Metadata,
    pub raising_funds: TLBCoins,
    pub liquidity_tokens: TLBCoins,
    pub seed: u128,
}
/// Bonding-curve state stored in the root Meme data cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV3BondingCurve {
    pub graduated: bool,
    pub alpha: TLBCoins,
    pub beta: TLBCoins,
    pub on_sell_supply: TLBCoins,
    pub raised_funds: TLBCoins,
    pub current_supply: TLBCoins,
}
/// Runtime Meme v3 root storage.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV3Storage {
    pub initialized: bool,
    pub migrated: bool,
    pub config: TLBRef<DedustUranusMemeV3Config>,
    /// Bonding-curve base trading fee in basis points.
    pub base_fee_bps: u16,
    pub curve: DedustUranusMemeV3BondingCurve,
    pub total_supply: TLBCoins,
    pub creator_fee: TLBCoins,
    /// Coins120 accumulated partner fee, reconciled against captured getter83180.
    pub partner_fee: TLBCoins,
    pub partner_fee_bps: u16,
    pub migration_config: TLBRef<DedustUranusMemeV3MigrationConfig>,
}
/// Fee breakdown emitted by bonding-curve trade events.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV3TradeFees {
    pub creator_fee: TLBCoins,
    pub protocol_fee: TLBCoins,
    pub partner_fee: TLBCoins,
    pub referrer_fee: TLBCoins,
}
/// Uranus exit-code catalog shared by Factory, Meme, and Meme Wallet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DedustUranusMemeV3Errors {
    /// Bonding curve parameters are malformed.
    BondingCurveParametersMalformed = 20,
    /// Output amount is below the supplied slippage limit.
    SlippageExceeded = 21,
    /// Selected meme preset does not exist.
    PresetNotExist = 22,
    /// Attached GRAM value is not enough for the requested operation.
    MessageValueTooLow = 23,
    /// Meme state was already initialized.
    AlreadyInitialized = 24,
    /// Sender is not authorized for the operation.
    SenderUnauthorized = 25,
    /// Initial buy exceeds the configured limit.
    InitialBuyLimitExceeded = 26,
    /// Requested amount exceeds the available balance.
    BalanceExceeded = 27,
    /// Meme state has not been initialized yet.
    NotInitialized = 28,
    /// Meme already graduated and no longer accepts curve trading.
    AlreadyGraduated = 29,
    /// Cross-workchain transfer is rejected.
    CrossWorkchainTransfer = 30,
    /// Supplied address is incorrect for the operation.
    IncorrectAddress = 31,
    /// Selected fee preset does not exist.
    FeePresetNotExist = 34,
    /// Selected total-supply preset does not exist.
    TotalSupplyPresetNotExist = 35,
    /// Fundraising target is outside the accepted range.
    RaisingFundsOutOfRange = 36,
    /// Partner fee is outside the accepted range.
    PartnerFeeOutOfRange = 37,
    /// On-sell supply percentage is outside the accepted range.
    OnSellSupplyPercentOutOfRange = 38,
    /// Migrated CPMM pool partner fee is outside the accepted range.
    PoolPartnerFeeOutOfRange = 39,
    /// Invalid low-level tag in a bounced wallet flow.
    InvalidTag = 63,
    /// Incoming message body tag is unknown.
    UnknownOperation = 65535,
}
/// Captured v3 migration configuration; see uranus_tail.test.tolk and library a63e4a80.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusMemeV3MigrationConfig {
    #[tlb(adapter = "InternalAddress")]
    pub partner_address: TonAddress,
    pub pool_partner_fee_bps: u16,
    pub pool_base_fee_bps: u16,
    #[tlb(adapter = "InternalAddress")]
    pub pool_liquidity_owner_address: TonAddress,
}
