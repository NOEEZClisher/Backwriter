# Backwriter Protocol

## 0.2.5 performance-recovery authority

Gates 1 through 4 are complete. Current production and the official release
remain closed `0.2.4`. The performance target preserves all public v5,
capability, Runtime, Adapter, failure, and publication meaning under one rule:
semantics stay unified while execution becomes specialized again.

`sourceLineCount` remains part of `SourceIdentity`, ordinary equality,
`CurrentObservation`, Host proof, and View, Check, and Apply currentness. A
same-hash, same-length address that claims a different Line count remains
`NotCurrent`. Raw observation derives the exact Line count in its one forward
read with a minimal accumulator; it omits Paragraph, parent, and target
geometry and does not invoke or duplicate the sole `StructuralCursor`.
Structural observation composes that same raw state with the cursor only when
a caller consumes Line or Paragraph geometry.

Safe Rust typed Anddresses remain valid by construction through strict v5
decode or the sole crate-private Issuer. Wire decode, public
`Anddress::validate()`, and existing error priorities remain strict. The Issuer
validates one shared source identity before constructing it and validates each
target geometry against that source. Typed View, Check, and Anchor therefore
do not repeat source-less validation; Edit and Runtime Apply retain their
distinct request and defensive validation boundaries.

The canonical encoding addition is
`Anddress::encode_into(&mut Vec<u8>) -> Result<(), AnddressError>`. It clears
the caller buffer first, computes complete capacity with checked arithmetic,
fallibly reserves before appending, and leaves buffer length zero on error;
capacity may remain reusable. Success writes exactly one canonical v5 object
with no trailing bytes. Existing `encode()` remains, delegates through one new
empty vector, and preserves exact KAT bytes and the existing error type. Search
and batch View reuse one operation-local scratch vector without duplicating the
writer or retaining a second result collection; single-result Edit and Check
keep their one-address `encode()` calls.

Bulk matching, raw/structural observation, and issuance/encoding are complete.
The remaining ordered implementation gates are chunked pending memory and
final consumer contraction, followed by fixed evidence/source readiness and
separately authorized release. Conditional structural demand, cursor
specialization, shared Paragraph allocation, and pending chunk size require
measured evidence.
No gate may change v5 fields or output, add another parser or authority, or
restore a retired carrier, relation scan, or private Edit View.

## Published and closed 0.2.4 structural authority

Gate 2 hard-cuts the source to `artext.backwriter-anddress.v5`. Published and
closed `0.2.3` remains immutable v4 release evidence. Current decode rejects v4
and v3 as unsupported; there is no compatibility alias, wrapper, parallel
schema, or parallel Runtime path. Gate 7 makes Cargo and `bw version`
source-ready `0.2.4`; Gate 8 publishes the matching four-target artifacts,
installers, manifest, Update target, and exact 60-file distribution and closes
the release at `0.2.4`.

One source state is `SourceIdentity`: Runtime workspace coordinate, observed
logical path, complete-source SHA-256, exact byte length, and exact Line count.
A File target has the full `[0, byteLength)` range and source Line count. A
Paragraph target has its exact range, zero-based `fileLineOffset`, and
`lineCount`. A Line target has its exact range and terminator plus a complete
parent geometry and zero-based `lineOffsetInParent`. A content Line's parent is
its enclosing Paragraph. A blank or horizontal-space/tab-only Line has no
Paragraph parent and uses File as parent. Line numbering is derived from this
geometry and is not a separate Search result fact.

Anddress owns exact source/state relationships, containment, overlap, parent
and projection, Line counts and numbers, byte ranges, terminator inspection,
and projection validity. A single crate-private Anddress Issuer consumes
completed source identity and target geometry and is the sole ordinary-address
construction authority. Decode and issue use one validator. File, Paragraph,
and Line values issued for one observation share one immutable
`SourceIdentity`; the self-contained wire flattens required target and parent
geometry. Exact field order and error classification are owned by the active
address model. Gate 3 installs one private allocation-bounded
`StructuralCursor` for complete-source CR, LF, CRLF, no-EOL, body-class, Line,
and blank-line-bounded Paragraph framing. Search, source-state observation,
and prospective Apply projection consume its events instead of owning
complete-source framers.

Capabilities retain only their distinct work:

- Search finds literal matches, preserves established tier/order/multiplicity,
  asks the Issuer for result addresses, and returns those Anddresses directly
  as `SearchOutcome::Found { anddresses }`.
- View validates the requested Anddress projection, confirms current source
  state, and reads the projected exact byte range. v5 geometry supplies every
  self/ancestor relation without a View-owned relation scanner.
- Check reports currentness and does not parse targets.
- Apply validates an Edit, prepares and publishes the mutation, and consumes
  prospective cursor/Issuer geometry for any exact resulting address.
- Anchor remains the sole live Runtime-local Backwriter continuity authority
  and consumes the same prospective geometry as Apply.

Gate 2 removes public raw and capability-local address constructors. Gate 3
removes Search's occurrence/position carrier and duplicate complete-source
structural projections. Human and machine Search positions derive only from
v5 Anddress geometry. Gate 4 removes the bounded View range/relation scanners.
Gate 5 removes one-shot Edit's private View and uses the decoded v5 Line
terminator before the sole Apply currentness/publication boundary. Unit Apply,
Replace receipts, Host proof, and Anchor reflection share one executor and one
prospective cursor/Issuer pass; Apply's provenance uses v5 containment and
overlap instead of local range helpers. Existing admission/no-follow
observation, literal matcher, batch source grouping, fixed-scratch staging,
prospective provenance, Host proof, publication, and Anchor reflection stay
because they have distinct consumers.
Gate 6 keeps Check's source-key grouping and shared source observer. It validates
the complete v5 input before I/O and compares only source SHA-256, byte length,
and Line count. A matching Host proof performs no source I/O, and a nonmatching
proof is I/O-free `NotCurrent`; miss or unusable proof falls back to one
observation per source without mutating proof or Anchor state. Data, Pick, and
Session carry the same direct v5 values and collections.
Gate 7 changes no Core, Runtime, wire, capability, or Adapter semantics. Fixed
clean A/V5/B measurements and complete GNU/musl suites pass, including exact
Search counts/order/multiplicity, View projections, every Line terminator,
fresh Edit receipts, Host proof, Anchor, and blind-drift fail-closure. It
therefore advances only source version authority. Gate 8 separately rebuilds
the pinned artifacts, publishes eight versioned files followed by the two
installers and manifest last, verifies an idempotent rerun, and changes no Core,
Runtime, capability, or Adapter meaning.
No history, relocation, registry, watcher, retry, merge, rollback, or implicit
capability order is created.

Status: normative current-only Core/Runtime contract. The closed public `0.1.0`
release remains immutable v3 evidence. Published and closed `0.2.4` uses the
hard-cutover v5 value/wire in all production callers. Published and closed
`0.2.3`, `0.2.2`, `0.2.1`, and `0.2.0` remain immutable v4 release evidence.
The call-local target-specific Search observation and direct View, Check,
Apply, and Anchor consumers are complete; Phase 7B verifies `0.2.1` source readiness.
The `0.2.2` Anddress-first general editing Adapter authority, minimum one-shot
implementation, integration, source-version decision, and separate Gate 7
publication are closed. Patch Box Gates 1–8 are published and closed `0.2.3`;
they change no v4 identity or currentness meaning. Official artifacts,
installers, update target, and publication are aligned at `0.2.4`.

## 0.2.0 current-observation authority

The following guard sentences are normative:

- **Current-only permits bounded call-local observation state.**
- **No history does not require cross-call observation retention.**
- **Search is the only capability that finds a target.**
- **An Anddress is the authority for its source state and byte range.**
- **View does not search.**
- **Check does not search.**
- **Apply does not search.**
- **A changed source invalidates an ordinary Anddress.**
- **Backwriter never relocates an old target after external change.**
- **Backwriter is not Git.**

The ordinary `artext.backwriter-anddress.v4` target contains exactly the
Runtime workspace coordinate, logical path, source-state hash, exact source
byte length, target kind, and one byte range. The range starts inclusively and
ends exclusively. A File range is its complete source range; Paragraph and Line
ranges are their exact current source extents. Target text and ordinal are not
v4 identity. Raw equality compares those v4 fields exactly. Admission remains
Runtime availability policy rather than raw equality.

The source-state hash is final currentness authority. It is SHA-256 through the
existing incremental hash implementation. `0.2.0` is a hard cutover: current
production has no v3 decoder, encoder, constructor, alias, migration layer, or
parallel public schema. A unique readable v3 version is
`UnsupportedVersion`; malformed or duplicate version input remains `Encoding`.

Search alone discovers File, Paragraph, or Line targets. During its one retained
source read it validates source text, parses current structure, matches or
selects the target, records exact byte ranges, and computes the source hash in
the same pass. A separate hash pass is forbidden. Every returned ordinary v4
Anddress carries the resulting hash, byte length, kind, and exact range.

View consumes one ordinary Anddress. It validates the current source hash and
length while capturing the caller-provided range during one direct observation;
it never relocates or structurally revalidates a target. Check compares only
source hash and exact length without searching or parsing target structure.
Apply accepts the hash and length as its source-state precondition, validates
the recorded public range, and patches that range directly from fixed-chunk
staging. It creates no private ordinal/text locator, target finder, or
relocation mapping. A mismatch fails closed before publication.

