# Backwriter 0.2.6 Operational Adapter & Verification Contraction — Source Note

Status: owner-provided planning source captured on 2026-09-04. This file
preserves requested direction and examples; it is task evidence, not active
implementation authority.

## Goal

`0.2.6` is an Adapter, help, and verification patch. Its governing sentence is:

> Explain what to type, what happens, and what comes back.

It makes the existing `bw` operational surface easier to invoke and verify
without changing Backwriter Core or Runtime meaning. Search remains discovery;
View, Check, and Replace retain their established roles. The intended shell
flow is:

```text
Search → ref → batch View → Replace → fresh ref → batch Check
```

The flow is an Adapter convenience, not a required Core capability order,
provenance claim, or durable lifecycle.

## Fixed carry-forward boundary

The following are immutable unless a later separately authorized task changes
them:

- Search's algorithm, literal matcher, structural path, traversal, order,
  tiers, storage, `SearchOutcome`, `bw.cli.search.v2`, v5 values/wire, output,
  and measured performance;
- existing Core and Runtime meaning, including v5 currentness, Host proof,
  Anchor, publication, and `Correct 1 / Safe Reject 6 / Wrong Apply 0`;
- raw Session as the advanced surface, including its explicit Edit/Apply and
  existing `@name[index]` binding form;
- closed source/public `0.2.5` artifacts, installer, Update, manifest, and
  68-file distribution, whose manifest SHA-256 is
  `2c8f19af7ee98be211e788f1e538a3bc476b554c614b6f07373572f16d09c2b7`.

`0.2.6` does not introduce v6, a Search wire revision, a persistent reference
registry, history, relocation, watcher, retry, transaction, CAS/lock,
rollback, a new parser framework, or Search throughput optimization. The
629 MiB Search evidence and musl throughput are not optimization targets for
this version.

## Adapter direction

### Help and usage

Top-level help will describe only global syntax, the capability list, and how
to obtain command help. Each command help page must use this fixed section
order:

```text
NAME
USAGE
DESCRIPTION
ARGUMENTS
OPTIONS
WHAT HAPPENS
OUTPUT
EXAMPLES
FAILURES
SEE ALSO
```

`bw help X` and `bw X --help` must be equivalent. Examples are executable
contract examples, not aspirational prose. Usage failures retain exits `0/1/2`
and will report a stable code, cause, usage, and hint. Only documented prefix
placement of canonical output options is promised. Trailing-option acceptance
is allowed only if it demonstrably simplifies the existing parser; operand
insertion is not allowed.

### Content transport and Replace

Line Replace accepts body content only, preserves the existing None/LF/CR/CRLF
terminator, and rejects NUL, CR, and LF without stripping. `--stdin` is
exclusive with positional Content and consumes input to EOF. File and Paragraph
Replace retain exact UTF-8 content and the existing NUL policy. Exact extent
and raw Apply remain ADVANCED; no new exact syntax exists before Gate 4 proves
its consumer.

The Adapter reuses `WorkspaceRuntime::apply_replace` for one-shot Replace. It
does not create a second executor, request DTO, history, old-to-new map, or
implicit retry.

### Process-local references and batch Check

References and aliases are process-local RAM only. They create no persistent
identifier, silent rebinding, relocation, history, or authenticity claim. The
existing `@name` and `@name[index]` forms and a possible concise `@N` form have
a collision rule that remains an explicit Gate 5 decision. Raw Session remains
ADVANCED rather than an alias for the shell flow.

Ordered batch Check must preserve input order, duplicates, and one state per
input. One malformed member is an all-usage failure before I/O. Gate 6 owns the
JSON schema KAT and may add a narrow ordered batch Runtime seam only if the
existing `check`, `check_search`, and `check_pick` seams cannot represent the
input without changing their meaning.

## Verification direction

New execution comparisons are candidate-versus-`0.2.5` only. N-2 and older
releases are evidence through task records, source revisions, and raw evidence,
not a routine three-version execution matrix. A three-version run is allowed
only for explicit regression recovery or a two-step migration. Gate 7 contracts
verification only after that evidence audit.

Every future Gate keeps the inherited v5 KATs, Search tier/order/duplicate
controls, currentness and publication failure boundaries, 268-test GNU/musl
baseline, and `Correct 1 / Safe Reject 6 / Wrong Apply 0` result. It must not
weaken these controls to make an Adapter path fit.

## Ordered implementation gates

1. Record authority and the N-1 verification boundary.
2. Consolidate help, usage-error, and KAT authority without a declarative
   parser rewrite; split `bw.rs` or CLI tests only when it removes duplication
   and never add another integration binary.
3. Add stable usage failure presentation and `--stdin` transport.
4. Close Line body/terminator and exact/advanced boundaries through actual
   consumers.
5. Add process-local reference and Replace ergonomics after resolving the
   collision rule.
6. Add ordered batch Check Adapter behavior and its exact JSON KAT only when
   the existing Runtime seams are insufficient.
7. Reaudit and contract tests/docs, including blind Dummy, Genie, and external
   evidence, before declaring verification complete.
8. Decide source readiness. Artifact reconstruction and publication require
   separately named owner authorization.

## Decisions intentionally left open

- the exact command grammar and help text for each capability;
- the stable usage-error code vocabulary and wording;
- whether trailing options reduce existing parser duplication;
- `@name`, `@name[index]`, and `@N` collision and precedence policy;
- the minimum batch Check Adapter input representation and JSON schema;
- whether a narrow ordered batch Check Runtime seam is required after an
  evidence audit;
- the Gate 7 proof required from blind Dummy, Genie, and external evidence;
- the source-readiness decision after Gates 2–7.
