# Backwriter 0.3.0 independent namespace and complete shell View

## Status — Gates 1–4 complete; Gate 5 evaluated, readiness NO-GO

Recorded 2026-09-05. Gate 1 closes authority, audited discrepancies, open
decisions and acceptance. Gate 2 implements the existing namespace predicate
and focused regressions only. Gate 3 implements Owner-approved D1/D2 after
the preserved proposal below; its focused verification is recorded separately.
Gate 4 completes Help, private CLI/test modules and evidence contraction.
Gate 5 runs the final 293/293 GNU/musl suites and all four independent AI arms.
Its NO-GO and exact evidence gaps are recorded below; this is not a source-ready
or release claim, and no production correction is made in this gate.
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
| 5 — integration/readiness | Evaluated, NO-GO | 293/293 full GNU/musl tests and 52 release smoke invocations pass; four fresh AI arms match the independent oracle, but required capture evidence is incomplete and Genie N uses nine rather than eight capability commands; no version promotion or release |

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

## Gate 5 D3 and final readiness — NO-GO

Recorded 2026-09-05 against clean `main = origin/main =
4c3038505b8f87d01483b60abd3ec7494d0bc94d`. This evaluation changes only
documentation. Production `src/**`, tests, Cargo/lock, build/toolchain inputs
and the version KAT remain byte-identical to that candidate. The verdict is
**NO-GO**, not a claim that native product semantics failed.

The final native matrix passes, and all sixteen final arm files match the
independent oracle. Mandatory comparison evidence is nevertheless incomplete:
three PTY arms merged stdout/stderr, the first Dummy N-1 Help also lacks
separate streams, complete whole-run elapsed/model-visible byte counts were
not captured, and Genie N added one pre-edit File View (nine rather than the
eight reference capability commands). These gaps are not zero values. No
coaching, best-of rerun, production fix, weakened criterion or version promotion
is substituted. Cargo/CLI remain `0.2.6`; public distribution and Update remain
closed `0.2.6`. Update was not run. A separate decision is needed before any
new evaluation or release work; this record creates no release authority.

### Contraction and evidence origin

Reuse: the existing native suites, shell, direct View writer, receipt path,
source-grouped View/Check and release executable. No test/helper/framework or
production code was added to the repository. Task-local exports, four isolated
workspaces and an independent smoke script served only this evaluation.
Store creation/read/write/spill/cleanup are N/A because no production consumer
exists. Retained raw Session remains the exact-range/lifetime consumer, not a
fallback hidden in the candidate's direct View writer.

Repository `rg` searches for four-file, unit-01, primary/secondary, oracle and
fixture references found the older three-file control in the 0.2.6 tracker
and the unresolved BOX 25 reference, but no original four-file bytes/oracle.
`git ls-tree` of docs/tests also found no corresponding fixture. This is a
declared **replacement fixture**, not proof of identity with the missing one.
No host-local historical fixture was inspected. N-1 is solely
`09bb6c424081594bd86a95f04345b786ef9b46b6`; no older comparison was executed.

### Pre-authored independent full-byte oracle

Before any AI run, the following two literal maps were fixed independently of
candidate output. Each arm received only the initial map in its own workspace.
File names are ordered unit-01 through unit-04. JSON string escapes below
represent exact UTF-8 file bytes; no implicit final LF is added to a value.

```json
{"initial":{"unit-01.txt":"role = primary\nretry_budget = 3\n\nrole = secondary\nretry_budget = 3\n","unit-02.txt":"role = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 3\r","unit-03.txt":"role = primary\r\nretry_budget = 3\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n","unit-04.txt":"role = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 3"},"expected":{"unit-01.txt":"role = primary\nretry_budget = 5\n\nrole = secondary\nretry_budget = 3\n","unit-02.txt":"role = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 5\r","unit-03.txt":"role = primary\r\nretry_budget = 5\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n","unit-04.txt":"role = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 5"}}
```

Serialization of each map is compact `JSON.stringify(map) + "\\n"`, in that
insertion order, with no BOM. Initial and expected serializations are each
366 bytes. Initial SHA-256:
`f7fa969215979bb27ae3a983c9c44f9673bc2f9573a2bda39839639cb6da8f75`.
Expected SHA-256:
`e1a729c3b3a8c73a730f92289f8b6d34e3681bdc018cacb4229503b747249bc2`.
The full initial/expected fixture JSON above, compact plus LF, is 755 bytes;
SHA-256 `26e2853962e5611c640c4d5c15f25ca68988a112600bce69f350d58497075998`.

| File | Bytes, initial = final | Initial SHA-256 | Expected/final SHA-256 in all four arms |
| --- | ---: | --- | --- |
| unit-01.txt | 67 | d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa | bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0 |
| unit-02.txt | 67 | f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389 | 6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a |
| unit-03.txt | 72 | 0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6 | 8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0 |
| unit-04.txt | 66 | 4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6 | 72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe |

After every arm exited, the parent compared exact bytes and the complete
workspace name set against the pre-authored expected map. All sixteen files
passed, with no extra workspace entry. Four primary changes, four secondary
preservations, newline mistakes 0 and Wrong Apply 0 hold in each arm. This
parent oracle check is not an external-tool comparison arm or an AI body read.
The N first Paragraph View supplied all decision-making Content. Neither N
arm used self-View, named-copy or raw-View to recover missing Content.

### Independent arm inputs and conditions

All four agents were spawned with `fork_turns="none"`, identical inherited
model/reasoning settings (no override) and the same enabled tool surface.
No implementation conversation, source, private tracker, oracle or sibling
result was supplied. The tools do not expose an independently pinned model
build identifier in these records; inheritance is the recorded condition,
not a claim of an externally reproducible model seed. With three child slots,
Dummy N-1, Dummy N and Genie N-1 started first; Genie N started once a slot
freed. Each was one actual AI run, not a scripted target-selection arm.

Common task instruction: discover the eight exact `retry_budget = 3` Lines
in four files; read surrounding Paragraph Content to select the four whose
Paragraph says `role = primary`; replace only those bodies with
`retry_budget = 5`; preserve secondary Paragraphs and every other
byte/terminator; confirm currentness and inspect final Files with bw.

Common restrictions: no source/AGENTS/private docs/oracle/sibling arm access;
all workspace body reads and edits through the supplied bw only, not filesystem
readers/writers, grep/cat/sed/Git; no installed bw, Update or unrelated directory
work. Existing shell/Node could invoke bw, preserve/parse output and interact
with stdin, but not decide targets. Record actual argv/stdin/output/exit,
process/command/tool counts, extra Content queries and elapsed, preserving
every failure without reset or best-of restart. Save an English transcript
with apply_patch. PTY echo/ONLCR disabling was supplied as transport advice,
with explicit warning to record merged-stream limitations or separately
redirect stderr; no answer command sequence was supplied.

| Arm | Public discovery input / instruction | Binary and isolated workspace under task root |
| --- | --- | --- |
| Dummy N-1 | Public Help only; choose own one-shot or shell route | nminus-target/release/bw; dummy-old/workspace |
| Dummy N | Public Help only; choose own one-shot or shell route | gnu-target/release/bw; dummy-new/workspace |
| Genie N-1 | Public Help and exact pinned N-1 README copy; derive the documented shell route, no supplied answer sequence | nminus-target/release/bw; genie-old/workspace |
| Genie N | Public Help and exact candidate README copy; derive the documented shell route, no supplied answer sequence | gnu-target/release/bw; genie-new/workspace |

Task root was `/tmp/backwriter-gate5.74tHth`. The Genie README copies were
named `public-guide.md` in each arm root, outside its workspace. Their
historical release wording was not rewritten for the comparison. No raw
8-command answer was injected; the reference is assessed after the runs.

### Actual comparison, without hidden normalization

| Measure | Dummy N-1 | Dummy N | Genie N-1 | Genie N |
| --- | ---: | ---: | ---: | ---: |
| bw processes | 13 | 3 | 3 | 3 |
| Help commands | 5 | 2 | 2 | 2 |
| Search | 1 | 1 | 2 | 1 |
| View commands | 2 | 2 | 14 | 3 |
| Edit/Replace commands | 4 | 4 | 4 | 4 |
| Check commands | 1 | 1 | 2 | 1 |
| Capability commands total | 8 | 8 | 22 | 9 |
| Explicit raw Apply | 0 | 0 | 0 | 0 |
| Named-copy let / exit | 0 / 0 | 0 / 1 | 12 / 1 | 0 / 1 |
| Extra Content-only calls beyond context + final inspection | 0 | 0 | 12 | 1 |
| Model functions.exec turns, including transcript save | 7 | 8 | 11 | 10 |
| Unexpected CLI failure / reset / retry | 0 / 0 / 0 | 0 / 0 / 0 | 0 / 0 / 0 | 0 / 0 / 0 |
| Final file oracle | exact | exact | exact | exact |
| Reported partial elapsed, not whole run | >=35.744 s | 26.162 s interactive | 40.459 s interactive | about 43 s after initial docs |

The Dummy N transcript calls its two required Views “extra”; the normalized
extra-call row above counts only work beyond required context/final inspection,
so it is zero. Genie N's additional baseline File View is an actual extra call,
not removed to fit eight. Genie N-1's projected direct View omitted Content,
so it made twelve named-copy/raw-View pairs. Its old secondary refs became
NotCurrent after same-source edits, an expected currentness contract rather
than a product error; the agent's surprise, fresh Search and extra Check remain
in the transcript. Dummy N-1 legitimately chose one-shot JSON and needed no
extra Content fetch. The shell lifecycle and Help are not capability work.
Internal Runtime invocation counts were not instrumented by these AI arms.

Decoded archived output payload counts (UTF-8, no JSON-escape overhead):

| Capture | Dummy N-1 | Dummy N | Genie N-1 | Genie N |
| --- | ---: | ---: | ---: | ---: |
| Product output payload bytes, including Help | 20,437 | 5,047 | 3,745 | 5,507 |
| All archived output payload bytes, including repeated guide reads | 20,437 | 5,047 | 37,393 | 40,555 |
| Independently captured stdout bytes | 19,474 after first Help | unavailable | unavailable | unavailable |
| Independently captured stderr bytes | 0 after first Help | unavailable | unavailable | unavailable |

The first Dummy N-1 Help's 963 bytes are merged, despite its raw transcript's
stdout/stderr field names; its captureNote controls that interpretation.
PTY arms preserve merged bytes, not a fabricated zero-stderr measurement.
All product processes finally exited 0; interactive records do not expose an
OS exit per shell command. The complete model-visible tool-envelope bytes and
full-run times are unavailable. Archived payload totals are reproducible
lower-scope counts, **not** replacements for those missing measurements.
Initial Genie guide output was reconstructed by the agent from its exact
second public-guide read; both reads are counted. These limitations themselves
prevent GO. No n=1 latency ratio, benchmark score or broad speed claim is made.

### Final native verification, executed on N

Rust `1.95.0 (59807616e 2026-04-14)`, Cargo
`1.95.0 (f2d3ce0bd 2026-03-21)`, rustc LLVM `22.1.2`; GNU host
`x86_64-unknown-linux-gnu`, installed `x86_64-unknown-linux-musl`
with unix/crt-static cfg. Default features (no feature override), checked-in
test/dev profiles; release panic=abort, strip=true, lto=true,
codegen-units=1. No new compiler-option override or dependency. All candidate
target output went into separate task-local GNU/musl directories.

Executed offline/locked metadata and dependency tree, fmt --check, all-target
check, full tests, all-target clippy with `-D warnings`, and release build
for GNU and musl. Initial full test output was terminal-truncated; both full
test commands were repeated to retain complete logs, not counted as distinct
test cases. Final per-target counts: library 72, binary 3, Anchor 31, Apply 35,
Check 13, CLI 74, Data 11, Edit 6, Pick 7, Search 20, support 0, View 21,
doctests 0: **293 passed, 0 failed, 0 ignored** on each target.

Existing regressions directly cover v5 KAT/strict errors/no older decoder,
namespace/safety, ordered Search/View/Check and duplicates, D1 mixed-kind
source grouping, D2 every writer byte boundary/flush/terminal Stream/no Drop
retry and slot reservation overflow, broken stdout, EOF Content/terminators,
three receipts/no-op, raw five Edit/four Position, Host hit/miss/invalidation
and Anchor same-after reflection. Existing duplicate drift asserts
Correct 1 / Safe Reject 6 / Wrong Apply 0 for both Untrusted and guarded Host;
duplicate Paragraph and stale prepublication controls also pass. No historical
performance result or earlier focused run is relabeled as this full suite.

The task-local release smoke ran **26 invocations per target, 52 total**:
top/named Help equality and all eleven topics, exact Version
`Backwriter 0.2.6\\n`, raw Session Insert/Apply, mixed File/Line/named/indexed
self View, Paragraph absent/duplicate framing, ordered Check, Search v2/v5,
raw View, JSON batch View, stdin JSON receipt followed by fresh View, stale
nonpublishing rejection, namespace exact-File Empty, and exact final CRLF
bytes. Output streams were separately captured in this parent smoke, unlike
the AI PTYs. This script was not supplied to agents. Node child spawning hit
sandbox EPERM and was rerun with approved execution permission; a later
read-only Node-to-Git hash audit also hit EPERM and used a plain Git pipe.
Neither is a CLI failure or concealed agent retry.

| Identity / raw evidence | SHA-256 or Git object |
| --- | --- |
| Gate 4 src Git tree | 8427811aaae0763aac22e774f01244c246c9d4ee |
| src framed SHA-256, 27 paths | 07cac94f1651db40a6bb6b7e64029740471f18e3c4ab626e5db3b46c7709e386 |
| N GNU release bw | 72ef3e058061ca97eb290ddfc5031bdfae2d7c92c2a5be1e6310482ef2da63d0 |
| N musl release bw | 1ea57f0c1d4c8a8e6e080b6a38bd7840c870e32aa038623660f2b03cfdd3d997 |
| N-1 GNU release bw | 5abd626ad17bb2847f65047bb744cdcb1f28f3b6a2a58691242a39d548f77559 |
| Full GNU test log | 608d9987a195b84282d872baa4588cfc1f2e880dbe0455570614d84c54d8fac1 |
| Full musl test log | e60fdb42d701b62fba8fba9dd15487b48688bace8e84bca81873233c871be6ed |
| Parent smoke script | 0296f2f1f25b055d618d376668e2bcff8a0f62c1c9d5245392cb36645cd0a9e0 |
| Parent exact smoke result JSON | 60a661343f135da62d24665b8a73564f397dbfefe16d20890a880d96ec914049 |

Source hash framing is each sorted `git ls-files -z src` path, NUL, exact
file bytes, NUL, concatenated then SHA-256. N-1 was built from its clean Git
export; its entire old suite was not rerun. N was tested from the clean pinned
checkout. No version change occurred, so there is no new-version binary/KAT
reuse claim. Documentation-only final edits preserve every test/build input.
Native macOS, Windows, PowerShell and CMD were not executed. No installer,
publisher, service, server, public endpoint, real HOME or installed bw was
accessed by this evaluation. No release or updater operation was performed.

### Unedited arm transcripts

The following fenced contents preserve the original agent transcripts,
including imprecise terminology and capture limitations; the normalized
assessment above does not silently rewrite them. Each SHA hashes the exact
UTF-8 transcript including its original terminal LF(s), excluding the fence.
Tool-wrapper metadata not captured there remains unavailable. Task-local
paths in these records are historical evidence and are removed after archival.

| Transcript | Bytes | SHA-256 |
| --- | ---: | --- |
| dummy-old | 55,824 | 470f0da66b7badbb3f4193041bdda4ffef17a0b2d6f535130786758183489da4 |
| dummy-new | 8,878 | 1f592454ded40d32b38a8d64c50c95a619de5ee99be7c9166c29667537786855 |
| genie-old | 43,638 | 8ee69ce4ccee59944fbe63b6956963fc743e97ffddb08cf1bb2a4509a2e62ac5 |
| genie-new | 48,806 | f62b653c26ed21d79a118883dea7abae247234109cb05fa308185eed9c8141ff |

#### D3 dummy-old transcript

````text
# Independent public-Help-only CLI run: dummy-old

## Result and decisions

