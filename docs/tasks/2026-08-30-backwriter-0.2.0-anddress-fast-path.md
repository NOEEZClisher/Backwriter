# Backwriter 0.2.0 Anddress Fast Path

Status: Phases 1–6 completed; Phase 7 next.

This is the sole progress tracker for the redesign. It records gates and
evidence but does not own semantics; the active Protocol, address model, and
principles do. Historical task evidence never overrides active authority.

## Goal and Owner intent

Keep the closed public `0.1.0` v3 release immutable while developing an
unpublished `0.2.0` v4 exact-source-state fast path. Search alone finds targets.
View, Check, and Apply consume an ordinary Anddress without searching,
reparsing to relocate, or context-matching an old target. Runtime may hold only
bounded call-local current observation state, never cross-call state or history.

## V3 problem and drift-Wrong-Apply reproduction

V3 addresses Paragraphs by current ordinal and Lines by current ordinal plus
exact Line text/terminator; File identity is unchanged by source content. With
duplicate text, an external rewrite can leave the old ordinal and exact extent
valid at a different occurrence:

1. Source A is `header\nneedle\nneedle\nfooter\n`.
2. Search selects the second `needle\n`, v3 Line ordinal 2.
3. An external writer produces
   `needle\nheader\nneedle\nneedle\nfooter\n`.
4. Ordinal 2 still has exact extent `needle\n`, but is the former first
   occurrence; the selected occurrence moved to ordinal 3.
5. Resolving the old v3 locator can Apply to the wrong occurrence.

V4 fails this case before range use because the complete source state changed.
It never searches for a plausible replacement occurrence.

## Target authority

An ordinary `artext.backwriter-anddress.v4` contains exactly:

- Runtime workspace coordinate;
- canonical logical path;
- authoritative complete source-state hash;
- exact source byte length;
- File, Paragraph, or Line kind;
- inclusive-start/exclusive-end byte range `[start, end)`.

File covers the complete source; Paragraph and Line cover exact current bytes.
Target text, terminator text, ordinal, and neighboring context are not identity.
The source-state hash is final currentness authority. Phase 3 fixed SHA-256,
the exact eight-field compact JSON wire order, and an incompatible hard cutover
with no v3 production decoder or compatibility surface.

An ordinary Anddress is immutable caller-owned authority for one exact source
state and range. A changed state invalidates it. Explicit Search may return a
new current Anddress; no consumer relocates the old one. Reappearance of the
same complete state may reproduce raw equality without proving continuity.

Anchor is the sole continuity exception. Only a live Runtime-local Anchor may
receive an arithmetic range transform across a successful Backwriter-owned
Apply. External or opaque source change invalidates continuity. Anchor does not
mutate an ordinary Anddress or add history, search, or context relocation.

`CurrentObservation` is Runtime-private producer state for one call-local
source. It contains only the current hash and exact length. Search owns its
target-required provisional ranges separately, consumes both states immediately
after success, and discards both on failure before opening another source.
Ordinary View adds only returned-range and optional Line-relation state; Check
adds no target projection. Each consumes or discards the observation before
return. It may not retain prior observations, whole-source bytes, a parse tree,
a complete Line collection, Search results, history, a persistent index,
relocation/context evidence, a full workspace cache, a watcher, or durability.

Capability responsibilities are fixed: Search hashes while discovering ranges
in its one source read and performs no separate hash pass; View validates hash
and length while returning exact caller-range bytes; Check compares only source
hash and length; Apply
requires a matching hash and validates the range against the recorded length
before patching. View, Check, and Apply do not search.

## Phase gates

