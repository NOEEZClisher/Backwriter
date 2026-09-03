# Verification

## 0.2.3 Patch Box Gates 1–8

Gate 1 records the direct Search, View, Edit, Check, Data, Session, Pick, and
writer consumers. Gate 2 implements only the Search observation carrier and
its two Adapter projections:

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

Late selected-source failure continues to discard every provisional occurrence
and position. Existing literal tiers, ordering, multiplicity, v4 KAT,
currentness, and single-observation controls remain unchanged. Production has
no v1 writer or compatibility branch; `bw.cli.search.v1` remains only immutable
published `0.2.2` evidence. Cargo, CLI version, README, toolchain, server, the
official 44-file public tree, and services remain unchanged.

The complete offline/locked GNU-host suite passes 245 tests: the 243 inherited
controls plus one Runtime position/framing boundary test and one public
occurrence validation/ownership test.

Gate 3 changes the native single View seams to accept one existing
`AnddressTarget` projection. Public regressions cover all six allowed
Line-to-Line/Paragraph/File, Paragraph-to-Paragraph/File, and File-to-File
relations; projected v4 kind/range/hash/length and exact Content; related File
and optional Paragraph addresses; and `RelationAbsent` for separator and
raw-valid nonstructural Line-to-Paragraph requests. File-to-Paragraph/Line and
Paragraph-to-Line return `InvalidInput` before a missing source can be opened.

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

Gate 5 adds `EditReceipt::{Unchanged, Changed}` and the Replace-only
`WorkspaceRuntime::apply_replace` seam while keeping the unit-returning public
`apply` and raw Session unchanged. Structural regression fixes one internal
executor, one admitted source open and observation, one prospective-after
source identity, receipt preparation before the existing Anchor plan and Host
proof, and successful receipt return only after confirmed publication. It
excludes Search, source reopen, second observation, request DTO, parallel
executor, binding, Data kind, JSON, and stdin.

Changed File coverage checks exact prospective hash, length, full range,
immediate View and Check, old-address rejection, and a following Replace with
the returned address. Line coverage checks exact None/LF/CR/CRLF results,
Unicode, empty body, empty no-EOL output, and current zero/nonzero ranges.
Paragraph coverage fixes zero, one, and multiple resulting Paragraphs as
`Changed(None)`, `Changed(Some)`, and `Changed(None)` without restricting
Content. Direct zero-range and assembled byte-identical no-op return the exact
input while preserving bytes, inode, Host proof, and live Anchor. Receipt and
Anchor regressions compare the same installed after source identity.

Non-Replace input rejects before filesystem access. Existing stale, invalid,
unadmitted, open/read/resource, staging collision, rename uncertainty, cleanup,
proof invalidation, and Anchor fail-closure tests continue through the shared
executor and return no successful receipt on error. Existing raw Apply covers
all five operations and four positions. The Gate 5 baseline had 255 tests: the
253 Gate 4 controls plus two public receipt regressions.

Gate 6 removes only the one-shot Adapter's discarded-receipt and shared `OK`
writer call. One direct writer emits exact human `Unchanged`/`Changed` rows or
the fixed-order `bw.cli.edit.v1` object. It calls `Anddress::encode()` once
before creating the stdout writer and directly reuses those bytes; structural
checks reject a JSON `Value`, reserialization, clone, result collection,
post-Apply Search, reopen, Check, stdin reader, parallel writer, or second
schema. Raw Session Apply continues to use the independent status writer and
emits exact `OK` plus LF.

