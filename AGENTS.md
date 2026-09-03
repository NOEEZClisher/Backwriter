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

Search, View, Pick, Anchor, and Check have Rust implementations. Current source
is published and closed through Gate 8 with the v5 Anddress algebra, wire, shared
structural cursor, direct Search result collection, geometry-driven
single/batch View, and View-free one-shot Edit, with source-state-only Check
classification. Their direct View source-state/range projection,
target-specific Search literal
projection and exact logical File lookup, Pick predicate semantics, direct
Anchor target projection and live continuity, and source-state Check batch
currentness reporting are implemented. Apply V1
semantic/public API/error authority and its single-source Edit Runtime
implementation uses direct v5 ranges and provenance and is complete. Data V1
semantic/public API/type/error authority and Rust implementation are complete.

The closed public `0.1.0` release is the immutable v3 baseline. The published
and closed `0.2.4` release source, package, and official distribution use the
hard-cutover `artext.backwriter-anddress.v5` Rust API and wire. The prior closed
`0.2.3`, `0.2.2`, `0.2.1`, and `0.2.0` distributions use the hard-cutover
`artext.backwriter-anddress.v4` Rust API and wire. SHA-256, exact source byte
length, target kind, and `[start, end)` byte range are implemented identity;
v3 remains only in Git history and immutable `0.1.0` release evidence. The
canonical four-target `0.2.4` artifacts, manifest, installers, exact 60-file
live publication, fresh installation, and explicit public `0.2.3` update are
complete from Source Authority revision
`0ee4dcce14da93f925c27a04d0e79051c83fd124`.

## 0.2.5 performance recovery — Gates 1–3 complete

Gates 1 through 3 of the
[performance-recovery tracker](docs/tasks/2026-09-04-backwriter-0.2.5-structural-specialization-performance-recovery.md)
close authority and bulk literal matching. Cargo, `bw version`, artifacts,
installers, Update, and the public distribution remain published and closed
`0.2.4`. The governing rule is: semantics stay unified; execution becomes
specialized again.

The target preserves v5 fields, algebra, wire bytes, Search/View/Edit output,
one `StructuralCursor`, one `AnddressIssuer`, single/batch View, fresh Edit
receipts, source Line-count currentness, Host proof, publication, and Anchor
fail-closure. A same-hash, same-length address with a false Line count remains
`NotCurrent`. The raw observer derives that count with a minimal same-read
counter that owns no Paragraph or parent geometry and invokes no structural
cursor. The structural observer composes the same raw state with the sole
`StructuralCursor` only for actual Line/Paragraph geometry consumers.

Strict Issuer/decode construction and public `Anddress::validate()` remain.
Only repeated validation on a proven already typed hot path may be removed.
Gate 4 is authorized to add public
`Anddress::encode_into(&mut Vec<u8>) -> Result<(), AnddressError>`: it clears
the destination, checks and fallibly reserves complete capacity before writing,
leaves length zero on error, and emits the exact existing canonical v5 bytes on
success. Existing `encode()` remains and delegates to it.

Gate 2 replaces the sole matcher's byte-at-a-time Runtime caller with checked
segment matching, preserves KMP partial state across chunks, and stops matcher
work after a Line, Paragraph, or File has its best tier. The sole structural
cursor, source validation, projection, issuance, sorting, and result buckets
remain unchanged. Fixed sparse measurement stays below the 1.15 ceiling, so it
does not authorize `StructuralDemand` or cursor specialization.

Gate 3 routes Check proof misses, ordinary/batch View, Apply before-state,
trusted exact-length staging, and unit Apply after-state through raw
observation. Content Search and proof-miss Anchor retain structural framing;
Apply after-state activates it only for a non-File receipt or live non-File
Anchor. Fixed A/B/C/D evidence passes the Check, View, 256 MiB Apply, CRLF
Edit, and 134,217,728-short-Line boundaries without changing v5 or output.

The remaining ordered work is issuance/encoding, chunked pending memory,
consumer contraction, fixed evidence/source readiness, then separately
authorized release. Shared Paragraph allocation and chunk size require
measurement. Do not introduce v6, change v5 or Adapter output, restore
retired carriers/scanners/private View, or add a parser, persistent
state/index/registry, stdin, CLI split, history, relocation, watcher, merge,
retry, rollback, or compatibility path.

## Published and closed 0.2.4 structural-authority target

