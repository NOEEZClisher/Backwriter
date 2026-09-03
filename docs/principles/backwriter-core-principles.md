# Backwriter Principles

The planned `0.2.5` performance recovery is governed by Principle 19 and its
[eight-gate tracker](../tasks/2026-09-04-backwriter-0.2.5-structural-specialization-performance-recovery.md).
Gate 1 changes authority documents only; current source and official release
remain closed `0.2.4`.

The published and closed `0.2.4` target is governed by Principle 18 and its
[eight-gate tracker](../tasks/2026-09-03-backwriter-0.2.4-structural-authority.md).
Gates 2–7 hard-cut current source to v5, install the sole Issuer and shared
complete-source structural cursor, contract Search results and View, and remove
one-shot Edit's private View, then confirm source-state-only Check and direct v5
consumers before integrated source readiness. Cargo and `bw version` are
`0.2.4`; Gate 8 publishes and closes the matching four-target v5 distribution
while preserving `0.2.3` as immutable v4 release evidence.

The closed public `0.1.0` release remains immutable v3 evidence. Published and
closed `0.2.3`, the prior published `0.2.2` and `0.2.1`, and the prior closed public
`0.2.0` release build implement the
hard-cutover v4 value/wire with SHA-256; they remain historical release
evidence while the active source contract is v5. The published and closed
`0.2.1` target adds
only the explicitly guarded observation-reuse authority below; Phase 2 provides
the Host kernel and Search proof installation, and Phase 3 adds bounded ordinary
View proof consumption. Phase 4 adds Check group classification from matching
proof, and Phase 5 adds Apply precondition reuse and prospective-after proof
replacement coupled to existing Anchor reflection. Phase 6 closes path-exact
invalidation, authority isolation, matching anchored View reuse, and guarded
drift semantics without adding a watcher or supported race.
The `0.2.2` Gates 1–6 add only the general Adapter contraction in Principle 16
and its integration evidence; Gate 7 separately closes publication. They
change no Core, Runtime, or v4 meaning.
The published and closed `0.2.3` Patch Box is governed by Principle 17.
Gates 1 through 8 close its authority, same-observation Search position carrier, explicit
single self-or-ancestor View projection, and ordered all-or-nothing batch View
plus the Replace-only native receipt and its direct human/JSON Adapter
projection, integrated Dummy, GNU/musl readiness, and release publication while
leaving v4 identity and currentness unchanged. Official release state is now
the v5 `0.2.4` distribution governed by Principle 18.

1. **Current-only permits only bounded evidence.** Current is source-visible.
   Untrusted Mode keeps one source observation's hash, length, and Line count only while its
   capability call runs; Search separately retains only target-required
   boundaries and provisional ranges. Host-authoritative Mode may retain only
   replace-only current SHA-256/length/Line-count proof records under the Protocol's
   complete writer guard. Neither mode retains an observation object, bytes,
   target map, results, or history.
2. **Backwriter is not Git.** It establishes current structure only and does not
   model merge, branch, ancestry, conflict resolution, history, rollback, or
   inheritance of past identity. Past-state recovery belongs outside
   Backwriter.
3. **Search is the only capability that finds a target.** It discovers exact
   current ranges and computes the source hash during the same retained read.
   It performs no separate hash pass and creates no persistent index.
4. **An Anddress authorizes exact state and geometry.** A v5 ordinary Anddress
   identifies workspace, logical path, source hash, byte length, Line count,
   and exact File, Paragraph, or Line geometry. The source hash is final
   currentness authority; target text is not identity.
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
7. **Capability responsibility stays narrow.** View validates source identity
   and returns exact caller-range bytes. Check compares only source hash,
   length, and Line count; a trusted path proof may supply that comparison without source
   access for the complete group. Apply requires the exact source state before
   direct range splice preparation and publication; a matching trusted proof
   may remove only its before hash work, and confirmed changed publication may
   retain only the already computed after hash/length/Line count. None is a target finder.
8. **Pick is pure input selection.** Pick preserves an input-order subsequence
   of valid caller-provided Anddress values without Runtime or Workspace access,
   currentness, relation discovery, or retained result state.
9. **Anchor is the sole continuity exception.** Only live Runtime-local Anchor
   state may arithmetically transform a range across a Backwriter-owned Apply.
   The same prospective-after identity drives both that reflection and Host
   proof replacement. External changes invalidate rather than relocate it;
   both public invalidation seams discard same-path proof and continuity through
   one I/O-free operation. A matching Host proof lets anchored View share the
   ordinary trusted View path; a proof mismatch fail-closes that source before
   I/O. Anchor adds no history, persistence, watcher, or generic transition
   engine.