`CurrentObservation` is Runtime-private call-local producer state for one
selected source. It contains only the current source hash and exact byte length.
Search owns its separate target-required matcher, boundary, and provisional
range state. Ordinary View owns only its returned-range buffer and optional
Line relation state; Check needs no target projection. On success each
capability consumes its call-local state immediately; on text, I/O, or resource
failure it is discarded without publication. Runtime stores none of it in
`WorkspaceRuntime`, wire, or Anchor and retains none across sources or calls.
No observation may retain a
prior observation, whole source, parse tree, complete Line collection, Search
result, history, persistent index, relocation context, context-matching
evidence, or full workspace cache. It is neither an ordinary Anddress field nor
durable authority.

An ordinary Anddress has no continuity across a changed source. Re-search may
find a target in the new state and produce a new Anddress, but View, Check, and
Apply never do so implicitly. Reappearance of the same exact source state can
re-establish the same raw v4 value without proving temporal continuity. Anchor
remains the only continuity boundary: only a live Anchor may receive an
arithmetic range transform caused by a Backwriter-owned Apply. External or
opaque source change invalidates rather than relocates ordinary Anddresses and
live Anchor continuity. This adds no history, watcher, retry, CAS, persistent
index, or Git semantics.

## 0.2.1 Host-authoritative observation-reuse authority

`0.2.1` is implemented, published, and closed. It preserves
the v4 Anddress algebra, SHA-256, exact source byte length, target kind, and
`[start,end)` range without a wire or compatibility change. Search remains the
only target finder. View, Check, and Apply do not relocate, context-match, or
search for a target.

The default `WorkspaceRuntime`, every one-shot CLI invocation, and an ordinary
CLI Session use **Untrusted Mode**. They retain the closed `0.2.0` behavior:
each consuming call obtains its own admitted retained-handle observation and
computes the source hash and length during that read. No watcher, metadata, or
prior call changes that default.

Reuse is permitted only in an explicit **Host-authoritative Mode**. The host
must coordinate every source-visible writer and logical-path replacement that
can affect the Runtime. It must exclude mutation from the reuse decision until
the consuming capability call completes and must synchronously notify the
Runtime before any coordinated mutation begins. A watcher, `mtime`, size,
inode, path identity, or notification after mutation is not current
source-state proof. A host that cannot satisfy the complete guard must use
Untrusted Mode or cause a proof miss.

The sole permitted cross-call state shape is a current SHA-256 and exact-length
proof record bound to the owning Runtime, workspace authority, admission
authority, source generation, and logical path. It is Runtime-local, RAM-only,
replace-only, and non-persistent. It contains no source bytes, Search results,
target map, previous hash, predecessor or successor, history chain, relocation
context, or retained observation object. It is not Anddress identity, target
continuity, a registry, a cache of capability results, or proof of any other
logical path, workspace, admission, or generation.

Only a complete trusted hit may reuse that proof. A miss, incomplete host
guard, or binding to a different logical path, workspace, admission, or source
generation falls back to the complete `0.2.0` observation path. Successful
Search observation may replace the proof for its observed source, but Search
retains no result or target projection. In Host mode, a successful Search
installs every source fully observed by that call only after the whole call has
succeeded. Those per-path entries are independent current proofs, not a
workspace snapshot or completeness statement. A failed Search installs none of
its provisional proofs.

View and Check may skip only the source-size-proportional observation/hash work
when their complete v4 source hash and length match a trusted proof. Their
target and result semantics do not change. Check remains stateless with respect
to results and history; this narrow current proof is its only cross-call
exception. Apply may rely on a trusted matching proof only while the host guard
remains in force. Confirmed publication may replace the old proof with the
SHA-256 and length already computed during prospective-after emission. An exact
no-op preserves the matching proof. Apply still performs no target search or
relocation.

Host-coordinated mutation, Runtime-known opaque mutation, explicit invalidation,
workspace or admission authority change, unavailable source, uncertain
publication, and Runtime drop discard the affected proof before it can authorize
reuse. A stale ordinary Anddress after invalidation must remain a Safe Reject,
and Wrong Apply must remain zero. Anchor's existing logical-source invalidation and
`PublicationUncertain` fail-closure remain reusable, but the proof creates no
Anchor and is not Anchor continuity.

Phase 2 implements the explicit constructors
`WorkspaceRuntime::open` for Untrusted Mode and
`WorkspaceRuntime::open_host_authoritative` for Host-authoritative Mode. The
host calls `WorkspaceRuntime::invalidate_source` synchronously before mutation;
the existing anchored-source invalidation uses the same path-exact operation.
The new public seams are:

```rust
WorkspaceRuntime::open_host_authoritative(
    workspace_root: impl AsRef<Path>,
    admission: WorkspaceAdmission,
) -> Result<WorkspaceRuntime, RuntimeError>

WorkspaceRuntime::invalidate_source(
    &mut self,
    logical_path: &str,
) -> Result<(), AnchorError>
```

Private synchronized state is a sorted vector with at most one proof per
logical path, no fixed cardinality, no eviction, and no retained file handle.
Runtime ownership plus immutable workspace/admission authority bind the vector;
successful installation replaces a path entry, while synchronous removal is
the source-generation boundary, so no separate public token or generation
counter is needed. There is no proof getter.

Phase 3 adds proof consumption only to ordinary View. After source-less input,
coordinate, private-path, and proof matching checks, the Runtime releases the
proof lock before I/O. An exact same-path hash/length hit opens the admitted
regular source through the existing capability-relative no-follow path and
seeks to the public v4 range. File reads its complete range; Paragraph and Line
read only their target range. The retained proof replaces the complete-source
hash/read work but does not replace admission or source access. Missing proof,
Untrusted Mode, or incomplete host authority uses the unchanged complete
observation fallback. An existing same-path proof with different hash or length
returns `Unavailable` before source access.

A trusted exact text Line computes its optional related Paragraph with
fixed-size forward and reverse scratch around that range. It stops at the
nearest separator Line or source boundary, retains no complete Line collection,
and does not scan from source start merely to establish the relation. Separator
and raw-valid nonstructural Lines keep `paragraph: None`. Short range or
boundary reads, seek/open failure, and recoverable resource failure return
`Unavailable` and remove the matching proof. A UTF-8 scalar-cut caller range is
unavailable without asserting that the complete proof is stale. The trusted
path computes no source hash, searches for no target, retains no handle, and
changes no `ViewOutcome` or related-Anddress semantics.

Phase 4 adds proof consumption to Check without changing any public Check type,
status, filtering, report, ordering, or error. Check completes source-less
validation for every occurrence first, then preserves its existing
coordinate/path groups and workspace, private-path, and admission boundaries.
For a path proof, Runtime copies only its fixed-size SHA-256 bytes and exact
length while holding the proof lock and releases the lock immediately. Every
occurrence in that group compares its own hash and length against the copied
evidence; kind and range remain irrelevant. A match is `Current`, a mismatch is
`NotCurrent`, and original order, duplicates, and multiplicity remain exact.

A path proof covers the complete group. Therefore a mixed matching/mismatching
group performs zero filesystem open, source read, or SHA-256 work, and a
mismatch neither falls back nor removes, replaces, or refreshes proof. Untrusted
Mode, a proof miss, poisoned proof state, or unusable private proof evidence
uses the unchanged admission plus one complete observation per eligible source
group. Check never installs, updates, invalidates, or removes proof. No proof
lock is held during fallback I/O, hashing, filtering, or report assembly.

Phase 5 adds proof consumption and replacement to Apply without changing its
public API, errors, v4 geometry, publication, or Anchor semantics. Apply first
keeps existing source-less Edit validation, same-coordinate/path validation,
Runtime coordinate, private path, and admission priority. It then copies at
most one fixed-size path proof and releases proof state. Every Edit operand must
match that proof's SHA-256 and exact length. Any mismatch returns `Unavailable`
before source access, temporary creation, or publication and preserves the
proof and every live Anchor.

A matching hit opens the existing retained no-follow source handle once and
stages exactly the proof length while enforcing UTF-8/NUL policy. Fixed scratch
requests at most one additional byte to reject growth; short input, extra input,
or invalid text fails closed. The trusted staging path computes no before
SHA-256 and retains no source bytes. Untrusted Mode, missing proof, poisoned
state, or unusable proof evidence uses the unchanged `0.2.0` staging observer
and complete before hash/length validation.

Direct and assembled byte-identical no-op remove their temporaries, publish
nothing, and preserve a matching old proof and existing Anchor state. A proof
miss no-op installs nothing. For changed output, the one existing output
emission computes the prospective-after SHA-256 and exact length. That single
identity constructs the prepared Anchor plan and the next proof; Apply performs
no after-source read or second hash pass.

Every proof record allocation, proof-vector capacity reservation, Anchor plan,
collision decision, and other fallible preparation completes before
publication. Confirmed publication first replaces or installs the prepared
same-path proof without allocation and then applies the existing allocation-free
Anchor reflection plan. Definite prepublication failure preserves the accepted
old source identity and continuity except where source invalidity or exact
length drift proves the path state unusable. Source read/open failure removes a
matching proof without inventing Anchor mutation evidence. Publication
uncertainty discards both proof and all live same-path Anchors through the
existing path-exact fail-closure. No proof state is held during source I/O,
hashing, emission, Anchor planning, or publication.

Phase 6 closes proof invalidation and race semantics without adding another
state or execution layer. `invalidate_source` and
`invalidate_anchored_source` call the same I/O-free path-exact operation. A
successful call discards the named proof and every same-path live Anchor before
the host mutates source; invalid syntax or unavailable admission changes no
state. Hard-link aliases remain distinct logical paths and the host reports
each mutated alias separately. Proof never crosses logical path, workspace,
admission, Runtime, authority mode, or Runtime lifetime.

