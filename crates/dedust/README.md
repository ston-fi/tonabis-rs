# tonabis_dedust

Revision-specific DeDust smart contract wrappers and TON wire codecs. This crate
contains contracts only: typed storage, messages, getter results and state-init
helpers. It has no feature flags, local pricing/simulation math, signing or
broadcasting. Rust 1.94.0 / edition 2024; currently `publish = false`.

For a local consumer, add `tonabis_dedust = { path = "../tonabis-rs/crates/dedust" }`.
Provider, codec and error types come directly from the public `ton` crate.
Add `ton = "0.4.12"` to consumers that use those types or derive their own codecs.
The workspace locks TON 0.4.12. No private Git dependency is required.

Classic V2 native and Jetton vaults share pool identity and swap continuation
records from `contracts::classic::vault_native_v2::storage`: `DedustPoolType`,
`DedustVaultNativeV2PoolParams`, `DedustSwapKind`, `DedustSwapStepParams`, `DedustSwapStep`,
`DedustSwapParams` and their tag records. Import that owner directly for either
vault; the former Jetton-local copies are removed without aliases. Wire formats
and independent fixtures are unchanged.

## Contract inventory

All paths below are relative to `tonabis_dedust::contracts`. Each contract has
its persistent records in a `storage` child, messages in `msgs`, and structured
getter results in `method_results` where applicable. Scalar getters return their
native value directly. There are no flattened aliases.

| Module | Wrapper |
|---|---|
| `classic::factory_v1` | `DedustFactoryV1Contract` |
| `classic::factory_v2` | `DedustFactoryV2Contract` |
| `classic::pool_v1` | `DedustPoolV1Contract` |
| `classic::pool_v2` | `DedustPoolV2Contract` |
| `classic::vault_native_v1` | `DedustVaultNativeV1Contract` |
| `classic::vault_native_v2` | `DedustVaultNativeV2Contract` |
| `classic::vault_jetton_v1` | `DedustVaultJettonV1Contract` |
| `classic::vault_jetton_v2` | `DedustVaultJettonV2Contract` |
| `classic::liquidity_deposit_v1` | `DedustLiquidityDepositV1Contract` |
| `classic::liquidity_deposit_v2` | `DedustLiquidityDepositV2Contract` |
| `classic::lp_wallet` | `DedustClassicLpWalletContract` |
| `classic::operator_pool` | `DedustClassicOperatorPoolContract` |
| `cpmm_v2::pool` | `DedustV2CpmmContract` |
| `cpmm_v2::position` | `DedustV2CpmmPositionContract` |
| `cpmm_v2::deposit` | `DedustV2CpmmDepositContract` |
| `uranus::factory_v3` | `DedustUranusFactoryV3Contract` |
| `uranus::meme_v2` | `DedustUranusMemeV2Contract` |
| `uranus::meme_v3` | `DedustUranusMemeV3Contract` |
| `uranus::wallet_v2` | `DedustUranusMemeWalletV2Contract` |
| `uranus::wallet_v3` | `DedustUranusMemeWalletV3Contract` |
| `operator::v2` | `DedustOperatorV2Contract` |
| `custom_wallet::v1` | `DedustX1000WalletV1Contract` |
| `custom_wallet::v2` | `DedustX1000WalletV2Contract` |

Classic includes factory code/address queries, pool reserves/assets/fees and
swap estimates, vault queries, liquidity deposits, LP wallet data and managed-pool
activation. `classic::asset` owns shared native/Jetton assets and stack traits;
`classic::common` owns installer parameters, proofs and shared code results.
Factory wrapper methods construct child state-init from caller-supplied code.

CPMM includes position/deposit data, pool queries, nested payments and
notifications; `cpmm_v2::deployment` owns explicit pool-library revisions and
pool/position/deposit state-init. Uranus includes mint/burn/migration messages,
minter/wallet queries and `uranus::deployment` helpers. Factory V3 and X1000 V1
have no invented getters. Operator V2 includes forwarding and configuration.
X1000 V1/V2 retain distinct signed actions, continuations, context, callback and
bounce formats. `signature_ref::DedustSignatureRef<T>` preserves signed graphs.

## Encode and decode a message

Import the TON `TLB` trait for `to_cell`, `from_cell` and `from_boc`.

```rust
use tonabis_dedust::contracts::operator::v2::msgs::DedustOperatorV2ForwardMsg;
use ton::ton_core::{
    cell::TonCell, traits::tlb::TLB,
    types::{TonAddress, tlb_core::TLBRef},
};

let command = DedustOperatorV2ForwardMsg {
    query_id: 7,
    destination: TonAddress::ZERO,
    body: TLBRef::new(TonCell::empty().clone()),
};
let cell = command.to_cell()?;
let decoded = DedustOperatorV2ForwardMsg::from_cell(&cell)?;
assert_eq!(decoded, command);
# Ok::<(), ton::ton_core::errors::TonCoreError>(())
```