The published `0.2.3` source, v4 API/wire, artifacts, installers, and public
tree remain closed immutable evidence. `0.2.4` is published and closed under the
[structural-authority tracker](docs/tasks/2026-09-03-backwriter-0.2.4-structural-authority.md).
Gates 1–7 close authority, v5 algebra/wire, the sole Issuer, the shared
structural cursor, Search result contraction, exact-range View, and Edit/Apply/
Anchor migration without a private Edit View, plus Check and remaining consumer
contraction, and integrated semantic, AI-workflow, and fixed A/V5/B evidence.
Gate 8 reconstructs the four canonical artifacts, publishes the eight new
versioned files followed by both installers and the manifest last, and closes
the exact 60-file distribution. Cargo, `bw version`, artifacts, installers,
the Update target, and the published distribution are aligned at `0.2.4`.

The target is a hard cut to `artext.backwriter-anddress.v5`, never a v4
compatibility decoder, alias, wrapper, or parallel Runtime path. Its shared
`SourceIdentity` owns workspace coordinate, logical path, complete-source
SHA-256, byte length, and Line count. File owns its full byte range and Line
count. Paragraph owns its range, zero-based File Line offset, and Line count.
Line owns its range, exact terminator, parent geometry, and zero-based offset
inside that parent; a text Line is parented by its Paragraph, while a blank or
horizontal-space/tab-only Line is parented directly by File.

Anddress owns exact-state and source relationships, containment and overlap,
parent/projection validation, Line counts and numbers, ranges, and Line
terminators. One crate-private Anddress Issuer is now the sole construction
authority; one shared source identity and allocation-free target/parent
geometry serve every issued value. One private `StructuralCursor` owns
complete-source Line and Paragraph framing for Search, source observation,
and prospective Apply output. Search retains
literal finding, tiers, ordering, and multiplicity while returning owned
Anddresses directly; Adapter positions derive only from their v5 geometry.
View projects then reads an exact range; Check determines currentness; Apply
owns mutation and publication; Anchor remains the sole Backwriter continuity
exception.

Gate 2 deletes public raw and capability-owned address constructors. Gate 3
deletes the Search position/occurrence wrappers and duplicate complete-source
framers. Gate 4 deletes View's relation/range scanners and hard-cuts its native
and Adapter results to the projected v5 address plus exact Content. Gate 5
deletes one-shot Edit's private View: the Adapter prepares Line Content from
the decoded v5 terminator before Runtime access, while `apply` and
`apply_replace` share one executor and prospective cursor/Issuer pass for
receipt and Anchor candidates. The plan
retains Check's source-key grouping and shared observer, while proving that it
compares only v5 source state and that Data, Pick, and Session carry the direct
v5 values without adapters. It
reuses existing admission/no-follow reads, literal matching, ordered batch
grouping, staging and prospective provenance, Host proof, publication, and
Anchor reflection. It adds no history, relocation, registry, watcher, retry,
merge, or rollback. Exact v5 wire bytes and in-memory sharing are closed; CLI/
JSON presentation contraction and stdin or `bw.rs` splitting remain follow-up
gate decisions.

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

`0.2.3` Patch Box Gates 1 through 8 are complete and the release is published
and closed. Gate 1 closes
authority and the consumer matrix. Gate 2 carries each Search result as one
`SearchOccurrence` containing its exact v4 Anddress and same-observation
descriptive position: a one-based Line number, a one-based inclusive Paragraph
Line range, or no position for File. Check preserves this metadata while its
report remains raw-Anddress evidence; Data and Session retain the occurrence
carrier, while Pick continues to consume and return raw Anddresses. Human
Search displays current Line positions, and machine Search hard-cuts to the
Adapter-only `bw.cli.search.v2` occurrence projection; there is no production
v1 branch. None of that metadata is v4 identity, currentness evidence, a
selector, history, relocation, or a second source read. Gate 3 hard-cuts native
single View to one explicit self-or-ancestor target projection and returns the
projected current v4 Anddress with exact Content from the same accepted
observation. A Line without a containing Paragraph returns the normal
`RelationAbsent` outcome. Existing CLI View consumers request self projection,
so their grammar and output bytes are unchanged. Gate 4 adds ordered,
duplicate-preserving, all-or-nothing native batch View. It validates every
input and relation before I/O, groups by workspace coordinate and logical path,
and uses one source handle and one accepted direct observation per source group;
matching Host proof groups reuse the existing trusted range scanner on one
handle. Gate 5 adds `WorkspaceRuntime::apply_replace(&Edit) ->
Result<EditReceipt, ApplyError>` for one Replace while preserving the existing
unit-returning public Apply seam. It reuses the same executor, after projection,
proof installation, publication, and Anchor reflection. Gate 6 exposes that
receipt through one direct human/JSON writer: human output distinguishes
`Unchanged`, `Changed` with a canonical v4 Anddress, and `Changed` with `None`;
the Adapter-only `bw.cli.edit.v1` JSON schema carries the same distinction and
embeds the canonical v4 object directly. Argv remains the sole Content
transport; literal `--json`, `--raw`, and `--stdin` in the Content position are
Content, while leading `--raw` remains unsupported. No stdin path was added
without reproduced consumer, measured payload, or concrete security evidence.
Gate 7 passes the integrated A/B Dummy and complete GNU/musl semantic matrix
and makes the Cargo package and `bw version` source-ready `0.2.3`. Gate 8
reconstructs the exact four-target release, publishes its manifest last, and
closes the 52-file distribution without changing Core, Runtime, services,
tunnel, DNS, or actual user HOME state.
The ordered
gates and consumer evidence are tracked in
[Backwriter 0.2.3 Patch Box](docs/tasks/2026-09-03-backwriter-0.2.3-patch-box.md).

