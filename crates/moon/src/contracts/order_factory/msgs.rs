//! Generated Moon wire records. See tools/reference/moon/README.md.
use crate::contracts::order_factory::storage::MoonOrderFactoryAssets;
use crate::contracts::order_factory::storage::MoonOrderFactoryFunding;
use crate::contracts::order_factory::storage::MoonOrderFactoryOrderParams;
use crate::contracts::order_factory::storage::MoonOrderFactoryUpgradeStorage;
use crate::contracts::order_factory::storage::MoonOrderFactoryWallets;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

/// Fee-bearing order notification accepted and intentionally ignored by this factory revision.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 914019334, bits_len = 32)]
pub struct MoonOrderFactoryFeeNotificationMsg {
    pub query_id: u64,
}

/// Authorized request to derive and deploy an asset pipe.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1594451250, bits_len = 32)]
pub struct MoonOrderFactoryDeployPipeRequestMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub asset: TonAddress,
    pub config0: TLBRef<TonCell>,
    pub config1: TLBRef<TonCell>,
}

/// Order registration emitted after its first asset wallet is resolved.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1537180745, bits_len = 32)]
pub struct MoonOrderFactoryRegisterOrderMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub creator: TonAddress,
    pub order_id: u64,
    pub assets: TLBRef<MoonOrderFactoryAssets>,
    #[tlb(adapter = "InternalAddress")]
    pub source_wallet: TonAddress,
    pub amount: TLBCoins,
}

/// Administrative request that forwards two opaque references to another contract.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1890366448, bits_len = 32)]
pub struct MoonOrderFactoryForwardRequestMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    pub payload0: TLBRef<TonCell>,
    pub payload1: TLBRef<TonCell>,
}

/// Administrative code and configuration replacement request.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 474318514, bits_len = 32)]
pub struct MoonOrderFactoryUpgradeMsg {
    pub query_id: u64,
    pub new_code: TLBRef<TonCell>,
    pub new_storage: TLBRef<MoonOrderFactoryUpgradeStorage>,
}

/// Direct native-GRAM request to create a Moon.cx order.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3657858073, bits_len = 32)]
pub struct MoonOrderFactoryCreateOrderMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub asset2: TonAddress,
    pub order_params: TLBRef<MoonOrderFactoryOrderParams>,
}

/// Moon.cx payload carried inside a referenced Jetton transfer-notification payload.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3657858073, bits_len = 32)]
pub struct MoonOrderFactoryJettonCreatePayloadMsg {
    #[tlb(adapter = "InternalAddress")]
    pub asset1: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub asset2: Option<TonAddress>,
    pub order_params: TLBRef<MoonOrderFactoryOrderParams>,
}

/// Standard TEP-74 transfer notification accepted for Jetton-backed order creation.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1935855772, bits_len = 32)]
pub struct MoonOrderFactoryJettonTransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    pub forward_payload: TonCell,
}

/// Initialization body sent to a newly derived asset pipe.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3755995675, bits_len = 32)]
pub struct MoonOrderFactoryDeployPipeMsg {
    pub query_id: u64,
    pub config0: TLBRef<TonCell>,
    pub config1: TLBRef<TonCell>,
}

/// Body forwarded by an authorized administrative request.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1890366448, bits_len = 32)]
pub struct MoonOrderFactoryForwardedMsg {
    pub query_id: u64,
    pub payload0: TLBRef<TonCell>,
    pub payload1: TLBRef<TonCell>,
}

/// Initialization body sent by the factory to a newly deployed order contract.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2553557730, bits_len = 32)]
pub struct MoonOrderFactoryDeployOrderMsg {
    pub query_id: u64,
    pub order_fee_bps: u8,
    pub order_params: TLBRef<MoonOrderFactoryOrderParams>,
    pub assets: TLBRef<MoonOrderFactoryAssets>,
    pub wallets: TLBRef<MoonOrderFactoryWallets>,
    pub funding: Option<TLBRef<MoonOrderFactoryFunding>>,
}

/// Standard TEP-74 transfer emitted for refunds and protocol funding flows.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 260734629, bits_len = 32)]
pub struct MoonOrderFactoryJettonTransferMsg {
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

/// Standard excess-value response emitted after a successful direct create request.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3576854235, bits_len = 32)]
pub struct MoonOrderFactoryExcessMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum MoonOrderFactoryIncomingMsg {
    MoonOrderFactoryFeeNotificationMsg(Box<MoonOrderFactoryFeeNotificationMsg>),
    MoonOrderFactoryDeployPipeRequestMsg(Box<MoonOrderFactoryDeployPipeRequestMsg>),
    MoonOrderFactoryCreateOrderMsg(Box<MoonOrderFactoryCreateOrderMsg>),
    MoonOrderFactoryRegisterOrderMsg(Box<MoonOrderFactoryRegisterOrderMsg>),
    MoonOrderFactoryJettonTransferNotificationMsg(Box<MoonOrderFactoryJettonTransferNotificationMsg>),
    MoonOrderFactoryForwardRequestMsg(Box<MoonOrderFactoryForwardRequestMsg>),
    MoonOrderFactoryUpgradeMsg(Box<MoonOrderFactoryUpgradeMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum MoonOrderFactoryOutgoingMsg {
    MoonOrderFactoryDeployPipeMsg(Box<MoonOrderFactoryDeployPipeMsg>),
    MoonOrderFactoryDeployOrderMsg(Box<MoonOrderFactoryDeployOrderMsg>),
    MoonOrderFactoryJettonTransferMsg(Box<MoonOrderFactoryJettonTransferMsg>),
    MoonOrderFactoryExcessMsg(Box<MoonOrderFactoryExcessMsg>),
    MoonOrderFactoryForwardedMsg(Box<MoonOrderFactoryForwardedMsg>),
    Unknown(TonCell),
}
