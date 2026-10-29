# tonabis_stonks

Generation-specific STONks smart contract wrappers and TON wire codecs. Rust
1.94.0, edition 2024; unpublished, with no feature flags or math dependency.
Uses the public `ton` types directly. The workspace lockfile validates TON 0.4.12,
`ton_core` 0.3.11 and `ton_macros` 0.2.6; older 0.4 releases are not all qualified.

| Module under `contracts` | Wrapper | Capability |
|---|---|---|
| `ston_factory` | `StonksStonFactoryContract` | STON factory storage, four-field getter, legacy creation/control messages |
| `dedust_factory` | `StonksDedustFactoryContract` | DeDust factory storage/getter, current v12 creation body |
| `ston_minter` | `StonksStonMinterContract` | STON v8/v10/v11; eight-field bonding, standard Jetton getters and selectors 82057, 68631, 95602 |
| `dedust_minter_v7` | `StonksDedustMinterV7Contract` | DeDust v7; eight-field bonding, standard getters, selector 82057, migration continuations |
| `dedust_minter_v12` | `StonksDedustMinterV12Contract` | DeDust v12; twelve-field bonding, standard getters, thirteen-field 82057 and fee selector 71772 |
| `ston_wallet` | `StonksStonWalletContract` | STON presale storage/context, standard getter and selector 74038 |
| `dedust_wallet` | `StonksDedustWalletContract` | DeDust v7/v12 presale storage/context, standard getter and selector 84724 |
| `custom_wallet` | `StonksCustomWalletContract` | Separate tax wallet; accounting, two-address context, transfer/bounce bodies and tax/wallet getters |

Each primary module owns its wrapper and inherent methods. Persistent records
live in `storage`, structured getter records in `method_results`, messages and
dispatch in `msgs`. There are 26 callable getters across eight wrappers.
`generation::StonksGeneration` names the five observed target/version identities; it
does not detect or authenticate a contract.

Shared records have one owner. All minters use
`ston_minter::storage::StonksVirtualMinterStorage`, including its `initial_state` API.
Eight-field bonding and standard Jetton results live in
`ston_minter::method_results`. The standard wallet result lives in
`dedust_wallet::method_results`. DeDust continuation messages and launch state
live in `dedust_minter_v7::{msgs,storage}` and are imported directly by v12.
Factory initialization/dev-buy records use `ston_minter::msgs`.
Factories share legacy messages through `ston_factory::msgs`; v12 creation lives
in `dedust_factory::msgs`. Current forwarding records live in
`ston_minter::msgs` and are shared directly by the relevant wallets/minters.

## Consumer path

Declare `tonabis_stonks = { path = "../tonabis-rs/crates/stonks" }` and `ton = "0.4.12"`
in a local consumer. Constructors are synchronous and do no I/O; async getters
borrow a caller-owned client and preserve typed TON errors.

```rust
use ton::{contracts::{ContractClient, TonContract}, errors::TonResult,
    ton_core::types::TonAddress};
use tonabis_stonks::contracts::dedust_minter_v12::{
    StonksDedustMinterV12Contract, method_results::StonksDedustMinterV12GetBondingDataResult};

async fn read(client: &ContractClient, address: &TonAddress)
    -> TonResult<(u32, StonksDedustMinterV12GetBondingDataResult)>
{
    let minter = StonksDedustMinterV12Contract::new(client, address, None);
    Ok((minter.get_version().await?, minter.get_bonding_data().await?))
}
```

```rust
use ton::ton_core::{traits::tlb::TLB, types::tlb_core::TLBCoins};
use tonabis_stonks::contracts::ston_minter::msgs::StonksStonMinterBuyFromPresaleMsg;
let buy = StonksStonMinterBuyFromPresaleMsg {
    query_id: 42, ton_amount: TLBCoins::new(1_000_000_000),
    min_jettons_out: TLBCoins::new(1_000),
};
let cell = buy.to_cell()?;
assert_eq!(StonksStonMinterBuyFromPresaleMsg::from_cell(&cell)?, buy);
# Ok::<(), ton::ton_core::errors::TonCoreError>(())
```

