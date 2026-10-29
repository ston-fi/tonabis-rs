//! Persistent state of the classic operator-managed pool.
use crate::contracts::classic::pool_v2;
use ton::ton_core::{
    cell::{CellBuilder, CellParser},
    errors::TonCoreError,
    traits::tlb::TLB as Tlb,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DedustClassicOperatorPoolStorage {
    pub pool: pool_v2::storage::DedustPoolV2Storage,
    /// Absent legacy tail loads as zero. Preserve absence versus explicit zero.
    pub swap_active_since: Option<u32>,
}
impl Tlb for DedustClassicOperatorPoolStorage {
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        let pool = pool_v2::storage::DedustPoolV2Storage::read(p)?;
        let swap_active_since = if p.data_bits_left()? > 0 {
            Some(p.read_num(32)?)
        } else {
            None
        };

        Ok(Self {
            pool,
            swap_active_since,
        })
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        self.pool.write(b)?;
        if let Some(time) = self.swap_active_since {
            b.write_num(&time, 32)?;
        }
        Ok(())
    }
}
