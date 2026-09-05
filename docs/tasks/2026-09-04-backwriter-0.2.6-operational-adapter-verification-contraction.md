# Backwriter 0.2.6 Operational Adapter & Verification Contraction

Status: Gates 1–8 and separately authorized R1–R3 release complete — closed `0.2.6`; authority, command-local help, actionable
errors/stdin, the Line body/advanced exact-extent boundary, shell-local
references plus high-level Replace, ordered batch Check, and verification/
documentation contraction. Source Cargo, `bw version`, official installers,
artifacts, manifest, and the 76-file public distribution now select `0.2.6`. This tracker is
execution authority; its companion [source note](2026-09-04-backwriter-0.2.6-operational-adapter-verification-contraction-source.md)
and [roadmap](2026-09-04-backwriter-0.2.6-operational-adapter-verification-contraction-roadmap.md)
remain preserved planning evidence.

## Governing rule

> Explain what to type, what happens, and what comes back.

The target is Adapter/help/verification contraction. It changes no Core or
Runtime meaning by implication and adds no required capability workflow.

## Fixed baseline and exclusions

- N-1 is closed `0.2.5` at
  `a9b47b06e0c4ac4c3058332f85a2885f47edd53a`.
- The N-1 public root at the Gates 1–8 baseline had 68 files and manifest SHA-256
  `2c8f19af7ee98be211e788f1e538a3bc476b554c614b6f07373572f16d09c2b7`.
- GNU and musl each retain the closed 268-test result until a later Gate adds
  justified regression coverage.
- v5 values/wire, Search algorithm/matcher/structural path/traversal/order/
  tier/storage, `SearchOutcome`, `bw.cli.search.v2`, one-shot output bytes,
  and Search performance are immutable in this target.
- Existing raw `apply`, raw Session, Search/View/Check/Anchor/Host proof,
  publication, and failure contracts remain authoritative.
- No v6, persistent registry, history, relocation, watcher, retry,
  transaction, CAS/lock, rollback, release, artifact, publication, or live
  infrastructure work belongs to Gates 1–8; R1–R3 required separate authority.

## Gate 1 — authority and N-1 boundary — complete

Gate 1 records the owner source, grounded roadmap, and this execution tracker.
It updates active authority only to state the future Adapter boundary. Rust,
Cargo, lockfile, toolchain, README, tests, server, and public state are
byte-identical to N-1. The existing parser, canonical writers, `apply_replace`,
raw Session, and Check group execution are retained as actual consumers; no
new parser, executor, result store, or Runtime seam is created.

## Gate 2 — help, usage error, and KAT authority — complete

One writer now owns top-level help and one command-local selection owns the
seven implemented command pages. The handwritten parser, Runtime execution,
normal writers, raw Session, and one integration test crate remain. No
declarative parser rewrite, second dispatch, wrapper, or extra integration
binary is added. Top-level help describes global syntax, capability list, and
additional help only. Command help orders sections as `NAME`, `USAGE`, `DESCRIPTION`,
`ARGUMENTS`, `OPTIONS`, `WHAT HAPPENS`, `OUTPUT`, `EXAMPLES`, `FAILURES`, and
`SEE ALSO`; `bw --help` equals `bw help`; `bw help X` equals `bw X --help`; and
examples execute against a task-local CRLF fixture. `--help` is help only when
it is the command's sole operand. Help returns before Runtime/source I/O and
the Update download. Existing usage failures still exit 2 and reuse top-level
help; stable code/cause/usage/hint work remains Gate 3. The three new CLI
regressions make the complete GNU and musl suites 271 tests each while keeping
the inherited 268-test N-1 evidence and all drift controls intact.

## Gate 3 — failures and stdin — complete

Top-level and one-shot usage failures now write a stable lowercase dot code,
cause, canonical usage extracted from the same help authority, and precise help
hint; they exit `2` with empty stdout. Execution/current-source/stream failures
remain exit `1`, and raw Session grammar/reporting stays unchanged. The tested
inventory distinguishes command/global placement and duplicate output, command
form/kind/target/address/request, missing/extra operands, unsupported output,
batch View projection, and unavailable one-shot capabilities.

