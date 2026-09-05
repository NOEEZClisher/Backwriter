# Backwriter 0.3.0 grounded roadmap

## Status and authority

Planning only, recorded 2026-09-05. No Gate is completed by this document.
The [owner source note](2026-09-05-backwriter-0.3.0-independent-namespace-complete-view-source.md) is preserved separately;
BOX references below refer to that note. This roadmap distinguishes owner
requirements from inspected implementation and unresolved decisions. Active
architecture remains semantic authority; a future Gate 1 must reconcile the
target there and create the execution tracker before product changes.

The source note preserves the supplied text, with one terminal LF added.
Original attachment SHA-256:
`fae8aec9b9c03698fa0078ec715eaeb0fe2d46865c6973309e1f08d64ec73c21`.

Verified checkout: `/home/NOEEZ/backwriter`, main and origin/main at
`4cb15c23f978dbf0dc3d4fbe884170df181b399e`, initially clean.
Closed N-1 Source Authority: `09bb6c424081594bd86a95f04345b786ef9b46b6`
(`0.2.6`). N readiness identity is not yet assigned.
The read-only server checkout is `/home/NOEEZ/server`, main and origin/main
at `04bd84a5a386638e4bf7e4d2e60d092bda3c3d35`, initially clean.
Existing release evidence reports public 0.2.6, 76 files, 11 directories
including root, and an 876-byte manifest with SHA-256
`47001acd4831954a5106a3aac5b9fdfe0b36791144f355f52523cd0d0eb7d5f1`.
This planning pass does not rerun endpoints, native targets, or host services.
The previously audited user-installed `bw` is 0.2.2, not the N-1 comparator;
do not execute or update it to construct comparison evidence.

## Objective and preserved contracts

Owner BOX 1–2, 7, 12, 16, 27: establish the independent private namespace,
complete direct shell View, improve discoverability, relocate existing CLI
code, and contract current verification documentation. Reuse existing
execution paths; do not build another search/edit/state engine.

Preserve Search literal matching, traversal, ordering, multiplicity, schema
and algorithm; the permitted private-path filter change is distinct from
Search optimization. Preserve `artext.backwriter-anddress.v5`, fields,
canonical encoding, hash transcript domains, workspace coordinates, source
identity/currentness including Line count, Apply publication, Anchor and Host
proof semantics. Keep one-shot raw/JSON View exact Content and existing
Edit receipt/Check/Search schemas. Keep body-only Line replacement, decoded
None/LF/CR/CRLF preservation, NUL/CR/LF rejection, File/Paragraph exact
Content, one-shot stdin and advanced raw Edit/Apply. No EOF-based stdin for
direct shell Replace.

Refs remain process-local append-only RAM slots, with no silent rebinding,
reuse, disk persistence, session recovery, or history. Any source-byte change
stales old ordinary same-source addresses. Multiple Replace commands remain
multiple publications, not an atomic transaction. No migration, fallback
reader, automatic deletion, relocation, retry, rollback, CAS/locking, wire
compression, NDJSON Search, identity interning, or musl throughput project.

## Repository findings and contraction decisions

### Namespace: a reservation exists, a state store was not found

Inspected [Runtime](../../src/runtime.rs) `open_with_authority` opens an
admitted workspace and creates RAM vectors/proofs; it does not create a
private directory. `is_backwriter_spill` is an exact Runtime-root-relative
`.artext/bw` exclusion. The Unix/non-Windows branch is case-sensitive;
Windows compares its two components ASCII-case-insensitively.
Its callers are [Search](../../src/runtime/search.rs), [View](../../src/runtime/view.rs),
[Check](../../src/runtime/check.rs), [Apply](../../src/runtime/apply.rs) and
[Anchor](../../src/runtime/anchor.rs). Existing
[Search](../../tests/search.rs) and [View](../../tests/view.rs) tests distinguish
the root private subtree from `.artext/bw2`, other `.artext` children and
nested ordinary paths.

