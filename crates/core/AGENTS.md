# Core maintainer guidance

Read the root AGENTS.md, this README and `../../docs/specs/workspace.md`.
`tonabis_core` owns shared checked helpers and typed errors. Currently these are
`arithmetic::{mul_div_floor,mul_div_ceil,div_ceil}`, `errors::ArithmeticError`,
`adapters::{nullable_internal_address,expected_bool}`, `address::internal_address_cell`
and `msgs::EmptyBodyMsg`.
Keep `lib.rs` to module declarations and test registration; no root re-exports.

U256 products widen to U512 before division. Preserve exact floor/ceiling rounding
and reject zero divisors and out-of-range results with typed errors. Ceiling can
overflow when floor fits. Do not use numerator + denominator - 1, silently
truncate, panic, or add protocol-specific fees, prices or simulation policies.
Keep arithmetic synchronous and deterministic. Wire helpers use `ton_core`;
do not add providers or runtime ownership. Shared additions need a concrete use; do not move protocol helpers into
core merely because they look similar. All five protocols consume the shared helpers. Each adapter lives in its own
module under `adapters`; no convenience re-exports. Prefer native TON adapters
for standard addresses. NullableInternalAddress delegates wire encoding to native
NullableAddress and also converts decoded standard addresses for getter results.
It rejects variable/anycast/external forms and preserves absence versus internal zero. TVM NULL handling belongs to the caller, not the TL-B adapter.
ExpectedBool rejects opposite literals on both read/write. EmptyBodyMsg must
report variant mismatch for nonempty cells so unknown dispatch remains reachable.
Keep exact-width fields unannotated and preserve non-native widths/prefixes.

Tests live in `src/tests/test_*.rs`, registered in `src/tests.rs`. Preserve the
independent BigUint oracle, exhaustive small domains and full-width boundary
cases. Run `cargo test -p tonabis_core --locked`, adding `--release` for arithmetic
changes. Run `cargo check -p tonabis_core --examples --locked`,
`cargo clippy -p tonabis_core --all-targets --locked -- -D warnings`,
`RUSTDOCFLAGS="-D warnings" cargo doc -p tonabis_core --no-deps --locked`, and
`cargo fmt --all -- --check` for public changes. Package/dependency checks follow
the root instructions. Update README, specification and umbrella documentation
when changing public methods, errors or dependencies. No feature flags exist.

Address layouts: `adapters::maybe_address::MaybeAddress` encodes Maybe internal
inline; `adapters::maybe_ref_address::MaybeRefAddress` encodes Maybe ^internal;
`adapters::maybe_ref_nullable_address::MaybeRefNullableAddress` encodes
Maybe ^nullable. They expose TonAddress values, preserve zero and markers/refs,
and accept ordinary tails. `address::nullable_address_cell` complements
`internal_address_cell` for callers needing addr_none. None of these adapters
handles TVM NULL; stack decoding belongs to protocol wrappers.
