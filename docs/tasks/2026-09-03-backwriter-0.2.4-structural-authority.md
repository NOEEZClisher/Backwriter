# Backwriter 0.2.4 Structural Authority

Status: Gates 1–3 complete. The source accepts and emits only v5 while Cargo,
CLI version, and the published closed `0.2.3` distribution remain unchanged.

## Objective and boundary

`0.2.4` hard-cuts the complete source to
`artext.backwriter-anddress.v5`. It centralizes exact source and structural
geometry in Anddress, replaces four or more Line/Paragraph framers with one
private `StructuralCursor`, and replaces capability-local address construction
with one Anddress Issuer. It removes duplicated Search position, View relation,
and private Edit View work without changing each capability's distinct job.

There is no v4 compatibility decoder, alias, wrapper, parallel wire, or
parallel Runtime. There is also no history, relocation, registry, watcher,
retry, merge, rollback, target lineage, or implicit capability workflow. Gate
2 fixes exact wire bytes and immutable source-identity sharing. CLI/JSON
presentation contraction, stdin, and `src/bin/bw.rs` splitting remain
decisions for their owning later gates.

## Gate 1 — authority and consumer baseline — complete

The Gate 1 baseline direct consumers establish the contraction boundary:

| Gate 1 surface | Actual consumers | 0.2.4 disposition |
| --- | --- | --- |
| v4 `SourceIdentity`, `construct_source_identity`, `construct_anddress` | Runtime Search, View, and Apply; encoded objects in CLI, Check, Data, Pick, Anchor, and tests | Replace with v5 source geometry and one Issuer; no capability constructor remains |
| `SearchOccurrence` plus optional `SearchPosition` | Runtime Search, Check filtering, Data/Session storage and indexing, human/JSON writers | Preserve order/multiplicity and caller ownership; remove the wrapper position after positions derive from Anddress |
| `FileProjection`, `LineProjection`, `ParagraphProjection`, `ParagraphState` | Runtime Search literal projection | Replace structural framing with `StructuralCursor`; retain literal matcher, tier buckets, ordering, and all-or-none collection |
| `TargetProjection` | anchored/current target observation | Replace structural framing with `StructuralCursor`; retain one-read source observation and currentness checks |
| `DirectViewProjection`, `LineRelation`, paragraph boundary scans | ordinary/trusted/anchored single and batch View | Project from validated Anddress geometry, then read the exact range; retain source grouping, one observation, order, duplicates, and all-or-none behavior |
| `AfterProjector`, `CompletedOutput`, receipt and reflection planning | unit Apply, Replace receipt, Host proof, and live Anchor reflection | Feed prospective bytes through the same cursor/Issuer; retain staging, provenance, publication, proof installation, and reflection |
| one-shot Edit private `run_view` | Line terminator lookup before `apply_replace` | Remove after v5 Line owns the terminator; raw Session and public Rust Apply remain |
| Check source grouping and `observe_source` | ordinary/Host currentness reports | Retain; Check needs source identity currentness, not target parsing |
| Anchor handles and invalidation | sole live Runtime-local continuity | Retain; consume the same prospective v5 geometry as Apply |

The current release stays Cargo/CLI `0.2.3`, v4-only, and fully published. Gate
1 changes documentation only and reuses the existing GNU/musl 256-test baseline.

## Target v5 algebra

`SourceIdentity` contains workspace coordinate, logical path, complete-source
SHA-256, exact byte length, and exact Line count. Target geometry is:

- File: full `[0, sourceByteLength)` range and source Line count.
- Paragraph: exact range, zero-based `fileLineOffset`, and `lineCount`.
- Line: exact range, None/LF/CR/CRLF terminator, complete parent geometry, and
  zero-based `lineOffsetInParent`.

A nonblank text Line is parented by its containing Paragraph. A blank or
horizontal-space/tab-only Line is parented by File. Paragraph display Lines are
`fileLineOffset + 1` through `fileLineOffset + lineCount`. A Paragraph child
Line number is `parent.fileLineOffset + lineOffsetInParent + 1`; a File child
Line number is `lineOffsetInParent + 1`.

Anddress owns `same_source`, `same_state`, `contains`, `overlaps`, `parent`,
`projection_valid`, `project`, `line_count`, `line_number`, `range`,
`line_range`, and `terminator`. These Gate 2 APIs fix semantic ownership
without defining a capability workflow. The self-contained wire flattens
target and parent geometry while issued values share immutable source identity.

## Gate 2 — v5 algebra, wire, and Issuer — complete

Acceptance:

