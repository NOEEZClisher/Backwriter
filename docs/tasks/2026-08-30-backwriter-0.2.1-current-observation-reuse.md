# Backwriter 0.2.1 Current-Observation Reuse

Status: Phases 1–7 complete; source readiness is NO-GO, the source version
remains `0.2.0`, and publication has not started.

This tracker records execution evidence and phase progress only. Normative
semantics belong to the active
[Protocol](../architecture/backwriter-text-coordination-protocol.md),
[address model](../architecture/rebuildable-structural-addressing.md), and
[principles](../principles/backwriter-core-principles.md). The closed public
`0.2.0` release and its 28-file publication are immutable. `0.2.1` remains
a partially implemented and unpublished target using the same Anddress v4 wire,
SHA-256, exact source length, target kind, and `[start,end)` range.

## Phase 1 current execution audit

The current common `observe_source` path performs one forward read from a
retained no-follow source handle with fixed scratch. `ObservationBuilder`
incrementally validates UTF-8/NUL, computes SHA-256, and counts checked source
length. Its `CurrentObservation` contains only the completed hash and length
and lives only inside the current call. `WorkspaceRuntime` retains workspace
and admission authority plus live Anchor bindings, but no ordinary observation
or source-state proof.

| Sequence | Current live-source read/hash work | State after return |
| --- | --- | --- |
| Search | One observation per selected source; target projection and hash share the pass. | Results are caller-owned; no Runtime proof. |
| Search → View | One Search observation, then one View reopen/observation for the selected source. | Both hashes are discarded after their calls. |
| Search → Check | One Search observation, then one Check observation per eligible coordinate/path group. | Check retains no result or proof. |
| Search → Apply | One Search observation, then one Apply live-source observation that emits accepted bytes to staging and computes before hash/length. | Before and prospective-after proofs are discarded. |
| Apply → next consumer | Apply computes prospective-after hash/length while emitting output, without rereading the resulting source. | The next Search/View/Check/Apply reopens and rehashes. |

Apply has no separate pre-hash source pass. Its single live-source observation
both stages the accepted before bytes and computes the before hash/length.
Fixed-chunk staging readback is preparation, not a second live-source
observation. Prospective-after validation, SHA-256, and length advance during
output emission; no after-source reread occurs. Existing Anchor dispositions
consume that evidence before it is discarded.

Ordinary and anchored View each use the common observer. Anchor creation uses it
with direct target projection. Raw/Search/Pick Check group inputs and observe
each eligible logical source once. One-shot Search/View/Check each creates a
fresh Runtime. Session retains one Runtime, but its bindings and `DataStore`
are Adapter/caller values rather than observation authority.

## Closed authority

- Default Runtime, one-shot CLI, and ordinary CLI Session remain Untrusted Mode
  and execute the full `0.2.0` observation/hash path.
- Reuse is allowed only in explicit Host-authoritative Mode. The host
  coordinates every visible writer and path replacement, excludes mutation
  from reuse selection through call completion, and synchronously invalidates
  before mutation.
- Watchers, `mtime`, size, inode, path identity, and after-the-fact
  notification are not source-state proof.
- The only cross-call state shape is a Runtime-local, RAM-only, replace-only
  current SHA-256/length proof bound to Runtime, workspace, admission, source
  generation, and logical path.
- No source bytes, Search results, target map, prior hash, history, lineage,
  relocation context, or persistent state may be retained.
- A complete trusted hit may reuse proof. Every miss, incomplete guard, or
  different path/workspace/admission/generation uses the full `0.2.0`
  observation path.
- Search may install proof but never caches results. Confirmed Apply may replace
  old proof with prospective-after hash/length; an exact no-op preserves
  matching proof.
- Host-coordinated or opaque mutation, explicit invalidation, authority change,
  unavailable source, uncertain publication, and Runtime drop discard affected
  proof. Stale input after invalidation remains Safe Reject and Wrong Apply
  remains zero.
- Check retains no result/history state. The narrow trusted current proof is its
  sole cross-call exception.

## Seven phases

1. **Authority and current-flow audit — complete.** Record actual observer,
   hash, Apply, Anchor, Check, and CLI Runtime paths; close the two-mode
   authority without naming a Rust API.
2. **Minimal host kernel and Search installation — complete.** The explicit
   Host constructor, path-exact source invalidation, private proof state, and
   whole-call successful Search installation are implemented.
3. **View bounded reuse — complete.** Trusted ordinary View uses direct-range
   reads and a fixed-scratch nearest-boundary related-Paragraph path without
   whole-source or complete-Line retention.
4. **Check trusted hit — complete.** Reuse matching proof with zero filesystem
   open/read/hash while preserving reports, order, and multiplicity.
5. **Apply and Anchor integration — complete.** Enforce proof preconditions,
   preserve exact no-op proof, install confirmed prospective-after proof, and
   share existing Anchor invalidation/publication fail-closure.
6. **Invalidation and semantic closure — complete.** Host mutation guards,
   explicit/opaque invalidation, authority isolation, fallback, failure
   transitions, and the full Correct/Safe-Reject/Wrong matrix are closed.
7. **Fixed A/B and release-readiness decision — complete, NO-GO.** The fixed
   fixtures and A/B environment reproduce semantic, memory, I/O, and safety
   evidence, but Host Search-to-late-Line View exceeds the formal 400 ms gate.
   Source version remains `0.2.0`; publication remains separate authority.

## Fixed 0.2.0 comparison inputs

These Owner-provided values are fixed comparison inputs and have not been
remeasured during Phase 1.

| Cell | Fixed 0.2.0 input |
| --- | ---: |
| Search | 298.980 ms |
| Late View | 354.830 ms |
| Check | 160.571 ms |
| Search → View | 644.057 ms |
| 1M Search | 406.133 ms / 56.594 MiB / 55.853 bytes-hit |
| Resident View | 27.059 µs |
| Resident Check | 11.498 µs |
| Resident Apply | 50.7 µs |
| Drift | Correct 1 / Safe Reject 6 / Wrong 0 |

## Release-readiness gates

