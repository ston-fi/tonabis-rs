//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{
    DedustUranusFactoryV3MemeMetadata, DedustUranusFactoryV3PartnerConfig, DedustUranusFactoryV3ReferrerConfig,
};
use ton::{
    contracts::tep::snake_data::SnakeData,
    ton_core::{
        TLB,
        cell::TonCell,
        types::{
            TonAddress,
            tlb_core::{
                TLBCoins, TLBRef,
                adapters::{ConstLen, InternalAddress, NullableAddress},
            },
        },
    },
};
/// Deploy a new meme from a predefined Uranus preset.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1878267612, bits_len = 32)]
pub struct DedustUranusFactoryV3DeployMemeMsg {
    pub query_id: u64,
    #[tlb(bits_len = 4)]
    pub preset_id: u8,
    pub metadata: DedustUranusFactoryV3MemeMetadata,
    pub initial_buy: TLBCoins,
    pub partner_config: Option<DedustUranusFactoryV3PartnerConfig>,
    pub referrer_config: Option<DedustUranusFactoryV3ReferrerConfig>,
}
/// Deploy a customized v3 meme with explicit economics and migration settings.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1664048412, bits_len = 32)]
pub struct DedustUranusFactoryV3DeployCustomizedMemeMsg {
    pub query_id: u64,
    #[tlb(bits_len = 4)]
    pub total_supply_preset_id: u8,
    #[tlb(bits_len = 4)]
    pub base_fee_preset_id: u8,
    pub raising_funds: TLBCoins,
    pub on_sell_supply_percent: u8,
    #[tlb(adapter = "InternalAddress")]
    pub partner_address: TonAddress,
    /// Partner share of the bonding-curve base fee in basis points.
    pub partner_fee_bps: u16,
    /// Partner share of the migrated CPMM pool base fee in basis points.
    pub pool_partner_fee_bps: u16,
    pub pool_base_fee_preset_id: u16,
    #[tlb(adapter = "NullableAddress")]
    pub pool_liquidity_owner_address: Option<TonAddress>,
    pub metadata_uri: TLBRef<SnakeData>,
    pub initial_buy: TLBCoins,
    pub partner_config: Option<TLBRef<DedustUranusFactoryV3PartnerConfig>>,
    pub referrer_config: Option<TLBRef<DedustUranusFactoryV3ReferrerConfig>>,
}
/// Factory-to-Meme initialization body emitted after deterministic child deployment.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2037340684, bits_len = 32)]
pub struct DedustUranusFactoryV3InitMemeMsg {
    pub query_id: u64,
    pub initial_buy: TLBCoins,
    pub partner_config: Option<DedustUranusFactoryV3PartnerConfig>,
    pub referrer_config: Option<DedustUranusFactoryV3ReferrerConfig>,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustUranusFactoryV3IncomingMsg {
    UranusDeployMeme(Box<DedustUranusFactoryV3DeployMemeMsg>),
    UranusDeployCustomizedMeme(Box<DedustUranusFactoryV3DeployCustomizedMemeMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustUranusFactoryV3OutgoingMsg {
    UranusInitMeme(Box<DedustUranusFactoryV3InitMemeMsg>),
    Unknown(TonCell),
}