- define one canonical v5 value and exact encoding KAT for every target;
- validate every source/target/range/count/parent/offset/terminator invariant;
- reject v4 as unsupported with no compatibility or parallel execution path;
- provide source/state, containment/overlap, parent/projection, Line geometry,
  range, terminator, and projection-validity algebra;
- make one crate-private Issuer the sole ordinary-address constructor;
- prove File/Paragraph/Line geometry sharing without requiring callers to
  rebuild or reinterpret it.

Fail closed on malformed or inconsistent flattened geometry, arithmetic or
allocation failure, unsupported version, invalid projection, or any second
constructor path. Gate 2 decides exact field names/order and in-memory sharing.

Implemented evidence:

- `SourceIdentity` shares coordinate, path, SHA-256, byte length, and Line
  count through one `Arc` for all values emitted by one Issuer.
- File derives full byte/Line ranges; Paragraph stores range, File-Line offset,
  and nonzero Line count; Line stores range, exact terminator, complete File or
  Paragraph parent geometry, and parent-relative offset.
- one validator serves the crate-private Issuer and decoder. Public raw and
  capability constructors are removed. Search, View, Check, Apply, and Anchor
  source-state checks include Line count.
- fixed-order compact KATs cover File, Paragraph, text Line, and File-child
  separator Line. The decoder rejects v4/v3, malformed/duplicate/missing/
  unknown/wrong-typed fields, noncanonical decimal, overflow, and inconsistent
  source or target geometry with the specified error classes.
- Anddress owns allocation-free source/state, range, containment/overlap,
  parent/projection, Line count/range/number, and terminator operations.
- existing Search projections and Apply prospective projection carry v5
  geometry until Gate 3 replaces their duplicate framing with the sole
  `StructuralCursor`; literal matching and publication behavior are unchanged.

## Gate 3 — StructuralCursor, Search, and result contraction — complete

Acceptance:

- one allocation-bounded `StructuralCursor` is the sole CR, LF, CRLF, no-EOL,
  blank/separator, Line, and Paragraph framer;
- Search retains literal tiers, deterministic order, duplicates, and
  all-or-none failure while emitting v5 Anddresses through the Issuer;
- remove `SearchPosition` and `SearchOccurrence` completely,
  `LineProjection.line_number`, and `ParagraphState.start_line/end_line`;
- prove sparse matching on 256 MiB and 1 GiB inputs, one million ordered hits,
  exact Unicode and terminator geometry, and no whole-source/result duplicate;
- machine/human position output, if retained, derives only from Anddress.

Any parser disagreement, missing/extra hit, order or multiplicity drift,
partial result, source reread, arithmetic failure, or v4 residue is a gate
failure. Exact Search JSON migration belongs to this gate and is not preclosed.

Implemented evidence:

- one private `StructuralCursor` owns complete-source byte offset, CR/LF/CRLF/
  no-EOL framing, body class, Line geometry, File Line offset, and Paragraph
  geometry. Search, source-state observation, direct View relation validation,
  and prospective Apply projection consume its forward events. The retained
  bounded trusted View range/relation scans remain Gate 4 work.
- Runtime Search deletes `FileProjection`, `LineProjection`,
  `ParagraphProjection`, and `ParagraphState`; Apply deletes `AfterProjector`'s
  local framer; source observation deletes `SourceTextBuilder`. Literal
  matching, tier buckets, traversal, currentness, staging, publication, Host
  proof, and Anchor reflection retain their existing consumers and meanings.
- Core hard-cuts `SearchOutcome::Found` to `anddresses: Vec<Anddress>` and
  deletes `SearchOccurrence`, `SearchPosition`, their validator, and all
  producer/consumer wrappers. Check, Data, Session indexing, Pick candidate
  extraction, human output, and JSON output consume the direct collection.
  The Adapter-only `bw.cli.search.v2` envelope and its key order and bytes are
  unchanged; positions derive only from `line_number` and `line_range`.
- fixed-scratch regressions cover all terminators and body classes across
  8,191/8,192/8,193-byte edges and fail checked offset overflow before input
  consumption. Unicode, exact geometry, provisional fail-all, order,
  multiplicity, and currentness suites remain green.
- parent/candidate JSON output is byte-identical for a 256 MiB one-hit source
  (output SHA-256 `735a389c98de40a126137d4654a72df46a40d37175a5a4c1839f3bba77b31d58`)
  and a 1 GiB one-hit source
  (`06e84d3dca2d35c8c6f2edc032a9814e0b68df7a71c8346a5b8a92fd02c59e6e`).
  Candidate peak RSS is 2,640 KiB and 2,504 KiB respectively; source size does
  not create whole-source retention.
