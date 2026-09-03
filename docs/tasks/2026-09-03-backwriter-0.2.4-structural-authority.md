# Backwriter 0.2.4 Structural Authority

Status: Gates 1–8 complete; `0.2.4` is published and closed. Source, Cargo,
`bw version`, four-target artifacts, installers, manifest, Update target, and
the exact 60-file public distribution are aligned at v5 `0.2.4`. Published
`0.2.3` remains immutable v4 release evidence.

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

The Gate 1 release baseline is Cargo/CLI `0.2.3`, v4-only, and fully published.
Gate 1 changes documentation only and reuses the existing GNU/musl 256-test
baseline; Gate 7 later advances source Cargo/CLI only.

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

## Gate 4 — View Runtime and single/batch Adapter — complete

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

Closure:

- `Anddress::project` resolves every self/ancestor request before I/O. A
  File-parent Line projected to Paragraph returns `RelationAbsent`; the three
  downward relations return `InvalidInput` before source access.
- ordinary and anchored direct execution validate current source state once
  while capturing only the requested ranges. A matching Host proof opens one
  handle and reads only those ranges. `DirectViewProjection`, `LineRelation`,
  Paragraph boundary scans, and View-owned address construction are removed.
- `ViewOutcome` is exactly `Projected { anddress, content } | RelationAbsent`.
  The Content is the projected exact range, including a Line terminator;
  ancestors and terminators remain available through v5 algebra.
- one-shot View accepts one self projection or explicit `--as`; JSON batch
  accepts a nonempty Anddress collection plus one `--as` projection. Both use
  the hard-cut `bw.cli.view.v2` outcomes envelope and one item writer. Human
  and raw single output remain byte-identical.
- public regressions cover all six allowed and three downward relations,
  RelationAbsent, every terminator, Unicode, range edges, order, duplicates,
  source grouping, proof hit/miss, and all-or-none failure. Search, Edit/Apply,
  Check, Data retention, and Anchor continuity meanings remain unchanged.
- a task-local native harness verifies one ordered File `view_batch` over
  exactly 200,000 admitted one-byte sources and 200,000 Search results, with
  input-equal projected Anddresses and exact `x` Content for every outcome;
  the harness and generated files are removed after the run.

## Gate 5 — Edit, Apply, Anchor, and private View removal — complete

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

Closure:

- one-shot Edit now performs v5 decode, target-specific Content preparation,
  `Edit::Replace` validation, Runtime open, and one `apply_replace` call in that
  order. A Line appends the terminator carried by its decoded v5 geometry and
  rejects NUL/CR/LF before Runtime access; File and Paragraph remain exact
  Content. There is no private View, Search, Check, or second target lookup;
- unit `apply` and Replace `apply_replace` retain one internal executor. One
  prospective `StructuralCursor` pass produces the after hash, length, Line
  count, and every receipt/Anchor candidate, and one `AnddressIssuer` emits the
  resulting current addresses before publication;
- the projector accepts borrowed same-path Anchor bindings and the optional
  receipt target directly. It removes a second relation allocation and receipt
  target clone, and uses v5 `contains`/`overlaps` instead of local range helpers;
- direct and assembled no-op preserve the original receipt address, bytes,
  inode, Host proof, and Anchor. Changed File, exact Line, and unique Paragraph
  results retain `Some`; Paragraph zero/multiple results retain `None`;
- existing definite/uncertain failure, proof/Anchor invalidation, old/fresh
  currentness, raw five-Edit/four-Position Session, Unicode, every terminator,
  scratch-boundary, large no-EOL, duplicate-drift, and writer-failure
  regressions remain green with 258 tests on GNU and musl.

## Gate 6 — Check and remaining contraction — complete

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

Closure:

- Check validates the complete v5 input collection before I/O, groups exact
  workspace-coordinate/logical-path source keys, and compares only complete
  source SHA-256, byte length, and Line count. It never parses or searches for
  target kind or geometry;
- a matching Host proof classifies its complete source group without open,
  read, or hash work, while a mismatch is I/O-free `NotCurrent`. Missing,
  invalidated, poisoned, or unusable proof falls back to the existing single
  `observe_source` call and never installs, replaces, or invalidates proof or
  Anchor state;
