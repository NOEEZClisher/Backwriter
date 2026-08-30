# Verification

## 0.2.1 observation-reuse development gates

`0.2.1` is partially implemented and unpublished. Phase 2 adds the Host Runtime
constructor, source invalidation kernel, private proof state, and
successful-Search proof installation. Phase 3 adds bounded ordinary View proof
consumption. Phase 4 adds Check current-proof group classification;
Phase 5 adds Apply proof precondition reuse, exact no-op preservation,
prospective-after proof installation, and coupled Anchor reflection;
Cargo/version, v4 wire, default Untrusted behavior, and the closed public
`0.2.0` release remain unchanged. The
Protocol owns default Untrusted Mode and explicit Host-authoritative Mode; the
[phase tracker](../tasks/2026-08-30-backwriter-0.2.1-current-observation-reuse.md)
owns the execution audit, fixed not-yet-remeasured `0.2.0` comparison inputs,
and phase evidence.

Phase 2 regressions prove that Untrusted Search installs no proof; Host exact
File and content Search install exact hash/length; re-search replaces one path;
workspaces and logical paths remain isolated; multi-source success installs all
fully observed sources while late failure installs none; and invalidation,
unavailable source, Anchor fail-closure, Apply entry, and uncertain publication
remove only affected proof. Structural checks exclude retained bytes, results,
ranges, history, public getters, CLI Host activation, and Debug disclosure. At
Phase 2 closure, Check and Apply still used the full `0.2.0` observation path.
Search remains the only target finder; Check and Apply must not relocate or
context-match.

Phase 3 regressions prove ordinary View proof hit, miss, mismatch, explicit
invalidation fallback, and unchanged Untrusted behavior. They cover
File/Paragraph/Line output, every terminator, Unicode, whitespace and
raw-valid nonstructural Lines, UTF-8 range cuts, short reads, matching-proof
removal, fixed-scratch boundaries, and long adjacent Lines. Structural and
counting-reader evidence proves the trusted path computes no SHA-256 or
complete observation, reads no source-size-proportional prefix merely to find a
Line relation, releases proof state before I/O, and retains only the returned
target plus fixed scratch. The Phase 3 development suite passes 215 GNU-host
Rust tests.

Phase 4 regressions prove Host Search to raw, Search-outcome, and Pick-outcome
Check hits; 10,000 mixed matching/stale occurrences with duplicates and exact
order; hash and length mismatches without fallback or proof mutation; and
raw-valid nonstructural Paragraph and Line currentness. They cover mixed
proof-hit/proof-miss sources, one observation per miss group, no Check proof
installation, Untrusted/miss/poison/unusable fallback, explicit invalidation
followed by changed, missing, and invalid source, and unchanged workspace,
private-path, admission, empty-report, and Resource boundaries. Structural
evidence fixes the proof lookup before filesystem open, one fallback observer,
owned fixed-size digest evidence, and lock release before I/O, hashing, and
report assembly. The Phase 4 development suite passes 220 GNU-host Rust tests.

Phase 5 regressions prove Host Search-to-Apply proof hit without a before hash,
exact changed publication, prospective-after proof installation, a second Apply
using that proof, and old-address Safe Reject. Direct and assembled identical
no-op preserve proof, live Anchor, inode, source bytes, and temporary state;
proof-mismatched operands reject before source access while preserving proof
and Anchor. They cover Host proof miss, Untrusted fallback through the complete
existing suite, post-Apply View and Check reuse, File/Paragraph/Line Anchor
reflection from the same after identity, unrelated-path isolation, short/grown/
invalid trusted source fail-closure, temporary collision, source read/resource
classification, publication uncertainty, and the existing duplicate-drift
matrix. Structural evidence fixes exact-length-plus-one fixed-scratch staging,
zero SHA-256 on the trusted before path, prospective proof preparation before
publication, no proof lock across I/O/hash/emission/publication, and no retained
source, previous proof, or history. The Phase 5 development suite passes 228
GNU-host Rust tests.

Future verification must prove the remaining invalidation/race matrix. No Phase
5 result is a benchmark or release-readiness claim.

The fixed later gates are Search median at most 105% of `0.2.0`, trusted
Search-to-View at most 400 ms with at most 350 ms recommended, zero
source-size-proportional I/O/hash for a trusted Check hit, memory slope at most
110%, zero whole-source retention, and zero Wrong Apply. These are future A/B
gates, not measurements or release-readiness claims in Phase 1.

## 0.2.0 Phase 7 and Search recommendation closure

