# Backwriter 0.2.5 Structural Specialization & Performance Recovery — Roadmap

Status: planning only. No `0.2.5` production authority, version, artifact, or
publication exists yet. This roadmap is grounded in the owner source note and
repository state at `1370ef29e702ee07125af396a96d3aadf5dd33c5`.

## Evidence labels

- **Owner note** means the requirements preserved in the companion
  [source note](2026-09-04-backwriter-0.2.5-structural-specialization-performance-recovery-source.md).
- **Repository** means current code or active documentation inspected at the
  revision above.
- **Decision required** means the owner note and current authority, or two
  requirements inside the owner note, cannot both be satisfied as written.

## Final objective

Keep the closed `0.2.4` v5 algebra and user-visible behavior while specializing
execution so each capability pays only for the byte or structural evidence it
actually consumes. Recover Search, Edit, View, and Check performance; reduce
dense Search peak memory and address-encoding allocation; retain one structural
parser and one Anddress issuer; then decide `0.2.5` source readiness from fixed
evidence. Release work remains a separate Gate 8 authority.

## Confirmed current state

**Repository:**

- branch `main`, `HEAD == origin/main == 1370ef29e702ee07125af396a96d3aadf5dd33c5`;
- package and `bw version` are closed public `0.2.4`;
- public v5 wire, `SourceIdentity`, target geometry, one `StructuralCursor`, one
  `AnddressIssuer`, single/batch View, and fresh Edit receipts are implemented;
- production baseline is 297,269 bytes and 8,954 lines;
- GNU and musl each have a 258-test release-closure result.

Hot-path evidence matches the note:

- `src/backwriter/search.rs` has byte-at-a-time
  `LiteralMatcher::{push,push_content_byte}` and no segment API;
- `src/runtime/search.rs::SearchProjection::segment` loops over every content
  byte and retains one `Vec<ProvisionalTarget>` until final issuance;
- `src/runtime/source_scan.rs::ObservationBuilder` owns UTF-8 state, SHA-256,
  and `StructuralCursor`; `observe_source()` delegates to
  `observe_structural()`, and `validate_source_exact()` also creates a cursor;
- `SourceProofEvidence`, `CurrentProof`, and `source_state_matches` include Line
  count; Check, View, and Apply consume that comparison;
- `AnddressIssuer::issue()` revalidates source plus geometry, and
  `Anddress::encode()` revalidates and creates a new `String`, decimal Strings,
  JSON field/value Strings, and a final byte vector;
- CLI Search/View/Edit writers call `Anddress::encode()` per item;
- Search and Apply duplicate Line-to-Paragraph attachment arithmetic;
- ordinary View and Apply-before paths already call `observe_source`, so a real
  raw observer will contract them without another execution layer.

The `0.2.4` fixed comparison uses published v4 A
`195aaa37068122097ecc04d2644642b6afcc6765` and v5 B
`8b20987893ea5ac454c4c0a50d0c470e26b5e650`; closed source authority is
`0ee4dcce14da93f925c27a04d0e79051c83fd124`. The new harness must label its
candidate explicitly and must not overload A/B names.

## Authority conflicts to close in Gate 1

### 1. `sourceLineCount` currentness

**Owner note:** retain `sourceLineCount` as v5 geometry but remove it from raw
source currentness and Host proof matching because hash plus exact length fixes
the bytes.

**Repository:** active authority defines Line count as part of `SourceIdentity`
and currentness. `runtime/check.rs` explicitly proves that a same-hash,
same-length address with a false Line count is `NotCurrent`.

**Internal conflict:** the note also requires that the diagnostic manual-v5
mutation matrix not reduce the existing safe-reject range. Removing the Line
count comparison makes a consistently forged in-bounds `sourceLineCount`
indistinguishable without deriving a current Line count.

**Decision required before implementation:** choose and document one of:

1. preserve the current safe-reject meaning and derive Line count in a cheaper
   non-hierarchy byte observer; or
2. explicitly narrow currentness to hash plus byte length, revise the mutation
   expectation and active authority, and accept that opaque exact objects—not
   manually altered geometry—are the supported boundary.

No Gate 3 code may silently choose between these contracts.

### 2. Typed-address validation

