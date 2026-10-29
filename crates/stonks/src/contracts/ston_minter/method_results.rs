//! ston_minter method_results records.
use fastnum::I512;
use fastnum::U256;
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
/// Complete eight-field bonding stack used by STON and DeDust v7.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksStonMinterGetBondingDataResult {
    pub progress_bps: I512,
    pub migration_ton_target: I512,
    pub cumulative_ton_reserve: I512,
    pub cumulative_jetton_reserve: I512,
    pub tons_collected: I512,
    pub max_buy_percent: I512,
    pub tokens_sold: I512,
    pub presale_open: I512,
}

/// Complete five-field Jetton master stack.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksStonMinterGetJettonDataResult {
    pub total_supply: I512,
    pub mintable: bool,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub admin_address: Option<TonAddress>,
    pub jetton_content: TonCell,
    pub jetton_wallet_code: TonCell,
}

/// Ten-field extended getter shared by captured legacy DeDust v7 and STON v8/v10/v11.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksStonMinterGetExtendedDataResult {
    /// TVM boolean (-1 for open).
    pub presale_open: bool,
    /// Target collection.
    pub migration_ton_target: U256,
    /// Virtual plus physical TON.
    pub cumulative_ton_reserve: U256,
    /// Virtual plus physical tokens.
    pub cumulative_jetton_reserve: U256,
    /// Physical TON collected.
    pub tons_collected: U256,
    /// Per-buy cap percent.
    pub max_buy_percent: u16,
    /// Physical tokens sold.
    pub tokens_sold: U256,
    /// Router for STON or derived pool for DeDust; interpretation requires target.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub destination: Option<TonAddress>,
    /// Destination fee-share field (zero on captured STON, 4000 on DeDust v7).
    pub destination_fee_share: u16,
    /// Source-defined launch state flag.
    pub migration_completed: bool,
}

/// Twelve-field STON extended getter at numeric selector68631.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksStonMinterGetStonDataResult {
    /// TVM presale boolean.
    pub presale_open: bool,
    /// Physical target.
    pub migration_ton_target: U256,
    /// Total TON reserve.
    pub cumulative_ton_reserve: U256,
    /// Total token reserve.
    pub cumulative_jetton_reserve: U256,
    /// Physical TON.
    pub tons_collected: U256,
    /// Cap percent.
    pub max_buy_percent: u16,
    /// Sold physical tokens.
    pub tokens_sold: U256,
    /// Destination router.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub router: Option<TonAddress>,
    /// Router-owned wallet for the launched token.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub router_wallet: Option<TonAddress>,
    /// Source-defined native proxy address.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub native_proxy: Option<TonAddress>,
    /// Source-defined routing flag.
    pub routing_flag: bool,
    /// Source-defined launch state flag.
    pub migration_completed: bool,
}

/// Two protocol addresses returned by numeric selector95602.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksStonMinterGetProtocolAddressesResult {
    /// Authenticated controller.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub controller: Option<TonAddress>,
    /// Destination router.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub router: Option<TonAddress>,
}
