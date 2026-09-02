# Backwriter 0.2.3 Patch Box

Status: Gates 1–2 complete. Gates 3–8 are pending and require their own scoped
authorization.

This tracker records order, evidence, and unresolved implementation choices.
Normative meaning belongs to the active
[Protocol](../architecture/backwriter-text-coordination-protocol.md),
[address model](../architecture/rebuildable-structural-addressing.md),
[principles](../principles/backwriter-core-principles.md), and
[CLI authority](../architecture/backwriter-cli-v1.md). The published `0.2.2`
source, Cargo and CLI version, artifacts, installers, exact 44-file public
tree, and service remain the closed baseline through Gate 2.

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
| `WorkspaceRuntime::view(&Anddress)` | Public Rust callers, one-shot View, Session View, and one-shot Edit's private terminator lookup | Ordinary View owns exact-state validation and one accepted current observation; it remains supported rather than becoming an internal helper |
| `WorkspaceRuntime::view_anchored(&Anchedress)` | Session anchored View and public Anchor callers | It shares current projection machinery but retains Anchor liveness, fail-closure, and Host-proof semantics |
| `ViewOutcome` | Human/raw/JSON writers, Session bindings, `DataStore`, ordinary and anchored callers | Line already carries File and optional Paragraph Anddresses; Paragraph carries File. Those relations are reuse evidence, not yet an explicit requested projection |
| Runtime View target projection | Ordinary, trusted, and anchored paths capture exact caller ranges and related Paragraph boundaries | Gate 3 must expose one explicit projection without a finder or generic graph. Gate 4 must group by source instead of looping over the public single call |

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

## Gate 3 — single View projection — pending

- Close the smallest public Core/Runtime request and result needed to select
  one allowed projection from one caller-held v4 Anddress.
- Reuse current ordinary/trusted/anchored source-state validation and target
  projection. Return exact target Content and the projected current v4
  Anddress from the same accepted observation.
- Reject downward or unrelated projections before source I/O when the relation
  is invalid. Preserve UnsupportedVersion, InvalidInput, Unavailable, Host
  proof, Anchor, and one-read meanings without Search, relocation, or a graph.
- Keep the existing single View form only if it remains an actual consumer;
  contract it into the new request rather than build a parallel executor.

## Gate 4 — ordered batch View — pending

- Accept an ordered collection and one requested projection. Preserve input
  order and duplicates exactly and publish no partial result.
- Validate source-less input and relation errors before I/O. Group admissible
  inputs by coordinate and logical path while preserving output order.
- Observe each logical source once and satisfy every same-source projection
  from that observation. Do not call the public single View once per item.
- Cover mixed source order, duplicates, same-source distinct ranges, stale and
  unavailable members, allocation failure, and all-or-nothing output.

## Gate 5 — Edit receipt and fresh current Anddress — pending

Gate 5 must close exact native result and error ownership before Adapter output:

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
  distinguishable from no-op and failure. Exact variant names, ownership, and
  whether raw public Apply also exposes the receipt are implementation-gate
  decisions that must be closed before code changes.

Construction must reuse the completed prospective-after hash/length,
target-range projection, Host-proof preparation, publication result, and live
Anchor reflection plan. It may not reopen the source, run Search, guess by
content/context, or retain an old-to-new relation after return.

## Gate 6 — Adapter output and conditional stdin — pending

- Define human and JSON Edit receipt output only after Gate 5 fixes the native
  result. Stream exact embedded v4 objects without re-encoding through a JSON
  value tree or retaining a second result collection.
- Preserve argv Content as a supported transport and the current usage,
  execution, stream, and publication-error boundaries.
- Treat stdin as optional. Implement it only if a concrete consumer need
  remains and one grammar cannot collide with literal `--stdin` argv Content.
  Before implementation, close exact EOF, UTF-8, NUL, Line CR/LF, File and
  Paragraph newline, empty input, read/resource failure, status, and
  publication behavior. Otherwise record a no-addition decision.
- Add no raw Edit transport, generic input source, retry, wrapper, dependency,
  or automatic Search/View/Check sequence.

## Gate 7 — Dummy integration and source-readiness decision — pending

- Run the same fixed Dummy fixture against the published `0.2.2` workflow and
  the integrated candidate. Record process invocations, Backwriter capability
  operations, repeated Search/View, JSON index bookkeeping, visible output
  bytes, and elapsed time as separate measurements rather than interchangeable
  counts or a broad performance claim.
- Prove JSON Search metadata to explicit projection/batch View to one-shot Edit
  receipt to continued use of a fresh address. Retain newline mistake `0`,
  explicit Apply `0` for the general Adapter flow, mandatory Check `0`, Wrong
  Apply `0`, stale fail-closure, history `0`, and relocation `0`.
- Run the complete GNU and musl semantic matrices and verify Core/Runtime/v4,
  raw Session, Host proof, Anchor, Apply, and failure behavior. Confirm no
  duplicated Adapter executor or source reread path remains.
- Make a GO/NO-GO decision before changing Cargo and `bw version`. A NO-GO
  leaves version and readiness documents unchanged. A GO may advance source to
  `0.2.3` while official artifacts/installers/publication remain `0.2.2` until
  Gate 8.

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
