# Backwriter CLI V1

Status: Adapter authority. The completed initial slices are the canonical
`backwriter` executable's one-shot human Search and View modes only. This
document follows the Core active documents in the authority-reading order.

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
- Session would retain one `WorkspaceRuntime`, live Anchor handles, Core
  `DataStore`, and explicit CLI-local named values.

Only one-shot Search and View are implemented. Session, `shell`, named
bindings, all other capabilities, JSON, and raw output are deferred and
rejected rather than silently accepted.

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

Unsupported `shell`, capabilities, `--json`, and `--raw` are explicit usage
errors in this slice. There are no short option aliases.

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
errors in this slice.

Human View output contains only the selected target's exact text: File and
Paragraph write their text unchanged, and Line writes its content followed by
its exact None/LF/CR/CRLF terminator. It adds no header, automatic newline,
preview, truncation, raw Anddress, or related File/Paragraph address.

## Deferred CLI V1 authority

The following are intentionally outside the completed initial slice:

- Session command grammar and shell lexical grammar.
- CLI-local binding ownership, lifetime, and explicit result projection.
- Anchor handle binding, including `AlreadyLive` binding behavior.
- One-shot Pick, Check, Anchor, Edit, Apply, and Data commands.
- JSON schema and the exact scope of raw output.

These require owner decisions before implementation. The high-level intended
forms remain one-shot capability execution and a Session that can explicitly
name Search results, View an Anddress or anchored handle, Check an explicit
input, build an inert Edit, Apply an explicit Edit, and expose Core Data without
equating CLI-local names with `DataStore`.

An intended Session must not introduce implicit `latest`, hidden current
selection, automatic Data storage, automatic Search-to-Pick/View/Edit/Apply
handoff, automatic Anchor creation, a persistent daemon, watcher, or a Core
workflow encoded by shell pipelines.

The intended future Search forms are `search <kind> <query> [scope]`, using
`--source` and `--subtree` selectors. Future View accepts an input form such as
an Anddress or anchored handle; Pick receives candidates before a predicate;
Check receives an explicit input form; Anchor creates or invalidates a
logical-source association; Edit constructs values; Apply consumes an explicit
Edit; and Data operates on explicit typed names. None of those spellings are a
public Core wire or a currently implemented CLI command.

Machine-oriented JSON and exact text raw output, if defined, are Adapter output
schemas rather than `SearchOutcome`, `PickOutcome`, or Anddress wire authority.
