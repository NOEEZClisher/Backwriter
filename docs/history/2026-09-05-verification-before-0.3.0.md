# Verification evidence preserved before 0.3.0

Historical evidence only, not current semantic or release authority. These excerpts
retain their original wording, version, paths, measurements and limitations.
They were taken from `docs/development/verification.md` at
`3d35f14338d2374777acd485d0bce49387800fbc` (1,487 lines, 96,993 bytes,
SHA-256 `7bd2851b1add8756590ab4f0888b2d6e4c0e5f78eaacaffd7afd0a60c2bb9666`).

Only evidence units with additional detail not retained by the existing trackers
are copied here; surrounding sentences retain the conditions needed to interpret
them. Repeated gate narratives and existing measurements remain in the
[historical evidence index](index.md). Old v3/v4 descriptions below are not
current v5 contracts. Original line ranges identify each verbatim excerpt;
Gate 4 verifies each excerpt against the pinned original bytes.

## Original lines 155–161

- Gate 7: complete GNU/musl semantics and fixed A/B/G performance, memory, I/O,
  output, drift, and code-size evidence; source-readiness is GO.
- Gate 8: artifacts, installers, manifest-last publication, endpoints, update,
  idempotent reuse, and release closure — complete. At that closure the live root had 68
  regular files; loopback and public HTTPS each passed 68 exact GET/HEAD body,
  length, MIME, cache-policy, and zero-HEAD-body checks. Isolated fresh install,
  public `0.2.4` update, and `0.2.5` reinstall selected the exact Linux member.

## Original lines 519–537

None/LF/CR/CRLF one-shot Edit preserves each exact terminator and gives
byte-identical V5/B receipts. CRLF A/B median/p95 is 1.187/2.533 ms versus
1.798/2.771 ms, maxRSS is 11,036 versus 11,040 KiB, and final SHA-256 is
`cc326fa86d3e5924c488283058e530b9413d6acec0f4f78a954882f85f92edbf`.
The raw Session Replace/Apply control is byte-identical across A/V5/B and has
the same final SHA; A/B `rchar` is 6,632 and output SHA-256 is
`fa25fff17c951881bb024d75d2129bcf8681c13cc54a113cafb5e7c4f74c37e8`.
Edit runner/evidence SHA-256 values are
`dbff910d4fa4159bd2b1b56834b44bcc7549028d4e0a3c09d5ad427569c80645`
and `1c564949ff842ca36ddc6a447c3dd3c596bf5b3d9a498e0fdc0c87128980ac7c`.
The original input SHA-256 values are
`7c9bc58081262feba4a5609d4c9f0ae1353edd0d31ae48f92d6a68b1089fe090`
for None,
`d35fe0ba542f0e6402a4b323b465c7a13484d702f698adad65945e39d0c50c6f`
for LF,
`54a2d1515d7157eca2f57a655558197db1223ee863a46c681b41c0d02e7d3234`
for CR, and
`ec43c761c4cd9b113208d4f99b2e912697e414e739e4c60324f4b5ae2c72a3fb`
for CRLF.

## Original lines 592–611

- `SearchOutcome::Found { occurrences: Vec<SearchOccurrence> }` owns one exact
  v4 Anddress and target-coherent optional `SearchPosition` per result. Public
  validation covers File/None, nonzero Line, ordered nonzero Paragraph bounds,
  Clone/Eq, borrowing, and ownership transfer.
- Runtime increments checked one-based Line state inside the existing Line and
  Paragraph framing projections. CR, LF, CRLF, bare CR, no-EOL, empty and
  separator Lines, Unicode, no synthetic EOF Line, and 8,191/8,192/8,193-byte
  scratch boundaries are exercised. No additional source open, read, hash pass,
  whole-source retention, observation object, or parallel result vector exists.
- Check filters complete occurrences while keeping report evidence as raw
  Anddresses. Data and Session store the carrier; Session indexing extracts the
  contained Anddress; Pick alone receives an explicit raw-Anddress collection
  and retains its established outcome and byte-range display.
