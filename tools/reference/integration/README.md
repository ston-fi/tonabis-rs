# Workspace consumer validation

Run from the repository root with Rust 1.94.0 and Python 3.9+:

```sh
python3 tools/reference/integration/check_consumers.py
```

The consumer uses crates.io TON dependencies and rejects local TON patches. Its
direct TON dependency matches the workspace lockfile, so direct and transitive
public types resolve to the same registry package.

The script creates a temporary consumer outside the workspace and removes it when
finished. It seeds that consumer with the workspace lockfile, adjusts only its
local package entries offline, then uses locked Cargo invocations. Build artifacts
stay in `target/integration-consumer`; no process or service remains running.

`consumer.rs` compiles and executes offline codecs, storage and local construction
through the umbrella's full crate names. It type-checks borrowed ContractClient
wrappers and structured/scalar getter returns against a direct TON dependency;
getter futures are never executed. It also passes one shared Coffee initialization
record across distinct vault storage types. The scratch manifest forwards each
protocol feature independently; the `all` case activates only the umbrella's
`all` alias. No registry substitute is used for an unpublished local package.

Every one of the 32 protocol feature subsets is built and run. Each selection's
normal umbrella dependency graph must contain core and exactly those protocols.
The `all` alias and ordinary defaults are checked separately. This proves local
path consumption and independent feature selection, not umbrella registry
packaging, which requires the underlying unpublished packages to be available.

For protocol package checks, use `cargo package -p <package> --allow-dirty --locked`,
inspect the actual `.crate` and normalized manifest, then compile/run a consumer
against a fresh extraction outside the workspace. Coffee and TONCO retain scoped
consumer sources in their reference directories; STONks' executable crate example
also exercises its public codec/storage/getter paths. Crate-local tests must run
from the extracted archive without another source checkout or executable corpus.

Protocol archives now depend on unpublished `tonabis_core`. Package and extract core
first. For protocol packaging, pass
`--config 'patch.crates-io.tonabis_core.path="/absolute/path/to/workspace/crates/core"'`.
For extracted-package checks and external consumers, point that first-party patch
to the extracted core directory instead. Refresh only scratch lockfiles for these
local path changes before using `--locked`. No TON patch is needed; all TON crates remain
registry dependencies. This validates local archives, not registry release readiness.
