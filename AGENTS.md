# Repository operating guard

This standalone repository provides the Rust Backwriter Core, its required
target Runtime, and the canonical `bw` CLI Adapter. Product prose uses
`Backwriter`; the Cargo package, library crate, and Core namespace use
`backwriter`; external callers invoke `bw`, which adapts to `backwriter` Core.
Do not add a `backwriter` executable, alias, or wrapper. Persisted
`artext.backwriter-*` values, the `.artext/bw` private path, and distribution
artifact/domain names keep their existing contracts. The
owner-defined Core capability inventory is Search, View, Pick, Anchor, Check,
Edit, Apply, and Data. `S` is assigned to Search, `P` to Pick, `A` to
Anchor, `C` to Check, and `D` to Data. `I`, `R`, and Apply's reference letter
are unassigned. Inventory names do not define a lifecycle, call order,
payload, error model, or adapter behavior.

Search, View, Pick, Anchor, and Check have Rust implementations. Their v4
direct View source-state/range projection, target-specific Search literal
projection and exact logical File lookup, Pick predicate semantics, direct
Anchor target projection and live continuity, and hash/length-only Check batch
currentness reporting are implemented. Apply V1
semantic/public API/error authority and its single-source Edit Runtime
implementation uses direct v4 ranges and provenance and is complete. Data V1
semantic/public API/type/error authority and Rust implementation are complete.

The closed public `0.1.0` release is the immutable v3 baseline. The published
and closed `0.2.2` source, Cargo package, and official distribution, the prior
closed `0.2.1` distribution, and the prior closed public `0.2.0` release use the hard-cutover
`artext.backwriter-anddress.v4` Rust API and wire. SHA-256, exact source byte
length, target kind, and `[start, end)` byte range are implemented identity;
v3 remains only in Git history and immutable `0.1.0` release evidence. The
canonical four-target `0.2.2` artifacts, manifest, installers, exact 44-file
live publication, fresh installation, and explicit public `0.2.1` update are
complete from Source Authority revision
`04b36d9ca9cc725bedeb17231339c67b5f0590ea`.

`0.2.1` is published and closed. Phases 2 through 6 implement its
minimal Host-authoritative observation kernel, bounded ordinary View reuse, and
Check, Apply, and anchored View current-proof reuse plus complete invalidation
and race semantics while preserving v4 identity and the existing `0.2.0`
execution path as the default Untrusted Mode.
`WorkspaceRuntime::open_host_authoritative` explicitly selects that mode and
`WorkspaceRuntime::invalidate_source` is its host mutation boundary. The
Runtime may retain one private current SHA-256/length proof per logical source.
Ordinary View consumes a complete matching proof for bounded direct-range
access. Check consumes a matching path proof entirely in RAM and classifies the
whole coordinate/path group from hash and length without source open, read, or
hash work. Check never installs, replaces, or removes proof. Apply uses a
matching proof as its source-state precondition, stages the exact proof length
without a before hash, preserves it across an exact no-op, and installs the
already computed prospective-after hash and length only after confirmed changed
publication.
Both public source-invalidation seams share one I/O-free path-exact proof and
Anchor invalidator. Correct Host sequencing invalidates before every visible
mutation and excludes mutation through capability completion; unsignaled or
in-call mutation is a host contract violation rather than a supported race.
Matching anchored View reuses ordinary trusted View execution, while a proof
mismatch fail-closes the same-path proof and live Anchors before source access.
Phase 7B remeasures the complete A/Untrusted/Host matrix against immutable
`0.2.0` baseline `2fad6e4`; every formal performance, memory, I/O, semantic,
and drift gate passes. Source readiness alone authorized no artifact or
publication; the separate release closure made the closed `0.2.1` release from
Source Authority revision `4a1b06fb375bfd906a6f27de4de15a8febfe08ec` the
official distribution.

`0.2.2` is published and closed. Gates 1–6 close its authority,
minimum implementation, transport decision, user/AI surface, consumer
separation, integration, and version decision; Gate 7 closes its artifacts,
installers, manifest-last publication, and release verification.
The canonical general editing Adapter operation accepts one encoded v4
Anddress plus one replacement Content value and reuses v4
decode, Runtime View, existing `Edit::Replace`, Runtime Apply, and the
existing CLI status/error writers. File and Paragraph use exact replacement
Content. Line accepts body Content without NUL, CR, or LF and preserves the
current terminator returned by View. View and Check are not caller-visible
prerequisites; the Adapter's private View supplies Line terminator evidence and
Apply remains the sole currentness/publication authority. This adds no Core
API, Runtime seam, wire, target finder, relocation, retry, or compatibility
layer. Gate 5 retains three non-aliasing responsibilities: public Rust
Edit/Position/Apply is the exact low-level primitive, Session `let ... = edit
...`/`apply` is the advanced exact-byte and lifetime-composition surface, and
one-shot Anddress-first Edit is the canonical Replace contraction. None is
internal, deprecated, renamed, aliased, or wrapped by that separation.

