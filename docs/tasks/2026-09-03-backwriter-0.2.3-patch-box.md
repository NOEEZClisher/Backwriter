# Backwriter 0.2.3 Patch Box

Status: Gates 1–7 complete. Source is ready and unpublished `0.2.3`; Gate 8 is
pending and requires its own scoped authorization.

This tracker records order, evidence, and unresolved implementation choices.
Normative meaning belongs to the active
[Protocol](../architecture/backwriter-text-coordination-protocol.md),
[address model](../architecture/rebuildable-structural-addressing.md),
[principles](../principles/backwriter-core-principles.md), and
[CLI authority](../architecture/backwriter-cli-v1.md). The published `0.2.2`
artifacts, installers, exact 44-file public tree, and service remain the closed
`0.2.2` baseline. Source Cargo and CLI version advance to unpublished `0.2.3`
only after Gate 7 GO.

## Goal and exclusions

`0.2.3` is an AI-facing information-surface patch over the existing engine.
Its target flow is:

```text
Search
-> identifiable observation information plus opaque v4 Anddress
-> requested View projection or ordered batch
-> one-shot Replace
-> fresh current Anddress when the published result has one
```

This is not a required capability order. Search remains the only finder, View
remains current Observe/Project, and Apply remains currentness and publication
authority. The work excludes engine performance, the million-hit matrix,
source-count scaling, File-View memory, history, diff, rollback,
predecessor/successor mapping, relocation, watcher, retry, merge, Git meaning,
persistent identity, registry, and retained observation state.

## Gate 1 — authority and consumer matrix — complete

### Search consumers

| Existing surface | Direct production consumers | Unique evidence and constraint |
| --- | --- | --- |
| `WorkspaceRuntime::search(&SearchRequest) -> Result<SearchOutcome, SearchError>` | Public Rust callers and CLI `run_search`; Runtime Check also consumes caller-provided `SearchOutcome` by value | Integration tests exercise content targets, exact File, traversal, currentness, and external-crate-style construction. Repository search cannot prove external Rust consumers absent |
| `SearchOutcome::{Empty, Found { occurrences: Vec<SearchOccurrence> }}` | Check filters and reconstructs complete occurrences; `DataStore` stores and returns it; Session stores, clones, indexes, checks, projects Pick candidates, and writes it | Each item owns its exact v4 Anddress and target-coherent optional `SearchPosition`; order, duplicate multiplicity, equality, Clone/Eq, Data ownership, and metadata-preserving Check filtering are public native meanings |
| Human `write_search` and raw-Anddress `write_address_rows` | One-shot Search, Session Search, Search binding display, and stored Search display use the former; Pick uses the latter | Search rows show current Line positions. Pick rows retain exact byte ranges and are not an accidental Search presentation consumer |
| Streaming `write_search_json` / `bw.cli.search.v2` | Documented JSON Search-to-one-shot Edit flow and machine-output tests | Exact envelope and item key order, position shape, embedded v4 objects, result order, duplicates, escaping, no whole-result clone, and error streams are current source contracts; v1 is immutable `0.2.2` release evidence only |
| Runtime provisional target projection | Content Search constructs target ranges while the sole selected-source observer hashes and frames the source | This is the only permitted producer for Line numbers and Paragraph Line ranges; an Adapter reread or second Search projection engine is forbidden |

Gate 2 hard-cuts the source-level machine boundary from `bw.cli.search.v1` to
`bw.cli.search.v2`. There is no simultaneous v1 mode, compatibility flag, dual
writer, or second Search engine. The published `0.2.2` v1 schema remains
immutable release evidence. The selected native carrier keeps Search, Check,
Data, Session, Pick, and public Rust meanings coherent without wrapping old
results in a parallel observation subsystem.

Every v2 result item is self-identifying: logical path, target kind, current
one-based Line number or Paragraph inclusive Line range when applicable, and
the exact opaque v4 Anddress. File omits Line position by default. Duplicate
occurrences and equal values remain in their original order. Descriptive
positions are not Anddress fields, equality/currentness evidence, selectors,
or Edit inputs.

