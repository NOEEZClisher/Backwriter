# Backwriter 0.2.0 Anddress Fast Path

Status: Phases 1–7 completed; source correctness and formal performance gates
passed, original performance recommendations missed, release decision NO-GO.

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

## Phase 7 integrated verification and decision

Phase 7 compared immutable Git object exports, without switching the working
tree: v3 `399805906b352f1c8d0cc2fa0bbe6dee1a73a13c` and v4
`55999768cc7ad75ea84a08d597dbc7a7913fe6c3`. The ordinary stripped v3 release
binary reproduced the Phase 2 SHA-256 exactly:
`f7b0aaea704561b6842f35778991344a1d5c1fc5a3aed464df27b526dc26db9d`.
The v4 release binary was 719,944 bytes with SHA-256
`fa4b8d28227057d239f653f584c5f29b8f39f98359b5a26ea6b008fd86590e5b`.
The separate object-export profiler binaries were v3
`6098d2f89c0ab7c7b48f95a013de868f5a024955f3d4b15bbe8c04d36dadde0a`
and v4
`296ae40f916490917612b7b3adaf8266653948ef22f896cbb83bcd1f2850739d`;
their source paths differ from Phase 2, so their hashes are recorded only as
profile-build identity and never used as timing baselines.

The fixed absent task root `/tmp/backwriter-phase2.oKVyJj`, CPU 8, existing
`powersave` governor, tmpfs fixture/output, Rust 1.95.0, LLVM 22.1.2, GNU time
1.10, perf 7.2.2-1, and `perf_event_paranoid=2` reproduced the Phase 2 host
contract without changing power or kernel state. The recreated generator
source SHA-256 was
`6b202eb5d4b30cf5a17b20416b621654b8d75d0d0383bb4f557824393690dd14`;
the C runner source SHA-256 was
`19f7fa7769054b89057447c15644451080253ecdfa6c3f812f29cd14bc53cb56`.
The original task-local sources were never tracked, so source hashes are not
claimed equal. The generator reproduced all five Phase 2 fixture/family hashes
exactly, and the runner retained `CLOCK_MONOTONIC_RAW`, child CPU affinity,
pre-exec `/proc/<pid>/io`, `waitid(WNOWAIT)`, and `wait4` HWM accounting. Every
cell had one untimed warm-up followed by seven fresh processes or Sessions.

The 256 MiB Search, 2,048-file Search, late View, Search→View, million-hit
Search, fresh patch, and resident Apply v3 outputs reproduced their Phase 2
hashes exactly. The historical 128 MiB logical filename was not recorded; this
run used `medium.txt`, preserving exact fixture bytes while producing a new
path-dependent raw output hash for both A/B sides.

### V4 drift safety

One table-driven production regression constructs the second duplicate-Line
Edit from the original Search result, applies the exact seven external byte
states below, and keeps a current File Anchor for each state. Only the no-drift
cell publishes. Every other cell returns `Unavailable`, leaves source bytes and
the current Anchor unchanged, and leaves no staging or after temporary. The
existing duplicate-Paragraph regression separately proves the same fail-closed
behavior for ordinal drift among equal Paragraphs.

| Cell | Apply result | Exact final SHA-256 |
| --- | --- | --- |
| No drift | Correct Apply | `a0cbd519a88a863fe562c4ac59b800693d562502706b82d220a84db20aa95d27` |
| Edit before target | Safe Reject | `5191696d17673f68282c2eca041b0a42a0c8a94e9a382f37bf0be2048e6b9a60` |
| Edit after target | Safe Reject | `9a9f5350603f134fb2ead346274807689c375b946a84c1d543256b30f57fa47b` |
| Adjacent similar context | Safe Reject | `e4edfddd75c2bcaff25cfb0b95dbe27d5407ae6edf0821ab152fe8454c3fe24b` |
| Target changed | Safe Reject | `e97c389bc1bf3d49a438998c025249196d261dd1e2e628562892d5a05ab6ba48` |
| Equal text inserted at another range | Safe Reject | `c8295ff2c29375a172c96c0e356a1b5c123d5c7c7aa8e6dc5ef1a8c09570a14a` |
| Target deleted | Safe Reject | `4f965179f0a9afac9c01f6ae49778dada99867660cdf879688dccfae4d03c1ff` |

