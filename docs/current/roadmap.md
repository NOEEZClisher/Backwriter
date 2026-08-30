# Backwriter Roadmap

## Planned: 0.2.1 current-observation reuse

`0.2.1` is a partially implemented and unpublished performance target governed by the
[seven-phase tracking task](../tasks/2026-08-30-backwriter-0.2.1-current-observation-reuse.md).
Phase 1 closes authority and records the current execution flow. Phase 2 adds
the explicit Host Runtime, source invalidation, private current proof state, and
successful-Search installation. V4 Anddress identity/wire, SHA-256, exact
source length, kind, and `[start,end)` remain unchanged, and Search remains the
only target finder.

Default Runtime, one-shot CLI, and ordinary CLI Session behavior remain
Untrusted Mode and preserve `0.2.0` per-call observation. Reuse is available
only under explicit Host-authoritative Mode with complete writer coordination,
mutation exclusion through call completion, and synchronous invalidation before
mutation. The only permitted retained state shape is a Runtime-local, RAM-only,
replace-only current SHA-256/length proof bound to workspace, admission, source
generation, and logical path. It retains no source bytes, results, target map,
prior proof, history, or relocation evidence.

The completed Phase 2 kernel uses `open_host_authoritative` and
`invalidate_source`, a synchronized sorted path vector with one independent
proof per observed source and no eviction or retained handle, and whole-call
successful installation for every fully observed Search source. Completed
Phase 3 adds ordinary View proof matching, direct target-range reads, and a
fixed-scratch bidirectional related-Paragraph scan to the nearest separator or
source boundary. Later phases add Check trusted hits; Apply and Anchor
integration; full race/drift semantics; then fixed A/B gates and the `0.2.1`
version decision.

## Completed: 0.2.0 release closure

The closed public `0.1.0` release remains immutable v3 evidence. The public
`0.2.0` release is the current v4 line governed by
the [seven-phase tracking task](../tasks/2026-08-30-backwriter-0.2.0-anddress-fast-path.md).
Phases 1–7 are complete. The exact drift matrix, full semantic suite, immutable
Git-object A/B builds, fixed Phase 2 fixtures, seven-sample release timings, and
representative counter/stack profiles pass every correctness and formal
performance gate. The Owner correction classifies million-hit peak-HWM slope,
not JSON payload size, as the result-memory recommendation; its 346.539 to
58.551 HWM bytes/hit change is an 83.10% reduction and passes. The subsequent
Line-only content-slice closure reaches a 272.111 ms 256 MiB median, 2.476×
faster than the paired v3 median and below the 336.949 ms target. Source
release-readiness is GO. The exact four-target artifacts, canonical manifest,
installers, 28-file live tree, endpoint verification, fresh installation, and
explicit `0.1.0`-to-`0.2.0` update are closed. Tags, GitHub Releases, crates.io,
Linux arm64, and any later platform or version remain separate Owner decisions.

The target replaces ordinal/exact-text identity with an ordinary Anddress that
authorizes one exact source state and byte range: workspace coordinate, logical
path, source-state hash, exact byte length, kind, and `[start, end)`. Search is
the only target finder and computes the hash during its discovery read. View
validates hash and length while capturing its bounded range, Check compares
only hash and length, and Apply requires that
hash before patching the recorded range. These consumers never search or
relocate an old target. A narrow call-local `CurrentObservation` contains only
the current hash and length; Search owns any target-required provisional ranges
and consumes or discards all of that source-local state before opening another
source. Only Anchor may carry live continuity across a Backwriter-owned Apply,
using direct before-range provenance and one unique same-kind after candidate.

History, a persistent index, context matching, external-change relocation, and
a full workspace cache remain excluded. SHA-256 and the v4 hard cutover are
closed Owner decisions implemented in Phase 3.

## Completed: Phase 7 and Line Search recommendation closure

V4 produces one Correct Apply, six Safe Rejects, and zero Wrong Applies across
the exact seven drift cells. Every reject preserves source bytes, a current
same-path File Anchor, and temporary/publication state; duplicate Paragraph,
ordinal drift, equal text at another range, and similar context are covered.