`0.2.3` Patch Box Gates 1 and 2 are complete in source. Gate 1 closes authority
and the consumer matrix. Gate 2 carries each Search result as one
`SearchOccurrence` containing its exact v4 Anddress and same-observation
descriptive position: a one-based Line number, a one-based inclusive Paragraph
Line range, or no position for File. Check preserves this metadata while its
report remains raw-Anddress evidence; Data and Session retain the occurrence
carrier, while Pick continues to consume and return raw Anddresses. Human
Search displays current Line positions, and machine Search hard-cuts to the
Adapter-only `bw.cli.search.v2` occurrence projection; there is no production
v1 branch. None of that metadata is v4 identity, currentness evidence, a
selector, history, relocation, or a second source read. Cargo, CLI version,
artifacts, and publication remain closed `0.2.2`. Later gates may add only the
authorized upward View projections and one-shot Replace receipt. The ordered
gates and consumer evidence are tracked in
[Backwriter 0.2.3 Patch Box](docs/tasks/2026-09-03-backwriter-0.2.3-patch-box.md).

The repository cutline ends at public Rust Core, required Runtime, and the
implemented Backwriter CLI V1 Adapter-owned one-shot Version and Update,
one-shot human and JSON Search/View/Check, raw View, Anddress-first one-shot
Edit, Session Pick, batch Check, Anchor, Edit, Apply, result-binding, explicit
Data, and JSON Adapter. Version
and Update add no Core capability, Runtime seam, wire, or workflow authority.
Native wire,
AI/Context/Profile, client, MCP, product integration, and external consumer
work remain outside that cutline. Beyond the completed Session Pick, batch
Check, Anchor, Edit, Apply, result-binding, Data, Search/View/Check JSON, raw
View, Version, Update, and `0.2.2` Anddress-first one-shot Edit slice, every
other capability remains deferred Adapter work. `bw update` is an explicit
user-invoked installer handoff with no version comparison. It installs or
reinstalls the current official `0.2.2` release through installers that accept
only the exact closed `0.2.1` and current `0.2.2` manifests; it
creates no daemon, background updater, retry authority, or version-comparison
engine.

## Current structure only

Backwriter is not Git. It does not model or perform merge, branch, ancestry,
conflict resolution, history, rollback, or inheritance of past identity. Each
accepted observation establishes only the current File/Paragraph/Line structure.
A structural change creates no past-structure predecessor/successor/survivor
lineage or reconciliation mapping. Past-state recovery belongs to Git or another
external history system, never Backwriter.

For the closed `0.2.0` release and default Untrusted Mode, current-only is not
history: a capability may hold only the bounded call-local observation defined
by the Protocol while that call is running. The `0.2.1` Host-authoritative Mode
may retain only the Protocol's narrow current source-state proof; it creates no
target continuity or history. Search is the only capability that finds a
target; View, Check, and Apply consume an Anddress without searching or
relocating it.

## Active authority

Read active documents in this order before product work:

1. [Current state](docs/current/now.md)
2. [Roadmap](docs/current/roadmap.md)
3. [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)
4. [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)
5. [Backwriter principles](docs/principles/backwriter-core-principles.md)
6. [Verification](docs/development/verification.md)
7. [Backwriter CLI V1](docs/architecture/backwriter-cli-v1.md)

Only active documents define the target. `docs/tasks/**` and `docs/history/**`
are preserved evidence, never current authority.

## Ownership and invariants

- **Core Protocol** owns exact File/Paragraph/Line structure, target-local
  Anddress semantics, Search query matching and target projection, View
  currentness, and caller-neutral capability contracts.
- Capability composition is caller-owned. Shared native values and explicit
  caller value passing establish neither provenance nor a required capability
  call order or workflow; the Protocol's named cross-capability contracts
  remain in force.
- **Runtime** owns admission, internal safe enumeration, current-call source
  reads, the optional Host-authoritative current-proof boundary, and the V1
  publication seam;
  Anchor owns only its closed target-local RAM continuity contract.
- Workspace Source owns canonical source bytes. Runtime never creates
  repository-local `.artext` authority.