One-shot Edit accepts exactly positional Content or `--stdin` at that position;
mixed/trailing Content is rejected before source I/O. It validates argv and v5
Anddress first, reads stdin to EOF as UTF-8, then opens Runtime and reuses the
existing `apply_replace`/receipt writers. File/Paragraph bytes are exact; Line
retains body-only NUL/CR/LF rejection and None/LF/CR/CRLF preservation. Empty,
Unicode, invalid UTF-8/NUL/Line break, multiple read chunks, source-byte
preservation, help equivalence/no-I/O, and raw Session controls are covered.
Gate 3 adds four CLI regressions, bringing complete GNU and musl suites to
275 tests. Output options remain prefix-only; trailing acceptance and operand
insertion remain prohibited.

## Gate 4 — Line body and exact boundary — complete

The existing one-shot `Edit::Replace → apply_replace → receipt writer` is the
only general Replace path. Its decoded v5 Line terminator is appended exactly
once; None/LF/CR/CRLF, empty, and Unicode bodies are preserved. NUL is the
target-independent `edit.content_contains_nul`; Line CR/LF is
`edit.line_body_contains_terminator`. Both are exact usage failures before
Runtime access, source mutation, or Unix inode change. File/Paragraph retain
exact UTF-8 CR/LF Content and the NUL rule.

The consumer audit confirms that raw Session `edit replace` already supplies
caller-owned exact v5 target-range Content and existing separate `apply @edit`
publishes it. Its terminator and multiline matrix proves that it is the
ADVANCED exact-extent surface. Therefore Gate 4 adds no exact one-shot command,
flag, DTO, Runtime/Core API, executor, parser, wire, terminator override,
implicit View/Search/Check, or retry. Raw Session grammar and Runtime/Core
production files remain unchanged. The complete GNU and musl suites each pass
276 tests.

## Gate 5 — process-local references and Replace — complete

`bw shell` owns one append-only `Vec<Anddress>` for the life of one process;
slots are zero-based canonical unsigned `@N` values and are never reused or
silently rebound. Names begin with an ASCII letter or underscore, so `@name`
and `@name[index]` retain raw Session meaning. `let name = @N` clones a slot
into the existing named `Anddress` binding; raw `edit`, `apply`, Pick, Anchor,
Data, and named raw View/Check keep their grammar and output.

Direct `search` appends complete Found results then writes `@N` plus target
location. Direct `view <REF>... [--as KIND]` reuses View or View batch,
preserves input order and duplicates, and issues refs only for a complete
Projected result set; `RelationAbsent` adds no ref. Direct `replace <REF>
<CONTENT>` reuses one-shot target-aware Content preparation and `Edit::Replace`
with `apply_replace`. It reserves its one possible slot before publication,
then writes `Unchanged` or `Changed` with a fresh ref, or `Changed\tNone`.
Search/View failure and `RelationAbsent` append nothing; Apply failure preserves
prior slots and source. No Core/Runtime API, wire, registry, persistence,
history, retry, or lifecycle is added. The normal convenience flow is Search →
ref → View → Replace → fresh ref; raw Session remains ADVANCED.

## Gate 6 — ordered batch Check Adapter — complete

`WorkspaceRuntime::check_batch(&[Anddress])` exposes the existing grouped
classifier's ordered `Current`, `NotCurrent`, and `Unavailable` states without
changing `check`, `check_search`, or `check_pick` filtering/report meaning.
Every input is decoded or resolved before Runtime access. One-shot JSON hard
cuts to `bw.cli.check.v2`, one outcomes array preserving input order and
duplicates; Current/Unavailable embed the input v5 object and NotCurrent uses
`null`. Human Check remains a one-input status line, while batch requires JSON.
Direct shell Check resolves all refs first, reserves once, then issues one fresh
numeric ref per Current occurrence only. Raw Session Check forms and aggregate
report output remain byte-identical.

## Gate 7 — verification and documentation contraction — complete

