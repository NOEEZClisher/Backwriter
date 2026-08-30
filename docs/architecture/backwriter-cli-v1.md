# Backwriter CLI V1

Status: Adapter authority. The completed slices are the canonical `bw`
executable's standalone Version and Update operations, one-shot human and JSON
Search/View/Check, raw View, Session Pick, batch Check, Anchor, Edit, Apply,
result-binding, and Data modes only. This document follows the Core active
documents in the authority-reading order.

The CLI is the first official Adapter inside the repository cutline. It exposes
Core semantics without redefining Core Rust APIs, target identity, wire, error
authority, provenance, or a capability workflow. Its syntax and caller value
passing never prescribe a Rust call order or make a new Core wire.

## Executable and execution forms

The canonical executable is:

```text
bw
```

`backwriter` is the Cargo package, library crate, and Core namespace, not an
executable. The repository provides no `backwriter` binary, alias, or wrapper.
External callers invoke `bw`, which adapts to `backwriter` Core. Product prose
uses Backwriter; persisted `artext.backwriter-*` wire values, `.artext/bw`, and
distribution artifact/domain names are unchanged contracts.

CLI V1 capability execution has two intended forms:

- One-shot invokes one capability and exits without retaining a result.
- The Session retains one `WorkspaceRuntime`, one explicit caller-owned
  `DataStore`, and CLI-local Search/Pick/Anddress/Edit/View/Check values plus
  non-aliasing owning Anchedress handles until EOF or `exit`.

One-shot human and JSON Search, View, and Check plus raw View, Session Pick,
batch Check, Anchor, Edit, Apply, result binding, and explicit typed Data are
implemented.
Standalone `version` and `update` are Adapter-owned executable operations, not
Core capabilities or Session commands.
One-shot Data and Anchor are intentionally unsupported because their DataStore
and live-handle contracts require Session lifetime. One-shot Pick, batch Check,
Edit, and Apply await collection or Edit transport schema authority. Raw output
other than one-shot View, all other capabilities, and further Session behavior
are deferred and rejected rather than silently accepted.

The intended expression roles remain:

```text
Capability → Operation → Kind → Operand → Position → Payload → Qualifier
```

This is an Adapter expression order, not a Core method signature, provenance
claim, automatic handoff, or required workflow.

## Implemented standalone Version and Update

The complete syntax is:

```text
bw version
bw update
```

Both forms reject every option and operand, including workspace, admission, and
output selections. They are unavailable inside Session. Neither opens a
workspace, calls Core or Runtime, defines a wire value, or establishes a
capability workflow.

`bw version` writes exactly:

```text
Backwriter 0.2.0
```

including the final LF and no other successful output.

`bw update` downloads the current platform's official installer over HTTPS and
delegates installation to it. The current official installer selects the
closed public `0.1.0` distribution; it does not publish or install the
unpublished `0.2.0` source build.
Update performs no local version comparison, retry,
daemon or background update, and adds no compatibility alias. On Unix it uses a
private temporary directory, runs the downloaded `install.sh` synchronously
with `sh`, propagates its exit status, and removes the temporary directory. On
Windows it starts the downloaded `install.ps1` with the current process ID and
the exact private bootstrap root, then exits so PowerShell can wait for the
parent before replacing `bw.exe`. A Windows parent status of `0` means only that
handoff started successfully; the child owns final installer output, final
status, replacement, and bootstrap cleanup.

## Implemented one-shot Search

The complete syntax for this slice is:

```text
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    search <line|paragraph|file> <query>
    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    search /file <logical-path>
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json
    search <line|paragraph|file> <query>
    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json
    search /file <logical-path>
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json
    view anddress <encoded-v4-Anddress>
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json
    check anddress <encoded-v4-Anddress>
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --raw
    view anddress <encoded-v4-Anddress>
```

The default workspace is the process current working directory. An explicit
`--workspace` must be absolute and is passed directly to `WorkspaceRuntime`;
the CLI does not canonicalize it or bypass Runtime root and symlink checks.