| Phase | Entry gate | Completion gate |
| --- | --- | --- |
| 1. Authority | Closed clean `0.1.0` baselines, Owner docs-only authority, active-doc review. | This tracker and active semantics cover every guard, field, responsibility, exclusion, test, and benchmark gate; protected code/runtime state is unchanged; hash and compatibility remain open. |
| 2. Reproduce/profile/baseline | Phase 1 committed, pushed, and clean. | Drift-Wrong-Apply is executable; release-build profiles locate actual parse/hash/allocation/I/O cost; fixed fixtures, commands, host/toolchain facts, repeated raw results, and variance are recorded without improvement claims. |
| 3. V4 value/wire kernel | Phase 2 evidence plus Owner decisions for hash algorithm, compatibility/cutover, and any dependency. | One canonical v4 value implements validation, equality, encoding/decoding, checked arbitrary-size length/range arithmetic, error priority, KATs, and decided cutover without hidden ordinal/text identity. |
| 4. Search/observation | Phase 3 v4 Search production is complete and all retained-state consumers are mapped. | The bounded observation producer and discard rules hold without changing Phase 3 one-read hash/range results, ordering, multiplicity, fail-all, Unicode/terminators, admission, or no-limit behavior. |
| 5. View/Check | Search produces accepted v4 values. | View validates hash/length while copying its range without target search/reparse; Check compares only source hash/length without search/refresh; raw nonstructural ranges, duplicate, rewrite, bounds, unavailable, text-policy, and resource regressions pass. |
| 6. Apply/Anchor | Ordinary View and Check consume v4 authority. | Apply enforces the hash precondition and range bounds before patching, and Wrong Apply fails without publication; publication/safety/resource boundaries remain; only Anchor transforms a live range under Backwriter-owned Apply; external change invalidates continuity. |
| 7. Integrate/release decision | Phases 3–6 individually green. | Full matrix and fixed benchmarks pass; structural audits exclude consumer search, second Search hash pass, history/index/relocation/context/cache; docs report actual evidence; version, compatibility, artifacts, and publication receive separate Owner decisions. |

## Required test matrix

| Area | Required evidence |
| --- | --- |
| V4 value/wire | Exact decided fields/order; every kind; empty File; zero/nonzero and arbitrary-size offsets/length; reversed/out-of-bounds range; malformed/duplicate/missing fields; error priority and KATs. |
| Source state | Same/different-length rewrite, mutation outside range, truncate/grow, A→B invalidation, exact A reappearance without continuity, replacement, missing/nonregular/symlink, UTF-8/NUL, I/O/resource failure. |
| Drift safety | Canonical duplicate-Line Wrong Apply, duplicate Paragraphs, ordinal drift, equal text at another range, similar context; stale consumers fail closed with no publication. |
| Search | Content and exact File; all kinds/ranges; CR/LF/CRLF/no-EOL, Unicode, empty/separator Lines, duplicates/order; one-read hash integration, no hash replay, late failure discard. |
| View/Check | Exact range bytes; changes inside/outside range; hash/length/bounds mismatch; Current/NotCurrent/Unavailable batch order and multiplicity; no search, refresh, or relocation. |
| Apply/Anchor | Every Edit range geometry; stale precondition/no wrong publication; no-op, race, cleanup, resource and uncertain publication; Anchor transform/collision/invalidation and no ordinary-address mutation. |
| CurrentObservation | Hash and exact length only; capability-owned minimum projection state; consume/discard on success/failure; structural absence of cross-call state, history, whole source, parse tree, result store, index, or full workspace cache. |
| Regression | Admission/no-follow/private path, no fixed semantic limit, Search determinism, Pick purity, Data explicitness, and existing public API/error/CLI boundaries until separately authorized. |

## Benchmark baseline and goals

Phase 2 records v3 before implementation. Required conditions are one pinned
revision, release build, toolchain, target, host, CPU/power state, filesystem,
and fixture set; separate cold/warm runs; enough samples for median, p95,
spread, and outliers; wall/CPU time, peak RSS, bytes read, available allocation
metrics, and profile evidence; identical validated output for v3 and v4; and
fixtures covering small/large source, one very long Line, many Lines, duplicate
targets, Unicode/terminators, large results, and stale addresses.

Mandatory structural gates are no second Search hash pass, no consumer target
search/relocation, no full-source `CurrentObservation`, no fixed-input
truncation, and no output/error regression. Recommended—not yet measured or
claimed—goals are:

- large-source View/Check median CPU at or below 75% of v3;
- range Apply prepublication median CPU at or below 75% of v3;
- Search median wall time and peak RSS at or below 105% of v3;
- no p95 or peak auxiliary-memory regression above 10% without Owner review.

Missing a recommendation never permits weaker semantics; it requires evidence
and an Owner decision before release closure.

## Phase 2 v3 reproduction and profile baseline