CLI regressions cover human and JSON `Unchanged`, changed File/Line/unique
Paragraph addresses, and changed zero/multiple-Paragraph `None`/`null`; exact
schema/key order/final LF; embedded canonical-v4 decode and byte equality;
every Line terminator, empty and Unicode Content; and a Search-v2 object passed
unchanged to Edit followed by View and another Edit using only the fresh
receipt address. Leading `--json`, rejected `--raw`, duplicate output choice,
extra operands, and literal positional `--json`, `--raw`, and `--stdin` are
covered. Existing stale, missing, unadmitted, Runtime resource/read, staging,
rename-uncertain, and publication failure controls plus writer-after-Apply
ordering establish zero success output and no receipt on Apply failure. A
Linux `/dev/full` control proves a post-publication flush failure exits `1`
while leaving the confirmed source publication intact and without retry.

Argv remains the only Content transport. Direct empty/Unicode,
File/Paragraph-newline, and Line-body coverage exists; argument length,
shell/newline behavior, and process-list/history exposure supply no reproduced
consumer failure, measured payload need, or concrete security requirement.
Gate 6 therefore adds no `--stdin` grammar, reader, EOF state, generic content
source, file transport, or placeholder. The complete offline/locked GNU-host
suite passes 256 tests: the 255 Gate 5 controls plus one CLI stream-failure
regression.

Gate 7 builds the published `0.2.2` Source Authority
`04b36d9ca9cc725bedeb17231339c67b5f0590ea` and the integrated Patch Box parent
`d3e2b2e65112e9f0f018cd29050652928e4ef412` from clean Git-object exports. On
the exact `retry_budget = 3\r\n` fixture, A performs Search/Edit/Search/View/Edit
and B performs Search/JSON Edit/View/JSON Edit. Both finish with exact bytes
`retry_budget = 7\r\n` and SHA-256
`798ba02ce45d505e56b0112210695a52931a40797aa9eb6f68d608d9c9b6173e`.
A/B process and Adapter-command counts are `5/4`, Search `2/1`, repeated
post-Edit Search `1/0`, JSON array indexing `2/1`, explicit View `1/1`, Edit
internal View `2/2`, Apply `2/2`, and total Runtime capability calls `7/6`.
Caller-visible raw Apply, mandatory Check, Wrong Apply, history, relocation,
retry, and newline mistakes are all `0` in both flows. B uses each fresh
receipt address directly for the following View and Edit without post-Edit
Search.

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

Before and after the source-version change, offline/locked GNU and musl runs
each pass the complete 256-test suite, all-target check, and release build.
GNU also passes formatting and clippy with warnings denied. Release Help,
Version, JSON Search-to-receipt-to-fresh-View/Edit, representative raw Session,
v4 KAT/no-v3, Search order and duplicates, single/batch View all-or-none and
one-observation behavior, all receipt states and writer failure, raw Apply,
Host proof hit/miss/mismatch/invalidation, Anchor reflection/fail-closure, and
the duplicate Line/Paragraph drift matrices remain green. Gate 7 therefore
advances only the root package, root lock entry, version KAT, and active status
documents to source-ready, unpublished `0.2.3`; production `src/**`, Core,
Runtime, v4 wire, toolchain, and dependencies remain byte-identical to the
Gate 6 parent. Official artifacts, installers, manifest, public root, service,
and `bw update` remain closed `0.2.2` until Gate 8.

Gate 8 reconstructs the four canonical `0.2.3` artifacts from Source Authority
revision `195aaa37068122097ecc04d2644642b6afcc6765`, their sidecars, and the
exact 876-byte manifest with SHA-256
`876ce79316663faa06cbcd2d374edcc7874e8374b1838f823e52fc623466ea73`.
Before live publication, GNU and musl each pass all 256 Rust tests and their
offline/locked all-target check and release build; GNU also passes formatting
and clippy with warnings denied. The installer, `0.2.3` publisher, CMD, and
Origin suites pass 38, 55, 12, and 13 regressions respectively.

The exact publisher adds only the eight `releases/0.2.3` files, then replaces
`install.sh`, `install.ps1`, and the manifest last. All 40 earlier versioned
files and `install.cmd` retain bytes, inode, mode, owner, size, and timestamps.
A second publisher execution reuses all 52 files without metadata change.
Every one of the 52 files passes loopback and public HTTPS GET and HEAD checks
for exact body or empty HEAD body, length, media type, and cache policy; root
and unknown paths remain empty no-store 404 responses.

