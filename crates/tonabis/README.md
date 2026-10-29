# tonabis

A crate of crates: same-name re-exports of the workspace libraries,
with their original package names preserved. Rust 1.94.0 / edition 2024;
unpublished, with no additional runtime behavior. `tonabis_core` is always
available, including with `default-features = false`. Default features are empty:
enable any of `dedust`, `moon`, `stonks`, `swap_coffee` and `tonco` to select
protocol crates, or `all` to enable all five.

| Public path | Underlying crate |
|---|---|
| `tonabis::tonabis_core` | Shared wire adapters, address helpers, funding bodies and checked arithmetic |
| `tonabis::tonabis_dedust` | DeDust contracts, storage, messages, getters and state-init helpers |
| `tonabis::tonabis_moon` | Moon SDK and catalog contracts, codecs and getters |
| `tonabis::tonabis_stonks` | STONks STON/DeDust presale generations and tax wallet |
| `tonabis::tonabis_swap_coffee` | Coffee AMM, vault, staking, CrossDex and MEV bindings |
| `tonabis::tonabis_tonco` | TONCO V1/V2, farming and native-token bindings |

```toml
[dependencies]
tonabis = { path = "../tonabis-rs/crates/tonabis", features = ["all"] }
```

For Moon plus the always-available core, use `features = ["moon"]`. Feature names
match crate folder names; public paths retain the full crate names shown above.
Disabled protocol libraries are not dependencies of `tonabis`.

```rust
# #[cfg(feature = "dedust")]
# {
use tonabis::{
    tonabis_core::{arithmetic::mul_div_floor, errors::ArithmeticError},
    tonabis_dedust::contracts::classic::asset::{DedustClassicAsset, DedustClassicNativeAsset},
};

let share = mul_div_floor(1_000u64.into(), 3u64.into(), 10u64.into())?;
assert_eq!(share, 300u64.into());
let asset: DedustClassicAsset = DedustClassicNativeAsset {}.into();
assert!(matches!(asset, DedustClassicAsset::Native(_)));
# Ok::<(), ArithmeticError>(())?;
# }
# Ok::<(), Box<dyn std::error::Error>>(())
```

For Moon, the unchanged full crate name also exposes contract messages:

```rust
# #[cfg(feature = "moon")]
# {
use tonabis::tonabis_moon::contracts::mempad::msgs::MoonMempadBuyMsg;
let request = MoonMempadBuyMsg { query_id: 42, min_out: 1_000u64.into() };
assert_eq!(request.query_id, 42);
# }
```

Types and errors are the original underlying types. There are no shortened
protocol or `core` aliases, flattened item exports, wrappers, adapters or shared error
conversion. Consumers using TON clients/codecs or their derives declare `ton`
directly. Consumers that need only one library can depend on it directly.
Adding a protocol library to the workspace requires adding its optional dependency,
folder-name feature and gated same-name re-export, including it in `all`, and updating
this inventory.

`cargo test -p tonabis --doc --all-features --locked` checks the documented consumer paths.
Also check default and individual features to preserve independent selection.
All 32 subsets of the five protocol features are supported, including the empty
selection with or without `default-features = false`; `all` selects their union.
`cargo test --workspace --locked` validates underlying behavior. The inherited
proprietary license and `publish = false` remain; no registry release is implied.

Local path consumption works without registry publication.
GitHub releases distribute the complete workspace, beginning with version/tag
`0.1.0`. Use a tagged Git dependency or a path into the extracted repository;
internal dependencies resolve together without registry publication or patches.
Per-crate registry packaging is not a release gate for this distribution model.
See the [GitHub distribution policy](../../docs/specs/workspace.md#github-distribution).