### View consumers

| Existing surface | Direct production consumers | Unique evidence and constraint |
| --- | --- | --- |
| `WorkspaceRuntime::view(&Anddress, AnddressTarget)` | Public Rust callers, one-shot View, Session View, and one-shot Edit's private terminator lookup | Ordinary View owns exact-state validation and one accepted current observation; existing Adapter consumers request self projection |
| `WorkspaceRuntime::view_batch(&[Anddress], AnddressTarget)` | Public Rust callers only; no Adapter or Anchor batch consumes it | Ordered borrowed inputs use one projection, retain order and duplicates, and return all outcomes or none after one direct observation per Untrusted/proof-miss group or one trusted handle per matching Host group |
| `WorkspaceRuntime::view_anchored(&Anchedress, AnddressTarget)` | Session anchored View and public Anchor callers | It shares the same projection machinery while retaining Anchor liveness, fail-closure, and Host-proof semantics |
| `ViewOutcome` | Human/raw/JSON writers, Session bindings, `DataStore`, ordinary and anchored callers | Target variants now own their projected current Anddress and exact Content; Line retains File and optional Paragraph relations, Paragraph retains File, and `RelationAbsent` represents only a missing Line-to-Paragraph relation |
| Runtime View target projection | Ordinary, trusted, anchored, and batch paths capture the requested self-or-ancestor target and related Paragraph boundaries | Gate 3 exposes one explicit projection without a finder or generic graph. Gate 4 groups exact source keys and feeds the existing projection machinery instead of looping over the public single call |

The only allowed relation matrix is Line to Line, Paragraph, or File;
Paragraph to Paragraph or File; and File to File. Downward projection, implicit
Search, relocation, context matching, descendants, arbitrary ranges, and
partial results are excluded. Batch uses one requested projection, preserves
input order and duplicate multiplicity, returns all results or none, and
reuses one accepted current observation per logical source.

### Edit, Apply, and Anchor consumers

| Existing surface | Direct production consumers | Unique evidence and constraint |
| --- | --- | --- |
| One-shot `execute_edit` | General CLI caller | It composes strict v4 decode, private View, target-specific Content handling, existing `Edit::Replace`, Runtime Apply, and the shared status writer. It performs no Search |
| Public `Edit`, `Position`, and `WorkspaceRuntime::apply` | External Rust callers, raw Session binding/clone/index checks, separate Session Apply, and direct Apply/Anchor tests | All five operations and four positions, exact geometry, publication failures, and stale fail-closure are distinct supported evidence and remain unchanged by the one-shot receipt |
| Apply `AfterProjector`, completed after hash/length, and `reflection_plan` | Prospective output construction, Host-proof installation, and live same-path Anchor reflection | This is the single reusable producer for a receipt's resulting state and ranges. It currently projects live Anchor candidates; Gate 5 must contract or extend this path rather than add post-publication Search |
| Anchor reflection and invalidation | Live Runtime-local Anchor handles and anchored View | Receipt construction may share the prospective-after evidence but must create no Anchor, continuity, old-to-new map, or persistent state |

No audited production path is dead or safely deletable in Phase 1. The
simplification decision is therefore to retain its actual consumers, reuse the
single Search observation, existing View projection machinery, and Apply's
prospective-after path, and forbid parallel engines, generic projectors,
observation objects, post-Search, and speculative compatibility layers.

## Gate 2 — Search observation metadata — complete

- `SearchOccurrence` owns one exact v4 Anddress and optional
  `SearchPosition`. File requires no position, Line requires a nonzero one-based
  Line, and Paragraph requires a nonzero inclusive ordered Line range. Its
  constructor, borrowed getters, Clone/Eq, and consuming Anddress conversion are
  the only new public carrier surface.
