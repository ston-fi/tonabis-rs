//! Getter result records and protocol-specific stack decoding.
use super::storage::DedustV2CpmmPositionReward;
use fastnum::{I512, U256};
use std::collections::HashMap;
use ton::ton_core::types::Coins;
use ton::ton_core::types::TonAddress;
use ton::{
    block_tlb::{FromTVMStack, TVMStack},
    errors::TonResult,
    tlb_adapters::{DictKeyAdapterUint, DictValAdapterTLB},
    ton_core::{FromTVMStack, cell::TonCell},
};
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, Clone, PartialEq)]
pub struct DedustV2CpmmPositionDataResult {
    pub pool: Option<TonAddress>,
    pub owner: Option<TonAddress>,
    pub total_liquidity: Coins,
    pub locked_liquidity: Coins,
    pub rewards: HashMap<u8, DedustV2CpmmPositionReward>,
    pub x_pending: Coins,
    pub y_pending: Coins,
    pub x_checkpoint: U256,
    pub y_checkpoint: U256,
}
impl FromTVMStack for DedustV2CpmmPositionDataResult {
    fn from_stack(stack: &mut TVMStack) -> TonResult<Self> {
        let y_checkpoint = U256::from_stack(stack)?;
        let x_checkpoint = U256::from_stack(stack)?;
        let y_pending = Coins::from_stack(stack)?;
        let x_pending = Coins::from_stack(stack)?;
        let rewards = crate::contracts::utils::dictionary_stack::<
            DictKeyAdapterUint<u8>,
            DictValAdapterTLB<DedustV2CpmmPositionReward>,
        >(stack, 2)?;
        let locked_liquidity = Coins::from_stack(stack)?;
        let total_liquidity = Coins::from_stack(stack)?;
        let owner = NullableInternalAddress.read(&mut TonCell::from_stack(stack)?.parser())?;
        let pool = NullableInternalAddress.read(&mut TonCell::from_stack(stack)?.parser())?;
        stack.ensure_empty()?;
        Ok(Self {
            pool,
            owner,
            total_liquidity,
            locked_liquidity,
            rewards,
            x_pending,
            y_pending,
            x_checkpoint,
            y_checkpoint,
        })
    }
}

/// Computed fees can be negative for checkpoints below the stored checkpoint.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustV2CpmmPositionAvailableFees {
    pub x_fee: I512,
    pub y_fee: I512,
}
