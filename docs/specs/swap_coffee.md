# swap.coffee contract bindings

`tonabis_swap_coffee` provides contract wrappers, codecs and getters based on
TON Core catalog revision `7dc57067d85c95dc25c09ccaaf108eaba62f6303` and
independently expected code-free fixtures. The binding inventory records
public symbols, getter signatures, protocol exceptions and fixture provenance.

The TEP-74 Jetton notification envelope shared by CrossDex (all generations),
MEV (both generations) and staking vault is
`contracts::cross_dex::msgs::CoffeeCrossDexJettonTransferNotificationMsg`.
Import this canonical record directly; the former per-contract copies are removed
without aliases. Each contract keeps its own dispatch enum and message roles.
The opcode, fields and independent family fixtures are unchanged.

## Shared record ownership

Common generated pool asset/proof records have one owner in
`pool_constant_product::storage`; common pool message values live in its `msgs`.
Common vault message values live in `vault_native::msgs`. Staking asset/position
data lives in `staking_item::storage`. Other families use direct imports from
these owners. Explicit canonical mappings retain the source schemas and all
independent wire vectors without exposing duplicate Rust representations.
Generation-specific storage and transport-specific bodies/dispatch remain distinct.
Pool proofs that embed different AMM settings remain pool-specific; identical
immediate fields alone do not establish equal nested wire representations.

## Surface and ownership

The 18 catalog families are factory, Init, pool creator, constant-product/stable
pools, native/Jetton/extra vaults, liquidity depository, LP wallet, staking
master/item/vault, current/legacy/BidAsk-facing CrossDex and current/legacy MEV.
The additional single-item staking master has two root references; catalog masters
have three. There are 26 explicit wrappers and 60 callable getters after expanding
shared runtime/deployment implementations. Deployment and runtime storage are
separate types; callers select the revision and supply code rather than requesting
layout detection. Coffee-facing BidAsk/foreign payloads do not add another DEX SDK.

Primary modules own wrappers; `storage` owns persistent records and identity data,
`msgs` owns bodies/role dispatch, and `method_results` owns structured getter DTOs.
Shared staking dictionary values and vault proofs retain one canonical owner.
Vault initialization identities and their asset union/variant records are owned
once by `vault_native::storage`; extra-currency and Jetton storage/messages use
direct imports of those types. The StateInit/address implementation also exists
once at that owner. Distinct vault wrappers, phase flags and runtime layouts are
preserved. No convenience re-exports or compatibility aliases are supplied. Scalar getters return native values and canonical asset sorting returns `TonCoreResult<([Asset; 2], bool)>`,
with identical assets rejected. Pair order uses tags, unsigned Jetton chain bytes
and uint32 extra IDs. Caller-supplied code/data yield native StateInit and local
addresses; parsing/derivation does not prove authority or factory admission.

## Wire and stack contracts