## Use a caller-owned provider

```rust,no_run
use tonabis_dedust::contracts::classic::{
    asset::DedustClassicAsset,
    factory_v2::DedustFactoryV2Contract,
};
use ton::{
    contracts::{ContractClient, TonContract}, errors::TonResult,
    ton_core::types::TonAddress,
};

async fn pool_address(
    client: &ContractClient,
    factory_address: &TonAddress,
    asset0: DedustClassicAsset,
    asset1: DedustClassicAsset,
) -> TonResult<TonAddress> {
    let factory = DedustFactoryV2Contract::new(client, factory_address, None);
    factory.get_pool_address(true, asset0, asset1).await
}
```

The constructor is synchronous; getters are async. `is_stable` packs integer
0/1. Callers own client/runtime configuration, coherent snapshots, revision/code
identity, authority checks, gas and settlement. The crate spawns no background
workers and does not sign or broadcast. The `contracts` example also compiles
storage parsing, reserves, versioned code and X1000 status queries.

## Wire and getter guarantees

- Ordinary records tolerate unused root/reference/address/dictionary tails;
  re-encoding need only retain modeled fields. Unknown bodies and modeled raw
  tails preserve their cell contents. Concrete body codecs reject malformed data; dispatch fallback is described below.
- Required standard addresses use `TonAddress` plus `InternalAddress`; nullable
  wire addresses use `Option<TonAddress>` plus `NullableAddress`. Internal `0:0`
  remains distinct from no address. Nullable structured fields accept encoded addr_none and reject TVM NULL; scalar
  `TonAddress` getters retain TON's NULL/addr_none-to-zero normalization.
- Scalars use native stack consumption; complete structured result DTOs enforce
  exact arity. Cells accept Cell/Slice, and optional cells distinguish NULL from
  an empty cell. Dictionaries use native TON adapters.
- Stored nonnegative getter amounts use `Coins` (checked u128); TL-B storage
  retains its exact wire widths. Unsigned growth/checkpoints/hashes use `U256`;
  genuinely signed results/arguments use `I512`. Q248 storage widths do not
  impose a separate 248-bit getter limit. Narrow counters use u32/u8.
- `DedustSignatureRef::new(value)` constructs a signed reference. A parsed reference
  retains the original graph while fields are unchanged; changing fields emits
  the updated value and requires the caller to re-sign.
- Typed TON errors propagate through `TonResult`/`TonCoreError`. Parsing alone
  proves neither authorization nor successful execution.

The crate's `AGENTS.md`, the repository specification and tooling guide describe
ownership, extension rules, provenance and exact validation commands. Local numerical
algorithms and conversions into math state remain outside this crate. The proprietary package policy
and TON ABI catalog MIT notice remain separate.


## Address representation

Ordinary address fields use `TonAddress`/`InternalAddress` or
`Option<TonAddress>`/`NullableAddress`; internal zero remains distinct from absence.
Core Maybe/reference adapters retain selector bits and reference placement.
Ordinary nullable result fields derive FromTVMStack with NullableInternalAddress:
Cell/Slice addr_none is None; TVM NULL is rejected. Explicit NULL fields keep their
protocol handling, and structured result arity stays exact;
see the [workspace address policy](../../docs/specs/workspace.md#consistent-consumer-types-and-parsing).
Variable, anycast and external addresses are rejected in ordinary fields.

Raw address types remain only in X1000 historical message/context records. Their
signed, opaque or bare-addr_std layouts are retained; these are not patterns for
ordinary new contracts. Address dictionaries use a required-internal value
adapter, including when the hash is zero.

Public types identify their contract or shared family, for example `DedustLiquidityDepositV1PoolParams` and `DedustLiquidityDepositV2PoolParams`.
Keep separate contract entities even when layouts match. See the
[workspace naming policy](../../docs/specs/workspace.md#public-type-naming) and
[type migration map](../../tools/reference/integration/type-renames.json); renamed
Rust types have no compatibility aliases.

Message dispatch is best-effort classification. Native derives may retain a known
opcode with a malformed nested prefix as `Unknown(TonCell)`; other errors still
propagate. Decode the concrete message type when validation of that type is needed.
Explicit classic-pool and X1000 V1 dispatch keep their existing strict errors.
See the [workspace dispatch policy](../../docs/specs/workspace.md#message-dispatch).

The reference-only `DedustVaultNativeV2InlineDepositLiquidityCandidate` is excluded
from the Rust API. Use `DedustVaultNativeV2DepositLiquidityMsg` with referenced
`DedustDepositLiquidityParams`; the pinned corpus and historical fixture remain
available as source evidence, not another supported encoding.
