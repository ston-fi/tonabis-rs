//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{DedustUranusMemeV2PartnerConfig, DedustUranusMemeV2ReferrerConfig, DedustUranusMemeV2TradeFees};
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
pub struct DedustUranusMemeV2InitMemeMsg {
    pub query_id: u64,
    pub initial_buy: TLBCoins,
    pub partner_config: Option<DedustUranusMemeV2PartnerConfig>,
    pub referrer_config: Option<DedustUranusMemeV2ReferrerConfig>,
}
/// User buy request accepted by the Meme contract.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2491573591, bits_len = 32)]
pub struct DedustUranusMemeV2BuyMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub minimal_amount_out: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub excesses_to: TonAddress,
    pub partner_config: Option<TLBRef<DedustUranusMemeV2PartnerConfig>>,
    pub referrer_config: Option<TLBRef<DedustUranusMemeV2ReferrerConfig>>,
}
/// Wallet-to-Meme sell request emitted by Uranus Meme Wallet.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1684722724, bits_len = 32)]
pub struct DedustUranusMemeV2SellMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub minimal_amount_out: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub r#from: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_to: Option<TonAddress>,
    pub partner_config: Option<TLBRef<DedustUranusMemeV2PartnerConfig>>,
    pub referrer_config: Option<TLBRef<DedustUranusMemeV2ReferrerConfig>>,
}
/// Standard Jetton burn notification accepted from deterministic Meme wallets.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2078119902, bits_len = 32)]
pub struct DedustUranusMemeV2BurnNotificationMsg {
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
pub struct DedustUranusMemeV2ProvideWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub owner_address: Option<TonAddress>,
    pub include_address: bool,
}
/// Claim accumulated creator fees.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2909956520, bits_len = 32)]
pub struct DedustUranusMemeV2ClaimCreatorFeeMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub to: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_to: Option<TonAddress>,
}
/// Unnamed fee-claim request with one optional payout address.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 4048246003, bits_len = 32)]
pub struct DedustUranusMemeV2OpcodeF14b54f3Msg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub to: Option<TonAddress>,
}
/// Standard excess-value response accepted during callback handling.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3576854235, bits_len = 32)]
pub struct DedustUranusMemeV2ExcessesMsg {
    pub query_id: u64,
}
/// CPMM initialization result callback consumed after migration.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3457702871, bits_len = 32)]
pub struct DedustUranusMemeV2CpmmInitResultMsg {
    pub query_id: u64,
    pub exit_code: i32,
    pub payload: Option<TLBRef<TonCell>>,
}
/// External-out event emitted after a successful buy.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2695523266, bits_len = 32)]
pub struct DedustUranusMemeV2BuyEventMsg {
    #[tlb(adapter = "InternalAddress")]
    pub trader_address: TonAddress,
    pub amount_in: TLBCoins,
    pub amount_out: TLBCoins,
    pub fees: DedustUranusMemeV2TradeFees,
    pub current_supply: TLBCoins,
    pub raised_funds: TLBCoins,
    pub is_graduated: bool,
}
/// External-out event emitted after a successful sell.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 984677580, bits_len = 32)]
pub struct DedustUranusMemeV2SellEventMsg {
    #[tlb(adapter = "InternalAddress")]
    pub trader_address: TonAddress,
    pub amount_in: TLBCoins,
    pub amount_out: TLBCoins,
    pub fees: DedustUranusMemeV2TradeFees,
    pub current_supply: TLBCoins,
    pub raised_funds: TLBCoins,
}
/// Compact successful-buy callback emitted to the excess receiver.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1724427634, bits_len = 32)]
pub struct DedustUranusMemeV2Opcode66c8ad72Msg {
    pub query_id: u64,
}
/// TEP-89 wallet discovery response emitted by the Meme minter.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3513996288, bits_len = 32)]
pub struct DedustUranusMemeV2TakeWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub wallet_address: Option<TonAddress>,
    #[tlb(adapter = "MaybeRefAddress")]
    pub owner_address: Option<TonAddress>,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustUranusMemeV2IncomingMsg {
    InitMeme(Box<DedustUranusMemeV2InitMemeMsg>),
    Buy(Box<DedustUranusMemeV2BuyMsg>),
    Sell(Box<DedustUranusMemeV2SellMsg>),
    BurnNotification(Box<DedustUranusMemeV2BurnNotificationMsg>),
    ProvideWalletAddress(Box<DedustUranusMemeV2ProvideWalletAddressMsg>),
    ClaimCreatorFee(Box<DedustUranusMemeV2ClaimCreatorFeeMsg>),
    OpcodeF14b54f3(Box<DedustUranusMemeV2OpcodeF14b54f3Msg>),
    Excesses(Box<DedustUranusMemeV2ExcessesMsg>),
    CpmmInitResult(Box<DedustUranusMemeV2CpmmInitResultMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustUranusMemeV2OutgoingMsg {
    BuyEvent(Box<DedustUranusMemeV2BuyEventMsg>),
    SellEvent(Box<DedustUranusMemeV2SellEventMsg>),
    Opcode66c8ad72(Box<DedustUranusMemeV2Opcode66c8ad72Msg>),
    Excesses(Box<DedustUranusMemeV2ExcessesMsg>),
    TakeWalletAddress(Box<DedustUranusMemeV2TakeWalletAddressMsg>),
    Unknown(TonCell),
}
