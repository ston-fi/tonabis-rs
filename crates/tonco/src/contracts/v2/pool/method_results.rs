//! Exact-arity structured getter results; integer units/widths follow the getter, not storage.
use fastnum::I512;
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPoolV2GetPendingTimelocksResult {
    pub delay: I512,
    pub code_time: I512,
    pub code: Option<TonCell>,
    pub alm_time: I512,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub alm: Option<TonAddress>,
    pub arbiter_time: I512,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub arbiter: Option<TonAddress>,
    pub flags_time: I512,
    pub flags: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPoolV2AmountsResult {
    pub amount0: I512,
    pub amount1: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPoolV2GetMintEstimateResult {
    pub amount0: I512,
    pub amount1: I512,
    pub errors: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPoolV2FeeGrowthResult {
    pub token0: I512,
    pub token1: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPoolV2GetPoolStateAndConfigurationResult {
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub router_address: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub admin_address: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub controller_address: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub jetton_wallet0: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub jetton_wallet1: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub jetton_minter0: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub jetton_minter1: Option<TonAddress>,
    pub is_active: bool,
    pub tick_spacing: I512,
    pub base_fee: I512,
    pub protocol_fee: I512,
    pub active_fee: I512,
    pub current_tick: I512,
    pub current_price: I512,
    pub current_liquidity: I512,
    pub fee_grows_global0: I512,
    pub fee_grows_global1: I512,
    pub collected_token0_protocol_fee: I512,
    pub collected_token1_protocol_fee: I512,
    pub minted_nft_positions: I512,
    pub reserve0: I512,
    pub reserve1: I512,
    pub active_nft_positions: I512,
    pub occupied_ticks: I512,
    pub operations_since_deploy: I512,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub arbiter: Option<TonAddress>,
    pub version: I512,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub alm: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub creator: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub oracle: Option<TonAddress>,
    pub flags: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPoolV2GetChildContractsResult {
    pub account: TonCell,
    pub position: TonCell,
    pub collection: TonCell,
    pub item: TonCell,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPoolV2GetCollectionDataResult {
    pub next_index: I512,
    pub content: TonCell,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub owner: Option<TonAddress>,
}
