# Backwriter Current State

## Version boundary

The closed public `0.1.0` release remains immutable v3 evidence. Current
source-ready `0.2.1` Rust, Cargo, tests, and CLI, plus the closed public `0.2.0`
release, use the hard-cutover
Anddress v4 API
and wire. Phases 3–7 implement and verify SHA-256 source identity, exact byte length,
target kind, `[start,end)` range, target-specific Search observation, direct
View/Check consumption, and direct Apply/Anchor consumers without a v3
compatibility seam. The integrated correctness and formal performance gates
pass. The Owner correction confirms that the million-hit recommendation is
result-memory HWM, not JSON payload: 346.539 to 58.551 HWM bytes/hit is an
83.10% reduction and passes. The Line-only content-slice fast path then lowers
the fixed 256 MiB median from the paired v3 673.898 ms to 272.111 ms, a 2.476×
speedup that closes the remaining recommendation. Source release-readiness is
GO, and the exact four-target `0.2.0` artifact set, canonical manifest,
installers, live publication, endpoint verification, fresh installation, and
explicit update are closed. The evidence
tracker is
[Backwriter 0.2.0 Anddress fast path](../tasks/2026-08-30-backwriter-0.2.0-anddress-fast-path.md).

## Backwriter 0.2.1 source-ready observation reuse

The `0.2.1` observation-reuse target is source-ready and not published. Phase
7A closes the sole failed historical Phase 7 performance gate; Phase 7B then
remeasures the complete fixed matrix and passes every formal gate. Cargo,
source, CLI, and version output are `0.2.1`. The work preserves the
complete v4 Anddress API/wire and the closed `0.2.0` release. The Protocol closes two
execution modes: the default `WorkspaceRuntime`, one-shot CLI, and ordinary CLI
Session remain Untrusted Mode with the existing per-call one-read/hash path;
only an explicit Host-authoritative Mode may reuse a Runtime-local,
RAM-only, replace-only current SHA-256/length proof bound to Runtime, workspace,
admission, source generation, and logical path.

The host must coordinate all source-visible writers and path replacements,
exclude mutation from proof selection through call completion, and invalidate
synchronously before mutation. Watchers and filesystem metadata are not proof.
A trusted miss or incomplete guard falls back to `0.2.0` observation. Search
may install proof without caching results; View and Check may skip only
source-size-proportional read/hash work on a complete trusted hit; confirmed
Apply may replace proof with its already computed prospective-after hash/length,
while exact no-op preserves it. Host-coordinated or opaque mutation, explicit
invalidation, authority change, unavailable source, uncertain publication, or
Runtime drop discards proof.

Phases 2 through 6 add `WorkspaceRuntime::open_host_authoritative` and
`WorkspaceRuntime::invalidate_source` without changing `open` or any capability
signature. Private synchronized state holds at most one replace-only
hash/length proof per logical path and no retained handle. A successful Host
Search installs every source fully observed by that whole successful call;
each entry remains independent and makes no workspace-completeness claim. A
failed call installs none of its provisional proofs. Untrusted Search installs
none. Ordinary Host View uses an exact matching proof to read only its recorded
range plus fixed-scratch nearest-boundary Line relation evidence; a proof miss
and every Untrusted View retain the complete `0.2.0` observation path. A
same-path proof mismatch fails before source access. Host Check uses one copied
matching path proof to classify every occurrence in that coordinate/path group
by hash and length with no source open, read, or hash. Mixed matches and
mismatches preserve their original order and multiplicity; a mismatch neither
falls back nor changes proof. Proof miss, Untrusted execution, poison, or
unusable private state keeps the existing one-observation-per-source fallback.
Host Apply validates and stages an exact matching proof length without a before
hash, rejects operand mismatches before source access, preserves proof across
direct and byte-identical no-op, and installs the existing prospective-after
hash/length only after confirmed changed publication. Proof misses and
Untrusted execution retain the complete before-observation path.

