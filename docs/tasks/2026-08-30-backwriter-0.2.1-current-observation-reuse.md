# Backwriter 0.2.1 Current-Observation Reuse

Status: Phases 1–6 complete; measurement, version change, and publication not
started.

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
7. **Fixed A/B and release-readiness decision — pending.** Reproduce fixtures,
   compare against the fixed `0.2.0` inputs, decide GO/NO-GO, and only on GO
   change source version to `0.2.1`. Publication remains separate authority.

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
  hook was added. Phase 7 alone retains measurement, version, and
  release-readiness decisions; Phase 6 makes no performance claim.
