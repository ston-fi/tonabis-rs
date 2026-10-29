//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use super::storage::{
    DedustPoolV2DepositLiquidityAllAmounts, DedustPoolV2LiquidityDepositProof, DedustPoolV2PoolProof,
    DedustPoolV2PoolStateAssets, DedustPoolV2SwapEventTail, DedustPoolV2SwapParams, DedustPoolV2SwapStepParams,
    DedustPoolV2VaultProof,
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
/// Vault-to-Pool swap message for the first step of a user swap.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1643009069, bits_len = 32)]
pub struct DedustPoolV2SwapExternalMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustPoolV2VaultProof>,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender_addr: TonAddress,
    pub current: DedustPoolV2SwapStepParams,
    pub swap_params: TLBRef<DedustPoolV2SwapParams>,
}
/// Pool-to-Pool peer swap message for later hops in a multi-hop route.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1923917994, bits_len = 32)]
pub struct DedustPoolV2SwapPeerMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustPoolV2PoolProof>,
    pub asset: DedustClassicAsset,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender_addr: TonAddress,
    pub current: DedustPoolV2SwapStepParams,
    pub swap_params: TLBRef<DedustPoolV2SwapParams>,
}
/// Liquidity Deposit-to-Pool callback that supplies both assets and requests LP minting.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3043726744, bits_len = 32)]
pub struct DedustPoolV2DepositLiquidityAllMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustPoolV2LiquidityDepositProof>,
    #[tlb(adapter = "InternalAddress")]
    pub owner_addr: TonAddress,
    pub min_lp_amount: TLBCoins,
    pub deposit_amounts: TLBRef<DedustPoolV2DepositLiquidityAllAmounts>,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
}
/// LP Jetton burn notification accepted by Pool-as-LP-minter to withdraw reserves.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2078119902, bits_len = 32)]
pub struct DedustPoolV2JettonBurnNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
}
/// Authorized update of the quote provider for one Pool asset.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2880727869, bits_len = 32)]
pub struct DedustPoolV2ConfigureQuoteProviderMsg {
    pub query_id: u64,
    pub asset: DedustClassicAsset,
    #[tlb(adapter = "NullableAddress")]
    pub quote_provider_addr: Option<TonAddress>,
}
/// Quote-provider request to refresh the cached quote for one Pool asset.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1752418572, bits_len = 32)]
pub struct DedustPoolV2SyncAssetQuoteMsg {
    pub query_id: u64,
    pub asset: DedustClassicAsset,
}
/// Request for the current reserves, LP supply, and optionally both Pool assets.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 1847882381, bits_len = 32)]
pub struct DedustPoolV2ProvidePoolStateMsg {
    pub query_id: u64,
    pub include_assets: bool,
}
/// Authorized update of the Pool trade-fee numerator.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3222612351, bits_len = 32)]
pub struct DedustPoolV2ConfigureTradeFeeMsg {
    pub query_id: u64,
    /// New trade-fee numerator; the denominator is fixed at 10000.
    pub trade_fee: u16,
}
/// Authorized request to return excess balance to a destination.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 650525367, bits_len = 32)]
pub struct DedustPoolV2ResetGasMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub response_destination: TonAddress,
}
/// Pool-to-Vault payout request used by swap and withdrawal flows.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2907617013, bits_len = 32)]
pub struct DedustPoolV2PayOutFromPoolMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustPoolV2PoolProof>,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub recipient_addr: Option<TonAddress>,
    pub payload: Option<TLBRef<TonCell>>,
}
/// Pool-state reply containing current reserves, LP supply, and optional assets.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3185396052, bits_len = 32)]
pub struct DedustPoolV2TakePoolStateMsg {
    pub query_id: u64,
    pub reserve0: TLBCoins,
    pub reserve1: TLBCoins,
    pub total_supply: TLBCoins,
    pub assets: Option<TLBRef<DedustPoolV2PoolStateAssets>>,
}
/// Pool-to-Liquidity-Deposit success callback after LP accounting succeeds.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2867302998, bits_len = 32)]
pub struct DedustPoolV2FulfillDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustPoolV2PoolProof>,
    pub asset0_deposit: TLBCoins,
    pub asset1_deposit: TLBCoins,
}
/// Pool-to-Liquidity-Deposit rejection callback when LP minting constraints fail.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3785583828, bits_len = 32)]
pub struct DedustPoolV2RejectDepositMsg {
    pub query_id: u64,
    pub proof: TLBRef<DedustPoolV2PoolProof>,
    pub payload: Option<TLBRef<TonCell>>,
}
/// LP wallet mint transfer emitted by Pool-as-LP-minter after successful liquidity deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 395134233, bits_len = 32)]
pub struct DedustPoolV2JettonInternalTransferMsg {
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
pub struct DedustPoolV2SwapEventMsg {
    pub asset_in: DedustClassicAsset,
    pub asset_out: DedustClassicAsset,
    pub amount_in: TLBCoins,
    pub amount_out: TLBCoins,
    pub details: TLBRef<DedustPoolV2SwapEventTail>,
}
/// External event emitted after successful liquidity deposit.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 3041195172, bits_len = 32)]
pub struct DedustPoolV2DepositEventMsg {
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
pub struct DedustPoolV2WithdrawalEventMsg {
    #[tlb(adapter = "InternalAddress")]
    pub sender_addr: TonAddress,
    pub liquidity: TLBCoins,
    pub amount0: TLBCoins,
    pub amount1: TLBCoins,
    pub reserve0: TLBCoins,
    pub reserve1: TLBCoins,
}
/// Operator command withdrawing accumulated protocol fees to the given recipients.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 188915538, bits_len = 32)]
pub struct DedustPoolV2CollectFeesMsg {
    pub query_id: u64,
    pub asset0_amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub asset0_destination: TonAddress,
    pub asset0_payload: Option<TLBRef<TonCell>>,
    pub asset1_amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub asset1_destination: Option<TonAddress>,
    pub asset1_payload: Option<TLBRef<TonCell>>,
}
/// Operator command setting the time after which swaps are accepted.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 2128082638, bits_len = 32)]
pub struct DedustPoolV2ConfigureStartTimeMsg {
    pub query_id: u64,
    pub start_at: u32,
}
/// Fixed quote-provider response omitted from the catalog. Captured function147 authenticates one configured provider, loads this complete body, then replaces normalization factors. Parsing proves no authority.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 172098648, bits_len = 32)]
pub struct DedustPoolV2QuoteResponseMsg {
    pub query_id: u64,
    pub numerator: u128,
    pub denominator: u128,
    /// LDDICT/ENDS followed by DROP: the optional referenced cell is not inspected. Preserve its bytes without inventing a context ABI.
    pub ignored_context: Option<TLBRef<TonCell>>,
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
/// Manual dispatch preserves errors from fixed-prefix nested proofs.
#[derive(Debug, Clone, PartialEq)]
pub enum DedustPoolV2IncomingMsg {
    SwapExternal(Box<DedustPoolV2SwapExternalMsg>),
    SwapPeer(Box<DedustPoolV2SwapPeerMsg>),
    DepositLiquidityAll(Box<DedustPoolV2DepositLiquidityAllMsg>),
    JettonBurnNotification(Box<DedustPoolV2JettonBurnNotificationMsg>),
    ConfigureQuoteProvider(Box<DedustPoolV2ConfigureQuoteProviderMsg>),
    SyncAssetQuote(Box<DedustPoolV2SyncAssetQuoteMsg>),
    ProvidePoolState(Box<DedustPoolV2ProvidePoolStateMsg>),
    ConfigureTradeFee(Box<DedustPoolV2ConfigureTradeFeeMsg>),
    ResetGas(Box<DedustPoolV2ResetGasMsg>),
    CollectFees(Box<DedustPoolV2CollectFeesMsg>),
    ConfigureStartTime(Box<DedustPoolV2ConfigureStartTimeMsg>),
    QuoteResponse(Box<DedustPoolV2QuoteResponseMsg>),
    Unknown(TonCell),
}
impl TLB for DedustPoolV2IncomingMsg {
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        if p.data_bits_left()? < 32 {
            return Ok(Self::Unknown(p.read_remaining()?));
        }
        match p.clone().read_num::<u64>(32)? {
            0x61ee542d => Ok(Self::SwapExternal(Box::new(<DedustPoolV2SwapExternalMsg>::read(p)?))),
            0x72aca8aa => Ok(Self::SwapPeer(Box::new(<DedustPoolV2SwapPeerMsg>::read(p)?))),
            0xb56b9598 => Ok(Self::DepositLiquidityAll(Box::new(<DedustPoolV2DepositLiquidityAllMsg>::read(
                p,
            )?))),
            0x7bdd97de => Ok(Self::JettonBurnNotification(Box::new(
                <DedustPoolV2JettonBurnNotificationMsg>::read(p)?,
            ))),
            0xabb46b3d => Ok(Self::ConfigureQuoteProvider(Box::new(
                <DedustPoolV2ConfigureQuoteProviderMsg>::read(p)?,
            ))),
            0x6873c90c => Ok(Self::SyncAssetQuote(Box::new(<DedustPoolV2SyncAssetQuoteMsg>::read(p)?))),
            0x6e24728d => Ok(Self::ProvidePoolState(Box::new(<DedustPoolV2ProvidePoolStateMsg>::read(p)?))),
            0xc015297f => Ok(Self::ConfigureTradeFee(Box::new(<DedustPoolV2ConfigureTradeFeeMsg>::read(p)?))),
            0x26c63ab7 => Ok(Self::ResetGas(Box::new(<DedustPoolV2ResetGasMsg>::read(p)?))),
            0xb429f52 => Ok(Self::CollectFees(Box::new(<DedustPoolV2CollectFeesMsg>::read(p)?))),
            0x7ed7f6ce => Ok(Self::ConfigureStartTime(Box::new(<DedustPoolV2ConfigureStartTimeMsg>::read(
                p,
            )?))),
            0xa420458 => Ok(Self::QuoteResponse(Box::new(<DedustPoolV2QuoteResponseMsg>::read(p)?))),
            _ => Ok(Self::Unknown(p.read_remaining()?)),
        }
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        match self {
            Self::SwapExternal(v) => v.write(b),
            Self::SwapPeer(v) => v.write(b),
            Self::DepositLiquidityAll(v) => v.write(b),
            Self::JettonBurnNotification(v) => v.write(b),
            Self::ConfigureQuoteProvider(v) => v.write(b),
            Self::SyncAssetQuote(v) => v.write(b),
            Self::ProvidePoolState(v) => v.write(b),
            Self::ConfigureTradeFee(v) => v.write(b),
            Self::ResetGas(v) => v.write(b),
            Self::CollectFees(v) => v.write(b),
            Self::ConfigureStartTime(v) => v.write(b),
            Self::QuoteResponse(v) => v.write(b),
            Self::Unknown(v) => b.write_cell(v),
        }
    }
}
/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.
/// Manual dispatch preserves errors from fixed-prefix nested proofs.
#[derive(Debug, Clone, PartialEq)]
pub enum DedustPoolV2OutgoingMsg {
    PayOutFromPool(Box<DedustPoolV2PayOutFromPoolMsg>),
    TakePoolState(Box<DedustPoolV2TakePoolStateMsg>),
    SwapPeer(Box<DedustPoolV2SwapPeerMsg>),
    FulfillDeposit(Box<DedustPoolV2FulfillDepositMsg>),
    RejectDeposit(Box<DedustPoolV2RejectDepositMsg>),
    JettonInternalTransfer(Box<DedustPoolV2JettonInternalTransferMsg>),
    SwapEvent(Box<DedustPoolV2SwapEventMsg>),
    DepositEvent(Box<DedustPoolV2DepositEventMsg>),
    WithdrawalEvent(Box<DedustPoolV2WithdrawalEventMsg>),
    Unknown(TonCell),
}
impl TLB for DedustPoolV2OutgoingMsg {
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        if p.data_bits_left()? < 32 {
            return Ok(Self::Unknown(p.read_remaining()?));
        }
        match p.clone().read_num::<u64>(32)? {
            0xad4eb6f5 => Ok(Self::PayOutFromPool(Box::new(<DedustPoolV2PayOutFromPoolMsg>::read(p)?))),
            0xbddd4954 => Ok(Self::TakePoolState(Box::new(<DedustPoolV2TakePoolStateMsg>::read(p)?))),
            0x72aca8aa => Ok(Self::SwapPeer(Box::new(<DedustPoolV2SwapPeerMsg>::read(p)?))),
            0xaae79256 => Ok(Self::FulfillDeposit(Box::new(<DedustPoolV2FulfillDepositMsg>::read(p)?))),
            0xe1a36cd4 => Ok(Self::RejectDeposit(Box::new(<DedustPoolV2RejectDepositMsg>::read(p)?))),
            0x178d4519 => Ok(Self::JettonInternalTransfer(Box::new(
                <DedustPoolV2JettonInternalTransferMsg>::read(p)?,
            ))),
            0x9c610de3 => Ok(Self::SwapEvent(Box::new(<DedustPoolV2SwapEventMsg>::read(p)?))),
            0xb544f4a4 => Ok(Self::DepositEvent(Box::new(<DedustPoolV2DepositEventMsg>::read(p)?))),
            0x3aa870a6 => Ok(Self::WithdrawalEvent(Box::new(<DedustPoolV2WithdrawalEventMsg>::read(p)?))),
            _ => Ok(Self::Unknown(p.read_remaining()?)),
        }
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        match self {
            Self::PayOutFromPool(v) => v.write(b),
            Self::TakePoolState(v) => v.write(b),
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