10. **Current observation is bounded and ephemeral.** The private
    `CurrentObservation` holds only one selected source's hash, byte length, and Line count
    until the current Search, View, Check, Apply, or Anchor consumer discards
    it. A trusted proof may copy only that completed source identity and
    binding; it is not the observation. Neither is a whole-source buffer, parse
    tree, complete Line collection, Search result, history, persistent index,
    relocation context, or full workspace cache.
11. **Search remains all-or-nothing.** Invalid text or actual allocation/I/O
    failure discards the whole result. Existing live traversal, exact File
    lookup, deterministic ordering, and no-fixed-limit behavior remain baseline
    constraints for the v5 cutover.
12. **Check result semantics remain stateless.** Check creates no result store
    or latest slot; Runtime stores no `CurrentObservation` across calls. A
    trusted proof hit classifies one path group without filesystem access and
    never installs, changes, or removes proof. The narrow trusted current proof
    is the sole cross-call exception and carries no Check result or history.
    Data remains explicit caller-owned state.
13. **Composition belongs to the caller.** Shared native types and explicit
    value passing establish neither provenance nor a required call order or
    general workflow. The Protocol's named integrations remain explicit.
14. **Bound source memory without changing semantics.** The target adds no
    fixed input cap, skip, truncation, retry, spill, result cache, or snapshot
    authority. The narrow RAM-only current proof is non-persistent and retains
    no source bytes. Resource and I/O failures remain valid.
15. **Edit values stay inert.** Edit values neither search nor retain current
    state. Apply is their only execution boundary and must enforce the ordinary
    Anddress source-state precondition before publication.
16. **General Adapter editing contracts existing primitives.** The implemented
    one-shot form accepts an encoded v5 Anddress and new Content, then
    prepares `Edit::Replace` directly from decoded target geometry and calls
    Apply. Only Line body replacement appends the terminator carried by its v5
    Anddress; File and Paragraph remain exact Content. Apply alone confirms
    currentness. This hides optional caller bookkeeping without adding a Core
    workflow, target finder, state machine, relocation, retry, or error alias.
17. **Information surfaces do not become identity or history.** Search may
    describe a hit's current Line position from the same source observation,
    View may project only from caller-held targets to themselves or ancestors,
    and a confirmed Apply may expose a fresh address for its exact resulting
    current state. Ordered batch View groups exact source keys only to reuse one
    direct source observation, then restores input order and duplicates; it
    creates neither cross-target relation state nor partial results.
    Descriptive positions are not selectors or v5 fields; projection is not
    discovery; and a receipt or its Adapter projection creates no predecessor,
    successor, persistence, retry, watcher, or relocation authority.
18. **Structure has one parser and one address authority.** Gate 2 installs the
    sole crate-private Anddress Issuer and self-contained v5 value. One shared
    source identity plus complete target/parent geometry now owns state/source
    relationships,
    containment, overlap, parent/projection, Line geometry, ranges, and
    terminators. Gate 3 consolidates complete-source Line and Paragraph framing
    into the sole `StructuralCursor` and removes Search's position and
    occurrence wrappers. Search finds and returns Anddresses directly; View
    projects then range-reads, Check checks currentness, Apply publishes
    mutation, and Anchor alone carries live continuity. Gate 4 removes bounded
    View relation/range scans and returns only the projected v5 address plus
    exact Content or `RelationAbsent`. Gate 5 removes the private composition
    View and reuses v5 containment/overlap plus one prospective cursor/Issuer
    pass for receipt and Anchor candidates. No compatibility path is retained.
    Gate 6 removes the remaining consumer adapters and verifies source-state-only
    Check; Gate 7 changes no production structure and closes integrated GNU,
    musl, fixed A/V5/B, and AI-workflow evidence before advancing source version.
    This consolidation creates no history, relocation, registry, watcher,
    retry, merge, rollback, or implicit workflow.
19. **Unified semantics permit specialized execution.** Performance recovery
    may remove only work a capability does not consume while preserving v5,
    exact output, errors, ordering, multiplicity, and fail-closure. Source Line
    count remains identity and currentness evidence even when a raw path uses a
    minimal same-read counter instead of the structural cursor. Strict decode,
    the sole Issuer, and public explicit validation remain construction
    authority; only proved typed hot-path repetition may disappear. One public
    reusable `encode_into` writer may replace per-result allocation while
    existing `encode()` delegates and canonical bytes remain exact. Measured
    evidence alone may select cursor demand, shared Paragraph allocation, or
    pending chunk size; none authorizes another parser, validator, writer,
    compatibility path, state, or release.