The closed public `0.1.0` release remains immutable v3 evidence. Current
published `0.2.0` Rust, Cargo, CLI, and tests use only Anddress v4. Phase 3
implements the SHA-256 source-state/value/wire kernel and hard-cutover decoding.
Phase 4 implements one-read target-specific Search observation. Phase 5 makes
ordinary View and Check direct consumers of the same hash/length observer.
Phase 6 makes Apply and Anchor direct v4 range/provenance consumers and removes
the last legacy locator/parser and generic event-framer path. Phase 7 completes
integrated correctness and A/B measurement without adding a benchmark framework
or changing public meaning. Their phase gates, the historical v3
drift reproduction, fixtures, raw samples, and profile results are tracked in
[Backwriter 0.2.0 Anddress fast path](../tasks/2026-08-30-backwriter-0.2.0-anddress-fast-path.md).
That task tracks progress only; the Protocol, address model, and principles own
semantics.

Phase 3 verification proves exact eight-field KATs for every kind; valid-only
public construction; canonical decimal and checked machine-range boundaries;
hash, length, kind, and range equality; strict malformed/duplicate/missing/
unknown/wrong-type handling; well-formed v3 `UnsupportedVersion`; shared private
source identity across same-source results; one-read hash/range Search; v4
human/JSON/Session round trips; and full existing capability regressions. The
exact seven-cell duplicate-Line drift matrix produces one Correct Apply, six
Safe Rejects, and zero Wrong Applies. Each reject preserves exact source bytes,
a current same-path File Anchor, and temporary/publication state. The duplicate
Paragraph regression separately preserves the same fail-closed result. Phase 6
preserves those results without an Apply/Anchor consumer indirection.

Current regressions cover SHA-256 transcript and platform-coordinate KATs,
strict v4 flat-wire encoding/decoding and error priority, zero/nonzero/maximum
machine ranges, exact source identity sharing, Search no-limit traversal, query
and scope preflight, canonical range ordering, selected-source fail-all,
NUL/invalid UTF-8 source handling, spill boundaries, no-follow symlinks and hard
links, View exact-source direct range projection and related v4 addresses,
terminator and Unicode reconstruction, and Pick stable order, multiplicity, target kinds,
complete-v4-value OneOf, and deep iterative boolean composition. Apply regressions cover Edit validation
priority, exact File/Paragraph/Line and raw-valid range splice geometry, all
line terminators, Unicode and scratch boundaries, cross-source rejection,
zero-range and byte-identical no-op publication avoidance, and late
invalid/incomplete/NUL/read/write closure. They also cover
same-parent staging cleanup, failed-publication prospective-after cleanup,
deterministic temporary-name collision preservation, logical-path independence
for hard links, Unix basic-mode preservation across changed publication,
unavailable and no-follow sources, and large whitespace Lines without
unnecessary Paragraph state.
Anchor regressions cover Runtime-local opaque handles, direct structural
projection and nonstructural-range rejection, duplicate anchoring,
drop-and-reanchor, foreign handles, stale-input preservation, one-read Apply
preparation, known-invalid-source and transient-read handling, exact
direct-target distinctions, unique post-splice target overlap across separators,
terminator absorption, collision reflection, mismatch fail-closure, and
path-exact explicit invalidation. They also cover containing Paragraph rebinding
to one remaining source Paragraph after line deletion or replacement, and
removal after a replacement splits it into two Paragraphs. They prove that a
Line outside a replaced Paragraph remains current and rebinds after its deletion,
and that terminal self-Copy rebinds a source-member Line to its joined exact
extent without an unavailable result or temporary leak. Check regressions cover source-less validation priority, raw and
native-result filtering, duplicate occurrence order and report counts, canonical
empty results, File/Paragraph/Line currentness, exact terminators, huge
source-state/range mismatches, UTF-8/NUL observations, spill and admission safety, hard-link path
independence, stateless recovery, and Anchor non-mutation. Data regressions
cover native UTF-8 names, all seven typed Store/Get pairs, duplicate input
return including the exact owned View allocation, borrowed kind/name listing,
all-kind Rename/Remove dispatch, rename and remove priority, all three
CheckOutcome payloads, and no fixed entry or name-length cap.

Exact File Search regressions cover source-less logical-path validation,
empty/nonempty regular sources without content matching, missing and directory
Empty outcomes, named admission and unadmitted paths, private spill, symlink and
hard-link boundaries, invalid UTF-8/NUL closure, one ordinary v4 File result,
Check integration, and empty-File Apply at both `StartOf` and `EndOf`.

