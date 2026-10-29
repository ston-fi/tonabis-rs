//! Generated Moon wire records. See tools/reference/moon/README.md.
use crate::contracts::legacy_pool::storage::MoonLegacyPoolAssetsAndContent;
use crate::contracts::legacy_pool::storage::MoonLegacyPoolBalances;
use crate::contracts::legacy_pool::storage::MoonLegacyPoolSwapParams;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

/// Inline TEP-74 forward-payload variant.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0, bits_len = 1)]
pub struct MoonLegacyPoolJettonForwardPayloadInlineMsg {
    pub value: TonCell,
}

/// Referenced TEP-74 forward-payload variant.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1, bits_len = 1)]
pub struct MoonLegacyPoolJettonForwardPayloadInRefMsg {
    pub value: TLBRef<TonCell>,
}

/// Observed one-time pool initialization message accepted by deployment storage.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1876179091, bits_len = 32)]
pub struct MoonLegacyPoolInitializeMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub asset1_wallet: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub asset2_wallet: Option<TonAddress>,
    pub swap_fee_bps: u8,
    pub protocol_fee_bps: u8,
    pub referral_fee_bps: u8,
    pub balances: TLBRef<MoonLegacyPoolBalances>,
    pub lp_wallet_code: TLBRef<TonCell>,
    pub lp_account_code: TLBRef<TonCell>,
    pub assets_and_content: TLBRef<MoonLegacyPoolAssetsAndContent>,
}

/// Direct native-asset Moon.cx swap request.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3011153931, bits_len = 32)]
pub struct MoonLegacyPoolSwapMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub swap_params: MoonLegacyPoolSwapParams,
}

/// Moon swap payload carried inside a Jetton transfer notification.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3011153931, bits_len = 32)]
pub struct MoonLegacyPoolSwapForwardPayloadMsg {
    pub swap_params: MoonLegacyPoolSwapParams,
}

/// Direct native-asset liquidity deposit record request.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3005069185, bits_len = 32)]
pub struct MoonLegacyPoolDepositLiquidityMsg {
    pub query_id: u64,
    pub min_lp_out: TLBCoins,
}

/// Liquidity deposit payload carried inside a Jetton transfer notification.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3005069185, bits_len = 32)]
pub struct MoonLegacyPoolDepositLiquidityForwardPayloadMsg {
    pub min_lp_out: TLBCoins,
}

/// Adds native GRAM to the pool reserve without minting LP Jettons.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2527729030, bits_len = 32)]
pub struct MoonLegacyPoolBoostMsg {
    pub query_id: u64,
}

/// Pool boost payload carried inside a Jetton transfer notification.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2527729030, bits_len = 32)]
pub struct MoonLegacyPoolBoostForwardPayloadMsg {
    #[tlb(adapter = "InternalAddress")]
    pub response_address: TonAddress,
}

/// Per-user LP account callback that commits a matched two-asset deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1796966975, bits_len = 32)]
pub struct MoonLegacyPoolProvideLiquidityMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub user_address: TonAddress,
    pub min_lp_amount: TLBCoins,
}

/// Per-user LP account callback that cancels unmatched deposits.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3272241059, bits_len = 32)]
pub struct MoonLegacyPoolCancelDepositMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub user_address: TonAddress,
}

/// Standard LP Jetton burn notification used to withdraw proportional reserves.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2078119902, bits_len = 32)]
pub struct MoonLegacyPoolLpBurnNotificationMsg {
    pub query_id: u64,
    pub lp_amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
}

/// Standard Jetton transfer notification carrying Moon forward payloads.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1935855772, bits_len = 32)]
pub struct MoonLegacyPoolJettonTransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender_address: TonAddress,
    pub forward_payload: TonCell,
}

/// Administrator command that transfers accumulated protocol fees.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 67889885, bits_len = 32)]
pub struct MoonLegacyPoolCollectFeesMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
}

/// Native protocol-fee payout emitted by the administrator collection flow.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 67889885, bits_len = 32)]
pub struct MoonLegacyPoolCollectFeesPayoutMsg {
    pub query_id: u64,
}

/// Administrator command that changes all three fee parameters.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 245109927, bits_len = 32)]
pub struct MoonLegacyPoolSetFeesMsg {
    pub query_id: u64,
    pub swap_fee_bps: u8,
    pub protocol_fee_bps: u8,
    pub referral_fee_bps: u8,
}

/// Administrator command that replaces LP Jetton metadata content.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1273146398, bits_len = 32)]
pub struct MoonLegacyPoolSetContentMsg {
    pub query_id: u64,
    pub lp_jetton_content: TLBRef<TonCell>,
}

/// Administrator command that replaces the pool code cell.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3798505520, bits_len = 32)]
pub struct MoonLegacyPoolUpgradeCodeMsg {
    pub query_id: u64,
    pub new_code: TLBRef<TonCell>,
}

/// Standard excess-value body accepted as a no-op and emitted by pool flows.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3576854235, bits_len = 32)]
pub struct MoonLegacyPoolExcessMsg {
    pub query_id: u64,
}

/// Standard LP Jetton internal transfer used by the initial and later mint paths.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 395134233, bits_len = 32)]
pub struct MoonLegacyPoolJettonInternalTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub from_address: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TonCell,
}

