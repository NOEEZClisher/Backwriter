# Backwriter 0.2.6 Operational Adapter & Verification Contraction

Status: Gates 1–4 complete — authority, command-local help, actionable
errors/stdin, and the Line body/advanced exact-extent boundary. `0.2.5` remains the closed source,
package, CLI, installer, artifact, and public distribution. This tracker is
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
- The closed public root has 68 files and its manifest SHA-256 is
  `2c8f19af7ee98be211e788f1e538a3bc476b554c614b6f07373572f16d09c2b7`.
- GNU and musl each retain the closed 268-test result until a later Gate adds
  justified regression coverage.
- v5 values/wire, Search algorithm/matcher/structural path/traversal/order/
  tier/storage, `SearchOutcome`, `bw.cli.search.v2`, output bytes, and Search
  performance are immutable in this target.
- Existing raw `apply`, raw Session, Search/View/Check/Anchor/Host proof,
  publication, and failure contracts remain authoritative.
- No v6, persistent registry, history, relocation, watcher, retry,
  transaction, CAS/lock, rollback, release, artifact, publication, or live
  infrastructure work belongs here.

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

## Gate 5 — process-local references and Replace

Implement only process-local RAM references/aliases with no durable ID,
silent rebinding, relocation, history, or retry. Resolve the exact collision
policy for existing `@name`, `@name[index]`, and any proposed `@N` before code.
The normal shell flow is Search → ref → batch View → Replace → fresh ref →
batch Check; it is not a Core lifecycle. Raw Session stays ADVANCED.

## Gate 6 — ordered batch Check Adapter

Preserve input order, duplicates, and one output state per input. Validate the
whole batch before I/O; one malformed member is a usage failure for the whole
operation. Freeze an exact JSON KAT. Add a narrow ordered batch Check Runtime
seam only if evidence proves that `check`, `check_search`, and `check_pick`
cannot represent the Adapter need without semantic change.

## Gate 7 — verification and documentation contraction

Audit inherited and new evidence before deleting tests or prose. Candidate
execution compares only with N-1 `0.2.5`; N-2/older comparisons use task/SHA/raw
evidence except for an explicitly justified regression recovery or two-step
migration. Blind Dummy, Genie, and external evidence requirements must be
closed before contraction. Preserve v5 KATs, Search tier/order/duplicates,
currentness/publication failure controls, and `Correct 1 / Safe Reject 6 /
Wrong Apply 0`.

## Gate 8 — source readiness

Decide GO/NO-GO after Gates 2–7 with full inherited GNU/musl verification. A
GO may align source version/status only under explicit Gate 8 authority.
Artifact reconstruction, installer changes, public publication, and live
operations remain separately authorized work.