The complete Host guard is a precondition rather than a Runtime mechanism. The
host excludes every source-visible writer and path replacement from proof
selection through capability return, invalidates synchronously before its own
mutation, and performs the mutation only after invalidation returns. An
unsignaled mutation or write during a capability call violates the Host
contract. Runtime does not detect or support that race with a watcher,
metadata, rehash, retained handle, lock, CAS, generation token, retry, or
rollback.

After correct invalidation, stale View, Check, and Apply use their unchanged
proof-miss observation paths. Same-length or different-length replacement and
deletion are confirmed stale; invalid UTF-8 or NUL remains unavailable; none
relocates or publishes from the stale input. A present proof mismatch returns
`Unavailable` before I/O for ordinary View and Apply and `NotCurrent` without
I/O for Check, preserving proof and Anchor state. Check never mutates proof.

Matching Host anchored View shares ordinary trusted View execution instead of
performing its full observer. A proof mismatch is known continuity drift and
discards same-path proof and Anchors before source access. A proof miss and
Untrusted anchored View keep the existing complete structural observer.
Trusted View open, seek, read, short, and recoverable resource failure remove
only the matching proof; invalid source or another existing mutation-evidence
boundary retains the existing same-path Anchor fail-closure.

Apply length drift, invalid source, or stale same-path binding discards proof
and same-path Anchors. Open or read failure removes only a matching proof;
resource and definite prepublication failure preserve accepted proof and
Anchor state when no mutation evidence exists. Direct and byte-identical no-op
preserve old state, confirmed changed publication installs the one prepared
after proof and reflects the matching after Anchor plan, and
`PublicationUncertain` discards both same-path state sets. The exact seven-cell
duplicate-Line drift matrix remains one Correct Apply, six Safe Rejects, and
zero Wrong Applies in both Untrusted and correctly guarded Host modes;
duplicate Paragraph drift also safe-rejects in both.

### Current execution audit

The raw `observe_source` path performs one forward read from one retained
no-follow source handle. Its `ObservationBuilder` incrementally enforces
UTF-8/NUL policy and computes SHA-256, checked byte length, and exact Line count
with fixed scratch and no `StructuralCursor`. `observe_structural` composes that
same raw builder with the sole cursor during the same read.
The resulting `CurrentObservation` is call-local and is discarded after its
consumer. Untrusted `WorkspaceRuntime` stores only admission/workspace state
and live Anchor bindings.

- Content and exact-File Search each observe a selected source once. Content
  projection uses structural observation during that same read; exact File uses
  raw observation. There is no separate hash pass.
- Ordinary View opens and observes the source once while capturing the requested
  range. Anchor creation and Untrusted or proof-miss anchored View use one
  source observation with direct target projection; matching Host anchored View
  shares the ordinary trusted direct-range execution.
- Raw, Search-outcome, and Pick-outcome Check group eligible inputs by
  coordinate and logical path. Untrusted and proof-miss groups observe each
  eligible path once for hash, length, and Line count; a Host proof hit uses copied proof
  evidence and performs no source observation. Check retains no result and does
  not mutate proof after returning.
- Apply's single raw live-source observation simultaneously writes accepted
  before bytes to staging and computes the before hash, length, and Line count. Apply has no
  separate pre-hash source pass. Staging readback is preparation, not a second
  live-source observation. Prospective-after SHA-256, length, and Line count are
  computed while output bytes are emitted, without an after-source reread. Unit
  Apply and File-only receipt/Anchor output remain raw; one structural
  composition is enabled only for a non-File receipt or live non-File Anchor. In Host mode,
  Phase 5 instead uses a matching proof to omit the before hash and retains only
  the confirmed changed prospective-after proof after publication.
- In Untrusted Mode, Search followed by View, Check, or Apply performs two full
  live observations of the same source: one in Search and one in the consumer.
  Apply followed by any later consumer likewise reopens and rehashes.
  In Host mode, confirmed changed Apply installs its prospective-after proof;
  matching View, Check, and a later Apply can reuse that identity. A confirmed
  no-op preserves only an already matching proof and never installs on a miss.
- Each one-shot Search, View, or Check opens a fresh Runtime. A CLI Session
  retains one default Runtime across commands, but that Runtime stores no
  ordinary proof; Adapter bindings and `DataStore` values are not observation
  authority.

Host Search uses the same observer and projection. It moves only the completed
logical path, SHA-256, exact length, and exact Line count into provisional proof records, then
installs them after the complete Search result succeeds. Phase 3 ordinary View
uses a matching proof as specified above. Phase 4 Check classifies a matching
path group entirely from copied hash/length evidence and retains the full
observation path for Untrusted execution and proof misses. Explicit invalidation
removes the same-path proof and live Anchors;
confirmed source unavailability, anchored fail-closure, publication uncertainty,
and Runtime drop leave no reusable matching proof. Phase 5 Apply preserves an
accepted proof through no-op and definite preparation failure, replaces it only
after confirmed changed publication, and leaves unrelated path proofs unchanged.
Phase 6 makes a matching anchored View share ordinary trusted View execution,
closes both public invalidation seams over one path-exact proof-plus-Anchor
operation, and fixes the guarded mutation and both-mode drift boundaries above.

## Published and closed 0.2.3 Patch Box information surface

Gates 1 through 8 close meaning, order, Search observation metadata, native
single View projection, ordered batch View, the native Replace receipt, and its
one-shot Adapter projection, plus integrated Dummy and GNU/musl source
readiness, artifacts, and manifest-last publication. Cargo, `bw version`, the
published release, and Update target are `0.2.3`. Patch Box is an
AI-facing information-surface patch over the current engine, not a
Search-performance, source-scaling, or File-View-memory project. Its intended
caller flow is Search, optional View projection or ordered batch, one-shot
Replace, and reuse of a fresh current Anddress when the confirmed result has
one. This named flow does not make one capability a prerequisite of another.

Search returns one ordered collection of `SearchOccurrence` values. Each value
owns one exact v4 Anddress and a target-coherent optional `SearchPosition`:
File requires `None`, Line requires a nonzero one-based `Line { line }`, and
Paragraph requires a nonzero inclusive `Paragraph { start_line, end_line }`
with `start_line <= end_line`. The public getters borrow the Anddress and copy
the position; `into_anddress` transfers the owned Anddress. Construction rejects
every mismatched target/position shape as `SearchOccurrenceError::Invalid`.

Runtime computes these positions during the same selected-source observation
that matches and constructs the v4 Anddress. Paragraph bounds follow the
existing blank-Line-bounded structure; separator Lines remain outside
Paragraphs, but retain their own Line number. CR, LF, CRLF, bare CR, no-EOL,
empty-Line, and no-synthetic-EOF-Line framing remain unchanged. Checked Line
arithmetic fails through the existing Resource-to-Unavailable boundary.
Position metadata is neither v4 identity nor a locator, equality input,
currentness proof, selector, Edit input, retained observation, or permission
for an Adapter to reread the source. Duplicate Search occurrences and equal
Anddress values remain present in their existing order.

Check retains the complete occurrence for `Current` and `Unavailable`, removes
only confirmed `NotCurrent`, and keeps raw Anddresses in its report. Data and
Session store the occurrence carrier. Session indexing extracts the contained
Anddress, and Pick receives only a caller-owned raw-Anddress collection; no
Pick outcome or predicate meaning changes.

View is Observe/Project from caller-held exact-state evidence, not Find. The
only authorized projections are Line to Line, Paragraph, or File; Paragraph
to Paragraph or File; and File to File. A target cannot project downward, and
View performs no implicit Search, relocation, context matching, or discovery.
The implemented public seams are:

```rust
WorkspaceRuntime::view(&Anddress, AnddressTarget) -> Result<ViewOutcome, ViewError>
WorkspaceRuntime::view_batch(&[Anddress], AnddressTarget) -> Result<Vec<ViewOutcome>, ViewError>
WorkspaceRuntime::view_anchored(&mut self, &Anchedress, AnddressTarget) -> Result<ViewOutcome, ViewError>
```

The requested `AnddressTarget` reuses the existing target-kind representation;
there is no request wrapper or relation enum. Source-less v5 validation precedes
relation validation. Unsupported input version remains `UnsupportedVersion`;
another raw input violation or a downward relation is `InvalidInput`, and both
precede source I/O. The sole successful target outcome is
`ViewOutcome::Projected { anddress, content }`, containing the projected current
v5 Anddress plus its exact range Content from the same accepted observation.
Ancestors and Line terminators remain available through Anddress algebra rather
than duplicated result fields. When a
Line-to-Paragraph request names a separator Line or a raw-valid nonstructural
Line with no containing current Paragraph, it succeeds as
`ViewOutcome::RelationAbsent`; this is not `Unavailable` or `InvalidInput`.
Batch applies one requested projection to an ordered borrowed input collection,
preserves input order and duplicates, and is all-or-nothing. Empty input returns
an empty vector without source access. Every source-less v5 and relation check
runs in input order before any Runtime preflight or I/O; coordinate, spill, and
admission preflight then covers the complete collection. Inputs are grouped by
workspace coordinate and logical path without changing returned order. An
Untrusted or Host-proof-miss group opens one retained source handle and feeds
every target projection from one direct observation. A matching Host-proof
group selects the proof once, opens one source handle, and reads only each
requested exact range. Any validation,
allocation, open, read, UTF-8/NUL, source-state, range, or resource failure
discards all provisional outcomes. A proof mismatch occurs before I/O and
preserves that proof; a matching trusted source or resource failure invalidates
it under the existing single-View rule. Batch does not call public single View,
create a generic batch framework, or add an Anchor batch seam.