- Line and Paragraph projections increment checked Line state inside the
  existing selected-source scan. They preserve CR, LF, CRLF, bare CR, no-EOL,
  empty and separator Lines, Unicode, no synthetic EOF Line, literal tiers,
  ordering, and duplicate multiplicity. Arithmetic failure follows the existing
  Resource-to-Unavailable path.
- Check preserves occurrence metadata for Current and Unavailable results while
  reports remain raw Anddresses. Data and Session retain the carrier; Session
  indexing and Pick candidate projection extract only its Anddress. PickOutcome
  and Pick human rows are unchanged.
- Machine Search hard-cuts to `bw.cli.search.v2`: envelope keys are `schema`,
  `outcome`, `occurrences`; each item is logical path, kind, applicable decimal
  Line field or Paragraph range, then directly streamed exact v4 `anddress`.
  Human Search uses path-only File, `path:line`, and `path:start-end` through one
  Search writer. No v1 production branch, JSON value tree, result clone, second
  result collection, source reopen, extra read, second hash pass, cache, or
  retained observation was added.

## Gate 3 — single View projection — complete

- `WorkspaceRuntime::view` and `view_anchored` accept the existing
  `AnddressTarget` as the requested projection. No request DTO, relation enum,
  old-signature alias, facade, or second executor exists.
- The exact allowed matrix is Line-to-Line/Paragraph/File,
  Paragraph-to-Paragraph/File, and File-to-File. Source-less v4 validation
  precedes relation validation; every downward request returns `InvalidInput`
  before source I/O.
- File, Paragraph, and Line outcomes own their projected current v4 Anddress
  and exact Content from the accepted observation. Existing related File and
  optional Paragraph addresses remain. A separator or raw-valid nonstructural
  Line projected to Paragraph returns the normal `RelationAbsent` outcome.
- One generalized `DirectViewProjection`, trusted fixed-scratch range and
  Paragraph-boundary path, and `finish_outcome` serve ordinary, matching Host,
  and anchored execution. `TargetProjection` remains only Anchor structural
  validation and does not reject ordinary raw-valid ranges.
- One-shot and Session View, anchored Session View, one-shot Edit, result
  binding, and Data continue to consume self projection. Their grammar and
  human/raw/JSON bytes are unchanged. Gate 3 adds no batch loop, CLI projection
  syntax, Search, relocation, cache, retry, state, dependency, or v4 change.

## Gate 4 — ordered batch View — complete

- `WorkspaceRuntime::view_batch(&[Anddress], AnddressTarget)` accepts one
  ordered borrowed collection and one projection. Empty input is an I/O-free
  empty success; nonempty success restores exact input order, duplicates, and
  per-item `RelationAbsent` outcomes.
- Every input runs source-less v4 and relation validation in input order before
  complete coordinate, spill, and admission preflight or I/O. Inputs then sort
  only as indices by workspace coordinate and logical path; provisional output
  slots restore caller order and remain private until every group succeeds.
- An Untrusted or Host-proof-miss group opens one source and feeds all existing
  `DirectViewProjection` captures from one `observe_source` call. A matching
  Host group selects its proof once, checks every member before I/O, opens one
  handle, and reuses `observe_trusted`; mismatch preserves the proof, while
  matching source/resource failure applies the single-View invalidation rule.
- Check and View share only the source-key comparator. There is no generic
  batch framework, public single-View loop, Anchor batch, CLI/Session/Data
  surface, cache, retry, compatibility path, dependency, or v4 change.
- Regression covers empty/single/duplicate, A/B/A, distinct and overlapping
  ranges, all six upward projections and three downward rejections,
  `RelationAbsent`, stale/foreign/spill/unadmitted/missing/symlink/UTF-8/NUL/
  late-read/resource fail-closure, Host hit/miss/mismatch/invalidation parity,
  every terminator, Unicode, raw ranges, and scratch boundaries.

## Gate 5 — Edit receipt and fresh current Anddress — complete

Gate 5 closes exact native result and error ownership before Adapter output:

- File changed publication yields a fresh File Anddress for the prospective
  after hash, exact length, and full range.
