# Backwriter 0.2.5 Structural Specialization and Performance Recovery

Status: Gates 1 and 2 complete. Bulk literal matching is implemented and
verified. Cargo, `bw version`, artifacts, installers, Update, and the public
distribution remain published and closed `0.2.4`.

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
- **C** is the Gate 2 candidate built from
  `0b7fbbd9d06c0f2417374d428089232704c49b8b` plus the exact Gate 2 Search
  diff. A future comparison must not relabel A or B.

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

### Gate 2 — bulk literal matching — complete

Add one segment operation to the existing matcher and delete its per-byte
caller loop. Preserve literal semantics, tiering, all-or-nothing failure,
ordering, duplicates, UTF-8/NUL policy, and chunk/overflow boundaries. Measure
the fixed 256 MiB and 1 GiB sparse cells and the 1,048,576-hit native cell.

One `LiteralMatcher::push_segment` now owns checked bulk Line-length accounting,
zero-state first-byte skipping, carried KMP partial state, and stop-after-match.
The old `push`/`push_content_byte` pair and Runtime byte loop are deleted.
`SearchProjection` alone decides File/Paragraph tier saturation; the existing
cursor, source observation, provisional store, Issuer, tier buckets, and final
sort remain because their structural, failure, ownership, and ordering
consumers are unchanged. No `StructuralDemand`, cursor mode, parser, result
collection, or error type is added.

Exhaustive binary-alphabet tests compare byte-at-a-time, every possible segment
partition, and whole-segment matching. Focused controls cover absent/terminal
first bytes, one-byte and overlapping queries, `abab`/`ababa`, prefix/suffix
substrings, Unicode splits, cross-chunk partials, a 65,536-byte matched suffix,
all terminators, every target, 8,191/8,192/8,193-byte edges, checked overflow,
and late invalid source disposal. GNU and musl each pass 258 tests plus
all-target check, clippy `-D warnings`, and release build.

#### Fixed native evidence

Clean Git-object exports use A and B above. C uses the same base plus only
`src/backwriter/search.rs` and `src/runtime/search.rs` from the candidate. The
fixed fixture recipes are one Line of `x` followed by `needle\n` at exactly
256 MiB and 1 GiB, and exactly 1,048,576 copies of `needle\n`. Their SHA-256
values are respectively
`641f7442659ee50a6c5e183fd0a95963deaa21490ac2884215639ea704614d9e`,
`904c75499d4dc222f3df76ad0c2dcc397e0a163b56ed5c65692f65de7d67a162`,
and `913515a8747b7f1bf66a0e60d4f7d62aee87266faeffbf1aa60509d478c86b8c`.

The A and B/C native harness source SHA-256 values are
`84faa44b5a1605c0078c760c514d2df654cf43a92036ba2f45da2a246c7cf2a8`
and `65b796a3c7c63d0b70a78c02cccdf860c5c69ddaee2c9919c74d98f8fda64e7d`.
They differ only at the incompatible A occurrence carrier versus B/C direct
Anddress collection. The fixture generator, measurement orchestrator, C
`CLOCK_MONOTONIC_RAW` runner source, and compiled runner SHA-256 values are
`2769b13a75c07e92208ba9d2ad78a36509c294adf31e66b07651827b06e22868`,
`0902b126661704bf048eebd5c3b4dfc16c348a36fdb52b754face63bd1d6dee7`,
`46ca9b3191485898ff8936806c70bbcbd867305695c977ab1377b233f5f1a4e5`,
and `7b0b8e2f25cdd2883034694114a3b403233678358737236e736595d5eacb8b2a`.
A/B/C harness binary SHA-256 values are
`39cde5c87ca6c13b726b6d13fb6f198e845c151a3c7bec945aec7fe3d468fd8f`,
`8391816ad24bd6f2d0a4318a02e0f864b574f650f1737c6549c39f56711c461d`,
and `0d93d7ff5ff3eaf36555607d3493e1f6ef7d29df6d2dd0b9e8edcfe0ff811f2c`.

The host is Linux `7.2.2-arch1-1` on an Intel Core i7-12700K with Rust/Cargo
1.95.0, LLVM 22.1.2, CPU 0, the existing `powersave` governor, and `/tmp`
tmpfs. Each variant receives one warm-up and seven fresh-process samples in
crossed orders `ABC/CBA/BCA/ACB/CAB/BAC/ABC`. Inner time covers native
`WorkspaceRuntime::search`; the retained result is then traversed once without
a second collection to compute semantic and canonical-wire digests. HWM and
process I/O include that verification traversal. Nearest-rank p95 over seven
samples is the maximum.