Check streaming regressions cover chunk-boundary UTF-8 and NUL validation; late
invalid, incomplete, NUL, and read failure; forward-only access; exact
hash/length equality; batch order and multiplicity; mixed hash/kind/range
inputs; duplicate occurrences; raw-valid nonstructural ranges; and Anchor
non-mutation.
Search streaming regressions cover target-specific File, Paragraph, and Line
projection; 8,191/8,192/8,193 scratch boundaries; split multibyte UTF-8, CRLF,
and literals; Line-scoped KMP; complete matching ranges; one-read incremental
SHA-256; shared same-source identity; File `FullLine` projection stop with
continued validation; and late text/read failure discard. The Line-only slice
fast path additionally covers one-byte queries and chunks, shorter/equal/longer
Lines, `aaaaab` and `ababaca` fallback/overlap, dense first-byte no-hit input,
cross-Line rejection, chunk-ending CR and every terminator, no-EOL, Unicode,
and a multi-scratch query. Exact File structure
audits prove one common observation and no generic Line framer. Ordinary View
regressions cover File/Paragraph/Line exact bytes, terminators, Unicode,
8,191/8,192/8,193 scratch boundaries, raw-valid nonstructural ranges, UTF-8
scalar-cut failure, every source-state mutation class, exact-state
reappearance, and late failure/resource discard. Its minimal Line relation
state preserves the optional Paragraph only for an exact current text Line.
Check regressions prove exact hash/length-only comparison while preserving
unsorted mixed geometry and duplicates. Anchor streaming regressions cover
direct File/Paragraph/Line projection without a View outcome, raw-valid
nonstructural rejection, selected anchored-binding success beside a stale
sibling, and exact-path fail-close on the selected mismatch. Anchored View uses
the ordinary direct target capture rather than a second parser.

Apply streaming regressions cover one retained no-follow source observation,
fixed-scratch staging readback, direct public v4 range geometry, incremental
prospective-after UTF-8/NUL/hash/length validation, one rename, and absence of
whole-source/after materialization. They cover 8,191/8,192/8,193 UTF-8 and
CR/LF/CRLF/no-EOL boundaries; exact File/Paragraph/Line and raw ranges; every
Position and forward/reverse Copy/Move direction; strict interior Move
rejection; zero-range, empty, boundary, and byte-identical no-ops; the seven
exact no-drift/edit-before/edit-after/adjacent/changed/equal-range/deleted Line
cells and duplicate Paragraph drift; late source read and staging write failure; temporary
collision/cleanup; Unix basic mode; and uncertain rename. Source-target
containment and overlap use only direct before ranges. The after projector
retains exact same-kind candidates and minimal original/copied/replacement
provenance, preserving unique rebind while removing split, join, absorption,
ambiguity, and collision cases. Copy follows only the original occurrence and
Position contributes geometry, never provenance.

Structural audits keep the forward observer, SHA-256, checked length, and text
policy in `source_scan.rs`; require one Apply source observation and staging as
the only readback input; and exclude `Natural`, `DecimalOrdinal`, private Apply
Anddress/Edit/Position, `LegacyResolver`, `ExactTargetTracker`, `SourceEvent`,
`SourceFramer`, `scan_source`, target extraction, source reopen/retry,
persistent observation, and source-sized complete before/after buffers from
production. Successful Anchor reflection remains allocation-free and
non-failing after all fallible planning completes.

Phase 7 benchmark verification exports v3
`399805906b352f1c8d0cc2fa0bbe6dee1a73a13c` and v4
`55999768cc7ad75ea84a08d597dbc7a7913fe6c3` directly from Git objects, builds
separate offline/locked stripped release and profiler binaries, and recreates
the exact Phase 2 fixture hashes under the fixed tmpfs task root. Each cell has
one warm-up and seven fresh processes or Sessions pinned to CPU 8. Exact
semantic output/order/count/final-source checks, deterministic output hashes,
empty stderr, wall/user/system time, HWM/RSS, `rchar`/`wchar`, throughput, and
million-hit bytes/HWM per result are required. Representative `perf stat` and
DWARF stack runs are profile evidence only and must have zero lost samples.

