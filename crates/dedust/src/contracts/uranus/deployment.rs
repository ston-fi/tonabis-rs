//! Deterministic current Uranus wallet state and shard-aware address commitment.
use super::wallet_v3::storage::DedustUranusMemeWalletV3Storage;
use ton::ton_core::types::TonAddress;
use ton::{
    block_tlb::StateInit,
    ton_core::{cell::TonHash, errors::TonCoreError, traits::tlb::TLB, types::tlb_core::TLBCoins},
};

/// Contract code generation used for deterministic deployment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DedustUranusRevision {
    V2,
    V3,
}

/// Commit a FactoryV3-created Meme's typed initial state. The seed is already
/// present in `storage.config`; callers obtain it from an observation or supply
/// their own test value. This function does not reproduce the factory's TVM
/// random generator, authenticate the state or predict an unknown future seed.
pub fn meme_state_init(
    storage: &super::meme_v3::storage::DedustUranusMemeV3Storage,
    factory: &TonAddress,
) -> Result<(StateInit, TonAddress), TonCoreError> {
    let hash = TonHash::from_slice_sized(&[
        0xa6, 0x3e, 0x4a, 0x80, 0x2c, 0x8b, 0x98, 0x2c, 0xd8, 0x88, 0xa3, 0x57, 0xb2, 0xc9, 0xee, 0x5a, 0x72, 0x26,
        0x44, 0xc2, 0xd7, 0xe7, 0xcd, 0xae, 0xd3, 0x50, 0xf2, 0x4f, 0xc9, 0x6e, 0xca, 0x3d,
    ]);
    let init = StateInit::new(crate::contracts::utils::library(hash)?, storage.to_cell()?);
    crate::contracts::utils::same_shard(init, factory, 8)
}

pub fn wallet_state_init(
    meme: TonAddress,
    owner: TonAddress,
    revision: DedustUranusRevision,
) -> Result<(StateInit, TonAddress), TonCoreError> {
    let hash = match revision {
        DedustUranusRevision::V2 => TonHash::from_slice_sized(&[
            0xee, 0xfc, 0xe5, 0xce, 0x3d, 0x1e, 0x55, 0x5a, 0x50, 0x10, 0xa3, 0x09, 0x34, 0x8c, 0x96, 0x5d, 0xfa, 0xf7,
            0x2e, 0xbd, 0xf3, 0x23, 0x3b, 0x4f, 0x69, 0x1f, 0xa3, 0xd3, 0x7b, 0xdb, 0x56, 0x05,
        ]),
        DedustUranusRevision::V3 => TonHash::from_slice_sized(&[
            0x89, 0x1e, 0x52, 0x28, 0xe4, 0x30, 0x0a, 0xe3, 0xff, 0x22, 0x58, 0x31, 0x63, 0xbb, 0xd5, 0x0e, 0x3e, 0x1e,
            0x8c, 0xd2, 0x30, 0x4c, 0x0d, 0x57, 0x16, 0x4f, 0x53, 0x18, 0xf9, 0xda, 0x12, 0x66,
        ]),
    };
    let data = DedustUranusMemeWalletV3Storage {
        balance: TLBCoins::new(0),
        owner_address: owner,
        meme_address: meme,
    };
    let init = StateInit::new(crate::contracts::utils::library(hash)?, data.to_cell()?);
    crate::contracts::utils::same_shard(init, &owner, 8)
}