- An accepted current observation is the bytes returned by one retained
  no-follow read of currently admitted Workspace Source. Editor-only buffers,
  keystrokes, IME, undo, and dirty-state lifecycle are outside Core. In default
  Untrusted Mode, a source-visible write can affect a call only through that
  call's one-read observation. Host-authoritative reuse instead requires the
  host to coordinate every source-visible writer and path replacement, exclude
  mutation from the reuse decision through call completion, and synchronously
  invalidate before mutation. Save is not an automatic Runtime event and
  creates no watcher, automatic re-evaluation, durability, or stable-read
  guarantee.
- Anddress, admission, current-call parsing, and path safety are foundation
  mechanisms, not separate Core capability names.
- Admission has one fixed regular UTF-8-text policy. It is path-safe, rejects
  reserved/private aliases and symlinks, opens selected components
  capability-relatively without following links, and does not scan unselected
  roots.
- File, Paragraph, and Line are independent target addresses with structural
  relationships. In v4 they share exact source-state identity, so any source
  byte change invalidates every ordinary Anddress from the prior source state.
- Current production source uses only `artext.backwriter-anddress.v4`. Do not
  add v2/v3 compatibility shims, decoders, aliases, migrations, or parallel
  schemas. Well-formed v3 input is rejected as `UnsupportedVersion`.
- The `0.2.0` value is `artext.backwriter-anddress.v4`: workspace,
  logical path, source-state hash, exact byte length, target kind, and one
  inclusive-start/exclusive-end byte range are ordinary Anddress identity.
  Target text and ordinal are not v4 identity. The hash algorithm is SHA-256
  and the compatibility policy is a hard cutover with no v3 production seam.
- Whole-source bytes and provenance remain private call-local construction
  context. The v4 SHA-256 and byte length are ordinary source identity, and an
  Anddress is authority for that exact source state and byte range. This adds
  no before/after stable-read or second-read guarantee.
- The Protocol owns the bounded source-memory direction. It removes only
  unnecessary auxiliary materialization proportional to complete Workspace
  Source; it creates no fixed-memory or arbitrary-input-success promise. Its
  Check, Search, View, Anchor, Apply streaming slices are complete.
- Backwriter Core constructs and provides target Anddress values from an
  accepted current observation. Search delivers them as results; it is not an
  issuer. This creates no target registry, issuance lifecycle, locator reuse,
  durable identity, or global identity. The optional `0.2.1` proof is only
  current SHA-256/length evidence, never target lookup or retained results.
- The [address model](docs/architecture/rebuildable-structural-addressing.md)
  is the sole detailed raw-locator contract. Its active production algebra is
  v4 source-state/range identity; the shipped v3 baseline is historical release
  evidence only. Admission decides construct/use availability, not raw equality.
- Search and View remain current-only. Default Untrusted Mode permits only
  Protocol-bounded, source-local `CurrentObservation` state during one call.
  Explicit Host-authoritative Mode may retain only the Protocol's Runtime-local,
  replace-only SHA-256/length proof bound to workspace, admission, source
  generation, and logical path. Pick remains pure and stateless over
  caller-provided Anddress values without asserting currentness.
  `WorkspaceRuntime` exposes Search, View, Apply, Check, and anchored Runtime
  execution seams. Runtime retains no ordinary observation, source bytes,
  result, target map, snapshot, lease, or history across calls or selected
  sources. The private Host proof is the sole exception. `CurrentObservation`
  contains only the current hash and exact byte
  length and is consumed or discarded before Search opens another source or any
  View, Check, Apply, or Anchor call returns. A successful Host Search may
  replace the narrow trusted proof without retaining its projection. Anchor
  may retain only target-local session continuity.
- Future spill belongs only to a host-provided system root. This repository does
  not create `.artext`; the exact Runtime-root-relative `.artext/bw` path and
  its descendants are ignored by Backwriter Runtime execution. Other `.artext`
  children remain ordinary Workspace Source subject to the normal safety policy.
- Search is read-only discovery over admitted live Workspace Source inside a
  structured scope. Every call scans live source directly; it creates or uses
  no persistent index, global snapshot, result cache, history, or past-result
  completeness evidence. A successful Host Search may install only independent
  current source proofs after the whole call succeeds.
- Search matches at Line-content granularity. Its requested target kind is
  Line, Paragraph, or File and changes only returned Anddress granularity.
  Separator Lines have no Paragraph.
- Search also accepts a distinct exact logical File request. It validates one
  logical path, observes that admitted regular source under the same UTF-8/NUL
  and no-follow policy, and returns its File Anddress without content matching
  or Line framing.
  Missing paths and directories are Empty; the operation creates no empty
  query, synthetic Line or Paragraph, scope traversal, index, or cache.
