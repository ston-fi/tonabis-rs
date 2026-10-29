# Structural provenance and limits

`bindings.json` records public mappings, source hashes, fixture provenance and
all callable getter capabilities.

Bundled interfaces retain TON Core's MIT notice at catalog revision
`7dc57067d85c95dc25c09ccaaf108eaba62f6303`. `schema-inventory.json` is a normalized,
enriched structural transcription from that baseline; it is hash-pinned together
with all interfaces. It includes recorded nullable ownership fields, active and
inactive phase flags, typed staking values, uint32 extra IDs, rescue bodies,
wallet-discovery forms and role-specific selector corrections. The original
interface filenames and SHA-256 values remain in each normalized family row.

Typed staking values and reconstructed emission/initialization layouts also use
MIT Coffee documentation revision `330f5603a6c53ebe3a9ed4c3328dce2171c0c46f`.
`documentation-license.txt` and the crate's THIRD_PARTY_NOTICES preserve that notice.
No BSL implementation expressions, executables, disassemblies, capture/replay
programs or acquisition tools are included.

Crate-local independent fixtures retain their original bytes. Their
source hashes and dispositions are retained in the inventory. Synthetic wire
vectors and independently authored observed message/state/hash data supplement
recording-provider tests. Source historical observations describe their original
context; they are not new VM/live runs in this destination. No numerical/API
parity, code authenticity, signature acceptance, production or release claim is
inferred from them. Fixture expectations never come from normalized generator input.