Large View and Check median CPU are 36.30% and 23.78% of v3, range Apply
prepublication CPU is 16.80%, Search wall is 79.78%–92.84%, and Search peak HWM
is 16.90%–92.89%. All p95 and peak-memory comparisons stay below the 110%
regression gate, while 128/256 MiB low-hit Search remains near 2.5 MiB peak HWM.
The million-hit cell lowers HWM from 346.539 to 58.551 bytes/hit. Its exact v4
Adapter output is separately 327.470 bytes/hit; payload/I/O size is not result
memory or a release gate. The Line-only fast path bulk-skips maximal ordinary
content slices, runs the existing KMP only for first-byte candidates or carried
partials, stops matching after a Line hit, and updates range/length in checked
chunks and spans. Fixed current/candidate measurements reduce the 256 MiB median
from 641.041 to 272.111 ms while preserving the exact 398-byte result and all
p95/HWM gates. Full raw evidence and profiles are retained in the tracking task.
Source release-readiness is GO and the `0.2.0` release is closed.

## Completed: Phase 6 direct Apply and Anchor consumers

Apply writes its one accepted no-follow source observation to same-parent
staging, verifies every operand and same-path live Anchor against that completed
hash/length state, and applies public v4 `[start,end)` geometry directly. Fixed
scratch readback of staging is the only before-source replay. Generated output
is incrementally validated and hashed while a direct prospective-after
projector retains only exact Line/Paragraph candidates and provenance markers.
No ordinal, exact-text locator, public-to-private mapper, target extractor, or
generic source event framer remains in production.

Anchor creation confirms an exact structural File, Paragraph, or Line with the
shared direct target projection. Anchored View reuses direct View projection.
After Apply, File continuity follows the new source identity; Paragraph/Line
continuity requires one same-kind candidate satisfying the existing
provenance, split/join/absorption, ambiguity, and collision rules. Raw-valid
nonstructural ranges remain ordinary View/Check/Apply inputs but cannot create
new Anchors.

## Completed: Phase 5 direct View and Check consumers

Ordinary View now performs one retained no-follow forward observation through
the common fixed-scratch UTF-8/NUL/hash/length reader. File retains only its
returned text, Paragraph retains only overlapping range bytes, and Line retains
only its range bytes plus minimal Paragraph-boundary state for the optional
relation of an exact current Line. The relation is output projection, not
currentness: caller-built valid nonstructural ranges return their exact UTF-8
bytes, and a nonstructural Line may have no related Paragraph.

Check retains coordinate/path grouping, original occurrence status, filtering,
reporting, order, and multiplicity, but each eligible source group now uses one
common observation and compares only SHA-256 and exact length. Kind and range
are not Check currentness evidence. Ordinary View and Check no longer consume
the generic event scanner or target tracker. Anchor creation and anchored View
now reuse the same direct target and View projection primitives completed in
Phase 6.

## Completed: Phase 4 target-specific Search observation

One common fixed-scratch reader now owns each retained source read, incremental
SHA-256, checked byte length, and UTF-8/NUL validation. Content Search selects a
minimal File, Paragraph, or Line projection instead of receiving generic
per-byte `SourceEvent` callbacks. Exact File lookup performs validation and hash
only, with no content matcher or Line framer. File content stops matcher and
framing work once `FullLine` is final while the common reader still validates
and hashes the remaining bytes. Paragraph and Line retain only their required
boundary, matcher, and provisional result-range state.

The observation and projections are private to one selected source and are
consumed immediately into shared v4 source identity and results after success;
late text, I/O, or resource failure discards them and publishes nothing. Search
retains no cross-call or cross-source observation. The existing final result
bucket sort remains required because component-wise directory DFS and mixed
scope traversal do not prove whole logical-path byte order.

## Completed: Phase 3 Anddress v4 value/wire kernel

Production source uses only `artext.backwriter-anddress.v4`: workspace
coordinate, logical path, complete-source SHA-256, exact source byte length,
target kind, and exact `[start,end)` range. Target text, terminator, ordinal,
and context are not identity. The public constructor is valid-only; encoding
uses eight ordered fields and canonical unsigned-decimal strings.

Past-structure mechanics excluded by the Protocol are not product or roadmap
work. Past-state recovery belongs to external history systems such as Git.

The completed H1 evidence kernel, H3 traversal/projection work, P1 Pick, V1
View, and A0–A2 Anchor work remain historical implementation milestones only.
They are not a freeze of v2 target semantics. Reuse their admission, no-follow,
UTF-8/NUL, exact Line, current-only, stateless, and no-limit mechanics where
they do not impose v2 source-wide identity.

