//! Coffee msgs. See tools/reference/swap_coffee/README.md.
use crate::contracts::staking_item::msgs::CoffeeStakingItemInitMsg;
use crate::contracts::{
    staking::{CoffeeStakingJettonConfiguration, CoffeeStakingPeriod},
    staking_item::storage::CoffeeStakingAssetData,
};
use std::collections::HashMap;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::{
    tlb_adapters::{DictKeyAdapterTonHash, DictKeyAdapterUint, DictValAdapterTLB, TLBHashMapE},
    ton_core::{
        TLB,
        cell::{TonCell, TonHash},
        types::tlb_core::{TLBCoins, TLBEither, TLBRef},
    },
};
use tonabis_core::msgs::EmptyBodyMsg;
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeStakingForwardData {
    pub gas: TLBCoins,
    pub payload: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xbe5a7595, bits_len = 32)]
pub struct CoffeeStakingMasterInitMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub distributor: TonAddress,
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB<TLBRef<CoffeeStakingJettonConfiguration>>>::new(256)"
    )]
    pub jettons: HashMap<TonHash, TLBRef<CoffeeStakingJettonConfiguration>>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u32>, DictValAdapterTLB<TLBRef<CoffeeStakingPeriod>>>::new(32)")]
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0a9577f0, bits_len = 32)]
pub struct CoffeeStakingUpdateRewardsMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub jetton_wallet: TonAddress,
    pub jetton_amount: TLBCoins,
    pub duration: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xf9471134, bits_len = 32)]
pub struct CoffeeStakingDepositMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    pub jetton_amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub from_user: TonAddress,
    pub period_id: u32,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xb30c7310, bits_len = 32)]
pub struct CoffeeStakingClaimRewardsMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub jetton_wallet: TonAddress,
    pub jetton_amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub receiver: TonAddress,
    pub payload: Option<TLBRef<CoffeeStakingForwardData>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x27ee4c7d, bits_len = 32)]
pub struct CoffeeStakingSetDistributorMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub distributor: Option<TonAddress>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xf1eef33c, bits_len = 32)]
pub struct CoffeeStakingSetOwnerMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xeec15c2a, bits_len = 32)]
pub struct CoffeeStakingSetJettonsMsg {
    pub query_id: u64,
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB<TLBRef<CoffeeStakingJettonConfiguration>>>::new(256)"
    )]
    pub jettons: HashMap<TonHash, TLBRef<CoffeeStakingJettonConfiguration>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xfab30dc2, bits_len = 32)]
pub struct CoffeeStakingSetPeriodsMsg {
    pub query_id: u64,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u32>, DictValAdapterTLB<TLBRef<CoffeeStakingPeriod>>>::new(32)")]
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xce4e2066, bits_len = 32)]
pub struct CoffeeStakingUpdateContractMsg {
    pub query_id: u64,
    pub new_code: TLBRef<TonCell>,
    pub new_data: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xeb666ce0, bits_len = 32)]
pub struct CoffeeStakingItemExtendedMsg {
    pub query_id: u64,
    pub nft_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    pub amount: TLBCoins,
    pub end_timestamp: u64,
    pub period_id: u32,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcb03bfaf, bits_len = 32)]
pub struct CoffeeStakingItemUnlockedMsg {
    pub query_id: u64,
    pub nft_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    pub amount: TLBCoins,
    pub asset_data: TLBRef<CoffeeStakingAssetData>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xbe5a7595, bits_len = 32)]
pub struct CoffeeStakingMasterInitializedMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0a9577f0, bits_len = 32)]
pub struct CoffeeStakingRewardsUpdatedMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub jetton_wallet: TonAddress,
    pub duration: u64,
    pub finish_at: u64,
    pub rewards_rate: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xb30c7310, bits_len = 32)]
pub struct CoffeeStakingRewardsClaimedMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub jetton_wallet: TonAddress,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub receiver: TonAddress,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x27ee4c7d, bits_len = 32)]
pub struct CoffeeStakingDistributorUpdatedMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xf1eef33c, bits_len = 32)]
pub struct CoffeeStakingOwnerUpdatedMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xeec15c2a, bits_len = 32)]
pub struct CoffeeStakingJettonsUpdatedMsg {
    pub query_id: u64,
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB<TLBRef<CoffeeStakingJettonConfiguration>>>::new(256)"
    )]
    pub jettons: HashMap<TonHash, TLBRef<CoffeeStakingJettonConfiguration>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xfab30dc2, bits_len = 32)]
