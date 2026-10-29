# DeDust maintainer and downstream-agent guide

Read the root AGENTS.md, this crate's README, `../../docs/specs/dedust.md` and
`../../tools/reference/dedust/README.md`. This crate owns contracts only. Its 23
wrappers cover classic V1/V2 (including LP wallet and managed pool), CPMM
pool/position/deposit, Uranus factory/minters/wallets, Operator V2 and X1000 V1/V2.
The README is the complete wrapper/path inventory. There is no math module.

Classic V2 native and Jetton vaults share pool identity and swap continuation
records from `contracts::classic::vault_native_v2::storage`: `DedustPoolType`,
`DedustVaultNativeV2PoolParams`, `DedustSwapKind`, `DedustSwapStepParams`, `DedustSwapStep`,
`DedustSwapParams` and their tag records. Import that owner directly for either
vault; the former Jetton-local copies are removed without aliases. Wire formats
and independent fixtures are unchanged.

## Module ownership and extension

- Keep canonical `contracts::{classic,cpmm_v2,uranus,operator,custom_wallet}` paths.
  Classic Protocol2 is distinct from CPMM v2; CPMM deployment `Revision::PoolV1`
  and `PoolV2` select library identities, not classic generations.
- Primary files own handwritten wrappers, constructors and getters. Every
  contract's persistent records live in `storage`; messages in `msgs`; useful
  multi-field getter DTOs and their decoders in `method_results`. No parent
  aliases/re-exports. The handwritten LP wallet, managed pool, CPMM position and
  deposit follow the same layout and use DeDust/family-qualified names.
- `classic/asset.rs` is the sole handwritten owner of `DedustClassicAsset` and its
  native/Jetton records plus stack traits. The generator must not recreate
  per-contract Asset copies. CPMM's different representation stays separate.
- The binding generator owns 19 catalog schemas and family indexes, plus mapping
  metadata for all 23 wrappers. Primary APIs, result modules, four handwritten
  contract layouts, complete X1000 V2 storage and X1000 V1 messages are excluded.
  Complete flat msgs modules owning authored payment/signed codecs stay excluded
  as well. X1000 V2 runtime dictionaries and continuation contexts belong directly
  to its `storage.rs`; no runtime/contexts children. Keep every type and
  implementation together in msgs/storage; do not recreate small construction or
  codec child modules. Consult its output allowlist.
- Keep constructors before getters in one authored inherent block. Use
  `#[ton_methods]` where the known name/native types match; unresolved names use
  explicit `#[ton_method(id = ...)]`. Do not invent names based solely on hashes.
  Preserve selector and argument order; internal-zero address packing and nullable
  wire-address getters retain explicit handling where native conversion differs.

## Wire invariants

Ordinary root/reference/address/dictionary tails are accepted; serialization
emits modeled fields. Unknown bodies and modeled raw payloads retain bits/refs.
Typed bodies reject malformed data. Native dispatch can retain nested prefix
mismatches as Unknown; other errors, including truncation, propagate. Classic pool and X1000 V1 dispatch are
manual because nested prefix failures must not become unknown top-level bodies.
Other ordinary dispatch enums derive TLB with `Unknown(TonCell)` last.

Ordinary address fields use `TonAddress`/`InternalAddress` or
`Option<TonAddress>`/`NullableAddress`; internal zero remains distinct from absence.
Core Maybe/reference adapters retain selector bits and reference placement.
Ordinary nullable result fields derive FromTVMStack with NullableInternalAddress:
Cell/Slice addr_none is None; TVM NULL is rejected. Explicit NULL fields keep their
protocol handling, and structured result arity stays exact;
see the [workspace address policy](../../docs/specs/workspace.md#consistent-consumer-types-and-parsing).
Variable, anycast and external addresses are rejected in ordinary fields.

DedustPoolV2GetQuoteProvidersResult derives with NullableAddress field adapters from released
TON: encoded addr_none is None, internal zero is Some, and TVM NULL is rejected.
Other native unannotated fields keep their previous NULL semantics.

Scalar getters use native stack consumption. Complete result DTOs enforce exact
arity (`ensure_empty`); do not add `allow_extra`. Cell/Slice are accepted by native
cell conversions; optional cells distinguish NULL from an empty cell. Native
TON dictionary adapters own label validation; no DeDust compatibility guard,
normalization or leaf/fork exhaustion rule is needed for the tested resolution.

Use `Coins` for stored nonnegative getter amounts (u128), `U256` for unsigned
checkpoints/growth/hashes, u32/u8 for counters and `I512` for truly signed values.
Storage retains exact TL-B widths (including Coin120 and Q248), while getters
follow their native domains. Do not add a Q248 getter bound or narrow signed
valuation/pending-key arguments. Factory `is_stable: bool` packs 0/1; classic
Assets pack Slice. Selector 97376 keeps out-of-uint64 key absence semantics.

`DedustSignatureRef<T>` preserves the original referenced graph while fields are
unchanged, including valid alternative dictionary labels. A mutation serializes
new fields and requires caller re-signing. Do not replace it with ordinary TLBRef.
Bounced is an envelope flag: some programs skip a marker, some check it, and some
never read a body. Preserve each proven generation-specific boundary. CPMM
Q120.120 uses `VarLenBytes<U256, 5>` directly.

## Consumers, evidence and checks

Callers construct wrappers using `TonContract::new` and a borrowed ContractClient,
then await getters. Storage decoding uses the TLB trait and the `storage` path.
See `examples/contracts.rs` and README doctests. Errors remain TON typed errors;
there are no hidden tasks, signer, broadcaster or authority guarantees.

Keep unit tests in `src/tests/test_*.rs`. Fixture values must be independent of
this crate's encoder; roundtrips supplement them. Tests protect selectors,
argument packing, result shapes, storage/wire widths, malformed-known dispatch,
nullable addresses, tail tolerance, signed graph retention and state-init hashes.
No live provider is required. Fixtures contain independently expected encodings and synthetic program markers,
without executable contract cells.

Run `cargo test -p tonabis_dedust --locked`, `cargo check -p tonabis_dedust
--examples --locked` and `cargo fmt --all -- --check`. Public/dependency changes
also require `cargo clippy -p tonabis_dedust --all-targets --locked -- -D warnings`
and `RUSTDOCFLAGS="-D warnings" cargo doc -p tonabis_dedust --no-deps --locked`.
Generator changes additionally require `python3 -m unittest discover -s tools/tests
-v` and two formatted regeneration passes with handwritten preservation checks.
Handwritten API changes need no fresh capture or executable replay.

Changing an API requires updating the README, specification, relevant mapping
metadata and getter-test generator when applicable. Preserve fixture identities
when renaming Rust paths. New protocol behavior needs independent evidence. Keep publication disabled until an
explicit release decision resolves licensing and publication policy.


Raw address types remain only in X1000 historical message/context records. Their
signed, opaque or bare-addr_std layouts are retained; these are not patterns for
ordinary new contracts. Address dictionaries use a required-internal value
adapter, including when the hash is zero.

Public types identify their contract or shared family, for example `DedustLiquidityDepositV1PoolParams` and `DedustLiquidityDepositV2PoolParams`.
Keep separate contract entities even when layouts match. See the
[workspace naming policy](../../docs/specs/workspace.md#public-type-naming) and
[type migration map](../../tools/reference/integration/type-renames.json); renamed
Rust types have no compatibility aliases.

Do not emit the reference-only Native Vault V2 inline deposit candidate. The
manifest records it under `excluded_declarations`; preserve its corpus/fixture
identity without exposing an unsupported public codec.