Candidate `c78e07f242035230e8b071d583491ac633f58d29` is compared only with a
task-local clean export of N-1 `0.2.5`
`a9b47b06e0c4ac4c3058332f85a2885f47edd53a`; no checkout and no N-2 binary
are used. The common duplicate/Paragraph fixture covers Help, usage failure,
stdin, Line body, shell references/Replace, and ordered Check. Candidate has
command-local Help, real stdin Content, direct numeric references, and ordered
Check; N-1 has neither command-local Help nor direct numeric-reference flow and
accepts `--stdin` as literal Content. This is descriptive Adapter evidence, not
a Search, wire, or performance comparison.

The blind packet contains only the candidate binary, fixture, and `bw --help`
plus command Help. Its transcript records duplicate Line Search, Paragraph
View, Line body `retry_budget = 3` to `retry_budget = 5`, fresh value reuse, and
two-current JSON batch Check. The final SHA-256 is
`271f454f74b7d04bf1a252feced3f3bdafa6754ac74c9d0f4391b419fe79675b`; it uses
17 tool calls, four unexpected CLI failures, zero manual raw-v5 constructions,
and zero terminator mistakes, within the 29/5 limits.

The trained packet is restricted to `bw --help`, `bw help shell`, and the
README's Anddress-first, shell-reference, and CLI sections. A preliminary
public-only trial exposed missing quoting guidance for whitespace. The README
now documents the existing quoted token form; the final first timed flow is:

```text
search line "duplicate = one"
view @0 @1 @2
replace @4 "duplicate = two"
check @6 @1
view @6
```

It has one Search, batch View, one Line-body Replace, immediate fresh-reference
reuse, batch Check, and final View. Old `@1` is `NotCurrent` without rebind;
post-Edit Search, raw-v5, raw Edit/Apply, individual Check retry, terminator
mistake, and unexpected Backwriter failure are zero. The final SHA-256 is
`084d54d2f243db7d40c11e841f57e00bcf862e41bc5e4af1ef474dedc30c5adc`; its n=1
elapsed record is 0.00 seconds, with difficulty/cognitive-load/confidence-risk
ratings `0/1/0` on the required 0–2 scale.

The external control stages the three-file duplicate/context fixture in a
task-local Git repository, uses `grep`, `cat`, and `sed` to make the three
`retry_budget = 3` to `retry_budget = 5` edits, then uses Git to inspect them.
All three final files have SHA-256
`73f090df1b5679f05bdddebeef9fec5b30e5fb7a87b8648e6ec6dff70c5cb31c`. A stale
old-text precondition fails and `git diff --exit-code` reports the pending
change, so the control performs no second write. It is not a winner, speed, or
release gate.

The audit retains every production test: v5 KAT, Search tier/order/duplicates,
exact Help/error/JSON KAT, stdin/terminator, refs, ordered Check, Host proof,
Anchor, currentness/publication, and distinct error boundaries have independent
consumers. `verification.md` removes 13 superseded Gate 1–6 lines in favor of
the current controls and tracker evidence. Production Rust, Cargo, lockfile,
and toolchain remain byte-identical to Gate 6.

## Gate 8 — source readiness — GO (historical source-only decision)

The decision matrix is GO. Top-level and seven command Help pairs, their fixed
section order and executable examples, usage exit 2, execution exit 1, and
success exit 0 remain exact. One-shot argv/stdin Edit parity, EOF UTF-8,
empty/Unicode/large Content, invalid UTF-8/NUL fail-closure, Line terminators,
raw Session exact extent, shell references, ordered duplicate-preserving Check,
Host proof, v5/Search immutability, and `Correct 1 / Safe Reject 6 / Wrong
Apply 0` all retain their Gate 2–7 regressions and evidence. GNU and musl each
pass 285 tests. Candidate production `src/**` is byte-identical to Gate 6.