The repository cutline ends at public Rust Core, required Runtime, and the
implemented Backwriter CLI V1 Adapter-owned one-shot Version and Update,
one-shot human and JSON Search/View/Check/Edit, raw View, Anddress-first one-shot
Edit, Session Pick, batch Check, Anchor, Edit, Apply, result-binding, explicit
Data, and JSON Adapter. Version
and Update add no Core capability, Runtime seam, wire, or workflow authority.
Native wire,
AI/Context/Profile, client, MCP, product integration, and external consumer
work remain outside that cutline. Beyond the completed Session Pick, batch
Check, Anchor, Edit, Apply, result-binding, Data, Search/View/Check/Edit JSON, raw
View, Version, Update, and `0.2.3` Patch Box Adapter slice, every
other capability remains deferred Adapter work. `bw update` is an explicit
user-invoked installer handoff with no version comparison. It installs or
reinstalls the current official `0.2.4` release through installers that accept
only the exact closed `0.2.3` and current `0.2.4` manifests; it
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
  relationships. In v5 they share exact source-state identity, so any source
  byte change invalidates every ordinary Anddress from the prior source state.
- Current production source uses only `artext.backwriter-anddress.v5`. Do not
  add v4/v3/v2 compatibility shims, decoders, aliases, migrations, or parallel
  schemas. A recognized non-v5 version is rejected as `UnsupportedVersion`.
- The v5 source identity is workspace, logical path, source-state SHA-256,
  exact byte length, and exact Line count. Target identity adds only its
  validated File, Paragraph, or Line geometry. Target text is not identity.
- Whole-source bytes and provenance remain private call-local construction
  context. The v5 SHA-256, byte length, and Line count are ordinary source
  identity, and an Anddress is authority for that exact source state and byte
  range. This adds
  no before/after stable-read or second-read guarantee.
- The Protocol owns the bounded source-memory direction. It removes only
  unnecessary auxiliary materialization proportional to complete Workspace
  Source; it creates no fixed-memory or arbitrary-input-success promise. Its
  Check, Search, View, Anchor, Apply streaming slices are complete.
- Backwriter Core constructs and provides target Anddress values from an
  accepted current observation. Search delivers them as results; it is not an
  issuer. This creates no target registry, issuance lifecycle, locator reuse,
  durable identity, or global identity. The optional Host proof is only
  current SHA-256/length/Line-count evidence, never target lookup or retained
  results.
- The [address model](docs/architecture/rebuildable-structural-addressing.md)
  is the sole detailed raw-locator contract. Its active production algebra is
  v5 source-state/structural-geometry identity; shipped v4/v3 values are release
  evidence only. Admission decides construct/use availability, not raw equality.
- Search and View remain current-only. Default Untrusted Mode permits only
  Protocol-bounded, source-local `CurrentObservation` state during one call.
  Explicit Host-authoritative Mode may retain only the Protocol's Runtime-local,
  replace-only SHA-256/length/Line-count proof bound to workspace, admission,
  source generation, and logical path. Pick remains pure and stateless over
  caller-provided Anddress values without asserting currentness.
  `WorkspaceRuntime` exposes Search, View, Apply, Check, and anchored Runtime
  execution seams. Runtime retains no ordinary observation, source bytes,
  result, target map, snapshot, lease, or history across calls or selected
  sources. The private Host proof is the sole exception. `CurrentObservation`
  contains only the current hash, exact byte length, and exact Line count and
  is consumed or discarded before Search opens another source or any
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
  or traversal. The shared cursor supplies the source Line count.
  Missing paths and directories are Empty; the operation creates no empty
  query, synthetic Line or Paragraph, scope traversal, index, or cache.
