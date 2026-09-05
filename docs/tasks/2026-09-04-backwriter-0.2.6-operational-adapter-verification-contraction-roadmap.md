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

## Release follow-up plan — R3 prompt preparation, 2026-09-05

This appendix plans separate release work after Gates 1–8; it does not change
their original exclusions or claim publication has happened. The appended
[owner handoff](2026-09-04-backwriter-0.2.6-operational-adapter-verification-contraction-source.md#later-owner-handoff--release-r2-complete-2026-09-05)
preserves the R2 report. No release operation is run while drafting the prompt.

**Repository verified:** server is clean at
`30c005f9dcdff73103e9151d329c0bcfe9b7f022`; Backwriter is clean at
`09bb6c424081594bd86a95f04345b786ef9b46b6`; both match local `origin/main`.
The actual installer and publisher SHA-256 values match the owner's handoff.
The server's release README pins the source revision and four artifacts. Its
publisher README and `publish-0.2.6.sh` accept a canonical private bundle plus
the 68-file tree, a canonical interrupted prefix, or the completed 76-file
tree. The script validates before mutation and publishes the manifest last.

**Owner-reported, not freshly probed during prompt preparation:** live remains
`0.2.5`, with 68 files and unchanged services. The execution agent must verify
that precondition before any publication. Tracked source/roadmap changes from
this preparation are documents only, left unstaged; they must be reviewed and
committed under the execution prompt's named Git scope before builders require
a clean source checkout. Unrelated changes must be preserved.

**Proposed execution order:**

1. Read both repositories' current guards, the release and publisher contracts,
   and this handoff. Record code hashes, the live file/directory inventory and
   metadata, current manifest, and service PID/InvocationID/restart/listener.
   Preserve the locally managed tunnel and existing ingress configuration.
2. Use only the pinned builders and generator to reconstruct one task-local
   canonical bundle. Match all artifact, sidecar, manifest, and installer
   bytes; first execute the existing publisher against a private copy of the
   68-file tree. Unexplained mismatches stop publication, not weaken validation.
3. Under an explicit R3 execution request, run the unchanged
   `/home/NOEEZ/server/backwriter/publish/publish-0.2.6.sh` with the verified
   absolute bundle and `/var/lib/pentagration/backwriter/public` as root. Allow
   only the eight versioned files and two installers plus manifest transition.
   Re-run once for idempotence; preserve all 64 previous versioned files and
   CMD metadata. A failed operation permits only the publisher's verified
   prefix-resume path, never manual pointer repair or rollback.
4. Require 76 regular files, root-owned directories mode `0755`, files `0644`,
   and no symlinks, unknown files, or staging. Compare complete file metadata
   before and after the idempotent run. Probe every path with GET and HEAD on
   loopback and public HTTPS: bytes/SHA, length, MIME, bodyless HEAD, immutable
   versioned caching, no-store pointers/errors, and root/unknown 404.
5. Use isolated task-local HOME directories for fresh install, the actual
   public `0.2.5` binary's Update, and `0.2.6` reinstall. Compare installed
   binaries with the canonical Linux member and test help, stdin, v5 values,
   receipts, shell refs, ordered Check, CRLF, and stale rejection. Reuse exact
   unchanged R1/R2 suite evidence, with provenance, and run changed or
   unproven checks; do not repeat blind trials or performance benchmarks.
6. After endpoint and installation success, align active release documentation
   to closed `0.2.6`, exact 76 files, and `0.2.5`/`0.2.6` acceptance. Keep
   Source Authority `09bb6c4...` and historical Gate/R1/R2 states explicit.
   Commit/push only named documentation paths, then verify clean source,
   immutable production/deployment code, unchanged service state, and cleanup.

**Completion and risks:** publication plus exact endpoint/install/update
evidence establishes release closure. No native macOS/Windows/PowerShell/CMD
execution is implied by cross-builds or HTTP byte equality. Existing publisher
concurrency-lock/rollback/fsync/crash-durability limitations remain. R3 adds no
runtime code, release framework, service restart, DNS/tunnel change, credential
content access, real-user HOME change, tag, GitHub Release, or crates.io work.