Root Cargo/lock, exact Version KAT, and active authority therefore advance to
source-ready unpublished `0.2.6`. Artifact reconstruction, installer changes,
public publication, and live operations remain separately authorized. The
official installer, manifest, Update target, and public distribution remain
closed `0.2.5`; Update has no version comparison and may install that release
from a source-built `0.2.6` until a later release closure.

## 0.2.6 R3 release closure — GO, 2026-09-05

The unchanged pinned builders and generator reconstructed the canonical twelve-file
bundle from Source Authority `09bb6c424081594bd86a95f04345b786ef9b46b6`.
All four artifact hashes and sizes, sidecars, macOS UUIDs, and installer hashes
match server R2 `30c005f9dcdff73103e9151d329c0bcfe9b7f022`. The manifest is
876 bytes, SHA-256 `47001acd4831954a5106a3aac5b9fdfe0b36791144f355f52523cd0d0eb7d5f1`.
The unchanged publisher first passed a private 68-to-76-file transition, then
ran once as root against the live root: eight versioned files, POSIX installer,
PowerShell installer, and manifest last. One idempotent rerun preserved all 76
files' bytes, device/inode, owner, mode, size, mtime, and ctime. All 64 earlier
versioned files and `install.cmd` retained their before snapshot. The final tree
has 76 regular files and 11 directories including its root, directories `0755`
and files `0644`, all root-owned, with no symlink, unknown entry, or staging.

Canonical sorted JSON snapshot SHA-256 is
`a002515b938f0e8611ec0602ab83d055f1bad232a2f35c886e3fa32d1ce6dc49`
before publication and
`c0224fc082100436673a4d4e6d8c49a2d9ce655e4d18e8c996c7584f5d1c17b0`
both after publication and after reuse. It covers the file evidence above,
directory names, service identity/restarts, and listener. Origin PID 629 and
cloudflared PID 998 retain InvocationIDs
`cb7eb656fe5a4b8bac0f378ac8a84cf6` and
`6d43980020c142099d9003c94624b0ef`, zero restarts, and `127.0.0.1:8080`.

Loopback and public HTTPS each pass 76 GET and 76 HEAD requests plus four
root/unknown GET/HEAD controls: 312 checks total, exact SHA/length,
`application/octet-stream`, immutable versioned caching, no-store pointers and
404s, and zero actual HEAD body bytes. The endpoint evidence SHA-256 is
`829bf154b8a03367605c538b38f1062e7602bdd38e1ccc1377b4ada5ec597136`.
Isolated fresh install, the actual public `0.2.5` binary's Update, and `0.2.6`
reinstall produce exact Installed/Updated `0.2.6` output and byte-identical
canonical Linux binaries. Seventeen recorded installer/CLI commands cover
Help/Version, Search v2/v5, stdin Edit receipt, fresh single/batch View,
ordered duplicate Check v2, shell refs/Replace/fresh Check, CRLF, old-ref
NotCurrent and stale nonpublication, and raw Session Apply. The final source
SHA-256 is `cc326fa86d3e5924c488283058e530b9413d6acec0f4f78a954882f85f92edbf`.

Unchanged source/code/input evidence reuses GNU/musl 285 tests each, installer
45, publisher 58, CMD 12, and Origin 13; these are prior passing suites, not
newly executed R3 suite runs. R3 changes only release documentation and no
production Rust, Cargo, tests, toolchain, builder, installer, publisher, service,
Cloudflare, DNS, credential, or actual user HOME/PATH/startup state. Native
macOS/Windows/PowerShell/CMD execution remains unverified. Existing lack of
publisher locking, rollback, fsync, and crash-durability guarantees is unchanged.

The task-local bundle, temporary build tool/output, isolated HOME directories,
fixtures, and raw verification files were removed after recording their evidence.
Both repositories retain no new untracked output or `.artext` state.

## Post-release authority audit — 2026-09-05

Entry revisions are Backwriter `2e928bfa513cd970cbfc8677d1fbcc0bda368e00`
and server `7438148cf46d30f7be300d36fdecb154dc50c3c2`, both matching
`origin/main`. The only pending changes were the reviewed source/roadmap R3
handoff appendices, preserved unchanged by this audit.