- Search's live scan, matching, ordering, all-or-nothing behavior, and
  no-fixed-limit contract remain valid. It constructs v4 values directly from
  the current coordinate, logical path, one-read SHA-256/length, kind, and range.
  Each result is one `SearchOccurrence`; its optional `SearchPosition` is
  descriptive output calculated by the same Line framing pass and is not an
  Anddress field or matching/currentness input.
  Search owns no registry, persistent identity, mutation lifecycle, or result
  store.
- In the v4 target, Search remains the only target finder. It computes the
  source-state hash during its one source read, without a separate hash pass,
  and its File, Paragraph, and Line projections retain only target-required
  matching and range state before constructing exact byte ranges. View validates
  source hash and length while capturing the caller range, Check compares only
  source hash and length, and Apply enforces the hash
  precondition before patching the public v4 range directly;
  none searches, reparses to relocate, context-matches, or retries an old
  target. Any source-state change invalidates an ordinary Anddress. Only Anchor
  may transform a live range arithmetically across a Backwriter-owned Apply.
- **Apply V1** has closed semantic/public API/error authority:
  Runtime-controlled Apply is the continuity-preserving editor Save path, while
  independent editor, CLI, and external writes are opaque mutations. Its
  single-source Edit Runtime implementation is complete; it creates no registry, retry,
  watcher, or automatic creation of a new `Anchedress` or `AnchorOutcome`;
  existing live Anchor continuity is reflected by direct range/provenance
  projection under the Protocol. Check V1
  semantic/API/type/report authority and
  Rust implementation are complete; Data V1 semantic/public API/type/error
  authority and Rust implementation are complete; Edit V1 semantic/public
  API/type/error authority and single-source Apply Runtime implementation are
  complete, and Apply's
  reference letter remains
  unassigned. Anchor and anchored seams are implemented after Apply.
  Concurrent-writer coordination is caller-owned; do not add Apply locks, CAS,
  serialization, conflict detection, or retries without explicit owner
  authority.
  In `0.2.1` Host-authoritative Mode, confirmed publication may replace a
  matching old proof with the already computed prospective-after SHA-256 and
  length, while an exact no-op preserves it. A proof mismatch rejects before
  source access without changing proof or Anchor state. Unavailable or
  uncertain source state discards the affected proof; publication uncertainty
  also fail-closes every same-path live Anchor. Untrusted execution and proof
  misses retain the complete `0.2.0` before-observation path.
- **Anchor** has closed live-continuity authority and an implemented public
  surface. It retains only opaque owning Runtime-local
  continuity, non-aliasing `AlreadyLive`, no history/persistence/
  re-identification, and logical-source invalidation. The A0–A2 source-wide
  transition model is retired. Anchor cannot infer a successor from changed
  structure or automatically Anchor a resulting current target. It considers
  only source-visible mutations, not editor-only buffers; this adds no Save
  event, watcher, or generic file-change inference.
- **Pick** is a read-only, pure Core capability over caller-provided valid
  Anddress values. It consumes an ordered candidate Vec and returns an
  input-order-preserving stable subsequence with multiplicity unchanged. It
  creates no target or Anddress, reads no text, Workspace, file, or Runtime
  state, validates no input, calls no capability, and retains no state or
  result. All, target kind, full-value OneOf, iterative boolean composition,
  and v4 `same_file` remain valid. `same_file` compares only WorkspaceCoordinate
  and LogicalPath; no relation enum or observation/paragraph/hierarchy relation
  exists in v4. It makes no activity, staleness, availability,
  authenticity, identity, issuance, reuse, registry, lifecycle, ranking,
  display, mutation, proposal, Apply, or retry claim. Search Found values may
  be caller input, but Pick does not interpret SearchOutcome, text, preview, or
  adapter payload.
- **View V1** has an implementation. Its Runtime seam is
  `WorkspaceRuntime::view(&Anddress) -> Result<ViewOutcome, ViewError>`; the
  input is exactly one Anddress, without a wrapper, collection, range, or
  selector. It has no query, ranking, mutation, display, adapter payload, or
  retained source/result/relation state, registry, cache, history, snapshot, or
  lease. Pick may supply the input, but View neither calls Pick nor proves Pick
  provenance; explicit selection is the caller's responsibility.
- View's admitted no-follow one-read access checks v4 coordinate/path,
  source-state hash/length, and exact range, constructing related results from
  the same read.
- Plural input, ranges, descendants, and partial behavior are post-V1 owner
  decisions. View does not classify input state; Check does not change View.
