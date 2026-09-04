# Backwriter Current State

## 0.2.6 operational Adapter and verification contraction — Gates 1–6 complete

Gate 1 records the approved Adapter boundary, Gate 2 closes command-local help,
and Gate 3 closes actionable usage failures and one-shot Edit stdin Content.
Gate 4 closes Line body replacement and separates it from advanced raw exact
extent replacement. Gate 5 adds shell-local numeric references and high-level
Replace, and Gate 6 adds ordered batch Check in the
[0.2.6 tracker](../tasks/2026-09-04-backwriter-0.2.6-operational-adapter-verification-contraction.md).
The governing rule is **explain what to type, what happens, and what comes
back**. Gate 6 adds only the narrow ordered Check Runtime seam and Adapter
writers; version, artifact, installer, publication, v5 wire, Search, and other
capability meanings remain unchanged. `0.2.5` remains the closed source and
public distribution.

`bw --help` equals `bw help`; `bw help X` equals `bw X --help` for Search,
View, Edit, Check, Shell, Update, and Version. Each command page uses fixed
section order and returns before Runtime/source I/O or Update download. Usage
errors use a stable code, cause, directly extracted canonical command usage,
and command help hint without a top-level help dump. Edit Content is exactly one
argv operand or the exclusive `--stdin` selector; it validates the address,
reads valid UTF-8 stdin to EOF, then opens Runtime. File/Paragraph bytes remain
exact and reject NUL with `edit.content_contains_nul`. Line accepts body only,
rejects CR/LF with `edit.line_body_contains_terminator`, and appends its decoded
None/LF/CR/CRLF terminator exactly once. Advanced raw Session Edit/Apply retains
caller-provided exact range bytes and a separate publication step; no exact
one-shot form exists. One `bw shell` process owns append-only `@N` references:
direct Search and View issue them, direct Replace consumes one and issues a
fresh one only when its receipt has an Anddress, and `Changed\tNone` issues
none. Direct Check resolves every reference before Runtime access, preserves
input order and duplicates, and issues fresh slots only for Current inputs.
`@N` does not collide with named raw bindings; `let name = @N` creates an
existing named Anddress alias. One-shot JSON Check is `bw.cli.check.v2` and
preserves one Current, NotCurrent, or Unavailable outcome per input; a batch
requires JSON while single human output remains one status line. The future
sequence is verification/docs contraction, then source readiness. Core Search
and one-shot Search output, v5 values/wire, raw Session, `apply_replace`, and
existing Check seam meanings remain fixed. Candidate execution
compares with `0.2.5`; inherited GNU/musl 268-test evidence and `Correct 1 /
Safe Reject 6 / Wrong Apply 0` remain required controls. Gate 2 adds three CLI
regressions, Gate 3 adds four, Gate 5 adds six, and Gate 6 adds three, so the
complete GNU and musl suites each pass 285 tests.

## 0.2.5 performance recovery — published and closed

Gates 1 through 8 are complete under the
[performance-recovery tracker](../tasks/2026-09-04-backwriter-0.2.5-structural-specialization-performance-recovery.md).
Gate 2 adds one checked segment operation to the existing literal matcher and
deletes the Runtime per-byte caller loop. Cargo, `bw version`, the canonical
four-target artifacts, installers, Update, and the official exact 68-file
distribution are published and closed `0.2.5`.

The governing rule is **semantics stay unified; execution becomes specialized
again**. V5 fields, algebra, canonical bytes, capability and Adapter outputs,
one structural cursor, one Issuer, single/batch View, receipts, Host proof,
publication, and Anchor behavior remain fixed. The performance target may
remove only capability work that has no consumer.

The matcher accounts for complete-Line length in one checked addition, skips
segments without the query's first byte, carries a partial KMP match across
chunks, and stops matching after a hit. File and Paragraph stop matcher work
after `FullLine` while the common cursor still validates and frames all source
bytes. Exhaustive partition tests and fixed native A/B/C measurements preserve
count, order, v5 bytes, tiers, terminators, UTF-8/NUL, and fail-all behavior.
The 256 MiB and 1 GiB C/A median ratios are 1.1389 and 1.1392, below the 1.15
ceiling but above the 1.10 target. This does not activate conditional cursor
specialization.

