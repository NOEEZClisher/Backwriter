# Verification

## 0.2.0 authority and future verification boundary

The closed `0.1.0` v3 suite below remains the implementation baseline. The
unpublished `0.2.0` v4 target is documentation-only in Phase 1; no v4 Rust,
Cargo, test, benchmark, or performance result exists yet. Its phase gates,
required test matrix, reproducible drift-Wrong-Apply case, and benchmark
conditions are tracked in
[Backwriter 0.2.0 Anddress fast path](../tasks/2026-08-30-backwriter-0.2.0-anddress-fast-path.md).
That task tracks progress only; the Protocol, address model, and principles own
semantics.

Future v4 verification must prove that Search computes the source hash while
discovering exact `[start, end)` ranges in one source read; View uses hash and
range without target search; Check compares the source hash without target search;
and Apply rejects a mismatched source hash before patching a range. It must also
prove that a changed source invalidates ordinary Anddresses, that no consumer
relocates duplicate text by ordinal or context, that bounded
`CurrentObservation` state retains only current hash, length, and minimum
ranges, and that only Anchor transforms live ranges across Backwriter-owned
Apply.

Phase 1 is docs-only. Its local verification is offline/locked metadata plus
Markdown fence/link, exact guard wording, diff/index, Rust/Cargo/test byte
identity, `.artext`, public `0.1.0`, and service/tunnel invariants. The unchanged
193-test `0.1.0` Rust result may be cited from the immediately preceding stable
closure; the suite is not rerun for this docs-only phase.

Current regressions cover SHA-256 transcript and platform-coordinate KATs,
canonical arbitrary Naturals, strict v3 flat-wire decoding and version priority,
exact extents, Search no-limit traversal, query and scope preflight, canonical
selected ordering, selected-source fail-all, NUL/invalid UTF-8 source handling,
spill boundaries, no-follow symlinks and hard links, View target-specific
currentness and full related v3 addresses, terminator and Unicode reconstruction,
and Pick stable order, multiplicity, target kinds, complete-value OneOf, and
deep iterative boolean composition. Apply regressions cover Edit validation
priority, exact File/Paragraph/Line splice geometry, all line terminators,
Unicode and scratch boundaries, cross-source rejection, no-op publication
avoidance, and late invalid/incomplete/NUL source closure. They also cover
same-parent staging cleanup, failed-publication prospective-after cleanup,
deterministic temporary-name collision preservation, logical-path independence
for hard links, Unix basic-mode preservation across changed publication,
unavailable and no-follow sources, and large whitespace Lines without
unnecessary Paragraph state.
Anchor regressions cover Runtime-local opaque handles, duplicate anchoring,
drop-and-reanchor, foreign handles, stale-input preservation, one-read Apply
preparation, known-invalid-source and transient-read handling, exact
direct-target distinctions, unique post-splice target overlap across separators,
terminator absorption, collision reflection, mismatch fail-closure, and
path-exact explicit invalidation. They also cover containing Paragraph rebinding
to one remaining source Paragraph after line deletion or replacement, and
removal after a replacement splits it into two Paragraphs. They prove that a
Line outside a replaced Paragraph remains current and rebinds after its deletion,
and that terminal self-Copy rebinds a source-member Line to its joined exact
extent without an unavailable result or temporary leak. Check regressions cover source-less validation priority, raw and
native-result filtering, duplicate occurrence order and report counts, canonical
empty results, File/Paragraph/Line currentness, exact terminators, huge
ordinals, UTF-8/NUL observations, spill and admission safety, hard-link path
independence, stateless recovery, and Anchor non-mutation. Data regressions
cover native UTF-8 names, all seven typed Store/Get pairs, duplicate input
return including the exact owned View allocation, borrowed kind/name listing,
all-kind Rename/Remove dispatch, rename and remove priority, all three
CheckOutcome payloads, and no fixed entry or name-length cap.

Exact File Search regressions cover source-less logical-path validation,
empty/nonempty regular sources without content matching, missing and directory
Empty outcomes, named admission and unadmitted paths, private spill, symlink and
hard-link boundaries, invalid UTF-8/NUL closure, one ordinary v3 File result,
Check integration, and empty-File Apply at both `StartOf` and `EndOf`.

