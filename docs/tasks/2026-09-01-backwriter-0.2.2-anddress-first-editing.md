# Backwriter 0.2.2 Anddress-First Editing

Status: Gates 1–2 complete; Gates 3–7 pending. Cargo, `bw version`, source
behavior, and the published distribution remain `0.2.1`.

This tracker records progress and consumer evidence only. Normative meaning
belongs to the active [Protocol](../architecture/backwriter-text-coordination-protocol.md),
[address model](../architecture/rebuildable-structural-addressing.md),
[principles](../principles/backwriter-core-principles.md), and
[CLI authority](../architecture/backwriter-cli-v1.md). The Gate 2 CLI slice is
implemented; `0.2.2` is not source-ready, released, or published by this task.

## Gate 1 — authority and consumer inventory — complete

- Canonical general editing is one Adapter operation over an encoded v4
  Anddress and new Content. Caller-visible View, Check, binding, index, and
  Core Edit construction are not prerequisites.
- The minimum implementation reuses v4 decode, Runtime View, target-specific
  Content normalization, existing `Edit::Replace`, Runtime Apply, and existing
  CLI status/error writers. It adds no engine, state machine, or Runtime seam.
- File and Paragraph use exact replacement Content. Line accepts body Content
  without NUL, CR, or LF and preserves the current terminator returned by View.
- Apply remains currentness and publication authority. A source state that no
  longer matches between View and Apply is a safe `Unavailable`; no Check,
  relocation, context matching, retry, merge, history, fallback, or
  `NotCurrent` Apply alias is added.
- Public raw Core `Edit` and `Position` are consumed by Runtime Apply and direct
  Edit/Apply/Anchor regressions. Session `let ... = edit ...`, Edit binding
  cloning/index rejection, and `apply` are implemented end-to-end CLI
  consumers. They remain advanced/raw surfaces without a compatibility layer.

## Gate 2 — one-shot Adapter implementation and regressions — complete

The one-shot Adapter now composes strict v4 decode, one ordinary Runtime, one
private View, target-specific Content handling, existing `Edit::Replace`,
existing validation and Runtime Apply, then the existing status writer. CLI
regressions cover File, Paragraph, every Line terminator, empty and Unicode
Content, CR/LF Line rejection, malformed/stale/missing/unadmitted inputs,
output-option rejection, exact no-op, and status/error boundaries. Existing
Apply and Anchor regressions continue to cover mutation, publication
uncertainty, Anchor reflection, and Host proof/invalidation without Core or
Runtime changes. The complete offline/locked suite passes 242 tests: 236
existing controls plus six Gate 2 CLI regressions.

## Gate 3 — content transport and machine output — pending if needed

Use the minimum existing argv and writer path first. Define any additional
content transport or machine-oriented output only if a real consumer proves it
necessary. Do not prebuild JSON, stdin, file, batch, or generic formatter
abstractions.

## Gate 4 — README, CLI, and AI surface alignment — pending

After implementation evidence, align user-facing examples and AI guidance.
Compare the canonical operation against the raw Session path by explicit tool
calls and failure points rather than by unsupported performance claims.

## Gate 5 — raw/internal consumer reaudit and separation — pending

Reaudit every public raw Core Edit/Position/Apply and Session binding/index
consumer. Decide separation only from remaining consumers; do not remove,
rename, alias, or wrap the surface preemptively.

## Gate 6 — full integration and 0.2.2 readiness/version decision — pending

Run the complete semantic and integration matrix, confirm v4 and existing
0.2.1 behavior are unchanged, then make the separate source-readiness and
version decision. Phase 1 does not change Cargo or `bw version`.

## Gate 7 — artifact and publication — pending separate server approval

Artifact generation, installer/publisher changes, public-root mutation,
service verification, and release publication require a separate server task
and explicit approval after source readiness.