Formal gates require large View/Check and range Apply median CPU at most 75% of
v3, Search median wall and peak HWM at most 105%, every p95 and peak-memory
comparison below a 10% regression, bounded low-hit source memory, and no
consumer target search, relocation, second Search hash pass, whole-source
`CurrentObservation`, fixed truncation, or output/error drift. Those gates pass.
The Owner correction defines the million-hit recommendation as result-memory
peak-HWM slope, not JSON output size. Phase 7's 346.539 to 58.551 HWM bytes/hit
is an 83.10% reduction and passes the 50% recommendation; 327.470 JSON bytes/hit
is canonical Adapter payload/I/O evidence only. The remaining 2× 256 MiB Line
Search recommendation is closed by a Line-only maximal content-slice fast path.
Against the paired v3 673.898 ms median, its 272.111 ms median is a 2.476×
speedup and below the 336.949 ms target. Fixed current/candidate 128 MiB,
256 MiB, 2,048-file, and million-hit measurements preserve exact output and
all p95/peak-HWM gates. These gates supplied the source-readiness evidence for
the separately executed and now closed `0.2.0` publication.

Edit V1 semantic/public API/type/error authority and inert Rust value
implementation are complete; the single-source Apply Runtime execution
and its regressions are complete.
Value regressions cover every operation and position target-kind boundary,
source-less Anddress error mapping and field priority, exact empty/Unicode/CR/
LF/CRLF content, NUL rejection, and absence of relation or fixed-size
constraints. They use no filesystem or Runtime. Runtime regressions cover every
Edit operation and exact position boundary; same-coordinate/path validation;
source-state drift; strict Move interior rejection; raw and zero ranges; late
UTF-8/NUL/read/write failure; staging cleanup; and exact source/Anchor
preservation for direct no-ops. Changed operations compare fixed staging chunks
against generated output, validate the result incrementally, prepare direct
Line/Paragraph candidates, and publish only after the full Anchor plan exists.
They also cover source-target-only replacement and Move provenance, Copy's
original-occurrence rule, split removal, unique replacement rebinding,
Position-only geometry, no-EOL and terminator behavior, and reverse Move for
source-contained Line and Paragraph bindings.
Edit still adds no Data
payload, wire form, or distinct anchored executor/publication path.

Run after Rust or Runtime behavior changes:

    cargo metadata --offline --locked --format-version 1
    cargo tree --offline --locked
    cargo fmt --all -- --check
    cargo check --offline --locked --all-targets
    cargo test --offline --locked
    cargo clippy --offline --locked --all-targets -- -D warnings
    cargo build --offline --locked --release

Linux x86_64 release-target verification uses the explicitly installed canonical
`x86_64-unknown-linux-musl` target. The GNU target remains the local development
and test-host target; `rust-toolchain.toml` does not auto-install musl for every
checkout. Run:

    rustup target add x86_64-unknown-linux-musl --toolchain 1.95.0
    rustc +1.95.0 --print cfg --target x86_64-unknown-linux-musl
    cargo check --offline --locked --all-targets --target x86_64-unknown-linux-musl
    cargo build --offline --locked --release --target x86_64-unknown-linux-musl
    cargo test --offline --locked --target x86_64-unknown-linux-musl

The release binary is `target/x86_64-unknown-linux-musl/release/bw`. These commands verify
target selection, build, test, and host execution; running them alone does not
publish a distribution.