The address model owns the exact-source/range algebra and sole v4 wire. Rust
producers, consumers, CLI round trips, and regressions cut over together
without a v3 decoder, encoder, migration, alias, or parallel schema. Apply and
Anchor consume v4 source-state/range identity directly; no private ordinal/text
compatibility representation remains.

## Remaining owner decisions

The persisted-source boundary is closed: editor-only buffers are outside Core,
and only source-visible mutations can affect observations or Anchor
continuity. This defines no Save event, watcher, durability guarantee, or
automatic address lifecycle.

Apply V1 semantic/public API/error authority and its single-source Edit Runtime
implementation are complete. Anchor's implemented seam is view-only.

Check V1 semantic/API/type/report authority and its stateless Runtime
implementation are complete. Data V1 semantic/public API/type/error authority
and Rust implementation are complete.

Edit V1 semantic/public API/type/error authority and its single-source Apply
Runtime implementation are complete.

The bounded source-memory Check, Search, View, Anchor, Apply streaming Rust
slices are complete.

For `0.2.1`, Phase 2 closes the minimal public host-authority seam, private
proof representation, cardinality, no-eviction/no-handle choice, and
multi-source Search installation policy. Phase 3 closes bounded ordinary View
reuse and its related-Paragraph path without whole-source retention. Check,
Apply, Anchor, invalidation closure, measurement, and the version decision
remain later phases.

## Completed: 0.1.0 exact File lookup

Core Search now has a distinct validated exact logical File request. It returns
one File Anddress for an admitted empty or nonempty regular UTF-8, NUL-free
source, returns Empty for a missing path or directory, and does not create an
empty content query, synthetic Line/Paragraph, index, cache, or new wire. The
canonical CLI exposes the request as `search /file <logical-path>` in one-shot
human/JSON and Session forms while reusing existing outcomes and writers.
Check accepts the resulting ordinary Search outcome, and its File Anddress can
drive existing Apply `StartOf` and `EndOf` positions for an empty source.

The historical milestone used Cargo `0.1.0`; current published source is Cargo
`0.2.0`, and `bw version` prints `Backwriter 0.2.0`. The separate `0.1.0`
stable-publication phase is completed below, while
the complete public `0.1.0-beta.3` bundle remains closed and immutable.

## Completed: CLI V1 capabilities and standalone Version/Update utilities