One independent run, no restart. Public Help chose the one-shot route. Search returned eight Lines in four files. A batch Paragraph View showed primary roles at unit-01.txt line 2, unit-02.txt line 5, unit-03.txt line 2, and unit-04.txt line 5. Only after reading that Content, I explicitly selected search result positions 0, 3, 4, 7. Scripts only invoked bw, preserved and parsed its outputs; they did not decide targets. Four Line-body replacements returned changed. The four fresh receipt Anddresses checked current. Final batch File View showed four primary budgets of 5 and four secondary budgets of 3, with LF, CR, CRLF, and final missing terminator represented as expected. No fixture filesystem reads/writes outside bw were performed. No oracle, repository, sibling arm, installed bw, or updater was inspected or invoked.

## Metrics

- bw processes: 13 (5 Help; Search 1; View 2; Edit 4; Check 1; Shell 0).
- Capability-bearing CLI processes: 8. View operands: 8 Paragraph projections plus 4 File projections; Check operands: 4; Edit targets: 4.
- Extra Content-only queries beyond required discovery context and final inspection: 0. Content-bearing View commands total: 2 (12 operands).
- Failures: 0; retries: 0; restarts: 0.
- Execution tool calls: 6 exec_command calls in 6 functions.exec calls. Transcript save adds 1 apply_patch in 1 functions.exec call (7 functions.exec calls total through save).
- Helper Node processes: 5. Six command submissions use the execution service's zsh launcher; launcher exec replacement/internal descendants were not instrumented. Count of explicitly launched workload processes: 18 = 13 bw + 5 Node.
- Measured elapsed: 35744 ms, from just after initial --help capture to final Check/File View capture. Initial help and agent setup time were not clocked, so full-run wall time is unavailable; this value is a lower bound, not falsely precise full elapsed.
- Output strings below are exact JSON-escaped UTF-8 captured strings including final newlines. Initial direct Help capture has the documented stream-attribution limitation; all remaining stdout/stderr were captured separately. No PTY was used. All bw stdin inputs are empty; replacement Content was passed as an argv operand.

## Shell submissions

All submissions used cwd `/tmp/backwriter-gate5.74tHth/dummy-old/workspace`. Default execution shell: zsh. Every shell submission exited 0.

### Submission 1

```json
"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw --help"
```

### Submission 2

```json
"node -e 'const {spawnSync}=require(\"node:child_process\");for(const c of [\"search\",\"view\",\"edit\",\"check\"]){const argv=[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"help\",c];const r=spawnSync(argv[0],argv.slice(1),{encoding:\"utf8\"});console.log(JSON.stringify({argv,stdin:\"\",stdout:r.stdout,stderr:r.stderr,exit:r.status}));}'"
```

### Submission 3

```json
"node -e 'const {spawnSync}=require(\"node:child_process\");const argv=[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"search\",\"line\",\"retry_budget = 3\"];const r=spawnSync(argv[0],argv.slice(1),{encoding:\"utf8\"});console.log(JSON.stringify({argv,stdin:\"\",stdout:r.stdout,stderr:r.stderr,exit:r.status}));'"
```

### Submission 4

```json
"node -e 'const {spawnSync}=require(\"node:child_process\");const argv=[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"view\",\"anddress\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-01.txt\\\",\\\"sourceStateHash\\\":\\\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"15\\\",\\\"byteEnd\\\":\\\"32\\\",\\\"terminator\\\":\\\"lf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"32\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-01.txt\\\",\\\"sourceStateHash\\\":\\\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"67\\\",\\\"terminator\\\":\\\"lf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"33\\\",\\\"parentByteEnd\\\":\\\"67\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-02.txt\\\",\\\"sourceStateHash\\\":\\\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"17\\\",\\\"byteEnd\\\":\\\"34\\\",\\\"terminator\\\":\\\"cr\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"34\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-02.txt\\\",\\\"sourceStateHash\\\":\\\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"67\\\",\\\"terminator\\\":\\\"cr\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"67\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-03.txt\\\",\\\"sourceStateHash\\\":\\\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\\\",\\\"sourceByteLength\\\":\\\"72\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"16\\\",\\\"byteEnd\\\":\\\"34\\\",\\\"terminator\\\":\\\"crlf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"34\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-03.txt\\\",\\\"sourceStateHash\\\":\\\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\\\",\\\"sourceByteLength\\\":\\\"72\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"54\\\",\\\"byteEnd\\\":\\\"72\\\",\\\"terminator\\\":\\\"crlf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"36\\\",\\\"parentByteEnd\\\":\\\"72\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-04.txt\\\",\\\"sourceStateHash\\\":\\\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\\\",\\\"sourceByteLength\\\":\\\"66\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"17\\\",\\\"byteEnd\\\":\\\"34\\\",\\\"terminator\\\":\\\"lf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"34\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-04.txt\\\",\\\"sourceStateHash\\\":\\\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\\\",\\\"sourceByteLength\\\":\\\"66\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"66\\\",\\\"terminator\\\":\\\"none\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"66\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"--as\",\"paragraph\"];const r=spawnSync(argv[0],argv.slice(1),{encoding:\"utf8\"});console.log(JSON.stringify({argv,stdin:\"\",stdout:r.stdout,stderr:r.stderr,exit:r.status}));'"
```

### Submission 5

```json
"node -e 'const {spawnSync}=require(\"node:child_process\");for(const argv of [[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"edit\",\"anddress\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-01.txt\\\",\\\"sourceStateHash\\\":\\\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"15\\\",\\\"byteEnd\\\":\\\"32\\\",\\\"terminator\\\":\\\"lf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"32\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"retry_budget = 5\"],[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"edit\",\"anddress\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-02.txt\\\",\\\"sourceStateHash\\\":\\\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"67\\\",\\\"terminator\\\":\\\"cr\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"67\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"retry_budget = 5\"],[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"edit\",\"anddress\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-03.txt\\\",\\\"sourceStateHash\\\":\\\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\\\",\\\"sourceByteLength\\\":\\\"72\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"16\\\",\\\"byteEnd\\\":\\\"34\\\",\\\"terminator\\\":\\\"crlf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"34\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"retry_budget = 5\"],[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"edit\",\"anddress\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-04.txt\\\",\\\"sourceStateHash\\\":\\\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\\\",\\\"sourceByteLength\\\":\\\"66\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"66\\\",\\\"terminator\\\":\\\"none\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"66\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"retry_budget = 5\"]]){const r=spawnSync(argv[0],argv.slice(1),{encoding:\"utf8\"});console.log(JSON.stringify({argv,stdin:\"\",stdout:r.stdout,stderr:r.stderr,exit:r.status}));}'"
```

### Submission 6

```json
"node -e 'const {spawnSync}=require(\"node:child_process\");for(const argv of [[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"check\",\"anddress\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-01.txt\\\",\\\"sourceStateHash\\\":\\\"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"15\\\",\\\"byteEnd\\\":\\\"32\\\",\\\"terminator\\\":\\\"lf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"32\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-02.txt\\\",\\\"sourceStateHash\\\":\\\"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"67\\\",\\\"terminator\\\":\\\"cr\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"67\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-03.txt\\\",\\\"sourceStateHash\\\":\\\"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0\\\",\\\"sourceByteLength\\\":\\\"72\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"16\\\",\\\"byteEnd\\\":\\\"34\\\",\\\"terminator\\\":\\\"crlf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"34\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-04.txt\\\",\\\"sourceStateHash\\\":\\\"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe\\\",\\\"sourceByteLength\\\":\\\"66\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"66\\\",\\\"terminator\\\":\\\"none\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"66\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\"],[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"view\",\"anddress\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-01.txt\\\",\\\"sourceStateHash\\\":\\\"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"15\\\",\\\"byteEnd\\\":\\\"32\\\",\\\"terminator\\\":\\\"lf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"32\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-02.txt\\\",\\\"sourceStateHash\\\":\\\"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"67\\\",\\\"terminator\\\":\\\"cr\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"67\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-03.txt\\\",\\\"sourceStateHash\\\":\\\"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0\\\",\\\"sourceByteLength\\\":\\\"72\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"16\\\",\\\"byteEnd\\\":\\\"34\\\",\\\"terminator\\\":\\\"crlf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"34\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-04.txt\\\",\\\"sourceStateHash\\\":\\\"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe\\\",\\\"sourceByteLength\\\":\\\"66\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"66\\\",\\\"terminator\\\":\\\"none\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"66\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"--as\",\"file\"]]){const r=spawnSync(argv[0],argv.slice(1),{encoding:\"utf8\"});console.log(JSON.stringify({argv,stdin:\"\",stdout:r.stdout,stderr:r.stderr,exit:r.status}));}'"
```

## Exact bw argv, inputs, outputs, and exits

### bw process 1

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--help"
  ],
  "exit": 0,
  "stderr": null,
  "stdout": "USAGE\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\n  bw help [<command>]\n\nGLOBAL OPTIONS\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\n  --json                     Select JSON output where the command supports it.\n  --raw                      Select raw View output only.\n\nCAPABILITIES\n  search   Discover current File, Paragraph, or Line Anddresses.\n  view     Read one or more current Anddresses.\n  edit     Replace one current Anddress.\n  check    Check one or more current Anddresses.\n  shell    Run advanced raw Session commands.\n  version  Print the Backwriter version.\n  update   Run the installed-platform updater.\n\nPick, Anchor, Apply, and Data have no one-shot command; use bw shell.\n\nADDITIONAL HELP\n  bw help <command>\n\nGlobal options precede the command. Canonical output options are documented only in that position.\n",
  "stdin": "",
  "captureNote": "Initial exec tool exposes a combined output field; recorded stdout is that field. Separate stderr bytes are unavailable for this first help invocation. Subsequent invocations capture separate stdout/stderr pipes through Node."
}
```

### bw process 2

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "help",
    "search"
  ],
  "stdin": "",
  "stdout": "NAME\n  bw search - discover current Anddresses by exact literal Line content or logical File path\n\nUSAGE\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] search <line|paragraph|file> <query> [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] search /file <logical-path>\n\nDESCRIPTION\n  Searches admitted Workspace Source. Literal queries are case-sensitive and match exact Line content without normalization.\n\nARGUMENTS\n  <line|paragraph|file>  Returned target kind.\n  <query>                Nonempty literal query.\n  /file <logical-path>   Exact logical File lookup.\n\nOPTIONS\n  --workspace, --admit, and --json must precede search.\n  --source LOGICAL_PATH and --subtree LOGICAL_PATH narrow a literal search scope.\n\nWHAT HAPPENS\n  Opens the Runtime, scans admitted source once per selected source, and returns all-or-nothing current results.\n\nOUTPUT\n  Human output lists matches. --json writes the fixed bw.cli.search.v2 envelope.\n\nEXAMPLES\n  bw search line needle --source note.txt\n  bw --json search paragraph needle\n  bw search /file note.txt\n\nFAILURES\n  Invalid request or scope is a usage failure. Unavailable source or Runtime failure exits 1.\n\nSEE ALSO\n  bw help view\n  bw help shell\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 3

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "help",
    "view"
  ],
  "stdin": "",
  "stdout": "NAME\n  bw view - project current content from one or more v5 Anddresses\n\nUSAGE\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json|--raw] view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json view anddress <encoded-v5-Anddress>... --as <line|paragraph|file>\n\nDESCRIPTION\n  Validates current source state and projects the requested target relation from caller-provided v5 Anddresses.\n\nARGUMENTS\n  anddress                  Required input form.\n  <encoded-v5-Anddress>     One or more canonical v5 objects.\n\nOPTIONS\n  --workspace, --admit, --json, and --raw must precede view.\n  --as selects line, paragraph, or file and must be last. Batch View requires --json and --as.\n\nWHAT HAPPENS\n  Opens the Runtime after input validation and returns the requested current projection.\n\nOUTPUT\n  One human or raw View writes content. JSON writes the fixed bw.cli.view.v2 envelope.\n\nEXAMPLES\n  bw view anddress '<v5-Anddress>'\n  bw --raw view anddress '<v5-Line-Anddress>'\n  bw --json view anddress '<v5-Anddress>' --as paragraph\n\nFAILURES\n  Invalid input or unsupported output form is a usage failure. Unavailable or stale source exits 1.\n\nSEE ALSO\n  bw help search\n  bw help check\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 4

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "help",
    "edit"
  ],
  "stdin": "",
  "stdout": "NAME\n  bw edit - replace one current v5 Anddress\n\nUSAGE\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] edit anddress <encoded-v5-Anddress> <content>\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] edit anddress <encoded-v5-Anddress> --stdin\n\nDESCRIPTION\n  Replaces exactly one current File, Paragraph, or Line target through the Runtime Replace seam.\n\nARGUMENTS\n  anddress                  Required input form.\n  <encoded-v5-Anddress>     One canonical v5 object.\n  <content>                  One positional replacement string.\n  --stdin                    Read replacement Content from standard input through EOF.\n\nOPTIONS\n  --workspace, --admit, and --json must precede edit.\n  --stdin is the exclusive Content selector; use standard input to pass literal --stdin Content.\n\nWHAT HAPPENS\n  Validates the Anddress, reads selected standard input before Runtime access, preserves an existing Line terminator automatically, then applies one Replace.\n\nOUTPUT\n  Human output writes the receipt outcome and fresh Anddress when present. --json writes bw.cli.edit.v1.\n\nEXAMPLES\n  bw edit anddress '<v5-Anddress>' 'replacement'\n  printf '%s' 'replacement' | bw edit anddress '<v5-Anddress>' --stdin\n\nFAILURES\n  Invalid input is a usage failure. Standard-input, stale, unavailable, or publication failure exits 1.\n\nSEE ALSO\n  bw help view\n  bw help check\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 5

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "help",
    "check"
  ],
  "stdin": "",
  "stdout": "NAME\n  bw check - check one or more current v5 Anddresses\n\nUSAGE\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... check anddress <encoded-v5-Anddress>\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json check anddress <encoded-v5-Anddress>...\n\nDESCRIPTION\n  Checks the current state of caller-provided v5 Anddresses in input order.\n\nARGUMENTS\n  anddress                  Required input form.\n  <encoded-v5-Anddress>     One or more canonical v5 objects.\n\nOPTIONS\n  --workspace, --admit, and --json must precede check.\n  Multiple inputs require --json. No command-local options are available.\n\nWHAT HAPPENS\n  Validates every input before opening the Runtime, then reports one currentness state per input.\n\nOUTPUT\n  One human input writes one state. --json writes the fixed bw.cli.check.v2 envelope.\n\nEXAMPLES\n  bw check anddress '<v5-Anddress>'\n  bw --json check anddress '<v5-Anddress>' '<v5-Anddress>'\n\nFAILURES\n  Invalid input or a non-JSON batch is a usage failure. Runtime failure exits 1.\n\nSEE ALSO\n  bw help search\n  bw help shell\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 6

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "search",
    "line",
    "retry_budget = 3"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.search.v2\",\"outcome\":\"found\",\"occurrences\":[{\"logicalPath\":\"unit-01.txt\",\"kind\":\"line\",\"line\":\"2\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"15\",\"byteEnd\":\"32\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"32\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"unit-01.txt\",\"kind\":\"line\",\"line\":\"5\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"33\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"unit-02.txt\",\"kind\":\"line\",\"line\":\"2\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"17\",\"byteEnd\":\"34\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"unit-02.txt\",\"kind\":\"line\",\"line\":\"5\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"unit-03.txt\",\"kind\":\"line\",\"line\":\"2\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"16\",\"byteEnd\":\"34\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"unit-03.txt\",\"kind\":\"line\",\"line\":\"5\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"54\",\"byteEnd\":\"72\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"36\",\"parentByteEnd\":\"72\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"unit-04.txt\",\"kind\":\"line\",\"line\":\"2\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"17\",\"byteEnd\":\"34\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"unit-04.txt\",\"kind\":\"line\",\"line\":\"5\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"66\",\"terminator\":\"none\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"66\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}}]}\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 7

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "view",
    "anddress",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"15\",\"byteEnd\":\"32\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"32\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"33\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"17\",\"byteEnd\":\"34\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"16\",\"byteEnd\":\"34\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"54\",\"byteEnd\":\"72\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"36\",\"parentByteEnd\":\"72\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"17\",\"byteEnd\":\"34\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"66\",\"terminator\":\"none\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"66\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "--as",
    "paragraph"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.view.v2\",\"outcomes\":[{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"0\",\"byteEnd\":\"32\",\"fileLineOffset\":\"0\",\"lineCount\":\"2\"},\"content\":\"role = primary\\nretry_budget = 3\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"33\",\"byteEnd\":\"67\",\"fileLineOffset\":\"3\",\"lineCount\":\"2\"},\"content\":\"role = secondary\\nretry_budget = 3\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"0\",\"byteEnd\":\"34\",\"fileLineOffset\":\"0\",\"lineCount\":\"2\"},\"content\":\"role = secondary\\rretry_budget = 3\\r\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"35\",\"byteEnd\":\"67\",\"fileLineOffset\":\"3\",\"lineCount\":\"2\"},\"content\":\"role = primary\\rretry_budget = 3\\r\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"0\",\"byteEnd\":\"34\",\"fileLineOffset\":\"0\",\"lineCount\":\"2\"},\"content\":\"role = primary\\r\\nretry_budget = 3\\r\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"36\",\"byteEnd\":\"72\",\"fileLineOffset\":\"3\",\"lineCount\":\"2\"},\"content\":\"role = secondary\\r\\nretry_budget = 3\\r\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"0\",\"byteEnd\":\"34\",\"fileLineOffset\":\"0\",\"lineCount\":\"2\"},\"content\":\"role = secondary\\nretry_budget = 3\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"35\",\"byteEnd\":\"66\",\"fileLineOffset\":\"3\",\"lineCount\":\"2\"},\"content\":\"role = primary\\nretry_budget = 3\"}]}\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 8

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "edit",
    "anddress",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"15\",\"byteEnd\":\"32\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"32\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "retry_budget = 5"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.edit.v1\",\"outcome\":\"changed\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"15\",\"byteEnd\":\"32\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"32\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}}\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 9

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "edit",
    "anddress",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "retry_budget = 5"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.edit.v1\",\"outcome\":\"changed\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}}\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 10

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "edit",
    "anddress",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"16\",\"byteEnd\":\"34\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "retry_budget = 5"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.edit.v1\",\"outcome\":\"changed\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"16\",\"byteEnd\":\"34\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}}\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 11

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "edit",
    "anddress",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"66\",\"terminator\":\"none\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"66\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "retry_budget = 5"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.edit.v1\",\"outcome\":\"changed\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"66\",\"terminator\":\"none\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"66\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}}\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 12

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "check",
    "anddress",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"15\",\"byteEnd\":\"32\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"32\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"16\",\"byteEnd\":\"34\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"66\",\"terminator\":\"none\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"66\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.check.v2\",\"outcomes\":[{\"status\":\"current\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"15\",\"byteEnd\":\"32\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"32\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}},{\"status\":\"current\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}},{\"status\":\"current\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"16\",\"byteEnd\":\"34\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}},{\"status\":\"current\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"66\",\"terminator\":\"none\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"66\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}}]}\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 13

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "view",
    "anddress",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"15\",\"byteEnd\":\"32\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"32\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"16\",\"byteEnd\":\"34\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"66\",\"terminator\":\"none\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"66\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "--as",
    "file"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.view.v2\",\"outcomes\":[{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"file\"},\"content\":\"role = primary\\nretry_budget = 5\\n\\nrole = secondary\\nretry_budget = 3\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"file\"},\"content\":\"role = secondary\\rretry_budget = 3\\r\\rrole = primary\\rretry_budget = 5\\r\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"file\"},\"content\":\"role = primary\\r\\nretry_budget = 5\\r\\n\\r\\nrole = secondary\\r\\nretry_budget = 3\\r\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"file\"},\"content\":\"role = secondary\\nretry_budget = 3\\n\\nrole = primary\\nretry_budget = 5\"}]}\n",
  "stderr": "",
  "exit": 0
}
```