`--admit` is repeatable only before the capability. With no `--admit`, the
single admission root is `.`. `--source` and `--subtree` are repeatable only
after a content query. With no scope selector, content Search uses
`AllAdmitted`. Exact File Search accepts no scope selector.

`--json` and `--raw` are optional mutually exclusive global output selections.
Each appears before the capability at most once and can occur in any order among
`--workspace` and `--admit`. `--json` is Search/View/Check-only, while `--raw`
is one-shot View-only. Duplicate, mixed, or post-capability use is a usage
error. Session and every other one-shot capability reject output selections.

A content query is exactly one argv value supplied by the host shell. The CLI
has no secondary quoting or tokenization. It directly uses `AdmissionRoot`,
`WorkspaceAdmission`, `SearchQuery`, `SearchScopeEntry`, and `SearchScope` for
all duplicate, overlap, logical-path, and query validation.

`search /file` is a separate exact logical File form, not a content query or
target projection. It accepts exactly one `logical-path`, validates it with
the Core request constructor, and passes the resulting `SearchRequest`
unchanged to `WorkspaceRuntime::search`. An admitted regular UTF-8, NUL-free
source returns one ordinary File Anddress whether it is empty or nonempty.
Missing paths and directories return Empty. Invalid paths are usage errors;
unadmitted or unavailable sources retain the existing Search execution-error
boundary. The form creates no fake query, Line, Paragraph, wire, or Adapter
result type.

The CLI has no parser framework, Core facade, second validation model, cache,
automatic selection, daemon, or background updater. Session state is limited to
the explicit Session contract, and the only detached process is the Windows
Update replacement handoff described above.

### Exit and stream rules

- Success exits `0` and writes only to stdout.
- CLI grammar or input errors exit `2`; their error and usage text write only
  to stderr.
- Runtime/Search execution errors and stdout write failures exit `1` and write
  errors only to stderr.
- Adapter Version/Update I/O or process-launch errors exit `1` and write only to
  stderr. Unix Update propagates the installer status; Windows status `0` has
  the narrower handoff meaning defined above.
- `--help` as the sole argument exits `0` and writes usage to stdout.

Unsupported capabilities and unsupported output forms are explicit usage errors
in this slice.
There are no short option aliases.

### Human Search projection

The sole implemented result projection is exactly:

```text
Found <count>
<index>\t<File|Paragraph|Line>\t<logical-path>[:<byte-start>-<byte-end>]
```

`Empty` is one line, `Found 0`. `Found` preserves the Core result vector's
existing deterministic order. File rows omit the full-source range; Paragraph
and Line rows include their exact byte range. The human projection never
modifies an internal `SearchOutcome` or `Anddress`; it omits raw Anddress,
workspace coordinate, source hash, and source length. Preview is not
implemented.

### JSON Search projection

With the global `--json` flag, Search writes exactly one compact UTF-8 JSON value
followed by one LF. Its keys are ordered `schema`, `outcome`, and `anddresses`:

```json
{"schema":"bw.cli.search.v1","outcome":"empty","anddresses":[]}
```

or:

```json
{"schema":"bw.cli.search.v1","outcome":"found","anddresses":[<exact-v4-Anddress-object>]}
```

The writer maps `SearchOutcome::Empty` and `Found` directly. It streams the
existing outcome in its existing order, retaining duplicate occurrences and
every source-state identity and exact range. Each array member is the exact v4
`Anddress::encode()` object, not a JSON string, preview, normalized value, or
new CLI/Core wire. It allocates neither a JSON `Value` nor a second result
collection. Encoding resource and stdout failure are execution errors; a
successful JSON response contains no diagnostic bytes.

## Implemented one-shot View

The complete syntax for this slice is:

```text
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    view anddress <encoded-v4-Anddress>
```

View reuses the same global workspace and admission parsing as Search. Its
operand is exactly one argv value decoded by `Anddress::decode`; it introduces
no CLI address schema, alias, shorthand, or wrapper. Invalid encoding, version,
or address is a usage error. Resource, Runtime, source, View, and stdout errors
are execution errors. `view anchored` and extra operands are explicit usage
errors in this one-shot form.

