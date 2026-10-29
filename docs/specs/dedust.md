# DeDust contract bindings

Classic V2 native and Jetton vaults share pool identity and swap continuation
records from `contracts::classic::vault_native_v2::storage`: `DedustPoolType`,
`DedustVaultNativeV2PoolParams`, `DedustSwapKind`, `DedustSwapStepParams`, `DedustSwapStep`,
`DedustSwapParams` and their tag records. Import that owner directly for either
vault; the former Jetton-local copies are removed without aliases. Wire formats
and independent fixtures are unchanged.

## Supported surface

`tonabis_dedust::contracts` exposes the 23 wrappers listed in the
[crate README](../../crates/dedust/README.md). Local numerical
algorithms and conversions into math state are outside this repository.

Classic factory, pool, vault and deposit V1/V2 remain separate. Classic Protocol2
is not CPMM v2. CPMM deployment selects `Revision::PoolV1` or `PoolV2` explicitly;
Uranus minter/wallet V2/V3 also remain distinct. Managed pools extend classic V2
storage; ordinary V2 parsing accepts the modeled prefix and ignores that tail.
X1000 V1 and Uranus Factory V3 have wrappers/storage/messages without invented
getters. Decoding never classifies deployed code or establishes authority.

## Consumer behavior

Constructors and codecs are synchronous; getters are async over caller-owned
TON clients. Types/errors use the public `ton` crate directly. Callers own
providers, runtime, coherent snapshots, code/revision identity, gas, authority
and settlement. There are no background workers, clock, signer or broadcaster.
Storage, messages and structured results have canonical child modules; scalar
getters return values directly. The crate has no feature flags.

Factories provide child state-init helpers using caller-provided blank code,
factory identity and revision-specific parameters. Equal asset pairs fail;
wire ordering determines canonical pairs. CPMM position and Uranus wallet
construction preserves library references, split depth and shard-prefix rules.
Construction is a local cell/hash operation, not deployment.

## Parsing and serialization

Ordinary cell/BOC, referenced-record, address-cell and dictionary tails are
accepted. Typed body codecs reject missing fields and invalid tags. Native dispatch
may preserve nested prefix mismatches as Unknown; other errors still propagate.
Unknown is an unclassified raw body, not a validity or unfamiliar-opcode guarantee. Ordinary
serialization emits modeled fields and may discard unmodeled tails. Unknown
message bodies, explicitly modeled forwarding payloads and unchanged signed
references retain their raw graphs. Native TON adapters validate dictionaries;
there are no DeDust label compatibility guards or forced leaf/fork exhaustion.

Required standard addresses use `TonAddress` with `InternalAddress`; nullable
standard addresses use `Option<TonAddress>` with `NullableAddress`. Both preserve
literal internal zero separately from absence. Raw address records remain only in historical X1000 layouts. Nullable structured fields
accept encoded addr_none and reject TVM NULL; existing scalar TonAddress getters retain native
NULL/addr_none-to-zero normalization. Native
cells accept Cell/Slice, while optional cells distinguish NULL and an empty cell.

Complete structured result DTOs check exact stack arity. Native scalar returns
use normal stack consumption and may leave extra stack values. Coins getter
amounts are checked u128; signed computed results remain I512. Unsigned
checkpoints/growth/hashes use U256 without an additional Q248 limit. Storage
retains its exact TL-B widths. Counters use native u32/u8. Factory `is_stable`
packs 0/1 and classic Asset arguments pack Slice. Pending-operation selector
97376 keeps signed/outside-uint64 arguments and reports protocol absence.

## Signed actions and special payloads

X1000 V1 typed authorization, command and restricted action lists are distinct
from V2 actions, caches, continuations and callbacks. DedustSignatureRef preserves
original referenced graphs while typed fields remain unchanged, including valid
alternative dictionary labels. Changing fields emits their new encoding and
requires caller-owned re-signing; no signature/authority verification is implied.

Ordinary dispatch derives TLB with unknown raw bodies. Classic pool proofs and
X1000 V1 continuations retain explicit opcode dispatch so nested prefix failures
remain errors. Bounced is an envelope flag; restoration paths may skip a marker,
check it, or return before parsing. Preserve the existing generation-specific
behavior. CPMM payment/notification and classic forwarded payloads retain their
inline/ref choices and context-specific suffix boundaries. Executable program
fields remain opaque; no arbitrary program ABI is implied by a typed envelope.

## Acceptance and maintenance

Offline wire, getter, address, signed-graph, initialization and malformed-input
regressions protect the public contract. README examples must compile and docs
must accurately describe runtime and dependency requirements.

Generation must match pinned ABI hashes, preserve handwritten files and produce
identical formatted output on a second pass. The mapping describes 19 generated
schemas and all 23 API wrappers. See the [tooling guide](../../tools/reference/dedust/README.md).
Future behavior changes require source evidence and focused tests; passing the
offline suite does not establish live deployment parity or economic safety.


## Address representation

Ordinary address fields use `TonAddress`/`InternalAddress` or
`Option<TonAddress>`/`NullableAddress`; internal zero remains distinct from absence.
Core Maybe/reference adapters retain selector bits and reference placement.
Ordinary nullable result fields derive FromTVMStack with NullableInternalAddress:
Cell/Slice addr_none is None; TVM NULL is rejected. Explicit NULL fields keep their
protocol handling, and structured result arity stays exact;
see the [workspace address policy](workspace.md#consistent-consumer-types-and-parsing).
Variable, anycast and external addresses are rejected in ordinary fields.

Raw address types remain only in X1000 historical message/context records. Their
signed, opaque or bare-addr_std layouts are retained; these are not patterns for
ordinary new contracts. Address dictionaries use a required-internal value
adapter, including when the hash is zero.

The Native Vault V2 inline deposit candidate is reference-only evidence, excluded
from public bindings and listed under `excluded_declarations` in the ownership
manifest. The supported deposit message carries `DedustDepositLiquidityParams`
by reference. The original catalog and fixture bytes remain unchanged.