Check streaming regressions cover chunk-boundary UTF-8, NUL, CRLF, standalone
CR, and no-EOL handling; late invalid, incomplete, NUL, and read failure;
forward-only access; exact final-byte/length equality; batch order and
multiplicity; unsorted mixed File/Paragraph/Line inputs; duplicate Line and
Paragraph occurrences; same-ordinal differing Line extents; leading and
consecutive separators; huge missing ordinals; and Anchor non-mutation.
Search streaming regressions cover the
same forward scanner boundaries, Line-scoped KMP, complete matching extents,
unmatched Line-buffer capacity reuse, fallible owned copies for matched extents,
and large-unmatched-to-short-match capacity-transfer closure, plus late source
failure discard. Check tracker regressions cover allocation-free canonical
decimal cursor comparison across digit boundaries while preserving unsorted
duplicates and wrong extents. View/Anchor streaming regressions cover the same
forward scanner boundaries, Line construction from caller ExactExtent,
target-only Paragraph candidate buffering, late invalid/read closure,
tracker-only File/Paragraph observations without a View outcome, selected
anchored-binding success beside a stale sibling, and exact-path fail-close on
the selected mismatch. Structural audits keep the forward read loop in
`source_scan.rs`, exclude stale Line-content buffers and the retired scanner
error alias, keep raw Anchor out of `ViewCapture`/`ViewOutcome`, and leave
File-only Check groups tracker-free. Apply streaming regressions cover its
single source forward scan, scanner-sized fixed retained-source batches,
incremental after framer, one rename, and absence of whole-source/after
materialization. They cover 8,191/8,192/8,193 UTF-8 and CR/LF/CRLF/no-EOL
source-to-replacement boundaries, Paragraph-input leading-whitespace
suppression and ending separators, live Paragraph binding beside large leading
whitespace, terminal removal after three prospective targets, late invalid
source after a partial batch without publication or a temporary leak, and a
late injected read failure that drops a partial batch's temporary. They
also cover after-framer feed and physical-Line marker boundaries, including
Line Copy from the tracker-verified caller exact extent and copied terminal-Line
rebinding across retained-source batch boundaries. They distinguish absence of
source-sized complete-Line capture without an eligible live Line candidate from
selected-Paragraph semantic pending, and preserve a
File-only live Anchor through a non-no-op File replacement without a relation
pass, and remove same-path Paragraph/Line Anchors while preserving that File
Anchor without a File-target relation scan. Structural audits
exclude source reopen/retry and source-sized complete before/after materialization;
the private staging entry is the only permitted replay input. Same-kind
Line/Paragraph source-target Anchor relations use tracker-confirmed ordinals
without a staging relation scan; cross-kind relations retain that scan. Collision
marking starts only from prospective Rebind left entries while retaining every
Rebind-to-Rebind pair and multiway disposition.

Edit V1 semantic/public API/type/error authority and inert Rust value
implementation are complete; the single-source Apply Runtime execution
and its regressions are complete.
Value regressions cover every operation and position target-kind boundary,
source-less Anddress error mapping and field priority, exact empty/Unicode/CR/
LF/CRLF content, NUL rejection, and absence of relation or fixed-size
constraints. They use no filesystem or Runtime. Runtime regressions cover every
Edit operation and exact position boundary; same-source rejection, strict Move
interior rejection, late UTF-8/NUL/read failure, explicit staging removal and
removal-failure closure, Empty Insert direct success after File/Line Anchor
currentness, Line/Paragraph Move self and adjacent-boundary direct success,
short-read comparison for Replace and nonidentity Move (File Replace retains its
initial accepted-source scan and required comparison, but skips probe/final source
replay; probe-only Line Move
uses tracker-verified `ExactExtent` without a target-extraction staging pass;
final and Paragraph Move retain Extractor provenance), and direct final replay
for length-changing Insert/Delete/Copy,
horizontal-whitespace Paragraph separation,
bounded whitespace pending only for Paragraph candidates, source currentness,
and prepared live-Anchor reflection for Move and Copy.
They also cover source-target-only replacement provenance, split removal and
unique replacement rebinding, Position-only Insert effects, no-EOL and
terminator Copy behavior, and reverse Move rebinding for source-contained Line
and Paragraph bindings.
Edit still adds no Data
payload, wire form, or distinct anchored executor/publication path.

Run after Rust or Runtime behavior changes:

    cargo metadata --offline --locked --format-version 1
    cargo tree --offline --locked
    cargo fmt --all -- --check
    cargo check --offline --locked --all-targets
    cargo test --offline --locked
    cargo clippy --offline --locked --all-targets -- -D warnings
    cargo build --offline --locked --release

