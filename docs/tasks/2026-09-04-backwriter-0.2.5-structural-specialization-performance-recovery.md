# Backwriter 0.2.5 Structural Specialization and Performance Recovery

Status: Gate 1 authority complete; implementation has not started. Cargo,
`bw version`, artifacts, installers, Update, and the public distribution remain
published and closed `0.2.4`.

This tracker resolves the planning questions preserved in the companion
[source note](2026-09-04-backwriter-0.2.5-structural-specialization-performance-recovery-source.md)
and [grounded roadmap](2026-09-04-backwriter-0.2.5-structural-specialization-performance-recovery-roadmap.md).
Those two documents remain input evidence. This tracker records the approved
authority where an input alternative conflicts with the closed `0.2.4`
contract.

## Governing rule

> Semantics stay unified. Execution becomes specialized again.

The shorter release definition is **unified authority, specialized hot
paths**. The target preserves v5 structure, wire, output, ordering,
multiplicity, errors, and failure boundaries while removing work that a
specific capability does not consume. It is a performance-recovery target,
not a structural redesign.

## Fixed evidence labels

- **A** is the published v4 comparison revision
  `195aaa37068122097ecc04d2644642b6afcc6765`.
- **B** is the closed `0.2.4` production revision
  `8b20987893ea5ac454c4c0a50d0c470e26b5e650`.
- The closed `0.2.4` release Source Authority is
  `0ee4dcce14da93f925c27a04d0e79051c83fd124`.
- **C** names a future `0.2.5` candidate only after an implementation gate
  creates one. A future comparison must not relabel A or B.

The B production baseline is 297,269 bytes and 8,954 lines. GNU and musl each
have a closed 258-test `0.2.4` result.

## Gate 1 decisions

### Source Line count remains currentness evidence

`sourceLineCount` remains part of v5 `SourceIdentity`, ordinary address
equality, Runtime `CurrentObservation`, Host proof, and View, Check, and Apply
source-state comparison. A same-hash, same-length typed value with a false Line
count remains `NotCurrent`; the existing manual-v5-mutation Safe Reject is not
narrowed.

This is the conservative choice where the source note proposed removing Line
count from proof/currentness. SHA-256 plus exact byte length identifies the
source bytes, but the active v5 contract also requires the address's claimed
derived Line count to agree with the accepted observation or trusted proof.
Gate 3 may remove Paragraph and parent-geometry work from raw consumers only by
counting Lines in the same forward read with a minimal raw accumulator that
does not own or invoke `StructuralCursor`. It may not stop deriving Line count,
weaken `NotCurrent`, or change v5 fields.

### Typed addresses are valid by construction

Safe Rust can construct an `Anddress` only through strict v5 decode or the sole
crate-private `AnddressIssuer`; private fields prevent caller mutation. Decode
and Issuer validation remain strict, and public `Anddress::validate()` remains
available and strict. Unsupported-version, encoding, invalid-geometry, and
resource classifications do not change.

Gate 4 may remove a repeated `validate()` only after production reachability
proves that the hot path accepts an already typed `Anddress` created by those
boundaries. It may not weaken wire decode, public explicit validation, Edit
validation, or any source-less error priority, and it may not introduce a
second validator or unchecked wire-to-value path.

### One reusable canonical encoder is public

Gate 4 is authorized to add exactly this narrow library surface:

```rust
pub fn encode_into(&self, output: &mut Vec<u8>) -> Result<(), AnddressError>
```

The method clears `output` on entry, computes the complete required length with
checked arithmetic, and uses fallible reserve before appending canonical bytes.
An arithmetic or reserve failure returns `AnddressError::Resource` with
`output.len() == 0`; existing capacity may remain available to the caller.
After successful reserve, writing the already validated typed value is
infallible. Success replaces any previous contents with exactly one canonical
v5 object and no trailing bytes.

Existing `Anddress::encode()` remains public, creates one empty `Vec<u8>`,
delegates to `encode_into`, and returns that vector. The four exact v5 KAT byte
sequences, JSON escaping, field order, canonical decimals, and existing error
type remain unchanged. The `bw` binary may reuse one scratch vector across
results; it must not duplicate the canonical writer or retain a second result
collection.

