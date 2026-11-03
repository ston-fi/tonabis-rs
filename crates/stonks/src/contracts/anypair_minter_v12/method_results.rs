//! Complete jetton-quoted v12 bonding result; amounts use the quote master's base units.
use fastnum::U256;
use ton::ton_core::FromTVMStack;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

/// Thirteen-field `get_bonding_data` described by the anypair integration guide.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct StonksAnypairMinterV12GetBondingDataResult {
    /// Floor(real quote reserve * 10000 / migration quote target).
    pub progress_bps: u16,
    /// Graduation target in quote jetton base units.
    pub migration_quote_target: U256,
    /// Real plus virtual quote reserve.
    pub cumulative_quote_reserve: U256,
    /// Remaining real plus virtual launched-token reserve.
    pub cumulative_jetton_reserve: U256,
    /// Real quote reserve: accepted buys minus gross sell outputs.
    pub quote_collected: U256,
    /// Percentage of issued supply allowed per buy; zero disables this cap.
    pub max_buy_percent: u16,
    /// Launched tokens sold, in base units of the fixed 100-million-token supply.
    pub tokens_sold: U256,
    /// Whether the presale is open.
    pub presale_open: bool,
    /// Additional creator presale fee, excluding the 40 protocol basis points.
    pub presale_tax_bps: u16,
    /// Reported CPMM fee, equal to 40 plus the additional presale fee.
    pub cpmm_fee_bps: u16,
    /// Creator share of post-protocol CPMM fees, denominator 10000.
    pub creator_fee_bps: u16,
    /// Recipient of the presale creator fee.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub dev_wallet: Option<TonAddress>,
    /// Configured quote jetton master; addr_none remains distinct from internal zero.
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub quote_master: Option<TonAddress>,
}
