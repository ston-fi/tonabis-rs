# Workspace crate boundaries

The workspace contains `tonabis_core` (`crates/core`), `tonabis_dedust`
(`crates/dedust`), `tonabis_moon` (`crates/moon`), `tonabis_stonks`
(`crates/stonks`), `tonabis_swap_coffee` (`crates/swap_coffee`), `tonabis_tonco`
(`crates/tonco`) and `tonabis` (`crates/tonabis`). All use edition 2024,
Rust 1.94.0 and the inherited unpublished/proprietary package policy. Dependencies
come from crates.io and the workspace itself. No local patches are required.

## Core

`tonabis_core::arithmetic` exposes `mul_div_floor`, `mul_div_ceil` and `div_ceil`.
The first two compute an exact U512 product from two U256 operands before
floor/ceiling division and checked narrowing. `div_ceil` divides U256 without
adding to the numerator. All reject zero denominators, including zero/zero.
`tonabis_core::errors::ArithmeticError` distinguishes `DivisionByZero` from
`Overflow`; a rounded-up result may overflow even when its floor fits.

Implementations live in `arithmetic.rs` and `errors.rs`; `lib.rs` only declares modules/tests.
There are no root item aliases. Operations are synchronous and deterministic,
without TON types, providers, runtime, I/O, protocol fees or pricing algorithms.
Existing independent BigUint and rounding-boundary tests retain their values.
All five protocols use core's shared wire helpers. Core depends on `ton_core`,
with no provider ownership. Public shared paths are:

- `adapters::nullable_internal_address::NullableInternalAddress`: nullable standard
  internal addresses, preserving absence versus internal zero and rejecting
  variable/anycast forms.
- `adapters::expected_bool::ExpectedBool<VALUE>`: literal phase/payload flags.
- `address::internal_address_cell`: checked standard-address cell construction.
- `msgs::EmptyBodyMsg`: empty funding with nonempty variant mismatch.

