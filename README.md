# tonabis-rs

Rust wrappers and wire codecs for TON smart contracts, organized as one crate per
protocol under `crates/{dex}`. It supports DeDust, Moon, STONks, swap.coffee and TONCO. Pure pricing,
liquidity and simulation math belongs in a separate repository.

## Available crates

| Package | Path | Purpose |
|---|---|---|
| [`tonabis_core`](crates/core/README.md) | `crates/core` | Shared wire adapters, address helpers, empty funding bodies and checked arithmetic |
| [`tonabis_dedust`](crates/dedust/README.md) | `crates/dedust` | Revision-specific DeDust contract wrappers and codecs |
| [`tonabis_moon`](crates/moon/README.md) | `crates/moon` | Moon SDK and catalog contract wrappers and codecs |
| [`tonabis_stonks`](crates/stonks/README.md) | `crates/stonks` | STON/DeDust presale generations and tax-wallet bindings |
| [`tonabis_swap_coffee`](crates/swap_coffee/README.md) | `crates/swap_coffee` | AMM, vault, staking, CrossDex and MEV bindings |
| [`tonabis_tonco`](crates/tonco/README.md) | `crates/tonco` | V1/V2 AMM, farming and native-token bindings |
| [`tonabis`](crates/tonabis/README.md) | `crates/tonabis` | Same-name re-exports of core and the five protocols |

Core supplies `mul_div_floor`, `mul_div_ceil` and `div_ceil`, with exact wide
products and explicit division-by-zero/overflow errors. It also supplies shared
wire adapters, address-cell encoding and empty funding bodies using `ton_core`,
without providers or protocol formulas. The umbrella preserves crate names and adds no behavior.

[`tonabis_dedust`](crates/dedust/README.md) exposes 23 revision-specific wrappers
under `tonabis_dedust::contracts`, with typed persistent storage, messages,
getter results and local state-init/address constructors:

| Family | Included contracts and capabilities |
|---|---|
| Classic | Factory, pool, native vault, Jetton vault and liquidity deposit V1/V2; LP wallet; operator-managed pool; shared native/Jetton assets, installer parameters and child deployment |
| CPMM v2 | Pool, position and deposit; storage, messages, payment/notification bodies, getters and revision-specific library-reference deployment |
| Uranus | Factory V3, minters and wallets V2/V3; deployment, migration and wallet message records |
| Operator | V2 forwarding, ownership/configuration records and getters |
| X1000 custom wallets | V1/V2 storage, signed envelopes, actions, continuations, callbacks and bounce bodies; V2 getters |
| Signed references | `DedustSignatureRef<T>` preserves the original signed cell graph while typed fields remain unchanged |

Classic V1/V2, CPMM library revisions and Uranus V2/V3 are separate wire/API
identities. X1000 callbacks may describe peer-protocol payloads; this does not add
wrappers for those protocols. There is no generic DEX trait.

[`tonabis_moon`](crates/moon/README.md) adds 18 wrappers and 54 getters across
current SDK CPMM, CLMM and mempad plus separate catalog pool/range, order factory,
old/new orders and booster. Its [crate guide](crates/moon/README.md) lists the
canonical modules and message/storage APIs; see the [Moon specification](docs/specs/moon.md).

The remaining protocol libraries retain legacy and auxiliary contract generations:

| Protocol | Wrappers / callable getters | Scope and specification |
|---|---:|---|
| [STONks](crates/stonks/README.md) | 8 / 26 | Separate STON and DeDust factories/minters/presale wallets, plus the tax wallet; [specification](docs/specs/stonks.md) |
| [swap.coffee](crates/swap_coffee/README.md) | 26 / 60 | Factory/init, pools, vaults, LP/depository, staking, CrossDex and MEV including deployment/legacy forms; [specification](docs/specs/swap_coffee.md) |
| [TONCO](crates/tonco/README.md) | 17 / 66 | V1/V2 pools/routers/factories/orders/accounts/NFTs, farming and native minter/wallets; [specification](docs/specs/tonco.md) |

## Use

