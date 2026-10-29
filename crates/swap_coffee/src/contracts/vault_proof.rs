//! Typed sender identities carried by vault payout requests.
//!
//! The two-bit tag selects an exact Coffee identity. Parsing is structural;
//! callers still authenticate the factory, Init code and derived sender address.
use crate::contracts::factory::msgs::CoffeeFactoryAsset;
use ton::ton_core::types::tlb_core::adapters::{ConstLen, InternalAddress};
use ton::ton_core::{
    cell::{CellBuilder, CellParser, TonCell},
    errors::TonCoreResult,
    traits::tlb::TLB,
    types::{TonAddress, tlb_core::TLBRef},
};
/// Common pair/strategy identity. AMM settings remain caller-supplied identity
/// bytes; interpreting them as numerical parameters requires a concrete family.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub struct CoffeeVaultPoolIdentity {
    pub asset1: CoffeeFactoryAsset,
    pub asset2: CoffeeFactoryAsset,
    #[tlb(bits_len = 3)]
    pub amm: u8,
    pub amm_settings: Option<TLBRef<TonCell>>,
}

/// Revision-defined child identity tags, with no guessed opcode registry.
#[derive(Debug, Clone, PartialEq)]
pub enum CoffeeVaultSenderIdentity {
    Vault {
        asset: CoffeeFactoryAsset,
    },
    Pool(CoffeeVaultPoolIdentity),
    Depository {
        owner: TonAddress,
        pool: TLBRef<CoffeeVaultPoolIdentity>,
    },
    Creator {
        owner: TonAddress,
        pool: TLBRef<CoffeeVaultPoolIdentity>,
    },
}
impl TLB for CoffeeVaultSenderIdentity {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        Ok(match p.read_num::<u8>(2)? {
            0 => Self::Vault {
                asset: CoffeeFactoryAsset::read(p)?,
            },
            1 => Self::Pool(CoffeeVaultPoolIdentity::read(p)?),
            2 => Self::Depository {
                owner: InternalAddress.read(p)?,
                pool: TLBRef::<CoffeeVaultPoolIdentity>::read(p)?,
            },
            _ => Self::Creator {
                owner: InternalAddress.read(p)?,
                pool: TLBRef::<CoffeeVaultPoolIdentity>::read(p)?,
            },
        })
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        match self {
            Self::Vault { asset } => {
                b.write_num(&0u8, 2)?;
                asset.write(b)
            },
            Self::Pool(pool) => {
                b.write_num(&1u8, 2)?;
                pool.write(b)
            },
            Self::Depository { owner, pool } => {
                b.write_num(&2u8, 2)?;
                InternalAddress.write(b, owner)?;
                pool.write(b)
            },
            Self::Creator { owner, pool } => {
                b.write_num(&3u8, 2)?;
                InternalAddress.write(b, owner)?;
                pool.write(b)
            },
        }
    }
}
/// Complete factory-bound identity. Sender verification is caller-owned.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub struct CoffeeVaultContractProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory: TonAddress,
    pub identity: CoffeeVaultSenderIdentity,
}