- report assembly restores caller order and duplicate multiplicity. Current
  and Unavailable values remain filtered, only NotCurrent values are removed,
  and empty Search/Pick results remain canonical;
- Data, Pick, raw Session, and external Rust tests already consume direct v5
  Anddress values, collections, and outcomes. Removed no-op occurrence helpers
  were the last result-carrier residue; no wrapper, DTO, shim, or second batch
  executor replaces them;
- production retains exactly one `StructuralCursor`, one ordinary-address
  `AnddressIssuer`, and the capability-specific source grouping, observation,
  report, and error boundaries with actual consumers. The Gate 1 production
  baseline of 302,614 bytes/9,155 lines remains contracted at the Gate 5 level
  of 297,269 bytes/8,954 lines; Gate 6 adds no production bytes or lines.

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

Closure: **GO**.

Clean Git exports use A
`195aaa37068122097ecc04d2644642b6afcc6765`, first-v5 V5
`f93f44b785961695402eaaffa521cd4de5071bc2`, and B
`8b20987893ea5ac454c4c0a50d0c470e26b5e650` under fixed root
`/tmp/backwriter-gate7.8VIlme`. `1020f22` is byte-identical to A across
`src/**`, Cargo inputs, and tests. All three use one locked toolchain, fixture
set, CPU 0, `CLOCK_MONOTONIC_RAW`, one warm-up, and seven crossed samples in
`AB/BA/AB/BA/AB/BA/AB` order. Binary SHA-256 is
`bd4aee49b531a525cc1375509d3d068e32538c061e84828f797f62101dc64a6e`
for A, `6c875cdcf2e1ae60c25b46e34b9840dab40480fbedf451fb35012d9e8feb14ad`
for V5, and
`68fba45ddee9d481213f5555d77ffa2b2a309e21a1ebc2c12ac45a6f29f2b105`
for B.
The fixture generator and Search runner SHA-256 values are
`61df1572529a92dc06d319efbc3cd1617e984daed61a610775bb1faa03ca8d6f`
and `6d942cdb894eefa413592c88fd2d4c2e32b25aba1428d0d2975a4b1556437df7`.
The 256 MiB sparse, 1 GiB sparse, dense, 200,000-file contract, View, and
None/LF/CR/CRLF Edit input SHA-256 values are respectively
`641f7442659ee50a6c5e183fd0a95963deaa21490ac2884215639ea704614d9e`,
`904c75499d4dc222f3df76ad0c2dcc397e0a163b56ed5c65692f65de7d67a162`,
`913515a8747b7f1bf66a0e60d4f7d62aee87266faeffbf1aa60509d478c86b8c`,
`08edfc37b84fc8a5e960bb2f9437590cc521c487b9aa098d3841d839e06ebf61`,
`70b89d947ded1c114ef109f8f45e4cd7d5e16497515cc38c6e4e5f1545a6ab78`,
`7c9bc58081262feba4a5609d4c9f0ae1353edd0d31ae48f92d6a68b1089fe090`,
`d35fe0ba542f0e6402a4b323b465c7a13484d702f698adad65945e39d0c50c6f`,
`54a2d1515d7157eca2f57a655558197db1223ee863a46c681b41c0d02e7d3234`,
and `ec43c761c4cd9b113208d4f99b2e912697e414e739e4c60324f4b5ae2c72a3fb`.
The many-file runner and A/V5/B harness SHA-256 values are
`f061fd9947b3fc739b6260dca4298d4f8b9502a2778d07bc29dcfe021eb8a97c`,
`3776bac25a81548191201674d97496eb805ed200a87e678994a4e01d3dcf8d86`,
and `3944f839c0facf3c54ef78015415959fa48f16e86a1aa6a9c6ad3d5673ee26f6`.
The small-View runner and A/V5/B harness SHA-256 values are
`4d15e2e3bb88f9dfd77fe2e4f7e089f7e1e976c1c605ae64b4c518b6719b7751`,
`ef290c86c116bafe43eba8c2baaadce65191a28a02ffe488c18345edfda3edb8`,
`821d91c905248521fde068fc7d2561d4aa6b52f4c4f4070bc8468a099cf50874`,
and `75eca9a722549de1c25a25a3108f6938b56dfab400e702413b98ee8a2b8f37e1`.
The Edit and AI-workflow runner SHA-256 values are
`dbff910d4fa4159bd2b1b56834b44bcc7549028d4e0a3c09d5ad427569c80645`
and `cddb3f9db977569073ee2b80fe72f1c97bca779f224012d715898dd2dec8256b`.

