//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{DedustUranusMemeWalletV2PartnerConfig, DedustUranusMemeWalletV2ReferrerConfig};
use ton::ton_core::{
    TLB,
    cell::TonCell,
    types::{
        TonAddress,
        tlb_core::{TLBCoins, TLBRef, adapters::NullableAddress},
    },
};
/// Owner-originated TEP-74 transfer request.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 260734629, bits_len = 32)]
pub struct DedustUranusMemeWalletV2JettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub destination: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TonCell,
}
/// Wallet-to-wallet or minter-to-wallet TEP-74 internal transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 395134233, bits_len = 32)]
pub struct DedustUranusMemeWalletV2JettonInternalTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub r#from: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TonCell,
}
/// Owner-originated burn request.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1499400124, bits_len = 32)]
pub struct DedustUranusMemeWalletV2JettonBurnMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    /// Captured V2 branch ignores this suffix; preserve all bits/refs losslessly.
    pub ignored_suffix: TonCell,
}
/// Owner-originated sell request routed from wallet to Meme.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3074793004, bits_len = 32)]
pub struct DedustUranusMemeWalletV2SellTokensMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub minimal_amount_out: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_to: Option<TonAddress>,
    pub partner_config: Option<TLBRef<DedustUranusMemeWalletV2PartnerConfig>>,
    pub referrer_config: Option<TLBRef<DedustUranusMemeWalletV2ReferrerConfig>>,
    /// Captured V2 branch ignores this suffix; preserve all bits/refs losslessly.
    pub ignored_suffix: TonCell,
}
/// Wallet-to-Meme sell request emitted after owner sell authorization.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1684722724, bits_len = 32)]
pub struct DedustUranusMemeWalletV2SellMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub minimal_amount_out: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub r#from: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub excesses_to: Option<TonAddress>,
    pub partner_config: Option<TLBRef<DedustUranusMemeWalletV2PartnerConfig>>,
    pub referrer_config: Option<TLBRef<DedustUranusMemeWalletV2ReferrerConfig>>,
}
/// Burn notification emitted to the Meme minter.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2078119902, bits_len = 32)]
pub struct DedustUranusMemeWalletV2BurnNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub sender: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
}
/// Owner notification emitted when an incoming transfer forwards GRAM.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1935855772, bits_len = 32)]
pub struct DedustUranusMemeWalletV2TransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub sender: Option<TonAddress>,
    pub forward_payload: TonCell,
}
/// Standard excess-value response.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3576854235, bits_len = 32)]
pub struct DedustUranusMemeWalletV2ExcessesMsg {
    pub query_id: u64,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustUranusMemeWalletV2IncomingMsg {
    UranusJettonTransfer(Box<DedustUranusMemeWalletV2JettonTransferMsg>),
    UranusJettonInternalTransfer(Box<DedustUranusMemeWalletV2JettonInternalTransferMsg>),
    UranusJettonBurn(Box<DedustUranusMemeWalletV2JettonBurnMsg>),
    UranusSellTokens(Box<DedustUranusMemeWalletV2SellTokensMsg>),
    Unknown(TonCell),
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum DedustUranusMemeWalletV2OutgoingMsg {
    UranusJettonInternalTransfer(Box<DedustUranusMemeWalletV2JettonInternalTransferMsg>),
    UranusBurnNotification(Box<DedustUranusMemeWalletV2BurnNotificationMsg>),
    UranusTransferNotification(Box<DedustUranusMemeWalletV2TransferNotificationMsg>),
    UranusSell(Box<DedustUranusMemeWalletV2SellMsg>),
    UranusExcesses(Box<DedustUranusMemeWalletV2ExcessesMsg>),
    Unknown(TonCell),
}
