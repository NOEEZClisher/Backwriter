# Backwriter 0.2.5 Structural Specialization & Performance Recovery — Source Note

Status: owner-provided planning source captured on 2026-09-04. This file preserves
the requested direction; it is task evidence, not active authority. Active
authority changes only through an approved gate.

## Goal and governing sentence

Preserve the complete `0.2.4` v5 structural model while recovering the execution
specialization and performance lost during its consolidation.

> Semantics stay unified. Execution becomes specialized again.

The shorter release definition is:

> Unified authority, specialized hot paths.

Required recovery areas:

- restore the Search-only literal fast path;
- remove `StructuralCursor` work from observations that need only bytes;
- reduce dense-Search pending-result peak memory;
- remove repeated Anddress issuance, validation, and encoding work;
- recover the `0.2.4` Search, Edit, View, and Check regressions.

## Invariants and prohibited rollback

Keep all of the following:

- `artext.backwriter-anddress.v5`;
- `SourceIdentity` and structural target geometry;
- File/Paragraph/Line parent relationships;
- exact Line terminator and `line_count`/`line_number`/`line_range`;
- `Anddress::parent()` and `Anddress::project()`;
- single and batch View;
- fresh Edit receipts;
- exactly one structural parser (`StructuralCursor`);
- exactly one construction authority (`AnddressIssuer`).

Do not restore `SearchPosition`, `SearchOccurrence`, View relation scanners,
private Edit View, capability-specific Line/Paragraph parsers, or a parallel v4
path. `0.2.5` is not a v5 rollback and does not reduce structural authority.

## Diagnosed regression sources

### A. Sparse Search CPU

`LiteralMatcher` is currently fed one content byte at a time. The requested
recovery is a segment API that skips spans without the query's first byte,
continues an existing KMP partial match across chunks, and stops matcher work
after a Line match. File and Paragraph searches may additionally stop matcher
work once their best possible tier is fixed, while hashing and required
structure processing continue.

### B. Structural work in every observation

The current observation builder combines UTF-8/NUL validation, SHA-256, byte
length, and `StructuralCursor`. Check, ordinary View, and Apply-before staging
therefore pay for Line/Paragraph framing even when they require only byte state.

### C. Dense Search memory

One source currently holds a monolithic provisional geometry vector and then a
final Anddress vector. During conversion, a substantial part of both is live.

### D. Dense Search serialization

Each Anddress encoding repeats full validation and allocates field-name,
field-value, decimal, and result buffers. Million-result output multiplies this
cost.

## Ordered gates

1. Define `0.2.5` performance-recovery authority.
2. Restore bulk literal matching.
3. Split byte-state and structural observation.
4. Contract Anddress issue and encoding hot paths.
5. Bound dense Search pending-result memory.
6. Finish remaining algebra consumers and remove residual duplication.
7. Run the integrated benchmark and decide source readiness.
8. Perform separately authorized release closure.

Measure the gate-specific core cells after every implementation gate so the
effect of each change remains attributable.

## Gate 2 requirements — bulk literal matching

Primary paths:

- `src/backwriter/search.rs`
- `src/runtime/search.rs`

Add a segment-level matcher operation equivalent in responsibility to:

```rust
LiteralMatcher::push_segment(bytes: &[u8])
```

Required behavior:

- account for full-Line length in checked bulk arithmetic;
- return immediately for empty input or an already found match;
- when KMP state is zero, jump to the next occurrence of the query's first byte;
- when a partial match exists, continue normal KMP processing across chunks;
- stop matching the remaining segment after a complete match;
- preserve the existing Resource/error boundary on overflow.

`SearchProjection` must call the segment API rather than inspect matcher state.
File Search stops matching after `FullLine`; Paragraph Search may stop until the
current Paragraph closes after `FullLine`; Line Search resets per Line and skips
only the matched Line suffix. Required parity fixtures include missing/terminal
first bytes, cross-chunk partials, `abab`/`ababa`, one-byte queries, exact and
substring Lines, long suffixes, Unicode, every split point, and 8,191/8,192/8,193
boundaries. Byte-at-a-time, arbitrarily chunked, and one-segment execution must
produce identical results.

## Gate 3 requirements — observation specialization

Primary paths:

- `src/runtime/source_scan.rs`
- `src/runtime.rs`
- `src/runtime/check.rs`
- `src/runtime/view.rs`
- `src/runtime/apply.rs`

