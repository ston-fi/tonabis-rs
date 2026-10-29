//! generation records.

/// Captured generation; later versions require new evidence, even with equal stack length.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StonksGeneration {
    /// STON target, catalog-era version8.
    StonV8,
    /// STON target, first legacy factory's version10.
    StonV10,
    /// STON target, current factory's version11.
    StonV11,
    /// DeDust target, both guide legacy factories' version7.
    DeDustV7,
    /// DeDust target, current factory's version12.
    DeDustV12,
}

impl StonksGeneration {
    /// Exact version expected for this explicitly selected family.
    pub fn version(self) -> u32 {
        match self {
            Self::StonV8 => 8,
            Self::StonV10 => 10,
            Self::StonV11 => 11,
            Self::DeDustV7 => 7,
            Self::DeDustV12 => 12,
        }
    }
}