**Owner note:** treat typed `Anddress` as valid by construction and remove
repeated hot-path validation.

**Repository:** private fields and strict Issuer/decode construction support the
invariant, but active docs and structural tests say View/Check validate every
input before I/O.

**Decision required:** define whether “validate before I/O” means strict wire
construction plus typed invariant, or a repeated public `validate()` call at
every capability. Keep wire decode and explicit `Anddress::validate()` strict in
either case.

### 3. Reusable encoder API

**Repository:** `bw` is a separate binary crate, so a library-private
`encode_into` cannot be called by the Adapter. Avoid duplicating the canonical
writer in `bw.rs`.

**Decision required:** authorize one narrow public allocation-reusing encoder
surface, most likely `Anddress::encode_into(&mut Vec<u8>)`, while retaining
`encode()` as the existing compatibility surface. Its error and buffer behavior
must be explicit before Gate 4.

## Scope and non-goals

Included production paths are exactly those named in the source note, plus
tests and the active authority documents required by the gate being closed.
`src/runtime/structural_cursor.rs` is conditional on the sparse ratio after
Gate 2. Server deployment files are out of scope until a separately approved
Gate 8.

Do not introduce v6, remove v5 fields, change Search/View/Edit schemas or bytes,
restore retired carriers/scanners/private View, add a parser, persistent state,
index, registry, stdin, CLI split, history, relocation, watcher, merge, retry,
or a compatibility path.

## Gate plan

### Gate 1 — Performance-recovery authority

**Prerequisite:** resolve the three authority questions above.

**Work:**

- create one `0.2.5` tracker rather than modifying this source or roadmap into
  active authority;
- update AGENTS and the active current/protocol/address/principles/verification/
  CLI documents with the chosen source-state, typed-validation, and encoder
  contracts;
- record exact consumers and retention reasons for matcher, cursor, Issuer,
  batch grouping, publication/provenance, Host proof, and Anchor mechanics;
- pin the eight ordered gates, fixed baseline revisions, benchmark cells,
  acceptance thresholds, exclusions, and separate Gate 8 release authority;
- leave Rust, Cargo, tests, README, version, server, and public deployment
  byte-identical.

**Exit:** authority is internally consistent; unresolved choices are not hidden;
the repository still reports `0.2.4`; documentation checks pass.

### Gate 2 — Bulk literal matching

**Work:** add the segment matcher and route Search content segments through it;
apply only target-specific matcher saturation justified by exact semantics.
Delete the per-byte Adapter loop rather than layering a second matcher.

**Verification:** exhaustive byte/segment parity matrix; existing matching,
tier, order, duplicate, all-or-none, UTF-8/NUL, chunk-boundary, and overflow
tests; fixed 256 MiB, 1 GiB, and million-hit native cells.

**Exit:** candidate/published-v4 sparse ratio targets at most 1.10 and must not
exceed 1.15. If it exceeds 1.15, record the result and activate only the
conditional sole-cursor optimization; do not add another parser.

### Gate 3 — Byte and structural observation

**Work:** split the builder so raw observation has no `StructuralCursor`; route
Check, ordinary/batch View, Apply-before, and trusted staging to the raw path;
retain structural work only at proven geometry consumers. Apply the Gate 1
Line-count decision to proof/state matching and tests. Condition unit Apply's
after structural pass on receipt or live-Anchor demand.

**Verification:** raw/structural parity, late failure all-or-none behavior,
UTF-8/NUL and all terminators/chunk edges; Host Check zero I/O; Untrusted Check
one raw observation/source; View exact ranges without structural events; Apply
publication/proof/Anchor failure matrix. Measure the prescribed Check, View,
Range Apply, CRLF Edit, and short-Line-density cells.

### Gate 4 — Anddress issue and encoding hot paths

**Work:** separate source and geometry validation, validate shared source once,
remove only proven typed-object revalidation, implement the authorized reusable
canonical encoder, and reuse one Adapter scratch buffer. Preserve exact KAT and
all Adapter output bytes.

**Verification:** strict malformed/non-v5/invalid wire rejection; target and
parent geometry failures; one, million same-source Line, and million
distinct-source File encodes; Search/View/Edit writer equality; 200,000 File
Search and View batch. Record allocations/result, ns/result, output bytes, and
peak scratch capacity.