Split the responsibilities into a raw byte-state builder and a structural
builder. Names are candidates, not fixed API:

```rust
struct SourceObservation { hash: String, byte_length: usize }
struct StructuralObservation { hash: String, byte_length: usize, line_count: usize }
```

The raw builder owns incremental UTF-8/NUL validation, SHA-256, checked byte
length, and a caller chunk callback. It must not contain or invoke
`StructuralCursor`. The structural builder composes the raw work with the sole
cursor and Line/Paragraph events. `observe_source()` must not delegate to
`observe_structural()`. `validate_source_exact()` is raw exact-length validation.

Structural observation remains required for content Search, exact File Search,
prospective Apply output when fresh geometry is required, and any Anchor boundary
proven by reachability to need geometry. Raw observation is required for
Untrusted Check fallback, ordinary/batch View proof miss, Apply before-source
staging, trusted exact-length staging, and current hash/length validation.

`sourceLineCount` remains a v5 field and a derived structural fact for File
Line count, positions, geometry validation, Adapter display, and fresh issuance.
Current source-byte authority is workspace/path plus SHA-256 and exact byte
length; Line count is not a second independently recomputed currentness proof.

Remove `line_count` from `SourceProofEvidence`, `CurrentProof`, and proof matching
and installation signatures. Check compares hash and byte length only, retains
input order/duplicates and Current/NotCurrent/Unavailable, preserves proof and
Anchor state, keeps Host hits at zero I/O, and uses one raw observation per
source on an Untrusted/proof-miss path.

Ordinary View proof misses use one raw observation plus projected-range capture.
Do not restore Paragraph, target, or relation scanners. Apply before staging is
raw. Prospective after output is structural only if a receipt or same-path live
Anchor needs fresh geometry; unit Apply with neither may use raw after-state
observation. Preserve atomic publication, fixed staging, prospective hash, proof
installation, Anchor reflection, and `PublicationUncertain` fail-closure.

## Gate 4 requirements — issuance and encoding

Primary paths:

- `src/backwriter/anddress.rs`
- `src/bin/bw.rs`

Treat typed `Anddress` as valid by construction. `AnddressIssuer::new()` validates
the shared source once; `issue()` validates only target geometry. Keep strict
full validation at wire decode and explicit public validation boundaries.
Remove or debug-assert redundant typed-object revalidation in encoding, View,
Check, Edit, and Anchor hot paths only after proving their construction boundary.

Provide an allocation-reusing canonical encoder, with `encode_into(&mut Vec<u8>)`
as the candidate shape. `encode()` may allocate once and delegate. Fixed keys and
validated fixed values write as bytes; only `logicalPath` needs JSON escaping;
unsigned decimals use a stack buffer. Per-result temporary Strings and a fresh
address buffer are prohibited. Exact v5 KAT bytes must remain unchanged.

Search JSON uses one reusable address scratch buffer. Reuse in View/Edit writers
is allowed when it removes the same proven duplication. Keep `bw.cli.search.v2`,
its key order, duplicated Adapter metadata, canonical nested v5 object, and exact
`0.2.4` output bytes. Search-envelope compaction is a later Adapter decision.

Measure one address, one million same-source Line addresses, and one million
distinct-source File addresses: allocations/result, ns/result, output bytes, and
peak scratch capacity. The address buffer changes from one million allocations
to one reusable buffer; field temporary String allocations are zero.

## Gate 5 requirements — dense pending memory

Primary paths:

- `src/runtime/search.rs`
- `src/backwriter/anddress.rs`
- `src/runtime/apply.rs`

Replace the monolithic provisional vector with chunked pending storage. A chunk
size of 4,096 is an example, not authority. The store must support global result
indices and mutable ranges spanning chunks so Paragraph-close promotion remains
exact. After SourceIdentity/Issuer completion, consume one chunk, issue its
Anddresses into final tier buckets, drop that chunk, and continue. Do not retain
the complete capacities of both pending and final collections to the end.

Move the duplicate Search/Apply Line-to-Paragraph attachment arithmetic behind
one geometry-owned helper. It alone checks Line kind, Paragraph containment,
offset subtraction, parent assignment, and invariants.

Changing `ParentGeometry::Paragraph` to shared `Arc` storage is conditional.
Consider it only after chunking remains insufficient, and accept it only if both
one Paragraph with one million Lines and one million one-Line Paragraphs improve
without pathological allocation overhead.

## Conditional structural optimization