The fixed Search matrix is:

| Fixture | Results | A median/p95 ms | B median/p95 ms | A/B maxRSS KiB | A/B `rchar` | A/B output bytes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 256 MiB sparse | 1 | 277.688/278.691 | 394.048/396.072 | 10,844/10,856 | 268,441,917/268,441,917 | 453/642 |
| 1 GiB sparse | 1 | 1,083.990/1,086.879 | 1,551.261/1,553.824 | 10,956/10,968 | 1,073,748,285/1,073,748,285 | 455/645 |
| 1,048,576 full hits | 1,048,576 | 541.525/568.415 | 1,017.803/1,066.878 | 109,172/166,488 | 7,346,493/7,346,493 | 414,856,172/628,703,142 |
| 200,000 one-byte Files | 200,000 | 614.027/623.230 | 648.221/657.925 | 109,648/114,984 | 206,461/206,461 | 74,400,064/72,800,064 |

V5 and B Search output is byte-identical at SHA-256
`56c62059fb5c0de9e5189bcc72808a280f9d5d5da00be425945b2a8fc5af89d3`,
`01d9f612a5d5c2220d173bf6e9369cf2f278dd8392787543b334b01954fc5fc6`,
`b740ea98080fc731b9a11a75190474c5c3487be5fe411007d420bc58a6bb44aa`,
and `3b8edb97992c30c45720ff87f153cae878be8dece1293e3f78512452a9f610ef`
for the four rows. The prior Gate 3 SHA values remain valid for its deleted
root; because workspace coordinate is encoded, this fixed root intentionally
records new root-bound values. Search evidence SHA-256 is
`14e6f0137a02456398de9644950967952868dbb782cd583993a38b9e041db293`.

The 200,000-file View harness fixes `d000/f000.txt` through
`d199/f999.txt`, exact `x`, and batch/sequential equality at digest
`47142f33ec75709312a40aa34b4b9f9f85ff15df50d76e644b99c29bb289451b`.
A/B median/p95 is 1,976.203/1,983.647 versus 2,018.248/2,064.369 ms;
maxRSS is 147,620/158,564 KiB and `rchar` is 606,529/606,529. The small
View fixture proves late Line self, Paragraph, File, separator
`RelationAbsent`, and duplicate batch equality at semantic SHA-256
`237e926aeaca9f6bcdfa779e633eaabe1ab9a31f0b55af30103d65932a2a44d9`;
V5/B normalized output is byte-identical. None/LF/CR/CRLF Edit final bytes are
exact and V5/B receipts are byte-identical. CRLF A/B median/p95 is
1.187/2.533 versus 1.798/2.771 ms, maxRSS is 11,036/11,040 KiB, and final
SHA-256 is
`cc326fa86d3e5924c488283058e530b9413d6acec0f4f78a954882f85f92edbf`.
The raw Session control is byte-identical across A/V5/B; its script and output
SHA-256 values are
`6e4e0179da0cf65843251765c071ce2886b3e66598efbaf7f6783554bde8a6e8`
and `fa25fff17c951881bb024d75d2129bcf8681c13cc54a113cafb5e7c4f74c37e8`.
View, Edit, and AI evidence SHA-256 values are
`ccb80ec6a8d8381d5ed940ce9989b9f1d08fe5e9cfe53aa8af185f51b08d58cf`,
`2eb8f6e677c629625db52daaf444533c7f08bc01754b918b8b57e4438e2c5262`,
`1c564949ff842ca36ddc6a447c3dd3c596bf5b3d9a498e0fdc0c87128980ac7c`,
and `4a3ceb5ef477f62f42cecb6e925a159513c19fe8d9f75a87698243fe9dac35b8`.

