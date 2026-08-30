# Backwriter 0.2.1 Current-Observation Reuse

Status: Phase 1 authority and current-flow audit complete; implementation,
measurement, version change, and publication not started.

This tracker records execution evidence and phase progress only. Normative
semantics belong to the active
[Protocol](../architecture/backwriter-text-coordination-protocol.md),
[address model](../architecture/rebuildable-structural-addressing.md), and
[principles](../principles/backwriter-core-principles.md). The closed public
`0.2.0` release and its 28-file publication are immutable. `0.2.1` remains
an unimplemented and unpublished target using the same Anddress v4 wire,
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
2. **Minimal host kernel and Search installation — pending.** Choose the
   smallest explicit host mode/invalidation seam and private proof state, then
   allow successful Search observation to install proof.
3. **View bounded reuse — pending.** Add trusted direct-range View and a bounded
   related-Paragraph path without whole-source or complete-Line retention.
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

## Phase 2 open decisions

Phase 1 deliberately does not name or choose:

- a constructor, mode, token, invalidation, or getter API;
- the private state container, cardinality, or eviction policy;
- retained handle versus reopen behavior;
- multi-source Search proof installation policy;
- the related Paragraph bounded lookup mechanism.

Phase 2 must audit direct consumers and race boundaries before making the
minimum choice. It must not introduce a watcher, metadata proof, result cache,
target registry, history, context matching, retry, global snapshot, new wire,
or compatibility layer.
