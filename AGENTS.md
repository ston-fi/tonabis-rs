# tonabis-rs agent and maintainer guide

This is a Rust workspace for TON smart contract wrappers. Read the root
[README](README.md), the affected crate's README/AGENTS.md, and
[workspace specification](docs/specs/workspace.md) and
[DeDust specification](docs/specs/dedust.md) before changes. For Moon, also read
the [Moon specification](docs/specs/moon.md) and its crate guide. `CLAUDE.md` imports
this file so Codex and Claude share one policy.

## Scope and capabilities

The workspace has seven crates:

- `crates/core` (`tonabis_core`): shared wire adapters in `adapters`, address helpers, empty
  funding bodies and checked division in `arithmetic`, with `errors::ArithmeticError`; keep implementations outside `lib.rs`.
- `crates/tonabis` (`tonabis`): pure same-name re-exports of all underlying crates,
  currently `tonabis::tonabis_core`, `tonabis::tonabis_dedust`,
  `tonabis::tonabis_moon`, `tonabis::tonabis_stonks`,
  `tonabis::tonabis_swap_coffee` and `tonabis::tonabis_tonco`. No aliases.
- `crates/dedust` (`tonabis_dedust`): 23 wrappers under
`contracts`: classic factories/pools/native and Jetton vaults/liquidity deposits
V1/V2, LP wallet and managed pool; CPMM pool/position/deposit; Uranus Factory V3
and minters/wallets V2/V3; Operator V2; X1000 V1/V2. Capabilities include typed
storage/messages, asynchronous getters, local state-init/address construction,
nested payload codecs and signature-preserving references. The crate README
lists all canonical modules/types; the ownership manifest maps all 23 APIs.

- `crates/moon` (`tonabis_moon`): 18 wrappers / 54 getters for SDK CPMM, CLMM
  and mempad plus separate catalog pool/range, order factory, old/new orders and
  booster. Storage/results are per wrapper; SDK messages are shared per family.
  Native TON codecs replace legacy strict tail/stack helpers. See its README and
  AGENTS.md for protocol exceptions, canonical paths and generation ownership.

- `crates/stonks` (`tonabis_stonks`): eight wrappers / 26 callable getters for
  STON and DeDust presale factories/minters/wallets and the distinct tax wallet.
  Read its crate guide and [specification](docs/specs/stonks.md).
- `crates/swap_coffee` (`tonabis_swap_coffee`): 26 wrappers / 60 callable getters,
  including deployment variants, staking, current/legacy/BidAsk-facing CrossDex
  and MEV. Read its crate guide, [specification](docs/specs/swap_coffee.md) and
  [generation guide](tools/reference/swap_coffee/README.md).
- `crates/tonco` (`tonabis_tonco`): 17 wrappers / 66 getters for V1/V2 AMMs,
  orders/accounts/NFTs, farming and native minter/wallet generations. Read its
  crate guide and [specification](docs/specs/tonco.md).

Keep protocol pricing, liquidity simulation, market adapters, routing and
application policy elsewhere. Core owns protocol-independent adapters, address helpers, empty funding bodies
and checked arithmetic/errors. It depends on `ton_core`, without providers or runtime ownership. Do not migrate
other DEXes, add universal traits or flatten public paths without authorization. Future protocols follow
`crates/{dex}/src/contracts.rs` plus `contracts/` and package `tonabis_{dex}`.
Peer-protocol callback payloads in X1000 are part of that wire format, not support
for another DEX. See the [workspace specification](docs/specs/workspace.md).

