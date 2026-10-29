//! dedust_wallet storage records.
use ton::ton_core::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
/// Wallet-owned context preserved on transfers and changed by the master.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksDedustWalletContext {
    /// Address participating in transfer authorization.
    #[tlb(adapter = "NullableAddress")]
    pub authority: Option<TonAddress>,
    /// Source-defined transfer status flag.
    pub flag: bool,
}

/// Exact DeDust v7/v12 presale wallet storage; code is supplied through StateInit.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksDedustWalletStorage {
    /// Spendable token balance.
    pub balance: TLBCoins,
    /// Authorized owner.
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
    /// Minter/master.
    #[tlb(adapter = "NullableAddress")]
    pub master: Option<TonAddress>,
    /// Additional presale wallet context.
    pub context: TLBRef<StonksDedustWalletContext>,
}