Human View output contains only the selected target's exact text: File and
Paragraph write their text unchanged, and Line writes its content followed by
its exact None/LF/CR/CRLF terminator. It adds no header, automatic newline,
preview, truncation, raw Anddress, or related File/Paragraph address.

### JSON View projection

With the global `--json` flag, View decodes one v4 Anddress, calls the existing
Runtime View seam once, and writes exactly one compact UTF-8 JSON value followed
by one LF. Its schema is Adapter-only, not a Core wire. Its fixed key orders are:

```json
{"schema":"bw.cli.view.v1","kind":"file","text":"..."}
{"schema":"bw.cli.view.v1","kind":"paragraph","text":"...","file":<exact-v4-Anddress-object>}
{"schema":"bw.cli.view.v1","kind":"line","content":"...","terminator":"none|lf|cr|crlf","file":<exact-v4-Anddress-object>,"paragraph":<exact-v4-Anddress-object-or-null>}
```

`text` and `content` use the existing JSON string writer directly. Related
File/Paragraph values are their exact existing v4 `Anddress::encode()` objects,
not strings or new CLI values. Line terminators project only to `none`, `lf`,
`cr`, or `crlf`; a separator Line has `paragraph:null`. The writer retains no
JSON `Value`, cloned `ViewOutcome`, complete JSON string, or result collection.
Encoding resource and stdout failure are execution errors, and a successful
JSON response contains no diagnostic bytes. The human View projection is
unchanged.

### Raw View projection

With the global `--raw` flag, View performs the same one-v4-Anddress decode and
one Runtime View call as ordinary View, then uses the existing human View writer
unchanged. File, Paragraph, and Line stdout is therefore byte-for-byte identical
to default View, including Unicode and exact None/LF/CR/CRLF/no-EOL terminators.
It adds no writer, buffer, normalization, header, automatic LF, Core wire, or
new View meaning. Raw output is otherwise deferred.

## Implemented one-shot Check

The complete syntax for this slice is:

```text
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    check anddress <encoded-v4-Anddress>
```

Check shares View's one-value v4 Anddress decoding and global workspace and
admission parsing. It passes the decoded value directly to
`WorkspaceRuntime::check`; it introduces no CLI input schema, request, wrapper,
alias, or retained result. The only successful human outputs are one of
`Current`, `NotCurrent`, or `Unavailable`, followed by one newline. They map
the one-input Check report exactly and never display an address or report member.
All three are successful Check outcomes. Invalid input is a usage error; Runtime,
Check resource, and stdout errors are execution errors. One-shot `check search`,
`check pick`, and extra operands are usage errors in this slice.

### JSON Check projection

With the global `--json` flag, Check decodes one v4 Anddress, calls the existing
Runtime Check seam once, and writes exactly one compact UTF-8 JSON value followed
by one LF. Its schema is Adapter-only, not a Core wire. Its fixed key orders are:

```json
{"schema":"bw.cli.check.v1","status":"current","filtered":<exact-v4-Anddress-object>}
{"schema":"bw.cli.check.v1","status":"not-current","filtered":null}
{"schema":"bw.cli.check.v1","status":"unavailable","filtered":<exact-v4-Anddress-object>}
```

The JSON and human writers share the existing raw one-input Check-report
classification. `current` and `unavailable` contain the exact existing filtered
v4 `Anddress::encode()` object; `not-current` contains only `filtered:null`.
An inconsistent report/filtered combination is an execution error before either
writer emits output. The writer keeps no JSON `Value`, cloned `CheckOutcome`, or
result collection. The human Check projection is unchanged.

## Implemented Session Pick, batch Check, Anchor, Edit, Apply, result binding, and Data

The Session starts with:

```text
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell
```