Read each crate's README and AGENTS.md for its complete capabilities and checks.
The workspace uses released crates.io TON dependencies; the checked lockfile is
the validated graph. Reuse FromTVMStack field TLB adapters for Cell/Slice fields.
User-facing getter arguments prefer TonAddress, or Option<TonAddress> for nullable
encoded addresses; use handwritten InternalAddress/NullableAddress packing to
preserve zero and absence. Raw address forms remain only where the API deliberately
exposes them. TVM NULL is a separate contract. See
[stack and address policies](docs/specs/workspace.md#ton-dependencies-and-stack-adapters).
When adding a protocol library, register it in the workspace and add its optional
dependency, folder-name feature and gated same-name re-export to `tonabis`; include it in `all`,
and update both inventories. All protocols depend on core for shared codecs/helpers; do not add dependencies without a concrete use.

## Public API and ownership

- Public type names identify the protocol and owning contract/family, including
  generation when needed. Avoid cross-module duplicate names such as `Errors`,
  `Message` or `PoolProof`. Distinct contract entities remain separate even when
  layouts match; shared semantic records retain their existing canonical owner.
  Update Rust destinations in generator metadata without renaming source identities.

- Keep protocol generations distinct. Constructors/codecs are synchronous;
  provider getters are asynchronous and use caller-owned `ContractClient`.
  Do not add runtime ownership, hidden retries, background tasks, signing,
  broadcasting, authority validation, gas or settlement policy.
- Use canonical `ton` crate types directly, including clients, codecs, errors and
  proc macros. Do not add a private dependency or re-export layer. The checked
  lockfile is the validated dependency graph; do not claim every older TON 0.4
  release works. Protocol crates have no feature flags. The umbrella always exports
  core; its independent `dedust`, `moon`, `stonks`, `swap_coffee` and `tonco`
  features default off, and `all` enables all five. No protocol depends on core
  solely to force connectivity.
- Contract primary files own wrappers and one handwritten inherent method block,
  with constructors before getters. Persistent records live in each contract's
  `storage` child, messages in `msgs`, structured results in `method_results`.
  Use direct module imports and no compatibility aliases or convenience exports.
  The umbrella intentionally re-exports whole crates under their unchanged names.
- Keep each contract flat below its primary file: `storage.rs`, `msgs.rs` and
  `method_results.rs`. Define a type and all its implementations in the same
  file; do not split small construction, forwarding, bounce or codec children.
  Keep distinct contract generations as separate primary modules.
  Do not create empty child directories or re-export-only modules. Shared records
  with identical meaning and encoding have one canonical owner, imported directly;
  update generator mappings and consumers when consolidating. Similar fields alone
  do not justify merging different generations, message roles or address domains.
- Message dispatch enums end in `Unknown(TonCell)`. Prefer native derives;
  document real limits such as nested prefixes, empty-body matching and signed
  graph preservation. Avoid redundant name overrides and numeric Rust-name
  suffixes; preserve proven selectors with `#[ton_method(id = ...)]`.
- Import types at the top of the file and use their short names, especially TON
  primitives. Keep qualified type paths only for real name collisions; qualified
  function calls are fine. Generators must follow the same import convention.
- Omit field `bits_len` when it equals the native type width; retain non-native
  widths, constructor prefixes and dictionary key widths. Update generators too.
- Ordinary internal-address fields use TonAddress/InternalAddress or
  Option<TonAddress>/NullableAddress. Retain raw addresses only for proven signed/historical or arbitrary-address
  domains. A reference or Maybe wrapper alone does not justify a raw type; use
  core MaybeRefAddress/MaybeRefNullableAddress/MaybeAddress as appropriate.
  Prefer `TonAddress` for getter results too whenever behavior can be preserved.
  Existing use of `MsgAddressInt` is not a reason to keep it. Use a native adapter
  or a narrow custom implementation before retaining a raw type.
- Prefer native TON macros/derives where they preserve selectors, wire layouts,
  stack ordering and failure behavior. Keep hand codecs for proven protocol
  exceptions. No production unwrap/expect/panic-driven control flow.
- Preserve typed TON errors. Parsing is not authentication. Ordinary tails are
  tolerated; typed bodies validate their fields and dispatch may retain malformed
  input as Unknown on a prefix mismatch. See the workspace dispatch policy.
  Signed graphs, address distinctions, stack arity and numerical widths are
  consumer contracts. Follow crate guidance for their exact exceptions.
- Use struct literals for public-field records and narrow visibility. Add
  abstractions only for demonstrated ownership/invariants. New public behavior
  needs matching rustdoc/README, agent guidance, specification, independent
  fixture or captured evidence, and a focused regression check.

## Address and parsing decisions

| Context | Type and conversion |
|---|---|
| Standard internal TLB address | Prefer `TonAddress` with `InternalAddress`; required internal zero stays internal |
| Nullable encoded TLB address | Prefer `Option<TonAddress>` with `NullableAddress` or core `NullableInternalAddress`; None is addr_none, without an extra Maybe bit |
| Getter address argument | Prefer `&TonAddress` or `Option<TonAddress>`; pack the protocol's exact Cell/Slice/NULL kind, using core helpers where available |
| Getter address result | Prefer `TonAddress` (or `Option<TonAddress>` for distinct absence). Use a FromTVMStack field TLB adapter or custom conversion to preserve the required behavior |
| Optional stack containing a nullable address | `Option<Option<TonAddress>>`: NULL / encoded addr_none / internal identity remain distinct |
| Maybe/reference address fields | Core `MaybeAddress`, `MaybeRefAddress`, `MaybeRefNullableAddress`; preserve markers/refs without exposing raw addresses |
| Arbitrary/historical address domain | Keep `MsgAddress`, `MsgAddressInt` or the explicit referenced form when the protocol requires that domain or signed graph |

`TonAddress` is a standard-address interface, not an interchangeable encoding for
every raw address. Native stack conversion can normalize NULL/addr_none to zero;
`InternalAddress` preserves internal zero and rejects absence, while a nullable
TLB adapter accepts encoded addr_none but does not itself accept TVM NULL.
Do not silently change these behaviors during conversion. Standard adapters reject
variable/anycast addresses; a protocol needing those forms must expose a raw domain.
`TonAddress` stores only a workchain and a 256-bit hash: an adapter cannot preserve
arbitrary bit lengths, anycast metadata or original address tags in that type.
Retain a raw type only for a documented representational/behavioral requirement,
not merely because native default conversions differ. Check both the supported
protocol domain and existing codec acceptance before claiming equivalence.
Current ordinary fields across all five protocol crates use TonAddress-based
interfaces, including structured getters, ordering and StateInit helpers. X1000
historical contexts/messages are the remaining raw-field exception. Do not retain
raw address types merely because an older schema used MsgAddress, the field is
referenced, or a default conversion differs. Check contract meaning and absence
states. Ordinary nullable result fields use `#[from_tvm_stack(adapter =
NullableInternalAddress)]` with derived FromTVMStack and exact arity. They accept
Cell/Slice containing addr_none, but reject TVM NULL. Keep manual decoding only
for explicit NULL-returning protocol fields or other proven codec limits. For
required address dictionary values use InternalAddress, never the default
TonAddress codec that writes zero as addr_none.

Convert already-decoded nullable addresses with core `NullableInternalAddress::from_address`
instead of serializing to a temporary cell to parse again.

Ordinary TLB structs accept extra trailing bits and references when those tails
do not change the modeled layout. Do not add blanket `ensure_empty`, remaining-bit
or remaining-reference checks to roots, referenced records, addresses or dictionaries.
Unmodeled tails need not survive re-encoding. This differs from a deliberately raw
unknown body or signed graph, whose preservation is part of its contract.

Keep checks that select/validate actual protocol structure: required fields,
prefixes, widths, literal flags, explicit suffixes and deployment/runtime phases.
An empty-body message may require emptiness to distinguish its variant. Native
dispatch may preserve a malformed body as Unknown after a nested prefix mismatch;
this accepted fallback is not proof of an unfamiliar opcode or valid input. Keep
existing explicit dispatch errors and propagate other decoding failures. Structured getter results
still enforce exact stack arity through the derive or a manual decoder; scalar
getters use native consumption. Cell tail tolerance does not relax stack shape.
Every manual codec or strict exception needs a concrete reason and regression test.

## Adding a contract group and consuming the library

Before adding another group, inventory its generations, wrappers, selectors,
argument stack kinds, storage/messages/results, address domains and shared records.
Use the existing flat layout and naming (`<Protocol><Contract>Contract`, explicit
storage identities, `Msg` bodies, descriptive getter result names). Prefer derives
and scalar returns; do not copy generic strict parsers or introduce convenience
aliases. Import types at module scope and use short names, including primitives;
qualified function calls and genuine type-name collisions are the exceptions.
Apply these conventions in generators as well as authored files, preserving literals
and comments during source transformations.

Give downstream agents a crate README with canonical imports, supported generations,
an executable codec/getter example and documented address/NULL/tail behavior.
Keep the crate AGENTS guide, specification and ownership inventory aligned. Consumers
should need only public crate docs/examples, direct `ton` types and a caller-owned
`ContractClient`, not generator internals, source checkouts or private tooling.
Document intentional public path changes and keep independent fixtures unchanged.
Register the workspace/umbrella feature and inventories, then run the checks below
appropriate to the changed surface. Local validation is not publication approval.

## Generated files and evidence

The [tooling guide](tools/reference/dedust/README.md) defines regeneration.
`generate_bindings.py` owns an explicit allowlist of catalog-backed storage,
message and family-index files plus `bindings-generation.json`. It does not own
primary wrappers, result DTOs, shared classic assets, signed codecs, X1000 V2 storage/runtime/context records or the four
other handwritten contract layouts. Never append handwritten behavior to generated
files or expand generator ownership accidentally.

Moon uses its separate [tooling guide](tools/reference/moon/README.md) and pinned
normalized schema. Its explicit allowlist owns ordinary storage/messages and
dispatch, excluding wrappers, result DTOs and flat modules that also own authored codecs.

Coffee uses pinned normalized inputs and a 22-file output allowlist. Modules
with authored implementations are maintained as complete flat files; the
generator must not recreate their former children.
Shared records have one canonical owner via explicit schema mappings; keep
wrapper generations, runtime/deployment storage and message dispatch roles distinct.
STONks and TONCO have binding inventories and handwritten derive-based records,
without generators. All three inventories record source hashes, source-to-public
paths, expanded getter coverage, fixtures and justified manual codec exceptions.

Handwritten edits do not require regeneration. For mechanical moves/renames,
rely on affected unit tests, doctests and formatting; do not repeat coverage,
packaging, feature-matrix or unrelated qualification scripts.
For generator/schema changes,
verify pinned hashes, generate, format, check handwritten-file preservation and
require a second run to produce identical output. Keep independently expected
fixture bytes unchanged unless new protocol evidence justifies a change.

The retained tools are local/offline generators and their ownership test. No
capture/replay/economic/compatibility tooling was imported. Never copy executable
capture cells merely to make a test pass; retain provenance and actual licenses.
Default tests must use local, code-free/synthetic fixtures and mock providers.
Coverage claims must identify the tested contract behavior and evidence.

## Validation

Use pinned Rust 1.94.0. Unit tests live in `src/tests/test_*.rs` and are registered
in `src/tests.rs`; name test functions `test_*`. Test observable protocol behavior,
not constants, trivial construction or third-party primitive behavior alone.

Start with the affected test filter, then run the affected crate suite
(`tonabis_core`, `tonabis_dedust`, `tonabis_moon`, `tonabis_stonks`,
`tonabis_swap_coffee`, `tonabis_tonco` or umbrella doctests). For integration:

```sh
cargo test --workspace --locked
cargo test -p tonabis --doc --all-features --locked
cargo check --workspace --examples --locked
cargo fmt --all -- --check
```

For public APIs, dependencies or initial integration also run:

```sh
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --locked
python3 -m unittest discover -s tools/tests -v
```

`cargo test` includes README doctests through crate rustdoc. `cargo test
--workspace --locked` covers all current crates. Python 3.9+ is sufficient. Core arithmetic changes also need
`cargo test -p tonabis_core --release --locked`; umbrella re-exports need no unit
tests duplicating their declarations.
Use `git diff --check` and, for staged changes, `git diff --cached --check`.
For dependency/package changes, run `cargo package -p <affected-crate>
--allow-dirty --locked`, inspect its contents and compile a standalone consumer
against the extracted package outside the workspace. Protocol archives require unpublished core, and the umbrella requires all underlying
packages. For local archive validation, use an explicit command-line crates.io patch
to workspace core when packaging, and to extracted core for archive consumers.
The scratch consumer lockfile may first need its local package paths refreshed. Keep TON dependencies on crates.io. Report this limitation rather than
claiming registry-only consumption or publishing dependencies. This is provisional local packaging
validation, not publication approval; use a clean accepted revision for release.
Avoid repeating successful checks without a relevant change.

## Git and distribution

Work in the current attached checkout and active branch. Inspect Git state before
editing, preserve unrelated changes and stage only task-owned changes. Follow the repository review workflow; do not push, publish or change infrastructure merely
to complete local work. Report checks actually run and residual limitations.

`publish = false` and the inherited proprietary license policy remain in force;
the catalog MIT notice is not a workspace-wide license grant. Do not invent
release status. Any later dependency/publication
change must preserve exposed TON type identity and document the consumer impact.

Distribution targets GitHub releases of the complete repository. The first version
and tag are exactly `0.1.0`; keep crates.io publishing disabled. Validate tagged
Git/source-archive consumption, not availability of unpublished first-party crates
on crates.io. Per-crate packaging checks above apply to future registry/package
changes, not this GitHub-only release gate. See the
[distribution policy](docs/specs/workspace.md#github-distribution). Do not create
or push a tag or publish a GitHub release without an explicit publication request.