A successful one-shot Replace receipt describes only the just-confirmed
current state. Its native seam is:

```rust
WorkspaceRuntime::apply_replace(&mut self, &Edit) -> Result<EditReceipt, ApplyError>
EditReceipt::{Unchanged { anddress: Anddress }, Changed { anddress: Option<Anddress> }}
```

Only `Edit::Replace` is accepted; another Edit returns `InvalidInput` before
source I/O. For a changed File, `Changed` contains the fresh resulting File
Anddress. For a changed Line, it contains the fresh resulting Line Anddress
for the exact terminator-preserving replacement. For a changed Paragraph it
contains a fresh Paragraph Anddress only when the replacement result is
exactly one Paragraph; zero or multiple resulting Paragraphs are successful
`Changed { anddress: None }`, and Content is not restricted to force one.
Direct and assembled byte-identical no-op return `Unchanged` with the validated
input Anddress without publication. Prepublication failure and
`PublicationUncertain` produce no successful receipt or fresh address.

Fresh-result construction reuses Apply's already computed prospective-after
hash, length, Line count, candidate projection, publication boundary, and
Anchor reflection plan. The one-shot Adapter obtains Line terminator geometry
from the decoded v5 target and does not call View. It must not run a CLI
post-Search, reread the published
source, guess a target, or infer relocation. The receipt creates no
predecessor, successor, survivor, history, rollback, watcher, retry, registry,
or persistent identity. The one-shot Adapter writes the receipt as one exact
human `Unchanged`/`Changed` row or the Adapter-only `bw.cli.edit.v1` object.
Both forms directly embed the canonical v5 object when one exists; changed
Paragraph publication without one uses human `None` or JSON `null`. Apply
failure writes no success bytes. Address encoding completes before the first
write, while a later stream failure cannot undo the already determined no-op
or publication and authorizes no retry.

Existing argv Content remains the only supported transport. Empty and Unicode
Content, File/Paragraph CR/LF, and permitted Line bodies have direct argv
coverage. OS argument and shell constraints plus process-list/history exposure
do not constitute a reproduced consumer failure, measured payload need, or
concrete security requirement, so Gate 6 adds no stdin grammar, reader, EOF
state, generic content source, file transport, or placeholder. Literal
`--stdin` in the Content position remains Content. Patch Box adds no
Git meaning, diff, retry, compatibility layer, persistent target state, or
performance claim.

## Current structure only

Backwriter is not Git. It establishes only the File/Paragraph/Line structure of
the accepted current observation. It does not model or perform merge, branch,
ancestry, conflict resolution, history, rollback, predecessor/successor
lineage, reconciliation, or inheritance of past identity. When structure
changes, Backwriter constructs only the resulting current structure; it does
not derive how past targets became current targets. Past-state recovery belongs
to Git or another external history system.

## Persisted Workspace Source

Backwriter's current is the bytes returned by a retained no-follow read of
currently admitted Workspace Source. An editor-only working buffer is not that
source. Keystrokes, IME composition, undo state, and editor dirty-state
lifecycle are outside Core.

`Persisted` means source-visible at the admitted logical path; it does not
promise `fsync`, crash durability, an atomic save, or a quiescent snapshot. A
write or source replacement racing a capability call remains subject to the
existing one-read contract: the call may observe old, new, or intermediate
bytes, or return `Unavailable`. Runtime adds no retry or second read.

A human save, autosave, formatter, future Apply, CLI write, or other external
write matters only when its result becomes Workspace Source that a capability
call can read. Save is not a Core event. Runtime creates no watcher,
notification, queue, automatic scan, address re-evaluation, or reissuance. A
source-visible mutation changes only what an observation can construct or
validate from the bytes it reads.

Raw v4 Anddress values remain caller-owned values; Runtime neither mutates nor
reissues them. A new observation independently constructs values for its exact
source SHA-256, byte length, kind, and ranges. Any source-byte change therefore
invalidates every ordinary Anddress for the prior source state. Reappearance of
the exact same source bytes may reconstruct the same raw value without proving
continuity, survival, or history.

The implemented `0.2.0` Runtime execution seams are
`WorkspaceRuntime::search(&SearchRequest)`,
`WorkspaceRuntime::view(&Anddress, AnddressTarget)`,
`WorkspaceRuntime::view_batch(&[Anddress], AnddressTarget)`,
`WorkspaceRuntime::apply(&mut self, &Edit)`,
`WorkspaceRuntime::check(Anddress)`, `check_search(SearchOutcome)`, and
`check_pick(PickOutcome)`. Across calls, Search, View, Pick, Check, and Apply
retain no ordinary observation object, source cache, result store, index,
snapshot, lease, registry, or authenticity state. Search enumerates admitted
Workspace Source deterministically through retained capability-relative
no-follow handles. For each selected regular file one common fixed-scratch
reader validates complete UTF-8 and NUL policy, computes SHA-256 and checked
length, and feeds one minimal File, Paragraph, or Line projection. The
projection matches literal Line content and retains only target-required
boundaries and provisional ranges, then drops its call-local state before
another file opens. Its separate exact File request validates one logical path,
opens and observes only that admitted regular source under the same policy, and
returns its File Anddress without content matching, Line framing, or traversal.

## Implemented 0.1.0 bounded source-memory authority

Bounded source-memory removes unnecessary auxiliary materialization proportional
to complete Workspace Source. It is not a fixed-memory promise or a guarantee
that every finite input succeeds. Caller-owned input, query or replacement,
public results and returned targets, `DataStore`, and live Anchor bindings and
dispositions are outside this accounting. The no-fixed-limit rule still forbids
an arbitrary maximum, skip, or truncation; Resource and I/O failures remain
valid outcomes.

One read means one forward observation of a byte sequence from one retained
no-follow source handle. Fixed-size chunk consumption is compatible with that
meaning. It does not authorize source retry, reopen, seek, a second source
pass, cache, spill, or snapshot state. Apply may close its accepted-before
staging entry and reopen, reread, or seek that entry only after the one source
observation has completed.

View V1 keeps its public API. A File, Paragraph, or Line result can itself be
source-sized, so its permitted working space is a streaming buffer plus its
returned target. Search may retain its public result and call-local range
projection state; deterministic byte-order directory-name materialization
remains in scope for now. The shared chunk observer reads retained source
through its fixed input scratch array; target-specific Search, View, Check, and
Anchor projections receive those validated chunks. Apply writes the accepted
before observation to one same-parent staging entry, then reads only fixed
chunks from staging while splicing the public range into one prospective-after
temporary. Its auxiliary state is fixed scratch plus prospective-after
Line/Paragraph candidate ranges and minimal provenance markers; it retains
neither a separate complete before source nor a complete after source in RAM. Only the
closed staging entry may be reread; the source handle and prospective-after
temporary are not reread, sought, or reopened. Runtime-private temporary writing is
prepublication preparation, and only source-visible replacement is publication.
Currentness, resulting UTF-8/NUL validation, after projection, every Anchor
disposition and collision, and every fallible preparation complete before
publication; successful reflection remains allocation-free and non-failing.

The v4 Check, Search, View, Anchor, and Apply streaming slices share one private
incremental chunk observer in `runtime/source_scan.rs`, which reads one retained
no-follow handle with a fixed scratch buffer, incrementally hashes and validates
the same bytes, and retains no complete source. Search consumes those chunks
directly through target-specific projections and does not use the generic
`SourceEvent` path. Search shares one immutable source identity across results
from the same source and copies no Line text into Anddress. Ordinary View
captures only its returned range and minimal optional Line relation state;
Check consumes only the completed hash and length. Anchor creation uses direct
target projection, anchored View reuses the direct View projection, and Apply
uses no generic `SourceEvent` or framer. This direction adds no public stream
API, spill, mmap, cache, async work,
worker, directory traversal change, dependency, or Runtime/Anddress split.

## Implemented 0.2.0 v4 exact-source Anddress kernel

File, Paragraph, and Line are independent target addresses with structural
relationships. They are not a durable parent/child identity tree. Their raw
equality is defined only by the v4 address model. Admission is not raw equality.
Source change creates a different exact source state rather than mapping a past
target to a current target. `Block` is historical wording for the existing
blank-line-bounded Paragraph and introduces no type, alias, variant, or wire
value.

v4 includes complete-source SHA-256 and byte length in every target equality
and wire, while complete source bytes and provenance remain private call-local
construction context. A digest from one retained-handle read neither proves a
stable source nor requires a second read. The sole production wire is
`artext.backwriter-anddress.v4`, whose exact encoding and errors belong only to
the address model. Backwriter must not add a v3 compatibility decoder,
migration, alias, or parallel schema.

Backwriter Core constructs and provides target Anddress values from an accepted
current observation. Search delivers those values as results; it is not an
issuer. This creates no target registry, issuance lifecycle, locator
lookup/reuse state, durable identity, or global identity. The optional current
source-state proof retains no target map or result.

Raw exact-source/range equality and v4 wire representation are closed only in the address
model. Target-specific View currentness, Search projection, Pick predicates,
and Anchor live continuity are closed below. None authorize a separate registry,
issuance lifecycle, snapshot,
stable-read retry, temporal identity, or continuity algorithm.

## Capability composition

Capability composition is caller-owned. Core defines neither provenance, a
required capability call order, automatic result passing or retention, nor a
general workflow. A shared native Core type and a caller's explicit value
passing do not assert that a producing capability was called, establish that
value's provenance, or define an official workflow.

### Backwriter Expression Order