````

#### D3 dummy-new transcript

````text
# Independent dummy-new public-Help-only evaluation

## Outcome

Discovered eight matching Lines. Read all eight surrounding Paragraphs before choosing targets. Replaced only primary Lines @0, @3, @4, @7, one per source. Four fresh receipts checked Current. Viewed all four final Files. Secondary values remain 3. No failed commands, retries, resets, or other fixture reads/writes.

## Measurement and capture limits

- Product processes: 3 (two Help processes and one persistent shell).
- Tool calls through task completion: 7 functions.exec calls, each containing one terminal call. Transcript persistence adds one functions.exec/apply_patch call, total 8.
- Shell commands: 9 including exit.
- Capability commands: Search 1; View 2 (8 Paragraph inputs then 4 File inputs); Replace 4; Check 1 (4 inputs). No other capability commands.
- Extra Content-only queries: 2 View commands / 12 inputs; zero additional Search queries.
- Measured interactive phase: 26162 ms from returned shell startup to returned shell completion; initial startup and two Help calls excluded. Whole-run wall time was not independently instrumented, so no exact whole-run elapsed is claimed.
- All outputs below are exact JSON-escaped strings returned by the terminal tool. Help commands used plain pipes, whose API returned merged output without separate stdout/stderr. Shell used a PTY with echo and ONLCR disabled: stdout/stderr are merged and cannot be separately attributed. Thus exact merged output is preserved, not separately captured streams. No diagnostic text appeared.
- cwd for every product process: /tmp/backwriter-gate5.74tHth/dummy-new/workspace.
- Terminal launch shell: zsh default login semantics. Explicit wrapper shell inputs and product argv are recorded below. No additional product processes were invoked.

## Exact transcript

### Call 1

Shell input: "/tmp/backwriter-gate5.74tHth/gnu-target/release/bw --help"

argv: ["/tmp/backwriter-gate5.74tHth/gnu-target/release/bw","--help"]

Merged output: "USAGE\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\n  bw help [<command>]\n\nGLOBAL OPTIONS\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\n  --json                     Select JSON output where the command supports it.\n  --raw                      Select raw View output only.\n\nCOMMANDS\n  shell    Reuse short references across search, view, replace, and check.\n  search   Discover current File, Paragraph, or Line Anddresses.\n  view     Read one or more current Anddresses.\n  edit     Replace one current Anddress.\n  check    Check one or more current Anddresses.\n  version  Print the Backwriter version.\n  update   Run the installed-platform updater.\n\nADDITIONAL HELP\n  bw help <command>\n  Advanced topics: pick, anchor, apply, data (raw Session only; no one-shot execution).\n\nGlobal options precede the command. Use bw help shell for ordinary short-ref work.\n"

Exit: 0.

### Call 2

Shell input: "/tmp/backwriter-gate5.74tHth/gnu-target/release/bw help shell"

argv: ["/tmp/backwriter-gate5.74tHth/gnu-target/release/bw","help","shell"]

Merged output: "NAME\n  bw shell - reuse short references across search, view, replace, and check\n\nUSAGE\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n\nCOMMANDS\n  search <line|paragraph|file> <query> [--source PATH | --subtree PATH]...\n  search /file <logical-path>\n  view <REF>... [--as <line|paragraph|file>]\n  replace <REF> <CONTENT>\n  check <REF>...\n  let <NAME> = <REF>\n  exit\n\nREFERENCES AND CONTENT\n  @N is a zero-based, append-only reference from this process. It ends at EOF or exit.\n  @name is a named Anddress; @hits[0] selects an indexed Search/Pick binding.\n  let name = @N makes a named alias. Names cannot be redefined.\n  Double-quote arguments containing spaces. Quoted escapes are \\\\, \\\", \\n, \\r, and \\t.\n  One physical input line is one command; no single quotes, pipes, or EOF Content selector.\n  Line Replace accepts body only: NUL/CR/LF are rejected; the existing terminator is preserved.\n  File/Paragraph Replace uses exact Content. Changed source bytes stale old same-source refs,\n  including other locations. Use the fresh receipt or explicitly search again; never auto-retry.\n\nOUTPUT\n  Search appends one @N per result; Empty writes nothing.\n  View returns each input ref, a fresh ref, kind/location, and complete Content in input order.\n  View<TAB>REF<TAB>bytes=N starts a record, followed by fresh-ref metadata and N exact bytes.\n  The following LF and EndView line are display framing, not source Content.\n  RelationAbsent is shown at its input position without Content or a fresh slot.\n  Replace writes Unchanged/Changed with a fresh ref, or Changed<TAB>None without one.\n  Check writes one status per input; only Current appends a fresh ref. Check is optional.\n\nEXAMPLES\n  Start bw shell in a workspace with only note.txt containing needle plus CRLF, then enter:\n  search line needle --source note.txt\n  view @0 --as paragraph\n  let selected = @0\n  replace @selected \"new value\"\n  check @0 @2\n  view @3\n  exit\n\nADVANCED\n  let hits = search line needle\n  view anddress @hits[0]\n  Raw named View writes exact Content without direct-View framing.\n  See bw help pick, bw help anchor, bw help apply, and bw help data.\n\nFAILURES\n  Usage errors record exit 2; Runtime/source errors record exit 1; later commands still run.\n  Stdin/stdout failure ends the shell with exit 1. Partial output cannot prove delivery or\n  undo publication. --json/--raw and command-line operands are unavailable.\n\nSEE ALSO\n  bw help search\n  bw help edit\n"

Exit: 0.

### Call 3

Shell input: "stty -echo -onlcr; exec /tmp/backwriter-gate5.74tHth/gnu-target/release/bw shell"

argv: ["stty","-echo","-onlcr"] followed by exec ["/tmp/backwriter-gate5.74tHth/gnu-target/release/bw","shell"]

Merged output: ""

Process remains running (session 8842).

### Call 4

Exact shell stdin: "search line \"retry_budget = 3\"\n"

Merged output: "@0\tLine\tunit-01.txt:2\n@1\tLine\tunit-01.txt:5\n@2\tLine\tunit-02.txt:2\n@3\tLine\tunit-02.txt:5\n@4\tLine\tunit-03.txt:2\n@5\tLine\tunit-03.txt:5\n@6\tLine\tunit-04.txt:2\n@7\tLine\tunit-04.txt:5\n"

Process remains running.

### Call 5

Exact shell stdin: "view @0 @1 @2 @3 @4 @5 @6 @7 --as paragraph\n"

Merged output: "View\t@0\tbytes=32\n@8\tParagraph\tunit-01.txt:1-2\nrole = primary\nretry_budget = 3\n\nEndView\nView\t@1\tbytes=34\n@9\tParagraph\tunit-01.txt:4-5\nrole = secondary\nretry_budget = 3\n\nEndView\nView\t@2\tbytes=34\n@10\tParagraph\tunit-02.txt:1-2\nrole = secondary\rretry_budget = 3\r\nEndView\nView\t@3\tbytes=32\n@11\tParagraph\tunit-02.txt:4-5\nrole = primary\rretry_budget = 3\r\nEndView\nView\t@4\tbytes=34\n@12\tParagraph\tunit-03.txt:1-2\nrole = primary\r\nretry_budget = 3\r\n\nEndView\nView\t@5\tbytes=36\n@13\tParagraph\tunit-03.txt:4-5\nrole = secondary\r\nretry_budget = 3\r\n\nEndView\nView\t@6\tbytes=34\n@14\tParagraph\tunit-04.txt:1-2\nrole = secondary\nretry_budget = 3\n\nEndView\nView\t@7\tbytes=31\n@15\tParagraph\tunit-04.txt:4-5\nrole = primary\nretry_budget = 3\nEndView\n"

Process remains running.

### Call 6

Exact shell stdin: "replace @0 \"retry_budget = 5\"\nreplace @3 \"retry_budget = 5\"\nreplace @4 \"retry_budget = 5\"\nreplace @7 \"retry_budget = 5\"\n"

Merged output: "@16\tChanged\tLine\tunit-01.txt:2\n@17\tChanged\tLine\tunit-02.txt:5\n@18\tChanged\tLine\tunit-03.txt:2\n@19\tChanged\tLine\tunit-04.txt:5\n"

Process remains running.

### Call 7

Exact shell stdin: "check @16 @17 @18 @19\nview @20 @21 @22 @23 --as file\nexit\n"

Merged output: "@20\tCurrent\tLine\tunit-01.txt:2\n@21\tCurrent\tLine\tunit-02.txt:5\n@22\tCurrent\tLine\tunit-03.txt:2\n@23\tCurrent\tLine\tunit-04.txt:5\nView\t@20\tbytes=67\n@24\tFile\tunit-01.txt\nrole = primary\nretry_budget = 5\n\nrole = secondary\nretry_budget = 3\n\nEndView\nView\t@21\tbytes=67\n@25\tFile\tunit-02.txt\nrole = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 5\r\nEndView\nView\t@22\tbytes=72\n@26\tFile\tunit-03.txt\nrole = primary\r\nretry_budget = 5\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n\nEndView\nView\t@23\tbytes=66\n@27\tFile\tunit-04.txt\nrole = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 5\nEndView\n"

Exit: 0.

## Human target decision

Paragraph Content showed primary at unit-01.txt:2, unit-02.txt:5, unit-03.txt:2, unit-04.txt:5. No program selected these targets. Line Replace's documented terminator preservation was used. Final Content shows LF, CR, CRLF, and an unterminated last Line respectively; no full pre-edit File view was taken, so whole-file byte preservation outside viewed Paragraphs relies on the documented Line Replace contract, not an independent byte comparison.

````

#### D3 genie-old transcript

````text
# Independent documented-shell evaluation: N-minus

Only supplied public README and public Help were used. Fixture Content reads and edits were exclusively through the supplied bw, in this arm's workspace. No source, oracle, other arm, installed bw, update, or filesystem fixture Content access occurred.

## Result and metrics

Four primary Line bodies changed: unit-01.txt:2, unit-02.txt:5, unit-03.txt:2, unit-04.txt:5. Eight initial literal matches were inspected via their Paragraph Content before deciding. Final bw File Content shows primary values 5 and secondary values 3 with LF, CR, CRLF, and absent-final-terminator cases preserved by the documented body-only Replace contract. No independent whole-file before/after byte comparison was performed.

All four replacement receipt references checked Current. All four pre-edit secondary references unexpectedly checked NotCurrent; this evidence is retained, not retried away. A new Search found the four remaining secondary Lines and all four fresh references checked Current. No syntax/runtime failures or resets occurred; final shell exit was 0.

Shell elapsed wall clock (from shell-start tool return to exit tool return): 40459 ms. Documentation discovery occurred before that timer; full-run elapsed was not instrumented and must not be inferred from this value.

Tool counts through transcript creation: 11 functions orchestration calls; 12 nested calls (5 exec_command, 6 write_stdin, 1 apply_patch). Product processes: 3 bw processes (--help, help shell, shell). Other explicitly launched processes: 5 zsh wrappers, 2 cat README reads, 1 stty; expected total 11 OS processes, excluding infrastructure and any implicit login-shell startup subprocesses. No process monitor was used.

Shell commands: 35 (Search 2; View 14; Replace 4; Check 2; let 12; exit 1). View commands comprise 2 reference-only projected batches (8 Paragraph and 4 File inputs) plus 12 extra Content-only raw queries (8 Paragraph, 4 File). Check has 12 inputs, 8 Current and 4 NotCurrent. Replace invokes 4 Edit/Apply workflows by documented semantics; no explicit raw Edit/Apply, Pick, Anchor, or Data commands. Internal capability invocation counts were not instrumented. Query literals: two `retry_budget = 3` searches. No extra search for role metadata.

## Capture limitations

All outputs below are exact tool-returned strings represented as JSON escaped UTF-8 text. The tool does not expose separate stdout/stderr for ordinary exec calls. The shell used a PTY, `stty -echo -onlcr`, and therefore its output is explicitly a merged stdout/stderr stream, not independently attributed streams. Input is the exact string sent, including LF. No PTY echo was included. The README was read a second time solely to archive its exact returned text; the initial read's output was the same complete public copy and is reproduced below. This duplication is included in tool/process counts. Output chunks can span multiple shell commands and are not assigned per-command exit status; only final process exit is available.

## Invocation transcript

