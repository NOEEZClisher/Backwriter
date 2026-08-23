# Backwriter Anddress and Exact Line Model

Status: normative target-local raw locator and implemented v3 wire algebra.

An Anddress describes one target in current structure and carries no past-target
lineage or inherited identity. Backwriter establishes only the resulting current
structure.

## Raw locator algebra

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

## V3 wire

The sole Anddress wire is `artext.backwriter-anddress.v3`. Its compact JSON
objects have these fixed encoder orders:

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

An encoder emits exactly the compact field order above. A decoder may accept
JSON whitespace and object-key order variation only. A unique readable non-v3
version returns `UnsupportedVersion` before a malformed body is considered. It
rejects unknown, duplicate, missing, or wrong-typed fields, and a noncanonical
ordinal, as `Encoding`.
Violations in decoded workspace coordinate, logical path, or exact extent are
`Invalid`. Actual allocation failure is `Resource`.

## Replacement cutover

v3 one-time replaced v2. There is no compatibility decoder, migration, alias,
or parallel schema. Search, View, and Pick producers, consumers, and regressions
use v3 together. This document does not authorize future API or capability
behavior changes.

Whole-source bytes and fingerprints may remain private call-local observation
evidence, but are not target identity. The bytes returned by one retained
no-follow handle read are the observation; they do not prove stable Workspace
Source, and no before/after or second-read guarantee is added.

Backwriter Core constructs and provides target Anddress values from an accepted
current observation. Search delivers those values as results; it is not an
issuer. There is no separate registry, issuance lifecycle, lookup/reuse state,
durable identity, or global identity.

No generic locator layer, registry, issuance lifecycle, temporal identity, or
locator algorithm is implied by this algebra. Anchor authority is closed only
by the Protocol.
