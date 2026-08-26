# Backwriter

Backwriter is a Rust Core and Runtime for current, structural text work over
admitted Workspace Source. It provides target-local File, Paragraph, and Line
addresses without turning source history or editor state into Core identity.

The Core capability inventory is Search, View, Pick, Anchor, Check, Edit,
Apply, and Data. The repository currently provides their Rust Core/Runtime
surfaces and the canonical `bw` executable's one-shot human and JSON
Search/View/Check, raw View, Session Pick, batch Check, Anchor, Edit, Apply,
result-binding, explicit Data modes.

## Quick start

Build the current beta from source:

```sh
cargo build --release
./target/release/bw search line "needle"
./target/release/bw --workspace /path/project search paragraph "needle"
```

The product is Backwriter. The Cargo package and library crate are `backwriter`
at `0.1.0-beta.2`; the sole canonical executable and external Adapter command
are `bw`. There is no `backwriter` binary, alias, or wrapper.

The default workspace is the process current working directory. An explicit
`--workspace` must be absolute and is checked by Runtime. Search admits `.` by
default; repeat `--admit LOGICAL_PATH` before `search` to narrow admission.
After the query, repeat `--source LOGICAL_PATH` or `--subtree LOGICAL_PATH` to
narrow a Search scope. Without a scope selector, Search covers all admitted
sources.

## Current CLI scope

`bw` currently implements one-shot human or JSON Search, View, and
Check, raw View, plus Session Pick, batch Check, Anchor, Edit, Apply, and Data:

```text
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    search <line|paragraph|file> <query>
    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json
    search <line|paragraph|file> <query>
    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json
    view anddress <encoded-v3-Anddress>
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json
    check anddress <encoded-v3-Anddress>
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --raw
    view anddress <encoded-v3-Anddress>
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    view anddress <encoded-v3-Anddress>
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    check anddress <encoded-v3-Anddress>
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell
```

The CLI preserves Core Search validation and deterministic result order. View
decodes a v3 Anddress and writes only its exact selected text. Check decodes one
v3 Anddress and writes only `Current`, `NotCurrent`, or `Unavailable`. Search,
View, and Check `--json` write one compact Adapter object with exact embedded v3
Anddress objects where applicable; each is an Adapter schema, not a Core wire.
Raw View is an explicit Adapter exact-text mode that reuses the ordinary View
projection without a Core wire or changed View meaning.
Human output does not expose address metadata.
The Session holds one Runtime until EOF
or `exit` and has explicit local Search, Pick, Anddress, Edit, View, and Check
bindings plus non-aliasing Anchedress handles. It owns one explicit `DataStore`
for the Session only; names are typed and never persist past EOF or `exit`.
Session Pick
passes a named Search or Pick collection and an Adapter-parsed predicate directly
to Core; Session batch Check passes a named matching outcome directly to its
Runtime batch seam and prints only report counts. Session Anchor creates a live
handle only through `let <name> = anchor create <anddress-ref>`, views it through
`view anchored @<name>`, and can invalidate its logical source with `anchor
invalidate-source <logical-path>`. One-shot Data and Anchor are intentionally
unsupported because their DataStore and live-handle contracts require Session
lifetime. One-shot Pick, batch Check, Edit, and Apply await collection or Edit
transport schema authority. Raw output other than one-shot View and further
Session behavior remain deferred.

## Scope

Backwriter is not Git, a file watcher, daemon, persistent index, or editor UI.
It reads admitted current Workspace Source through Runtime's safe no-follow
access. It does not model branches, merges, history, automatic re-evaluation,
or editor buffers.

Apply uses its accepted current observation and does not coordinate concurrent
writers. Writers may race and one publication may overwrite another
source-visible change; hosts requiring a stronger guarantee coordinate outside
Backwriter.

## Build and test

```sh
cargo build --offline --locked --release
cargo test --offline --locked
```

## Official desktop distribution

The official distribution authority is
[https://backwriter.pentagration.com](https://backwriter.pentagration.com).
It publishes Backwriter `0.1.0-beta.2` for Linux/WSL x86_64, macOS arm64,
macOS x86_64, and Windows x86_64 from Source Authority revision
`209f606db08415ef5fd7f1cfbe1e43bf0c96dc73`. Linux uses canonical target
`x86_64-unknown-linux-musl`;
`x86_64-unknown-linux-gnu` remains the local development/test-host target.
macOS uses `aarch64-apple-darwin` with minimum macOS 11.0 and
`x86_64-apple-darwin` with minimum macOS 10.12. The macOS artifacts receive
static cross-build verification but are not claimed to have been executed on a
native Mac before publication. Windows uses `x86_64-pc-windows-gnu` and the
canonical executable `bw.exe`; its static cross-build verification does not
claim native Windows or PowerShell execution. Linux arm64 is not currently
provided, and no universal host-compatibility claim is made.

`install.sh` reads the canonical manifest, verifies the downloaded artifact
against the manifest SHA-256, and installs the verified binary at
`$HOME/.local/bin/bw` with a same-directory rename. Concurrent same-user
HOME mutation is caller-owned. The published `.sha256` sidecar is for manual
verification and is not installer authority. Windows PowerShell installation uses
`irm https://backwriter.pentagration.com/install.ps1 | iex`, verifies the same
manifest authority and exact ZIP, and installs to `$HOME\.local\bin\bw.exe`
without editing PATH or the PowerShell profile. Windows CMD is unsupported.
The distribution provides no
publisher-authenticity signature or trusted signing identity, automatic update,
telemetry, `sudo` execution, or automatic `PATH` or shell-startup-file change.
GitHub is a public source and documentation mirror, not the distribution
authority. The prior beta.1 files remain unchanged and immutable. Every
published beta.2 artifact and sidecar is also immutable, while the beta.2
version directory remains append-only and the release stays open for Windows
CMD support. Linux arm64, tags, GitHub Releases,
crates.io publication, and auto-update remain outside the completed publication.

## Architecture

- [Current state](docs/current/now.md)
- [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)
- [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)
- [CLI V1 authority](docs/architecture/backwriter-cli-v1.md)
- [Verification](docs/development/verification.md)

## License

[MIT License](LICENSE)