Gate 1 resolves three planning questions. First, exact source Line count stays
in v5 identity, observations, Host proof, and currentness. Same-hash,
same-length input with a false Line count remains `NotCurrent`; Gate 3 counts
Lines in a minimal raw same-read accumulator without Paragraph or parent
geometry rather than weakening this Safe Reject. Second, strict decode and
public `validate()` remain; the sole Issuer validates shared source once and
target geometry once, while typed View, Check, and Anchor no longer repeat
source-less validation. Third, one public allocation-reusing
`Anddress::encode_into(&mut Vec<u8>)` now clears and fallibly reserves a caller
buffer, emits the canonical v5 bytes through the sole writer, and leaves
`encode()` as a delegating surface.

Gate 3 splits one raw builder from the sole structural builder. Raw observation
owns UTF-8/NUL validation, SHA-256, checked byte length, exact Line count, and
chunk delivery; structural observation composes that state with exactly one
`StructuralCursor`. Check proof misses, ordinary/batch View, Apply before-state,
trusted exact-length staging, and unit Apply after-state are raw. Content Search
and proof-miss Anchor remain structural, and changed Apply output enables
structural projection only for a non-File receipt or live non-File Anchor.
Exact File Search and File-only receipt/Anchor work need no cursor.

On the fixed CPU-0/tmpfs A/B/C/D evidence, D/A median ratios are 1.0812 for
Untrusted Check, 1.0582/1.0501/1.0513 for unit/receipt/live-Anchor 256 MiB
Apply, 1.0287 for CRLF one-shot Edit, and 1.0948 for 134,217,728 `x\n`
Lines. Host Check remains in the approximately one-microsecond class with zero
capability I/O; Host and Untrusted self-Line View preserve the exact source
digest. Gate 4 preserves all four v5 KATs and every measured output digest.
Search and batch View reuse one operation-local encoding scratch; single Edit
and Check retain their one-address `encode()` path. The complete suite is 263
tests at the Gate 4 boundary. Production there is 304,475 bytes and 9,197 lines, only 12 bytes and 31
lines above Gate 3 and 2.42%/2.71% above the fixed
297,269-byte/8,954-line baseline. This remains within the existing
direct-evidence allowance and does not renew it; later contraction still owns
the final target.

Gate 5 replaces the monolithic Search provisional vector with measured
16,384-entry chunks. It retains global result indexes and promotes matched
content Lines across chunk boundaries while leaving separator matches attached
to File. Issuance starts only after the complete source observation and source
identity succeed; it consumes chunks in order and releases each before the next
one. Search and Apply now use one geometry-owned Paragraph attachment helper,
while retaining their distinct invalid-source and ordinary unattached-result
classification. On the fixed CPU-0/tmpfs comparison, peak HWM falls from about
166 MiB to below 86 MiB for both one huge Paragraph and 1,048,576 one-Line
Paragraphs. Shared Paragraph allocation is therefore not activated.
GNU and musl each pass the complete 268-test suite.

Gate 6 removes false fallibility from raw/structural observation and Apply
output construction, removes raw/cursor offset parity returns, and drops
Anchor observation's all-index copy. Existing `Anddress::same_source` and the
one Runtime source-state comparator replace local source/state/proof wrappers;
one tier slot serves mutually exclusive File/Paragraph Search, and Issuer
construction delegates to its strict owned-source path. Production G is
304,431 bytes/9,213 lines, -1,727/-48 from F and +7,162/+259
(2.41%/2.89%) from B. It is below the 306,187-byte/9,222-line ceiling; B remains
the target, and the retained delta is not a refreshed allowance.

Gate 7 preserves production `src/**` byte-for-byte at G and records a GO. On
the fixed CPU-0/tmpfs A/B/G comparison, 256 MiB and 1 GiB sparse G/A
median/p95 ratios are 1.0983/1.1163 and 1.0959/1.0984. The first p95 misses the
1.10 target but remains below the 1.15 hard ceiling. Dense G peaks at 87,924
KiB and 87,992 KiB for the two prescribed 1,048,576-result shapes, below the
130 MiB target, with exact B/G v5 output. CRLF Edit is 1.0062/1.0324 G/A;
Untrusted Check is 1.0804/1.0689. A repeated full Apply confirmation passes
unit, receipt, and live-Anchor ratios within 1.10 after one initial receipt p95
outlier. GNU and musl each pass 268 tests; drift remains Correct 1 / Safe
Reject 6 / Wrong Apply 0.