Ordinary Coffee fields, DeDust Operator parameters, LP/CPMM child storage and
PTON refund fields use `TonAddress` with `InternalAddress`, or
`Option<TonAddress>` with `NullableAddress`. These types
reject nonstandard addresses. Maybe/reference adapters retain their selector bits and reference placement.
Opaque MEV payloads and X1000 historical signed/context records retain their raw
graph contracts. TVM v10 prohibits variable/anycast parsing in current address
instructions; historical cell decoding is a separate contract. See the official
[version rules](https://github.com/ton-blockchain/ton/blob/master/doc/GlobalVersions.md).

These pre-release Rust API changes preserve standard wire encodings. Callers
construct the affected fields and child StateInit arguments with `TonAddress`,
and import empty funding bodies directly from core. No compatibility aliases
are supplied. Width annotations are omitted when native types express the width;
non-native widths, message prefixes and dictionary keys remain explicit.

## Protocol libraries

Each protocol owns `contracts.rs` and `contracts/`; primary modules own wrappers
and their constructors/getters. Persistent records live in `storage`, message
bodies/dispatch in `msgs` and meaningful structured getter records in
`method_results`. Shared records have one actual owner, imported directly.
No protocol features, aliases, math bridges or runtime ownership are introduced.

| Package | Wrappers / getters | Contract specification |
|---|---:|---|
| `tonabis_dedust` | 23 wrappers | [DeDust](dedust.md) |
| `tonabis_moon` | 18 / 54 | [Moon](moon.md) |
| `tonabis_stonks` | 8 / 26 | [STONks](stonks.md) |
| `tonabis_swap_coffee` | 26 / 60 | [swap.coffee](swap_coffee.md) |
| `tonabis_tonco` | 17 / 66 | [TONCO](tonco.md) |

Getters borrow caller-owned `ContractClient` and return native `TonResult` values.
Structured DTOs enforce their exact shape; scalar getters use native consumption.
Ordinary tails and native Cell/Slice conversions are accepted. Protocol-specific
flags, suffixes, nested selectors, wider address forms and signed graph identities
retain the constraints documented by their owning crate. Unknown bodies keep raw
cells; typed bodies validate their fields and dispatch follows the policy below. Codecs are synchronous and return
`TonCoreResult`; local StateInit constructors use caller-supplied code.
No local pricing, code authentication, signing, broadcasting, retries or hidden
background work is supplied. Source/capability/fixture manifests and focused
regressions document supported behavior and protocol constraints.

## Umbrella

`tonabis` is strictly a crate of crates. It always re-exports core and
re-exports selected protocol libraries under their original crate names:

- `tonabis::tonabis_core::arithmetic::mul_div_floor`
- `tonabis::tonabis_core::errors::ArithmeticError`
- `tonabis::tonabis_dedust::contracts::classic::factory_v2::DedustFactoryV2Contract`
- `tonabis::tonabis_moon::contracts::mempad::msgs::MoonMempadBuyMsg`
- `tonabis::tonabis_stonks::contracts::dedust_minter_v12::StonksDedustMinterV12Contract`
- `tonabis::tonabis_swap_coffee::contracts::factory::CoffeeFactoryContract`
- `tonabis::tonabis_tonco::contracts::v2::pool::ToncoPoolV2Contract`

Re-exports preserve original types and errors. No `tonabis::dedust` shortcut,
flattened item exports, convenience wrappers, conversions or new behavior are
introduced. Core is an unconditional dependency and re-export, including when
default features are disabled; there is no `core` feature. Default features are
empty. Independent `dedust`, `moon`, `stonks`, `swap_coffee` and `tonco` features
match protocol folder names and enable only their matching optional dependencies
and re-exports; `all` enables all five. Consumers may still depend
directly on an individual crate. Adding a protocol library requires adding a
matching workspace dependency, optional umbrella dependency, folder-name feature
and gated same-name re-export, inclusion in `all`, and a documentation inventory entry.

## Validation

Core's deterministic tests cover exact products, full-width overflow, rounding
and an unbounded integer oracle. Run them in debug, and in release for arithmetic
changes. The core example and README doctest demonstrate public module paths.
Umbrella README doctests and an external consumer exercise real usage through
all full crate names with `--all-features`. Validate the default build and each
of the 32 feature combinations independently, plus `all` and explicit no-default
behavior; unit tests merely repeating re-exports are unnecessary. The external
consumer also verifies codec/storage paths, caller-owned wrapper construction and
direct TON type identity; the default dependency graph must include core and no
protocols. GitHub distribution is validated through Git dependencies and the complete
repository source archive. Optional per-crate registry archive checks need local
patches for unpublished first-party dependencies; they are not a GitHub release gate.
Integration requires workspace tests, examples, strict Clippy, rustdoc and
formatting. Crate-specific commands and packaging limits live in AGENTS.md.

## TON dependencies and stack adapters

The workspace requires `ton = "0.4.12"` from crates.io. The checked lockfile
resolves `ton 0.4.12`, `ton_core 0.3.11` and `ton_macros 0.2.6`. These released
versions provide the stack adapters used here and support Rust 1.94.0. Consumers
can depend on `ton = "0.4.12"` directly to share the exposed TON types.

`FromTVMStack` result fields accept TLB adapters, e.g.
`#[from_tvm_stack(adapter = NullableAddress)]`. The derive consumes exactly one
Cell/Slice then delegates to `read(&mut CellParser)`, preserving the adapter's
address validation and ordinary-tail policy. Both quoted and unquoted adapter
expressions are accepted. TVM NULL is rejected, distinct from encoded addr_none.
Unannotated fields keep their existing conversions; structured result arity
remains exact. No parameter attributes or extra stack-adapter traits are added.

DeDust's `DedustPoolV2GetQuoteProvidersResult` derives with two NullableAddress fields.
Coffee's four factory owner getters and two pool wallet-address getters use
`&TonAddress` with handwritten InternalAddress Slice packing. This preserves
internal zero and rejects workchains outside int8. Pools return `TonAddress`;
the factory returns a structured address/hash result. Dictionary roots, nested
tuples, typed-reference contents and nullable address fields retain
their protocol-specific decoders.

The released native `MsgAddressInt` argument conversion supports broader wire
forms, but user-facing getter arguments follow the standard-address policy below.
External consumers need no TON patches. Extracted protocol packages currently
need the local core patch described above.

## Friendly address interfaces

Address-taking getters use TonAddress or Option<TonAddress> arguments,
with handwritten InternalAddress/NullableAddress packing where native argument
conversion would conflate zero and absence. The four Coffee factory owner getters
also follow this policy.
The concrete inventory is in tools/reference/integration/tonaddress-api.json.
Ordinary wire records, nullable result DTOs and construction/ordering helpers use
standard addresses with explicit absence. X1000 historical graphs retain raw forms. The upstream stack features remain available
but do not force raw types into user-facing getter signatures.

External, variable and anycast forms are not accepted as getter arguments. Required standard arguments use &TonAddress;
nullable encoded identifiers use Option<TonAddress> and a NullableAddress codec.
Standard scalar address results follow native TonAddress normalization, including
NULL/addr_none-to-zero where applicable. STONks nullable wallet results instead
preserve absence as None versus Some(internal zero). Selectors and stack order
are protocol-specific.

None does not have one universal stack representation: nullable encoded-address
arguments use an addr_none Slice; Moon's existing CLMM factory native-master
argument uses TVM NULL. Keep these protocol contracts explicit. No argument
attributes, public conversion wrappers, compatibility aliases or universal traits
are introduced. Raw types remain only in documented historical X1000 fields. StateInit and
address-ordering helpers expose standard identities with explicit absence.

## Consistent consumer types and parsing

Prefer `TonAddress` for standard TLB fields, caller-facing arguments and getter
results wherever behavior can be preserved. Use native adapters or narrow custom
conversions to preserve zero, absence, NULL, Cell/Slice acceptance and tail policy;
do not retain `MsgAddressInt` solely because its default conversion differs.

Ordinary contract addresses use `TonAddress` for required internal identities and
`Option<TonAddress>` where `addr_none` is allowed. `Some(TonAddress::ZERO)` is
internal zero, never absence. Wire fields use `InternalAddress`/`NullableAddress`;
Maybe and referenced fields use the matching core adapters, retaining their exact
selector bits and reference placement. Nonstandard variable/anycast/external
forms are rejected in these ordinary fields.

Ordinary nullable fields in structured getter results use derived FromTVMStack
with `#[from_tvm_stack(adapter = NullableInternalAddress)]`. Cell/Slice containing
addr_none becomes `None`; internal zero remains `Some(TonAddress::ZERO)`. TVM NULL
is rejected before the field adapter runs. This replaces the former generic
NULL-to-addr_none tolerance for these fields. Exact result arity and ordinary
cell tails remain unchanged.

Explicitly NULL-returning fields retain their protocol-specific decoder. Moon's
mempad admin and TONCO V2 router optional addresses use `Option<Option<TonAddress>>`:
outer None is TVM NULL, Some(None) is encoded addr_none, and Some(Some(address))
is an internal identity. Dictionary and typed-reference results may also need
manual decoding; that is not a reason to retain boilerplate on ordinary records.
Scalar getter results retain their separately documented NULL normalization.


Ordinary TLB records tolerate unused bits/references after their modeled fields;
there is no blanket empty-parser requirement. Required fields, tags, phase flags,
explicit suffixes and empty-body variant matching still
validate their structure. Structured TVM results retain exact stack arity. Unknown
bodies and signed graphs retain their explicit preservation guarantees.

For a new group, follow the root AGENTS address/parsing decisions and flat module
ownership rules. Crate README examples are the consumer entry points; generator
inventories document ownership but are not required to construct messages or call
getters. Use module-level type imports and direct canonical shared-record paths.


### Address API migration

Former ordinary `MsgAddress` fields now use `Option<TonAddress>`; replace addr_none
with None and internal values with Some(address). Former required `MsgAddressInt`
fields use TonAddress. Do not call TonAddress::to_cell for required internal zero;
serialize the containing record or use core address helpers. MaybeRefAddress,
MaybeRefNullableAddress and MaybeAddress preserve explicit Maybe/ref layouts.
A nested Option means two distinct absence states; see the table in root AGENTS.md.
These are intentional source API changes before publication, not aliases. Wire
bytes for supported standard identities, selectors and independent fixture bytes
remain unchanged. Rejecting nonstandard forms in ordinary fields is intentional;
this is not a promise of equivalence for arbitrary historical cells.

The implementation audit covered all seven crates. Direct raw address fields
remain only in DeDust X1000 historical message/context records. Raw values may
also occur privately while decoding nullable stacks. The current public mapping
is tools/reference/integration/tonaddress-api.json. The production sample checked
on 2026-10-29 covered 12 Coffee/DeDust accounts and 29 address observations (28
standard, one addr_none, no var/anycast); this is not exhaustive live validation
of every contract generation. TON's v10 address-instruction restrictions support
the current standard domain; opaque historical cells remain a separate contract.

## Public type naming

Public type names identify the protocol, contract or shared family, and generation
where needed. Contract-local helpers, proofs, messages, dispatch enums and structured
getter results follow the same rule as wrappers and storage records. For example,
`CoffeeConstantProductPoolProof` and `CoffeeCurveFiStablePoolProof` remain distinct
entities. Equal layouts alone do not justify merging them. Existing shared records
keep their canonical owner and a family-level name.

The [type migration map](../../tools/reference/integration/type-renames.json) records
the old and new names with their defining files. These are breaking Rust source
renames without aliases; module paths, fields, variants, selectors, wire layouts
and independent fixture identities remain unchanged. Generators map upstream
schema names to the public names rather than altering source evidence.

## Message dispatch

Typed body codecs validate required fields, widths, tags and explicit constraints.
Open dispatch enums ending in `Unknown(TonCell)` perform best-effort classification.
Native TON derives may fall through to Unknown on a prefix mismatch, including a
nested mismatch inside an otherwise recognized opcode. This is accepted behavior:
the original raw body is retained and re-encodes unchanged. Other decode errors,
including truncation, may still propagate. Unknown means unclassified, not valid
input or necessarily an unfamiliar opcode. Decode the concrete body type when
its validation result is needed; parsing never authenticates a message.

Existing explicit dispatchers can commit to a recognized opcode and propagate
nested errors. Their behavior is retained, including DeDust classic pools/X1000 V1
and Coffee factory proofs. Do not add a blanket catch-all for every decoding error
or silently convert these stricter paths. The Native Vault V2 nested-invalid-asset
regression protects the accepted raw-body fallback separately from truncation.

## GitHub distribution

The first release version and Git tag are both `0.1.0` (without a `v` prefix).
All seven packages inherit that workspace version. Distribute the complete
repository through a GitHub release at that tag; crates.io publishing remains
disabled with `publish = false`. Internal workspace path dependencies resolve
within the Git checkout or extracted repository archive; they do not require
published first-party crates or consumer patches. TON dependencies remain on
crates.io. Existing licensing and upstream notices are unchanged.

After the release exists, a consumer can select the umbrella or a protocol:

```toml
[dependencies]
tonabis = { git = "https://github.com/ston-fi/tonabis-rs", tag = "0.1.0", features = ["dedust"] }
# Alternatively: tonabis_dedust = { git = "https://github.com/ston-fi/tonabis-rs", tag = "0.1.0" }
ton = "0.4.12"
```

Validate the accepted release revision with the workspace checks and standalone
consumers using a Git dependency and the complete source archive. Verify the tag,
package versions and retained notices before publishing the GitHub release.
A local candidate snapshot proves packaging/consumption mechanics only; it does
not establish that the remote tag or GitHub release exists. Creation/push of the
release tag and GitHub release are separate publication actions. Per-crate
`cargo package` registry-resolution failures are not blockers for this GitHub-only
distribution model; do not enable crates.io publication to bypass them.
