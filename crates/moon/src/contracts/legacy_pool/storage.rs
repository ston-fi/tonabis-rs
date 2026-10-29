//! Generated Moon wire records. See tools/reference/moon/README.md.
use fastnum::I512;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

/// Pool state present before the one-time initialization message is accepted.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolDeploymentStorage {
    #[tlb(adapter = "InternalAddress")]
    pub admin_address: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub asset1_id: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub asset2_id: Option<TonAddress>,
}

/// Initialized persistent Moon.cx pool state.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolStorage {
    #[tlb(adapter = "InternalAddress")]
    pub admin_address: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub asset1_wallet: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub asset2_wallet: Option<TonAddress>,
    pub swap_fee_bps: u8,
    pub protocol_fee_bps: u8,
    pub referral_fee_bps: u8,
    pub lp_total_supply: TLBCoins,
    pub balances: TLBRef<MoonLegacyPoolBalances>,
    pub lp_wallet_code: TLBRef<TonCell>,
    pub lp_account_code: TLBRef<TonCell>,
    pub assets_and_content: TLBRef<MoonLegacyPoolAssetsAndContent>,
}

/// Pool reserves and protocol fees stored in asset order.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolBalances {
    pub asset1_reserve: TLBCoins,
    pub asset2_reserve: TLBCoins,
    pub asset1_protocol_fees: TLBCoins,
    pub asset2_protocol_fees: TLBCoins,
}

/// Asset identifiers and LP Jetton metadata retained by the pool.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolAssetsAndContent {
    #[tlb(adapter = "NullableAddress")]
    pub asset1_id: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub asset2_id: Option<TonAddress>,
    pub lp_jetton_content: TLBRef<TonCell>,
}

/// Asset identifier and amount cell accepted by pool quote getters.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolAssetAmount {
    #[tlb(adapter = "NullableAddress")]
    pub asset_id: Option<TonAddress>,
    pub amount: TLBCoins,
}

/// Optional continuation selected after a swap succeeds or fails.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolNextPayload {
    #[tlb(adapter = "NullableAddress")]
    pub recipient: Option<TonAddress>,
    pub payload: Option<TLBRef<TonCell>>,
}

/// Swap limits, routing addresses, and optional success/failure continuations.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolSwapParams {
    pub min_out: TLBCoins,
    pub deadline: u64,
    #[tlb(adapter = "NullableAddress")]
    pub excess_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub referral_address: Option<TonAddress>,
    pub next_fulfill: Option<TLBRef<MoonLegacyPoolNextPayload>>,
    pub next_reject: Option<TLBRef<MoonLegacyPoolNextPayload>>,
}

/// Asset and reserve tuple returned by `get_reserves`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolReservesReply {
    #[tlb(adapter = "NullableAddress")]
    pub asset1_id: Option<TonAddress>,
    #[tlb(bits_len = 257)]
    pub asset1_reserve: I512,
    #[tlb(adapter = "NullableAddress")]
    pub asset2_id: Option<TonAddress>,
    #[tlb(bits_len = 257)]
    pub asset2_reserve: I512,
}

/// Asset identifiers returned by `get_assets`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolAssetsReply {
    #[tlb(adapter = "NullableAddress")]
    pub asset1_id: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub asset2_id: Option<TonAddress>,
}

/// Pool wallet addresses returned by `get_wallets`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolWalletsReply {
    #[tlb(adapter = "NullableAddress")]
    pub asset1_wallet: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub asset2_wallet: Option<TonAddress>,
}

/// Fee parameters returned by `get_fees`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolFeesReply {
    #[tlb(bits_len = 257)]
    pub swap_fee_bps: I512,
    #[tlb(bits_len = 257)]
    pub protocol_fee_bps: I512,
    #[tlb(bits_len = 257)]
    pub referral_fee_bps: I512,
}

/// Collected protocol fees returned by `get_collected_fees`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolCollectedFeesReply {
    #[tlb(bits_len = 257)]
    pub asset1_fees: I512,
    #[tlb(bits_len = 257)]
    pub asset2_fees: I512,
}

/// Proportional reserve amounts returned by `get_estimated_lp_burn`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolEstimatedLpBurnReply {
    #[tlb(adapter = "NullableAddress")]
    pub asset1_id: Option<TonAddress>,
    #[tlb(bits_len = 257)]
    pub asset1_amount: I512,
    #[tlb(adapter = "NullableAddress")]
    pub asset2_id: Option<TonAddress>,
    #[tlb(bits_len = 257)]
    pub asset2_amount: I512,
}

/// Swap quote returned by `get_estimated_swap_out`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolEstimatedSwapOutReply {
    #[tlb(adapter = "NullableAddress")]
    pub output_asset_id: Option<TonAddress>,
    #[tlb(bits_len = 257)]
    pub output_amount: I512,
}

/// Standard LP Jetton master data returned by `get_jetton_data`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonLegacyPoolJettonDataReply {
    #[tlb(bits_len = 257)]
    pub total_supply: I512,
    pub mintable: bool,
    #[tlb(adapter = "NullableAddress")]
    pub admin_address: Option<TonAddress>,
    pub jetton_content: TLBRef<TonCell>,
    pub jetton_wallet_code: TLBRef<TonCell>,
}