The positive AI workflow has five processes and five Adapter/Runtime calls:
Search 1, two-input Paragraph `view_batch` 1, `apply_replace` 2, and final
View 1. It has zero redundant JSON indexing, repeated individual View,
post-Edit Search, mandatory Check, history, relocation, and retry. Exact
argv/output evidence follows; every omitted stderr value is the explicit empty
string shown below.

```text
W1 argv = ["/tmp/backwriter-gate7.8VIlme/B/target/release/bw","--json","search","line","needle","--source","note.txt"]
W1 exit = 0
W1 stdout = "{\"schema\":\"bw.cli.search.v2\",\"outcome\":\"found\",\"occurrences\":[{\"logicalPath\":\"note.txt\",\"kind\":\"line\",\"line\":\"1\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"b482b80f491af87ed66ba9416c9a2c33501370523846dd8ae88f2bce3682a39a\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"8b9271103720ed0bd862a255b34a2445ecc93d77ecffb0d28f85952cfc930748\",\"sourceByteLength\":\"29\",\"sourceLineCount\":\"2\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"14\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"0\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"29\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"note.txt\",\"kind\":\"line\",\"line\":\"2\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"b482b80f491af87ed66ba9416c9a2c33501370523846dd8ae88f2bce3682a39a\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"8b9271103720ed0bd862a255b34a2445ecc93d77ecffb0d28f85952cfc930748\",\"sourceByteLength\":\"29\",\"sourceLineCount\":\"2\",\"kind\":\"line\",\"byteStart\":\"14\",\"byteEnd\":\"29\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"29\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}}]}\n"
W1 stderr = ""
W2 argv = ["/tmp/backwriter-gate7.8VIlme/B/target/release/bw","--json","view","anddress","{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"b482b80f491af87ed66ba9416c9a2c33501370523846dd8ae88f2bce3682a39a\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"8b9271103720ed0bd862a255b34a2445ecc93d77ecffb0d28f85952cfc930748\",\"sourceByteLength\":\"29\",\"sourceLineCount\":\"2\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"14\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"0\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"29\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}","{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"b482b80f491af87ed66ba9416c9a2c33501370523846dd8ae88f2bce3682a39a\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"8b9271103720ed0bd862a255b34a2445ecc93d77ecffb0d28f85952cfc930748\",\"sourceByteLength\":\"29\",\"sourceLineCount\":\"2\",\"kind\":\"line\",\"byteStart\":\"14\",\"byteEnd\":\"29\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"29\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}","--as","paragraph"]
W2 exit = 0
W2 stdout = "{\"schema\":\"bw.cli.view.v2\",\"outcomes\":[{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"b482b80f491af87ed66ba9416c9a2c33501370523846dd8ae88f2bce3682a39a\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"8b9271103720ed0bd862a255b34a2445ecc93d77ecffb0d28f85952cfc930748\",\"sourceByteLength\":\"29\",\"sourceLineCount\":\"2\",\"kind\":\"paragraph\",\"byteStart\":\"0\",\"byteEnd\":\"29\",\"fileLineOffset\":\"0\",\"lineCount\":\"2\"},\"content\":\"first needle\\r\\nsecond needle\\r\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"b482b80f491af87ed66ba9416c9a2c33501370523846dd8ae88f2bce3682a39a\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"8b9271103720ed0bd862a255b34a2445ecc93d77ecffb0d28f85952cfc930748\",\"sourceByteLength\":\"29\",\"sourceLineCount\":\"2\",\"kind\":\"paragraph\",\"byteStart\":\"0\",\"byteEnd\":\"29\",\"fileLineOffset\":\"0\",\"lineCount\":\"2\"},\"content\":\"first needle\\r\\nsecond needle\\r\\n\"}]}\n"
W2 stderr = ""
W3 argv = ["/tmp/backwriter-gate7.8VIlme/B/target/release/bw","--json","edit","anddress","{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"b482b80f491af87ed66ba9416c9a2c33501370523846dd8ae88f2bce3682a39a\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"8b9271103720ed0bd862a255b34a2445ecc93d77ecffb0d28f85952cfc930748\",\"sourceByteLength\":\"29\",\"sourceLineCount\":\"2\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"14\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"0\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"29\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}","first changed"]
W3 exit = 0
W3 stdout = "{\"schema\":\"bw.cli.edit.v1\",\"outcome\":\"changed\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"b482b80f491af87ed66ba9416c9a2c33501370523846dd8ae88f2bce3682a39a\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"0e7106b18b8e62d9c91a5953e52fbde915a5d1a8b24613486eb77281c1c49591\",\"sourceByteLength\":\"30\",\"sourceLineCount\":\"2\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"15\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"0\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"30\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}}\n"
W3 stderr = ""
```