- Search median no more than 105% of the fixed `0.2.0` input.
- Trusted Search → View no more than 400 ms; 350 ms or less is recommended.
- Trusted Check hit performs zero source-size-proportional I/O/hash.
- Peak-memory slope no more than 110%.
- Whole-source retention remains zero.
- Wrong Apply remains zero.

Phase 7 must also preserve exact semantic output, ordering, multiplicity,
Untrusted fallback, v4 KATs, and the complete drift matrix. Passing these gates
does not itself publish a release.

## Phase 2 closure

- `WorkspaceRuntime::open` remains Untrusted;
  `WorkspaceRuntime::open_host_authoritative` is the sole explicit Host entry.
- `WorkspaceRuntime::invalidate_source` validates through existing logical-path,
  admission, and spill rules and shares Anchor's path-exact invalidation.
- A private synchronized sorted vector retains at most one hash/length proof per
  logical path. It has no fixed cap, eviction, retained handle, public getter,
  or generation token; invalidation/removal is the per-path generation boundary.
- A successful content or exact-File Search installs every fully observed source
  only after the whole call succeeds. Entries are independent and do not claim
  workspace completeness. A failed call installs no provisional proof.
- At Phase 2 closure, View and Check did not consume proof. Every Apply call
  removes matching proof before validation and preserves unrelated paths; Apply
  installs none.
- The implementation introduces no watcher, metadata proof, result cache,
  target registry, history, context matching, retry, global snapshot, new wire,
  or compatibility layer.

## Phase 3 closure

- Ordinary View validates source-less input and Runtime coordinate/private-path
  boundaries before privately matching exact path/hash/length proof. The proof
  lock is released before any filesystem access.
- A trusted hit opens the admitted regular source through the existing
  capability-relative no-follow path. File reads its complete v4 range;
  Paragraph and Line read only their target range. No source hash is recomputed.
- A proof miss and every Untrusted View use the unchanged complete one-read/hash
  fallback. An existing same-path mismatch returns `Unavailable` before source
  access.
- Line relation projection uses fixed reverse/forward scratch from the target to
  the nearest separator or source boundary. It retains no whole source or Line
  collection and preserves `None` for separator and nonstructural ranges.
- Short reads, seek/open failures, and recoverable resource failures fail closed
  and remove the matching proof. The public API, errors, v4 identity/wire,
  target text, terminators, related addresses, ordering, and CLI behavior are
  unchanged.
- Apply and Anchor proof consumption is complete in Phase 5; Phase 6 retains
  the complete invalidation/race closure.

## Phase 4 closure

- Host-authoritative raw, Search-outcome, and Pick-outcome Check preserve the
  existing source-less validation, coordinate/path grouping, filtering, report,
  order, duplicates, multiplicity, and public errors.
- After workspace, private-path, and admission classification, a path proof is
  copied as fixed-size SHA-256 bytes plus exact length under the proof lock; the
  lock is released before occurrence comparison or any later work.
- Every matching-proof group performs zero filesystem open, source read, and
  SHA-256 work. Matching occurrences are `Current`; hash or length mismatches
  are `NotCurrent`; kind and range are ignored exactly as before.
- A present proof covers the entire group. Mismatches do not fall back and Check
  never installs, replaces, invalidates, removes, or refreshes proof.
- Untrusted Mode, proof miss, poison, and unusable private proof evidence retain
  the unchanged admission and one-observation-per-eligible-source fallback.
- Regressions cover every Check input form, a 10,000-occurrence mixed group,
  duplicates and arbitrary order, raw-valid nonstructural ranges, multiple hit
  and miss sources, explicit invalidation boundaries, and structural zero-I/O/
  hash and lock-scope evidence. The complete GNU-host suite passes 220 tests.
- Apply and Anchor proof consumption is complete in Phase 5; Phase 6 keeps the
  broader invalidation/race closure.

## Phase 5 closure

- Host Apply keeps Edit, same-coordinate/path, Runtime coordinate, private-path,
  and admission validation priority, then copies one fixed hash/length proof
  and releases proof state before any later work.
- Every operand must match a present path proof. A mismatch is `Unavailable`
  before source access, publication, or Anchor mutation and preserves the proof.
- A hit stages one retained no-follow source read with fixed scratch, exact proof
  length plus one growth-byte check, and UTF-8/NUL validation, but no before
  SHA-256. Miss, poison, and Untrusted execution retain the `0.2.0` staging and
  full before-hash path.
- Direct and assembled byte-identical no-op preserve matching proof, live
  Anchors, source bytes, inode, and temporary state. A miss no-op installs no
  proof.
- Changed output uses its existing prospective-after SHA-256/length as the sole
  identity for both the Anchor plan and a preallocated next proof. Confirmed
  publication installs proof and reflects Anchors without fallible work;
  uncertainty invalidates both on only that logical path.
- Regressions cover a second trusted Apply, old-address Safe Reject, after View
  and Check reuse, File/Paragraph/Line reflection, unrelated paths, short/grown/
  invalid sources, read/resource and temporary boundaries, publication
  uncertainty, no proof lock during I/O/hash/emission/publication, and the
  existing zero-Wrong-Apply drift matrix. The GNU-host suite passes 228 tests.
- Phase 6 closes host mutation/race and authority-drift semantics. Phase 7
  retains all measurement, version, and release-readiness decisions; Phase 5
  makes no performance claim.

## Phase 6 closure

The proven transition table is:

| Event | Proof | Same-path Anchor | Source | Publication |
| --- | --- | --- | --- | --- |
| Successful Host Search | Replace/install only observed paths | Preserve | Read-only | None |
| Failed Search | Install no provisional proof; discard failed-path proof | Preserve | Read-only | None |
| Either public source invalidation | Discard target path | Discard target path | No I/O; unchanged | None |
| Ordinary View or Apply proof mismatch | Preserve | Preserve | No I/O; unchanged | None |
| Check proof mismatch | Preserve | Preserve | No I/O; unchanged | None |
| Anchored View proof mismatch | Discard target path | Discard target path | No I/O; unchanged | None |
| Trusted View open/seek/read/short/resource failure | Discard target proof | Preserve | Unchanged | None |
| Apply length drift, invalid source, or stale binding | Discard target proof | Discard target path | Unchanged | None |
| Apply open/read failure | Discard target proof | Preserve | Unchanged | None |
| Apply resource or definite prepublication failure without mutation evidence | Preserve accepted proof | Preserve | Unchanged | None |
| Direct or byte-identical no-op | Preserve old proof | Preserve | Unchanged | None |
| Confirmed changed Apply | Install prepared after proof | Reflect prepared after plan | After bytes | Confirmed |
| `PublicationUncertain` | Discard target proof | Discard target path | Unknown result | Uncertain |
| Runtime drop | Discard all RAM proof | Discard all RAM continuity | Unchanged | None |

