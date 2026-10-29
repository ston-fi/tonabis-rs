# tonabis_core

Shared deterministic helpers and typed errors. Currently this crate provides
checked U256 division, shared TL-B adapters and address helpers. Rust 1.94.0 / edition 2024; unpublished, no features.

| Public path | Behavior |
|---|---|
| `arithmetic::mul_div_floor` | `floor(lhs * rhs / denominator)` using an exact U512 product |
| `arithmetic::mul_div_ceil` | `ceil(lhs * rhs / denominator)` using an exact U512 product |
| `arithmetic::div_ceil` | `ceil(numerator / denominator)` without adding to the numerator |
| `errors::ArithmeticError` | `DivisionByZero` or `Overflow` when the result exceeds U256 |
| `adapters::nullable_internal_address::NullableInternalAddress` | Nullable standard internal addresses without a Maybe bit; preserves internal zero |
| `adapters::expected_bool::ExpectedBool<VALUE>` | Literal layout flag validated on read and write |
| `adapters::maybe_address::MaybeAddress` | Maybe bit followed by a required internal address |
| `adapters::maybe_ref_address::MaybeRefAddress` | Maybe bit followed by a reference to a required internal address |
| `adapters::maybe_ref_nullable_address::MaybeRefNullableAddress` | Maybe reference to a nullable address; three distinct states |
| `address::nullable_address_cell` | Nullable standard address cell; None differs from internal zero |
| `address::internal_address_cell` | Standard internal address cell preserving zero and validating int8 workchains |
| `msgs::EmptyBodyMsg` | Empty funding body with variant-mismatch behavior for unknown dispatch |

```toml
[dependencies]
tonabis_core = { path = "../tonabis-rs/crates/core" }
fastnum = "=0.7.5"
```

```rust
use tonabis_core::{arithmetic::mul_div_floor, errors::ArithmeticError};
use fastnum::U256;

let share = mul_div_floor(U256::from(1_000u64), U256::from(3u64), U256::from(10u64))?;
assert_eq!(share, U256::from(300u64));
# Ok::<(), ArithmeticError>(())
```

Zero denominators fail even when the numerator/product is zero. A floor result
can fit while rounding up overflows; both cases retain their distinct result.
Arithmetic remains synchronous and side-effect free. The crate depends on
`ton_core` for wire types, with no provider, execution runtime or I/O ownership. This is not a pricing, liquidity, fee or simulation library.
All five protocol crates share these codecs and helpers through direct module imports.

`lib.rs` only declares modules. Import functions from `arithmetic` and errors
from `errors`; no root aliases are provided. The umbrella crate also exposes
these exact paths under `tonabis::tonabis_core`.

For a nullable standard internal field:

```rust
use ton_core::{TLB, traits::tlb::TLB, types::TonAddress};
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, PartialEq, TLB)]
struct Recipient {
    #[tlb(adapter = "NullableInternalAddress")]
    recipient: Option<TonAddress>,
    query_id: u64,
}
let value = Recipient { recipient: None, query_id: 7 };
let cell = value.to_cell()?;
assert_eq!(cell.data_len_bits(), 2 + 64);
assert_eq!(Recipient::from_cell(&cell)?, value);
# Ok::<(), ton_core::errors::TonCoreError>(())
```

For ordinary standard fields, use `TonAddress` with TON's `InternalAddress`, or
`Option<TonAddress>` with `NullableAddress`. These reject variable-length/anycast
forms and preserve internal zero. Do not add `bits_len` when it equals the native
field width; retain non-native widths and constructor prefixes.

Run `cargo test -p tonabis_core --locked` for independent BigUint comparisons,
exhaustive small domains and full-width rounding/overflow boundaries. Run
`cargo check -p tonabis_core --examples --locked` for the `lp_share` example.
Arithmetic changes also require `cargo test -p tonabis_core --release --locked`.
The inherited proprietary license and `publish = false` remain in force.

Address layouts: `adapters::maybe_address::MaybeAddress` encodes Maybe internal
inline; `adapters::maybe_ref_address::MaybeRefAddress` encodes Maybe ^internal;
`adapters::maybe_ref_nullable_address::MaybeRefNullableAddress` encodes
Maybe ^nullable. They expose TonAddress values, preserve zero and markers/refs,
and accept ordinary tails. `address::nullable_address_cell` complements
`internal_address_cell` for callers needing addr_none. None of these adapters
handles TVM NULL; stack decoding belongs to protocol wrappers.
