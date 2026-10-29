//! Generated Moon wire records. See tools/reference/moon/README.md.
use crate::contracts::order_old::storage::MoonOrderOldAssets;
use crate::contracts::order_old::storage::MoonOrderOldFunding;
use crate::contracts::order_old::storage::MoonOrderOldOrderParams;
use crate::contracts::order_old::storage::MoonOrderOldWallets;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;

/// Creator request to cancel the remaining order amount.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 512588228, bits_len = 32)]
pub struct MoonOrderOldCancelMsg {
    pub query_id: u64,
}

/// Direct native-GRAM request to fill the order.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2578749506, bits_len = 32)]
pub struct MoonOrderOldFillMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    pub recipient_payload: Option<TLBRef<TonCell>>,
    #[tlb(adapter = "InternalAddress")]
    pub reject_address: TonAddress,
}

/// Moon.cx fill payload carried inside a referenced Jetton notification payload.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2578749506, bits_len = 32)]
pub struct MoonOrderOldJettonFillPayloadMsg {
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    pub recipient_payload: Option<TLBRef<TonCell>>,
    #[tlb(adapter = "InternalAddress")]
    pub reject_address: TonAddress,
}

/// Standard TEP-74 notification accepted for a Jetton-backed fill.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1935855772, bits_len = 32)]
pub struct MoonOrderOldJettonTransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    pub forward_payload: TonCell,
}

/// Asset-pipe callback that supplies one derived order-wallet address.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2560895535, bits_len = 32)]
pub struct MoonOrderOldPipeWalletResolvedMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub wallet: TonAddress,
}

/// Factory initialization body received immediately after deployment.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2553557730, bits_len = 32)]
pub struct MoonOrderOldDeployMsg {
    pub query_id: u64,
    pub order_fee_bps: u8,
    pub order_params: TLBRef<MoonOrderOldOrderParams>,
    pub assets: TLBRef<MoonOrderOldAssets>,
    pub wallets: TLBRef<MoonOrderOldWallets>,
    pub funding: Option<TLBRef<MoonOrderOldFunding>>,
}

/// Fee-bearing notification sent to the factory for native-asset settlement.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 914019334, bits_len = 32)]
pub struct MoonOrderOldFeeNotificationMsg {
    pub query_id: u64,
}

/// Order registration sent to the factory after the first wallet is resolved.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1537180745, bits_len = 32)]
pub struct MoonOrderOldFactoryRegistrationMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub creator: TonAddress,
    pub order_id: u64,
    pub assets: TLBRef<MoonOrderOldAssets>,
    #[tlb(adapter = "InternalAddress")]
    pub source_wallet: TonAddress,
    pub amount: TLBCoins,
}

/// Request sent to an asset pipe to derive this order's wallet.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2783868962, bits_len = 32)]
pub struct MoonOrderOldPipeWalletRequestMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub order: TonAddress,
}

/// Two-word rejection body whose opcode is the protocol response code 510.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 510, bits_len = 32)]
pub struct MoonOrderOldResponse510Msg {
    pub query_id: u64,
}

/// Two-word rejection body whose opcode is the protocol response code 702.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 702, bits_len = 32)]
pub struct MoonOrderOldResponse702Msg {
    pub query_id: u64,
}

/// Confirms that cancellation processing completed.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1240764137, bits_len = 32)]
pub struct MoonOrderOldCancelledMsg {
    pub query_id: u64,
}

/// Confirms an immediate order payout.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2522951223, bits_len = 32)]
pub struct MoonOrderOldPayoutMsg {
    pub query_id: u64,
}

/// Confirms release of a completed vesting payout.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3036428566, bits_len = 32)]
pub struct MoonOrderOldVestingPayoutMsg {
    pub query_id: u64,
}

/// Standard TEP-74 transfer emitted for payouts and Jetton refunds.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 260734629, bits_len = 32)]
pub struct MoonOrderOldJettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub response_destination: TonAddress,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TonCell,
}

/// Standard excess-value response.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3576854235, bits_len = 32)]
pub struct MoonOrderOldExcessMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonOrderOldExternalVestingReleaseMsg {
    pub ignored_body: TonCell,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum MoonOrderOldIncomingMsg {
    MoonOrderOldCancelMsg(Box<MoonOrderOldCancelMsg>),
    MoonOrderOldFillMsg(Box<MoonOrderOldFillMsg>),
    MoonOrderOldJettonTransferNotificationMsg(Box<MoonOrderOldJettonTransferNotificationMsg>),
    MoonOrderOldPipeWalletResolvedMsg(Box<MoonOrderOldPipeWalletResolvedMsg>),
    MoonOrderOldDeployMsg(Box<MoonOrderOldDeployMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum MoonOrderOldOutgoingMsg {
    MoonOrderOldFeeNotificationMsg(Box<MoonOrderOldFeeNotificationMsg>),
    MoonOrderOldFactoryRegistrationMsg(Box<MoonOrderOldFactoryRegistrationMsg>),
    MoonOrderOldPipeWalletRequestMsg(Box<MoonOrderOldPipeWalletRequestMsg>),
    MoonOrderOldResponse510Msg(Box<MoonOrderOldResponse510Msg>),
    MoonOrderOldResponse702Msg(Box<MoonOrderOldResponse702Msg>),
    MoonOrderOldCancelledMsg(Box<MoonOrderOldCancelledMsg>),
    MoonOrderOldPayoutMsg(Box<MoonOrderOldPayoutMsg>),
    MoonOrderOldVestingPayoutMsg(Box<MoonOrderOldVestingPayoutMsg>),
    MoonOrderOldJettonTransferMsg(Box<MoonOrderOldJettonTransferMsg>),
    MoonOrderOldExcessMsg(Box<MoonOrderOldExcessMsg>),
    Unknown(TonCell),
}