Rust **1.94.0**, edition 2024. The toolchain is pinned in `rust-toolchain.toml`.
The first planned GitHub release and tag are `0.1.0`; crates.io publishing stays
disabled (`publish = false`). See [GitHub consumption](docs/specs/workspace.md#github-distribution).
From a local checkout or extracted complete release source archive:

```toml
[dependencies]
tonabis_dedust = { path = "../tonabis-rs/crates/dedust" }
ton = "0.4.12"
```

For the complete surface, depend on `tonabis = { path = "../tonabis-rs/crates/tonabis", features = ["all"] }`
and import, for example, `tonabis::tonabis_dedust::contracts::classic::factory_v2`
or `tonabis::tonabis_core::arithmetic::mul_div_floor`. Direct crate dependencies
remain available. The umbrella always exposes core; select the independent `dedust`,
`moon`, `stonks`, `swap_coffee` and `tonco` features as needed. No protocol is enabled by default. See the
[workspace specification](docs/specs/workspace.md).

TON dependencies come from crates.io: `ton 0.4.12`, `ton_core 0.3.11` and
`ton_macros 0.2.6` in the checked lockfile. No sibling checkout or Cargo patch
is required. See [TON dependencies](docs/specs/workspace.md#ton-dependencies-and-stack-adapters).

The protocol crates use public `ton` crate types directly, including `ContractClient`,
TON codecs and proc macros. Declare `ton` directly in consumers that use these
APIs or derive their own codecs. The lockfile pins the tested registry resolution;
it does not promise compatibility with every older 0.4 release. There
are no private Git dependencies or math-only build mode. Protocol crates have
no feature flags; umbrella features select whole crates.

See the [crate guide](crates/dedust/README.md) for executable message/getter
examples, the complete wrapper inventory and canonical imports. The
[`contracts` example](crates/dedust/examples/contracts.rs) compiles caller-owned
client, factory, reserve, wallet-status and storage workflows without network I/O.

## Behavior and boundaries

Constructors/codecs are synchronous. Getters are async and use a caller-owned
`ContractClient`; the caller configures providers and the execution runtime.
Errors preserve TON's `TonResult`/`TonCoreError` types. This library does not sign,
broadcast, authenticate code or senders, select a contract revision, manage gas,
or establish settlement. A getter that estimates a swap is a contract call, not
a local pricing implementation.

Ordinary cell/BOC and referenced-record parsing accepts unused tails. Required
fields are validated by typed body codecs. Dispatch may retain malformed input as
Unknown on a prefix mismatch; other decoding errors still propagate. See the
[dispatch policy](docs/specs/workspace.md#message-dispatch). Unknown bodies and explicitly
modeled forwarding tails retain raw cells; unmodeled tails need not survive
serialization. Signed references preserve their original graphs until changed;
mutations require caller-owned re-signing. Required internal zero addresses,
nullable wire addresses and TVM NULL conversions are distinct contracts.
See the [DeDust](docs/specs/dedust.md), [Moon](docs/specs/moon.md),
[STONks](docs/specs/stonks.md), [Coffee](docs/specs/swap_coffee.md) and
[TONCO](docs/specs/tonco.md) specifications
before changing codecs or getters.

## Layout and maintenance

- `crates/core/src/{arithmetic,errors}.rs`: shared helpers and errors; `lib.rs` declares modules.
- `crates/tonabis/src/lib.rs`: same-name re-exports of every underlying library.
- `crates/dedust/src/contracts.rs` and `contracts/`: contract APIs and codecs.
- `crates/moon/src/contracts.rs` and `contracts/`: Moon wrappers, storage, messages and results.
- `crates/{stonks,swap_coffee,tonco}/src/contracts.rs` and `contracts/`: protocol wrappers and their owned `storage`, `msgs` and `method_results` children.
- `tools/reference/{stonks,swap_coffee,tonco}/`: source/capability/fixture inventories, manual-codec rationale and consumer examples. Coffee also has a bounded offline generator.
- `crates/moon/src/tests/`: independent wire vectors and getter/provider regressions.
- `tools/reference/moon/`: pinned normalized schema and bounded generator; [guide](tools/reference/moon/README.md).
- `crates/dedust/src/tests/`: offline contract regressions and required fixtures.
- `tools/reference/dedust/`: binding/getter-test generators, ownership metadata
  and hash-pinned ABI JSON inputs. [Tooling guide](tools/reference/dedust/README.md).
- `docs/specs/dedust.md`: supported behavior and invariants.
- [AGENTS.md](AGENTS.md) and [crate guidance](crates/dedust/AGENTS.md): maintainer
  instructions for Codex and Claude; `CLAUDE.md` imports the root instructions.

```sh
cargo test --workspace --locked
cargo test -p tonabis --doc --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --locked
cargo check --workspace --examples --locked
cargo fmt --all -- --check
python3 -m unittest discover -s tools/tests -v
```

Tests are offline after dependency acquisition. Schema generation needs only
Python 3.9+ and the included ABI inputs. Tests and generators do not require live chain access or executable contract cells.

## Distribution

The workspace declares a proprietary license and sets `publish = false`. The MIT notice for TON Core's ABI catalog is included at
[`crates/dedust/LICENSE.ton-abi-catalog`](crates/dedust/LICENSE.ton-abi-catalog).
That notice covers the catalog-derived material, not an open-source license for
the entire workspace. Distribution targets GitHub releases of the complete
repository, beginning with version/tag `0.1.0`. Use a tagged Git dependency or
paths into the extracted source archive; internal workspace dependencies resolve
together without crates.io publication or patches. Registry publication and
broader licensing remain separate future decisions. TON dependencies remain
registry releases.
No executable capture corpus is included.

Public contract types use protocol, contract/family and generation names so they
can be distinguished in IDE search. See the [naming policy](docs/specs/workspace.md#public-type-naming)
and [source migration map](tools/reference/integration/type-renames.json).