Phase 2 ran on revision `399805906b352f1c8d0cc2fa0bbe6dee1a73a13c`
without changing Rust, Cargo, tests, or production behavior. The timing binary
was the ordinary stripped release build, SHA-256
`f7b0aaea704561b6842f35778991344a1d5c1fc5a3aed464df27b526dc26db9d`.
The separate sampling binary used release optimization plus debug information,
no stripping, and frame pointers, SHA-256
`af40b6c0df0cf78f73cbce6b2c67812ce8ca51abfd70540809e585a2d4043467`.
Profiler-build measurements are evidence about stacks and counters, not timing
baselines.

The host was Linux `7.1.11-arch1-1` x86_64 on one Intel Core i7-12700K
(20 logical CPUs, one NUMA node), Rust/Cargo 1.95.0 and LLVM 22.1.2. All
measurements were pinned to P-core logical CPU 8. The existing governor was
`powersave`, affinity availability was `0-19`, and no power or kernel setting
was changed. The repository was on ext4 and task-local fixtures and output were
on tmpfs. GNU time 1.10 and perf 7.2.2-1 were installed as host tools; perf ran
with `perf_event_paranoid=2` and collected user-space core counters and DWARF
call chains without lost samples.

The fresh release build was:

```sh
CARGO_TARGET_DIR="$ROOT/target-release" \
  cargo build --offline --locked --release
```

The distinct profiler build was:

```sh
CARGO_TARGET_DIR="$ROOT/target-profile" \
  CARGO_PROFILE_RELEASE_STRIP=false CARGO_PROFILE_RELEASE_DEBUG=1 \
  RUSTFLAGS='-C force-frame-pointers=yes' \
  cargo build --offline --locked --release
```

### Drift cells

Every cell began with exact bytes
`header\nneedle\nneedle\nfooter\n` (SHA-256
`720d9eab1bb95741e7d651a612479568e2cf0c0197b6ec9c405b5c4f36f83220`),
then one Session constructed `edit replace @hits[1] "TARGET\\n"` from the
same Search output
`Found 2\n0\tLine\tnote.txt:1\n1\tLine\tnote.txt:2\n`. The external
mutation occurred after Search and Edit construction and before `apply @edit`.
No Anchor was created or used.

| Cell | External bytes before Apply | Result and exact final bytes | Final SHA-256 |
| --- | --- | --- | --- |
| No drift | `header\nneedle\nneedle\nfooter\n` | Correct Apply; `header\nneedle\nTARGET\nfooter\n` | `a0cbd519a88a863fe562c4ac59b800693d562502706b82d220a84db20aa95d27` |
| Edit before target | `expanded-header\nneedle\nneedle\nfooter\n` | Correct Apply; `expanded-header\nneedle\nTARGET\nfooter\n` | `e6293bc7681bacaf5a3b74b1103a0f3a5f7359c5cbf2c09795049639acb3d870` |
| Edit after target | `header\nneedle\nneedle\nfooter changed\n` | Correct Apply; `header\nneedle\nTARGET\nfooter changed\n` | `5aebe8d881007e74880d5fd6647aea8dad30cd56142dfad4bea25914d897f056` |
| Adjacent context edit | `header\ncontext\nneedle\nfooter\n` | Correct Apply; `header\ncontext\nTARGET\nfooter\n` | `e5748a9d5f4234163f0cde1800761883ce696117a777181fba6e889c934aa17e` |
| Target changed | `header\nneedle\nchanged\nfooter\n` | Safe Reject, exit 1 `current source is unavailable`; bytes unchanged | `e97c389bc1bf3d49a438998c025249196d261dd1e2e628562892d5a05ab6ba48` |
| Identical duplicate inserted | `needle\nheader\nneedle\nneedle\nfooter\n` | **Wrong Apply**; `needle\nheader\nTARGET\nneedle\nfooter\n`, while the selected old occurrence is now ordinal 3 | `10e2cec0e2de551772656b879f40455beb53bac94a4e4391783c9cd761191f25` |
| Target deleted | `header\nneedle\nfooter\n` | Safe Reject, exit 1 `current source is unavailable`; bytes unchanged | `4f965179f0a9afac9c01f6ae49778dada99867660cdf879688dccfae4d03c1ff` |

The seven fresh fixtures therefore produced four Correct Applies, two Safe
Rejects, and one byte-proven Wrong Apply. The Wrong Apply is the v3 defect this
task gates; it is not Backwriter-owned Anchor continuity.

