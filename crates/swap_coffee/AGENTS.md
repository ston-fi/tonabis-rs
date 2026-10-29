# swap.coffee maintainer guide

Read the root guide, this crate's README, `docs/specs/swap_coffee.md` and
`tools/reference/swap_coffee/README.md`. The binding inventory in that reference
folder pins source files, public mappings, expanded getters, exceptions and fixture
dispositions. It is an audit artifact, not a runtime ABI layer.

This package supplies contract wrappers, storage, messages, getters and caller-code
StateInit identities. It has no feature flags, math bridge, routing, signing, broadcasting or provider ownership. Constructors are
synchronous; getters borrow the caller's `ContractClient`, propagate typed errors
and add no retry. Dropping a getter future follows caller/client cancellation.

Keep one inherent block on each primary wrapper. Persistent records live in
`storage`, messages in `msgs`, structured results in `method_results`. Shared staking
values belong to `contracts::staking`. Shared vault initialization/asset records
belong to `vault_native::storage`; import them directly in other vault families
and keep their construction methods there once. Vault sender identities belong to
`contracts::vault_proof`. Import owners directly; do not add compatibility exports.
Common pool asset/proof records belong to `pool_constant_product::storage` and
common pool message values to `pool_constant_product::msgs`. Common vault message
values belong to `vault_native::msgs`; distinct transport bodies/dispatch stay with
their vault family. Staking position data belongs to `staking_item::storage`.
The normalized schema's `canonical_path` entries and inventory list every shared
record. Never regenerate a second type for the same shared protocol value.
Pool proofs embedding different AMM settings remain pool-specific even when
their immediate fields have the same names; compare referenced layouts as well.
Deployment and runtime wrappers remain distinct, including two- versus three-ref
staking master generations. Callers explicitly select the code generation.

Use native TON derives/adapters/macros before adding manual codecs. Ordinary tails
are tolerated. Structured result arity stays exact. Scalar getters return native
values and consume one stack item. Native cells accept Cell/Slice. Factory and pool asset arguments are typed assets with small ToTVMStack
implementations that write a Slice; converting TonCell natively would write Cell.
Pool get_wallet_address methods take &TonAddress and return TonAddress. Keep
these two handwritten: pack owners with InternalAddress, preserving internal zero
and checking int8 workchains, then use native TonAddress result conversion.
Factory owner arguments also use &TonAddress with handwritten InternalAddress
packing; typed asset arguments retain their native Slice-producing stack traits.
Ordinary address fields use TonAddress/InternalAddress or Option<TonAddress>/
NullableAddress, with zero distinct from absence. Nullable getter decoders use core
NullableInternalAddress and return Option<TonAddress>, rejecting variable/anycast
forms while preserving absence versus internal zero. These are not Maybe
fields or native optional stack values. Shared literal flags and empty funding
come from core ExpectedBool and EmptyBodyMsg. Ordinary nullable structured address fields reject TVM NULL. See the inventory for
precise manual exceptions. Parsing is not code, authority or signature verification.

Typed bodies validate their fields; native dispatch follows the workspace prefix-fallback policy. Unknown bodies preserve raw cells. All message
dispatch ends in Unknown. Unboxed native dispatch retains opcode prefixes; the
locked Box<T> codec erases them, permitting malformed messages to fall through.
The few large-enum lint allowances preserve direct native dispatch without heap
wrappers. Factory incoming dispatch stays explicit for nested fixed-tag proofs.
EmptyBody uses a variant-mismatch codec because ensure_empty errors are fatal
to native dispatch; a short or reference-only unknown is never Empty. Empty funding
is legal only for documented incoming roles. Preserve phase flags, bounded untagged
selectors, TEP-89 owner forms, full uint32 extra IDs, signed16 MEV dictionary keys
and original unsigned signed-batch cells. Signature-only mutations must reuse the
body; logical mutation canonicalizes it. New API behavior requires matching docs,
mapping, independent expected data and a focused regression check.

The offline generator owns only its explicit 22 storage/message files. Wrappers,
results, construction helpers, initial phase constraints and signed codecs are
protected. Co-locate each type and all its implementations in its flat storage/msgs file.
Those complete authored modules are excluded from generation; do not recreate
construction, emissions, bounce, initialization or special-codec child modules. Pin normalized input
hashes intentionally, run generation/format twice, and verify identical output plus
unchanged handwritten files. Do not import source executables or capture tooling.
Independent fixtures are crate-local and must remain byte-identical without new
protocol evidence. Tests are registered under `src/tests.rs` and use `test_*` names.

Use Rust 1.94.0 and a target directory inside the checkout, for example
`CARGO_TARGET_DIR="$PWD/target/coffee"`. Start with the affected test filter, then:

```sh
cargo test -p tonabis_swap_coffee --locked
cargo check -p tonabis_swap_coffee --examples --locked
cargo clippy -p tonabis_swap_coffee --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p tonabis_swap_coffee --no-deps --locked
cargo fmt --all -- --check
python3 -m unittest discover -s tools/tests -v
cargo package -p tonabis_swap_coffee --allow-dirty --locked
git diff --check
git diff --cached --check
```

Inspect the archive/normalized manifest, then compile an external consumer against
its extracted path with direct TON type identity and the canonical APIs. Dirty
packaging is provisional local evidence; `publish = false` remains. Follow the root guide for umbrella changes and combined workspace validation.

The TEP-74 Jetton notification envelope shared by CrossDex (all generations),
MEV (both generations) and staking vault is
`contracts::cross_dex::msgs::CoffeeCrossDexJettonTransferNotificationMsg`.
Import this canonical record directly; the former per-contract copies are removed
without aliases. Each contract keeps its own dispatch enum and message roles.
The opcode, fields and independent family fixtures are unchanged.

## Standard-address getter APIs

The four factory owner getters and both pool wallet-address getters take
&TonAddress and use handwritten InternalAddress Slice packing. Internal zero is
preserved and int8 workchains are checked before provider access. Pools return
TonAddress; the factory retains its structured address/hash result. Tagged asset
arguments, nullable standard-address result records and wire domains stay distinct.
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

Public types identify their contract or shared family, for example `CoffeeConstantProductPoolProof` and `CoffeeCurveFiStablePoolProof`.
Keep separate contract entities even when layouts match. See the
[workspace naming policy](../../docs/specs/workspace.md#public-type-naming) and
[type migration map](../../tools/reference/integration/type-renames.json); renamed
Rust types have no compatibility aliases.