Reusable encoding records zero loop allocations for one repeated target,
1,048,576 Lines, and 1,000,000 Files while preserving exact B/G v5 digests.
The 200,000-file Search order and batch/sequential View results agree exactly.
Host Check retains zero capability open/read/hash/cursor work. No completed
gate activates `StructuralDemand`, cursor specialization, or shared Paragraph
allocation. At the Gate 7 boundary, the only remaining gate was separately
authorized release work; Gate 8 is now complete.

## Published and closed 0.2.4 structural authority

Gates 2–7 implement and verify the v5 Rust algebra, canonical wire, sole
crate-private Issuer, common structural cursor, direct Search result
collection, and geometry-driven exact-range View and Edit/Apply/Anchor
projection.
Cargo, `bw version`, the four canonical artifacts, installers, manifest,
Update target, and official distribution are published and closed `0.2.4`.
Published and closed `0.2.3` remains immutable v4 release evidence. Update
performs no version comparison and installs or reinstalls official `0.2.4`.

Current source is hard-cut to `artext.backwriter-anddress.v5`. Shared
`SourceIdentity` includes the exact source Line count alongside workspace
coordinate, logical path, SHA-256, and byte length. File carries full range and
Line count; Paragraph carries range, zero-based File Line offset, and Line
count; Line carries range, exact terminator, parent geometry, and zero-based
Line offset within that parent. Text Lines belong to a Paragraph. Blank and
horizontal-space/tab-only Lines belong directly to File.

Anddress is now the authority for source/state relationships, containment,
overlap, parent and projection, Line count/number/range, terminator, and
projection validity. One validator serves decode and the sole crate-private
Issuer. Issued targets share one immutable source identity, while the v5 wire
is self-contained and omits unused geometry. Runtime Search and prospective
Apply output now carry exact parent/range/count/offset/terminator geometry.
One allocation-bounded `StructuralCursor` supplies complete-source CR/LF/CRLF/
no-EOL, body-class, Line, and Paragraph framing to Search, source observation,
and prospective Apply output without changing matching, ordering, staging,
publication, proof, or Anchor semantics.

Gate 3 hard-cuts `SearchOutcome::Found` to direct owned Anddresses and
removes the native occurrence/position wrappers. Human and JSON Search bytes,
order, and duplicate multiplicity are unchanged; every display position now
derives from v5 geometry. Gate 4 removes View's relation and Paragraph scans:
every request first uses `Anddress::project`, then confirms currentness and
returns only `ViewOutcome::Projected { anddress, content }` or
`RelationAbsent`. Single and batch CLI View share the hard-cut
`bw.cli.view.v2` item writer; batch requires JSON plus one explicit `--as`
projection. Existing human/raw single output remains byte-identical. Gate 5
removes one-shot Edit's private View: strict v5 decode supplies target geometry
and the exact Line terminator, invalid Line Content fails before Runtime access,
and Apply remains the sole currentness and publication boundary. Public unit
Apply, Replace receipts, Host proof, and Anchor reflection share the existing
executor and one prospective `StructuralCursor`/Issuer pass; local range
helpers and a separate relation allocation are gone in favor of v5
`contains`/`overlaps`. Existing matching, batch grouping, publication, proof,
and Anchor mechanics retain their actual consumers. Gate 6 confirms that Check
validates all v5 inputs before I/O, groups by source key, compares only SHA-256,
byte length, and Line count, and uses matching Host proof without opening the
source. A nonmatching proof is I/O-free `NotCurrent`; proof miss uses one shared
observation per source and does not install or mutate proof or Anchor state.
Data, Pick, and Session already consume the
direct v5 values, so no adapter or parallel collection was added. Gate 7
passes 258 tests on both GNU and musl, the fixed A/V5/B sparse, dense,
200,000-file, View, and Edit comparisons, the blind-drift 1/6/0 matrix, and
the exact Search-to-batch-View-to-fresh-receipt workflow without changing
production `src/**`. The [eight-gate tracker](../tasks/2026-09-03-backwriter-0.2.4-structural-authority.md)
separates authority, implementation, semantic evidence, version readiness, and
release approval. Gate 8 rebuilds the pinned artifacts, publishes eight
versioned files followed by both installers and the manifest last, verifies
all 60 files and an idempotent rerun, and closes the release. Stdin and CLI file
splitting remain separately owned later decisions.

