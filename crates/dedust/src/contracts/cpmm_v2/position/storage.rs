//! Persistent CPMM position state and its fee/reward records.
use fastnum::U256;
use std::collections::HashMap;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::{
    tlb_adapters::{DictKeyAdapterUint, DictValAdapterTLB, TLBHashMapE},
    ton_core::{
        TLB,
        types::tlb_core::{TLBCoins, TLBRef, VarLenBytes},
    },
};

/// Complete persistent position state. Locked liquidity is retained separately.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmPositionStorage {
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    pub total_liquidity: TLBCoins,
    pub locked_liquidity: TLBCoins,
    pub fees: TLBRef<DedustV2CpmmPositionFees>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u8>, DictValAdapterTLB<DedustV2CpmmPositionReward>>::new(2)")]
    pub rewards: HashMap<u8, DedustV2CpmmPositionReward>,
}

/// Checkpoint and two pending balances per asset, in exact serialized order.
/// The second pending balance is kept distinct; no entitlement is inferred from decoding.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmPositionFees {
    pub x_checkpoint: VarLenBytes<U256, 5>,
    pub x_pending: TLBCoins,
    pub x_pending_secondary: TLBCoins,
    pub y_checkpoint: VarLenBytes<U256, 5>,
    pub y_pending: TLBCoins,
    pub y_pending_secondary: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmPositionReward {
    pub unclaimed: TLBCoins,
    pub checkpoint: VarLenBytes<U256, 5>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustV2CpmmPositionRewards {
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u8>, DictValAdapterTLB<DedustV2CpmmPositionReward>>::new(2)")]
    pub entries: HashMap<u8, DedustV2CpmmPositionReward>,
}