Preserve exact selectors, field widths, references, argument order and stack kinds.
Native TL-B derives encode ordinary records and dispatch. Ordinary root, referenced
and dictionary tails are tolerated. Typed codecs validate message fields; native dispatch follows the
[workspace prefix-fallback policy](workspace.md#message-dispatch). Unknown
bodies retain their raw cells. Internal/external/bounce roles remain distinct even
when selectors collide. Only documented incoming roles accept a truly empty
funding body; a cell with references is not empty.

Wire nullable addresses encode addr_none without a Maybe selector. Literal internal
zero remains present. Ordinary fields use standard TonAddress values; opaque signed payloads retain their raw graphs. Nullable getter results use
Option<TonAddress> and reject variable/anycast addresses. Structured nullable address results accept Cell/Slice containing addr_none and
reject TVM NULL, preserving absence versus internal zero. Scalar CrossDex
`get_admin` retains native MsgAddress conversion, accepting NULL as absence.
Structured results reject extra, missing and wrong-type entries; scalar conversions
consume one item and permit older stack entries. Native cell results accept Cell
and Slice. Getter arguments requiring Slice are explicitly packed, while cell
arguments remain Cell. Provider errors retain their type and each method issues one
request, without retry or fallback.

Protocol constraints survive the general tail policy: active/inactive Jetton flags;
zero/uninitialized staking identities; bounded optional selector suffixes; TEP-89
omitted/not-included/included-owner responses; full-width extra-currency IDs; typed
staking dictionaries/callback roles; and signed external envelopes. MEV action keys
are signed16. Decoded signed batches cache the exact unsigned body together with
its canonical logical identity. Unchanged logical fields (including signature-only
changes) reuse that body; changed fields encode canonically. The library neither
creates nor checks signatures. The unsigned envelope is completely consumed, while
its original dictionary graph is preserved.

## Runtime, errors and evidence

`TonContract::new` is synchronous and does not access the network. Getters borrow
a caller-owned client and optional transaction context, return `TonResult`, and
inherit cancellation from the caller's future. Codecs/local identities use
`TonCoreResult`. There are no background tasks, runtime ownership, request queues,
shutdown routines, state services, signing, broadcasting, gas or settlement policy.
On-chain estimate getters remain contract APIs; local quote/math behavior is excluded.

Default tests use crate-local synthetic/observed data, recording providers and
independent expected hashes/bytes. Roundtrips supplement those expectations.
Fixtures contain no imported executable cells and need no external source checkout. Captured observations retain their historical provenance and do
not prove current chain execution, contract authenticity, cryptographic acceptance,
universal equivalence or deployment/release readiness. See the crate guide for
exact checks, packaging and extracted-package consumer requirements.

## Flat API and native dispatch

Types and their implementations share flat `storage.rs`, `msgs.rs` and
`method_results.rs` files. Construction, signatures, emissions and bounces have
no extra child modules. Complete authored modules are outside generator ownership.
Factory getters accept `factory::msgs::CoffeeFactoryAsset`; pool estimation takes
`pool_constant_product::storage::CoffeePoolAsset`. Their ToTVMStack implementations
pack tagged Native/Jetton/Extra Slice values. A TonAddress alone cannot represent
that asset domain. Factory owner arguments use &TonAddress with explicit InternalAddress Slice
packing, preserving internal zero.
The two pool get_wallet_address APIs use &TonAddress -> TonResult<TonAddress>
with standard-address input validation. Their handwritten bodies encode the standard
owner with InternalAddress as a Slice, preserving literal zero and rejecting
workchains outside int8 before making a request. The result follows native
TonAddress conversion (including NULL/addr_none-to-zero normalization). These APIs accept standard internal addresses only.

Message enums have one final Unknown(TonCell). Short/reference-only bodies retain
their graphs there; empty funding is a distinct typed variant only in supported
incoming roles. Empty-body matching uses a narrow mismatch codec because native
ensure_empty errors cannot fall through. Ordinary dispatch is derived and unboxed:
the locked Box<T> codec hides opcode prefixes and can swallow malformed known
fields. A few large-enum lint allowances avoid that ambiguity without inventing
wrapper types. Factory incoming dispatch remains explicit for fixed-tag nested
proofs. Signed bodies still retain their original unsigned graphs.

## User-facing address signatures

The four factory owner getters and both pool wallet-address getters take
&TonAddress and use handwritten InternalAddress Slice packing. Internal zero is
preserved and int8 workchains are checked before provider access. Pools return
TonAddress; the factory retains its structured address/hash result. Tagged asset
arguments, nullable standard-address result records and wire domains stay distinct.
See the [shared address API policy](workspace.md#friendly-address-interfaces)
for address encoding and result semantics.


## Address representation

Ordinary address fields use `TonAddress`/`InternalAddress` or
`Option<TonAddress>`/`NullableAddress`; internal zero remains distinct from absence.
Core Maybe/reference adapters retain selector bits and reference placement.
Ordinary nullable result fields derive FromTVMStack with NullableInternalAddress:
Cell/Slice addr_none is None; TVM NULL is rejected. Explicit NULL fields keep their
protocol handling, and structured result arity stays exact;
see the [workspace address policy](workspace.md#consistent-consumer-types-and-parsing).
Variable, anycast and external addresses are rejected in ordinary fields.
