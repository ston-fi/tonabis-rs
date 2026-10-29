//! Caller-owned provider bindings and tolerant, generation-specific wire records.
//! Decoding does not establish sender authority, code identity or settlement.
pub mod classic;
pub mod cpmm_v2;
pub mod custom_wallet;
pub mod operator;
pub mod signature_ref;
pub mod uranus;

mod utils;
