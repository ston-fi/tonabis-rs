//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{
    DedustPoolV1DepositLiquidityAllAmounts, DedustPoolV1LiquidityDepositProof, DedustPoolV1PoolProof,
    DedustPoolV1SwapEventTail, DedustPoolV1SwapParams, DedustPoolV1SwapStepParams, DedustPoolV1VaultProof,
};
use crate::contracts::classic::asset::DedustClassicAsset;
use ton::ton_core::{
    TLB,
    cell::{CellBuilder, CellParser, TonCell},
    errors::TonCoreError,
    traits::tlb::TLB,
    types::{
        TonAddress,
        tlb_core::{
            TLBCoins, TLBRef,
            adapters::{InternalAddress, NullableAddress},
        },
    },
};
/// Factory-to-Pool command that replaces Pool code with a newer version.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 444, bits_len = 32)]
pub struct DedustPoolV1UpgradeCodeMsg {
    pub query_id: u64,
    pub code_version: u16,
    pub code: TLBRef<TonCell>,
}
/// Vault-to-Pool swap message for the first step of a user swap.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1643009069, bits_len = 32)]
pub struct DedustPoolV1SwapExternalMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustPoolV1VaultProof>,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender_addr: TonAddress,
    pub current: DedustPoolV1SwapStepParams,
    pub swap_params: TLBRef<DedustPoolV1SwapParams>,
}
/// Pool-to-Pool peer swap message for later hops in a multi-hop route.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1923917994, bits_len = 32)]
pub struct DedustPoolV1SwapPeerMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustPoolV1PoolProof>,
    pub asset: DedustClassicAsset,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender_addr: TonAddress,
    pub current: DedustPoolV1SwapStepParams,
    pub swap_params: TLBRef<DedustPoolV1SwapParams>,
}
/// Liquidity Deposit-to-Pool callback that supplies both assets and requests LP minting.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3043726744, bits_len = 32)]
pub struct DedustPoolV1DepositLiquidityAllMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustPoolV1LiquidityDepositProof>,
    #[tlb(adapter = "InternalAddress")]
    pub owner_addr: TonAddress,
    pub min_lp_amount: TLBCoins,
    pub deposit_amounts: TLBRef<DedustPoolV1DepositLiquidityAllAmounts>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// LP Jetton burn notification accepted by Pool-as-LP-minter to withdraw reserves.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2078119902, bits_len = 32)]