This section owns only the semantic-slot order for how people, AI, and Adapters
present a Backwriter expression. It is not Rust API, execution order, workflow,
provenance, wire, parser, formatter, AST, dispatcher, facade, alias, or token
syntax authority. The common order is:

```text
Capability → Operation → Kind → Operand(s) → Position → Payload → Qualifier
```

Absent slots are omitted, while present slots retain that relative order. The
normal forms are:

- Search content: `Kind(target) → Payload(query) → Qualifier(scope)`. Exact
  File lookup has one logical-path Operand and no query, target projection, or
  scope qualifier. This semantic distinction does not assign CLI token syntax.
- View: `Kind(input form: anddress|anchored) → Operand`.
- Pick: `Operand(candidates) → Qualifier(predicate)`. The predicate is one
  existing Pick predicate value; this section changes neither its nesting nor
  semantics and defines no external spelling or AST. Predicate-before-candidates
  is not a normal form.
- Check: `Kind(input form: anddress|search|pick) → Operand`.
- Anchor: `Operation(Create|InvalidateSource) → Operand`. `Create` is the
  existing Anchor-request expression operation; its result remains
  `Anchored(handle)` or `AlreadyLive`, and it creates no Anddress.
  `InvalidateSource` is a single operation label and introduces no generic
  Source kind.
- Data Store: `Operation → Kind → Name Operand → Value Payload`; Data Get and
  Remove: `Operation → Kind → Name Operand`; Data Rename: `Operation → Kind →
  Old Name Operand → New Name Operand`; Data List: `Operation` only.
  `CheckAnddress`, `CheckSearch`, and `CheckPick` are three distinct native
  `DataKind` namespaces; this implies no nested capability execution, hierarchy,
  path, or delimiter syntax.
- Edit: `Operation → [source Target Operand] → [Position] → [Content Payload]`.
- Apply: one caller-owned `Edit` Operand.

Within Position, the exact operator precedes its reference Anddress:
`Before|After → Paragraph|Line reference`, and `StartOf|EndOf → File reference`.
Position is destination geometry, never provenance. An otherwise-required Kind
slot is not added merely to repeat the File/Paragraph/Line target kind already
carried by an Anddress. This does not affect operation-specific Kind omission
for Data List, Anchor, or Edit. Search target kind, View carrier form, Check
input form, and Data native namespace remain Kind slots. A target-kind
constraint inside a Pick predicate is part of the Qualifier, not a top-level
Kind. Operand and Payload describe roles rather than types: the same
`SearchOutcome` is an Operand to Check and a Payload to Data Store. Search scope
retains all-admitted versus subtree/source selector meaning. The initial CLI V1
Search token syntax is Adapter authority in the CLI document and changes no Core
method, type, validation, or workflow contract.

This ordering changes none of the Rust method, type, variant, field, or
argument names or order; validation/error priority; result ordering; Anddress
wire field order; delimiter, quoting, escaping, variable, pipeline, or latest
syntax; capability call order; provenance; or automatic value passing or
storage.

This rule neither requires complete type or state isolation between capabilities
nor prohibits every integration. It leaves the Protocol's explicit contracts
unchanged: `check_search` and `check_pick`; `view_anchored` and
Apply's prepared live-Anchor disposition and reflection after
publication, including logical-path fail-closure on
`PublicationUncertain`. "Apply never automatically anchors a new target" means
only that Apply creates no new `Anchedress` or `AnchorOutcome`; it does not
remove reflection of existing live continuity.

`DataStore` keeps its separate boundaries: it calls no capability, performs no
automatic Store, has no latest slot, and performs no automatic update.

The completed one-shot Search, View, Check, and Edit JSON projections are
Adapter-only. Their compact envelopes identify `bw.cli.search.v2`,
`bw.cli.view.v2`, `bw.cli.check.v1`, and `bw.cli.edit.v1`. Search identifies each result by logical path, target
kind, applicable current Line number or Paragraph Line range, and its directly
embedded encoded v5 Anddress object. View uses one ordered `outcomes` array for
single and batch requests; each item is either `projected` with the exact v5
Anddress and Content or `relation-absent`. Check embeds its filtered v5
Anddress object directly when present. Edit embeds the one native receipt
address directly or uses `null`. They create no Core wire, value model,
Search/View/Check/Edit state, result collection, or capability workflow. The
published `0.2.2` Search v1 envelope remains immutable release evidence, not a
production compatibility branch.

One-shot raw View is likewise Adapter-only: it is an explicit exact-text output
selection that reuses the existing View projection without a Core wire, state,
or View semantic change.

## Implemented 0.2.0 Search and Pick behavior

Search is all-or-nothing. Invalid input, unsafe or unavailable source, invalid
UTF-8/NUL, or actual allocation/I/O failure discards every provisional result. A
source change after return has no Runtime-tracked lifecycle meaning.

A content request evaluates its query once against each current Line content:
equality is `FullLine`; any other contiguous match is `Substring`. Repeated
occurrences within one Line do not add results. Existing scope, traversal,
query validation, exact Line framing, KMP matching, target projection, tier
buckets, ordering, fail-all, no-limit, and one-read contracts remain unchanged.
Search adds no HashSet, global deduplication, or evidence.

An exact File request is constructed only from one valid logical path. It has
no query, requested target kind, scope selector, directory traversal, match
tier, ranking, or projection. Runtime resolves admission and opens only that
path capability-relatively without following links. A currently admitted
regular source is consumed once by the existing streaming UTF-8/NUL validator;
both empty and nonempty valid sources return `Found` with exactly one ordinary
v4 File Anddress. A missing path or directory returns `Empty`. Invalid paths
fail source-less validation, while unadmitted and unavailable observations fail
closed under the existing Search error boundary. The request creates no fake
Line or Paragraph, empty literal, separate result type, wire, registry, index,
or cache. Its `SearchOutcome` is accepted unchanged by `check_search` and its
File Anddress is an ordinary input to existing View, Check, Anchor, Edit, and
Apply contracts.

The public Core constructor is
`SearchRequest::exact_file(logical_path) -> Result<SearchRequest,
SearchInputError>`; source-less path rejection is
`SearchInputError::InvalidFile`. `SearchRequest` keeps the content/exact choice
private instead of exposing a second request enum or a stringly query mode.

For a Line target, every matching current Line is returned exactly once as its
v5 source identity and exact range. For a Paragraph target, every current
Paragraph containing one or more matching text Lines is returned exactly once
as its v5 range; a matching separator Line does not create a Paragraph. For a
File target, any matching Line, including a separator, returns the current v5
complete-source range exactly once. A parent has `FullLine` tier when any included match is
`FullLine`, otherwise `Substring`.

Results order `FullLine` before `Substring`. Within each tier they order by
logical-path UTF-8 bytes and then byte start/end. File targets have no
additional target key. There is no best-matching-Line concept. Runtime retains
only call-local tier buckets and transfers the result vector to the caller.

Pick is a separate pure Core function, not a Runtime seam. It returns a stable
subsequence of ordered caller input without validation, Workspace access,
currentness claim, or retained state. All, target kind, full-value OneOf, and
iterative AllOf/AnyOf/Not composition remain valid with their existing Resource
behavior. `PickPredicate::same_file(reference: Anddress)` is the only direct
file relation: it compares candidate and reference `WorkspaceCoordinate` plus
`LogicalPath` only. It does not compare source state, target kind, or range,
currentness, observation, continuity, or any other field. v4 has no
`SameObservation`, `SameParagraph`, `AncestorOf`, `DescendantOf`,
`PickRelation`, `related_to`, compatibility alias, or generic single-variant
relation enum. Pick does not read text or call Runtime to replace them.

## Implemented View behavior

View V1 has one single-input seam and one ordered borrowed-collection seam,
each with one requested existing target kind:
`WorkspaceRuntime::view(&Anddress, AnddressTarget)` and
`WorkspaceRuntime::view_batch(&[Anddress], AnddressTarget)`. Its admitted
capability-relative no-follow access and File/Paragraph/Line text projection
shape remain reusable. Its former v2 evidence-based construction and v4 result
variants are rejected; successful results use v5 source identity and geometry.
View stays
current-only, result/history-stateless, non-mutating, and without arbitrary
range, descendant, or partial behavior. Batch preserves order and duplicates,
groups by exact source key, and makes one accepted direct observation per
Untrusted or Host-proof-miss logical source rather than invoking single View
for every item. Matching Host groups reuse one handle and read only requested
exact ranges.

View first performs source-less v5 validation and then validates the allowed
projection relation before any I/O. It preserves the existing
`UnsupportedVersion`, `InvalidInput`, and `Unavailable` errors. Unsupported
version, another invalid source-less v5 input, or a downward projection returns
the corresponding first two errors. After that validation, every
coordinate, admission, open, read, UTF-8/NUL, source-state, range-text,
or resource failure returns `Unavailable`. Batch performs all source-less and
relation validation before Runtime preflight or I/O and publishes no partial
vector. View adds no public evidence,
registry, cache, retry, second read, error, or type.

`ViewOutcome` is exactly
`Projected { anddress: Anddress, content: String } | RelationAbsent`.
`Projected` uses the requested v5 target and its complete exact byte range,
including a Line terminator. `RelationAbsent` is valid only for
Line-to-Paragraph without a containing Paragraph and carries no fabricated
address or Content.

A File is current exactly when the input coordinate/path resolves to an
admitted regular UTF-8, NUL-free source whose complete SHA-256, byte length, and
Line count match and whose range is `[0,length)`. Admission is not raw equality: another
Runtime with the same workspace coordinate may use the same value whenever it
currently admits that path and observes the exact same source state.