## Published and closed 0.2.3 Patch Box

Gates 1 through 8 close authority, the consumer matrix, Search observation
metadata, native single View projection, and ordered batch View for the `0.2.3`
Patch Box. It is an AI-facing information-surface
patch, not an engine-performance project. Search now returns ordered
`SearchOccurrence` values that pair each exact opaque v4 Anddress with its
same-observation descriptive position. Single View now accepts one existing
`AnddressTarget` projection and returns the projected current v4 Anddress plus
exact Content from the same accepted observation. Batch View preserves caller
order and duplicates, returns all outcomes or none, and groups inputs so each
Untrusted or Host-proof-miss source is opened once and directly observed once.
Gate 5 adds the Replace-only native receipt seam and reuses its returned fresh
current Anddress when the confirmed result has one. Gate 6 exposes that result
through exact human and JSON one-shot Edit output and closes stdin as a
no-addition decision. Gate 7 closes integrated Dummy and GNU/musl source
readiness without changing production feature code. Gate 8 reconstructs the
four exact artifacts and manifest, publishes the 52-file tree manifest-last,
and closes the official release.

Line Search metadata is the current one-based Line number. Paragraph Search
metadata is the current one-based inclusive start-to-end Line range. File
Search has no position. This information is produced while Search already
frames and hashes the source; it adds no open, read, hash pass, retained source,
or Adapter reread. It is not an Anddress field, identity, currentness evidence,
selector, or Edit input. Equal hits and duplicate results remain present.
Machine Search is the Adapter-only `bw.cli.search.v2` occurrence envelope;
every item is self-identifying by logical path, target kind, applicable decimal
Line position, and exact embedded v4 Anddress. Human Search uses `path:line` for
Line, `path:start-end` for Paragraph, and path alone for File. Pick retains its
raw-Anddress byte-range rows unchanged.

View remains Observe/Project, never Find. Its implemented seam is
`WorkspaceRuntime::view(&Anddress, AnddressTarget) -> Result<ViewOutcome,
ViewError>`. A caller-held Line may project to Line, Paragraph, or File; a
Paragraph to Paragraph or File; and a File only to File. Downward projection
is `InvalidInput` before source I/O; implicit Search and relocation are
excluded. File, Paragraph, and Line outcomes include the projected current v4
Anddress and exact target Content. A Line-to-Paragraph request with no exact
containing current Paragraph returns the normal `RelationAbsent` outcome. The
batch seam is `WorkspaceRuntime::view_batch(&[Anddress], AnddressTarget) ->
Result<Vec<ViewOutcome>, ViewError>`. It validates the complete collection
before I/O, restores exact input order and multiplicity after source grouping,
and uses one direct observation per source for Untrusted and Host-proof-miss
execution. A matching Host proof is selected once per source and serves every
member through one handle and the existing trusted scanner. Empty input is an
I/O-free empty success; any later failure discards every provisional outcome.

`WorkspaceRuntime::apply_replace(&Edit) -> Result<EditReceipt, ApplyError>`
accepts only `Edit::Replace`; every other Edit is `InvalidInput` before source
I/O. `Unchanged { anddress }` returns the validated input after either no-op
path without publication. `Changed { anddress }` follows confirmed publication:
File always has a fresh full-range address, a terminator-preserving Line has
its exact fresh range, and Paragraph has an address only when direct after
projection produces exactly one Paragraph. Zero or multiple Paragraphs remain
successful as `Changed { anddress: None }`.

