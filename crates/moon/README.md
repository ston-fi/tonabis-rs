# tonabis_moon

Moon.cx contract wrappers and wire codecs, using public `ton` types directly.
The current SDK 2.0.1 families and older catalog generations have separate APIs.
This crate contains **contracts only**: no pricing, liquidity simulation, math-state
conversion, market adapter, signer, broadcaster, provider ownership or hidden tasks.
Rust 1.94.0 / edition 2024; unpublished (`publish = false`), no feature flags.

## Contract inventory

All paths below start at `tonabis_moon::contracts`. Each wrapper uses
`TonContract::new(&client, &address, tx)` with a caller-owned `ContractClient`.
Construction is synchronous; getters are asynchronous. There are 18 wrappers and
54 getter methods. A module without getters does not invent them from storage.

| Module | Wrapper | Getters / role |
|---|---|---|
| `cpmm::factory` | `MoonCpmmFactoryContract` | Raw factory state; current and legacy layouts |
| `cpmm::pool` | `MoonCpmmPoolContract` | Raw pool state; current and legacy masters |
| `cpmm::account` | `MoonCpmmAccountContract` | LP account state |
| `cpmm::position` | `MoonCpmmPositionContract` | `get_locked`, `get_position`; init/live state |
| `clmm::factory` | `MoonClmmFactoryContract` | `get_pool_address` |
| `clmm::pool` | `MoonClmmPoolContract` | `get_pool`, `get_range_by_num`, `get_ranges` |
| `clmm::range` | `MoonClmmRangeContract` | `get_range`, numeric `get_account_address` |
| `clmm::account` | `MoonClmmAccountContract` | LP account state |
| `clmm::position` | `MoonClmmPositionContract` | `get_position` |
| `mempad::factory` | `MoonMempadFactoryContract` | Launch factory state |
| `mempad::curve` | `MoonMempadCurveContract` | `get_wallet_address`, `get_jetton_data`, `get_null` |
| `mempad::wallet` | `MoonMempadWalletContract` | `get_wallet_data` |
| `legacy_pool` | `MoonLegacyPoolContract` | 13 catalog reserve, asset, wallet, fee, LP, estimate and raw-state getters |
| `catalog_range` | `MoonCatalogClmmRangeContract` | Six catalog range, bin, LP-account, NFT and price getters |
| `order_factory` | `MoonOrderFactoryContract` | `get_id`, `get_pipe` |
| `order_old` | `MoonOrderOldContract` | Eight status, order, vesting, assets, fill and collected-fee getters |
| `order_new` | `MoonOrderNewContract` | Eight corresponding getters for the distinct new generation |
| `booster` | `MoonBoosterContract` | `get_booster`, `get_status`, `get_boost_data`, `get_pool` |

Persistent records live under each wrapper's `storage`; useful multi-field getter
records live in `method_results`. Scalar getters return values directly.
CPMM, CLMM and mempad share wire records within each family's `msgs` module;
these messages span multiple contracts and retain a single canonical definition.
Catalog wrappers each have their own `msgs`. Message/payload types use a `Msg`
suffix; no parent re-exports or compatibility aliases are provided.

Special codecs and their types live directly in the owning `msgs.rs` or
`storage.rs`. SDK transfer records derive TLB and use a literal payload-flag
adapter: false selects the no-payload layout; true selects the referenced-payload
layout. Booster mode/funding constraints, launch lifecycle domains and CPMM
revision recognition remain explicit in those flat files.
External order and booster trigger bodies live directly in their wrapper's `msgs`.
`metadata::MoonPrefixedString` preserves byte-snake chunk boundaries and arbitrary
bytes, including empty continuation cells. It validates a zero prefix, byte
alignment and linear references; native `SnakeData` does not enforce these
constraints or preserve empty continuations. Shared TEP-74/bounce envelopes live
in `contracts::msgs`.

## Use

```toml
[dependencies]
tonabis_moon = { path = "../tonabis-rs/crates/moon" }
ton = "0.4.12"
```

Encode a native launch buy with caller-chosen protection:

```rust
use tonabis_moon::contracts::mempad::msgs::MoonMempadBuyMsg;
use ton::ton_core::{traits::tlb::TLB, types::tlb_core::TLBCoins};

let request = MoonMempadBuyMsg {
    query_id: 42,
    min_out: TLBCoins::new(1_000),
};
let body = request.to_cell()?;
assert_eq!(MoonMempadBuyMsg::from_cell(&body)?.query_id, 42);
# Ok::<(), ton::ton_core::errors::TonCoreError>(())
```

