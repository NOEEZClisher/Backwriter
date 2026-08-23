# Backwriter CLI V1

Status: Adapter authority. The completed slices are the canonical `backwriter`
executable's one-shot human Search, View, Check, Session Pick, batch Check, and
Anchor modes only. This document follows the Core active documents in the
authority-reading order.

The CLI is the first official Adapter inside the repository cutline. It exposes
Core semantics without redefining Core Rust APIs, target identity, wire, error
authority, provenance, or a capability workflow. Its syntax and caller value
passing never prescribe a Rust call order or make a new Core wire.

## Executable and execution forms

The canonical executable is:

```text
backwriter
```

`bw` is not a Backwriter binary. A user may create a personal shell alias, but
that alias is outside this Adapter contract.

CLI V1 has two intended execution forms:

- One-shot invokes one capability and exits without retaining a result.
- The Session retains one `WorkspaceRuntime` and explicit CLI-local
  Search/Pick/Anddress values plus non-aliasing owning Anchedress handles until
  EOF or `exit`.

One-shot Search, View, and Check plus Session Pick, batch Check, and Anchor are
implemented. Core `DataStore`, one-shot Pick, one-shot batch Check, one-shot
Anchor, all other capabilities, JSON, raw output, and further Session behavior
are deferred and rejected rather than silently accepted.

The intended expression roles remain:

```text
Capability → Operation → Kind → Operand → Position → Payload → Qualifier
```

This is an Adapter expression order, not a Core method signature, provenance
claim, automatic handoff, or required workflow.

## Implemented one-shot Search

The complete syntax for this slice is:

```text
backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    search <line|paragraph|file> <query>
    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...
```

The default workspace is the process current working directory. An explicit
`--workspace` must be absolute and is passed directly to `WorkspaceRuntime`;
the CLI does not canonicalize it or bypass Runtime root and symlink checks.

`--admit` is repeatable only before the capability. With no `--admit`, the
single admission root is `.`. `--source` and `--subtree` are repeatable only
after the query. With no scope selector, Search uses `AllAdmitted`.

The query is exactly one argv value supplied by the host shell. The CLI has no
secondary quoting or tokenization. It directly uses `AdmissionRoot`,
`WorkspaceAdmission`, `SearchQuery`, `SearchScopeEntry`, and `SearchScope` for
all duplicate, overlap, logical-path, and query validation. It passes a
validated `SearchRequest` unchanged to `WorkspaceRuntime::search`.

The CLI has no parser framework, Core facade, second validation model, cache,
session state, automatic selection, or background process.

### Exit and stream rules

- Success exits `0` and writes only to stdout.
- CLI grammar or input errors exit `2`; their error and usage text write only
  to stderr.
- Runtime/Search execution errors and stdout write failures exit `1` and write
  errors only to stderr.
- `--help` as the sole argument exits `0` and writes usage to stdout.

Unsupported capabilities, `--json`, and `--raw` are explicit usage errors in
this slice. There are no short option aliases.

### Human Search projection

The sole implemented result projection is exactly:

```text
Found <count>
<index>\t<File|Paragraph|Line>\t<logical-path>[:<zero-based-ordinal>]
```

`Empty` is one line, `Found 0`. `Found` preserves the Core result vector's
existing deterministic order. The human projection never modifies an internal
`SearchOutcome` or `Anddress`; it only omits raw Anddress, workspace coordinate,
and complete Line `ExactExtent` from display. Preview is not implemented.

## Implemented one-shot View

The complete syntax for this slice is:

