# Backwriter Anddress and Exact Line Model

## 0.2.5 encoding, attachment, and currentness boundary

Gates 1 through 7 preserve the complete published v5 algebra and wire. Exact
source Line count remains a `SourceIdentity` field and currentness requirement. A typed
address whose hash and byte length match but whose claimed Line count differs
from the accepted observation or Host proof remains `NotCurrent`. The raw
observer now derives that count without Paragraph, parent, target geometry, or
`StructuralCursor`; structural consumers compose the same state with the sole
cursor in the same read.

Strict decode and the sole crate-private Issuer remain the only safe
construction boundaries, and public `validate()` remains strict. The Issuer
validates the shared source once and each target geometry once. View, Check,
and Anchor accept those already typed invariants without repeating validation.
The public reusable encoding surface is:

```rust
pub fn encode_into(&self, output: &mut Vec<u8>) -> Result<(), AnddressError>
```

`encode_into` clears the vector on entry, calculates the complete canonical
length with checked arithmetic, and fallibly reserves before appending. On
error its length is zero, although its capacity may remain reusable. On success
it contains exactly the current fixed-order v5 object and no trailing bytes.
Existing `encode()` remains and delegates through one newly allocated empty
vector to the same private emitter. This adds no second wire, builder, JSON
model, compatibility path, or change to the four canonical KAT byte sequences.

One crate-private geometry helper attaches a provisional File-parented Line to
an exact containing Paragraph. It alone checks target kind, containment,
checked Line-offset subtraction, Paragraph count arithmetic, and parent
assignment before mutation. Search maps impossible observed geometry to its
invalid-source failure; Apply leaves ordinary non-Line or outside candidates
unattached and preserves its existing preparation failure boundary. This is
not new public algebra or wire.

Gate 6 makes the Issuer's crate-private construction entry delegate to the
same strict owned-source validation and `Arc` construction used by decode.
Parent/project operations still clone the already validated shared source.
`Anddress::same_source` now directly serves Runtime source grouping and
same-source Edit validation, while the sole Runtime source-state comparator
continues to require SHA-256, byte length, and Line count. No unchecked
constructor, second validator, or second writer exists. Production G is
304,431 bytes/9,213 lines, -1,727/-48 from F and +7,162/+259 from B.
Gate 7 leaves those production bytes unchanged, passes the fixed A/B/G and
GNU/musl evidence, and advances only Cargo and `bw version` to source-ready,
unpublished `0.2.5`.

## Published and closed 0.2.4 v5 target algebra

The source is hard-cut to `artext.backwriter-anddress.v5`. Published and closed
`0.2.3` remains immutable v4 release evidence; current decode returns
`UnsupportedVersion` for v4 and v3 and has no compatibility alias, wrapper, or
parallel execution path. Gate 7 makes Cargo and `bw version` source-ready
`0.2.4`; Gate 8 publishes and closes the matching v5 four-target distribution.

Every v5 target shares one exact `SourceIdentity`:

```text
WorkspaceCoordinate + LogicalPath + SourceStateSHA256 + SourceByteLength
                    + SourceLineCount
```

The target geometry is:

```text
File      := [0, SourceByteLength) + SourceLineCount
Paragraph := [StartByte, EndByte) + FileLineOffset + LineCount
Line      := [StartByte, EndByte) + Terminator + ParentGeometry
             + LineOffsetInParent
```

Offsets and counts are nonnegative exact naturals. `FileLineOffset` is the
zero-based distance from the File's first Line to the Paragraph's first Line.
A Paragraph display range is therefore `fileLineOffset + 1` through
`fileLineOffset + lineCount`. For a Paragraph-parented Line, the absolute
one-based Line number is `parent.fileLineOffset + lineOffsetInParent + 1`; for
a File-parented Line it is `lineOffsetInParent + 1`. Terminator is exactly
None, LF, CR, or CRLF. A nonblank content Line uses its enclosing Paragraph as
parent. A blank or horizontal-space/tab-only Line uses File as parent.

Raw equality includes the complete flattened v5 value. Source/state relations
come from the shared `SourceIdentity`; byte containment and overlap come from
exact ranges; parent/project, Line count/number/range, terminator, and
projection validity come only from the target geometry. These operations are
Anddress algebra, not Search metadata, a View relation scan, or capability
provenance. Reappearing equal bytes may recreate an equal raw value but proves
no history or continuity.