- Both public invalidation methods delegate to one I/O-free path-exact
  proof-plus-Anchor operation. Invalid syntax, private paths, and unadmitted
  paths change no association. Hard-link aliases require separate logical-path
  notification.
- The Host guard is caller authority: every visible writer/path replacement is
  excluded through capability completion, invalidation returns before mutation
  begins, and unsignaled or in-call mutation is a contract violation. Runtime
  adds no watcher, metadata check, rehash, retained handle, lock, CAS, token,
  retry, or supported race.
- Correct invalidation followed by same-length or different-length change,
  deletion, invalid UTF-8, or NUL makes stale View, Check, and Apply safe-reject.
  A guarded mutation after confirmed Apply likewise rejects the old after
  address. No stale path relocates or publishes.
- Matching anchored View now shares ordinary trusted View execution; proof miss
  and Untrusted execution retain its complete direct structural observer. A
  proof mismatch fail-closes proof and continuity before source access.
- Proof remains isolated by exact logical path even for equal hashes, and by
  workspace, admission, Runtime, authority mode, and Runtime lifetime. Failed
  Search installs no provisional proof, Check fallback installs none, and drop
  retains none.
- The seven-cell duplicate-Line drift matrix yields Correct `1`, Safe Reject
  `6`, Wrong Apply `0` in both Untrusted and correctly guarded Host modes.
  Duplicate Paragraph drift safe-rejects in both. The GNU-host development
  suite passes 234 tests.
- Proof locks remain absent from I/O, hashing, emission, and publication. No
  whole source, prior proof chain, history, persistent cache, or public failure
  hook was added. At Phase 6 closure, Phase 7 retained every measurement,
  version, and release-readiness decision; Phase 6 made no performance claim.

## Phase 7 fixed A/B closure

Phase 7 is **NO-GO**. The formal Host Search-to-View gate fails, so this phase
changes no production Rust, public API, v4 wire, CLI behavior, Cargo version,
dependency, artifact, installer, manifest, server, or publication state. The
development implementation remains available at source version `0.2.0` for a
separately authorized follow-up; it is not `0.2.1` source-ready.

### Authority and environment

- A is immutable Source revision
  `2fad6e46d3a9d1da01f79f34b9ffc187447c76a8`; B is immutable pre-decision
  revision `a24ff5eff9b1d2506de32ba740c8f5d5049979ee`. Both were exported directly
  from Git objects without changing the working tree, branch, or index.
- A and B used independent offline/locked release builds. The same
  `phase7_harness.rs` source compiled A Untrusted (`A`), B Untrusted (`BU`),
  and B Host-authoritative (`BH`); only `cfg(phase7_host)` selects B's public
  Host constructor. No production instrumentation or benchmark framework was
  added.
- Linux `7.1.11-arch1-1` ran on Intel Core i7-12700K CPU 2, a 4.9 GHz P-core
  with sibling 3. Its existing governor was `powersave`. The repository was on
  NVMe/ext4 and task-local fixtures/build/output were on `/tmp` tmpfs. No CPU,
  power, kernel, filesystem, or host setting changed.
- Each cell used one warm-up and seven measured runs. Round order rotated
  `A→BU→BH`, `BH→BU→A`, and `BU→A→BH`. The child used
  `CLOCK_MONOTONIC_RAW`; the external runner sampled operation-only process
  CPU ticks and `/proc/<pid>/io`, plus process peak HWM. Its fixed one-byte
  start and five-byte completion protocol are excluded from reported
  `rchar`/`wchar`.
- The 1M cell includes the one existing result collection plus an ordered
  `Anddress::encode()`/SHA-256 verification traversal inside the timed interval.
  It constructs no JSON value, encoded result collection, or second result
  collection. Other result/source checksums run after the timed interval.

The fixture generator reproduced the fixed source bytes exactly:

| Fixture | Bytes | SHA-256 |
| --- | ---: | --- |
| 128 MiB low-hit | 134,217,728 | `efa0da04277e9bcb2c2ce81c8b8886e685e1db4d50959fa5ac917c7e011725f6` |
| 256 MiB low-hit / late target | 268,435,456 | `d4edd123621cf230590d7812e64bec69460789eba3e0c7136b88a3f26c88f5e5` |
| 1,048,576 hits | 4,194,304 | `f7621d1ce80529f36f9e5c467399bd27c4aeae11aef1d0bf85e104a29f2804fa` |
| Small patch | 19 | `f469b9feb6a6a617ff103cae4bc0126d71c11e8d487b5534172c1006de6f421e` |
| Unicode/terminator audit | 27 | `3a198be5951e78d7e6bd3219a9f1187451ae0d9365352105aae5a5941c9abade` |

### Formal gate decision

| Gate | Evidence | Result |
| --- | --- | --- |
| 256 MiB Search median ≤313.929 ms | BU `268.333`; BH `268.379` ms, 89.75% and 89.76% of fixed `298.980` ms | PASS |
| Host Search-to-View ≤400 ms; ≤350 ms recommended | BH late-Line `1,079.943` ms, p95 `1,096.362` ms; `536,870,913` `rchar` | **FAIL** |
| Host Check hit source read/hash/target search 0 | BH `0.004077` ms; exact `rchar=0`, `wchar=0`; Phase 4 structural audit unchanged | PASS |
| 1M result-memory slope ≤61.4383 bytes/hit | BU `58.5078`; BH `58.5156` peak-HWM bytes/hit | PASS |
| Whole-source retention 0 | BU/BH 128→256 MiB Search peak HWM is `2640→2640` / `2624→2624` KiB; production audit retains only hash/length proof | PASS |
| Wrong Apply 0 | Both modes retain Correct `1`, Safe Reject `6`, Wrong `0`; duplicate Paragraph also rejects | PASS |
| v4 KAT and semantic equivalence | Every A/BU/BH cell has identical count, result SHA-256, source SHA-256, order, and multiplicity; full suites pass | PASS |

