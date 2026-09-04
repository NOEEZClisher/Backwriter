# Backwriter 0.2.6 Operational Adapter & Verification Contraction

Status: Gate 1 complete — authority only. `0.2.5` remains the closed source,
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

## Gate 2 — help, usage error, and KAT authority

Consolidate only demonstrated duplicate command-local help, usage error, and
KAT plumbing. Keep a handwritten parser unless deletion proves a simpler result.
No declarative parser rewrite or extra integration binary is allowed. Top-level
help describes global syntax, capability list, and additional help only.
Command help must order sections as `NAME`, `USAGE`, `DESCRIPTION`,
`ARGUMENTS`, `OPTIONS`, `WHAT HAPPENS`, `OUTPUT`, `EXAMPLES`, `FAILURES`, and
`SEE ALSO`; `bw help X` equals `bw X --help`, and examples execute.

## Gate 3 — failures and stdin

Retain exit codes `0`, `1`, and `2`. Usage failure presentation must have a
stable code, cause, usage, and hint. Decide the exact vocabulary before tests
freeze it. Add `--stdin` only as an XOR alternative to positional Content and
read it to EOF. Canonical output options are documented as prefix-only;
trailing acceptance is permitted only after a direct simplification proof, and
operand insertion is prohibited.

## Gate 4 — Line body and exact boundary

Prove the consumer before adding or exposing syntax. Line Replace accepts body
only, preserves None/LF/CR/CRLF, and rejects NUL/CR/LF without stripping.
File/Paragraph retain exact UTF-8 plus existing NUL policy. Exact extent/raw
Apply remains ADVANCED. One-shot Replace continues to use `apply_replace` and
does not change raw Apply.

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