| Fixture | Results | A median/p95 ms | B median/p95 ms | C median/p95 ms | A/B/C peak HWM KiB | C/A median/p95 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 256 MiB sparse | 1 | 265.839/284.172 | 379.335/380.655 | 302.766/304.684 | 2,324/2,308/2,316 | 1.1389/1.0722 |
| 1 GiB sparse | 1 | 1,046.983/1,057.311 | 1,495.575/1,505.156 | 1,192.754/1,196.308 | 2,360/2,360/2,332 | 1.1392/1.1315 |
| 1,048,576 Lines | 1,048,576 | 43.362/44.197 | 127.498/142.103 | 132.171/134.278 | 108,988/166,332/166,136 | 3.0480/3.0382 |

Raw inner milliseconds and HWM KiB, in each variant's execution-round order:

| Fixture / variant | Raw inner ms | Raw HWM KiB | `rchar` / `wchar` |
| --- | --- | --- | --- |
| 256 MiB A | 265.839, 266.722, 265.096, 265.368, 265.208, 284.172, 265.913 | 2040, 2180, 2244, 2324, 2168, 2160, 2160 | 268442042 / 188 |
| 256 MiB B | 380.655, 379.675, 376.356, 379.277, 378.473, 379.448, 379.335 | 2152, 2220, 2144, 2308, 2296, 2200, 2056 | 268441993 / 188 |
| 256 MiB C | 301.954, 301.651, 304.381, 304.608, 300.892, 304.684, 302.766 | 2068, 2316, 2172, 2300, 2172, 2312, 2248 | 268441993 / 188 |
| 1 GiB A | 1047.804, 1047.751, 1040.596, 1057.311, 1045.615, 1046.983, 1044.883 | 2324, 2360, 2252, 2152, 2308, 2316, 2160 | 1073748410 / 189 |
| 1 GiB B | 1496.897, 1491.814, 1505.156, 1495.575, 1495.139, 1494.941, 1498.056 | 2320, 2360, 2328, 2100, 2160, 2216, 2172 | 1073748361 / 189 |
| 1 GiB C | 1196.308, 1192.754, 1186.904, 1194.166, 1194.676, 1188.533, 1191.303 | 2072, 2300, 2332, 2332, 2120, 2172, 2152 | 1073748361 / 189 |
| Dense A | 44.197, 42.992, 43.303, 43.362, 43.556, 43.565, 43.142 | 108756, 108900, 108720, 108904, 108988, 108976, 108852 | 7346618 / 199 |
| Dense B | 126.119, 127.498, 127.711, 125.339, 127.909, 125.707, 142.103 | 166256, 166132, 166108, 166248, 166128, 166264, 166332 | 7346569 / 200 |
| Dense C | 132.171, 131.479, 130.224, 132.932, 134.278, 131.517, 133.520 | 166104, 166004, 166124, 166136, 165968, 166136, 166136 | 7346569 / 200 |

All A/B/C semantic order digests match per fixture:
`7c1543a3dd75740c7e69fc7dd3ea3687894a843cb3e22aa6d1cce4aa54c92e43`,
`38d02b0d6f9556df1f1ade5a30b6cb70cc69736740bc30fd3ef6a257b30fde9d`,
and `9aa0320348d16abc85e47b9533a2e59480b3d6ca3e3bf7a52e3ba00c0caac690`.
B/C canonical-wire digests match at
`55c2cb1d7b2bbe23b5d2b05f71452060bad8ac8c691a1058bc120981e4af8639`,
`fef9fa3fc86aa6d18bf9a34f1aa84fa8589f09df0074b3cef67a0186ff143a96`,
and `b547d3fbd6a59fc54789e25d41de6eb12fa5823a68f6818035417718f3044063`.
The three raw CSV SHA-256 values are
`33976cbfe73e87f97020b83332d881e8cd3d104e1b506f078179efeb425179e9`,
`ee544b2434621a409289b5949b59252d5d2e3988f24d203b5171987cde7644c0`,
and `27959f35dbaa58af979c500e4467efbcc0284a11b87536a754b66d5757c0c84f`.

The sparse median target of 1.10 is not met, but both cells pass the fixed 1.15
ceiling and every sparse p95 ratio also passes. Gate 2 is therefore **GO**
without conditional cursor specialization. Dense memory remains deliberately
unchanged and exceeds the later Gate 5 hard threshold; it is recorded as Gate
5 input rather than misclassified as a Gate 2 regression. Production is
298,222 bytes/8,981 lines, +953 bytes/+27 lines (0.32%/0.30%) over B. The growth
is one matcher operation and target saturation, below the allowed 3 percent
with direct semantic and measured evidence; Gate 6 must still contract the
final target to its fixed bound.

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

## Gate 3 input

Gate 3 starts from the completed segment matcher with no conditional cursor
mode. It separates raw byte-state observation from the sole structural cursor,
retains Line-count currentness with a minimal same-read accumulator, and routes
only proven raw consumers away from Paragraph and parent geometry. It must
preserve one read, UTF-8/NUL and checked-length failure boundaries, Host proof,
publication, Anchor, and v5 wire authority while measuring the fixed Check,
View, Range Apply, CRLF Edit, and short-Line-density cells.