An isolated public fresh install prints exact `Installed Backwriter: 0.2.3`.
The public `0.2.2` binary Update and a `0.2.3` reinstall print exact `Updated
Backwriter: 0.2.3`; the installed binary equals the canonical Linux archive
member. Installed Help, Version, Search v2 position, original embedded v4
object to JSON Edit receipt to fresh View and next Edit with zero post-Edit
Search, CRLF preservation, old-address no-publication rejection, Check, raw
Session Apply, and duplicate-drift Safe Reject probes pass. The actual user
HOME, PATH, and shell startup files remain unchanged.

Origin and cloudflared PID, InvocationID, restart count, the loopback listener,
unit and ingress YAML bytes, credential metadata, tunnel UUID and connector,
and DNS remain unchanged. macOS and Windows receive static cross-build and
archive verification only; no native macOS, Windows, PowerShell, or CMD
execution is claimed. No tag, GitHub Release, crates.io publication, cache
purge, service, tunnel, DNS, route, or credential mutation closes this release.

## 0.2.2 Anddress-first editing Gates 1–6

Gate 1 closes authority and Gate 2 implements only the one-shot Adapter
composition tracked in the
[0.2.2 tracker](../tasks/2026-09-01-backwriter-0.2.2-anddress-first-editing.md).
CLI regressions prove exact File and Paragraph replacement; Line body
replacement with None, LF, CR, and CRLF preservation; empty and Unicode
Content; CR/LF Line rejection before Apply; strict v4 decode; stale, missing,
and unadmitted source rejection; byte-identical no-op and Unix inode
preservation; exact `OK` plus LF success; stderr-only exit `1`/`2` failures;
literal `--json`/`--raw` File, Paragraph, and terminator-preserving Line Content;
and leading global output-option plus trailing extra-operand rejection without
source mutation.

Structural evidence fixes one ordinary Runtime, one private View, only
`Edit::Replace`, the original decoded Anddress as Apply target, existing Edit
validation, Runtime Apply, and the existing status writer in that order. It
excludes Search, Check, a new Runtime seam, engine, state machine, retained
observation, relocation, retry, fallback, v4 schema, error alias, or
compatibility layer. Existing Apply/Anchor regressions continue to own
View-to-Apply mutation, uncertain publication, Anchor reflection, Host proof,
and invalidation evidence. Raw Core Edit/Position/Apply, Session Edit and Apply,
Cargo `0.2.1`, `Backwriter 0.2.1`, Core, Runtime, and the public distribution
remain unchanged controls. The complete offline/locked GNU-host suite passes
242 tests: 236 existing controls plus six Gate 2 CLI regressions.

Gate 3's source and consumer audit closes without an additional Content
transport, machine-output schema, parser, writer, type, dependency, or Runtime
seam. Existing argv covers accepted Content, Search JSON carries exact v4
Anddress objects, and the existing status/error path distinguishes success,
usage, and execution outcomes without claiming that exit `1` preserves source
bytes or permits retry. Documented residual constraints are OS argument limits,
shell quoting/newline portability, and process-list/history exposure.

Gate 4 uses one task-local workspace whose sole source is the exact Line
`retry_budget = 3` plus CRLF. JSON Search returned one exact 311-byte v4 object;
passing those object bytes unchanged to one-shot Edit with body
`retry_budget = 5` exited `0`, wrote exactly `OK` plus LF, preserved CRLF, and
used two processes and two one-shot Adapter commands. The Edit command itself
privately invoked View and Apply.
A separate stale reuse control exited `1` with the existing Unavailable error
and preserved the already-edited bytes. The raw comparison used one Session
process with Search binding, optional View, indexed Replace Edit binding,
separate Apply, and `exit`: four work expressions plus one control expression.
It produced byte-identical final source while keeping binding, index, escape,
terminator, and publication responsibility with the raw caller. No elapsed-time
claim is made.