The result is one Correct Apply, six Safe Rejects, and zero Wrong Applies. It
also covers edits before/after, changed and deleted targets, equal text at a
different range, ordinal drift, duplicate text, and similar adjacent context.

### Raw A/B samples

Elapsed values below are raw milliseconds in run order. HWM values are raw
KiB in the same order. Nearest-rank p95 over seven samples is the maximum.

| Build / cell | Raw elapsed ms | Raw HWM KiB |
| --- | --- | --- |
| v3 128 MiB Search | 338.671, 338.656, 334.337, 341.844, 346.669, 338.369, 335.310 | 2636, 2860, 2808, 2636, 2680, 2620, 2716 |
| v4 128 MiB Search | 312.383, 285.053, 311.987, 309.132, 309.064, 301.338, 312.159 | 2432, 2572, 2576, 2416, 2512, 2448, 2556 |
| v3 256 MiB Search | 674.754, 673.898, 670.633, 668.501, 675.934, 678.609, 670.777 | 2744, 2676, 2668, 2756, 2712, 2668, 2744 |
| v4 256 MiB Search | 625.999, 625.227, 625.628, 628.018, 612.050, 614.168, 634.207 | 2544, 2520, 2560, 2512, 2544, 2420, 2464 |
| v3 2,048-file Search | 339.425, 337.139, 343.484, 348.319, 354.136, 347.654, 343.449 | 2880, 2828, 2908, 2876, 2956, 2864, 3100 |
| v4 2,048-file Search | 316.679, 320.207, 310.159, 312.506, 307.529, 314.610, 301.387 | 2684, 2564, 2712, 2556, 2684, 2672, 2684 |
| v3 late Line View | 953.871, 950.105, 967.235, 949.986, 965.561, 966.373, 974.967 | 2848, 2684, 2676, 2620, 2572, 2864, 2680 |
| v4 late Line View | 348.903, 347.469, 350.578, 349.888, 350.500, 355.033, 352.704 | 2548, 2524, 2516, 2524, 2448, 2512, 2544 |
| v3 Search→View | 1630.262, 1634.518, 1653.215, 1651.792, 1627.493, 1631.957, 1636.699 | 2712, 2832, 2704, 2848, 2900, 2740, 2848 |
| v4 Search→View | 993.567, 972.550, 973.478, 971.083, 966.568, 973.465, 968.969 | 2708, 2576, 2624, 2640, 2708, 2544, 2664 |
| v3 1,048,576-hit Search | 620.143, 621.643, 611.297, 604.992, 604.849, 610.300, 621.530 | 354676, 354848, 354856, 354728, 354788, 354688, 354780 |
| v4 1,048,576-hit Search | 484.616, 489.714, 484.652, 489.427, 483.866, 487.666, 526.514 | 59956, 59928, 59844, 59860, 59828, 59872, 59804 |
| v3 fresh Search→Edit→Apply | 2.866, 0.452, 0.429, 0.379, 0.368, 0.368, 0.362 | 2568, 2636, 2708, 2720, 2704, 2740, 2856 |
| v4 fresh Search→Edit→Apply | 0.482, 0.463, 0.437, 0.442, 0.446, 0.417, 0.375 | 2516, 2488, 2524, 2416, 2524, 2524, 2624 |
| v3 large late-range Check | 665.322, 655.934, 653.927, 653.845, 650.003, 653.956, 651.414 | 2860, 2860, 2744, 2680, 2636, 2660, 2672 |
| v4 large late-range Check | 155.684, 156.774, 156.804, 154.808, 157.021, 154.390, 154.265 | 2548, 2524, 2556, 2516, 2536, 2452, 2512 |
| v3 resident Edit→Apply | 0.039, 0.368, 0.042, 0.039, 0.037, 0.038, 0.037 | 2732, 2792, 2688, 2700, 2800, 2796, 2780 |
| v4 resident Edit→Apply | 0.035, 0.039, 0.037, 0.035, 0.033, 0.032, 0.033 | 2700, 2736, 2736, 2756, 2732, 2716, 2700 |
| v3 range Apply prepublication | 1256.187, 1239.583, 1239.731, 1246.171, 1242.780, 1260.389, 1240.960 | 2808, 2796, 2792, 2796, 2816, 2832, 2836 |
| v4 range Apply prepublication | 208.122, 209.329, 211.847, 210.305, 212.253, 212.210, 209.767 | 2652, 2692, 2692, 2696, 2696, 2644, 2732 |