Separately, the external operations-owned distribution at
[https://backwriter.pentagration.com](https://backwriter.pentagration.com) has
completed publication verification for Backwriter `0.1.0-beta.3` on
Linux/WSL x86_64, macOS arm64, macOS x86_64, and Windows x86_64. Targets are
`x86_64-unknown-linux-musl`, `aarch64-apple-darwin` at minimum macOS 11.0, and
`x86_64-apple-darwin` at minimum macOS 10.12, and
`x86_64-pc-windows-gnu`. The artifacts and manifest retain
Source Authority revision `7d7469563a357215261c42fa2067d7f587c5eb1b`, and the
POSIX installer destination is `$HOME/.local/bin/bw`; the PowerShell destination
is `$HOME\.local\bin\bw.exe`. The CMD path is the public CRLF Adapter that
downloads and delegates to the same PowerShell installer. Verification covered
the archive, manual
`.sha256` sidecars, expanded canonical manifest, installers, closed version
directory, local and public GET/HEAD status, zero-length HEAD bodies, exact
cache policy, canonical body equality, manifest-authoritative artifact SHA-256,
and task-local fresh installation plus explicit `bw update`. Fresh installation
printed the installed version, replacement printed the updated version, and
destination/PATH guidance remained separate. `bw version` produced exactly
`Backwriter 0.1.0-beta.3` plus LF. Verification passed 188 GNU tests, 188 musl tests, 13
origin tests, 35 POSIX installer regressions, 36 PowerShell regressions, 12 CMD
regressions, 10 Windows release regressions, and 18 publisher regressions. It
also verified 12 local and 12 public GET/HEAD responses with exact bodies and
cache policy. Host verification confirmed enabled and active
`backwriter-origin.service` and official `cloudflared.service` processes with
zero restarts, one `127.0.0.1:8080` listener, byte-identical tracked and
installed ingress YAML, and a root-only Git-external tunnel credential. No
token or credential value is present in Git, unit arguments, service
environment, or service journal. macOS and Windows verification is static and
does not claim native execution. The beta.1 and beta.2 files and complete
beta.3 version directory are immutable. The planned matrix is complete and
beta.3 is closed. Linux arm64, tags, GitHub Releases,
crates.io publication, universal host compatibility, background or automatic
update, and GitHub distribution authority remain outside this verification.

Stable `0.1.0` publication verification regenerated the four artifacts and
sidecars from source revision `25a0dbc38dc78cc7592b219e9070af3c0e201c17`
and reproduced the canonical 876-byte manifest with SHA-256
`551ee8b6fc4c5df83421ba7244f191fee8cc70287775088f08f5e1b8e2290570`.
The tracked publisher installed the stable eight-file version directory,
replaced the POSIX and PowerShell pointers, reused the CMD Adapter, and replaced
the manifest last; a complete rerun reused the resulting exact 20-file tree.
All 20 public GET and HEAD endpoints returned exact bodies, lengths, and cache
policy; root and unknown paths remained 404/no-store. Task-local fresh install
and an actual beta.3 binary's explicit update both installed byte-identical
stable Linux binaries and printed `Installed Backwriter: 0.1.0` and `Updated
Backwriter: 0.1.0`. The published binary passed help, exact version, Search,
Session, and empty-File StartOf/EndOf Apply verification. Stable closure also
passed 193 Backwriter tests, 13 Origin tests, 32 installer regressions, 16
stable-publisher regressions, and 12 CMD regressions. Origin and cloudflared
process identity, restart counts, loopback listener, tunnel connector, ingress
YAML, DNS, credential metadata, and actual user HOME/PATH/shell files remained
unchanged. macOS and Windows verification remains static and makes no native
execution claim.

Backwriter `0.2.0` release closure regenerated the four artifacts and sidecars
from source revision `2fad6e46d3a9d1da01f79f34b9ffc187447c76a8` and
reproduced the canonical 876-byte manifest with SHA-256
`b63589acd1c06606e62f08ea83dd1c2c36fbc5987665287218481f74b06a5cd4`.
The existing publisher added the eight `releases/0.2.0` files, atomically
replaced the POSIX and PowerShell installers, and replaced the manifest last.
It preserved metadata and bytes for all 16 beta.3/stable versioned files and
`install.cmd`; a complete rerun preserved metadata for all 28 files. All 28
files passed both loopback and public HTTPS GET and HEAD checks, for 56 GET and
56 HEAD responses with exact bodies, lengths, content types, and cache policy.
Root and unknown-path GET/HEAD checks remained 404/no-store. A task-local
canonical `curl | sh` fresh install and an actual public `0.1.0` binary's
explicit update installed byte-identical `0.2.0` Linux binaries. The installed
binary passed help, exact version, Search, View, Check, Session, empty-File
Apply, and duplicate-drift safe-rejection checks. Origin and cloudflared PID,
InvocationID, restart count, loopback listener, units, ingress YAML, credential
metadata, actual user HOME, and process PATH remained unchanged. macOS and
Windows artifacts received static cross-build verification only; no native
macOS, Windows, PowerShell, or CMD execution is claimed. No tag, GitHub Release,
crates.io publication, cache purge, service, tunnel, DNS, route, or credential
change occurred.

Before staging, verify the diff and empty index, confirm repository-root
`.artext` is absent and untracked, and preserve unrelated task/history files.
Owner-authorized work then stages only the reviewed paths and repeats the
cached diff audit before commit.

The repository source package and closed public release are `0.2.0`, and its
release build must print exactly `Backwriter 0.2.0` plus LF. Source verification remains
distinct from the separately executed operations publication: the current official
distribution is the closed `0.2.0` release, while `0.1.0` and prior beta files
remain immutable. The current `0.2.0` source suite passes 203 GNU-host Rust tests; the
historical `0.1.0` source suite passed 193.

CLI process regressions cover the canonical `bw` binary without a `backwriter`
binary, `--help`, exact `bw version`, explicit `bw update` download/exit/output
propagation and platform handoff, default-current-directory and explicit absolute workspaces,
default and repeated admission, Line/Paragraph/File Search, repeated source and
subtree scope selectors, Core scope rejection, deterministic human output,
space-preserving query argv, raw-Anddress/workspace-coordinate omission, Empty,
usage versus Runtime execution exits, unsupported deferred forms, and strict
stdout/stderr separation. They also cover one-shot human/JSON and Session
`search /file`, exact empty-File retrieval, missing/directory Empty, invalid and
unadmitted paths, existing writer reuse, Check, and end-to-end empty-File Apply.
View regressions cover v4 decode, File/Paragraph/Line exact bytes,
None/LF/CR/CRLF terminators, large no-EOL output, stale source-state/range and
unadmitted source closure, plus one-shot anchored/extra-operand
rejection.
One-shot Search JSON regressions cover exact compact envelope key order, Empty
and Found mapping, exact v4 object embedding and re-decoding, File/Paragraph/
Line targets, CR/LF/CRLF/no-EOL byte ranges, Unicode and JSON escaping, result
order, repeated Line content, global-option placement, rejected duplicate/late
or non-Search JSON, and a structural audit that excludes a JSON Value or cloned
result collection in the production writer.
One-shot View JSON regressions cover exact compact envelope key order for File,
Paragraph, and Line; related v4 File/Paragraph object re-decoding; every Line
terminator including a separator Line's `paragraph:null`; Unicode and JSON
escaping; unchanged human projection; rejected duplicate/late/anchored/extra
forms; unavailable stdout/stderr closure; and a structural audit that excludes
a JSON Value, ViewOutcome clone, or result collection in the production writer.
One-shot Check JSON regressions cover exact compact Current, NotCurrent, and
Unavailable envelopes; direct v4 filtered-value re-decoding; File/Paragraph/Line
inputs; missing-source NotCurrent versus invalid-source Unavailable; unchanged
human statuses; rejected duplicate/late/search/pick/extra/invalid forms; and
structural audits for shared fail-closed status classification, no JSON Value,
no CheckOutcome clone or collection in the JSON writer, and removed display-only
CheckOutcome clones at Session/Data writer callsites.
One-shot raw View regressions cover exact default-human equality for File,
Paragraph, Line, Unicode, every terminator, and large no-EOL output; admitted
global-option order; stale/invalid closure; rejected duplicate/mixed/late flags
and non-View raw forms; and structural absence of a raw writer or the retired
global JSON bool.
Check regressions cover File/Paragraph/Line Current status, stale and missing
NotCurrent status, unavailable-source status, strict v4 decoding, and rejected
search/pick/extra forms. They create no CLI Session, binding, JSON, raw, or
other capability authority. Session regressions cover one retained Runtime,
Search and Pick bindings with exact indexed address projection, direct Search and
Pick non-retention, Core Pick target-kind, same-file, OneOf, and iterative boolean
composition, batch Search/Pick Check report counts, mixed current/NotCurrent/
Unavailable outcomes, empty outcomes, unchanged bindings after batch Check,
copied Search/Anddress bindings, non-aliasing Anchor handle creation,
`AlreadyLive` without a new binding, File/Paragraph/Line anchored View,
source-specific invalidation, rejected handle cloning/indexing/type misuse,
lexer quoting and errors, blank Lines, EOF and exit, error continuation and exit
precedence, and absence of latest, pipeline, registry, or persistence. Session
Data regressions cover all seven typed Core kinds, direct and `let` Get human
projection, duplicate and cross-kind names, Rename/Remove/List order and safe
name escaping, wrong-kind and unsupported-value rejection, mutation failure
preservation, and DataStore drop at Session end.
Session result-binding regressions cover exact View/Check output before storage,
cloneable result values, anchored View, raw and batch Check reports, and rejected
cross-capability use without implicit filtered-value conversion.
Session lexer regressions cover `\\`, `\"`, `\n`, `\r`, and `\t` for non-Edit
commands plus rejected malformed escapes and quotes. Session View/Check
regressions preserve direct, anchored, binding, and Data Get bytes/counts while
displaying through borrowed outcomes. Session Edit/Apply regressions cover all
five Edit variants, all four Position forms, exact source bytes, explicit Edit
binding cloning and repeated Apply reuse, invalid forms, and continued execution
after errors without CLI recovery.