The JSON extractor existed only in the task-local fixture and established exact
substring transfer rather than parse-and-reserialize behavior. It is not a
product dependency. The fixture was removed after verification. Rust, tests,
Cargo metadata inputs, lockfile, and toolchain remain byte-identical to the
Gate 3 parent, so the existing complete 242-test result is reused rather than
misrepresented as a new code-test run. Direct source-release `bw --help`,
`bw version`, Search, Edit, stale rejection, and raw Session smoke evidence was
rerun for this documentation gate.

Gate 5's tracker consumer matrix binds each retained surface to its production
caller and behavioral regression. Public Rust Edit/Position validation,
`WorkspaceRuntime::apply`, Runtime geometry/publication, and Anchor reflection
remain covered by external-crate-style `edit`, `apply`, and `anchor`
integration tests. Raw Session remains covered by its parser, explicit binding
and clone/reuse paths, borrowed unindexed Apply, exact source results, and Data
rejection. Canonical one-shot Edit remains covered by the six existing
File/Paragraph/Line, terminator, invalid-input, unavailable-source, no-op, and
structural-composition regressions.

The existing Session operations case now proves all five Edit variants and all
four Position forms by adding valid `After(Line)` and `StartOf(File)` inserts.
Its unused clone is removed because the separate clone-and-both-Apply regression
is the unique reuse evidence. The invalid-form case now asserts the actual
`Edit input is invalid` stderr from `StartOf(Line)` and removes one duplicate
wrong-binding assertion. No test function, helper, fixture, production Rust,
Cargo input, CLI behavior, or public contract is added. Core NUL validation,
public Apply revalidation, exact source assertions, borrowed Apply structure,
and direct Apply/Anchor regressions remain distinct controls. The complete
offline/locked GNU-host suite remains 242 tests. Automated JSON
Search-to-one-shot Edit end-to-end coverage is intentionally Gate 6 input.

Gate 6 adds that one independent regression without a helper, parser, or second
wire path. It verifies the exact single-found `bw.cli.search.v1` prefix and
suffix, removes only those fixed bytes, decodes the remaining original object
as v4, passes its unchanged UTF-8 bytes as one Edit argv, and proves exact
`retry_budget = 3\r\n` to `retry_budget = 5\r\n` replacement with exit `0`,
empty stderr, and `OK` plus LF. Existing no-op, stale reuse, and terminator
regressions remain the unique controls for those meanings.

The complete GNU and musl suites each pass 243 tests. V4 KATs,
Search/View/Check/Apply semantics, Correct `1`/Safe Reject `6`/Wrong Apply `0`,
raw Session's five Edit variants and four Positions, binding/index,
clone/reuse, separate Apply, every one-shot target/terminator, and exact
`0`/`1`/`2` output boundaries remain intact. Compared with Source Authority
`4a1b06fb375bfd906a6f27de4de15a8febfe08ec`, Core, Runtime, Anddress v4,
toolchain, and dependency inputs are byte-identical, and the Adapter and Runtime
retain one Edit executor each. Source Cargo and `bw version` advance to
`0.2.2`; official `0.2.1` artifacts, installers, manifest, public tree, and
service remain unchanged. A source-built `0.2.2` Update may install official
`0.2.1` because the command has no version comparison; Gate 7 owns publication.

## 0.2.2 release closure

Gate 7 reconstructs the four canonical artifacts and sidecars from Source
Authority revision `04b36d9ca9cc725bedeb17231339c67b5f0590ea` and reproduces
the exact 876-byte manifest with SHA-256
`c2e55c9617db5a30fc5320d00e70d547ed9720bacbeac7e0a3cbec33b2fb079d`.
The publisher adds the eight `releases/0.2.2` files, replaces `install.sh`,
replaces `install.ps1`, and publishes the manifest last. It preserves bytes,
inode, mode, owner, size, and mtime for all 32 earlier versioned files and
`install.cmd`; an idempotent rerun reuses all 44 files and their metadata.