- Search's live scan, matching, ordering, all-or-nothing behavior, and
  no-fixed-limit contract remain valid. It constructs v5 values through the
  Issuer from the current source identity and exact target/parent geometry.
  `SearchOutcome::Found` owns `Vec<Anddress>` directly. Human and machine
  positions are descriptive Adapter output derived from each v5 Anddress's
  Line geometry; there is no parallel Core position or occurrence carrier.
  Search owns no registry, persistent identity, mutation lifecycle, or result
  store.
- In the v5 source, Search remains the only target finder. It computes the
  source-state hash during its one source read, without a separate hash pass,
  and its File, Paragraph, and Line projections retain only target-required
  matching and structural geometry before Issuer construction. View validates
  source identity while capturing the caller range, Check compares only source
  identity, and Apply enforces that identity before patching the public v5
  range directly;
  none searches, reparses to relocate, context-matches, or retries an old
  target. Any source-state change invalidates an ordinary Anddress. Only Anchor
  may transform a live range arithmetically across a Backwriter-owned Apply.
- **Apply V1** has closed semantic/public API/error authority:
  Runtime-controlled Apply is the continuity-preserving editor Save path, while
  independent editor, CLI, and external writes are opaque mutations. Its
  single-source Edit Runtime implementation is complete; it creates no registry, retry,
  watcher, or automatic creation of a new `Anchedress` or `AnchorOutcome`;
  existing live Anchor continuity is reflected by direct range/provenance
  projection under the Protocol. The Patch Box companion
  `WorkspaceRuntime::apply_replace` accepts only `Edit::Replace` and returns an
  `EditReceipt`: exact no-op returns the validated input, while confirmed
  change returns the fresh same-kind target when direct after projection has
  one. It shares the existing executor and changes neither raw Apply nor
  Session Apply. Check V1
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
- **View V1** has an implementation. Its Runtime seams are
  `WorkspaceRuntime::view(&Anddress, AnddressTarget) -> Result<ViewOutcome,
  ViewError>` and `WorkspaceRuntime::view_batch(&[Anddress], AnddressTarget)
  -> Result<Vec<ViewOutcome>, ViewError>`. The single form accepts exactly one
  Anddress; the batch form accepts an ordered borrowed collection. Both use one
  requested existing target kind, without a wrapper, arbitrary range, or
  selector.
  Line may project to Line, Paragraph, or File; Paragraph to Paragraph or File;
  File only to File. Downward requests are `InvalidInput` before source I/O.
  A successful target outcome contains its projected current v5 Anddress and
  exact Content; Line-to-Paragraph without a containing current Paragraph is
  the normal `ViewOutcome::RelationAbsent` result. Batch preserves order,
  duplicates, and per-item outcomes, returns all results or none, and groups
  inputs so each Untrusted or Host-proof-miss source has one accepted direct
  observation; matching Host groups reuse one handle and read only requested
  exact ranges. It has no
  query, ranking,
  mutation, display, adapter payload, or
  retained source/result/relation state, registry, cache, history, snapshot, or
  lease. Pick may supply the input, but View neither calls Pick nor proves Pick
  provenance; explicit selection is the caller's responsibility.
- View's admitted no-follow one-read access checks v5 coordinate/path,
  source-state hash/length/Line count, and exact range. Its only successful
  target result is `ViewOutcome::Projected { anddress, content }`; v5 algebra,
  rather than extra outcome fields or a relation scan, provides ancestors and
  Line terminators.
- Arbitrary range selection, descendants, and partial behavior are post-V1 owner
  decisions. View does not classify input state; Check does not change View.
- **Check V1** has closed semantic/API/type/report authority and a stateless
  result/history contract. Default execution remains the `0.2.0` observation
  path. A Host-authoritative matching path proof classifies every occurrence in
  that source group by hash, length, and Line count without filesystem access; mismatches do
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
- Implemented v5 raw File, Paragraph, and Line equality follows exact source
  identity plus target and parent geometry. It
  makes no continuity, survivor, relocation, or historical-identity claim.

## Search execution contract

- The implemented Runtime execution seams are
  `WorkspaceRuntime::search(&SearchRequest)`,
  `WorkspaceRuntime::view(&Anddress, AnddressTarget)`,
  `WorkspaceRuntime::view_batch(&[Anddress], AnddressTarget)`,
  `WorkspaceRuntime::apply(&mut self, &Edit)`, and
  `WorkspaceRuntime::apply_replace(&mut self, &Edit)`,
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