The [Protocol](../architecture/backwriter-text-coordination-protocol.md)
explicitly says no repository-local `.artext` is created; future spill
belongs to a host-provided system root. No implemented private-state
creator/reader/writer was found in the inspected Runtime/CLI paths.
Installer scripts also contain no `.artext` or `.bw` use.

Two different temporary mechanisms actually have consumers:

- [CLI](../../src/bin/bw.rs) `UpdateTemporary::create` uses
  `env::temp_dir()/backwriter-update-<nonce>` for installer handoff.
- Apply's `Temporary` uses an admitted destination parent, no-follow,
  create-new staging, and same-directory rename. Its
  `.env.artext-apply-edit-` name and
  `artext.backwriter-apply-edit-v1-temporary` transcript are not a
  `.artext/bw` state store.

Retain these mechanisms and locations; a global `artext` replacement would
change unrelated safety/identity contracts. Do not move Update or Apply
staging into a new workspace store.

**Owner-note discrepancy, to close at Gate 1:** BOX 3–4/23 assumes existing
storage IO, but this checkout exposes only a reserved exclusion boundary.
Inventory production creators/readers/writers, their callers and any external
documented consumers. If none exist, document the namespace as a hard-cut
private reservation with no current state IO; do not create a store or eager
`.bw` directory just to make the checklist applicable. Mark nonexistent
storage rows not applicable with evidence, not fictitious successes.
Preserve the existing root/base and Windows boundary policy. Retaining the
old exact subtree as an exclusion protects leftovers without reading them.

Do not inspect real historical state contents to resolve the discrepancy.
Use code/schema evidence and task-local old-state sentinels. If an actual
non-reconstructible-data consumer is found, stop the dependent cutover and
seek Owner authority; the no-migration/no-deletion boundary is not a license
to lose user data.

### Shell View: reuse the returned Content, remove output loss

[CLI](../../src/bin/bw.rs) `execute_session_ref_view` already receives
`ViewOutcome::Projected { anddress, content }`. It currently drops Content
through `{ anddress, .. }`. If any outcome is RelationAbsent, the separate
`write_session_relation_absent` branch prints only absent outcomes and
suppresses projected peers. Existing `write_session_ref_line`, `BufWriter`,
stream errors, `reserve_session_refs`, and raw/JSON View writers supply
reusable formatting/allocation/error boundaries.

Owner BOX 8–12/24 requires each input's chunk to identify its input ref,
fresh projected ref, kind/location and actual returned Content. Preserve
input order and duplicates. Emit RelationAbsent at its position, with no
new slot. Remove the all-results-or-only-absent presentation branch; do not
change Runtime's all-or-nothing failure semantics. Reserve required ref
capacity before emitting results, then consume outcomes without cloning
Content, re-resolving returned addresses or re-reading Source.

Fix and test a display contract that separates metadata/framing from exact
Content, including empty Content, LF/CR/CRLF/no-EOL and content resembling
framing text. Owner's arrow-format example is illustrative, not an existing
KAT. Byte length and/or explicit end-of-content/no-EOL markers are possible
choices; select one minimal unambiguous contract before coding. Do not
change the address format or the existing one-shot exact writers.

**Batch mismatch, to close before its implementation:** the explicit
`--as` branch calls `view_batch(&[Anddress], AnddressTarget)` once;
the omitted-`--as` branch loops over single self-Views, even for plural
inputs. That API accepts one common target, not per-input self targets.
Single input can use one `view`, plural common-target input one batch.
Mixed-kind plural self-View cannot be called a one-batch implementation of
the current API. Gate 1 must identify this unresolved behavior explicitly;
before Gate 3 changes it, obtain a bounded decision if satisfying all
requirements needs API/grammar expansion. Do not silently reject previously
accepted input, change projection, hide N single calls in a wrapper, or
invent a second executor. No extra Search/View is allowed for output.

### Help, physical modules and verification

