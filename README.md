# Backwriter

Backwriter is a Rust Core and Runtime for current, structural text work over
admitted Workspace Source. It provides target-local File, Paragraph, and Line
addresses without turning source history or editor state into Core identity.

The Core capability inventory is Search, View, Pick, Anchor, Check, Edit,
Apply, and Data. The repository currently provides their Rust Core/Runtime
surfaces and the canonical `bw` executable's one-shot human and JSON
Search/View/Check/Edit, raw View, Anddress-first one-shot Edit, Session Pick, batch
Check, Anchor, Edit, Apply, result-binding, explicit Data modes, and
Adapter-owned Version and Update.

## Quick start

Install the closed official `0.3.0` release with the command for your
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

The closed official `0.3.0` build prints exactly:

```text
Backwriter 0.3.0
```

### Update

```sh
bw update
```

`bw update` downloads and delegates to the current official installer. The
current manifest selects the closed `0.3.0` distribution. The installer
reads that manifest, verifies the selected artifact, and installs or reinstalls
that manifest version only after validation succeeds. It does not run a
background updater or compare release versions. The installer accepts only the
exact previous `0.2.6` manifest and current `0.3.0` manifest; `0.2.5`, `0.2.4`, `0.2.3`, `0.2.2`, `0.2.1`, `0.2.0`, stable
`0.1.0`, and beta.3 acceptance is retired. The command still performs no
version comparison and adds no version guard, retry or rollback.

The product is Backwriter. The source Cargo package and library crate are
`backwriter` at published and closed `0.3.0`; a source-built `bw version`
prints `Backwriter 0.3.0`. The sole canonical executable and external Adapter
command are `bw`. There is no `backwriter` binary, alias, or wrapper. The
official installer selects the closed public `0.3.0`
distribution.

