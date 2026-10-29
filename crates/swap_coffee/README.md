# tonabis_swap_coffee

Revision-specific swap.coffee smart contract wrappers and TON wire codecs. This unpublished
package uses Rust 1.94.0, edition 2024, and the workspace's locked TON dependency
graph (`ton` 0.4.12, `ton_core` 0.3.11, `ton_macros` 0.2.6). It has no feature flags.

Shared public values have one owner: common pool assets/proofs under
`contracts::pool_constant_product::storage`, common pool message values under
`pool_constant_product::msgs`, common vault message values under
`vault_native::msgs`, and staking asset/position values under
`staking_item::storage`. Import those modules directly when using the stable pool,
other vaults or staking master. Runtime/deployment storage and transport-specific
message bodies/dispatch remain distinct. The binding manifest lists exact paths.

```rust
use ton::contracts::{ContractClient, TonContract};
use ton::errors::TonResult;
use ton::ton_core::{traits::tlb::TLB, types::TonAddress};
use tonabis_swap_coffee::contracts::{
    factory::{CoffeeFactoryContract, method_results::{CoffeeFactoryGetCodeResult, CoffeeFactoryAddressWithHashResult},
              msgs::CoffeeFactoryAsset},
    lp_wallet::msgs::CoffeeLpWalletExcessesMsg,
};

async fn read_code(client: &ContractClient, address: &TonAddress) -> TonResult<CoffeeFactoryGetCodeResult> {
    CoffeeFactoryContract::new(client, address, None).get_code().await
}
async fn vault_address(client: &ContractClient, factory: &TonAddress,
                       asset: &CoffeeFactoryAsset) -> TonResult<CoffeeFactoryAddressWithHashResult> {
    CoffeeFactoryContract::new(client, factory, None).get_vault_address(asset).await
}
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let message = CoffeeLpWalletExcessesMsg { query_id: 7 };
assert_eq!(CoffeeLpWalletExcessesMsg::from_cell(&message.to_cell()?)?, message);
# Ok(()) }
```

Consumers depend directly on `ton` and `tonabis_swap_coffee`; exposed TON types
have direct dependency identity. Constructors are synchronous and getters borrow
the caller's `ContractClient` and optional transaction context. Each getter makes
one request, propagates typed errors and adds no retries. Cancellation belongs to
the caller/client. There is no library-owned runtime, signing, broadcasting, code
selection, authority checking, routing or local price/liquidity math. On-chain
estimate getters remain supported.

The contract inventory is:

| Modules | Wrappers and capabilities |
| --- | --- |
| `factory`, `init`, `pool_creator` | Factory getters/configuration, installation, creation and caller-code child identities |
| `pool_constant_product`, `pool_curve_fi_stable` | Distinct pool storage/messages, Jetton metadata/wallet getters, on-chain estimates |
| `vault_native`, `vault_jetton`, `vault_extra` | Asset/funding/payout flows, typed sender proofs, uint32 extra IDs; separate active/inactive Jetton forms |
| `liquidity_depository`, `lp_wallet` | Child identities, deposit/withdrawal/LP wire flows, wallet getter and transport-selected bounce role |
| `staking_master`, `staking_item`, `staking_vault` | Typed period/reward/configuration dictionaries, NFT/collection getters, initialization and emission roles |
| `staking_master::single_item` | Explicit two-reference master generation with separate runtime/deployment wrappers |
| `cross_dex`, `cross_dex_legacy`, `cross_dex_bidask` | Revision-specific Coffee forwarding/rescue wire interfaces, with distinct deployment data |
| `mev_protector`, `mev_protector_legacy` | Signed batch codecs, replay dictionaries and public-key/query getters |

There are 26 wrappers and 60 callable getters after expanding runtime/deployment
methods. Primary modules own wrapper APIs; `storage`, `msgs` and `method_results`
children own records. Shared staking values live in `contracts::staking` and
shared sender proofs in `contracts::vault_proof`. Import these owners directly.
The umbrella exposes this crate through its default-off `swap_coffee` feature.