The failure is specific and reproduced in all seven Host samples. Search reads
the 256 MiB source once, then matching trusted late-Line View reads its Line and
the entire no-separator related Paragraph boundary in reverse/forward scratch;
the composite reports `536,870,913` `rchar`. This retains no whole source in
RAM, but it defeats the intended Host latency gate. Phase 7 does not optimize
that path or weaken Paragraph relation semantics.

The 1M timed verification median is `517.819` ms in BU and `518.400` ms in BH,
127.50% and 127.64% of the fixed `406.133` ms; its only formal gate is the
passing result-memory slope. Resident Apply is `25.942` µs BU and `26.826` µs
BH versus the fixed `50.7` µs. No new acceptance line is inferred for either
informational delta.

### Summary samples

`p95` is nearest-rank and therefore the maximum of seven samples. HWM is the
maximum of the seven process peaks; I/O is the median and was constant across
the seven samples unless the raw table shows otherwise.

| Cell | Variant | Median / p95 ms | Peak HWM KiB | Median `rchar` / `wchar` | Count |
| --- | --- | ---: | ---: | ---: | ---: |
| changed-chain | A | 0.046957 / 0.120964 | 2620 | 37 / 25 | 1 |
| changed-chain | BH | 0.041433 / 0.049955 | 2632 | 31 / 25 | 1 |
| changed-chain | BU | 0.044411 / 0.054934 | 2572 | 37 / 25 | 1 |
| check256-line | A | 157.848530 / 159.295319 | 2624 | 268435456 / 0 | 1 |
| check256-line | BH | 0.004077 / 0.004606 | 2636 | 0 / 0 | 1 |
| check256-line | BU | 157.240170 / 159.054096 | 2624 | 268435456 / 0 | 1 |
| range-apply256 | A | 213.753077 / 215.745051 | 2632 | 268435456 / 268435456 | 1 |
| range-apply256 | BH | 103.096221 / 105.251717 | 2648 | 268435456 / 268435456 | 1 |
| range-apply256 | BU | 214.012574 / 215.728413 | 2676 | 268435456 / 268435456 | 1 |
| resident-anchored-view | A | 0.006076 / 0.007812 | 2620 | 19 / 0 | 1 |
| resident-anchored-view | BH | 0.007011 / 0.010163 | 2640 | 20 / 0 | 1 |
| resident-anchored-view | BU | 0.006057 / 0.008513 | 2608 | 19 / 0 | 1 |
| resident-apply | A | 0.026146 / 0.028158 | 2624 | 49 / 43 | 1 |
| resident-apply | BH | 0.026826 / 0.032202 | 2632 | 49 / 43 | 1 |
| resident-apply | BU | 0.025942 / 0.029787 | 2640 | 49 / 43 | 1 |
| resident-check | A | 0.004264 / 0.004915 | 2624 | 19 / 0 | 1 |
| resident-check | BH | 0.001308 / 0.002155 | 2576 | 0 / 0 | 1 |
| resident-check | BU | 0.004245 / 0.004819 | 2640 | 19 / 0 | 1 |
| resident-view | A | 0.004081 / 0.004789 | 2620 | 19 / 0 | 1 |
| resident-view | BH | 0.004817 / 0.005990 | 2640 | 20 / 0 | 1 |
| resident-view | BU | 0.004378 / 0.004908 | 2640 | 19 / 0 | 1 |
| search-apply | A | 0.033916 / 0.038931 | 2624 | 68 / 43 | 1 |
| search-apply | BH | 0.035119 / 0.036862 | 2640 | 68 / 43 | 1 |
| search-apply | BU | 0.035727 / 0.039778 | 2616 | 68 / 43 | 1 |
| search-check256 | A | 426.071260 / 427.336644 | 2616 | 536870912 / 0 | 1 |
| search-check256 | BH | 268.246839 / 269.643193 | 2624 | 268435456 / 0 | 1 |
| search-check256 | BU | 426.136446 / 427.767626 | 2632 | 536870912 / 0 | 1 |
| search-view256 | A | 594.154296 / 597.705237 | 2616 | 536870912 / 0 | 1 |
| search-view256 | BH | 1079.942654 / 1096.361926 | 2644 | 536870913 / 0 | 1 |
| search-view256 | BU | 609.897790 / 614.720657 | 2624 | 536870912 / 0 | 1 |
| search128-line | A | 135.170704 / 135.720668 | 2560 | 134217728 / 0 | 1 |
| search128-line | BH | 134.391363 / 136.651819 | 2624 | 134217728 / 0 | 1 |
| search128-line | BU | 134.202285 / 135.758530 | 2640 | 134217728 / 0 | 1 |
| search1m-line | A | 516.579029 / 518.546321 | 59884 | 4194304 / 0 | 1048576 |
| search1m-line | BH | 518.399520 / 556.561916 | 59920 | 4194304 / 0 | 1048576 |
| search1m-line | BU | 517.818931 / 528.658144 | 59912 | 4194304 / 0 | 1048576 |
| search256-line | A | 268.721977 / 288.777132 | 2644 | 268435456 / 0 | 1 |
| search256-line | BH | 268.378671 / 269.634286 | 2624 | 268435456 / 0 | 1 |
| search256-line | BU | 268.333155 / 269.681022 | 2640 | 268435456 / 0 | 1 |
| view256-file | A | 200.575047 / 202.222399 | 264828 | 268435456 / 0 | 1 |
| view256-file | BH | 76.932331 / 77.783033 | 264716 | 268435456 / 0 | 1 |
| view256-file | BU | 201.499711 / 203.235894 | 264848 | 268435456 / 0 | 1 |
| view256-line | A | 324.568132 / 343.652700 | 2624 | 268435456 / 0 | 1 |
| view256-line | BH | 810.848473 / 811.247647 | 2644 | 268435457 / 0 | 1 |
| view256-line | BU | 338.977338 / 340.314927 | 2640 | 268435456 / 0 | 1 |
| view256-paragraph | A | 200.727033 / 201.942698 | 264816 | 268435456 / 0 | 1 |
| view256-paragraph | BH | 78.726507 / 79.718922 | 264788 | 268435456 / 0 | 1 |
| view256-paragraph | BU | 199.413292 / 201.470564 | 264852 | 268435456 / 0 | 1 |

