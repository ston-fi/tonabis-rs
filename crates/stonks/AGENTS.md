# STONks maintainer and downstream-agent guide

Read the workspace AGENTS.md, this crate's README, `docs/specs/stonks.md` and
`tools/reference/stonks/{README.md,bindings.json}` at the repository root. This
crate owns contract bindings only, with eight wrappers and 26 callable getters.
STON v8/v10/v11 share a getter wrapper; DeDust v7 and v12 have separate wrappers.
No feature flags, math bridge, runtime ownership or hidden dependencies.

## Ownership and extension

- Primary modules own wrappers and one inherent method block, with constructors
  before getters. Use native `ton_contract!` and `ton_methods`, explicit numeric
  IDs for unnamed selectors. Providers/context are caller-owned; errors remain
  `TonResult` or `TonCoreResult`. Do not introduce retries or defaults on failure.
- `storage.rs`, `msgs.rs` and `method_results.rs` are flat files containing types
  and their implementations. They own persistent records, message/dispatch
  records and structured results. No aliases or re-export-only modules. Shared
  minter storage belongs only to `ston_minter::storage`; shared Jetton results and
  eight-field bonding results belong to `ston_minter::method_results`.
- Shared DeDust v7/v12 continuation records belong to `dedust_minter_v7` children.
  Standard wallet results and presale bounce records belong to `dedust_wallet`.
  Forwarding belongs to `ston_minter::msgs`. Import canonical types
  directly; never duplicate layouts just to make module trees symmetrical.
- Factory initialization/dev-buy records are the same as minter initialization
  records, so `ston_minter::msgs` owns them. Legacy factory messages belong to
  `ston_factory::msgs`; only v12's extended creation belongs to `dedust_factory::msgs`.
- `StonksVirtualMinterStorage::initial_state` initializes shared deployment data.
  Each wallet's inherent `initial_state` method owns its distinct initial context.
  Code is always supplied by the caller; parsing never authenticates code/senders.
- New public behavior needs useful rustdoc, README/spec updates, manifest mapping,
  independent expected bytes and focused regression tests. No generator is needed.

## Invariants and common mistakes

Eight-field bonding results and twelve-field v12 results are incompatible exact
stack shapes. v12 fee fields are CPMM base fee, total presale tax, destination
creator share, recipient; never follow the guide's reversed first two labels.
Numeric selectors 82057, 68631, 95602, 71772, 74038 and 84724 retain their identities.
STON-only methods belong only to the STON minter.

Ordinary root/reference tails are accepted and unmodeled tails need not survive
serialization. Structured getter results enforce exact arity; nullable address
results derive FromTVMStack with NullableInternalAddress field adapters. Scalar versions return native values;
nullable address results return Option<TonAddress>. Cell/Slice are accepted.
Wire addresses preserve literal internal zero separately from addr_none. Nullable minter owner arguments use Option<TonAddress> with explicit addr_none Slice packing.
Do not reintroduce the source's generic strict stack or Cell-only helpers.

Typed bodies validate nested fields; native dispatch follows the workspace prefix-fallback policy. Unknown bodies preserve raw bits/refs.
Dispatch derives TLB with Unknown last. Message-field adapters own forward-tail
and nested continuation validation. The empty-body codec reports a variant
mismatch for nonempty input: ton_macros 0.2.6 treats an ensure_empty error as fatal
rather than trying Unknown. Keep that narrow exception beside its type.

Presale transfers require a true payload selector and two separate references
(payload then context), enforced by the shared `ReferencedPayload` field adapter.
STON's context has two addresses; DeDust's has one. Caller selection remains
necessary because ordinary tail tolerance permits shorter schema prefixes.
Current STON v10/v11 sells require a standard recipient. Untagged forwarding tails
preserve omission versus a present Either payload; their explicit suffix shape
remains validated. Tax and presale bounce opcode domains differ.

Keep production free of unwrap/expect/panics. Public-field records use literals;
only validation/preservation invariants justify constructors/private fields.

## Evidence and validation

Unit tests live in `src/tests/test_*.rs`, registered in `src/tests.rs`, named test_*.
The fixture directory is packaged. Preserve independent fixture bytes and hashes;
roundtrips supplement independent encodings. `initialization.json` is independent
raw-bit code-free StateInit evidence; getter provider tests reconcile all expanded
methods. No default or ignored test may require another source checkout or an executable sidecar.
Captured executable replay and live network behavior are outside this artifact.

Use a unique absolute target directory, e.g. `CARGO_TARGET_DIR="$PWD/target/stonks"`:

```sh
cargo test -p tonabis_stonks test_schema --locked
cargo test -p tonabis_stonks --locked
cargo check -p tonabis_stonks --examples --locked
cargo clippy -p tonabis_stonks --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p tonabis_stonks --no-deps --locked
cargo fmt --all -- --check
cargo package -p tonabis_stonks --allow-dirty --locked
git diff --check
git diff --cached --check
```

For package/API changes inspect the archive and normalized manifest, run its tests
and compile a consumer outside the workspace against the extracted package, using
the direct TON dependency to establish type identity. Allow-dirty packaging is
provisional local validation; publication remains unauthorized. Follow the root guide for workspace integration checks and umbrella changes.

Standard burn-notification and excess acknowledgements have one canonical owner in
`ston_minter::msgs`. Tax-wallet dispatch imports those records directly; the
shared burn record exposes `owner` and `response_address`.

## Standard-address getter APIs

All three minter get_wallet_address methods take and return Option<TonAddress>.
The owner is a nullable-address Slice, with None encoded as addr_none and
Some(internal zero) encoded internally. Results map NULL/addr_none to None and
retain Some(internal zero). Handwritten bodies validate int8 workchains before
provider access. Wire records and StateInit inputs now expose standard addresses with explicit absence.
The current getter inventory is `tools/reference/integration/tonaddress-api.json`.
See `docs/specs/workspace.md#friendly-address-interfaces` for the shared policy.


## Address representation

Ordinary address fields use `TonAddress`/`InternalAddress` or
`Option<TonAddress>`/`NullableAddress`; internal zero remains distinct from absence.
Core Maybe/reference adapters retain selector bits and reference placement.
Ordinary nullable result fields derive FromTVMStack with NullableInternalAddress:
Cell/Slice addr_none is None; TVM NULL is rejected. Explicit NULL fields keep their
protocol handling, and structured result arity stays exact;
see the [workspace address policy](../../docs/specs/workspace.md#consistent-consumer-types-and-parsing).
Variable, anycast and external addresses are rejected in ordinary fields.

Public types identify their contract or shared family, for example `StonksStonWalletContext` and `StonksDedustWalletContext`.
Keep separate contract entities even when layouts match. See the
[workspace naming policy](../../docs/specs/workspace.md#public-type-naming) and
[type migration map](../../tools/reference/integration/type-renames.json); renamed
Rust types have no compatibility aliases.