The canonical `bw` executable implements exact `bw version`, explicit
`bw update`, one-shot human and JSON Search,
View, and Check plus raw View, Session Pick, batch Check, Anchor, Edit, Apply, result
binding, and Data. JSON Search, View, and Check stream compact Adapter envelopes
with exact v4 Anddress objects where applicable and create no Core wire. The
Raw View is an exact-text Adapter projection that reuses ordinary View output
without a Core wire or new View meaning. The Session owns one Runtime and one
explicit caller-owned `DataStore`
until EOF or `exit`, plus local bindings and non-aliasing owning Anchedress
handles. It passes Pick candidate collections and
parsed predicates to the existing pure Core function, while direct Pick remains
unretained. Its batch Check passes exact matching binding clones to the existing
Runtime seams and prints only count summaries. Its Anchor commands call the
existing Runtime anchor, anchored View, and source-invalidation seams without a
registry, persistence, or automatic re-identification. Explicit typed Data
commands transfer exact Session-value clones to/from Core `DataStore` without
automatic storage or persistence. It directly reuses Core validation and public
Runtime seams. One-shot Data and Anchor remain intentionally unsupported because
their DataStore and live-handle contracts require Session lifetime. One-shot
Pick, batch Check, Edit, and Apply await collection or Edit transport schema
authority. Raw output other than completed one-shot View and further Session
behavior remain deferred Adapter decisions; CLI syntax creates no Core workflow
or wire authority. Version and Update are Adapter-owned standalone utilities
outside Core. Explicit Update invokes the canonical installer; background and
automatic update remain deferred.
The published `0.2.0` Core/Runtime and CLI surface is frozen after its recorded
phase gates; Adapter collection/Edit transport and Session machine output remain
separate Owner decisions.
The canonical Linux x86_64 release target is `x86_64-unknown-linux-musl`.
`x86_64-unknown-linux-gnu` remains the local development and test-host target.
The target choice and direct build verification are complete. The external
operations-owned distribution at
[https://backwriter.pentagration.com](https://backwriter.pentagration.com)
publishes the closed Backwriter `0.2.0` release for Linux/WSL x86_64,
macOS arm64, macOS x86_64, and Windows x86_64 from Source Authority revision
`2fad6e46d3a9d1da01f79f34b9ffc187447c76a8`. Linux uses
`x86_64-unknown-linux-musl`; macOS uses `aarch64-apple-darwin` at minimum 11.0
and `x86_64-apple-darwin` at minimum 10.12. Windows uses
`x86_64-pc-windows-gnu` and canonical `bw.exe`. Archives, checksum sidecars,
the expanded canonical manifest, POSIX and PowerShell installers, the CMD
Adapter, and publication are complete. The
installer uses the selected manifest SHA-256 and installs to
`$HOME/.local/bin/bw` with a same-directory rename without modifying
`PATH` or shell startup files. Fresh installation and replacement report the
installed or updated version respectively, with destination/PATH guidance kept
separate. Concurrent same-user HOME mutation is
caller-owned. This makes no universal Linux or kernel-compatibility
claim and gives GitHub no distribution authority. macOS artifacts have static
cross-build validation without a native-runtime test claim. Windows PowerShell
installs to `$HOME\.local\bin\bw.exe` without editing PATH or the profile;
Windows build and installer verification make no native-runtime or native-CMD
claim.
The current Cargo package and library crate are `backwriter` at published
`0.2.0`; the sole canonical executable and external Adapter command are `bw`.
The public beta.1, beta.2, and beta.3 files remain unchanged immutable
prior artifacts. The complete stable `0.1.0` Linux/macOS/Windows version
directory is immutable, the planned matrix is complete, and the stable release
is closed. The complete `0.2.0` version directory is likewise immutable; the
current installers and manifest select `0.2.0` in the exact 28-file public tree.

Backwriter Core construction from an accepted current observation and Search
delivery of those values are fixed authority boundaries, not a target registry,
issuance lifecycle, locator lookup/reuse state, durable identity, or global
identity. The optional current source-state proof retains no target map or
result.

## Completed: Anchor live continuity

Anchor live-continuity authority and its public Runtime surface are implemented.
Its retained contract is opaque owning Runtime-local
continuity, non-aliasing `AlreadyLive`, no history, persistence, or re-
identification, and logical-source invalidation. The A0–A2 source-wide
transition model is retired.

## Deferred capability decisions

Plural View input, ranges, descendants, and partial behavior remain owner
decisions. Apply's reference letter is unassigned. Future Search spill is
separately owned by a host-provided system root and must not create
repository-local authority.

## Completed: 0.1.0 stable distribution

The Linux/WSL x86_64, macOS arm64/x86_64, Windows PowerShell x86_64, and
Windows CMD stable `0.1.0` publication is complete. At that milestone, the
installers and manifest selected `0.1.0`; its complete version directory remains
immutable. Current installers and the manifest select the closed `0.2.0`
distribution recorded below.
Existing public `0.1.0-beta.1`, `0.1.0-beta.2`, and `0.1.0-beta.3` files remain
unchanged and immutable. Explicit `bw update` is complete, while background or
automatic update remains deferred. Linux arm64, later versions, tags, GitHub
Releases, and crates.io remain deferred and require separate Owner authority.
The completed publication defines no universal host-compatibility,
native-macOS-runtime, native-Windows-runtime, or native-CMD claim.

## Completed: 0.2.0 distribution

The existing builders reproduced the four canonical artifacts from Source
Authority revision `2fad6e46d3a9d1da01f79f34b9ffc187447c76a8`, and the
generator reproduced the exact 876-byte manifest. The version-specific
publisher added eight immutable `0.2.0` files, replaced the POSIX and
PowerShell installers, and published the manifest last without changing the 16
prior versioned files or `install.cmd`. An idempotent rerun reused the complete
28-file tree without metadata change. Loopback and public HTTPS GET/HEAD,
task-local fresh installation, explicit stable `0.1.0` update, and installed
`0.2.0` capability probes passed. Native macOS and Windows runtime execution was
not performed. Tags, GitHub Releases, crates.io, cache purge, and new platform
support remain outside this closure.
