# Backwriter Protocol

Status: normative current-only Core/Runtime contract. The closed public `0.1.0`
release remains immutable v3 evidence. Current unpublished `0.2.0` source uses
the hard-cutover v4 value/wire and all production callers; the retained-
observation and direct consumer fast paths remain Phase 4–6 work.

## Unpublished 0.2.0 current-observation authority

The following guard sentences are normative:

- **Current-only is not stateless.**
- **No history does not mean forgetting the current observation.**
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

View consumes one ordinary Anddress. Phase 3 validates the current source hash,
length, kind, and range during its existing one-read structural scan; it never
relocates a stale target. Check compares the source state and exact structural
range without searching. Apply accepts the hash as its source-state
precondition, validates the recorded range, and only then maps the verified
target to its existing private call-local parser representation. That temporary
ordinal/text representation is not identity, wire, or retained state. Direct
bounded-range consumer paths remain Phase 5–6 work. A mismatch fails closed
before publication.

`CurrentObservation` is permitted Runtime-private current state for later fast
paths; Phase 3 retains no such state across calls. It may retain
only the current source hash, exact byte length, and byte ranges minimally
required by the current capability or fast path. It may not retain a prior
observation, whole source, parse tree, complete Line collection, Search result,
history, persistent index, relocation context, context-matching evidence, or
full workspace cache. When source state changes or cannot be proven current,
Runtime discards the observation before accepting a replacement. This state is
neither an ordinary Anddress field nor durable authority.

An ordinary Anddress has no continuity across a changed source. Re-search may
find a target in the new state and produce a new Anddress, but View, Check, and
Apply never do so implicitly. Reappearance of the same exact source state can
re-establish the same raw v4 value without proving temporal continuity. Anchor
remains the only continuity boundary: only a live Anchor may receive an
arithmetic range transform caused by a Backwriter-owned Apply. External or
opaque source change invalidates rather than relocates ordinary Anddresses and
live Anchor continuity. This adds no history, watcher, retry, CAS, persistent
index, or Git semantics.

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
`WorkspaceRuntime::search(&SearchRequest)`, `WorkspaceRuntime::view(&Anddress)`,
`WorkspaceRuntime::apply(&mut self, &Edit)`,
`WorkspaceRuntime::check(Anddress)`, `check_search(SearchOutcome)`, and
`check_pick(PickOutcome)`. In current Phase 3, Search, View, Pick, Check, and Apply retain no
observation object, source cache, result store, index, snapshot, lease,
registry, or authenticity state. Search enumerates admitted Workspace Source
deterministically through retained capability-relative no-follow handles. For
each selected regular file it observes one byte sequence, validates complete
UTF-8 and NUL policy, parses exact File/Paragraph/Line structure, matches
literal Line content, projects the requested target, and drops its call-local
observation before opening another file. Its separate exact File request
validates one logical path, opens and observes only that admitted regular
source under the same policy, and returns its File Anddress without content
matching or traversal.

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
remains in scope for now. The shared scanner reads retained source through its fixed input scratch
array. Apply separately owns an equally sized fixed output batch. It writes the
accepted before observation to one same-parent staging entry, then writes each
flushed source batch and replacement or required semantic slice to one
prospective-after temporary through the same incremental framing contract. Its
auxiliary source state is that output batch plus only the necessary current
physical-Line/Line disposition and caller/live state; it retains neither a
separate complete before source nor a complete after source in RAM. Only the
closed staging entry may be reread; the source handle and prospective-after
temporary are not reread, sought, or reopened. Runtime-private temporary writing is
prepublication preparation, and only source-visible replacement is publication.
Current resolve, resulting UTF-8/NUL validation, after-parse, every Anchor
disposition and collision, and every fallible preparation complete before
publication; successful reflection remains allocation-free and non-failing.

The v4 Check, Search, View, Anchor, and Apply streaming slices share one private
incremental forward scanner in `runtime/source_scan.rs`, which reads one
retained no-follow handle with a fixed scratch buffer, incrementally hashes the
same bytes, and retains no complete source. Check keeps currentness
classification, while Search keeps KMP matching and range projection. Search
shares one immutable source identity across results from the same source and
copies no Line text into Anddress. View accumulates only its returned text and
the minimum candidate bytes needed by its transitional structural scan. Anchor
shares exact-target evidence with its selected binding observation. Apply
reuses the framer for source-state/range proof, its private call-local parser,
and prospective-after parsing, then prepares Anchor dispositions before the
sole rename. This direction adds no public stream
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
issuer. This creates no separate registry, issuance lifecycle, lookup/reuse
state, durable identity, or global identity.

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

