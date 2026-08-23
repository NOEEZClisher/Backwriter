# Artext

Artext provides the Rust Backwriter Core and required Runtime for admitted
UTF-8 text. `WorkspaceRuntime::search(&SearchRequest)`,
`WorkspaceRuntime::view(&Anddress)`, `WorkspaceRuntime::apply(&mut self,
&Edit)`, `check`, `check_search`, `check_pick`, `anchor`, `view_anchored`, and
`invalidate_anchored_source` are its implemented Runtime seams.

Capability composition is caller-owned: shared native values and explicit value
passing do not establish provenance, a required call order, or a general
workflow. The Protocol defines the named cross-capability contracts that remain.

Backwriter is not Git. It establishes only current File/Paragraph/Line
structure and does not model merge, branch, ancestry, conflict resolution,
history, rollback, or inheritance of past identity. Git or another external
history system owns past-state recovery.

Search is current-only and stateless. Runtime traverses admitted source through
capability-relative no-follow handles, observes one selected regular file once,
parses exact File/Paragraph/Line structure, and drops that source before opening
the next file. Returned Search, View, and Pick values
belong to the caller; those capabilities keep no source, result, snapshot,
index, registry, or authenticity state.

The Protocol's Check, Search, View, Anchor, and Apply streaming Rust slices are
complete. They forbid arbitrary caps, skips, and truncation while allowing
Resource and I/O failures; bounded source memory is not a fixed-memory promise.

Backwriter current is the admitted Workspace Source visible to a retained
no-follow read. Unsaved editor buffers are outside Core. Save, autosave, and
external writes are not Runtime events; their results can affect a capability
call only when source-visible. Runtime performs no automatic scan,
Anddress reissuance, watcher, durability check, retry, or second read.

File, Paragraph, and Line are independent target addresses with structural
relationships. Their raw locators are defined in the
[address model](docs/architecture/rebuildable-structural-addressing.md): File
uses Runtime workspace coordinate plus observed logical path, Paragraph and
Line add current 0-based ordinals, and Line also uses its full exact extent. A
separator boundary change establishes only current Paragraphs, and ordinal
movement makes a new raw address, without a past-to-current mapping. The v3
Rust model uses only workspace coordinate, logical path, target kind, ordinal,
and (for a Line) exact extent; it has no source-wide evidence or compatibility
decoder. View currentness, Search projection, and Pick predicate semantics are
defined by the Protocol.

Backwriter Core constructs and provides target Anddress values from an accepted
current observation. Search delivers those values in its results; it is not an
issuer. There is no separate registry, issuance lifecycle, lookup/reuse state,
durable identity, or global identity.

The Runtime ignores only root-relative `.artext/bw` and descendants. Other
`.artext` children are ordinary source paths. Future spill is host-owned and is
not created or configured in this repository.

Pick is pure Core selection over caller-provided valid Anddress values. It
returns an input-order-preserving stable subsequence without reading Runtime or
Workspace state. Its v3 `same_file` predicate compares only workspace coordinate
and logical path; it has no observation, paragraph, or hierarchy relation.

View's one-read admitted access and File/Paragraph/Line text projection use v3
target-specific currentness. Apply V1 executes one caller-owned `Edit` through
`WorkspaceRuntime::apply(&mut self, &Edit)`. Anchor live-continuity authority,
public surface, and Rust
implementation are complete. It retains only
opaque owning Runtime-local continuity, non-aliasing `AlreadyLive`, and logical-source
invalidation; it has no history, persistence, re-identification, or source-wide
transition engine. It considers only source-visible mutations, never editor-
only buffer changes, without inferring continuity from a generic file-changed
signal. `WorkspaceRuntime` implements Search, View, Apply, Check, Anchor,
anchored View, and explicit source invalidation seams.
Check V1 semantic/API/type/report authority and its stateless Runtime
implementation are complete. Data V1 semantic/public API/type/error authority
and Rust implementation are complete. Edit V1 semantic/public API/type/error
authority, inert Rust value implementation, and single-source Apply
Runtime implementation are complete. Apply's reference letter is unassigned.

## Verify

```sh
cargo build --offline --locked --release
cargo test --offline --locked
```

See [the current protocol](docs/architecture/backwriter-text-coordination-protocol.md).