- Terminator-preserving Line changed publication yields the one exact fresh
  Line Anddress for its prospective after range.
- Paragraph changed publication yields a fresh Paragraph Anddress only when
  the replacement result is exactly one Paragraph. Zero or multiple resulting
  Paragraphs are successful publication with no fresh target; Paragraph
  Content is not restricted merely to force one result.
- Byte-identical no-op is successful current-state information and returns the
  still-current input Anddress without publication or a fabricated new state.
- Definite prepublication failure returns the existing error and no receipt.
  `PublicationUncertain` returns that error and no receipt or fresh address.
- A confirmed changed publication with no fresh Paragraph target remains
  distinguishable from no-op and failure.

The selected native surface is
`WorkspaceRuntime::apply_replace(&Edit) -> Result<EditReceipt, ApplyError>` with
`EditReceipt::Unchanged { anddress }` and
`EditReceipt::Changed { anddress: Option<Anddress> }`. It accepts only Replace;
another Edit is `InvalidInput` before source I/O. Existing
`WorkspaceRuntime::apply(&Edit) -> Result<(), ApplyError>` remains unchanged for
external Rust and raw Session. Both methods call one executor.

Construction reuses the completed prospective-after hash/length,
target-range projection, Host-proof preparation, publication result, and live
Anchor reflection plan. It may not reopen the source, run Search, guess by
content/context, or retain an old-to-new relation after return.

At Gate 5 the one-shot Adapter alone switched to `apply_replace` and temporarily
discarded the receipt while preserving exact `OK` plus LF. Gate 6 replaces only
that transitional status projection. No binding, Data kind, request DTO,
parallel executor, second observation, or post-publication Search is added.

Regression covers fresh File hash/length/full range with immediate View, Check,
and a following Replace; Line None/LF/CR/CRLF, Unicode, empty body, and empty
no-EOL exact ranges; and Paragraph zero/one/multiple results. Direct and
assembled no-op preserve the exact input, bytes, inode, Host proof, and Anchor.
Receipt and reflected Anchor use the same prospective-after identity. Existing
stale, invalid, unadmitted, open/read/resource, staging, rename uncertainty,
cleanup, proof, and Anchor failure controls remain owned by the shared
executor, while structural checks exclude a source reopen, second observation,
or Search. The complete GNU-host suite has 255 tests: the 253 Gate 4 controls
plus two public receipt regressions.

## Gate 6 — Adapter output and stdin decision — complete

- Human Edit emits exactly `Unchanged` or `Changed`, a tab, and one canonical
  v4 object or `None`, followed by LF. `Unchanged` is the original validated
  current target; `Changed(Some)` is the fresh result; `Changed(None)` is a
  confirmed zero/multiple-Paragraph result.
- Leading `--json` emits the compact `bw.cli.edit.v1` object with fixed
  `schema`, `outcome`, and `anddress` order. The single writer encodes an
  address once before stdout access and directly embeds those bytes; it has no
  JSON value tree, reserialization, clone, or result collection.
- Apply and `PublicationUncertain` errors emit zero stdout and return no
  receipt. A write/flush failure after Apply exits `1` without undoing the
  already confirmed publication or no-op; partial stdout is possible and no
  retry follows.
- Argv stays the sole Content transport. Empty/Unicode, File/Paragraph CR/LF,
  and permitted Line bodies already have direct coverage. Known OS/shell and
  process-list/history constraints do not provide a reproduced consumer
  failure, measured payload need, or concrete security requirement, so no
  stdin syntax, reader, EOF state, generic content source, file transport, or
  placeholder is added. Positional `--json`, `--raw`, and `--stdin` remain
  literal Content; leading `--raw` remains a usage error.
- Regression covers both output modes and every receipt state/target shape,
  canonical embedded-v4 equality, Search-v2 object to Edit to direct fresh
  View/Edit reuse, all Line terminators, grammar/error streams, post-Apply
  output failure, raw Session `OK`, and structural absence of post-Search,
  reopen, Check, stdin, parallel writer, or schema paths. The complete
  GNU-host suite has 256 tests.

