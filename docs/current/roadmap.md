# Backwriter Roadmap

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

## Completed: CLI V1 human and JSON Search/View/Check, raw View, Session Pick, batch Check, Anchor, Edit, Apply, result-binding, and Data Adapter

The canonical `backwriter` executable implements one-shot human and JSON Search,
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
or wire authority.
Core/Runtime beta implementation freeze holds. CLI V1 beta implementation
freeze is complete; work resumes only with owner authority for collection/Edit
transport or Session machine output.
The canonical Linux x86_64 release target is `x86_64-unknown-linux-musl`.
`x86_64-unknown-linux-gnu` remains the local development and test-host target.
The target choice and direct build verification are complete. The external
operations-owned distribution at
[https://backwriter.pentagration.com](https://backwriter.pentagration.com)
publishes Backwriter `0.1.0-beta.1` for Linux x86_64 and WSL x86_64 from Source
Authority revision `e6217d93bf241edd4040319113b7116c3126a8e6`. Its archive,
manual-verification checksum sidecar, canonical manifest, installer, and initial
publication are complete. The installer uses the manifest SHA-256 and
atomically installs to `$HOME/.local/bin/backwriter` without modifying `PATH`
or shell startup files. This makes no universal Linux or kernel-compatibility
claim and gives GitHub no distribution authority.
The Cargo package and library crate are `backwriter` at `0.1.0-beta.1`; the
canonical executable is `backwriter`.

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

## Deferred distribution decisions

Linux arm64, macOS, and Windows distributions remain unsupported. Their future
support and every version contract after `0.1.0-beta.1` require separate owner
authority. The completed Linux/WSL x86_64 publication defines neither universal
host compatibility nor a tag, GitHub Release, or crates.io publication.