### Repeatable fixtures and commands

All fixture setup was outside the timed interval. The generator wrote the
following exact byte recipes; its source SHA-256 was
`43e117a46576c70912e54286d41d3c553bff4626cbbe68feed3d09f4bb2764fd`.

| Fixture | Exact recipe | Size and content SHA-256 |
| --- | --- | --- |
| 128 MiB low-hit | 32,768 blocks: 32,767 × (`a` × 4,095 + LF), then (`a` × 4,089 + `needle` + LF) | 134,217,728; `efa0da04277e9bcb2c2ce81c8b8886e685e1db4d50959fa5ac917c7e011725f6` |
| 256 MiB low-hit / late Line | 65,536 blocks with the same 4,096-byte recipe and only the last block containing `needle` | 268,435,456; `d4edd123621cf230590d7812e64bec69460789eba3e0c7136b88a3f26c88f5e5` |
| 2,048-file corpus | Files `0000.txt`…`2047.txt`, each 65,536 bytes; all are (`b` × 65,535 + LF) except the last is (`b` × 65,529 + `needle` + LF) | 134,217,728 total; sorted `path NUL size NUL sha256 LF` family hash `c79bfcfea0474cad1c7214e9387147a81295a559600c0e9b9310b7bdb27d5d8d` |
| 1,048,576 hits | `hit` + LF repeated exactly 1,048,576 times | 4,194,304; `f7621d1ce80529f36f9e5c467399bd27c4aeae11aef1d0bf85e104a29f2804fa` |
| Small patch | `alpha\nneedle\nomega\n` | 19; `f469b9feb6a6a617ff103cae4bc0126d71c11e8d487b5534172c1006de6f421e` |

The task-local C runner (source SHA-256
`28ebb82c76d69cc1855f71a575a00fb8807770c21983886996aff703b1edc86c`)
used `CLOCK_MONOTONIC_RAW`, fork/exec, child `sched_setaffinity` to CPU 8,
opened `/proc/<pid>/io` before exec, retained the exited child with
`waitid(WNOWAIT)`, then read `rchar`/`wchar` and reaped with `wait4` for peak
RSS. It redirected stdout/stderr to task-local regular files. This is the
measurement contract Phase 7 must reproduce; it does not instrument Backwriter.
The fixed task root was `/tmp/backwriter-phase2.oKVyJj`; Phase 7 can recreate
that exact absent path to preserve workspace-coordinate-dependent output. The
late v3 address was the first object from an untimed 256 MiB Search, compactly
encoded with SHA-256
`b279fbe2386fb6d01ef775502c3e32b93d720bfdf9f99fe1bf93f0b80026f6f0`.
Every command had an untimed warm-up, then seven fresh-process samples:

```sh
$MEASURE --cpu 8 --stdin "$ROOT/empty.in" --stdout out --stderr err -- \
  $BW --workspace "$FIX/single" --json search line needle --source large.txt
$MEASURE --cpu 8 --stdin "$ROOT/empty.in" --stdout out --stderr err -- \
  $BW --workspace "$FIX/multi" --json search line needle
$MEASURE --cpu 8 --stdin "$ROOT/empty.in" --stdout out --stderr err -- \
  $BW --workspace "$FIX/single" --json view anddress "$LATE_V3_ADDRESS"
$MEASURE --cpu 8 --stdin search-view.in --stdout out --stderr err -- \
  $BW --workspace "$FIX/single" shell
$MEASURE --cpu 8 --stdin "$ROOT/empty.in" --stdout out --stderr err -- \
  $BW --workspace "$FIX/hits" --json search line hit --source hits.txt
$MEASURE --cpu 8 --stdin patch.in --stdout out --stderr err -- \
  $BW --workspace "$FIX/patch" shell
```

`search-view.in` was exactly the following LF-terminated input:

```text
let hits = search line needle --source large.txt
view anddress @hits[0]
exit
```

`patch.in` was exactly:

```text
let hits = search line needle --source patch.txt
let edit = edit replace @hits[0] "replacement\n"
apply @edit
exit
```

The resident cell used a fresh Session per sample, constructed the same Search
and Edit before timing, then measured only the write of `apply @edit` through
receipt of exact `OK\n`; live `/proc` readings supplied HWM and I/O deltas.