Phase 6 closes the complete proof lifecycle. Both source invalidation methods
delegate to one I/O-free path-exact proof-plus-Anchor invalidator. After
invalidation, same-length or different-length replacement, deletion, invalid
UTF-8, and NUL source all make stale View, Check, and Apply safe-reject through
the existing fallback paths. Proof mismatch remains I/O-free and preserves
state for ordinary View and Apply; anchored View instead fail-closes its live
same-path continuity and proof. Matching anchored View now shares ordinary
trusted View execution rather than performing a second full-source observer.
Open, seek, read, short, and resource failure remove a consumed View proof but
preserve Anchor continuity without mutation evidence. Apply retains its Phase
5 failure boundaries, no-op preservation, confirmed after installation, and
uncertain-publication fail-closure. Unsignaled mutation and mutation during a
capability call remain Host contract violations; Phase 6 adds no watcher,
metadata check, lock, CAS, retry, or supported race behavior.

Phase 7 exports immutable A=`2fad6e4` and B=`a24ff5e`, reproduces the fixed
fixtures, and measures A Untrusted, B Untrusted, and B Host on CPU 2 with one
warm-up and seven order-rotated samples. Search, Host Check zero-I/O,
result-memory, whole-source-retention, v4 semantics, and Wrong-Apply gates pass.
The formal Host Search-to-late-Line View gate fails: median `1,079.943` ms and
p95 `1,096.362` ms exceed the 400 ms ceiling and 350 ms recommendation. The
late Line's related Paragraph has no separator before it, so matching trusted
View performs source-size-proportional reverse/forward boundary reads; the
composite records `536,870,913` `rchar` without retaining the whole source.
Phase 7 starts no optimization and changes no production Rust or version.
Phase 7A then removes the related-Paragraph path's private per-byte cursor
layer and scans the same fixed 8,192-byte scratch chunks directly. Against
baseline `0f1cc6b`, the exact 256 MiB no-separator Host Search-to-late-Line View
median is `331.527` ms with p95 `332.547` ms, passing both the 400 ms gate and
350 ms recommendation. Its output and `536,870,913` `rchar` remain byte-exact;
Search, zero-I/O Host Check, one-million-result memory, ordinary/anchored and
Untrusted View, and Wrong-Apply evidence remain intact. This performance
closure does not decide the `0.2.1` version or authorize artifacts or
publication.

Phase 7B uses immutable A=`2fad6e4` and candidate `d3d0861`, the same task-local
source for A, B Untrusted, and B Host, CPU 2 P-core, `powersave`, one warm-up,
and seven order-crossed samples. BU/BH 256 MiB Search medians are `267.397` and
`267.273` ms. Host Search-to-late-Line View is `324.254` ms, Host Check proof
hit is exact zero source I/O, and one-million-hit peak memory is `58.609`
bytes/hit. All 17 cell payloads, ordering, multiplicity, v4 evidence, related
Paragraph evidence, and source results are identical across A/BU/BH. The
seven-cell drift regression remains Correct `1`, Safe Reject `6`, Wrong `0` in
both modes. This closes source readiness only; artifacts and publication remain
separate, and official public `0.2.0` is unchanged.

Trusted Search followed by ordinary View no longer performs View's complete
source read or hash; File still returns the complete file range, while
Paragraph and Line retain only their returned target allocation. Host Search
followed by Check performs no Check source observation on a proof hit; Search
followed by Host Apply still stages one retained live-source read, but a
matching proof removes its before SHA-256 work. Apply has no separate pre-hash
source pass: proof misses and Untrusted execution compute before hash/length
while staging, while prospective-after hash/length are computed during output
emission. Confirmed changed Host publication retains only that after proof; the
next trusted View, Check, or Apply may reuse it without an intervening Search.
The
[0.2.1 phase tracker](../tasks/2026-08-30-backwriter-0.2.1-current-observation-reuse.md)
owns the audited flow, Phase 2–7B choices, complete raw samples, checksums, and
gate evidence. Artifacts and publication remain separate Owner-authorized
scopes.

## Core capability inventory