- a 1,048,576-Line/full-hit fixture produces exactly 1,048,576 ordered results.
  Parent/candidate streaming JSON is byte-identical at SHA-256
  `823b903c89f45bb739e0e9a65a6b04df9313e4dc012f6b30c54e6ad06ee20c9e`;
  peak RSS changes from 215,660 KiB to 166,404 KiB after removing the duplicate
  occurrence/position carrier. Timing is descriptive and is not a gate.

## Gate 4 — View Runtime and single/batch Adapter

Acceptance:

- validate self/ancestor projection from Anddress, then currentness, then read
  the projected exact byte range;
- remove `DirectViewProjection`, `LineRelation`, and paragraph boundary scans;
- preserve single and batch order, duplicates, `RelationAbsent` where the v5
  algebra permits it, all-or-none behavior, and one observation per source
  group on proof miss;
- cover File, Paragraph, text Line, blank/space-tab Line, every terminator,
  overlapping inputs, A/B/A groups, and 200,000 files;
- migrate CLI output without a v4 branch or new finder.

Invalid projection, stale state, unavailable source, inconsistent geometry,
resource failure, or one member failure rejects the required scope without a
partial batch. Exact CLI/JSON representation is decided here.

## Gate 5 — Edit, Apply, Anchor, and private View removal

Acceptance:

- one-shot Edit obtains Line terminator and target geometry from v5 Anddress
  and performs no private View;
- public unit Apply and Replace receipt share the existing executor, staging,
  prospective hash, provenance, publication, proof installation, and Anchor
  reflection;
- prospective bytes use `StructuralCursor` and Issuer once for receipt and
  Anchor candidates;
- cover File/Paragraph/Line changes, None/LF/CR/CRLF preservation, no-op,
  `Changed(Some/None)`, output failure, and old/fresh address currentness;
- Anchor remains the sole Backwriter continuity exception.

No receipt follows definite failure or `PublicationUncertain`. No reopen,
post-Search, second parse, history, relocation, retry, or alternate executor is
allowed.

## Gate 6 — Check and remaining contraction

Acceptance:

- Check compares current source identity without target parsing and preserves
  ordered/multiplicity-aware reports and Host proof hit/miss/invalidation;
- Data, Pick, Session, and external Rust consumers migrate to the v5 hard cut;
- production contains one structural parser, one Issuer, no capability-owned
  constructor, no Search position wrapper, no View relation finder, and no
  private Edit View;
- measure production Rust bytes and explain every material addition; total
  structure should contract rather than merely move duplication.

Any v4 runtime branch, duplicate parser/constructor, result semantic drift, or
unexplained code growth is a gate failure.

## Gate 7 — integration, evidence, and source-readiness decision

Acceptance:

- pass complete GNU and musl suites, v5 KAT/no-v4, Search/View/Check/Apply,
  raw Session, Data/Pick, Host proof, Anchor, admission, and failure matrices;
- blind duplicate drift remains Correct 1 / Safe Reject 6 / Wrong Apply 0;
- compare sparse Search, one million hits, 200,000 files, View single/batch,
  terminator Edit, code size, peak memory, reads, and output equality against
  fixed clean baselines;
- record an AI workflow using Search geometry, one batch Line-to-Paragraph
  projection, and fresh Edit receipt with no redundant JSON indexing, repeated
  Search/View, mandatory Check, history, relocation, or retry;
- decide source-ready `0.2.4` GO/NO-GO only after semantics and evidence pass.

Timing is descriptive, not a correctness gate. A failure records evidence and
leaves Cargo/CLI at `0.2.3`; it does not lower the contract or patch features
inside the readiness gate.

## Gate 8 — artifacts and publication

Gate 8 requires separate Owner authority. It may reconstruct artifacts,
manifest, installers, update handoff, and publication only from an accepted
Gate 7 Source Authority. Phase 1 and Gates 2–7 authorize no artifact, tag,
release, service, tunnel, DNS, HOME, or live public-root mutation.

Acceptance and fail-closure must be specified from the exact release inputs at
that time. Existing closed `0.2.3` artifacts and 52-file public tree remain
immutable throughout earlier gates.

## Fixed exclusions

No gate adds history, predecessor/successor identity, relocation, registry,
watcher, automatic retry, merge, rollback, persistent index, global snapshot,
or required capability workflow. Stdin transport and splitting `bw.rs` require
independent consumer evidence and an owning gate decision; neither is implied
by structural consolidation.