Fresh-process HWM is `wait4` peak RSS. Resident HWM and RSS were read from the
live process after Apply; both vectors were identical to the resident HWM
vectors above. The resident CPU clock is 100 Hz, so sub-millisecond Apply user
and system deltas correctly quantize to zero rather than creating synthetic
precision.

| Cell | v3 min / median / p95 wall ms | v4 min / median / p95 wall ms | v3 / v4 median CPU ms | v3 / v4 peak HWM KiB | v3 / v4 median `rchar` / `wchar` | v3 / v4 source throughput MiB/s |
| --- | --- | --- | --- | --- | --- | --- |
| 128 MiB Search | 334.337 / 338.656 / 346.669 | 285.053 / 309.132 / 312.383 | 337.919 / 308.629 | 2860 / 2576 | 134224198 / 4369; 134224198 / 399 | 377.965 / 414.063 |
| 256 MiB Search | 668.501 / 673.898 / 678.609 | 612.050 / 625.628 / 634.207 | 672.803 / 624.733 | 2756 / 2560 | 268441926 / 4368; 268441926 / 398 | 379.879 / 409.189 |
| 2,048-file Search | 337.139 / 343.484 / 354.136 | 301.387 / 312.506 / 320.207 | 342.886 / 311.937 | 3100 / 2712 | 134224198 / 65803; 134224198 / 381 | 372.652 / 409.592 |
| Late Line View | 949.986 / 965.561 / 974.967 | 347.469 / 350.500 / 355.033 | 963.845 / 349.845 | 2864 / 2548 | 268441926 / 4552; 268441926 / 4846 | 265.131 / 730.385 |
| Search→View | 1627.493 / 1634.518 / 1653.215 | 966.568 / 972.550 / 993.567 | 1631.620 / 971.077 | 2900 / 2708 | 536877459 / 4127; 536877459 / 4141 | 313.242 / 526.451 over two passes |
| 1,048,576-hit Search | 604.849 / 611.297 / 621.643 | 483.866 / 487.666 / 526.514 | 609.835 / 483.547 | 354856 / 59956 | 4200774 / 223284217; 4200774 / 343377441 | 6.543 / 8.202 |
| Fresh Search→Edit→Apply | 0.362 / 0.379 / 2.866 | 0.375 / 0.442 / 0.482 | 0.402 / 0.466 | 2856 / 2624 | 6679 / 73; 6653 / 76 | n/a |
| Large late-range Check | 650.003 / 653.927 / 665.322 | 154.265 / 155.684 / 157.021 | 652.918 / 155.280 | 2860 / 2556 | 268441926 / 4364; 268441926 / 394 | 391.481 / 1644.357 |
| Resident Edit→Apply | 0.037 / 0.039 / 0.368 | 0.032 / 0.035 / 0.039 | 0 / 0 | 2800 / 2756 | delta 87 / 46; delta 61 / 46 | n/a |
| Range Apply prepublication | 1239.583 / 1242.780 / 1260.389 | 208.122 / 210.305 / 212.253 | 1250 / 210 | 2836 / 2732 | delta 536870924 / 268435459; delta 268435468 / 268435459 | 205.990 / 1217.280 |

Fresh user/system median pairs were, in the same table order: v3/v4 128 MiB
`323.347+15.993` / `294.921+13.990`; 256 MiB `643.757+28.961` /
`593.252+30.955`; multi-file `328.620+14.994` / `292.011+19.926`; late View
`934.265+26.955` / `320.905+30.930`; Search→View `1562.616+67.864` /
`911.200+56.872`; million-hit `505.164+103.796` / `411.617+77.941`; fresh
Apply `0.402+0` / `0.450+0`; and late Check `621.909+30.943` /
`124.989+28.998` milliseconds. Range Apply resident medians were
`1130+100` and `130+80` milliseconds.

Every output was deterministic across warm-up and seven samples, stderr was
empty, and exits were zero. Raw output identities were:

