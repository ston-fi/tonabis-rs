//! Two-reference staking master storage and constrained initial phase.
use crate::contracts::staking_master::storage::{CoffeeStakingMasterState, CoffeeStakingMasterStateRefs};
use crate::contracts::wire::wire_error;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::{
    block_tlb::StateInit,
    ton_core::{
        cell::{CellBuilder, CellParser, TonCell},
        errors::TonCoreResult,
        traits::tlb::TLB as TlbTrait,
        types::tlb_core::{TLBCoins, TLBRef},
    },
};
/// Initialized storage. Message bodies and numerical dictionary values use the
/// canonical parent records; emission-role differences are documented separately.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub struct CoffeeStakingMasterSingleItemStorage {
    pub initialized: bool,
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub distributor: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub vault: TonAddress,
    pub state_refs: TLBRef<CoffeeStakingMasterStateRefs>,
    pub item_code: TLBRef<TonCell>,
}

/// Vault-derived initial identity with absent owner/distributor, zero aggregate
/// state, empty dictionaries and the caller's exact item code.
#[derive(Debug, Clone, PartialEq)]
pub struct CoffeeStakingMasterSingleItemInitialData {
    pub vault: TonAddress,
    pub item_code: TLBRef<TonCell>,
}
impl CoffeeStakingMasterSingleItemInitialData {
    pub fn state_init(&self, master_code: &TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(master_code.clone(), self.to_cell()?))
    }
}
fn empty_state() -> CoffeeStakingMasterStateRefs {
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
impl TlbTrait for CoffeeStakingMasterSingleItemInitialData {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        if bool::read(p)? || NullableAddress.read(p)?.is_some() || NullableAddress.read(p)?.is_some() {
            return Err(wire_error("initial master must be uninitialized with absent owner/distributor"));
        }
        let vault = InternalAddress.read(p)?;
        let state_refs = TLBRef::<CoffeeStakingMasterStateRefs>::read(p)?;
        if *state_refs != empty_state() {
            return Err(wire_error(
                "initial master must have zero aggregate state and empty dictionaries",
            ));
        }
        Ok(Self {
            vault,
            item_code: TLBRef::read(p)?,
        })
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        false.write(b)?;
        NullableAddress.write(b, &None)?;
        NullableAddress.write(b, &None)?;
        InternalAddress.write(b, &self.vault)?;
        TLBRef::new(empty_state()).write(b)?;
        self.item_code.write(b)
    }
}