pub struct CoffeeStakingPeriodsUpdatedMsg {
    pub query_id: u64,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u32>, DictValAdapterTLB<TLBRef<CoffeeStakingPeriod>>>::new(32)")]
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub enum CoffeeStakingMasterIncomingMessage {
    Init(CoffeeStakingMasterInitMsg),
    CoffeeStakingUpdateRewards(CoffeeStakingUpdateRewardsMsg),
    CoffeeStakingDeposit(CoffeeStakingDepositMsg),
    CoffeeStakingClaimRewards(CoffeeStakingClaimRewardsMsg),
    CoffeeStakingSetDistributor(CoffeeStakingSetDistributorMsg),
    CoffeeStakingSetOwner(CoffeeStakingSetOwnerMsg),
    CoffeeStakingSetJettons(CoffeeStakingSetJettonsMsg),
    CoffeeStakingSetPeriods(CoffeeStakingSetPeriodsMsg),
    CoffeeStakingUpdateContract(CoffeeStakingUpdateContractMsg),
    CoffeeStakingItemExtended(CoffeeStakingItemExtendedMsg),
    CoffeeStakingItemUnlocked(CoffeeStakingItemUnlockedMsg),
    Empty(EmptyBodyMsg),

    Unknown(TonCell),
}

/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub enum CoffeeStakingMasterOutgoingMessage {
    Initialized(CoffeeStakingMasterInitializedMsg),
    CoffeeStakingRewardsUpdated(CoffeeStakingRewardsUpdatedMsg),
    CoffeeStakingRewardsClaimed(CoffeeStakingRewardsClaimedMsg),
    CoffeeStakingDistributorUpdated(CoffeeStakingDistributorUpdatedMsg),
    CoffeeStakingOwnerUpdated(CoffeeStakingOwnerUpdatedMsg),
    CoffeeStakingJettonsUpdated(CoffeeStakingJettonsUpdatedMsg),
    CoffeeStakingPeriodsUpdated(CoffeeStakingPeriodsUpdatedMsg),
    CoffeeStakingItemExtended(CoffeeStakingItemExtendedMsg),
    CoffeeStakingItemUnlocked(CoffeeStakingItemUnlockedMsg),
    DepositLog(CoffeeStakingMasterDepositLogMsg),

    Unknown(TonCell),
}

// Master emissions omitted from the catalog's original outgoing enum.
// Layouts were independently reconstructed from original local transactions and
// confirmed against pinned captured-code field construction (see provenance).

/// Summary attached to a successful staking deposit log.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeStakingMasterDepositSummary {
    pub total_points: TLBCoins,
    pub end_timestamp: u64,
    pub period_id: u32,
}

/// External deposit log. It shares an opcode with the incompatible incoming
/// deposit request, so callers select the external role before decoding it.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
#[tlb(prefix = 0xf9471134, bits_len = 32)]
pub struct CoffeeStakingMasterDepositLogMsg {
    pub query_id: u64,
    pub nft_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    pub summary: TLBRef<CoffeeStakingMasterDepositSummary>,
    pub asset_data: Option<TLBRef<CoffeeStakingAssetData>>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u32>, DictValAdapterTLB<TLBRef<CoffeeStakingPeriod>>>::new(32)")]
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
}

/// Internal emission role, separate from external master-initialized events
/// sharing opcode0xbe5a7595. The payout, reward transfer and excess roles are separate from external logs.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub enum CoffeeStakingMasterInternalMessage {
    /// Empty extension response observed in the single-item generation.
    Empty(EmptyBodyMsg),
    ItemInit(CoffeeStakingItemInitMsg),
    UnlockToVault(CoffeeStakingMasterUnlockToVaultMsg),
    JettonTransfer(CoffeeStakingMasterJettonTransferMsg),
    Excesses(CoffeeStakingMasterExcessesMsg),

    Unknown(TonCell),
}

/// Master-to-vault payout instruction. The amount is locked token units; the
/// item's master callback carries points in its different same-opcode layout.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcb03bfaf, bits_len = 32)]
pub struct CoffeeStakingMasterUnlockToVaultMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub wallet: TonAddress,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
}
/// Standard excess response after an item extension.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct CoffeeStakingMasterExcessesMsg {
    pub query_id: u64,
}
/// TEP-74 reward transfer. Forward payload retains the source-defined Either
/// inline/reference payload, allowing caller-owned application contents.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
#[tlb(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct CoffeeStakingMasterJettonTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub destination: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}
