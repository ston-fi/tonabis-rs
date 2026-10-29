//! Getter result records and protocol-specific stack decoding.
use super::storage::{DedustV2CpmmAsset, DedustV2CpmmPoolReward};
use fastnum::{I512, U256};
use std::collections::HashMap;
use ton::ton_core::types::Coins;
use ton::{
    block_tlb::{FromTVMStack, TVMStack},
    errors::TonResult,
    tlb_adapters::{DictKeyAdapterTonHash, DictKeyAdapterUint, DictValAdapterTLB},
    ton_core::{FromTVMStack, cell::TonHash, types::TonAddress},
};

#[derive(Debug, Clone, PartialEq)]
pub struct DedustV2CpmmPoolGetPoolDataResult {
    /// Pool status enum returned as a signed 8-bit value.
    pub status: i8,
    pub deposit_active: bool,
    pub swap_active: bool,
    pub asset_x: Option<TonAddress>,
    pub asset_y: Option<TonAddress>,
    pub wallets_by_assets: HashMap<TonHash, TonAddress>,
    pub assets_by_wallets: HashMap<TonHash, DedustV2CpmmAsset>,
    pub wallets_resolutions: HashMap<TonHash, DedustV2CpmmAsset>,
    pub base_fee_bps: i32,
    pub reserve_x: Coins,
    pub reserve_y: Coins,
    pub liquidity: Coins,
    pub protocol_fee_x: Coins,
    pub protocol_fee_y: Coins,
    pub creator_fee_x: Coins,
    pub creator_fee_y: Coins,
    pub x_fee_per_liquidity: U256,
    pub y_fee_per_liquidity: U256,
    pub rewards: HashMap<u8, DedustV2CpmmPoolReward>,
}
impl FromTVMStack for DedustV2CpmmPoolGetPoolDataResult {
    fn from_stack(stack: &mut TVMStack) -> TonResult<Self> {
        let rewards = crate::contracts::utils::dictionary_stack::<
            DictKeyAdapterUint<u8>,
            DictValAdapterTLB<DedustV2CpmmPoolReward>,
        >(stack, 2)?;
        let y_fee_per_liquidity = U256::from_stack(stack)?;
        let x_fee_per_liquidity = U256::from_stack(stack)?;
        let creator_fee_y = Coins::from_stack(stack)?;
        let creator_fee_x = Coins::from_stack(stack)?;
        let protocol_fee_y = Coins::from_stack(stack)?;
        let protocol_fee_x = Coins::from_stack(stack)?;
        let liquidity = Coins::from_stack(stack)?;
        let reserve_y = Coins::from_stack(stack)?;
        let reserve_x = Coins::from_stack(stack)?;
        let base_fee_bps = i32::from_stack(stack)?;
        let wallets_resolutions = crate::contracts::utils::dictionary_stack::<
            DictKeyAdapterTonHash,
            DictValAdapterTLB<DedustV2CpmmAsset>,
        >(stack, 256)?;
        let assets_by_wallets = crate::contracts::utils::dictionary_stack::<
            DictKeyAdapterTonHash,
            DictValAdapterTLB<DedustV2CpmmAsset>,
        >(stack, 256)?;
        let wallets_by_assets = crate::contracts::utils::dictionary_stack::<
            DictKeyAdapterTonHash,
            crate::contracts::utils::DictValueInternalAddress,
        >(stack, 256)?;
        let asset_y = crate::contracts::utils::stack_nullable_address(stack)?;
        let asset_x = crate::contracts::utils::stack_nullable_address(stack)?;
        let swap_active = <bool>::from_stack(stack)?;
        let deposit_active = <bool>::from_stack(stack)?;
        let status = i8::from_stack(stack)?;
        stack.ensure_empty()?;
        Ok(Self {
            status,
            deposit_active,
            swap_active,
            asset_x,
            asset_y,
            wallets_by_assets,
            assets_by_wallets,
            wallets_resolutions,
            base_fee_bps,
            reserve_x,
            reserve_y,
            liquidity,
            protocol_fee_x,
            protocol_fee_y,
            creator_fee_x,
            creator_fee_y,
            x_fee_per_liquidity,
            y_fee_per_liquidity,
            rewards,
        })
    }
}

/// Captured nominal liquidity valuation. Zero pool liquidity returns zeroes; signed inputs and amounts above total are arithmetic getter values, not executable withdrawal authorization.
#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustV2CpmmPoolGetLiquidityAmountsResult {
    pub amount_x: I512,
    pub amount_y: I512,
}
