# Repository operating guard

Artext ships the Rust Backwriter Core and its required target Runtime. The
owner-defined Core capability inventory is Search, View, Pick, Anchor, Check,
Edit, Apply, and Data. `S` is assigned to Search, `P` to Pick, `A` to
Anchor, `C` to Check, and `D` to Data. `I`, `R`, and Apply's reference letter
are unassigned. Inventory names do not define a lifecycle, call order,
payload, error model, or adapter behavior.

Search, View, Pick, Anchor, and Check have Rust implementations. Their v3 View
currentness, Search projection, Pick predicate semantics, Anchor live
continuity, and Check batch currentness reporting are implemented. Apply V1
semantic/public API/error authority and its single-source Edit Runtime
implementation are complete. Data V1
semantic/public API/type/error authority and Rust implementation are complete.

The repository cutline ends at public Rust Core, required Runtime, and the
implemented Backwriter CLI V1 one-shot human and JSON Search/View/Check, raw
View, Session Pick, batch Check, Anchor, Edit, Apply, result-binding, explicit
Data, and JSON Adapter.
Native wire,
AI/Context/Profile, client, MCP, product integration, and external consumer
work remain outside that cutline. Beyond the completed Session Pick, batch
Check, Anchor, Edit, Apply, result-binding, Data, Search/View/Check JSON, and raw View slice, every other
capability remains deferred Adapter work.

## Current structure only

Backwriter is not Git. It does not model or perform merge, branch, ancestry,
conflict resolution, history, rollback, or inheritance of past identity. Each
accepted observation establishes only the current File/Paragraph/Line structure.
A structural change creates no past-structure predecessor/successor/survivor
lineage or reconciliation mapping. Past-state recovery belongs to Git or another
external history system, never Backwriter.

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
  reads, and the V1 publication seam;
  Anchor owns only its closed target-local RAM continuity contract.
- Workspace Source owns canonical source bytes. Runtime never creates
  repository-local `.artext` authority.
- An accepted current observation is the bytes returned by one retained
  no-follow read of currently admitted Workspace Source. Editor-only buffers,
  keystrokes, IME, undo, and dirty-state lifecycle are outside Core. A
  source-visible write can affect a call only through that call's one-read
  observation; Save is not a Runtime event and creates no watcher, automatic
  re-evaluation, durability, or stable-read guarantee.
- Anddress, admission, current-call parsing, and path safety are foundation
  mechanisms, not separate Core capability names.
- Admission has one fixed regular UTF-8-text policy. It is path-safe, rejects
  reserved/private aliases and symlinks, opens selected components
  capability-relatively without following links, and does not scan unselected
  roots.
- File, Paragraph, and Line are independent target addresses with structural
  relationships. A Line change does not automatically change its File, a
  boundary-preserved Paragraph, or unrelated Lines.
- Production uses `artext.backwriter-anddress.v3` target-local values. Do not
  add v2 compatibility shims, decoders, aliases, or parallel schemas.
- Whole-source bytes, length, provenance, and fingerprints may be private
  call-local construction context. They are not target identity, and a digest
  computed from one read does not prove a stable source. The observation is the
  bytes returned by one retained no-follow handle read; Runtime adds no
  before/after stability or second-read guarantee.
- The Protocol owns the bounded source-memory direction. It removes only
  unnecessary auxiliary materialization proportional to complete Workspace
  Source; it creates no fixed-memory or arbitrary-input-success promise. Its
  Check, Search, View, Anchor, Apply streaming slices are complete.
- Backwriter Core constructs and provides target Anddress values from an
  accepted current observation. Search delivers them as results; it is not an
  issuer. This creates no separate registry, issuance lifecycle, lookup/reuse
  state, durable identity, or global identity.
- The [address model](docs/architecture/rebuildable-structural-addressing.md)
  is the sole detailed raw-locator contract. It keys File by Runtime workspace
  coordinate and observed logical path, adds a current 0-based ordinal for
  Paragraph or Line, and adds exact extent only for Line. Admission decides
  construct/use availability, not raw equality.
- Search and View are current-only and stateless. Pick is pure and stateless
  over caller-provided Anddress values without asserting currentness.
  `WorkspaceRuntime` exposes Search, View, Apply, Check, and anchored Runtime
  execution seams. Outside Anchor, Runtime retains no observation, source,
  result, snapshot, lease, or authenticity state. Anchor may retain only
  target-local session continuity. Search observes each selected source once
  through a retained no-follow capability, parses, projects, and drops its
  call-local observation before opening another source.
