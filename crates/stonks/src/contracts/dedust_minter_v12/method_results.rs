//! dedust_minter_v12 method_results records.
use fastnum::U256;
use ton::ton_core::FromTVMStack;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
/// Complete stack, not an eight-field prefix plus an untyped tail.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksDedustMinterV12GetBondingDataResult {
    /// Progress, floor(collected*10000/target).
    pub progress_bps: u16,
    /// Graduation collection target.
    pub migration_ton_target: U256,
    /// Physical plus virtual TON.
    pub cumulative_ton_reserve: U256,
    /// Remaining physical plus virtual tokens.
    pub cumulative_jetton_reserve: U256,
    /// Redeemable physical TON.
    pub tons_collected: U256,
    /// Per-buy percentage cap, zero disables the cap.
    pub max_buy_percent: u16,
    /// Sold tokens in base units.
    pub tokens_sold: U256,
    /// Zero or one, as returned by this getter.
    pub presale_open: bool,
    /// Post-graduation pool base fee (first added numeric field).
    pub cpmm_fee_bps: u16,
    /// Total presale TON tax, including 40 protocol bps (second added field).
    pub presale_tax_bps: u16,
    /// Creator share of post-graduation pool fees, denominator 10000.
    pub creator_fee_bps: u16,
    /// Creator recipient; wire address retained.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub dev_wallet: Option<TonAddress>,
}

/// Four-field getter at the captured numeric selector 71772.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksDedustMinterV12GetFeeDataResult {
    /// Destination CPMM base fee.
    pub cpmm_fee_bps: u16,
    /// Total presale fee, including base 40 bps.
    pub presale_tax_bps: u16,
    /// Creator share of destination CPMM fees.
    pub creator_fee_bps: u16,
    /// Creator wallet.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub dev_wallet: Option<TonAddress>,
}

/// Complete thirteen-field stack at captured numeric selector 82057.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksDedustMinterV12GetExtendedDataResult {
    /// TVM boolean, source returns -1 for true here.
    pub presale_open: bool,
    /// Target physical TON.
    pub migration_ton_target: U256,
    /// Cumulative TON.
    pub cumulative_ton_reserve: U256,
    /// Cumulative tokens.
    pub cumulative_jetton_reserve: U256,
    /// Physical TON.
    pub tons_collected: U256,
    /// Buy cap percent.
    pub max_buy_percent: u16,
    /// Physical sold tokens.
    pub tokens_sold: U256,
    /// Derived destination pool address.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub pool_address: Option<TonAddress>,
    /// Destination creator fee share.
    pub creator_fee_bps: u16,
    /// Source-defined launch completion flag.
    pub migration_completed: bool,
    /// Destination pool base fee.
    pub cpmm_fee_bps: u16,
    /// Total presale fee.
    pub presale_tax_bps: u16,
    /// Creator recipient.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub dev_wallet: Option<TonAddress>,
}
