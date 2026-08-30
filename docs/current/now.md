# Backwriter Current State

## Version boundary

The closed public `0.1.0` release remains immutable v3 evidence. Current Rust,
Cargo, tests, and CLI use the unpublished `0.2.0` hard-cutover Anddress v4 API
and wire. Phases 3–4 implement SHA-256 source identity, exact byte length,
target kind, `[start,end)` range, and target-specific Search observation without
a v3 compatibility seam. No `0.2.0`
artifact or publication exists. The tracking plan is
[Backwriter 0.2.0 Anddress fast path](../tasks/2026-08-30-backwriter-0.2.0-anddress-fast-path.md).

## Core capability inventory

| Letter | Word | Current status |
| --- | --- | --- |
| S | Search | Rust implementation with one-read target-specific v4 projection and exact File lookup. |
| V | View | Rust implementation with v4 exact-source currentness. |
| P | Pick | Rust implementation over complete v4 values. |
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

## CLI Adapter V1 capabilities and standalone Version/Update utilities

The repository includes the canonical `bw` CLI Adapter. Its completed
scope includes exact `bw version`, explicit `bw update`, one-shot human and
JSON Search, View, and Check plus raw View and Session Pick,
batch Check, Anchor, Edit, Apply, result binding, and explicit Data over the existing public
Runtime seams. The Session retains one Runtime, one caller-owned `DataStore`,
and explicit CLI-local bindings plus non-aliasing owning Anchedress handles.
Session Pick passes a named candidate collection and a
CLI-parsed predicate to the existing pure Core function. Session batch Check
passes an exact matching binding clone to `check_search` or `check_pick` and
exposes only its report counts. Session Anchor creates an opaque Runtime-local
handle, views it through the existing anchored seam, and invalidates only its
logical source. One-shot Search, View, and Check JSON stream compact Adapter
envelopes without creating a Core wire; related or filtered values are exact
existing v4 Anddress objects. Raw View is an explicit Adapter exact-text
projection and creates no Core wire or new View meaning. Data transfers exact
clones from explicit Session values into the existing typed Core store and reads
them back without capability execution.
Content Search keeps its literal query, scope, and target projection. Its
distinct `search /file <logical-path>` form performs exact logical File lookup
without a content query and reuses the same human/JSON outcome writers in both
one-shot and Session execution. Empty and nonempty admitted regular UTF-8,
NUL-free sources return one File Anddress; missing paths and directories are
Empty. Version and Update are Adapter-owned standalone utilities outside Core;
explicit Update invokes the canonical distribution installer and adds no
background or automatic updater. The Adapter adds no Core API, wire, workflow,
provenance, automatic Data storage,
registry, persistence, or retained Core state beyond existing Anchor continuity.
One-shot Data and Anchor remain intentionally unsupported because their
DataStore and live-handle contracts are Session-lifetime state. One-shot Pick,
batch Check, Edit, and Apply await collection or Edit transport schema
authority. Raw output other than completed one-shot View and further Session
behavior remain deferred under the [CLI V1 authority](../architecture/backwriter-cli-v1.md).
The published `0.1.0` Core/Runtime and CLI surface remains frozen. Current
unpublished `0.2.0` source has completed the v4 value/wire hard cutover. Further
Adapter work still requires owner authority for
collection/Edit transport or Session machine output.
The canonical Linux x86_64 release target is `x86_64-unknown-linux-musl`; the
GNU target is retained for local development and tests. Target selection and
direct build verification are complete. The external operations-owned
distribution at
[https://backwriter.pentagration.com](https://backwriter.pentagration.com)
publishes the closed Backwriter `0.1.0` stable release for Linux/WSL x86_64,
macOS arm64, macOS x86_64, and Windows x86_64. Linux uses
`x86_64-unknown-linux-musl`; macOS uses
`aarch64-apple-darwin` at minimum 11.0 and `x86_64-apple-darwin` at minimum
10.12. Windows uses `x86_64-pc-windows-gnu` and canonical `bw.exe`. Their
artifacts, manual-verification checksum sidecars, expanded canonical manifest,
POSIX and PowerShell installers, and publication are complete from Source
Authority revision
`25a0dbc38dc78cc7592b219e9070af3c0e201c17`. The installer verifies the
manifest-authoritative SHA-256 and installs to `$HOME/.local/bin/bw`
with a same-directory rename, without changing `PATH` or shell startup files.
Fresh installation prints the installed version and replacement prints the
updated version; destination/PATH guidance is separate.
PowerShell installs to `$HOME\.local\bin\bw.exe` without changing PATH or the
PowerShell profile. The public CRLF CMD Adapter downloads exactly that
PowerShell installer over HTTPS-only TLS transport, delegates all installation
meaning, cleans its temporary task directory, and preserves the child exit
code. It duplicates no installer authority.
Concurrent same-user HOME mutation is caller-owned.
macOS and Windows support are based on static cross-build verification without
native runtime-test or native CMD claims. Linux arm64 remains unsupported, and
no universal host compatibility is claimed. The public beta.1, beta.2, and
beta.3 files remain unchanged and immutable. The complete stable `0.1.0`
version directory is immutable, the full planned matrix is complete, and the
stable release is closed; any later platform or version requires separate Owner
authority. Tags, GitHub Releases,
crates.io publication, and GitHub distribution
remain outside the completed publication.
The current Cargo package and library crate are `backwriter` at unpublished
`0.2.0`; the sole canonical executable and external Adapter command are `bw`.
There is no current `backwriter` binary, alias, or wrapper. Product prose
continues to use Backwriter, and persisted Core wire/private-path and
distribution artifact/domain contracts keep their existing names. Stable
publication is closed: the current installers and manifest select `0.1.0`, and
`bw update` delegates to that official stable installer. Exact beta.3 manifest
acceptance remains transition compatibility only.

## Unpublished 0.2.0 authority

Current-only does not require historical identity. Phase 4 keeps each
`CurrentObservation` call-local to one selected source. It contains only the
current source hash and exact byte length; Search projections separately retain
only their target-required matcher, boundary, and provisional range state.
Success consumes that state immediately into v4 identity and results, while any
text, I/O, or resource failure discards it without publication. Runtime retains
no prior observation, whole source, parse tree, result, persistent index,
relocation context, or full workspace cache.

An ordinary v4 Anddress is workspace coordinate, logical path, complete-source
SHA-256, exact byte length, target kind, and one inclusive-start/exclusive-end
byte range. Phase 3 implements this value/wire and all current production
callers. Search computes the hash during the same source read that discovers
exact ranges. View and Check enforce the hash, length, kind, and range from
their one-read observations. Apply first enforces the complete source-state
precondition, then temporarily resolves the verified range into its existing
private call-local parser representation. It does not relocate across source
drift. A changed source invalidates an ordinary Anddress.

Anchor remains the sole continuity boundary. Only a live Anchor may undergo an
arithmetic range transform caused by a Backwriter-owned Apply. External changes
invalidate rather than relocate ordinary Anddresses or Anchors. The source-hash
algorithm is SHA-256 and the compatibility policy is a hard cutover: production
has no v3 decoder, encoder, alias, or migration layer.

## Implemented unpublished 0.2.0 current-only Runtime contract

The v4 Search and View implementation is current-only and stateless;
Pick is pure and stateless over caller input. `WorkspaceRuntime::search`, `WorkspaceRuntime::view`,
`WorkspaceRuntime::apply(&mut self, &Edit)`, `WorkspaceRuntime::check`,
`check_search`, `check_pick`, `anchor`, `view_anchored`, and
`invalidate_anchored_source` are the implemented Runtime seams. Search traverses
admitted Workspace Source through retained capability-relative no-follow
handles. Content Search observes each selected regular file once through a
common fixed-scratch reader that owns UTF-8/NUL validation, SHA-256, and checked
byte length. It chooses a File-, Paragraph-, or Line-specific projection,
orders the completed results, then drops that source before opening another.
Exact File Search validates one logical path and observes that admitted regular
source once under the same safety and text policy before returning its File
Anddress; it performs no content matching, Line framing, or traversal.
After a capability call returns, Runtime retains no ordinary observation,
source, result, snapshot, lease, registry, history, or authenticity state.

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

## Implemented 0.2.0 v4 exact-source address kernel

File, Paragraph, and Line are independent target addresses with structural
relationships, not a persistent parent/child identity tree. Their raw equality
is workspace coordinate, logical path, complete-source SHA-256, exact source
byte length, target kind, and exact byte range. Admission is not raw equality.
Any source-byte change invalidates every ordinary Anddress for the prior source
state; no ordinal, target text, or context is identity.
`Block` is historical wording for the existing blank-line-bounded Paragraph and
creates no type, alias, variant, or wire value.

`artext.backwriter-anddress.v4` is the sole accepted production wire. Its eight
canonical ordered fields are version, workspace coordinate, logical path,
source-state hash, source byte length, kind, byte start, and byte end. Length
and offsets are canonical unsigned-decimal strings. Well-formed v3 input is
`UnsupportedVersion`; v3 remains only immutable `0.1.0` release evidence.

Backwriter Core constructs and provides target Anddress values from an accepted
current observation. Search delivers them as results; it is not an issuer. This
creates no separate registry, issuance lifecycle, lookup/reuse state, durable
identity, or global identity.

Search projects v4 source identity and ranges directly; Pick provides
`same_file` instead of observation, paragraph, or hierarchy relations; and
View checks exact source state and range from one current read. There is no
compatibility decoder, migration, alias, or parallel schema. The algebra
creates no continuity or historical-identity claim.

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