## Gate 7 — Dummy integration and source-readiness decision — complete

The decision is **GO**. Both variants were built from clean Git-object exports:
A is published `0.2.2` Source Authority
`04b36d9ca9cc725bedeb17231339c67b5f0590ea`; B is the integrated Gate 6 parent
`d3e2b2e65112e9f0f018cd29050652928e4ef412`. The task-local fixture began as
exact `retry_budget = 3\r\n`, SHA-256
`ec43c761c4cd9b113208d4f99b2e912697e414e739e4c60324f4b5ae2c72a3fb`.
Intermediate exact bytes were `retry_budget = 5\r\n`, SHA-256
`cc326fa86d3e5924c488283058e530b9413d6acec0f4f78a954882f85f92edbf`.
Both variants finished as exact `retry_budget = 7\r\n`, SHA-256
`798ba02ce45d505e56b0112210695a52931a40797aa9eb6f68d608d9c9b6173e`.

The first measured run recorded these exact argv and results; every command
had exit `0` and empty stderr:

```text
A1 ["/tmp/backwriter-gate7.v8Oi4x/A/target/release/bw","--json","search","line","retry_budget = 3","--source","note.txt"]
   stdout {"schema":"bw.cli.search.v1","outcome":"found","anddresses":[{"version":"artext.backwriter-anddress.v4","workspaceCoordinate":"ce23715f0cf945c0ed423276a62fa7f6108ba15df5e08885d523a6ab1efa52cc","logicalPath":"note.txt","sourceStateHash":"ec43c761c4cd9b113208d4f99b2e912697e414e739e4c60324f4b5ae2c72a3fb","sourceByteLength":"18","kind":"line","byteStart":"0","byteEnd":"18"}]}\n
A2 ["/tmp/backwriter-gate7.v8Oi4x/A/target/release/bw","edit","anddress","{\"version\":\"artext.backwriter-anddress.v4\",\"workspaceCoordinate\":\"ce23715f0cf945c0ed423276a62fa7f6108ba15df5e08885d523a6ab1efa52cc\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"ec43c761c4cd9b113208d4f99b2e912697e414e739e4c60324f4b5ae2c72a3fb\",\"sourceByteLength\":\"18\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"18\"}","retry_budget = 5"]
   stdout OK\n
A3 ["/tmp/backwriter-gate7.v8Oi4x/A/target/release/bw","--json","search","line","retry_budget = 5","--source","note.txt"]
   stdout {"schema":"bw.cli.search.v1","outcome":"found","anddresses":[{"version":"artext.backwriter-anddress.v4","workspaceCoordinate":"ce23715f0cf945c0ed423276a62fa7f6108ba15df5e08885d523a6ab1efa52cc","logicalPath":"note.txt","sourceStateHash":"cc326fa86d3e5924c488283058e530b9413d6acec0f4f78a954882f85f92edbf","sourceByteLength":"18","kind":"line","byteStart":"0","byteEnd":"18"}]}\n
A4 ["/tmp/backwriter-gate7.v8Oi4x/A/target/release/bw","view","anddress","{\"version\":\"artext.backwriter-anddress.v4\",\"workspaceCoordinate\":\"ce23715f0cf945c0ed423276a62fa7f6108ba15df5e08885d523a6ab1efa52cc\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"cc326fa86d3e5924c488283058e530b9413d6acec0f4f78a954882f85f92edbf\",\"sourceByteLength\":\"18\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"18\"}"]
   stdout hex 72657472795f627564676574203d20350d0a
A5 ["/tmp/backwriter-gate7.v8Oi4x/A/target/release/bw","edit","anddress","{\"version\":\"artext.backwriter-anddress.v4\",\"workspaceCoordinate\":\"ce23715f0cf945c0ed423276a62fa7f6108ba15df5e08885d523a6ab1efa52cc\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"cc326fa86d3e5924c488283058e530b9413d6acec0f4f78a954882f85f92edbf\",\"sourceByteLength\":\"18\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"18\"}","retry_budget = 7"]
   stdout OK\n
B1 ["/tmp/backwriter-gate7.v8Oi4x/B/target/release/bw","--json","search","line","retry_budget = 3","--source","note.txt"]
   stdout {"schema":"bw.cli.search.v2","outcome":"found","occurrences":[{"logicalPath":"note.txt","kind":"line","line":"1","anddress":{"version":"artext.backwriter-anddress.v4","workspaceCoordinate":"41bbd9d2ef13150ddd485772f8cc881421aac642c12bc9ccc88eab734e449ac1","logicalPath":"note.txt","sourceStateHash":"ec43c761c4cd9b113208d4f99b2e912697e414e739e4c60324f4b5ae2c72a3fb","sourceByteLength":"18","kind":"line","byteStart":"0","byteEnd":"18"}}]}\n
B2 ["/tmp/backwriter-gate7.v8Oi4x/B/target/release/bw","--json","edit","anddress","{\"version\":\"artext.backwriter-anddress.v4\",\"workspaceCoordinate\":\"41bbd9d2ef13150ddd485772f8cc881421aac642c12bc9ccc88eab734e449ac1\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"ec43c761c4cd9b113208d4f99b2e912697e414e739e4c60324f4b5ae2c72a3fb\",\"sourceByteLength\":\"18\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"18\"}","retry_budget = 5"]
   stdout {"schema":"bw.cli.edit.v1","outcome":"changed","anddress":{"version":"artext.backwriter-anddress.v4","workspaceCoordinate":"41bbd9d2ef13150ddd485772f8cc881421aac642c12bc9ccc88eab734e449ac1","logicalPath":"note.txt","sourceStateHash":"cc326fa86d3e5924c488283058e530b9413d6acec0f4f78a954882f85f92edbf","sourceByteLength":"18","kind":"line","byteStart":"0","byteEnd":"18"}}\n
B3 ["/tmp/backwriter-gate7.v8Oi4x/B/target/release/bw","view","anddress","{\"version\":\"artext.backwriter-anddress.v4\",\"workspaceCoordinate\":\"41bbd9d2ef13150ddd485772f8cc881421aac642c12bc9ccc88eab734e449ac1\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"cc326fa86d3e5924c488283058e530b9413d6acec0f4f78a954882f85f92edbf\",\"sourceByteLength\":\"18\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"18\"}"]
   stdout hex 72657472795f627564676574203d20350d0a
B4 ["/tmp/backwriter-gate7.v8Oi4x/B/target/release/bw","--json","edit","anddress","{\"version\":\"artext.backwriter-anddress.v4\",\"workspaceCoordinate\":\"41bbd9d2ef13150ddd485772f8cc881421aac642c12bc9ccc88eab734e449ac1\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"cc326fa86d3e5924c488283058e530b9413d6acec0f4f78a954882f85f92edbf\",\"sourceByteLength\":\"18\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"18\"}","retry_budget = 7"]
   stdout {"schema":"bw.cli.edit.v1","outcome":"changed","anddress":{"version":"artext.backwriter-anddress.v4","workspaceCoordinate":"41bbd9d2ef13150ddd485772f8cc881421aac642c12bc9ccc88eab734e449ac1","logicalPath":"note.txt","sourceStateHash":"798ba02ce45d505e56b0112210695a52931a40797aa9eb6f68d608d9c9b6173e","sourceByteLength":"18","kind":"line","byteStart":"0","byteEnd":"18"}}\n
```