Classification and corrections:

- Current authority: closed public `0.2.6`, Source Authority
  `09bb6c424081594bd86a95f04345b786ef9b46b6`, the 76-file tree, exact
  `0.2.5`/`0.2.6` installer acceptance, and version-comparison-free Update.
- Historical: N-1 `0.2.5` features, Gates 1–8's then-unpublished source,
  R1/R2's 68-file baseline, `0.2.5`-to-`0.2.6` Update, and old publisher
  prefix/resume inputs remain intact. Private reconstruction output remains a
  valid current tool contract alongside the published release; prepared inputs
  do not imply that publication is still pending.
- Stale: README's official-distribution paragraph still named `0.2.5` and its
  source revision. Older current-version/Update statements and pending Gate,
  stdin, ordered Check, and plural View wording conflicted with completed
  authority. Corrections scope old release claims to their closure and remove
  only already-completed items from current pending lists. Server README's
  ambiguous closure antecedent and the old publisher's "now" are historicalized.

Read-only checks compare all 38 Backwriter non-Markdown tracked files directly
with Source Authority bytes and all 35 server non-Markdown tracked files with
R2 `30c005f9dcdff73103e9151d329c0bcfe9b7f022`: no differences. Offline/locked
Cargo metadata succeeds for Backwriter `0.2.6` (67 resolved packages) and
Origin `0.1.0` (one package). The Version writer and KAT use `0.2.6`; Update
delegates without a version comparison. Both installer hash-selection branches
accept only the two canonical manifest digests before artifact download.

The local live tree has exactly 76 regular files and 11 directories including
the public root, root-owned with directory/file modes `0755`/`0644`, and no
symlink, unknown entry, or staging. All 76 file hashes match the unchanged
publisher constants and exact sidecar bytes. The manifest is 876 bytes with
the R3 digest above. Live and tracked POSIX/PowerShell/CMD installers match:

| Installer | Bytes | SHA-256 |
| --- | ---: | --- |
| POSIX | 14200 | `fae43945969beb574133ffae7d378cd402a11702b69fd861d9cd0ba7c0393337` |
| PowerShell | 15802 | `dd865fe62f67b9e4b46978ae40e72b7a0d56eac527b3706b4dae44c4ecd28239` |
| CMD | 1549 | `cb2708ab47a693eb1f79b01b1def3dd6d6cb87931848aebfaf3a6893326da3e4` |

Origin/cloudflared retain the R3 PIDs and InvocationIDs above, zero restarts,
active state, and `127.0.0.1:8080`. The removed raw snapshot serialization is
not reproduced. Directory names, hashes, sizes, owner/modes, and service fields
are checked individually; historical inode/mtime/ctime equality and equality
with the R3 composite snapshot fingerprint are not claimed. Remote DNS,
connector state, and current endpoint responses are not re-probed.

The existing user binary `/home/NOEEZ/.local/bin/bw` prints exact
`Backwriter 0.2.2` plus LF, exits zero, and has empty stderr. Its 836208 bytes
hash to `ba6a7486c9b7290f01fdfdb4296c979c2957653aef01ce15cd97c6f3faf898bf`
and equal the validated public `0.2.2` Linux archive member, not `0.2.6`.
This is an unchanged old user installation, not evidence against public
`0.2.6` closure or a new `0.2.6` runtime test. No install or Update is run.

GNU/musl 285 each, installer 45, publisher 58, CMD 12, and Origin 13 are reused
passing results under the byte-identity evidence. R3's 312 endpoint checks and
17 installer/CLI records remain past execution evidence, not fresh audit runs.
Markdown links/fences/conflict markers, diff/cached paths and whitespace,
empty starting indexes, and `.artext`/new tracked-output absence are audited.
No task-local artifact or fixture is created. No suite, build, benchmark, blind
trial, endpoint sweep, publication, service, credential, or actual HOME/PATH/rc
mutation is performed. Native macOS/Windows/PowerShell/CMD execution remains
unverified; publisher lock/rollback/fsync/crash-durability limits remain.
