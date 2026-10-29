# Bounded offline Coffee bindings generation

`bindings.json` records 26 wrappers, all 60 callable getters, public symbols,
construction methods, canonical owners, manual codec exceptions and fixture
hashes. `SOURCE_PROVENANCE.md` documents the catalog and SDK evidence.
Generation describes wire layouts; it does not validate protocol execution or economics.

`schema-inventory.json` contains normalized and explicitly enriched MIT interface
facts. The bundled `interfaces/` retain the original files/license and catalog
revision. `bindings-generation.json` pins all input hashes and lists exactly 22
writable storage/message paths and the complete protected flat authored modules. There is no
network dependency, `/tmp` research corpus, source checkout or executable input.
Python 3.9+ is sufficient.

```sh
python3 tools/reference/swap_coffee/generate_bindings.py
cargo fmt --all
python3 -m unittest discover -s tools/tests -v
```

Wrappers, getter results, construction, shared typed values, initial-phase codecs,
TEP-89/bounce/emission helpers and signed caches are handwritten. Generation never
writes them. Ordinary records use native TL-B derives, widths and dictionary
adapters. Tagged algebraic unions use native derives. Message-role dispatch derives TLB with direct variants and Unknown last.
Empty funding uses one narrow variant-mismatch codec; short/reference-only raw
bodies reach Unknown. Factory incoming dispatch remains explicit because nested
fixed-prefix proofs must propagate errors. Narrow nullable-address/flag/suffix adapters retain protocol
constraints while ordinary containing records remain derived. The native stack derive supports TLB field adapters. Typed dictionary roots,
referenced DTOs and nullable internal Slice results still need their documented
conversion semantics and exact arity. Every manual trait implementation is listed in the
inventory with a source mapping, reason and test.

Independent expected data live only in `crates/swap_coffee/src/tests/fixtures`.
They are not generator input and must not change during regeneration.
The three vault families resolve their identical initialization/asset records
through canonical mappings to `vault_native::storage`; generation emits these
declarations and construction registration only once. Keep both
original bytes and fixture hashes. Changes to the schema/interface pins require
protocol evidence, a reviewed inventory/doc update and focused wire/getter checks.
Do not copy executable capture cells or revive excluded numerical/capture tooling.

The same explicit mapping resolves common pool asset/proof records to
`pool_constant_product::storage`, pool message values to
`pool_constant_product::msgs`, vault message values to `vault_native::msgs`, and
staking position data to `staking_item::storage`. Source declarations remain in
the normalized inventory for provenance and independent family-specific vectors;
generation emits each shared Rust type only at its canonical owner. Do not
replace distinct runtime/deployment or transport dispatch identities with unions.

For a generator change, hash every non-allowlisted crate file before/after, generate
and format twice, and require identical output plus unchanged handwritten files.
The tooling tests exercise input-pin failure, output allowlist failure, preservation,
determinism, fixture hashes and wrapper/getter/test reconciliation in temporary
copies. Scoped Rust tests, strict Clippy/rustdoc, examples, package archive inspection
and an external consumer of the extracted package are separate checks, documented
in the crate's AGENTS.md. Dirty-package success is provisional local packaging
validation; it neither publishes nor approves a release.

The TEP-74 Jetton notification envelope shared by CrossDex (all generations),
MEV (both generations) and staking vault is
`contracts::cross_dex::msgs::CoffeeCrossDexJettonTransferNotificationMsg`.
Import this canonical record directly; the former per-contract copies are removed
without aliases. Each contract keeps its own dispatch enum and message roles.
The opcode, fields and independent family fixtures are unchanged.

## Flat record ownership

A contract's msgs/storage file contains its types and their implementations.
Modules that combine schema records with authored codecs or construction methods
are maintained as complete flat files and excluded from generated outputs.
The generator must preserve them and never recreate former child modules.
For a mechanical authored move, unit tests and formatting suffice; generation
checks apply only when generator inputs or ownership change.