| Cell | v3 bytes / SHA-256 | v4 bytes / SHA-256 |
| --- | --- | --- |
| 128 MiB Search | 4369 / `da40a48f13d4796477c0198bc00ed2f70d08cb62d4fe33c66161c946f06e3310` | 399 / `ee2da437d7f91d64fb7448c497b69ea81dc01d08a5ace04b7ff767fe204fb79b` |
| 256 MiB Search | 4368 / `2374addb8a18a9ca8f7fab461edcff46fa638adb06bcadebdbcec3988b28f8cd` | 398 / `21655979a82f7bfd7355436b095c6976a7de02aa6174172082c2c5e22f0363a0` |
| 2,048-file Search | 65803 / `bad1f4a218df7202458aff97c949327df4c11ecca039e1bd5c870f1e74126442` | 381 / `8bae96272f810a46853e1ad707ffd6c673f8a09888852be95488a4ca7a4b96cc` |
| Late Line View | 4552 / `287c9979267e8723ecb61cb7dc54f0a452335e5c7b39e32dac3608cf03a3516d` | 4846 / `ca936648107d44413ca76cde66cf1c088af8942482dcd149aaa0f66b4d644c4e` |
| Search→View | 4127 / `bf65c2f9c52694d665050b82815cb8c129c26770c4c352c3a6672eab6a3b6659` | 4141 / `31d7a6a438b1d123f0b21b6d0fb8d6712eb15f39cda47ca6711685dbb5835c72` |
| 1,048,576-hit Search | 223284217 / `7c3b582c88f2a635d00879a5fc7d00e0ddce114d4fb5e67eed26a3cc35ce70e6` | 343377441 / `3d31f29d76fe4d90495d0f1a68b0db015dc35bf53ca3947fd21e0902a6a4d2f5` |
| Fresh Search→Edit→Apply | 30 / `eb564f8009332769e900dfac7c491c1ddaff59d2543854226670a64a1038c7ee` | 33 / `63ca2b15c5a5820d9edac1f9b12da943e76cce2fc13296ed1a5092d0a02c68cf` |
| Late Check | 4364 / `ab5993f33c131bf935891d7951fce16c7d04b4145d28c92c98feac4b9bf970f5` | 394 / `be3793aad45723b6be1f717e22f9452fd391e08072fa42357a8d130246ae41c4` |
| Resident/range Apply | 3 / `a12b7cb43c9d9134b5bb1b35e9096b66775d9e92e7611d1cc92b02edd6782a87` | same |

The fresh and resident patch final source was exactly SHA-256
`2e86294b88ae347ebddb6d982a6d5cfe71696d420978db34fcb715714736ef25`.
The prepublication Move-at-own-boundary cell left the 256 MiB source exactly
`d4edd123621cf230590d7812e64bec69460789eba3e0c7136b88a3f26c88f5e5`.
The million-hit outputs contained exactly 1,048,576 ordered Line objects. v3
used consecutive ordinals and v4 used exact starts `0,4,…,4194300`.

### Profiles and gates

Profiler executions used the separate debuginfo/frame-pointer builds and are
not timings. Representative `perf stat` values were:

| Build / profile | cycles | instructions | branches | branch misses | cache misses |
| --- | ---: | ---: | ---: | ---: | ---: |
| v4 256 MiB Search | 2,680,997,853 | 12,877,820,510 | 3,830,035,802 | 226,947 | 64,253 |
| v4 late Line View | 1,473,761,888 | 7,243,676,051 | 1,684,169,920 | 153,478 | 64,965 |
| v4 late-range Check | 610,278,939 | 1,064,931,757 | 72,825,671 | 71,592 | 11,941 |
| v4 Search→View | 4,159,070,049 | 20,121,072,117 | 5,514,118,051 | 378,874 | 125,849 |
| v4 1,048,576-hit Search | 2,020,886,309 | 9,551,185,891 | 1,920,121,572 | 1,886,009 | 1,147,417 |
| v3 Search→range Apply pipeline | 9,128,954,957 | 56,229,660,134 | 14,447,318,163 | 497,015 | 105,711 |
| v4 Search→range Apply pipeline | 3,301,784,578 | 13,944,718,787 | 3,903,312,745 | 300,143 | 154,170 |

The four DWARF stack runs lost zero samples: v4 Search→View 921, v4
million-hit 491, v3 range pipeline approximately 2,000, and v4 range pipeline
780. The v3 range pipeline's largest self symbols were
`ExactTargetTracker::consume` 23.53%, Search `scan_source` 19.40%, and two
additional Apply structural scans at 13.38% and 11.97%. The v4 range pipeline
instead showed one Search projection at 37.22% and SHA-256 compression shared
by Search and the single Apply staging observation at 33.78%; no target search,
relocation, or legacy tracker appeared. V4 Search→View showed Search projection
28.91%, direct View projection 22.25%, and shared SHA-256 compression 27.09%.
The million-hit profile moved to Adapter wire cost: `serde_json::ser::to_vec`
was 31.20%, while Search projection was 2.46%.