All 44 loopback and public HTTPS files pass GET and HEAD with exact bodies,
SHA-256, lengths, content type, zero HEAD downloads, and cache policy. Root and
unknown-path GET/HEAD remain 404/no-store. A task-local canonical `curl | sh`
fresh install and an actual public `0.2.1` binary's explicit update install
byte-identical `0.2.2` Linux binaries and print the exact Installed and Updated
outcomes. The installed binary passes Help, Version, JSON Search-to-exact-v4
one-shot Edit with CRLF preservation, View, Check, raw Session Apply, stale
reuse, and duplicate-drift Safe Reject probes.

Closure includes the completed 243-test GNU and 243-test musl source matrices and
their offline/locked metadata, tree, formatting, all-target checking, clippy
with warnings denied, and release builds. Origin 13, installer 37, publisher
53, and CMD 12 regressions pass with their standard checks and build. Origin
and cloudflared PID, InvocationID, restart count, listener, unit/YAML,
credential metadata, tunnel, DNS, actual user HOME, process PATH, and shell
startup files remain unchanged. macOS and Windows artifacts receive static
cross-build verification only; no native macOS, Windows, PowerShell, or CMD
execution is claimed. No tag, GitHub Release, crates.io publication, cache
purge, service, tunnel, DNS, route, or credential change occurs.

## 0.2.1 observation-reuse and release closure

The `0.2.1` target is published and closed. Phase 2 adds the Host Runtime
constructor, source invalidation kernel, private proof state, and
successful-Search proof installation. Phase 3 adds bounded ordinary View proof
consumption. Phase 4 adds Check current-proof group classification;
Phase 5 adds Apply proof precondition reuse, exact no-op preservation,
prospective-after proof installation, and coupled Anchor reflection;
Phase 6 closes path-exact invalidation, guarded mutation sequencing,
authority isolation, matching anchored View reuse, failure transitions, and the
both-mode drift matrix;
Phase 7 records the historical fixed A/B source-readiness NO-GO; Phase 7A
closes that run's sole failed related-Paragraph performance gate; Phase 7B
remeasures the complete matrix and closes source readiness with every gate PASS.
V4 wire and default Untrusted behavior remain unchanged; the development phases
preserved the then-current public `0.2.0` release until the separate `0.2.1`
publication closure. The
Protocol owns default Untrusted Mode and explicit Host-authoritative Mode; the
[phase tracker](../tasks/2026-08-30-backwriter-0.2.1-current-observation-reuse.md)
owns the execution audit, fixed `0.2.0` comparison inputs, complete raw Phase 7,
7A, and 7B samples, checksums, and gate evidence.

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

Phase 6 regressions prove both public invalidation methods delegate to one
I/O-free path-exact proof-plus-Anchor operation; invalid syntax, private paths,
and unadmitted paths preserve unrelated state; and same-hash paths, workspaces,
admissions, Runtimes, Host versus Untrusted mode, and Runtime lifetimes remain
isolated. After correct pre-mutation invalidation, same-length and
different-length replacement, deletion, invalid UTF-8, and NUL make stale View,
Check, and Apply safe-reject without publication. A confirmed Apply followed by
invalidation and external mutation likewise rejects its old after address.

Proof mismatches use filesystem-absence tripwires to prove zero source access
and state preservation for ordinary View and Apply, while Check remains
`NotCurrent` without mutation. Matching anchored View structurally shares the
ordinary trusted View helper; an anchored proof mismatch removes same-path
proof and continuity before source access. Existing private failure seams and
integration regressions fix trusted View open/seek/read/short/resource proof
removal, Apply open/read/resource and definite-prepublication preservation
boundaries,
invalid/length-drift fail-closure, no-op preservation, confirmed after
installation/reflection, and path-exact uncertain-publication invalidation.
Failed Search installs no provisional proof, Check fallback installs none, and
Runtime drop retains none.