Linux x86_64 release-target verification uses the explicitly installed canonical
`x86_64-unknown-linux-musl` target. The GNU target remains the local development
and test-host target; `rust-toolchain.toml` does not auto-install musl for every
checkout. Run:

    rustup target add x86_64-unknown-linux-musl --toolchain 1.95.0
    rustc +1.95.0 --print cfg --target x86_64-unknown-linux-musl
    cargo check --offline --locked --all-targets --target x86_64-unknown-linux-musl
    cargo build --offline --locked --release --target x86_64-unknown-linux-musl
    cargo test --offline --locked --target x86_64-unknown-linux-musl

The release binary is `target/x86_64-unknown-linux-musl/release/bw`. These commands verify
target selection, build, test, and host execution; running them alone does not
publish a distribution.

Separately, the external operations-owned distribution at
[https://backwriter.pentagration.com](https://backwriter.pentagration.com) has
completed publication verification for Backwriter `0.1.0-beta.3` on
Linux/WSL x86_64, macOS arm64, macOS x86_64, and Windows x86_64. Targets are
`x86_64-unknown-linux-musl`, `aarch64-apple-darwin` at minimum macOS 11.0, and
`x86_64-apple-darwin` at minimum macOS 10.12, and
`x86_64-pc-windows-gnu`. The artifacts and manifest retain
Source Authority revision `7d7469563a357215261c42fa2067d7f587c5eb1b`, and the
POSIX installer destination is `$HOME/.local/bin/bw`; the PowerShell destination
is `$HOME\.local\bin\bw.exe`. The CMD path is the public CRLF Adapter that
downloads and delegates to the same PowerShell installer. Verification covered
the archive, manual
`.sha256` sidecars, expanded canonical manifest, installers, closed version
directory, local and public GET/HEAD status, zero-length HEAD bodies, exact
cache policy, canonical body equality, manifest-authoritative artifact SHA-256,
and task-local fresh installation plus explicit `bw update`. Fresh installation
printed the installed version, replacement printed the updated version, and
destination/PATH guidance remained separate. `bw version` produced exactly
`Backwriter 0.1.0-beta.3` plus LF. Verification passed 188 GNU tests, 188 musl tests, 13
origin tests, 35 POSIX installer regressions, 36 PowerShell regressions, 12 CMD
regressions, 10 Windows release regressions, and 18 publisher regressions. It
also verified 12 local and 12 public GET/HEAD responses with exact bodies and
cache policy. Host verification confirmed enabled and active
`backwriter-origin.service` and official `cloudflared.service` processes with
zero restarts, one `127.0.0.1:8080` listener, byte-identical tracked and
installed ingress YAML, and a root-only Git-external tunnel credential. No
token or credential value is present in Git, unit arguments, service
environment, or service journal. macOS and Windows verification is static and
does not claim native execution. The beta.1 and beta.2 files and complete
beta.3 version directory are immutable. The planned matrix is complete and
beta.3 is closed. Linux arm64, tags, GitHub Releases,
crates.io publication, universal host compatibility, background or automatic
update, and GitHub distribution authority remain outside this verification.

Stable `0.1.0` publication verification regenerated the four artifacts and
sidecars from source revision `25a0dbc38dc78cc7592b219e9070af3c0e201c17`
and reproduced the canonical 876-byte manifest with SHA-256
`551ee8b6fc4c5df83421ba7244f191fee8cc70287775088f08f5e1b8e2290570`.
The tracked publisher installed the stable eight-file version directory,
replaced the POSIX and PowerShell pointers, reused the CMD Adapter, and replaced
the manifest last; a complete rerun reused the resulting exact 20-file tree.
All 20 public GET and HEAD endpoints returned exact bodies, lengths, and cache
policy; root and unknown paths remained 404/no-store. Task-local fresh install
and an actual beta.3 binary's explicit update both installed byte-identical
stable Linux binaries and printed `Installed Backwriter: 0.1.0` and `Updated
Backwriter: 0.1.0`. The published binary passed help, exact version, Search,
Session, and empty-File StartOf/EndOf Apply verification. Stable closure also
passed 193 Backwriter tests, 13 Origin tests, 32 installer regressions, 16
stable-publisher regressions, and 12 CMD regressions. Origin and cloudflared
process identity, restart counts, loopback listener, tunnel connector, ingress
YAML, DNS, credential metadata, and actual user HOME/PATH/shell files remained
unchanged. macOS and Windows verification remains static and makes no native
execution claim.

Before handoff, verify the diff and index, confirm repository-root `.artext` is
absent and untracked, preserve historical task/history files, and leave the
index empty.

The repository source package is `0.1.0`, and its release build must print
exactly `Backwriter 0.1.0` plus LF. Source verification remains distinct from
the separately executed operations publication: the current official
distribution is the closed stable `0.1.0` release, while prior beta files remain
immutable. The `0.1.0` source suite passes 193 GNU-host Rust tests.

CLI process regressions cover the canonical `bw` binary without a `backwriter`
binary, `--help`, exact `bw version`, explicit `bw update` download/exit/output
propagation and platform handoff, default-current-directory and explicit absolute workspaces,
default and repeated admission, Line/Paragraph/File Search, repeated source and
subtree scope selectors, Core scope rejection, deterministic human output,
space-preserving query argv, raw-Anddress/workspace-coordinate omission, Empty,
usage versus Runtime execution exits, unsupported deferred forms, and strict
stdout/stderr separation. They also cover one-shot human/JSON and Session
`search /file`, exact empty-File retrieval, missing/directory Empty, invalid and
unadmitted paths, existing writer reuse, Check, and end-to-end empty-File Apply.
View regressions cover v3 decode, File/Paragraph/Line
exact bytes, None/LF/CR/CRLF terminators, large no-EOL output, stale/wrong-
extent and unadmitted source closure, plus one-shot anchored/extra-operand
rejection.
One-shot Search JSON regressions cover exact compact envelope key order, Empty
and Found mapping, existing v3 object embedding and re-decoding, File/Paragraph/
Line targets, CR/LF/CRLF/no-EOL exact extents, Unicode and JSON escaping, result
order, repeated Line content, global-option placement, rejected duplicate/late
or non-Search JSON, and a structural audit that excludes a JSON Value or cloned
result collection in the production writer.
One-shot View JSON regressions cover exact compact envelope key order for File,
Paragraph, and Line; related v3 File/Paragraph object re-decoding; every Line
terminator including a separator Line's `paragraph:null`; Unicode and JSON
escaping; unchanged human projection; rejected duplicate/late/anchored/extra
forms; unavailable stdout/stderr closure; and a structural audit that excludes
a JSON Value, ViewOutcome clone, or result collection in the production writer.
One-shot Check JSON regressions cover exact compact Current, NotCurrent, and
Unavailable envelopes; direct v3 filtered-value re-decoding; File/Paragraph/Line
inputs; missing-source NotCurrent versus invalid-source Unavailable; unchanged
human statuses; rejected duplicate/late/search/pick/extra/invalid forms; and
structural audits for shared fail-closed status classification, no JSON Value,
no CheckOutcome clone or collection in the JSON writer, and removed display-only
CheckOutcome clones at Session/Data writer callsites.
One-shot raw View regressions cover exact default-human equality for File,
Paragraph, Line, Unicode, every terminator, and large no-EOL output; admitted
global-option order; stale/invalid closure; rejected duplicate/mixed/late flags
and non-View raw forms; and structural absence of a raw writer or the retired
global JSON bool.
Check regressions cover File/Paragraph/Line Current status, stale and missing
NotCurrent status, unavailable-source status, strict v3 decoding, and rejected
search/pick/extra forms. They create no CLI Session, binding, JSON, raw, or
other capability authority. Session regressions cover one retained Runtime,
Search and Pick bindings with exact indexed address projection, direct Search and
Pick non-retention, Core Pick target-kind, same-file, OneOf, and iterative boolean
composition, batch Search/Pick Check report counts, mixed current/NotCurrent/
Unavailable outcomes, empty outcomes, unchanged bindings after batch Check,
copied Search/Anddress bindings, non-aliasing Anchor handle creation,
`AlreadyLive` without a new binding, File/Paragraph/Line anchored View,
source-specific invalidation, rejected handle cloning/indexing/type misuse,
lexer quoting and errors, blank Lines, EOF and exit, error continuation and exit
precedence, and absence of latest, pipeline, registry, or persistence. Session
Data regressions cover all seven typed Core kinds, direct and `let` Get human
projection, duplicate and cross-kind names, Rename/Remove/List order and safe
name escaping, wrong-kind and unsupported-value rejection, mutation failure
preservation, and DataStore drop at Session end.
Session result-binding regressions cover exact View/Check output before storage,
cloneable result values, anchored View, raw and batch Check reports, and rejected
cross-capability use without implicit filtered-value conversion.
Session lexer regressions cover `\\`, `\"`, `\n`, `\r`, and `\t` for non-Edit
commands plus rejected malformed escapes and quotes. Session View/Check
regressions preserve direct, anchored, binding, and Data Get bytes/counts while
displaying through borrowed outcomes. Session Edit/Apply regressions cover all
five Edit variants, all four Position forms, exact source bytes, explicit Edit
binding cloning and repeated Apply reuse, invalid forms, and continued execution
after errors without CLI recovery.