pub struct DedustPoolV1JettonBurnNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
}
/// Fee-collector request to withdraw accumulated protocol fees.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1647321691, bits_len = 32)]
pub struct DedustPoolV1WithdrawProtocolFeesMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub recipient_addr: TonAddress,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Pool-to-Vault payout request used by swap, withdrawal, and protocol-fee flows.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2907617013, bits_len = 32)]
pub struct DedustPoolV1PayOutFromPoolMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustPoolV1PoolProof>,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub recipient_addr: Option<TonAddress>,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Pool-to-Liquidity-Deposit success callback after LP accounting succeeds.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2867302998, bits_len = 32)]
pub struct DedustPoolV1FulfillDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustPoolV1PoolProof>,
    pub asset0_deposit: TLBCoins,
    pub asset1_deposit: TLBCoins,
}
/// Pool-to-Liquidity-Deposit rejection callback when LP minting constraints fail.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3785583828, bits_len = 32)]
pub struct DedustPoolV1RejectDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustPoolV1PoolProof>,
    pub payload: Option<TLBRef<TonCell>>,
}
/// LP wallet mint transfer emitted by Pool-as-LP-minter after successful liquidity deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 395134233, bits_len = 32)]
pub struct DedustPoolV1JettonInternalTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub from_address: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TonCell,
}
/// External event emitted after a successful swap.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2623606243, bits_len = 32)]
pub struct DedustPoolV1SwapEventMsg {
    pub asset_in: DedustClassicAsset,
    pub asset_out: DedustClassicAsset,
    pub amount_in: TLBCoins,
    pub amount_out: TLBCoins,
    pub details: TLBRef<DedustPoolV1SwapEventTail>,
}
/// External event emitted after successful liquidity deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3041195172, bits_len = 32)]
pub struct DedustPoolV1DepositEventMsg {
    #[tlb(adapter = "InternalAddress")]
    pub sender_addr: TonAddress,
    pub amount0: TLBCoins,
    pub amount1: TLBCoins,
    pub reserve0: TLBCoins,
    pub reserve1: TLBCoins,
    pub liquidity: TLBCoins,
}
/// External event emitted after LP burn withdrawal.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 984117414, bits_len = 32)]
pub struct DedustPoolV1WithdrawalEventMsg {
    #[tlb(adapter = "InternalAddress")]
    pub sender_addr: TonAddress,
    pub liquidity: TLBCoins,
    pub amount0: TLBCoins,
    pub amount1: TLBCoins,
    pub reserve0: TLBCoins,
    pub reserve1: TLBCoins,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
/// Manual dispatch preserves errors from fixed-prefix nested proofs.
#[derive(Debug, Clone, PartialEq)]
pub enum DedustPoolV1IncomingMsg {
    UpgradeCode(Box<DedustPoolV1UpgradeCodeMsg>),
    SwapExternal(Box<DedustPoolV1SwapExternalMsg>),
    SwapPeer(Box<DedustPoolV1SwapPeerMsg>),
    DepositLiquidityAll(Box<DedustPoolV1DepositLiquidityAllMsg>),
    JettonBurnNotification(Box<DedustPoolV1JettonBurnNotificationMsg>),
    WithdrawProtocolFees(Box<DedustPoolV1WithdrawProtocolFeesMsg>),
    Unknown(TonCell),
}
impl TLB for DedustPoolV1IncomingMsg {
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        if p.data_bits_left()? < 32 {
            return Ok(Self::Unknown(p.read_remaining()?));
        }
        match p.clone().read_num::<u64>(32)? {
            0x1bc => Ok(Self::UpgradeCode(Box::new(<DedustPoolV1UpgradeCodeMsg>::read(p)?))),
            0x61ee542d => Ok(Self::SwapExternal(Box::new(<DedustPoolV1SwapExternalMsg>::read(p)?))),
            0x72aca8aa => Ok(Self::SwapPeer(Box::new(<DedustPoolV1SwapPeerMsg>::read(p)?))),
            0xb56b9598 => Ok(Self::DepositLiquidityAll(Box::new(<DedustPoolV1DepositLiquidityAllMsg>::read(
                p,
            )?))),
            0x7bdd97de => Ok(Self::JettonBurnNotification(Box::new(
                <DedustPoolV1JettonBurnNotificationMsg>::read(p)?,
            ))),
            0x6230225b => Ok(Self::WithdrawProtocolFees(Box::new(
                <DedustPoolV1WithdrawProtocolFeesMsg>::read(p)?,
            ))),
            _ => Ok(Self::Unknown(p.read_remaining()?)),
        }
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        match self {
            Self::UpgradeCode(v) => v.write(b),
            Self::SwapExternal(v) => v.write(b),
            Self::SwapPeer(v) => v.write(b),
            Self::DepositLiquidityAll(v) => v.write(b),
            Self::JettonBurnNotification(v) => v.write(b),
            Self::WithdrawProtocolFees(v) => v.write(b),
            Self::Unknown(v) => b.write_cell(v),
        }
    }
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
/// Manual dispatch preserves errors from fixed-prefix nested proofs.
#[derive(Debug, Clone, PartialEq)]
pub enum DedustPoolV1OutgoingMsg {
    PayOutFromPool(Box<DedustPoolV1PayOutFromPoolMsg>),
    SwapPeer(Box<DedustPoolV1SwapPeerMsg>),
    FulfillDeposit(Box<DedustPoolV1FulfillDepositMsg>),
    RejectDeposit(Box<DedustPoolV1RejectDepositMsg>),
    JettonInternalTransfer(Box<DedustPoolV1JettonInternalTransferMsg>),
    SwapEvent(Box<DedustPoolV1SwapEventMsg>),
    DepositEvent(Box<DedustPoolV1DepositEventMsg>),
    WithdrawalEvent(Box<DedustPoolV1WithdrawalEventMsg>),
    Unknown(TonCell),
}
impl TLB for DedustPoolV1OutgoingMsg {
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        if p.data_bits_left()? < 32 {
            return Ok(Self::Unknown(p.read_remaining()?));
        }
        match p.clone().read_num::<u64>(32)? {
            0xad4eb6f5 => Ok(Self::PayOutFromPool(Box::new(<DedustPoolV1PayOutFromPoolMsg>::read(p)?))),
            0x72aca8aa => Ok(Self::SwapPeer(Box::new(<DedustPoolV1SwapPeerMsg>::read(p)?))),
            0xaae79256 => Ok(Self::FulfillDeposit(Box::new(<DedustPoolV1FulfillDepositMsg>::read(p)?))),
            0xe1a36cd4 => Ok(Self::RejectDeposit(Box::new(<DedustPoolV1RejectDepositMsg>::read(p)?))),
            0x178d4519 => Ok(Self::JettonInternalTransfer(Box::new(
                <DedustPoolV1JettonInternalTransferMsg>::read(p)?,
            ))),
            0x9c610de3 => Ok(Self::SwapEvent(Box::new(<DedustPoolV1SwapEventMsg>::read(p)?))),
            0xb544f4a4 => Ok(Self::DepositEvent(Box::new(<DedustPoolV1DepositEventMsg>::read(p)?))),
            0x3aa870a6 => Ok(Self::WithdrawalEvent(Box::new(<DedustPoolV1WithdrawalEventMsg>::read(p)?))),
            _ => Ok(Self::Unknown(p.read_remaining()?)),
        }
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        match self {
            Self::PayOutFromPool(v) => v.write(b),
            Self::SwapPeer(v) => v.write(b),
            Self::FulfillDeposit(v) => v.write(b),
            Self::RejectDeposit(v) => v.write(b),
            Self::JettonInternalTransfer(v) => v.write(b),
            Self::SwapEvent(v) => v.write(b),
            Self::DepositEvent(v) => v.write(b),
            Self::WithdrawalEvent(v) => v.write(b),
            Self::Unknown(v) => b.write_cell(v),
        }
    }
}
