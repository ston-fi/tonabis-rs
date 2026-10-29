//! Coffee msgs. See tools/reference/swap_coffee/README.md.
use crate::contracts::{
    vault_native::msgs::{
        CoffeeCreateLiquidityDepositoryRequestMsg, CoffeeCreatePoolCreatorRequestMsg, CoffeeDepositLiquidityParams,
        CoffeeExcessesMsg, CoffeeNotificationMsg, CoffeePayoutInternalMsg, CoffeePoolCreationParams, CoffeePoolParams,
        CoffeeSwapInternalMsg, CoffeeSwapParams, CoffeeSwapStepParams, CoffeeUpdateContractInternalMsg,
    },
    wire::TrailingSelector,
};
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
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7362d09c, bits_len = 32)]
pub struct CoffeeTransferNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub sender: Option<TonAddress>,
    #[tlb(adapter = "TrailingSelector")]
    pub forward_payload_selector: Option<bool>,
    pub forward_payload: TLBRef<CoffeeVaultJettonForwardPayload>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee10, bits_len = 32)]
pub struct CoffeeSwapJettonMsg {
    pub step: CoffeeSwapStepParams,
    pub params: TLBRef<CoffeeSwapParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee12, bits_len = 32)]
pub struct CoffeeDepositLiquidityJettonMsg {
    pub params: CoffeeDepositLiquidityParams,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee11, bits_len = 32)]
pub struct CoffeeCreatePoolJettonMsg {
    pub params: CoffeePoolParams,
    pub creation_params: CoffeePoolCreationParams,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2a, bits_len = 32)]
pub struct CoffeeCreateVaultInternalMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd1735400, bits_len = 32)]
pub struct CoffeeTakeWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub wallet_address: TonAddress,
    pub owner_response: CoffeeVaultJettonOwnerResponse,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2c, bits_len = 32)]
pub struct CoffeeActivateVaultInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub wallet: TonAddress,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct CoffeeJettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub response_destination: TonAddress,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x2c76b973, bits_len = 32)]
pub struct CoffeeProvideWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    pub include_address: bool,
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeVaultJettonForwardPayload {
    CoffeeSwapJetton(Box<CoffeeSwapJettonMsg>),
    CoffeeDepositLiquidityJetton(Box<CoffeeDepositLiquidityJettonMsg>),
    CoffeeCreatePoolJetton(Box<CoffeeCreatePoolJettonMsg>),
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
// Keep opcode prefixes visible: Box<T> erases them in locked TON dispatch.
#[allow(clippy::large_enum_variant)]
pub enum CoffeeVaultJettonIncomingMessage {
    CoffeeTransferNotification(CoffeeTransferNotificationMsg),
    CoffeePayoutInternal(CoffeePayoutInternalMsg),
    CoffeeCreateVaultInternal(CoffeeCreateVaultInternalMsg),
    CoffeeTakeWalletAddress(CoffeeTakeWalletAddressMsg),
    CoffeeActivateVaultInternal(CoffeeActivateVaultInternalMsg),
    CoffeeUpdateContractInternal(CoffeeUpdateContractInternalMsg),

    Unknown(TonCell),
}

/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
// Keep opcode prefixes visible: Box<T> erases them in locked TON dispatch.
#[allow(clippy::large_enum_variant)]
pub enum CoffeeVaultJettonOutgoingMessage {
    CoffeeNotification(CoffeeNotificationMsg),
    CoffeeSwapInternal(CoffeeSwapInternalMsg),
    CoffeeCreateLiquidityDepositoryRequest(CoffeeCreateLiquidityDepositoryRequestMsg),
    CoffeeCreatePoolCreatorRequest(CoffeeCreatePoolCreatorRequestMsg),
    CoffeeJettonTransfer(CoffeeJettonTransferMsg),
    CoffeeProvideWalletAddress(CoffeeProvideWalletAddressMsg),
    CoffeeExcesses(CoffeeExcessesMsg),

    Unknown(TonCell),
}

// TEP-89 owner suffix plus the observed legacy absent-suffix form.
/// The owner is referenced when included; absence of a selector is distinct from
/// the standard selector-zero response. No zero address is used as a sentinel.
#[derive(Debug, Clone, PartialEq)]
pub enum CoffeeVaultJettonOwnerResponse {
    Omitted,
    NotIncluded,
    Included(TonAddress),
}
impl TLB for CoffeeVaultJettonOwnerResponse {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        if p.data_bits_left()? == 0 && p.refs_left() == 0 {
            return Ok(Self::Omitted);
        }
        if bool::read(p)? {
            Ok(Self::Included(InternalAddress.read(&mut p.read_next_ref()?.parser())?))
        } else {
            Ok(Self::NotIncluded)
        }
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        match self {
            Self::Omitted => Ok(()),
            Self::NotIncluded => false.write(b),
            Self::Included(address) => {
                true.write(b)?;
                b.write_ref(tonabis_core::address::internal_address_cell(address)?)
            },
        }
    }
}
