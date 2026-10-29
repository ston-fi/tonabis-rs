# STONks binding inventory

`bindings.json` records public symbols, constructors, all 26 callable getters,
canonical shared ownership, native conversions and manual codec exceptions.
It also retains source and fixture hashes for independent wire evidence.
It is an audit inventory, not a code generator.

No executable corpus, acquisition/replay scripts or general generator is retained.
Changes are ordinary handwritten Rust declarations with native derives. The
independently expected fixture bytes live in `crates/stonks/src/tests/fixtures` so
packaged tests work outside this repository. Fixture bytes are immutable absent
new protocol evidence. New initialization vectors explicitly record independent
raw-bit/representation-hash authoring with empty code; their expected values were
not obtained from the Rust initializer.

The source catalog MIT notice is packaged in the crate. Provenance metadata does
not license captured executable BOCs. Source historical replay claims are not new
validation. See `docs/specs/stonks.md` and `crates/stonks/AGENTS.md` for behavior,
ownership, evidence limits and exact validation commands.

For maintenance compare the pinned source/public mappings, then verify that every
callable getter remains represented by a recording-provider test, including methods
shared across wrapper generations. A declaration count alone is insufficient.
Each retained manual codec/dispatcher entry names its required behavior, the native
limitation and a focused regression. Ordinary tails and primitive field decoding
are delegated to the locked TON libraries; strict source stack helpers are excluded.
