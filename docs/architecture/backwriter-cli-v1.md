# Backwriter CLI V1

Status: Adapter authority. The completed slices are the canonical `bw`
executable's standalone Version and Update operations, one-shot human and JSON
Search/View/Check, raw View, Anddress-first one-shot Edit, Session Pick, batch
Check, Anchor, Edit, Apply, result-binding, and Data modes only. This document follows the Core active
documents in the authority-reading order.
The `0.2.2` one-shot Anddress-first Edit contract below is closed, implemented,
and published Adapter authority. Source Cargo, `bw version`, the official
installer, and the closed distribution are `0.2.2`.

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

CLI V1 execution has two intended forms:

- One-shot invokes one Adapter command and exits without retaining a result;
  the Anddress-first Edit command contracts its named View and Apply calls.
- The Session retains one `WorkspaceRuntime`, one explicit caller-owned
  `DataStore`, and CLI-local Search/Pick/Anddress/Edit/View/Check values plus
  non-aliasing owning Anchedress handles until EOF or `exit`.

Both forms construct the existing default Runtime and therefore use Untrusted
Mode. One-shot creates a new Runtime for its command; Session retains one
Runtime but has no CLI syntax or implicit authority that enables the
implemented `0.2.1` Host-authoritative Mode. The Rust host seam exists, but complete writer
coordination remains a host responsibility; the CLI defines no flag, command,
token, or Session behavior for it.