/// Standard Jetton transfer emitted to an asset wallet.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 260734629, bits_len = 32)]
pub struct MoonLegacyPoolJettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TonCell,
}

/// Successful swap callback; custom continuation bytes may follow the opcode.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3414112470, bits_len = 32)]
pub struct MoonLegacyPoolSwapSucceedMsg {
    pub tail: TonCell,
}

/// Failed swap callback; custom continuation bytes may follow the opcode.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3296468823, bits_len = 32)]
pub struct MoonLegacyPoolSwapFailedMsg {
    pub tail: TonCell,
}

/// Successful two-asset liquidity provision callback.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1937460918, bits_len = 32)]
pub struct MoonLegacyPoolProvideLiquiditySucceedMsg {}

/// Failed liquidity provision callback in either refund form.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2108649568, bits_len = 32)]
pub struct MoonLegacyPoolProvideLiquidityFailedMsg {
    pub tail: TonCell,
}

/// Records one side of a liquidity deposit in the per-user LP account.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3131714587, bits_len = 32)]
pub struct MoonLegacyPoolDepositRecordMsg {
    pub query_id: u64,
    pub asset_number: u8,
    pub deposit_amount: TLBCoins,
    pub min_lp_amount: TLBCoins,
}

/// Native asset payout emitted during LP withdrawal.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2429793453, bits_len = 32)]
pub struct MoonLegacyPoolWithdrawLiquidityPayoutMsg {
    pub query_id: u64,
}

/// Completion notification emitted after both withdrawal payouts.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 4287033447, bits_len = 32)]
pub struct MoonLegacyPoolWithdrawLiquidityNotifyMsg {
    pub query_id: u64,
}

/// Native asset refund emitted when a pending deposit is cancelled.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 380920756, bits_len = 32)]
pub struct MoonLegacyPoolCancelDepositPayoutMsg {
    pub query_id: u64,
}

/// Native referral-fee payout emitted for a swap with a referral address.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1756446661, bits_len = 32)]
pub struct MoonLegacyPoolReferralPayoutMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum MoonLegacyPoolIncomingMsg {
    MoonPoolInitializeMsg(Box<MoonLegacyPoolInitializeMsg>),
    MoonPoolSwapMsg(Box<MoonLegacyPoolSwapMsg>),
    MoonPoolDepositLiquidityMsg(Box<MoonLegacyPoolDepositLiquidityMsg>),
    MoonPoolBoostMsg(Box<MoonLegacyPoolBoostMsg>),
    MoonPoolProvideLiquidityMsg(Box<MoonLegacyPoolProvideLiquidityMsg>),
    MoonPoolCancelDepositMsg(Box<MoonLegacyPoolCancelDepositMsg>),
    MoonPoolLpBurnNotificationMsg(Box<MoonLegacyPoolLpBurnNotificationMsg>),
    MoonPoolJettonTransferNotificationMsg(Box<MoonLegacyPoolJettonTransferNotificationMsg>),
    MoonPoolCollectFeesMsg(Box<MoonLegacyPoolCollectFeesMsg>),
    MoonPoolSetFeesMsg(Box<MoonLegacyPoolSetFeesMsg>),
    MoonPoolSetContentMsg(Box<MoonLegacyPoolSetContentMsg>),
    MoonPoolUpgradeCodeMsg(Box<MoonLegacyPoolUpgradeCodeMsg>),
    MoonPoolExcessMsg(Box<MoonLegacyPoolExcessMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum MoonLegacyPoolOutgoingMsg {
    MoonPoolJettonInternalTransferMsg(Box<MoonLegacyPoolJettonInternalTransferMsg>),
    MoonPoolJettonTransferMsg(Box<MoonLegacyPoolJettonTransferMsg>),
    MoonPoolSwapSucceedMsg(Box<MoonLegacyPoolSwapSucceedMsg>),
    MoonPoolSwapFailedMsg(Box<MoonLegacyPoolSwapFailedMsg>),
    MoonPoolProvideLiquiditySucceedMsg(Box<MoonLegacyPoolProvideLiquiditySucceedMsg>),
    MoonPoolProvideLiquidityFailedMsg(Box<MoonLegacyPoolProvideLiquidityFailedMsg>),
    MoonPoolDepositRecordMsg(Box<MoonLegacyPoolDepositRecordMsg>),
    MoonPoolWithdrawLiquidityPayoutMsg(Box<MoonLegacyPoolWithdrawLiquidityPayoutMsg>),
    MoonPoolWithdrawLiquidityNotifyMsg(Box<MoonLegacyPoolWithdrawLiquidityNotifyMsg>),
    MoonPoolCancelDepositPayoutMsg(Box<MoonLegacyPoolCancelDepositPayoutMsg>),
    MoonPoolReferralPayoutMsg(Box<MoonLegacyPoolReferralPayoutMsg>),
    MoonPoolCollectFeesPayoutMsg(Box<MoonLegacyPoolCollectFeesPayoutMsg>),
    MoonPoolExcessMsg(Box<MoonLegacyPoolExcessMsg>),
    Unknown(TonCell),
}