- One-shot and Session human Search share `write_search`: File is path-only,
  Line is `path:line`, and Paragraph is `path:start-end`. The streaming JSON
  writer emits exact `bw.cli.search.v2` occurrence items and directly embeds
  `Anddress::encode()` output without a JSON value tree, result clone, or second
  collection. Empty/Found, all target kinds, position fields, key order,
  duplicate/order retention, escaping, large results, and Search-to-Edit v4
  extraction are byte-exact regressions.

## Original lines 632–645

The direct one-forward-observation path, matching Host trusted range path, and
anchored path produce equal results. Scalar-aligned raw ranges exercise every
allowed projection through direct/trusted parity. CR, LF, CRLF, bare CR,
no-EOL, Unicode, separator relations, and 8,191/8,192/8,193-byte boundaries
cover exact upward Content. Existing stale/hash/length/range, workspace/path,
admission, no-follow, UTF-8/NUL, resource, proof invalidation, and Anchor
fail-closure controls remain. CLI one-shot/Session/anchored View and one-shot
Edit pass self projection, while Data retains the extended `ViewOutcome`; all
existing human/raw/JSON bytes and grammar remain unchanged. Structural checks
retain one `finish_outcome`, no `view_projected` facade, no request DTO, and no
second View parser or executor.

The complete offline/locked GNU-host suite now passes 247 tests: the 245 Gate 2
controls plus two public Gate 3 projection and pre-I/O validation regressions.

## Original lines 647–666

Gate 4 adds the public ordered batch seam without changing single View,
anchored View, or Adapter execution. Empty, single, duplicate, mixed A/B/A,
same-source distinct and overlapping ranges, all six allowed projections, the
three downward relations, `RelationAbsent`, Unicode, separators, every Line
terminator, no-EOL, raw ranges, and the 8,191/8,192/8,193-byte boundaries are
covered. Returned outcomes preserve exact input order and multiplicity.

Every source-less input and requested relation is validated in input order
before complete coordinate, spill, and admission preflight or source I/O.
Stale state, foreign coordinate, spill, unadmitted and missing paths, symlinks,
invalid UTF-8, NUL, late read failure, and Resource failure return no partial
vector. Structural regression fixes one source-key group pass, one open and
one direct observer per group, no public single-View loop, and fallible
allocation for indices, captures, provisional slots, and final output.

Host regressions cover A/B/A proof groups, trusted/direct output parity,
proof-miss fallback, mismatch before I/O with proof preservation, one matching
group handle, and existing source/resource invalidation. The complete
offline/locked GNU-host suite passes 253 tests: the 247 Gate 3 controls plus
two private observer/all-or-nothing tests and four public batch regressions.

## Original lines 740–749

One untimed warm-up per variant precedes five order-crossed `AB`, `BA`, `AB`,
`BA`, `AB` samples. The monotonic interval spans the first `bw` spawn through
the final source read and excludes fixture reset. A median/p95-nearest-rank is
`25.168625`/`25.812745` ms; B is `19.793078`/`20.982414` ms. These elapsed
samples are diagnostic evidence, not a performance gate or general speed
claim. The task-local driver SHA-256 is
`18ec103d2815f52957d29e3be986f3b9e8027d3442a14bd252f60145033e410d` and its
raw JSON evidence SHA-256 is
`8c48cb6192621f0d7c92a6be76d432beb2c11bd134c00be008349ca01dd8243a`;
both are removed after verification.

## Original lines 908–915

All 44 loopback and public HTTPS files pass GET and HEAD with exact bodies,
SHA-256, lengths, content type, zero HEAD downloads, and cache policy. Root and
unknown-path GET/HEAD remain 404/no-store. A task-local canonical `curl | sh`
fresh install and an actual public `0.2.1` binary's explicit update install
byte-identical `0.2.2` Linux binaries and print the exact Installed and Updated
outcomes. The installed binary passes Help, Version, JSON Search-to-exact-v4
one-shot Edit with CRLF preservation, View, Check, raw Session Apply, stale
reuse, and duplicate-drift Safe Reject probes.

## Original lines 1059–1080

