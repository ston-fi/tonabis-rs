# DeDust binding maintenance

Offline contract maintenance tools:

| File | Responsibility |
|---|---|
| `generate_bindings.py` | Generate storage/message records and family indexes from 19 pinned catalog schema identities |
| `rust-type-names.json` / `type_names.py` | Map catalog-derived Rust type identities to explicit public contract/generation names; preserve source identities and variant labels |
| `bindings-generation.json` | ABI hashes, generated-file ownership, declaration mapping and getter API metadata for all 23 wrappers |
| `bindings_api.py` | Resolve canonical Rust declaration/getter paths from that metadata |
| `generate_getter_tests.py` | Regenerate public-method assertions from the existing independent captured getter stacks |
| `../../tests/test_dedust_generation.py` | Regression test that rejects writes to handwritten files |
| `corpus/ton-blockchain-abis/build/abi/` | The 18 required ABI JSON files, hash-checked against the mapping before generation |

The V2 Uranus wallet uses the V3 catalog input with the generator's explicit
name substitution, hence 18 inputs for 19 schema identities. Four handwritten
contracts (classic LP wallet/managed pool, CPMM position/deposit) are also mapped
but have no generated schema. Catalog inputs come from TON Core's
`ton-blockchain/abis` revision `7dc57067d85c95dc25c09ccaaf108eaba62f6303` and
retain its MIT notice in `corpus/LICENSE`. Individual byte hashes are in the
mapping; the generator refuses mismatched inputs before writing output.

Classic V2 native and Jetton vaults share pool identity and swap continuation
records from `contracts::classic::vault_native_v2::storage`: `DedustPoolType`,
`DedustVaultNativeV2PoolParams`, `DedustSwapKind`, `DedustSwapStepParams`, `DedustSwapStep`,
`DedustSwapParams` and their tag records. Import that owner directly for either
vault; the former Jetton-local copies are removed without aliases. Wire formats
and independent fixtures are unchanged.

## Regenerate

Python 3.9+ standard library and the repository's Rust formatter are sufficient.
Run from the repository root:

```sh
python3 tools/reference/dedust/generate_bindings.py tools/reference/dedust/corpus
python3 tools/reference/dedust/generate_getter_tests.py
cargo fmt --all
python3 -m unittest discover -s tools/tests -v
cargo test -p tonabis_dedust --locked
```

Review generated changes against the independent fixture values. Repeat the
first three commands and require byte-identical output. Before and after
regeneration, compare hashes of all handwritten files: the generator's
`generated_paths()` allowlist identifies its only permitted outputs; the getter
test generator additionally owns `src/tests/test_getter_stacks.rs`. Formatting
should not alter other files when starting from a formatted tree.

Generated ownership excludes wrapper primary files, all `method_results`,
`classic/asset.rs`, the four handwritten storage layouts, X1000 V1 messages and
the complete flat CPMM pool/X1000 V2 message modules containing authored
signed/payload codecs, and X1000 V2 storage containing runtime/context codecs. Change the generator rather than editing its
output. Handwritten API edits need no regeneration, but update mapping/test
metadata if a recorded selector's Rust path or return type changes.

`schema-vectors.json` and getter stack fixture values remain independent inputs,
not generated expected values from Rust codecs. Updating schemas or recapturing
wire vectors requires separate source evidence. This repository does not carry
executable cells, live acquisition scripts, numerical oracles or market adapters.
The included tools never access the network.

## Flat record ownership

A contract's msgs/storage file contains its types and their implementations.
Modules that combine schema records with authored codecs or construction methods
are maintained as complete flat files and excluded from generated outputs.
The generator must preserve them and never recreate former child modules.
For a mechanical authored move, unit tests and formatting suffice; generation
checks apply only when generator inputs or ownership change.

`excluded_declarations` records reference-only source layouts with no Rust API.
The Native Vault V2 inline deposit candidate stays in the pinned corpus and
historical fixture file, but is not generated or counted as a supported codec.
