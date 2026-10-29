# Moon contract bindings

## Scope and identities

The [crate README](../../crates/moon/README.md) inventories all 18 wrappers and 54
getters. Current SDK 2.0.1 CPMM/CLMM/mempad and catalog pool/range, order factory,
old/new orders and booster remain separate generations. Local pricing, liquidity
simulation, state-to-math conversions, market adapters, capture and replay tools
are excluded. Moon uses TON, integer value types and core wire helpers; it has no pricing math.

The package is unpublished, with no feature flags. Public `ton` identities,
caller-owned clients, typed errors and asynchronous getter cancellation are used
directly. There are no retries, signer, broadcaster, hidden tasks, gas estimates
or settlement policy. Local code/data constructors do not deploy contracts.

## Canonical structure

Wrappers own their primary files and one inherent method block. Each wrapper's
persistent schema is in `storage`; nontrivial result DTOs are in `method_results`.
SDK families own shared `msgs` because their wire messages span cooperating
contracts; catalog wrappers each own `msgs`. There are no parent aliases or
flattened exports. Ordinary records use native TLB derives/adapters, with
handwritten codecs only for demonstrated protocol exceptions. Scalar getters
return values directly. The umbrella re-exports the full `tonabis_moon` crate.

## Wire and getter behavior

Ordinary address fields use `TonAddress`/`InternalAddress` or
`Option<TonAddress>`/`NullableAddress`; internal zero remains distinct from absence.
Core Maybe/reference adapters retain selector bits and reference placement.
Ordinary nullable result fields derive FromTVMStack with NullableInternalAddress:
Cell/Slice addr_none is None; TVM NULL is rejected. Explicit NULL fields keep their
protocol handling, and structured result arity stays exact;
see the [workspace address policy](workspace.md#consistent-consumer-types-and-parsing).
Variable, anycast and external addresses are rejected in ordinary fields.

Typed body codecs validate their fields; native dispatch follows the
[workspace prefix-fallback policy](workspace.md#message-dispatch). Mempad dispatch is explicit because a nested mint
transfer opcode failure must not fall through to Unknown. Other message enums
use native TLB dispatch. Transfer flags, booster mode/funding constraints, lifecycle
discriminants, metadata byte alignment/linear references and layout recognition
remain enforced. Metadata preserves chunk boundaries and non-UTF8 bytes. Standard
and referenced bounce bodies preserve the source-defined remainder/diagnostics.

Structured result records enforce exact stack arity; scalar results use native TON
consumption. Cell and Slice results follow native conversion, optional cells keep
NULL distinct from an empty cell. Ordinary structured address fields reject TVM NULL and accept encoded addr_none;
explicit optional addresses distinguish NULL and present addr_none/internal zero. No custom
stack leaf exhaustion or cell-kind compatibility layer remains. Original signed
getter integer domains are retained, independently of storage widths.

Current CLMM camel-case getters retain their selectors, distinct from catalog
snake-case APIs. Selectors 112561 and 78683 use get_account_address/get_null with explicit IDs. The
factory's optional native master argument packs TVM NULL; a present zero master
packs a standard internal Slice. The two observed order-fee getters use 80604.
CPMM factory/pool/account, CLMM account and launch factory have no invented getters.

CPMM explicit layout recognition distinguishes legacy factory fees, pool masters
without initiator, and init-only positions. Ordinary typed records do not classify
code. Address derivation takes caller-supplied revision-correct code and asset
ordering. Launch wallets use StateInit split depth eight and splice the owner's
first hash byte; use their dedicated helper rather than ordinary derivation.
No decoding operation establishes authenticity, permission or block coherence.

## Native primitive boundaries

The workspace uses the locked `ton` 0.4.12, `ton_core` 0.3.11 and
`ton_macros` 0.2.6 sources. Cell, dictionary, address, integer, stack and StateInit
operations use those crates directly. Moon validation uses native typed data
errors; their producer identifies the validating module.

`MoonReferencedAddress` supplies `InternalAddress` validation inside a reference or
dictionary value, where the native adapters cannot be composed directly. The
CLMM factory writes its argument with `InternalAddress` directly: unlike
`TonAddress::to_msg_address_int()`, this rejects workchains outside signed eight
bits instead of truncating them. Native jetton transfer messages allow broader
addresses; Moon's records enforce their required address domains. Both native
`TLBEitherRef` and Moon's native `TLBEither` field can preserve payload placement;
`TLBEither` also distinguishes inline and referenced variants in equality.

Transfer records derive TLB with a core ExpectedBool field adapter, validating
both read and write. A syntactically valid opposite flag selects a different
layout and is rejected. Native derives alone cannot express booster mode/tail
relationship, restricted lifecycle tags or untagged CPMM revision recognition.
The exact one-NULL result and range dictionary conversion also require small
handwritten stack decoders. These exceptions retain native field codecs.

`MoonPrefixedString` deliberately retains Moon's zero-prefix, byte-alignment and
linear-reference validation, including exact preservation of empty continuation
cells. Native `SnakeData` accepts broader cell layouts and drops empty continuation
cells on re-encoding, so it is not an equivalent replacement for this contract.

## Evidence and acceptance

All 315 independent schema wire vectors retain their original bytes and hashes.
Synthetic legacy/current states, signed dictionaries and the independent wallet
hash also remain. Tests cover native tail and Cell/Slice handling alongside protocol-specific
constraints. Every getter is exercised through a mock provider
against independently specified numeric selectors, with argument order/kinds and
structured shape checks. Tests do not contact a live provider.

The generator consumes a pinned normalized wire schema and writes an explicit
allowlist. Original source-file hashes and source type names preserve provenance;
independent fixtures remain separate expected evidence. Generation must preserve
handwritten files and be identical after a second formatted pass. Packaging and
an external consumer check public imports and TON identities. Existing SDK/catalog
notices are retained; no executable capture corpus or new licensing grant is added.
Passing local tests is not live parity, economic qualification or publication approval.

## User-facing address signatures

The five account/wallet getters use &TonAddress and return TonAddress. Order
factory get_pipe takes Option<TonAddress>: None encodes an addr_none Slice and
Some(internal zero) stays internal. The existing CLMM factory get_pool_address
instead encodes a None master as TVM NULL; preserve that distinct contract.
Handwritten bodies preserve selectors/order and validate int8 workchains before
provider access. Wire/result records use standard addresses and preserve explicit absence.
See the [shared address API policy](workspace.md#friendly-address-interfaces)
for address encoding and result semantics.


Optional stack address fields use `Option<Option<TonAddress>>`: outer `None` is
TVM NULL, `Some(None)` is encoded addr_none, and `Some(Some(address))` is internal.
