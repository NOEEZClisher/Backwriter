# Backwriter 0.3.0 independent namespace and complete shell View

## Status — Gates 1–2 complete; Gates 3–5 pending

Recorded 2026-09-05. Gate 1 closes authority, audited discrepancies, open
decisions and acceptance. Gate 2 implements the existing namespace predicate
and focused regressions only. No version bump, readiness SHA,
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
| D1: mixed-kind plural self-View | Open before Gate 3. The current common-target API cannot express mixed self kinds in one call. Obtain a bounded Owner decision if satisfying the requirements needs API/grammar change; no silent rejection, projection change, per-kind calls disguised as one batch, or second executor |
| D2: exact framing and stream-failure slot reporting | Open before Gate 3 coding. Freeze one minimal exact byte contract/KAT for input/fresh refs, metadata, body length/end framing, empty/Unicode/LF/CR/CRLF/no-EOL and delimiter-like Content. Preserve reserve-before-output, no slot for absent/Runtime failure, append-only/no-reuse; distinguish partial write/flush failure and terminal stream error from Runtime failure. Do not promise delivery, publication cancellation or retry |
| D3: original four-file fixture | Locate before Gate 5. Repository docs/tests search found the older three-file control, not the exact independent four-file bytes/oracle. If unavailable, explicitly define one common spec-conformant replacement, independent full-byte oracle and new digest for all four arms; no lost-fixture identity claim |

D1–D3 block their downstream claims, not this documentation Gate. Exact framing
is not invented from the illustrative arrow example. No private storage base
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
| 3 — complete shell View | Pending, D1/D2 required first | Single self and Line-to-Paragraph/File; one single/batch call; ordered duplicate input/ref/Content mapping; mixed projected/absent peers; empty/terminator/framing KATs; reserve/allocation/Runtime/write/flush failures; zero Content-only extra observation; unchanged raw/JSON/Replace |
| 4 — help/modules/verification | Pending | Executable direct/named/quoted examples including ref producers, fresh Current slots and same-source staleness; real advanced Pick/Anchor/Apply/Data help; no new one-shot/parser/crate; private code relocation; tests split only for useful navigation within one CLI integration crate; current rules extracted before duplicate history links, unique evidence preserved verbatim |
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
