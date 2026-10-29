# tonabis_tonco

TONCO V1 and V2 (SDK v1.6 / executable version 16000) smart contract wrappers,
TL-B storage/messages, asynchronous getters and local StateInit construction.
Rust 1.94.0, edition 2024; unpublished (`publish = false`), no feature flags.
The validated lockfile uses `ton` 0.4.12, `ton_core` 0.3.11 and `ton_macros` 0.2.6.

| Canonical module under `contracts` | Wrapper | Getters |
|---|---|---:|
| `v1::pool` | `ToncoPoolV1Contract` | 15 |
| `v1::router` | `ToncoRouterV1Contract` | 5 |
| `v1::factory` | `ToncoFactoryV1Contract` | 2 |
| `v1::order` | `ToncoOrderV1Contract` | 0 |
| `v1::account` | `ToncoAccountV1Contract` | 1 |
| `v1::position_nft` | `ToncoPositionNftV1Contract` | 4 |
| `v2::pool` | `ToncoPoolV2Contract` | 16 |
| `v2::router` | `ToncoRouterV2Contract` | 6 |
| `v2::factory` | `ToncoFactoryV2Contract` | 3 |
| `v2::order` | `ToncoOrderV2Contract` | 0 |
| `v2::account` | `ToncoAccountV2Contract` | 1 |
| `v2::position_nft` | `ToncoPositionNftV2Contract` | 4 |
| `native::minter` | `ToncoNativeMinterContract` | 3 |
| `native::wallet` | `ToncoNativeWalletV1Contract`, `ToncoNativeWalletV2Contract` | 1 each |
| `farming::distributor` | `ToncoDistributorContract` | 3 |
| `farming::user_distributor` | `ToncoUserDistributorContract` | 1 |

The 17 wrappers retain all 66 callable source getters. Order children have storage,
messages and caller-code StateInit construction without invented getters.
Native minter code is shared by the two SDK-selected addresses; wallet generations
remain separate. Wallet V2 additionally recognizes the special jetton-transfer body.

## Consumer use

Depend on `tonabis_tonco` by local path and declare `ton = "0.4.12"` directly when
using the public TON types. Public types are the registry TON types, with no
private dependency or re-export layer.

```rust
use ton::ton_core::{traits::tlb::TLB, cell::TonCell};
use tonabis_tonco::contracts::v2::pool::msgs::ToncoPoolV2ResetGasMsg;

let body = ToncoPoolV2ResetGasMsg { query_id: 7 }.to_cell()?;
let parsed = ToncoPoolV2ResetGasMsg::from_cell(&body)?;
assert_eq!(parsed.query_id, 7);
# Ok::<(), ton::ton_core::errors::TonCoreError>(())
```

Constructing a wrapper sends no request. The caller owns providers, transaction
context and executor; dropping a getter future follows the caller/client's
cancellation behavior. The library adds no retries, background tasks or runtime.

```rust,no_run
use ton::{contracts::{ContractClient, TonContract}, errors::TonResult,
    ton_core::types::TonAddress};
use tonabis_tonco::contracts::v2::pool::{ToncoPoolV2Contract,
    method_results::ToncoPoolV2GetPoolStateAndConfigurationResult};

async fn inspect(client: &ContractClient, address: &TonAddress)
    -> TonResult<ToncoPoolV2GetPoolStateAndConfigurationResult>
{
    let pool = ToncoPoolV2Contract::new(client, address, None);
    let maximum: fastnum::I512 = pool.get_max_liquidity_per_tick().await?;
    let state = pool.get_pool_state_and_configuration().await?;
    // These separate calls do not establish a coherent block snapshot.
    let _ = maximum;
    Ok(state)
}
```

Each primary contract owns `storage`, `msgs`, and structured `method_results`
where applicable. For example, `v2::pool::storage::ToncoPoolV2Storage` exposes
`state_init(code)` and `derive_address(code, workchain)`; these only encode/hash
caller-supplied data and code. Order storage exposes `state_init(code)`.
Use public-field struct literals. `native::minter::storage::ToncoNativeMinterStorage`
also derives wallet addresses with caller-selected standard owner/minter addresses.