`write_command_help` currently rejects pick/anchor/apply/data as
`capability.one_shot_unavailable`. Existing raw Session parsers and tests
are the source for real help syntax, operands, bindings, examples, outputs
and errors. Add help topics, not one-shot execution.

Top-level and shell help must make ordinary direct refs discoverable.
Include actual quoting and numeric/named forms, Content plus ref output,
body-only Line Replace, Current-only fresh Check slots and same-source
staleness. Validate complete examples including ref-producing commands.
The existing ten-section help-order test is not a 0.3.0 requirement:
the new note permits concise Version help. Keep a single source of canonical
usage for errors, and keep help/version before Runtime/download/private IO.

Current physical sizes are 3,178 lines in `src/bin/bw.rs`, 3,750 in
`tests/cli.rs`. Relocate existing parser/dispatch/help/output/error/shell
code into internal modules as in BOX 17. No new public crate, second parser,
executor, traits, factory or forwarding-only facade. Test modules are
conditional, remain inside the single CLI integration crate, and reuse
fixtures. Update structural `include_str!` checks to follow moved code
without deleting their distinct safety evidence.

Current `verification.md` is 1,401 lines / 91,137 bytes with old release
tables and common rules embedded in historical sections. Keep current policy,
actual N Source Authority, N-1, required matrix, current results and links.
First extract still-current shared rules; remove duplicates only after
existing task evidence is linked or unique evidence is preserved verbatim.
No `docs/history` directory or dedicated historical index was found in
the inspected tree. Gate 4 may add one minimal evidence index if none exists
then; do not grow a per-release index inside active verification. Preserve
old metrics, environments, path spellings and Source Authority facts.
Update production-equivalent SHAs only after actual tree comparison.

## Gates and dependencies

### Gate 1 — inventory and documentation authority only

Read current authority in the repository order and reconcile the new target
without rewriting historical facts. Record the path/consumer matrix above,
the absent-store discrepancy, mixed self-batch issue, output framing decisions
or explicit open items, unchanged wire/domain boundary, and the Gate 1–5
acceptance/error/verification matrix in a new execution tracker. Source and
roadmap remain planning evidence, not competing semantic authority.

Keep Cargo/CLI/source/public release at 0.2.6. Do not implement namespace,
writer, help/module/test reorganization or verification-history moves yet.
Check document links/fences/diffs and offline/locked metadata; reuse existing
285-test GNU/musl evidence after confirming the full relevant input boundary.
Close independently decidable authority; flag blocked downstream decisions
instead of silently manufacturing technical choices.

### Gate 2 — namespace boundary

Prerequisite: Gate 1 closes the real base/store inventory.
Reuse the common private-path filter for `.bw` plus the minimal legacy
exclusion; no Source-reader bypass, new path policy stack, or eager storage.
Audit admission, traversal, exact File/direct capability access, case policy,
no-follow/ordinary-directory safety, temporary cleanup, Update and installers.

Use task-local fixtures for absent/new/old-only/both roots, byte-identical old
sentinels, abnormal file/symlink rejection, exact component exclusion,
`.bw-notes` and other ordinary paths, and help/version noncreation.
An ignored subtree need not be opened to validate its contents. Storage IO
tests apply only to actual inventoried consumers; never add IO for a test.
Retain source memory/currentness and all unchanged v5 KATs.

### Gate 3 — complete single/batch direct shell View

Prerequisite: settle mixed-kind plural self projection without hidden
semantic expansion and fix minimal framing/slot/error KATs.
Reuse Runtime results and existing ref/writer/error machinery. Test
single self, Line-to-Paragraph/File, batch order/duplicates, mixed
Projected/RelationAbsent, all terminators, empty and delimiter-like Content,
resource/Runtime/stream failures and stable append-only refs.
Structural evidence must show one single View or one batch View as contracted,
zero output-motivated Search/View/re-resolution, and no Content collection
solely for display. Preserve one-shot/raw/JSON, raw Session and Replace.

### Gate 4 — help, modules and verification contraction