The original `WorkspaceRuntime::apply(&Edit) -> Result<(), ApplyError>` remains
the external Rust and raw Session five-operation/four-position seam. Both
public calls use one executor and the same prospective-after hash, length,
projection, prepared Host proof, publication, and Anchor plan. No successful
receipt exists for a definite failure or `PublicationUncertain`; uncertain
publication retains the existing same-path proof and Anchor invalidation. The
one-shot CLI consumes `apply_replace` and writes its receipt directly. Human
output is `Unchanged` or `Changed`, tab, then a canonical v4 object or `None`;
`bw.cli.edit.v1` uses `unchanged`/`changed` and the same object or `null` in
fixed key order. It performs no post-publication Search, reopen, or second
observation. Apply failures write no stdout. A later output failure remains
exit `1` after the no-op/publication decision and creates no rollback or retry.
Argv remains the only Content transport; literal `--json`, `--raw`, and
`--stdin` remain Content in that position. Empty/Unicode, File/Paragraph
newline, and permitted Line-body cases have direct coverage. Known argument,
shell, process-list, and history constraints provide no reproduced consumer
failure, measured payload need, or concrete security requirement for adding a
stdin reader or EOF state.

The [eight-gate tracker](../tasks/2026-09-03-backwriter-0.2.3-patch-box.md)
records the completed carrier, View projections, native Edit receipt,
one-shot Adapter output, integrated source-readiness evidence, and release
closure. Cargo, `bw version`, artifacts, installers, manifest, public root,
and `bw update` are published and closed `0.2.3`. The exact public tree has 52
files. Services, tunnel, DNS, and actual user HOME state were unchanged by
publication.

## Version boundary

The closed public `0.1.0` release remains immutable v3 evidence. Published
`0.2.2` Rust, Cargo, tests, CLI, and distribution use the hard-cutover Anddress
v4 API and wire. Phases 3–7 implement and verify SHA-256 source identity, exact byte length,
target kind, `[start,end)` range, target-specific Search observation, direct
View/Check consumption, and direct Apply/Anchor consumers without a v3
compatibility seam. The integrated correctness and formal performance gates
pass. The Owner correction confirms that the million-hit recommendation is
result-memory HWM, not JSON payload: 346.539 to 58.551 HWM bytes/hit is an
83.10% reduction and passes. The Line-only content-slice fast path then lowers
the fixed 256 MiB median from the paired v3 673.898 ms to 272.111 ms, a 2.476×
speedup that closes the remaining recommendation. Source release-readiness is
GO, and the exact four-target `0.2.2` artifact set, canonical manifest,
installers, 44-file live publication, endpoint verification, fresh
installation, and explicit public `0.2.1` update are closed. The earlier evidence
tracker is
[Backwriter 0.2.1 current-observation reuse](../tasks/2026-08-30-backwriter-0.2.1-current-observation-reuse.md).

## Published Backwriter 0.2.2 Anddress-first editing authority

`0.2.2` is published and closed. Gates 1–6 close its general
editing authority, minimum implementation, no-addition transport decision,
user/AI surface alignment, consumer separation, integration, and version
decision: one
Adapter operation accepts an encoded v4
Anddress and new Content, then privately reuses v4 decode, Runtime View,
target-specific Content normalization, existing `Edit::Replace`, Runtime
Apply, and the existing CLI status/error writers. Caller-visible View, Check,
binding, index, and Core Edit construction are not prerequisites.

File and Paragraph use exact replacement Content under the existing NUL rule.
Line accepts body Content without NUL, CR, or LF and preserves the current
None/LF/CR/CRLF terminator obtained by the private View. Mutation between View
and Apply is rejected by Apply's existing exact source-state precondition.
There is no Check prerequisite, relocation, context matching, retry, merge,
history, fallback, new Runtime seam, or `NotCurrent` Apply alias. V4 wire,
hash, length, kind, range, byte-identical no-op, publication, Anchor reflection,
and Host proof/invalidation meanings are unchanged.