One crate-private `AnddressIssuer` is the only ordinary-address constructor;
decode and issue pass through the same source/geometry validator. Addresses
issued for one observed source share one `Arc<SourceIdentity>`. Target geometry
is allocation-free and a Line stores its complete File or Paragraph parent
geometry, so `parent` and `project` neither parse nor reconstruct source
structure. Gate 3 installs one private allocation-bounded `StructuralCursor`
for complete-source Line/Paragraph framing. Search emits Anddresses directly,
and display positions derive from the address algebra rather than a parallel
position value. View calls `project` before I/O and returns that exact projected
address with its range Content; it neither scans relations nor reconstructs an
ancestor. One-shot Line Edit reads its terminator directly from this validated
geometry; Apply uses `contains` and `overlaps` for prospective provenance and
does not invoke View or add a second range algebra.

The encoder emits one compact object with this fixed field order:

```text
common    version, workspaceCoordinate, logicalPath, sourceStateHash,
          sourceByteLength, sourceLineCount, kind
File      <common only>
Paragraph <common>, byteStart, byteEnd, fileLineOffset, lineCount
Line      <common>, byteStart, byteEnd, terminator, lineOffsetInParent,
          parentKind
Line/Paragraph parent additionally: parentByteStart, parentByteEnd,
          parentFileLineOffset, parentLineCount
```

All lengths, ranges, counts, and offsets are canonical unsigned-decimal JSON
strings. `terminator` is `none`, `lf`, `cr`, or `crlf`; `parentKind` is `file`
or `paragraph`. File-parent Lines carry no unused Paragraph fields. Unknown,
duplicate, missing, wrong-typed, or target-inapplicable fields and
noncanonical or overflowing decimals are `Encoding`; a recognized non-v5
version is `UnsupportedVersion`; inconsistent source or geometry is `Invalid`;
fallible construction/encoding allocation is `Resource`.

The public allocation-free algebra is `same_source`, `same_state`, `contains`,
`overlaps`, `parent`, `projection_valid`, `project`, `range`, `line_count`,
`line_range`, `line_number`, and `terminator`. Line ranges are zero-based
half-open File-Line ranges; Line numbers are one-based. A downward projection
returns `Invalid`; Line-to-Paragraph for a File-parent Line is valid but
returns `None`.

Status: implemented, published, and closed v5 raw-address, Search, View,
Check, Edit/Apply, Anchor, and direct-consumer authority in `0.2.4`.
Published and closed `0.2.3`, `0.2.2`, `0.2.1`, and `0.2.0` implement the v4
algebra and hard cutover below. The closed public `0.1.0` v3
algebra is preserved later in this document only as immutable release evidence;
it is not accepted by current production code. The published and closed
`0.2.3` Patch Box leaves this v4 algebra and wire byte-identical.

An Anddress describes one target in current structure and carries no past-target
lineage or inherited identity. Backwriter establishes only the resulting current
structure.

## Implemented 0.2.0 v4 algebra

An ordinary v4 Anddress has exactly this semantic identity:

```text
Anddress = WorkspaceCoordinate
         + LogicalPath
         + SourceStateHash
         + SourceByteLength
         + TargetKind
         + [StartByte, EndByte)
```

`StartByte` is inclusive and `EndByte` is exclusive. Both are byte offsets into
the exact source state named by `SourceStateHash` and `SourceByteLength`, with
`0 <= StartByte <= EndByte <= SourceByteLength`. A File covers
`[0, SourceByteLength)`; Paragraph and Line cover their exact current source
bytes. Target text, terminator text, Paragraph or Line ordinal, and contextual
neighbors are not v4 identity. Duplicate equal text targets are distinguished
by their ranges within the same exact source state.

Raw v4 equality is exact equality of every field above. The source-state hash
is final currentness authority. Admission decides only whether Runtime may use
the logical source; it is not raw equality. A changed source has a different
authoritative state and invalidates every ordinary Anddress for the previous
state. If the exact complete source state later reappears, raw equality may
reappear without establishing history, survival, or continuity.

