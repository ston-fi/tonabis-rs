//! Coffee msgs. See tools/reference/swap_coffee/README.md.
use crate::contracts::wire::wire_error;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::ton_core::{
    TLB,
    cell::{CellBuilder, CellParser, TonCell},
    errors::TonCoreResult,
    traits::tlb::TLB,
    types::tlb_core::{TLBCoins, TLBEither, TLBRef},
};
use tonabis_core::msgs::EmptyBodyMsg;
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct CoffeeLpWalletTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x178d4519, bits_len = 32)]
pub struct CoffeeLpWalletInternalTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub from_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x595f07bc, bits_len = 32)]
pub struct CoffeeLpWalletBurnMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7362d09c, bits_len = 32)]
pub struct CoffeeLpWalletTransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub from_address: Option<TonAddress>,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct CoffeeLpWalletExcessesMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7bdd97de, bits_len = 32)]
pub struct CoffeeLpWalletBurnNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub from_address: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xffffffff, bits_len = 32)]
pub struct CoffeeLpWalletBouncedKnownBodyMsg {
    pub original_opcode: CoffeeLpWalletBounceOpcode,
    pub query_id: u64,
    pub amount: TLBCoins,
    pub tail: TonCell,
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub enum CoffeeLpWalletIncomingMessage {
    Transfer(CoffeeLpWalletTransferMsg),
    InternalTransfer(CoffeeLpWalletInternalTransferMsg),
    Burn(CoffeeLpWalletBurnMsg),
    Empty(EmptyBodyMsg),

    Unknown(TonCell),
}

/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub enum CoffeeLpWalletOutgoingMessage {
    InternalTransfer(CoffeeLpWalletInternalTransferMsg),
    TransferNotification(CoffeeLpWalletTransferNotificationMsg),
    Excesses(CoffeeLpWalletExcessesMsg),
    BurnNotification(CoffeeLpWalletBurnNotificationMsg),

    Unknown(TonCell),
}

// LP wallet bounced-body role. Select this only when transport metadata says
// bounced; the ordinary incoming role does not treat a 0xffffffff body as one.
/// Only these original operations restore the LP wallet balance on a bounce.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CoffeeLpWalletBounceOpcode {
    InternalTransfer = 0x178d4519,
    BurnNotification = 0x7bdd97de,
}
impl TLB for CoffeeLpWalletBounceOpcode {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        match u32::read(p)? {
            0x178d4519 => Ok(Self::InternalTransfer),
            0x7bdd97de => Ok(Self::BurnNotification),
            _ => Err(wire_error("LP wallet does not restore this bounced opcode")),
        }
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        (*self as u32).write(b)
    }
}
/// Canonical known bounce or an unknown raw envelope. A malformed known prefix
/// fails; unknown envelope bytes remain available for caller-owned handling.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub enum CoffeeLpWalletBounceMessage {
    Known(crate::contracts::lp_wallet::msgs::CoffeeLpWalletBouncedKnownBodyMsg),

    Unknown(TonCell),
}
