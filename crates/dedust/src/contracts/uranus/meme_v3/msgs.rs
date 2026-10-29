//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{DedustUranusMemeV3PartnerConfig, DedustUranusMemeV3ReferrerConfig, DedustUranusMemeV3TradeFees};
use ton::ton_core::{
    TLB,
    cell::TonCell,
    types::{
        TonAddress,
        tlb_core::{
            TLBCoins, TLBRef,
            adapters::{InternalAddress, NullableAddress},
        },
    },
};
use tonabis_core::adapters::maybe_ref_address::MaybeRefAddress;
/// Factory-to-Meme initialization body.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2037340684, bits_len = 32)]
pub struct DedustUranusMemeV3InitMemeMsg {
    pub query_id: u64,
    pub initial_buy: TLBCoins,
    pub partner_config: Option<DedustUranusMemeV3PartnerConfig>,
    pub referrer_config: Option<DedustUranusMemeV3ReferrerConfig>,
}
/// User buy request accepted by the Meme contract.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2491573591, bits_len = 32)]
pub struct DedustUranusMemeV3BuyMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub minimal_amount_out: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub excesses_to: TonAddress,
    pub partner_config: Option<TLBRef<DedustUranusMemeV3PartnerConfig>>,
    pub referrer_config: Option<TLBRef<DedustUranusMemeV3ReferrerConfig>>,
}
/// Wallet-to-Meme sell request emitted by Uranus Meme Wallet.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1684722724, bits_len = 32)]
pub struct DedustUranusMemeV3SellMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub minimal_amount_out: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub r#from: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_to: Option<TonAddress>,
    pub partner_config: Option<TLBRef<DedustUranusMemeV3PartnerConfig>>,
    pub referrer_config: Option<TLBRef<DedustUranusMemeV3ReferrerConfig>>,
}
/// Standard Jetton burn notification accepted from deterministic Meme wallets.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2078119902, bits_len = 32)]
pub struct DedustUranusMemeV3BurnNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
}
/// TEP-89 wallet discovery request accepted by the Meme minter.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 745978227, bits_len = 32)]
pub struct DedustUranusMemeV3ProvideWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub owner_address: Option<TonAddress>,
    pub include_address: bool,
}
/// Claim accumulated creator fees.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2909956520, bits_len = 32)]
pub struct DedustUranusMemeV3ClaimCreatorFeeMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub to: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_to: Option<TonAddress>,
}
/// Claim accumulated partner fees.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2135673844, bits_len = 32)]
pub struct DedustUranusMemeV3ClaimPartnerFeeMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub to: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_to: Option<TonAddress>,
}
/// Unnamed v3 fee-claim request with one optional payout address.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 4048246003, bits_len = 32)]
pub struct DedustUranusMemeV3OpcodeF14b54f3Msg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub to: Option<TonAddress>,
}
/// Standard excess-value response.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3576854235, bits_len = 32)]
pub struct DedustUranusMemeV3ExcessesMsg {
    pub query_id: u64,
}
/// CPMM initialization result callback consumed after migration.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3457702871, bits_len = 32)]
pub struct DedustUranusMemeV3CpmmInitResultMsg {
    pub query_id: u64,
    pub exit_code: i32,
    pub payload: Option<TLBRef<TonCell>>,
}
/// External-out event emitted after a successful buy.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2695523266, bits_len = 32)]
pub struct DedustUranusMemeV3BuyEventMsg {
    #[tlb(adapter = "InternalAddress")]
    pub trader_address: TonAddress,
    pub amount_in: TLBCoins,
    pub amount_out: TLBCoins,
    pub fees: DedustUranusMemeV3TradeFees,
    pub current_supply: TLBCoins,
    pub raised_funds: TLBCoins,
    pub is_graduated: bool,
}
/// External-out event emitted after a successful sell.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 984677580, bits_len = 32)]
pub struct DedustUranusMemeV3SellEventMsg {
    #[tlb(adapter = "InternalAddress")]
    pub trader_address: TonAddress,
    pub amount_in: TLBCoins,
    pub amount_out: TLBCoins,
    pub fees: DedustUranusMemeV3TradeFees,
    pub current_supply: TLBCoins,
    pub raised_funds: TLBCoins,
}
/// Compact successful-buy callback emitted to the excess receiver.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1724427634, bits_len = 32)]
pub struct DedustUranusMemeV3Opcode66c8ad72Msg {
    pub query_id: u64,
}
/// TEP-89 wallet discovery response emitted by the Meme minter.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3513996288, bits_len = 32)]
pub struct DedustUranusMemeV3TakeWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub wallet_address: Option<TonAddress>,
    #[tlb(adapter = "MaybeRefAddress")]
    pub owner_address: Option<TonAddress>,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustUranusMemeV3IncomingMsg {
    UranusInitMeme(Box<DedustUranusMemeV3InitMemeMsg>),
    UranusBuy(Box<DedustUranusMemeV3BuyMsg>),
    UranusSell(Box<DedustUranusMemeV3SellMsg>),
    UranusBurnNotification(Box<DedustUranusMemeV3BurnNotificationMsg>),
    UranusProvideWalletAddress(Box<DedustUranusMemeV3ProvideWalletAddressMsg>),
    UranusClaimCreatorFee(Box<DedustUranusMemeV3ClaimCreatorFeeMsg>),
    UranusClaimPartnerFee(Box<DedustUranusMemeV3ClaimPartnerFeeMsg>),
    UranusOpcodeF14b54f3(Box<DedustUranusMemeV3OpcodeF14b54f3Msg>),
    UranusExcesses(Box<DedustUranusMemeV3ExcessesMsg>),
    UranusCpmmInitResult(Box<DedustUranusMemeV3CpmmInitResultMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustUranusMemeV3OutgoingMsg {
    UranusBuyEvent(Box<DedustUranusMemeV3BuyEventMsg>),
    UranusSellEvent(Box<DedustUranusMemeV3SellEventMsg>),
    UranusOpcode66c8ad72(Box<DedustUranusMemeV3Opcode66c8ad72Msg>),
    UranusExcesses(Box<DedustUranusMemeV3ExcessesMsg>),
    UranusTakeWalletAddress(Box<DedustUranusMemeV3TakeWalletAddressMsg>),
    Unknown(TonCell),
}
