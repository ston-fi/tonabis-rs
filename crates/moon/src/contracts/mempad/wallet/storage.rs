//! Generated Moon wire records. See tools/reference/moon/README.md.
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadJettonWalletDataReply {
    pub jetton_balance: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub minter_address: TonAddress,
    pub jetton_wallet_code: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadWalletStorage {
    pub jetton_balance: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub minter_address: TonAddress,
}
