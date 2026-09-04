# Backwriter 0.2.6 Operational Adapter & Verification Contraction — Roadmap

Status: planning evidence only. The companion [source note](2026-09-04-backwriter-0.2.6-operational-adapter-verification-contraction-source.md)
preserves owner direction; the tracker is the execution authority after Gate 1.

## Evidence labels

- **Owner note**: requirement preserved in the source note.
- **Repository**: current `0.2.5` source and active authority at
  `a9b47b06e0c4ac4c3058332f85a2885f47edd53a`.
- **Decision required**: a choice that must be closed in the named Gate before
  behavior or tests can be changed.

## Objective

Make the canonical `bw` Adapter explain what to type, what happens, and what
comes back while keeping `0.2.5` Core/Runtime/Search/v5 behavior intact. This
is an operational Adapter and verification contraction, not a capability,
identity, search, or release redesign.

## Current repository facts

**Repository:**

- `WorkspaceRuntime::apply_replace(&Edit)` already supplies the native
  Replace-only receipt seam; raw `apply(&Edit)` remains the public Rust and raw
  Session primitive.
- `WorkspaceRuntime::{check,check_search,check_pick}` already supply current
  Check collection behavior for their respective input carriers.
- `bw` has a handwritten parser, command-specific usage paths, top-level help,
  raw Session bindings, and `@name[index]` selection. These are existing
  consumers, not an authorization to preserve duplication.
- `0.2.5` is published and closed. GNU and musl each have 268 passing tests.

## Confirmed authority

**Owner note:**

- Search remains byte-for-byte and semantically unchanged; help and shell-side
  reference post-processing are the only Search-adjacent work.
- one-shot Replace reuses `apply_replace`; no second executor or generic
  command framework is allowed.
- raw Session remains the advanced explicit Edit/Apply surface.
- references are process-local only; batch Check is ordered and duplicate
  preserving; malformed batch input fails before I/O.
- routine execution comparison is candidate versus `0.2.5`; older releases are
  historical evidence unless a specifically justified recovery/migration needs
  three versions.

## Decision required

1. **Gate 2:** exact shared help, usage-error, and KAT ownership; keep the
   handwritten parser unless a direct deletion proves a smaller implementation.
2. **Gate 3:** stable usage-error vocabulary and the exact `--stdin` grammar.
3. **Gate 4:** exact advanced extent/raw Apply documentation and the Line body
   consumer proof; do not invent syntax before that proof.
4. **Gate 5:** collision/precedence between `@name`, `@name[index]`, and `@N`.
5. **Gate 6:** batch Check Adapter representation, KAT, and whether an ordered
   narrow Runtime seam is truly necessary.
6. **Gate 7:** evidence threshold for blind Dummy, Genie, and external
   verification before tests/docs may contract.
7. **Gate 8:** GO/NO-GO source readiness; distribution work is outside this
   roadmap and requires separate owner authorization.

## Gate sequence

The [tracker](2026-09-04-backwriter-0.2.6-operational-adapter-verification-contraction.md)
is the sole execution plan. It orders authority/N-1, help, errors/stdin, Line
body/exact boundaries, refs/Replace, batch Check, verification contraction,
and source readiness. It retains inherited KAT/currentness/publication controls
and `Correct 1 / Safe Reject 6 / Wrong Apply 0` throughout.

## Exclusions

Do not change v5/v6 identity, Search matching/traversal/order/tier/storage,
`SearchOutcome`, `bw.cli.search.v2`, Search output/performance, raw Session
semantics, persistence, history, relocation, watcher, retry, transactions,
CAS/locks, rollback, installers, artifacts, publication, or live operations.