Search is the only target finder and constructs v4 values while reading and
hashing current source once. Ordinary View validates hash and length while
copying an allowed self-or-ancestor projection and returns that target's exact
current v4 value; Check currentness compares only hash and length. View neither
relocates nor searches downward. A Line-to-Paragraph request without an exact
containing current Paragraph returns relation-absent, and a valid caller-built
nonstructural Paragraph or Line range remains consumable. Apply
enforces the exact v4 source-state/range precondition and patches that public
range directly from fixed-chunk staging. It creates no private ordinal/text
locator or relocation mapping.
`CurrentObservation` is Runtime-private, call-local hash/length producer state
for one selected source and is not part of wire or equality.
The Protocol's optional `0.2.1` Host-authoritative proof may retain only the
completed source hash and length plus its Runtime/workspace/admission/generation/
logical-path binding. It is neither an Anddress field nor target identity,
locator state, equality evidence for another binding, or continuity.
Anchor is not an ordinary Anddress; creation directly confirms exact current
structure, and only its live Runtime-local continuity may follow one unique
same-kind range/provenance candidate across a Backwriter-owned Apply.

The implemented and published `0.2.2` general editing Adapter accepts an encoded
v4 Anddress unchanged, decodes it through this wire authority, and uses that
same value as the existing `Edit::Replace` target after a private Runtime View.
Line body replacement may append only the terminator observed by that View.
This creates no alternate locator, content identity, ordinal, context,
relocation evidence, wire field, equality rule, or compatibility schema.

The closed `0.2.3` Patch Box does not change this algebra or wire. Gate 2
attaches a current one-based Line number or Paragraph inclusive Line range to
each applicable Search occurrence from that same observation; File has no
position. This is descriptive information, not an Anddress field, locator,
equality input, currentness proof, or selector. A fresh Anddress in a
successful Gate 5 Edit receipt names only the exact resulting current state
already described by v4. It adds no predecessor, successor, survivor,
relocation, or publication-history relation to the input address.

The source hash is SHA-256 using the existing incremental implementation. The
v4 wire version is `artext.backwriter-anddress.v4`. The encoder emits exactly
this compact field order for every kind:

```json
{"version":"artext.backwriter-anddress.v4","workspaceCoordinate":"<64 lower-case hex>","logicalPath":"<logical path>","sourceStateHash":"<64 lower-case hex SHA-256>","sourceByteLength":"4","kind":"file|paragraph|line","byteStart":"0","byteEnd":"4"}
```

`sourceByteLength`, `byteStart`, and `byteEnd` are canonical unsigned decimal
JSON strings. They are `"0"`, or begin with an ASCII digit `1`–`9` followed only
by ASCII digits, and must fit the implementation's checked filesystem-range
integer. Empty, signed, leading-zero, and overflowing decimals are `Encoding`.
Ranges must satisfy `start <= end <= sourceByteLength`; File must be exactly
`[0,sourceByteLength)`, including `[0,0)` for an empty File. Invalid workspace
coordinate, logical path, source hash, or range is `Invalid`.

The public constructor accepts all eight semantic inputs and returns only a
valid v4 value. Its inspectors expose borrowed source fields, machine-integer
length/ranges, and the unit target kind. Values from one Search source share one
private immutable source-identity allocation while remaining independently
cloneable, comparable, and encodable; no public `Arc` detail is exposed.

The decoder accepts JSON whitespace and object-key order variation, but rejects
unknown, duplicate, missing, and wrong-typed fields as `Encoding`. Malformed
JSON and duplicate `version` are `Encoding`; a unique readable non-v4 version,
including well-formed v3, is `UnsupportedVersion` before body semantics. A v4
body with invalid semantic values is `Invalid`. Recoverable reserve/copy/encode
allocation failure is `Resource`; allocation failure inside infallible standard
library ownership primitives remains the process allocator boundary and is not
misreported as a typed error.

`0.2.0` is a hard cutover. Current production contains no v3 decoder, encoder,
constructor, alias, shim, migration layer, or dual-schema API.

## Historical immutable 0.1.0 v3 release evidence

### v3 raw locator algebra

Raw Anddress equality is exactly this target-local algebra:

```text
File        = WorkspaceCoordinate + LogicalPath
Paragraph   = File + 0-based current ParagraphOrdinal
Line        = File + 0-based current LineOrdinal + ExactExtent
ExactExtent = content + exact CR/LF/CRLF/absent terminator
```

`LogicalPath` is the workspace-root-relative canonical UTF-8 component spelling
Runtime observed; Core does not case-fold it. Different logical paths are
different Files even if the underlying filesystem object is a hard link.
Paragraph and Line ordinals identify duplicate current targets without a
content-, range-, or fingerprint-based discriminator. Any one-byte change to
either part of `ExactExtent` makes a new Line Anddress.

Admission decides only whether a source can be constructed or used; raw equality
does not include admission. It also excludes source bytes, source length,
source fingerprint, admission root or policy, byte range, Paragraph content,
Paragraph range, Paragraph fingerprint, Paragraph Line count, and Line
Paragraph evidence.