Every sample had empty stderr, exit 0, the expected source result, and an exact
output SHA-256. Output contracts were: 128 MiB Search 4,371 bytes
`85650a08d886f4011f7f983927d150a7e3e760777f939f3b96021c2e29edae8d`;
256 MiB Search 4,368 bytes
`2374addb8a18a9ca8f7fab461edcff46fa638adb06bcadebdbcec3988b28f8cd`;
multi-file Search 65,803 bytes
`bad1f4a218df7202458aff97c949327df4c11ecca039e1bd5c870f1e74126442`;
late View 4,552 bytes
`287c9979267e8723ecb61cb7dc54f0a452335e5c7b39e32dac3608cf03a3516d`;
Search→View 4,127 bytes
`bf65c2f9c52694d665050b82815cb8c129c26770c4c352c3a6672eab6a3b6659`;
million-hit JSON 223,284,217 bytes
`7c3b582c88f2a635d00879a5fc7d00e0ddce114d4fb5e67eed26a3cc35ce70e6`;
fresh patch 30 bytes
`eb564f8009332769e900dfac7c491c1ddaff59d2543854226670a64a1038c7ee`;
and resident Apply 3 bytes
`a12b7cb43c9d9134b5bb1b35e9096b66775d9e92e7611d1cc92b02edd6782a87`.
The million-hit output contained exactly 1,048,576 ordered Line objects.

### Release timing baseline

Elapsed values below are raw milliseconds in run order. HWM is raw KiB in run
order. `p95` is nearest-rank; with seven samples it equals the maximum. `rchar`
and `wchar` were identical across all seven samples and are shown once.

| Cell | Raw elapsed ms | min / median / p95=max ms | Raw HWM KiB | `rchar` / `wchar` | Median throughput |
| --- | --- | --- | --- | --- | --- |
| 128 MiB Search | 329.236, 327.857, 327.534, 329.035, 326.713, 328.110, 328.309 | 326.713 / 328.110 / 329.236 | 2584, 2756, 2672, 2568, 2572, 2672, 2760 | 134224217 / 4371 | 390.114 MiB/s |
| 256 MiB Search | 656.782, 650.000, 656.130, 657.507, 654.478, 654.565, 653.854 | 650.000 / 654.565 / 657.507 | 2716, 2732, 2616, 2616, 2668, 2672, 2796 | 268441945 / 4368 | 391.100 MiB/s |
| 2,048-file Search | 330.220, 326.663, 329.535, 329.682, 328.601, 331.789, 328.840 | 326.663 / 329.535 / 331.789 | 3020, 2876, 3012, 2832, 2896, 2796, 2832 | 134224217 / 65803 | 388.426 MiB/s |
| Late Line View | 933.664, 929.846, 956.718, 934.070, 931.297, 931.414, 933.104 | 929.846 / 933.104 / 956.718 | 2668, 2856, 2672, 2708, 2864, 2860, 2864 | 268441945 / 4552 | 274.353 MiB/s |
| Warm Search→View | 1587.874, 1584.639, 1587.527, 1590.007, 1584.926, 1587.049, 1591.735 | 1584.639 / 1587.527 / 1591.735 | 2620, 2636, 2744, 2724, 2620, 2712, 2572 | 536877478 / 4127 | 322.514 MiB/s over two passes |
| 1,048,576-hit Search | 601.899, 602.132, 602.649, 600.311, 604.860, 604.970, 608.226 | 600.311 / 602.649 / 608.226 | 354648, 354656, 354824, 354652, 354912, 354600, 354656 | 4200793 / 223284217 | 6.637 source MiB/s |
| Fresh Search→Edit→Apply | 0.601, 0.540, 0.422, 0.415, 0.424, 0.418, 0.409 | 0.409 / 0.422 / 0.601 | 2772, 2760, 2604, 2716, 2716, 2780, 2756 | 6698 / 73 | n/a |
| Resident Edit→Apply | 0.047, 0.046, 0.045, 0.045, 0.046, 0.044, 0.045 | 0.044 / 0.045 / 0.047 | 2796, 2820, 2816, 2812, 2700, 2752, 2768 | delta 87 / 70 | n/a |

