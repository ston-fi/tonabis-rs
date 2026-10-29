# TONCO contract binding specification

## Scope and provenance

`tonabis_tonco` provides 17 wrappers, 66 callable getters,
persistent storage, message codecs/dispatch, local StateInit/address methods and
wallet ordering. V1, V2 (SDK v1.6 / version 16000), factory orders, shared native
minter, separate native wallets and farming are included. See the crate README
for canonical imports and the reference manifest for every symbol/fixture mapping.

SDK evidence is `@toncodex/sdk` 1.3.1-mainnet; V1 catalog revision is
`7dc57067d85c95dc25c09ccaaf108eaba62f6303`. Executable contract cells and replay
tooling are not distributed.
No pricing, simulation, math-state conversion, capture tool, compatibility harness
or runtime/application service is included. The package depends only on registry
TON and integer values; it is unpublished, proprietary, and has no features.

## Public contract and ownership

`contracts::v1` and `contracts::v2` each own pool/router/factory/order/account/
position_nft primary modules. Names identify protocol and generation. Each primary
module has `storage`, `msgs`, and `method_results` where needed. Storage exposes
caller-code StateInit; non-order root storage also derives addresses. Native
minter storage derives wallets using its caller-supplied wallet code. No helper
authenticates code or executes deployment.

Farming separates distributor and user-distributor wrappers; shared claim,
callback, funding and Merkle entry messages belong to `farming::msgs`.
ToncoFarmingDistributorMerkleRoots belongs to distributor storage. Shared V2 reforge/payout records live
under `v2::msgs`. Identical TEP-89 responses and referenced owners belong only to
native minter msgs, including when used by factory orders. The two native wallet
wrappers share storage/results/messages while retaining their different dispatch
enums. ToncoPoolTickPage and ToncoPoolTickRecord live in V1 pool method_results and are imported by
V2 directly. ToncoRouterV2OptionalU64 belongs to V2 router storage; other explicit sequence
suffix users import it directly. Rich-bounce records belong to V2 position NFT
msgs, the consumer that restores deposited positions.

Getters are `#[ton_methods]` methods on caller-owned `ContractClient` wrappers;
camel-case selectors, arguments/order and numeric selectors are unchanged.
Constructors send nothing. There is no retry or runtime ownership; cancellation
follows the caller future/client. Provider errors propagate as `TonResult`;
codec/local methods return `TonCoreResult`. Local protocol validation uses native
typed errors. Callers own authority, gas, coherence and settlement policy.

## Wire invariants

- V1 ticks are in a referenced signed-key dictionary; V2 HashmapE is inline in
  State. Tick keys/bounds are signed 24-bit. V2 fields preserve uint160 cached
  prices, uint256 gross liquidity, int128 net liquidity and int224 growth.
- Mint/burn bodies differ by generation. V2 uses account order, funding,
  add-liquidity, reforge, NFT burn and position-deposit flows. Unsupported stale
  SDK POOL_MINT/POOL_BURN selectors do not become active V2 wrappers.
- V2 pool swap keeps source, owner, origin, direction and refund wallet before
  its references. NFT initialization references position state and may append an
  untagged indexer-log reference; report data retains optional fee/payload refs.
- Router CreatePool roles start with controller/creator; pool Init has admin/
  controller/creator and referenced secondary/oracle roles. Their types differ.
- Factory whitelist keys are 255 bits after the standard-address three-bit
  prefix, including workchain bits, and values are referenced address/permission
  records. Runtime factory and pool Init sequence suffixes are untagged.
- V1 payout code 0 retains its second optional indexer tag. Payout forwarding is
  an optional untagged ref. Known indexer records are typed; unknown bodies survive.
- Native minter deploy-wallet uses `0x4f5f4313`, not stale `0xcf5f4313`.
  Wallet V1/V2 remain separate; V2 additionally recognizes jetton-transfer.
- Farming claim reads query ID and proof ref. Its legacy inline index suffix is
  preserved without authenticating the claim. Merkle-root dictionary entries,
  callback fields and helper storage/derivation identities remain available.