The formal gates pass:

- late View and late Check median CPU are 36.30% and 23.78% of v3, below 75%;
- range Apply prepublication median CPU is 16.80% of v3, below 75%;
- Search median wall ratios are 79.78%–92.84% and peak HWM ratios are
  16.90%–92.89%, below 105%;
- every measured p95 and peak HWM ratio is below 110%;
- v4 128/256 MiB low-hit peak HWM is 2,576/2,560 KiB while input and `rchar`
  double, so retained memory is not proportional to source size;
- production audits find no consumer target search/relocation, second Search
  hash pass, whole-source `CurrentObservation`, fixed result cap, skip, or
  truncation. The retained `line_bytes.truncate` removes an already captured
  Line terminator for View, and Pick's `stack.truncate` unwinds its iterative
  boolean parser; neither truncates input or results.

No allocator-specific instrumentation was added. Available allocation evidence
is peak HWM, result/output size, and profiler symbols. Million-hit peak HWM
improved from 346.539 to 58.551 bytes/hit, while public JSON output grew from
212.940 to 327.470 bytes/hit because every v4 target carries exact source-state
and range fields.

The final external-state audit left `/home/NOEEZ/server` clean at
`c6f0b1e46db45646f9abc00401d1749833c2ed8a`, equal to `origin/main`. The exact
20-file public tree and all recorded hashes remained unchanged, including the
876-byte manifest SHA-256
`551ee8b6fc4c5df83421ba7244f191fee8cc70287775088f08f5e1b8e2290570`.
`backwriter-origin.service` remained loaded, enabled, active/running with zero
restarts and one `127.0.0.1:8080` listener. Phase 7 changed no server, service,
tunnel, DNS, installer, public root, artifact, or publication state.

The separately retained original recommendations do not all pass. Late View
exceeds the requested 2× improvement, but 256 MiB Search is only 1.077× faster,
not 2×. Million-hit output is 153.79% of the v3 bytes/hit, not at most 50%.
These are performance recommendation failures rather than correctness or
formal-gate regressions, so Phase 7 adds no speculative optimization or wire
change. The integrated source and benchmark are verified, but the release
decision is **NO-GO** until the Owner resolves or explicitly revises those two
recommendations. This is not an artifact, publication, or release claim.

## Forbidden work and release boundary

History, past-target lineage, relocation, context matching, persistent Search
index, full workspace cache, whole-source retained observation, watcher, retry,
CAS, merge, Git behavior, implicit v3 compatibility, unapproved hash/dependency,
and benchmark-only semantic shortcuts are forbidden. Phase 1 also forbids
profiler execution/install and Rust, Cargo, tests, CLI, version, server,
deployment, service, tunnel, DNS, artifact, or public-root changes.

Public `0.1.0` and prior betas remain closed and immutable. `0.2.0` has no
artifact, installer, manifest, tag, GitHub Release, crates.io release, or public
endpoint. Resolving the Phase 7 NO-GO and any release construction or
publication require separate Owner authority.

## Status and evidence

- [x] Phase 1 — authority record (completed 2026-08-30)
- [x] Phase 2 — reproduction, profile, and baseline (completed 2026-08-30)
- [x] Phase 3 — v4 value and wire kernel (completed 2026-08-30)
- [x] Phase 4 — Search producer and `CurrentObservation` (completed 2026-08-30)
- [x] Phase 5 — View and Check consumers (completed 2026-08-30)
- [x] Phase 6 — Apply and Anchor cutover (completed 2026-08-30)
- [x] Phase 7 — integrated verification and release decision

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
- Phase 7: the exact seven-cell drift matrix produces one Correct Apply, six
  Safe Rejects, and zero Wrong Applies while every reject preserves source,
  current File Anchor, and temporary/publication state. Git-object A/B builds,
  fixed Phase 2 fixtures, seven-sample release measurements, and representative
  profiles prove the formal correctness, CPU, Search wall/RSS, p95, and bounded
  source-memory gates. All current 201 GNU-host tests and every offline/locked
  gate pass. The original 2× 256 MiB Search and 50% million-hit output
  bytes/hit recommendations remain unmet, so source verification is complete
  but the recorded release decision is NO-GO. No `0.2.0` artifact or
  publication exists.
