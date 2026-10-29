//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use fastnum::U256;
use ton::{
    contracts::tep::snake_data::SnakeData,
    ton_core::{
        TLB,
        types::{
            TonAddress,
            tlb_core::{TLBRef, adapters::InternalAddress},
        },
    },
};
/// Root Factory state; the v3 dispatcher authenticates the controller address from this cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusFactoryV3Storage {
    #[tlb(adapter = "InternalAddress")]
    pub controller_address: TonAddress,
}
/// Metadata URI wrapper used by factory deployment bodies.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusFactoryV3MemeMetadata {
    pub uri: TLBRef<SnakeData>,
}
/// Affiliate partner configuration stored inline or in an optional ref depending on the parent message.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusFactoryV3PartnerConfig {
    pub partner_id: U256,
    /// Partner fee share in basis points.
    pub partner_fee_bps: u16,
}
/// Affiliate referrer configuration stored inline or in an optional ref depending on the parent message.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustUranusFactoryV3ReferrerConfig {
    pub referrer_id: U256,
    /// Referrer fee share in basis points.
    pub referrer_fee_bps: u16,
}
/// Uranus exit-code catalog shared by Factory, Meme, and Meme Wallet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DedustUranusFactoryV3Errors {
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
    /// Incoming message body tag is unknown.
    UnknownOperation = 65535,
}