- Wallet ordering compares descending hash of complete serialized addresses,
  rejecting identical cells; it does not compare only account hash bytes.

## Parsing policy and manual exceptions

Ordinary root/reference/address/dictionary tails follow native TON adapters and
may be discarded on re-encoding. Typed codecs validate message fields; native dispatch follows the
[workspace prefix-fallback policy](workspace.md#message-dispatch). Unknown bodies
retain raw graphs. Native cell results accept Cell/Slice; optional cells retain
NULL separately from an empty cell. Structured result DTOs enforce exact stack arity. Scalar results return native values directly and may
leave stack tails. Signed getter integer domains remain independent of storage
bit widths. Nullable address getters preserve NULL, addr_none and internal zero as described below.

Ordinary address fields use `TonAddress`/`InternalAddress` or
`Option<TonAddress>`/`NullableAddress`; internal zero remains distinct from absence.
Core Maybe/reference adapters retain selector bits and reference placement.
Ordinary nullable result fields derive FromTVMStack with NullableInternalAddress:
Cell/Slice addr_none is None; TVM NULL is rejected. Explicit NULL fields keep their
protocol handling, and structured result arity stays exact;
see the [workspace address policy](workspace.md#consistent-consumer-types-and-parsing).
Variable, anycast and external addresses are rejected in ordinary fields.

Native stack/dictionary conversions are used where compatible. Handwritten codecs
preserve explicit optional router-address NULL values and protocol behavior:
ToncoPoolTickPage validates grouped tuples, three/five
fields, tick domain and whole-stack arity; the other ten model untagged
sequence/index, initial/runtime timelocks, remaining-reference roles, ignored SDK
settings tags, funding orders, collect targets, NFT indexer logs and V1 payout
forwarding. Native tagged Option or an ordinary record derive cannot encode
these distinctions. Each exact exception is mapped to focused tests in
`tools/reference/tonco/bindings.json`. All ordinary records derive TLB; other structured results derive FromTVMStack where native field conversion
matches the declared stack kinds. No manual ToTVMStack remains.

## Offline evidence and limits

Unchanged source fixtures include 46 SDK wire cases (including stale-layout
rejections), 70 independent protocol messages, 66 structural getter rows, 16
independent synthetic StateInit identities, 10 wallet-ordering rows, 32 dictionary
boundary encodings and synthetic native/V2 flow cells. The mixed V2 flow test
retains structural/message assertions and excludes numerical simulation. Generic
strict-tail/Cell-only/address-leaf expectations are rewritten for native policy;
explicit protocol suffix validation remains. Every one of the 66 callable
getters has a recording-provider invocation test. Boundary checks cover NULL,
zero, Cell/Slice, malformed structured stacks, nested ticks, untagged suffixes
and typed failure propagation without retry.

Fixtures are package-local and use inert/empty opaque code fields. No captured
executable, numerical snapshot, live acquisition or sidecar test is included.
Package/consumer checks establish local distribution/import/type identity;
they do not establish deployed parity, economic safety, code authentication,
registry availability or publication approval. Applicable SDK/catalog notices
are retained separately from the proprietary workspace policy.

## User-facing address signatures

The eleven address-taking V1/V2/native getters use &TonAddress with handwritten
InternalAddress Slice packing and int8 workchain validation. Address scalars
return TonAddress; initial-data/state-init cells remain TonCell. whitelist_key
also takes &TonAddress and preserves exact 255-bit extraction. ordered_wallets takes and returns Option<TonAddress>, preserving addr_none and
ordering by complete address-cell hashes.
Optional router result DTOs retain NULL versus present addr_none/internal zero.
See the [shared address API policy](workspace.md#friendly-address-interfaces)
for address encoding and result semantics.


Optional stack address fields use `Option<Option<TonAddress>>`: outer `None` is
TVM NULL, `Some(None)` is encoded addr_none, and `Some(Some(address))` is internal.
