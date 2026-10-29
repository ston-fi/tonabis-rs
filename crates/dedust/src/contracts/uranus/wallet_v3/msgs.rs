//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{DedustUranusMemeWalletV3PartnerConfig, DedustUranusMemeWalletV3ReferrerConfig};
use ton::ton_core::types::tlb_core::TLBEitherRef;
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
/// Owner-originated TEP-74 transfer request.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 260734629, bits_len = 32)]
pub struct DedustUranusMemeWalletV3JettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TLBEitherRef<TonCell>,
}
/// Wallet-to-wallet or minter-to-wallet TEP-74 internal transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 395134233, bits_len = 32)]
pub struct DedustUranusMemeWalletV3JettonInternalTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub r#from: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TLBEitherRef<TonCell>,
}
/// Owner-originated burn request.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1499400124, bits_len = 32)]
pub struct DedustUranusMemeWalletV3JettonBurnMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
}
/// Owner-originated sell request routed from wallet to Meme.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3074793004, bits_len = 32)]
pub struct DedustUranusMemeWalletV3SellTokensMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub minimal_amount_out: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_to: Option<TonAddress>,
    pub partner_config: Option<TLBRef<DedustUranusMemeWalletV3PartnerConfig>>,
    pub referrer_config: Option<TLBRef<DedustUranusMemeWalletV3ReferrerConfig>>,
}
/// Wallet-to-Meme sell request emitted after owner sell authorization.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1684722724, bits_len = 32)]
pub struct DedustUranusMemeWalletV3SellMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub minimal_amount_out: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub r#from: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_to: Option<TonAddress>,
    pub partner_config: Option<TLBRef<DedustUranusMemeWalletV3PartnerConfig>>,
    pub referrer_config: Option<TLBRef<DedustUranusMemeWalletV3ReferrerConfig>>,
}
/// Burn notification emitted to the Meme minter.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2078119902, bits_len = 32)]
pub struct DedustUranusMemeWalletV3BurnNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
}
/// Owner notification emitted when an incoming transfer forwards GRAM.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1935855772, bits_len = 32)]
pub struct DedustUranusMemeWalletV3TransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    pub forward_payload: TLBEitherRef<TonCell>,
}
/// Standard excess-value response.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3576854235, bits_len = 32)]
pub struct DedustUranusMemeWalletV3ExcessesMsg {
    pub query_id: u64,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustUranusMemeWalletV3IncomingMsg {
    UranusJettonTransfer(Box<DedustUranusMemeWalletV3JettonTransferMsg>),
    UranusJettonInternalTransfer(Box<DedustUranusMemeWalletV3JettonInternalTransferMsg>),
    UranusJettonBurn(Box<DedustUranusMemeWalletV3JettonBurnMsg>),
    UranusSellTokens(Box<DedustUranusMemeWalletV3SellTokensMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustUranusMemeWalletV3OutgoingMsg {
    UranusJettonInternalTransfer(Box<DedustUranusMemeWalletV3JettonInternalTransferMsg>),
    UranusBurnNotification(Box<DedustUranusMemeWalletV3BurnNotificationMsg>),
    UranusTransferNotification(Box<DedustUranusMemeWalletV3TransferNotificationMsg>),
    UranusSell(Box<DedustUranusMemeWalletV3SellMsg>),
    UranusExcesses(Box<DedustUranusMemeWalletV3ExcessesMsg>),
    Unknown(TonCell),
}