### Semantic checksums

Every checksum below is identical across A, BU, and BH for that cell.

| Cell | Result SHA-256 | Final source SHA-256 |
| --- | --- | --- |
| changed-chain | `849cb10a9ba5436e39ebd89346bcfff016db779a4bc3bc4d209c6c6277431bc7` | `9149a1639fd729ca74b4353844d37528182883bc3b68bda8c864cd7064dd1043` |
| check256-line | `e07cecbbccb14cf0005fe291d6d5423562b45c77439253afdce94e642b415366` | n/a |
| range-apply256 | `d4edd123621cf230590d7812e64bec69460789eba3e0c7136b88a3f26c88f5e5` | `d4edd123621cf230590d7812e64bec69460789eba3e0c7136b88a3f26c88f5e5` |
| resident-anchored-view | `c45d419ebd6901a417413422db0de9bdae624691734616ab1f95d57b35d13cbe` | n/a |
| resident-apply | `2e86294b88ae347ebddb6d982a6d5cfe71696d420978db34fcb715714736ef25` | `2e86294b88ae347ebddb6d982a6d5cfe71696d420978db34fcb715714736ef25` |
| resident-check | `764b929f67af1ae32a06fbbb79759517f1a6e9fb7a0a09e28977dbb0fd159ef0` | n/a |
| resident-view | `c45d419ebd6901a417413422db0de9bdae624691734616ab1f95d57b35d13cbe` | n/a |
| search-apply | `2e86294b88ae347ebddb6d982a6d5cfe71696d420978db34fcb715714736ef25` | `2e86294b88ae347ebddb6d982a6d5cfe71696d420978db34fcb715714736ef25` |
| search-check256 | `e07cecbbccb14cf0005fe291d6d5423562b45c77439253afdce94e642b415366` | n/a |
| search-view256 | `a364fdada45de1d89cc5b38bef4d38ff4b9395acd442cbeed55774925db3035d` | n/a |
| search128-line | `2d08f65ab1acc3c61f2c48dfa8783ac77948f6475e417c9ac3a6d3585766912e` | n/a |
| search1m-line | `d6d01924894bf28d4b1a6bae7d27aae636dce4917cd1ab024bcb965dd6a827ef` | n/a |
| search256-line | `db49038b7cb1e21ce03ebd5946bebfbb7cbc968a410d771241f2e1ea53e7d11d` | n/a |
| view256-file | `feb08e0a27f970109901c8915bccdbedcfd019b7b96f968fcda06c72e9fea220` | n/a |
| view256-line | `a364fdada45de1d89cc5b38bef4d38ff4b9395acd442cbeed55774925db3035d` | n/a |
| view256-paragraph | `4d7abdd29d4feaf593815b9222db5e5dfd792f018024ef3b820857e17255f2de` | n/a |

### Raw samples

Raw rows remain in execution-round order within each variant. CPU ticks use the
host's 100 Hz process clock. The complete machine TSV contained 384 rows: 48
warm-ups plus the 336 rows below.