It opens one Runtime before reading stdin, retains it until EOF or `exit`, and
prints no prompt. One physical input Line is one expression; blank Lines are
ignored. Its private lexer splits spaces and tabs, and every standalone
double-quoted token decodes exactly `\\`, `\"`, `\n`, `\r`, and `\t`. It rejects
NUL, unmatched quotes, and every other quoted escape. It creates no single-quote, comment,
continuation, interpolation, or pipe grammar. Only the Pick predicate tail splits
parentheses; it changes no other Session token syntax.

The completed commands are `search <kind> <query> [scope]`, `search /file
<logical-path>`, `let <name> = search <kind> <query> [scope]`, `let <name> =
search /file <logical-path>`, `pick @<search-or-pick-binding> <predicate>`,
`let <name> = pick @<search-or-pick-binding> <predicate>`, `let <name> =
@<name>`, `let <name> = @<name>[<index>]`, `view anddress @<name>[<index>]`,
`check anddress @<name>[<index>]`, `check search @<search-binding>`,
`check pick @<pick-binding>`, `let <name> = anchor create <anddress-ref>`,
`view anchored @<handle>`, `anchor invalidate-source <logical-path>`,
`let <name> = edit <operation> ...`, `apply @<edit-binding>`,
`data store <kind> <name> <value-ref>`, `data get <kind> <name>`,
`let <binding> = data get <kind> <name>`, `data rename <kind> <old> <new>`,
`data remove <kind> <name>`, `data list`, and `exit`. Pick predicates are exactly
`all`, `target-kind <file|paragraph|line>`, `one-of <anddress-ref>...`,
`same-file <anddress-ref>`, `not (<predicate>)`, `all-of (<predicate>)...`, and
`any-of (<predicate>)...`; attached and separate parentheses are equivalent.
The adapter parses those predicates into the existing `PickPredicate` constructors
and calls Core `pick`; it never evaluates a predicate itself. Direct address
bindings may be supplied to View or Check without an index. Names are ASCII
identifiers and cannot be redefined. The Session values are exact
`SearchOutcome`, `PickOutcome`, copied `Anddress`, `Edit`, `ViewOutcome`, and
Check outcome values plus owning non-aliasing `Anchedress` handles. Bindings are
private Adapter memory and never cause automatic Core Data storage. Search,
Pick, View, and Check output are written before a `let` binding becomes
available.

Pick preserves the Core result vector's order and multiplicity. Its human
projection is exactly:

```text
Selected <count>
<index>\t<File|Paragraph|Line>\t<logical-path>[:<byte-start>-<byte-end>]
```

`Empty` writes `Selected 0`. Pick address references use an Anddress binding or
an indexed Search/Pick binding. A malformed predicate, parenthesis, binding, or
index is a Session usage error. Pick resource and stdout errors are execution
errors. The Session lexer remains unchanged for every non-Pick command; only the
Pick predicate tail splits attached parentheses.

Session batch Check accepts exactly one unindexed, matching named binding and
passes its exact clone to `WorkspaceRuntime::check_search` or `check_pick`. It
does not consume, filter, replace, or store that binding. Its successful human
output is exactly:

```text
Checked <checked_count>
Current <current_count>
NotCurrent <removed_count>
Unavailable <unavailable_count>
```

The counts are the Core `CheckReport` counts, including zero for an empty native
outcome. The adapter displays neither the filtered outcome nor any address,
coordinate, or extent. A report whose categories cannot sum to `Checked` is an
execution error. An indexed, unknown, wrong-kind, or extra binding operand is a
usage error. This adds no Check result binding, DataStore transfer, automatic
binding update, latest value, JSON, or raw output.

Session Anchor accepts `create` only as the right-hand side of `let`. Its
operand is a direct Anddress binding or an indexed Search/Pick result. Duplicate
binding names fail before the Runtime call. `Anchored` prints `Anchored` and
binds the requested name; `AlreadyLive` prints `AlreadyLive` without creating an
alias. An Anchedress can neither be cloned, indexed, nor used where an Anddress
is required. `view anchored` passes the owning handle to the existing Runtime
anchored View seam and uses the ordinary exact View writer. `anchor
invalidate-source` accepts exactly one logical path and prints `OK` on success;
it invalidates only handles for that source. Anchor input/version errors are usage
errors; unavailable and View errors are execution errors. The Session never
deletes, recreates, persists, registers, or re-identifies a handle.

