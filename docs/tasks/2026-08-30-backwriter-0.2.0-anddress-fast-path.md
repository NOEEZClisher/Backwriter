# Backwriter 0.2.0 Anddress Fast Path

Status: Phase 1 completed; Phase 2 next; Phases 2–7 pending.

This is the sole progress tracker for the redesign. It records gates and
evidence but does not own semantics; the active Protocol, address model, and
principles do. Historical task evidence never overrides active authority.

## Goal and Owner intent

Keep the closed public `0.1.0` v3 release immutable while developing an
unpublished `0.2.0` v4 exact-source-state fast path. Search alone finds targets.
View, Check, and Apply consume an ordinary Anddress without searching,
reparsing to relocate, or context-matching an old target. Runtime may remember
only bounded current observation state, never history.

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
The source-state hash is final currentness authority. Its algorithm, exact wire
encoding/order, and v3/v4 compatibility or migration policy remain unresolved
Owner decisions.

An ordinary Anddress is immutable caller-owned authority for one exact source
state and range. A changed state invalidates it. Explicit Search may return a
new current Anddress; no consumer relocates the old one. Reappearance of the
same complete state may reproduce raw equality without proving continuity.

Anchor is the sole continuity exception. Only a live Runtime-local Anchor may
receive an arithmetic range transform across a successful Backwriter-owned
Apply. External or opaque source change invalidates continuity. Anchor does not
mutate an ordinary Anddress or add history, search, or context relocation.

`CurrentObservation` is Runtime-private current state. It may retain only the
current hash, exact length, and byte ranges minimally required by the current
capability or fast path. It is discarded when state changes or cannot be proven
current. It may not retain prior observations, whole-source bytes, a parse tree,
a complete Line collection, Search results, history, a persistent index,
relocation/context evidence, a full workspace cache, a watcher, or durability.

Capability responsibilities are fixed: Search hashes while discovering ranges
in its one source read and performs no separate hash pass; View validates hash,
length, and range and returns exact bytes; Check compares the source hash; Apply
requires a matching hash and validates the range against the recorded length
before patching. View, Check, and Apply do not search.

## Phase gates

| Phase | Entry gate | Completion gate |
| --- | --- | --- |
| 1. Authority | Closed clean `0.1.0` baselines, Owner docs-only authority, active-doc review. | This tracker and active semantics cover every guard, field, responsibility, exclusion, test, and benchmark gate; protected code/runtime state is unchanged; hash and compatibility remain open. |
| 2. Reproduce/profile/baseline | Phase 1 committed, pushed, and clean. | Drift-Wrong-Apply is executable; release-build profiles locate actual parse/hash/allocation/I/O cost; fixed fixtures, commands, host/toolchain facts, repeated raw results, and variance are recorded without improvement claims. |
| 3. V4 value/wire kernel | Phase 2 evidence plus Owner decisions for hash algorithm, compatibility/cutover, and any dependency. | One canonical v4 value implements validation, equality, encoding/decoding, checked arbitrary-size length/range arithmetic, error priority, KATs, and decided cutover without hidden ordinal/text identity. |
| 4. Search/observation | V4 kernel complete and all producers mapped. | Content and exact-File Search produce v4 state/ranges; hash is computed in the existing one read; bounded observation state and discard rules hold; ordering, multiplicity, fail-all, Unicode/terminators, admission, and no-limit behavior remain correct. |
| 5. View/Check | Search produces accepted v4 values. | View uses hash+range without target search/reparse; Check compares the source hash without search/refresh; duplicate, rewrite, bounds, unavailable, text-policy, and resource regressions pass; retired v3 currentness remains only if explicitly authorized. |
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
| CurrentObservation | Allowed fields only, minimum ranges, discard on mismatch/change/failure, and structural absence of history, whole source, parse tree, result store, index, or full workspace cache. |
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
- [ ] Phase 2 — reproduction, profile, and baseline (next)
- [ ] Phase 3 — v4 value and wire kernel
- [ ] Phase 4 — Search producer and `CurrentObservation`
- [ ] Phase 5 — View and Check consumers
- [ ] Phase 6 — Apply and Anchor cutover
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
- Phases 2–7: pending; Phase 2 must profile and record the v3 baseline before
  any v4 implementation.