A Paragraph or Line is current when the complete source hash, byte length, and
Line count match. The projected range is then copied directly from the same
observation; kind and structural range membership are not separate currentness
evidence. If the range cuts a UTF-8 scalar and cannot form its public text,
View returns `Unavailable`.

No View-owned constructor or relation scan creates related addresses. The
projected v5 address already carries the source and parent geometry needed for
`parent` and `project`. Re-establishing an identical tuple makes only a current
lookup succeed. It makes no continuity, authenticity, survivor, or
historical-identity claim.

## Implemented 0.1.0 Check V1 Runtime authority

`C` is Check. V1 closes Check's semantic, public API, type, and report
authority, and its stateless Rust Runtime implementation is complete. Its
public surface is exactly:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckOutcome<T> {
    pub filtered: T,
    pub report: CheckReport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckReport { /* private fields */ }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckError {
    UnsupportedVersion,
    InvalidInput,
    Resource,
}

WorkspaceRuntime::check(
    &self,
    input: Anddress,
) -> Result<CheckOutcome<Option<Anddress>>, CheckError>;

WorkspaceRuntime::check_search(
    &self,
    input: SearchOutcome,
) -> Result<CheckOutcome<SearchOutcome>, CheckError>;

WorkspaceRuntime::check_pick(
    &self,
    input: PickOutcome,
) -> Result<CheckOutcome<PickOutcome>, CheckError>;

impl CheckReport {
    pub fn current_count(&self) -> usize { /* ... */ }
    pub fn removed_count(&self) -> usize { /* ... */ }
    pub fn unavailable_count(&self) -> usize { /* ... */ }
    pub fn checked_count(&self) -> usize { /* ... */ }
    pub fn removed(&self) -> &[Anddress] { /* ... */ }
    pub fn unavailable(&self) -> &[Anddress] { /* ... */ }
}
```

`CheckError` implements `std::error::Error`.

`CheckReport` stores only `current_count`, `removed`, and `unavailable` behind
private fields. Read-only getters expose the current, removed, and unavailable
counts; `checked_count`; and removed and unavailable slices.
`checked_count` is computed as the sum of those three counts. Every report list
preserves occurrence order and multiplicity.

`D` is Data. Its V1 semantic authority is defined below. Check has no Data
state or retention and Adapter syntax is not Data authority.

Any Adapter-facing spelling for a native Search, Pick, View, or Check value,
including an unnamed-result form, is external syntax only; it creates neither
automatic result retention nor an unnamed latest slot.
Check owns no store and is stateless. The only V1 Core retention is a separate,
caller-owned `DataStore` after an explicit ownership transfer. Check takes one
raw Anddress or a native `SearchOutcome` or `PickOutcome`, and returns a
filtered native result together with a report. Check owns its input for
filtering. Filtered values are not a `Current` authenticity claim: `Current`
and `Unavailable` occurrences remain, while only confirmed `NotCurrent`
occurrences are removed. A raw input yields `Some` for `Current` or
`Unavailable`, and `None` for `NotCurrent`. Search and Pick canonicalize every
zero-survivor result, including caller-constructed empty `Found` or `Selected`,
to their existing `Empty` variants. Check does not reinterpret native form:
all order and multiplicity remain occurrence-based.

Check preserves `Current` and `Unavailable` occurrences, excluding only
`NotCurrent`. Report counts, removed occurrences, and unavailable occurrences
are occurrence-based and preserve their original order and multiplicity.
`NotCurrent` means a coordinate mismatch; a confirmed unadmitted, missing,
nonregular, or symlink source; or a complete-source hash/length mismatch. Kind
and range are not Check currentness evidence. Transient I/O or resource failure,
or UTF-8/NUL
classification failure, is `Unavailable` and is never automatically excluded.
Every occurrence completes source-less validation in input order before I/O.
Any failure returns `UnsupportedVersion` or `InvalidInput` without a partial
result. Only allocation failure for output, report, or working memory returns
`Resource`. A source-observation failure is occurrence `Unavailable`, not a
call error. `NotCurrent` requires sufficient proof that its locator does not
hold.

For each call, only a group at the current Runtime coordinate and an admitted
logical path that needs source observation uses at most one retained read. For
normal observed bytes, a group uses one private incremental forward observation
to compute hash and exact length only. A coordinate mismatch or confirmed
unadmitted, missing, nonregular, or symlink source requires neither read nor
parse. Check performs no target-kind branch or structural parse and has no
snapshot, retry, or second read. It does not re-evaluate
Search or Pick, refresh View, retarget, create an Anddress, mutate Anchor or
Workspace state, or otherwise mutate Core state. It introduces no `CheckInput`,
request, builder, disposition enum, public trait, state, or store. Anchedress
values, in-place updates of stored Check outputs, and their associated RAM commit
semantics remain deferred. Any Adapter-facing Check spelling remains Adapter
syntax rather than storage syntax or Data authority. Wire remains outside the
Core cutline; Adapter spelling does not alter Check Core authority.

In Host-authoritative Mode, a matching path proof replaces the source
observation for that complete group. Each occurrence is `Current` exactly when
its hash and length match the proof and otherwise `NotCurrent`; kind and range
remain irrelevant. A present proof never falls back or mutates proof. Untrusted,
missing, poisoned, or unusable proof state retains the complete observation
classification above. This creates no Check result retention, target lookup,
new status, public proof API, or metadata authority.

## Implemented 0.1.0 Data V1 public authority

`D` is Data. Data V1 semantic/public API/type/error authority and Rust
implementation are complete. Its public module is `backwriter::data`, and its
public surface is exactly:

```rust
pub struct DataName(String);

impl DataName {
    pub fn new(value: String) -> Result<DataName, DataNameError>;
    pub fn as_str(&self) -> &str;
}

pub enum DataNameError {
    Empty,
}

pub enum DataKind {
    Anddress,
    Search,
    Pick,
    View,
    CheckAnddress,
    CheckSearch,
    CheckPick,
}

pub struct DataStore { /* private fields */ }

pub enum StoreError<T> {
    AlreadyExists { value: T },
    Resource { value: T },
}

pub enum DataError {
    NotFound,
    AlreadyExists,
    Resource,
}

impl DataStore {
    pub fn new() -> DataStore;

    pub fn store_anddress(&mut self, name: &DataName, value: Anddress) -> Result<(), StoreError<Anddress>>;
    pub fn get_anddress(&self, name: &DataName) -> Option<&Anddress>;
    pub fn store_search(&mut self, name: &DataName, value: SearchOutcome) -> Result<(), StoreError<SearchOutcome>>;
    pub fn get_search(&self, name: &DataName) -> Option<&SearchOutcome>;
    pub fn store_pick(&mut self, name: &DataName, value: PickOutcome) -> Result<(), StoreError<PickOutcome>>;
    pub fn get_pick(&self, name: &DataName) -> Option<&PickOutcome>;
    pub fn store_view(&mut self, name: &DataName, value: ViewOutcome) -> Result<(), StoreError<ViewOutcome>>;
    pub fn get_view(&self, name: &DataName) -> Option<&ViewOutcome>;
    pub fn store_check_anddress(&mut self, name: &DataName, value: CheckOutcome<Option<Anddress>>) -> Result<(), StoreError<CheckOutcome<Option<Anddress>>>>;
    pub fn get_check_anddress(&self, name: &DataName) -> Option<&CheckOutcome<Option<Anddress>>>;
    pub fn store_check_search(&mut self, name: &DataName, value: CheckOutcome<SearchOutcome>) -> Result<(), StoreError<CheckOutcome<SearchOutcome>>>;
    pub fn get_check_search(&self, name: &DataName) -> Option<&CheckOutcome<SearchOutcome>>;
    pub fn store_check_pick(&mut self, name: &DataName, value: CheckOutcome<PickOutcome>) -> Result<(), StoreError<CheckOutcome<PickOutcome>>>;
    pub fn get_check_pick(&self, name: &DataName) -> Option<&CheckOutcome<PickOutcome>>;

    pub fn list(&self) -> impl Iterator<Item = (DataKind, &DataName)> + '_;
    pub fn rename(&mut self, kind: DataKind, old: &DataName, new: &DataName) -> Result<(), DataError>;
    pub fn remove(&mut self, kind: DataKind, name: &DataName) -> Result<(), DataError>;
}
```

`DataName::new` rejects only an empty value with `DataNameError::Empty`.
It adds no NUL, whitespace, or length restriction, normalization, or case
folding. `DataName` has `Debug`, `Eq`, and `PartialEq`; `DataKind` has `Clone`,
`Copy`, `Debug`, `Eq`, and `PartialEq`; and `DataNameError` and `DataError` have
`Clone`, `Copy`, `Debug`, `Eq`, `PartialEq`, and `std::error::Error`.
`StoreError<T>` has `Debug`, `Eq`, `PartialEq`, and `std::error::Error` for each
supported payload. `DataStore` has private fields and adds no `Clone`,
`Default`, serde, or `IntoIterator` guarantee.

A `DataStore` is separate from `WorkspaceRuntime`: it is caller-owned, explicit
RAM state, not a Runtime field, global store, automatic result retention, or
latest slot. Only an explicit Store transfers ownership into it; retained
payloads remain until an exact Remove or `DataStore` drop. The complete native
payload set is exactly `Anddress`, `SearchOutcome`, `PickOutcome`, `ViewOutcome`,
`CheckOutcome<Option<Anddress>>`, `CheckOutcome<SearchOutcome>`, and
`CheckOutcome<PickOutcome>`. An `Anchedress`, an `AnchorOutcome`, and every
other native value are excluded.

Each entry key is `(exact native kind namespace, name)`. Core owns that exact
key. A name is a nonempty UTF-8 value with exact, case-sensitive equality; it
has no normalization or case folding. The same name may exist for different
native kinds, but is unique within one kind; Adapter syntax, labels, and
presentation do not change it. Store retains the supplied native value
untouched: it performs no transformation, flattening, serialization,
normalization, or currentness check, and cannot overwrite, upsert, or replace
an existing exact key. A typed Get provides a read-only borrow only; it neither
consumes, clones, removes, refreshes, nor Checks the payload. Missing Get
returns `None`. List allocates nothing and yields only a kind and borrowed name;
its order is not Core authority.

Store first tests exact-key existence and then returns `AlreadyExists { value }`.
Only a later preparation allocation failure returns `Resource { value }`. Both
errors return the exact owned input value and preserve the complete existing
`DataStore`. Rename resolves an old missing entry as `NotFound`, then an existing
destination as `AlreadyExists`, then preparation allocation failure as
`Resource`; when `old == new` and the source exists, it returns `AlreadyExists`.
Rename changes only an entry's name within its existing kind and preserves its
payload; it creates no alias and cannot overwrite a destination. Remove missing
returns `NotFound`; successful Remove deletes the exact typed binding, drops its
payload, and returns `Ok(())`, never a taken, popped, or extracted payload.
Every mutation failure preserves the complete existing `DataStore`.

Stored values can become stale. They make no currentness, authenticity,
Workspace snapshot, history, continuity, or identity-registry claim; a stored
View output is past output, not Workspace Source. `DataStore` reads no
Workspace and does not call or mutate Search, View, Pick, Check, Apply, or
Anchor. Check remains a stateless ownership-filter API.

There is no public payload enum, `DataValue` trait, generic Store/Get,
request/builder, dispatcher, take, pop, upsert, replace, reserve, or capacity
API. Search-over-Data; Append; persistence, durability, serialization, or wire;
automatic latest or Store; stored Check mutation and its RAM commit; and Adapter
API remain deferred.

## Implemented 0.2.2 Anddress-first editing authority

The canonical general editing Adapter operation accepts exactly one encoded v4
Anddress and one new Content value. A Search or Pick result may supply that
encoded value, but the operation requires no caller-visible View, Check,
binding, index, or Core `Edit` construction. This is an Adapter contraction,
not a new Core capability, required capability workflow, or Anddress wire.

The implemented composition is fixed: decode the v4 Anddress; call
Runtime View with that exact value; normalize only the replacement Content
required by the target kind; construct the existing `Edit::Replace` with the
original Anddress; then call Runtime Apply on the same Runtime. It must not add
an editing engine, state machine, Runtime seam, retained observation, automatic
Check, or compatibility layer.

For File and Paragraph, replacement Content is the exact caller value, subject
only to the existing Edit NUL rejection. For Line, caller Content denotes only
the Line body and must contain no NUL, CR, or LF. The Adapter appends exactly
the current `None`, `Lf`, `Cr`, or `Crlf` terminator returned by View before it
constructs `Edit::Replace`. It performs no other trimming, normalization,
separator insertion, target-kind conversion, or text reconstruction. The raw
Core `Edit::Replace` contract below remains exact-Content authority and does
not gain terminator preservation.

The original Anddress remains the Apply target. If source state changes between
the private View and Apply, Apply's existing exact source-state precondition
rejects the operation. Check is not a prerequisite. The Adapter neither
relocates nor context-matches a stale range and adds no retry, merge, history,
fallback, or automatic re-search. It preserves the existing Apply errors,
including broad `Unavailable`; it does not introduce or substitute a
`NotCurrent` Apply error.

Byte-identical replacement no-op behavior, source-visible publication,
`PublicationUncertain`, live Anchor reflection and fail-closure, and optional
Host-authoritative proof/invalidation semantics remain exactly the existing
Apply contract. V4 hash, source length, target kind, range, encoding, equality,
and every Core/Runtime meaning are unchanged. The existing public raw
`Edit`/`Position`/Apply surface and Session Edit binding/index forms have direct
consumers and remain an advanced/raw surface; they are neither removed nor
aliased by this authority.

Gate 5 fixes the existing three-part separation without adding a type or
execution layer:

- The public Rust exact primitive is `Edit::{Insert, Replace, Delete, Move,
  Copy}`, `Position::{Before, After, StartOf, EndOf}`, `EditError`,
  `ApplyError`, `Edit::validate`, and `WorkspaceRuntime::apply`. Direct Rust
  callers own exact Content and every operation/position; Runtime owns geometry
  and publication, including reflection of existing live Anchor continuity.
- The advanced raw Session constructs those same values through explicit
  bindings and indexes, preserves exact replacement bytes and all operations
  and positions, permits explicit Edit clone/reuse, and invokes Apply
  separately. Edit bindings are unindexed and are neither stored nor persisted
  by `DataStore`.
- The canonical general Adapter accepts only Anddress plus Content Replace. It
  reuses `Edit::Replace` and Apply while keeping Line terminator preservation
  solely in the Adapter; it does not change raw Replace semantics.

Repository-local reference search cannot establish that no external Rust
caller consumes the public surface, so public Rust and raw Session remain
public, supported, and non-deprecated. The separation creates no raw prefix,
rename, alias, facade, re-export, feature gate, parallel enum or executor,
compatibility shim, one-shot Insert/Delete/Move/Copy, raw Edit transport, or
Edit `DataKind`.

Gate 6 closed integration and source readiness without changing this authority.
One exact CRLF fixture passes the JSON Search object's original encoded v4
bytes directly to one-shot Edit; that command privately performs View and Apply
and preserves the terminator. The complete v4 KAT, Search/View/Check/Apply,
Correct `1`/Safe Reject `6`/Wrong Apply `0`, raw Session, target-specific
one-shot Edit, and exit/output matrices remain unchanged. Source Cargo and
`bw version` became `0.2.2`; at that gate the official artifacts, installers,
manifest, and public root remained the closed `0.2.1` release. Source-built
`bw update` performed no version comparison and could install that official
`0.2.1` until Gate 7. The separately authorized Gate 7 subsequently published
and closed the official `0.2.2` distribution without changing this authority.

## Implemented 0.1.0 Edit V1 public authority

Edit V1 semantic/public API/type/error authority and its inert Rust value
implementation are complete. One Edit value expresses exactly one caller-owned
primitive and neither reads nor writes a Workspace or Runtime. It makes no
currentness, provenance, publication, resulting-structure, Anchor-change, or
Apply-execution claim.

Its public Rust surface is exactly:

```rust
pub enum Position {
    Before(Anddress),
    After(Anddress),
    StartOf(Anddress),
    EndOf(Anddress),
}

pub enum Edit {
    Insert { position: Position, content: String },
    Replace { target: Anddress, content: String },
    Delete { target: Anddress },
    Move { target: Anddress, position: Position },
    Copy { target: Anddress, position: Position },
}

pub enum EditError {
    UnsupportedVersion,
    InvalidInput,
    Resource,
}

impl Edit {
    pub fn validate(&self) -> Result<(), EditError>;
}
```

`Position` and `Edit` implement `Clone`, `Debug`, `Eq`, and `PartialEq`.
`EditError` derives `thiserror::Error` and implements `Clone`, `Copy`, `Debug`,
`Eq`, `PartialEq`, and `std::error::Error`. There is no `Default`,
serialization, wire, hashing, constructor, getter, request, outcome, builder,
trait, dispatcher, or Edit-owned executor.

`Before` and `After` accept only Paragraph or Line. `StartOf` and `EndOf`
accept only File. `Replace` accepts File, Paragraph, or Line; `Delete`, `Move`,
and `Copy` accept only Paragraph or Line.

There is no `Between` position and no File `Delete`, `Move`, or `Copy`.
`Content` is an exact UTF-8 literal with no NUL; empty Content is valid. Empty
`Insert` and empty `Replace` remain their stated primitives and are not
normalized to `Delete`. Edit does not preserve, insert, or normalize a
terminator, Paragraph separator, format, or target kind.

Every target or position Anddress first uses source-less `Anddress::validate`.
`UnsupportedVersion` maps to `EditError::UnsupportedVersion`; `Invalid` or
`Encoding`, a disallowed target kind, or NUL content maps to
`EditError::InvalidInput`; and `Resource` maps to `EditError::Resource`.
Validation is strictly field ordered: Insert validates position then content;
Replace target then content; Move and Copy target then position; Delete its
target. No validation reads Workspace Source or Runtime state.

Every target or position Anddress is exact source-state/range authority, but it
carries no source bytes. `Move` and `Copy` carry no source bytes. There is no cross-source, same-file,
adjacency, overlap, self-reference, destination-currentness, or structural
validation. Resolution, splice geometry, and every execution decision belong to
the Edit-to-Apply executor contract below. V1 does not define the ordering, batch behavior,
transaction, or atomicity of multiple Edits.

## Implemented 0.2.0 Edit-to-Apply V1 executor authority

The single-source Apply authority, Rust implementation, and regressions
are complete. Its public Runtime seam is exactly:

```rust
WorkspaceRuntime::apply(&mut self, &Edit) -> Result<(), ApplyError>
ApplyError::{UnsupportedVersion, InvalidInput, Unavailable, PublicationUncertain}
```

Patch Box Gate 5 adds the Replace-only `apply_replace` companion and
`EditReceipt` defined above without changing this exact unit-returning seam or
its errors. Both call the same executor. There is no request, additional error,
trait, batch type, or anchored Apply seam. One call handles exactly one Edit in exactly one logical
source; ordering, batch behavior, transactions, and multi-source atomicity
remain deferred.

Execution validates in this order: `Edit::validate`; equality of the
`workspace_coordinate` and `logical_path` in every target and position
Anddress; Runtime coordinate and admission; one retained, complete UTF-8,
NUL-free source observation copied to staging; exact source-state equality for
every Edit operand and same-path live Anchor; direct range geometry;
prospective-after construction and direct structural/provenance projection; Anchor
disposition, and every other fallible preparation; then one source-visible
publication. Cross-source operands, including distinct logical paths to one
hard-linked object, return `ApplyError::InvalidInput` at this seam without
changing `Edit::validate`.

Backwriter does not require a single-writer Workspace. Concurrency coordination
is caller-owned. Apply executes from the accepted current observation above; it
does not detect, serialize, merge, retry, reconcile, or CAS-protect concurrent
external writes. Writers may race, and one publication may overwrite another
writer's source-visible change. This is an accepted contract boundary, not an
Apply correctness failure. A host needing a stronger multi-writer guarantee
must coordinate outside Backwriter or adapt this open-source implementation to
its environment. This paragraph governs default Untrusted execution. An
explicit Host-authoritative Runtime instead accepts proof reuse only under the
complete mutation exclusion and pre-mutation invalidation guard above; an
uncoordinated race violates that Host contract rather than defining supported
proof behavior.

`EditError::UnsupportedVersion` and `EditError::InvalidInput` map to their
same-named `ApplyError`; `EditError::Resource` maps to
`ApplyError::Unavailable`. Coordinate, admission, currentness, source-read,
resource, and definite prepublication failures return `Unavailable`. An unknown
publication result returns `PublicationUncertain`.

Positions resolve against the accepted before observation. `Before` is the
target extent start and `After` its end. `StartOf(File)` is byte zero and
`EndOf(File)` is EOF. A Line extent includes its exact terminator. A Paragraph
extent begins at its first Text Line and ends after its last Text Line's complete
extent, excluding surrounding separator Lines.

Insert places exact Content at the resolved boundary. Replace replaces the
target's complete extent with Content, and Delete replaces that extent with zero
bytes. Copy inserts the mutation-before target bytes at the mutation-before
destination boundary. Move removes those target bytes and inserts them at the
mutation-before destination, adjusting a later destination only by the removed
earlier length. A Move destination in the target extent's strict interior is
`InvalidInput`; its start and end boundaries are valid no-ops. Copy permits an
interior or boundary destination, including the same target. No operation adds,
preserves, or normalizes terminators, separators, format, or resulting target
kind.

After the required source-validity and currentness checks, Empty Insert,
zero-range Delete/Copy/Move, empty zero-range Replace, and Move at its start or
end boundary remove staging and return `Ok(())` without comparison,
publication, or Anchor change. A nonempty zero-range Replace is insertion.
Every other potential no-op
compares its prospective after bytes to the accepted before bytes and returns
`Ok(())` when they are equal. To prepare reverse Move or Copy within bounded
RAM and one source read, only `apply` may write accepted before bytes to a
same-parent, call-local staging entry. After closing it, Apply may reopen,
reread, or seek staging while assembling a prospective-after temporary and
preparing its direct projection and Anchor plan. It never rereads, seeks, or reopens
the retained source handle or prospective-after temporary, and no temporary
other than staging has readback authority. It closes and removes staging before
publication. The prospective-after temporary remains armed until rename
succeeds; if rename fails, its ordinary Drop cleanup attempts removal before
`PublicationUncertain` returns. This does not promise cleanup after a crash or
forced process termination. This adds no collision retry, retained cache,
durable snapshot, rollback, CAS, or generic spill authority. Exact temporary
names and primitives are Runtime-private.

On Unix, a changed publication retains only the accepted source handle's basic
`mode & 0o777` value, applies it to the still-open prospective-after temporary
after its writes complete and before rename, then follows the ordinary
close/rename/reflection path. A metadata or mode-application failure is a
definite prepublication `Unavailable` failure and leaves the armed temporary
for ordinary cleanup. This preserves neither special mode bits, ownership,
ACLs, xattrs, timestamps, hard-link relationships, nor external-writer
atomicity.

Every same-path live Anchor must match the accepted source state before
publication. A File binding is preserved. Only an Edit source target supplies
mutation-before provenance; Position supplies splice geometry only. Apply
classifies source-target containment and overlap directly from public v4
ranges before prospective-after emission; it performs no target-extraction or
relation scan. A fully contained live Paragraph or Line binding follows moved
source provenance only when direct after projection yields exactly one
same-kind target. A Position-neighbor binding can rebind only from its own
original bytes in the after target. Insert and replacement bytes are mutation
evidence; Move gives source provenance only to a source-member candidate and
treats every other candidate as mutation; Copy leaves the source-member
occurrence neutral and treats every other candidate as mutation. Copy keeps its
original occurrence and never automatically anchors the copied occurrence.
Containing or crossing bindings use the exact after-projection candidate rule. Zero
or multiple candidates, split/join, absorption, ambiguity, and collisions remove
the binding. A known-invalid source or stale same-path binding fail-closes every
live binding for that path. Read, resource, or definite prepublication failures
preserve continuity. `PublicationUncertain` fail-closes every same-path live
binding. Successful reflection is allocation-free and non-failing.

The Rust executor stages the accepted before source once, computes geometry
from public v4 ranges, completes all direct no-ops after staging removal, and
assembles every other result in fixed chunks from staging. Generated bytes are
validated and hashed incrementally while the direct after projector prepares
Anchor candidates. Publication occurs only after every fallible preparation
has completed. Cross- or multi-source execution, Data storage, wire, and
general raw Edit transport remain deferred. The closed `0.2.2` one-shot
Anddress-first Adapter composition above is the sole implemented general-edit
contraction and does not change this executor.

## Implemented 0.1.0 Anchor live-continuity authority

Anchor has this implemented public Runtime surface exactly:

```rust
WorkspaceRuntime::anchor(&mut self, &Anddress) -> Result<AnchorOutcome, AnchorError>
AnchorOutcome::{Anchored(Anchedress), AlreadyLive}
AnchorError::{UnsupportedVersion, InvalidInput, Unavailable}
WorkspaceRuntime::view_anchored(&mut self, &Anchedress, AnddressTarget) -> Result<ViewOutcome, ViewError>
WorkspaceRuntime::invalidate_anchored_source(&mut self, &str) -> Result<(), AnchorError>
```

`Anchedress` is an opaque owning Runtime-local handle. It has no `Clone`,
`Copy`, equality, hashing, serialization, wire form, raw-Anddress getter,
adoption, re-anchor operation, or global identifier. Dropping it ends its
continuity; there is no explicit release. It makes no `Send` or `Sync`
guarantee. Using a handle with another Runtime, or using an invalidated handle,
returns the consuming capability's `Unavailable` error.

`anchor` first performs source-less validation, then one retained observation
with direct exact File/Paragraph/Line target projection, then live equality. A
raw-valid nonstructural Paragraph or Line is not anchorable. At most one live
handle exists for one Runtime and one current raw Anddress. An equal live target returns `AlreadyLive` without a
handle, alias, or reference count. An invalidated or dropped handle does not
prevent a fresh anchor, and an equal tuple never revives an old handle. Anchor
has no fixed live-handle cap; a resource failure returns `Unavailable`.

`view_anchored` first validates the requested self-or-ancestor relation against
its selected live binding. It then compares only that binding with current
resolution. Relation-absent has the same normal meaning as ordinary View. In
Host mode, a matching proof shares ordinary trusted View
execution, a mismatch fail-closes same-path proof and continuity before I/O,
and a miss keeps the complete direct target observer. A stale sibling does not
prevent that selected current binding from returning a
View result. A selected-binding mismatch is Runtime-known opaque mutation:
Runtime invalidates every live anchor for that logical source and returns
`Unavailable`. `anchor` keeps its same-path batch check only after its new input
is current, before accepting it alongside the existing live bindings.
An I/O, resource, or invalid Apply failure without mutation evidence retains
continuity. Search and View calls neither create, refresh, nor invalidate
Anchor continuity. Anchor, its consuming View seam, and invalidation use only
`&mut WorkspaceRuntime` call order; they add no counter, lock, worker,
concurrency, or `Send`/`Sync` guarantee.

Apply reuses the exact splice dispositions above. All geometry, after projection,
and disposition preparation completes before publication; after successful
publication Runtime performs only allocation-free, no-failure RAM binding.
There is no filesystem-and-RAM transaction or rollback. If multiple live
handles bind to one resulting raw Anddress, every colliding handle is
invalidated. Apply never automatically anchors a new target.

`invalidate_anchored_source` is the explicit host-known opaque
mutation/deletion ingress. Invalid logical-path syntax returns `InvalidInput`; a
valid but unadmitted path returns `Unavailable`; and an admitted path returns
`Ok(())` even when it has no live handle. It needs no source read and invalidates
every live anchor for that logical path. Validation failure changes no
association. Hard-linked logical paths are reported separately by the host.
There is no watcher, automatic external detection, alias inference, generic
transition engine, successor inference, or past-to-current reconciliation.
Anchor applies only to source-visible mutation boundaries, never editor-only
buffers.

The only special ignored path is Runtime-root-relative `.artext/bw` and its
subtree. `.artext` itself is not globally reserved; other children follow normal
admission and safety policy. Future spill belongs only to a host-provided system
root and is not created or configured in this repository.