The same current tuple may reconstruct the same raw Anddress. Deletion and
recreation, a separator change followed by the same ordinal, and A→B→A do not
make continuity, survivor, or historical-identity claims. Moving an ordinal
makes a new raw Anddress. The Protocol's closed Anchor/Anchedress authority
owns work continuity; raw Anddress does not.

Every selected UTF-8, NUL-free source has one File structure. Lines preserve
CRLF, CR, LF, and no terminator exactly; empty and ASCII space/tab-only Lines
are separators. A Paragraph is a maximal contiguous run of text Lines. `Block`
is historical wording for that Paragraph and creates no type, alias, variant, or
wire value.

### v3 wire

The historical `0.1.0` Anddress wire was
`artext.backwriter-anddress.v3`. Its compact JSON objects had these fixed
encoder orders:

```json
{"version":"artext.backwriter-anddress.v3","workspaceCoordinate":"<64 lower-case hex>","logicalPath":"<logical path>","kind":"file"}
{"version":"artext.backwriter-anddress.v3","workspaceCoordinate":"<64 lower-case hex>","logicalPath":"<logical path>","kind":"paragraph","ordinal":"0"}
{"version":"artext.backwriter-anddress.v3","workspaceCoordinate":"<64 lower-case hex>","logicalPath":"<logical path>","kind":"line","ordinal":"0","exactExtent":"x\r\n"}
```

Thus File is version, workspace coordinate, logical path, and
`{"kind":"file"}`; Paragraph adds `{"kind":"paragraph","ordinal":"0"}`;
and Line adds `{"kind":"line","ordinal":"0","exactExtent":"x\r\n"}`.
`ordinal` is an arbitrary-size canonical decimal JSON string: `"0"`, or a
nonzero ASCII digit followed only by ASCII digits. It has no numeric upper
bound. `exactExtent` is one UTF-8 JSON string containing the complete Line
extent: content followed by exactly one terminal CRLF, CR, LF, or no terminator.
It rejects NUL, any internal CR or LF, and an empty extent without a terminator.
There is no range, fingerprint, source/admission material, Paragraph evidence,
or separate terminator field.

`WorkspaceCoordinate` is exactly 64 lower-case hexadecimal characters encoding
the SHA-256 digest of this transcript:

```text
transcript(
  "artext.backwriter-workspace-coordinate.v3",
  PlatformTag,
  CanonicalWorkspaceRootBytes,
)
```

The existing transcript framing applies to every field in that order: each data
byte `00` is encoded as `00 ff`, and each field ends with `00 00`. On Unix,
`PlatformTag` is `unix` and `CanonicalWorkspaceRootBytes` are the raw `OsStr`
bytes of Runtime's accepted, opened canonical workspace root. On Windows,
`PlatformTag` is `windows` and the root bytes are that canonical `OsStr`'s UTF-16
code units, each serialized little-endian as one `u16`. Neither platform forces
UTF-8, performs lossy conversion, case-folding, or normalization. The algorithm
is SHA-256 only: there is no setting, negotiation, SHA-3, or fallback.

The historical encoder emitted exactly the compact field order above. Its
decoder accepted
JSON whitespace and object-key order variation only. A unique readable non-v3
version returns `UnsupportedVersion` before a malformed body is considered. It
rejects unknown, duplicate, missing, or wrong-typed fields, and a noncanonical
ordinal, as `Encoding`.
Violations in decoded workspace coordinate, logical path, or exact extent are
`Invalid`. Actual allocation failure is `Resource`.

### v3 replacement cutover history

v3 one-time replaced v2 in `0.1.0`. Current `0.2.0` then hard-cut over every
producer, consumer, and regression to v4. No current compatibility decoder,
migration, alias, or parallel schema survives.

Whole-source bytes and fingerprints may remain private call-local observation
evidence, but are not target identity. The bytes returned by one retained
no-follow handle read are the observation; they do not prove stable Workspace
Source, and no before/after or second-read guarantee is added.

Backwriter Core constructs and provides target Anddress values from an accepted
current observation. Search delivers those values as results; it is not an
issuer. There is no target registry, issuance lifecycle, locator lookup/reuse
state, durable identity, or global identity. Optional Runtime-local current
source-state proof is governed only by the Protocol and retains no target map or
result.

No generic locator layer, registry, issuance lifecycle, temporal identity, or
locator algorithm is implied by this algebra. Anchor authority is closed only
by the Protocol.