`examples/contracts.rs` also demonstrates local storage/StateInit/address construction.
Wallet `initial_state` methods require caller-provided code, owner and master.
The shared minter initializer additionally takes factory, metadata and wallet code.
No signing, broadcasting, provider selection, retries, authentication, runtime,
pricing, simulation or settlement policy is owned here. Cancellation follows the
caller future/client. Query IDs are encoded fields, with no deduplication service.

## Wire and result rules

Ordinary fixed records and referenced records tolerate unused bits/references;
serialization emits modeled fields. A shorter generation schema may parse a longer
schema's prefix: choose the target/version explicitly. Unknown dispatch bodies
retain their raw cells. Typed codecs enforce protocol constraints; native dispatch follows the
[workspace prefix-fallback policy](../../docs/specs/workspace.md#message-dispatch).
Current untagged forwarding suffixes preserve omitted versus present Either payloads
and require their complete modeled shape. STON v10/v11 sell notifications require
a standard recipient, unlike the older/DeDust form. Use `StonksStonMinterCurrentMessage` for current
STON dispatch and the ordinary `ston_minter::msgs::StonksStonMinterLegacyMessage` for the older form.

Presale transfers require a true payload selector, a payload reference, and a
separate context reference. STON and DeDust contexts have different layouts.
Tax-wallet accounting and bounce rules remain distinct. Decoding is structural,
without proving authority, contract identity or execution success.

Ordinary address fields use `TonAddress`/`InternalAddress` or
`Option<TonAddress>`/`NullableAddress`; internal zero remains distinct from absence.
Core Maybe/reference adapters retain selector bits and reference placement.
Ordinary nullable result fields derive FromTVMStack with NullableInternalAddress:
Cell/Slice addr_none is None; TVM NULL is rejected. Explicit NULL fields keep their
protocol handling, and structured result arity stays exact;
see the [workspace address policy](../../docs/specs/workspace.md#consistent-consumer-types-and-parsing).
Variable, anycast and external addresses are rejected in ordinary fields.

Structured results enforce exact stack arity. `get_version` returns `u32` and
`get_wallet_address` takes and returns `Option<TonAddress>`, keeping absence
separate from a present internal-zero address.
The v12 fee order is CPMM base fee, total presale tax, destination creator share,
creator recipient; the independently recorded values are 100, 140, 5000, address.

## Evidence and maintenance

Tests use crate-local independent schema/catalog/synthetic fixtures; no external
source checkout, network or executable sidecar is needed. Initialization vectors
use empty code and independently assembled TL-B representation hashes. These checks
prove bindings and package usability, not live deployment or captured-code replay.
See [AGENTS.md](AGENTS.md), the repository `docs/specs/stonks.md` and
`tools/reference/stonks/bindings.json` for binding ownership, source hashes,
manual exceptions, full getter coverage and fixture/test dispositions.

Scalars return native values, and ordinary stack conversion follows TON policy.
Bindings use handwritten declarations and native derives; no generator is required.

The workspace proprietary policy and `publish = false` remain in force. The
catalog MIT notice in `LICENSE.ton-abi-catalog` applies only to catalog-derived
interfaces; it grants no rights to captured executable code. See
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

Standard burn-notification and excess acknowledgements have one canonical owner in
`ston_minter::msgs`. Tax-wallet dispatch imports those records directly; the
shared burn record exposes `owner` and `response_address`.

Address-taking getters use TonAddress-based signatures with explicit standard
address packing. See the crate's agent guide and [workspace address policy](../../docs/specs/workspace.md#friendly-address-interfaces) for
nullable identifiers and internal-zero handling.
Wire records follow the standard-address and explicit-absence policy above.

Public types identify their contract or shared family, for example `StonksStonWalletContext` and `StonksDedustWalletContext`.
Keep separate contract entities even when layouts match. See the
[workspace naming policy](../../docs/specs/workspace.md#public-type-naming) and
[type migration map](../../tools/reference/integration/type-renames.json); renamed
Rust types have no compatibility aliases.