Call a current CLMM getter and decode its typed storage:

```rust,no_run
use tonabis_moon::contracts::clmm::range::{
    MoonClmmRangeContract, storage::MoonClmmRangeStorage,
};
use ton::{contracts::{ContractClient, TonContract}, errors::TonResult,
          ton_core::{traits::tlb::TLB, types::TonAddress}};

async fn range_state(client: &ContractClient, address: &TonAddress)
    -> TonResult<MoonClmmRangeStorage>
{
    let range = MoonClmmRangeContract::new(client, address, None);
    Ok(MoonClmmRangeStorage::from_cell(&range.get_range().await?)?)
}
```

`address::state_init` and `address::derive` use caller-supplied code and typed init
data. Callers own asset ordering and code/revision identity. Launch wallets must
use `address::mempad_wallet`: split depth eight and the owner's first hash byte
are part of that address derivation. These helpers do not deploy anything.

## Decoding contracts

Ordinary root, referenced-record, address and dictionary tails are accepted,
matching DeDust and native TON adapters. Serialization emits modeled fields;
unmodeled tails may be discarded. Typed body codecs validate required fields and tags; native dispatch follows the
[workspace prefix-fallback policy](../../docs/specs/workspace.md#message-dispatch). Unknown bodies, explicit remainders and bounce
diagnostics retain their graphs. Mempad dispatch stays explicit because a wrong
nested internal-transfer opcode must remain an error for a known mint message.
Metadata snakes reject branching and non-byte-aligned data. Layout discriminators
and booster mode/tail checks retain their specific structural constraints.

Ordinary address fields use `TonAddress`/`InternalAddress` or
`Option<TonAddress>`/`NullableAddress`; internal zero remains distinct from absence.
Core Maybe/reference adapters retain selector bits and reference placement.
Ordinary nullable result fields derive FromTVMStack with NullableInternalAddress:
Cell/Slice addr_none is None; TVM NULL is rejected. Explicit NULL fields keep their
protocol handling, and structured result arity stays exact;
see the [workspace address policy](../../docs/specs/workspace.md#consistent-consumer-types-and-parsing).
Variable, anycast and external addresses are rejected in ordinary fields.

Current CLMM camel-case getter selectors differ from catalog snake-case methods.
`get_pool_address(None, Some(TonAddress::ZERO), step)` packs TVM NULL then an
internal-zero Slice, not two absent addresses. Unresolved source names use descriptive Rust methods with explicit numeric selector attributes.
Parsing and getter results do not authenticate code, ownership, sender, block
coherence, gas sufficiency or settlement. CLMM messages use price-limit protection;
there is no invented minimum-output field.

## Evidence and maintenance

The [Moon specification](../../docs/specs/moon.md) and [agent guide](AGENTS.md)
describe supported behavior and maintenance rules. The checked lockfile validates
TON 0.4.12. The crate depends on TON, integer value types and core wire helpers, with no
pricing or simulation library dependency.

The offline suite retains 315 independently produced wire vectors, synthetic state
and wallet-address fixtures, and tests all 54 getters through a recording provider.
Math/replay/capture tooling and executable contract cells are excluded.
[Generation ownership](../../tools/reference/moon/README.md) limits the generator
to ordinary schema records and message dispatch. APIs, result DTOs and protocol
exceptions are handwritten. The original SDK package metadata and catalog MIT
notice are retained; they do not change the workspace's proprietary license or
authorize publication. Local fixtures do not establish live protocol qualification.

Address-taking getters use TonAddress-based signatures with explicit standard
address packing. See the crate's agent guide and [workspace address policy](../../docs/specs/workspace.md#friendly-address-interfaces) for
nullable identifiers and internal-zero handling.
Wire records follow the standard-address and explicit-absence policy above.


Optional stack address fields use `Option<Option<TonAddress>>`: outer `None` is
TVM NULL, `Some(None)` is encoded addr_none, and `Some(Some(address))` is internal.

Public types identify their contract or shared family, for example `MoonClmmRangeStorage` and `MoonCatalogClmmRangeStorage`.
Keep separate contract entities even when layouts match. See the
[workspace naming policy](../../docs/specs/workspace.md#public-type-naming) and
[type migration map](../../tools/reference/integration/type-renames.json); renamed
Rust types have no compatibility aliases.