Gate 3 adds no Content transport or Edit machine output. Existing argv carries
empty and Unicode values, File/Paragraph CR/LF, and allowed Line bodies; literal
`--json` and `--raw` in the Content position are exact Content. Search JSON
already supplies exact v4 Anddress objects, Edit has no target/result to return,
and the existing `0`/`1`/`2` status and stdout/stderr boundary remains the only
result contract. Returning a new Anddress would be an implicit re-search, while
JSON cannot refine broad `Unavailable`, publication uncertainty, or an output
failure after Apply. Exit `1` therefore does not prove source preservation and
adds no retry authority. OS argument limits, shell quoting/newline portability,
and process-list/history exposure remain known argv constraints; only reproduced
consumer, measured payload, or security evidence can justify a later
Owner-selected single transport.

Gate 4 makes JSON Search followed by exact opaque-v4 one-shot Edit the default
documented Replace flow. Human Search rows are not Edit input; callers do not
interpret or rewrite address fields, do not reuse the old address after
success, and do not automatically retry exit `1`. The exact single-Line CRLF
fixture proves two one-shot processes and Adapter commands produce the same
final bytes as one raw Session process with Search binding, optional View, indexed
raw Replace Edit binding, separate Apply, and `exit`. The raw surface remains
the advanced lifetime/composition path, with no timing claim or new tool,
wrapper, transport, or schema.

Gate 5 retains public Rust `Edit`/`Position`/Apply as the exact primitive for
direct callers, Runtime geometry/publication, and Anchor reflection. Raw
Session remains the advanced exact-byte, all-operation/all-position,
binding/index/clone, separate-Apply, and Anchor/Data-lifetime composition
surface. Canonical one-shot Edit remains the Anddress-plus-Content Replace
contraction and alone owns Line terminator preservation. The reaudit adds no
type, layer, alias, prefix, facade, transport, or Edit `DataKind`; external Rust
consumers are not inferred absent from repository-local references. The ordered
work is tracked in
[Backwriter 0.2.2 Anddress-first editing](../tasks/2026-09-01-backwriter-0.2.2-anddress-first-editing.md).
Gate 6 passes the complete 243-test GNU and musl semantic matrix and one exact
JSON Search-to-one-shot Edit CRLF E2E. At Gate 6, Source Cargo and `bw version`
became `0.2.2`, while Core, Runtime, Anddress v4, toolchain, and dependencies
remain unchanged from the closed `0.2.1` Source Authority. Gate 7 reconstructs
the four canonical artifacts from revision
`04b36d9ca9cc725bedeb17231339c67b5f0590ea`, publishes the manifest last,
and closes the exact 44-file public root. At that closure, the installers and
`bw update` selected official `0.2.2`; Update still performs no version
comparison.

## Published Backwriter 0.2.1 observation reuse

The `0.2.1` observation-reuse target is published and closed. Phase
7A closes the sole failed historical Phase 7 performance gate; Phase 7B then
remeasures the complete fixed matrix and passes every formal gate. Cargo,
source, CLI, version output, and official distribution are `0.2.1`. The work
preserves the complete v4 Anddress API/wire and immutable prior releases. The
Protocol closes two execution modes: the default `WorkspaceRuntime`, one-shot
CLI, and ordinary CLI Session remain Untrusted Mode with the existing per-call
one-read/hash path; only an explicit Host-authoritative Mode may reuse a Runtime-local,
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
both modes. At that phase this closed source readiness only; artifacts and
publication remained separate, and official public `0.2.0` was unchanged.

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
gate evidence. The separate Owner-authorized release closure regenerated the
four canonical artifacts and manifest from Source Authority revision
`4a1b06fb375bfd906a6f27de4de15a8febfe08ec`, published the exact 36-file tree
with the manifest last, and idempotently reused every file on a second run.
All 36 loopback and public HTTPS GET/HEAD endpoints, task-local fresh install,
actual public `0.2.0` update, installed capability probes, GNU and musl 236-test
suites, and operations regressions passed without changing service, tunnel,
credential, or actual user HOME state. macOS and Windows remain static
cross-build evidence without native execution claims.

## Core capability inventory