- **Check V1** has closed semantic/API/type/report authority and a stateless
  result/history contract. Default execution remains the `0.2.0` observation
  path. A Host-authoritative matching path proof classifies every occurrence in
  that source group by hash and length without filesystem access; mismatches do
  not fall back or alter proof. The optional trusted current proof is the sole
  cross-call exception and is not a Check result store. **Data V1**
  semantic/public API/type/error authority and Rust implementation are complete.
  Edit V1 semantic/public API/type/error authority and single-source Apply
  Runtime implementation are complete. Apply has closed V1 semantic/public
  API/error authority and Runtime implementation. Apply's reference letter is
  unassigned. Read is retired; `R` is unassigned and its historical meaning
  transfers to no capability. Block is historical wording for the existing
  blank-line-bounded Paragraph and introduces no type, alias, variant, or wire
  value.
- Implemented v4 raw File, Paragraph, and Line equality follows exact workspace,
  path, source SHA-256, byte length, target kind, and byte-range algebra. It
  makes no continuity, survivor, relocation, or historical-identity claim.

## Search execution contract

- The implemented Runtime execution seams are
  `WorkspaceRuntime::search(&SearchRequest)`,
  `WorkspaceRuntime::view(&Anddress)`, and
  `WorkspaceRuntime::apply(&mut self, &Edit)`,
  `WorkspaceRuntime::check(Anddress)`, `check_search(SearchOutcome)`, and
  `check_pick(PickOutcome)`.
  `WorkspaceRuntime::open_host_authoritative` explicitly selects Host mode;
  `WorkspaceRuntime::invalidate_source` is its pre-mutation source boundary.
  There is no public Runtime enumeration or listing API. Core owns the validated
  content and exact-File Search requests, scope, query, target, outcome, and
  error types.
- Scope is all admitted roots or a nonempty narrowing-only list of subtree and
  source selectors. Selectors use safe platform-neutral UTF-8 logical paths,
  reject explicit `.`, duplicates, cross-kind same paths, and every
  component-boundary overlap. Admission roots and selector paths have no
  semantic length or count limit; they are fully admission-validated before
  filesystem I/O.
- A content query is a nonempty literal UTF-8 value without NUL, CR, or LF.
  Search performs case-sensitive contiguous matching on exact Line content
  without trimming, normalization, folding, token, regex, fuzzy, or semantic
  behavior.
- An exact File request has one validated logical path and no query, target
  projection, or scope selector. Only a currently admitted regular UTF-8,
  NUL-free source returns one File Anddress; missing paths and directories
  return Empty.
- Search is all-or-nothing. The outcome is Empty or Found Anddresses; invalid
  input is rejected before I/O, and invalid scope or unavailable source discards
  the entire result. There is no partial, truncated, cursor, or paginated
  outcome.
- Runtime traversal is single-threaded deterministic DFS over byte-sorted roots,
  selectors, and names. It consumes each retained no-follow source handle in one
  forward observation and retains no complete source or complete corpus.
- Core and Runtime have no arbitrary fixed semantic maximum for a valid text,
  source, query, path, scope, collection, workspace, result, Line count,
  encoded size, or traversal depth. Size alone must not make a finite valid
  input rejection solely from fixed numeric size, nor cause silent skipping,
  truncation, or false completeness. Fixed streaming chunks and digest widths
  are not input maxima. The caller-owned `Found` vector is the public result
  representation; Runtime keeps no result store after the call.

Core and Runtime contain no model, prompt, agent, or tool-use policy.

## Engineering guard

Do not change Rust Anddress, Search projection, Pick predicates, or Anchor
without owner authority. Do not preserve rejected v2 equality
through an adapter, compatibility alias, parallel schema, or source-wide
transition layer.
Do not implement the past-structure machinery prohibited above.

Preserve unrelated user changes. Do not reset, stash, clean, or rewrite history.
Use `rg` for discovery and `apply_patch` for edits and deletions. Keep temporary
output untracked. Do not read, delete, or migrate host-local historical state.
Before each implementation slice, use `rg` to audit production reachability,
duplication, and obsolete or temporary seams. Delete safely removable code
first; do not preserve retired code through compatibility shims, aliases, or
new abstractions. Report every retained item with its actual consumer and
reason. Agents may stage, commit, and push only when the owner explicitly names
those mutations and paths for the current task. Force push, history rewrite,
branch creation or switching, PR, Actions, `gh`, publish, deploy, release, and
upload remain forbidden. Before handoff, run
[verification](docs/development/verification.md), confirm `.artext` is absent
and untracked, and report exact deltas and remaining work.

For owner-directed unstaged handoff, the agent leaves the Git index empty.