The completed one-shot Search, View, and Check JSON projections are Adapter-only.
Their compact envelopes identify `bw.cli.search.v1`, `bw.cli.view.v1`, and
`bw.cli.check.v1`; Search embeds each
existing encoded v4 Anddress JSON object directly, View embeds its related v4
File and optional Paragraph objects directly, and Check embeds its filtered v4
Anddress object directly when present. They create no Core wire, value model,
Search/View/Check state, result collection, or capability workflow.

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
v4 source identity and exact range. For a Paragraph target, every current
Paragraph containing one or more matching text Lines is returned exactly once
as its v4 range; a matching separator Line does not create a Paragraph. For a
File target, any matching Line, including a separator, returns the current v4
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

## Implemented 0.2.0 View behavior

View V1 has one `&Anddress` input and the implemented
`WorkspaceRuntime::view(&Anddress)` seam. Its admitted capability-relative
no-follow one-read access and File/Paragraph/Line text projection shape remain
reusable. Its former v2 evidence-based construction is rejected; successful
related results use v4 source identity and ranges. View stays current-only, stateless,
non-mutating, and without range, plural, descendant, or partial behavior.

View first performs source-less v4 validation before any I/O. It preserves the
existing `UnsupportedVersion`, `InvalidInput`, and `Unavailable` errors and the
existing `ViewOutcome` shape. Unsupported version or invalid source-less v4
input returns the corresponding first two errors. After that validation, every
coordinate, admission, open, read, UTF-8/NUL, source-state, structural-range,
or resource failure returns `Unavailable`. View adds no public evidence,
registry, cache, retry, second read, error, or type.

A File is current exactly when the input coordinate/path resolves to an
admitted regular UTF-8, NUL-free source whose complete SHA-256 and byte length
match and whose range is `[0,length)`. Admission is not raw equality: another
Runtime with the same workspace coordinate may use the same value whenever it
currently admits that path and observes the exact same source state.

A Paragraph or Line is current only when the complete source identity matches
and the recorded range is exactly a current Paragraph or Line range. Phase 3
uses the shared structural scanner for that proof; it never searches for equal
text at another range.

For a successful Paragraph, the returned File is constructed with the same v4
source identity from the one-read observation. For a successful Line, its
returned File and optional Paragraph share that same identity; a
separator Line has no Paragraph. Re-establishing an identical tuple makes only
a current lookup succeed. It makes no continuity, authenticity, survivor, or
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
nonregular, or symlink source; a complete-source hash/length mismatch; or a
Paragraph or Line range/kind mismatch in an otherwise normal source. Transient
I/O or resource failure, or UTF-8/NUL
classification failure, is `Unavailable` and is never automatically excluded.
Every occurrence completes source-less validation in input order before I/O.
Any failure returns `UnsupportedVersion` or `InvalidInput` without a partial
result. Only allocation failure for output, report, or working memory returns
`Resource`. A source-observation failure is occurrence `Unavailable`, not a
call error. `NotCurrent` requires sufficient proof that its locator does not
hold.

For each call, only a group at the current Runtime coordinate and an admitted
logical path that needs source observation uses at most one retained read. For
normal observed bytes, a group uses one private incremental forward scanner pass
only when a locator needs adjudication. A coordinate mismatch or confirmed
unadmitted, missing, nonregular, or symlink source requires neither read nor
parse. Check has no snapshot, retry, or second read. It does not re-evaluate
Search or Pick, refresh View, retarget, create an Anddress, mutate Anchor or
Workspace state, or otherwise mutate Core state. It introduces no `CheckInput`,
request, builder, disposition enum, public trait, state, or store. Anchedress
values, in-place updates of stored Check outputs, and their associated RAM commit
semantics remain deferred. Any Adapter-facing Check spelling remains Adapter
syntax rather than storage syntax or Data authority. Wire remains outside the
Core cutline; Adapter spelling does not alter Check Core authority.

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

There is no request, outcome, additional error, trait, batch type, or anchored
Apply seam. One call handles exactly one Edit in exactly one logical
source; ordering, batch behavior, transactions, and multi-source atomicity
remain deferred.

Execution validates in this order: `Edit::validate`; equality of the
`workspace_coordinate` and `logical_path` in every target and position
Anddress; Runtime coordinate and admission; one retained, complete UTF-8,
NUL-free source observation; current resolution of every Edit locator and every
same-path live Anchor; prospective-after construction, after-parse, Anchor
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
its environment.

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