## Audited current consumers

- `LiteralMatcher` and `SearchProjection` are retained because content Search
  consumes exact literal tiering, Line boundaries, ordering, and duplicates.
  Gate 2 replaces the per-byte caller loop with one equivalent segment path;
  it does not add a second matcher.
- `StructuralCursor` is retained as the sole complete Line/Paragraph framer for
  content Search and prospective output that actually needs receipt or Anchor
  geometry. Gate 3 separates raw byte-state observation without creating a
  second structural parser.
- `CurrentObservation`, `CurrentProof`, and `SourceProofEvidence` are retained
  by Untrusted and Host View, Check, Apply, Search proof installation, and
  invalidation. Their Line-count comparison remains authoritative.
- `AnddressIssuer` is retained as the sole ordinary-address construction
  boundary for Search, View projection, prospective Apply receipts, and Anchor
  reflection. Gate 4 may validate its shared source once and each target
  geometry once; no capability-local constructor is permitted.
- Search's tier buckets and final sort are retained for deterministic global
  ordering. Gate 5 may replace only the monolithic provisional geometry store
  with storage that demonstrably releases consumed capacity.
- View batch source grouping is retained because it provides ordered,
  duplicate-preserving, all-or-nothing one-observation-per-source execution.
- Apply staging, prospective provenance, publication, Host-proof installation,
  and Anchor reflection are retained because unit Apply, Replace receipts, and
  live continuity consume their distinct failure and publication boundaries.
- Search and Apply currently duplicate Line-to-Paragraph attachment arithmetic.
  Gate 5 or 6 may move only that arithmetic behind one geometry-owned helper.
- CLI Search, View, Check, and Edit writers retain their Adapter schemas and
  exact bytes. Gate 4 may replace per-object allocation with the one reusable
  encoder buffer, not add a JSON model or writer.

## Ordered gates

### Gate 1 — authority — complete

Close the Line-count, typed-validation, and reusable-encoder decisions; pin
baselines, thresholds, exclusions, and release separation. No production,
version, test, README, server, or public-state change occurs.

### Gate 2 — bulk literal matching

Add one segment operation to the existing matcher and delete its per-byte
caller loop. Preserve literal semantics, tiering, all-or-nothing failure,
ordering, duplicates, UTF-8/NUL policy, and chunk/overflow boundaries. Measure
the fixed 256 MiB and 1 GiB sparse cells and the 1,048,576-hit native cell.

### Gate 3 — raw and structural observation

Separate raw UTF-8/NUL, SHA-256, checked length, Line-count accumulation, and
chunk delivery from the sole structural cursor. Route Check, ordinary/batch
View, Apply-before, and trusted exact-length staging through raw observation.
Retain structural observation only for proven geometry consumers. Measure Host
and Untrusted Check, self-Line View, 256 MiB Range Apply, CRLF Edit, and the
134-million-short-Line density cell.

### Gate 4 — issuance and encoding

Validate the shared source and target geometry at their single construction
boundaries, remove only proven typed-object revalidation, implement
`encode_into`, and reuse one Adapter scratch buffer. Preserve every KAT and
output byte. Measure one address, one million same-source Lines, one million
distinct-source Files, million-result CLI Search, and 200,000-file Search/View
batch.

### Gate 5 — chunked pending memory

Replace only the dense Search provisional store with a minimal chunked form
whose consumed storage is actually released, and centralize Paragraph
attachment. Preserve global indices, cross-chunk Paragraph promotion, result
tier/order/multiplicity, and all-or-none cleanup.

### Gate 6 — consumer reaudit and contraction

Reaudit production reachability after Gates 2–5, remove dead validation,
observation, and writer plumbing, and move Search and Apply to one Paragraph
attachment helper. Add no feature or generic framework.

### Gate 7 — fixed evidence and source readiness

Run complete GNU/musl semantics and crossed fixed A/B/C measurement. Record
source/binary revisions, fixture and harness hashes, CPU conditions, raw
samples, medians, p95, HWM, I/O, allocations, output hashes, and code-size
delta. Only a complete GO may advance Cargo, lockfile, README, version KAT, and
`bw version` to source-ready unpublished `0.2.5`. NO-GO leaves `0.2.4` current.

