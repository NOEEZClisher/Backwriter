# Backwriter Principles

The closed public `0.1.0` release remains immutable v3 evidence. Current
`0.2.0` Rust, including the closed public release build, implements the
hard-cutover v4 value/wire with SHA-256; these principles describe the active
v4 Core contract.

1. **Current-only permits bounded call-local state.** Current is source-visible.
   One source observation may retain only its hash and length while its
   capability call runs; Search separately retains only target-required
   boundaries and provisional ranges. Success consumes it and failure discards
   it. No observation persists across sources or calls.
2. **Backwriter is not Git.** It establishes current structure only and does not
   model merge, branch, ancestry, conflict resolution, history, rollback, or
   inheritance of past identity. Past-state recovery belongs outside
   Backwriter.
3. **Search is the only capability that finds a target.** It discovers exact
   current ranges and computes the source hash during the same retained read.
   It performs no separate hash pass and creates no persistent index.
4. **An Anddress authorizes exact state and range.** A v4 ordinary Anddress
   identifies workspace, logical path, source hash, byte length, kind, and
   `[start, end)`. The source hash is final currentness authority; target text
   and ordinal are not identity.
5. **Ordinary addresses do not relocate.** A changed source invalidates an
   ordinary Anddress. View, Check, and Apply never search, reparse, or
   context-match to move an old target after external change. Ordinary View and
   Check do not structurally revalidate ranges; Apply patches the exact public
   range only after exact source-state proof. Re-search is an
   explicit caller choice and returns a new current address.
6. **Safety remains selective.** Admission, private/unsafe policy, and symlink
   rejection remain capability-relative. `.artext/bw` alone is ignored;
   `.artext/other` is ordinary source. Unsaved editor buffers remain outside
   Core, and source-visible does not promise durability, retry, or a second
   read.
7. **Capability responsibility stays narrow.** View validates source hash and
   length and returns exact caller-range bytes. Check compares only source hash
   and length. Apply requires the exact source state before direct range splice
   preparation and publication. None is a target finder.
8. **Pick is pure input selection.** Pick preserves an input-order subsequence
   of valid caller-provided Anddress values without Runtime or Workspace access,
   currentness, relation discovery, or retained result state.
9. **Anchor is the sole continuity exception.** Only live Runtime-local Anchor
   state may arithmetically transform a range across a Backwriter-owned Apply.
   External changes invalidate rather than relocate it; Anchor adds no history,
   persistence, watcher, or generic transition engine.
10. **Current observation is bounded and ephemeral.** The private
    `CurrentObservation` holds only one selected source's hash and byte length
    until the current Search, View, Check, Apply, or Anchor consumer discards
    it. It is not a
    whole-source buffer, parse tree, complete Line collection, Search result,
    history, persistent index, relocation context, or full workspace cache.
11. **Search remains all-or-nothing.** Invalid text or actual allocation/I/O
    failure discards the whole result. Existing live traversal, exact File
    lookup, deterministic ordering, and no-fixed-limit behavior remain baseline
    constraints for the v4 cutover.
12. **Check result semantics remain stateless.** Check creates no result store
    or latest slot; Runtime stores no `CurrentObservation` across calls. Data
    remains explicit caller-owned state.
13. **Composition belongs to the caller.** Shared native types and explicit
    value passing establish neither provenance nor a required call order or
    general workflow. The Protocol's named integrations remain explicit.
14. **Bound source memory without changing semantics.** The target adds no
    fixed input cap, skip, truncation, retry, spill, persistent cache, or
    snapshot authority. Resource and I/O failures remain valid.
15. **Edit values stay inert.** Edit values neither search nor retain current
    state. Apply is their only execution boundary and must enforce the ordinary
    Anddress source-state precondition before publication.