The 128 MiB and 256 MiB low-hit Search medians both used 2,672 KiB HWM,
while elapsed and `rchar` doubled. Low-hit retained memory therefore did not
scale with source size on this fixture. The million-hit median calculation is
`354656 KiB × 1024 / 1048576 hits = 346.34375 HWM bytes/hit`; output alone was
212.940423 bytes/hit. That cell exposes result construction/allocation cost,
not source retention.

No earlier raw same-host v3 performance sample was present in the repository or
Phase 1 evidence, so there is no direct prior numeric comparison. These pinned
samples are the Phase 7 comparison anchor; the recommended percentages above
remain future v4 gates rather than claimed improvements.

### Counter and stack evidence

The profiler command used CPU 8 and these user-space events:

```sh
taskset -c 8 perf stat -x '\t' \
  -e cpu_core/cycles/u,cpu_core/instructions/u,cpu_core/branches/u,\
cpu_core/branch-misses/u,cpu_core/cache-misses/u -- COMMAND
taskset -c 8 perf record -F 999 -g --call-graph dwarf -- COMMAND
```

| Profile workload | cycles | instructions | branches | branch misses | cache misses | samples / lost |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 256 MiB Search | 3,359,260,383 | 21,052,412,189 | 5,706,314,354 | 216,164 | 67,871 | 723 / 0 |
| 2,048-file Search | 1,653,492,788 | 10,539,403,126 | 2,856,729,691 | 84,166 | 39,893 | 367 / 0 |
| Search→View | 7,961,220,542 | 50,994,177,768 | 12,225,152,930 | 353,780 | 127,814 | 1,715 / 0 |
| 1,048,576-hit JSON Search | 2,586,464,205 | 11,530,871,951 | 2,449,141,233 | 3,244,577 | 10,877,088 | 621 / 0 |
| Fresh small patch | 674,671 | 441,213 | 90,201 | 6,074 | 3,863 | repeated resident stack run: 169 / 0 |

For the two low-hit Search profiles, `source_scan::scan_source` was 57.14% and
53.89% self, the inlined Search event/matcher closure was 41.22% and 42.63%,
and `Utf8Validator::push` was 1.22% and 2.05%; multi-file `from_utf8` was 0.85%.
No allocation, JSON, directory-sort, or filesystem wrapper reached the 0.5%
self threshold there. This warm tmpfs/user-space sample does not claim kernel
I/O is free; it shows per-byte scan and event dispatch dominate visible CPU.

Search→View showed separate full-source stacks: Search scanner descendants
accounted for 41.41% children and View scanner descendants for 53.74%.
`ExactTargetTracker::consume` was 30.12% self, Search event handling 16.44%,
View observation handling 15.63%, and the two `scan_source` specializations
24.45% and 11.95% self. Its exact `rchar` was 536,877,478 versus 268,441,945
for Search alone, proving two complete structural passes rather than inferring
them from wall time.

The million-hit profile moved cost to projection and ownership:
`serde_json::ser::to_vec` 16.02%, Anddress validation 13.77%, field append
5.92%, `malloc` 4.99%, logical-path validation 4.33%, free 4.23%, split
iteration 3.94%, Search event handling 3.55%, vector growth 2.54%, Anddress
encode 2.53%, result projection 2.22%, construction 1.77%, Natural parsing
1.63%, and scanning 1.58% self. Decimal ordinal conversion, result construction,
allocation, and JSON projection are therefore evidenced costs for large result
sets; sorting did not reach the reporting threshold.

The fresh small-patch stat covers one Search→Edit→Apply. Because that process
is too short for reliable sampling, the stack run prepared one resident
identity replacement and executed it 20,000 times; it is stack evidence only,
not a timing sample. Its visible self costs included transcript hashing 4.40%,
shell dispatch 4.06%, capability-relative open 3.81%, Apply event scanning
3.39%, formatting/hex work, allocation/free, Anddress validation,
`ExactTargetTracker`, `Output::emit`, SHA-256 compression, stat, and file
removal. This confirms multiple structural/hash/open/publication paths in the
small Apply without using the repeated run as a latency baseline.

## Forbidden work and release boundary

History, past-target lineage, relocation, context matching, persistent Search
index, full workspace cache, whole-source retained observation, watcher, retry,
CAS, merge, Git behavior, implicit v3 compatibility, unapproved hash/dependency,
and benchmark-only semantic shortcuts are forbidden. Phase 1 also forbids
profiler execution/install and Rust, Cargo, tests, CLI, version, server,
deployment, service, tunnel, DNS, artifact, or public-root changes.

