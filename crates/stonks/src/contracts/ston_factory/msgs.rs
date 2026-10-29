//! ston_factory msgs records.
use crate::contracts::ston_minter::msgs::StonksStonMinterVirtualMinterInitMsg;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
/// Legacy factory creation request; v12 creation has a separate extended layout.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a9801, bits_len = 32)]
pub struct StonksStonFactoryCreateVirtualLiquidityJettonMsg {
    pub query_id: u64,
    pub content: TLBRef<TonCell>,
    pub dev_buy_ton_amount: TLBCoins,
    pub max_buy_percent: u16,
    #[tlb(adapter = "NullableAddress")]
    pub dev_wallet: Option<TonAddress>,
}

/// Factory TON withdrawal request.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a9804, bits_len = 32)]
pub struct StonksStonFactoryWithdrawTonMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
}

/// Factory creation-disable flag update.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a9805, bits_len = 32)]
pub struct StonksStonFactorySetDisabledMsg {
    pub query_id: u64,
    pub disabled: bool,
}

/// Known typed bodies and losslessly retained unknown messages.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum StonksStonFactoryMessage {
    CreateVirtualLiquidityJetton(StonksStonFactoryCreateVirtualLiquidityJettonMsg),
    VirtualFactoryWithdrawTon(StonksStonFactoryWithdrawTonMsg),
    VirtualFactorySetDisabled(StonksStonFactorySetDisabledMsg),
    VirtualMinterFactoryInit(StonksStonMinterVirtualMinterInitMsg),
    /// Unrecognized opcode or body shorter than an opcode.
    Unknown(TonCell),
}