```text
backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    view anddress <encoded-v3-Anddress>
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

## Implemented one-shot Check

The complete syntax for this slice is:

```text
backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    check anddress <encoded-v3-Anddress>
```

Check shares View's one-value v3 Anddress decoding and global workspace and
admission parsing. It passes the decoded value directly to
`WorkspaceRuntime::check`; it introduces no CLI input schema, request, wrapper,
alias, or retained result. The only successful human outputs are one of
`Current`, `NotCurrent`, or `Unavailable`, followed by one newline. They map
the one-input Check report exactly and never display an address or report member.
All three are successful Check outcomes. Invalid input is a usage error; Runtime,
Check resource, and stdout errors are execution errors. One-shot `check search`,
`check pick`, and extra operands are usage errors in this slice.

## Implemented Session Pick, batch Check, and Anchor

The Session starts with:

```text
backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell
```

It opens one Runtime before reading stdin, retains it until EOF or `exit`, and
prints no prompt. One physical input Line is one expression; blank Lines are
ignored. Its private lexer splits spaces and tabs, supports a standalone
double-quoted token with only `\\` and `\"` escapes, and rejects NUL, unmatched
quotes, and every other quoted escape. It creates no single-quote, comment,
continuation, interpolation, or pipe grammar. Only the Pick predicate tail splits
parentheses; it changes no other Session token syntax.

The completed commands are `search <kind> <query> [scope]`, `let <name> =
search <kind> <query> [scope]`, `pick @<search-or-pick-binding> <predicate>`,
`let <name> = pick @<search-or-pick-binding> <predicate>`, `let <name> =
@<name>`, `let <name> = @<name>[<index>]`, `view anddress @<name>[<index>]`,
`check anddress @<name>[<index>]`, `check search @<search-binding>`,
`check pick @<pick-binding>`, `let <name> = anchor create <anddress-ref>`,
`view anchored @<handle>`, `anchor invalidate-source <logical-path>`, and
`exit`. Pick predicates are exactly
`all`, `target-kind <file|paragraph|line>`, `one-of <anddress-ref>...`,
`same-file <anddress-ref>`, `not (<predicate>)`, `all-of (<predicate>)...`, and
`any-of (<predicate>)...`; attached and separate parentheses are equivalent.
The adapter parses those predicates into the existing `PickPredicate` constructors
and calls Core `pick`; it never evaluates a predicate itself. Direct address
bindings may be supplied to View or Check without an index. Names are ASCII
identifiers and cannot be redefined. The Session values are exact
`SearchOutcome`, `PickOutcome`, copied `Anddress` values, and owning
non-aliasing `Anchedress` handles; they are private Adapter memory, not Core
DataStore state. Search and Pick output are written before a `let` binding
becomes available; View and Check results are not bindable.

Pick preserves the Core result vector's order and multiplicity. Its human
projection is exactly:

```text
Selected <count>
<index>\t<File|Paragraph|Line>\t<logical-path>[:<zero-based-ordinal>]
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

Each Session command reuses the completed one-shot Search, View, and Check
validation, Runtime execution, and human projection. A command error writes to
stderr and leaves later Lines runnable. Any usage error makes the final process
status `2`; otherwise execution/resource errors make it `1`; otherwise it is
`0`. Stdin or stdout failure ends the process with `1`.

## Deferred CLI V1 authority

The following are intentionally outside the completed initial slice:

- One-shot Pick, one-shot batch Check, one-shot Anchor, Edit, Apply, and Data
  commands.
- JSON schema and the exact scope of raw output.

These require owner decisions before implementation. The high-level intended
forms remain one-shot capability execution and later Session work that can bind
anchored handles, build an inert Edit, Apply an explicit Edit, and expose Core
Data without equating CLI-local names with `DataStore`.

The Session introduces no implicit `latest`, hidden current selection,
automatic Data storage, automatic Search-to-Pick/View/Edit/Apply handoff,
automatic Anchor creation, persistent daemon, watcher, or Core workflow encoded
by shell pipelines.

Search scope selectors, Pick candidates before an Adapter predicate, raw Anddress
View/Check references, batch Check named outcome bindings, anchored View input,
and Anchor logical-source invalidation are implemented in Session. Later Session
work may accept inert Edit construction, explicit Apply input, and typed Data
names. None of those spellings are a public Core wire or a currently implemented
CLI command.

Machine-oriented JSON and exact text raw output, if defined, are Adapter output
schemas rather than `SearchOutcome`, `PickOutcome`, or Anddress wire authority.