Public `0.1.0` and prior betas remain closed and immutable. `0.2.0` has no
artifact, installer, manifest, tag, GitHub Release, crates.io release, or public
endpoint. Phase 7 completion still requires separate Owner authority for source
versioning, release construction, and publication.

## Status and evidence

- [x] Phase 1 — authority record (completed 2026-08-30)
- [x] Phase 2 — reproduction, profile, and baseline (completed 2026-08-30)
- [x] Phase 3 — v4 value and wire kernel (completed 2026-08-30)
- [x] Phase 4 — Search producer and `CurrentObservation` (completed 2026-08-30)
- [x] Phase 5 — View and Check consumers (completed 2026-08-30)
- [x] Phase 6 — Apply and Anchor cutover (completed 2026-08-30)
- [ ] Phase 7 — integrated verification and release decision

Evidence:

- Phase 1: active authority contains every exact guard and separates the closed
  v3 implementation from the unpublished v4 target. The task is tracking-only;
  active architecture remains semantic authority.
- Phase 1: Rust, Cargo, tests, source version, and server repository are
  byte-identical to their stated baselines. Offline/locked metadata, Markdown
  fences/links, diff, index, and `.artext` audits passed. The unchanged
  193-test v3 result is cited from stable closure and was not rerun.
- Phase 1: the exact public `0.1.0` 20-file fingerprint and manifest, both
  service identities/restart counts, loopback listener, and named-tunnel
  connector set remained unchanged. The containing commit and handoff record
  provide commit/push evidence.
- Phase 2: the seven drift cells contain one byte-exact Wrong Apply; the fixed
  release baseline contains seven raw samples per cell, exact fixture/output
  hashes, HWM and I/O accounting, and five counter/stack profiles. The full
  unchanged 193-test suite and every offline/locked gate passed. Task-local
  harnesses, fixtures, profiler data, and build output were removed after the
  evidence was transcribed.
- Phase 3: SHA-256, exact v4 value/wire validation and KATs, incompatible v3
  rejection, one-read Search hash/range projection, and compilation of every
  current capability consumer are complete. Apply retains only a private
  call-local structural mapper after v4 source-state/range proof; removing that
  transitional execution path remains Phase 6 work. All 186 GNU-host tests and
  every offline/locked Phase 3 gate pass. No benchmark or release claim is
  made.
- Phase 4: one fixed-scratch chunk observer now owns UTF-8/NUL validation,
  incremental SHA-256, and checked byte length. Exact File uses that observer
  without Line framing; content Search uses File-, Paragraph-, or Line-specific
  projections without generic per-byte `SourceEvent` callbacks. Late source
  failure discards source-local provisional results, same-source output shares
  one v4 source identity, and the existing deterministic bucket sort remains
  because component DFS does not prove whole-path byte order. All 191 GNU-host
  tests and every offline/locked Phase 4 gate pass. No benchmark or release
  claim is made.
- Phase 5: ordinary View now captures File or exact target-range output during
  one common UTF-8/NUL/hash/length observation. Minimal Line boundary state
  projects an optional related Paragraph only for an exact current text Line;
  raw-valid nonstructural ranges remain accepted and do not assert that
  relation. Check retains grouping, filtering, reports, order, and multiplicity
  while comparing only source hash and length once per eligible source. The
  ordinary View and Check paths have no generic event scanner or target
  tracker; Anchor creation/anchored View and Apply retain their Phase 6
  consumers. All 200 GNU-host tests and every offline/locked Phase 5 gate pass.
  No benchmark or release claim is made.
- Phase 6: Apply now stages one accepted source observation and patches public
  v4 ranges directly with fixed-chunk readback. Direct after projection keeps
  only exact structural candidates and provenance markers for Anchor
  reflection. Anchor creation uses direct target projection and anchored View
  reuses direct View capture. The private ordinal/text mapper, resolver,
  extractor, target tracker, and generic event framer are removed. Raw-valid
  nonstructural Apply ranges and direct no-ops are covered; raw-valid
  nonstructural Anchor creation remains unavailable. The full GNU-host suite
  and every offline/locked Phase 6 gate pass. No benchmark or release claim is
  made.
- Phase 7: pending integrated verification, benchmark comparison, and release
  decision.
