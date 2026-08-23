# Artext Current State

## Core capability inventory

| Letter | Word | Current status |
| --- | --- | --- |
| S | Search | Rust implementation with v3 target projection. |
| V | View | Rust implementation with v3 currentness. |
| P | Pick | Rust implementation with v3 predicate semantics. |
| A | Anchor | Rust implementation with Runtime-local live continuity. |
| C | Check | Rust implementation with V1 batch currentness reporting. |
| D | Data | Rust implementation with V1 typed caller-owned storage. |
| E | Edit | V1 values and single-source Apply Runtime implementation. |
| unassigned | Apply | V1 public authority and Runtime implementation complete. |

`S` is assigned to Search, `P` to Pick, and `A` to Anchor. `I`, `R`, and
Apply's reference letter are unassigned. Read is retired. Check's V1
semantic/API/type/report authority and its stateless Runtime implementation are
complete. Data V1 semantic/public API/type/error authority and Rust
implementation are complete. Edit V1 semantic/public API/type/error authority,
Rust value implementation, and single-source Apply Runtime implementation are
complete. Apply's V1 semantic/public API/error authority and Runtime
implementation are complete.

## CLI Adapter V1 Search JSON, Session Pick, batch Check, Anchor, Edit, Apply, result-binding, and Data slice

The repository includes the canonical `backwriter` CLI Adapter. Its completed
scope is one-shot human and JSON Search, View, and Check plus Session Pick,
batch Check, Anchor, Edit, Apply, result binding, and explicit Data over the existing public
Runtime seams. The Session retains one Runtime, one caller-owned `DataStore`,
and explicit CLI-local bindings plus non-aliasing owning Anchedress handles.
Session Pick passes a named candidate collection and a
CLI-parsed predicate to the existing pure Core function. Session batch Check
passes an exact matching binding clone to `check_search` or `check_pick` and
exposes only its report counts. Session Anchor creates an opaque Runtime-local
handle, views it through the existing anchored seam, and invalidates only its
logical source. One-shot Search JSON streams a compact Adapter envelope and
exact existing v3 Anddress objects without creating a Core wire. Data transfers exact clones from explicit Session values into
the existing typed Core store and reads them back without capability execution.
It adds no Core API, wire, workflow, provenance, automatic Data storage,
registry, persistence, or retained Core state beyond existing Anchor continuity.
One-shot Data and Anchor remain intentionally unsupported because their
DataStore and live-handle contracts are Session-lifetime state. One-shot Pick,
batch Check, Edit, and Apply await collection or Edit transport schema
authority. Raw output and further Session behavior remain
deferred under the [CLI V1 authority](../architecture/backwriter-cli-v1.md).

## Current-only Runtime contract

Search and View are current-only and stateless; Pick is pure and stateless over
caller input. `WorkspaceRuntime::search`, `WorkspaceRuntime::view`,
`WorkspaceRuntime::apply(&mut self, &Edit)`, `WorkspaceRuntime::check`,
`check_search`, `check_pick`, `anchor`, `view_anchored`, and
`invalidate_anchored_source` are the implemented Runtime seams. Search traverses
admitted Workspace Source through
retained capability-relative no-follow handles, observes one selected regular
file once, validates UTF-8/NUL, parses exact Line structure, matches and orders
results, then drops that source before opening another.
Runtime retains no observation, source, result, snapshot, lease, registry,
history, or authenticity state.

An accepted current observation is the bytes returned by a retained no-follow
read of currently admitted Workspace Source. Unsaved editor buffers,
keystrokes, IME, undo, and dirty state are outside Core. A completed save or
external write matters only when its result is source-visible to a capability
call; Runtime has no Save event, watcher, automatic address re-evaluation,
durability promise, retry, or second read.

The Protocol's bounded source-memory Check, Search, View, Anchor, Apply
streaming slices are complete. They add no fixed input cap, skip, truncation,
retry, cache, spill, or snapshot authority.

The Protocol's current-structure-only boundary is active. Structural change
yields only the resulting current structure, without past-target mapping or
inherited identity.

Admission/path safety, capability-relative no-follow access, UTF-8/NUL
fail-all, the exact Runtime-root-relative `.artext/bw` boundary, the exact Line
cursor, Search live traversal/matching/ordering/no-limit behavior, View's
one-read text projection, and Pick's stable-subsequence/non-relational
predicates remain reusable foundation.

## Target-local address correction

File, Paragraph, and Line are independent target addresses with structural
relationships, not a persistent parent/child identity tree. The address model
defines their raw coordinate/path/ordinal/extent locator algebra. Admission is
not raw equality. A separator-boundary change establishes only the resulting
current Paragraphs, and ordinal movement makes a new raw address; neither has a
relation to past Paragraphs.
`Block` is historical wording for the existing blank-line-bounded Paragraph and
creates no type, alias, variant, or wire value.

`artext.backwriter-anddress.v3` is the sole accepted wire and production model.
It keeps source-wide bytes, length, provenance, and fingerprints out of target
identity, using only workspace coordinate, logical path, and target locators.

Backwriter Core constructs and provides target Anddress values from an accepted
current observation. Search delivers them as results; it is not an issuer. This
creates no separate registry, issuance lifecycle, lookup/reuse state, durable
identity, or global identity.

Search projects v3 locators directly; Pick provides `same_file` instead of
observation, paragraph, or hierarchy relations; and View checks its target
locator from one current read. There is no compatibility decoder, migration,
alias, or parallel schema. The locator algebra creates no continuity or
historical-identity claim.

## Anchor

Apply V1 semantic/public API/error authority and Runtime implementation are
complete: it applies one caller-owned Edit while independent source-visible
writes remain opaque mutations.

Anchor live-continuity authority and its public Runtime surface are implemented.
It retains only opaque owning
Runtime-local continuity, non-aliasing `AlreadyLive`, no history, persistence,
or re-identification, and logical-source invalidation. The A0–A2 source-wide
transition model is retired. Future continuity considers only source-visible
mutations, never editor-only buffers; it creates no Save notification, watcher,
generic file-change inference, or continuity mapping for opaque mutations.