| Cell | Variant | Raw inner wall ms | Raw outer wall ms | Raw user/system ticks | Raw HWM KiB | `rchar` / `wchar` |
| --- | --- | --- | --- | --- | --- | --- |
| changed-chain | A | 0.041875, 0.043479, 0.046957, 0.042583, 0.120964, 0.050737, 0.053386 | 0.049326, 0.051462, 0.054756, 0.050522, 0.129439, 0.059925, 0.062506 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2532, 2620, 2616, 2552, 2620, 2552, 2616 | 37 / 25 |
| changed-chain | BH | 0.039319, 0.039310, 0.040151, 0.041774, 0.041433, 0.049039, 0.049955 | 0.047685, 0.047066, 0.047697, 0.050305, 0.049843, 0.058679, 0.059289 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2548, 2560, 2572, 2624, 2560, 2624, 2632 | 31 / 25 |
| changed-chain | BU | 0.042835, 0.044411, 0.043528, 0.043989, 0.047411, 0.048038, 0.054934 | 0.050584, 0.052867, 0.051416, 0.052273, 0.056046, 0.056276, 0.065282 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2548, 2568, 2572, 2568, 2560, 2568, 2572 | 37 / 25 |
| check256-line | A | 157.743861, 157.125296, 157.848530, 158.092119, 159.295319, 155.302011, 157.929554 | 157.860814, 157.183507, 157.902059, 158.189887, 159.388511, 155.397448, 158.030083 | 13/2, 12/3, 13/3, 13/2, 13/2, 12/3, 12/3 | 2560, 2624, 2556, 2528, 2604, 2624, 2520 | 268435456 / 0 |
| check256-line | BH | 0.002656, 0.002605, 0.004583, 0.004077, 0.002382, 0.004606, 0.004407 | 0.019847, 0.013905, 0.092202, 0.113103, 0.017569, 0.055313, 0.099102 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2576, 2568, 2636, 2560, 2636, 2572, 2612 | 0 / 0 |
| check256-line | BU | 157.211156, 157.324002, 156.048120, 157.240170, 157.642655, 156.910941, 159.054096 | 157.266318, 157.378928, 156.140082, 157.296049, 157.738020, 157.001173, 159.073210 | 12/4, 11/4, 12/3, 12/3, 13/3, 11/4, 12/4 | 2544, 2624, 2576, 2560, 2576, 2576, 2556 | 268435456 / 0 |
| range-apply256 | A | 214.235367, 213.302692, 213.408680, 214.717588, 212.896937, 213.753077, 215.745051 | 214.331345, 213.360178, 213.502557, 214.768533, 212.955426, 213.811118, 215.847612 | 12/8, 12/9, 12/10, 13/9, 13/9, 12/10, 12/10 | 2568, 2560, 2536, 2568, 2632, 2632, 2568 | 268435456 / 268435456 |
| range-apply256 | BH | 102.637646, 105.251717, 102.396103, 103.096221, 103.228494, 102.169232, 103.782021 | 102.731910, 105.342823, 102.489514, 103.153875, 103.340248, 102.226459, 103.877533 | 2/9, 1/9, 2/8, 1/9, 2/9, 1/9, 1/9 | 2648, 2564, 2648, 2648, 2644, 2584, 2644 | 268435456 / 268435456 |
| range-apply256 | BU | 212.877526, 214.939197, 213.307731, 213.358369, 215.728413, 214.012574, 214.609180 | 212.915203, 215.030887, 213.401533, 213.414635, 215.784725, 214.107357, 214.665120 | 12/9, 12/10, 12/9, 12/9, 13/9, 13/8, 15/7 | 2676, 2632, 2632, 2572, 2640, 2552, 2564 | 268435456 / 268435456 |
| resident-anchored-view | A | 0.004928, 0.005377, 0.006076, 0.005813, 0.007128, 0.007812, 0.007427 | 0.014574, 0.017183, 0.023629, 0.018243, 0.021776, 0.023336, 0.023236 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2560, 2600, 2560, 2608, 2560, 2620, 2528 | 19 / 0 |
| resident-anchored-view | BH | 0.005779, 0.006185, 0.006124, 0.007011, 0.007601, 0.008180, 0.010163 | 0.016000, 0.017930, 0.018290, 0.020103, 0.020759, 0.023213, 0.037872 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2544, 2576, 2576, 2568, 2640, 2608, 2576 | 20 / 0 |
| resident-anchored-view | BU | 0.005529, 0.005811, 0.006057, 0.005777, 0.007111, 0.007511, 0.008513 | 0.016321, 0.016905, 0.183482, 0.017456, 0.021330, 0.021861, 0.025880 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2576, 2576, 2572, 2560, 2608, 2576, 2572 | 19 / 0 |
| resident-apply | A | 0.027482, 0.026146, 0.024722, 0.024746, 0.024978, 0.026894, 0.028158 | 0.035046, 0.076298, 0.032086, 0.032878, 0.032683, 0.035581, 0.037470 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2560, 2544, 2544, 2544, 2556, 2624, 2604 | 49 / 43 |
| resident-apply | BH | 0.026986, 0.026826, 0.024235, 0.024926, 0.025491, 0.030064, 0.032202 | 0.034595, 0.035207, 0.031402, 0.032184, 0.033269, 0.038326, 0.041289 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2624, 2576, 2544, 2572, 2560, 2632, 2576 | 49 / 43 |
| resident-apply | BU | 0.028901, 0.026543, 0.025735, 0.024749, 0.024884, 0.025942, 0.029787 | 0.037398, 0.034571, 0.033551, 0.032367, 0.032880, 0.034245, 0.038604 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2608, 2632, 2576, 2576, 2544, 2640, 2640 | 49 / 43 |
| resident-check | A | 0.004264, 0.004110, 0.004071, 0.003936, 0.004391, 0.004293, 0.004915 | 0.012186, 0.011516, 0.011139, 0.011953, 0.012509, 0.012263, 0.013021 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2556, 2556, 2608, 2608, 2532, 2544, 2624 | 19 / 0 |
| resident-check | BH | 0.001274, 0.001240, 0.002155, 0.001622, 0.001338, 0.001280, 0.001308 | 0.008675, 0.009473, 0.009314, 0.009561, 0.009618, 0.008802, 0.009751 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2576, 2576, 2576, 2576, 2576, 2576, 2564 | 0 / 0 |
| resident-check | BU | 0.004162, 0.004134, 0.004358, 0.004541, 0.004819, 0.004245, 0.003994 | 0.011559, 0.011550, 0.012265, 0.012620, 0.011985, 0.012441, 0.011816 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2576, 2556, 2620, 2576, 2568, 2556, 2640 | 19 / 0 |
| resident-view | A | 0.004050, 0.004295, 0.003931, 0.004081, 0.004789, 0.004023, 0.004132 | 0.011888, 0.012032, 0.011295, 0.012226, 0.011918, 0.011736, 0.012756 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2604, 2540, 2556, 2616, 2620, 2560, 2552 | 19 / 0 |
| resident-view | BH | 0.005990, 0.004817, 0.005080, 0.004600, 0.004647, 0.005241, 0.004770 | 0.013822, 0.012542, 0.012459, 0.012104, 0.012468, 0.013012, 0.012634 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2568, 2576, 2576, 2640, 2576, 2576, 2632 | 20 / 0 |
| resident-view | BU | 0.004378, 0.004908, 0.004319, 0.004070, 0.003995, 0.004714, 0.004668 | 0.011786, 0.013030, 0.012032, 0.011990, 0.011954, 0.012786, 0.012700 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2640, 2576, 2568, 2576, 2560, 2576, 2576 | 19 / 0 |
| search-apply | A | 0.034581, 0.033916, 0.031370, 0.032885, 0.038931, 0.033940, 0.031785 | 0.042241, 0.041341, 0.038673, 0.040966, 0.048181, 0.042219, 0.039713 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2556, 2560, 2540, 2560, 2560, 2544, 2624 | 68 / 43 |
| search-apply | BH | 0.035336, 0.032299, 0.036492, 0.036862, 0.034107, 0.035119, 0.032956 | 0.043131, 0.040577, 0.045224, 0.045767, 0.041866, 0.043144, 0.040641 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2552, 2560, 2580, 2640, 2632, 2576, 2572 | 68 / 43 |
| search-apply | BU | 0.035727, 0.033362, 0.037319, 0.034181, 0.039778, 0.034411, 0.036026 | 0.043787, 0.041061, 0.044851, 0.042714, 0.048209, 0.042464, 0.044678 | 0/0, 0/0, 0/0, 0/0, 0/0, 0/0, 0/0 | 2568, 2568, 2576, 2616, 2576, 2560, 2580 | 68 / 43 |
| search-check256 | A | 425.708361, 427.336644, 424.028460, 426.330460, 427.182158, 426.019574, 426.071260 | 425.760200, 427.391204, 424.084670, 426.347824, 427.235184, 426.072752, 426.123665 | 35/6, 35/6, 36/6, 36/6, 35/7, 35/7, 37/4 | 2544, 2544, 2616, 2536, 2544, 2560, 2552 | 536870912 / 0 |
| search-check256 | BH | 267.654484, 268.246839, 267.751548, 267.965833, 269.643193, 268.478755, 268.371507 | 267.707026, 268.300967, 267.801013, 268.028459, 269.700952, 268.498172, 268.424055 | 23/3, 23/3, 23/3, 24/2, 24/2, 24/2, 23/3 | 2560, 2576, 2588, 2556, 2576, 2624, 2576 | 268435456 / 0 |
| search-check256 | BU | 426.846676, 426.136446, 427.767626, 423.510081, 426.071814, 424.098362, 426.460063 | 426.903710, 426.188400, 427.826071, 423.563815, 426.124303, 424.151491, 426.478774 | 37/4, 36/5, 36/6, 36/5, 36/5, 36/5, 35/6 | 2576, 2572, 2576, 2572, 2576, 2576, 2632 | 536870912 / 0 |
| search-view256 | A | 595.340713, 597.705237, 595.495232, 593.761915, 591.306400, 593.358686, 594.154296 | 595.392785, 597.757484, 595.521709, 593.813909, 591.322049, 593.374150, 594.203714 | 53/6, 53/6, 52/6, 52/6, 52/6, 54/5, 54/4 | 2556, 2548, 2532, 2616, 2556, 2544, 2564 | 536870912 / 0 |
| search-view256 | BH | 1085.845941, 1096.361926, 1079.393750, 1077.134990, 1079.113160, 1079.942654, 1080.785011 | 1085.921776, 1096.379567, 1079.445496, 1077.186985, 1079.167687, 1079.958258, 1080.836707 | 101/7, 102/7, 101/6, 100/7, 101/6, 101/6, 101/6 | 2644, 2572, 2580, 2644, 2580, 2644, 2596 | 536870913 / 0 |
| search-view256 | BU | 603.128839, 614.720657, 609.897790, 612.566471, 608.981048, 610.217599, 607.306518 | 603.180897, 614.737700, 609.950217, 612.617203, 609.030453, 610.232982, 607.325077 | 54/5, 54/6, 53/7, 54/6, 53/6, 54/6, 55/5 | 2556, 2580, 2580, 2564, 2576, 2524, 2624 | 536870912 / 0 |
| search128-line | A | 134.702781, 135.170704, 135.493199, 135.720668, 135.268666, 134.228038, 134.979571 | 134.754625, 135.221185, 135.545960, 135.775634, 135.319928, 134.281959, 135.032113 | 11/1, 12/1, 11/1, 11/1, 12/1, 12/1, 11/1 | 2544, 2560, 2560, 2552, 2560, 2556, 2560 | 134217728 / 0 |
| search128-line | BH | 133.859198, 136.651819, 136.207693, 135.567162, 134.317770, 134.391363, 133.593456 | 133.914285, 136.704789, 136.258790, 135.624558, 134.368693, 134.449041, 133.645813 | 11/1, 12/1, 12/1, 11/1, 12/1, 11/1, 11/1 | 2576, 2560, 2556, 2556, 2576, 2608, 2624 | 134217728 / 0 |
| search128-line | BU | 134.713039, 133.499339, 135.758530, 135.379372, 132.684608, 133.726911, 134.202285 | 134.792670, 133.554428, 135.809655, 135.398631, 132.752043, 133.778979, 134.253806 | 11/1, 12/1, 11/1, 11/1, 12/0, 11/1, 11/1 | 2576, 2640, 2572, 2560, 2576, 2576, 2580 | 134217728 / 0 |
| search1m-line | A | 518.183379, 517.179842, 513.890889, 518.546321, 514.826107, 516.579029, 515.206515 | 518.236627, 517.233669, 513.944020, 518.564341, 514.844596, 516.635411, 515.263427 | 51/0, 50/0, 50/1, 51/0, 50/0, 50/0, 50/0 | 59724, 59884, 59848, 59740, 59796, 59864, 59804 | 4194304 / 0 |
| search1m-line | BH | 518.399520, 518.948206, 519.344001, 556.561916, 517.531623, 517.637977, 516.125161 | 518.455110, 519.005589, 519.397823, 556.612833, 517.586424, 517.693605, 516.177756 | 50/0, 51/0, 50/0, 54/0, 50/0, 51/0, 50/0 | 59784, 59728, 59756, 59920, 59796, 59836, 59792 | 4194304 / 0 |
| search1m-line | BU | 517.818931, 515.427356, 516.867318, 520.656813, 515.848326, 518.101238, 528.658144 | 517.875026, 515.480219, 516.922187, 520.716833, 515.902073, 518.159536, 528.711852 | 50/0, 50/0, 50/0, 51/0, 50/0, 50/0, 52/0 | 59768, 59840, 59832, 59864, 59752, 59736, 59912 | 4194304 / 0 |
| search256-line | A | 268.721977, 269.079195, 268.323596, 268.099855, 269.085974, 268.054548, 288.777132 | 268.773134, 269.131426, 268.378390, 268.152983, 269.137973, 268.106954, 288.829875 | 24/2, 24/2, 23/3, 24/2, 23/3, 24/2, 23/3 | 2616, 2556, 2560, 2624, 2644, 2544, 2560 | 268435456 / 0 |
| search256-line | BH | 267.076907, 269.634286, 267.743989, 268.505782, 268.378671, 269.536110, 265.874403 | 267.133246, 269.685363, 267.796601, 268.560410, 268.432430, 269.591394, 265.928368 | 23/2, 23/3, 23/2, 24/2, 23/3, 24/2, 23/3 | 2576, 2576, 2560, 2576, 2576, 2576, 2624 | 268435456 / 0 |
| search256-line | BU | 268.484460, 266.708274, 268.569619, 268.101405, 265.545521, 268.333155, 269.681022 | 268.538435, 266.759094, 268.622298, 268.152523, 265.596357, 268.386248, 269.732246 | 23/2, 23/2, 24/2, 24/2, 23/2, 23/3, 24/2 | 2560, 2576, 2608, 2568, 2576, 2640, 2556 | 268435456 / 0 |
| view256-file | A | 199.302374, 202.222399, 199.361721, 201.502069, 201.165468, 200.575047, 199.931744 | 199.363592, 202.324958, 199.416826, 201.558619, 201.269601, 200.635651, 200.045157 | 14/5, 14/7, 15/5, 14/6, 14/6, 13/7, 13/6 | 264764, 264828, 264828, 264808, 264828, 264828, 264764 | 268435456 / 0 |
| view256-file | BH | 76.155314, 76.833722, 76.885550, 77.592820, 77.250369, 76.932331, 77.783033 | 76.210240, 76.933725, 76.940356, 77.612390, 77.311025, 76.953177, 77.875288 | 2/5, 2/5, 2/5, 2/5, 2/6, 2/6, 2/6 | 264716, 264684, 264716, 264716, 264684, 264716, 264708 | 268435456 / 0 |
| view256-file | BU | 202.346004, 202.478647, 197.664750, 200.563982, 203.235894, 201.499711, 197.493682 | 202.406727, 202.536603, 197.720563, 200.661908, 203.291588, 201.597612, 197.587631 | 13/7, 14/6, 15/5, 13/7, 15/5, 14/6, 14/5 | 264764, 264844, 264784, 264780, 264848, 264832, 264764 | 268435456 / 0 |
| view256-line | A | 322.736875, 321.994961, 324.693498, 320.742155, 324.568132, 343.652700, 324.985556 | 322.832605, 322.089045, 324.756857, 320.846911, 324.632528, 343.709330, 325.082783 | 30/2, 29/3, 28/5, 28/4, 30/2, 29/3, 29/3 | 2556, 2560, 2624, 2544, 2564, 2624, 2564 | 268435456 / 0 |
| view256-line | BH | 811.138648, 811.203174, 811.247647, 809.341271, 810.016317, 810.848473, 810.412303 | 811.239799, 811.300955, 811.387204, 809.443251, 810.118184, 810.939755, 810.508092 | 77/4, 78/3, 78/3, 78/2, 79/3, 77/3, 77/4 | 2560, 2580, 2576, 2572, 2584, 2564, 2644 | 268435457 / 0 |
| view256-line | BU | 339.173860, 336.864810, 336.179999, 338.620918, 339.482205, 338.977338, 340.314927 | 339.276004, 336.958233, 336.234049, 338.713651, 339.536070, 339.039950, 340.369777 | 31/2, 30/3, 31/2, 31/3, 30/4, 32/3, 31/3 | 2580, 2564, 2580, 2612, 2568, 2580, 2640 | 268435456 / 0 |
| view256-paragraph | A | 200.727033, 200.007966, 201.942698, 201.518009, 199.317419, 198.919412, 200.770617 | 200.824894, 200.100353, 202.027232, 201.610514, 199.371890, 198.943836, 200.832269 | 14/5, 14/6, 14/7, 15/6, 15/5, 16/4, 14/6 | 264740, 264772, 264816, 264772, 264740, 264764, 264764 | 268435456 / 0 |
| view256-paragraph | BH | 78.557894, 78.951215, 78.726507, 79.718922, 79.646573, 77.538039, 77.218658 | 78.616963, 79.008775, 78.826216, 79.811023, 79.734868, 77.593689, 77.314911 | 3/5, 2/6, 3/5, 2/5, 2/6, 2/5, 3/5 | 264708, 264724, 264780, 264772, 264732, 264724, 264788 | 268435456 / 0 |
| view256-paragraph | BU | 198.257978, 201.470564, 197.206923, 197.957881, 200.953966, 199.413292, 199.919426 | 198.316405, 201.572882, 197.308522, 198.055780, 201.009958, 199.478457, 200.019293 | 15/5, 14/6, 14/5, 15/5, 14/6, 13/7, 13/6 | 264836, 264788, 264828, 264756, 264852, 264784, 264788 | 268435456 / 0 |

