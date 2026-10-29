//! Deterministic initial state for the two CPMM pool generations and their positions.
use super::{pool, position};
use fastnum::U256;
use std::collections::HashMap;
use ton::ton_core::types::TonAddress;
use ton::{
    block_tlb::StateInit,
    ton_core::{
        cell::{TonCell, TonHash},
        errors::TonCoreError,
        traits::tlb::TLB,
        types::tlb_core::{TLBCoins, TLBRef, VarLenBytes},
    },
};

/// Contract code generation used for deterministic deployment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DedustV2CpmmRevision {
    PoolV1,
    PoolV2,
}

pub fn pool_code(revision: DedustV2CpmmRevision) -> Result<TonCell, TonCoreError> {
    let hash = match revision {
        DedustV2CpmmRevision::PoolV1 => TonHash::from_slice_sized(&[
            0x87, 0xb5, 0x66, 0xc0, 0x19, 0xa1, 0xc6, 0xfc, 0x69, 0x1d, 0xcc, 0x50, 0x42, 0x55, 0x93, 0x68, 0xdb, 0x75,
            0x54, 0x28, 0xef, 0x8e, 0x0f, 0x45, 0xeb, 0x23, 0xad, 0xe0, 0x11, 0x8c, 0x4c, 0x36,
        ]),
        DedustV2CpmmRevision::PoolV2 => TonHash::from_slice_sized(&[
            0x60, 0x45, 0xe6, 0x7b, 0x61, 0x7e, 0x73, 0x48, 0x6a, 0xac, 0x44, 0x5c, 0xfc, 0xad, 0x0d, 0xbe, 0xad, 0x6d,
            0x31, 0x5c, 0xcc, 0x8f, 0x4a, 0xed, 0xe3, 0xf2, 0xe2, 0x3a, 0xd1, 0x4d, 0xa4, 0xb0,
        ]),
    };
    crate::contracts::utils::library(hash)
}
/// Controller is explicit; no embedded SDK address is selected on the caller's behalf.
pub fn pool_state_init(
    config: pool::storage::DedustV2CpmmPoolConfig,
    controller: TonAddress,
    revision: DedustV2CpmmRevision,
) -> Result<StateInit, TonCoreError> {
    let zero = || VarLenBytes::from_value(U256::ZERO, 0);
    let storage = pool::storage::DedustV2CpmmStorage {
        config: TLBRef::new(config),
        fees: TLBRef::new(pool::storage::DedustV2CpmmPoolFees {
            protocol_fee_x: TLBCoins::new(0),
            protocol_fee_y: TLBCoins::new(0),
            creator_fee_x: TLBCoins::new(0),
            creator_fee_y: TLBCoins::new(0),
            x_fee_per_token: zero(),
            y_fee_per_token: zero(),
        }),
        extra: TLBRef::new(pool::storage::DedustV2CpmmPoolExtra {
            controller,
            wallets: pool::storage::DedustV2CpmmPoolWallets {
                wallets_by_assets: HashMap::new(),
                assets_by_wallets: HashMap::new(),
                resolutions: HashMap::new(),
            },
        }),
        rewards: pool::storage::DedustV2CpmmPoolRewards {
            entries: HashMap::new(),
        },
        status: pool::storage::DedustV2CpmmStatusUninitialized {}.into(),
        deposit_active: false,
        swap_active: false,
        liquidity: TLBCoins::new(0),
        reserve_x: TLBCoins::new(0),
        reserve_y: TLBCoins::new(0),
    };
    Ok(StateInit::new(pool_code(revision)?, storage.to_cell()?))
}
/// Library identity is embedded in both pinned pool revisions. Eight shard bits
/// are inherited from the pool; owner and pool remain part of the data commitment.
pub fn position_state_init(pool: TonAddress, owner: TonAddress) -> Result<(StateInit, TonAddress), TonCoreError> {
    let zero = || VarLenBytes::from_value(U256::ZERO, 0);
    let data = position::storage::DedustV2CpmmPositionStorage {
        pool,
        owner,
        total_liquidity: TLBCoins::new(0),
        locked_liquidity: TLBCoins::new(0),
        fees: TLBRef::new(position::storage::DedustV2CpmmPositionFees {
            x_checkpoint: zero(),
            x_pending: TLBCoins::new(0),
            x_pending_secondary: TLBCoins::new(0),
            y_checkpoint: zero(),
            y_pending: TLBCoins::new(0),
            y_pending_secondary: TLBCoins::new(0),
        }),
        rewards: HashMap::new(),
    };
    let hash = TonHash::from_slice_sized(&[
        0xdd, 0x82, 0xf2, 0x4d, 0xb6, 0x14, 0x79, 0x8e, 0xe7, 0xc5, 0x79, 0xf8, 0xb3, 0xf0, 0x7f, 0x06, 0x45, 0xd0,
        0x6d, 0x65, 0xce, 0xde, 0x36, 0x8d, 0x80, 0xd0, 0xd7, 0x4b, 0x18, 0x0d, 0x2d, 0xd6,
    ]);
    let init = StateInit::new(crate::contracts::utils::library(hash)?, data.to_cell()?);
    crate::contracts::utils::same_shard(init, &pool, 8)
}

/// Deposit identity commits fixed constraints, payout options, pool and initiator.
/// Both pool revisions use library2cac3fdd and inherit eight pool shard bits.
/// The initial balances are zero. Finalization/refund destroys the child after
/// forwarding its remaining value; this constructor does not send or fund it.
pub fn deposit_state_init(
    config: super::deposit::storage::DedustV2CpmmDepositConfig,
    pool: TonAddress,
    initiator: TonAddress,
) -> Result<(StateInit, TonAddress), TonCoreError> {
    let data = super::deposit::storage::DedustV2CpmmDepositStorage {
        config: TLBRef::new(config),
        pool,
        initiator,
        amount_x: TLBCoins::new(0),
        amount_y: TLBCoins::new(0),
    };
    let library = TonHash::from_slice_sized(&[
        0x2c, 0xac, 0x3f, 0xdd, 0xd3, 0x09, 0x69, 0xd0, 0x8d, 0xf0, 0x36, 0x06, 0x71, 0x08, 0xc6, 0xe7, 0xd6, 0x97,
        0x80, 0xa9, 0x45, 0x9d, 0x93, 0x1d, 0x2e, 0xb6, 0x3d, 0x95, 0xd5, 0xff, 0x68, 0x25,
    ]);
    let init = StateInit::new(crate::contracts::utils::library(library)?, data.to_cell()?);
    crate::contracts::utils::same_shard(init, &pool, 8)
}
