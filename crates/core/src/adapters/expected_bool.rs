use ton_core::{
    cell::{CellBuilder, CellParser},
    errors::{TonCoreError, TonCoreResult},
    traits::tlb::TLB,
};

/// A literal boolean field selecting one specific record layout.
///
/// Rejects the opposite value on both read and write. Use this for payload or
/// storage-phase flags whose other value requires a different record type.
pub struct ExpectedBool<const EXPECTED: bool>;

impl<const EXPECTED: bool> ExpectedBool<EXPECTED> {
    /// Reads and validates the literal bit.
    pub fn read(&self, parser: &mut CellParser) -> TonCoreResult<bool> {
        let value = bool::read(parser)?;
        Self::validate(value)?;
        Ok(value)
    }

    /// Writes the bit only when it matches the record's layout.
    pub fn write(&self, builder: &mut CellBuilder, value: &bool) -> TonCoreResult<()> {
        Self::validate(*value)?;
        value.write(builder)
    }

    fn validate(value: bool) -> TonCoreResult<()> {
        if value != EXPECTED {
            return Err(TonCoreError::data(
                module_path!(),
                format!("layout requires boolean {EXPECTED}"),
            ));
        }
        Ok(())
    }
}
