# Backwriter 0.3.0 independent namespace and complete shell View

## Status — Gates 1–4 complete; Gate 5 pending

Recorded 2026-09-05. Gate 1 closes authority, audited discrepancies, open
decisions and acceptance. Gate 2 implements the existing namespace predicate
and focused regressions only. Gate 3 implements Owner-approved D1/D2 after
the preserved proposal below; its focused verification is recorded separately.
Gate 4 completes Help, private CLI/test modules and evidence contraction.
No version bump, readiness SHA,
artifact, installer, publication or operational change is made. Cargo/CLI and
official distribution remain published and closed `0.2.6`.

The [source note](2026-09-05-backwriter-0.3.0-independent-namespace-complete-view-source.md)
and [grounded roadmap](2026-09-05-backwriter-0.3.0-independent-namespace-complete-view-roadmap.md)
are preserved planning evidence. BOX references identify the source note, not
another semantic authority. Active [Protocol](../architecture/backwriter-text-coordination-protocol.md),
[address model](../architecture/rebuildable-structural-addressing.md),
[CLI](../architecture/backwriter-cli-v1.md), [principles](../principles/backwriter-core-principles.md)
and [verification](../development/verification.md) own the reconciled target.

| Input | Pinned evidence |
| --- | --- |
| Backwriter entry | `main`, HEAD = origin/main = `4cb15c23f978dbf0dc3d4fbe884170df181b399e`; tracked worktree/index clean; only the two intended planning notes untracked |
| N-1 Source Authority | `09bb6c424081594bd86a95f04345b786ef9b46b6`, closed `0.2.6` |
| Server read-only control | `main`, HEAD = origin/main = `04bd84a5a386638e4bf7e4d2e60d092bda3c3d35`, clean |
| Public control, inherited R3 evidence | `0.2.6`, 76 files/11 directories, manifest 876 bytes, SHA-256 `47001acd4831954a5106a3aac5b9fdfe0b36791144f355f52523cd0d0eb7d5f1`; no new live probe |
| Source-note file SHA-256 | `c0c26b95b762bccb951e9b2b4fa02333a2bb4ddf230931a102b2a3d03eb9b377` (includes the planning pass's terminal LF) |
| Roadmap file SHA-256 | `6bedb46814df6442976a9a46c23488ecad672dd92d5f65f6ef95116a51866821` |

Both planning files are committed byte-for-byte as received. Their references
to a future Gate 1 remain historical planning language, not current status.

## Gate 1 consumer audit and contraction decisions

The audit used `rg` definitions, direct callers and related tests, not a global
`artext` replacement. It does not infer absence of external Rust users or read
real historical state. The storage assertion in BOX 3–4/23 is reconciled with
the following actual production evidence.

| Responsibility | Current production consumer/evidence | Decision and target |
| --- | --- | --- |
| Root/base resolution | `src/runtime.rs::open_with_authority` accepts an absolute ordinary workspace, canonicalizes/opens it and creates RAM anchors/proofs | Keep accepted Runtime workspace root and no-follow policy; no private directory creator |
| Private source exclusion | `runtime.rs::is_backwriter_spill`, both platform branches | Reuse one shared policy for exact root `.bw` plus legacy `.artext/bw`; Windows ASCII-case policy, otherwise case-sensitive; Gate 2 only |
| Search content and exact File | `runtime/search.rs::execute_exact_file`, executor preflight, traversal and selected-source checks call that predicate | Keep caller-specific Empty/skip/error behavior and validation order; no matcher, tier, traversal algorithm, order, duplicate or writer change |
| View single/batch | `runtime/view.rs::validate_runtime_input` before accepted source access | Reuse guard and existing Unavailable boundary; no new observer |
| Check | `runtime/check.rs::classify_group` before proof/source access | Preserve private-path NotCurrent and hash/length/Line-count classification |
| Apply | `runtime/apply.rs` shared executor preflight checks the same predicate before staging | Preserve errors, prospective result, publication and proof/Anchor reflection |
| Anchor/invalidation | `runtime/anchor.rs::observe_current` and `invalidate_source` share the guard | Preserve fail-closed observation and path-exact invalidation |
| Private root creation | No creator in Runtime/CLI; `open_with_authority` opens only the workspace | N/A; no eager `.bw`, `.bw/bw`, HOME relocation or new environment precedence |
| Private stored-state read/write | No reader/writer/schema consumer found; DataStore, refs and Host proof are RAM only | N/A; no store, old reader, fallback, migration or deletion |
| Generic spill and stored-state cleanup | Protocol reserves future spill for a host-provided system root; no implementation | N/A; do not introduce spill, source retention or cleanup to satisfy a checklist |
| Update bootstrap | `src/bin/bw.rs::UpdateTemporary::{create,installer,cleanup,handoff}` and Drop | Keep `env::temp_dir()/backwriter-update-<nonce>`, ordinary-directory checks, Unix cleanup and Windows handoff; unrelated to workspace state |
| Apply temporaries | `runtime/apply.rs::Temporary`, `edit_temporary_name`, `publish` | Keep admitted same-parent create-new/no-follow staging/readback/removal/rename; `.env.artext-apply-edit-` and `artext.backwriter-apply-edit-v1-temporary` unchanged |
| Installers | Read-only server `backwriter/install.sh`, `install.ps1`, `install.cmd`; no `.artext`/`.bw` consumer | Keep HOME `.local/bin/bw[.exe]`, existing task temporaries and staging; no installer/Update execution or change |
| Existing path regression | `tests/search.rs` exact File and private/sibling/nested cases; `tests/view.rs::view_rejects_private_path_before_access_and_allows_other_artext_children`; Runtime Windows predicate test | Reuse unique boundaries and extend focused coverage in Gate 2, without actual old-state access |

No non-reconstructible stored-data consumer was found. Discovery of one later
stops only the dependent cutover for an Owner decision; it does not authorize
opening, migrating or deleting its real contents. `.bw` is a reservation and
source-exclusion target, not a claim that a store already exists. The retained
legacy guard protects leftovers without supporting old state.

| Shell responsibility | Actual consumer / regression | Keep, remove or pending |
| --- | --- | --- |
| Dispatch and refs | `execute_session_command` -> `execute_session_view` -> `execute_session_ref_view`; `parse_session_ref_view`, `resolve_session_ref` | Keep handwritten parsing, numeric/named syntax, validation and one initial resolution |
| Runtime result acquisition | Explicit `--as` uses `view_batch`; omitted `--as` loops through `run_view` | Reuse native results; mixed-kind plural self issue remains D1 below, not a hidden wrapper |
| Lost Content | `execute_session_ref_view` destructures `Projected { anddress, .. }` | Remove discarded Content in Gate 3; write the owned result without cloning a display collection |
| Lost successful peers | `outcomes.iter().any(RelationAbsent)` returns `write_session_relation_absent`, which writes only absent entries | Remove this whole-command presentation branch; preserve each normal outcome in place |
| Ref allocation and output | `reserve_session_refs`, `write_session_ref_line`, `BufWriter`, existing stream errors | Reuse capacity/error/location machinery; absent consumes no slot, duplicates each consume one; exact framing/stream-slot KATs are D2 |
| Retained raw/JSON output | `run_view` and existing human/raw/JSON writers, raw named Session result bindings | Keep byte-exact output and actual advanced consumers; do not use them as a Content-fetch workaround |
| Behavioral ref evidence | `shell_local_references_start_at_zero_append_in_order_and_keep_named_raw_aliases`; `shell_local_view_relation_absent_and_search_failure_do_not_consume_reference_slots`; malformed-ref and Replace/Check slot tests in `tests/cli.rs` | Existing all-projected and all-absent KATs do not prove mixed-peer Content; Gate 3 must fill that gap |
| Help | `write_command_help` and shared constants; `command_local_help_kats_are_exact_and_skip_runtime_opening`, usage and example tests | Keep one canonical usage source. Replace unsupported advanced-topic help with real raw Session help in Gate 4, not one-shot execution |
| CLI physical structure | `src/bin/bw.rs`, `tests/cli.rs`, structural `include_str!` consumers | Relocate existing responsibilities only in Gate 4; conditional test modules within one CLI integration crate, retain distinct structural evidence |

Source lookup confirms `view_batch` takes one common `AnddressTarget`. Current
single-input explicit `--as` also takes that batch branch. Current plural
without `--as` is not a one-batch implementation. No tests or production code
are changed to conceal those facts. The existing body-only Replace and raw
Session exact-extent regressions remain distinct consumers, not duplicates.

## Closed target boundaries and explicit prerequisites

The Protocol fixes exact Runtime-root-relative `.bw` and legacy `.artext/bw`
components plus descendants. `.bw-notes`, `.artext/bw2`, other `.artext`
children and nested `x/.bw` / `x/.artext/bw` remain ordinary sources subject
to admission/no-follow policy. No whole `.artext` reservation, path alias,
storage reader or eager creation is authorized.

The CLI target requires input ref -> fresh ref, kind/location and exact returned
Content for each projected occurrence. Order and duplicates survive; absent is
shown at its own position without a slot or fabricated Content. Runtime failure
returns no partial success. Count/reserve projected slots before append/output,
consume existing outcomes, and perform no display-only Search/View/re-resolution.
Single input requires one single View; batch requires one batch View. No second
executor or Content-copy collection is allowed.

| Decision | Status / required closure |
| --- | --- |
| D1: mixed-kind plural self-View | Owner approved; Gate 3 implemented and focused tests passed. Replace common-kind argument with Option: None=self, Some=common upward. Single View unchanged; source-breaking caller migration without facade or second executor |
| D2: exact framing and stream-failure slot reporting | Owner approved; Gate 3 implemented and exact KAT/failure tests passed. Byte-length Content framing, input/fresh refs, reserve-before-output, append-before-record, no absent slot, terminal Stream and no Drop retry; no delivery, cancellation or rollback claim |
| D3: original four-file fixture | Locate before Gate 5. Repository docs/tests search found the older three-file control, not the exact independent four-file bytes/oracle. If unavailable, explicitly define one common spec-conformant replacement, independent full-byte oracle and new digest for all four arms; no lost-fixture identity claim |

D1/D2 are closed by Gate 3; D3 still blocks its Gate 5 claim. Exact framing
comes from the approved proposal, not the illustrative arrow. No private storage base
decision remains open: the implemented exclusion base is the Runtime root and
there is no store to relocate. Actual future IO needs a new consumer audit.

Keep all v5 fields/wire/KATs/hash domains/workspace coordinate, source identity
and `sourceLineCount` currentness, including false-count `NotCurrent`. Keep
Search matching/traversal algorithm/tiers/order/multiplicity/schema, one-shot
raw/JSON View, exact File/Paragraph Content, Line body-only replacement with
None/LF/CR/CRLF preservation, raw Session, one-shot stdin, Apply/Anchor/Host
proof semantics. Shell stdin remains command input, not EOF Content transport.
No persistence/history/relocation/rebinding, rollback/CAS/lock, new parser,
public crate, generic framework, compatibility reader or performance project.
Multiple Replace commands are still separate publications, not a transaction.

## Five gates and acceptance

| Gate | State | Required evidence before completion |
| --- | --- | --- |
| 1 — authority | Complete | Consumer/N/A inventory; active target versus closed release; D1–D3; unchanged planning notes; input equality, metadata, document/Git hygiene |
| 2 — namespace | Complete | BOX 23 focused tests using existing filter; absent/new/old-only/both roots; task-local old sentinel byte equality; new reserved file/symlink never exposed; exact components/case and ordinary sibling/nested paths; no help/version creation; existing no-follow/admission/direct-access errors; nonexistent store IO stays N/A |
| 3 — complete shell View | Complete, approved D1/D2 | Single self and Line-to-Paragraph/File; one single/batch call; ordered duplicate input/ref/Content mapping; mixed projected/absent peers; empty/terminator/framing KATs; reserve/Runtime/write/flush failures; zero Content-only extra observation; unchanged raw/JSON/Replace; recoverable reserve overflow is not allocator-exhaustion proof |
| 4 — help/modules/verification | Complete | Executable direct/named/quoted examples including ref producers, fresh Current slots and same-source staleness; real advanced Pick/Anchor/Apply/Data help; no new one-shot/parser/crate; private code relocation; tests split only for useful navigation within one CLI integration crate; current rules extracted before duplicate history links, unique evidence preserved verbatim |
| 5 — integration/readiness | Pending, D3 required | Final GNU/musl semantic matrix, metadata/tree/fmt/check/test/clippy/release and release smoke; fresh N-1/N four arms; exact independent oracle; actual candidate identity and GO/NO-GO, then separately approved release slices |

Gate 2 sentinels belong only to a task-local fixture, never the real old path.
An ignored subtree need not be opened to validate its contents. BOX 23's absent
new-root and both-root storage expectations mean noncreation/nonconsumption in
this store-less implementation. No test may introduce IO just to obtain PASS.

Gate 4 must not remove unique tests to meet file-size targets or replace
existing safety checks with forwarding wrappers. Ten help sections are the
old KAT, not a requirement for empty Version sections in the target. Preserve
current common verification rules before moving history; duplicate evidence may
be linked, unique metrics/environments/path spellings/SHAs need a preserved
location first. A minimal historical index is only a later bounded decision,
not a Gate 1 file. Do not rewrite past release facts or claim speed from moves.

Gate 5's BOX 25–26 fixture has eight duplicate Lines in four files, one primary
edit per file, four untouched secondary Lines and LF/CR/CRLF/None. Run four
independent arms: Dummy N-1, Dummy N, Genie N-1, Genie N. Dummy may freely use
public help and select one-shot; that choice is not failure. Genie follows
Search 1 + context Paragraph batch View 1 + Replace 4 + batch Check 1 + final
File batch View 1 = eight capability commands, not eight processes/tool turns.
Candidate context View must supply primary-selection Content with zero
Content-fetch self-View, named-binding-copy or raw-View workaround, zero
terminator mistakes/Wrong Apply and exact independent oracle. N-1's missing
Content is measured honestly, not repaired by modifying the comparator.

Record process count, actual bw commands, model tool turns, unexpected CLI
failures, extra Content-only commands, stdout/stderr bytes, model-visible bytes
and elapsed separately. Help/start/exit do not inflate the eight capability
commands. Increased useful Content bytes are not automatically a regression.
Each n=1 arm gives observations, not a mandatory ratio or broad speed claim.
Retain stale Safe Reject controls; one Search is not generalized to multiple
edits within a file. Do not execute <=0.2.5 comparisons, external-tool arms or
the user-installed 0.2.2 bw, and do not update that binary.

## Gate 1 executed and reused verification

- Read AGENTS/active authority, both planning notes and the identified production
  definitions/callers/tests. Read server AGENTS and installer source only; no
  server write, installer, publisher, endpoint or service command was run.
- Compared all 38 tracked non-Markdown Backwriter files byte-for-byte with
  pinned N-1. This includes production, tests and embedded fixtures, Cargo/lock,
  toolchain and release profile. There is no tracked build script or Cargo
  config. No fixture/test/build input is added or changed; the two new planning
  files are Markdown only and are not compiler/test inputs.
- `cargo metadata --offline --locked --format-version 1` succeeds: package
  `0.2.6`, no package feature definitions, existing library/bin/test targets.
  No build/test flags, target or profile are changed. Reuse the recorded GNU
  `x86_64-unknown-linux-gnu` and musl `x86_64-unknown-linux-musl` default-feature
  suites under pinned Rust/Cargo 1.95.0, the same Cargo test profile and recorded
  flags: **285 passed each**. This is retained evidence, not a fresh suite run
  or proof of an untested custom feature/flag combination.
- Run local Markdown-link, fence, conflict-marker and diff checks; inspect
  exact allowed paths and empty pre-stage index; confirm `.artext` is absent
  and untracked and no task-local output was created. Stage only approved
  documents after those checks and recheck the cached diff before commit.

This Gate has no suite, benchmark, artifact build, installed bw execution,
HOME change, publication, Actions, gh, tag, service or DNS action. README,
Rust/tests/Cargo/lock/toolchain, server and live state remain untouched. Full
final candidate verification belongs to Gate 5, not this input-equivalent
documentation change. Native macOS/Windows/PowerShell/CMD gaps and absent
lock/rollback/fsync/crash-durability guarantees remain explicit limitations.

At the Gate 1 boundary, next was Gate 2 only. Gates 3–5 require their preceding evidence and unresolved
decisions; separate future artifact/installer/publication authority is not
granted here.

## Gate 2 namespace implementation and focused evidence

Entry: clean `main = origin/main = f0379b0059a1c51be511742fd9f17cb21b61ac23`.
Only the existing `is_backwriter_spill` non-Windows and Windows branches change
production behavior. Exact root `.bw` joins legacy `.artext/bw`; descendants
require a slash component boundary. Windows compares only these components
ASCII-case-insensitively; non-Windows remains case-sensitive. There is no new
filter, constructor, registry, IO, store, fallback, migration or cleanup path.
Store creation/read/write/spill/cleanup remain N/A.

The Gate 1 consumer table was rechecked against each definition and caller.
All nine production call sites stay byte-identical: four Search, one View,
one Check, one Apply, two Anchor. Search scope preflight still precedes selected
private skipping; exact private File lookup is Empty. View relation validation
still precedes Unavailable. Check returns NotCurrent before proof/source access.
Edit validation precedes private Unavailable, with neither unit Apply nor
receipt Apply publishing. Anchor and invalidation retain existing errors and
leave the unrelated live Anchor/current proof intact. Update bootstrap and
Apply same-parent staging, names, domains and cleanup are unchanged.

| Focused GNU group | Passed | Direct evidence |
| --- | ---: | --- |
| Runtime | 1 | Existing Windows-only predicate test expanded to both platform case policies and exact components |
| Search | 6 | Existing range/private regression extended with absent/new-only/old-only/both sentinels, invalid UTF-8/NUL exclusion, ordinary siblings/nested roots, narrowed scope/admission, private file/symlink and direct View/Check/Anchor/Apply rejection; exact File and admission/no-follow controls |
| View | 5 | Private single/batch Unavailable, projection InvalidInput priority, all-or-none, symlink/nonregular safety, alternate admission |
| Check | 3 | Private NotCurrent in ordered duplicate batches under Untrusted and Host; ordinary Current/NotCurrent/Unavailable controls |
| Apply | 3 | Private unit/receipt rejection, NUL validation priority, unchanged bytes, live Anchor and staging contents; symlink and late-invalid-source controls |
| Anchor | 2 | Private Anchor and both invalidation seams reject without consuming sentinels or changing the ordinary live Anchor/proof; invalid source fail-close control |
| CLI | 5 | Exact help/version, four namespace noncreation/sentinel cases, workspace/admission, Update-help no-download and View error priority |

Total: **25 distinct focused GNU tests passed**. Existing functions/fixtures are
extended, not replaced with a new harness or crate; `tests/support.rs` is
unchanged. Verifier reads of task-local sentinels are not Runtime store IO.
The sole predicate unit now runs on GNU as well as Windows; no native Windows
or Windows-only test execution is claimed. Full GNU/musl suites remain Gate 5;
the earlier 285/285 counts are baseline only, not post-change test results.

Executed offline/locked metadata (full dependency graph), tree, fmt check,
GNU all-target check, clippy `-D warnings`, and release build with Rust/Cargo
1.95.0. Clippy initially found two unnecessary test clones; they were removed
and the final check passes. Default features, existing test/release profiles
and flags are unchanged, with `CARGO_TARGET_DIR` isolated under the task-local
temporary root. No toolchain was installed. Six asserted release smoke commands
cover ordered Search v2/v5 output, both private exact File Empty results,
`--admit x` retaining nested private-looking names, Help and exact
`Backwriter 0.2.6\n`; five fixture files remain byte-identical with no additions.
No user-installed `bw`, Update, benchmark, older comparator or artifact runs.

Input audit: of 38 tracked non-Markdown files, the seven allowed Rust/test
files change; the other 31 are byte-identical to entry. Core/v5 and its KAT
definitions, CLI, remaining Runtime, Cargo/lock/toolchain, support fixtures,
target definitions, features and profiles are unchanged. There is no tracked
build script or Cargo configuration. Changed inputs are tested above, not
covered by blanket reuse of an old suite. The Runtime file delta, including
its test, is **+654 bytes / +27 lines**; the six integration files total
**+12,438 bytes / +312 lines**. This is coverage expansion, not a performance
or size-contraction claim.

Both planning-note SHA-256 values above remain exact; README is unchanged.
Server stays clean at its pinned SHA with no Gate 2 server/live operation.
Document links/fences, allowed paths, empty pre-stage index, untracked output
and repository `.artext`/`.bw` absence are audited. Only this task's temporary
target and CLI fixture are removed before the approved commit/push.

Next: Gate 3 after D1/D2 closure; D3 remains required before Gate 5. Gate 2
does not implement complete shell View, module/help contraction, readiness,
new storage or a release. Official distribution remains closed `0.2.6`.

## Gate 3 D1/D2 proposals — awaiting Owner approval

Proposal only, inspected at clean `main = origin/main =
3f06c8b2ee37c094fef887d8e33bc7304cd27ef7`. Neither decision below is approved
API/format authority, implementation, a Gate 3 GO, or source readiness.
Gates 1–2 remain complete; Gates 3–5 and D3 remain pending. Cargo/CLI/public
remain `0.2.6`. The existing evidence and both planning notes are unchanged.

### D1 recommendation: extend the existing batch projection argument

Recommend replacing, not supplementing, the current batch signature with:

```rust
pub fn view_batch(
    &self,
    anddresses: &[Anddress],
    projection: Option<AnddressTarget>,
) -> Result<Vec<ViewOutcome>, ViewError>
```

Proposed meaning: `None` projects each input to its own `input.target()`;
`Some(kind)` retains the existing common upward projection. Empty input remains
`Ok(Vec::new())` for either form. The existing single `view(&Anddress,
AnddressTarget)` stays unchanged. This is a Rust source-breaking change:
existing callers must replace `view_batch(inputs, kind)` with
`view_batch(inputs, Some(kind))`, including function-pointer expectations.
There is no implicit conversion, compatibility overload/alias, wrapper, new
request type, second executor or claim that external Rust callers do not exist.
v5 wire, ViewOutcome and error variants do not change.

Evidence: `src/bin/bw.rs::execute_session_ref_view` currently uses batch only
with `--as`; omitted `--as` loops over `run_view`. The Runtime batch already
projects every input before source validation and groups the resulting targets
by source, not by target kind. `RangeCapture` consumes each projected range;
`finish_batch` restores input positions. No mixed-kind observation engine is
needed. Proposed flow:

1. Parse and resolve all references once, retaining borrowed input spellings.
2. Exactly one input: one `run_view`/single Runtime View, using explicit kind
   or that input's self kind, including the explicit `--as` case.
3. Multiple inputs: exactly one `view_batch(&inputs, projection)`. Delete the
   self-View loop; do not split by kind, reject mixed kinds or change projection.
4. Pass the optional argument through `runtime.rs` to the existing
   `runtime/view.rs::execute_batch` and `project_inputs`. Only at each
   `project_request` choose `projection.unwrap_or_else(|| input.target())`.
5. Reuse `validate_runtime_input`, source-key ordering, `batch_group_end`,
   `execute_batch_group`, direct/trusted capture, and `finish_batch` unchanged.

All projections are preflighted before admission/private checks and IO, so a
later downward InvalidInput still wins over an earlier unavailable source.
Absent projections remain indexed normal outcomes and do not cause source IO;
this does not add currentness checks to an absent relation. Preserve each
source's one direct observation in Untrusted/proof-miss mode, one retained
handle with requested range reads in matching Host mode, hash/length/Line-count
checks, proof invalidation rules, order, duplicate multiplicity and all-or-none
failure. A stale projected peer still fails the whole batch.

Direct production callers are only one-shot batch View and direct shell View
in `src/bin/bw.rs`; one-shot supplies `Some(kind)` and keeps its existing
required `--as`/JSON grammar and output. Raw named Session single View keeps
`run_view`. Known Rust test callers are `tests/view.rs`, the native batch
oracle inside `tests/cli.rs`, and a resource regression in
`src/runtime/view.rs`; existing explicit kinds become `Some(kind)`.
Approval would therefore minimally permit `src/runtime.rs`,
`src/runtime/view.rs`, `src/bin/bw.rs`, `tests/view.rs`, `tests/cli.rs`, and
targeted active-authority/tracker updates. No other production consumer was
found by repository-wide `rg view_batch`; external source compatibility remains
an explicit Owner cost, not a reason to add a parallel API.

### D2 recommendation: length-delimited Content with existing ref metadata

Recommend this one direct-shell-only presentation, with ASCII TAB/LF as shown:

```text
View<TAB><input REF><TAB>bytes=<N><LF>
<existing write_session_ref_line output for fresh REF, kind and location>
<exact N UTF-8 Content bytes><LF>EndView<LF>
```

`N = content.len()` in bytes, never Unicode scalar/character count. There is
no blank line between the existing metadata line's LF and the first Content
byte. Exactly one LF followed by `EndView` and one LF is display framing after
the N bytes; none of these bytes belongs to Content. Preserve all original
None/LF/CR/CRLF terminators. Do not scan Content for `EndView`, normalize it,
append a source terminator or build another encoded Content value. This is a
human display contract, not a new Core wire, JSON/Content schema or parser.

An absent item is exactly `View<TAB><input REF><TAB>RelationAbsent<LF>` at its
input position, with no fresh ref, kind, fabricated Content or end record.
Echo the once-resolved input token (`@N`, `@name`, `@name[index]`) without its
shell quoting. The existing valid reference grammar is ASCII and contains no
TAB/LF. Duplicate inputs each produce their own projected slot and record.

Path audit corrects a possible premise: `src/source.rs::validate_logical_path`
already rejects every `char::is_control`, including TAB, CR, LF and NUL, as
well as colon/backslash. There are no currently admitted control-character
paths to escape. Spaces inside components, quotes, Unicode and delimiter-like
names stay verbatim in the existing TAB-separated metadata line; they cannot
introduce a structural TAB/LF or location colon. Do not tighten or extend the
path grammar. Byte framing does not promise safe visual rendering of every
Unicode glyph or terminal control contained in Content. Any future allowance
of control-character paths would require a separate presentation review.

Proposed exact stdout KATs below use Rust string escape notation, not literal
backslashes on stdout. Each independent projected example starts with refs
length 3, input `@0` and fresh `@3`; named paths are illustrative existing
valid-v5 fixtures to use after approval, not new fixtures created in this pass.

| Content / case | Exact expected stdout |
| --- | --- |
| Empty File | `"View\t@0\tbytes=0\n@3\tFile\tnote.txt\n\nEndView\n"` |
| Line `x`, None | `"View\t@0\tbytes=1\n@3\tLine\tnote.txt:1\nx\nEndView\n"` |
| Line `x\n`, LF | `"View\t@0\tbytes=2\n@3\tLine\tnote.txt:1\nx\n\nEndView\n"` |
| Line `x\r`, CR | `"View\t@0\tbytes=2\n@3\tLine\tnote.txt:1\nx\r\nEndView\n"` |
| Line `x\r\n`, CRLF | `"View\t@0\tbytes=3\n@3\tLine\tnote.txt:1\nx\r\n\nEndView\n"` |
| Unicode Line `β\r\n` | `"View\t@0\tbytes=4\n@3\tLine\tnote.txt:1\nβ\r\n\nEndView\n"` |
| File `EndView\n` | `"View\t@0\tbytes=8\n@3\tFile\tnote.txt\nEndView\n\nEndView\n"` |
| File `x`, path with space | `"View\t@0\tbytes=1\n@3\tFile\tdir/a b.txt\nx\nEndView\n"` |
| Absent input `@1` | `"View\t@1\tRelationAbsent\n"` |

For mixed input `@0 @1 @0 --as paragraph`, where `@0` has Paragraph Content
`x\n` at `note.txt:1-1` and `@1` is a separator Line, initial refs length 3:

```text
"View\t@0\tbytes=2\n@3\tParagraph\tnote.txt:1-1\nx\n\nEndView\nView\t@1\tRelationAbsent\nView\t@0\tbytes=2\n@4\tParagraph\tnote.txt:1-1\nx\n\nEndView\n"
```

Proposed append/failure rule: acquire all Runtime outcomes first, count only
Projected items P, then call the existing `reserve_session_refs(refs, P)`
before any append/output. Keep a single outcomes collection; for single View
its one-element vector is fallibly reserved before the call. Consume outcomes
zipped with borrowed input spellings in input order. For each Projected item,
append its owned Anddress immediately BEFORE the first write of that record,
then write header, existing `write_session_ref_line(..., None, &refs[slot])`,
the borrowed bytes of its owned Content, and the end framing. Drop Content
after use. Direct indexing of the just-appended slot is not ref re-resolution.
An absent record appends nothing. Flush once after the last record. No Content
clone, second result/display collection, extra resolution/Search/View/read is
needed. Preserve append-only slots; do not undo or reuse appended slots.

Let L be entry refs length, P the total projected count and k the number of
projected records whose processing has begun (append already performed).
Lengths below describe RAM just before returning/error unwind; terminal shell
exit drops all refs, not persistent state. Stdout means bytes from this command.

| Boundary | Ref length | Possible stdout | Error / subsequent commands |
| --- | --- | --- | --- |
| Malformed or unresolved ref | L | Empty | Existing Usage, status 2 recorded; shell continues |
| Runtime failure, including stale/downward | L | Empty | Existing Execution mapping, status 1 recorded; shell continues |
| Input/outcome/ref reserve failure before output | L | Empty | Execution, status 1 recorded; shell continues |
| First logical write fails on first Projected record | L + 1 | Empty or a prefix accepted by the sink; no complete-delivery claim | Stream; immediate shell exit 1 |
| First logical write fails on first absent record | L | Empty or prefix | Stream; immediate shell exit 1 |
| Later/partial header, metadata, Content or end write fails | L + k, including current Projected item but not a current absent item | Prefix through the failing write; possibly earlier complete records | Stream; immediate shell exit 1; unvisited items are not appended |
| Final flush fails | L + P | Empty, partial, or even all bytes; delivery still unconfirmed | Stream; immediate shell exit 1 |
| Writes and explicit flush succeed | L + P | All ordered records | Success; shell continues |

BufWriter may defer the first underlying write until several records have been
appended; that sink failure belongs to the L+k row, not necessarily L+1.
To avoid an implicit retry by BufWriter Drop after a Stream error, the proposed
writer consumes it with `into_parts` and discards its unflushed buffer on error.
Already written bytes cannot be recalled. This is not rollback; no explicit or
Drop-driven retry is added, no previous Apply publication is cancelled, and
partial stdout never establishes successful delivery. The existing
`execute_shell` Stream arm already terminates rather than processing later
commands. Non-Stream command errors retain its highest-error accounting.

### Minimal approval-dependent implementation and focused acceptance

Remove the direct View any-RelationAbsent early return, Content discard and
plural single-View loop. `write_session_relation_absent` has only that direct
View caller; replace its narrow body/name with the complete per-item View
writer, not a retained obsolete writer plus a facade. Keep
`write_session_refs` for Search, `write_session_replace` for Replace and
`write_session_ref_line` unchanged for all actual callers. Reuse BufWriter and
CliError::Stream, with one narrow writer taking ordinary `Write` for tests;
no output trait/framework, production failure hook, module split or new crate.
Single and batch acquisition feed this same writer. One-shot/raw/JSON View,
Search, Replace and Check output bytes remain unchanged.

| Existing regression / location | Preserve and extend after approval |
| --- | --- |
| `tests/cli.rs::shell_local_references_start_at_zero_append_in_order_and_keep_named_raw_aliases` | Exact new direct-View bytes, input/fresh refs, duplicates, named alias and single/mixed-kind self; retain raw named View bytes; structural single-vs-batch call and no re-resolution/Content clone audit |
| `shell_local_view_relation_absent_and_search_failure_do_not_consume_reference_slots` | Replace old absent-only KAT with input-associated absent record; add mixed projected/absent/duplicate ordering and later-slot checks, not suppression of peers |
| `shell_local_references_reject_malformed_numeric_forms_before_runtime_access`, `shell_local_replace_failure_does_not_consume_a_fresh_slot` | Retain malformed/stale rejection; assert failed View adds no slots and later valid command still runs |
| Existing CLI Line terminator and one-shot/raw/JSON View regressions | Keep their bytes unchanged; integrate direct View empty, Unicode, None/LF/CR/CRLF, delimiter-like Content and space-path table cases without another fixture engine |
| `tests/view.rs::view_batch_preserves_empty_single_duplicate_and_mixed_source_order` | Add None with mixed File/Paragraph/Line self targets across same/different sources; explicit Some keeps old expected results |
| `view_batch_preserves_relations_terminators_unicode_and_raw_ranges`, `view_batch_preflights_relations_and_fails_all_for_unavailable_members` | Retain Some upward/absent/downward priorities, mixed absent/current and late failure; extend None identity/duplicate ranges |
| `host_view_batch_reuses_and_invalidates_proof_per_source_group` | Add mixed-kind self to matching/missing/mismatched Host proofs; preserve per-source invalidation and ordinary equivalence |
| `src/runtime/view.rs` existing OneByteReader, provisional-output, structural one-observation and resource regressions | Reuse for mixed targets and Some migration; no new observer or runtime counter |

The CLI integration suite currently has no deterministic in-process ref-vector
allocation/write/flush failure seam. Do not call a successful process KAT proof
of those cases. In the existing binary's test compilation, directly exercise
`reserve_session_refs` with a capacity-overflow request and the actual narrow
View writer with a test-only `Write` sink failing at header/body/end/flush
boundaries (including buffered first-sink-write and an absent first item).
Assert refs length, captured prefix and no flush-on-drop retry. Use existing
valid-v5 decoding and outcomes, not a copied writer or production injection flag.
Capacity-overflow covers the existing recoverable reserve error mapping;
structural ordering proves that real P reservation precedes output. It is not
process allocator-exhaustion proof. Retain the existing terminal Stream arm
and add an integration broken-pipe case only for terminal behavior, not as a
deterministic substitute for byte-boundary tests. A small `#[cfg(test)]` binary
test module is within the proposed `src/bin/bw.rs` change, not a new harness or
integration crate. No such code/tests/fixtures are added by this proposal.

After approval and implementation, run impacted GNU focused CLI/View/Host/
namespace regressions plus fmt and static checks. Full GNU/musl is Gate 5.
Gate 2's 25 focused tests and six smoke commands, and N-1's 285/285 baseline,
are historical evidence only, not fresh D1/D2 or full candidate passes.
This proposal runs document/link/fence/conflict/diff/path and unchanged-input
audits only: no product tests, build, benchmark, operational probe or installed
bw. Only this section is appended; index stays empty, with no commit/push.

Owner decisions requested separately:

1. Approve D1's source-breaking Option batch signature and the bounded caller/
   regression migration above, preserving the existing single seam.
2. Approve D2's exact direct-shell framing and per-record append-before-write,
   reserve-before-output, terminal Stream/no-retry contract and focused tests.

Until those approvals, D1/D2 remain awaiting Owner approval and Gate 3 remains
unimplemented, without a GO claim. D3 and Gates 4–5 are unaffected.

## Gate 3 implementation and focused verification — complete

Entry: `main`, HEAD = origin/main =
`3f06c8b2ee37c094fef887d8e33bc7304cd27ef7`, empty index, with only the
224-line D1/D2 proposal appended above. That proposal is preserved verbatim as
pre-approval evidence. The Owner subsequently explicitly approved D1/D2's
source-breaking API/CLI implementation and allowed-path commit/non-force push.
Approval preceded production mutation; this section records implementation and
verification, not a retroactive implementation claim for the proposal.

| Consumer | Gate 3 decision / evidence |
| --- | --- |
| Runtime batch API and `project_inputs` | Option selects self or common upward kind at the sole `project_request` call; explicit callers migrate to Some. No new type, error, overload or executor; external Rust source compatibility intentionally breaks |
| Direct shell single/plural acquisition | Resolve each input once; reserve singleton result before one single View, otherwise invoke one batch with optional projection. Delete repeated self loop; retain grouping, capture, finish and currentness/Host proof |
| Direct shell presentation | Replace absent-only writer and any-absent suppression with one consuming View writer; count/reserve P before append/output; append owned Anddress before each projected record; write exact Content and discard it |
| Retained writers | Search consumes `write_session_refs`, Replace consumes `write_session_replace`, and all three use the unchanged `write_session_ref_line`. One-shot human/raw/JSON and named Session writers remain their distinct consumers |
| Output failure | Real Write sink exercises every byte boundary with unbuffered and buffered writes, absent-first, final flush, prefixes and L+k/L+P. `into_parts` prevents Drop retry; broken pipe exits before a later Replace |
| Observation evidence | Existing OneByteReader covers mixed File/Paragraph/Line in one forward observation and all-provisional late failure. Structural call audits retain single=1/batch=1, one Untrusted observation or matching Host handle, no Content-only Search/View/re-resolution |

Exactly **116 distinct focused GNU tests passed** after the final Rust edits:
3 binary writer/reservation tests, 74 CLI integration tests, 21 View integration
tests, 13 Runtime View unit tests, and 5 namespace controls (Runtime predicate,
Search private/sibling fixture, Host Check private/admission, Apply unavailable
path, Anchor path-exact invalidation). The existing CLI crate is tested as one
impacted component, not a full GNU product suite. No test/helper file is added;
binary cfg(test) reuses unchanged `tests/support.rs` valid-v5 fixtures. Four
CLI tests and three binary tests are new; existing batch/Host/KAT tests extend
their original evidence.

Covered: mixed-kind None including empty/single/duplicates and multiple sources;
Some upward/absent/downward priority; Host match/mismatch/miss and failure
invalidation; late source failure without output or slots; exact empty/Unicode,
LF/CR/CRLF/None, delimiter-like Content, spaces/quotes in safe metadata; named
and indexed refs; absent peers and subsequent slot numbering; stale and malformed
refs with continuation; raw/JSON/one-shot/Replace/Check bytes. Ref reservation
overflow proves recoverable Execution mapping and length preservation, not
process allocator exhaustion. Production ordering proves P is reserved before
output. No production hook, copied writer, new observer or allocation claim.

Verification corrected new test fixture coordinates and obsolete direct-View
expectations, moved the binary test module to satisfy clippy, and enlarged the
broken-pipe fixture beyond pipe buffering after a concurrent child descriptor
window exposed the initial small-output test's weakness. Final focused tests
pass; no behavior was relaxed to suppress those failures.

Rust/Cargo 1.95.0, default features and existing profiles: full offline/locked
metadata and dependency tree, fmt, GNU all-target check, clippy `-D warnings`,
and release build pass. Nine asserted task-local release commands cover raw
Session Apply fixture setup, Help/Version, Search v2/v5, raw View, Check,
mixed projected/absent/duplicate direct View, mixed-kind self View, and unchanged
named raw View. Each has exact expected stdout, empty stderr and exit 0. The
final fixture is exact `x\r\n \t\n`, SHA-256
`1f55a163eac1472bc23893cf93791df0d0e5434017aebed04d3f15803b29a7cf`.

No full GNU/musl, AI arm, benchmark, historical comparator or native Windows
run is claimed. The 285/285 baseline remains historical. Cargo/CLI/public
stay `0.2.6`; Gate 4 help/private modules/verification contraction and Gate 5
integration/readiness, including D3, remain pending. Existing help constants
are intentionally unchanged until Gate 4. No server, installed bw, actual HOME,
public tree, service or Cloudflare operation occurs. Planning notes, README,
Cargo/lock/toolchain/dependencies and all non-allowlisted tracked inputs remain
byte-identical. Document/Git hygiene and exact-path temporary cleanup precede
the approved commit and non-force push; no artifact or release is produced.

## Gate 4 Help, private modules and verification contraction — complete

Entry: clean `main = origin/main =
3d35f14338d2374777acd485d0bce49387800fbc`. The source note, grounded roadmap
and 224-line pre-approval D1/D2 proposal remain byte-identical. This gate changes
Help and physical organization only; it does not reopen D1/D2, v5, Core/Runtime,
schemas, namespace, version, readiness or operational authority.

### Consumer and deletion decisions

| Responsibility / actual consumer | Decision |
| --- | --- |
| Top dispatch, one-shot argument validation, shared Search/Replace preparation, Runtime calls, Update bootstrap | Keep in `src/bin/bw.rs`; no second executor/parser, Update or HOME behavior change |
| `write_command_help`, constants and `canonical_usage` consumed by dispatch/error reporting | Move to private `bw/help.rs`; reuse usage extraction, add only four raw Session Help topics; remove empty Version sections and the uniform ten-section assertion |
| CLI error construction, promotion, stream/exit reporting consumed by all command paths | Move existing bodies to private `bw/error.rs`; retain error codes/priority and status meanings |
| Direct and raw Session parsing, refs, bindings, Pick composition and Data lifetime | Move to private `bw/shell.rs`; preserve actual advanced consumers and shared one-shot helpers |
| Human/JSON/raw and direct-ref output, Data display, batch reports | Move to private `bw/output.rs`; keep distinct native-result consumers, one canonical address encoder and Gate 3 consuming writer/failure tests |
| Test organization | Keep one CLI integration crate and its shared fixture/support; move only responsibility groups into child modules; no helper/framework or integration binary added |

All 117 top-level production function definitions survive, including platform
alternatives. Excluding signature visibility/wrapping, 116 bodies are exact;
only `write_command_help` changes to accept advanced topics. No non-Help
executor/writer body changes. Module imports and sibling visibility are private
and explicit. Error methods retain their bodies. Production byte equality with
N-1 or the Gate 3 parent is not claimed; module moves do not imply speed gains.
README's inherited 0.2.5 Gate 6 reference is corrected to the actual 0.2.6
Gate 6 `c78e07f242035230e8b071d583491ac633f58d29`: its `src/**` diff against
published Source Authority `09bb6c424081594bd86a95f04345b786ef9b46b6` is empty.

Top Help now introduces shell as ordinary short-reference work. Shell Help
covers all direct commands, numeric/named/indexed refs, quoting/escapes,
Content framing and fresh refs, body-only Line Replace, Current-only Check
slots, same-source staleness and terminal stream failure. Four advanced topics
derive their operands/bindings/output/failures from the unchanged raw parsers.
Their one-shot execution, including `<capability> --help`, stays rejected;
only `bw help <topic>` is added. Existing one-shot Help forms are unchanged.

### Test inventory and focused evidence

| Location after move | Existing test functions |
| --- | ---: |
| `tests/cli.rs` (shared fixtures plus Search/global/Update controls) | 12 |
| `tests/cli/help.rs` | 5 |
| `tests/cli/edit.rs` | 10 |
| `tests/cli/view.rs` | 9 |
| `tests/cli/check.rs` | 6 |
| `tests/cli/shell.rs` | 32 |
| `src/bin/bw/output.rs::view_output_tests` | 3 |

CLI inventory is exactly the same 74 names before and after movement; module
qualification alone changes their displayed paths. Help cases and five complete
direct/advanced example rows extend existing functions. The removed section-order
helper asserted a retired presentation constraint, not a failure boundary.
Independent test-owned Help KATs do not include production constants.
Structural source assertions now read the actual entry/output/shell locations.

Final focused GNU evidence is **116 distinct passed**: CLI 74, binary 3,
View integration 21, Runtime View 13, namespace controls 5. The latter are the
existing spill-boundary unit, Search private/sibling fixture, Host Check
private/admission, Apply unavailable path, and Anchor path-exact invalidation.
Gate 3's reserve/append/framing, every byte-boundary/flush failure, no Drop retry,
single=1/batch=1 and terminal broken pipe all pass. No new observer, counter,
hook, Content clone, parser or result collection is introduced.

Full offline/locked metadata/tree, fmt, GNU all-target check, clippy
`-D warnings` and release build pass with Rust/Cargo 1.95.0, default features
and existing profiles in task-local targets. Initial relocation validation
found one stale include path and unnecessary imports; both were corrected
without weakening their assertions. Final tests/checks pass. This is not a
whole GNU product-suite pass; full GNU/musl, AI arms, D3 and benchmarks remain
Gate 5 work. Previous 285/285 and Gate 3 116/9 are baseline evidence, not fresh
full-suite claims.

### Release smoke and unchanged execution

The clean Gate 3 parent and current candidate are built offline/locked in
separate task-local targets. A common workspace is reset to exact
`needle\r\n` before each comparison; neither installed bw nor actual HOME is
used. **33 process invocations** pass: 12 candidate Help topics, one Version,
five parent/candidate shell example pairs and five one-shot command pairs.
All examples include their ref producers. There is no timing comparison.

The direct example obtains @0 from Search, @1 from Paragraph View, aliases @0,
replaces through that alias to @2, checks old @0 as NotCurrent and @2 as
Current/@3, then reads @3 as @4. Exact final Content is `new value\r\n`.
Pick, Anchor, raw Apply and Data examples match documented commands and
byte-exact expected output. Parent/candidate stdout, stderr, exit and final
source bytes are equal for all five flows.

The one-shot pair uses original Search-v2/v5 embedded object bytes without
reencoding, raw View, Current Check, JSON receipt Edit and stale nonpublication.
Both end with `replacement\r\n`; stale Apply exits 1 with empty stdout and
exact `error: current source is unavailable\n`. Help/Version create no private
state. Usage Help text intentionally changes while codes/status meanings stay
fixed. Release binary hashes are identity evidence, not artifact authority:

- Parent: `f0ecce7326d745b9286c4c6ce86dcae1241dba5116b3f7e1f48cf6bf213e295d`.
- Candidate: `72ef3e058061ca97eb290ddfc5031bdfae2d7c92c2a5be1e6310482ef2da63d0`.
- Task-local smoke driver: `9e1569362ac9234f8160dfcd697cf3ff44d073c5ae1f8b7658ebc921c6deea47`.

### Verification evidence preservation

Current safety/target/reuse/hygiene rules were extracted before removal:
v5/count currentness, no-follow/text/resource fail-close, observation and
all-or-none controls, receipt/proof/Anchor/publication boundaries, exact output
and streaming, full stable-candidate target commands and bounded N/N-1 comparison.
Repeated release/gate narratives and detailed measurements link to their existing
trackers via the [history index](../history/index.md), not a second full archive.

The original verification file is pinned above at 96,993 bytes/1,487 lines,
SHA-256 `7bd2851b1add8756590ab4f0888b2d6e4c0e5f78eaacaffd7afd0a60c2bb9666`.
The [verbatim excerpts](../history/2026-09-05-verification-before-0.3.0.md)
preserve only evidence units with additional details, plus their interpreting
conditions. Each original line span equals the stored excerpt bytes exactly:

| Original verification lines | Preserved additional evidence |
| --- | --- |
| 155–161 | 0.2.5 endpoint/install closure detail with gate context |
| 519–537 | 0.2.4 raw Session rchar and paired terminator evidence |
| 592–611, 632–645, 647–666 | Patch Box scratch-boundary and intermediate 247-test details |
| 740–749 | Exact unrounded Patch Box elapsed values and conditions |
| 908–915 | 0.2.2 endpoint/error/cache and isolated install details |
| 1059–1080 | Phase 7A exact baseline values and follow-up conditions |
| 1116–1210, 1239–1257 | Detailed historical raw/streaming/capability regression inventory |
| 1284–1386 | Beta.3/stable/0.2.0/0.2.1 release verification details |
| 1404–1471 | Historical CLI/Session regression inventory |

All 12 excerpt units pass byte identity. All 273 distinct original long
digests/revisions and multi-decimal measurement values remain present in the
linked/preserved documents. Existing task evidence is not rewritten or rerun;
old v3/v4 paths and contracts are explicitly historical, not current authority.
Link/anchor/fence/conflict-marker audits pass. Current verification keeps only
policy, candidate/N-1, required matrix, this gate's evidence and three evidence
links; no readiness SHA or final suite count is invented.

### Exact sizes and boundary

| Scope | Before lines / bytes | After lines / bytes | Delta |
| --- | ---: | ---: | ---: |
| CLI entry file | 3,437 / 132,293 | 962 / 33,746 | -2,475 / -98,547 |
| CLI entry plus four private modules | 3,437 / 132,293 | 3,917 / 142,109 | +480 / +9,816 |
| CLI test entry | 3,905 / 140,960 | 1,139 / 38,240 | -2,766 / -102,720 |
| CLI test crate including five child modules | 3,905 / 140,960 | 4,129 / 151,117 | +224 / +10,157 |
| Active verification | 1,487 / 96,993 | 141 / 9,790 | -1,346 / -87,203 |
| Unique historical excerpts | absent | 455 / 29,650 | preservation, not active growth |
| Historical index | absent | 18 / 1,413 | existing evidence discovery |

Help is now readable multiline Rust text, not line-golfed escaped constants.
Added bytes document actual commands and independent KATs; reduced entry files
reflect navigation, not total-code or performance contraction.

The 49 tracked paths outside the approved set remain byte-identical to entry,
including Core/Runtime/v5, Cargo/lock/toolchain and both planning notes. The
proposal remains exact. No server, live/public/service/Cloudflare/credential,
installed executable or actual HOME access/change is part of this gate.
Only inspected task-owned exports, targets, scripts and fixtures are removed.
Exact allowed-path staging and cached review precede one non-force push.
Gate 5/D3, native-platform gaps and separately authorized release remain open.
