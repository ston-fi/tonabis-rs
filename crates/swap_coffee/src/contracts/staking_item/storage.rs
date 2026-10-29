//! Coffee storage. See tools/reference/swap_coffee/README.md.
use crate::contracts::{staking::CoffeeStakingPeriod, wire::wire_error};
use std::collections::HashMap;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::{
    block_tlb::StateInit,
    tlb_adapters::{DictKeyAdapterUint, DictValAdapterTLB, TLBHashMapE},
    ton_core::{
        TLB,
        cell::{CellBuilder, CellParser, TonCell},
        errors::TonCoreResult,
        traits::tlb::TLB,
        types::tlb_core::{TLBCoins, TLBRef},
    },
};
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeStakingItemStorage {
    pub initialized: bool,
    pub nft_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub master: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    pub asset: TLBRef<CoffeeStakingAssetData>,
    pub info: TLBRef<CoffeeStakingPositionData>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u32>, DictValAdapterTLB<TLBRef<CoffeeStakingPeriod>>>::new(32)")]
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeStakingAssetData {
    #[tlb(adapter = "NullableAddress")]
    pub wallet: Option<TonAddress>,
    pub amount: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeStakingPositionData {
    pub user_points: TLBCoins,
    pub additional_points: TLBCoins,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub period_id: u32,
}

// Pre-initialization NFT identity, distinct from initialized item storage.
/// Identity data observed in master-created NFT StateInit. Uninitialized owner
/// and wallet are addr_none; asset amount, position fields and periods are zero.
#[derive(Debug, Clone, PartialEq)]
pub struct CoffeeStakingItemInitialData {
    pub nft_id: u64,
    pub master: TonAddress,
}
impl CoffeeStakingItemInitialData {
    /// Derive with explicit caller-owned item code; no code selection/authentication.
    pub fn state_init(&self, item_code: &TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(item_code.clone(), self.to_cell()?))
    }
}
impl TLB for CoffeeStakingItemInitialData {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        if bool::read(p)? {
            return Err(wire_error("initialized NFT is not InitialData"));
        }
        let nft_id = u64::read(p)?;
        let master = InternalAddress.read(p)?;
        if NullableAddress.read(p)?.is_some() {
            return Err(wire_error("initial NFT owner must be absent"));
        }
        let asset = TLBRef::<TonCell>::read(p)?;
        let mut a = asset.parser();
        if NullableAddress.read(&mut a)?.is_some() || TLBCoins::read(&mut a)?.to_u128() != 0 {
            return Err(wire_error("initial NFT asset must be empty"));
        }
        let info = TLBRef::<CoffeeStakingPositionData>::read(p)?;
        if info.user_points.to_u128() != 0
            || info.additional_points.to_u128() != 0
            || info.start_timestamp != 0
            || info.end_timestamp != 0
            || info.period_id != 0
        {
            return Err(wire_error("initial NFT position must be zero"));
        }
        if bool::read(p)? {
            return Err(wire_error("initial NFT periods must be empty"));
        }
        Ok(Self { nft_id, master })
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        false.write(b)?;
        self.nft_id.write(b)?;
        InternalAddress.write(b, &self.master)?;
        NullableAddress.write(b, &None)?;
        let mut asset = TonCell::builder();
        NullableAddress.write(&mut asset, &None)?;
        TLBCoins::new(0).write(&mut asset)?;
        b.write_ref(asset.build()?)?;
        TLBRef::new(CoffeeStakingPositionData {
            user_points: TLBCoins::new(0),
            additional_points: TLBCoins::new(0),
            start_timestamp: 0,
            end_timestamp: 0,
            period_id: 0,
        })
        .write(b)?;
        false.write(b)
    }
}