Session Edit directly constructs and validates the existing Core `Edit` and
`Position` values. Its positions are `before`, `after`, `start-of`, and `end-of`
with one Anddress reference; only `let` can retain an Edit. `apply` accepts one
unindexed Edit binding, retains that caller-owned binding, calls Runtime Apply,
and writes `OK` only on success. All Runtime Apply failures are execution errors.
No preview, retry, rollback, transaction, or CLI recovery is added.

Session `let` can retain exact Core `ViewOutcome`, raw `CheckOutcome`, Search
Check outcome, and Pick Check outcome after writing the same existing human
projection. These values can also be transferred only by an explicit typed Data
Store, which retains an exact clone and leaves the source binding unchanged.

Session Data has one private `DataStore` for the complete shell lifetime. Its
seven exact native kinds are `anddress`, `search`, `pick`, `view`,
`check-anddress`, `check-search`, and `check-pick`. `anddress` Store accepts a
direct Anddress binding or indexed Search/Pick result; every other kind accepts
only an unindexed matching value binding. Anchedress and Edit are rejected. Get
uses the existing human writer; an Anddress Get writes exactly
`Anddress\t<File|Paragraph|Line>\t<logical-path>[:<byte-start>-<byte-end>]` and
never raw wire. `let ... = data get ...` gets once, writes once, then retains
the exact cloned value under the requested binding name.

Data names are passed unchanged to `DataName`: only an empty name is invalid.
Store duplicates and Rename collisions are scoped to one kind, so equal names in
different kinds are independent. Successful Rename and Remove write `OK`. List
uses Core `DataStore::list` order and writes
`<kind>\t"<escaped-name>"`; quote, backslash, LF, CR, tab, and other control
characters are escaped. Empty List writes nothing. Data missing, duplicate,
empty-name, malformed reference, and wrong-kind failures are usage errors;
resource or stdout failures are execution errors. Every Data failure preserves
existing Data entries and Session bindings. The store is dropped at EOF or
`exit`: it adds no persistence, automatic Store/latest, wire, cache, registry,
or capability execution.

Each Session command reuses the completed one-shot Search, View, and Check
validation, Runtime execution, and human projection. A command error writes to
stderr and leaves later Lines runnable. Any usage error makes the final process
status `2`; otherwise execution/resource errors make it `1`; otherwise it is
`0`. Stdin or stdout failure ends the process with `1`.

## Deferred CLI V1 authority

The following are intentionally outside the completed initial slice:

- One-shot Data and Anchor, which require the Session-owned DataStore or live
  handle lifetime.
- One-shot Pick, batch Check, Edit, and Apply, pending collection or Edit
  transport schema authority.
- Raw output other than one-shot View, and any JSON form other than one-shot
  Search, View, or Check.

These require owner decisions before implementation. The high-level intended
form remains one-shot capability execution without equating CLI-local names with
`DataStore`.

The Session introduces no implicit `latest`, hidden current selection,
automatic Data storage, automatic Search-to-Pick/View/Edit/Apply/Data handoff,
automatic Anchor creation, persistent daemon, watcher, or Core workflow encoded
by shell pipelines.

Search scope selectors, Pick candidates before an Adapter predicate, raw Anddress
View/Check references, batch Check named outcome bindings, anchored View input,
and Anchor logical-source invalidation are implemented in Session. The explicit
Edit, Apply, result-binding, and typed Data forms remain Adapter syntax only;
none is a public Core wire or a new Core workflow.

Machine-oriented JSON other than the completed Search/View/Check schemas, and
exact text raw output other than completed View if defined, are Adapter output
schemas rather than
`SearchOutcome`, `PickOutcome`, `ViewOutcome`, `CheckOutcome`, or Anddress wire authority.