Shared V2 reforge and payout records live in `v2::msgs`;
farming messages in `farming::msgs`. TEP-89 responses and their referenced owner
have one owner in `native::minter::msgs`, imported directly by both order generations.
Both wallet wrappers use `native::wallet::storage::ToncoNativeWalletStorage`.
Both pools return `v1::pool::method_results::ToncoPoolTickPage`, which validates the nested
three/five-field tick tuples. `addresses::ordered_wallets` orders by descending
hash of the complete address cell and rejects equal wallets.

## Wire and error contracts

V1 and V2 layouts are distinct even when their opcodes coincide. V1 ticks live in
a referenced signed-key dictionary; V2 ticks are inline HashmapE with signed
int224 growth. V2 mint/burn/funding uses account orders and reforge callbacks;
stale direct SDK mint/burn selectors are not added. Router CreatePool roles and
pool Init roles differ. Factory whitelist keys are the 255 bits following a
standard address's three-bit prefix, and permission records remain referenced.

Ordinary root, reference, dictionary and address tails follow native TON parsing.
Unmodeled tails may disappear on re-encoding. Typed body codecs reject malformed data; native dispatch follows the
[workspace prefix-fallback policy](../../docs/specs/workspace.md#message-dispatch);
unknown message variants retain their complete raw cell graphs. Explicit untagged
sequence, indexer, claim-index, forwarding, order and timelock suffixes retain
their protocol constraints; they are not generic tagged `Option` fields.

Ordinary address fields use `TonAddress`/`InternalAddress` or
`Option<TonAddress>`/`NullableAddress`; internal zero remains distinct from absence.
Core Maybe/reference adapters retain selector bits and reference placement.
Ordinary nullable result fields derive FromTVMStack with NullableInternalAddress:
Cell/Slice addr_none is None; TVM NULL is rejected. Explicit NULL fields keep their
protocol handling, and structured result arity stays exact;
see the [workspace address policy](../../docs/specs/workspace.md#consistent-consumer-types-and-parsing).
Variable, anycast and external addresses are rejected in ordinary fields.

Codec/construction errors use `TonCoreResult`; getters use `TonResult` and retain
provider failures. Parsing does not authenticate code, permissions, sender,
proofs, block coherence or settlement. The crate does not sign, broadcast, price,
simulate liquidity, route trades, own providers or infer contract generations.
On-chain estimate getters remain available as contract calls.

## Maintenance and evidence

`examples/contracts.rs` compiles codec, caller-client, result and StateInit usage.
`AGENTS.md` gives extension rules and exact checks. The workspace specification is
`docs/specs/tonco.md`; `tools/reference/tonco/bindings.json` maps every public source
type, getter, constructor/helper, manual exception and fixture disposition.
No TONCO generator is needed: ordinary codecs derive TLB/FromTVMStack; the locked
TON macros provide no ToTVMStack derive.

Offline tests retain unchanged independent SDK/protocol encodings, structural
getter rows, synthetic StateInit/address fixtures and wallet ordering. Numerical
assertions and executable sidecars are excluded. Passing these tests is not live
parity, economic qualification or publication approval. The proprietary workspace
policy remains; `LICENSE-TON-ABIS`, `LICENSE-TONCO-SDK` and `THIRD_PARTY_NOTICES.md`
preserve applicable upstream notices without granting a workspace-wide license.

Address-taking getters use TonAddress-based signatures with explicit standard
address packing. See the crate's agent guide and [workspace address policy](../../docs/specs/workspace.md#friendly-address-interfaces) for
nullable identifiers and internal-zero handling.
Wire records follow the standard-address and explicit-absence policy above.


Optional stack address fields use `Option<Option<TonAddress>>`: outer `None` is
TVM NULL, `Some(None)` is encoded addr_none, and `Some(Some(address))` is internal.

Public types identify their contract or shared family, for example `ToncoRouterV1Timelocks` and `ToncoRouterV2Timelock`.
Keep separate contract entities even when layouts match. See the
[workspace naming policy](../../docs/specs/workspace.md#public-type-naming) and
[type migration map](../../tools/reference/integration/type-renames.json); renamed
Rust types have no compatibility aliases.
