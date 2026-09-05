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

## Later owner handoff — Release R2 complete, 2026-09-05

This appended handoff preserves the owner's later completion report. The
original Gates 1–8 planning above remains historical evidence. This prompt
preparation does not execute or authorize live publication by itself.

- Backwriter Source Authority is
  `09bb6c424081594bd86a95f04345b786ef9b46b6`, source-ready `0.2.6`.
- Server R2 commit is `30c005f9dcdff73103e9151d329c0bcfe9b7f022`, pushed
  non-force from `ea9cdcbb565931218826aad988cbb6cafd64f151`. Both repositories
  were reported at `HEAD == origin/main` with clean worktrees and indexes.
- Installer acceptance is exact `0.2.5` plus `0.2.6`; `0.2.4` and earlier,
  altered, reordered, duplicate, and mixed manifests are rejected.
- POSIX installer: 14,200 bytes, SHA-256
  `fae43945969beb574133ffae7d378cd402a11702b69fd861d9cd0ba7c0393337`.
- PowerShell installer: 15,802 bytes, SHA-256
  `dd865fe62f67b9e4b46978ae40e72b7a0d56eac527b3706b4dae44c4ecd28239`.
- Publisher SHA-256:
  `0cb9359913462c447465bc2ee2c10ab80f925c274ba5843e4e62635bb0d06853`.
  It prepares the exact 68-to-76-file transition: eight versioned files,
  POSIX installer, PowerShell installer, and manifest last. All 64 previous
  versioned files and `install.cmd` are preserved. Prefix resume, eleven
  failure boundaries, idempotence, collisions, mutations, symlink, mode, and
  owner failures were verified.

Canonical private artifacts were regenerated:

| Target | Bytes | SHA-256 |
| --- | ---: | --- |
| Linux x86_64 | 460219 | `cb7783fdfdf726508f884d16e254d8c8daa1eb7a5640c249ce534f5b5103c89a` |
| macOS arm64 | 358257 | `997b18ed3c8ad43ca9a47e061f1e382ff5302d22a76fe45a029af524ba1335ba` |
| macOS x86_64 | 392024 | `9aa645bf5a2605f0eed10ccee21e8ddb40f639304f2990ea634ccfadb4f9a6dd` |
| Windows x86_64 | 906101 | `f90d0f8a9d6779d343db178c37879d30e2ce8bfb2d9d6b85526336d3d7221d50` |

The canonical manifest is 876 bytes with SHA-256
`47001acd4831954a5106a3aac5b9fdfe0b36791144f355f52523cd0d0eb7d5f1`.

Reported verification: installer 45, new publisher 58, previous publisher 57,
CMD static 12, and Origin 13 passing cases; Origin offline metadata/tree,
fmt/check/clippy/release passed. Linux Help/Version, Search v2/v5, View v2,
Check v2, stdin Edit, shell refs/Replace, ordered Check, and stale
nonpublication passed. GNU/musl 285-test results were reused because source
was byte-identical after R1. macOS/Windows were cross-built, not run natively;
PowerShell was statically checked because `pwsh` was absent. Private bundles,
fixtures, caches, and task-local cargo-zigbuild were removed.

The nine changed server paths were `AGENTS.md`, `README.md`,
`backwriter/install.sh`, `backwriter/install.ps1`,
`backwriter/release/README.md`, `backwriter/publish/README.md`,
`backwriter/publish/publish-0.2.6.sh`, `backwriter/tests/install.sh`, and
`backwriter/tests/publish-0.2.6.sh`.

Live publication was not performed. The reported public root remained the
closed 68-file `0.2.5` tree with the original manifest SHA-256
`2c8f19af7ee98be211e788f1e538a3bc476b554c614b6f07373572f16d09c2b7`.
Origin/cloudflared PID, InvocationID, restart counts, and the
`127.0.0.1:8080` listener were unchanged. Reported duration was about
32 minutes. The next proposed operation is separately scoped R3 publication.
