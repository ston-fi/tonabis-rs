//! Coffee storage. See tools/reference/swap_coffee/README.md.
use crate::contracts::{
    staking::{CoffeeStakingJettonConfiguration, CoffeeStakingPeriod, CoffeeStakingRewardSchedule},
    wire::wire_error,
};
use std::collections::HashMap;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::{
    block_tlb::StateInit,
    tlb_adapters::{DictKeyAdapterTonHash, DictKeyAdapterUint, DictValAdapterTLB, TLBHashMapE},
    ton_core::{
        TLB,
        cell::{CellBuilder, CellParser, TonCell, TonHash},
        errors::TonCoreResult,
        traits::tlb::TLB,
        types::tlb_core::{TLBCoins, TLBRef},
    },
};
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeStakingMasterStorage {
    pub initialized: bool,
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub distributor: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub vault: TonAddress,
    pub state_refs: TLBRef<CoffeeStakingMasterStateRefs>,
    pub item_code: TLBRef<TonCell>,
    pub alternate_item_code: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeStakingMasterStateRefs {
    pub state: TLBRef<CoffeeStakingMasterState>,
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB<TLBRef<CoffeeStakingRewardSchedule>>>::new(256)"
    )]
    pub rewards: HashMap<TonHash, TLBRef<CoffeeStakingRewardSchedule>>,
    #[tlb(
        adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB<TLBRef<CoffeeStakingJettonConfiguration>>>::new(256)"
    )]
    pub jettons: HashMap<TonHash, TLBRef<CoffeeStakingJettonConfiguration>>,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u32>, DictValAdapterTLB<TLBRef<CoffeeStakingPeriod>>>::new(32)")]
    pub periods: HashMap<u32, TLBRef<CoffeeStakingPeriod>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeStakingMasterState {
    pub total_points: TLBCoins,
    pub next_item_index: u64,
}

// Three-reference master pre-initialization data, separate from single_item.
/// Canonical zero master identity before the vault supplies owner/distributor.
#[derive(Debug, Clone, PartialEq)]
pub struct CoffeeStakingMasterInitialData {
    pub vault: TonAddress,
    pub item_code: TLBRef<TonCell>,
    pub alternate_item_code: TLBRef<TonCell>,
}
fn zero_state() -> CoffeeStakingMasterStateRefs {
    CoffeeStakingMasterStateRefs {
        state: TLBRef::new(CoffeeStakingMasterState {
            total_points: TLBCoins::new(0),
            next_item_index: 0,
        }),
        rewards: Default::default(),
        jettons: Default::default(),
        periods: Default::default(),
    }
}
impl CoffeeStakingMasterInitialData {
    pub fn state_init(&self, master_code: &TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(master_code.clone(), self.to_cell()?))
    }
}
impl TLB for CoffeeStakingMasterInitialData {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        if bool::read(p)? || NullableAddress.read(p)?.is_some() || NullableAddress.read(p)?.is_some() {
            return Err(wire_error("initial master requires absent owner/distributor"));
        }
        let vault = InternalAddress.read(p)?;
        let state = TLBRef::<CoffeeStakingMasterStateRefs>::read(p)?;
        if *state != zero_state() {
            return Err(wire_error("initial master requires zero state and empty dictionaries"));
        }
        Ok(Self {
            vault,
            item_code: TLBRef::read(p)?,
            alternate_item_code: TLBRef::read(p)?,
        })
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        false.write(b)?;
        NullableAddress.write(b, &None)?;
        NullableAddress.write(b, &None)?;
        InternalAddress.write(b, &self.vault)?;
        TLBRef::new(zero_state()).write(b)?;
        self.item_code.write(b)?;
        self.alternate_item_code.write(b)
    }
}
