# Backwriter 0.2.1 Current-Observation Reuse

Status: Phases 1–3 complete; Check/Apply/Anchor trusted consumption,
measurement, version change, and publication not started.

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
4. **Check trusted hit — pending.** Reuse matching proof with zero
   source-size-proportional I/O/hash while preserving reports, order, and
   multiplicity.
5. **Apply and Anchor integration — pending.** Enforce proof preconditions,
   preserve exact no-op proof, install confirmed prospective-after proof, and
   share existing Anchor invalidation/publication fail-closure.
6. **Invalidation and semantic closure — pending.** Prove host mutation guards,
   explicit/opaque invalidation, races, authority drift, fallback, and the full
   Correct/Safe-Reject/Wrong matrix.
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
- Check, Apply, and Anchor proof consumption remains deferred to Phases 4–5;
  Phase 6 retains the complete invalidation/race closure.