```json
{"cmd":"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw --help","argv":["/tmp/backwriter-gate5.74tHth/nminus-target/release/bw","--help"],"exit":0,"merged_output":"USAGE\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\n  bw help [<command>]\n\nGLOBAL OPTIONS\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\n  --json                     Select JSON output where the command supports it.\n  --raw                      Select raw View output only.\n\nCAPABILITIES\n  search   Discover current File, Paragraph, or Line Anddresses.\n  view     Read one or more current Anddresses.\n  edit     Replace one current Anddress.\n  check    Check one or more current Anddresses.\n  shell    Run advanced raw Session commands.\n  version  Print the Backwriter version.\n  update   Run the installed-platform updater.\n\nPick, Anchor, Apply, and Data have no one-shot command; use bw shell.\n\nADDITIONAL HELP\n  bw help <command>\n\nGlobal options precede the command. Canonical output options are documented only in that position.\n"}
{"cmd":"cat /tmp/backwriter-gate5.74tHth/genie-old/public-guide.md","argv":["cat","/tmp/backwriter-gate5.74tHth/genie-old/public-guide.md"],"exit":0,"merged_output":"# Backwriter\n\nBackwriter is a Rust Core and Runtime for current, structural text work over\nadmitted Workspace Source. It provides target-local File, Paragraph, and Line\naddresses without turning source history or editor state into Core identity.\n\nThe Core capability inventory is Search, View, Pick, Anchor, Check, Edit,\nApply, and Data. The repository currently provides their Rust Core/Runtime\nsurfaces and the canonical `bw` executable's one-shot human and JSON\nSearch/View/Check/Edit, raw View, Anddress-first one-shot Edit, Session Pick, batch\nCheck, Anchor, Edit, Apply, result-binding, explicit Data modes, and\nAdapter-owned Version and Update.\n\n## Quick start\n\nInstall the closed official `0.2.5` release with the command for your\nplatform.\n\nLinux, macOS, or WSL:\n\n```sh\ncurl -fsSL https://backwriter.pentagration.com/install.sh | sh\n```\n\nWindows PowerShell:\n\n```powershell\nirm https://backwriter.pentagration.com/install.ps1 | iex\n```\n\nWindows CMD:\n\n```cmd\ncurl.exe -fsSL https://backwriter.pentagration.com/install.cmd -o install.cmd && call install.cmd && del install.cmd\n```\n\nThe POSIX installer places `bw` at `$HOME/.local/bin/bw`; the PowerShell and\nCMD paths place `bw.exe` at `$HOME\\.local\\bin\\bw.exe`. They do not change\n`PATH`, a shell startup file, the PowerShell profile, or the registry. A fresh\ninstall prints `Installed Backwriter: <version>`; replacing an existing\ndestination prints `Updated Backwriter: <version>`. The executable path and\n`PATH` guidance are printed separately only when the installation directory is\nnot already on `PATH`.\n\n### Version\n\n```sh\nbw version\n```\n\nThe closed official `0.2.5` build prints exactly:\n\n```text\nBackwriter 0.2.5\n```\n\n### Update\n\n```sh\nbw update\n```\n\n`bw update` downloads and delegates to the current official installer. The\ncurrent manifest selects the closed `0.2.5` distribution. The installer\nreads that manifest, verifies the selected artifact, and installs or reinstalls\nthat manifest version only after validation succeeds. It does not run a\nbackground updater or compare release versions. The installer accepts only the\nexact immutable `0.2.4` manifest and current `0.2.5` manifest; `0.2.3`, `0.2.2`, `0.2.1`, `0.2.0`, stable\n`0.1.0`, and beta.3 acceptance is retired. The command still performs no\nversion comparison and adds no version guard.\n\nThe product is Backwriter. The source Cargo package and library crate are\n`backwriter` at source-ready unpublished `0.2.6`; a source-built `bw version`\nprints `Backwriter 0.2.6`. The sole canonical executable and external Adapter\ncommand are `bw`. There is no `backwriter` binary, alias, or wrapper. The\nofficial installer remains separate and selects the closed public `0.2.5`\ndistribution.\n\nProduction `src/**` remains byte-identical to Gate 6 candidate\n`22e6df23755cdc80b299b77be313d307b67bc37f`. Official artifacts, installers,\nmanifest, Update target, and the exact 68-file public distribution remain\npublished and closed `0.2.5` from Source Authority\n`e4022fc073e9df3928e1c3817b266ce92121a03c`. Update performs no version\ncomparison, so source-built `0.2.6` still installs or reinstalls that official\n`0.2.5` release until a separately authorized release closes the gap.\n\n## Anddress-first editing\n\nThe default source-checkout replacement flow is:\n\n1. Run `bw --json search ...`.\n2. Select one occurrence from `occurrences` and pass its exact embedded v5\n   `anddress` object unchanged as one argv value.\n3. Run `bw edit anddress '<opaque-v5-object>' '<new-content>'`.\n\nHuman Search rows are not encoded Anddress values and cannot be Edit input.\nTreat the selected JSON object as opaque: do not interpret or rewrite its hash,\nrange, length, or other fields. File and Paragraph Content is the exact\nreplacement. Line Content is body-only, rejects NUL, CR, and LF, and preserves\nthe None, LF, CR, or CRLF terminator carried by the exact v5 Line. A Line NUL\nis `edit.content_contains_nul`; a Line CR or LF is\n`edit.line_body_contains_terminator`. The latter explains that Backwriter adds\nthe current terminator and that advanced raw Session Edit/Apply owns exact\nextent replacement. Apply alone confirms that source state before publication.\nView or Pick may help a caller select a target; neither View nor Check is\nrequired.\n\nHuman success writes one exact LF-terminated receipt row. `Unchanged` is\nfollowed by the still-current input v5 object; `Changed` is followed by the\nfresh v5 object when the resulting File, Line, or unique Paragraph has one,\nand otherwise by `None`. With leading `--json`, the same result is the compact\nAdapter-only `bw.cli.edit.v1` object with `schema`, `outcome`, and `anddress`\nkeys in that order; `anddress` is the exact v5 object or JSON `null`. Reuse only\nthe address returned by the receipt. A changed Paragraph with `None` requires\nan explicit Search before later target work. Exit `1` is neither a stale-only\nclassification nor proof that source bytes are unchanged, so it must not\ntrigger automatic retry.\n\nOne-shot Content is either one UTF-8 argv value or the exclusive `--stdin`\nselector in that position; stdin is read to EOF after v5 address validation and\nbefore Runtime access. Both forms have the same target-specific Content rules.\nFile/Paragraph CR and LF are exact Content, while Line body Content never\nsilently strips a trailing newline. Literal `--json` and `--raw` remain exact\nContent; a literal `--stdin` Content value is supplied through standard input.\n\nRaw Session is the advanced composition surface for Insert/Delete/Move/Copy,\nPosition, Anchor/Data lifetime, explicit bindings, and separate Apply. It is\nnot a prerequisite or alias for ordinary Replace. Its existing raw\n`edit replace` accepts caller-provided exact range Content, including an\nexplicit terminator or multiline replacement, and `apply @edit` publishes it\nseparately. The caller owns the binding, index, quoting, terminator, and\npublication boundary. General replacement should use one-shot body Content\nfirst; no exact one-shot flag or alternate executor exists.\n\n### Shell-local references\n\n`bw shell` also provides a short-lived interactive flow without adding a Core\nwire or persistent identity:\n\n```text\nsearch line needle\nview @0\nreplace @1 replacement\ncheck @2 @3\nexit\n```\n\nQuote one direct-shell query or replacement argument when it contains\nwhitespace; the quotes are shell syntax and are not part of the exact literal\nor replacement Content:\n\n```text\nsearch line \"duplicate = one\"\nreplace @1 \"duplicate = two\"\n```\n\nSuccessful direct `search` and projected direct `view` append `@N` references\nin output order, including duplicates. Direct `check <REF>...` resolves every\nreference before Runtime access, writes one Current/NotCurrent/Unavailable\nstate per input, and appends a fresh slot only for Current. `replace @N <content>` uses the same\ntarget-aware Content rules as one-shot Edit and appends a fresh reference for\n`Unchanged` or `Changed` when the receipt has an Anddress; `Changed\\tNone`\nadds none. Slots are zero-based canonical unsigned decimals, append-only, and\ndiscarded at `exit` or EOF. `@name` and `@name[index]` remain the advanced raw\nSession forms; `let name = @N` explicitly clones a numeric slot into that\nexisting named Anddress binding. Raw `edit replace` and separate `apply @edit`\nremain the advanced exact-range path.\n\nThe default workspace is the process current working directory. An explicit\n`--workspace` must be absolute and is checked by Runtime. Search admits `.` by\ndefault; repeat `--admit LOGICAL_PATH` before `search` to narrow admission.\nAfter the query, repeat `--source LOGICAL_PATH` or `--subtree LOGICAL_PATH` to\nnarrow a Search scope. Without a scope selector, Search covers all admitted\nsources.\n\n## Current CLI scope\n\n`bw` currently implements Adapter-owned one-shot Version and Update, one-shot\nhuman or JSON Search, View, Check, and Edit, raw View, Anddress-first one-shot Edit,\nplus Session Pick, batch Check, Anchor, Edit, Apply, and Data:\n\n```text\nbw version\nbw update\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    view anddress <encoded-v5-Anddress>... [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    check anddress <encoded-v5-Anddress>...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --raw\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    check anddress <encoded-v5-Anddress>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n```\n\nVersion and Update do not call Backwriter Core or Runtime and create no Core\nwire or capability workflow.\n\nContent Search preserves Core literal validation, scope, projection, and\ndeterministic result order. The distinct `search /file` form validates one\nlogical path and returns the current File Anddress for an admitted regular\nUTF-8, NUL-free source regardless of whether it is empty or contains matching\ntext. Missing paths and directories return Empty; the form has no scope\nselectors or synthetic content query. View decodes v5 Anddresses and projects\neach to itself or one ancestor before reading its exact content. A single input\ndefaults to self projection. `--as` chooses one target kind; multiple inputs\nrequire both `--json` and `--as`. Check decodes every v5 operand before Runtime\naccess. One human input writes `Current`, `NotCurrent`, or `Unavailable`; a\nbatch requires `--json` and preserves one outcome per input. Search, View,\nCheck, and Edit `--json`\nwrite compact Adapter objects with exact embedded v5 Anddress objects where\napplicable; each is an Adapter schema, not a second Core wire. View uses the\nhard-cut `bw.cli.view.v2` outcomes array for both single and batch results;\nCheck uses the hard-cut `bw.cli.check.v2` ordered outcomes array.\nRaw View is an explicit Adapter exact-text mode that reuses the ordinary View\nprojection without a Core wire or changed View meaning.\nHuman Search, View, and Check keep their existing projections; human Edit\nreceipts intentionally return the exact current v5 object when one exists.\nThe Session holds one Runtime until EOF\nor `exit` and has explicit local Search, Pick, Anddress, Edit, View, and Check\nbindings plus non-aliasing Anchedress handles. It owns one explicit `DataStore`\nfor the Session only; names are typed and never persist past EOF or `exit`.\nSession Pick\npasses a named Search or Pick collection and an Adapter-parsed predicate directly\nto Core; Session batch Check passes a named matching outcome directly to its\nRuntime batch seam and prints only report counts. Session Anchor creates a live\nhandle only through `let <name> = anchor create <anddress-ref>`, views it through\n`view anchored @<name>`, and can invalidate its logical source with `anchor\ninvalidate-source <logical-path>`. One-shot Data and Anchor are intentionally\nunsupported because their DataStore and live-handle contracts require Session\nlifetime. One-shot Pick, raw Edit-object transport, and a separate\nApply transport await collection or Edit transport schema authority. The\ndistinct Anddress-first one-shot Edit above is implemented. Raw output other\nthan one-shot View and further Session behavior remain deferred.\n\n## Scope\n\nBackwriter is not Git, a file watcher, daemon, persistent index, or editor UI.\nIt reads admitted current Workspace Source through Runtime's safe no-follow\naccess. It does not model branches, merges, history, automatic re-evaluation,\nor editor buffers.\n\nApply uses its accepted current observation and does not coordinate concurrent\nwriters. Writers may race and one publication may overwrite another\nsource-visible change; hosts requiring a stronger guarantee coordinate outside\nBackwriter.\n\n## Build from source and test\n\n```sh\ncargo build --offline --locked --release\n./target/release/bw search line \"needle\"\n./target/release/bw --workspace /path/project search paragraph \"needle\"\ncargo test --offline --locked\n```\n\n## Official desktop distribution\n\nThe official distribution authority is\n[https://backwriter.pentagration.com](https://backwriter.pentagration.com).\nIt publishes the closed Backwriter `0.2.5` release for Linux/WSL x86_64,\nmacOS arm64, macOS x86_64, and Windows x86_64 from Source Authority revision\n`e4022fc073e9df3928e1c3817b266ce92121a03c`. Linux uses canonical target\n`x86_64-unknown-linux-musl`;\n`x86_64-unknown-linux-gnu` remains the local development/test-host target.\nmacOS uses `aarch64-apple-darwin` with minimum macOS 11.0 and\n`x86_64-apple-darwin` with minimum macOS 10.12. The macOS artifacts receive\nstatic cross-build verification but are not claimed to have been executed on a\nnative Mac before publication. Windows uses `x86_64-pc-windows-gnu` and the\ncanonical executable `bw.exe`; its static cross-build verification does not\nclaim native Windows, PowerShell, or CMD execution. Linux arm64 is not currently\nprovided, and no universal host-compatibility claim is made.\n\n`install.sh` reads the canonical manifest, verifies the downloaded artifact\nagainst the manifest SHA-256, and installs the verified binary at\n`$HOME/.local/bin/bw` with a same-directory rename. Concurrent same-user\nHOME mutation is caller-owned. The published `.sha256` sidecar is for manual\nverification and is not installer authority. Windows PowerShell verifies the\nsame manifest authority and exact ZIP, and installs to\n`$HOME\\.local\\bin\\bw.exe` without editing PATH or the PowerShell profile. The\nthree canonical install commands and the canonical `bw update` command are kept\ntogether in [Quick start](#quick-start).\n\nThe CMD command writes `install.cmd` in the current directory and removes it\nafter a successful installation. An existing file with that name is replaced.\n\nThe CRLF `install.cmd` checks `curl.exe` and `powershell.exe`, downloads exactly\nthe canonical `install.ps1` over HTTPS-only TLS 1.2-or-newer transport into a\ncollision-failing `%TEMP%` task directory, delegates all installation meaning,\ncleans the directory, and preserves the child exit code. It owns no manifest,\nSHA-256, ZIP, or installation logic. The Linux-hosted CMD regression is static;\nno native CMD execution is claimed.\nThe distribution provides no\npublisher-authenticity signature or trusted signing identity, background or\nautomatic update, telemetry, `sudo` execution, or automatic `PATH` or\nshell-startup-file change.\nGitHub is a public source and documentation mirror, not the distribution\nauthority. The complete beta.1, beta.2, beta.3, stable `0.1.0`, `0.2.0`, and\n`0.2.1` and `0.2.2` version directories remain unchanged and immutable. The\ncomplete `0.2.3`, `0.2.4`, and `0.2.5` version directories are immutable; the\n`0.2.5` release is closed. The active public tree has exactly 68 files, and\nthe current installers and manifest select `0.2.5`. Any later platform or\nversion requires separate Owner authority. Linux arm64, tags, GitHub Releases,\ncrates.io publication, and background or automatic update remain outside the\ncompleted publication. The earlier `0.2.4` publication added its eight\nversioned files and replaced only the two installers and manifest pointers\nwithout replacing any of the 48 prior versioned files or the CMD Adapter. Its\nsecond publisher run reused all 60 files without metadata change. The `0.2.5`\npublication then added its eight versioned files, replaced the two installers\nand manifest pointers, and reused all 68 files on its second run.\n\n## Architecture\n\n- [Current state](docs/current/now.md)\n- [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)\n- [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)\n- [CLI V1 authority](docs/architecture/backwriter-cli-v1.md)\n- [Verification](docs/development/verification.md)\n\n## License\n\n[MIT License](LICENSE)\n"}
{"cmd":"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw help shell","argv":["/tmp/backwriter-gate5.74tHth/nminus-target/release/bw","help","shell"],"exit":0,"merged_output":"NAME\n  bw shell - run one local reference session and advanced raw Session commands\n\nUSAGE\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n\nDESCRIPTION\n  Reads commands from standard input until exit. Direct search, view, replace, and check use session-local numeric Anddress references. Raw bindings and raw capability composition remain the advanced surface.\n\nARGUMENTS\n  None.\n\nOPTIONS\n  --workspace and --admit must precede shell.\n  --json and --raw are unavailable.\n\nWHAT HAPPENS\n  A successful direct search, view, or current check emits append-only @N references. Direct replace uses one reference and emits a fresh reference when one exists. References end with this shell process. Raw let, Pick, View, Check, Anchor, Edit, Apply, and Data retain their existing grammar.\n\nOUTPUT\n  Direct references write @N, target kind, and location. Direct Check writes one state per input. Raw commands write their existing human result.\n\nEXAMPLES\n  bw shell\n  search line needle\n  view @0\n  replace @1 replacement\n  check @2 @3\n  let hits = search line needle\n  view anddress @hits[0]\n  exit\n\nFAILURES\n  Invalid shell grammar is a usage failure. Runtime and source failures exit 1.\n\nSEE ALSO\n  bw help search\n  bw help edit\n"}
{"cmd":"stty -echo -onlcr; exec /tmp/backwriter-gate5.74tHth/nminus-target/release/bw shell","argvs":[["stty","-echo","-onlcr"],["/tmp/backwriter-gate5.74tHth/nminus-target/release/bw","shell"]],"cwd":"/tmp/backwriter-gate5.74tHth/genie-old/workspace","tty":true}
{"input":null,"merged_output":"","exit":null,"session_running":true}
{"input":"search line \"retry_budget = 3\"\n","merged_output":"@0\tLine\tunit-01.txt:2\n@1\tLine\tunit-01.txt:5\n@2\tLine\tunit-02.txt:2\n@3\tLine\tunit-02.txt:5\n@4\tLine\tunit-03.txt:2\n@5\tLine\tunit-03.txt:5\n@6\tLine\tunit-04.txt:2\n@7\tLine\tunit-04.txt:5\n","exit":null,"session_running":true}
{"input":"view @0 @1 @2 @3 @4 @5 @6 @7 --as paragraph\n","merged_output":"@8\tParagraph\tunit-01.txt:1-2\n@9\tParagraph\tunit-01.txt:4-5\n@10\tParagraph\tunit-02.txt:1-2\n@11\tParagraph\tunit-02.txt:4-5\n@12\tParagraph\tunit-03.txt:1-2\n@13\tParagraph\tunit-03.txt:4-5\n@14\tParagraph\tunit-04.txt:1-2\n@15\tParagraph\tunit-04.txt:4-5\n","exit":null,"session_running":true}
{"input":"let p0 = @8\nview anddress @p0\n","merged_output":"role = primary\nretry_budget = 3\n","exit":null,"session_running":true}
{"input":"let p1 = @9\nview anddress @p1\nlet p2 = @10\nview anddress @p2\nlet p3 = @11\nview anddress @p3\nlet p4 = @12\nview anddress @p4\nlet p5 = @13\nview anddress @p5\nlet p6 = @14\nview anddress @p6\nlet p7 = @15\nview anddress @p7\n","merged_output":"role = secondary\nretry_budget = 3\nrole = secondary\rretry_budget = 3\rrole = primary\rretry_budget = 3\rrole = primary\r\nretry_budget = 3\r\nrole = secondary\r\nretry_budget = 3\r\nrole = secondary\nretry_budget = 3\nrole = primary\nretry_budget = 3","exit":null,"session_running":true}
{"input":"replace @0 \"retry_budget = 5\"\nreplace @3 \"retry_budget = 5\"\nreplace @4 \"retry_budget = 5\"\nreplace @7 \"retry_budget = 5\"\ncheck @16 @17 @18 @19 @1 @2 @5 @6\n","merged_output":"@16\tChanged\tLine\tunit-01.txt:2\n@17\tChanged\tLine\tunit-02.txt:5\n@18\tChanged\tLine\tunit-03.txt:2\n@19\tChanged\tLine\tunit-04.txt:5\n@20\tCurrent\tLine\tunit-01.txt:2\n@21\tCurrent\tLine\tunit-02.txt:5\n@22\tCurrent\tLine\tunit-03.txt:2\n@23\tCurrent\tLine\tunit-04.txt:5\nNotCurrent\nNotCurrent\nNotCurrent\nNotCurrent\n","exit":null,"session_running":true}
{"input":"view @16 @17 @18 @19 --as file\nlet f0 = @24\nview anddress @f0\nlet f1 = @25\nview anddress @f1\nlet f2 = @26\nview anddress @f2\nlet f3 = @27\nview anddress @f3\nsearch line \"retry_budget = 3\"\ncheck @28 @29 @30 @31\nexit\n","merged_output":"@24\tFile\tunit-01.txt\n@25\tFile\tunit-02.txt\n@26\tFile\tunit-03.txt\n@27\tFile\tunit-04.txt\nrole = primary\nretry_budget = 5\n\nrole = secondary\nretry_budget = 3\nrole = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 5\rrole = primary\r\nretry_budget = 5\r\n\r\nrole = secondary\r\nretry_budget = 3\r\nrole = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 5@28\tLine\tunit-01.txt:5\n@29\tLine\tunit-02.txt:2\n@30\tLine\tunit-03.txt:5\n@31\tLine\tunit-04.txt:2\n@32\tCurrent\tLine\tunit-01.txt:5\n@33\tCurrent\tLine\tunit-02.txt:2\n@34\tCurrent\tLine\tunit-03.txt:5\n@35\tCurrent\tLine\tunit-04.txt:2\n","exit":0,"session_running":false}
{"cmd":"cat /tmp/backwriter-gate5.74tHth/genie-old/public-guide.md","argv":["cat","/tmp/backwriter-gate5.74tHth/genie-old/public-guide.md"],"purpose":"exact public-guide archival reread","exit":0,"merged_output":"# Backwriter\n\nBackwriter is a Rust Core and Runtime for current, structural text work over\nadmitted Workspace Source. It provides target-local File, Paragraph, and Line\naddresses without turning source history or editor state into Core identity.\n\nThe Core capability inventory is Search, View, Pick, Anchor, Check, Edit,\nApply, and Data. The repository currently provides their Rust Core/Runtime\nsurfaces and the canonical `bw` executable's one-shot human and JSON\nSearch/View/Check/Edit, raw View, Anddress-first one-shot Edit, Session Pick, batch\nCheck, Anchor, Edit, Apply, result-binding, explicit Data modes, and\nAdapter-owned Version and Update.\n\n## Quick start\n\nInstall the closed official `0.2.5` release with the command for your\nplatform.\n\nLinux, macOS, or WSL:\n\n```sh\ncurl -fsSL https://backwriter.pentagration.com/install.sh | sh\n```\n\nWindows PowerShell:\n\n```powershell\nirm https://backwriter.pentagration.com/install.ps1 | iex\n```\n\nWindows CMD:\n\n```cmd\ncurl.exe -fsSL https://backwriter.pentagration.com/install.cmd -o install.cmd && call install.cmd && del install.cmd\n```\n\nThe POSIX installer places `bw` at `$HOME/.local/bin/bw`; the PowerShell and\nCMD paths place `bw.exe` at `$HOME\\.local\\bin\\bw.exe`. They do not change\n`PATH`, a shell startup file, the PowerShell profile, or the registry. A fresh\ninstall prints `Installed Backwriter: <version>`; replacing an existing\ndestination prints `Updated Backwriter: <version>`. The executable path and\n`PATH` guidance are printed separately only when the installation directory is\nnot already on `PATH`.\n\n### Version\n\n```sh\nbw version\n```\n\nThe closed official `0.2.5` build prints exactly:\n\n```text\nBackwriter 0.2.5\n```\n\n### Update\n\n```sh\nbw update\n```\n\n`bw update` downloads and delegates to the current official installer. The\ncurrent manifest selects the closed `0.2.5` distribution. The installer\nreads that manifest, verifies the selected artifact, and installs or reinstalls\nthat manifest version only after validation succeeds. It does not run a\nbackground updater or compare release versions. The installer accepts only the\nexact immutable `0.2.4` manifest and current `0.2.5` manifest; `0.2.3`, `0.2.2`, `0.2.1`, `0.2.0`, stable\n`0.1.0`, and beta.3 acceptance is retired. The command still performs no\nversion comparison and adds no version guard.\n\nThe product is Backwriter. The source Cargo package and library crate are\n`backwriter` at source-ready unpublished `0.2.6`; a source-built `bw version`\nprints `Backwriter 0.2.6`. The sole canonical executable and external Adapter\ncommand are `bw`. There is no `backwriter` binary, alias, or wrapper. The\nofficial installer remains separate and selects the closed public `0.2.5`\ndistribution.\n\nProduction `src/**` remains byte-identical to Gate 6 candidate\n`22e6df23755cdc80b299b77be313d307b67bc37f`. Official artifacts, installers,\nmanifest, Update target, and the exact 68-file public distribution remain\npublished and closed `0.2.5` from Source Authority\n`e4022fc073e9df3928e1c3817b266ce92121a03c`. Update performs no version\ncomparison, so source-built `0.2.6` still installs or reinstalls that official\n`0.2.5` release until a separately authorized release closes the gap.\n\n## Anddress-first editing\n\nThe default source-checkout replacement flow is:\n\n1. Run `bw --json search ...`.\n2. Select one occurrence from `occurrences` and pass its exact embedded v5\n   `anddress` object unchanged as one argv value.\n3. Run `bw edit anddress '<opaque-v5-object>' '<new-content>'`.\n\nHuman Search rows are not encoded Anddress values and cannot be Edit input.\nTreat the selected JSON object as opaque: do not interpret or rewrite its hash,\nrange, length, or other fields. File and Paragraph Content is the exact\nreplacement. Line Content is body-only, rejects NUL, CR, and LF, and preserves\nthe None, LF, CR, or CRLF terminator carried by the exact v5 Line. A Line NUL\nis `edit.content_contains_nul`; a Line CR or LF is\n`edit.line_body_contains_terminator`. The latter explains that Backwriter adds\nthe current terminator and that advanced raw Session Edit/Apply owns exact\nextent replacement. Apply alone confirms that source state before publication.\nView or Pick may help a caller select a target; neither View nor Check is\nrequired.\n\nHuman success writes one exact LF-terminated receipt row. `Unchanged` is\nfollowed by the still-current input v5 object; `Changed` is followed by the\nfresh v5 object when the resulting File, Line, or unique Paragraph has one,\nand otherwise by `None`. With leading `--json`, the same result is the compact\nAdapter-only `bw.cli.edit.v1` object with `schema`, `outcome`, and `anddress`\nkeys in that order; `anddress` is the exact v5 object or JSON `null`. Reuse only\nthe address returned by the receipt. A changed Paragraph with `None` requires\nan explicit Search before later target work. Exit `1` is neither a stale-only\nclassification nor proof that source bytes are unchanged, so it must not\ntrigger automatic retry.\n\nOne-shot Content is either one UTF-8 argv value or the exclusive `--stdin`\nselector in that position; stdin is read to EOF after v5 address validation and\nbefore Runtime access. Both forms have the same target-specific Content rules.\nFile/Paragraph CR and LF are exact Content, while Line body Content never\nsilently strips a trailing newline. Literal `--json` and `--raw` remain exact\nContent; a literal `--stdin` Content value is supplied through standard input.\n\nRaw Session is the advanced composition surface for Insert/Delete/Move/Copy,\nPosition, Anchor/Data lifetime, explicit bindings, and separate Apply. It is\nnot a prerequisite or alias for ordinary Replace. Its existing raw\n`edit replace` accepts caller-provided exact range Content, including an\nexplicit terminator or multiline replacement, and `apply @edit` publishes it\nseparately. The caller owns the binding, index, quoting, terminator, and\npublication boundary. General replacement should use one-shot body Content\nfirst; no exact one-shot flag or alternate executor exists.\n\n### Shell-local references\n\n`bw shell` also provides a short-lived interactive flow without adding a Core\nwire or persistent identity:\n\n```text\nsearch line needle\nview @0\nreplace @1 replacement\ncheck @2 @3\nexit\n```\n\nQuote one direct-shell query or replacement argument when it contains\nwhitespace; the quotes are shell syntax and are not part of the exact literal\nor replacement Content:\n\n```text\nsearch line \"duplicate = one\"\nreplace @1 \"duplicate = two\"\n```\n\nSuccessful direct `search` and projected direct `view` append `@N` references\nin output order, including duplicates. Direct `check <REF>...` resolves every\nreference before Runtime access, writes one Current/NotCurrent/Unavailable\nstate per input, and appends a fresh slot only for Current. `replace @N <content>` uses the same\ntarget-aware Content rules as one-shot Edit and appends a fresh reference for\n`Unchanged` or `Changed` when the receipt has an Anddress; `Changed\\tNone`\nadds none. Slots are zero-based canonical unsigned decimals, append-only, and\ndiscarded at `exit` or EOF. `@name` and `@name[index]` remain the advanced raw\nSession forms; `let name = @N` explicitly clones a numeric slot into that\nexisting named Anddress binding. Raw `edit replace` and separate `apply @edit`\nremain the advanced exact-range path.\n\nThe default workspace is the process current working directory. An explicit\n`--workspace` must be absolute and is checked by Runtime. Search admits `.` by\ndefault; repeat `--admit LOGICAL_PATH` before `search` to narrow admission.\nAfter the query, repeat `--source LOGICAL_PATH` or `--subtree LOGICAL_PATH` to\nnarrow a Search scope. Without a scope selector, Search covers all admitted\nsources.\n\n## Current CLI scope\n\n`bw` currently implements Adapter-owned one-shot Version and Update, one-shot\nhuman or JSON Search, View, Check, and Edit, raw View, Anddress-first one-shot Edit,\nplus Session Pick, batch Check, Anchor, Edit, Apply, and Data:\n\n```text\nbw version\nbw update\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    view anddress <encoded-v5-Anddress>... [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    check anddress <encoded-v5-Anddress>...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --raw\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    check anddress <encoded-v5-Anddress>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n```\n\nVersion and Update do not call Backwriter Core or Runtime and create no Core\nwire or capability workflow.\n\nContent Search preserves Core literal validation, scope, projection, and\ndeterministic result order. The distinct `search /file` form validates one\nlogical path and returns the current File Anddress for an admitted regular\nUTF-8, NUL-free source regardless of whether it is empty or contains matching\ntext. Missing paths and directories return Empty; the form has no scope\nselectors or synthetic content query. View decodes v5 Anddresses and projects\neach to itself or one ancestor before reading its exact content. A single input\ndefaults to self projection. `--as` chooses one target kind; multiple inputs\nrequire both `--json` and `--as`. Check decodes every v5 operand before Runtime\naccess. One human input writes `Current`, `NotCurrent`, or `Unavailable`; a\nbatch requires `--json` and preserves one outcome per input. Search, View,\nCheck, and Edit `--json`\nwrite compact Adapter objects with exact embedded v5 Anddress objects where\napplicable; each is an Adapter schema, not a second Core wire. View uses the\nhard-cut `bw.cli.view.v2` outcomes array for both single and batch results;\nCheck uses the hard-cut `bw.cli.check.v2` ordered outcomes array.\nRaw View is an explicit Adapter exact-text mode that reuses the ordinary View\nprojection without a Core wire or changed View meaning.\nHuman Search, View, and Check keep their existing projections; human Edit\nreceipts intentionally return the exact current v5 object when one exists.\nThe Session holds one Runtime until EOF\nor `exit` and has explicit local Search, Pick, Anddress, Edit, View, and Check\nbindings plus non-aliasing Anchedress handles. It owns one explicit `DataStore`\nfor the Session only; names are typed and never persist past EOF or `exit`.\nSession Pick\npasses a named Search or Pick collection and an Adapter-parsed predicate directly\nto Core; Session batch Check passes a named matching outcome directly to its\nRuntime batch seam and prints only report counts. Session Anchor creates a live\nhandle only through `let <name> = anchor create <anddress-ref>`, views it through\n`view anchored @<name>`, and can invalidate its logical source with `anchor\ninvalidate-source <logical-path>`. One-shot Data and Anchor are intentionally\nunsupported because their DataStore and live-handle contracts require Session\nlifetime. One-shot Pick, raw Edit-object transport, and a separate\nApply transport await collection or Edit transport schema authority. The\ndistinct Anddress-first one-shot Edit above is implemented. Raw output other\nthan one-shot View and further Session behavior remain deferred.\n\n## Scope\n\nBackwriter is not Git, a file watcher, daemon, persistent index, or editor UI.\nIt reads admitted current Workspace Source through Runtime's safe no-follow\naccess. It does not model branches, merges, history, automatic re-evaluation,\nor editor buffers.\n\nApply uses its accepted current observation and does not coordinate concurrent\nwriters. Writers may race and one publication may overwrite another\nsource-visible change; hosts requiring a stronger guarantee coordinate outside\nBackwriter.\n\n## Build from source and test\n\n```sh\ncargo build --offline --locked --release\n./target/release/bw search line \"needle\"\n./target/release/bw --workspace /path/project search paragraph \"needle\"\ncargo test --offline --locked\n```\n\n## Official desktop distribution\n\nThe official distribution authority is\n[https://backwriter.pentagration.com](https://backwriter.pentagration.com).\nIt publishes the closed Backwriter `0.2.5` release for Linux/WSL x86_64,\nmacOS arm64, macOS x86_64, and Windows x86_64 from Source Authority revision\n`e4022fc073e9df3928e1c3817b266ce92121a03c`. Linux uses canonical target\n`x86_64-unknown-linux-musl`;\n`x86_64-unknown-linux-gnu` remains the local development/test-host target.\nmacOS uses `aarch64-apple-darwin` with minimum macOS 11.0 and\n`x86_64-apple-darwin` with minimum macOS 10.12. The macOS artifacts receive\nstatic cross-build verification but are not claimed to have been executed on a\nnative Mac before publication. Windows uses `x86_64-pc-windows-gnu` and the\ncanonical executable `bw.exe`; its static cross-build verification does not\nclaim native Windows, PowerShell, or CMD execution. Linux arm64 is not currently\nprovided, and no universal host-compatibility claim is made.\n\n`install.sh` reads the canonical manifest, verifies the downloaded artifact\nagainst the manifest SHA-256, and installs the verified binary at\n`$HOME/.local/bin/bw` with a same-directory rename. Concurrent same-user\nHOME mutation is caller-owned. The published `.sha256` sidecar is for manual\nverification and is not installer authority. Windows PowerShell verifies the\nsame manifest authority and exact ZIP, and installs to\n`$HOME\\.local\\bin\\bw.exe` without editing PATH or the PowerShell profile. The\nthree canonical install commands and the canonical `bw update` command are kept\ntogether in [Quick start](#quick-start).\n\nThe CMD command writes `install.cmd` in the current directory and removes it\nafter a successful installation. An existing file with that name is replaced.\n\nThe CRLF `install.cmd` checks `curl.exe` and `powershell.exe`, downloads exactly\nthe canonical `install.ps1` over HTTPS-only TLS 1.2-or-newer transport into a\ncollision-failing `%TEMP%` task directory, delegates all installation meaning,\ncleans the directory, and preserves the child exit code. It owns no manifest,\nSHA-256, ZIP, or installation logic. The Linux-hosted CMD regression is static;\nno native CMD execution is claimed.\nThe distribution provides no\npublisher-authenticity signature or trusted signing identity, background or\nautomatic update, telemetry, `sudo` execution, or automatic `PATH` or\nshell-startup-file change.\nGitHub is a public source and documentation mirror, not the distribution\nauthority. The complete beta.1, beta.2, beta.3, stable `0.1.0`, `0.2.0`, and\n`0.2.1` and `0.2.2` version directories remain unchanged and immutable. The\ncomplete `0.2.3`, `0.2.4`, and `0.2.5` version directories are immutable; the\n`0.2.5` release is closed. The active public tree has exactly 68 files, and\nthe current installers and manifest select `0.2.5`. Any later platform or\nversion requires separate Owner authority. Linux arm64, tags, GitHub Releases,\ncrates.io publication, and background or automatic update remain outside the\ncompleted publication. The earlier `0.2.4` publication added its eight\nversioned files and replaced only the two installers and manifest pointers\nwithout replacing any of the 48 prior versioned files or the CMD Adapter. Its\nsecond publisher run reused all 60 files without metadata change. The `0.2.5`\npublication then added its eight versioned files, replaced the two installers\nand manifest pointers, and reused all 68 files on its second run.\n\n## Architecture\n\n- [Current state](docs/current/now.md)\n- [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)\n- [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)\n- [CLI V1 authority](docs/architecture/backwriter-cli-v1.md)\n- [Verification](docs/development/verification.md)\n\n## License\n\n[MIT License](LICENSE)\n"}
```

````

#### D3 genie-new transcript

````text
# Independent documented-shell evaluation: new CLI

One uninterrupted run. All fixture body reads and edits used the specified bw binary. No oracle, repository source, private documents, or sibling arms inspected. Commands derived from the supplied public guide and Help. No failure or reset occurred.

## Result and decisions

Discovery returned eight Lines across four files. Paragraph View established primary targets @0, @3, @4, @7; the other four Paragraphs said secondary. Full-file View was taken before edits solely to verify byte preservation. Four body-only replacements returned fresh @20–@23; Check reported all Current. Final file Views confirm exactly one ASCII `3` to `5` substitution per file, no length or terminator changes, and unchanged secondary Content. LF, CR, CRLF, and final unterminated Line were preserved.

Exact byte differences, computed solely from bw View output, with zero-based offsets:

```json
[
  {
    "path": "unit-01.txt",
    "bytesBefore": 67,
    "bytesAfter": 67,
    "differences": [
      {
        "offset": 30,
        "before": "3",
        "after": "5"
      }
    ]
  },
  {
    "path": "unit-02.txt",
    "bytesBefore": 67,
    "bytesAfter": 67,
    "differences": [
      {
        "offset": 65,
        "before": "3",
        "after": "5"
      }
    ]
  },
  {
    "path": "unit-03.txt",
    "bytesBefore": 72,
    "bytesAfter": 72,
    "differences": [
      {
        "offset": 31,
        "before": "3",
        "after": "5"
      }
    ]
  },
  {
    "path": "unit-04.txt",
    "bytesBefore": 66,
    "bytesAfter": 66,
    "differences": [
      {
        "offset": 65,
        "before": "3",
        "after": "5"
      }
    ]
  }
]
```

## Metrics

- Product processes: 3 (top-level Help, shell Help, one interactive shell).
- Additional explicitly invoked utility processes: 3 (two public-guide head reads, one stty). Five command-launch calls each use the configured host shell; the interactive launch exec-replaced that host shell with bw. No task-decision script.
- Public bw command counts: Help 2; Search 1; View 3 (8 Paragraph inputs + 4 baseline File inputs + 4 final File inputs); Replace 4; Check 1 (4 inputs); exit 1.
- Capability work: Search 1, View 16 target reads, Check 4 target inputs; 4 successful body-only replacements (Edit/Apply path). No explicit raw Apply/Pick/Anchor/Data command. Internal capability invocation counts were not instrumented.
- Extra Content-only Search queries: 0. One Content query was the required discovery. Extra baseline preservation inspection: one View command / four File inputs.
- Failures: 0; interactive shell final exit 0.
- Elapsed task interaction: approximately 43 seconds (06:52:07 UTC immediately after initial documentation read through 06:52:50 UTC after final verification/exit). Initial Help/guide tool calls preceded the starting timestamp by under one displayed second; reporting/transcript work is excluded.
- Through transcript creation: 10 functions.exec wrapper calls, containing 5 exec_command, 5 write_stdin, 2 clock reads, and 1 apply_patch = 13 nested calls. No further fixture operation is planned.

## Capture limitations

All output strings below are exact JSON-escaped strings returned by execution tooling, preserving CR/LF. Initial Help was copied verbatim from its tool return; initial public-guide output was reconstructed from an identical second public-guide read, recorded below. No body was read by that utility. Execution tooling exposes a combined output field rather than independent stdout/stderr. Interactive execution used a PTY with `stty -echo -onlcr`, suppressing input echo and LF-to-CRLF output translation; stdout and stderr therefore remain merged, and separate exact stderr attribution cannot be proved. No error text appeared. PTY launch and intermediate shell inputs have no per-command OS exit status; the retained process finally exited 0. No hidden command output is omitted.

Working directory for every process: `/tmp/backwriter-gate5.74tHth/genie-new/workspace`.
Interactive launch actual utility argv: `["stty","-echo","-onlcr"]`; product argv: `["/tmp/backwriter-gate5.74tHth/gnu-target/release/bw","shell"]`. Shell Help argv: `["/tmp/backwriter-gate5.74tHth/gnu-target/release/bw","help","shell"]`. Other launch strings map literally to their whitespace-separated argv; all interactive input bytes are recorded below.

## Exact chronological transcript

### Event 1

```json
{
  "cmd": "/tmp/backwriter-gate5.74tHth/gnu-target/release/bw --help",
  "argv": [
    "/tmp/backwriter-gate5.74tHth/gnu-target/release/bw",
    "--help"
  ],
  "result": {
    "exit_code": 0,
    "output": "USAGE\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\n  bw help [<command>]\n\nGLOBAL OPTIONS\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\n  --json                     Select JSON output where the command supports it.\n  --raw                      Select raw View output only.\n\nCOMMANDS\n  shell    Reuse short references across search, view, replace, and check.\n  search   Discover current File, Paragraph, or Line Anddresses.\n  view     Read one or more current Anddresses.\n  edit     Replace one current Anddress.\n  check    Check one or more current Anddresses.\n  version  Print the Backwriter version.\n  update   Run the installed-platform updater.\n\nADDITIONAL HELP\n  bw help <command>\n  Advanced topics: pick, anchor, apply, data (raw Session only; no one-shot execution).\n\nGlobal options precede the command. Use bw help shell for ordinary short-ref work.\n"
  }
}
```

### Event 2

```json
{
  "cmd": "head -c 40000 /tmp/backwriter-gate5.74tHth/genie-new/public-guide.md",
  "argv": [
    "head",
    "-c",
    "40000",
    "/tmp/backwriter-gate5.74tHth/genie-new/public-guide.md"
  ],
  "result": {
    "exit_code": 0,
    "output": "# Backwriter\n\nBackwriter is a Rust Core and Runtime for current, structural text work over\nadmitted Workspace Source. It provides target-local File, Paragraph, and Line\naddresses without turning source history or editor state into Core identity.\n\nThe Core capability inventory is Search, View, Pick, Anchor, Check, Edit,\nApply, and Data. The repository currently provides their Rust Core/Runtime\nsurfaces and the canonical `bw` executable's one-shot human and JSON\nSearch/View/Check/Edit, raw View, Anddress-first one-shot Edit, Session Pick, batch\nCheck, Anchor, Edit, Apply, result-binding, explicit Data modes, and\nAdapter-owned Version and Update.\n\n## Quick start\n\nInstall the closed official `0.2.6` release with the command for your\nplatform.\n\nLinux, macOS, or WSL:\n\n```sh\ncurl -fsSL https://backwriter.pentagration.com/install.sh | sh\n```\n\nWindows PowerShell:\n\n```powershell\nirm https://backwriter.pentagration.com/install.ps1 | iex\n```\n\nWindows CMD:\n\n```cmd\ncurl.exe -fsSL https://backwriter.pentagration.com/install.cmd -o install.cmd && call install.cmd && del install.cmd\n```\n\nThe POSIX installer places `bw` at `$HOME/.local/bin/bw`; the PowerShell and\nCMD paths place `bw.exe` at `$HOME\\.local\\bin\\bw.exe`. They do not change\n`PATH`, a shell startup file, the PowerShell profile, or the registry. A fresh\ninstall prints `Installed Backwriter: <version>`; replacing an existing\ndestination prints `Updated Backwriter: <version>`. The executable path and\n`PATH` guidance are printed separately only when the installation directory is\nnot already on `PATH`.\n\n### Version\n\n```sh\nbw version\n```\n\nThe closed official `0.2.6` build prints exactly:\n\n```text\nBackwriter 0.2.6\n```\n\n### Update\n\n```sh\nbw update\n```\n\n`bw update` downloads and delegates to the current official installer. The\ncurrent manifest selects the closed `0.2.6` distribution. The installer\nreads that manifest, verifies the selected artifact, and installs or reinstalls\nthat manifest version only after validation succeeds. It does not run a\nbackground updater or compare release versions. The installer accepts only the\nexact immutable `0.2.5` manifest and current `0.2.6` manifest; `0.2.4`, `0.2.3`, `0.2.2`, `0.2.1`, `0.2.0`, stable\n`0.1.0`, and beta.3 acceptance is retired. The command still performs no\nversion comparison and adds no version guard.\n\nThe product is Backwriter. The source Cargo package and library crate are\n`backwriter` at published and closed `0.2.6`; a source-built `bw version`\nprints `Backwriter 0.2.6`. The sole canonical executable and external Adapter\ncommand are `bw`. There is no `backwriter` binary, alias, or wrapper. The\nofficial installer selects the closed public `0.2.6`\ndistribution.\n\nThe published `0.2.6` Source Authority's `src/**` is byte-identical to its Gate 6\ncandidate `c78e07f242035230e8b071d583491ac633f58d29`; that claim does not cover\nthis later checkout. Unpublished `0.3.0` Gates 1–4 add the private namespace\nboundary, complete direct shell View, discoverable Help and private CLI modules.\nCargo and `bw version` still report `0.2.6`; readiness and release remain pending.\nUse `bw help shell` for short-reference Search/View/Replace/Check and\n`bw help pick`, `bw help anchor`, `bw help apply`, or `bw help data` for advanced\nraw Session topics. These Help topics add no one-shot capability execution.\nR3 release closure is complete: Source Authority\n`09bb6c424081594bd86a95f04345b786ef9b46b6` supplies published and closed\n`0.2.6`. Official artifacts, installers, manifest, and Update now select\n`0.2.6` in the exact 76-file tree. Installers accept only exact `0.2.5` and\n`0.2.6` manifests. Update still performs no version comparison. Production\nRust, Cargo, tests, toolchain, and v5 wire are unchanged by release closure.\n\n## Anddress-first editing\n\nThe default replacement flow is:\n\n1. Run `bw --json search ...`.\n2. Select one occurrence from `occurrences` and pass its exact embedded v5\n   `anddress` object unchanged as one argv value.\n3. Run `bw edit anddress '<opaque-v5-object>' '<new-content>'`.\n\nHuman Search rows are not encoded Anddress values and cannot be Edit input.\nTreat the selected JSON object as opaque: do not interpret or rewrite its hash,\nrange, length, or other fields. File and Paragraph Content is the exact\nreplacement. Line Content is body-only, rejects NUL, CR, and LF, and preserves\nthe None, LF, CR, or CRLF terminator carried by the exact v5 Line. A Line NUL\nis `edit.content_contains_nul`; a Line CR or LF is\n`edit.line_body_contains_terminator`. The latter explains that Backwriter adds\nthe current terminator and that advanced raw Session Edit/Apply owns exact\nextent replacement. Apply alone confirms that source state before publication.\nView or Pick may help a caller select a target; neither View nor Check is\nrequired.\n\nHuman success writes one exact LF-terminated receipt row. `Unchanged` is\nfollowed by the still-current input v5 object; `Changed` is followed by the\nfresh v5 object when the resulting File, Line, or unique Paragraph has one,\nand otherwise by `None`. With leading `--json`, the same result is the compact\nAdapter-only `bw.cli.edit.v1` object with `schema`, `outcome`, and `anddress`\nkeys in that order; `anddress` is the exact v5 object or JSON `null`. Reuse only\nthe address returned by the receipt. A changed Paragraph with `None` requires\nan explicit Search before later target work. Exit `1` is neither a stale-only\nclassification nor proof that source bytes are unchanged, so it must not\ntrigger automatic retry.\n\nOne-shot Content is either one UTF-8 argv value or the exclusive `--stdin`\nselector in that position; stdin is read to EOF after v5 address validation and\nbefore Runtime access. Both forms have the same target-specific Content rules.\nFile/Paragraph CR and LF are exact Content, while Line body Content never\nsilently strips a trailing newline. Literal `--json` and `--raw` remain exact\nContent; a literal `--stdin` Content value is supplied through standard input.\n\nRaw Session is the advanced composition surface for Insert/Delete/Move/Copy,\nPosition, Anchor/Data lifetime, explicit bindings, and separate Apply. It is\nnot a prerequisite or alias for ordinary Replace. Its existing raw\n`edit replace` accepts caller-provided exact range Content, including an\nexplicit terminator or multiline replacement, and `apply @edit` publishes it\nseparately. The caller owns the binding, index, quoting, terminator, and\npublication boundary. General replacement should use one-shot body Content\nfirst; no exact one-shot flag or alternate executor exists.\n\n### Shell-local references\n\n`bw shell` also provides a short-lived interactive flow without adding a Core\nwire or persistent identity:\n\n```text\nsearch line needle\nview @0\nreplace @1 replacement\ncheck @2 @3\nexit\n```\n\nQuote one direct-shell query or replacement argument when it contains\nwhitespace; the quotes are shell syntax and are not part of the exact literal\nor replacement Content:\n\n```text\nsearch line \"duplicate = one\"\nreplace @1 \"duplicate = two\"\n```\n\nSuccessful direct `search` and projected direct `view` append `@N` references\nin output order, including duplicates. Direct `check <REF>...` resolves every\nreference before Runtime access, writes one Current/NotCurrent/Unavailable\nstate per input, and appends a fresh slot only for Current. `replace @N <content>` uses the same\ntarget-aware Content rules as one-shot Edit and appends a fresh reference for\n`Unchanged` or `Changed` when the receipt has an Anddress; `Changed\\tNone`\nadds none. Slots are zero-based canonical unsigned decimals, append-only, and\ndiscarded at `exit` or EOF. `@name` and `@name[index]` remain the advanced raw\nSession forms; `let name = @N` explicitly clones a numeric slot into that\nexisting named Anddress binding. Raw `edit replace` and separate `apply @edit`\nremain the advanced exact-range path.\n\nThe default workspace is the process current working directory. An explicit\n`--workspace` must be absolute and is checked by Runtime. Search admits `.` by\ndefault; repeat `--admit LOGICAL_PATH` before `search` to narrow admission.\nAfter the query, repeat `--source LOGICAL_PATH` or `--subtree LOGICAL_PATH` to\nnarrow a Search scope. Without a scope selector, Search covers all admitted\nsources.\n\n## Current CLI scope\n\n`bw` currently implements Adapter-owned one-shot Version and Update, one-shot\nhuman or JSON Search, View, Check, and Edit, raw View, Anddress-first one-shot Edit,\nplus Session Pick, batch Check, Anchor, Edit, Apply, and Data:\n\n```text\nbw version\nbw update\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    view anddress <encoded-v5-Anddress>... [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    check anddress <encoded-v5-Anddress>...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --raw\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    check anddress <encoded-v5-Anddress>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n```\n\nVersion and Update do not call Backwriter Core or Runtime and create no Core\nwire or capability workflow.\n\nContent Search preserves Core literal validation, scope, projection, and\ndeterministic result order. The distinct `search /file` form validates one\nlogical path and returns the current File Anddress for an admitted regular\nUTF-8, NUL-free source regardless of whether it is empty or contains matching\ntext. Missing paths and directories return Empty; the form has no scope\nselectors or synthetic content query. View decodes v5 Anddresses and projects\neach to itself or one ancestor before reading its exact content. A single input\ndefaults to self projection. `--as` chooses one target kind; multiple inputs\nrequire both `--json` and `--as`. Check decodes every v5 operand before Runtime\naccess. One human input writes `Current`, `NotCurrent`, or `Unavailable`; a\nbatch requires `--json` and preserves one outcome per input. Search, View,\nCheck, and Edit `--json`\nwrite compact Adapter objects with exact embedded v5 Anddress objects where\napplicable; each is an Adapter schema, not a second Core wire. View uses the\nhard-cut `bw.cli.view.v2` outcomes array for both single and batch results;\nCheck uses the hard-cut `bw.cli.check.v2` ordered outcomes array.\nRaw View is an explicit Adapter exact-text mode that reuses the ordinary View\nprojection without a Core wire or changed View meaning.\nHuman Search, View, and Check keep their existing projections; human Edit\nreceipts intentionally return the exact current v5 object when one exists.\nThe Session holds one Runtime until EOF\nor `exit` and has explicit local Search, Pick, Anddress, Edit, View, and Check\nbindings plus non-aliasing Anchedress handles. It owns one explicit `DataStore`\nfor the Session only; names are typed and never persist past EOF or `exit`.\nSession Pick\npasses a named Search or Pick collection and an Adapter-parsed predicate directly\nto Core; Session batch Check passes a named matching outcome directly to its\nRuntime batch seam and prints only report counts. Session Anchor creates a live\nhandle only through `let <name> = anchor create <anddress-ref>`, views it through\n`view anchored @<name>`, and can invalidate its logical source with `anchor\ninvalidate-source <logical-path>`. One-shot Data and Anchor are intentionally\nunsupported because their DataStore and live-handle contracts require Session\nlifetime. One-shot Pick, raw Edit-object transport, and a separate\nApply transport await collection or Edit transport schema authority. The\ndistinct Anddress-first one-shot Edit above is implemented. Raw output other\nthan one-shot View and further Session behavior remain deferred.\n\n## Scope\n\nBackwriter is not Git, a file watcher, daemon, persistent index, or editor UI.\nIt reads admitted current Workspace Source through Runtime's safe no-follow\naccess. It does not model branches, merges, history, automatic re-evaluation,\nor editor buffers.\n\nApply uses its accepted current observation and does not coordinate concurrent\nwriters. Writers may race and one publication may overwrite another\nsource-visible change; hosts requiring a stronger guarantee coordinate outside\nBackwriter.\n\n## Build from source and test\n\n```sh\ncargo build --offline --locked --release\n./target/release/bw search line \"needle\"\n./target/release/bw --workspace /path/project search paragraph \"needle\"\ncargo test --offline --locked\n```\n\n## Official desktop distribution\n\nThe official distribution authority is\n[https://backwriter.pentagration.com](https://backwriter.pentagration.com).\nIt publishes the closed Backwriter `0.2.6` release for Linux/WSL x86_64,\nmacOS arm64, macOS x86_64, and Windows x86_64 from Source Authority revision\n`09bb6c424081594bd86a95f04345b786ef9b46b6`. Linux uses canonical target\n`x86_64-unknown-linux-musl`;\n`x86_64-unknown-linux-gnu` remains the local development/test-host target.\nmacOS uses `aarch64-apple-darwin` with minimum macOS 11.0 and\n`x86_64-apple-darwin` with minimum macOS 10.12. The macOS artifacts receive\nstatic cross-build verification but are not claimed to have been executed on a\nnative Mac before publication. Windows uses `x86_64-pc-windows-gnu` and the\ncanonical executable `bw.exe`; its static cross-build verification does not\nclaim native Windows, PowerShell, or CMD execution. Linux arm64 is not currently\nprovided, and no universal host-compatibility claim is made.\n\n`install.sh` reads the canonical manifest, verifies the downloaded artifact\nagainst the manifest SHA-256, and installs the verified binary at\n`$HOME/.local/bin/bw` with a same-directory rename. Concurrent same-user\nHOME mutation is caller-owned. The published `.sha256` sidecar is for manual\nverification and is not installer authority. Windows PowerShell verifies the\nsame manifest authority and exact ZIP, and installs to\n`$HOME\\.local\\bin\\bw.exe` without editing PATH or the PowerShell profile. The\nthree canonical install commands and the canonical `bw update` command are kept\ntogether in [Quick start](#quick-start).\n\nThe CMD command writes `install.cmd` in the current directory and removes it\nafter a successful installation. An existing file with that name is replaced.\n\nThe CRLF `install.cmd` checks `curl.exe` and `powershell.exe`, downloads exactly\nthe canonical `install.ps1` over HTTPS-only TLS 1.2-or-newer transport into a\ncollision-failing `%TEMP%` task directory, delegates all installation meaning,\ncleans the directory, and preserves the child exit code. It owns no manifest,\nSHA-256, ZIP, or installation logic. The Linux-hosted CMD regression is static;\nno native CMD execution is claimed.\nThe distribution provides no\npublisher-authenticity signature or trusted signing identity, background or\nautomatic update, telemetry, `sudo` execution, or automatic `PATH` or\nshell-startup-file change.\nGitHub is a public source and documentation mirror, not the distribution\nauthority. The complete beta.1, beta.2, beta.3, stable `0.1.0`, `0.2.0`, and\n`0.2.1` and `0.2.2` version directories remain unchanged and immutable. The\ncomplete `0.2.3`, `0.2.4`, `0.2.5`, and `0.2.6` version directories are immutable;\nthe `0.2.6` release is closed. The active public tree has exactly 76 files,\nand the current installers and manifest select `0.2.6`. Any later platform or\nversion requires separate Owner authority. Linux arm64, tags, GitHub Releases,\ncrates.io publication, and background or automatic update remain outside the\ncompleted publication. The earlier `0.2.4` publication added its eight\nversioned files and replaced only the two installers and manifest pointers\nwithout replacing any of the 48 prior versioned files or the CMD Adapter. Its\nsecond publisher run reused all 60 files without metadata change. The `0.2.5`\npublication then added its eight versioned files, replaced the two installers\nand manifest pointers, and reused all 68 files on its second run. R3 adds the\neight `0.2.6` files and replaces the two installers and manifest last; its\nsecond run reuses all 76 files without byte or metadata change.\n\n## Architecture\n\n- [Current state](docs/current/now.md)\n- [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)\n- [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)\n- [CLI V1 authority](docs/architecture/backwriter-cli-v1.md)\n- [Verification](docs/development/verification.md)\n\n## License\n\n[MIT License](LICENSE)\n"
  }
}
```

### Event 3

```json
{
  "cmd": "/tmp/backwriter-gate5.74tHth/gnu-target/release/bw help shell",
  "result": {
    "chunk_id": "4ae29b",
    "wall_time_seconds": 0.000003669,
    "exit_code": 0,
    "original_token_count": 615,
    "output": "NAME\n  bw shell - reuse short references across search, view, replace, and check\n\nUSAGE\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n\nCOMMANDS\n  search <line|paragraph|file> <query> [--source PATH | --subtree PATH]...\n  search /file <logical-path>\n  view <REF>... [--as <line|paragraph|file>]\n  replace <REF> <CONTENT>\n  check <REF>...\n  let <NAME> = <REF>\n  exit\n\nREFERENCES AND CONTENT\n  @N is a zero-based, append-only reference from this process. It ends at EOF or exit.\n  @name is a named Anddress; @hits[0] selects an indexed Search/Pick binding.\n  let name = @N makes a named alias. Names cannot be redefined.\n  Double-quote arguments containing spaces. Quoted escapes are \\\\, \\\", \\n, \\r, and \\t.\n  One physical input line is one command; no single quotes, pipes, or EOF Content selector.\n  Line Replace accepts body only: NUL/CR/LF are rejected; the existing terminator is preserved.\n  File/Paragraph Replace uses exact Content. Changed source bytes stale old same-source refs,\n  including other locations. Use the fresh receipt or explicitly search again; never auto-retry.\n\nOUTPUT\n  Search appends one @N per result; Empty writes nothing.\n  View returns each input ref, a fresh ref, kind/location, and complete Content in input order.\n  View<TAB>REF<TAB>bytes=N starts a record, followed by fresh-ref metadata and N exact bytes.\n  The following LF and EndView line are display framing, not source Content.\n  RelationAbsent is shown at its input position without Content or a fresh slot.\n  Replace writes Unchanged/Changed with a fresh ref, or Changed<TAB>None without one.\n  Check writes one status per input; only Current appends a fresh ref. Check is optional.\n\nEXAMPLES\n  Start bw shell in a workspace with only note.txt containing needle plus CRLF, then enter:\n  search line needle --source note.txt\n  view @0 --as paragraph\n  let selected = @0\n  replace @selected \"new value\"\n  check @0 @2\n  view @3\n  exit\n\nADVANCED\n  let hits = search line needle\n  view anddress @hits[0]\n  Raw named View writes exact Content without direct-View framing.\n  See bw help pick, bw help anchor, bw help apply, and bw help data.\n\nFAILURES\n  Usage errors record exit 2; Runtime/source errors record exit 1; later commands still run.\n  Stdin/stdout failure ends the shell with exit 1. Partial output cannot prove delivery or\n  undo publication. --json/--raw and command-line operands are unavailable.\n\nSEE ALSO\n  bw help search\n  bw help edit\n"
  }
}
```

### Event 4

```json
{
  "cmd": "stty -echo -onlcr; exec /tmp/backwriter-gate5.74tHth/gnu-target/release/bw shell",
  "result": {
    "chunk_id": "8944b6",
    "wall_time_seconds": 1.00114906,
    "session_id": 20127,
    "original_token_count": 0,
    "output": ""
  }
}
```

### Event 5

```json
{
  "stdin": "search line \"retry_budget = 3\"\n",
  "result": {
    "chunk_id": "1f64a3",
    "wall_time_seconds": 1.000797233,
    "session_id": 20127,
    "original_token_count": 44,
    "output": "@0\tLine\tunit-01.txt:2\n@1\tLine\tunit-01.txt:5\n@2\tLine\tunit-02.txt:2\n@3\tLine\tunit-02.txt:5\n@4\tLine\tunit-03.txt:2\n@5\tLine\tunit-03.txt:5\n@6\tLine\tunit-04.txt:2\n@7\tLine\tunit-04.txt:5\n"
  }
}
```

### Event 6

```json
{
  "stdin": "view @0 @1 @2 @3 @4 @5 @6 @7 --as paragraph\n",
  "result": {
    "chunk_id": "0f889f",
    "wall_time_seconds": 1.000483504,
    "session_id": 20127,
    "original_token_count": 179,
    "output": "View\t@0\tbytes=32\n@8\tParagraph\tunit-01.txt:1-2\nrole = primary\nretry_budget = 3\n\nEndView\nView\t@1\tbytes=34\n@9\tParagraph\tunit-01.txt:4-5\nrole = secondary\nretry_budget = 3\n\nEndView\nView\t@2\tbytes=34\n@10\tParagraph\tunit-02.txt:1-2\nrole = secondary\rretry_budget = 3\r\nEndView\nView\t@3\tbytes=32\n@11\tParagraph\tunit-02.txt:4-5\nrole = primary\rretry_budget = 3\r\nEndView\nView\t@4\tbytes=34\n@12\tParagraph\tunit-03.txt:1-2\nrole = primary\r\nretry_budget = 3\r\n\nEndView\nView\t@5\tbytes=36\n@13\tParagraph\tunit-03.txt:4-5\nrole = secondary\r\nretry_budget = 3\r\n\nEndView\nView\t@6\tbytes=34\n@14\tParagraph\tunit-04.txt:1-2\nrole = secondary\nretry_budget = 3\n\nEndView\nView\t@7\tbytes=31\n@15\tParagraph\tunit-04.txt:4-5\nrole = primary\nretry_budget = 3\nEndView\n"
  }
}
```

### Event 7

```json
{
  "stdin": "view @0 @2 @4 @6 --as file\n",
  "result": {
    "chunk_id": "f805a1",
    "wall_time_seconds": 1.001154642,
    "session_id": 20127,
    "original_token_count": 115,
    "output": "View\t@0\tbytes=67\n@16\tFile\tunit-01.txt\nrole = primary\nretry_budget = 3\n\nrole = secondary\nretry_budget = 3\n\nEndView\nView\t@2\tbytes=67\n@17\tFile\tunit-02.txt\nrole = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 3\r\nEndView\nView\t@4\tbytes=72\n@18\tFile\tunit-03.txt\nrole = primary\r\nretry_budget = 3\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n\nEndView\nView\t@6\tbytes=66\n@19\tFile\tunit-04.txt\nrole = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 3\nEndView\n"
  }
}
```

### Event 8

```json
{
  "stdin": "replace @0 \"retry_budget = 5\"\nreplace @3 \"retry_budget = 5\"\nreplace @4 \"retry_budget = 5\"\nreplace @7 \"retry_budget = 5\"\n",
  "result": {
    "chunk_id": "9db1ae",
    "wall_time_seconds": 1.000935374,
    "session_id": 20127,
    "original_token_count": 31,
    "output": "@20\tChanged\tLine\tunit-01.txt:2\n@21\tChanged\tLine\tunit-02.txt:5\n@22\tChanged\tLine\tunit-03.txt:2\n@23\tChanged\tLine\tunit-04.txt:5\n"
  }
}
```

### Event 9

```json
{
  "stdin": "check @20 @21 @22 @23\nview @20 @21 @22 @23 --as file\nexit\n",
  "result": {
    "chunk_id": "4e2aac",
    "wall_time_seconds": 0.000003668,
    "exit_code": 0,
    "original_token_count": 147,
    "output": "@24\tCurrent\tLine\tunit-01.txt:2\n@25\tCurrent\tLine\tunit-02.txt:5\n@26\tCurrent\tLine\tunit-03.txt:2\n@27\tCurrent\tLine\tunit-04.txt:5\nView\t@20\tbytes=67\n@28\tFile\tunit-01.txt\nrole = primary\nretry_budget = 5\n\nrole = secondary\nretry_budget = 3\n\nEndView\nView\t@21\tbytes=67\n@29\tFile\tunit-02.txt\nrole = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 5\r\nEndView\nView\t@22\tbytes=72\n@30\tFile\tunit-03.txt\nrole = primary\r\nretry_budget = 5\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n\nEndView\nView\t@23\tbytes=66\n@31\tFile\tunit-04.txt\nrole = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 5\nEndView\n"
  }
}
```

### Event 10

```json
{
  "cmd": "head -c 40000 /tmp/backwriter-gate5.74tHth/genie-new/public-guide.md",
  "note": "Re-read public guide only to preserve the identical initial documentation stdout in transcript; no fixture read.",
  "result": {
    "chunk_id": "d67c14",
    "wall_time_seconds": 0.000005018,
    "exit_code": 0,
    "original_token_count": 4381,
    "output": "# Backwriter\n\nBackwriter is a Rust Core and Runtime for current, structural text work over\nadmitted Workspace Source. It provides target-local File, Paragraph, and Line\naddresses without turning source history or editor state into Core identity.\n\nThe Core capability inventory is Search, View, Pick, Anchor, Check, Edit,\nApply, and Data. The repository currently provides their Rust Core/Runtime\nsurfaces and the canonical `bw` executable's one-shot human and JSON\nSearch/View/Check/Edit, raw View, Anddress-first one-shot Edit, Session Pick, batch\nCheck, Anchor, Edit, Apply, result-binding, explicit Data modes, and\nAdapter-owned Version and Update.\n\n## Quick start\n\nInstall the closed official `0.2.6` release with the command for your\nplatform.\n\nLinux, macOS, or WSL:\n\n```sh\ncurl -fsSL https://backwriter.pentagration.com/install.sh | sh\n```\n\nWindows PowerShell:\n\n```powershell\nirm https://backwriter.pentagration.com/install.ps1 | iex\n```\n\nWindows CMD:\n\n```cmd\ncurl.exe -fsSL https://backwriter.pentagration.com/install.cmd -o install.cmd && call install.cmd && del install.cmd\n```\n\nThe POSIX installer places `bw` at `$HOME/.local/bin/bw`; the PowerShell and\nCMD paths place `bw.exe` at `$HOME\\.local\\bin\\bw.exe`. They do not change\n`PATH`, a shell startup file, the PowerShell profile, or the registry. A fresh\ninstall prints `Installed Backwriter: <version>`; replacing an existing\ndestination prints `Updated Backwriter: <version>`. The executable path and\n`PATH` guidance are printed separately only when the installation directory is\nnot already on `PATH`.\n\n### Version\n\n```sh\nbw version\n```\n\nThe closed official `0.2.6` build prints exactly:\n\n```text\nBackwriter 0.2.6\n```\n\n### Update\n\n```sh\nbw update\n```\n\n`bw update` downloads and delegates to the current official installer. The\ncurrent manifest selects the closed `0.2.6` distribution. The installer\nreads that manifest, verifies the selected artifact, and installs or reinstalls\nthat manifest version only after validation succeeds. It does not run a\nbackground updater or compare release versions. The installer accepts only the\nexact immutable `0.2.5` manifest and current `0.2.6` manifest; `0.2.4`, `0.2.3`, `0.2.2`, `0.2.1`, `0.2.0`, stable\n`0.1.0`, and beta.3 acceptance is retired. The command still performs no\nversion comparison and adds no version guard.\n\nThe product is Backwriter. The source Cargo package and library crate are\n`backwriter` at published and closed `0.2.6`; a source-built `bw version`\nprints `Backwriter 0.2.6`. The sole canonical executable and external Adapter\ncommand are `bw`. There is no `backwriter` binary, alias, or wrapper. The\nofficial installer selects the closed public `0.2.6`\ndistribution.\n\nThe published `0.2.6` Source Authority's `src/**` is byte-identical to its Gate 6\ncandidate `c78e07f242035230e8b071d583491ac633f58d29`; that claim does not cover\nthis later checkout. Unpublished `0.3.0` Gates 1–4 add the private namespace\nboundary, complete direct shell View, discoverable Help and private CLI modules.\nCargo and `bw version` still report `0.2.6`; readiness and release remain pending.\nUse `bw help shell` for short-reference Search/View/Replace/Check and\n`bw help pick`, `bw help anchor`, `bw help apply`, or `bw help data` for advanced\nraw Session topics. These Help topics add no one-shot capability execution.\nR3 release closure is complete: Source Authority\n`09bb6c424081594bd86a95f04345b786ef9b46b6` supplies published and closed\n`0.2.6`. Official artifacts, installers, manifest, and Update now select\n`0.2.6` in the exact 76-file tree. Installers accept only exact `0.2.5` and\n`0.2.6` manifests. Update still performs no version comparison. Production\nRust, Cargo, tests, toolchain, and v5 wire are unchanged by release closure.\n\n## Anddress-first editing\n\nThe default replacement flow is:\n\n1. Run `bw --json search ...`.\n2. Select one occurrence from `occurrences` and pass its exact embedded v5\n   `anddress` object unchanged as one argv value.\n3. Run `bw edit anddress '<opaque-v5-object>' '<new-content>'`.\n\nHuman Search rows are not encoded Anddress values and cannot be Edit input.\nTreat the selected JSON object as opaque: do not interpret or rewrite its hash,\nrange, length, or other fields. File and Paragraph Content is the exact\nreplacement. Line Content is body-only, rejects NUL, CR, and LF, and preserves\nthe None, LF, CR, or CRLF terminator carried by the exact v5 Line. A Line NUL\nis `edit.content_contains_nul`; a Line CR or LF is\n`edit.line_body_contains_terminator`. The latter explains that Backwriter adds\nthe current terminator and that advanced raw Session Edit/Apply owns exact\nextent replacement. Apply alone confirms that source state before publication.\nView or Pick may help a caller select a target; neither View nor Check is\nrequired.\n\nHuman success writes one exact LF-terminated receipt row. `Unchanged` is\nfollowed by the still-current input v5 object; `Changed` is followed by the\nfresh v5 object when the resulting File, Line, or unique Paragraph has one,\nand otherwise by `None`. With leading `--json`, the same result is the compact\nAdapter-only `bw.cli.edit.v1` object with `schema`, `outcome`, and `anddress`\nkeys in that order; `anddress` is the exact v5 object or JSON `null`. Reuse only\nthe address returned by the receipt. A changed Paragraph with `None` requires\nan explicit Search before later target work. Exit `1` is neither a stale-only\nclassification nor proof that source bytes are unchanged, so it must not\ntrigger automatic retry.\n\nOne-shot Content is either one UTF-8 argv value or the exclusive `--stdin`\nselector in that position; stdin is read to EOF after v5 address validation and\nbefore Runtime access. Both forms have the same target-specific Content rules.\nFile/Paragraph CR and LF are exact Content, while Line body Content never\nsilently strips a trailing newline. Literal `--json` and `--raw` remain exact\nContent; a literal `--stdin` Content value is supplied through standard input.\n\nRaw Session is the advanced composition surface for Insert/Delete/Move/Copy,\nPosition, Anchor/Data lifetime, explicit bindings, and separate Apply. It is\nnot a prerequisite or alias for ordinary Replace. Its existing raw\n`edit replace` accepts caller-provided exact range Content, including an\nexplicit terminator or multiline replacement, and `apply @edit` publishes it\nseparately. The caller owns the binding, index, quoting, terminator, and\npublication boundary. General replacement should use one-shot body Content\nfirst; no exact one-shot flag or alternate executor exists.\n\n### Shell-local references\n\n`bw shell` also provides a short-lived interactive flow without adding a Core\nwire or persistent identity:\n\n```text\nsearch line needle\nview @0\nreplace @1 replacement\ncheck @2 @3\nexit\n```\n\nQuote one direct-shell query or replacement argument when it contains\nwhitespace; the quotes are shell syntax and are not part of the exact literal\nor replacement Content:\n\n```text\nsearch line \"duplicate = one\"\nreplace @1 \"duplicate = two\"\n```\n\nSuccessful direct `search` and projected direct `view` append `@N` references\nin output order, including duplicates. Direct `check <REF>...` resolves every\nreference before Runtime access, writes one Current/NotCurrent/Unavailable\nstate per input, and appends a fresh slot only for Current. `replace @N <content>` uses the same\ntarget-aware Content rules as one-shot Edit and appends a fresh reference for\n`Unchanged` or `Changed` when the receipt has an Anddress; `Changed\\tNone`\nadds none. Slots are zero-based canonical unsigned decimals, append-only, and\ndiscarded at `exit` or EOF. `@name` and `@name[index]` remain the advanced raw\nSession forms; `let name = @N` explicitly clones a numeric slot into that\nexisting named Anddress binding. Raw `edit replace` and separate `apply @edit`\nremain the advanced exact-range path.\n\nThe default workspace is the process current working directory. An explicit\n`--workspace` must be absolute and is checked by Runtime. Search admits `.` by\ndefault; repeat `--admit LOGICAL_PATH` before `search` to narrow admission.\nAfter the query, repeat `--source LOGICAL_PATH` or `--subtree LOGICAL_PATH` to\nnarrow a Search scope. Without a scope selector, Search covers all admitted\nsources.\n\n## Current CLI scope\n\n`bw` currently implements Adapter-owned one-shot Version and Update, one-shot\nhuman or JSON Search, View, Check, and Edit, raw View, Anddress-first one-shot Edit,\nplus Session Pick, batch Check, Anchor, Edit, Apply, and Data:\n\n```text\nbw version\nbw update\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    view anddress <encoded-v5-Anddress>... [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    check anddress <encoded-v5-Anddress>...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --raw\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    check anddress <encoded-v5-Anddress>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n```\n\nVersion and Update do not call Backwriter Core or Runtime and create no Core\nwire or capability workflow.\n\nContent Search preserves Core literal validation, scope, projection, and\ndeterministic result order. The distinct `search /file` form validates one\nlogical path and returns the current File Anddress for an admitted regular\nUTF-8, NUL-free source regardless of whether it is empty or contains matching\ntext. Missing paths and directories return Empty; the form has no scope\nselectors or synthetic content query. View decodes v5 Anddresses and projects\neach to itself or one ancestor before reading its exact content. A single input\ndefaults to self projection. `--as` chooses one target kind; multiple inputs\nrequire both `--json` and `--as`. Check decodes every v5 operand before Runtime\naccess. One human input writes `Current`, `NotCurrent`, or `Unavailable`; a\nbatch requires `--json` and preserves one outcome per input. Search, View,\nCheck, and Edit `--json`\nwrite compact Adapter objects with exact embedded v5 Anddress objects where\napplicable; each is an Adapter schema, not a second Core wire. View uses the\nhard-cut `bw.cli.view.v2` outcomes array for both single and batch results;\nCheck uses the hard-cut `bw.cli.check.v2` ordered outcomes array.\nRaw View is an explicit Adapter exact-text mode that reuses the ordinary View\nprojection without a Core wire or changed View meaning.\nHuman Search, View, and Check keep their existing projections; human Edit\nreceipts intentionally return the exact current v5 object when one exists.\nThe Session holds one Runtime until EOF\nor `exit` and has explicit local Search, Pick, Anddress, Edit, View, and Check\nbindings plus non-aliasing Anchedress handles. It owns one explicit `DataStore`\nfor the Session only; names are typed and never persist past EOF or `exit`.\nSession Pick\npasses a named Search or Pick collection and an Adapter-parsed predicate directly\nto Core; Session batch Check passes a named matching outcome directly to its\nRuntime batch seam and prints only report counts. Session Anchor creates a live\nhandle only through `let <name> = anchor create <anddress-ref>`, views it through\n`view anchored @<name>`, and can invalidate its logical source with `anchor\ninvalidate-source <logical-path>`. One-shot Data and Anchor are intentionally\nunsupported because their DataStore and live-handle contracts require Session\nlifetime. One-shot Pick, raw Edit-object transport, and a separate\nApply transport await collection or Edit transport schema authority. The\ndistinct Anddress-first one-shot Edit above is implemented. Raw output other\nthan one-shot View and further Session behavior remain deferred.\n\n## Scope\n\nBackwriter is not Git, a file watcher, daemon, persistent index, or editor UI.\nIt reads admitted current Workspace Source through Runtime's safe no-follow\naccess. It does not model branches, merges, history, automatic re-evaluation,\nor editor buffers.\n\nApply uses its accepted current observation and does not coordinate concurrent\nwriters. Writers may race and one publication may overwrite another\nsource-visible change; hosts requiring a stronger guarantee coordinate outside\nBackwriter.\n\n## Build from source and test\n\n```sh\ncargo build --offline --locked --release\n./target/release/bw search line \"needle\"\n./target/release/bw --workspace /path/project search paragraph \"needle\"\ncargo test --offline --locked\n```\n\n## Official desktop distribution\n\nThe official distribution authority is\n[https://backwriter.pentagration.com](https://backwriter.pentagration.com).\nIt publishes the closed Backwriter `0.2.6` release for Linux/WSL x86_64,\nmacOS arm64, macOS x86_64, and Windows x86_64 from Source Authority revision\n`09bb6c424081594bd86a95f04345b786ef9b46b6`. Linux uses canonical target\n`x86_64-unknown-linux-musl`;\n`x86_64-unknown-linux-gnu` remains the local development/test-host target.\nmacOS uses `aarch64-apple-darwin` with minimum macOS 11.0 and\n`x86_64-apple-darwin` with minimum macOS 10.12. The macOS artifacts receive\nstatic cross-build verification but are not claimed to have been executed on a\nnative Mac before publication. Windows uses `x86_64-pc-windows-gnu` and the\ncanonical executable `bw.exe`; its static cross-build verification does not\nclaim native Windows, PowerShell, or CMD execution. Linux arm64 is not currently\nprovided, and no universal host-compatibility claim is made.\n\n`install.sh` reads the canonical manifest, verifies the downloaded artifact\nagainst the manifest SHA-256, and installs the verified binary at\n`$HOME/.local/bin/bw` with a same-directory rename. Concurrent same-user\nHOME mutation is caller-owned. The published `.sha256` sidecar is for manual\nverification and is not installer authority. Windows PowerShell verifies the\nsame manifest authority and exact ZIP, and installs to\n`$HOME\\.local\\bin\\bw.exe` without editing PATH or the PowerShell profile. The\nthree canonical install commands and the canonical `bw update` command are kept\ntogether in [Quick start](#quick-start).\n\nThe CMD command writes `install.cmd` in the current directory and removes it\nafter a successful installation. An existing file with that name is replaced.\n\nThe CRLF `install.cmd` checks `curl.exe` and `powershell.exe`, downloads exactly\nthe canonical `install.ps1` over HTTPS-only TLS 1.2-or-newer transport into a\ncollision-failing `%TEMP%` task directory, delegates all installation meaning,\ncleans the directory, and preserves the child exit code. It owns no manifest,\nSHA-256, ZIP, or installation logic. The Linux-hosted CMD regression is static;\nno native CMD execution is claimed.\nThe distribution provides no\npublisher-authenticity signature or trusted signing identity, background or\nautomatic update, telemetry, `sudo` execution, or automatic `PATH` or\nshell-startup-file change.\nGitHub is a public source and documentation mirror, not the distribution\nauthority. The complete beta.1, beta.2, beta.3, stable `0.1.0`, `0.2.0`, and\n`0.2.1` and `0.2.2` version directories remain unchanged and immutable. The\ncomplete `0.2.3`, `0.2.4`, `0.2.5`, and `0.2.6` version directories are immutable;\nthe `0.2.6` release is closed. The active public tree has exactly 76 files,\nand the current installers and manifest select `0.2.6`. Any later platform or\nversion requires separate Owner authority. Linux arm64, tags, GitHub Releases,\ncrates.io publication, and background or automatic update remain outside the\ncompleted publication. The earlier `0.2.4` publication added its eight\nversioned files and replaced only the two installers and manifest pointers\nwithout replacing any of the 48 prior versioned files or the CMD Adapter. Its\nsecond publisher run reused all 60 files without metadata change. The `0.2.5`\npublication then added its eight versioned files, replaced the two installers\nand manifest pointers, and reused all 68 files on its second run. R3 adds the\neight `0.2.6` files and replaces the two installers and manifest last; its\nsecond run reuses all 76 files without byte or metadata change.\n\n## Architecture\n\n- [Current state](docs/current/now.md)\n- [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)\n- [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)\n- [CLI V1 authority](docs/architecture/backwriter-cli-v1.md)\n- [Verification](docs/development/verification.md)\n\n## License\n\n[MIT License](LICENSE)\n"
  }
}
```


````
