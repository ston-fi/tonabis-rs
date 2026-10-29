# TONCO binding inventory

`bindings.json` records public types and helpers, all 17 wrappers and 66 callable
getters, selectors, argument kinds, shared ownership, source and fixture hashes.
It is an audit inventory, not a generator input. Ordinary records use native
derives/adapters; handwritten exceptions have protocol-specific rationale and
focused test references. SDK/catalog provenance and notices are retained in the crate.

Independent expected fixtures live under `crates/tonco/src/tests/fixtures` so
packaged tests need no external source checkout or sidecar. Protocol suffixes,
structured arity and native tail/Cell/Slice behavior have focused regression tests. Numerical
and executable corpora/tools remain excluded. Do not rewrite expected BOCs to
agree with a codec. New protocol evidence requires reviewing source and fixture
provenance before changing either.

Use the crate AGENTS guide for scoped Cargo checks, formatting, packaging and an
extracted-package consumer.

`consumer.rs` is the standalone archive-consumer smoke source. After the scoped
package command, extract `<target-dir>/package/tonabis_tonco-0.1.0.crate` into a
fresh directory outside the workspace. Put this source at `consumer/src/main.rs`
next to the extracted directory, with:

```toml
[package]
name = "tonco_external_consumer"
version = "0.0.0"
edition = "2024"
[dependencies]
tonabis_tonco = { path = "../tonabis_tonco-0.1.0" }
ton = "=0.4.12"
fastnum = "0.7"
```

Run `cargo run --manifest-path <external>/consumer/Cargo.toml --offline`, then
`cargo test --manifest-path <external>/tonabis_tonco-0.1.0/Cargo.toml --locked --offline`.
Use a unique target directory. The smoke compiles construction of all 17 wrappers,
codec/storage paths, structured/scalar results and exact direct TON field types;
its main executes only offline codecs/results. `cargo tree -i ton` from that
consumer must show the same TON package for the consumer and protocol dependency.