```text
W4 argv = ["/tmp/backwriter-gate7.8VIlme/B/target/release/bw","--json","edit","anddress","{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"b482b80f491af87ed66ba9416c9a2c33501370523846dd8ae88f2bce3682a39a\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"0e7106b18b8e62d9c91a5953e52fbde915a5d1a8b24613486eb77281c1c49591\",\"sourceByteLength\":\"30\",\"sourceLineCount\":\"2\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"15\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"0\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"30\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}","first final"]
W4 exit = 0
W4 stdout = "{\"schema\":\"bw.cli.edit.v1\",\"outcome\":\"changed\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"b482b80f491af87ed66ba9416c9a2c33501370523846dd8ae88f2bce3682a39a\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"275b708709f7fbdcbfd6f150a430003dc636ea9a09884a34a110546633e3e5f0\",\"sourceByteLength\":\"28\",\"sourceLineCount\":\"2\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"13\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"0\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"28\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}}\n"
W4 stderr = ""
W5 argv = ["/tmp/backwriter-gate7.8VIlme/B/target/release/bw","--json","view","anddress","{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"b482b80f491af87ed66ba9416c9a2c33501370523846dd8ae88f2bce3682a39a\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"275b708709f7fbdcbfd6f150a430003dc636ea9a09884a34a110546633e3e5f0\",\"sourceByteLength\":\"28\",\"sourceLineCount\":\"2\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"13\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"0\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"28\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}"]
W5 exit = 0
W5 stdout = "{\"schema\":\"bw.cli.view.v2\",\"outcomes\":[{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"b482b80f491af87ed66ba9416c9a2c33501370523846dd8ae88f2bce3682a39a\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"275b708709f7fbdcbfd6f150a430003dc636ea9a09884a34a110546633e3e5f0\",\"sourceByteLength\":\"28\",\"sourceLineCount\":\"2\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"13\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"0\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"28\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"},\"content\":\"first final\\r\\n\"}]}\n"
W5 stderr = ""
N1 argv = ["/tmp/backwriter-gate7.8VIlme/B/target/release/bw","--json","edit","anddress","{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"b482b80f491af87ed66ba9416c9a2c33501370523846dd8ae88f2bce3682a39a\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"8b9271103720ed0bd862a255b34a2445ecc93d77ecffb0d28f85952cfc930748\",\"sourceByteLength\":\"29\",\"sourceLineCount\":\"2\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"14\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"0\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"29\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}","must not publish"]
N1 exit = 1
N1 stdout = ""
N1 stderr = "error: current source is unavailable\n"
```

W1 slices both opaque address objects once without parse/re-encode. W2 projects
both through one batch and returns two equal Paragraph outcomes. W3 publishes
`first changed\r\n`, W4 consumes W3's fresh exact object and publishes
`first final\r\n`, and W5 consumes W4's fresh object. N1 reuses the old W1
object only as a negative control and publishes nothing. Initial/final SHA-256
is `8b9271103720ed0bd862a255b34a2445ecc93d77ecffb0d28f85952cfc930748`
and `275b708709f7fbdcbfd6f150a430003dc636ea9a09884a34a110546633e3e5f0`.

