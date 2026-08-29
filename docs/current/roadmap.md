# Backwriter Roadmap

## Next: 0.2.0 Anddress fast path

The closed public `0.1.0` release remains the immutable implemented v3
baseline. `0.2.0` is an unpublished local source-development target governed by
the [seven-phase tracking task](../tasks/2026-08-30-backwriter-0.2.0-anddress-fast-path.md).
Phase 1 records the semantic authority and is complete; Phase 2 profiling and
reproducible baseline measurement are next. No v4 Rust, Cargo, benchmark,
version, CLI, or release implementation is complete.

The target replaces ordinal/exact-text identity with an ordinary Anddress that
authorizes one exact source state and byte range: workspace coordinate, logical
path, source-state hash, exact byte length, kind, and `[start, end)`. Search is
the only target finder and computes the hash during its discovery read. View
uses hash plus bounded range, Check compares the hash, and Apply requires that
hash before patching the recorded range. These consumers never search or
relocate an old target. A narrow `CurrentObservation` may retain only current
hash, length, and minimum required ranges, and must be discarded on state
change. Only Anchor may arithmetically transform live ranges across a
Backwriter-owned Apply.

History, a persistent index, context matching, external-change relocation, and
a full workspace cache remain excluded. The source-hash algorithm and v3/v4
compatibility policy are unresolved Owner decisions and are gates for a later
implementation phase, not Phase 1 decisions.

## Completed: v3 target-local Anddress

Production uses `artext.backwriter-anddress.v3`: independent target-local
File/Paragraph/Line values with workspace coordinate, logical path, ordinals,
and Line exact extent. Source-wide observation material is not target equality.

Past-structure mechanics excluded by the Protocol are not product or roadmap
work. Past-state recovery belongs to external history systems such as Git.

The completed H1 evidence kernel, H3 traversal/projection work, P1 Pick, V1
View, and A0–A2 Anchor work remain historical implementation milestones only.
They are not a freeze of v2 target semantics. Reuse their admission, no-follow,
UTF-8/NUL, exact Line, current-only, stateless, and no-limit mechanics where
they do not impose v2 source-wide identity.

The address model owns the target-local raw locator algebra and the sole v3
wire. The Rust producers, consumers, and regressions cut over together without
a compatibility decoder, migration, alias, or parallel schema.

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

## Completed: 0.1.0 exact File lookup

Core Search now has a distinct validated exact logical File request. It returns
one File Anddress for an admitted empty or nonempty regular UTF-8, NUL-free
source, returns Empty for a missing path or directory, and does not create an
empty content query, synthetic Line/Paragraph, index, cache, or new wire. The
canonical CLI exposes the request as `search /file <logical-path>` in one-shot
human/JSON and Session forms while reusing existing outcomes and writers.
Check accepts the resulting ordinary Search outcome, and its File Anddress can
drive existing Apply `StartOf` and `EndOf` positions for an empty source.

The Cargo package and source executable version are `0.1.0`, and `bw version`
prints `Backwriter 0.1.0`. This source milestone did not itself publish a
distribution; the separate stable-publication phase is completed below, while
the complete public `0.1.0-beta.3` bundle remains closed and immutable.

## Completed: CLI V1 capabilities and standalone Version/Update utilities

The canonical `bw` executable implements exact `bw version`, explicit
`bw update`, one-shot human and JSON Search,
View, and Check plus raw View, Session Pick, batch Check, Anchor, Edit, Apply, result
binding, and Data. JSON Search, View, and Check stream compact Adapter envelopes
with exact v3 Anddress objects where applicable and create no Core wire. The
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
The `0.1.0` Core/Runtime and CLI source surface is frozen after exact File
lookup. The separate `0.2.0` source target proceeds only through its recorded
phase gates; Adapter collection/Edit transport and Session machine output remain
separate Owner decisions.
The canonical Linux x86_64 release target is `x86_64-unknown-linux-musl`.
`x86_64-unknown-linux-gnu` remains the local development and test-host target.
The target choice and direct build verification are complete. The external
operations-owned distribution at
[https://backwriter.pentagration.com](https://backwriter.pentagration.com)
publishes the closed Backwriter `0.1.0` stable release for Linux/WSL x86_64,
macOS arm64, macOS x86_64, and Windows x86_64 from Source Authority revision
`25a0dbc38dc78cc7592b219e9070af3c0e201c17`. Linux uses
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
The current Cargo package and library crate are `backwriter` at
`0.1.0`; the sole canonical executable and external Adapter command are
`bw`. The public beta.1, beta.2, and beta.3 files remain unchanged immutable
prior artifacts. The complete stable `0.1.0` Linux/macOS/Windows version
directory is immutable, the planned matrix is complete, and the stable release
is closed.

Backwriter Core construction from an accepted current observation and Search
delivery of those values are fixed authority boundaries, not a separate registry,
issuance lifecycle, lookup/reuse state, durable identity, or global identity.

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
Windows CMD stable `0.1.0` publication is complete. Current installers and the
manifest select `0.1.0`; the complete stable version directory is immutable.
Existing public `0.1.0-beta.1`, `0.1.0-beta.2`, and `0.1.0-beta.3` files remain
unchanged and immutable. Explicit `bw update` is complete, while background or
automatic update remains deferred. Linux arm64, later versions, tags, GitHub
Releases, and crates.io remain deferred and require separate Owner authority.
The completed publication defines no universal host-compatibility,
native-macOS-runtime, native-Windows-runtime, or native-CMD claim.
