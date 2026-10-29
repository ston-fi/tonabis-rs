//! ston_wallet storage records.
use ton::ton_core::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
/// Two-address wallet context; it must not be decoded as DeDust's one-address context.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksStonWalletContext {
    /// First source context address.
    #[tlb(adapter = "NullableAddress")]
    pub address0: Option<TonAddress>,
    /// Second source context address.
    #[tlb(adapter = "NullableAddress")]
    pub address1: Option<TonAddress>,
    /// Source status bit; initial derivation sets it true.
    pub flag: bool,
}

/// Presale wallet state for captured STON generations.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksStonWalletStorage {
    /// Spendable tokens.
    pub balance: TLBCoins,
    /// Authorized owner.
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
    /// Minter.
    #[tlb(adapter = "NullableAddress")]
    pub master: Option<TonAddress>,
    /// STON-specific two-address context.
    pub context: TLBRef<StonksStonWalletContext>,
}
