# Umbrella crate guidance

Read the root AGENTS.md, this README and `../../docs/specs/workspace.md`.
`tonabis` is exclusively a crate of crates. Its only public items are same-name
crate re-exports: `pub use tonabis_core;`, `pub use tonabis_dedust;`,
`pub use tonabis_moon;`, `pub use tonabis_stonks;`, `pub use tonabis_swap_coffee;`
and `pub use tonabis_tonco;`.
Preserve paths such as `tonabis::tonabis_dedust::contracts`; do not alias crates,
flatten their items, add wrappers/helpers, convert errors or introduce behavior.
Core must remain an unconditional dependency and re-export, even with
`--no-default-features`; there is no `core` feature. Default features remain empty.
Protocol libraries have optional dependencies and features matching their folder
names (`dedust`, `moon`, `stonks`, `swap_coffee`, `tonco`); gate their same-name
crate re-exports with those features.
`all` enables every protocol feature. Do not couple independent selections.

For every new protocol library, add its workspace dependency, optional umbrella
dependency, folder-name feature and gated same-name re-export, and include it in `all`,
then update root/crate README and agent inventories. Keep documentation examples
executable: `cargo test -p tonabis --doc --all-features --locked`. Do not add unit tests that
only restate re-export declarations. Public type compatibility is checked through
real consumer examples and external builds. Run `cargo test --workspace --locked`,
`cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`,
`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --locked`, and formatting
for integration changes. Check all 32 combinations of the five protocol
features, plus the `all` alias and `default-features = false`, with an external
consumer; verify default dependency resolution includes core and no protocols.
`tools/reference/integration/check_consumers.py` runs that external-consumer
matrix and the default dependency-graph check. Its consumer also exercises codecs,
storage and shared TON identity without network I/O. Underlying package extraction
and validation remain distinct from umbrella path-consumption evidence.
The crate adds no runtime ownership or errors; these
remain the responsibility of the underlying crates and their callers.

Publication is disabled. Its path dependencies must resolve to the matching local
packages; package checks must not silently substitute unrelated registry crates.
Use a standalone external consumer for local integration and document packaging
limits until underlying crates have a supported registry release process.