### Gate 8 — separately authorized release

Artifact reconstruction, installer allowlist, publisher, live publication,
endpoint/install/update verification, and release closure require a new exact
Owner authorization. Gate 7 source readiness does not authorize Gate 8.

## Fixed acceptance gates

- Sparse native Search uses the fixed 256 MiB and 1 GiB fixtures. C/A target is
  at most 1.10 and the allowed ceiling is 1.15. A result above 1.15 may activate
  only a measured optimization inside the sole cursor; it does not authorize a
  second parser.
- Dense Search uses exactly 1,048,576 hits. B peak RSS is 166,488 KiB; C target
  is at most 130 MiB, soft gate at most 140 MiB, and hard NO-GO above 145 MiB.
  Result count, order, multiplicity, and output digest must be exact.
- CRLF one-shot Edit C/A target is at most 1.20 and hard ceiling 1.25. It must
  retain zero private View/Search/Check calls, one `apply_replace`, every Line
  terminator, fresh receipt behavior, stale-old-address rejection, and zero
  Wrong Apply.
- Host Check proof hit has zero logical I/O, open, hash, and cursor work and
  retains its sub-microsecond class. Untrusted Check performs one open, one
  forward read, one UTF-8/NUL validation, one SHA-256, one byte count, and one
  Line-count accumulator per source, with zero cursor and within 10 percent of
  `0.2.3`.
- Host View remains project plus exact-range seek/read. Untrusted View uses one
  complete raw observation and range capture with no Paragraph, relation,
  event, or address-reconstruction work.
- A 256 MiB Range Apply has no before cursor; its after cursor runs at most once
  and only for receipt or live-Anchor geometry, within 10 percent of `0.2.3`.
- Encoder measurements record allocations/result, ns/result, output bytes, and
  peak scratch capacity. The canonical writer and all exact KAT bytes remain
  singular and unchanged.
- The 200,000-file Search and View-batch cells preserve exact ordering,
  duplicates, source grouping, output, and one accepted observation per source.
- The drift matrix remains Correct 1 / Safe Reject 6 / Wrong Apply 0. Stale,
  foreign, missing, unadmitted, invalid UTF-8, NUL, symlink, publication, and
  writer failure remain fail-closed.
- Final production target is no larger than 297,269 bytes and 8,954 lines.
  Growth up to 3 percent requires direct evidence; any duplicate parser,
  validator, or writer is a hard NO-GO.

Native Search and CLI Search are measured separately. The roughly 629 MB
million-result JSON stream is Adapter output volume, not native engine memory.

## Conditional decisions

`StructuralDemand`, cursor specialization, a shared Paragraph `Arc`, and the
pending chunk size are not Gate 1 implementation choices. Cursor work is
considered only if Gate 2 exceeds the 1.15 sparse ceiling. A shared Paragraph
allocation is considered only if chunking misses the dense target and must
improve both one huge Paragraph and many one-Line Paragraphs without a material
regression. Chunk size is selected from measured behavior, not fixed at 4,096
by this authority.

## Exclusions

No gate may add v6, remove or reinterpret a v5 field, alter Search/View/Edit
schemas or bytes, restore Search position/occurrence carriers, a View relation
scanner, or private Edit View, add a capability parser, persistent source
dictionary, state, index, registry, stdin transport, CLI split, history,
relocation, watcher, merge, retry, rollback, or compatibility path.

Gate 1 changes documentation only. Server, public root, services, cloudflared,
DNS, tunnel, credentials, actual HOME, artifact, release, and deployment state
remain outside the target until separately authorized.

## Gate 2 input

Gate 2 starts from B with one existing matcher. Its first proof is exhaustive
byte-at-a-time versus arbitrary-segment parity, including one-byte and
overlapping queries, every split point, Unicode, carried KMP partials, exact
and substring tiers, terminators, and 8,191/8,192/8,193-byte boundaries. It
then replaces the caller byte loop and records the fixed sparse and dense
native measurements. It must not modify structural cursor, wire, encoder,
proof, publication, or release authority.