- Future spill belongs only to a host-provided system root. This repository does
  not create `.artext`; the exact Runtime-root-relative `.artext/bw` path and
  its descendants are ignored by Backwriter Runtime execution. Other `.artext`
  children remain ordinary Workspace Source subject to the normal safety policy.
- Search is read-only discovery over admitted live Workspace Source inside a
  structured scope. Every call scans live source directly; it creates or uses
  no persistent index or authority, global snapshot, cache, history, or
  past-result completeness evidence.
- Search matches at Line-content granularity. Its requested target kind is
  Line, Paragraph, or File and changes only returned Anddress granularity.
  Separator Lines have no Paragraph.
- Search's live scan, matching, ordering, all-or-nothing behavior, and
  no-fixed-limit contract remain valid. It constructs v3 target-local values
  directly from current coordinate, logical path, and target locators.
  Search owns no registry, persistent identity, mutation lifecycle, or result
  store.
- **Apply V1** has closed semantic/public API/error authority:
  Runtime-controlled Apply is the continuity-preserving editor Save path, while
  independent editor, CLI, and external writes are opaque mutations. Its
  single-source Edit Runtime implementation is complete; it creates no registry, retry,
  watcher, or automatic creation of a new `Anchedress` or `AnchorOutcome`;
  existing live Anchor continuity is reflected under the Protocol. Check V1
  semantic/API/type/report authority and
  Rust implementation are complete; Data V1 semantic/public API/type/error
  authority and Rust implementation are complete; Edit V1 semantic/public
  API/type/error authority and single-source Apply Runtime implementation are
  complete, and Apply's
  reference letter remains
  unassigned. Anchor and anchored seams are implemented after Apply.
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
  and v3 `same_file` remain valid. `same_file` compares only WorkspaceCoordinate
  and LogicalPath; no relation enum or observation/paragraph/hierarchy relation
  exists in v3. It makes no activity, staleness, availability,
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
- View's admitted no-follow one-read access checks v3 coordinate/path and
  target-specific locators, constructing related results from the same read.
- Plural input, ranges, descendants, and partial behavior are post-V1 owner
  decisions. View does not classify input state; Check does not change View.
- **Check V1** has closed semantic/API/type/report authority and a stateless
  Runtime implementation. **Data V1** semantic/public API/type/error authority
  and Rust implementation are complete.
  Edit V1 semantic/public API/type/error authority and single-source Apply
  Runtime implementation are complete. Apply has closed V1 semantic/public
  API/error authority and Runtime implementation. Apply's reference letter is
  unassigned. Read is retired; `R` is unassigned and its historical meaning
  transfers to no capability. Block is historical wording for the existing
  blank-line-bounded Paragraph and introduces no type, alias, variant, or wire
  value.
- Raw File, Paragraph, and Line equality follows the address model's
  target-local coordinate/path/ordinal/extent algebra. Separator changes create
  only current Paragraph structure; ordinal movement creates a new raw address.
  Raw reconstruction makes no continuity, survivor, or historical-identity
  claim.

## Search execution contract

- The implemented Runtime execution seams are
  `WorkspaceRuntime::search(&SearchRequest)`,
  `WorkspaceRuntime::view(&Anddress)`, and
  `WorkspaceRuntime::apply(&mut self, &Edit)`,
  `WorkspaceRuntime::check(Anddress)`, `check_search(SearchOutcome)`, and
  `check_pick(PickOutcome)`.
  There is no public Runtime enumeration or listing API. Core owns the validated
  Search request, scope, query, target, outcome, and error types.
- Scope is all admitted roots or a nonempty narrowing-only list of subtree and
  source selectors. Selectors use safe platform-neutral UTF-8 logical paths,
  reject explicit `.`, duplicates, cross-kind same paths, and every
  component-boundary overlap. Admission roots and selector paths have no
  semantic length or count limit; they are fully admission-validated before
  filesystem I/O.
- Query is a nonempty literal UTF-8 value without NUL, CR, or LF. Search
  performs case-sensitive contiguous matching on exact Line content without
  trimming, normalization, folding, token, regex, fuzzy, or semantic behavior.
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
