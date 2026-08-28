# Backwriter Principles

1. **Current is source-visible.** Search and View read admitted Workspace Source
   through retained no-follow capabilities. Unsaved editor buffers are outside
   Core; Save is not a Runtime event or automatic re-evaluation trigger.
2. **Backwriter is not Git.** It establishes current structure only and does not
   model merge, branch, ancestry, conflict resolution, history, rollback, or
   inheritance of past identity. Past-state recovery belongs outside
   Backwriter.
3. **Targets are independent.** File, Paragraph, and Line have structural
   relationships without source-wide equality or a persistent identity tree.
   Their raw locator algebra belongs only to the address model; admission is not
   raw equality, and ordinal movement makes a new raw address.
4. **Observation is not target identity.** Whole-source bytes, length,
   provenance, and fingerprints may support private call-local construction but
   do not make a target address or prove stable live state. Source-visible does
   not imply `fsync`, crash durability, an atomic save, retry, or a second read.
5. **Safety remains selective.** Admission, private/unsafe policy, and symlink
   rejection remain capability-relative. `.artext/bw` alone is ignored;
   `.artext/other` is ordinary source.
6. **Search is all-or-nothing.** Invalid text or actual allocation/I/O failure
   discards the whole result; no partial result, cache, or persistent index is
   created. The Protocol separates v3 literal matching and target projection
   from exact logical File lookup. Exact lookup observes one admitted regular
   source under the same safety and text policy and never invents a query, Line,
   or Paragraph. Live traversal and no-limit behavior remain valid where content
   Search uses them.
7. **Core constructs; Search delivers.** Backwriter Core constructs and
   provides target Anddress values from an accepted current observation, and
   Search only delivers them as results. Returned values belong to the caller;
   there is no separate registry, issuance lifecycle, lookup/reuse state,
   durable identity, or global identity.
8. **Pick is pure input selection.** Pick preserves an input-order subsequence
   of valid caller-provided Anddress values without Runtime or Workspace access.
   Its v3 `same_file` predicate compares only WorkspaceCoordinate and
   LogicalPath; it has no observation, paragraph, or hierarchy relation.
9. **View reuses safe reading, not v2 equality.** Its one-read File/Paragraph/
   Line text projection is reusable; the Protocol alone defines v3
   target-specific currentness and related results. Plural, range, descendant,
   and partial behavior remain pending.
10. **Apply V1 is Runtime-controlled.** Its semantic/public API/error
   authority and single-source Edit Runtime implementation close the editor
   Save continuity path. It has no watcher, retry, rollback, or automatic
   creation of a new
   `Anchedress` or `AnchorOutcome`; it still reflects existing live Anchor
   continuity under the Protocol. Concurrent-writer coordination is
   caller-owned. Anchor and anchored seams are implemented after it.
11. **Anchor stays minimal.** Its live-continuity authority and public Runtime
   surface are implemented. It retains
   only opaque owning Runtime-local continuity, non-aliasing `AlreadyLive`, no
   history or persistence, and logical-source invalidation. It has no
   source-wide transition engine, considers only source-visible mutations, and
   cannot infer continuity from a generic file-changed signal.
12. **Check remains stateless.** Check V1 implements its semantic, API, type,
    and report contract without a result store or latest slot. Data V1
    semantic/public API/type/error authority and Rust implementation are
    complete.
13. **Composition belongs to the caller.** Shared native types and explicit
    value passing establish neither provenance nor a required call order or
    general workflow. They do not require complete capability type/state
    isolation or prohibit the Protocol's named integration contracts.
14. **Bound source memory without changing semantics.** The Protocol's Check,
    Search, View, Anchor, Apply streaming slices remove only unnecessary
    complete-source auxiliary materialization. They create no fixed-memory
    promise, input cap, skip, truncation, retry, cache, spill, or snapshot
    authority; Resource and I/O failures remain valid.
15. **Edit values stay inert.** Edit V1 semantic/public API/type/error
    authority is complete; its only execution path is single-source Runtime
    Apply.
