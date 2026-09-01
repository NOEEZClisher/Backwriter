# Backwriter 0.2.2 Anddress-First Editing

Status: Gates 1–4 complete; Gates 5–7 pending. Cargo, `bw version`, and the
published distribution remain `0.2.1`; the current checkout's implemented
`0.2.2` slice remains unpublished and is not source-ready.

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
literal `--json`/`--raw` Content, leading output-option and trailing
extra-operand rejection, exact no-op, and status/error boundaries. Existing
Apply and Anchor regressions continue to cover mutation, publication
uncertainty, Anchor reflection, and Host proof/invalidation without Core or
Runtime changes. The complete offline/locked suite passes 242 tests: 236
existing controls plus six Gate 2 CLI regressions.

## Gate 3 — content transport and machine output — complete without addition

The existing argv and writer paths are sufficient for the current operation.
Argv carries empty and Unicode Content, File and Paragraph CR/LF, and every
allowed Line body; NUL remains invalid. Search `--json` provides exact v4
Anddress objects. Edit produces no new target or result: success is exit `0`
with `OK` and LF, while usage and execution failures retain exit `2`/`1` and
stderr. Returning an Anddress would require an implicit re-search, and wrapping
the operation in JSON would not refine broad `Unavailable`, publication
uncertainty, or stdout failure after Apply. Exit `1` is not evidence that the
source is unchanged and creates no automatic retry authority.

OS argv length, shell quoting and newline portability, and process-list or
history exposure remain real constraints. Only a reproduced consumer failure,
measured payload requirement, or concrete security requirement may justify a
later Owner-selected single transport. This gate reserves no syntax or
implementation and adds no stdin, file, JSON, batch, formatter, parser, type,
dependency, Core/Runtime seam, retry, relocation, or raw Edit transport.

## Gate 4 — README, CLI, and AI surface alignment — complete

README and CLI authority now present JSON Search followed by exact opaque-v4
one-shot Edit as the default Replace flow. Human Search rows are not address
input, address fields remain opaque, View/Pick are optional selection aids,
Check is not required, and a successful old address is not reused. Exit `1`
does not authorize retry or prove unchanged source. The official installer,
Cargo version, and `bw version` remain the closed `0.2.1`; only this checkout
contains the unpublished Edit slice.

The same sole-Line `retry_budget = 3` plus CRLF fixture produced this evidence:

| Evidence | Anddress-first one-shot | Raw Session |
| --- | --- | --- |
| Process and operation accounting | Two processes and two one-shot capability operations when Search is needed; one Edit operation if the address is already known | One `bw shell` process; four work expressions plus one `exit` control expression |
| Selection | Exact 311-byte v4 object transferred unchanged from the JSON `anddresses` array | Search binding plus index; optional View confirms the terminator |
| Replacement responsibility | Adapter accepts body `retry_budget = 5` and preserves CRLF privately | Caller supplies `retry_budget = 5\r\n`, binds raw Replace Edit, then invokes Apply separately |
| Success | Exit `0`, exact `OK` plus LF, final CRLF bytes | Exit `0`, Search/View output then exact `OK` plus LF, byte-identical final CRLF bytes |
| Stale control | Reusing the old address separately exits `1` with Unavailable and preserves the already-edited bytes | Binding/Edit failures precede publication; Apply remains a distinct failure and publication boundary |

The comparison makes no timing claim and does not turn tool calls, processes,
or expressions into interchangeable counts. Its task-local exact-JSON extractor
is verification evidence, not a product tool, wrapper, dependency, or schema.
Raw Session remains the advanced Insert/Delete/Move/Copy, Position, Anchor/Data
lifetime composition surface rather than a Replace prerequisite or alias.

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