One-shot human and JSON Search, View, and Check plus raw View and Anddress-first
Edit, Session Pick, batch Check, Anchor, Edit, Apply, result binding, and
explicit typed Data are implemented.
Standalone `version` and `update` are Adapter-owned executable operations, not
Core capabilities or Session commands.
One-shot Data and Anchor are intentionally unsupported because their DataStore
and live-handle contracts require Session lifetime. One-shot Pick, batch Check,
and raw Edit/Apply transport remain deferred. The distinct `0.2.2`
Anddress-first one-shot Edit form is implemented. Raw
output other than one-shot View, all other capabilities, and further Session
behavior are deferred and rejected rather than silently accepted.

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
Backwriter 0.2.2
```

including the final LF and no other successful output.

`bw update` downloads the current platform's official installer over HTTPS and
delegates installation to it. The current official manifest selects the closed
public `0.2.2` distribution. The installer accepts only the exact `0.2.1` and
`0.2.2` manifests; `0.2.0`, stable `0.1.0`, and beta.3 acceptance is retired. Update does not
publish a release and performs no local version comparison, retry,
daemon or background update, and adds no compatibility alias. On Unix it uses a
private temporary directory, runs the downloaded `install.sh` synchronously
with `sh`, propagates its exit status, and removes the temporary directory. On
Windows it starts the downloaded `install.ps1` with the current process ID and
the exact private bootstrap root, then exits so PowerShell can wait for the
parent before replacing `bw.exe`. A Windows parent status of `0` means only that
handoff started successfully; the child owns final installer output, final
status, replacement, and bootstrap cleanup.

The source-built and published `0.2.2` command has no version-comparison guard.
Invoking Update therefore installs or reinstalls the official `0.2.2` release.
This boundary does not authorize a guard, retry, rollback, alternate installer,
or publication.

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
is one-shot View-only. Duplicate, mixed, or output-option-position use after a
capability is a usage error. In the required one-shot Edit Content position,
the exact strings `--json` and `--raw` are ordinary Content rather than output
selections; a later token remains an extra-operand usage error. Session and
every other one-shot capability reject output selections.

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

## Implemented 0.2.2 one-shot Anddress-first Edit

The exact syntax is:

```text
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    edit anddress <encoded-v4-Anddress> <content>
```

This is the canonical general editing form. It accepts one host argv Content
value and no output selection. Search or Pick may provide the encoded v4
Anddress to a caller, but the CLI requires no caller-visible View, Check,
Session binding, index, or Core Edit value. It opens one ordinary Untrusted
Runtime, reuses the existing strict v4 decoder, privately calls Runtime View,
constructs one existing `Edit::Replace`, and calls Runtime Apply on that same
Runtime. It adds no request type, result type, Core wire, Runtime seam, retained
state, or automatic capability handoff.

For File and Paragraph, `<content>` is the exact replacement string and retains
the existing NUL rejection. For Line, it is body-only Content and any NUL, CR,
or LF is a usage error. The Adapter appends exactly the current None, LF, CR,
or CRLF terminator reported by the private View. It performs no other escape
decoding, trimming, normalization, separator insertion, or target conversion.
The positional values `--json` and `--raw` follow those same rules exactly.
Leading global `--json edit ...` or `--raw edit ...` remains unsupported, and
either string after the required Content remains an extra operand.

Success writes exactly `OK` followed by one LF through the existing Session
status writer and exits `0`. CLI grammar, v4 decode/validation other than
resource exhaustion, and invalid Content are usage errors: the existing usage
reporter writes `error: <message>`, one blank line, the complete usage text, and
one final LF to stderr, then exits `2`. Target-specific Content rejection uses
the existing `Edit input is invalid` message. Decoder resource exhaustion,
Runtime View or Apply failure, and stdout failure are execution errors: the
existing execution reporter writes exactly `error: <message>` plus LF to stderr
and exits `1`. Existing View/Apply error text and Apply variants are retained,
including `current source is unavailable` and `source replacement result is
uncertain`; mutation that leaves a nonmatching source state between the private
View and Apply reports the former broad `Unavailable`, never a new
`NotCurrent`. No success bytes are written before Apply succeeds.

Byte-identical no-op, publication uncertainty, Anchor reflection, and optional
Host proof/invalidation semantics remain owned by existing Runtime Apply. The
implemented raw Session Edit/Apply form below remains an advanced surface and
is not an alias or prerequisite for this command. JSON, raw output, batch Edit,
stdin/file content transport, retry, merge, relocation, and automatic re-search
are not part of this closed form.

Gate 3 closes without another Content transport or machine-output form. One
argv value already carries empty and Unicode Content, File and Paragraph CR/LF,
and every allowed Line body; NUL is invalid by contract. Search JSON already
provides an exact v4 Anddress object, while Edit returns no new target or result.
Success is exit `0` plus `OK` and LF, and existing usage/execution failures are
exit `2`/`1` with stderr. Returning an Anddress would require the forbidden
implicit re-search, and a JSON envelope could not refine broad `Unavailable`,
publication uncertainty, or stdout failure after Apply. Because the status
write follows Apply, exit `1` is not evidence that source bytes are unchanged
and must not trigger an automatic retry.

The retained argv transport is constrained by operating-system argument-length
limits, shell-specific quoting and newline portability, and possible Content
exposure through process listings or shell history. Only a reproduced consumer
failure, measured payload need, or concrete security requirement can reopen an
Owner decision for one additional transport. No future syntax or implementation
is reserved here.

### Gate 4 default flow and raw Session comparison

The default caller flow is JSON Search followed by one-shot Edit. The caller
selects one exact object from the Search envelope's `anddresses` array and
passes that complete v4 object unchanged as one argv value:

```text
bw --json search line "retry_budget = 3" --source note.txt
bw edit anddress '<opaque-v4-object>' 'retry_budget = 5'
```

A human Search row contains display text rather than an encoded Anddress and
cannot be used here. The caller treats every v4 field, including hash, length,
kind, and range, as opaque. View or Pick may assist selection, but neither is a
prerequisite and Check is optional. After success, the old Anddress is not
reusable; obtaining another current address requires an explicit Search.

The comparison fixture is one admitted `note.txt` whose only Line has exact
bytes `retry_budget = 3` plus CRLF. JSON Search produced one exact 311-byte v4
object. Passing those bytes unchanged to one-shot Edit with body Content
`retry_budget = 5` exited `0`, wrote exactly `OK` plus LF, and preserved CRLF.
Reusing the old object in a separate control invocation exited `1`, wrote
`error: current source is unavailable` plus LF to stderr, and left the edited
bytes unchanged. This control demonstrates one stale case; exit `1` generally
is neither stale-only nor proof that no publication occurred.

The corresponding raw Session input was:

```text
let hits = search line "retry_budget = 3"
view anddress @hits[0]
let replacement = edit replace @hits[0] "retry_budget = 5\r\n"
apply @replacement
exit
```

It used one `bw shell` process, four work expressions, and one `exit` control
expression. Its exact successful output was `Found 1` plus LF, the indexed
human row plus LF, the viewed Line with CRLF, then `OK` plus LF. The final
source bytes were byte-identical to the canonical path. The View expression is
optional when the caller already knows the terminator. The raw caller owns the
Search binding, result index, Session quoting and escape decoding, exact
terminator, Edit binding, and separate Apply; raw Session is therefore the
advanced Insert/Delete/Move/Copy, Position, Anchor/Data lifetime composition
surface, not a prerequisite or alias for ordinary Replace.

| Boundary | Anddress-first one-shot Edit | Raw Session Edit/Apply |
| --- | --- | --- |
| Invocation count | Two processes and two one-shot Adapter commands when Search is needed; one process and Edit command when the address is already known. The Edit command internally calls View and Apply | One Session process; four work expressions in the compared flow, plus `exit` |
| Selection and Content | One opaque v4 argv object; private View; File/Paragraph exact Content or Line body-only Content | Named binding and index; optional explicit View; caller supplies the raw replacement including the exact Line terminator |
| Pre-publication failure | Grammar, decode, or Content rejection exits `2`; Runtime View or Apply failure exits `1` | Search/Edit expression failure retains no new publication; Apply is a separate expression and publication boundary |
| Success and output failure | Apply precedes exact `OK` plus LF; a later stdout failure exits `1` without proving unchanged source | Apply also precedes its status write; the Session retains bindings and accumulates expression status until EOF or `exit` |

No time or speed advantage is claimed. The task-local JSON extraction used to
verify exact object transfer is test evidence, not an installed tool, wrapper,
schema, or README dependency. Neither path authorizes automatic retry.

### Gate 5 retained surface separation

Consumer reaudit retains the existing raw Session and canonical one-shot forms
as different Adapter responsibilities. Raw Session exposes all five Core Edit
variants and four Position forms with exact bytes, explicit binding/index
selection, Edit clone/reuse, and a separate borrowed Apply call. An Edit binding
cannot be indexed or transferred to `DataStore`. The canonical one-shot form
continues to expose only Anddress-plus-Content Replace and privately reuses
`Edit::Replace` plus Apply; only that Adapter preserves a Line terminator.

The deferred top-level one-shot `apply` branch remains an explicit usage-error
boundary; treating it as an unknown capability would remove no concept. Session
validates an Edit before storing its binding, while `WorkspaceRuntime::apply`
validates again to defend every public Rust caller. Those checks have distinct
consumers and are not duplicate execution paths. Gate 5 adds no raw prefix,
rename, alias, facade, re-export, feature gate, parallel enum/executor, shim,
one-shot Insert/Delete/Move/Copy, raw Edit transport, or Edit Data kind.

Gate 6 added one integration control without changing this execution path. It
removes only the fixed Search JSON envelope bytes, validates the remaining
single object as v4, and passes those exact bytes unchanged as one Edit argv.
At Gate 6 the source version became `0.2.2`, while Core, Runtime, v4 wire, and
then-published `0.2.1` behavior remained unchanged. The subsequent Gate 7
publication changed only the official distribution boundary to `0.2.2`.

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
- One-shot Pick, batch Check, and raw Edit/Apply transport, pending collection
  or transport authority. The implemented Anddress-first one-shot Edit form
  above is distinct from raw Edit transport.
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