GNU and musl each pass the complete 258-test suite before and after the
version-only change, including v5 KAT/no-v4, all capabilities, Host proof,
Anchor, admission/failure matrices, and blind drift Correct 1 / Safe Reject 6 /
Wrong Apply 0. Both pass offline/locked all-target check, clippy with warnings
denied, and release build; metadata, dependency tree, and rustfmt pass as
well. The final GNU release binary SHA-256 is
`3a5988d74606ea5307083d5de7f469d4e318f72ac7295c80c3bb6c9687f83e3e`;
its repeated AI workflow evidence SHA-256 is
`03619d573711c8557bc1e19b7930ec4504e34e23ce76996ea75b139b28321f9c`
with the same final source SHA. Production `src/**` is byte-identical to Gate 6, preserving its measured
297,269 bytes/8,954 lines versus Gate 1's 302,614/9,155. No unexplained
whole-source read, duplicate result retention, memory, or code-size growth is
present. Gate 7 therefore advances only Cargo, the root lock entry, version
KAT, and active status to source-ready `0.2.4`. At that Gate 7 decision,
Gate 8 remained the sole artifact, installer, manifest, Update, publication,
and release authority.

## Gate 8 — artifacts and publication — complete

Owner-authorized Gate 8 reconstructs artifacts and the manifest only from Gate
7 Source Authority `0ee4dcce14da93f925c27a04d0e79051c83fd124`. The canonical
outputs are:

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| `backwriter-0.2.4-linux-x86_64.tar.gz` | 438,200 | `b7771618f47a827e7d331cddbc4f57789f0f1296fd0768740c3dc12e9cfc9577` |
| `backwriter-0.2.4-macos-aarch64.tar.gz` | 336,527 | `1ff09abaaf97b9cc513716a18fe8d7e8d444a54f3452f458d7083b050ed0cf78` |
| `backwriter-0.2.4-macos-x86_64.tar.gz` | 372,309 | `87021af3ed5cbad7c41f6647f99f0ce95ff51a6b539d80c53c17cdf95cc7ed4e` |
| `backwriter-0.2.4-windows-x86_64.zip` | 847,733 | `c1442b7c53dfec1403e42f1d4e3c5bb3923001a59569ec0115fda4abb8e1a584` |

The canonical manifest is 876 bytes with SHA-256
`64db11f3851b9d490c1135877fc975e841bbe231153073b7e5397fc008cfde6e`.
The macOS UUIDs are `E7F1A491-BD15-87CB-B043-E7521FFB2526` for arm64 and
`E3868F30-6F57-8F63-80FE-9BC78CA94C5C` for x86_64. macOS and Windows receive
static cross-build verification only; no native runtime, PowerShell, or CMD
execution is claimed.

The dedicated publisher installs the eight `0.2.4` versioned files, replaces
the POSIX and PowerShell installers, and publishes the manifest last. The first
live run produces the exact 60-file tree; the second reuses all 60 files with
bytes, inode, mode, owner, size, mtime, and ctime unchanged. The earlier 48
versioned files and `install.cmd` preserve their complete snapshot state.
Loopback and public HTTPS each pass 60 GET and 60 HEAD checks plus root and
unknown-path GET/HEAD 404 checks. Fresh installation, `0.2.3` Update, and
`0.2.4` reinstall install the exact Linux archive member and print the exact
Installed/Updated `0.2.4` rows. Search v2/v5, View v2 single/batch, Edit receipt
reuse, CRLF, stale nonpublication, Check, raw Session Apply, and duplicate-drift
Safe Reject smoke pass.

GNU and musl each pass 258 tests plus offline/locked metadata, dependency tree,
format, all-target check, clippy with warnings denied, and release build.
Installer, publisher, CMD static, and Origin regressions pass 41, 56, 12, and
13 cases. Publication changes no source, service, listener, Cloudflare unit or
YAML, DNS, tunnel, connector, credential, actual HOME, tag, GitHub Release,
crates.io state, or cache policy.

The existing closed `0.2.3` artifacts and its predecessor public-tree state
remain immutable release evidence.

## Fixed exclusions

No gate adds history, predecessor/successor identity, relocation, registry,
watcher, automatic retry, merge, rollback, persistent index, global snapshot,
or required capability workflow. Stdin transport and splitting `bw.rs` require
independent consumer evidence and an owning gate decision; neither is implied
by structural consolidation.