The exact Phase 7 256 MiB fixture
`d4edd123621cf230590d7812e64bec69460789eba3e0c7136b88a3f26c88f5e5`
is measured against baseline `0f1cc6b` and the candidate on CPU 2 P-core with
`powersave`, one warm-up, and seven order-crossed samples. Host
Search-to-late-Line View moves from `1,035.274` / `1,041.995` ms median/p95 to
`331.527` / `332.547` ms and therefore passes both 400 ms and recommended 350
ms gates. Baseline and candidate stdout SHA-256 are both
`1813f9bf4a219f21de1c4a539a5057676ed37835cc405b78501ea894566212b3`;
both record HWM below 2.7 MiB, exact `536,870,913` `rchar`, and zero residual
`wchar`. Search-only stays `270.734` versus `270.534` ms; Host Check remains
zero-I/O; the one-million-result candidate peaks at 60,080 KiB, 58.672
bytes/hit and below the 61.4383 bound. Separator, forward, File/Paragraph,
ordinary/anchored, and Untrusted controls have byte-identical outputs and no
unexplained material regression. Apply code is untouched and the full suite
retains the existing one-Correct/six-Safe-Reject/zero-Wrong matrix. This closes
the performance gate only. Phase 7B reuses the fixed large fixtures and one
task-local A/BU/BH source across 17 cells. BU/BH 256 MiB Search medians are
`267.397`/`267.273` ms, Host Search-to-late-Line View is `324.254` ms, Host
Check proof hit is exact zero I/O, and one-million-hit HWM is `58.609`
bytes/hit. All cell payloads agree and the drift regression retains one
Correct, six Safe Rejects, and zero Wrong Applies. Source version is `0.2.1`;
artifact and publication authority is unchanged.

## Original lines 1116–1210

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

## Original lines 1239–1257

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

## Original lines 1284–1386

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

Backwriter `0.2.1` release closure regenerated the four artifacts and sidecars
from source revision `4a1b06fb375bfd906a6f27de4de15a8febfe08ec` and
reproduced the canonical 876-byte manifest with SHA-256
`04b111122f844bee17d40f68358386e6e64112ef9e3c2e7ef7547439586afc46`.
The existing publisher added the eight `releases/0.2.1` files, atomically
replaced the POSIX and PowerShell installers, and replaced the manifest last.
It preserved metadata and bytes for all 24 beta.3/stable/`0.2.0` versioned
files and `install.cmd`; a complete rerun preserved metadata for all 36 files.

All 36 files passed both loopback and public HTTPS GET and HEAD checks with
exact bodies, lengths, content types, zero HEAD bodies, and cache policy. Root
and unknown-path GET/HEAD checks remained 404/no-store. A task-local canonical
`curl | sh` fresh install and an actual public `0.2.0` binary's explicit update
installed byte-identical `0.2.1` Linux binaries and printed exact Installed and
Updated outcomes. The installed binary passed help, version, Search, View,
Check, Session, empty-File Apply, and duplicate-drift safe-rejection probes.

Closure passed 236 GNU-host tests, 236 musl tests, 13 Origin tests, 35 installer
regressions, 52 publisher regressions, 12 CMD regressions, and offline/locked
metadata, tree, formatting, all-target checking, clippy with warnings denied,
and release builds. Origin and cloudflared PID, InvocationID, restart count,
listener, units, ingress YAML, credential metadata, actual user HOME, process
PATH, and shell startup files remained unchanged. macOS and Windows artifacts
received static cross-build verification only; no native macOS, Windows,
PowerShell, or CMD execution is claimed. No tag, GitHub Release, crates.io
publication, cache purge, service, tunnel, DNS, route, or credential change
occurred.

## Original lines 1404–1471

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
One-shot Search JSON regressions cover the exact compact v2 envelope and item
key order, Empty and Found mapping, exact v4 object embedding and re-decoding,
File/Paragraph/Line position shapes, CR/LF/CRLF/bare-CR/no-EOL framing, Unicode
and JSON escaping, result order, repeated Line content, global-option placement,
rejected duplicate/late or non-Search JSON, and a structural audit that excludes
a JSON Value or cloned result collection in the production writer.
The Search-to-Edit integration control removes only the fixed single-occurrence
v2 wrapper, validates the original embedded v4 bytes, passes them unchanged to
one-shot Edit, and verifies exact CRLF-preserving source output.
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