### Reproduction identities and cleanup contract

| Item | SHA-256 |
| --- | --- |
| Harness source | `03b103b94b53ba64a4085a1aab8de51f547f26cfb38643cb51e2bd1a6dd92333` |
| Runner source | `debe3e52f2e4d8b401d5bae62434b149cc216539a01f873b15a4a6a8956a85a6` |
| Fixture generator | `e719df847498e307e26d739d869664f8cc69e4594025f20e0e5bec04178eb0df` |
| Matrix driver | `1b438810edc2d07e248673d2ac92bf735835ac64d15d619d7b679fb094ad1a91` |
| Analysis source | `3de54cc7a78a8bc910d9930a16aabfcaec650fcfe6b46bc7daa8f44e81266bd6` |
| Raw TSV | `ccbfa232d7f8ae834451e993edea78f7060c39fa20d384dca9f952b887ccd20c` |
| Summary TSV | `a09e983407a55c3b3d5926050f5111c62bf886b1de7b06bc95ce7caa5841d6f9` |
| Raw Markdown projection | `dd33a203181ee8246ef6de9fc65e8dc882003df5e003a74d2030a79ea2f52ac2` |
| Environment record | `794f96d026770f32d682ebd281ea1d5442ccc268bcc627f2d46a7a198d40b3b1` |
| A release `bw` | `1eb3b78bbae54b1895d5a73099deb0e7bf75508de94f7e03bdc8fb935c967fb3` |
| B release `bw` | `43d5f29ece52b66aae234acff9af84cecafdd034397caeb747048623f7a4e186` |
| A harness binary | `962d2932ea05daf52e146aeace7acd44cf171e63f47e85f5f8ec7d80f4eb569e` |
| BU harness binary | `150723061ee4097767899cab5ba1f0e8be7a66951ae6cbb98c1654e325d5e14e` |
| BH harness binary | `376082631a55dcb0dbe2b40bd0d8de93d23a734485a90b98f17fd19548918685` |

Both immutable exports pass offline/locked metadata, dependency tree, format,
all-target check, complete tests, and release builds. A passes 203 GNU-host
tests and B passes 234. The final source verification is recorded in the active
verification document. Task-local exports, targets, fixtures, binaries,
harnesses, runner, summaries, raw files, and the task root are removed after
their necessary evidence is recorded. No separate profiler binary or trace was
created; the formal memory/I/O gates use runner HWM/I/O plus the retained
production structural audits.
