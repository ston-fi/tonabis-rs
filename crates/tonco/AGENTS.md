# TONCO maintainer and consumer guide

Read `README.md`, the workspace's `docs/specs/tonco.md` and
`tools/reference/tonco/{README.md,bindings.json}` before changing public behavior.
The JSON inventory records binding ownership and fixture provenance; it does not generate code.

## Ownership and extension

Keep all 17 wrappers/66 source getters, including auxiliary factory orders,
native wallet generations and farming. Primary files own wrappers and one getter
impl; storage and local StateInit methods belong in `storage`, message/dispatch
records in `msgs`, exact-arity getter DTOs in `method_results`. Keep these
files flat, with each type and all its implementations together. V2 payout and
reforge records share `v2::msgs`, with distinct ToncoV2PayoutPayloads/ToncoV2ReforgePayloads. Constructors are
synchronous. Getters use caller-owned `ContractClient`, transaction context and
executor, returning native typed TON errors. No signing, broadcasting, hidden
retries, background tasks, provider ownership, pricing or numerical snapshots.
Core supplies the shared address helper. No feature flags, pricing math or convenience re-exports.

V1/V2 storage and mint/burn bodies must stay distinct. Reforge/payout records have
one shared V2 owner under `v2::msgs`; TEP-89 wallet-response/owner records belong
to native minter messages. Wallet generations share the actual identical wallet
storage. Both pools import the tick page from V1 pool `method_results` directly.
Preserve full-address-cell hash ordering, referenced whitelist permissions,
signed 24-bit tick keys, V2 int224 outside growth and V1 referenced/V2 inline ticks.
Do not resurrect stale SDK selectors as executable methods.

Ordinary address fields use `TonAddress`/`InternalAddress` or
`Option<TonAddress>`/`NullableAddress`; internal zero remains distinct from absence.
Core Maybe/reference adapters retain selector bits and reference placement.
Ordinary nullable result fields derive FromTVMStack with NullableInternalAddress:
Cell/Slice addr_none is None; TVM NULL is rejected. Explicit NULL fields keep their
protocol handling, and structured result arity stays exact;
see the [workspace address policy](../../docs/specs/workspace.md#consistent-consumer-types-and-parsing).
Variable, anycast and external addresses are rejected in ordinary fields.

Manual codecs include nullable address results, nested tick-page decoding and ten source-defined
untagged suffix/layout codecs (including separate pool/router timelocks).
Their manifest rows explain why tagged native Option/ordinary derives cannot
represent these formats and identify focused tests. Keep suffix strictness and
existing explicit-dispatch errors; native dispatch follows the workspace prefix-fallback
policy and unknown messages retain raw graphs. No authentication or
signature/proof verification is implied. New public behavior needs evidence,
focused tests, inventory/spec/README/rustdoc updates and package-consumer checks.
Use public-field literals; add no generic facade or generator without a concrete
need. Preserve notices and proprietary/unpublished policy.

## Tests and checks

Tests live in `src/tests/test_*.rs`, registered in `src/tests.rs`, with `test_*`
functions. Fixtures are crate-local, independent and code-free/synthetic. Never
change expected bytes to agree with the implementation. Retain structural/message
parts of mixed source tests; exclude local economic simulation and executable
sidecars. `test_getters.rs` calls every getter through the recording provider,
checking numeric selectors, argument values/order and Cell/Slice kinds against
source-derived inventory. Errors must propagate without library retries.

Use pinned Rust 1.94.0 and a target inside the checkout:

```sh
export CARGO_TARGET_DIR="$PWD/target/tonco"
cargo test -p tonabis_tonco test_sdk_wire_vectors --locked
cargo test -p tonabis_tonco --locked
cargo check -p tonabis_tonco --examples --locked
cargo clippy -p tonabis_tonco --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p tonabis_tonco --no-deps --locked
cargo fmt --all -- --check
cargo package -p tonabis_tonco --allow-dirty --locked
git diff --check
git diff --cached --check
```

Inspect the `.crate` file list and normalized manifest; compile an external
consumer against the extracted crate, including direct TON type identity.
Provisional dirty-tree packaging is not publication approval. Source-code or
fixture edits do not require regeneration. Aggregate workspace/umbrella checks
follow the root guide.

## Standard-address getter APIs

The eleven address-taking V1/V2/native getters use &TonAddress with handwritten
InternalAddress Slice packing and int8 workchain validation. Address scalars
return TonAddress; initial-data/state-init cells remain TonCell. whitelist_key
also takes &TonAddress and preserves exact 255-bit extraction. ordered_wallets takes and returns Option<TonAddress>, preserving addr_none and
ordering by complete address-cell hashes.
Optional router result DTOs retain NULL versus present addr_none/internal zero.
The current getter inventory is `tools/reference/integration/tonaddress-api.json`.
See `docs/specs/workspace.md#friendly-address-interfaces` for the shared policy.


Optional stack address fields use `Option<Option<TonAddress>>`: outer `None` is
TVM NULL, `Some(None)` is encoded addr_none, and `Some(Some(address))` is internal.

Public types identify their contract or shared family, for example `ToncoRouterV1Timelocks` and `ToncoRouterV2Timelock`.
Keep separate contract entities even when layouts match. See the
[workspace naming policy](../../docs/specs/workspace.md#public-type-naming) and
[type migration map](../../tools/reference/integration/type-renames.json); renamed
Rust types have no compatibility aliases.