| Letter | Word | Current status |
| --- | --- | --- |
| S | Search | Rust implementation with one-read target-specific v5 projection and exact File lookup. |
| V | View | Rust implementation with direct v5 exact-source/range projection. |
| P | Pick | Rust implementation over complete v5 values. |
| A | Anchor | Rust implementation with Runtime-local live continuity. |
| C | Check | Rust implementation with v5 source-state batch currentness reporting. |
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
JSON Search, View, Check, and Edit plus raw View and Session Pick,
batch Check, Anchor, Edit, Apply, result binding, and explicit Data over the existing public
Runtime seams. The Session retains one Runtime, one caller-owned `DataStore`,
and explicit CLI-local bindings plus non-aliasing owning Anchedress handles.
Session Pick passes a named candidate collection and a
CLI-parsed predicate to the existing pure Core function. Session batch Check
passes an exact matching binding clone to `check_search` or `check_pick` and
exposes only its report counts. Session Anchor creates an opaque Runtime-local
handle, views it through the existing anchored seam, and invalidates only its
logical source. One-shot Search, View, Check, and Edit JSON stream compact Adapter
envelopes without creating a Core wire; related or filtered values are exact
existing v5 Anddress objects. Raw View is an explicit Adapter exact-text
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
batch Check, and raw Edit/Apply transport still await collection or transport
authority. The distinct `0.2.2` Anddress-first one-shot Edit is implemented.
Raw output other than completed one-shot View and further
Session behavior remain deferred under the
[CLI V1 authority](../architecture/backwriter-cli-v1.md). The published
`0.2.2` Core/Runtime and CLI behavior remains frozen; no collection wire, raw
Edit transport, or Session machine output is implied by the new contract.
The canonical Linux x86_64 release target is `x86_64-unknown-linux-musl`; the
GNU target is retained for local development and tests. Target selection and
direct build verification are complete. The external operations-owned
distribution at
[https://backwriter.pentagration.com](https://backwriter.pentagration.com)
publishes the closed Backwriter `0.2.5` release for Linux/WSL x86_64,
macOS arm64, macOS x86_64, and Windows x86_64. Linux uses
`x86_64-unknown-linux-musl`; macOS uses
`aarch64-apple-darwin` at minimum 11.0 and `x86_64-apple-darwin` at minimum
10.12. Windows uses `x86_64-pc-windows-gnu` and canonical `bw.exe`. Their
artifacts, manual-verification checksum sidecars, expanded canonical manifest,
POSIX and PowerShell installers, and publication are complete from Source
Authority revision `e4022fc073e9df3928e1c3817b266ce92121a03c`. The installer verifies the
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
beta.3 files remain unchanged and immutable. The complete stable `0.1.0`,
`0.2.0`, `0.2.1`, `0.2.2`, `0.2.3`, and `0.2.4` version directories are
immutable. The complete current `0.2.5` directory is immutable, its planned
matrix is complete, and the `0.2.5` release is closed; any later platform or
version requires separate Owner authority. Tags, GitHub Releases, crates.io
publication, and GitHub distribution remain outside the completed publication.
The current Cargo package and library crate are `backwriter` at published
`0.2.5`; the sole canonical executable and external Adapter command are `bw`.
There is no current `backwriter` binary, alias, or wrapper. Product prose
continues to use Backwriter, and persisted Core wire/private-path and
distribution artifact/domain contracts keep their existing names. `0.2.5`
publication is closed: the exact 68-file public tree retains all 56 prior
versioned files and `install.cmd`, while the current installers and manifest
select `0.2.5` and `bw update` delegates to that official installer. The
installers accept only the exact closed `0.2.4` and current `0.2.5` manifests;
`0.2.3`, `0.2.2`, `0.2.1`, `0.2.0`, stable `0.1.0`, and beta.3 acceptance is
retired.

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

## Implemented published 0.2.1 current-only Runtime contract

The v4 Search and View implementation is current-only and stateless;
Pick is pure and stateless over caller input. `WorkspaceRuntime::search`,
`WorkspaceRuntime::view(&Anddress, AnddressTarget)`,
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
Ordinary View uses that common observer directly. It captures only the requested
self-or-ancestor File, Paragraph, or Line Content while validating the caller
input against the same observation. Line self projection also keeps minimal
Paragraph-boundary state for its optional related Paragraph; Line-to-Paragraph
captures only the containing Paragraph and returns `RelationAbsent` for a
separator or valid caller-built nonstructural Line. Check groups by coordinate/path, observes each
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

## Implemented 0.2.1 v4 exact-source address kernel

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
