use std::cmp::Ordering;
use ton::ton_core::errors::{TonCoreError, TonCoreResult};

use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;

/// Order wallets by descending hash of their complete serialized address cells.
/// Equal cells cannot define a pool. No token metadata or provider is consulted.
pub fn ordered_wallets(
    first: Option<TonAddress>,
    second: Option<TonAddress>,
) -> TonCoreResult<(Option<TonAddress>, Option<TonAddress>)> {
    let a: TonCell = tonabis_core::address::nullable_address_cell(&first)?;
    let b = tonabis_core::address::nullable_address_cell(&second)?;
    match a.hash()?.as_slice().cmp(b.hash()?.as_slice()) {
        Ordering::Greater => Ok((first, second)),
        Ordering::Less => Ok((second, first)),
        Ordering::Equal => Err(TonCoreError::data(
            "tonabis_tonco::contracts::addresses",
            "pool wallets must differ",
        )),
    }
}
