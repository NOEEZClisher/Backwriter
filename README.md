# Backwriter

Backwriter is a Rust Core and Runtime for current, structural text work over
admitted Workspace Source. It provides target-local File, Paragraph, and Line
addresses without turning source history or editor state into Core identity.

The Core capability inventory is Search, View, Pick, Anchor, Check, Edit,
Apply, and Data. The repository currently provides their Rust Core/Runtime
surfaces and the canonical `backwriter` executable's one-shot human Search,
View, Check, Session Pick, batch Check, Anchor, Edit, and Apply modes.

## Quick start

```arduino
cargo build --release
backwriter search line "needle"
backwriter --workspace /path/project search paragraph "needle"
```

The default workspace is the process current working directory. An explicit
`--workspace` must be absolute and is checked by Runtime. Search admits `.` by
default; repeat `--admit LOGICAL_PATH` before `search` to narrow admission.
After the query, repeat `--source LOGICAL_PATH` or `--subtree LOGICAL_PATH` to
narrow a Search scope. Without a scope selector, Search covers all admitted
sources.

## Current CLI scope

`backwriter` currently implements one-shot human Search, View, Check, and
Session Pick, batch Check, Anchor, Edit, and Apply:

```text
backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    search <line|paragraph|file> <query>
    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...
backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    view anddress <encoded-v3-Anddress>
backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    check anddress <encoded-v3-Anddress>
backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell
```

The CLI preserves Core Search validation and deterministic result order. View
decodes a v3 Anddress and writes only its exact selected text. Check decodes one
v3 Anddress and writes only `Current`, `NotCurrent`, or `Unavailable`. Human
output does not expose address metadata. The Session holds one Runtime until EOF
or `exit` and has explicit local Search, Pick, Anddress, and non-aliasing
Anchedress bindings. It has no latest value or DataStore integration. Session Pick
passes a named Search or Pick collection and an Adapter-parsed predicate directly
to Core; Session batch Check passes a named matching outcome directly to its
Runtime batch seam and prints only report counts. Session Anchor creates a live
handle only through `let <name> = anchor create <anddress-ref>`, views it through
`view anchored @<name>`, and can invalidate its logical source with `anchor
invalidate-source <logical-path>`. One-shot Pick, one-shot batch Check, Anchor,
Edit, Apply, Data, JSON, raw output, and further Session behavior remain deferred.

## Scope

Backwriter is not Git, a file watcher, daemon, persistent index, or editor UI.
It reads admitted current Workspace Source through Runtime's safe no-follow
access. It does not model branches, merges, history, automatic re-evaluation,
or editor buffers.

## Build and test

```sh
cargo build --offline --locked --release
cargo test --offline --locked
```

## Architecture

- [Current state](docs/current/now.md)
- [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)
- [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)
- [CLI V1 authority](docs/architecture/backwriter-cli-v1.md)
- [Verification](docs/development/verification.md)

## License

[MIT License](LICENSE)