The exact seven-cell duplicate-Line drift matrix passes in both Untrusted and
correctly guarded Host modes with one Correct Apply, six Safe Rejects, and zero
Wrong Applies; duplicate Paragraph drift rejects in both modes. Structural
evidence continues to exclude proof locks across I/O, hashing, emission, and
publication, plus whole-source retention, prior-proof chains, history, public
hooks, or persistent cache. The Phase 6 development suite passes 234 GNU-host
Rust tests. No Phase 6 result is a benchmark or release-readiness claim.

Phase 7 uses immutable A=`2fad6e4` and B=`a24ff5e`, independent offline/locked
release targets, fixed historical fixture SHA-256 values, the same task-local
harness, CPU 2 P-core, `powersave`, and one warm-up plus seven order-rotated
samples per A/BU/BH cell. Search passes at `268.333` ms BU and `268.379` ms BH
against `313.929`. Host Check hit passes with exact zero `rchar`/`wchar` and the
unchanged zero-read/hash/target-search structural evidence. One-million-result
peak HWM passes at `58.5078` and `58.5156` bytes/hit against `61.4383`; low-hit
128-to-256 MiB HWM is flat, whole-source retention remains absent, and Wrong
Apply remains zero.

Host Search-to-late-Line View fails: median `1,079.943` ms and p95 `1,096.362`
ms exceed the formal 400 ms ceiling and 350 ms recommendation. Its exact
`536,870,913` `rchar` consists of Search's complete source read plus the
trusted Line relation's complete no-separator Paragraph boundary read. Result
SHA-256, source SHA-256, count, v4 wire, order, multiplicity, Untrusted
fallback, and all Phase 6 transitions remain exact. The decision is therefore
NO-GO; production Rust and Cargo version remain unchanged. The Phase 7 suite
uses no new benchmark framework or repository instrumentation, and its
complete raw evidence and reproduction hashes are in the tracker.

Phase 7A removes the related-Paragraph path's private byte-at-a-time reverse and
forward cursors and reuses the same two fixed 8,192-byte scratch arrays. Direct
chunk scans preserve CR, LF, CRLF split across scratch boundaries, EOF bare CR,
no-EOL, Unicode scalar ranges, long Lines, surrounding space/tab separators,
BOF/EOF Paragraphs, source/error boundaries, and exact related addresses. A
word candidate filter is exhaustively checked across every byte alignment and
scratch-edge lengths; trusted and direct projection remain equal for every
scalar-aligned target range. The GNU-host suite passes 236 tests.

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

Phase 7A source verification passed 236 GNU-host tests. Phase 7B post-decision
verification passes offline/locked metadata and dependency tree, format,
all-target check, all 236 GNU-host tests, clippy with warnings denied, release
build, and release capability probes; version output is exactly
`Backwriter 0.2.1`.

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

Before staging, verify the diff and empty index, confirm repository-root
`.artext` is absent and untracked, and preserve unrelated task/history files.
Owner-authorized work then stages only the reviewed paths and repeats the
cached diff audit before commit.

The repository source package, source-built command, closed public
distribution, and installed release are `0.2.3`; the command prints exactly
`Backwriter 0.2.3` plus LF. The current installers, manifest, and Update handoff
select `0.2.3`; exact `0.2.2` remains the only other accepted manifest. Prior
`0.2.2`, `0.2.1`, `0.2.0`, `0.1.0`, and beta
versioned files remain immutable. The closed `0.2.1`
source suite passed 236 GNU-host and 236 musl Rust tests; the closed `0.2.0`
source suite passed 203 GNU-host Rust tests, and the historical `0.1.0` source
suite passed 193.

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