After the required source-validity and currentness checks, Empty Insert and a
classified Move at its start or end boundary remove staging and return `Ok(())`
without comparison, publication, or Anchor change. Every other potential no-op
compares its prospective after bytes to the accepted before bytes and returns
`Ok(())` when they are equal. To prepare reverse Move or Copy within bounded
RAM and one source read, only `apply` may write accepted before bytes to a
same-parent, call-local staging entry. After closing it, Apply may reopen,
reread, or seek staging while assembling a prospective-after temporary and
preparing its after-parse and Anchor plan. It never rereads, seeks, or reopens
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

Every same-path live Anchor must be current before publication. A File binding
is preserved. Only an Edit source target supplies mutation-before provenance;
Position supplies splice geometry only. Before prospective-after emission,
when an Edit has a source target and live Paragraph or Line binding
source-target relation or Copy membership requires classification, `apply` makes
one bounded private staging pass. It never derives a relation from Position
during replay. A fully contained live Paragraph or Line
binding follows moved source provenance only when after parsing yields exactly
one same-kind target. A Position-neighbor binding can rebind only from its own
original bytes in the after target. Insert and replacement bytes are mutation
evidence; Move gives source provenance only to a source-member candidate and
treats every other candidate as mutation; Copy leaves the source-member
occurrence neutral and treats every other candidate as mutation. Copy keeps its
original occurrence and never automatically anchors the copied occurrence.
Containing or crossing bindings use the exact after-parse candidate rule. Zero
or multiple candidates, split/join, absorption, ambiguity, and collisions remove
the binding. A known-invalid source or stale same-path binding fail-closes every
live binding for that path. Read, resource, or definite prepublication failures
preserve continuity. `PublicationUncertain` fail-closes every same-path live
binding. Successful reflection is allocation-free and non-failing.

The Rust executor stages the accepted before source once, classifies Move
geometry from that private entry, directly completes Empty Insert and Move
start/end no-ops after staging removal, replays every other potential no-op in
fixed chunks against staged bytes, and publishes only after every fallible
preparation has completed. Cross- or multi-source execution, Data storage,
wire, and Adapter forms remain deferred.

## Implemented 0.1.0 Anchor live-continuity authority

Anchor has this implemented public Runtime surface exactly:

```rust
WorkspaceRuntime::anchor(&mut self, &Anddress) -> Result<AnchorOutcome, AnchorError>
AnchorOutcome::{Anchored(Anchedress), AlreadyLive}
AnchorError::{UnsupportedVersion, InvalidInput, Unavailable}
WorkspaceRuntime::view_anchored(&mut self, &Anchedress) -> Result<ViewOutcome, ViewError>
WorkspaceRuntime::invalidate_anchored_source(&mut self, &str) -> Result<(), AnchorError>
```

`Anchedress` is an opaque owning Runtime-local handle. It has no `Clone`,
`Copy`, equality, hashing, serialization, wire form, raw-Anddress getter,
adoption, re-anchor operation, or global identifier. Dropping it ends its
continuity; there is no explicit release. It makes no `Send` or `Sync`
guarantee. Using a handle with another Runtime, or using an invalidated handle,
returns the consuming capability's `Unavailable` error.

`anchor` first performs source-less validation, then one-read current resolve,
then live equality. At most one live handle exists for one Runtime and one
current raw Anddress. An equal live target returns `AlreadyLive` without a
handle, alias, or reference count. An invalidated or dropped handle does not
prevent a fresh anchor, and an equal tuple never revives an old handle. Anchor
has no fixed live-handle cap; a resource failure returns `Unavailable`.

`view_anchored` compares only its selected live binding with current resolution;
a stale sibling does not prevent that selected current binding from returning a
View result. A selected-binding mismatch is Runtime-known opaque mutation:
Runtime invalidates every live anchor for that logical source and returns
`Unavailable`. `anchor` keeps its same-path batch check only after its new input
is current, before accepting it alongside the existing live bindings.
An I/O, resource, or invalid Apply failure without mutation evidence retains
continuity. Search and View calls neither create, refresh, nor invalidate
Anchor continuity. Anchor, its consuming View seam, and invalidation use only
`&mut WorkspaceRuntime` call order; they add no counter, lock, worker,
concurrency, or `Send`/`Sync` guarantee.

Apply reuses the exact splice dispositions above. All geometry, after-parse,
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