After the bulk matcher, optimize `src/runtime/structural_cursor.rs` only if sparse
Search still exceeds the allowed ratio. The sole cursor may combine delimiter
detection/body classification, skip Paragraph body classification for File
Search, or accept a measured structural demand (`LinesOnly` versus
`FullHierarchy`). Do not introduce a second Search parser, Apply terminator
parser, or capability-owned structural parser.

## Fixed acceptance and benchmark authority

Use one fixed A/B harness and report median, p95, peak HWM, I/O, exact output
digest, binary/source revision, fixture construction, and raw samples.

### Dense Search

- fixture: exactly 1,048,576 hits;
- `0.2.4` peak: 166,488 KiB;
- target: at most 130 MiB;
- soft gate: at most 140 MiB;
- hard NO-GO: more than 145 MiB;
- exact result count, order, multiplicity, and output digest; zero wrong or
  partial results.

Output byte count is tracked separately because v5 wire is unchanged.

### Sparse Search

- fixed 256 MiB and 1 GiB sparse fixtures;
- target B/A ratio at most 1.10;
- allowed ceiling at most 1.15;
- above 1.15 triggers the conditional sole-cursor optimization.

### Edit, Check, View, and Apply

- CRLF one-shot Edit target B/A at most 1.20; hard ceiling 1.25; preserve no
  private View/Search/Check, one `apply_replace`, all terminators, fresh receipt,
  stale old address, and zero Wrong Apply;
- Host Check proof hit: zero logical I/O, open, hash, and cursor, retaining the
  sub-microsecond class;
- Untrusted Check: one open, one forward read, one UTF-8/NUL validation, one
  SHA-256, one byte count, zero cursor, within ±10% of `0.2.3`;
- Host View: project plus exact-range seek/read only;
- Untrusted View: one complete raw observation plus range capture, with zero
  Paragraph/relation/event/address-reconstruction work;
- 256 MiB Range Apply: no before cursor; after cursor at most once and only for
  receipt/Anchor geometry, within ±10% of `0.2.3`.

Separate native Search measurement (matching, framing, pending, issuance,
sorting) from CLI Search (native work plus Adapter metadata, v5 encoding, and
stdout). Do not classify the roughly 629 MB million-result JSON stream as native
engine memory.

Minimum gate measurements:

- after Gate 2: 256 MiB/1 GiB sparse and 4 MiB million-hit native Search;
- after Gate 3: Untrusted/Host Check, self-Line View, 256 MiB Range Apply, CRLF
  one-shot Edit, and 134M short-Line density;
- after Gate 4: million-address encode, million-result CLI Search, 200,000 File
  Search, and 200,000-item View batch;
- after Gate 5: million-hit RSS, one huge Paragraph, and many one-Line Paragraphs.

## Safety, compatibility, and code-size gates

Preserve the drift matrix exactly: Correct Apply 1, Safe Reject 6, Wrong Apply 0.
Rerun stale old address, foreign workspace, missing/unadmitted source, invalid
UTF-8, NUL, symlink, publication uncertainty, and writer-failure fail-closure.

The opaque exact Search/View receipt flow remains the supported v5 mutation
boundary. Manual but internally consistent mutation of terminator, parent
geometry, or `sourceLineCount` remains a separate security/transport decision;
record only a diagnostic mutation matrix and do not broaden this patch.

Production baseline is 297,269 bytes and 8,954 lines. Target no growth over
`0.2.4`; growth up to 3% requires a direct explanation. Any new duplicate parser,
validator, or writer is a hard NO-GO. Final production retains one
`StructuralCursor`, one `AnddressIssuer`, and zero Search position/occurrence
wrappers, View relation finder, and private Edit View.

## Explicit exclusions

Do not add Anddress v6, remove v5 fields, compact Search JSON, add a persistent
source dictionary, result registry, index binding, stdin Content, CLI file split,
history, relocation, watcher, merge, retry, or a compatibility path.

## Files named by the owner note

- `src/backwriter/search.rs`
- `src/runtime/search.rs`
- `src/runtime/source_scan.rs`
- `src/runtime.rs`
- `src/runtime/check.rs`
- `src/runtime/view.rs`
- `src/runtime/apply.rs`
- `src/backwriter/anddress.rs`
- `src/bin/bw.rs`
- conditionally, and only after measurement: `src/runtime/structural_cursor.rs`

Tests and active authority documents must be updated by the gate that changes
their corresponding contract. Gate 8 release mechanics remain separately
authorized work.