The exact operation counts are:

| Evidence | A | B |
| --- | ---: | ---: |
| process / Adapter commands | 5 | 4 |
| Search | 2 | 1 |
| repeated or post-Edit Search | 1 | 0 |
| JSON array-index bookkeeping | 2 | 1 |
| explicit View | 1 | 1 |
| Edit-internal View | 2 | 2 |
| Apply | 2 | 2 |
| total Runtime capability calls | 7 | 6 |
| caller-visible raw `apply` | 0 | 0 |
| mandatory Check | 0 | 0 |
| Wrong Apply | 0 | 0 |
| history / relocation / retry | 0 / 0 / 0 | 0 / 0 / 0 |
| newline mistakes | 0 | 0 |

B used only the fresh receipt object for its following View and Edit, with no
post-Edit Search. Reusing the old object remains a nonpublishing
`Unavailable`; native receipt regressions keep `Unchanged`, `Changed(Some)`,
and `Changed(None)` distinct. Existing Gate 2 native duplicate
self-identification and Gate 4 native batch tests already cover Search
occurrence to Paragraph projection/batch and one-observation grouping, so no
second harness or duplicate regression was added.

After one untimed warm-up per variant, five elapsed samples ran in crossed
order `AB`, `BA`, `AB`, `BA`, `AB`. The monotonic interval began immediately
before the first `bw` spawn and ended after the final source read; fixture reset
was excluded. A samples in ns were `25812745`, `25298433`, `25168625`,
`19043375`, `24342452` (median `25168625`, p95 nearest-rank `25812745`). B
samples were `19793078`, `20167653`, `20982414`, `15941451`, `13804112`
(median `19793078`, p95 nearest-rank `20982414`). They are diagnostic elapsed
evidence only, not a performance gate or broad speed claim.

