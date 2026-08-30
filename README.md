# Backwriter

Backwriter is a Rust Core and Runtime for current, structural text work over
admitted Workspace Source. It provides target-local File, Paragraph, and Line
addresses without turning source history or editor state into Core identity.

The Core capability inventory is Search, View, Pick, Anchor, Check, Edit,
Apply, and Data. The repository currently provides their Rust Core/Runtime
surfaces and the canonical `bw` executable's one-shot human and JSON
Search/View/Check, raw View, Session Pick, batch Check, Anchor, Edit, Apply,
result-binding, explicit Data modes, and Adapter-owned Version and Update.

## Quick start

Install the closed official `0.1.0` stable release with the command for your
platform.

Linux, macOS, or WSL:

```sh
curl -fsSL https://backwriter.pentagration.com/install.sh | sh
```

Windows PowerShell:

```powershell
irm https://backwriter.pentagration.com/install.ps1 | iex
```

Windows CMD:

```cmd
curl.exe -fsSL https://backwriter.pentagration.com/install.cmd -o install.cmd && call install.cmd && del install.cmd
```

The POSIX installer places `bw` at `$HOME/.local/bin/bw`; the PowerShell and
CMD paths place `bw.exe` at `$HOME\.local\bin\bw.exe`. They do not change
`PATH`, a shell startup file, the PowerShell profile, or the registry. A fresh
install prints `Installed Backwriter: <version>`; replacing an existing
destination prints `Updated Backwriter: <version>`. The executable path and
`PATH` guidance are printed separately only when the installation directory is
not already on `PATH`.

### Version

```sh
bw version
```

The current source build prints exactly:

```text
Backwriter 0.2.0
```

### Update

```sh
bw update
```

`bw update` downloads and delegates to the current official installer. The
current manifest selects the closed `0.1.0` stable distribution. The installer
reads that manifest, verifies the selected artifact, and installs or reinstalls
that manifest version only after validation succeeds. It does not run a
background updater or compare release versions. The transition installer also
retains exact acceptance of the immutable beta.3 manifest; that compatibility
does not change the current stable pointer.

The product is Backwriter. The unpublished Cargo package and library crate are
`backwriter` at `0.2.0`; the sole canonical executable and external Adapter
command are `bw`. There is no `backwriter` binary, alias, or wrapper. The
official installer remains separate and continues to select the closed public
`0.1.0` distribution.

The default workspace is the process current working directory. An explicit
`--workspace` must be absolute and is checked by Runtime. Search admits `.` by
default; repeat `--admit LOGICAL_PATH` before `search` to narrow admission.
After the query, repeat `--source LOGICAL_PATH` or `--subtree LOGICAL_PATH` to
narrow a Search scope. Without a scope selector, Search covers all admitted
sources.

## Current CLI scope

`bw` currently implements Adapter-owned one-shot Version and Update, one-shot
human or JSON Search, View, and Check, raw View, plus Session Pick, batch Check,
Anchor, Edit, Apply, and Data:

```text
bw version
bw update
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
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    view anddress <encoded-v4-Anddress>
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    check anddress <encoded-v4-Anddress>
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell
```

Version and Update do not call Backwriter Core or Runtime and create no Core
wire or capability workflow.

Content Search preserves Core literal validation, scope, projection, and
deterministic result order. The distinct `search /file` form validates one
logical path and returns the current File Anddress for an admitted regular
UTF-8, NUL-free source regardless of whether it is empty or contains matching
text. Missing paths and directories return Empty; the form has no scope
selectors or synthetic content query. View decodes a v4 Anddress and writes only
its exact selected text. Check decodes one v4 Anddress and writes only
`Current`, `NotCurrent`, or `Unavailable`. Search, View, and Check `--json`
write one compact Adapter object with exact embedded v4 Anddress objects where
applicable; each is an Adapter schema, not a second Core wire.
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

## Build from source and test

```sh
cargo build --offline --locked --release
./target/release/bw search line "needle"
./target/release/bw --workspace /path/project search paragraph "needle"
cargo test --offline --locked
```

## Official desktop distribution

The official distribution authority is
[https://backwriter.pentagration.com](https://backwriter.pentagration.com).
It publishes the closed Backwriter `0.1.0` stable release for Linux/WSL x86_64,
macOS arm64, macOS x86_64, and Windows x86_64 from Source Authority revision
`25a0dbc38dc78cc7592b219e9070af3c0e201c17`. Linux uses canonical target
`x86_64-unknown-linux-musl`;
`x86_64-unknown-linux-gnu` remains the local development/test-host target.
macOS uses `aarch64-apple-darwin` with minimum macOS 11.0 and
`x86_64-apple-darwin` with minimum macOS 10.12. The macOS artifacts receive
static cross-build verification but are not claimed to have been executed on a
native Mac before publication. Windows uses `x86_64-pc-windows-gnu` and the
canonical executable `bw.exe`; its static cross-build verification does not
claim native Windows, PowerShell, or CMD execution. Linux arm64 is not currently
provided, and no universal host-compatibility claim is made.

`install.sh` reads the canonical manifest, verifies the downloaded artifact
against the manifest SHA-256, and installs the verified binary at
`$HOME/.local/bin/bw` with a same-directory rename. Concurrent same-user
HOME mutation is caller-owned. The published `.sha256` sidecar is for manual
verification and is not installer authority. Windows PowerShell verifies the
same manifest authority and exact ZIP, and installs to
`$HOME\.local\bin\bw.exe` without editing PATH or the PowerShell profile. The
three canonical install commands and the canonical `bw update` command are kept
together in [Quick start](#quick-start).

The CMD command writes `install.cmd` in the current directory and removes it
after a successful installation. An existing file with that name is replaced.

The CRLF `install.cmd` checks `curl.exe` and `powershell.exe`, downloads exactly
the canonical `install.ps1` over HTTPS-only TLS 1.2-or-newer transport into a
collision-failing `%TEMP%` task directory, delegates all installation meaning,
cleans the directory, and preserves the child exit code. It owns no manifest,
SHA-256, ZIP, or installation logic. The Linux-hosted CMD regression is static;
no native CMD execution is claimed.
The distribution provides no
publisher-authenticity signature or trusted signing identity, background or
automatic update, telemetry, `sudo` execution, or automatic `PATH` or
shell-startup-file change.
GitHub is a public source and documentation mirror, not the distribution
authority. The complete beta.1, beta.2, and beta.3 version directories remain
unchanged and immutable. The complete stable `0.1.0` version directory is also
immutable, its planned matrix is complete, and the stable release is closed.
Any later platform or version requires separate Owner authority. Linux arm64,
tags, GitHub Releases, crates.io publication, and background or automatic
update remain outside the completed publication. Stable publication added the
`0.1.0` directory and current pointers without replacing or reopening any
beta.3 file.

## Architecture

- [Current state](docs/current/now.md)
- [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)
- [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)
- [CLI V1 authority](docs/architecture/backwriter-cli-v1.md)
- [Verification](docs/development/verification.md)

## License

[MIT License](LICENSE)
