# STONks contract bindings

`tonabis_stonks` provides contract wrappers, storage, messages and getters for
STONks. It contains no pricing math, signing, broadcasting, provider ownership,
retries or code authentication.

## Supported identities and capabilities

STON v8/v10/v11 and DeDust v7 expose eight bonding fields. DeDust v12 exposes
twelve. `generation::StonksGeneration` enumerates these exact identities without runtime
authentication or a version-threshold heuristic. The crate README lists all eight
wrappers and all 26 callable getters, including shared methods on each minter.
STON-specific selectors are exposed only on the STON minter wrapper.

| Selector | Wrappers | Result |
|---|---|---|
| `get_factory_data` | STON / DeDust factories | Four fields; third is router address / config cell respectively |
| `get_bonding_data` | STON / DeDust v7 / v12 minters | Eight / eight / twelve fields |
| `get_jetton_data` | All three minters | Five-field Jetton result |
| `get_version` | All three minters | Direct u32 |
| `get_wallet_address` | All three minters | Option<TonAddress> result/owner; one nullable-address Slice argument |
| 82057 | STON / DeDust v7 / v12 minters | Ten / ten / thirteen fields |
| 68631 | STON minter | Twelve-field STON routing/bonding result |
| 95602 | STON minter | Controller and router addresses |
| 71772 | DeDust v12 minter | CPMM base fee, total presale fee, creator share, creator recipient |
| `get_wallet_data` | STON / DeDust / tax wallets | Four-field results; tax balance retains its source signed type |
| 74038 / 84724 | STON / DeDust wallets | Different seven-field extended results |
| `get_tax` | Tax wallet | Buy and sell per-mille rates |

The v12 added bonding fields follow the same fee order as 71772. Pinned
independent observations contain 100, 140, 5000, recipient. The upstream guide's
reversed first two labels are not followed. This library exposes on-chain values,
without deriving curves or local prices.

## Canonical ownership and construction

Primary modules own wrappers and one inherent block. Storage lives in `storage`,
messages/dispatch in `msgs`, structured results in `method_results`. All minters
share `ston_minter::storage::StonksVirtualMinterStorage`; they do not duplicate its root
type. Its `initial_state(code, factory, content, wallet_code)` serializes full
supply 100000000000000000 and an absent bonding state. Standard/eight-field
results live in `ston_minter::method_results`. Factory initializer and developer
buy records use the identical canonical minter messages.

DeDust shared launch state and continuation records live in the v7 minter's
storage/messages; v12 imports them directly. Each generation retains its own
migration config and dispatch. Legacy factory creation is shared from the STON
factory's messages; current v12 creation belongs to DeDust factory messages.
Current forwarding bodies have one owner in STON minter messages and are imported
by DeDust/wallet code. Standard wallet results and presale bounces belong to DeDust
wallet children. The separate tax wallet retains accounting balance, its two-address
transfer context, code reference, taxed transfer and bounce domain.

Each wallet's `initial_state(code, owner, master)` creates its own exact initial
context: STON two literal-zero addresses and true flag; DeDust one literal-zero
authority and false flag; tax two literal-zero context addresses, both balances
zero and a stored code reference. Native StateInit owns serialization and address
derivation for the caller-selected workchain. No code templates are embedded.

## Parsing and protocol constraints

Ordinary root/referenced records tolerate unused tails, including address leaves.
Serialization emits modeled fields. A shorter layout can parse a longer record's
prefix, so successful parsing cannot select a generation. Structured results use
native FromTVMStack and exact whole-stack arity. Scalars consume a native value
without a one-field result wrapper. Wrong primitive types and missing fields fail.

Ordinary address fields use `TonAddress`/`InternalAddress` or
`Option<TonAddress>`/`NullableAddress`; internal zero remains distinct from absence.
Core Maybe/reference adapters retain selector bits and reference placement.
Ordinary nullable result fields derive FromTVMStack with NullableInternalAddress:
Cell/Slice addr_none is None; TVM NULL is rejected. Explicit NULL fields keep their
protocol handling, and structured result arity stays exact;
see the [workspace address policy](workspace.md#consistent-consumer-types-and-parsing).
Variable, anycast and external addresses are rejected in ordinary fields.

Unknown message dispatch bodies preserve the raw graph, including short bodies.
Typed codecs enforce body constraints; native dispatch follows the
[workspace prefix-fallback policy](workspace.md#message-dispatch). Ordinary factory/legacy-minter dispatch derives
TLB, including current STON and DeDust dispatch. Narrow field adapters preserve
nested continuation and forward-selector errors. Empty tax funding has a small
codec because native ensure_empty failures do not fall through to Unknown.

Presale internal transfers require a true payload selector, payload reference and
separate context reference. A single narrow field adapter enforces this invariant
in both derived transfer records. STON and DeDust context layouts remain distinct.
Current STON v10/v11 preliminary sell notifications validate a standard TON
recipient; the older/DeDust form permits addr_none. Current STON dispatch uses
`StonksStonMinterCurrentMessage`; ordinary legacy dispatch remains `StonksStonMinterLegacyMessage` in the same owner.

Current buy/sell bodies have untagged optional recipient/Either payload suffixes.
Omission and present-empty are distinct and preserved. These suffixes have explicit
shape validation rather than generic fixed-record tail rejection. Presale bounce
records accept transfer, burn-notification and sell-notification opcodes; tax bounce
records accept only transfer and burn-notification opcodes. Their raw truncated
suffix survives encoding. Parsing proves no sender authority or settlement.

## Evidence and boundaries

`tools/reference/stonks/bindings.json` records exact source hashes, source-to-public
symbol mappings, all expanded callable getters/selectors/argument kinds, shared
ownership, excluded math/scalar helpers, each manual exception with tests and
fixture/test dispositions. No STONks generator or shared ABI framework was added.

Retained fixtures are copied byte-for-byte into the crate. They are independent
schema/catalog/synthetic encodings with executable references replaced by empty
structural cells, plus sanitized scalar/address getter observations. Independent
new raw-bit representation hashes cover minter and all three wallet initialization
paths with empty caller code. Recording-provider tests exercise every callable
getter and typed provider-error propagation without retry. Native policy tests
cover NULL/addr_none/literal zero, Cell/Slice, address tails, missing/wrong types
and exact structured arity.

No acquisition, executable capture corpus, replay tool, ignored sidecar test,
math test or economic claim is included. The catalog MIT notice does not grant
rights to executable cells. Default tests, examples and packaged tests are offline
after dependency acquisition. Package/archive/consumer checks are provisional local
evidence; no publication, live execution or broader release qualification follows.

Standard burn-notification and excess acknowledgements have one canonical owner in
`ston_minter::msgs`. Tax-wallet dispatch imports those records directly; the
shared burn record exposes `owner` and `response_address`.

## User-facing address signatures

All three minter get_wallet_address methods take and return Option<TonAddress>.
The owner is a nullable-address Slice, with None encoded as addr_none and
Some(internal zero) encoded internally. Results map NULL/addr_none to None and
retain Some(internal zero). Handwritten bodies validate int8 workchains before
provider access. Wire records and StateInit inputs now expose standard addresses with explicit absence.
See the [shared address API policy](workspace.md#friendly-address-interfaces)
for address encoding and result semantics.