The task-local driver was 11,999 bytes with SHA-256
`18ec103d2815f52957d29e3be986f3b9e8027d3442a14bd252f60145033e410d`.
The 68,551-byte raw JSON evidence had SHA-256
`8c48cb6192621f0d7c92a6be76d432beb2c11bd134c00be008349ca01dd8243a`.
Both, all exports, fixtures, and build outputs were removed after verification.

Before and after the version-only change, GNU and musl each passed the full
256-test semantic matrix, all-target check, and release build. GNU also passed
format and clippy with warnings denied. V4 KAT/no-v3, Search metadata/order/
duplicates, single/batch View order/all-or-none/per-source observation, all
receipt states and output failure, raw five-Edit/four-Position Apply and
Session, Host proof hit/miss/mismatch/invalidation, Anchor same-after
reflection/fail-closure, and duplicate Line/Paragraph drift remained green.
The drift regressions retain Correct `1`, Safe Reject `6`, Wrong `0` in both
Untrusted and guarded Host modes. Production `src/**`, v4 wire, toolchain, and
dependencies are byte-identical to the Gate 6 parent.

The post-version B Dummy rerun retained process/Adapter `4`, Search `1`,
post-Edit Search `0`, JSON indexing `1`, explicit View `1`, internal View `2`,
Apply `2`, total Runtime capability `6`, and every zero-count invariant above;
its final bytes and SHA-256 remained exact.

Gate 7 advances only the Cargo package, root lock entry, version KAT, README,
and active status to source-ready, unpublished `0.2.3`. `bw version` is exact
`Backwriter 0.2.3\n`. Official artifacts, installers, manifest, public root,
service, and Update remain closed `0.2.2` until Gate 8.

## Gate 8 — separately authorized artifact and publication — pending

Artifact reconstruction, manifest and installer transition, private
cross-target verification, publisher preparation, live publication, endpoint
verification, update, and release closure require separate server and host
authorization. No earlier gate implies or reserves permission to modify the
server repository, public 44-file tree, origin service, cloudflared tunnel,
DNS, actual user HOME, tag, registry, or release service.

## Phase 1 verification boundary

Phase 1 changes only active documentation and this tracker. Rust, Cargo,
tests, README, toolchain, artifacts, installers, server, services, tunnel, DNS,
and the live public root are byte- and state-identical to the parent. The
existing complete 243-test closure is therefore reused rather than rerun.
Offline locked Cargo metadata, Markdown structure and links, conflict markers,
diff hygiene, exact changed paths, tracked outputs, `.artext`, server revision,
public-tree manifest/count, and service identity are the required Phase 1
checks.