### Gate 5 — Dense pending-result memory

**Work:** introduce only the minimal chunked pending collection, progressively
consume/drop chunks during issuance, and centralize Paragraph attachment in the
geometry authority. Preserve tier buckets, global order, duplicates, all-or-none
failure, and multi-chunk Paragraph promotion.

**Verification:** exact million-hit count/order/digest and error cleanup; one
huge Paragraph and many one-Line Paragraphs; dense peak target ≤130 MiB, soft
gate ≤140 MiB, hard NO-GO >145 MiB. Test a shared Paragraph `Arc` only if
chunking misses the target, and reject it if either shape regresses materially.

### Gate 6 — Remaining consumer contraction

**Work:** audit production reachability after Gates 2–5; move Search and Apply
to the single Paragraph attachment helper; remove dead validation, observation,
and writer plumbing; retain every path with a real consumer. Do not invent a
new Gate-6 feature or abstraction.

**Exit:** exactly one cursor, one Issuer, zero retired wrappers/relation finder/
private Edit View, and no duplicate parser/validator/writer. Production target
is no larger than 297,269 bytes/8,954 lines; growth up to 3% requires direct
evidence, while any duplicate authority is NO-GO.

### Gate 7 — Integrated evidence and source readiness

**Prerequisite:** clean commits for each preceding gate and reproducible fixed
harnesses.

**Work:** run full GNU/musl verification and a crossed fixed comparison among
published v4, closed `0.2.4`, and the candidate. Record raw samples, source and
binary revisions, fixture/harness hashes, CPU conditions, medians, p95, HWM,
I/O, allocations, output hashes, and code-size delta. Native and CLI Search are
separate measurements.

**Mandatory gates:** sparse, dense-memory, Edit, Check, View, Range Apply,
encoder, 200,000-file/batch, semantic equality, and Correct 1 / Safe Reject 6 /
Wrong Apply 0 all pass. Re-run stale/foreign/missing/unadmitted/UTF-8/NUL/
symlink/publication/writer fail-closure and the opaque-v5 mutation diagnostic.

**Decision:** only a complete GO advances Cargo, lockfile, README, version KAT,
and `bw version` to source-ready unpublished `0.2.5`. NO-GO keeps `0.2.4` and
records the failed cells without release claims.

### Gate 8 — Release closure

Gate 8 requires a new explicit owner authorization. It covers server-owned
artifact reconstruction, installer allowlist, publisher, live publication,
endpoint/install/update verification, closure docs, and release state. It is
not implied by source-readiness GO and must not run Actions, `gh`, tags, GitHub
Release, or crates.io unless separately authorized.

## Cross-gate verification

Every implementation gate runs the relevant focused tests plus offline/locked
metadata and dependency tree, formatting, all-target checks, full tests when
production behavior changes, clippy with warnings denied, and release build.
Keep task-local harnesses and artifacts outside the repository and remove them
after evidence is recorded. Confirm `.artext`, tracked build output, unexpected
untracked files, and staged paths. Preserve unrelated user changes and leave the
index empty unless the owner explicitly authorizes staging.

## Principal risks

- A performance-only patch can accidentally redefine source currentness through
  the Line-count conflict.
- Removing typed validation without closing the public invariant can turn a
  structural test deletion into an undocumented API change.
- A public reusable encoder surface expands API even though bytes are unchanged.
- Chunking can preserve total live capacity or break Paragraph promotion if the
  consumed storage is not actually released.
- Shared parent allocation can help one huge Paragraph while harming many small
  Paragraphs.
- CLI output volume can obscure native improvements; the two layers must remain
  separately measured.
- Recreated benchmark harnesses can drift from `0.2.4`; revision, fixture,
  environment, raw-sample, and digest evidence is mandatory.

## Roadmap completion condition

The roadmap is complete only when the owner-approved Gate 1 decisions are
recorded, Gates 2–6 have attributable evidence, Gate 7 issues an explicit
GO/NO-GO without weakening v5 semantics or fail-closure, and any Gate 8 work is
performed under separate release authority. Until then, `0.2.4` remains the
only current source and official distribution.