| Letter | Word | Current status |
| --- | --- | --- |
| S | Search | Rust implementation with one-read target-specific v4 projection and exact File lookup. |
| V | View | Rust implementation with direct v4 exact-source/range projection. |
| P | Pick | Rust implementation over complete v4 values. |
| A | Anchor | Rust implementation with Runtime-local live continuity. |
| C | Check | Rust implementation with v4 hash/length-only batch currentness reporting. |
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
The published `0.2.0` Core/Runtime and CLI surface is frozen. The current source
has completed the v4 value/wire hard cutover. Further
Adapter work still requires owner authority for
collection/Edit transport or Session machine output.
The canonical Linux x86_64 release target is `x86_64-unknown-linux-musl`; the
GNU target is retained for local development and tests. Target selection and
direct build verification are complete. The external operations-owned
distribution at
[https://backwriter.pentagration.com](https://backwriter.pentagration.com)
publishes the closed Backwriter `0.2.0` release for Linux/WSL x86_64,
macOS arm64, macOS x86_64, and Windows x86_64. Linux uses
`x86_64-unknown-linux-musl`; macOS uses
`aarch64-apple-darwin` at minimum 11.0 and `x86_64-apple-darwin` at minimum
10.12. Windows uses `x86_64-pc-windows-gnu` and canonical `bw.exe`. Their
artifacts, manual-verification checksum sidecars, expanded canonical manifest,
POSIX and PowerShell installers, and publication are complete from Source
Authority revision `2fad6e46d3a9d1da01f79f34b9ffc187447c76a8`. The installer verifies the
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
beta.3 files remain unchanged and immutable. The complete stable `0.1.0` and
current `0.2.0` version directories are immutable, the full planned matrix is
complete, and the `0.2.0` release is closed; any later platform or version
requires separate Owner authority. Tags, GitHub Releases,
crates.io publication, and GitHub distribution
remain outside the completed publication.
The current Cargo package and library crate are `backwriter` at published
`0.2.0`; the sole canonical executable and external Adapter command are `bw`.
There is no current `backwriter` binary, alias, or wrapper. Product prose
continues to use Backwriter, and persisted Core wire/private-path and
distribution artifact/domain contracts keep their existing names. `0.2.0`
publication is closed: the exact 28-file public tree retains all prior
versioned files and `install.cmd`, while the current installers and manifest
select `0.2.0` and `bw update` delegates to that official installer. The
installers accept only the exact stable `0.1.0` and current `0.2.0` manifests;
beta.3 acceptance is retired.

## Published 0.2.0 authority

Phase 7 reproduces the Phase 2 fixtures and v3 timing binary from immutable Git
objects, proves one Correct Apply plus six Safe Rejects and zero Wrong Applies
across the exact drift matrix, and preserves source, current Anchor, and
publication state on rejection. Late View, late Check, range Apply, Search wall,
p95, peak HWM, and bounded low-hit source-memory gates pass. The corrected
million-hit result-memory recommendation and the subsequent 2× Line Search
closure also pass. The Line fast path removes per-byte checked-offset and
state-zero KMP work while reusing the one-read observer, existing failure table,
terminator carry, and result buckets. Source release-readiness is GO. Release
closure used the existing external operations authority to publish the exact
artifacts and pointers; it creates no tag, GitHub Release, crates.io
publication, or new Core authority.

Current-only does not require historical identity. Runtime keeps each
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
exact ranges. Ordinary View validates hash and length while capturing the
caller-provided range from that same one-read observation. Check compares only
hash and length; kind and range are not currentness evidence. Apply enforces the
complete source-state precondition, then patches the public v4 range directly
from fixed-chunk staging. It neither finds nor relocates a target. A changed
source invalidates an ordinary Anddress.

Anchor remains the sole continuity boundary. Anchor creation confirms exact
File/Paragraph/Line structure by direct target projection. A Backwriter-owned
Apply reflects a live Anchor only through exact before-range provenance and a
unique same-kind prospective-after candidate; external changes invalidate
rather than relocate ordinary Anddresses or Anchors. The source-hash
algorithm is SHA-256 and the compatibility policy is a hard cutover: production
has no v3 decoder, encoder, alias, or migration layer.

## Implemented published 0.2.0 current-only Runtime contract

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
Ordinary View uses that common observer directly: File captures its returned
text, Paragraph captures only its range overlap, and Line captures only its
range before classifying a tail terminator. The same pass keeps only minimal
Paragraph-boundary state for a Search-issued Line's optional related Paragraph;
a valid caller-built nonstructural Line still returns exact range bytes and may
have no related Paragraph. Check groups by coordinate/path, observes each
eligible source once, and compares each occurrence's hash and length only.
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
creates no target registry, issuance lifecycle, locator reuse, durable identity,
or global identity. The narrow current source-state proof is neither
target lookup nor result retention.

Search projects v4 source identity and ranges directly; Pick provides
`same_file` instead of observation, paragraph, or hierarchy relations; and
View checks exact source state and captures the caller range from one current
read. Check currentness uses only that source state's hash and length. There is
no compatibility decoder, migration, alias, or parallel schema. The algebra
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