Prerequisite: output and namespace contracts are implemented and tested.
Make help examples executable; separate ordinary shell and advanced topics.
Physically relocate existing code; split tests only if it improves navigation
without multiplying integration binaries or fixture ownership.
Preserve usage/error mapping and unique test boundaries. Contract active
verification by evidence relocation/linking, not by dropping current rules.
Record actual before/after scope and live consumers; no speed claim from moves.

### Gate 5 — final verification, comparison and release boundaries

First close source readiness: final candidate GNU/musl suites, offline/locked
metadata/tree, fmt, all-target check, clippy with warnings denied, release build,
Help/Version and deterministic namespace/View/Edit/Check/raw Session smoke.
Reuse same-input results; package/build metadata changes require fresh binary
identity and Version KAT, not all historical benchmarks. Do not invent or
preassign 0.3.0 readiness SHA or fixed test count.

Run only N-1 0.2.6 and N candidate; no <=0.2.5 checkout/build/benchmark,
all-release validation or external grep/cat/sed/Git comparison arm. Compare
Dummy N-1/N and Genie N-1/N as four independently scoped AI evaluations.
Dummy uses public help and chooses its route; one-shot is not itself failure.
Genie uses the recommended shell route.

The note references a prior independent four-file fixture. Its exact bytes/
oracle were not located in this checkout's docs/tests during this audit.
Locate that evidence first. If unavailable, use a clearly documented
spec-conformant replacement shared by all four arms, independently define its
full-byte oracle and new fixture digest, and do not claim identity to a lost
benchmark. Required shape: eight duplicate Lines, four primary edits (one per
file), four untouched secondary Lines, LF/CR/CRLF/None.

Genie's reference flow is Search 1 + Paragraph batch View 1 + Replace 4 +
batch Check 1 + File batch View 1 = eight capability commands. Candidate
acceptance: first context View supplies decision-making Content; zero
content-fetch self-View, named-copy or raw-View workaround; zero terminator
mistakes/Wrong Apply; exact independent oracle. Keep existing stale rejection
regressions; do not generalize one Search to multiple edits in the same file.

Record processes, actual bw commands, model tool turns, unexpected CLI failures,
content-only extra commands, stdout/stderr bytes, model-visible output bytes
and elapsed separately. More returned Content is not a byte-count regression.
N=1 timing/turn improvements are observations, not mandatory ratio gates.
Publish measured improvements only with comparable fresh N-1/N conditions.

Artifact reconstruction, server installer/publisher changes, publication and
release-authority audit remain separately authorized execution slices after
source readiness, within Gate 5's release boundary. No such authority is
granted by this roadmap or the initial Gate 1 prompt. Retain pinned artifacts,
manifest-last/no-clobber release practices and truthful native/platform limits;
do not infer new locking, durability, rollback or service/DNS changes.

## Evidence reuse, handoff and open risks

Owner BOX 22 supersedes a full GNU/musl rerun at every small Gate. During
development use affected namespace/writer/help/module tests plus proportionate
format/check checks; run final current-target suites at the stable candidate.
For reuse compare production, tests/fixtures, build scripts, Cargo/lock,
toolchain, target/features/profile and relevant build flags. Record what was
run, reused or not executed. Source equality alone is insufficient.

Preserve unrelated changes and the index. No reset/stash/clean, history rewrite,
branch change, Actions, gh, tag, release, deployment, credential or real HOME
changes. Any future stage/commit/non-force push must name its permitted paths.
Temporary cleanup must validate exact owned targets and never traverse
real historical data or broad prefixes. Gate 1 planning creates no `.bw`
or `.artext` authority.

Open before implementation: actual private-state consumers/base (currently
none), mixed-kind plural self-View vs the single-target batch API, exact shell
framing/stream-failure slot reporting, and location of the original independent
fixture. Existing native macOS/Windows/PowerShell/CMD evidence gaps and
publication concurrency/rollback/fsync limits remain; this patch does not
claim to close them.
