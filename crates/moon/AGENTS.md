# Moon maintainer and downstream-agent guide

Read the root AGENTS.md, this README, `../../docs/specs/moon.md` and
`../../tools/reference/moon/README.md`. Moon owns contract wrappers/codecs only.
Keep SDK CPMM/CLMM/mempad separate from catalog pool/range, old/new orders and booster.
The README inventories all 18 wrappers and 54 getters. No math, conversion to math
state, market adapters or implicit runtime/provider ownership belongs here.

## API and module ownership

Primary wrapper modules own one inherent block and use caller-owned TON clients.
Storage lives in `storage`, structured getters in `method_results`; return scalar
values directly. SDK families share `msgs` across their cooperating contracts;
catalog wrappers own `msgs` individually. Do not duplicate shared wire records,
flatten paths or add aliases. Use protocol/family-qualified records and `Msg`
suffixes, direct `ton` imports, `TLB`/`FromTVMStack` derives and `#[ton_methods]`
where behavior matches. Preserve selectors, widths, argument order and stack kinds.
The CLMM account-address and curve-null getters use descriptive Rust names with
explicit numeric selector attributes; their original names remain unresolved.
`get_pool_address` must pack native masters as TVM NULL and internal zero as Slice.

Generated schema modules and dispatch are explicitly allowlisted in
`tools/reference/moon/bindings.json`. Do not append handwritten methods there.
Wrappers, result DTOs, transfer literals, booster configuration, lifecycle values,
CPMM layout discriminators, metadata and address helpers are handwritten. External
trigger records use generated native derives and retain opaque body cells. Changing the schema requires reviewing its hash pin, regenerating,
formatting, checking handwritten-file preservation and proving a second formatted
pass is identical. Handwritten edits do not require generation.

## Codec invariants

Ordinary root/reference/address/dictionary tails are tolerated; serialization may
omit those tails. Typed bodies validate their fields; native dispatch follows the workspace prefix-fallback policy. Unknown bodies retain bits/refs.
Mempad's mint includes a nested opcode: keep explicit dispatch so a nested prefix
error cannot become Unknown. Other dispatch uses native derives. Metadata is a
zero-prefixed linear byte snake with preserved chunks, not lossy UTF-8. Bounces
retain the original prefix or referenced body plus opaque diagnostics.
Keep types and their implementations in flat msgs/storage files. Transfer
records use TLB with the documented literal payload-flag adapter.
Keep the metadata codec: native `SnakeData` in the locked TON version does not
validate the zero prefix or linear byte layout and drops empty continuation cells.

Ordinary address fields use `TonAddress`/`InternalAddress` or
`Option<TonAddress>`/`NullableAddress`; internal zero remains distinct from absence.
Core Maybe/reference adapters retain selector bits and reference placement.
Ordinary nullable result fields derive FromTVMStack with NullableInternalAddress:
Cell/Slice addr_none is None; TVM NULL is rejected. Explicit NULL fields keep their
protocol handling, and structured result arity stays exact;
see the [workspace address policy](../../docs/specs/workspace.md#consistent-consumer-types-and-parsing).
Variable, anycast and external addresses are rejected in ordinary fields.

CPMM init-only/live and legacy/current layouts have explicit discriminators.
Booster configuration has mode/tail constraints. Transfer variants enforce their
literal payload flag on both read and write. Launch lifecycle tags are constrained
2/3-bit domains. Wallet derivation uses native StateInit with split depth eight,
then splices the owner's first hash byte. Do not replace it with ordinary hashing.

## Validation and consumers

Unit tests live in `src/tests/test_*.rs`, registered by `src/tests.rs`. Begin with
an affected test filter, then `cargo test -p tonabis_moon --locked`. Existing fixed
wire/address values come from independent source fixtures; do not regenerate them
from production code or copy executable captures. Test observable protocol
boundaries, selectors, arguments, malformed known messages and typed result shapes.
Do not add scalar DTO tests that merely retest TON conversions.

Integration requires root workspace tests/examples/fmt, strict Clippy, rustdoc and
`python3 -m unittest discover -s tools/tests -v`. Public/package changes also require
`cargo package -p tonabis_moon --allow-dirty --locked`, inspection of the archive,
and an external consumer of the extracted package. Keep README doctests and
`examples/contracts.rs` usable. New public behavior must update the README,
specification, relevant schema/path mapping and focused evidence. Preserve notices,
`publish = false` and proprietary package policy; package validation is not release
approval. See the specification for API behavior and offline evidence limits.

## Standard-address getter APIs

The five account/wallet getters use &TonAddress and return TonAddress. Order
factory get_pipe takes Option<TonAddress>: None encodes an addr_none Slice and
Some(internal zero) stays internal. The existing CLMM factory get_pool_address
instead encodes a None master as TVM NULL; preserve that distinct contract.
Handwritten bodies preserve selectors/order and validate int8 workchains before
provider access. Wire/result records use standard addresses and preserve explicit absence.
The current getter inventory is `tools/reference/integration/tonaddress-api.json`.
See `docs/specs/workspace.md#friendly-address-interfaces` for the shared policy.


Optional stack address fields use `Option<Option<TonAddress>>`: outer `None` is
TVM NULL, `Some(None)` is encoded addr_none, and `Some(Some(address))` is internal.

Public types identify their contract or shared family, for example `MoonClmmRangeStorage` and `MoonCatalogClmmRangeStorage`.
Keep separate contract entities even when layouts match. See the
[workspace naming policy](../../docs/specs/workspace.md#public-type-naming) and
[type migration map](../../tools/reference/integration/type-renames.json); renamed
Rust types have no compatibility aliases.