Ordinary root, reference and dictionary tails are tolerated. Structured getter
results enforce exact arity; scalars return native values and consume one item.
Both pool `get_wallet_address` methods take `&TonAddress` and return
`TonAddress`. Their handwritten argument packing uses InternalAddress, preserving
internal zero and rejecting workchains outside int8. Results follow native
TonAddress conversion, including NULL/addr_none-to-zero normalization. Factory owner inputs follow the same standard-address policy; structured results
use standard addresses with explicit nullable fields where required.
Native Cell results accept Cell/Slice. Required Slice arguments remain explicitly
packed. Nullable wire addresses use addr_none without a Maybe bit, preserving
literal internal zero. Ordinary TLB fields use `TonAddress` or `Option<TonAddress>`
with native address adapters. Opaque signed payloads keep their raw graphs.
Ordinary nullable getter fields use `Option<TonAddress>`: encoded addr_none is
absence, internal zero remains present, and TVM NULL is rejected. Scalar CrossDex
`get_admin` retains native MsgAddress conversion and also accepts TVM NULL as absence. Variable/anycast forms fail.
Parsing does not authenticate addresses.

Typed bodies validate their fields; dispatch follows the
[workspace prefix-fallback policy](../../docs/specs/workspace.md#message-dispatch).
Unknown bodies preserve raw cells. Empty
funding exists only in the documented incoming roles. Internal, external and
bounce roles remain distinct even with colliding selectors. Select runtime and
deployment wrappers explicitly; no runtime layout detection is performed. Phase
flags, zero staking initialization constraints, bounded untagged suffixes and
TEP-89 omitted/not-included/included-owner forms retain their protocol meaning.

MEV keys are signed16. Decoded batches retain the original unsigned cell graph
while logical fields are unchanged, including signature-only changes. Changed
fields encode canonically. The caller supplies signatures; this crate neither
verifies nor creates them. StateInit/address helpers similarly use caller-supplied
code and do not prove factory admission. Asset sorting uses tagged unsigned order,
returns `TonCoreResult<([CoffeeFactoryAsset; 2], bool)>` and rejects identical assets.

Intentional source API changes include module ownership, explicit Coffee wrapper
and initial-storage names, `Msg` body suffixes, scalar returns, `Result` DTO names,
removal of `AssetPairError` in favor of typed TON errors, and removal of math bridges.
The shared staking asset record has one owner under `staking_item::storage`.
All vault identity records (`CoffeeVaultInitData`, `CoffeeAsset` and its Native/Jetton/Extra
variant records) have one owner under `vault_native::storage`. The extra-currency
and Jetton storage/messages import those exact types directly. Their former
duplicate public paths are removed, while wrappers and runtime layouts remain distinct.

See `examples/contracts.rs`, [maintainer guidance](AGENTS.md) and
[third-party notices](THIRD_PARTY_NOTICES.md). Repository maintainers also use
`docs/specs/swap_coffee.md` and `tools/reference/swap_coffee/bindings.json` for
source mappings, manual exceptions, fixture provenance and test dispositions.
Fixtures are crate-local, code-free and independent of the generator. Local
checks and dirty-package validation do not prove current chain execution,
cryptographic acceptance, universal protocol equivalence or publication readiness.

Each contract keeps types and implementations together in flat `storage.rs`,
`msgs.rs` and `method_results.rs` files. Factory address getters accept
`factory::msgs::CoffeeFactoryAsset`; pool swap estimates accept
`pool_constant_product::storage::CoffeePoolAsset`. Both encode tagged Native,
Jetton and Extra assets as a TVM Slice through their stack trait implementations.
Message dispatch ends in Unknown; empty funding uses `Empty(tonabis_core::msgs::EmptyBodyMsg {})`.

Address-taking getters use TonAddress-based signatures with explicit standard
address packing. See the crate's agent guide and [workspace address policy](../../docs/specs/workspace.md#friendly-address-interfaces) for
nullable identifiers and internal-zero handling.
Wire records follow the standard-address and explicit-absence policy above.

The TEP-74 Jetton notification envelope shared by CrossDex (all generations),
MEV (both generations) and staking vault is
`contracts::cross_dex::msgs::CoffeeCrossDexJettonTransferNotificationMsg`.
Import this canonical record directly; the former per-contract copies are removed
without aliases. Each contract keeps its own dispatch enum and message roles.
The opcode, fields and independent family fixtures are unchanged.


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