The published `0.2.6` Source Authority's `src/**` is byte-identical to its Gate 6
candidate `c78e07f242035230e8b071d583491ac633f58d29`; that claim does not cover
this later checkout. Published `0.3.0` Gates 1–4 add the private namespace
boundary, complete direct shell View, discoverable Help and private CLI modules.
Cargo and source-built `bw version` now report `0.3.0`. Gate 5R readiness is **GO**:
four new independently captured evaluations preserve exact final bytes, and
both candidate arms obtain complete Paragraph Content without recovery calls.
The identical-input native 293/293 tests and 52 smokes are reused, not rerun.
The prior incomplete evaluation and its optional extra File View remain in the
[evaluation record](docs/tasks/2026-09-05-backwriter-0.3.0-independent-namespace-complete-view.md#gate-5r-result--go).
The subsequent source-version decision and separately authorized `0.3.0`
artifact, installer and manifest-last publication closure are complete.
Deployment Source Authority remains
`237e468993372d1bb079cbaeebd36feea6aa27ea`, not a later documentation commit.
Update now installs or reinstalls official `0.3.0` without comparing versions.
Use `bw help shell` for short-reference Search/View/Replace/Check and
`bw help pick`, `bw help anchor`, `bw help apply`, or `bw help data` for advanced
raw Session topics. These Help topics add no one-shot capability execution.
The prior R3 release closure used Source Authority
`09bb6c424081594bd86a95f04345b786ef9b46b6` for published and closed
`0.2.6`. At that closure, artifacts, installers, manifest, and Update selected
`0.2.6` in the exact 76-file tree. Its installers accepted exact `0.2.5` and
`0.2.6` manifests. Update still performs no version comparison. Production
Rust, Cargo, tests, toolchain, and v5 wire are unchanged by release closure.

## Anddress-first editing

The default replacement flow is:

1. Run `bw --json search ...`.
2. Select one occurrence from `occurrences` and pass its exact embedded v5
   `anddress` object unchanged as one argv value.
3. Run `bw edit anddress '<opaque-v5-object>' '<new-content>'`.

Human Search rows are not encoded Anddress values and cannot be Edit input.
Treat the selected JSON object as opaque: do not interpret or rewrite its hash,
range, length, or other fields. File and Paragraph Content is the exact
replacement. Line Content is body-only, rejects NUL, CR, and LF, and preserves
the None, LF, CR, or CRLF terminator carried by the exact v5 Line. A Line NUL
is `edit.content_contains_nul`; a Line CR or LF is
`edit.line_body_contains_terminator`. The latter explains that Backwriter adds
the current terminator and that advanced raw Session Edit/Apply owns exact
extent replacement. Apply alone confirms that source state before publication.
View or Pick may help a caller select a target; neither View nor Check is
required.

Human success writes one exact LF-terminated receipt row. `Unchanged` is
followed by the still-current input v5 object; `Changed` is followed by the
fresh v5 object when the resulting File, Line, or unique Paragraph has one,
and otherwise by `None`. With leading `--json`, the same result is the compact
Adapter-only `bw.cli.edit.v1` object with `schema`, `outcome`, and `anddress`
keys in that order; `anddress` is the exact v5 object or JSON `null`. Reuse only
the address returned by the receipt. A changed Paragraph with `None` requires
an explicit Search before later target work. Exit `1` is neither a stale-only
classification nor proof that source bytes are unchanged, so it must not
trigger automatic retry.

One-shot Content is either one UTF-8 argv value or the exclusive `--stdin`
selector in that position; stdin is read to EOF after v5 address validation and
before Runtime access. Both forms have the same target-specific Content rules.
File/Paragraph CR and LF are exact Content, while Line body Content never
silently strips a trailing newline. Literal `--json` and `--raw` remain exact
Content; a literal `--stdin` Content value is supplied through standard input.

Raw Session is the advanced composition surface for Insert/Delete/Move/Copy,
Position, Anchor/Data lifetime, explicit bindings, and separate Apply. It is
not a prerequisite or alias for ordinary Replace. Its existing raw
`edit replace` accepts caller-provided exact range Content, including an
explicit terminator or multiline replacement, and `apply @edit` publishes it
separately. The caller owns the binding, index, quoting, terminator, and
publication boundary. General replacement should use one-shot body Content
first; no exact one-shot flag or alternate executor exists.

### Shell-local references

`bw shell` also provides a short-lived interactive flow without adding a Core
wire or persistent identity:

```text
search line needle
view @0
replace @1 replacement
check @2 @3
exit
```

Quote one direct-shell query or replacement argument when it contains
whitespace; the quotes are shell syntax and are not part of the exact literal
or replacement Content:

```text
search line "duplicate = one"
replace @1 "duplicate = two"
```

Successful direct `search` and projected direct `view` append `@N` references
in output order, including duplicates. Direct `check <REF>...` resolves every
reference before Runtime access, writes one Current/NotCurrent/Unavailable
state per input, and appends a fresh slot only for Current. `replace @N <content>` uses the same
target-aware Content rules as one-shot Edit and appends a fresh reference for
`Unchanged` or `Changed` when the receipt has an Anddress; `Changed\tNone`
adds none. Slots are zero-based canonical unsigned decimals, append-only, and
discarded at `exit` or EOF. `@name` and `@name[index]` remain the advanced raw
Session forms; `let name = @N` explicitly clones a numeric slot into that
existing named Anddress binding. Raw `edit replace` and separate `apply @edit`
remain the advanced exact-range path.

The default workspace is the process current working directory. An explicit
`--workspace` must be absolute and is checked by Runtime. Search admits `.` by
default; repeat `--admit LOGICAL_PATH` before `search` to narrow admission.
After the query, repeat `--source LOGICAL_PATH` or `--subtree LOGICAL_PATH` to
narrow a Search scope. Without a scope selector, Search covers all admitted
sources.

## Current CLI scope

`bw` currently implements Adapter-owned one-shot Version and Update, one-shot
human or JSON Search, View, Check, and Edit, raw View, Anddress-first one-shot Edit,
plus Session Pick, batch Check, Anchor, Edit, Apply, and Data:

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
    view anddress <encoded-v5-Anddress>... [--as <line|paragraph|file>]
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json
    check anddress <encoded-v5-Anddress>...
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --raw
    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    check anddress <encoded-v5-Anddress>
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    edit anddress <encoded-v5-Anddress> <content>
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...
    edit anddress <encoded-v5-Anddress> --stdin
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json
    edit anddress <encoded-v5-Anddress> <content>
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json
    edit anddress <encoded-v5-Anddress> --stdin
bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell
```

Version and Update do not call Backwriter Core or Runtime and create no Core
wire or capability workflow.

Content Search preserves Core literal validation, scope, projection, and
deterministic result order. The distinct `search /file` form validates one
logical path and returns the current File Anddress for an admitted regular
UTF-8, NUL-free source regardless of whether it is empty or contains matching
text. Missing paths and directories return Empty; the form has no scope
selectors or synthetic content query. View decodes v5 Anddresses and projects
each to itself or one ancestor before reading its exact content. A single input
defaults to self projection. `--as` chooses one target kind; multiple inputs
require both `--json` and `--as`. Check decodes every v5 operand before Runtime
access. One human input writes `Current`, `NotCurrent`, or `Unavailable`; a
batch requires `--json` and preserves one outcome per input. Search, View,
Check, and Edit `--json`
write compact Adapter objects with exact embedded v5 Anddress objects where
applicable; each is an Adapter schema, not a second Core wire. View uses the
hard-cut `bw.cli.view.v2` outcomes array for both single and batch results;
Check uses the hard-cut `bw.cli.check.v2` ordered outcomes array.
Raw View is an explicit Adapter exact-text mode that reuses the ordinary View
projection without a Core wire or changed View meaning.
Human Search, View, and Check keep their existing projections; human Edit
receipts intentionally return the exact current v5 object when one exists.
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
lifetime. One-shot Pick, raw Edit-object transport, and a separate
Apply transport await collection or Edit transport schema authority. The
distinct Anddress-first one-shot Edit above is implemented. Raw output other
than one-shot View and further Session behavior remain deferred.

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
It publishes the closed Backwriter `0.3.0` release for Linux/WSL x86_64,
macOS arm64, macOS x86_64, and Windows x86_64 from Source Authority revision
`237e468993372d1bb079cbaeebd36feea6aa27ea`. Linux uses canonical target
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
authority. The complete beta.1, beta.2, beta.3, stable `0.1.0`, `0.2.0`, and
`0.2.1` and `0.2.2` version directories remain unchanged and immutable. The
complete `0.2.3`, `0.2.4`, `0.2.5`, `0.2.6`, and `0.3.0` version directories are immutable;
the `0.3.0` release is closed. The active public tree has exactly 84 files
and 12 directories including root. Its 80 versioned files are immutable-cacheable;
the three installers and manifest are no-store. The current manifest selects
`0.3.0`: 876 bytes, SHA-256
`4e95f3f810cc610fcdacf787bf0c5210cb00ccfe066298fa43bdb3d1fe09ffde`.
Any later platform or
version requires separate Owner authority. Linux arm64, tags, GitHub Releases,
crates.io publication, and background or automatic update remain outside the
completed publication. The earlier `0.2.4` publication added its eight
versioned files and replaced only the two installers and manifest pointers
without replacing any of the 48 prior versioned files or the CMD Adapter. Its
second publisher run reused all 60 files without metadata change. The `0.2.5`
publication then added its eight versioned files, replaced the two installers
and manifest pointers, and reused all 68 files on its second run. R3 added the
eight `0.2.6` files and replaced the two installers and manifest last; its
second run reused all 76 files without byte or metadata change. The `0.3.0`
publication then added eight versioned files, replaced POSIX, PowerShell and
the manifest last, and reused all 84 files unchanged on its second run.
Its 73 preserved files are 72 prior versioned files plus CMD; CMD preservation
does not make it immutable-cacheable.

## Architecture

- [Current state](docs/current/now.md)
- [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)
- [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)
- [CLI V1 authority](docs/architecture/backwriter-cli-v1.md)
- [Verification](docs/development/verification.md)

## License

[MIT License](LICENSE)
