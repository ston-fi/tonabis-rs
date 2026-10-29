//! custom_wallet storage records.
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
/// Tax-wallet peer context; both address wire forms are retained.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksCustomWalletJettonTransferContext {
    #[tlb(adapter = "NullableAddress")]
    pub context_address0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub context_address1: Option<TonAddress>,
}

/// Tax-wallet spendable/accounting balances, peer context and stored code.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksCustomWalletStorage {
    pub jetton_balance: TLBCoins,
    pub accounting_balance: TLBCoins,
    pub transfer_context: TLBRef<StonksCustomWalletJettonTransferContext>,
    #[tlb(adapter = "NullableAddress")]
    pub owner_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub jetton_master_address: Option<TonAddress>,
    pub jetton_wallet_code: TLBRef<TonCell>,
}
