# Backwriter 0.3.0 independent namespace and complete shell View

## Source-version decision — source-ready unpublished 0.3.0

Recorded 2026-09-05. Entry was clean `main = origin/main =
bde689b22fbe6f5ebc06247b094730f5066b05af`. The Owner selects source-ready
unpublished `0.3.0` on Gate 5R GO. This decision is complete; artifact,
installer, publisher and publication closure is not. Official `0.2.6`
Source Authority remains `09bb6c424081594bd86a95f04345b786ef9b46b6`.
Source-built Update can install official `0.2.6` without version comparison;
neither installed `bw` nor Update was executed.

### Minimal change and evidence boundary

The existing `write_version` consumes `env!("CARGO_PKG_VERSION")`; no executor,
writer, helper, API, schema, compatibility path or production code is changed.
Only Cargo.toml's root version, Cargo.lock's `backwriter` entry and two
`Backwriter 0.3.0\n` expectations in `tests/cli/help.rs` change outside docs.
The Version KATs remain independent literals. Dependencies, features, profile,
toolchain, flags, other tests/fixtures and every production byte are unchanged
from entry. Cargo/tests are explicitly not wholly byte-identical.

Gate 5 GNU/musl **293/293 tests**, **52 smokes**, and Gate 5R's four arms are
reused under verification's version/build-metadata exception. Complete input
comparison permits only these four version substitutions. The previous NO-GO,
nine-command record, all 21 archived files and complete Gate 5R section below
are preserved byte-for-byte. T-source/T-roadmap and history remain unchanged.
No full suite, AI/capture, N-1 comparison, benchmark or performance claim is
repeated. Delivered text-content excludes unobservable platform envelopes;
the historical n=1 times remain observations, not performance guarantees.

### Fresh verification

Rust `1.95.0 (59807616e 2026-04-14)`, Cargo
`1.95.0 (f2d3ce0bd 2026-03-21)`, LLVM `22.1.2`; existing GNU and musl targets,
default features, unchanged release profile (`panic=abort`, strip, LTO,
codegen-units=1), no added Rust/build flags. Task-local target output only.
Offline/locked full metadata and dependency tree equal the entry versions
after normalizing only the root version/ID. Fmt and each target's all-target
check, clippy `-D warnings` and release build pass.

| Newly executed evidence | GNU | musl |
| --- | --- | --- |
| `canonical_binary_help_and_default_workspace_search` | 1 passed | 1 passed |
| `command_local_help_kats_are_exact_and_skip_runtime_opening` | 1 passed | 1 passed |
| Release smoke processes | 7 passed | 7 passed |
| Release `bw` size | 864848 bytes | 959088 bytes |
| Release `bw` SHA-256 | `ed0affd068ef017bccc3047ac93d705150ee74fd34d70dffc22be4f88b113732` | `d1916ee22a4d3ea8bfe684ed317456fd966a52520082f2830a33e92d82e190b0` |

These are **4 new test executions and 14 new smoke processes**, not newly run
293/293 suites or 52 smokes. Additionally, **42 release command-local Help/usage
processes** pass: seven commands, direct/named Help plus trailing-operand
rejection, on each target. Independent Help KAT bytes match; usage exits 2
with empty stdout and nonempty stderr. Update Help never executes Update.
Both Version runs exit 0, stdout exact
`Backwriter 0.3.0\n`, stderr empty. `--help` and `help` equal the existing
independent top-level KAT. Command-local Help/usage and private sentinel
noncreation are covered by the two KATs on each target. No `.bw` or `.artext`
appears in the isolated release fixtures.

Each target's seven processes are Version, `--help`, `help`, JSON Search,
direct shell, stale one-shot Edit and raw Session shell. On exact
`retry_budget = 3\r\n`, JSON Search v2 returns one occurrence containing v5.
Direct shell executes:

```text
search line "retry_budget = 3"
view @0 @0 --as paragraph
replace @0 "retry_budget = 5"
check @3
view @3
exit
```

Exit 0 and empty stderr; two Paragraph records have `bytes=18`, fresh refs
`@1/@2` and exact old CRLF Content; Replace returns `@3\tChanged`, Check
returns `@4\tCurrent`, and fresh View returns `@5`, `bytes=18` and exact
`retry_budget = 5\r\n`. Each record uses existing `View`/`EndView` framing
without trimming. Reusing the original JSON occurrence's address in one-shot
Edit exits 1 with unavailable on stderr, empty stdout and unchanged bytes.
An independent reset to `one\r\n` followed by raw Search binding, exact
`edit replace @lines[0] "two\\r\\n"`, then separate `apply @edit`, exits 0
with exact stdout `Found 1\n0\tLine\tnote.txt:1\nOK\n`, empty stderr and
final `two\r\n`. No live source, HOME, PATH or installation is involved.

The task-local assertion driver SHA-256 is
`46a933cd2af8b23e6cefd2716d11bb871c6f7aa209e426c271eedb6bfff8a967`;
the exact argv/stdin/exit/stdout/stderr JSON record SHA-256 is
`fc18bba8e6e098218883fae756bed01a6cfbb120b78ed6058ce209167db3f3c5`.
An initial sandbox child-spawn denial required the approved execution path;
the verification-only extractor was corrected to existing v2 `occurrences`,
not a production schema change. Both complete final runs passed. New binary
identities are recorded above, never asserted equal to the `0.2.6` binaries.

Native macOS/Windows/PowerShell/CMD and release reconstruction/installation/
publication are not verified here. No server, public root, service, tunnel,
DNS, credential or actual HOME change is authorized by this source decision.
The source commit is reported after commit rather than inventing a self-SHA.

Final audit passes 13 allowed changed paths, 63 local Markdown links and six
anchors, balanced fences, no conflict markers, empty pre-stage index and no
untracked/tracked output or `.bw`/`.artext`. The complete prior tracker suffix
is byte-identical. The exact task-local root `/tmp/backwriter-source030.iiIisc`
is removed after verification; existing repository targets and user data are
preserved. Recorded identities and assertions above remain the evidence;
temporary binaries, logs and drivers are not release artifacts.

## Gate 5R result — GO

Recorded 2026-09-05. Gates 1–5 are complete for the evaluated source. D3's
missing capture is closed within the predeclared text-content scope below.
All four new independent runs are retained, including both N-1 shell-help
errors. Both N arms satisfy BOX 26: the first Paragraph View supplies complete
decision-making Content, missing-Content self/raw View and named-copy recovery
are zero, final bytes match, newline mistakes and Wrong Apply are zero.
BOX 25's eight-command reference is descriptive, not an automatic rejection
threshold for optional verification. No product, API, wire, tests, build input
or version changes occur. Cargo/CLI/public remain closed `0.2.6`; the next
decision is version selection, not an authorized artifact or release.

The old Gate 5 NO-GO, nine-command Genie N and four original raw transcripts
remain byte-identical below. That ninth pre-edit File View follows complete
Paragraph Content and precedes replacement, so it is optional preservation
verification, not Content recovery. New runs do not erase or repair the old
missing stream/time/visibility records.

### Documentation and cleanup closure

After source/record archival, verification reconstructed all 21 archived files,
all raw events, 60 evaluated responses totaling 163,311 delivered text bytes,
and the sixteen exact final files. The same reconstruction passes after removal
of the task root: it depends on the tracker, not surviving temporary files.
The three preflight source files remain in the preflight checkpoint.
Final Markdown audit passes 10 files, 62 local file
links, six local anchors, balanced fences and zero conflict markers.

Both recorder sessions and all evaluated product processes are terminated.
The exact owner-owned, non-symlink, non-mounted task root
`/tmp/backwriter-gate5r.kUMPDh` was removed after verifying archival. This
removed only task-local exports, GNU target output, binary copy, fixtures,
recordings, socket and helpers. Code/records remain reconstructable from this
tracker and pinned Git objects; the pre-existing repository target was not
removed. No backup, new service, install or deployment was created.

The index was empty before staging; only the ten authorized Markdown paths
are changed. Product/test/build inputs, planning notes and the complete old
tracker evidence suffix remain byte-identical to their stated baselines.
`.bw` and `.artext` are absent/untracked, with no tracked build output or
task-local residue. Diff checks pass. A temporary approval-scope rejection
interrupted documentation work after evaluation; the Owner reaffirmed the ten
document paths. That pause did not enter any measured arm or cause a rerun.

### New independent runs and exact measurements

Each arm ran once with fork_turns=none, inherited model/reasoning settings,
no override and the same tool surface/transport. Dummy could freely choose
one-shot or shell; both chose shell this time. Genie followed its pinned
public README. No oracle, implementation conversation, private docs, current
NO-GO README, other arm output or eight-command answer sequence was supplied.
Three arms started first; Genie N started after Dummy N completed. Post-run
reporting-only follow-ups confirmed tool counts without executing any tool or
product command. They are outside the measured intervals and are not reruns.

| Measure | Dummy N-1 | Dummy N | Genie N-1 | Genie N |
| --- | ---: | ---: | ---: | ---: |
| bw processes | 5 | 3 | 3 | 2 |
| Successful public Help commands | 4 | 2 | 2 | 1 |
| Persistent shell starts / exits | 1 / 1 | 1 / 1 | 1 / 1 | 1 / 1 |
| Search commands, including binding Search | 4 | 1 | 2 | 1 |
| View commands, direct and raw | 13 | 2 | 24 | 2 |
| Replace commands | 4 | 4 | 4 | 4 |
| Check commands | 1 | 1 | 2 | 1 |
| Capability commands total | 22 | 8 | 32 | 8 |
| Explicit raw Apply | 0 | 0 | 0 | 0 |
| Named-copy aliases, excluding Search binding | 0 | 0 | 12 | 0 |
| Unexpected CLI errors: unsupported shell help | 1 | 0 | 1 | 0 |
| Final shell OS exit | 2 | 0 | 2 | 0 |
| functions.exec / functions.wait | 13 / 0 | 8 / 0 | 21 / 0 | 18 / 0 |
| Recorder client invocations: request + ack | 26 | 16 | 42 | 36 |
| Product stdout bytes, including Help | 6,648 | 5,047 | 3,745 | 4,058 |
| Product stderr bytes | 1,005 | 0 | 1,005 | 0 |
| Public README bytes delivered | 0 | 0 | 16,824 | 17,524 |
| Model-delivered text-content UTF-8 bytes | 26,238 | 16,600 | 61,779 | 58,694 |
| Whole-arm elapsed ns | 159293909843 | 78465998512 | 261773365789 | 214018000320 |
| Four final files / complete name set | exact | exact | exact | exact |
| Mutation retry / restart / Wrong Apply / newline mistakes | 0 / 0 / 0 / 0 | 0 / 0 / 0 / 0 | 0 / 0 / 0 / 0 | 0 / 0 / 0 / 0 |

These are n=1 observations, not a performance score or causal speed claim.
Whole-arm time includes model thinking, transport, permissions and scheduling
between the first Help/guide request and final product close. Recorder daemon
and shell/sandbox/tool-service launch infrastructure are not bw processes.
The shared daemon is one task-local process; client counts above are explicit
transport launches, not a total of all platform OS descendants. Fixture/build/
preflight precede the evaluated intervals; parent oracle/audit/report and
approval-blocked documentation work follow them. Complete preparation/report
elapsed times were not instrumented and are not folded into arm time.

Dummy N-1 first obtained eight Lines, then tried a Paragraph Search and direct
View that still omitted Content. It used a bound Paragraph Search and eight
raw Views for context, then a File Search and four raw Views for final
inspection. Thus 12 raw Content reads remain explicit, plus two extra
Paragraph Searches and one File discovery Search. Its four Help queries and
one failed shell `help` are also retained. The four old secondary refs in its
Check are NotCurrent after source edits; that is expected currentness, not
a wrong edit or a retry.

Genie N-1 made eight individual Paragraph projections and four individual
File projections, then twelve named-copy/raw-View pairs to obtain their
Content. The additional twelve raw Views are Content recovery. Its extra
Search/Check of unchanged secondary Lines is optional currentness verification,
not mutation retry. Individual projection granularity is an observed caller
choice, not a hidden batch capability or automatic defect. It also tried
unsupported shell `help` once and completed in the same process with exit 2.

Both N arms used one eight-input Paragraph View, four Replace commands, one
Check of fresh receipts and one four-input File View. Their shell stdin and
complete shell stdout are byte-identical to each other; stderr is empty.
There is no additional Content retrieval or optional baseline View in these
new N runs. Check was requested by the evaluation task, not introduced as a
mandatory product workflow. Each finished arm was independently compared to
the original fixed expected map: all sixteen files and directory name sets
match, preserving all secondary bytes and terminators.

### Reuse, executed checks and limits

Reused, not rerun: Gate 5's GNU/musl 293/293 and 52 release smokes, after
byte equality of src/tests/Cargo/lock/toolchain and absence of additional
repository build configuration. The src tree remains
`8427811aaae0763aac22e774f01244c246c9d4ee`. Default GNU target/features/
release profile and Rust/Cargo 1.95.0 match the recorded boundary. The rebuilt
N GNU SHA and reused N-1 GNU SHA match exactly. No musl build, full suite, old
benchmark, installer, updater, public endpoint or native macOS/Windows run
was performed. Server, installed bw, real HOME and operational state were not
accessed or changed.

Executed: pinned N GNU release reconstruction, Help/Version digest checks,
capture preflight, fixed-fixture/hash checks, four fresh arms, independent
oracle/capture reconstruction, offline/locked full metadata, and document/
diff/path/index/hygiene audits. Ten Markdown files pass balanced fences,
61 local file links and five local anchor checks before final archival;
these checks are repeated after archival. Historical source/roadmap notes
and every old transcript are preserved. The complete prior tracker suffix
from its old Status heading onward was checked byte-identical to the entry
commit, not merely compared by selected hashes.

The platform-added tool envelope/tokenization/UI serialization and model build
identifier/seed are not observable. Counts cover the exact text strings passed
to text(), including our full JSON metadata/escaping/base64 duplication; they
do not claim the unknowable outer envelope. Raw stream bytes are separate.
No unobserved output is represented as zero. Clock resolution is not a claim
of nanosecond physical accuracy. These declared limitations do not conceal
missing required streams or replace BOX 26 with command-count optimization.

### Reconstruction and arm inputs

Reuse the two pinned Git objects and the binary hashes from the preflight
section. The pinned README SHAs are:

- N-1: `7a5f257cc100af7f78125a8cad0582b507f86522be8a0f3cc94c2202276ae91a`.
- N: `daececdcfaed7f72fc0311c996d08fdcacf099aa514a4a987206b6cf7c419ce5`.

The fixed initial/expected map is preserved in the original Gate 5 section.
The archived prepare-fixture source below recreates the same 366-byte maps,
755-byte combined JSON and individual file hashes, using absent directories
and exclusive writes. It is not a target-selection driver.

The common evaluator task was: perform exactly one independent run; discover
the eight exact `retry_budget = 3` Lines in the four workspace files; read
surrounding Paragraph Content; select the four whose Paragraph says
`role = primary`; replace only those bodies with `retry_budget = 5`;
preserve secondary Paragraphs and every other byte/terminator; confirm
currentness and inspect final Files with bw.

The full shared restrictions were: no repository source, AGENTS, private docs,
other arms, evaluator/oracle files or installed bw; no reset/retry/best-of
restart; no install/update; all body reads/writes through supplied bw and
recorder, never filesystem tools. Transport may preserve/parse output but
must not choose targets; the evaluator selects from Content itself. Dummy
receives public Help only and chooses one-shot/shell freely. Genie receives
the pinned public README and Help and derives the README's shell route.
A final English account reports route, actual commands, failures, additional
Content retrieval or optional verification and result; it does not write a
transcript file. No source or sibling result is exposed.

For each arm, substitute its literal name (dummy-old, dummy-new, genie-old,
genie-new) as ARM in the following exact transport pattern. The supplied
request examples were start with args=['--help'], eof=true for Dummy, and
guide for Genie. send uses processId plus literal stdin; eof=true closes
stdin. poll must drain every more=true response; active lists live child IDs.
After all processes close and output drains, finish is requested once.
Unexpected recorder failure must be reported, never hidden or retried.
Stored public output may be parsed in functions store/load to pass exact
Anddresses without retyping. Every visible response uses one functions.exec
call, including empty event lists; no other tool is needed for the body task.

```javascript
const request = {arm: ARM, op: 'start', args: ['--help'], eof: true};
const q = s => "'" + s.replaceAll("'", "'\\''") + "'";
const call = async v => await tools.exec_command({
  cmd: 'node /tmp/backwriter-gate5r.kUMPDh/recorder.mjs client /tmp/backwriter-gate5r.kUMPDh ' + q(JSON.stringify(v)),
  sandbox_permissions: 'require_escalated',
  justification: 'Run the owner-approved isolated Gate 5R pipe evaluation and preserve exact delivery records.',
  max_output_tokens: 6000
});
const r = await call(request);
if (r.exit_code !== 0) throw Error(r.output);
const response = JSON.parse(r.output);
if (response.recorderError) throw Error(response.recorderError);
const ack = await call({arm: ARM, op: 'ack', id: response.id, content: r.output});
if (ack.output !== 'OK\n') throw Error('Capture acknowledgement failed');
text(r.output);
```

The archived capture-config maps each arm to its pinned binary, isolated cwd
and (Genie only) public guide. The config shown is the evaluation config;
the initial preflight used only its first three entries. The preflight
daemon was stopped before starting the evaluation daemon. No evaluation was
restarted. Four reporting-only follow-ups asked for functions.exec/wait counts
from the agents' own conversations, with no tools/files/bw permitted; all
reported one exact text emission per response, zero wait calls and no other
tools.

For offline evidence reconstruction, each archived file section below gives
its relative task-root path, exact byte count, SHA and UTF-8 content. Write the
fence contents without including fences or adding a newline. Reconstruct
events.jsonl by flattening JSON.parse(entry.content).events from delivered.jsonl
in entry order and serializing each event as compact JSON plus LF. Reconstruct
response-N.json directly from each entry.content. This reproduces every raw
byte segment, stdout/stderr separation, request/argv/cwd/stdin, ordering,
process exit and model-delivered response. Raw stream bytes are base64-decoded
and concatenated by kind/processId in sequence order. No command-level OS
exit is invented for a shell.

Count visible bytes as sum(Buffer.byteLength(entry.content, 'utf8')); verify
each entry.bytes/SHA independently. Whole time is final close.ns minus the
first requests.jsonl ns. Count bw starts, actual shell command lines and
one-shot argv separately; exclude Help/shell lifecycle from capability counts.
Classify Content recovery and optional verification from command order and
actual Content, not count differences alone. audit-arms source and its exact
output preserve the independent full-byte oracle and formula checks.
The records include unsuccessful CLI commands, empty responses and all guide/
Help chunks. SHA alone is never substituted for the archived source/records.

### Gate 5R archived files




#### Archived file: prepare-fixture.mjs

Bytes: 2274; SHA-256: `a2cc5ded3911fa59683a44bd71991e40067c5b70214c325381a67925ffcbaaca`.

```javascript
import fs from 'node:fs';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
const root = '/tmp/backwriter-gate5r.kUMPDh';
const initial = {
  'unit-01.txt': 'role = primary\nretry_budget = 3\n\nrole = secondary\nretry_budget = 3\n',
  'unit-02.txt': 'role = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 3\r',
  'unit-03.txt': 'role = primary\r\nretry_budget = 3\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n',
  'unit-04.txt': 'role = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 3',
};
const expected = {
  'unit-01.txt': 'role = primary\nretry_budget = 5\n\nrole = secondary\nretry_budget = 3\n',
  'unit-02.txt': 'role = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 5\r',
  'unit-03.txt': 'role = primary\r\nretry_budget = 5\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n',
  'unit-04.txt': 'role = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 5',
};
const sha = value => createHash('sha256').update(value).digest('hex');
const serialize = value => JSON.stringify(value) + '\n';
assert.equal(Buffer.byteLength(serialize(initial)), 366);
assert.equal(Buffer.byteLength(serialize(expected)), 366);
assert.equal(sha(serialize(initial)), 'f7fa969215979bb27ae3a983c9c44f9673bc2f9573a2bda39839639cb6da8f75');
assert.equal(sha(serialize(expected)), 'e1a729c3b3a8c73a730f92289f8b6d34e3681bdc018cacb4229503b747249bc2');
assert.equal(sha(serialize({ initial, expected })), '26e2853962e5611c640c4d5c15f25ca68988a112600bce69f350d58497075998');
for (const arm of ['dummy-old', 'dummy-new', 'genie-old', 'genie-new']) {
  fs.mkdirSync(`${root}/${arm}`);
  fs.mkdirSync(`${root}/${arm}/workspace`);
  for (const [name, content] of Object.entries(initial)) {
    fs.writeFileSync(`${root}/${arm}/workspace/${name}`, content, { flag: 'wx' });
    assert.equal(sha(fs.readFileSync(`${root}/${arm}/workspace/${name}`)), sha(content));
  }
}
fs.writeFileSync(`${root}/fixture.json`, serialize({ initial, expected }), { flag: 'wx' });
process.stdout.write(JSON.stringify({ initial: Object.fromEntries(Object.entries(initial).map(([name, content]) => [name, sha(content)])), expected: Object.fromEntries(Object.entries(expected).map(([name, content]) => [name, sha(content)])) }) + '\n');
```

#### Archived file: audit-arms.mjs

Bytes: 3844; SHA-256: `d77188d9e645c6797b447dcd2b2c4725e5a76f30cb2bd7582ff6b13c06235819`.

```javascript
import fs from 'node:fs';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
const root = '/tmp/backwriter-gate5r.kUMPDh';
const fixture = JSON.parse(fs.readFileSync(`${root}/fixture.json`));
const sha = value => createHash('sha256').update(value).digest('hex');
const lines = file => fs.readFileSync(file, 'utf8').split('\n').filter(Boolean).map(x => JSON.parse(x));
const byteStream = (events, kind, id) => Buffer.concat(events.filter(x => x.kind === kind && (id === undefined || x.processId === id)).map(x => Buffer.from(x.base64, 'base64')));
const summaries = [];
for (const arm of process.argv.slice(2)) {
  const record = `${root}/records/${arm}`;
  const events = lines(`${record}/events.jsonl`);
  const requests = lines(`${record}/requests.jsonl`);
  const delivered = lines(`${record}/delivered.jsonl`);
  const finished = events.at(-1);
  assert.equal(finished.kind, 'finished');
  const responseNames = fs.readdirSync(record).filter(x => /^response-[0-9]+\.json$/.test(x));
  assert.equal(responseNames.length, delivered.length);
  for (const entry of delivered) {
    assert.equal(entry.content, fs.readFileSync(`${record}/response-${entry.id}.json`, 'utf8'));
    assert.equal(entry.bytes, Buffer.byteLength(entry.content));
    assert.equal(entry.sha256, sha(entry.content));
  }
  assert.deepEqual(delivered.flatMap(x => JSON.parse(x.content).events), events);
  assert.deepEqual(delivered.map(x => x.id), delivered.map((_, i) => i + 1));
  assert.equal(finished.firstNs, requests[0].ns);
  const closes = events.filter(x => x.kind === 'close');
  assert.equal(finished.lastProductCloseNs, closes.at(-1).ns);
  assert.equal(finished.elapsedNs, (BigInt(closes.at(-1).ns) - BigInt(requests[0].ns)).toString());
  assert(events.every((x, i) => i === 0 || BigInt(x.ns) >= BigInt(events[i - 1].ns)));
  const starts = events.filter(x => x.kind === 'start');
  assert.equal(starts.length, closes.length);
  const processes = starts.map(start => {
    const id = start.processId;
    const stdin = byteStream(events, 'stdin', id);
    const stdout = byteStream(events, 'stdout', id);
    const stderr = byteStream(events, 'stderr', id);
    const close = closes.find(x => x.processId === id);
    assert(close);
    assert(events.some(x => x.kind === 'stdout-eof' && x.processId === id));
    assert(events.some(x => x.kind === 'stderr-eof' && x.processId === id));
    return { processId: id, argv: start.argv, cwd: start.cwd, startNs: start.ns, closeNs: close.ns, exit: close.code, signal: close.signal, stdin: stdin.toString('utf8'), stdoutBytes: stdout.length, stderrBytes: stderr.length, stdoutSha256: sha(stdout), stderrSha256: sha(stderr) };
  });
  const fileNames = fs.readdirSync(`${root}/${arm}/workspace`).sort();
  assert.deepEqual(fileNames, Object.keys(fixture.expected).sort());
  const finalFiles = Object.fromEntries(fileNames.map(name => {
    const file = `${root}/${arm}/workspace/${name}`;
    assert(fs.lstatSync(file).isFile());
    const bytes = fs.readFileSync(file);
    assert(bytes.equals(Buffer.from(fixture.expected[name])));
    return [name, { bytes: bytes.length, sha256: sha(bytes), content: bytes.toString('utf8') }];
  }));
  const summary = { arm, captureComplete: true, exactOracle: true, processes, finalFiles, visibleResponses: delivered.length, visibleBytes: delivered.reduce((sum, x) => sum + x.bytes, 0), stdoutBytes: byteStream(events, 'stdout').length, stderrBytes: byteStream(events, 'stderr').length, guideBytes: byteStream(events, 'guide').length, wholeArmElapsedNs: finished.elapsedNs, firstRequest: requests[0].request, recorderErrors: events.filter(x => x.kind.endsWith('-error')) };
  fs.writeFileSync(`${root}/${arm}-audit.json`, JSON.stringify(summary) + '\n');
  summaries.push(summary);
}
process.stdout.write(JSON.stringify(summaries) + '\n');
```

#### Archived file: audit-docs.mjs

Bytes: 2418; SHA-256: `c22a91e549fd9e6c7db0bdd650ae1643d07050ba2c7d44fd683ea00034615945`.

```javascript
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
const root = '/home/NOEEZ/backwriter';
const files = ['AGENTS.md', 'README.md', 'docs/architecture/backwriter-cli-v1.md', 'docs/architecture/backwriter-text-coordination-protocol.md', 'docs/architecture/rebuildable-structural-addressing.md', 'docs/current/now.md', 'docs/current/roadmap.md', 'docs/development/verification.md', 'docs/principles/backwriter-core-principles.md', 'docs/tasks/2026-09-05-backwriter-0.3.0-independent-namespace-complete-view.md'];
function prose(file) {
  let fence = null;
  const output = [];
  for (const line of fs.readFileSync(file, 'utf8').split('\n')) {
    const match = line.match(/^\s{0,3}(`{3,}|~{3,})(.*)$/);
    if (match) {
      if (!fence) fence = match[1];
      else if (match[1][0] === fence[0] && match[1].length >= fence.length && match[2].trim() === '') fence = null;
      continue;
    }
    if (!fence) output.push(line);
  }
  assert.equal(fence, null, `Unclosed fence: ${file}`);
  return output;
}
let links = 0;
let anchors = 0;
for (const name of files) {
  const file = path.join(root, name);
  const text = fs.readFileSync(file, 'utf8');
  assert(!/^<{7} |^={7}$|^>{7} /m.test(text), `Conflict marker: ${name}`);
  for (const line of prose(file)) {
    for (const match of line.matchAll(/\[[^\]]*\]\(([^)]+)\)/g)) {
      const href = match[1];
      if (/^[a-z][a-z0-9+.-]*:/i.test(href)) continue;
      const [target, fragment] = href.split('#');
      const destination = target ? path.resolve(path.dirname(file), decodeURIComponent(target)) : file;
      assert(fs.existsSync(destination), `${name}: ${href}`);
      if (target) links++;
      if (fragment && destination.endsWith('.md')) {
        const seen = new Map();
        const ids = prose(destination).filter(x => /^#{1,6} /.test(x)).map(x => {
          const id = x.replace(/^#{1,6} /, '').toLowerCase().replace(/[^\p{L}\p{N}_\-\s]/gu, '').replace(/\s/g, '-');
          const count = seen.get(id) || 0;
          seen.set(id, count + 1);
          return count ? `${id}-${count}` : id;
        });
        assert(ids.includes(decodeURIComponent(fragment)), `${name}: unresolved fragment ${href}`);
        anchors++;
      }
    }
  }
}
process.stdout.write(JSON.stringify({ files: files.length, localFileLinks: links, localAnchors: anchors, fences: 'balanced', conflictMarkers: 0 }) + '\n');
```

#### Archived file: capture-config.json

Bytes: 941; SHA-256: `bb54b8eab9c672dc2b464d75f64dbc486836516831724dd68a18bbfc32aa9d56`.

```json
{
  "preflight": {"binary":"/usr/bin/node","cwd":"/tmp/backwriter-gate5r.kUMPDh"},
  "help-n": {"binary":"/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw","cwd":"/tmp/backwriter-gate5r.kUMPDh"},
  "help-old": {"binary":"/tmp/backwriter-gate5r.kUMPDh/bw-nminus","cwd":"/tmp/backwriter-gate5r.kUMPDh"},
  "dummy-old": {"binary":"/tmp/backwriter-gate5r.kUMPDh/bw-nminus","cwd":"/tmp/backwriter-gate5r.kUMPDh/dummy-old/workspace"},
  "dummy-new": {"binary":"/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw","cwd":"/tmp/backwriter-gate5r.kUMPDh/dummy-new/workspace"},
  "genie-old": {"binary":"/tmp/backwriter-gate5r.kUMPDh/bw-nminus","cwd":"/tmp/backwriter-gate5r.kUMPDh/genie-old/workspace","guide":"/tmp/backwriter-gate5r.kUMPDh/nminus/README.md"},
  "genie-new": {"binary":"/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw","cwd":"/tmp/backwriter-gate5r.kUMPDh/genie-new/workspace","guide":"/tmp/backwriter-gate5r.kUMPDh/n/README.md"}
}
```

#### Archived file: fixture.json

Bytes: 755; SHA-256: `26e2853962e5611c640c4d5c15f25ca68988a112600bce69f350d58497075998`.

```json
{"initial":{"unit-01.txt":"role = primary\nretry_budget = 3\n\nrole = secondary\nretry_budget = 3\n","unit-02.txt":"role = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 3\r","unit-03.txt":"role = primary\r\nretry_budget = 3\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n","unit-04.txt":"role = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 3"},"expected":{"unit-01.txt":"role = primary\nretry_budget = 5\n\nrole = secondary\nretry_budget = 3\n","unit-02.txt":"role = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 5\r","unit-03.txt":"role = primary\r\nretry_budget = 5\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n","unit-04.txt":"role = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 5"}}
```

#### Archived file: all-arms-audit.json

Bytes: 12003; SHA-256: `5122a9106be24a6d9d8316ea0e417f85d4f86e73dd9ec7c68d5f41603e05ebc6`.

```json
[{"arm":"dummy-old","captureComplete":true,"exactOracle":true,"processes":[{"processId":1,"argv":["/tmp/backwriter-gate5r.kUMPDh/bw-nminus","--help"],"cwd":"/tmp/backwriter-gate5r.kUMPDh/dummy-old/workspace","startNs":"166481367033280","closeNs":"166481370028299","exit":0,"signal":null,"stdin":"","stdoutBytes":963,"stderrBytes":0,"stdoutSha256":"3e2695cbc789f1fd7073926bae5ec3675c9c0c30f71425446ea8510e477e2bec","stderrSha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"},{"processId":2,"argv":["/tmp/backwriter-gate5r.kUMPDh/bw-nminus","help","shell"],"cwd":"/tmp/backwriter-gate5r.kUMPDh/dummy-old/workspace","startNs":"166499290336647","closeNs":"166499291791914","exit":0,"signal":null,"stdin":"","stdoutBytes":1237,"stderrBytes":0,"stdoutSha256":"82ac225914e15f2f5cb8e9c800c5cae67620682d9d541cf7c7ba27c40119ac79","stderrSha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"},{"processId":3,"argv":["/tmp/backwriter-gate5r.kUMPDh/bw-nminus","shell"],"cwd":"/tmp/backwriter-gate5r.kUMPDh/dummy-old/workspace","startNs":"166513276949953","closeNs":"166640660817381","exit":2,"signal":null,"stdin":"search line \"retry_budget = 3\"\nsearch paragraph \"retry_budget = 3\"\nview @8 @9 @10 @11 @12 @13 @14 @15\nhelp\nlet paras = search paragraph \"retry_budget = 3\"\nview anddress @paras[0]\nview anddress @paras[1]\nview anddress @paras[2]\nview anddress @paras[3]\nview anddress @paras[4]\nview anddress @paras[5]\nview anddress @paras[6]\nview anddress @paras[7]\nreplace @0 \"retry_budget = 5\"\nreplace @3 \"retry_budget = 5\"\nreplace @4 \"retry_budget = 5\"\nreplace @7 \"retry_budget = 5\"\ncheck @24 @25 @26 @27 @1 @2 @5 @6\nlet files = search file \"retry_budget\"\nview anddress @files[0]\nview anddress @files[1]\nview anddress @files[2]\nview anddress @files[3]\nexit\n","stdoutBytes":1801,"stderrBytes":1005,"stdoutSha256":"e0333a30040448f1811fb5a9c7cb1623a26723717572cf08849358fd5b436817","stderrSha256":"d5d3111e7bc277a9fb3597d145619021d8d52f2402e97ab60f1eb6eb838c28e5"},{"processId":4,"argv":["/tmp/backwriter-gate5r.kUMPDh/bw-nminus","help","view"],"cwd":"/tmp/backwriter-gate5r.kUMPDh/dummy-old/workspace","startNs":"166579183007902","closeNs":"166579184864806","exit":0,"signal":null,"stdin":"","stdoutBytes":1261,"stderrBytes":0,"stdoutSha256":"ea073b42100b802de237e9c08ea473d75729858a6f337a18970a54f9781b7c09","stderrSha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"},{"processId":5,"argv":["/tmp/backwriter-gate5r.kUMPDh/bw-nminus","help","edit"],"cwd":"/tmp/backwriter-gate5r.kUMPDh/dummy-old/workspace","startNs":"166610362576778","closeNs":"166610364220203","exit":0,"signal":null,"stdin":"","stdoutBytes":1386,"stderrBytes":0,"stdoutSha256":"d489240b93239b818bc2c2ba9bbb66af9489519f6f67af863902b0e55a45a65c","stderrSha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"}],"finalFiles":{"unit-01.txt":{"bytes":67,"sha256":"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0","content":"role = primary\nretry_budget = 5\n\nrole = secondary\nretry_budget = 3\n"},"unit-02.txt":{"bytes":67,"sha256":"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a","content":"role = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 5\r"},"unit-03.txt":{"bytes":72,"sha256":"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0","content":"role = primary\r\nretry_budget = 5\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n"},"unit-04.txt":{"bytes":66,"sha256":"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe","content":"role = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 5"}},"visibleResponses":13,"visibleBytes":26238,"stdoutBytes":6648,"stderrBytes":1005,"guideBytes":0,"wholeArmElapsedNs":"159293909843","firstRequest":{"arm":"dummy-old","op":"start","args":["--help"],"eof":true},"recorderErrors":[]},{"arm":"dummy-new","captureComplete":true,"exactOracle":true,"processes":[{"processId":1,"argv":["/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw","--help"],"cwd":"/tmp/backwriter-gate5r.kUMPDh/dummy-new/workspace","startNs":"166485596566364","closeNs":"166485598644948","exit":0,"signal":null,"stdin":"","stdoutBytes":989,"stderrBytes":0,"stdoutSha256":"c8eb57673d78976404fbb6bb4d9b4aeee6dabbc483ca258c53426cd01d66e1ac","stderrSha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"},{"processId":2,"argv":["/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw","help","shell"],"cwd":"/tmp/backwriter-gate5r.kUMPDh/dummy-new/workspace","startNs":"166498897645169","closeNs":"166498899398902","exit":0,"signal":null,"stdin":"","stdoutBytes":2457,"stderrBytes":0,"stdoutSha256":"d13e993e7101361987d8b54722753a4d815c6724671e974e50d40ea3fa6e28b8","stderrSha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"},{"processId":3,"argv":["/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw","shell"],"cwd":"/tmp/backwriter-gate5r.kUMPDh/dummy-new/workspace","startNs":"166524071705095","closeNs":"166564062512531","exit":0,"signal":null,"stdin":"search line \"retry_budget = 3\"\nview @0 @1 @2 @3 @4 @5 @6 @7 --as paragraph\nreplace @0 \"retry_budget = 5\"\nreplace @3 \"retry_budget = 5\"\nreplace @4 \"retry_budget = 5\"\nreplace @7 \"retry_budget = 5\"\ncheck @16 @17 @18 @19\nview @16 @17 @18 @19 --as file\nexit\n","stdoutBytes":1601,"stderrBytes":0,"stdoutSha256":"7d0819f22d99bd5aeaf02865282b7359e6f90b3c62f182dd5283966f898b3910","stderrSha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"}],"finalFiles":{"unit-01.txt":{"bytes":67,"sha256":"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0","content":"role = primary\nretry_budget = 5\n\nrole = secondary\nretry_budget = 3\n"},"unit-02.txt":{"bytes":67,"sha256":"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a","content":"role = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 5\r"},"unit-03.txt":{"bytes":72,"sha256":"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0","content":"role = primary\r\nretry_budget = 5\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n"},"unit-04.txt":{"bytes":66,"sha256":"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe","content":"role = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 5"}},"visibleResponses":8,"visibleBytes":16600,"stdoutBytes":5047,"stderrBytes":0,"guideBytes":0,"wholeArmElapsedNs":"78465998512","firstRequest":{"arm":"dummy-new","op":"start","args":["--help"],"eof":true},"recorderErrors":[]},{"arm":"genie-old","captureComplete":true,"exactOracle":true,"processes":[{"processId":1,"argv":["/tmp/backwriter-gate5r.kUMPDh/bw-nminus","--help"],"cwd":"/tmp/backwriter-gate5r.kUMPDh/genie-old/workspace","startNs":"166625404831215","closeNs":"166625406300843","exit":0,"signal":null,"stdin":"","stdoutBytes":963,"stderrBytes":0,"stdoutSha256":"3e2695cbc789f1fd7073926bae5ec3675c9c0c30f71425446ea8510e477e2bec","stderrSha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"},{"processId":2,"argv":["/tmp/backwriter-gate5r.kUMPDh/bw-nminus","shell"],"cwd":"/tmp/backwriter-gate5r.kUMPDh/genie-old/workspace","startNs":"166639824762347","closeNs":"166768764076968","exit":2,"signal":null,"stdin":"search line \"retry_budget = 3\"\nview @0 --as paragraph\nview @1 --as paragraph\nview @2 --as paragraph\nview @3 --as paragraph\nview @4 --as paragraph\nview @5 --as paragraph\nview @6 --as paragraph\nview @7 --as paragraph\nhelp\nlet p0 = @8\nlet p1 = @9\nlet p2 = @10\nlet p3 = @11\nlet p4 = @12\nlet p5 = @13\nlet p6 = @14\nlet p7 = @15\nview anddress @p0\nview anddress @p1\nview anddress @p2\nview anddress @p3\nview anddress @p4\nview anddress @p5\nview anddress @p6\nview anddress @p7\nreplace @0 \"retry_budget = 5\"\nreplace @3 \"retry_budget = 5\"\nreplace @4 \"retry_budget = 5\"\nreplace @7 \"retry_budget = 5\"\ncheck @16 @17 @18 @19 @1 @2 @5 @6\nview @16 --as file\nview @17 --as file\nview @18 --as file\nview @19 --as file\nlet f0 = @24\nlet f1 = @25\nlet f2 = @26\nlet f3 = @27\nview anddress @f0\nview anddress @f1\nview anddress @f2\nview anddress @f3\nsearch line \"retry_budget = 3\"\ncheck @28 @29 @30 @31\nexit\n","stdoutBytes":1545,"stderrBytes":1005,"stdoutSha256":"0eb8758f2fc9f15c072be5568b3aa03448a465b116a09b16c98da8c23c3a3ee3","stderrSha256":"d5d3111e7bc277a9fb3597d145619021d8d52f2402e97ab60f1eb6eb838c28e5"},{"processId":3,"argv":["/tmp/backwriter-gate5r.kUMPDh/bw-nminus","help","shell"],"cwd":"/tmp/backwriter-gate5r.kUMPDh/genie-old/workspace","startNs":"166696703302619","closeNs":"166696705187148","exit":0,"signal":null,"stdin":"","stdoutBytes":1237,"stderrBytes":0,"stdoutSha256":"82ac225914e15f2f5cb8e9c800c5cae67620682d9d541cf7c7ba27c40119ac79","stderrSha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"}],"finalFiles":{"unit-01.txt":{"bytes":67,"sha256":"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0","content":"role = primary\nretry_budget = 5\n\nrole = secondary\nretry_budget = 3\n"},"unit-02.txt":{"bytes":67,"sha256":"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a","content":"role = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 5\r"},"unit-03.txt":{"bytes":72,"sha256":"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0","content":"role = primary\r\nretry_budget = 5\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n"},"unit-04.txt":{"bytes":66,"sha256":"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe","content":"role = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 5"}},"visibleResponses":21,"visibleBytes":61779,"stdoutBytes":3745,"stderrBytes":1005,"guideBytes":16824,"wholeArmElapsedNs":"261773365789","firstRequest":{"arm":"genie-old","op":"guide"},"recorderErrors":[]},{"arm":"genie-new","captureComplete":true,"exactOracle":true,"processes":[{"processId":1,"argv":["/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw","help","shell"],"cwd":"/tmp/backwriter-gate5r.kUMPDh/genie-new/workspace","startNs":"166745584717025","closeNs":"166745586327880","exit":0,"signal":null,"stdin":"","stdoutBytes":2457,"stderrBytes":0,"stdoutSha256":"d13e993e7101361987d8b54722753a4d815c6724671e974e50d40ea3fa6e28b8","stderrSha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"},{"processId":2,"argv":["/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw","shell"],"cwd":"/tmp/backwriter-gate5r.kUMPDh/genie-new/workspace","startNs":"166771478373617","closeNs":"166844205804526","exit":0,"signal":null,"stdin":"search line \"retry_budget = 3\"\nview @0 @1 @2 @3 @4 @5 @6 @7 --as paragraph\nreplace @0 \"retry_budget = 5\"\nreplace @3 \"retry_budget = 5\"\nreplace @4 \"retry_budget = 5\"\nreplace @7 \"retry_budget = 5\"\ncheck @16 @17 @18 @19\nview @16 @17 @18 @19 --as file\nexit\n","stdoutBytes":1601,"stderrBytes":0,"stdoutSha256":"7d0819f22d99bd5aeaf02865282b7359e6f90b3c62f182dd5283966f898b3910","stderrSha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"}],"finalFiles":{"unit-01.txt":{"bytes":67,"sha256":"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0","content":"role = primary\nretry_budget = 5\n\nrole = secondary\nretry_budget = 3\n"},"unit-02.txt":{"bytes":67,"sha256":"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a","content":"role = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 5\r"},"unit-03.txt":{"bytes":72,"sha256":"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0","content":"role = primary\r\nretry_budget = 5\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n"},"unit-04.txt":{"bytes":66,"sha256":"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe","content":"role = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 5"}},"visibleResponses":18,"visibleBytes":58694,"stdoutBytes":4058,"stderrBytes":0,"guideBytes":17524,"wholeArmElapsedNs":"214018000320","firstRequest":{"arm":"genie-new","op":"guide"},"recorderErrors":[]}]
```


#### Archived file: records/preflight/requests.jsonl

Bytes: 233; SHA-256: `56bbc4defa36c65a8617ce79deda936833169e07a3a71cb9566efb93f23d3920`.

```jsonl
{"ns":"166144189712500","request":{"arm":"preflight","op":"start","args":["/tmp/backwriter-gate5r.kUMPDh/preflight-child.mjs"],"stdin":"입력\r\n끝\n","eof":true}}
{"ns":"166176394085180","request":{"arm":"preflight","op":"poll"}}
```

#### Archived file: records/preflight/delivered.jsonl

Bytes: 1768; SHA-256: `4838c2f0b68e7050fef71e3d6195c7cba796b8a31b9e3fd6dc3bc4bbd80025a1`.

```jsonl
{"id":1,"bytes":1279,"sha256":"9d79a252b102c481a548044f4750fc1ab55daaed448de63f6f38ffe14d484e42","content":"{\"id\":1,\"events\":[{\"sequence\":1,\"ns\":\"166144190228422\",\"kind\":\"start\",\"processId\":1,\"argv\":[\"/usr/bin/node\",\"/tmp/backwriter-gate5r.kUMPDh/preflight-child.mjs\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh\"},{\"sequence\":2,\"ns\":\"166144192290991\",\"kind\":\"stdin\",\"processId\":1,\"base64\":\"7J6F66ClDQrrgZ0K\",\"text\":\"입력\\r\\n끝\\n\"},{\"sequence\":3,\"ns\":\"166144192353078\",\"kind\":\"stdin-eof\",\"processId\":1},{\"sequence\":4,\"ns\":\"166144192615252\",\"kind\":\"spawn\",\"processId\":1,\"pid\":2719798},{\"sequence\":5,\"ns\":\"166144212668931\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"7LKrIOykhA0KcXVvdGU9IiBzbGFzaD1cIHRhYj0JCg==\",\"text\":\"첫 줄\\r\\nquote=\\\" slash=\\\\ tab=\\t\\n\"},{\"sequence\":6,\"ns\":\"166144212874599\",\"kind\":\"stderr\",\"processId\":1,\"base64\":\"7KCV7IOBIHN0ZGVycgo=\",\"text\":\"정상 stderr\\n\"},{\"sequence\":7,\"ns\":\"166144244156367\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"7J6F66ClDQrrgZ0K\",\"text\":\"입력\\r\\n끝\\n\"},{\"sequence\":8,\"ns\":\"166144244486187\",\"kind\":\"stderr\",\"processId\":1,\"base64\":\"7Iuk7YyoIHN0ZGVycg0K\",\"text\":\"실패 stderr\\r\\n\"},{\"sequence\":9,\"ns\":\"166144247547822\",\"kind\":\"stderr-eof\",\"processId\":1},{\"sequence\":10,\"ns\":\"166144248195530\",\"kind\":\"stdout-eof\",\"processId\":1},{\"sequence\":11,\"ns\":\"166144248813197\",\"kind\":\"close\",\"processId\":1,\"code\":7,\"signal\":null}],\"more\":false,\"active\":[]}\n"}
{"id":2,"bytes":46,"sha256":"f2fc61251fc94d3904766791b1bf4bca678e080da28c4c0356caccb2f91c8450","content":"{\"id\":2,\"events\":[],\"more\":false,\"active\":[]}\n"}
```

#### Archived file: records/help-n/requests.jsonl

Bytes: 258; SHA-256: `cf19b2cdf4d635c50ef76263789fe2c96c94e27c7d29cd8bbd47bdf7e44e396c`.

```jsonl
{"ns":"166191709243891","request":{"arm":"help-n","op":"start","args":["--help"],"eof":true}}
{"ns":"166206028714643","request":{"arm":"help-n","op":"start","args":["help","shell"],"eof":true}}
{"ns":"166224435153627","request":{"arm":"help-n","op":"poll"}}
```

#### Archived file: records/help-n/delivered.jsonl

Bytes: 10821; SHA-256: `f961d46b41347e414761634a7e939882f83a236596a8572843908494f1cc1ecf`.

```jsonl
{"id":1,"bytes":3119,"sha256":"592f9c269e84f50ed8224cf294cf85b270d1920d9100c74abe0ef6d935bddbca","content":"{\"id\":1,\"events\":[{\"sequence\":1,\"ns\":\"166191709297113\",\"kind\":\"start\",\"processId\":1,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw\",\"--help\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh\"},{\"sequence\":2,\"ns\":\"166191710439450\",\"kind\":\"stdin-eof\",\"processId\":1},{\"sequence\":3,\"ns\":\"166191710505387\",\"kind\":\"spawn\",\"processId\":1,\"pid\":2720188},{\"sequence\":4,\"ns\":\"166191711101831\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"VVNBR0UKICBidyBbR0xPQkFMIE9QVElPTlNdIDxjb21tYW5kPiBbY29tbWFuZCBvcHRpb25zIGFuZCBvcGVyYW5kc10KICBidyBoZWxwIFs8Y29tbWFuZD5dCgpHTE9CQUwgT1BUSU9OUwogIC0td29ya3NwYWNlIEFCU09MVVRFX1BBVEggIFNlbGVjdCBhbiBhYnNvbHV0ZSB3b3Jrc3BhY2UgYmVmb3JlIHRoZSBjb21tYW5kLgogIC0tYWRtaXQgTE9HSUNBTF9QQVRIICAgICAgIEFkbWl0IGEgbG9naWNhbCByb290IGJlZm9yZSB0aGUgY29tbWFuZDsgcmVwZWF0YWJsZS4KICAtLWpzb24gICAgICAgICAgICAgICAgICAgICBTZWxlY3QgSlNPTiBvdXRwdXQgd2hlcmUgdGhlIGNvbW1hbmQgc3VwcG9ydHMgaXQuCiAgLS1yYXcgICAgICAgICAgICAgICAgICAgICAgU2VsZWN0IHJhdyBWaWV3IG91dHB1dCBvbmx5LgoKQ09NTUFORFMKICBzaGVsbCAgICBSZXVzZSBzaG9ydCByZWZlcmVuY2VzIGFjcm9zcyBzZWFyY2gsIHZpZXcsIHJlcGxhY2UsIGFuZCBjaGVjay4KICBzZWFyY2ggICBEaXNjb3ZlciBjdXI=\",\"text\":\"USAGE\\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\\n  bw help [<command>]\\n\\nGLOBAL OPTIONS\\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\\n  --json                     Select JSON output where the command supports it.\\n  --raw                      Select raw View output only.\\n\\nCOMMANDS\\n  shell    Reuse short references across search, view, replace, and check.\\n  search   Discover cur\"},{\"sequence\":5,\"ns\":\"166191711133302\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"cmVudCBGaWxlLCBQYXJhZ3JhcGgsIG9yIExpbmUgQW5kZHJlc3Nlcy4KICB2aWV3ICAgICBSZWFkIG9uZSBvciBtb3JlIGN1cnJlbnQgQW5kZHJlc3Nlcy4KICBlZGl0ICAgICBSZXBsYWNlIG9uZSBjdXJyZW50IEFuZGRyZXNzLgogIGNoZWNrICAgIENoZWNrIG9uZSBvciBtb3JlIGN1cnJlbnQgQW5kZHJlc3Nlcy4KICB2ZXJzaW9uICBQcmludCB0aGUgQmFja3dyaXRlciB2ZXJzaW9uLgogIHVwZGF0ZSAgIFJ1biB0aGUgaW5zdGFsbGVkLXBsYXRmb3JtIHVwZGF0ZXIuCgpBRERJVElPTkFMIEhFTFAKICBidyBoZWxwIDxjb21tYW5kPgogIEFkdmFuY2VkIHRvcGljczogcGljaywgYW5jaG9yLCBhcHBseSwgZGF0YSAocmF3IFNlc3Npb24gb25seTsgbm8gb25lLXNob3QgZXhlY3V0aW9uKS4KCkdsb2JhbCBvcHRpb25zIHByZWNlZGUgdGhlIGNvbW1hbmQuIFVzZSBidyBoZWxwIHNoZWxsIGZvciBvcmRpbmFyeSBzaG9ydC1yZWYgd29yay4K\",\"text\":\"rent File, Paragraph, or Line Anddresses.\\n  view     Read one or more current Anddresses.\\n  edit     Replace one current Anddress.\\n  check    Check one or more current Anddresses.\\n  version  Print the Backwriter version.\\n  update   Run the installed-platform updater.\\n\\nADDITIONAL HELP\\n  bw help <command>\\n  Advanced topics: pick, anchor, apply, data (raw Session only; no one-shot execution).\\n\\nGlobal options precede the command. Use bw help shell for ordinary short-ref work.\\n\"},{\"sequence\":6,\"ns\":\"166191711171013\",\"kind\":\"stderr-eof\",\"processId\":1},{\"sequence\":7,\"ns\":\"166191711273221\",\"kind\":\"stdout-eof\",\"processId\":1},{\"sequence\":8,\"ns\":\"166191711324724\",\"kind\":\"close\",\"processId\":1,\"code\":0,\"signal\":null}],\"more\":false,\"active\":[]}\n"}
{"id":2,"bytes":5583,"sha256":"ed2a825ef54325bc16c4011291b17dd8793e34b7a280129a9b1096936d29b985","content":"{\"id\":2,\"events\":[{\"sequence\":9,\"ns\":\"166206028770444\",\"kind\":\"start\",\"processId\":2,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw\",\"help\",\"shell\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh\"},{\"sequence\":10,\"ns\":\"166206029895482\",\"kind\":\"stdin-eof\",\"processId\":2},{\"sequence\":11,\"ns\":\"166206029954499\",\"kind\":\"spawn\",\"processId\":2,\"pid\":2720340},{\"sequence\":12,\"ns\":\"166206030300197\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"TkFNRQogIGJ3IHNoZWxsIC0gcmV1c2Ugc2hvcnQgcmVmZXJlbmNlcyBhY3Jvc3Mgc2VhcmNoLCB2aWV3LCByZXBsYWNlLCBhbmQgY2hlY2sKClVTQUdFCiAgYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4gc2hlbGwKCkNPTU1BTkRTCiAgc2VhcmNoIDxsaW5lfHBhcmFncmFwaHxmaWxlPiA8cXVlcnk+IFstLXNvdXJjZSBQQVRIIHwgLS1zdWJ0cmVlIFBBVEhdLi4uCiAgc2VhcmNoIC9maWxlIDxsb2dpY2FsLXBhdGg+CiAgdmlldyA8UkVGPi4uLiBbLS1hcyA8bGluZXxwYXJhZ3JhcGh8ZmlsZT5dCiAgcmVwbGFjZSA8UkVGPiA8Q09OVEVOVD4KICBjaGVjayA8UkVGPi4uLgogIGxldCA8TkFNRT4gPSA8UkVGPgogIGV4aXQKClJFRkVSRU5DRVMgQU5EIENPTlRFTlQKICBATiBpcyBhIHplcm8tYmFzZWQsIGFwcGVuZC1vbmx5IHJlZmVyZW5jZSBmcm9tIHRoaXMgcHJvY2Vzcy4gSXQgZW5kcyBhdCBFT0Ygb3IgZXhpdC4KICBAbmFtZSBpcyBhIG5hbWU=\",\"text\":\"NAME\\n  bw shell - reuse short references across search, view, replace, and check\\n\\nUSAGE\\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\\n\\nCOMMANDS\\n  search <line|paragraph|file> <query> [--source PATH | --subtree PATH]...\\n  search /file <logical-path>\\n  view <REF>... [--as <line|paragraph|file>]\\n  replace <REF> <CONTENT>\\n  check <REF>...\\n  let <NAME> = <REF>\\n  exit\\n\\nREFERENCES AND CONTENT\\n  @N is a zero-based, append-only reference from this process. It ends at EOF or exit.\\n  @name is a name\"},{\"sequence\":13,\"ns\":\"166206030387411\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"ZCBBbmRkcmVzczsgQGhpdHNbMF0gc2VsZWN0cyBhbiBpbmRleGVkIFNlYXJjaC9QaWNrIGJpbmRpbmcuCiAgbGV0IG5hbWUgPSBATiBtYWtlcyBhIG5hbWVkIGFsaWFzLiBOYW1lcyBjYW5ub3QgYmUgcmVkZWZpbmVkLgogIERvdWJsZS1xdW90ZSBhcmd1bWVudHMgY29udGFpbmluZyBzcGFjZXMuIFF1b3RlZCBlc2NhcGVzIGFyZSBcXCwgXCIsIFxuLCBcciwgYW5kIFx0LgogIE9uZSBwaHlzaWNhbCBpbnB1dCBsaW5lIGlzIG9uZSBjb21tYW5kOyBubyBzaW5nbGUgcXVvdGVzLCBwaXBlcywgb3IgRU9GIENvbnRlbnQgc2VsZWN0b3IuCiAgTGluZSBSZXBsYWNlIGFjY2VwdHMgYm9keSBvbmx5OiBOVUwvQ1IvTEYgYXJlIHJlamVjdGVkOyB0aGUgZXhpc3RpbmcgdGVybWluYXRvciBpcyBwcmVzZXJ2ZWQuCiAgRmlsZS9QYXJhZ3JhcGggUmVwbGFjZSB1c2VzIGV4YWN0IENvbnRlbnQuIENoYW5nZWQgc291cmNlIGJ5dGVzIHN0YWxlIG9sZCBzYW1lLXNvdXJjZSByZWZzLAogIGluY2x1ZGluZyBvdGhlciA=\",\"text\":\"d Anddress; @hits[0] selects an indexed Search/Pick binding.\\n  let name = @N makes a named alias. Names cannot be redefined.\\n  Double-quote arguments containing spaces. Quoted escapes are \\\\\\\\, \\\\\\\", \\\\n, \\\\r, and \\\\t.\\n  One physical input line is one command; no single quotes, pipes, or EOF Content selector.\\n  Line Replace accepts body only: NUL/CR/LF are rejected; the existing terminator is preserved.\\n  File/Paragraph Replace uses exact Content. Changed source bytes stale old same-source refs,\\n  including other \"},{\"sequence\":14,\"ns\":\"166206030425641\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"bG9jYXRpb25zLiBVc2UgdGhlIGZyZXNoIHJlY2VpcHQgb3IgZXhwbGljaXRseSBzZWFyY2ggYWdhaW47IG5ldmVyIGF1dG8tcmV0cnkuCgpPVVRQVVQKICBTZWFyY2ggYXBwZW5kcyBvbmUgQE4gcGVyIHJlc3VsdDsgRW1wdHkgd3JpdGVzIG5vdGhpbmcuCiAgVmlldyByZXR1cm5zIGVhY2ggaW5wdXQgcmVmLCBhIGZyZXNoIHJlZiwga2luZC9sb2NhdGlvbiwgYW5kIGNvbXBsZXRlIENvbnRlbnQgaW4gaW5wdXQgb3JkZXIuCiAgVmlldzxUQUI+UkVGPFRBQj5ieXRlcz1OIHN0YXJ0cyBhIHJlY29yZCwgZm9sbG93ZWQgYnkgZnJlc2gtcmVmIG1ldGFkYXRhIGFuZCBOIGV4YWN0IGJ5dGVzLgogIFRoZSBmb2xsb3dpbmcgTEYgYW5kIEVuZFZpZXcgbGluZSBhcmUgZGlzcGxheSBmcmFtaW5nLCBub3Qgc291cmNlIENvbnRlbnQuCiAgUmVsYXRpb25BYnNlbnQgaXMgc2hvd24gYXQgaXRzIGlucHV0IHBvc2l0aW9uIHdpdGhvdXQgQ29udGVudCBvciBhIGZyZXNoIHNsb3QuCiAgUmVwbGFjZSB3cml0ZXMgVW4=\",\"text\":\"locations. Use the fresh receipt or explicitly search again; never auto-retry.\\n\\nOUTPUT\\n  Search appends one @N per result; Empty writes nothing.\\n  View returns each input ref, a fresh ref, kind/location, and complete Content in input order.\\n  View<TAB>REF<TAB>bytes=N starts a record, followed by fresh-ref metadata and N exact bytes.\\n  The following LF and EndView line are display framing, not source Content.\\n  RelationAbsent is shown at its input position without Content or a fresh slot.\\n  Replace writes Un\"},{\"sequence\":15,\"ns\":\"166206030447812\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"Y2hhbmdlZC9DaGFuZ2VkIHdpdGggYSBmcmVzaCByZWYsIG9yIENoYW5nZWQ8VEFCPk5vbmUgd2l0aG91dCBvbmUuCiAgQ2hlY2sgd3JpdGVzIG9uZSBzdGF0dXMgcGVyIGlucHV0OyBvbmx5IEN1cnJlbnQgYXBwZW5kcyBhIGZyZXNoIHJlZi4gQ2hlY2sgaXMgb3B0aW9uYWwuCgpFWEFNUExFUwogIFN0YXJ0IGJ3IHNoZWxsIGluIGEgd29ya3NwYWNlIHdpdGggb25seSBub3RlLnR4dCBjb250YWluaW5nIG5lZWRsZSBwbHVzIENSTEYsIHRoZW4gZW50ZXI6CiAgc2VhcmNoIGxpbmUgbmVlZGxlIC0tc291cmNlIG5vdGUudHh0CiAgdmlldyBAMCAtLWFzIHBhcmFncmFwaAogIGxldCBzZWxlY3RlZCA9IEAwCiAgcmVwbGFjZSBAc2VsZWN0ZWQgIm5ldyB2YWx1ZSIKICBjaGVjayBAMCBAMgogIHZpZXcgQDMKICBleGl0CgpBRFZBTkNFRAogIGxldCBoaXRzID0gc2VhcmNoIGxpbmUgbmVlZGxlCiAgdmlldyBhbmRkcmVzcyBAaGl0c1swXQogIFJhdyBuYW1lZCBWaWV3IHdyaXRlcyBleGFjdCBDb250ZW50IHc=\",\"text\":\"changed/Changed with a fresh ref, or Changed<TAB>None without one.\\n  Check writes one status per input; only Current appends a fresh ref. Check is optional.\\n\\nEXAMPLES\\n  Start bw shell in a workspace with only note.txt containing needle plus CRLF, then enter:\\n  search line needle --source note.txt\\n  view @0 --as paragraph\\n  let selected = @0\\n  replace @selected \\\"new value\\\"\\n  check @0 @2\\n  view @3\\n  exit\\n\\nADVANCED\\n  let hits = search line needle\\n  view anddress @hits[0]\\n  Raw named View writes exact Content w\"}],\"more\":true,\"active\":[]}\n"}
{"id":3,"bytes":1341,"sha256":"365de5cfaa92e8eba1bb44be1d22bf1e827a35f8fdc355c69107e523f3e8fc29","content":"{\"id\":3,\"events\":[{\"sequence\":16,\"ns\":\"166206030469956\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"aXRob3V0IGRpcmVjdC1WaWV3IGZyYW1pbmcuCiAgU2VlIGJ3IGhlbHAgcGljaywgYncgaGVscCBhbmNob3IsIGJ3IGhlbHAgYXBwbHksIGFuZCBidyBoZWxwIGRhdGEuCgpGQUlMVVJFUwogIFVzYWdlIGVycm9ycyByZWNvcmQgZXhpdCAyOyBSdW50aW1lL3NvdXJjZSBlcnJvcnMgcmVjb3JkIGV4aXQgMTsgbGF0ZXIgY29tbWFuZHMgc3RpbGwgcnVuLgogIFN0ZGluL3N0ZG91dCBmYWlsdXJlIGVuZHMgdGhlIHNoZWxsIHdpdGggZXhpdCAxLiBQYXJ0aWFsIG91dHB1dCBjYW5ub3QgcHJvdmUgZGVsaXZlcnkgb3IKICB1bmRvIHB1YmxpY2F0aW9uLiAtLWpzb24vLS1yYXcgYW5kIGNvbW1hbmQtbGluZSBvcGVyYW5kcyBhcmUgdW5hdmFpbGFibGUuCgpTRUUgQUxTTwogIGJ3IGhlbHAgc2VhcmNoCiAgYncgaGVscCBlZGl0Cg==\",\"text\":\"ithout direct-View framing.\\n  See bw help pick, bw help anchor, bw help apply, and bw help data.\\n\\nFAILURES\\n  Usage errors record exit 2; Runtime/source errors record exit 1; later commands still run.\\n  Stdin/stdout failure ends the shell with exit 1. Partial output cannot prove delivery or\\n  undo publication. --json/--raw and command-line operands are unavailable.\\n\\nSEE ALSO\\n  bw help search\\n  bw help edit\\n\"},{\"sequence\":17,\"ns\":\"166206030532071\",\"kind\":\"stdout-eof\",\"processId\":2},{\"sequence\":18,\"ns\":\"166206030626498\",\"kind\":\"stderr-eof\",\"processId\":2},{\"sequence\":19,\"ns\":\"166206030731293\",\"kind\":\"close\",\"processId\":2,\"code\":0,\"signal\":null}],\"more\":false,\"active\":[]}\n"}
```

#### Archived file: records/help-old/requests.jsonl

Bytes: 96; SHA-256: `f422d8c68d8d022e5d3e692af198ebb6eaeb6622dd889f0d5f28b471c897107f`.

```jsonl
{"ns":"166253564628836","request":{"arm":"help-old","op":"start","args":["--help"],"eof":true}}
```

#### Archived file: records/help-old/delivered.jsonl

Bytes: 3325; SHA-256: `a89c176a15a789da4db05bce54bedc53d08efc2631c5c9680bc94d0172eb67de`.

```jsonl
{"id":1,"bytes":3052,"sha256":"cc56340bb60cc4b72e4fb49c0766918d8904c0da9c5b3dd225b9c601485ed011","content":"{\"id\":1,\"events\":[{\"sequence\":1,\"ns\":\"166253564678186\",\"kind\":\"start\",\"processId\":1,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/bw-nminus\",\"--help\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh\"},{\"sequence\":2,\"ns\":\"166253565720478\",\"kind\":\"stdin-eof\",\"processId\":1},{\"sequence\":3,\"ns\":\"166253565785848\",\"kind\":\"spawn\",\"processId\":1,\"pid\":2720731},{\"sequence\":4,\"ns\":\"166253566058128\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"VVNBR0UKICBidyBbR0xPQkFMIE9QVElPTlNdIDxjb21tYW5kPiBbY29tbWFuZCBvcHRpb25zIGFuZCBvcGVyYW5kc10KICBidyBoZWxwIFs8Y29tbWFuZD5dCgpHTE9CQUwgT1BUSU9OUwogIC0td29ya3NwYWNlIEFCU09MVVRFX1BBVEggIFNlbGVjdCBhbiBhYnNvbHV0ZSB3b3Jrc3BhY2UgYmVmb3JlIHRoZSBjb21tYW5kLgogIC0tYWRtaXQgTE9HSUNBTF9QQVRIICAgICAgIEFkbWl0IGEgbG9naWNhbCByb290IGJlZm9yZSB0aGUgY29tbWFuZDsgcmVwZWF0YWJsZS4KICAtLWpzb24gICAgICAgICAgICAgICAgICAgICBTZWxlY3QgSlNPTiBvdXRwdXQgd2hlcmUgdGhlIGNvbW1hbmQgc3VwcG9ydHMgaXQuCiAgLS1yYXcgICAgICAgICAgICAgICAgICAgICAgU2VsZWN0IHJhdyBWaWV3IG91dHB1dCBvbmx5LgoKQ0FQQUJJTElUSUVTCiAgc2VhcmNoICAgRGlzY292ZXIgY3VycmVudCBGaWxlLCBQYXJhZ3JhcGgsIG9yIExpbmUgQW5kZHJlc3Nlcy4KICB2aWV3ICAgICBSZWFkIG9uZSBvciBtb3JlIGM=\",\"text\":\"USAGE\\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\\n  bw help [<command>]\\n\\nGLOBAL OPTIONS\\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\\n  --json                     Select JSON output where the command supports it.\\n  --raw                      Select raw View output only.\\n\\nCAPABILITIES\\n  search   Discover current File, Paragraph, or Line Anddresses.\\n  view     Read one or more c\"},{\"sequence\":5,\"ns\":\"166253566116946\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"dXJyZW50IEFuZGRyZXNzZXMuCiAgZWRpdCAgICAgUmVwbGFjZSBvbmUgY3VycmVudCBBbmRkcmVzcy4KICBjaGVjayAgICBDaGVjayBvbmUgb3IgbW9yZSBjdXJyZW50IEFuZGRyZXNzZXMuCiAgc2hlbGwgICAgUnVuIGFkdmFuY2VkIHJhdyBTZXNzaW9uIGNvbW1hbmRzLgogIHZlcnNpb24gIFByaW50IHRoZSBCYWNrd3JpdGVyIHZlcnNpb24uCiAgdXBkYXRlICAgUnVuIHRoZSBpbnN0YWxsZWQtcGxhdGZvcm0gdXBkYXRlci4KClBpY2ssIEFuY2hvciwgQXBwbHksIGFuZCBEYXRhIGhhdmUgbm8gb25lLXNob3QgY29tbWFuZDsgdXNlIGJ3IHNoZWxsLgoKQURESVRJT05BTCBIRUxQCiAgYncgaGVscCA8Y29tbWFuZD4KCkdsb2JhbCBvcHRpb25zIHByZWNlZGUgdGhlIGNvbW1hbmQuIENhbm9uaWNhbCBvdXRwdXQgb3B0aW9ucyBhcmUgZG9jdW1lbnRlZCBvbmx5IGluIHRoYXQgcG9zaXRpb24uCg==\",\"text\":\"urrent Anddresses.\\n  edit     Replace one current Anddress.\\n  check    Check one or more current Anddresses.\\n  shell    Run advanced raw Session commands.\\n  version  Print the Backwriter version.\\n  update   Run the installed-platform updater.\\n\\nPick, Anchor, Apply, and Data have no one-shot command; use bw shell.\\n\\nADDITIONAL HELP\\n  bw help <command>\\n\\nGlobal options precede the command. Canonical output options are documented only in that position.\\n\"},{\"sequence\":6,\"ns\":\"166253566173327\",\"kind\":\"stdout-eof\",\"processId\":1},{\"sequence\":7,\"ns\":\"166253566252720\",\"kind\":\"stderr-eof\",\"processId\":1},{\"sequence\":8,\"ns\":\"166253566358888\",\"kind\":\"close\",\"processId\":1,\"code\":0,\"signal\":null}],\"more\":false,\"active\":[]}\n"}
```


#### Archived file: records/dummy-old/requests.jsonl

Bytes: 1966; SHA-256: `6714bcae5c2f9fe1bd9d906fd34b8b2c84e037316e6a051cc89120ddae7b4615`.

```jsonl
{"ns":"166481366907538","request":{"arm":"dummy-old","op":"start","args":["--help"],"eof":true}}
{"ns":"166499290285566","request":{"arm":"dummy-old","op":"start","args":["help","shell"],"eof":true}}
{"ns":"166513276894239","request":{"arm":"dummy-old","op":"start","args":["shell"]}}
{"ns":"166527007072057","request":{"arm":"dummy-old","op":"send","processId":3,"stdin":"search line \"retry_budget = 3\"\n","eof":false}}
{"ns":"166540281511510","request":{"arm":"dummy-old","op":"send","processId":3,"stdin":"search paragraph \"retry_budget = 3\"\n","eof":false}}
{"ns":"166553713713592","request":{"arm":"dummy-old","op":"send","processId":3,"stdin":"view @8 @9 @10 @11 @12 @13 @14 @15\n","eof":false}}
{"ns":"166566756032430","request":{"arm":"dummy-old","op":"send","processId":3,"stdin":"help\n","eof":false}}
{"ns":"166579182953130","request":{"arm":"dummy-old","op":"start","args":["help","view"],"eof":true}}
{"ns":"166593495434625","request":{"arm":"dummy-old","op":"send","processId":3,"stdin":"let paras = search paragraph \"retry_budget = 3\"\nview anddress @paras[0]\nview anddress @paras[1]\nview anddress @paras[2]\nview anddress @paras[3]\nview anddress @paras[4]\nview anddress @paras[5]\nview anddress @paras[6]\nview anddress @paras[7]\n","eof":false}}
{"ns":"166610362540109","request":{"arm":"dummy-old","op":"start","args":["help","edit"],"eof":true}}
{"ns":"166625228474692","request":{"arm":"dummy-old","op":"send","processId":3,"stdin":"replace @0 \"retry_budget = 5\"\nreplace @3 \"retry_budget = 5\"\nreplace @4 \"retry_budget = 5\"\nreplace @7 \"retry_budget = 5\"\n","eof":false}}
{"ns":"166640660245150","request":{"arm":"dummy-old","op":"send","processId":3,"stdin":"check @24 @25 @26 @27 @1 @2 @5 @6\nlet files = search file \"retry_budget\"\nview anddress @files[0]\nview anddress @files[1]\nview anddress @files[2]\nview anddress @files[3]\nexit\n","eof":true}}
{"ns":"166653550284570","request":{"arm":"dummy-old","op":"finish"}}
```


#### Archived file: records/dummy-old/delivered.jsonl

Bytes: 29291; SHA-256: `5fc9847a3ff2ba035fef969dba09f56283b504c357560366e80e4dd9003583cd`.

```jsonl
{"id":1,"bytes":3072,"sha256":"12eae4bb798997a31ed533c950aba3542c250483d10863a1d41d9628b94a1862","content":"{\"id\":1,\"events\":[{\"sequence\":1,\"ns\":\"166481367033280\",\"kind\":\"start\",\"processId\":1,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/bw-nminus\",\"--help\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh/dummy-old/workspace\"},{\"sequence\":2,\"ns\":\"166481368824387\",\"kind\":\"stdin-eof\",\"processId\":1},{\"sequence\":3,\"ns\":\"166481369105438\",\"kind\":\"spawn\",\"processId\":1,\"pid\":2722568},{\"sequence\":4,\"ns\":\"166481369594001\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"VVNBR0UKICBidyBbR0xPQkFMIE9QVElPTlNdIDxjb21tYW5kPiBbY29tbWFuZCBvcHRpb25zIGFuZCBvcGVyYW5kc10KICBidyBoZWxwIFs8Y29tbWFuZD5dCgpHTE9CQUwgT1BUSU9OUwogIC0td29ya3NwYWNlIEFCU09MVVRFX1BBVEggIFNlbGVjdCBhbiBhYnNvbHV0ZSB3b3Jrc3BhY2UgYmVmb3JlIHRoZSBjb21tYW5kLgogIC0tYWRtaXQgTE9HSUNBTF9QQVRIICAgICAgIEFkbWl0IGEgbG9naWNhbCByb290IGJlZm9yZSB0aGUgY29tbWFuZDsgcmVwZWF0YWJsZS4KICAtLWpzb24gICAgICAgICAgICAgICAgICAgICBTZWxlY3QgSlNPTiBvdXRwdXQgd2hlcmUgdGhlIGNvbW1hbmQgc3VwcG9ydHMgaXQuCiAgLS1yYXcgICAgICAgICAgICAgICAgICAgICAgU2VsZWN0IHJhdyBWaWV3IG91dHB1dCBvbmx5LgoKQ0FQQUJJTElUSUVTCiAgc2VhcmNoICAgRGlzY292ZXIgY3VycmVudCBGaWxlLCBQYXJhZ3JhcGgsIG9yIExpbmUgQW5kZHJlc3Nlcy4KICB2aWV3ICAgICBSZWFkIG9uZSBvciBtb3JlIGM=\",\"text\":\"USAGE\\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\\n  bw help [<command>]\\n\\nGLOBAL OPTIONS\\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\\n  --json                     Select JSON output where the command supports it.\\n  --raw                      Select raw View output only.\\n\\nCAPABILITIES\\n  search   Discover current File, Paragraph, or Line Anddresses.\\n  view     Read one or more c\"},{\"sequence\":5,\"ns\":\"166481369640321\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"dXJyZW50IEFuZGRyZXNzZXMuCiAgZWRpdCAgICAgUmVwbGFjZSBvbmUgY3VycmVudCBBbmRkcmVzcy4KICBjaGVjayAgICBDaGVjayBvbmUgb3IgbW9yZSBjdXJyZW50IEFuZGRyZXNzZXMuCiAgc2hlbGwgICAgUnVuIGFkdmFuY2VkIHJhdyBTZXNzaW9uIGNvbW1hbmRzLgogIHZlcnNpb24gIFByaW50IHRoZSBCYWNrd3JpdGVyIHZlcnNpb24uCiAgdXBkYXRlICAgUnVuIHRoZSBpbnN0YWxsZWQtcGxhdGZvcm0gdXBkYXRlci4KClBpY2ssIEFuY2hvciwgQXBwbHksIGFuZCBEYXRhIGhhdmUgbm8gb25lLXNob3QgY29tbWFuZDsgdXNlIGJ3IHNoZWxsLgoKQURESVRJT05BTCBIRUxQCiAgYncgaGVscCA8Y29tbWFuZD4KCkdsb2JhbCBvcHRpb25zIHByZWNlZGUgdGhlIGNvbW1hbmQuIENhbm9uaWNhbCBvdXRwdXQgb3B0aW9ucyBhcmUgZG9jdW1lbnRlZCBvbmx5IGluIHRoYXQgcG9zaXRpb24uCg==\",\"text\":\"urrent Anddresses.\\n  edit     Replace one current Anddress.\\n  check    Check one or more current Anddresses.\\n  shell    Run advanced raw Session commands.\\n  version  Print the Backwriter version.\\n  update   Run the installed-platform updater.\\n\\nPick, Anchor, Apply, and Data have no one-shot command; use bw shell.\\n\\nADDITIONAL HELP\\n  bw help <command>\\n\\nGlobal options precede the command. Canonical output options are documented only in that position.\\n\"},{\"sequence\":6,\"ns\":\"166481369752613\",\"kind\":\"stderr-eof\",\"processId\":1},{\"sequence\":7,\"ns\":\"166481369956642\",\"kind\":\"stdout-eof\",\"processId\":1},{\"sequence\":8,\"ns\":\"166481370028299\",\"kind\":\"close\",\"processId\":1,\"code\":0,\"signal\":null}],\"more\":false,\"active\":[]}\n"}
{"id":2,"bytes":3827,"sha256":"4428c654912341ca666eedc87b3f8a241baa5bb5e0d53043b6d1ec6e3b494ca3","content":"{\"id\":2,\"events\":[{\"sequence\":9,\"ns\":\"166499290336647\",\"kind\":\"start\",\"processId\":2,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/bw-nminus\",\"help\",\"shell\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh/dummy-old/workspace\"},{\"sequence\":10,\"ns\":\"166499291390437\",\"kind\":\"stdin-eof\",\"processId\":2},{\"sequence\":11,\"ns\":\"166499291460048\",\"kind\":\"spawn\",\"processId\":2,\"pid\":2723172},{\"sequence\":12,\"ns\":\"166499291611159\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"TkFNRQogIGJ3IHNoZWxsIC0gcnVuIG9uZSBsb2NhbCByZWZlcmVuY2Ugc2Vzc2lvbiBhbmQgYWR2YW5jZWQgcmF3IFNlc3Npb24gY29tbWFuZHMKClVTQUdFCiAgYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4gc2hlbGwKCkRFU0NSSVBUSU9OCiAgUmVhZHMgY29tbWFuZHMgZnJvbSBzdGFuZGFyZCBpbnB1dCB1bnRpbCBleGl0LiBEaXJlY3Qgc2VhcmNoLCB2aWV3LCByZXBsYWNlLCBhbmQgY2hlY2sgdXNlIHNlc3Npb24tbG9jYWwgbnVtZXJpYyBBbmRkcmVzcyByZWZlcmVuY2VzLiBSYXcgYmluZGluZ3MgYW5kIHJhdyBjYXBhYmlsaXR5IGNvbXBvc2l0aW9uIHJlbWFpbiB0aGUgYWR2YW5jZWQgc3VyZmFjZS4KCkFSR1VNRU5UUwogIE5vbmUuCgpPUFRJT05TCiAgLS13b3Jrc3BhY2UgYW5kIC0tYWRtaXQgbXVzdCBwcmVjZWRlIHNoZWxsLgogIC0tanNvbiBhbmQgLS1yYXcgYXJlIHVuYXZhaWxhYmxlLgoKV0hBVCBIQVBQRU5TCiAgQSBzdWNjZXM=\",\"text\":\"NAME\\n  bw shell - run one local reference session and advanced raw Session commands\\n\\nUSAGE\\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\\n\\nDESCRIPTION\\n  Reads commands from standard input until exit. Direct search, view, replace, and check use session-local numeric Anddress references. Raw bindings and raw capability composition remain the advanced surface.\\n\\nARGUMENTS\\n  None.\\n\\nOPTIONS\\n  --workspace and --admit must precede shell.\\n  --json and --raw are unavailable.\\n\\nWHAT HAPPENS\\n  A succes\"},{\"sequence\":13,\"ns\":\"166499291643127\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"c2Z1bCBkaXJlY3Qgc2VhcmNoLCB2aWV3LCBvciBjdXJyZW50IGNoZWNrIGVtaXRzIGFwcGVuZC1vbmx5IEBOIHJlZmVyZW5jZXMuIERpcmVjdCByZXBsYWNlIHVzZXMgb25lIHJlZmVyZW5jZSBhbmQgZW1pdHMgYSBmcmVzaCByZWZlcmVuY2Ugd2hlbiBvbmUgZXhpc3RzLiBSZWZlcmVuY2VzIGVuZCB3aXRoIHRoaXMgc2hlbGwgcHJvY2Vzcy4gUmF3IGxldCwgUGljaywgVmlldywgQ2hlY2ssIEFuY2hvciwgRWRpdCwgQXBwbHksIGFuZCBEYXRhIHJldGFpbiB0aGVpciBleGlzdGluZyBncmFtbWFyLgoKT1VUUFVUCiAgRGlyZWN0IHJlZmVyZW5jZXMgd3JpdGUgQE4sIHRhcmdldCBraW5kLCBhbmQgbG9jYXRpb24uIERpcmVjdCBDaGVjayB3cml0ZXMgb25lIHN0YXRlIHBlciBpbnB1dC4gUmF3IGNvbW1hbmRzIHdyaXRlIHRoZWlyIGV4aXN0aW5nIGh1bWFuIHJlc3VsdC4KCkVYQU1QTEVTCiAgYncgc2hlbGwKICBzZWFyY2ggbGluZSBuZWVkbGUKICB2aWV3IEAwCiAgcmVwbGFjZSBAMSByZXBsYWNlbWU=\",\"text\":\"sful direct search, view, or current check emits append-only @N references. Direct replace uses one reference and emits a fresh reference when one exists. References end with this shell process. Raw let, Pick, View, Check, Anchor, Edit, Apply, and Data retain their existing grammar.\\n\\nOUTPUT\\n  Direct references write @N, target kind, and location. Direct Check writes one state per input. Raw commands write their existing human result.\\n\\nEXAMPLES\\n  bw shell\\n  search line needle\\n  view @0\\n  replace @1 replaceme\"},{\"sequence\":14,\"ns\":\"166499291654629\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"bnQKICBjaGVjayBAMiBAMwogIGxldCBoaXRzID0gc2VhcmNoIGxpbmUgbmVlZGxlCiAgdmlldyBhbmRkcmVzcyBAaGl0c1swXQogIGV4aXQKCkZBSUxVUkVTCiAgSW52YWxpZCBzaGVsbCBncmFtbWFyIGlzIGEgdXNhZ2UgZmFpbHVyZS4gUnVudGltZSBhbmQgc291cmNlIGZhaWx1cmVzIGV4aXQgMS4KClNFRSBBTFNPCiAgYncgaGVscCBzZWFyY2gKICBidyBoZWxwIGVkaXQK\",\"text\":\"nt\\n  check @2 @3\\n  let hits = search line needle\\n  view anddress @hits[0]\\n  exit\\n\\nFAILURES\\n  Invalid shell grammar is a usage failure. Runtime and source failures exit 1.\\n\\nSEE ALSO\\n  bw help search\\n  bw help edit\\n\"},{\"sequence\":15,\"ns\":\"166499291688041\",\"kind\":\"stdout-eof\",\"processId\":2},{\"sequence\":16,\"ns\":\"166499291732571\",\"kind\":\"stderr-eof\",\"processId\":2},{\"sequence\":17,\"ns\":\"166499291791914\",\"kind\":\"close\",\"processId\":2,\"code\":0,\"signal\":null}],\"more\":false,\"active\":[]}\n"}
{"id":3,"bytes":313,"sha256":"8bf9f92a82cafd4cae9246d2960070b07c7020dffa803cdb907030d480da7886","content":"{\"id\":3,\"events\":[{\"sequence\":18,\"ns\":\"166513276949953\",\"kind\":\"start\",\"processId\":3,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/bw-nminus\",\"shell\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh/dummy-old/workspace\"},{\"sequence\":19,\"ns\":\"166513278031367\",\"kind\":\"spawn\",\"processId\":3,\"pid\":2723573}],\"more\":false,\"active\":[3]}\n"}
{"id":4,"bytes":741,"sha256":"f712a601f1c9be874d76e4b30317fb47705cb8b6db50a445bcc5e005bccf080c","content":"{\"id\":4,\"events\":[{\"sequence\":20,\"ns\":\"166527007126580\",\"kind\":\"stdin\",\"processId\":3,\"base64\":\"c2VhcmNoIGxpbmUgInJldHJ5X2J1ZGdldCA9IDMiCg==\",\"text\":\"search line \\\"retry_budget = 3\\\"\\n\"},{\"sequence\":21,\"ns\":\"166527007250349\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"QDAJTGluZQl1bml0LTAxLnR4dDoyCkAxCUxpbmUJdW5pdC0wMS50eHQ6NQpAMglMaW5lCXVuaXQtMDIudHh0OjIKQDMJTGluZQl1bml0LTAyLnR4dDo1CkA0CUxpbmUJdW5pdC0wMy50eHQ6MgpANQlMaW5lCXVuaXQtMDMudHh0OjUKQDYJTGluZQl1bml0LTA0LnR4dDoyCkA3CUxpbmUJdW5pdC0wNC50eHQ6NQo=\",\"text\":\"@0\\tLine\\tunit-01.txt:2\\n@1\\tLine\\tunit-01.txt:5\\n@2\\tLine\\tunit-02.txt:2\\n@3\\tLine\\tunit-02.txt:5\\n@4\\tLine\\tunit-03.txt:2\\n@5\\tLine\\tunit-03.txt:5\\n@6\\tLine\\tunit-04.txt:2\\n@7\\tLine\\tunit-04.txt:5\\n\"}],\"more\":false,\"active\":[3]}\n"}
{"id":5,"bytes":896,"sha256":"2bc4824bb0c9191e7f81dce287667c666e205c64f4fe2536b3bb62f82edb4e1f","content":"{\"id\":5,\"events\":[{\"sequence\":22,\"ns\":\"166540281559176\",\"kind\":\"stdin\",\"processId\":3,\"base64\":\"c2VhcmNoIHBhcmFncmFwaCAicmV0cnlfYnVkZ2V0ID0gMyIK\",\"text\":\"search paragraph \\\"retry_budget = 3\\\"\\n\"},{\"sequence\":23,\"ns\":\"166540281662957\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"QDgJUGFyYWdyYXBoCXVuaXQtMDEudHh0OjEtMgpAOQlQYXJhZ3JhcGgJdW5pdC0wMS50eHQ6NC01CkAxMAlQYXJhZ3JhcGgJdW5pdC0wMi50eHQ6MS0yCkAxMQlQYXJhZ3JhcGgJdW5pdC0wMi50eHQ6NC01CkAxMglQYXJhZ3JhcGgJdW5pdC0wMy50eHQ6MS0yCkAxMwlQYXJhZ3JhcGgJdW5pdC0wMy50eHQ6NC01CkAxNAlQYXJhZ3JhcGgJdW5pdC0wNC50eHQ6MS0yCkAxNQlQYXJhZ3JhcGgJdW5pdC0wNC50eHQ6NC01Cg==\",\"text\":\"@8\\tParagraph\\tunit-01.txt:1-2\\n@9\\tParagraph\\tunit-01.txt:4-5\\n@10\\tParagraph\\tunit-02.txt:1-2\\n@11\\tParagraph\\tunit-02.txt:4-5\\n@12\\tParagraph\\tunit-03.txt:1-2\\n@13\\tParagraph\\tunit-03.txt:4-5\\n@14\\tParagraph\\tunit-04.txt:1-2\\n@15\\tParagraph\\tunit-04.txt:4-5\\n\"}],\"more\":false,\"active\":[3]}\n"}
{"id":6,"bytes":895,"sha256":"38ef792eb8bdfd1c1abe05d9904abadad6ef6ca2a571389e3c9707971b91ef3e","content":"{\"id\":6,\"events\":[{\"sequence\":24,\"ns\":\"166553713778526\",\"kind\":\"stdin\",\"processId\":3,\"base64\":\"dmlldyBAOCBAOSBAMTAgQDExIEAxMiBAMTMgQDE0IEAxNQo=\",\"text\":\"view @8 @9 @10 @11 @12 @13 @14 @15\\n\"},{\"sequence\":25,\"ns\":\"166553713937243\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"QDE2CVBhcmFncmFwaAl1bml0LTAxLnR4dDoxLTIKQDE3CVBhcmFncmFwaAl1bml0LTAxLnR4dDo0LTUKQDE4CVBhcmFncmFwaAl1bml0LTAyLnR4dDoxLTIKQDE5CVBhcmFncmFwaAl1bml0LTAyLnR4dDo0LTUKQDIwCVBhcmFncmFwaAl1bml0LTAzLnR4dDoxLTIKQDIxCVBhcmFncmFwaAl1bml0LTAzLnR4dDo0LTUKQDIyCVBhcmFncmFwaAl1bml0LTA0LnR4dDoxLTIKQDIzCVBhcmFncmFwaAl1bml0LTA0LnR4dDo0LTUK\",\"text\":\"@16\\tParagraph\\tunit-01.txt:1-2\\n@17\\tParagraph\\tunit-01.txt:4-5\\n@18\\tParagraph\\tunit-02.txt:1-2\\n@19\\tParagraph\\tunit-02.txt:4-5\\n@20\\tParagraph\\tunit-03.txt:1-2\\n@21\\tParagraph\\tunit-03.txt:4-5\\n@22\\tParagraph\\tunit-04.txt:1-2\\n@23\\tParagraph\\tunit-04.txt:4-5\\n\"}],\"more\":false,\"active\":[3]}\n"}
{"id":7,"bytes":2708,"sha256":"2503c7a68ae063b9c19dd39dc7be4cc3f03e86246b7bef6a609257a1e25da328","content":"{\"id\":7,\"events\":[{\"sequence\":26,\"ns\":\"166566756085098\",\"kind\":\"stdin\",\"processId\":3,\"base64\":\"aGVscAo=\",\"text\":\"help\\n\"},{\"sequence\":27,\"ns\":\"166566756184668\",\"kind\":\"stderr\",\"processId\":3,\"base64\":\"ZXJyb3I6IHVuc3VwcG9ydGVkIFNlc3Npb24gY29tbWFuZDogaGVscAoKVVNBR0UKICBidyBbR0xPQkFMIE9QVElPTlNdIDxjb21tYW5kPiBbY29tbWFuZCBvcHRpb25zIGFuZCBvcGVyYW5kc10KICBidyBoZWxwIFs8Y29tbWFuZD5dCgpHTE9CQUwgT1BUSU9OUwogIC0td29ya3NwYWNlIEFCU09MVVRFX1BBVEggIFNlbGVjdCBhbiBhYnNvbHV0ZSB3b3Jrc3BhY2UgYmVmb3JlIHRoZSBjb21tYW5kLgogIC0tYWRtaXQgTE9HSUNBTF9QQVRIICAgICAgIEFkbWl0IGEgbG9naWNhbCByb290IGJlZm9yZSB0aGUgY29tbWFuZDsgcmVwZWF0YWJsZS4KICAtLWpzb24gICAgICAgICAgICAgICAgICAgICBTZWxlY3QgSlNPTiBvdXRwdXQgd2hlcmUgdGhlIGNvbW1hbmQgc3VwcG9ydHMgaXQuCiAgLS1yYXcgICAgICAgICAgICAgICAgICAgICAgU2VsZWN0IHJhdyBWaWV3IG91dHB1dCBvbmx5LgoKQ0FQQUJJTElUSUVTCiAgc2VhcmNoICAgRGlzY292ZXIgY3VycmVudCBGaWxlLCBQYXJhZ3JhcGgsIG9yIExpbmU=\",\"text\":\"error: unsupported Session command: help\\n\\nUSAGE\\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\\n  bw help [<command>]\\n\\nGLOBAL OPTIONS\\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\\n  --json                     Select JSON output where the command supports it.\\n  --raw                      Select raw View output only.\\n\\nCAPABILITIES\\n  search   Discover current File, Paragraph, or Line\"},{\"sequence\":28,\"ns\":\"166566756201838\",\"kind\":\"stderr\",\"processId\":3,\"base64\":\"IEFuZGRyZXNzZXMuCiAgdmlldyAgICAgUmVhZCBvbmUgb3IgbW9yZSBjdXJyZW50IEFuZGRyZXNzZXMuCiAgZWRpdCAgICAgUmVwbGFjZSBvbmUgY3VycmVudCBBbmRkcmVzcy4KICBjaGVjayAgICBDaGVjayBvbmUgb3IgbW9yZSBjdXJyZW50IEFuZGRyZXNzZXMuCiAgc2hlbGwgICAgUnVuIGFkdmFuY2VkIHJhdyBTZXNzaW9uIGNvbW1hbmRzLgogIHZlcnNpb24gIFByaW50IHRoZSBCYWNrd3JpdGVyIHZlcnNpb24uCiAgdXBkYXRlICAgUnVuIHRoZSBpbnN0YWxsZWQtcGxhdGZvcm0gdXBkYXRlci4KClBpY2ssIEFuY2hvciwgQXBwbHksIGFuZCBEYXRhIGhhdmUgbm8gb25lLXNob3QgY29tbWFuZDsgdXNlIGJ3IHNoZWxsLgoKQURESVRJT05BTCBIRUxQCiAgYncgaGVscCA8Y29tbWFuZD4KCkdsb2JhbCBvcHRpb25zIHByZWNlZGUgdGhlIGNvbW1hbmQuIENhbm9uaWNhbCBvdXRwdXQgb3B0aW9ucyBhcmUgZG9jdW1lbnRlZCBvbmx5IGluIHRoYXQgcG9zaXRpb24uCg==\",\"text\":\" Anddresses.\\n  view     Read one or more current Anddresses.\\n  edit     Replace one current Anddress.\\n  check    Check one or more current Anddresses.\\n  shell    Run advanced raw Session commands.\\n  version  Print the Backwriter version.\\n  update   Run the installed-platform updater.\\n\\nPick, Anchor, Apply, and Data have no one-shot command; use bw shell.\\n\\nADDITIONAL HELP\\n  bw help <command>\\n\\nGlobal options precede the command. Canonical output options are documented only in that position.\\n\"}],\"more\":false,\"active\":[3]}\n"}
{"id":8,"bytes":3881,"sha256":"70b54bab062055bc0a799943fa01abc44eea8b1258c057ea36bf56060527a5e3","content":"{\"id\":8,\"events\":[{\"sequence\":29,\"ns\":\"166579183007902\",\"kind\":\"start\",\"processId\":4,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/bw-nminus\",\"help\",\"view\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh/dummy-old/workspace\"},{\"sequence\":30,\"ns\":\"166579184140375\",\"kind\":\"stdin-eof\",\"processId\":4},{\"sequence\":31,\"ns\":\"166579184203368\",\"kind\":\"spawn\",\"processId\":4,\"pid\":2725515},{\"sequence\":32,\"ns\":\"166579184564047\",\"kind\":\"stdout\",\"processId\":4,\"base64\":\"TkFNRQogIGJ3IHZpZXcgLSBwcm9qZWN0IGN1cnJlbnQgY29udGVudCBmcm9tIG9uZSBvciBtb3JlIHY1IEFuZGRyZXNzZXMKClVTQUdFCiAgYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4gWy0tanNvbnwtLXJhd10gdmlldyBhbmRkcmVzcyA8ZW5jb2RlZC12NS1BbmRkcmVzcz4gWy0tYXMgPGxpbmV8cGFyYWdyYXBofGZpbGU+XQogIGJ3IFstLXdvcmtzcGFjZSBBQlNPTFVURV9QQVRIXSBbLS1hZG1pdCBMT0dJQ0FMX1BBVEhdLi4uIC0tanNvbiB2aWV3IGFuZGRyZXNzIDxlbmNvZGVkLXY1LUFuZGRyZXNzPi4uLiAtLWFzIDxsaW5lfHBhcmFncmFwaHxmaWxlPgoKREVTQ1JJUFRJT04KICBWYWxpZGF0ZXMgY3VycmVudCBzb3VyY2Ugc3RhdGUgYW5kIHByb2plY3RzIHRoZSByZXF1ZXN0ZWQgdGFyZ2V0IHJlbGF0aW9uIGZyb20gY2FsbGVyLXByb3ZpZGVkIHY1IEFuZGRyZXNzZXMuCgpBUkdVTUVOVFMKICBhbmRkcmVzcyAgICAgICAgICAgICAgICA=\",\"text\":\"NAME\\n  bw view - project current content from one or more v5 Anddresses\\n\\nUSAGE\\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json|--raw] view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json view anddress <encoded-v5-Anddress>... --as <line|paragraph|file>\\n\\nDESCRIPTION\\n  Validates current source state and projects the requested target relation from caller-provided v5 Anddresses.\\n\\nARGUMENTS\\n  anddress                \"},{\"sequence\":33,\"ns\":\"166579184609241\",\"kind\":\"stdout\",\"processId\":4,\"base64\":\"ICBSZXF1aXJlZCBpbnB1dCBmb3JtLgogIDxlbmNvZGVkLXY1LUFuZGRyZXNzPiAgICAgT25lIG9yIG1vcmUgY2Fub25pY2FsIHY1IG9iamVjdHMuCgpPUFRJT05TCiAgLS13b3Jrc3BhY2UsIC0tYWRtaXQsIC0tanNvbiwgYW5kIC0tcmF3IG11c3QgcHJlY2VkZSB2aWV3LgogIC0tYXMgc2VsZWN0cyBsaW5lLCBwYXJhZ3JhcGgsIG9yIGZpbGUgYW5kIG11c3QgYmUgbGFzdC4gQmF0Y2ggVmlldyByZXF1aXJlcyAtLWpzb24gYW5kIC0tYXMuCgpXSEFUIEhBUFBFTlMKICBPcGVucyB0aGUgUnVudGltZSBhZnRlciBpbnB1dCB2YWxpZGF0aW9uIGFuZCByZXR1cm5zIHRoZSByZXF1ZXN0ZWQgY3VycmVudCBwcm9qZWN0aW9uLgoKT1VUUFVUCiAgT25lIGh1bWFuIG9yIHJhdyBWaWV3IHdyaXRlcyBjb250ZW50LiBKU09OIHdyaXRlcyB0aGUgZml4ZWQgYncuY2xpLnZpZXcudjIgZW52ZWxvcGUuCgpFWEFNUExFUwogIGJ3IHZpZXcgYW5kZHJlc3MgJzx2NS1BbmRkcmVzcz4nCiAgYncgLS1yYXcgdmlldyBhbmQ=\",\"text\":\"  Required input form.\\n  <encoded-v5-Anddress>     One or more canonical v5 objects.\\n\\nOPTIONS\\n  --workspace, --admit, --json, and --raw must precede view.\\n  --as selects line, paragraph, or file and must be last. Batch View requires --json and --as.\\n\\nWHAT HAPPENS\\n  Opens the Runtime after input validation and returns the requested current projection.\\n\\nOUTPUT\\n  One human or raw View writes content. JSON writes the fixed bw.cli.view.v2 envelope.\\n\\nEXAMPLES\\n  bw view anddress '<v5-Anddress>'\\n  bw --raw view and\"},{\"sequence\":34,\"ns\":\"166579184629641\",\"kind\":\"stdout\",\"processId\":4,\"base64\":\"ZHJlc3MgJzx2NS1MaW5lLUFuZGRyZXNzPicKICBidyAtLWpzb24gdmlldyBhbmRkcmVzcyAnPHY1LUFuZGRyZXNzPicgLS1hcyBwYXJhZ3JhcGgKCkZBSUxVUkVTCiAgSW52YWxpZCBpbnB1dCBvciB1bnN1cHBvcnRlZCBvdXRwdXQgZm9ybSBpcyBhIHVzYWdlIGZhaWx1cmUuIFVuYXZhaWxhYmxlIG9yIHN0YWxlIHNvdXJjZSBleGl0cyAxLgoKU0VFIEFMU08KICBidyBoZWxwIHNlYXJjaAogIGJ3IGhlbHAgY2hlY2sK\",\"text\":\"dress '<v5-Line-Anddress>'\\n  bw --json view anddress '<v5-Anddress>' --as paragraph\\n\\nFAILURES\\n  Invalid input or unsupported output form is a usage failure. Unavailable or stale source exits 1.\\n\\nSEE ALSO\\n  bw help search\\n  bw help check\\n\"},{\"sequence\":35,\"ns\":\"166579184684344\",\"kind\":\"stdout-eof\",\"processId\":4},{\"sequence\":36,\"ns\":\"166579184763796\",\"kind\":\"stderr-eof\",\"processId\":4},{\"sequence\":37,\"ns\":\"166579184864806\",\"kind\":\"close\",\"processId\":4,\"code\":0,\"signal\":null}],\"more\":false,\"active\":[3]}\n"}
{"id":9,"bytes":2100,"sha256":"3ab893f46529e965256ae7a6a6ae41b4a31775e09d8fc830a98d07b90e73c575","content":"{\"id\":9,\"events\":[{\"sequence\":38,\"ns\":\"166593495484783\",\"kind\":\"stdin\",\"processId\":3,\"base64\":\"bGV0IHBhcmFzID0gc2VhcmNoIHBhcmFncmFwaCAicmV0cnlfYnVkZ2V0ID0gMyIKdmlldyBhbmRkcmVzcyBAcGFyYXNbMF0KdmlldyBhbmRkcmVzcyBAcGFyYXNbMV0KdmlldyBhbmRkcmVzcyBAcGFyYXNbMl0KdmlldyBhbmRkcmVzcyBAcGFyYXNbM10KdmlldyBhbmRkcmVzcyBAcGFyYXNbNF0KdmlldyBhbmRkcmVzcyBAcGFyYXNbNV0KdmlldyBhbmRkcmVzcyBAcGFyYXNbNl0KdmlldyBhbmRkcmVzcyBAcGFyYXNbN10K\",\"text\":\"let paras = search paragraph \\\"retry_budget = 3\\\"\\nview anddress @paras[0]\\nview anddress @paras[1]\\nview anddress @paras[2]\\nview anddress @paras[3]\\nview anddress @paras[4]\\nview anddress @paras[5]\\nview anddress @paras[6]\\nview anddress @paras[7]\\n\"},{\"sequence\":39,\"ns\":\"166593495639963\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"Rm91bmQgOAowCVBhcmFncmFwaAl1bml0LTAxLnR4dDoxLTIKMQlQYXJhZ3JhcGgJdW5pdC0wMS50eHQ6NC01CjIJUGFyYWdyYXBoCXVuaXQtMDIudHh0OjEtMgozCVBhcmFncmFwaAl1bml0LTAyLnR4dDo0LTUKNAlQYXJhZ3JhcGgJdW5pdC0wMy50eHQ6MS0yCjUJUGFyYWdyYXBoCXVuaXQtMDMudHh0OjQtNQo2CVBhcmFncmFwaAl1bml0LTA0LnR4dDoxLTIKNwlQYXJhZ3JhcGgJdW5pdC0wNC50eHQ6NC01Cg==\",\"text\":\"Found 8\\n0\\tParagraph\\tunit-01.txt:1-2\\n1\\tParagraph\\tunit-01.txt:4-5\\n2\\tParagraph\\tunit-02.txt:1-2\\n3\\tParagraph\\tunit-02.txt:4-5\\n4\\tParagraph\\tunit-03.txt:1-2\\n5\\tParagraph\\tunit-03.txt:4-5\\n6\\tParagraph\\tunit-04.txt:1-2\\n7\\tParagraph\\tunit-04.txt:4-5\\n\"},{\"sequence\":40,\"ns\":\"166593495666555\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"cm9sZSA9IHByaW1hcnkKcmV0cnlfYnVkZ2V0ID0gMwpyb2xlID0gc2Vjb25kYXJ5CnJldHJ5X2J1ZGdldCA9IDMKcm9sZSA9IHNlY29uZGFyeQ1yZXRyeV9idWRnZXQgPSAzDXJvbGUgPSBwcmltYXJ5DXJldHJ5X2J1ZGdldCA9IDMNcm9sZSA9IHByaW1hcnkNCnJldHJ5X2J1ZGdldCA9IDMNCnJvbGUgPSBzZWNvbmRhcnkNCnJldHJ5X2J1ZGdldCA9IDMNCnJvbGUgPSBzZWNvbmRhcnkKcmV0cnlfYnVkZ2V0ID0gMwpyb2xlID0gcHJpbWFyeQpyZXRyeV9idWRnZXQgPSAz\",\"text\":\"role = primary\\nretry_budget = 3\\nrole = secondary\\nretry_budget = 3\\nrole = secondary\\rretry_budget = 3\\rrole = primary\\rretry_budget = 3\\rrole = primary\\r\\nretry_budget = 3\\r\\nrole = secondary\\r\\nretry_budget = 3\\r\\nrole = secondary\\nretry_budget = 3\\nrole = primary\\nretry_budget = 3\"}],\"more\":false,\"active\":[3]}\n"}
{"id":10,"bytes":4176,"sha256":"89a32908a3cc4623532b4039fcb2f05a518f04e88bca382bac4ffb752f3293e2","content":"{\"id\":10,\"events\":[{\"sequence\":41,\"ns\":\"166610362576778\",\"kind\":\"start\",\"processId\":5,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/bw-nminus\",\"help\",\"edit\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh/dummy-old/workspace\"},{\"sequence\":42,\"ns\":\"166610363606751\",\"kind\":\"stdin-eof\",\"processId\":5},{\"sequence\":43,\"ns\":\"166610363665341\",\"kind\":\"spawn\",\"processId\":5,\"pid\":2726417},{\"sequence\":44,\"ns\":\"166610363966584\",\"kind\":\"stdout\",\"processId\":5,\"base64\":\"TkFNRQogIGJ3IGVkaXQgLSByZXBsYWNlIG9uZSBjdXJyZW50IHY1IEFuZGRyZXNzCgpVU0FHRQogIGJ3IFstLXdvcmtzcGFjZSBBQlNPTFVURV9QQVRIXSBbLS1hZG1pdCBMT0dJQ0FMX1BBVEhdLi4uIFstLWpzb25dIGVkaXQgYW5kZHJlc3MgPGVuY29kZWQtdjUtQW5kZHJlc3M+IDxjb250ZW50PgogIGJ3IFstLXdvcmtzcGFjZSBBQlNPTFVURV9QQVRIXSBbLS1hZG1pdCBMT0dJQ0FMX1BBVEhdLi4uIFstLWpzb25dIGVkaXQgYW5kZHJlc3MgPGVuY29kZWQtdjUtQW5kZHJlc3M+IC0tc3RkaW4KCkRFU0NSSVBUSU9OCiAgUmVwbGFjZXMgZXhhY3RseSBvbmUgY3VycmVudCBGaWxlLCBQYXJhZ3JhcGgsIG9yIExpbmUgdGFyZ2V0IHRocm91Z2ggdGhlIFJ1bnRpbWUgUmVwbGFjZSBzZWFtLgoKQVJHVU1FTlRTCiAgYW5kZHJlc3MgICAgICAgICAgICAgICAgICBSZXF1aXJlZCBpbnB1dCBmb3JtLgogIDxlbmNvZGVkLXY1LUFuZGRyZXNzPiAgICAgT25lIGNhbm9uaWNhbCB2NSBvYmplY3QuCiAgPGNvbnQ=\",\"text\":\"NAME\\n  bw edit - replace one current v5 Anddress\\n\\nUSAGE\\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] edit anddress <encoded-v5-Anddress> <content>\\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] edit anddress <encoded-v5-Anddress> --stdin\\n\\nDESCRIPTION\\n  Replaces exactly one current File, Paragraph, or Line target through the Runtime Replace seam.\\n\\nARGUMENTS\\n  anddress                  Required input form.\\n  <encoded-v5-Anddress>     One canonical v5 object.\\n  <cont\"},{\"sequence\":45,\"ns\":\"166610364053635\",\"kind\":\"stdout\",\"processId\":5,\"base64\":\"ZW50PiAgICAgICAgICAgICAgICAgIE9uZSBwb3NpdGlvbmFsIHJlcGxhY2VtZW50IHN0cmluZy4KICAtLXN0ZGluICAgICAgICAgICAgICAgICAgICBSZWFkIHJlcGxhY2VtZW50IENvbnRlbnQgZnJvbSBzdGFuZGFyZCBpbnB1dCB0aHJvdWdoIEVPRi4KCk9QVElPTlMKICAtLXdvcmtzcGFjZSwgLS1hZG1pdCwgYW5kIC0tanNvbiBtdXN0IHByZWNlZGUgZWRpdC4KICAtLXN0ZGluIGlzIHRoZSBleGNsdXNpdmUgQ29udGVudCBzZWxlY3RvcjsgdXNlIHN0YW5kYXJkIGlucHV0IHRvIHBhc3MgbGl0ZXJhbCAtLXN0ZGluIENvbnRlbnQuCgpXSEFUIEhBUFBFTlMKICBWYWxpZGF0ZXMgdGhlIEFuZGRyZXNzLCByZWFkcyBzZWxlY3RlZCBzdGFuZGFyZCBpbnB1dCBiZWZvcmUgUnVudGltZSBhY2Nlc3MsIHByZXNlcnZlcyBhbiBleGlzdGluZyBMaW5lIHRlcm1pbmF0b3IgYXV0b21hdGljYWxseSwgdGhlbiBhcHBsaWVzIG9uZSBSZXBsYWNlLgoKT1VUUFVUCiAgSHVtYW4gb3V0cHV0IHdyaXRlcyB0aGUgcmU=\",\"text\":\"ent>                  One positional replacement string.\\n  --stdin                    Read replacement Content from standard input through EOF.\\n\\nOPTIONS\\n  --workspace, --admit, and --json must precede edit.\\n  --stdin is the exclusive Content selector; use standard input to pass literal --stdin Content.\\n\\nWHAT HAPPENS\\n  Validates the Anddress, reads selected standard input before Runtime access, preserves an existing Line terminator automatically, then applies one Replace.\\n\\nOUTPUT\\n  Human output writes the re\"},{\"sequence\":46,\"ns\":\"166610364084461\",\"kind\":\"stdout\",\"processId\":5,\"base64\":\"Y2VpcHQgb3V0Y29tZSBhbmQgZnJlc2ggQW5kZHJlc3Mgd2hlbiBwcmVzZW50LiAtLWpzb24gd3JpdGVzIGJ3LmNsaS5lZGl0LnYxLgoKRVhBTVBMRVMKICBidyBlZGl0IGFuZGRyZXNzICc8djUtQW5kZHJlc3M+JyAncmVwbGFjZW1lbnQnCiAgcHJpbnRmICclcycgJ3JlcGxhY2VtZW50JyB8IGJ3IGVkaXQgYW5kZHJlc3MgJzx2NS1BbmRkcmVzcz4nIC0tc3RkaW4KCkZBSUxVUkVTCiAgSW52YWxpZCBpbnB1dCBpcyBhIHVzYWdlIGZhaWx1cmUuIFN0YW5kYXJkLWlucHV0LCBzdGFsZSwgdW5hdmFpbGFibGUsIG9yIHB1YmxpY2F0aW9uIGZhaWx1cmUgZXhpdHMgMS4KClNFRSBBTFNPCiAgYncgaGVscCB2aWV3CiAgYncgaGVscCBjaGVjawo=\",\"text\":\"ceipt outcome and fresh Anddress when present. --json writes bw.cli.edit.v1.\\n\\nEXAMPLES\\n  bw edit anddress '<v5-Anddress>' 'replacement'\\n  printf '%s' 'replacement' | bw edit anddress '<v5-Anddress>' --stdin\\n\\nFAILURES\\n  Invalid input is a usage failure. Standard-input, stale, unavailable, or publication failure exits 1.\\n\\nSEE ALSO\\n  bw help view\\n  bw help check\\n\"},{\"sequence\":47,\"ns\":\"166610364127660\",\"kind\":\"stdout-eof\",\"processId\":5},{\"sequence\":48,\"ns\":\"166610364176670\",\"kind\":\"stderr-eof\",\"processId\":5},{\"sequence\":49,\"ns\":\"166610364220203\",\"kind\":\"close\",\"processId\":5,\"code\":0,\"signal\":null}],\"more\":false,\"active\":[3]}\n"}
{"id":11,"bytes":1109,"sha256":"9836d0439d2a821302ba100653fccc0a25d602c353ec528530ba12b9a9747b22","content":"{\"id\":11,\"events\":[{\"sequence\":50,\"ns\":\"166625228535174\",\"kind\":\"stdin\",\"processId\":3,\"base64\":\"cmVwbGFjZSBAMCAicmV0cnlfYnVkZ2V0ID0gNSIKcmVwbGFjZSBAMyAicmV0cnlfYnVkZ2V0ID0gNSIKcmVwbGFjZSBANCAicmV0cnlfYnVkZ2V0ID0gNSIKcmVwbGFjZSBANyAicmV0cnlfYnVkZ2V0ID0gNSIK\",\"text\":\"replace @0 \\\"retry_budget = 5\\\"\\nreplace @3 \\\"retry_budget = 5\\\"\\nreplace @4 \\\"retry_budget = 5\\\"\\nreplace @7 \\\"retry_budget = 5\\\"\\n\"},{\"sequence\":51,\"ns\":\"166625228663832\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"QDI0CUNoYW5nZWQJTGluZQl1bml0LTAxLnR4dDoyCg==\",\"text\":\"@24\\tChanged\\tLine\\tunit-01.txt:2\\n\"},{\"sequence\":52,\"ns\":\"166625228688529\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"QDI1CUNoYW5nZWQJTGluZQl1bml0LTAyLnR4dDo1Cg==\",\"text\":\"@25\\tChanged\\tLine\\tunit-02.txt:5\\n\"},{\"sequence\":53,\"ns\":\"166625228704176\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"QDI2CUNoYW5nZWQJTGluZQl1bml0LTAzLnR4dDoyCg==\",\"text\":\"@26\\tChanged\\tLine\\tunit-03.txt:2\\n\"},{\"sequence\":54,\"ns\":\"166625228719059\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"QDI3CUNoYW5nZWQJTGluZQl1bml0LTA0LnR4dDo1Cg==\",\"text\":\"@27\\tChanged\\tLine\\tunit-04.txt:5\\n\"}],\"more\":false,\"active\":[3]}\n"}
{"id":12,"bytes":2323,"sha256":"6fc28afd1d61b77152f43bfcf6913c92b3414013bc8b8c19743fd94dbc911a78","content":"{\"id\":12,\"events\":[{\"sequence\":55,\"ns\":\"166640660317451\",\"kind\":\"stdin\",\"processId\":3,\"base64\":\"Y2hlY2sgQDI0IEAyNSBAMjYgQDI3IEAxIEAyIEA1IEA2CmxldCBmaWxlcyA9IHNlYXJjaCBmaWxlICJyZXRyeV9idWRnZXQiCnZpZXcgYW5kZHJlc3MgQGZpbGVzWzBdCnZpZXcgYW5kZHJlc3MgQGZpbGVzWzFdCnZpZXcgYW5kZHJlc3MgQGZpbGVzWzJdCnZpZXcgYW5kZHJlc3MgQGZpbGVzWzNdCmV4aXQK\",\"text\":\"check @24 @25 @26 @27 @1 @2 @5 @6\\nlet files = search file \\\"retry_budget\\\"\\nview anddress @files[0]\\nview anddress @files[1]\\nview anddress @files[2]\\nview anddress @files[3]\\nexit\\n\"},{\"sequence\":56,\"ns\":\"166640660371102\",\"kind\":\"stdin-eof\",\"processId\":3},{\"sequence\":57,\"ns\":\"166640660481624\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"QDI4CUN1cnJlbnQJTGluZQl1bml0LTAxLnR4dDoyCkAyOQlDdXJyZW50CUxpbmUJdW5pdC0wMi50eHQ6NQpAMzAJQ3VycmVudAlMaW5lCXVuaXQtMDMudHh0OjIKQDMxCUN1cnJlbnQJTGluZQl1bml0LTA0LnR4dDo1Ck5vdEN1cnJlbnQKTm90Q3VycmVudApOb3RDdXJyZW50Ck5vdEN1cnJlbnQKRm91bmQgNAowCUZpbGUJdW5pdC0wMS50eHQKMQlGaWxlCXVuaXQtMDIudHh0CjIJRmlsZQl1bml0LTAzLnR4dAozCUZpbGUJdW5pdC0wNC50eHQK\",\"text\":\"@28\\tCurrent\\tLine\\tunit-01.txt:2\\n@29\\tCurrent\\tLine\\tunit-02.txt:5\\n@30\\tCurrent\\tLine\\tunit-03.txt:2\\n@31\\tCurrent\\tLine\\tunit-04.txt:5\\nNotCurrent\\nNotCurrent\\nNotCurrent\\nNotCurrent\\nFound 4\\n0\\tFile\\tunit-01.txt\\n1\\tFile\\tunit-02.txt\\n2\\tFile\\tunit-03.txt\\n3\\tFile\\tunit-04.txt\\n\"},{\"sequence\":58,\"ns\":\"166640660522188\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"cm9sZSA9IHByaW1hcnkKcmV0cnlfYnVkZ2V0ID0gNQoKcm9sZSA9IHNlY29uZGFyeQpyZXRyeV9idWRnZXQgPSAzCnJvbGUgPSBzZWNvbmRhcnkNcmV0cnlfYnVkZ2V0ID0gMw0Ncm9sZSA9IHByaW1hcnkNcmV0cnlfYnVkZ2V0ID0gNQ1yb2xlID0gcHJpbWFyeQ0KcmV0cnlfYnVkZ2V0ID0gNQ0KDQpyb2xlID0gc2Vjb25kYXJ5DQpyZXRyeV9idWRnZXQgPSAzDQpyb2xlID0gc2Vjb25kYXJ5CnJldHJ5X2J1ZGdldCA9IDMKCnJvbGUgPSBwcmltYXJ5CnJldHJ5X2J1ZGdldCA9IDU=\",\"text\":\"role = primary\\nretry_budget = 5\\n\\nrole = secondary\\nretry_budget = 3\\nrole = secondary\\rretry_budget = 3\\r\\rrole = primary\\rretry_budget = 5\\rrole = primary\\r\\nretry_budget = 5\\r\\n\\r\\nrole = secondary\\r\\nretry_budget = 3\\r\\nrole = secondary\\nretry_budget = 3\\n\\nrole = primary\\nretry_budget = 5\"},{\"sequence\":59,\"ns\":\"166640660658059\",\"kind\":\"stderr-eof\",\"processId\":3},{\"sequence\":60,\"ns\":\"166640660732991\",\"kind\":\"stdout-eof\",\"processId\":3},{\"sequence\":61,\"ns\":\"166640660817381\",\"kind\":\"close\",\"processId\":3,\"code\":2,\"signal\":null}],\"more\":false,\"active\":[]}\n"}
{"id":13,"bytes":197,"sha256":"f5b0a70f84a9f8237ea7e57610c0270de4bf5f749e683915a90dc54c61a2d75a","content":"{\"id\":13,\"events\":[{\"sequence\":62,\"ns\":\"166653550331181\",\"kind\":\"finished\",\"firstNs\":\"166481366907538\",\"lastProductCloseNs\":\"166640660817381\",\"elapsedNs\":\"159293909843\"}],\"more\":false,\"active\":[]}\n"}
```


#### Archived file: records/dummy-new/requests.jsonl

Bytes: 1015; SHA-256: `88441e219648075fc4a25c49988d9eff9aaea758760b54dc7406765f15724d1e`.

```jsonl
{"ns":"166485596514019","request":{"arm":"dummy-new","op":"start","args":["--help"],"eof":true}}
{"ns":"166498897595140","request":{"arm":"dummy-new","op":"start","args":["help","shell"],"eof":true}}
{"ns":"166510597150608","request":{"arm":"dummy-new","op":"poll"}}
{"ns":"166524071659420","request":{"arm":"dummy-new","op":"start","args":["shell"],"stdin":"search line \"retry_budget = 3\"\n"}}
{"ns":"166537303333218","request":{"arm":"dummy-new","op":"send","processId":3,"stdin":"view @0 @1 @2 @3 @4 @5 @6 @7 --as paragraph\n","eof":false}}
{"ns":"166551209873819","request":{"arm":"dummy-new","op":"send","processId":3,"stdin":"replace @0 \"retry_budget = 5\"\nreplace @3 \"retry_budget = 5\"\nreplace @4 \"retry_budget = 5\"\nreplace @7 \"retry_budget = 5\"\n","eof":false}}
{"ns":"166564061918402","request":{"arm":"dummy-new","op":"send","processId":3,"stdin":"check @16 @17 @18 @19\nview @16 @17 @18 @19 --as file\nexit\n","eof":true}}
{"ns":"166577080472245","request":{"arm":"dummy-new","op":"finish"}}
```


#### Archived file: records/dummy-new/delivered.jsonl

Bytes: 18530; SHA-256: `03530baf35d1a3642e02cefd8be6a833e2a7aeea2713a82dc04a0ef53806b6b6`.

```jsonl
{"id":1,"bytes":3139,"sha256":"660e928cba7763361ef0b76b9424d82dec942353d0eb7a011b674fb4ff1c74c7","content":"{\"id\":1,\"events\":[{\"sequence\":1,\"ns\":\"166485596566364\",\"kind\":\"start\",\"processId\":1,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw\",\"--help\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh/dummy-new/workspace\"},{\"sequence\":2,\"ns\":\"166485597816715\",\"kind\":\"stdin-eof\",\"processId\":1},{\"sequence\":3,\"ns\":\"166485597894049\",\"kind\":\"spawn\",\"processId\":1,\"pid\":2722783},{\"sequence\":4,\"ns\":\"166485598170543\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"VVNBR0UKICBidyBbR0xPQkFMIE9QVElPTlNdIDxjb21tYW5kPiBbY29tbWFuZCBvcHRpb25zIGFuZCBvcGVyYW5kc10KICBidyBoZWxwIFs8Y29tbWFuZD5dCgpHTE9CQUwgT1BUSU9OUwogIC0td29ya3NwYWNlIEFCU09MVVRFX1BBVEggIFNlbGVjdCBhbiBhYnNvbHV0ZSB3b3Jrc3BhY2UgYmVmb3JlIHRoZSBjb21tYW5kLgogIC0tYWRtaXQgTE9HSUNBTF9QQVRIICAgICAgIEFkbWl0IGEgbG9naWNhbCByb290IGJlZm9yZSB0aGUgY29tbWFuZDsgcmVwZWF0YWJsZS4KICAtLWpzb24gICAgICAgICAgICAgICAgICAgICBTZWxlY3QgSlNPTiBvdXRwdXQgd2hlcmUgdGhlIGNvbW1hbmQgc3VwcG9ydHMgaXQuCiAgLS1yYXcgICAgICAgICAgICAgICAgICAgICAgU2VsZWN0IHJhdyBWaWV3IG91dHB1dCBvbmx5LgoKQ09NTUFORFMKICBzaGVsbCAgICBSZXVzZSBzaG9ydCByZWZlcmVuY2VzIGFjcm9zcyBzZWFyY2gsIHZpZXcsIHJlcGxhY2UsIGFuZCBjaGVjay4KICBzZWFyY2ggICBEaXNjb3ZlciBjdXI=\",\"text\":\"USAGE\\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\\n  bw help [<command>]\\n\\nGLOBAL OPTIONS\\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\\n  --json                     Select JSON output where the command supports it.\\n  --raw                      Select raw View output only.\\n\\nCOMMANDS\\n  shell    Reuse short references across search, view, replace, and check.\\n  search   Discover cur\"},{\"sequence\":5,\"ns\":\"166485598246779\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"cmVudCBGaWxlLCBQYXJhZ3JhcGgsIG9yIExpbmUgQW5kZHJlc3Nlcy4KICB2aWV3ICAgICBSZWFkIG9uZSBvciBtb3JlIGN1cnJlbnQgQW5kZHJlc3Nlcy4KICBlZGl0ICAgICBSZXBsYWNlIG9uZSBjdXJyZW50IEFuZGRyZXNzLgogIGNoZWNrICAgIENoZWNrIG9uZSBvciBtb3JlIGN1cnJlbnQgQW5kZHJlc3Nlcy4KICB2ZXJzaW9uICBQcmludCB0aGUgQmFja3dyaXRlciB2ZXJzaW9uLgogIHVwZGF0ZSAgIFJ1biB0aGUgaW5zdGFsbGVkLXBsYXRmb3JtIHVwZGF0ZXIuCgpBRERJVElPTkFMIEhFTFAKICBidyBoZWxwIDxjb21tYW5kPgogIEFkdmFuY2VkIHRvcGljczogcGljaywgYW5jaG9yLCBhcHBseSwgZGF0YSAocmF3IFNlc3Npb24gb25seTsgbm8gb25lLXNob3QgZXhlY3V0aW9uKS4KCkdsb2JhbCBvcHRpb25zIHByZWNlZGUgdGhlIGNvbW1hbmQuIFVzZSBidyBoZWxwIHNoZWxsIGZvciBvcmRpbmFyeSBzaG9ydC1yZWYgd29yay4K\",\"text\":\"rent File, Paragraph, or Line Anddresses.\\n  view     Read one or more current Anddresses.\\n  edit     Replace one current Anddress.\\n  check    Check one or more current Anddresses.\\n  version  Print the Backwriter version.\\n  update   Run the installed-platform updater.\\n\\nADDITIONAL HELP\\n  bw help <command>\\n  Advanced topics: pick, anchor, apply, data (raw Session only; no one-shot execution).\\n\\nGlobal options precede the command. Use bw help shell for ordinary short-ref work.\\n\"},{\"sequence\":6,\"ns\":\"166485598347589\",\"kind\":\"stdout-eof\",\"processId\":1},{\"sequence\":7,\"ns\":\"166485598492676\",\"kind\":\"stderr-eof\",\"processId\":1},{\"sequence\":8,\"ns\":\"166485598644948\",\"kind\":\"close\",\"processId\":1,\"code\":0,\"signal\":null}],\"more\":false,\"active\":[]}\n"}
{"id":2,"bytes":5603,"sha256":"06d27e8e7e8854271044cdd0af806649d1d28d629d69fa0b7a54b25502e1156f","content":"{\"id\":2,\"events\":[{\"sequence\":9,\"ns\":\"166498897645169\",\"kind\":\"start\",\"processId\":2,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw\",\"help\",\"shell\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh/dummy-new/workspace\"},{\"sequence\":10,\"ns\":\"166498898785479\",\"kind\":\"stdin-eof\",\"processId\":2},{\"sequence\":11,\"ns\":\"166498898850070\",\"kind\":\"spawn\",\"processId\":2,\"pid\":2723139},{\"sequence\":12,\"ns\":\"166498899135935\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"TkFNRQogIGJ3IHNoZWxsIC0gcmV1c2Ugc2hvcnQgcmVmZXJlbmNlcyBhY3Jvc3Mgc2VhcmNoLCB2aWV3LCByZXBsYWNlLCBhbmQgY2hlY2sKClVTQUdFCiAgYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4gc2hlbGwKCkNPTU1BTkRTCiAgc2VhcmNoIDxsaW5lfHBhcmFncmFwaHxmaWxlPiA8cXVlcnk+IFstLXNvdXJjZSBQQVRIIHwgLS1zdWJ0cmVlIFBBVEhdLi4uCiAgc2VhcmNoIC9maWxlIDxsb2dpY2FsLXBhdGg+CiAgdmlldyA8UkVGPi4uLiBbLS1hcyA8bGluZXxwYXJhZ3JhcGh8ZmlsZT5dCiAgcmVwbGFjZSA8UkVGPiA8Q09OVEVOVD4KICBjaGVjayA8UkVGPi4uLgogIGxldCA8TkFNRT4gPSA8UkVGPgogIGV4aXQKClJFRkVSRU5DRVMgQU5EIENPTlRFTlQKICBATiBpcyBhIHplcm8tYmFzZWQsIGFwcGVuZC1vbmx5IHJlZmVyZW5jZSBmcm9tIHRoaXMgcHJvY2Vzcy4gSXQgZW5kcyBhdCBFT0Ygb3IgZXhpdC4KICBAbmFtZSBpcyBhIG5hbWU=\",\"text\":\"NAME\\n  bw shell - reuse short references across search, view, replace, and check\\n\\nUSAGE\\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\\n\\nCOMMANDS\\n  search <line|paragraph|file> <query> [--source PATH | --subtree PATH]...\\n  search /file <logical-path>\\n  view <REF>... [--as <line|paragraph|file>]\\n  replace <REF> <CONTENT>\\n  check <REF>...\\n  let <NAME> = <REF>\\n  exit\\n\\nREFERENCES AND CONTENT\\n  @N is a zero-based, append-only reference from this process. It ends at EOF or exit.\\n  @name is a name\"},{\"sequence\":13,\"ns\":\"166498899176255\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"ZCBBbmRkcmVzczsgQGhpdHNbMF0gc2VsZWN0cyBhbiBpbmRleGVkIFNlYXJjaC9QaWNrIGJpbmRpbmcuCiAgbGV0IG5hbWUgPSBATiBtYWtlcyBhIG5hbWVkIGFsaWFzLiBOYW1lcyBjYW5ub3QgYmUgcmVkZWZpbmVkLgogIERvdWJsZS1xdW90ZSBhcmd1bWVudHMgY29udGFpbmluZyBzcGFjZXMuIFF1b3RlZCBlc2NhcGVzIGFyZSBcXCwgXCIsIFxuLCBcciwgYW5kIFx0LgogIE9uZSBwaHlzaWNhbCBpbnB1dCBsaW5lIGlzIG9uZSBjb21tYW5kOyBubyBzaW5nbGUgcXVvdGVzLCBwaXBlcywgb3IgRU9GIENvbnRlbnQgc2VsZWN0b3IuCiAgTGluZSBSZXBsYWNlIGFjY2VwdHMgYm9keSBvbmx5OiBOVUwvQ1IvTEYgYXJlIHJlamVjdGVkOyB0aGUgZXhpc3RpbmcgdGVybWluYXRvciBpcyBwcmVzZXJ2ZWQuCiAgRmlsZS9QYXJhZ3JhcGggUmVwbGFjZSB1c2VzIGV4YWN0IENvbnRlbnQuIENoYW5nZWQgc291cmNlIGJ5dGVzIHN0YWxlIG9sZCBzYW1lLXNvdXJjZSByZWZzLAogIGluY2x1ZGluZyBvdGhlciA=\",\"text\":\"d Anddress; @hits[0] selects an indexed Search/Pick binding.\\n  let name = @N makes a named alias. Names cannot be redefined.\\n  Double-quote arguments containing spaces. Quoted escapes are \\\\\\\\, \\\\\\\", \\\\n, \\\\r, and \\\\t.\\n  One physical input line is one command; no single quotes, pipes, or EOF Content selector.\\n  Line Replace accepts body only: NUL/CR/LF are rejected; the existing terminator is preserved.\\n  File/Paragraph Replace uses exact Content. Changed source bytes stale old same-source refs,\\n  including other \"},{\"sequence\":14,\"ns\":\"166498899190301\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"bG9jYXRpb25zLiBVc2UgdGhlIGZyZXNoIHJlY2VpcHQgb3IgZXhwbGljaXRseSBzZWFyY2ggYWdhaW47IG5ldmVyIGF1dG8tcmV0cnkuCgpPVVRQVVQKICBTZWFyY2ggYXBwZW5kcyBvbmUgQE4gcGVyIHJlc3VsdDsgRW1wdHkgd3JpdGVzIG5vdGhpbmcuCiAgVmlldyByZXR1cm5zIGVhY2ggaW5wdXQgcmVmLCBhIGZyZXNoIHJlZiwga2luZC9sb2NhdGlvbiwgYW5kIGNvbXBsZXRlIENvbnRlbnQgaW4gaW5wdXQgb3JkZXIuCiAgVmlldzxUQUI+UkVGPFRBQj5ieXRlcz1OIHN0YXJ0cyBhIHJlY29yZCwgZm9sbG93ZWQgYnkgZnJlc2gtcmVmIG1ldGFkYXRhIGFuZCBOIGV4YWN0IGJ5dGVzLgogIFRoZSBmb2xsb3dpbmcgTEYgYW5kIEVuZFZpZXcgbGluZSBhcmUgZGlzcGxheSBmcmFtaW5nLCBub3Qgc291cmNlIENvbnRlbnQuCiAgUmVsYXRpb25BYnNlbnQgaXMgc2hvd24gYXQgaXRzIGlucHV0IHBvc2l0aW9uIHdpdGhvdXQgQ29udGVudCBvciBhIGZyZXNoIHNsb3QuCiAgUmVwbGFjZSB3cml0ZXMgVW4=\",\"text\":\"locations. Use the fresh receipt or explicitly search again; never auto-retry.\\n\\nOUTPUT\\n  Search appends one @N per result; Empty writes nothing.\\n  View returns each input ref, a fresh ref, kind/location, and complete Content in input order.\\n  View<TAB>REF<TAB>bytes=N starts a record, followed by fresh-ref metadata and N exact bytes.\\n  The following LF and EndView line are display framing, not source Content.\\n  RelationAbsent is shown at its input position without Content or a fresh slot.\\n  Replace writes Un\"},{\"sequence\":15,\"ns\":\"166498899201783\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"Y2hhbmdlZC9DaGFuZ2VkIHdpdGggYSBmcmVzaCByZWYsIG9yIENoYW5nZWQ8VEFCPk5vbmUgd2l0aG91dCBvbmUuCiAgQ2hlY2sgd3JpdGVzIG9uZSBzdGF0dXMgcGVyIGlucHV0OyBvbmx5IEN1cnJlbnQgYXBwZW5kcyBhIGZyZXNoIHJlZi4gQ2hlY2sgaXMgb3B0aW9uYWwuCgpFWEFNUExFUwogIFN0YXJ0IGJ3IHNoZWxsIGluIGEgd29ya3NwYWNlIHdpdGggb25seSBub3RlLnR4dCBjb250YWluaW5nIG5lZWRsZSBwbHVzIENSTEYsIHRoZW4gZW50ZXI6CiAgc2VhcmNoIGxpbmUgbmVlZGxlIC0tc291cmNlIG5vdGUudHh0CiAgdmlldyBAMCAtLWFzIHBhcmFncmFwaAogIGxldCBzZWxlY3RlZCA9IEAwCiAgcmVwbGFjZSBAc2VsZWN0ZWQgIm5ldyB2YWx1ZSIKICBjaGVjayBAMCBAMgogIHZpZXcgQDMKICBleGl0CgpBRFZBTkNFRAogIGxldCBoaXRzID0gc2VhcmNoIGxpbmUgbmVlZGxlCiAgdmlldyBhbmRkcmVzcyBAaGl0c1swXQogIFJhdyBuYW1lZCBWaWV3IHdyaXRlcyBleGFjdCBDb250ZW50IHc=\",\"text\":\"changed/Changed with a fresh ref, or Changed<TAB>None without one.\\n  Check writes one status per input; only Current appends a fresh ref. Check is optional.\\n\\nEXAMPLES\\n  Start bw shell in a workspace with only note.txt containing needle plus CRLF, then enter:\\n  search line needle --source note.txt\\n  view @0 --as paragraph\\n  let selected = @0\\n  replace @selected \\\"new value\\\"\\n  check @0 @2\\n  view @3\\n  exit\\n\\nADVANCED\\n  let hits = search line needle\\n  view anddress @hits[0]\\n  Raw named View writes exact Content w\"}],\"more\":true,\"active\":[]}\n"}
{"id":3,"bytes":1341,"sha256":"a76513aa81540849c193d9566df8154c7cf95c9b03cce59ec867dfd34dc78844","content":"{\"id\":3,\"events\":[{\"sequence\":16,\"ns\":\"166498899229324\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"aXRob3V0IGRpcmVjdC1WaWV3IGZyYW1pbmcuCiAgU2VlIGJ3IGhlbHAgcGljaywgYncgaGVscCBhbmNob3IsIGJ3IGhlbHAgYXBwbHksIGFuZCBidyBoZWxwIGRhdGEuCgpGQUlMVVJFUwogIFVzYWdlIGVycm9ycyByZWNvcmQgZXhpdCAyOyBSdW50aW1lL3NvdXJjZSBlcnJvcnMgcmVjb3JkIGV4aXQgMTsgbGF0ZXIgY29tbWFuZHMgc3RpbGwgcnVuLgogIFN0ZGluL3N0ZG91dCBmYWlsdXJlIGVuZHMgdGhlIHNoZWxsIHdpdGggZXhpdCAxLiBQYXJ0aWFsIG91dHB1dCBjYW5ub3QgcHJvdmUgZGVsaXZlcnkgb3IKICB1bmRvIHB1YmxpY2F0aW9uLiAtLWpzb24vLS1yYXcgYW5kIGNvbW1hbmQtbGluZSBvcGVyYW5kcyBhcmUgdW5hdmFpbGFibGUuCgpTRUUgQUxTTwogIGJ3IGhlbHAgc2VhcmNoCiAgYncgaGVscCBlZGl0Cg==\",\"text\":\"ithout direct-View framing.\\n  See bw help pick, bw help anchor, bw help apply, and bw help data.\\n\\nFAILURES\\n  Usage errors record exit 2; Runtime/source errors record exit 1; later commands still run.\\n  Stdin/stdout failure ends the shell with exit 1. Partial output cannot prove delivery or\\n  undo publication. --json/--raw and command-line operands are unavailable.\\n\\nSEE ALSO\\n  bw help search\\n  bw help edit\\n\"},{\"sequence\":17,\"ns\":\"166498899270218\",\"kind\":\"stdout-eof\",\"processId\":2},{\"sequence\":18,\"ns\":\"166498899341548\",\"kind\":\"stderr-eof\",\"processId\":2},{\"sequence\":19,\"ns\":\"166498899398902\",\"kind\":\"close\",\"processId\":2,\"code\":0,\"signal\":null}],\"more\":false,\"active\":[]}\n"}
{"id":4,"bytes":1018,"sha256":"cb9abc67a28f943f6b4dbc5d6194150d5959cd4289424df2aad3782a1e750cb7","content":"{\"id\":4,\"events\":[{\"sequence\":20,\"ns\":\"166524071705095\",\"kind\":\"start\",\"processId\":3,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw\",\"shell\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh/dummy-new/workspace\"},{\"sequence\":21,\"ns\":\"166524072795641\",\"kind\":\"stdin\",\"processId\":3,\"base64\":\"c2VhcmNoIGxpbmUgInJldHJ5X2J1ZGdldCA9IDMiCg==\",\"text\":\"search line \\\"retry_budget = 3\\\"\\n\"},{\"sequence\":22,\"ns\":\"166524072886724\",\"kind\":\"spawn\",\"processId\":3,\"pid\":2723983},{\"sequence\":23,\"ns\":\"166524073274067\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"QDAJTGluZQl1bml0LTAxLnR4dDoyCkAxCUxpbmUJdW5pdC0wMS50eHQ6NQpAMglMaW5lCXVuaXQtMDIudHh0OjIKQDMJTGluZQl1bml0LTAyLnR4dDo1CkA0CUxpbmUJdW5pdC0wMy50eHQ6MgpANQlMaW5lCXVuaXQtMDMudHh0OjUKQDYJTGluZQl1bml0LTA0LnR4dDoyCkA3CUxpbmUJdW5pdC0wNC50eHQ6NQo=\",\"text\":\"@0\\tLine\\tunit-01.txt:2\\n@1\\tLine\\tunit-01.txt:5\\n@2\\tLine\\tunit-02.txt:2\\n@3\\tLine\\tunit-02.txt:5\\n@4\\tLine\\tunit-03.txt:2\\n@5\\tLine\\tunit-03.txt:5\\n@6\\tLine\\tunit-04.txt:2\\n@7\\tLine\\tunit-04.txt:5\\n\"}],\"more\":false,\"active\":[3]}\n"}
{"id":5,"bytes":2171,"sha256":"e0de3c78f6c9c598734979d2732c288a349105df0e03d5a0eb54a04631e2810c","content":"{\"id\":5,\"events\":[{\"sequence\":24,\"ns\":\"166537303380768\",\"kind\":\"stdin\",\"processId\":3,\"base64\":\"dmlldyBAMCBAMSBAMiBAMyBANCBANSBANiBANyAtLWFzIHBhcmFncmFwaAo=\",\"text\":\"view @0 @1 @2 @3 @4 @5 @6 @7 --as paragraph\\n\"},{\"sequence\":25,\"ns\":\"166537303483137\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"VmlldwlAMAlieXRlcz0zMgpAOAlQYXJhZ3JhcGgJdW5pdC0wMS50eHQ6MS0yCnJvbGUgPSBwcmltYXJ5CnJldHJ5X2J1ZGdldCA9IDMKCkVuZFZpZXcKVmlldwlAMQlieXRlcz0zNApAOQlQYXJhZ3JhcGgJdW5pdC0wMS50eHQ6NC01CnJvbGUgPSBzZWNvbmRhcnkKcmV0cnlfYnVkZ2V0ID0gMwoKRW5kVmlldwpWaWV3CUAyCWJ5dGVzPTM0CkAxMAlQYXJhZ3JhcGgJdW5pdC0wMi50eHQ6MS0yCnJvbGUgPSBzZWNvbmRhcnkNcmV0cnlfYnVkZ2V0ID0gMw0KRW5kVmlldwpWaWV3CUAzCWJ5dGVzPTMyCkAxMQlQYXJhZ3JhcGgJdW5pdC0wMi50eHQ6NC01CnJvbGUgPSBwcmltYXJ5DXJldHJ5X2J1ZGdldCA9IDMNCkVuZFZpZXcKVmlldwlANAlieXRlcz0zNApAMTIJUGFyYWdyYXBoCXVuaXQtMDMudHh0OjEtMgpyb2xlID0gcHJpbWFyeQ0KcmV0cnlfYnVkZ2V0ID0gMw0KCkVuZFZpZXcKVmlldwlANQlieXRlcz0zNgpAMTMJUGFyYWdyYXBoCXVuaXQtMDMudHh0OjQtNQpyb2xlID0gc2Vjb25kYXJ5DQpyZXQ=\",\"text\":\"View\\t@0\\tbytes=32\\n@8\\tParagraph\\tunit-01.txt:1-2\\nrole = primary\\nretry_budget = 3\\n\\nEndView\\nView\\t@1\\tbytes=34\\n@9\\tParagraph\\tunit-01.txt:4-5\\nrole = secondary\\nretry_budget = 3\\n\\nEndView\\nView\\t@2\\tbytes=34\\n@10\\tParagraph\\tunit-02.txt:1-2\\nrole = secondary\\rretry_budget = 3\\r\\nEndView\\nView\\t@3\\tbytes=32\\n@11\\tParagraph\\tunit-02.txt:4-5\\nrole = primary\\rretry_budget = 3\\r\\nEndView\\nView\\t@4\\tbytes=34\\n@12\\tParagraph\\tunit-03.txt:1-2\\nrole = primary\\r\\nretry_budget = 3\\r\\n\\nEndView\\nView\\t@5\\tbytes=36\\n@13\\tParagraph\\tunit-03.txt:4-5\\nrole = secondary\\r\\nret\"},{\"sequence\":26,\"ns\":\"166537303503924\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"cnlfYnVkZ2V0ID0gMw0KCkVuZFZpZXcKVmlldwlANglieXRlcz0zNApAMTQJUGFyYWdyYXBoCXVuaXQtMDQudHh0OjEtMgpyb2xlID0gc2Vjb25kYXJ5CnJldHJ5X2J1ZGdldCA9IDMKCkVuZFZpZXcKVmlldwlANwlieXRlcz0zMQpAMTUJUGFyYWdyYXBoCXVuaXQtMDQudHh0OjQtNQpyb2xlID0gcHJpbWFyeQpyZXRyeV9idWRnZXQgPSAzCkVuZFZpZXcK\",\"text\":\"ry_budget = 3\\r\\n\\nEndView\\nView\\t@6\\tbytes=34\\n@14\\tParagraph\\tunit-04.txt:1-2\\nrole = secondary\\nretry_budget = 3\\n\\nEndView\\nView\\t@7\\tbytes=31\\n@15\\tParagraph\\tunit-04.txt:4-5\\nrole = primary\\nretry_budget = 3\\nEndView\\n\"}],\"more\":false,\"active\":[3]}\n"}
{"id":6,"bytes":918,"sha256":"d1bf01da3528661fd8ffec0e98f483c394258f296fdc9166dca42bf5b746e033","content":"{\"id\":6,\"events\":[{\"sequence\":27,\"ns\":\"166551209923017\",\"kind\":\"stdin\",\"processId\":3,\"base64\":\"cmVwbGFjZSBAMCAicmV0cnlfYnVkZ2V0ID0gNSIKcmVwbGFjZSBAMyAicmV0cnlfYnVkZ2V0ID0gNSIKcmVwbGFjZSBANCAicmV0cnlfYnVkZ2V0ID0gNSIKcmVwbGFjZSBANyAicmV0cnlfYnVkZ2V0ID0gNSIK\",\"text\":\"replace @0 \\\"retry_budget = 5\\\"\\nreplace @3 \\\"retry_budget = 5\\\"\\nreplace @4 \\\"retry_budget = 5\\\"\\nreplace @7 \\\"retry_budget = 5\\\"\\n\"},{\"sequence\":28,\"ns\":\"166551210105495\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"QDE2CUNoYW5nZWQJTGluZQl1bml0LTAxLnR4dDoyCkAxNwlDaGFuZ2VkCUxpbmUJdW5pdC0wMi50eHQ6NQpAMTgJQ2hhbmdlZAlMaW5lCXVuaXQtMDMudHh0OjIK\",\"text\":\"@16\\tChanged\\tLine\\tunit-01.txt:2\\n@17\\tChanged\\tLine\\tunit-02.txt:5\\n@18\\tChanged\\tLine\\tunit-03.txt:2\\n\"},{\"sequence\":29,\"ns\":\"166551210153489\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"QDE5CUNoYW5nZWQJTGluZQl1bml0LTA0LnR4dDo1Cg==\",\"text\":\"@19\\tChanged\\tLine\\tunit-04.txt:5\\n\"}],\"more\":false,\"active\":[3]}\n"}
{"id":7,"bytes":2215,"sha256":"345339b6ad6af7a0c9e0f12679c1cfb71737ddc02d5544493a237c53efa195b1","content":"{\"id\":7,\"events\":[{\"sequence\":30,\"ns\":\"166564061972753\",\"kind\":\"stdin\",\"processId\":3,\"base64\":\"Y2hlY2sgQDE2IEAxNyBAMTggQDE5CnZpZXcgQDE2IEAxNyBAMTggQDE5IC0tYXMgZmlsZQpleGl0Cg==\",\"text\":\"check @16 @17 @18 @19\\nview @16 @17 @18 @19 --as file\\nexit\\n\"},{\"sequence\":31,\"ns\":\"166564062009293\",\"kind\":\"stdin-eof\",\"processId\":3},{\"sequence\":32,\"ns\":\"166564062092879\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"QDIwCUN1cnJlbnQJTGluZQl1bml0LTAxLnR4dDoyCkAyMQlDdXJyZW50CUxpbmUJdW5pdC0wMi50eHQ6NQpAMjIJQ3VycmVudAlMaW5lCXVuaXQtMDMudHh0OjIKQDIzCUN1cnJlbnQJTGluZQl1bml0LTA0LnR4dDo1ClZpZXcJQDE2CWJ5dGVzPTY3CkAyNAlGaWxlCXVuaXQtMDEudHh0CnJvbGUgPSBwcmltYXJ5CnJldHJ5X2J1ZGdldCA9IDUKCnJvbGUgPSBzZWNvbmRhcnkKcmV0cnlfYnVkZ2V0ID0gMwoKRW5kVmlldwpWaWV3CUAxNwlieXRlcz02NwpAMjUJRmlsZQl1bml0LTAyLnR4dApyb2xlID0gc2Vjb25kYXJ5DXJldHJ5X2J1ZGdldCA9IDMNDXJvbGUgPSBwcmltYXJ5DXJldHJ5X2J1ZGdldCA9IDUNCkVuZFZpZXcKVmlldwlAMTgJYnl0ZXM9NzIKQDI2CUZpbGUJdW5pdC0wMy50eHQKcm9sZSA9IHByaW1hcnkNCnJldHJ5X2J1ZGdldCA9IDUNCg0Kcm9sZSA9IHNlY29uZGFyeQ0KcmV0cnlfYnVkZ2V0ID0gMw0KCkVuZFZpZXcKVmlldwlAMTkJYnl0ZXM9NjYKQDI3CUZpbGUJdW5pdC0wNC50eHQ=\",\"text\":\"@20\\tCurrent\\tLine\\tunit-01.txt:2\\n@21\\tCurrent\\tLine\\tunit-02.txt:5\\n@22\\tCurrent\\tLine\\tunit-03.txt:2\\n@23\\tCurrent\\tLine\\tunit-04.txt:5\\nView\\t@16\\tbytes=67\\n@24\\tFile\\tunit-01.txt\\nrole = primary\\nretry_budget = 5\\n\\nrole = secondary\\nretry_budget = 3\\n\\nEndView\\nView\\t@17\\tbytes=67\\n@25\\tFile\\tunit-02.txt\\nrole = secondary\\rretry_budget = 3\\r\\rrole = primary\\rretry_budget = 5\\r\\nEndView\\nView\\t@18\\tbytes=72\\n@26\\tFile\\tunit-03.txt\\nrole = primary\\r\\nretry_budget = 5\\r\\n\\r\\nrole = secondary\\r\\nretry_budget = 3\\r\\n\\nEndView\\nView\\t@19\\tbytes=66\\n@27\\tFile\\tunit-04.txt\"},{\"sequence\":33,\"ns\":\"166564062136758\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"CnJvbGUgPSBzZWNvbmRhcnkKcmV0cnlfYnVkZ2V0ID0gMwoKcm9sZSA9IHByaW1hcnkKcmV0cnlfYnVkZ2V0ID0gNQpFbmRWaWV3Cg==\",\"text\":\"\\nrole = secondary\\nretry_budget = 3\\n\\nrole = primary\\nretry_budget = 5\\nEndView\\n\"},{\"sequence\":34,\"ns\":\"166564062355368\",\"kind\":\"stderr-eof\",\"processId\":3},{\"sequence\":35,\"ns\":\"166564062436926\",\"kind\":\"stdout-eof\",\"processId\":3},{\"sequence\":36,\"ns\":\"166564062512531\",\"kind\":\"close\",\"processId\":3,\"code\":0,\"signal\":null}],\"more\":false,\"active\":[]}\n"}
{"id":8,"bytes":195,"sha256":"15735b8a65830014ae0a3136771b189e20cf1697711646b494895a2d83e52991","content":"{\"id\":8,\"events\":[{\"sequence\":37,\"ns\":\"166577080549298\",\"kind\":\"finished\",\"firstNs\":\"166485596514019\",\"lastProductCloseNs\":\"166564062512531\",\"elapsedNs\":\"78465998512\"}],\"more\":false,\"active\":[]}\n"}
```


#### Archived file: records/genie-old/requests.jsonl

Bytes: 2725; SHA-256: `f6e17a986d5ab3e05683278f38692dc1d0fb0321e663b05a9aeb6f63f4e580e0`.

```jsonl
{"ns":"166506990711179","request":{"arm":"genie-old","op":"guide"}}
{"ns":"166520283314443","request":{"arm":"genie-old","op":"poll"}}
{"ns":"166533820586006","request":{"arm":"genie-old","op":"poll"}}
{"ns":"166546125332549","request":{"arm":"genie-old","op":"poll"}}
{"ns":"166557917382076","request":{"arm":"genie-old","op":"poll"}}
{"ns":"166570002889013","request":{"arm":"genie-old","op":"poll"}}
{"ns":"166583384138219","request":{"arm":"genie-old","op":"poll"}}
{"ns":"166595766330843","request":{"arm":"genie-old","op":"poll"}}
{"ns":"166609363270475","request":{"arm":"genie-old","op":"poll"}}
{"ns":"166625404783163","request":{"arm":"genie-old","op":"start","args":["--help"],"eof":true}}
{"ns":"166639824721277","request":{"arm":"genie-old","op":"start","args":["shell"]}}
{"ns":"166653866804336","request":{"arm":"genie-old","op":"send","processId":2,"stdin":"search line \"retry_budget = 3\"\n","eof":false}}
{"ns":"166668646554688","request":{"arm":"genie-old","op":"send","processId":2,"stdin":"view @0 --as paragraph\nview @1 --as paragraph\nview @2 --as paragraph\nview @3 --as paragraph\nview @4 --as paragraph\nview @5 --as paragraph\nview @6 --as paragraph\nview @7 --as paragraph\n","eof":false}}
{"ns":"166682054007482","request":{"arm":"genie-old","op":"send","processId":2,"stdin":"help\n","eof":false}}
{"ns":"166696703249789","request":{"arm":"genie-old","op":"start","args":["help","shell"],"eof":true}}
{"ns":"166710970118222","request":{"arm":"genie-old","op":"send","processId":2,"stdin":"let p0 = @8\nlet p1 = @9\nlet p2 = @10\nlet p3 = @11\nlet p4 = @12\nlet p5 = @13\nlet p6 = @14\nlet p7 = @15\nview anddress @p0\nview anddress @p1\nview anddress @p2\nview anddress @p3\nview anddress @p4\nview anddress @p5\nview anddress @p6\nview anddress @p7\n","eof":false}}
{"ns":"166727603556265","request":{"arm":"genie-old","op":"send","processId":2,"stdin":"replace @0 \"retry_budget = 5\"\nreplace @3 \"retry_budget = 5\"\nreplace @4 \"retry_budget = 5\"\nreplace @7 \"retry_budget = 5\"\n","eof":false}}
{"ns":"166741330498462","request":{"arm":"genie-old","op":"send","processId":2,"stdin":"check @16 @17 @18 @19 @1 @2 @5 @6\nview @16 --as file\nview @17 --as file\nview @18 --as file\nview @19 --as file\n","eof":false}}
{"ns":"166755169200432","request":{"arm":"genie-old","op":"send","processId":2,"stdin":"let f0 = @24\nlet f1 = @25\nlet f2 = @26\nlet f3 = @27\nview anddress @f0\nview anddress @f1\nview anddress @f2\nview anddress @f3\nsearch line \"retry_budget = 3\"\n","eof":false}}
{"ns":"166768763705132","request":{"arm":"genie-old","op":"send","processId":2,"stdin":"check @28 @29 @30 @31\nexit\n","eof":true}}
{"ns":"166783230968216","request":{"arm":"genie-old","op":"finish"}}
```


#### Archived file: records/genie-old/delivered.jsonl

Bytes: 66554; SHA-256: `52569d046232ad9815dc008a95e19075b54c2034e12c9d1fe37bdb7ff8295560`.

```jsonl
{"id":1,"bytes":5265,"sha256":"2d4a047c1befded96168360ee1779f97d3231d2eddb6b922fa26fc52ed31b338","content":"{\"id\":1,\"events\":[{\"sequence\":1,\"ns\":\"166506990786856\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"IyBCYWNrd3JpdGVyCgpCYWNrd3JpdGVyIGlzIGEgUnVzdCBDb3JlIGFuZCBSdW50aW1lIGZvciBjdXJyZW50LCBzdHJ1Y3R1cmFsIHRleHQgd29yayBvdmVyCmFkbWl0dGVkIFdvcmtzcGFjZSBTb3VyY2UuIEl0IHByb3ZpZGVzIHRhcmdldC1sb2NhbCBGaWxlLCBQYXJhZ3JhcGgsIGFuZCBMaW5lCmFkZHJlc3NlcyB3aXRob3V0IHR1cm5pbmcgc291cmNlIGhpc3Rvcnkgb3IgZWRpdG9yIHN0YXRlIGludG8gQ29yZSBpZGVudGl0eS4KClRoZSBDb3JlIGNhcGFiaWxpdHkgaW52ZW50b3J5IGlzIFNlYXJjaCwgVmlldywgUGljaywgQW5jaG9yLCBDaGVjaywgRWRpdCwKQXBwbHksIGFuZCBEYXRhLiBUaGUgcmVwb3NpdG9yeSBjdXJyZW50bHkgcHJvdmlkZXMgdGhlaXIgUnVzdCBDb3JlL1J1bnRpbWUKc3VyZmFjZXMgYW5kIHRoZSBjYW5vbmljYWwgYGJ3YCBleGVjdXRhYmxlJ3Mgb25lLXNob3QgaHVtYW4gYW5kIEpTT04KU2VhcmNoL1ZpZXcvQ2hlY2svRWRpdCwgcmF3IFZpZXcsIEFuZGRyZXNzLWZpcnM=\",\"text\":\"# Backwriter\\n\\nBackwriter is a Rust Core and Runtime for current, structural text work over\\nadmitted Workspace Source. It provides target-local File, Paragraph, and Line\\naddresses without turning source history or editor state into Core identity.\\n\\nThe Core capability inventory is Search, View, Pick, Anchor, Check, Edit,\\nApply, and Data. The repository currently provides their Rust Core/Runtime\\nsurfaces and the canonical `bw` executable's one-shot human and JSON\\nSearch/View/Check/Edit, raw View, Anddress-firs\"},{\"sequence\":2,\"ns\":\"166506990812538\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"dCBvbmUtc2hvdCBFZGl0LCBTZXNzaW9uIFBpY2ssIGJhdGNoCkNoZWNrLCBBbmNob3IsIEVkaXQsIEFwcGx5LCByZXN1bHQtYmluZGluZywgZXhwbGljaXQgRGF0YSBtb2RlcywgYW5kCkFkYXB0ZXItb3duZWQgVmVyc2lvbiBhbmQgVXBkYXRlLgoKIyMgUXVpY2sgc3RhcnQKCkluc3RhbGwgdGhlIGNsb3NlZCBvZmZpY2lhbCBgMC4yLjVgIHJlbGVhc2Ugd2l0aCB0aGUgY29tbWFuZCBmb3IgeW91cgpwbGF0Zm9ybS4KCkxpbnV4LCBtYWNPUywgb3IgV1NMOgoKYGBgc2gKY3VybCAtZnNTTCBodHRwczovL2JhY2t3cml0ZXIucGVudGFncmF0aW9uLmNvbS9pbnN0YWxsLnNoIHwgc2gKYGBgCgpXaW5kb3dzIFBvd2VyU2hlbGw6CgpgYGBwb3dlcnNoZWxsCmlybSBodHRwczovL2JhY2t3cml0ZXIucGVudGFncmF0aW9uLmNvbS9pbnN0YWxsLnBzMSB8IGlleApgYGAKCldpbmRvd3MgQ01EOgoKYGBgY21kCmN1cmwuZXhlIC1mc1NMIGh0dHBzOi8vYmFja3dyaXRlci5wZW50YWdyYXRpb24uY29tL2luc3RhbGw=\",\"text\":\"t one-shot Edit, Session Pick, batch\\nCheck, Anchor, Edit, Apply, result-binding, explicit Data modes, and\\nAdapter-owned Version and Update.\\n\\n## Quick start\\n\\nInstall the closed official `0.2.5` release with the command for your\\nplatform.\\n\\nLinux, macOS, or WSL:\\n\\n```sh\\ncurl -fsSL https://backwriter.pentagration.com/install.sh | sh\\n```\\n\\nWindows PowerShell:\\n\\n```powershell\\nirm https://backwriter.pentagration.com/install.ps1 | iex\\n```\\n\\nWindows CMD:\\n\\n```cmd\\ncurl.exe -fsSL https://backwriter.pentagration.com/install\"},{\"sequence\":3,\"ns\":\"166506990823095\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"LmNtZCAtbyBpbnN0YWxsLmNtZCAmJiBjYWxsIGluc3RhbGwuY21kICYmIGRlbCBpbnN0YWxsLmNtZApgYGAKClRoZSBQT1NJWCBpbnN0YWxsZXIgcGxhY2VzIGBid2AgYXQgYCRIT01FLy5sb2NhbC9iaW4vYndgOyB0aGUgUG93ZXJTaGVsbCBhbmQKQ01EIHBhdGhzIHBsYWNlIGBidy5leGVgIGF0IGAkSE9NRVwubG9jYWxcYmluXGJ3LmV4ZWAuIFRoZXkgZG8gbm90IGNoYW5nZQpgUEFUSGAsIGEgc2hlbGwgc3RhcnR1cCBmaWxlLCB0aGUgUG93ZXJTaGVsbCBwcm9maWxlLCBvciB0aGUgcmVnaXN0cnkuIEEgZnJlc2gKaW5zdGFsbCBwcmludHMgYEluc3RhbGxlZCBCYWNrd3JpdGVyOiA8dmVyc2lvbj5gOyByZXBsYWNpbmcgYW4gZXhpc3RpbmcKZGVzdGluYXRpb24gcHJpbnRzIGBVcGRhdGVkIEJhY2t3cml0ZXI6IDx2ZXJzaW9uPmAuIFRoZSBleGVjdXRhYmxlIHBhdGggYW5kCmBQQVRIYCBndWlkYW5jZSBhcmUgcHJpbnRlZCBzZXBhcmF0ZWx5IG9ubHkgd2hlbiB0aGUgaW5zdGFsbGF0aW9uIGRpcmU=\",\"text\":\".cmd -o install.cmd && call install.cmd && del install.cmd\\n```\\n\\nThe POSIX installer places `bw` at `$HOME/.local/bin/bw`; the PowerShell and\\nCMD paths place `bw.exe` at `$HOME\\\\.local\\\\bin\\\\bw.exe`. They do not change\\n`PATH`, a shell startup file, the PowerShell profile, or the registry. A fresh\\ninstall prints `Installed Backwriter: <version>`; replacing an existing\\ndestination prints `Updated Backwriter: <version>`. The executable path and\\n`PATH` guidance are printed separately only when the installation dire\"},{\"sequence\":4,\"ns\":\"166506990831325\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"Y3RvcnkgaXMKbm90IGFscmVhZHkgb24gYFBBVEhgLgoKIyMjIFZlcnNpb24KCmBgYHNoCmJ3IHZlcnNpb24KYGBgCgpUaGUgY2xvc2VkIG9mZmljaWFsIGAwLjIuNWAgYnVpbGQgcHJpbnRzIGV4YWN0bHk6CgpgYGB0ZXh0CkJhY2t3cml0ZXIgMC4yLjUKYGBgCgojIyMgVXBkYXRlCgpgYGBzaApidyB1cGRhdGUKYGBgCgpgYncgdXBkYXRlYCBkb3dubG9hZHMgYW5kIGRlbGVnYXRlcyB0byB0aGUgY3VycmVudCBvZmZpY2lhbCBpbnN0YWxsZXIuIFRoZQpjdXJyZW50IG1hbmlmZXN0IHNlbGVjdHMgdGhlIGNsb3NlZCBgMC4yLjVgIGRpc3RyaWJ1dGlvbi4gVGhlIGluc3RhbGxlcgpyZWFkcyB0aGF0IG1hbmlmZXN0LCB2ZXJpZmllcyB0aGUgc2VsZWN0ZWQgYXJ0aWZhY3QsIGFuZCBpbnN0YWxscyBvciByZWluc3RhbGxzCnRoYXQgbWFuaWZlc3QgdmVyc2lvbiBvbmx5IGFmdGVyIHZhbGlkYXRpb24gc3VjY2VlZHMuIEl0IGRvZXMgbm90IHJ1biBhCmJhY2tncm91bmQgdXBkYXRlciBvciBjb21wYXJlIHI=\",\"text\":\"ctory is\\nnot already on `PATH`.\\n\\n### Version\\n\\n```sh\\nbw version\\n```\\n\\nThe closed official `0.2.5` build prints exactly:\\n\\n```text\\nBackwriter 0.2.5\\n```\\n\\n### Update\\n\\n```sh\\nbw update\\n```\\n\\n`bw update` downloads and delegates to the current official installer. The\\ncurrent manifest selects the closed `0.2.5` distribution. The installer\\nreads that manifest, verifies the selected artifact, and installs or reinstalls\\nthat manifest version only after validation succeeds. It does not run a\\nbackground updater or compare r\"}],\"more\":true,\"active\":[]}\n"}
{"id":2,"bytes":5233,"sha256":"6dde957f3584a068a6c9b5c21e35b3cd2c6058bf51d3221085098f30b8678d10","content":"{\"id\":2,\"events\":[{\"sequence\":5,\"ns\":\"166506990840412\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"ZWxlYXNlIHZlcnNpb25zLiBUaGUgaW5zdGFsbGVyIGFjY2VwdHMgb25seSB0aGUKZXhhY3QgaW1tdXRhYmxlIGAwLjIuNGAgbWFuaWZlc3QgYW5kIGN1cnJlbnQgYDAuMi41YCBtYW5pZmVzdDsgYDAuMi4zYCwgYDAuMi4yYCwgYDAuMi4xYCwgYDAuMi4wYCwgc3RhYmxlCmAwLjEuMGAsIGFuZCBiZXRhLjMgYWNjZXB0YW5jZSBpcyByZXRpcmVkLiBUaGUgY29tbWFuZCBzdGlsbCBwZXJmb3JtcyBubwp2ZXJzaW9uIGNvbXBhcmlzb24gYW5kIGFkZHMgbm8gdmVyc2lvbiBndWFyZC4KClRoZSBwcm9kdWN0IGlzIEJhY2t3cml0ZXIuIFRoZSBzb3VyY2UgQ2FyZ28gcGFja2FnZSBhbmQgbGlicmFyeSBjcmF0ZSBhcmUKYGJhY2t3cml0ZXJgIGF0IHNvdXJjZS1yZWFkeSB1bnB1Ymxpc2hlZCBgMC4yLjZgOyBhIHNvdXJjZS1idWlsdCBgYncgdmVyc2lvbmAKcHJpbnRzIGBCYWNrd3JpdGVyIDAuMi42YC4gVGhlIHNvbGUgY2Fub25pY2FsIGV4ZWN1dGFibGUgYW5kIGV4dGVybmFsIEFkYXB0ZXIKY29tbWFuZCA=\",\"text\":\"elease versions. The installer accepts only the\\nexact immutable `0.2.4` manifest and current `0.2.5` manifest; `0.2.3`, `0.2.2`, `0.2.1`, `0.2.0`, stable\\n`0.1.0`, and beta.3 acceptance is retired. The command still performs no\\nversion comparison and adds no version guard.\\n\\nThe product is Backwriter. The source Cargo package and library crate are\\n`backwriter` at source-ready unpublished `0.2.6`; a source-built `bw version`\\nprints `Backwriter 0.2.6`. The sole canonical executable and external Adapter\\ncommand \"},{\"sequence\":6,\"ns\":\"166506990847542\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"YXJlIGBid2AuIFRoZXJlIGlzIG5vIGBiYWNrd3JpdGVyYCBiaW5hcnksIGFsaWFzLCBvciB3cmFwcGVyLiBUaGUKb2ZmaWNpYWwgaW5zdGFsbGVyIHJlbWFpbnMgc2VwYXJhdGUgYW5kIHNlbGVjdHMgdGhlIGNsb3NlZCBwdWJsaWMgYDAuMi41YApkaXN0cmlidXRpb24uCgpQcm9kdWN0aW9uIGBzcmMvKipgIHJlbWFpbnMgYnl0ZS1pZGVudGljYWwgdG8gR2F0ZSA2IGNhbmRpZGF0ZQpgMjJlNmRmMjM3NTVjZGM4MGIyOTliNzdiZTMxM2QzMDdiNjdiYzM3ZmAuIE9mZmljaWFsIGFydGlmYWN0cywgaW5zdGFsbGVycywKbWFuaWZlc3QsIFVwZGF0ZSB0YXJnZXQsIGFuZCB0aGUgZXhhY3QgNjgtZmlsZSBwdWJsaWMgZGlzdHJpYnV0aW9uIHJlbWFpbgpwdWJsaXNoZWQgYW5kIGNsb3NlZCBgMC4yLjVgIGZyb20gU291cmNlIEF1dGhvcml0eQpgZTQwMjJmYzA3M2U5ZGYzOTI4ZTFjMzgxN2IyNjZjZTkyMTIxYTAzY2AuIFVwZGF0ZSBwZXJmb3JtcyBubyB2ZXJzaW9uCmNvbXBhcmlzb24sIHNvIHNvdXJjZS0=\",\"text\":\"are `bw`. There is no `backwriter` binary, alias, or wrapper. The\\nofficial installer remains separate and selects the closed public `0.2.5`\\ndistribution.\\n\\nProduction `src/**` remains byte-identical to Gate 6 candidate\\n`22e6df23755cdc80b299b77be313d307b67bc37f`. Official artifacts, installers,\\nmanifest, Update target, and the exact 68-file public distribution remain\\npublished and closed `0.2.5` from Source Authority\\n`e4022fc073e9df3928e1c3817b266ce92121a03c`. Update performs no version\\ncomparison, so source-\"},{\"sequence\":7,\"ns\":\"166506990854449\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"YnVpbHQgYDAuMi42YCBzdGlsbCBpbnN0YWxscyBvciByZWluc3RhbGxzIHRoYXQgb2ZmaWNpYWwKYDAuMi41YCByZWxlYXNlIHVudGlsIGEgc2VwYXJhdGVseSBhdXRob3JpemVkIHJlbGVhc2UgY2xvc2VzIHRoZSBnYXAuCgojIyBBbmRkcmVzcy1maXJzdCBlZGl0aW5nCgpUaGUgZGVmYXVsdCBzb3VyY2UtY2hlY2tvdXQgcmVwbGFjZW1lbnQgZmxvdyBpczoKCjEuIFJ1biBgYncgLS1qc29uIHNlYXJjaCAuLi5gLgoyLiBTZWxlY3Qgb25lIG9jY3VycmVuY2UgZnJvbSBgb2NjdXJyZW5jZXNgIGFuZCBwYXNzIGl0cyBleGFjdCBlbWJlZGRlZCB2NQogICBgYW5kZHJlc3NgIG9iamVjdCB1bmNoYW5nZWQgYXMgb25lIGFyZ3YgdmFsdWUuCjMuIFJ1biBgYncgZWRpdCBhbmRkcmVzcyAnPG9wYXF1ZS12NS1vYmplY3Q+JyAnPG5ldy1jb250ZW50PidgLgoKSHVtYW4gU2VhcmNoIHJvd3MgYXJlIG5vdCBlbmNvZGVkIEFuZGRyZXNzIHZhbHVlcyBhbmQgY2Fubm90IGJlIEVkaXQgaW5wdXQuClRyZWF0IHRoZSA=\",\"text\":\"built `0.2.6` still installs or reinstalls that official\\n`0.2.5` release until a separately authorized release closes the gap.\\n\\n## Anddress-first editing\\n\\nThe default source-checkout replacement flow is:\\n\\n1. Run `bw --json search ...`.\\n2. Select one occurrence from `occurrences` and pass its exact embedded v5\\n   `anddress` object unchanged as one argv value.\\n3. Run `bw edit anddress '<opaque-v5-object>' '<new-content>'`.\\n\\nHuman Search rows are not encoded Anddress values and cannot be Edit input.\\nTreat the \"},{\"sequence\":8,\"ns\":\"166506990862362\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"c2VsZWN0ZWQgSlNPTiBvYmplY3QgYXMgb3BhcXVlOiBkbyBub3QgaW50ZXJwcmV0IG9yIHJld3JpdGUgaXRzIGhhc2gsCnJhbmdlLCBsZW5ndGgsIG9yIG90aGVyIGZpZWxkcy4gRmlsZSBhbmQgUGFyYWdyYXBoIENvbnRlbnQgaXMgdGhlIGV4YWN0CnJlcGxhY2VtZW50LiBMaW5lIENvbnRlbnQgaXMgYm9keS1vbmx5LCByZWplY3RzIE5VTCwgQ1IsIGFuZCBMRiwgYW5kIHByZXNlcnZlcwp0aGUgTm9uZSwgTEYsIENSLCBvciBDUkxGIHRlcm1pbmF0b3IgY2FycmllZCBieSB0aGUgZXhhY3QgdjUgTGluZS4gQSBMaW5lIE5VTAppcyBgZWRpdC5jb250ZW50X2NvbnRhaW5zX251bGA7IGEgTGluZSBDUiBvciBMRiBpcwpgZWRpdC5saW5lX2JvZHlfY29udGFpbnNfdGVybWluYXRvcmAuIFRoZSBsYXR0ZXIgZXhwbGFpbnMgdGhhdCBCYWNrd3JpdGVyIGFkZHMKdGhlIGN1cnJlbnQgdGVybWluYXRvciBhbmQgdGhhdCBhZHZhbmNlZCByYXcgU2Vzc2lvbiBFZGl0L0FwcGx5IG93bnMgZXhhY3QKZXh0ZW50IHI=\",\"text\":\"selected JSON object as opaque: do not interpret or rewrite its hash,\\nrange, length, or other fields. File and Paragraph Content is the exact\\nreplacement. Line Content is body-only, rejects NUL, CR, and LF, and preserves\\nthe None, LF, CR, or CRLF terminator carried by the exact v5 Line. A Line NUL\\nis `edit.content_contains_nul`; a Line CR or LF is\\n`edit.line_body_contains_terminator`. The latter explains that Backwriter adds\\nthe current terminator and that advanced raw Session Edit/Apply owns exact\\nextent r\"}],\"more\":true,\"active\":[]}\n"}
{"id":3,"bytes":5233,"sha256":"415464be4f6a5949380309d5fa936a1101cf5f3c36e226133a5b5e8cb8ec63f7","content":"{\"id\":3,\"events\":[{\"sequence\":9,\"ns\":\"166506990870267\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"ZXBsYWNlbWVudC4gQXBwbHkgYWxvbmUgY29uZmlybXMgdGhhdCBzb3VyY2Ugc3RhdGUgYmVmb3JlIHB1YmxpY2F0aW9uLgpWaWV3IG9yIFBpY2sgbWF5IGhlbHAgYSBjYWxsZXIgc2VsZWN0IGEgdGFyZ2V0OyBuZWl0aGVyIFZpZXcgbm9yIENoZWNrIGlzCnJlcXVpcmVkLgoKSHVtYW4gc3VjY2VzcyB3cml0ZXMgb25lIGV4YWN0IExGLXRlcm1pbmF0ZWQgcmVjZWlwdCByb3cuIGBVbmNoYW5nZWRgIGlzCmZvbGxvd2VkIGJ5IHRoZSBzdGlsbC1jdXJyZW50IGlucHV0IHY1IG9iamVjdDsgYENoYW5nZWRgIGlzIGZvbGxvd2VkIGJ5IHRoZQpmcmVzaCB2NSBvYmplY3Qgd2hlbiB0aGUgcmVzdWx0aW5nIEZpbGUsIExpbmUsIG9yIHVuaXF1ZSBQYXJhZ3JhcGggaGFzIG9uZSwKYW5kIG90aGVyd2lzZSBieSBgTm9uZWAuIFdpdGggbGVhZGluZyBgLS1qc29uYCwgdGhlIHNhbWUgcmVzdWx0IGlzIHRoZSBjb21wYWN0CkFkYXB0ZXItb25seSBgYncuY2xpLmVkaXQudjFgIG9iamVjdCB3aXRoIGBzY2hlbWFgLCA=\",\"text\":\"eplacement. Apply alone confirms that source state before publication.\\nView or Pick may help a caller select a target; neither View nor Check is\\nrequired.\\n\\nHuman success writes one exact LF-terminated receipt row. `Unchanged` is\\nfollowed by the still-current input v5 object; `Changed` is followed by the\\nfresh v5 object when the resulting File, Line, or unique Paragraph has one,\\nand otherwise by `None`. With leading `--json`, the same result is the compact\\nAdapter-only `bw.cli.edit.v1` object with `schema`, \"},{\"sequence\":10,\"ns\":\"166506990877397\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"YG91dGNvbWVgLCBhbmQgYGFuZGRyZXNzYAprZXlzIGluIHRoYXQgb3JkZXI7IGBhbmRkcmVzc2AgaXMgdGhlIGV4YWN0IHY1IG9iamVjdCBvciBKU09OIGBudWxsYC4gUmV1c2Ugb25seQp0aGUgYWRkcmVzcyByZXR1cm5lZCBieSB0aGUgcmVjZWlwdC4gQSBjaGFuZ2VkIFBhcmFncmFwaCB3aXRoIGBOb25lYCByZXF1aXJlcwphbiBleHBsaWNpdCBTZWFyY2ggYmVmb3JlIGxhdGVyIHRhcmdldCB3b3JrLiBFeGl0IGAxYCBpcyBuZWl0aGVyIGEgc3RhbGUtb25seQpjbGFzc2lmaWNhdGlvbiBub3IgcHJvb2YgdGhhdCBzb3VyY2UgYnl0ZXMgYXJlIHVuY2hhbmdlZCwgc28gaXQgbXVzdCBub3QKdHJpZ2dlciBhdXRvbWF0aWMgcmV0cnkuCgpPbmUtc2hvdCBDb250ZW50IGlzIGVpdGhlciBvbmUgVVRGLTggYXJndiB2YWx1ZSBvciB0aGUgZXhjbHVzaXZlIGAtLXN0ZGluYApzZWxlY3RvciBpbiB0aGF0IHBvc2l0aW9uOyBzdGRpbiBpcyByZWFkIHRvIEVPRiBhZnRlciB2NSBhZGRyZXNzIHZhbGlkYXRpb24=\",\"text\":\"`outcome`, and `anddress`\\nkeys in that order; `anddress` is the exact v5 object or JSON `null`. Reuse only\\nthe address returned by the receipt. A changed Paragraph with `None` requires\\nan explicit Search before later target work. Exit `1` is neither a stale-only\\nclassification nor proof that source bytes are unchanged, so it must not\\ntrigger automatic retry.\\n\\nOne-shot Content is either one UTF-8 argv value or the exclusive `--stdin`\\nselector in that position; stdin is read to EOF after v5 address validation\"},{\"sequence\":11,\"ns\":\"166506990884992\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"IGFuZApiZWZvcmUgUnVudGltZSBhY2Nlc3MuIEJvdGggZm9ybXMgaGF2ZSB0aGUgc2FtZSB0YXJnZXQtc3BlY2lmaWMgQ29udGVudCBydWxlcy4KRmlsZS9QYXJhZ3JhcGggQ1IgYW5kIExGIGFyZSBleGFjdCBDb250ZW50LCB3aGlsZSBMaW5lIGJvZHkgQ29udGVudCBuZXZlcgpzaWxlbnRseSBzdHJpcHMgYSB0cmFpbGluZyBuZXdsaW5lLiBMaXRlcmFsIGAtLWpzb25gIGFuZCBgLS1yYXdgIHJlbWFpbiBleGFjdApDb250ZW50OyBhIGxpdGVyYWwgYC0tc3RkaW5gIENvbnRlbnQgdmFsdWUgaXMgc3VwcGxpZWQgdGhyb3VnaCBzdGFuZGFyZCBpbnB1dC4KClJhdyBTZXNzaW9uIGlzIHRoZSBhZHZhbmNlZCBjb21wb3NpdGlvbiBzdXJmYWNlIGZvciBJbnNlcnQvRGVsZXRlL01vdmUvQ29weSwKUG9zaXRpb24sIEFuY2hvci9EYXRhIGxpZmV0aW1lLCBleHBsaWNpdCBiaW5kaW5ncywgYW5kIHNlcGFyYXRlIEFwcGx5LiBJdCBpcwpub3QgYSBwcmVyZXF1aXNpdGUgb3IgYWxpYXMgZm9yIG9yZGluYXJ5IFI=\",\"text\":\" and\\nbefore Runtime access. Both forms have the same target-specific Content rules.\\nFile/Paragraph CR and LF are exact Content, while Line body Content never\\nsilently strips a trailing newline. Literal `--json` and `--raw` remain exact\\nContent; a literal `--stdin` Content value is supplied through standard input.\\n\\nRaw Session is the advanced composition surface for Insert/Delete/Move/Copy,\\nPosition, Anchor/Data lifetime, explicit bindings, and separate Apply. It is\\nnot a prerequisite or alias for ordinary R\"},{\"sequence\":12,\"ns\":\"166506990891722\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"ZXBsYWNlLiBJdHMgZXhpc3RpbmcgcmF3CmBlZGl0IHJlcGxhY2VgIGFjY2VwdHMgY2FsbGVyLXByb3ZpZGVkIGV4YWN0IHJhbmdlIENvbnRlbnQsIGluY2x1ZGluZyBhbgpleHBsaWNpdCB0ZXJtaW5hdG9yIG9yIG11bHRpbGluZSByZXBsYWNlbWVudCwgYW5kIGBhcHBseSBAZWRpdGAgcHVibGlzaGVzIGl0CnNlcGFyYXRlbHkuIFRoZSBjYWxsZXIgb3ducyB0aGUgYmluZGluZywgaW5kZXgsIHF1b3RpbmcsIHRlcm1pbmF0b3IsIGFuZApwdWJsaWNhdGlvbiBib3VuZGFyeS4gR2VuZXJhbCByZXBsYWNlbWVudCBzaG91bGQgdXNlIG9uZS1zaG90IGJvZHkgQ29udGVudApmaXJzdDsgbm8gZXhhY3Qgb25lLXNob3QgZmxhZyBvciBhbHRlcm5hdGUgZXhlY3V0b3IgZXhpc3RzLgoKIyMjIFNoZWxsLWxvY2FsIHJlZmVyZW5jZXMKCmBidyBzaGVsbGAgYWxzbyBwcm92aWRlcyBhIHNob3J0LWxpdmVkIGludGVyYWN0aXZlIGZsb3cgd2l0aG91dCBhZGRpbmcgYSBDb3JlCndpcmUgb3IgcGVyc2lzdGVudCBpZGU=\",\"text\":\"eplace. Its existing raw\\n`edit replace` accepts caller-provided exact range Content, including an\\nexplicit terminator or multiline replacement, and `apply @edit` publishes it\\nseparately. The caller owns the binding, index, quoting, terminator, and\\npublication boundary. General replacement should use one-shot body Content\\nfirst; no exact one-shot flag or alternate executor exists.\\n\\n### Shell-local references\\n\\n`bw shell` also provides a short-lived interactive flow without adding a Core\\nwire or persistent ide\"}],\"more\":true,\"active\":[]}\n"}
{"id":4,"bytes":5256,"sha256":"e92e9fb1e0ee09067562a93a3e0ed343c60a441afe65411f18b19c1afb659134","content":"{\"id\":4,\"events\":[{\"sequence\":13,\"ns\":\"166506990898481\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"bnRpdHk6CgpgYGB0ZXh0CnNlYXJjaCBsaW5lIG5lZWRsZQp2aWV3IEAwCnJlcGxhY2UgQDEgcmVwbGFjZW1lbnQKY2hlY2sgQDIgQDMKZXhpdApgYGAKClF1b3RlIG9uZSBkaXJlY3Qtc2hlbGwgcXVlcnkgb3IgcmVwbGFjZW1lbnQgYXJndW1lbnQgd2hlbiBpdCBjb250YWlucwp3aGl0ZXNwYWNlOyB0aGUgcXVvdGVzIGFyZSBzaGVsbCBzeW50YXggYW5kIGFyZSBub3QgcGFydCBvZiB0aGUgZXhhY3QgbGl0ZXJhbApvciByZXBsYWNlbWVudCBDb250ZW50OgoKYGBgdGV4dApzZWFyY2ggbGluZSAiZHVwbGljYXRlID0gb25lIgpyZXBsYWNlIEAxICJkdXBsaWNhdGUgPSB0d28iCmBgYAoKU3VjY2Vzc2Z1bCBkaXJlY3QgYHNlYXJjaGAgYW5kIHByb2plY3RlZCBkaXJlY3QgYHZpZXdgIGFwcGVuZCBgQE5gIHJlZmVyZW5jZXMKaW4gb3V0cHV0IG9yZGVyLCBpbmNsdWRpbmcgZHVwbGljYXRlcy4gRGlyZWN0IGBjaGVjayA8UkVGPi4uLmAgcmVzb2x2ZXMgZXZlcnkKcmVmZXJlbmNlIGJlZm9yZSBSdW50aW0=\",\"text\":\"ntity:\\n\\n```text\\nsearch line needle\\nview @0\\nreplace @1 replacement\\ncheck @2 @3\\nexit\\n```\\n\\nQuote one direct-shell query or replacement argument when it contains\\nwhitespace; the quotes are shell syntax and are not part of the exact literal\\nor replacement Content:\\n\\n```text\\nsearch line \\\"duplicate = one\\\"\\nreplace @1 \\\"duplicate = two\\\"\\n```\\n\\nSuccessful direct `search` and projected direct `view` append `@N` references\\nin output order, including duplicates. Direct `check <REF>...` resolves every\\nreference before Runtim\"},{\"sequence\":14,\"ns\":\"166506990906401\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"ZSBhY2Nlc3MsIHdyaXRlcyBvbmUgQ3VycmVudC9Ob3RDdXJyZW50L1VuYXZhaWxhYmxlCnN0YXRlIHBlciBpbnB1dCwgYW5kIGFwcGVuZHMgYSBmcmVzaCBzbG90IG9ubHkgZm9yIEN1cnJlbnQuIGByZXBsYWNlIEBOIDxjb250ZW50PmAgdXNlcyB0aGUgc2FtZQp0YXJnZXQtYXdhcmUgQ29udGVudCBydWxlcyBhcyBvbmUtc2hvdCBFZGl0IGFuZCBhcHBlbmRzIGEgZnJlc2ggcmVmZXJlbmNlIGZvcgpgVW5jaGFuZ2VkYCBvciBgQ2hhbmdlZGAgd2hlbiB0aGUgcmVjZWlwdCBoYXMgYW4gQW5kZHJlc3M7IGBDaGFuZ2VkXHROb25lYAphZGRzIG5vbmUuIFNsb3RzIGFyZSB6ZXJvLWJhc2VkIGNhbm9uaWNhbCB1bnNpZ25lZCBkZWNpbWFscywgYXBwZW5kLW9ubHksIGFuZApkaXNjYXJkZWQgYXQgYGV4aXRgIG9yIEVPRi4gYEBuYW1lYCBhbmQgYEBuYW1lW2luZGV4XWAgcmVtYWluIHRoZSBhZHZhbmNlZCByYXcKU2Vzc2lvbiBmb3JtczsgYGxldCBuYW1lID0gQE5gIGV4cGxpY2l0bHkgY2xvbmVzIGEgbnU=\",\"text\":\"e access, writes one Current/NotCurrent/Unavailable\\nstate per input, and appends a fresh slot only for Current. `replace @N <content>` uses the same\\ntarget-aware Content rules as one-shot Edit and appends a fresh reference for\\n`Unchanged` or `Changed` when the receipt has an Anddress; `Changed\\\\tNone`\\nadds none. Slots are zero-based canonical unsigned decimals, append-only, and\\ndiscarded at `exit` or EOF. `@name` and `@name[index]` remain the advanced raw\\nSession forms; `let name = @N` explicitly clones a nu\"},{\"sequence\":15,\"ns\":\"166506990913114\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"bWVyaWMgc2xvdCBpbnRvIHRoYXQKZXhpc3RpbmcgbmFtZWQgQW5kZHJlc3MgYmluZGluZy4gUmF3IGBlZGl0IHJlcGxhY2VgIGFuZCBzZXBhcmF0ZSBgYXBwbHkgQGVkaXRgCnJlbWFpbiB0aGUgYWR2YW5jZWQgZXhhY3QtcmFuZ2UgcGF0aC4KClRoZSBkZWZhdWx0IHdvcmtzcGFjZSBpcyB0aGUgcHJvY2VzcyBjdXJyZW50IHdvcmtpbmcgZGlyZWN0b3J5LiBBbiBleHBsaWNpdApgLS13b3Jrc3BhY2VgIG11c3QgYmUgYWJzb2x1dGUgYW5kIGlzIGNoZWNrZWQgYnkgUnVudGltZS4gU2VhcmNoIGFkbWl0cyBgLmAgYnkKZGVmYXVsdDsgcmVwZWF0IGAtLWFkbWl0IExPR0lDQUxfUEFUSGAgYmVmb3JlIGBzZWFyY2hgIHRvIG5hcnJvdyBhZG1pc3Npb24uCkFmdGVyIHRoZSBxdWVyeSwgcmVwZWF0IGAtLXNvdXJjZSBMT0dJQ0FMX1BBVEhgIG9yIGAtLXN1YnRyZWUgTE9HSUNBTF9QQVRIYCB0bwpuYXJyb3cgYSBTZWFyY2ggc2NvcGUuIFdpdGhvdXQgYSBzY29wZSBzZWxlY3RvciwgU2VhcmNoIGNvdmVycyA=\",\"text\":\"meric slot into that\\nexisting named Anddress binding. Raw `edit replace` and separate `apply @edit`\\nremain the advanced exact-range path.\\n\\nThe default workspace is the process current working directory. An explicit\\n`--workspace` must be absolute and is checked by Runtime. Search admits `.` by\\ndefault; repeat `--admit LOGICAL_PATH` before `search` to narrow admission.\\nAfter the query, repeat `--source LOGICAL_PATH` or `--subtree LOGICAL_PATH` to\\nnarrow a Search scope. Without a scope selector, Search covers \"},{\"sequence\":16,\"ns\":\"166506990920395\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"YWxsIGFkbWl0dGVkCnNvdXJjZXMuCgojIyBDdXJyZW50IENMSSBzY29wZQoKYGJ3YCBjdXJyZW50bHkgaW1wbGVtZW50cyBBZGFwdGVyLW93bmVkIG9uZS1zaG90IFZlcnNpb24gYW5kIFVwZGF0ZSwgb25lLXNob3QKaHVtYW4gb3IgSlNPTiBTZWFyY2gsIFZpZXcsIENoZWNrLCBhbmQgRWRpdCwgcmF3IFZpZXcsIEFuZGRyZXNzLWZpcnN0IG9uZS1zaG90IEVkaXQsCnBsdXMgU2Vzc2lvbiBQaWNrLCBiYXRjaCBDaGVjaywgQW5jaG9yLCBFZGl0LCBBcHBseSwgYW5kIERhdGE6CgpgYGB0ZXh0CmJ3IHZlcnNpb24KYncgdXBkYXRlCmJ3IFstLXdvcmtzcGFjZSBBQlNPTFVURV9QQVRIXSBbLS1hZG1pdCBMT0dJQ0FMX1BBVEhdLi4uCiAgICBzZWFyY2ggPGxpbmV8cGFyYWdyYXBofGZpbGU+IDxxdWVyeT4KICAgIFstLXNvdXJjZSBMT0dJQ0FMX1BBVEggfCAtLXN1YnRyZWUgTE9HSUNBTF9QQVRIXS4uLgpidyBbLS13b3Jrc3BhY2UgQUJTT0xVVEVfUEFUSF0gWy0tYWRtaXQgTE9HSUNBTF9QQVRIXS4uLgo=\",\"text\":\"all admitted\\nsources.\\n\\n## Current CLI scope\\n\\n`bw` currently implements Adapter-owned one-shot Version and Update, one-shot\\nhuman or JSON Search, View, Check, and Edit, raw View, Anddress-first one-shot Edit,\\nplus Session Pick, batch Check, Anchor, Edit, Apply, and Data:\\n\\n```text\\nbw version\\nbw update\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\\n    search <line|paragraph|file> <query>\\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\\n\"}],\"more\":true,\"active\":[]}\n"}
{"id":5,"bytes":5237,"sha256":"16dc8d48238631f6e0a850d25e98b17b486a3ab21df6dbb98f84a695315d86c1","content":"{\"id\":5,\"events\":[{\"sequence\":17,\"ns\":\"166506990926913\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"ICAgIHNlYXJjaCAvZmlsZSA8bG9naWNhbC1wYXRoPgpidyBbLS13b3Jrc3BhY2UgQUJTT0xVVEVfUEFUSF0gWy0tYWRtaXQgTE9HSUNBTF9QQVRIXS4uLiAtLWpzb24KICAgIHNlYXJjaCA8bGluZXxwYXJhZ3JhcGh8ZmlsZT4gPHF1ZXJ5PgogICAgWy0tc291cmNlIExPR0lDQUxfUEFUSCB8IC0tc3VidHJlZSBMT0dJQ0FMX1BBVEhdLi4uCmJ3IFstLXdvcmtzcGFjZSBBQlNPTFVURV9QQVRIXSBbLS1hZG1pdCBMT0dJQ0FMX1BBVEhdLi4uIC0tanNvbgogICAgc2VhcmNoIC9maWxlIDxsb2dpY2FsLXBhdGg+CmJ3IFstLXdvcmtzcGFjZSBBQlNPTFVURV9QQVRIXSBbLS1hZG1pdCBMT0dJQ0FMX1BBVEhdLi4uIC0tanNvbgogICAgdmlldyBhbmRkcmVzcyA8ZW5jb2RlZC12NS1BbmRkcmVzcz4uLi4gWy0tYXMgPGxpbmV8cGFyYWdyYXBofGZpbGU+XQpidyBbLS13b3Jrc3BhY2UgQUJTT0xVVEVfUEFUSF0gWy0tYWRtaXQgTE9HSUNBTF9QQVRIXS4uLiAtLWpzb24KICAgIGNoZWNrIGFuZGRyZXNzIDxlbmM=\",\"text\":\"    search /file <logical-path>\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\\n    search <line|paragraph|file> <query>\\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\\n    search /file <logical-path>\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\\n    view anddress <encoded-v5-Anddress>... [--as <line|paragraph|file>]\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\\n    check anddress <enc\"},{\"sequence\":18,\"ns\":\"166506990932566\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"b2RlZC12NS1BbmRkcmVzcz4uLi4KYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4gLS1yYXcKICAgIHZpZXcgYW5kZHJlc3MgPGVuY29kZWQtdjUtQW5kZHJlc3M+IFstLWFzIDxsaW5lfHBhcmFncmFwaHxmaWxlPl0KYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4KICAgIHZpZXcgYW5kZHJlc3MgPGVuY29kZWQtdjUtQW5kZHJlc3M+IFstLWFzIDxsaW5lfHBhcmFncmFwaHxmaWxlPl0KYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4KICAgIGNoZWNrIGFuZGRyZXNzIDxlbmNvZGVkLXY1LUFuZGRyZXNzPgpidyBbLS13b3Jrc3BhY2UgQUJTT0xVVEVfUEFUSF0gWy0tYWRtaXQgTE9HSUNBTF9QQVRIXS4uLgogICAgZWRpdCBhbmRkcmVzcyA8ZW5jb2RlZC12NS1BbmRkcmVzcz4gPGNvbnRlbnQ+CmJ3IFstLXdvcmtzcGFjZSBBQlNPTFVURV9QQVQ=\",\"text\":\"oded-v5-Anddress>...\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --raw\\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\\n    check anddress <encoded-v5-Anddress>\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\\n    edit anddress <encoded-v5-Anddress> <content>\\nbw [--workspace ABSOLUTE_PAT\"},{\"sequence\":19,\"ns\":\"166506990938116\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"SF0gWy0tYWRtaXQgTE9HSUNBTF9QQVRIXS4uLgogICAgZWRpdCBhbmRkcmVzcyA8ZW5jb2RlZC12NS1BbmRkcmVzcz4gLS1zdGRpbgpidyBbLS13b3Jrc3BhY2UgQUJTT0xVVEVfUEFUSF0gWy0tYWRtaXQgTE9HSUNBTF9QQVRIXS4uLiAtLWpzb24KICAgIGVkaXQgYW5kZHJlc3MgPGVuY29kZWQtdjUtQW5kZHJlc3M+IDxjb250ZW50PgpidyBbLS13b3Jrc3BhY2UgQUJTT0xVVEVfUEFUSF0gWy0tYWRtaXQgTE9HSUNBTF9QQVRIXS4uLiAtLWpzb24KICAgIGVkaXQgYW5kZHJlc3MgPGVuY29kZWQtdjUtQW5kZHJlc3M+IC0tc3RkaW4KYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4gc2hlbGwKYGBgCgpWZXJzaW9uIGFuZCBVcGRhdGUgZG8gbm90IGNhbGwgQmFja3dyaXRlciBDb3JlIG9yIFJ1bnRpbWUgYW5kIGNyZWF0ZSBubyBDb3JlCndpcmUgb3IgY2FwYWJpbGl0eSB3b3JrZmxvdy4KCkNvbnRlbnQgU2VhcmNoIHByZXNlcnZlcyBDb3JlIGxpdGU=\",\"text\":\"H] [--admit LOGICAL_PATH]...\\n    edit anddress <encoded-v5-Anddress> --stdin\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\\n    edit anddress <encoded-v5-Anddress> <content>\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\\n    edit anddress <encoded-v5-Anddress> --stdin\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\\n```\\n\\nVersion and Update do not call Backwriter Core or Runtime and create no Core\\nwire or capability workflow.\\n\\nContent Search preserves Core lite\"},{\"sequence\":20,\"ns\":\"166506990947727\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"cmFsIHZhbGlkYXRpb24sIHNjb3BlLCBwcm9qZWN0aW9uLCBhbmQKZGV0ZXJtaW5pc3RpYyByZXN1bHQgb3JkZXIuIFRoZSBkaXN0aW5jdCBgc2VhcmNoIC9maWxlYCBmb3JtIHZhbGlkYXRlcyBvbmUKbG9naWNhbCBwYXRoIGFuZCByZXR1cm5zIHRoZSBjdXJyZW50IEZpbGUgQW5kZHJlc3MgZm9yIGFuIGFkbWl0dGVkIHJlZ3VsYXIKVVRGLTgsIE5VTC1mcmVlIHNvdXJjZSByZWdhcmRsZXNzIG9mIHdoZXRoZXIgaXQgaXMgZW1wdHkgb3IgY29udGFpbnMgbWF0Y2hpbmcKdGV4dC4gTWlzc2luZyBwYXRocyBhbmQgZGlyZWN0b3JpZXMgcmV0dXJuIEVtcHR5OyB0aGUgZm9ybSBoYXMgbm8gc2NvcGUKc2VsZWN0b3JzIG9yIHN5bnRoZXRpYyBjb250ZW50IHF1ZXJ5LiBWaWV3IGRlY29kZXMgdjUgQW5kZHJlc3NlcyBhbmQgcHJvamVjdHMKZWFjaCB0byBpdHNlbGYgb3Igb25lIGFuY2VzdG9yIGJlZm9yZSByZWFkaW5nIGl0cyBleGFjdCBjb250ZW50LiBBIHNpbmdsZSBpbnB1dApkZWZhdWx0cyB0byBzZWw=\",\"text\":\"ral validation, scope, projection, and\\ndeterministic result order. The distinct `search /file` form validates one\\nlogical path and returns the current File Anddress for an admitted regular\\nUTF-8, NUL-free source regardless of whether it is empty or contains matching\\ntext. Missing paths and directories return Empty; the form has no scope\\nselectors or synthetic content query. View decodes v5 Anddresses and projects\\neach to itself or one ancestor before reading its exact content. A single input\\ndefaults to sel\"}],\"more\":true,\"active\":[]}\n"}
{"id":6,"bytes":5232,"sha256":"e01756d923058d507c4fc78fb068fc4699c9ffad4d0ddbc8db084658ce41f994","content":"{\"id\":6,\"events\":[{\"sequence\":21,\"ns\":\"166506990953233\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"ZiBwcm9qZWN0aW9uLiBgLS1hc2AgY2hvb3NlcyBvbmUgdGFyZ2V0IGtpbmQ7IG11bHRpcGxlIGlucHV0cwpyZXF1aXJlIGJvdGggYC0tanNvbmAgYW5kIGAtLWFzYC4gQ2hlY2sgZGVjb2RlcyBldmVyeSB2NSBvcGVyYW5kIGJlZm9yZSBSdW50aW1lCmFjY2Vzcy4gT25lIGh1bWFuIGlucHV0IHdyaXRlcyBgQ3VycmVudGAsIGBOb3RDdXJyZW50YCwgb3IgYFVuYXZhaWxhYmxlYDsgYQpiYXRjaCByZXF1aXJlcyBgLS1qc29uYCBhbmQgcHJlc2VydmVzIG9uZSBvdXRjb21lIHBlciBpbnB1dC4gU2VhcmNoLCBWaWV3LApDaGVjaywgYW5kIEVkaXQgYC0tanNvbmAKd3JpdGUgY29tcGFjdCBBZGFwdGVyIG9iamVjdHMgd2l0aCBleGFjdCBlbWJlZGRlZCB2NSBBbmRkcmVzcyBvYmplY3RzIHdoZXJlCmFwcGxpY2FibGU7IGVhY2ggaXMgYW4gQWRhcHRlciBzY2hlbWEsIG5vdCBhIHNlY29uZCBDb3JlIHdpcmUuIFZpZXcgdXNlcyB0aGUKaGFyZC1jdXQgYGJ3LmNsaS52aWV3LnYyYCBvdXRjb21lcyBhcnJheSA=\",\"text\":\"f projection. `--as` chooses one target kind; multiple inputs\\nrequire both `--json` and `--as`. Check decodes every v5 operand before Runtime\\naccess. One human input writes `Current`, `NotCurrent`, or `Unavailable`; a\\nbatch requires `--json` and preserves one outcome per input. Search, View,\\nCheck, and Edit `--json`\\nwrite compact Adapter objects with exact embedded v5 Anddress objects where\\napplicable; each is an Adapter schema, not a second Core wire. View uses the\\nhard-cut `bw.cli.view.v2` outcomes array \"},{\"sequence\":22,\"ns\":\"166506990958639\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"Zm9yIGJvdGggc2luZ2xlIGFuZCBiYXRjaCByZXN1bHRzOwpDaGVjayB1c2VzIHRoZSBoYXJkLWN1dCBgYncuY2xpLmNoZWNrLnYyYCBvcmRlcmVkIG91dGNvbWVzIGFycmF5LgpSYXcgVmlldyBpcyBhbiBleHBsaWNpdCBBZGFwdGVyIGV4YWN0LXRleHQgbW9kZSB0aGF0IHJldXNlcyB0aGUgb3JkaW5hcnkgVmlldwpwcm9qZWN0aW9uIHdpdGhvdXQgYSBDb3JlIHdpcmUgb3IgY2hhbmdlZCBWaWV3IG1lYW5pbmcuCkh1bWFuIFNlYXJjaCwgVmlldywgYW5kIENoZWNrIGtlZXAgdGhlaXIgZXhpc3RpbmcgcHJvamVjdGlvbnM7IGh1bWFuIEVkaXQKcmVjZWlwdHMgaW50ZW50aW9uYWxseSByZXR1cm4gdGhlIGV4YWN0IGN1cnJlbnQgdjUgb2JqZWN0IHdoZW4gb25lIGV4aXN0cy4KVGhlIFNlc3Npb24gaG9sZHMgb25lIFJ1bnRpbWUgdW50aWwgRU9GCm9yIGBleGl0YCBhbmQgaGFzIGV4cGxpY2l0IGxvY2FsIFNlYXJjaCwgUGljaywgQW5kZHJlc3MsIEVkaXQsIFZpZXcsIGFuZCBDaGVjawpiaW5kaW5ncyA=\",\"text\":\"for both single and batch results;\\nCheck uses the hard-cut `bw.cli.check.v2` ordered outcomes array.\\nRaw View is an explicit Adapter exact-text mode that reuses the ordinary View\\nprojection without a Core wire or changed View meaning.\\nHuman Search, View, and Check keep their existing projections; human Edit\\nreceipts intentionally return the exact current v5 object when one exists.\\nThe Session holds one Runtime until EOF\\nor `exit` and has explicit local Search, Pick, Anddress, Edit, View, and Check\\nbindings \"},{\"sequence\":23,\"ns\":\"166506990964002\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"cGx1cyBub24tYWxpYXNpbmcgQW5jaGVkcmVzcyBoYW5kbGVzLiBJdCBvd25zIG9uZSBleHBsaWNpdCBgRGF0YVN0b3JlYApmb3IgdGhlIFNlc3Npb24gb25seTsgbmFtZXMgYXJlIHR5cGVkIGFuZCBuZXZlciBwZXJzaXN0IHBhc3QgRU9GIG9yIGBleGl0YC4KU2Vzc2lvbiBQaWNrCnBhc3NlcyBhIG5hbWVkIFNlYXJjaCBvciBQaWNrIGNvbGxlY3Rpb24gYW5kIGFuIEFkYXB0ZXItcGFyc2VkIHByZWRpY2F0ZSBkaXJlY3RseQp0byBDb3JlOyBTZXNzaW9uIGJhdGNoIENoZWNrIHBhc3NlcyBhIG5hbWVkIG1hdGNoaW5nIG91dGNvbWUgZGlyZWN0bHkgdG8gaXRzClJ1bnRpbWUgYmF0Y2ggc2VhbSBhbmQgcHJpbnRzIG9ubHkgcmVwb3J0IGNvdW50cy4gU2Vzc2lvbiBBbmNob3IgY3JlYXRlcyBhIGxpdmUKaGFuZGxlIG9ubHkgdGhyb3VnaCBgbGV0IDxuYW1lPiA9IGFuY2hvciBjcmVhdGUgPGFuZGRyZXNzLXJlZj5gLCB2aWV3cyBpdCB0aHJvdWdoCmB2aWV3IGFuY2hvcmVkIEA8bmFtZT5gLCBhbmQgY2E=\",\"text\":\"plus non-aliasing Anchedress handles. It owns one explicit `DataStore`\\nfor the Session only; names are typed and never persist past EOF or `exit`.\\nSession Pick\\npasses a named Search or Pick collection and an Adapter-parsed predicate directly\\nto Core; Session batch Check passes a named matching outcome directly to its\\nRuntime batch seam and prints only report counts. Session Anchor creates a live\\nhandle only through `let <name> = anchor create <anddress-ref>`, views it through\\n`view anchored @<name>`, and ca\"},{\"sequence\":24,\"ns\":\"166506990969954\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"biBpbnZhbGlkYXRlIGl0cyBsb2dpY2FsIHNvdXJjZSB3aXRoIGBhbmNob3IKaW52YWxpZGF0ZS1zb3VyY2UgPGxvZ2ljYWwtcGF0aD5gLiBPbmUtc2hvdCBEYXRhIGFuZCBBbmNob3IgYXJlIGludGVudGlvbmFsbHkKdW5zdXBwb3J0ZWQgYmVjYXVzZSB0aGVpciBEYXRhU3RvcmUgYW5kIGxpdmUtaGFuZGxlIGNvbnRyYWN0cyByZXF1aXJlIFNlc3Npb24KbGlmZXRpbWUuIE9uZS1zaG90IFBpY2ssIHJhdyBFZGl0LW9iamVjdCB0cmFuc3BvcnQsIGFuZCBhIHNlcGFyYXRlCkFwcGx5IHRyYW5zcG9ydCBhd2FpdCBjb2xsZWN0aW9uIG9yIEVkaXQgdHJhbnNwb3J0IHNjaGVtYSBhdXRob3JpdHkuIFRoZQpkaXN0aW5jdCBBbmRkcmVzcy1maXJzdCBvbmUtc2hvdCBFZGl0IGFib3ZlIGlzIGltcGxlbWVudGVkLiBSYXcgb3V0cHV0IG90aGVyCnRoYW4gb25lLXNob3QgVmlldyBhbmQgZnVydGhlciBTZXNzaW9uIGJlaGF2aW9yIHJlbWFpbiBkZWZlcnJlZC4KCiMjIFNjb3BlCgpCYWNrd3JpdGVyIGlzIG5vdCA=\",\"text\":\"n invalidate its logical source with `anchor\\ninvalidate-source <logical-path>`. One-shot Data and Anchor are intentionally\\nunsupported because their DataStore and live-handle contracts require Session\\nlifetime. One-shot Pick, raw Edit-object transport, and a separate\\nApply transport await collection or Edit transport schema authority. The\\ndistinct Anddress-first one-shot Edit above is implemented. Raw output other\\nthan one-shot View and further Session behavior remain deferred.\\n\\n## Scope\\n\\nBackwriter is not \"}],\"more\":true,\"active\":[]}\n"}
{"id":7,"bytes":5244,"sha256":"2b6a920305e25c7d6256df67ae4a015d735f226c74bf2dc8d6281dc30dd12ae3","content":"{\"id\":7,\"events\":[{\"sequence\":25,\"ns\":\"166506990976135\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"R2l0LCBhIGZpbGUgd2F0Y2hlciwgZGFlbW9uLCBwZXJzaXN0ZW50IGluZGV4LCBvciBlZGl0b3IgVUkuCkl0IHJlYWRzIGFkbWl0dGVkIGN1cnJlbnQgV29ya3NwYWNlIFNvdXJjZSB0aHJvdWdoIFJ1bnRpbWUncyBzYWZlIG5vLWZvbGxvdwphY2Nlc3MuIEl0IGRvZXMgbm90IG1vZGVsIGJyYW5jaGVzLCBtZXJnZXMsIGhpc3RvcnksIGF1dG9tYXRpYyByZS1ldmFsdWF0aW9uLApvciBlZGl0b3IgYnVmZmVycy4KCkFwcGx5IHVzZXMgaXRzIGFjY2VwdGVkIGN1cnJlbnQgb2JzZXJ2YXRpb24gYW5kIGRvZXMgbm90IGNvb3JkaW5hdGUgY29uY3VycmVudAp3cml0ZXJzLiBXcml0ZXJzIG1heSByYWNlIGFuZCBvbmUgcHVibGljYXRpb24gbWF5IG92ZXJ3cml0ZSBhbm90aGVyCnNvdXJjZS12aXNpYmxlIGNoYW5nZTsgaG9zdHMgcmVxdWlyaW5nIGEgc3Ryb25nZXIgZ3VhcmFudGVlIGNvb3JkaW5hdGUgb3V0c2lkZQpCYWNrd3JpdGVyLgoKIyMgQnVpbGQgZnJvbSBzb3VyY2UgYW5kIHRlc3QKCmBgYHNoCmM=\",\"text\":\"Git, a file watcher, daemon, persistent index, or editor UI.\\nIt reads admitted current Workspace Source through Runtime's safe no-follow\\naccess. It does not model branches, merges, history, automatic re-evaluation,\\nor editor buffers.\\n\\nApply uses its accepted current observation and does not coordinate concurrent\\nwriters. Writers may race and one publication may overwrite another\\nsource-visible change; hosts requiring a stronger guarantee coordinate outside\\nBackwriter.\\n\\n## Build from source and test\\n\\n```sh\\nc\"},{\"sequence\":26,\"ns\":\"166506990981656\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"YXJnbyBidWlsZCAtLW9mZmxpbmUgLS1sb2NrZWQgLS1yZWxlYXNlCi4vdGFyZ2V0L3JlbGVhc2UvYncgc2VhcmNoIGxpbmUgIm5lZWRsZSIKLi90YXJnZXQvcmVsZWFzZS9idyAtLXdvcmtzcGFjZSAvcGF0aC9wcm9qZWN0IHNlYXJjaCBwYXJhZ3JhcGggIm5lZWRsZSIKY2FyZ28gdGVzdCAtLW9mZmxpbmUgLS1sb2NrZWQKYGBgCgojIyBPZmZpY2lhbCBkZXNrdG9wIGRpc3RyaWJ1dGlvbgoKVGhlIG9mZmljaWFsIGRpc3RyaWJ1dGlvbiBhdXRob3JpdHkgaXMKW2h0dHBzOi8vYmFja3dyaXRlci5wZW50YWdyYXRpb24uY29tXShodHRwczovL2JhY2t3cml0ZXIucGVudGFncmF0aW9uLmNvbSkuCkl0IHB1Ymxpc2hlcyB0aGUgY2xvc2VkIEJhY2t3cml0ZXIgYDAuMi41YCByZWxlYXNlIGZvciBMaW51eC9XU0wgeDg2XzY0LAptYWNPUyBhcm02NCwgbWFjT1MgeDg2XzY0LCBhbmQgV2luZG93cyB4ODZfNjQgZnJvbSBTb3VyY2UgQXV0aG9yaXR5IHJldmlzaW9uCmBlNDAyMmZjMDczZTlkZjM5MjhlMWMzODE=\",\"text\":\"argo build --offline --locked --release\\n./target/release/bw search line \\\"needle\\\"\\n./target/release/bw --workspace /path/project search paragraph \\\"needle\\\"\\ncargo test --offline --locked\\n```\\n\\n## Official desktop distribution\\n\\nThe official distribution authority is\\n[https://backwriter.pentagration.com](https://backwriter.pentagration.com).\\nIt publishes the closed Backwriter `0.2.5` release for Linux/WSL x86_64,\\nmacOS arm64, macOS x86_64, and Windows x86_64 from Source Authority revision\\n`e4022fc073e9df3928e1c381\"},{\"sequence\":27,\"ns\":\"166506990987731\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"N2IyNjZjZTkyMTIxYTAzY2AuIExpbnV4IHVzZXMgY2Fub25pY2FsIHRhcmdldApgeDg2XzY0LXVua25vd24tbGludXgtbXVzbGA7CmB4ODZfNjQtdW5rbm93bi1saW51eC1nbnVgIHJlbWFpbnMgdGhlIGxvY2FsIGRldmVsb3BtZW50L3Rlc3QtaG9zdCB0YXJnZXQuCm1hY09TIHVzZXMgYGFhcmNoNjQtYXBwbGUtZGFyd2luYCB3aXRoIG1pbmltdW0gbWFjT1MgMTEuMCBhbmQKYHg4Nl82NC1hcHBsZS1kYXJ3aW5gIHdpdGggbWluaW11bSBtYWNPUyAxMC4xMi4gVGhlIG1hY09TIGFydGlmYWN0cyByZWNlaXZlCnN0YXRpYyBjcm9zcy1idWlsZCB2ZXJpZmljYXRpb24gYnV0IGFyZSBub3QgY2xhaW1lZCB0byBoYXZlIGJlZW4gZXhlY3V0ZWQgb24gYQpuYXRpdmUgTWFjIGJlZm9yZSBwdWJsaWNhdGlvbi4gV2luZG93cyB1c2VzIGB4ODZfNjQtcGMtd2luZG93cy1nbnVgIGFuZCB0aGUKY2Fub25pY2FsIGV4ZWN1dGFibGUgYGJ3LmV4ZWA7IGl0cyBzdGF0aWMgY3Jvc3MtYnVpbGQgdmVyaWZpY2F0aW9uIGQ=\",\"text\":\"7b266ce92121a03c`. Linux uses canonical target\\n`x86_64-unknown-linux-musl`;\\n`x86_64-unknown-linux-gnu` remains the local development/test-host target.\\nmacOS uses `aarch64-apple-darwin` with minimum macOS 11.0 and\\n`x86_64-apple-darwin` with minimum macOS 10.12. The macOS artifacts receive\\nstatic cross-build verification but are not claimed to have been executed on a\\nnative Mac before publication. Windows uses `x86_64-pc-windows-gnu` and the\\ncanonical executable `bw.exe`; its static cross-build verification d\"},{\"sequence\":28,\"ns\":\"166506990994078\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"b2VzIG5vdApjbGFpbSBuYXRpdmUgV2luZG93cywgUG93ZXJTaGVsbCwgb3IgQ01EIGV4ZWN1dGlvbi4gTGludXggYXJtNjQgaXMgbm90IGN1cnJlbnRseQpwcm92aWRlZCwgYW5kIG5vIHVuaXZlcnNhbCBob3N0LWNvbXBhdGliaWxpdHkgY2xhaW0gaXMgbWFkZS4KCmBpbnN0YWxsLnNoYCByZWFkcyB0aGUgY2Fub25pY2FsIG1hbmlmZXN0LCB2ZXJpZmllcyB0aGUgZG93bmxvYWRlZCBhcnRpZmFjdAphZ2FpbnN0IHRoZSBtYW5pZmVzdCBTSEEtMjU2LCBhbmQgaW5zdGFsbHMgdGhlIHZlcmlmaWVkIGJpbmFyeSBhdApgJEhPTUUvLmxvY2FsL2Jpbi9id2Agd2l0aCBhIHNhbWUtZGlyZWN0b3J5IHJlbmFtZS4gQ29uY3VycmVudCBzYW1lLXVzZXIKSE9NRSBtdXRhdGlvbiBpcyBjYWxsZXItb3duZWQuIFRoZSBwdWJsaXNoZWQgYC5zaGEyNTZgIHNpZGVjYXIgaXMgZm9yIG1hbnVhbAp2ZXJpZmljYXRpb24gYW5kIGlzIG5vdCBpbnN0YWxsZXIgYXV0aG9yaXR5LiBXaW5kb3dzIFBvd2VyU2hlbGwgdmVyaWY=\",\"text\":\"oes not\\nclaim native Windows, PowerShell, or CMD execution. Linux arm64 is not currently\\nprovided, and no universal host-compatibility claim is made.\\n\\n`install.sh` reads the canonical manifest, verifies the downloaded artifact\\nagainst the manifest SHA-256, and installs the verified binary at\\n`$HOME/.local/bin/bw` with a same-directory rename. Concurrent same-user\\nHOME mutation is caller-owned. The published `.sha256` sidecar is for manual\\nverification and is not installer authority. Windows PowerShell verif\"}],\"more\":true,\"active\":[]}\n"}
{"id":8,"bytes":5235,"sha256":"2decbd1f08279e0b5875be972960f4b2533efebd3635fab9e301a9e1c001743b","content":"{\"id\":8,\"events\":[{\"sequence\":29,\"ns\":\"166506990999617\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"aWVzIHRoZQpzYW1lIG1hbmlmZXN0IGF1dGhvcml0eSBhbmQgZXhhY3QgWklQLCBhbmQgaW5zdGFsbHMgdG8KYCRIT01FXC5sb2NhbFxiaW5cYncuZXhlYCB3aXRob3V0IGVkaXRpbmcgUEFUSCBvciB0aGUgUG93ZXJTaGVsbCBwcm9maWxlLiBUaGUKdGhyZWUgY2Fub25pY2FsIGluc3RhbGwgY29tbWFuZHMgYW5kIHRoZSBjYW5vbmljYWwgYGJ3IHVwZGF0ZWAgY29tbWFuZCBhcmUga2VwdAp0b2dldGhlciBpbiBbUXVpY2sgc3RhcnRdKCNxdWljay1zdGFydCkuCgpUaGUgQ01EIGNvbW1hbmQgd3JpdGVzIGBpbnN0YWxsLmNtZGAgaW4gdGhlIGN1cnJlbnQgZGlyZWN0b3J5IGFuZCByZW1vdmVzIGl0CmFmdGVyIGEgc3VjY2Vzc2Z1bCBpbnN0YWxsYXRpb24uIEFuIGV4aXN0aW5nIGZpbGUgd2l0aCB0aGF0IG5hbWUgaXMgcmVwbGFjZWQuCgpUaGUgQ1JMRiBgaW5zdGFsbC5jbWRgIGNoZWNrcyBgY3VybC5leGVgIGFuZCBgcG93ZXJzaGVsbC5leGVgLCBkb3dubG9hZHMgZXhhY3RseQp0aGUgY2Fub25pY2E=\",\"text\":\"ies the\\nsame manifest authority and exact ZIP, and installs to\\n`$HOME\\\\.local\\\\bin\\\\bw.exe` without editing PATH or the PowerShell profile. The\\nthree canonical install commands and the canonical `bw update` command are kept\\ntogether in [Quick start](#quick-start).\\n\\nThe CMD command writes `install.cmd` in the current directory and removes it\\nafter a successful installation. An existing file with that name is replaced.\\n\\nThe CRLF `install.cmd` checks `curl.exe` and `powershell.exe`, downloads exactly\\nthe canonica\"},{\"sequence\":30,\"ns\":\"166506991011148\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"bCBgaW5zdGFsbC5wczFgIG92ZXIgSFRUUFMtb25seSBUTFMgMS4yLW9yLW5ld2VyIHRyYW5zcG9ydCBpbnRvIGEKY29sbGlzaW9uLWZhaWxpbmcgYCVURU1QJWAgdGFzayBkaXJlY3RvcnksIGRlbGVnYXRlcyBhbGwgaW5zdGFsbGF0aW9uIG1lYW5pbmcsCmNsZWFucyB0aGUgZGlyZWN0b3J5LCBhbmQgcHJlc2VydmVzIHRoZSBjaGlsZCBleGl0IGNvZGUuIEl0IG93bnMgbm8gbWFuaWZlc3QsClNIQS0yNTYsIFpJUCwgb3IgaW5zdGFsbGF0aW9uIGxvZ2ljLiBUaGUgTGludXgtaG9zdGVkIENNRCByZWdyZXNzaW9uIGlzIHN0YXRpYzsKbm8gbmF0aXZlIENNRCBleGVjdXRpb24gaXMgY2xhaW1lZC4KVGhlIGRpc3RyaWJ1dGlvbiBwcm92aWRlcyBubwpwdWJsaXNoZXItYXV0aGVudGljaXR5IHNpZ25hdHVyZSBvciB0cnVzdGVkIHNpZ25pbmcgaWRlbnRpdHksIGJhY2tncm91bmQgb3IKYXV0b21hdGljIHVwZGF0ZSwgdGVsZW1ldHJ5LCBgc3Vkb2AgZXhlY3V0aW9uLCBvciBhdXRvbWF0aWMgYFBBVEhgIG8=\",\"text\":\"l `install.ps1` over HTTPS-only TLS 1.2-or-newer transport into a\\ncollision-failing `%TEMP%` task directory, delegates all installation meaning,\\ncleans the directory, and preserves the child exit code. It owns no manifest,\\nSHA-256, ZIP, or installation logic. The Linux-hosted CMD regression is static;\\nno native CMD execution is claimed.\\nThe distribution provides no\\npublisher-authenticity signature or trusted signing identity, background or\\nautomatic update, telemetry, `sudo` execution, or automatic `PATH` o\"},{\"sequence\":31,\"ns\":\"166506991016590\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"cgpzaGVsbC1zdGFydHVwLWZpbGUgY2hhbmdlLgpHaXRIdWIgaXMgYSBwdWJsaWMgc291cmNlIGFuZCBkb2N1bWVudGF0aW9uIG1pcnJvciwgbm90IHRoZSBkaXN0cmlidXRpb24KYXV0aG9yaXR5LiBUaGUgY29tcGxldGUgYmV0YS4xLCBiZXRhLjIsIGJldGEuMywgc3RhYmxlIGAwLjEuMGAsIGAwLjIuMGAsIGFuZApgMC4yLjFgIGFuZCBgMC4yLjJgIHZlcnNpb24gZGlyZWN0b3JpZXMgcmVtYWluIHVuY2hhbmdlZCBhbmQgaW1tdXRhYmxlLiBUaGUKY29tcGxldGUgYDAuMi4zYCwgYDAuMi40YCwgYW5kIGAwLjIuNWAgdmVyc2lvbiBkaXJlY3RvcmllcyBhcmUgaW1tdXRhYmxlOyB0aGUKYDAuMi41YCByZWxlYXNlIGlzIGNsb3NlZC4gVGhlIGFjdGl2ZSBwdWJsaWMgdHJlZSBoYXMgZXhhY3RseSA2OCBmaWxlcywgYW5kCnRoZSBjdXJyZW50IGluc3RhbGxlcnMgYW5kIG1hbmlmZXN0IHNlbGVjdCBgMC4yLjVgLiBBbnkgbGF0ZXIgcGxhdGZvcm0gb3IKdmVyc2lvbiByZXF1aXJlcyBzZXBhcmF0ZSBPd24=\",\"text\":\"r\\nshell-startup-file change.\\nGitHub is a public source and documentation mirror, not the distribution\\nauthority. The complete beta.1, beta.2, beta.3, stable `0.1.0`, `0.2.0`, and\\n`0.2.1` and `0.2.2` version directories remain unchanged and immutable. The\\ncomplete `0.2.3`, `0.2.4`, and `0.2.5` version directories are immutable; the\\n`0.2.5` release is closed. The active public tree has exactly 68 files, and\\nthe current installers and manifest select `0.2.5`. Any later platform or\\nversion requires separate Own\"},{\"sequence\":32,\"ns\":\"166506991033791\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"ZXIgYXV0aG9yaXR5LiBMaW51eCBhcm02NCwgdGFncywgR2l0SHViIFJlbGVhc2VzLApjcmF0ZXMuaW8gcHVibGljYXRpb24sIGFuZCBiYWNrZ3JvdW5kIG9yIGF1dG9tYXRpYyB1cGRhdGUgcmVtYWluIG91dHNpZGUgdGhlCmNvbXBsZXRlZCBwdWJsaWNhdGlvbi4gVGhlIGVhcmxpZXIgYDAuMi40YCBwdWJsaWNhdGlvbiBhZGRlZCBpdHMgZWlnaHQKdmVyc2lvbmVkIGZpbGVzIGFuZCByZXBsYWNlZCBvbmx5IHRoZSB0d28gaW5zdGFsbGVycyBhbmQgbWFuaWZlc3QgcG9pbnRlcnMKd2l0aG91dCByZXBsYWNpbmcgYW55IG9mIHRoZSA0OCBwcmlvciB2ZXJzaW9uZWQgZmlsZXMgb3IgdGhlIENNRCBBZGFwdGVyLiBJdHMKc2Vjb25kIHB1Ymxpc2hlciBydW4gcmV1c2VkIGFsbCA2MCBmaWxlcyB3aXRob3V0IG1ldGFkYXRhIGNoYW5nZS4gVGhlIGAwLjIuNWAKcHVibGljYXRpb24gdGhlbiBhZGRlZCBpdHMgZWlnaHQgdmVyc2lvbmVkIGZpbGVzLCByZXBsYWNlZCB0aGUgdHdvIGluc3RhbGxlcnMKYW5kIG0=\",\"text\":\"er authority. Linux arm64, tags, GitHub Releases,\\ncrates.io publication, and background or automatic update remain outside the\\ncompleted publication. The earlier `0.2.4` publication added its eight\\nversioned files and replaced only the two installers and manifest pointers\\nwithout replacing any of the 48 prior versioned files or the CMD Adapter. Its\\nsecond publisher run reused all 60 files without metadata change. The `0.2.5`\\npublication then added its eight versioned files, replaced the two installers\\nand m\"}],\"more\":true,\"active\":[]}\n"}
{"id":9,"bytes":1179,"sha256":"0614715e594e6e23bc1c5d7c977f5d7ce4f3975940d033e427d167fe4149864c","content":"{\"id\":9,\"events\":[{\"sequence\":33,\"ns\":\"166506991040880\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"YW5pZmVzdCBwb2ludGVycywgYW5kIHJldXNlZCBhbGwgNjggZmlsZXMgb24gaXRzIHNlY29uZCBydW4uCgojIyBBcmNoaXRlY3R1cmUKCi0gW0N1cnJlbnQgc3RhdGVdKGRvY3MvY3VycmVudC9ub3cubWQpCi0gW0JhY2t3cml0ZXIgcHJvdG9jb2xdKGRvY3MvYXJjaGl0ZWN0dXJlL2JhY2t3cml0ZXItdGV4dC1jb29yZGluYXRpb24tcHJvdG9jb2wubWQpCi0gW0FuZGRyZXNzIGFuZCBleGFjdCBMaW5lIG1vZGVsXShkb2NzL2FyY2hpdGVjdHVyZS9yZWJ1aWxkYWJsZS1zdHJ1Y3R1cmFsLWFkZHJlc3NpbmcubWQpCi0gW0NMSSBWMSBhdXRob3JpdHldKGRvY3MvYXJjaGl0ZWN0dXJlL2JhY2t3cml0ZXItY2xpLXYxLm1kKQotIFtWZXJpZmljYXRpb25dKGRvY3MvZGV2ZWxvcG1lbnQvdmVyaWZpY2F0aW9uLm1kKQoKIyMgTGljZW5zZQoKW01JVCBMaWNlbnNlXShMSUNFTlNFKQo=\",\"text\":\"anifest pointers, and reused all 68 files on its second run.\\n\\n## Architecture\\n\\n- [Current state](docs/current/now.md)\\n- [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)\\n- [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)\\n- [CLI V1 authority](docs/architecture/backwriter-cli-v1.md)\\n- [Verification](docs/development/verification.md)\\n\\n## License\\n\\n[MIT License](LICENSE)\\n\"}],\"more\":false,\"active\":[]}\n"}
{"id":10,"bytes":3081,"sha256":"e3a899ca5a6c07073c0a77767d5d0a19a64c54db12da5ab7de997f563304af50","content":"{\"id\":10,\"events\":[{\"sequence\":34,\"ns\":\"166625404831215\",\"kind\":\"start\",\"processId\":1,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/bw-nminus\",\"--help\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh/genie-old/workspace\"},{\"sequence\":35,\"ns\":\"166625405855762\",\"kind\":\"stdin-eof\",\"processId\":1},{\"sequence\":36,\"ns\":\"166625405939090\",\"kind\":\"spawn\",\"processId\":1,\"pid\":2726673},{\"sequence\":37,\"ns\":\"166625406094575\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"VVNBR0UKICBidyBbR0xPQkFMIE9QVElPTlNdIDxjb21tYW5kPiBbY29tbWFuZCBvcHRpb25zIGFuZCBvcGVyYW5kc10KICBidyBoZWxwIFs8Y29tbWFuZD5dCgpHTE9CQUwgT1BUSU9OUwogIC0td29ya3NwYWNlIEFCU09MVVRFX1BBVEggIFNlbGVjdCBhbiBhYnNvbHV0ZSB3b3Jrc3BhY2UgYmVmb3JlIHRoZSBjb21tYW5kLgogIC0tYWRtaXQgTE9HSUNBTF9QQVRIICAgICAgIEFkbWl0IGEgbG9naWNhbCByb290IGJlZm9yZSB0aGUgY29tbWFuZDsgcmVwZWF0YWJsZS4KICAtLWpzb24gICAgICAgICAgICAgICAgICAgICBTZWxlY3QgSlNPTiBvdXRwdXQgd2hlcmUgdGhlIGNvbW1hbmQgc3VwcG9ydHMgaXQuCiAgLS1yYXcgICAgICAgICAgICAgICAgICAgICAgU2VsZWN0IHJhdyBWaWV3IG91dHB1dCBvbmx5LgoKQ0FQQUJJTElUSUVTCiAgc2VhcmNoICAgRGlzY292ZXIgY3VycmVudCBGaWxlLCBQYXJhZ3JhcGgsIG9yIExpbmUgQW5kZHJlc3Nlcy4KICB2aWV3ICAgICBSZWFkIG9uZSBvciBtb3JlIGM=\",\"text\":\"USAGE\\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\\n  bw help [<command>]\\n\\nGLOBAL OPTIONS\\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\\n  --json                     Select JSON output where the command supports it.\\n  --raw                      Select raw View output only.\\n\\nCAPABILITIES\\n  search   Discover current File, Paragraph, or Line Anddresses.\\n  view     Read one or more c\"},{\"sequence\":38,\"ns\":\"166625406116541\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"dXJyZW50IEFuZGRyZXNzZXMuCiAgZWRpdCAgICAgUmVwbGFjZSBvbmUgY3VycmVudCBBbmRkcmVzcy4KICBjaGVjayAgICBDaGVjayBvbmUgb3IgbW9yZSBjdXJyZW50IEFuZGRyZXNzZXMuCiAgc2hlbGwgICAgUnVuIGFkdmFuY2VkIHJhdyBTZXNzaW9uIGNvbW1hbmRzLgogIHZlcnNpb24gIFByaW50IHRoZSBCYWNrd3JpdGVyIHZlcnNpb24uCiAgdXBkYXRlICAgUnVuIHRoZSBpbnN0YWxsZWQtcGxhdGZvcm0gdXBkYXRlci4KClBpY2ssIEFuY2hvciwgQXBwbHksIGFuZCBEYXRhIGhhdmUgbm8gb25lLXNob3QgY29tbWFuZDsgdXNlIGJ3IHNoZWxsLgoKQURESVRJT05BTCBIRUxQCiAgYncgaGVscCA8Y29tbWFuZD4KCkdsb2JhbCBvcHRpb25zIHByZWNlZGUgdGhlIGNvbW1hbmQuIENhbm9uaWNhbCBvdXRwdXQgb3B0aW9ucyBhcmUgZG9jdW1lbnRlZCBvbmx5IGluIHRoYXQgcG9zaXRpb24uCg==\",\"text\":\"urrent Anddresses.\\n  edit     Replace one current Anddress.\\n  check    Check one or more current Anddresses.\\n  shell    Run advanced raw Session commands.\\n  version  Print the Backwriter version.\\n  update   Run the installed-platform updater.\\n\\nPick, Anchor, Apply, and Data have no one-shot command; use bw shell.\\n\\nADDITIONAL HELP\\n  bw help <command>\\n\\nGlobal options precede the command. Canonical output options are documented only in that position.\\n\"},{\"sequence\":39,\"ns\":\"166625406198638\",\"kind\":\"stdout-eof\",\"processId\":1},{\"sequence\":40,\"ns\":\"166625406251144\",\"kind\":\"stderr-eof\",\"processId\":1},{\"sequence\":41,\"ns\":\"166625406300843\",\"kind\":\"close\",\"processId\":1,\"code\":0,\"signal\":null}],\"more\":false,\"active\":[]}\n"}
{"id":11,"bytes":314,"sha256":"220eaaad99ff09ad98dc183447482967b63e565eb7dc7074005781bcda067765","content":"{\"id\":11,\"events\":[{\"sequence\":42,\"ns\":\"166639824762347\",\"kind\":\"start\",\"processId\":2,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/bw-nminus\",\"shell\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh/genie-old/workspace\"},{\"sequence\":43,\"ns\":\"166639825888813\",\"kind\":\"spawn\",\"processId\":2,\"pid\":2727066}],\"more\":false,\"active\":[2]}\n"}
{"id":12,"bytes":742,"sha256":"e98c5cbe47287d1bbd92cc7d5c02c6bb8853946d5226a160a506cfdccd1d7f5a","content":"{\"id\":12,\"events\":[{\"sequence\":44,\"ns\":\"166653866854251\",\"kind\":\"stdin\",\"processId\":2,\"base64\":\"c2VhcmNoIGxpbmUgInJldHJ5X2J1ZGdldCA9IDMiCg==\",\"text\":\"search line \\\"retry_budget = 3\\\"\\n\"},{\"sequence\":45,\"ns\":\"166653867008747\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDAJTGluZQl1bml0LTAxLnR4dDoyCkAxCUxpbmUJdW5pdC0wMS50eHQ6NQpAMglMaW5lCXVuaXQtMDIudHh0OjIKQDMJTGluZQl1bml0LTAyLnR4dDo1CkA0CUxpbmUJdW5pdC0wMy50eHQ6MgpANQlMaW5lCXVuaXQtMDMudHh0OjUKQDYJTGluZQl1bml0LTA0LnR4dDoyCkA3CUxpbmUJdW5pdC0wNC50eHQ6NQo=\",\"text\":\"@0\\tLine\\tunit-01.txt:2\\n@1\\tLine\\tunit-01.txt:5\\n@2\\tLine\\tunit-02.txt:2\\n@3\\tLine\\tunit-02.txt:5\\n@4\\tLine\\tunit-03.txt:2\\n@5\\tLine\\tunit-03.txt:5\\n@6\\tLine\\tunit-04.txt:2\\n@7\\tLine\\tunit-04.txt:5\\n\"}],\"more\":false,\"active\":[2]}\n"}
{"id":13,"bytes":1341,"sha256":"f3775c43521fcf0ae82dd094f047e71a9f6e57585d35cb9147b90a002d18ff4e","content":"{\"id\":13,\"events\":[{\"sequence\":46,\"ns\":\"166668646604095\",\"kind\":\"stdin\",\"processId\":2,\"base64\":\"dmlldyBAMCAtLWFzIHBhcmFncmFwaAp2aWV3IEAxIC0tYXMgcGFyYWdyYXBoCnZpZXcgQDIgLS1hcyBwYXJhZ3JhcGgKdmlldyBAMyAtLWFzIHBhcmFncmFwaAp2aWV3IEA0IC0tYXMgcGFyYWdyYXBoCnZpZXcgQDUgLS1hcyBwYXJhZ3JhcGgKdmlldyBANiAtLWFzIHBhcmFncmFwaAp2aWV3IEA3IC0tYXMgcGFyYWdyYXBoCg==\",\"text\":\"view @0 --as paragraph\\nview @1 --as paragraph\\nview @2 --as paragraph\\nview @3 --as paragraph\\nview @4 --as paragraph\\nview @5 --as paragraph\\nview @6 --as paragraph\\nview @7 --as paragraph\\n\"},{\"sequence\":47,\"ns\":\"166668646675045\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDgJUGFyYWdyYXBoCXVuaXQtMDEudHh0OjEtMgo=\",\"text\":\"@8\\tParagraph\\tunit-01.txt:1-2\\n\"},{\"sequence\":48,\"ns\":\"166668646700062\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDkJUGFyYWdyYXBoCXVuaXQtMDEudHh0OjQtNQpAMTAJUGFyYWdyYXBoCXVuaXQtMDIudHh0OjEtMgpAMTEJUGFyYWdyYXBoCXVuaXQtMDIudHh0OjQtNQpAMTIJUGFyYWdyYXBoCXVuaXQtMDMudHh0OjEtMgpAMTMJUGFyYWdyYXBoCXVuaXQtMDMudHh0OjQtNQpAMTQJUGFyYWdyYXBoCXVuaXQtMDQudHh0OjEtMgpAMTUJUGFyYWdyYXBoCXVuaXQtMDQudHh0OjQtNQo=\",\"text\":\"@9\\tParagraph\\tunit-01.txt:4-5\\n@10\\tParagraph\\tunit-02.txt:1-2\\n@11\\tParagraph\\tunit-02.txt:4-5\\n@12\\tParagraph\\tunit-03.txt:1-2\\n@13\\tParagraph\\tunit-03.txt:4-5\\n@14\\tParagraph\\tunit-04.txt:1-2\\n@15\\tParagraph\\tunit-04.txt:4-5\\n\"}],\"more\":false,\"active\":[2]}\n"}
{"id":14,"bytes":2709,"sha256":"1656d471eb51ff15d6d971d7b22d34f205be725e18f589ca2dd17d68aeeff18c","content":"{\"id\":14,\"events\":[{\"sequence\":49,\"ns\":\"166682054073140\",\"kind\":\"stdin\",\"processId\":2,\"base64\":\"aGVscAo=\",\"text\":\"help\\n\"},{\"sequence\":50,\"ns\":\"166682054150794\",\"kind\":\"stderr\",\"processId\":2,\"base64\":\"ZXJyb3I6IHVuc3VwcG9ydGVkIFNlc3Npb24gY29tbWFuZDogaGVscAoKVVNBR0UKICBidyBbR0xPQkFMIE9QVElPTlNdIDxjb21tYW5kPiBbY29tbWFuZCBvcHRpb25zIGFuZCBvcGVyYW5kc10KICBidyBoZWxwIFs8Y29tbWFuZD5dCgpHTE9CQUwgT1BUSU9OUwogIC0td29ya3NwYWNlIEFCU09MVVRFX1BBVEggIFNlbGVjdCBhbiBhYnNvbHV0ZSB3b3Jrc3BhY2UgYmVmb3JlIHRoZSBjb21tYW5kLgogIC0tYWRtaXQgTE9HSUNBTF9QQVRIICAgICAgIEFkbWl0IGEgbG9naWNhbCByb290IGJlZm9yZSB0aGUgY29tbWFuZDsgcmVwZWF0YWJsZS4KICAtLWpzb24gICAgICAgICAgICAgICAgICAgICBTZWxlY3QgSlNPTiBvdXRwdXQgd2hlcmUgdGhlIGNvbW1hbmQgc3VwcG9ydHMgaXQuCiAgLS1yYXcgICAgICAgICAgICAgICAgICAgICAgU2VsZWN0IHJhdyBWaWV3IG91dHB1dCBvbmx5LgoKQ0FQQUJJTElUSUVTCiAgc2VhcmNoICAgRGlzY292ZXIgY3VycmVudCBGaWxlLCBQYXJhZ3JhcGgsIG9yIExpbmU=\",\"text\":\"error: unsupported Session command: help\\n\\nUSAGE\\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\\n  bw help [<command>]\\n\\nGLOBAL OPTIONS\\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\\n  --json                     Select JSON output where the command supports it.\\n  --raw                      Select raw View output only.\\n\\nCAPABILITIES\\n  search   Discover current File, Paragraph, or Line\"},{\"sequence\":51,\"ns\":\"166682054181528\",\"kind\":\"stderr\",\"processId\":2,\"base64\":\"IEFuZGRyZXNzZXMuCiAgdmlldyAgICAgUmVhZCBvbmUgb3IgbW9yZSBjdXJyZW50IEFuZGRyZXNzZXMuCiAgZWRpdCAgICAgUmVwbGFjZSBvbmUgY3VycmVudCBBbmRkcmVzcy4KICBjaGVjayAgICBDaGVjayBvbmUgb3IgbW9yZSBjdXJyZW50IEFuZGRyZXNzZXMuCiAgc2hlbGwgICAgUnVuIGFkdmFuY2VkIHJhdyBTZXNzaW9uIGNvbW1hbmRzLgogIHZlcnNpb24gIFByaW50IHRoZSBCYWNrd3JpdGVyIHZlcnNpb24uCiAgdXBkYXRlICAgUnVuIHRoZSBpbnN0YWxsZWQtcGxhdGZvcm0gdXBkYXRlci4KClBpY2ssIEFuY2hvciwgQXBwbHksIGFuZCBEYXRhIGhhdmUgbm8gb25lLXNob3QgY29tbWFuZDsgdXNlIGJ3IHNoZWxsLgoKQURESVRJT05BTCBIRUxQCiAgYncgaGVscCA8Y29tbWFuZD4KCkdsb2JhbCBvcHRpb25zIHByZWNlZGUgdGhlIGNvbW1hbmQuIENhbm9uaWNhbCBvdXRwdXQgb3B0aW9ucyBhcmUgZG9jdW1lbnRlZCBvbmx5IGluIHRoYXQgcG9zaXRpb24uCg==\",\"text\":\" Anddresses.\\n  view     Read one or more current Anddresses.\\n  edit     Replace one current Anddress.\\n  check    Check one or more current Anddresses.\\n  shell    Run advanced raw Session commands.\\n  version  Print the Backwriter version.\\n  update   Run the installed-platform updater.\\n\\nPick, Anchor, Apply, and Data have no one-shot command; use bw shell.\\n\\nADDITIONAL HELP\\n  bw help <command>\\n\\nGlobal options precede the command. Canonical output options are documented only in that position.\\n\"}],\"more\":false,\"active\":[2]}\n"}
{"id":15,"bytes":3830,"sha256":"343216a04bd7f7033f99b88c961130bc2c7c3624c7fb968311a7a4a4a5771fea","content":"{\"id\":15,\"events\":[{\"sequence\":52,\"ns\":\"166696703302619\",\"kind\":\"start\",\"processId\":3,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/bw-nminus\",\"help\",\"shell\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh/genie-old/workspace\"},{\"sequence\":53,\"ns\":\"166696704446409\",\"kind\":\"stdin-eof\",\"processId\":3},{\"sequence\":54,\"ns\":\"166696704505508\",\"kind\":\"spawn\",\"processId\":3,\"pid\":2728377},{\"sequence\":55,\"ns\":\"166696704880695\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"TkFNRQogIGJ3IHNoZWxsIC0gcnVuIG9uZSBsb2NhbCByZWZlcmVuY2Ugc2Vzc2lvbiBhbmQgYWR2YW5jZWQgcmF3IFNlc3Npb24gY29tbWFuZHMKClVTQUdFCiAgYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4gc2hlbGwKCkRFU0NSSVBUSU9OCiAgUmVhZHMgY29tbWFuZHMgZnJvbSBzdGFuZGFyZCBpbnB1dCB1bnRpbCBleGl0LiBEaXJlY3Qgc2VhcmNoLCB2aWV3LCByZXBsYWNlLCBhbmQgY2hlY2sgdXNlIHNlc3Npb24tbG9jYWwgbnVtZXJpYyBBbmRkcmVzcyByZWZlcmVuY2VzLiBSYXcgYmluZGluZ3MgYW5kIHJhdyBjYXBhYmlsaXR5IGNvbXBvc2l0aW9uIHJlbWFpbiB0aGUgYWR2YW5jZWQgc3VyZmFjZS4KCkFSR1VNRU5UUwogIE5vbmUuCgpPUFRJT05TCiAgLS13b3Jrc3BhY2UgYW5kIC0tYWRtaXQgbXVzdCBwcmVjZWRlIHNoZWxsLgogIC0tanNvbiBhbmQgLS1yYXcgYXJlIHVuYXZhaWxhYmxlLgoKV0hBVCBIQVBQRU5TCiAgQSBzdWNjZXM=\",\"text\":\"NAME\\n  bw shell - run one local reference session and advanced raw Session commands\\n\\nUSAGE\\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\\n\\nDESCRIPTION\\n  Reads commands from standard input until exit. Direct search, view, replace, and check use session-local numeric Anddress references. Raw bindings and raw capability composition remain the advanced surface.\\n\\nARGUMENTS\\n  None.\\n\\nOPTIONS\\n  --workspace and --admit must precede shell.\\n  --json and --raw are unavailable.\\n\\nWHAT HAPPENS\\n  A succes\"},{\"sequence\":56,\"ns\":\"166696704934193\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"c2Z1bCBkaXJlY3Qgc2VhcmNoLCB2aWV3LCBvciBjdXJyZW50IGNoZWNrIGVtaXRzIGFwcGVuZC1vbmx5IEBOIHJlZmVyZW5jZXMuIERpcmVjdCByZXBsYWNlIHVzZXMgb25lIHJlZmVyZW5jZSBhbmQgZW1pdHMgYSBmcmVzaCByZWZlcmVuY2Ugd2hlbiBvbmUgZXhpc3RzLiBSZWZlcmVuY2VzIGVuZCB3aXRoIHRoaXMgc2hlbGwgcHJvY2Vzcy4gUmF3IGxldCwgUGljaywgVmlldywgQ2hlY2ssIEFuY2hvciwgRWRpdCwgQXBwbHksIGFuZCBEYXRhIHJldGFpbiB0aGVpciBleGlzdGluZyBncmFtbWFyLgoKT1VUUFVUCiAgRGlyZWN0IHJlZmVyZW5jZXMgd3JpdGUgQE4sIHRhcmdldCBraW5kLCBhbmQgbG9jYXRpb24uIERpcmVjdCBDaGVjayB3cml0ZXMgb25lIHN0YXRlIHBlciBpbnB1dC4gUmF3IGNvbW1hbmRzIHdyaXRlIHRoZWlyIGV4aXN0aW5nIGh1bWFuIHJlc3VsdC4KCkVYQU1QTEVTCiAgYncgc2hlbGwKICBzZWFyY2ggbGluZSBuZWVkbGUKICB2aWV3IEAwCiAgcmVwbGFjZSBAMSByZXBsYWNlbWU=\",\"text\":\"sful direct search, view, or current check emits append-only @N references. Direct replace uses one reference and emits a fresh reference when one exists. References end with this shell process. Raw let, Pick, View, Check, Anchor, Edit, Apply, and Data retain their existing grammar.\\n\\nOUTPUT\\n  Direct references write @N, target kind, and location. Direct Check writes one state per input. Raw commands write their existing human result.\\n\\nEXAMPLES\\n  bw shell\\n  search line needle\\n  view @0\\n  replace @1 replaceme\"},{\"sequence\":57,\"ns\":\"166696704960844\",\"kind\":\"stdout\",\"processId\":3,\"base64\":\"bnQKICBjaGVjayBAMiBAMwogIGxldCBoaXRzID0gc2VhcmNoIGxpbmUgbmVlZGxlCiAgdmlldyBhbmRkcmVzcyBAaGl0c1swXQogIGV4aXQKCkZBSUxVUkVTCiAgSW52YWxpZCBzaGVsbCBncmFtbWFyIGlzIGEgdXNhZ2UgZmFpbHVyZS4gUnVudGltZSBhbmQgc291cmNlIGZhaWx1cmVzIGV4aXQgMS4KClNFRSBBTFNPCiAgYncgaGVscCBzZWFyY2gKICBidyBoZWxwIGVkaXQK\",\"text\":\"nt\\n  check @2 @3\\n  let hits = search line needle\\n  view anddress @hits[0]\\n  exit\\n\\nFAILURES\\n  Invalid shell grammar is a usage failure. Runtime and source failures exit 1.\\n\\nSEE ALSO\\n  bw help search\\n  bw help edit\\n\"},{\"sequence\":58,\"ns\":\"166696705019966\",\"kind\":\"stdout-eof\",\"processId\":3},{\"sequence\":59,\"ns\":\"166696705094645\",\"kind\":\"stderr-eof\",\"processId\":3},{\"sequence\":60,\"ns\":\"166696705187148\",\"kind\":\"close\",\"processId\":3,\"code\":0,\"signal\":null}],\"more\":false,\"active\":[2]}\n"}
{"id":16,"bytes":1555,"sha256":"be941ccbfe397b29e49a1c9865b54427b59d809a39e6408cdc71d7fc309cff0f","content":"{\"id\":16,\"events\":[{\"sequence\":61,\"ns\":\"166710970169401\",\"kind\":\"stdin\",\"processId\":2,\"base64\":\"bGV0IHAwID0gQDgKbGV0IHAxID0gQDkKbGV0IHAyID0gQDEwCmxldCBwMyA9IEAxMQpsZXQgcDQgPSBAMTIKbGV0IHA1ID0gQDEzCmxldCBwNiA9IEAxNApsZXQgcDcgPSBAMTUKdmlldyBhbmRkcmVzcyBAcDAKdmlldyBhbmRkcmVzcyBAcDEKdmlldyBhbmRkcmVzcyBAcDIKdmlldyBhbmRkcmVzcyBAcDMKdmlldyBhbmRkcmVzcyBAcDQKdmlldyBhbmRkcmVzcyBAcDUKdmlldyBhbmRkcmVzcyBAcDYKdmlldyBhbmRkcmVzcyBAcDcK\",\"text\":\"let p0 = @8\\nlet p1 = @9\\nlet p2 = @10\\nlet p3 = @11\\nlet p4 = @12\\nlet p5 = @13\\nlet p6 = @14\\nlet p7 = @15\\nview anddress @p0\\nview anddress @p1\\nview anddress @p2\\nview anddress @p3\\nview anddress @p4\\nview anddress @p5\\nview anddress @p6\\nview anddress @p7\\n\"},{\"sequence\":62,\"ns\":\"166710970302130\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"cm9sZSA9IHByaW1hcnkKcmV0cnlfYnVkZ2V0ID0gMwpyb2xlID0gc2Vjb25kYXJ5CnJldHJ5X2J1ZGdldCA9IDMKcm9sZSA9IHNlY29uZGFyeQ1yZXRyeV9idWRnZXQgPSAzDQ==\",\"text\":\"role = primary\\nretry_budget = 3\\nrole = secondary\\nretry_budget = 3\\nrole = secondary\\rretry_budget = 3\\r\"},{\"sequence\":63,\"ns\":\"166710970358430\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"cm9sZSA9IHByaW1hcnkNcmV0cnlfYnVkZ2V0ID0gMw1yb2xlID0gcHJpbWFyeQ0KcmV0cnlfYnVkZ2V0ID0gMw0Kcm9sZSA9IHNlY29uZGFyeQ0KcmV0cnlfYnVkZ2V0ID0gMw0Kcm9sZSA9IHNlY29uZGFyeQpyZXRyeV9idWRnZXQgPSAzCnJvbGUgPSBwcmltYXJ5CnJldHJ5X2J1ZGdldCA9IDM=\",\"text\":\"role = primary\\rretry_budget = 3\\rrole = primary\\r\\nretry_budget = 3\\r\\nrole = secondary\\r\\nretry_budget = 3\\r\\nrole = secondary\\nretry_budget = 3\\nrole = primary\\nretry_budget = 3\"}],\"more\":false,\"active\":[2]}\n"}
{"id":17,"bytes":1109,"sha256":"8b56f983bdc7afe871c904da894ae7e7ca96a08dbff8a4d9df13329a7e554806","content":"{\"id\":17,\"events\":[{\"sequence\":64,\"ns\":\"166727603610414\",\"kind\":\"stdin\",\"processId\":2,\"base64\":\"cmVwbGFjZSBAMCAicmV0cnlfYnVkZ2V0ID0gNSIKcmVwbGFjZSBAMyAicmV0cnlfYnVkZ2V0ID0gNSIKcmVwbGFjZSBANCAicmV0cnlfYnVkZ2V0ID0gNSIKcmVwbGFjZSBANyAicmV0cnlfYnVkZ2V0ID0gNSIK\",\"text\":\"replace @0 \\\"retry_budget = 5\\\"\\nreplace @3 \\\"retry_budget = 5\\\"\\nreplace @4 \\\"retry_budget = 5\\\"\\nreplace @7 \\\"retry_budget = 5\\\"\\n\"},{\"sequence\":65,\"ns\":\"166727604873978\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDE2CUNoYW5nZWQJTGluZQl1bml0LTAxLnR4dDoyCg==\",\"text\":\"@16\\tChanged\\tLine\\tunit-01.txt:2\\n\"},{\"sequence\":66,\"ns\":\"166727604899122\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDE3CUNoYW5nZWQJTGluZQl1bml0LTAyLnR4dDo1Cg==\",\"text\":\"@17\\tChanged\\tLine\\tunit-02.txt:5\\n\"},{\"sequence\":67,\"ns\":\"166727604911290\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDE4CUNoYW5nZWQJTGluZQl1bml0LTAzLnR4dDoyCg==\",\"text\":\"@18\\tChanged\\tLine\\tunit-03.txt:2\\n\"},{\"sequence\":68,\"ns\":\"166727604921936\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDE5CUNoYW5nZWQJTGluZQl1bml0LTA0LnR4dDo1Cg==\",\"text\":\"@19\\tChanged\\tLine\\tunit-04.txt:5\\n\"}],\"more\":false,\"active\":[2]}\n"}
{"id":18,"bytes":1202,"sha256":"3201139d6351fc7aad4c0796d4f1e13c0c08868fecf1f3f44d2755e2ba5848bb","content":"{\"id\":18,\"events\":[{\"sequence\":69,\"ns\":\"166741330553670\",\"kind\":\"stdin\",\"processId\":2,\"base64\":\"Y2hlY2sgQDE2IEAxNyBAMTggQDE5IEAxIEAyIEA1IEA2CnZpZXcgQDE2IC0tYXMgZmlsZQp2aWV3IEAxNyAtLWFzIGZpbGUKdmlldyBAMTggLS1hcyBmaWxlCnZpZXcgQDE5IC0tYXMgZmlsZQo=\",\"text\":\"check @16 @17 @18 @19 @1 @2 @5 @6\\nview @16 --as file\\nview @17 --as file\\nview @18 --as file\\nview @19 --as file\\n\"},{\"sequence\":70,\"ns\":\"166741330639121\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDIwCUN1cnJlbnQJTGluZQl1bml0LTAxLnR4dDoyCkAyMQlDdXJyZW50CUxpbmUJdW5pdC0wMi50eHQ6NQpAMjIJQ3VycmVudAlMaW5lCXVuaXQtMDMudHh0OjIKQDIzCUN1cnJlbnQJTGluZQl1bml0LTA0LnR4dDo1Ck5vdEN1cnJlbnQKTm90Q3VycmVudApOb3RDdXJyZW50Ck5vdEN1cnJlbnQK\",\"text\":\"@20\\tCurrent\\tLine\\tunit-01.txt:2\\n@21\\tCurrent\\tLine\\tunit-02.txt:5\\n@22\\tCurrent\\tLine\\tunit-03.txt:2\\n@23\\tCurrent\\tLine\\tunit-04.txt:5\\nNotCurrent\\nNotCurrent\\nNotCurrent\\nNotCurrent\\n\"},{\"sequence\":71,\"ns\":\"166741330660133\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDI0CUZpbGUJdW5pdC0wMS50eHQKQDI1CUZpbGUJdW5pdC0wMi50eHQKQDI2CUZpbGUJdW5pdC0wMy50eHQKQDI3CUZpbGUJdW5pdC0wNC50eHQK\",\"text\":\"@24\\tFile\\tunit-01.txt\\n@25\\tFile\\tunit-02.txt\\n@26\\tFile\\tunit-03.txt\\n@27\\tFile\\tunit-04.txt\\n\"}],\"more\":false,\"active\":[2]}\n"}
{"id":19,"bytes":1676,"sha256":"d10faef18ff07caf1dd336840c9fc4219ba70dd02038135dcf19cd522571ed09","content":"{\"id\":19,\"events\":[{\"sequence\":72,\"ns\":\"166755169263883\",\"kind\":\"stdin\",\"processId\":2,\"base64\":\"bGV0IGYwID0gQDI0CmxldCBmMSA9IEAyNQpsZXQgZjIgPSBAMjYKbGV0IGYzID0gQDI3CnZpZXcgYW5kZHJlc3MgQGYwCnZpZXcgYW5kZHJlc3MgQGYxCnZpZXcgYW5kZHJlc3MgQGYyCnZpZXcgYW5kZHJlc3MgQGYzCnNlYXJjaCBsaW5lICJyZXRyeV9idWRnZXQgPSAzIgo=\",\"text\":\"let f0 = @24\\nlet f1 = @25\\nlet f2 = @26\\nlet f3 = @27\\nview anddress @f0\\nview anddress @f1\\nview anddress @f2\\nview anddress @f3\\nsearch line \\\"retry_budget = 3\\\"\\n\"},{\"sequence\":73,\"ns\":\"166755169340626\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"cm9sZSA9IHByaW1hcnkKcmV0cnlfYnVkZ2V0ID0gNQoKcm9sZSA9IHNlY29uZGFyeQpyZXRyeV9idWRnZXQgPSAzCg==\",\"text\":\"role = primary\\nretry_budget = 5\\n\\nrole = secondary\\nretry_budget = 3\\n\"},{\"sequence\":74,\"ns\":\"166755169361487\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"cm9sZSA9IHNlY29uZGFyeQ1yZXRyeV9idWRnZXQgPSAzDQ1yb2xlID0gcHJpbWFyeQ1yZXRyeV9idWRnZXQgPSA1DXJvbGUgPSBwcmltYXJ5DQpyZXRyeV9idWRnZXQgPSA1DQoNCnJvbGUgPSBzZWNvbmRhcnkNCnJldHJ5X2J1ZGdldCA9IDMNCnJvbGUgPSBzZWNvbmRhcnkKcmV0cnlfYnVkZ2V0ID0gMwoKcm9sZSA9IHByaW1hcnkKcmV0cnlfYnVkZ2V0ID0gNQ==\",\"text\":\"role = secondary\\rretry_budget = 3\\r\\rrole = primary\\rretry_budget = 5\\rrole = primary\\r\\nretry_budget = 5\\r\\n\\r\\nrole = secondary\\r\\nretry_budget = 3\\r\\nrole = secondary\\nretry_budget = 3\\n\\nrole = primary\\nretry_budget = 5\"},{\"sequence\":75,\"ns\":\"166755169388194\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDI4CUxpbmUJdW5pdC0wMS50eHQ6NQpAMjkJTGluZQl1bml0LTAyLnR4dDoyCkAzMAlMaW5lCXVuaXQtMDMudHh0OjUKQDMxCUxpbmUJdW5pdC0wNC50eHQ6Mgo=\",\"text\":\"@28\\tLine\\tunit-01.txt:5\\n@29\\tLine\\tunit-02.txt:2\\n@30\\tLine\\tunit-03.txt:5\\n@31\\tLine\\tunit-04.txt:2\\n\"}],\"more\":false,\"active\":[2]}\n"}
{"id":20,"bytes":909,"sha256":"921ce0ba05982c5a70edf1301018ce3d13bada7408e23ff01f2a958670c12433","content":"{\"id\":20,\"events\":[{\"sequence\":76,\"ns\":\"166768763754350\",\"kind\":\"stdin\",\"processId\":2,\"base64\":\"Y2hlY2sgQDI4IEAyOSBAMzAgQDMxCmV4aXQK\",\"text\":\"check @28 @29 @30 @31\\nexit\\n\"},{\"sequence\":77,\"ns\":\"166768763777954\",\"kind\":\"stdin-eof\",\"processId\":2},{\"sequence\":78,\"ns\":\"166768763863633\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDMyCUN1cnJlbnQJTGluZQl1bml0LTAxLnR4dDo1CkAzMwlDdXJyZW50CUxpbmUJdW5pdC0wMi50eHQ6MgpAMzQJQ3VycmVudAlMaW5lCXVuaXQtMDMudHh0OjUKQDM1CUN1cnJlbnQJTGluZQl1bml0LTA0LnR4dDoyCg==\",\"text\":\"@32\\tCurrent\\tLine\\tunit-01.txt:5\\n@33\\tCurrent\\tLine\\tunit-02.txt:2\\n@34\\tCurrent\\tLine\\tunit-03.txt:5\\n@35\\tCurrent\\tLine\\tunit-04.txt:2\\n\"},{\"sequence\":79,\"ns\":\"166768763968456\",\"kind\":\"stderr-eof\",\"processId\":2},{\"sequence\":80,\"ns\":\"166768764020973\",\"kind\":\"stdout-eof\",\"processId\":2},{\"sequence\":81,\"ns\":\"166768764076968\",\"kind\":\"close\",\"processId\":2,\"code\":2,\"signal\":null}],\"more\":false,\"active\":[]}\n"}
{"id":21,"bytes":197,"sha256":"a4ce299ef53bd43113643fdf70c7a308306360a08ae893163f687853843260d6","content":"{\"id\":21,\"events\":[{\"sequence\":82,\"ns\":\"166783231019674\",\"kind\":\"finished\",\"firstNs\":\"166506990711179\",\"lastProductCloseNs\":\"166768764076968\",\"elapsedNs\":\"261773365789\"}],\"more\":false,\"active\":[]}\n"}
```


#### Archived file: records/genie-new/requests.jsonl

Bytes: 1719; SHA-256: `cfd276e5390622281ca15e2165c11b9c5dd3ff9514127e66395e35ddd74aa1f6`.

```jsonl
{"ns":"166630187804206","request":{"arm":"genie-new","op":"guide"}}
{"ns":"166644550244190","request":{"arm":"genie-new","op":"poll"}}
{"ns":"166656666996393","request":{"arm":"genie-new","op":"poll"}}
{"ns":"166668922642445","request":{"arm":"genie-new","op":"poll"}}
{"ns":"166681272508608","request":{"arm":"genie-new","op":"poll"}}
{"ns":"166695857983392","request":{"arm":"genie-new","op":"poll"}}
{"ns":"166707881713972","request":{"arm":"genie-new","op":"poll"}}
{"ns":"166720083061406","request":{"arm":"genie-new","op":"poll"}}
{"ns":"166732029196447","request":{"arm":"genie-new","op":"poll"}}
{"ns":"166745584675826","request":{"arm":"genie-new","op":"start","args":["help","shell"],"eof":true}}
{"ns":"166757448585981","request":{"arm":"genie-new","op":"poll"}}
{"ns":"166771478327840","request":{"arm":"genie-new","op":"start","args":["shell"]}}
{"ns":"166785338314931","request":{"arm":"genie-new","op":"send","processId":2,"stdin":"search line \"retry_budget = 3\"\n","eof":false}}
{"ns":"166800373287721","request":{"arm":"genie-new","op":"send","processId":2,"stdin":"view @0 @1 @2 @3 @4 @5 @6 @7 --as paragraph\n","eof":false}}
{"ns":"166815717489494","request":{"arm":"genie-new","op":"send","processId":2,"stdin":"replace @0 \"retry_budget = 5\"\nreplace @3 \"retry_budget = 5\"\nreplace @4 \"retry_budget = 5\"\nreplace @7 \"retry_budget = 5\"\n","eof":false}}
{"ns":"166830242842508","request":{"arm":"genie-new","op":"send","processId":2,"stdin":"check @16 @17 @18 @19\nview @16 @17 @18 @19 --as file\n","eof":false}}
{"ns":"166844205439558","request":{"arm":"genie-new","op":"send","processId":2,"stdin":"exit\n","eof":true}}
{"ns":"166856844752544","request":{"arm":"genie-new","op":"finish"}}
```


#### Archived file: records/genie-new/delivered.jsonl

Bytes: 62800; SHA-256: `ee309d64756e97717a8aaca66b1b2c55f0969e95663210e3cfcac070a26247df`.

```jsonl
{"id":1,"bytes":5265,"sha256":"9c512202f1a31e020e323686d1cda92cb8b1a2067be52a940b00c5d7ee856c07","content":"{\"id\":1,\"events\":[{\"sequence\":1,\"ns\":\"166630187880145\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"IyBCYWNrd3JpdGVyCgpCYWNrd3JpdGVyIGlzIGEgUnVzdCBDb3JlIGFuZCBSdW50aW1lIGZvciBjdXJyZW50LCBzdHJ1Y3R1cmFsIHRleHQgd29yayBvdmVyCmFkbWl0dGVkIFdvcmtzcGFjZSBTb3VyY2UuIEl0IHByb3ZpZGVzIHRhcmdldC1sb2NhbCBGaWxlLCBQYXJhZ3JhcGgsIGFuZCBMaW5lCmFkZHJlc3NlcyB3aXRob3V0IHR1cm5pbmcgc291cmNlIGhpc3Rvcnkgb3IgZWRpdG9yIHN0YXRlIGludG8gQ29yZSBpZGVudGl0eS4KClRoZSBDb3JlIGNhcGFiaWxpdHkgaW52ZW50b3J5IGlzIFNlYXJjaCwgVmlldywgUGljaywgQW5jaG9yLCBDaGVjaywgRWRpdCwKQXBwbHksIGFuZCBEYXRhLiBUaGUgcmVwb3NpdG9yeSBjdXJyZW50bHkgcHJvdmlkZXMgdGhlaXIgUnVzdCBDb3JlL1J1bnRpbWUKc3VyZmFjZXMgYW5kIHRoZSBjYW5vbmljYWwgYGJ3YCBleGVjdXRhYmxlJ3Mgb25lLXNob3QgaHVtYW4gYW5kIEpTT04KU2VhcmNoL1ZpZXcvQ2hlY2svRWRpdCwgcmF3IFZpZXcsIEFuZGRyZXNzLWZpcnM=\",\"text\":\"# Backwriter\\n\\nBackwriter is a Rust Core and Runtime for current, structural text work over\\nadmitted Workspace Source. It provides target-local File, Paragraph, and Line\\naddresses without turning source history or editor state into Core identity.\\n\\nThe Core capability inventory is Search, View, Pick, Anchor, Check, Edit,\\nApply, and Data. The repository currently provides their Rust Core/Runtime\\nsurfaces and the canonical `bw` executable's one-shot human and JSON\\nSearch/View/Check/Edit, raw View, Anddress-firs\"},{\"sequence\":2,\"ns\":\"166630187904739\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"dCBvbmUtc2hvdCBFZGl0LCBTZXNzaW9uIFBpY2ssIGJhdGNoCkNoZWNrLCBBbmNob3IsIEVkaXQsIEFwcGx5LCByZXN1bHQtYmluZGluZywgZXhwbGljaXQgRGF0YSBtb2RlcywgYW5kCkFkYXB0ZXItb3duZWQgVmVyc2lvbiBhbmQgVXBkYXRlLgoKIyMgUXVpY2sgc3RhcnQKCkluc3RhbGwgdGhlIGNsb3NlZCBvZmZpY2lhbCBgMC4yLjZgIHJlbGVhc2Ugd2l0aCB0aGUgY29tbWFuZCBmb3IgeW91cgpwbGF0Zm9ybS4KCkxpbnV4LCBtYWNPUywgb3IgV1NMOgoKYGBgc2gKY3VybCAtZnNTTCBodHRwczovL2JhY2t3cml0ZXIucGVudGFncmF0aW9uLmNvbS9pbnN0YWxsLnNoIHwgc2gKYGBgCgpXaW5kb3dzIFBvd2VyU2hlbGw6CgpgYGBwb3dlcnNoZWxsCmlybSBodHRwczovL2JhY2t3cml0ZXIucGVudGFncmF0aW9uLmNvbS9pbnN0YWxsLnBzMSB8IGlleApgYGAKCldpbmRvd3MgQ01EOgoKYGBgY21kCmN1cmwuZXhlIC1mc1NMIGh0dHBzOi8vYmFja3dyaXRlci5wZW50YWdyYXRpb24uY29tL2luc3RhbGw=\",\"text\":\"t one-shot Edit, Session Pick, batch\\nCheck, Anchor, Edit, Apply, result-binding, explicit Data modes, and\\nAdapter-owned Version and Update.\\n\\n## Quick start\\n\\nInstall the closed official `0.2.6` release with the command for your\\nplatform.\\n\\nLinux, macOS, or WSL:\\n\\n```sh\\ncurl -fsSL https://backwriter.pentagration.com/install.sh | sh\\n```\\n\\nWindows PowerShell:\\n\\n```powershell\\nirm https://backwriter.pentagration.com/install.ps1 | iex\\n```\\n\\nWindows CMD:\\n\\n```cmd\\ncurl.exe -fsSL https://backwriter.pentagration.com/install\"},{\"sequence\":3,\"ns\":\"166630187917197\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"LmNtZCAtbyBpbnN0YWxsLmNtZCAmJiBjYWxsIGluc3RhbGwuY21kICYmIGRlbCBpbnN0YWxsLmNtZApgYGAKClRoZSBQT1NJWCBpbnN0YWxsZXIgcGxhY2VzIGBid2AgYXQgYCRIT01FLy5sb2NhbC9iaW4vYndgOyB0aGUgUG93ZXJTaGVsbCBhbmQKQ01EIHBhdGhzIHBsYWNlIGBidy5leGVgIGF0IGAkSE9NRVwubG9jYWxcYmluXGJ3LmV4ZWAuIFRoZXkgZG8gbm90IGNoYW5nZQpgUEFUSGAsIGEgc2hlbGwgc3RhcnR1cCBmaWxlLCB0aGUgUG93ZXJTaGVsbCBwcm9maWxlLCBvciB0aGUgcmVnaXN0cnkuIEEgZnJlc2gKaW5zdGFsbCBwcmludHMgYEluc3RhbGxlZCBCYWNrd3JpdGVyOiA8dmVyc2lvbj5gOyByZXBsYWNpbmcgYW4gZXhpc3RpbmcKZGVzdGluYXRpb24gcHJpbnRzIGBVcGRhdGVkIEJhY2t3cml0ZXI6IDx2ZXJzaW9uPmAuIFRoZSBleGVjdXRhYmxlIHBhdGggYW5kCmBQQVRIYCBndWlkYW5jZSBhcmUgcHJpbnRlZCBzZXBhcmF0ZWx5IG9ubHkgd2hlbiB0aGUgaW5zdGFsbGF0aW9uIGRpcmU=\",\"text\":\".cmd -o install.cmd && call install.cmd && del install.cmd\\n```\\n\\nThe POSIX installer places `bw` at `$HOME/.local/bin/bw`; the PowerShell and\\nCMD paths place `bw.exe` at `$HOME\\\\.local\\\\bin\\\\bw.exe`. They do not change\\n`PATH`, a shell startup file, the PowerShell profile, or the registry. A fresh\\ninstall prints `Installed Backwriter: <version>`; replacing an existing\\ndestination prints `Updated Backwriter: <version>`. The executable path and\\n`PATH` guidance are printed separately only when the installation dire\"},{\"sequence\":4,\"ns\":\"166630187927363\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"Y3RvcnkgaXMKbm90IGFscmVhZHkgb24gYFBBVEhgLgoKIyMjIFZlcnNpb24KCmBgYHNoCmJ3IHZlcnNpb24KYGBgCgpUaGUgY2xvc2VkIG9mZmljaWFsIGAwLjIuNmAgYnVpbGQgcHJpbnRzIGV4YWN0bHk6CgpgYGB0ZXh0CkJhY2t3cml0ZXIgMC4yLjYKYGBgCgojIyMgVXBkYXRlCgpgYGBzaApidyB1cGRhdGUKYGBgCgpgYncgdXBkYXRlYCBkb3dubG9hZHMgYW5kIGRlbGVnYXRlcyB0byB0aGUgY3VycmVudCBvZmZpY2lhbCBpbnN0YWxsZXIuIFRoZQpjdXJyZW50IG1hbmlmZXN0IHNlbGVjdHMgdGhlIGNsb3NlZCBgMC4yLjZgIGRpc3RyaWJ1dGlvbi4gVGhlIGluc3RhbGxlcgpyZWFkcyB0aGF0IG1hbmlmZXN0LCB2ZXJpZmllcyB0aGUgc2VsZWN0ZWQgYXJ0aWZhY3QsIGFuZCBpbnN0YWxscyBvciByZWluc3RhbGxzCnRoYXQgbWFuaWZlc3QgdmVyc2lvbiBvbmx5IGFmdGVyIHZhbGlkYXRpb24gc3VjY2VlZHMuIEl0IGRvZXMgbm90IHJ1biBhCmJhY2tncm91bmQgdXBkYXRlciBvciBjb21wYXJlIHI=\",\"text\":\"ctory is\\nnot already on `PATH`.\\n\\n### Version\\n\\n```sh\\nbw version\\n```\\n\\nThe closed official `0.2.6` build prints exactly:\\n\\n```text\\nBackwriter 0.2.6\\n```\\n\\n### Update\\n\\n```sh\\nbw update\\n```\\n\\n`bw update` downloads and delegates to the current official installer. The\\ncurrent manifest selects the closed `0.2.6` distribution. The installer\\nreads that manifest, verifies the selected artifact, and installs or reinstalls\\nthat manifest version only after validation succeeds. It does not run a\\nbackground updater or compare r\"}],\"more\":true,\"active\":[]}\n"}
{"id":2,"bytes":5232,"sha256":"7618623f1aa2cdc7b09c1161149f45bc552a4c14421b778d725f4e252e69df75","content":"{\"id\":2,\"events\":[{\"sequence\":5,\"ns\":\"166630187938656\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"ZWxlYXNlIHZlcnNpb25zLiBUaGUgaW5zdGFsbGVyIGFjY2VwdHMgb25seSB0aGUKZXhhY3QgaW1tdXRhYmxlIGAwLjIuNWAgbWFuaWZlc3QgYW5kIGN1cnJlbnQgYDAuMi42YCBtYW5pZmVzdDsgYDAuMi40YCwgYDAuMi4zYCwgYDAuMi4yYCwgYDAuMi4xYCwgYDAuMi4wYCwgc3RhYmxlCmAwLjEuMGAsIGFuZCBiZXRhLjMgYWNjZXB0YW5jZSBpcyByZXRpcmVkLiBUaGUgY29tbWFuZCBzdGlsbCBwZXJmb3JtcyBubwp2ZXJzaW9uIGNvbXBhcmlzb24gYW5kIGFkZHMgbm8gdmVyc2lvbiBndWFyZC4KClRoZSBwcm9kdWN0IGlzIEJhY2t3cml0ZXIuIFRoZSBzb3VyY2UgQ2FyZ28gcGFja2FnZSBhbmQgbGlicmFyeSBjcmF0ZSBhcmUKYGJhY2t3cml0ZXJgIGF0IHB1Ymxpc2hlZCBhbmQgY2xvc2VkIGAwLjIuNmA7IGEgc291cmNlLWJ1aWx0IGBidyB2ZXJzaW9uYApwcmludHMgYEJhY2t3cml0ZXIgMC4yLjZgLiBUaGUgc29sZSBjYW5vbmljYWwgZXhlY3V0YWJsZSBhbmQgZXh0ZXJuYWwgQWRhcHRlcgpjb20=\",\"text\":\"elease versions. The installer accepts only the\\nexact immutable `0.2.5` manifest and current `0.2.6` manifest; `0.2.4`, `0.2.3`, `0.2.2`, `0.2.1`, `0.2.0`, stable\\n`0.1.0`, and beta.3 acceptance is retired. The command still performs no\\nversion comparison and adds no version guard.\\n\\nThe product is Backwriter. The source Cargo package and library crate are\\n`backwriter` at published and closed `0.2.6`; a source-built `bw version`\\nprints `Backwriter 0.2.6`. The sole canonical executable and external Adapter\\ncom\"},{\"sequence\":6,\"ns\":\"166630187957745\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"bWFuZCBhcmUgYGJ3YC4gVGhlcmUgaXMgbm8gYGJhY2t3cml0ZXJgIGJpbmFyeSwgYWxpYXMsIG9yIHdyYXBwZXIuIFRoZQpvZmZpY2lhbCBpbnN0YWxsZXIgc2VsZWN0cyB0aGUgY2xvc2VkIHB1YmxpYyBgMC4yLjZgCmRpc3RyaWJ1dGlvbi4KClRoZSBwdWJsaXNoZWQgYDAuMi42YCBTb3VyY2UgQXV0aG9yaXR5J3MgYHNyYy8qKmAgaXMgYnl0ZS1pZGVudGljYWwgdG8gaXRzIEdhdGUgNgpjYW5kaWRhdGUgYGM3OGUwN2YyNDIwMzUyMzBlOGIwNzFkNTgzNDkxYWM2MzNmNThkMjlgOyB0aGF0IGNsYWltIGRvZXMgbm90IGNvdmVyCnRoaXMgbGF0ZXIgY2hlY2tvdXQuIFVucHVibGlzaGVkIGAwLjMuMGAgR2F0ZXMgMeKAkzQgYWRkIHRoZSBwcml2YXRlIG5hbWVzcGFjZQpib3VuZGFyeSwgY29tcGxldGUgZGlyZWN0IHNoZWxsIFZpZXcsIGRpc2NvdmVyYWJsZSBIZWxwIGFuZCBwcml2YXRlIENMSSBtb2R1bGVzLgpDYXJnbyBhbmQgYGJ3IHZlcnNpb25gIHN0aWxsIHJlcG9ydCBgMC4yLjZgOyByZWFkaW4=\",\"text\":\"mand are `bw`. There is no `backwriter` binary, alias, or wrapper. The\\nofficial installer selects the closed public `0.2.6`\\ndistribution.\\n\\nThe published `0.2.6` Source Authority's `src/**` is byte-identical to its Gate 6\\ncandidate `c78e07f242035230e8b071d583491ac633f58d29`; that claim does not cover\\nthis later checkout. Unpublished `0.3.0` Gates 1–4 add the private namespace\\nboundary, complete direct shell View, discoverable Help and private CLI modules.\\nCargo and `bw version` still report `0.2.6`; readin\"},{\"sequence\":7,\"ns\":\"166630187968584\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"ZXNzIGFuZCByZWxlYXNlIHJlbWFpbiBwZW5kaW5nLgpVc2UgYGJ3IGhlbHAgc2hlbGxgIGZvciBzaG9ydC1yZWZlcmVuY2UgU2VhcmNoL1ZpZXcvUmVwbGFjZS9DaGVjayBhbmQKYGJ3IGhlbHAgcGlja2AsIGBidyBoZWxwIGFuY2hvcmAsIGBidyBoZWxwIGFwcGx5YCwgb3IgYGJ3IGhlbHAgZGF0YWAgZm9yIGFkdmFuY2VkCnJhdyBTZXNzaW9uIHRvcGljcy4gVGhlc2UgSGVscCB0b3BpY3MgYWRkIG5vIG9uZS1zaG90IGNhcGFiaWxpdHkgZXhlY3V0aW9uLgpSMyByZWxlYXNlIGNsb3N1cmUgaXMgY29tcGxldGU6IFNvdXJjZSBBdXRob3JpdHkKYDA5YmI2YzQyNDA4MTU5NGJkODZhOTVmMDQzNDViNzg2ZWY5YjQ2YjZgIHN1cHBsaWVzIHB1Ymxpc2hlZCBhbmQgY2xvc2VkCmAwLjIuNmAuIE9mZmljaWFsIGFydGlmYWN0cywgaW5zdGFsbGVycywgbWFuaWZlc3QsIGFuZCBVcGRhdGUgbm93IHNlbGVjdApgMC4yLjZgIGluIHRoZSBleGFjdCA3Ni1maWxlIHRyZWUuIEluc3RhbGxlcnMgYWNjZXB0IG9ubHk=\",\"text\":\"ess and release remain pending.\\nUse `bw help shell` for short-reference Search/View/Replace/Check and\\n`bw help pick`, `bw help anchor`, `bw help apply`, or `bw help data` for advanced\\nraw Session topics. These Help topics add no one-shot capability execution.\\nR3 release closure is complete: Source Authority\\n`09bb6c424081594bd86a95f04345b786ef9b46b6` supplies published and closed\\n`0.2.6`. Official artifacts, installers, manifest, and Update now select\\n`0.2.6` in the exact 76-file tree. Installers accept only\"},{\"sequence\":8,\"ns\":\"166630187976744\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"IGV4YWN0IGAwLjIuNWAgYW5kCmAwLjIuNmAgbWFuaWZlc3RzLiBVcGRhdGUgc3RpbGwgcGVyZm9ybXMgbm8gdmVyc2lvbiBjb21wYXJpc29uLiBQcm9kdWN0aW9uClJ1c3QsIENhcmdvLCB0ZXN0cywgdG9vbGNoYWluLCBhbmQgdjUgd2lyZSBhcmUgdW5jaGFuZ2VkIGJ5IHJlbGVhc2UgY2xvc3VyZS4KCiMjIEFuZGRyZXNzLWZpcnN0IGVkaXRpbmcKClRoZSBkZWZhdWx0IHJlcGxhY2VtZW50IGZsb3cgaXM6CgoxLiBSdW4gYGJ3IC0tanNvbiBzZWFyY2ggLi4uYC4KMi4gU2VsZWN0IG9uZSBvY2N1cnJlbmNlIGZyb20gYG9jY3VycmVuY2VzYCBhbmQgcGFzcyBpdHMgZXhhY3QgZW1iZWRkZWQgdjUKICAgYGFuZGRyZXNzYCBvYmplY3QgdW5jaGFuZ2VkIGFzIG9uZSBhcmd2IHZhbHVlLgozLiBSdW4gYGJ3IGVkaXQgYW5kZHJlc3MgJzxvcGFxdWUtdjUtb2JqZWN0PicgJzxuZXctY29udGVudD4nYC4KCkh1bWFuIFNlYXJjaCByb3dzIGFyZSBub3QgZW5jb2RlZCBBbmRkcmVzcyB2YWx1ZXMgYW5kIGNhbm4=\",\"text\":\" exact `0.2.5` and\\n`0.2.6` manifests. Update still performs no version comparison. Production\\nRust, Cargo, tests, toolchain, and v5 wire are unchanged by release closure.\\n\\n## Anddress-first editing\\n\\nThe default replacement flow is:\\n\\n1. Run `bw --json search ...`.\\n2. Select one occurrence from `occurrences` and pass its exact embedded v5\\n   `anddress` object unchanged as one argv value.\\n3. Run `bw edit anddress '<opaque-v5-object>' '<new-content>'`.\\n\\nHuman Search rows are not encoded Anddress values and cann\"}],\"more\":true,\"active\":[]}\n"}
{"id":3,"bytes":5231,"sha256":"960774d46e58c4f0dbeb8ccc518524d3cdc434bebaea68e9d612a183dd933534","content":"{\"id\":3,\"events\":[{\"sequence\":9,\"ns\":\"166630187982909\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"b3QgYmUgRWRpdCBpbnB1dC4KVHJlYXQgdGhlIHNlbGVjdGVkIEpTT04gb2JqZWN0IGFzIG9wYXF1ZTogZG8gbm90IGludGVycHJldCBvciByZXdyaXRlIGl0cyBoYXNoLApyYW5nZSwgbGVuZ3RoLCBvciBvdGhlciBmaWVsZHMuIEZpbGUgYW5kIFBhcmFncmFwaCBDb250ZW50IGlzIHRoZSBleGFjdApyZXBsYWNlbWVudC4gTGluZSBDb250ZW50IGlzIGJvZHktb25seSwgcmVqZWN0cyBOVUwsIENSLCBhbmQgTEYsIGFuZCBwcmVzZXJ2ZXMKdGhlIE5vbmUsIExGLCBDUiwgb3IgQ1JMRiB0ZXJtaW5hdG9yIGNhcnJpZWQgYnkgdGhlIGV4YWN0IHY1IExpbmUuIEEgTGluZSBOVUwKaXMgYGVkaXQuY29udGVudF9jb250YWluc19udWxgOyBhIExpbmUgQ1Igb3IgTEYgaXMKYGVkaXQubGluZV9ib2R5X2NvbnRhaW5zX3Rlcm1pbmF0b3JgLiBUaGUgbGF0dGVyIGV4cGxhaW5zIHRoYXQgQmFja3dyaXRlciBhZGRzCnRoZSBjdXJyZW50IHRlcm1pbmF0b3IgYW5kIHRoYXQgYWR2YW5jZWQgcmF3IFNlc3Npb24gRWQ=\",\"text\":\"ot be Edit input.\\nTreat the selected JSON object as opaque: do not interpret or rewrite its hash,\\nrange, length, or other fields. File and Paragraph Content is the exact\\nreplacement. Line Content is body-only, rejects NUL, CR, and LF, and preserves\\nthe None, LF, CR, or CRLF terminator carried by the exact v5 Line. A Line NUL\\nis `edit.content_contains_nul`; a Line CR or LF is\\n`edit.line_body_contains_terminator`. The latter explains that Backwriter adds\\nthe current terminator and that advanced raw Session Ed\"},{\"sequence\":10,\"ns\":\"166630187989712\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"aXQvQXBwbHkgb3ducyBleGFjdApleHRlbnQgcmVwbGFjZW1lbnQuIEFwcGx5IGFsb25lIGNvbmZpcm1zIHRoYXQgc291cmNlIHN0YXRlIGJlZm9yZSBwdWJsaWNhdGlvbi4KVmlldyBvciBQaWNrIG1heSBoZWxwIGEgY2FsbGVyIHNlbGVjdCBhIHRhcmdldDsgbmVpdGhlciBWaWV3IG5vciBDaGVjayBpcwpyZXF1aXJlZC4KCkh1bWFuIHN1Y2Nlc3Mgd3JpdGVzIG9uZSBleGFjdCBMRi10ZXJtaW5hdGVkIHJlY2VpcHQgcm93LiBgVW5jaGFuZ2VkYCBpcwpmb2xsb3dlZCBieSB0aGUgc3RpbGwtY3VycmVudCBpbnB1dCB2NSBvYmplY3Q7IGBDaGFuZ2VkYCBpcyBmb2xsb3dlZCBieSB0aGUKZnJlc2ggdjUgb2JqZWN0IHdoZW4gdGhlIHJlc3VsdGluZyBGaWxlLCBMaW5lLCBvciB1bmlxdWUgUGFyYWdyYXBoIGhhcyBvbmUsCmFuZCBvdGhlcndpc2UgYnkgYE5vbmVgLiBXaXRoIGxlYWRpbmcgYC0tanNvbmAsIHRoZSBzYW1lIHJlc3VsdCBpcyB0aGUgY29tcGFjdApBZGFwdGVyLW9ubHkgYGJ3LmNsaS5lZGk=\",\"text\":\"it/Apply owns exact\\nextent replacement. Apply alone confirms that source state before publication.\\nView or Pick may help a caller select a target; neither View nor Check is\\nrequired.\\n\\nHuman success writes one exact LF-terminated receipt row. `Unchanged` is\\nfollowed by the still-current input v5 object; `Changed` is followed by the\\nfresh v5 object when the resulting File, Line, or unique Paragraph has one,\\nand otherwise by `None`. With leading `--json`, the same result is the compact\\nAdapter-only `bw.cli.edi\"},{\"sequence\":11,\"ns\":\"166630187996352\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"dC52MWAgb2JqZWN0IHdpdGggYHNjaGVtYWAsIGBvdXRjb21lYCwgYW5kIGBhbmRkcmVzc2AKa2V5cyBpbiB0aGF0IG9yZGVyOyBgYW5kZHJlc3NgIGlzIHRoZSBleGFjdCB2NSBvYmplY3Qgb3IgSlNPTiBgbnVsbGAuIFJldXNlIG9ubHkKdGhlIGFkZHJlc3MgcmV0dXJuZWQgYnkgdGhlIHJlY2VpcHQuIEEgY2hhbmdlZCBQYXJhZ3JhcGggd2l0aCBgTm9uZWAgcmVxdWlyZXMKYW4gZXhwbGljaXQgU2VhcmNoIGJlZm9yZSBsYXRlciB0YXJnZXQgd29yay4gRXhpdCBgMWAgaXMgbmVpdGhlciBhIHN0YWxlLW9ubHkKY2xhc3NpZmljYXRpb24gbm9yIHByb29mIHRoYXQgc291cmNlIGJ5dGVzIGFyZSB1bmNoYW5nZWQsIHNvIGl0IG11c3Qgbm90CnRyaWdnZXIgYXV0b21hdGljIHJldHJ5LgoKT25lLXNob3QgQ29udGVudCBpcyBlaXRoZXIgb25lIFVURi04IGFyZ3YgdmFsdWUgb3IgdGhlIGV4Y2x1c2l2ZSBgLS1zdGRpbmAKc2VsZWN0b3IgaW4gdGhhdCBwb3NpdGlvbjsgc3RkaW4gaXMgcmVhZCB0byBFT0Y=\",\"text\":\"t.v1` object with `schema`, `outcome`, and `anddress`\\nkeys in that order; `anddress` is the exact v5 object or JSON `null`. Reuse only\\nthe address returned by the receipt. A changed Paragraph with `None` requires\\nan explicit Search before later target work. Exit `1` is neither a stale-only\\nclassification nor proof that source bytes are unchanged, so it must not\\ntrigger automatic retry.\\n\\nOne-shot Content is either one UTF-8 argv value or the exclusive `--stdin`\\nselector in that position; stdin is read to EOF\"},{\"sequence\":12,\"ns\":\"166630188009840\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"IGFmdGVyIHY1IGFkZHJlc3MgdmFsaWRhdGlvbiBhbmQKYmVmb3JlIFJ1bnRpbWUgYWNjZXNzLiBCb3RoIGZvcm1zIGhhdmUgdGhlIHNhbWUgdGFyZ2V0LXNwZWNpZmljIENvbnRlbnQgcnVsZXMuCkZpbGUvUGFyYWdyYXBoIENSIGFuZCBMRiBhcmUgZXhhY3QgQ29udGVudCwgd2hpbGUgTGluZSBib2R5IENvbnRlbnQgbmV2ZXIKc2lsZW50bHkgc3RyaXBzIGEgdHJhaWxpbmcgbmV3bGluZS4gTGl0ZXJhbCBgLS1qc29uYCBhbmQgYC0tcmF3YCByZW1haW4gZXhhY3QKQ29udGVudDsgYSBsaXRlcmFsIGAtLXN0ZGluYCBDb250ZW50IHZhbHVlIGlzIHN1cHBsaWVkIHRocm91Z2ggc3RhbmRhcmQgaW5wdXQuCgpSYXcgU2Vzc2lvbiBpcyB0aGUgYWR2YW5jZWQgY29tcG9zaXRpb24gc3VyZmFjZSBmb3IgSW5zZXJ0L0RlbGV0ZS9Nb3ZlL0NvcHksClBvc2l0aW9uLCBBbmNob3IvRGF0YSBsaWZldGltZSwgZXhwbGljaXQgYmluZGluZ3MsIGFuZCBzZXBhcmF0ZSBBcHBseS4gSXQgaXMKbm90IGEgcHJlcmVxdWk=\",\"text\":\" after v5 address validation and\\nbefore Runtime access. Both forms have the same target-specific Content rules.\\nFile/Paragraph CR and LF are exact Content, while Line body Content never\\nsilently strips a trailing newline. Literal `--json` and `--raw` remain exact\\nContent; a literal `--stdin` Content value is supplied through standard input.\\n\\nRaw Session is the advanced composition surface for Insert/Delete/Move/Copy,\\nPosition, Anchor/Data lifetime, explicit bindings, and separate Apply. It is\\nnot a prerequi\"}],\"more\":true,\"active\":[]}\n"}
{"id":4,"bytes":5250,"sha256":"22ed378ea57e15450b373c44bb7c926b526a48deaec142fea4c70ac74611a117","content":"{\"id\":4,\"events\":[{\"sequence\":13,\"ns\":\"166630188016026\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"c2l0ZSBvciBhbGlhcyBmb3Igb3JkaW5hcnkgUmVwbGFjZS4gSXRzIGV4aXN0aW5nIHJhdwpgZWRpdCByZXBsYWNlYCBhY2NlcHRzIGNhbGxlci1wcm92aWRlZCBleGFjdCByYW5nZSBDb250ZW50LCBpbmNsdWRpbmcgYW4KZXhwbGljaXQgdGVybWluYXRvciBvciBtdWx0aWxpbmUgcmVwbGFjZW1lbnQsIGFuZCBgYXBwbHkgQGVkaXRgIHB1Ymxpc2hlcyBpdApzZXBhcmF0ZWx5LiBUaGUgY2FsbGVyIG93bnMgdGhlIGJpbmRpbmcsIGluZGV4LCBxdW90aW5nLCB0ZXJtaW5hdG9yLCBhbmQKcHVibGljYXRpb24gYm91bmRhcnkuIEdlbmVyYWwgcmVwbGFjZW1lbnQgc2hvdWxkIHVzZSBvbmUtc2hvdCBib2R5IENvbnRlbnQKZmlyc3Q7IG5vIGV4YWN0IG9uZS1zaG90IGZsYWcgb3IgYWx0ZXJuYXRlIGV4ZWN1dG9yIGV4aXN0cy4KCiMjIyBTaGVsbC1sb2NhbCByZWZlcmVuY2VzCgpgYncgc2hlbGxgIGFsc28gcHJvdmlkZXMgYSBzaG9ydC1saXZlZCBpbnRlcmFjdGl2ZSBmbG93IHdpdGhvdXQgYWRkaW5nIGE=\",\"text\":\"site or alias for ordinary Replace. Its existing raw\\n`edit replace` accepts caller-provided exact range Content, including an\\nexplicit terminator or multiline replacement, and `apply @edit` publishes it\\nseparately. The caller owns the binding, index, quoting, terminator, and\\npublication boundary. General replacement should use one-shot body Content\\nfirst; no exact one-shot flag or alternate executor exists.\\n\\n### Shell-local references\\n\\n`bw shell` also provides a short-lived interactive flow without adding a\"},{\"sequence\":14,\"ns\":\"166630188023600\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"IENvcmUKd2lyZSBvciBwZXJzaXN0ZW50IGlkZW50aXR5OgoKYGBgdGV4dApzZWFyY2ggbGluZSBuZWVkbGUKdmlldyBAMApyZXBsYWNlIEAxIHJlcGxhY2VtZW50CmNoZWNrIEAyIEAzCmV4aXQKYGBgCgpRdW90ZSBvbmUgZGlyZWN0LXNoZWxsIHF1ZXJ5IG9yIHJlcGxhY2VtZW50IGFyZ3VtZW50IHdoZW4gaXQgY29udGFpbnMKd2hpdGVzcGFjZTsgdGhlIHF1b3RlcyBhcmUgc2hlbGwgc3ludGF4IGFuZCBhcmUgbm90IHBhcnQgb2YgdGhlIGV4YWN0IGxpdGVyYWwKb3IgcmVwbGFjZW1lbnQgQ29udGVudDoKCmBgYHRleHQKc2VhcmNoIGxpbmUgImR1cGxpY2F0ZSA9IG9uZSIKcmVwbGFjZSBAMSAiZHVwbGljYXRlID0gdHdvIgpgYGAKClN1Y2Nlc3NmdWwgZGlyZWN0IGBzZWFyY2hgIGFuZCBwcm9qZWN0ZWQgZGlyZWN0IGB2aWV3YCBhcHBlbmQgYEBOYCByZWZlcmVuY2VzCmluIG91dHB1dCBvcmRlciwgaW5jbHVkaW5nIGR1cGxpY2F0ZXMuIERpcmVjdCBgY2hlY2sgPFJFRj4uLi5gIHJlc29sdmVzIGU=\",\"text\":\" Core\\nwire or persistent identity:\\n\\n```text\\nsearch line needle\\nview @0\\nreplace @1 replacement\\ncheck @2 @3\\nexit\\n```\\n\\nQuote one direct-shell query or replacement argument when it contains\\nwhitespace; the quotes are shell syntax and are not part of the exact literal\\nor replacement Content:\\n\\n```text\\nsearch line \\\"duplicate = one\\\"\\nreplace @1 \\\"duplicate = two\\\"\\n```\\n\\nSuccessful direct `search` and projected direct `view` append `@N` references\\nin output order, including duplicates. Direct `check <REF>...` resolves e\"},{\"sequence\":15,\"ns\":\"166630188029809\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"dmVyeQpyZWZlcmVuY2UgYmVmb3JlIFJ1bnRpbWUgYWNjZXNzLCB3cml0ZXMgb25lIEN1cnJlbnQvTm90Q3VycmVudC9VbmF2YWlsYWJsZQpzdGF0ZSBwZXIgaW5wdXQsIGFuZCBhcHBlbmRzIGEgZnJlc2ggc2xvdCBvbmx5IGZvciBDdXJyZW50LiBgcmVwbGFjZSBATiA8Y29udGVudD5gIHVzZXMgdGhlIHNhbWUKdGFyZ2V0LWF3YXJlIENvbnRlbnQgcnVsZXMgYXMgb25lLXNob3QgRWRpdCBhbmQgYXBwZW5kcyBhIGZyZXNoIHJlZmVyZW5jZSBmb3IKYFVuY2hhbmdlZGAgb3IgYENoYW5nZWRgIHdoZW4gdGhlIHJlY2VpcHQgaGFzIGFuIEFuZGRyZXNzOyBgQ2hhbmdlZFx0Tm9uZWAKYWRkcyBub25lLiBTbG90cyBhcmUgemVyby1iYXNlZCBjYW5vbmljYWwgdW5zaWduZWQgZGVjaW1hbHMsIGFwcGVuZC1vbmx5LCBhbmQKZGlzY2FyZGVkIGF0IGBleGl0YCBvciBFT0YuIGBAbmFtZWAgYW5kIGBAbmFtZVtpbmRleF1gIHJlbWFpbiB0aGUgYWR2YW5jZWQgcmF3ClNlc3Npb24gZm9ybXM7IGBsZXQgbmFtZSA=\",\"text\":\"very\\nreference before Runtime access, writes one Current/NotCurrent/Unavailable\\nstate per input, and appends a fresh slot only for Current. `replace @N <content>` uses the same\\ntarget-aware Content rules as one-shot Edit and appends a fresh reference for\\n`Unchanged` or `Changed` when the receipt has an Anddress; `Changed\\\\tNone`\\nadds none. Slots are zero-based canonical unsigned decimals, append-only, and\\ndiscarded at `exit` or EOF. `@name` and `@name[index]` remain the advanced raw\\nSession forms; `let name \"},{\"sequence\":16,\"ns\":\"166630188048574\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"PSBATmAgZXhwbGljaXRseSBjbG9uZXMgYSBudW1lcmljIHNsb3QgaW50byB0aGF0CmV4aXN0aW5nIG5hbWVkIEFuZGRyZXNzIGJpbmRpbmcuIFJhdyBgZWRpdCByZXBsYWNlYCBhbmQgc2VwYXJhdGUgYGFwcGx5IEBlZGl0YApyZW1haW4gdGhlIGFkdmFuY2VkIGV4YWN0LXJhbmdlIHBhdGguCgpUaGUgZGVmYXVsdCB3b3Jrc3BhY2UgaXMgdGhlIHByb2Nlc3MgY3VycmVudCB3b3JraW5nIGRpcmVjdG9yeS4gQW4gZXhwbGljaXQKYC0td29ya3NwYWNlYCBtdXN0IGJlIGFic29sdXRlIGFuZCBpcyBjaGVja2VkIGJ5IFJ1bnRpbWUuIFNlYXJjaCBhZG1pdHMgYC5gIGJ5CmRlZmF1bHQ7IHJlcGVhdCBgLS1hZG1pdCBMT0dJQ0FMX1BBVEhgIGJlZm9yZSBgc2VhcmNoYCB0byBuYXJyb3cgYWRtaXNzaW9uLgpBZnRlciB0aGUgcXVlcnksIHJlcGVhdCBgLS1zb3VyY2UgTE9HSUNBTF9QQVRIYCBvciBgLS1zdWJ0cmVlIExPR0lDQUxfUEFUSGAgdG8KbmFycm93IGEgU2VhcmNoIHNjb3BlLiBXaXRob3V0IGEgc2M=\",\"text\":\"= @N` explicitly clones a numeric slot into that\\nexisting named Anddress binding. Raw `edit replace` and separate `apply @edit`\\nremain the advanced exact-range path.\\n\\nThe default workspace is the process current working directory. An explicit\\n`--workspace` must be absolute and is checked by Runtime. Search admits `.` by\\ndefault; repeat `--admit LOGICAL_PATH` before `search` to narrow admission.\\nAfter the query, repeat `--source LOGICAL_PATH` or `--subtree LOGICAL_PATH` to\\nnarrow a Search scope. Without a sc\"}],\"more\":true,\"active\":[]}\n"}
{"id":5,"bytes":5246,"sha256":"298a4d4a6b04981c560fe72df1bf1b95d956c6b0061a80172133e57ae746fdf2","content":"{\"id\":5,\"events\":[{\"sequence\":17,\"ns\":\"166630188057568\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"b3BlIHNlbGVjdG9yLCBTZWFyY2ggY292ZXJzIGFsbCBhZG1pdHRlZApzb3VyY2VzLgoKIyMgQ3VycmVudCBDTEkgc2NvcGUKCmBid2AgY3VycmVudGx5IGltcGxlbWVudHMgQWRhcHRlci1vd25lZCBvbmUtc2hvdCBWZXJzaW9uIGFuZCBVcGRhdGUsIG9uZS1zaG90Cmh1bWFuIG9yIEpTT04gU2VhcmNoLCBWaWV3LCBDaGVjaywgYW5kIEVkaXQsIHJhdyBWaWV3LCBBbmRkcmVzcy1maXJzdCBvbmUtc2hvdCBFZGl0LApwbHVzIFNlc3Npb24gUGljaywgYmF0Y2ggQ2hlY2ssIEFuY2hvciwgRWRpdCwgQXBwbHksIGFuZCBEYXRhOgoKYGBgdGV4dApidyB2ZXJzaW9uCmJ3IHVwZGF0ZQpidyBbLS13b3Jrc3BhY2UgQUJTT0xVVEVfUEFUSF0gWy0tYWRtaXQgTE9HSUNBTF9QQVRIXS4uLgogICAgc2VhcmNoIDxsaW5lfHBhcmFncmFwaHxmaWxlPiA8cXVlcnk+CiAgICBbLS1zb3VyY2UgTE9HSUNBTF9QQVRIIHwgLS1zdWJ0cmVlIExPR0lDQUxfUEFUSF0uLi4KYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEg=\",\"text\":\"ope selector, Search covers all admitted\\nsources.\\n\\n## Current CLI scope\\n\\n`bw` currently implements Adapter-owned one-shot Version and Update, one-shot\\nhuman or JSON Search, View, Check, and Edit, raw View, Anddress-first one-shot Edit,\\nplus Session Pick, batch Check, Anchor, Edit, Apply, and Data:\\n\\n```text\\nbw version\\nbw update\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\\n    search <line|paragraph|file> <query>\\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\\nbw [--workspace ABSOLUTE_PATH\"},{\"sequence\":18,\"ns\":\"166630188064590\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"XSBbLS1hZG1pdCBMT0dJQ0FMX1BBVEhdLi4uCiAgICBzZWFyY2ggL2ZpbGUgPGxvZ2ljYWwtcGF0aD4KYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4gLS1qc29uCiAgICBzZWFyY2ggPGxpbmV8cGFyYWdyYXBofGZpbGU+IDxxdWVyeT4KICAgIFstLXNvdXJjZSBMT0dJQ0FMX1BBVEggfCAtLXN1YnRyZWUgTE9HSUNBTF9QQVRIXS4uLgpidyBbLS13b3Jrc3BhY2UgQUJTT0xVVEVfUEFUSF0gWy0tYWRtaXQgTE9HSUNBTF9QQVRIXS4uLiAtLWpzb24KICAgIHNlYXJjaCAvZmlsZSA8bG9naWNhbC1wYXRoPgpidyBbLS13b3Jrc3BhY2UgQUJTT0xVVEVfUEFUSF0gWy0tYWRtaXQgTE9HSUNBTF9QQVRIXS4uLiAtLWpzb24KICAgIHZpZXcgYW5kZHJlc3MgPGVuY29kZWQtdjUtQW5kZHJlc3M+Li4uIFstLWFzIDxsaW5lfHBhcmFncmFwaHxmaWxlPl0KYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4gLS0=\",\"text\":\"] [--admit LOGICAL_PATH]...\\n    search /file <logical-path>\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\\n    search <line|paragraph|file> <query>\\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\\n    search /file <logical-path>\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\\n    view anddress <encoded-v5-Anddress>... [--as <line|paragraph|file>]\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --\"},{\"sequence\":19,\"ns\":\"166630188071576\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"anNvbgogICAgY2hlY2sgYW5kZHJlc3MgPGVuY29kZWQtdjUtQW5kZHJlc3M+Li4uCmJ3IFstLXdvcmtzcGFjZSBBQlNPTFVURV9QQVRIXSBbLS1hZG1pdCBMT0dJQ0FMX1BBVEhdLi4uIC0tcmF3CiAgICB2aWV3IGFuZGRyZXNzIDxlbmNvZGVkLXY1LUFuZGRyZXNzPiBbLS1hcyA8bGluZXxwYXJhZ3JhcGh8ZmlsZT5dCmJ3IFstLXdvcmtzcGFjZSBBQlNPTFVURV9QQVRIXSBbLS1hZG1pdCBMT0dJQ0FMX1BBVEhdLi4uCiAgICB2aWV3IGFuZGRyZXNzIDxlbmNvZGVkLXY1LUFuZGRyZXNzPiBbLS1hcyA8bGluZXxwYXJhZ3JhcGh8ZmlsZT5dCmJ3IFstLXdvcmtzcGFjZSBBQlNPTFVURV9QQVRIXSBbLS1hZG1pdCBMT0dJQ0FMX1BBVEhdLi4uCiAgICBjaGVjayBhbmRkcmVzcyA8ZW5jb2RlZC12NS1BbmRkcmVzcz4KYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4KICAgIGVkaXQgYW5kZHJlc3MgPGVuY29kZWQtdjUtQW5kZHJlc3M+IDxjb250ZW50Pgo=\",\"text\":\"json\\n    check anddress <encoded-v5-Anddress>...\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --raw\\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\\n    check anddress <encoded-v5-Anddress>\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\\n    edit anddress <encoded-v5-Anddress> <content>\\n\"},{\"sequence\":20,\"ns\":\"166630188079153\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"YncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4KICAgIGVkaXQgYW5kZHJlc3MgPGVuY29kZWQtdjUtQW5kZHJlc3M+IC0tc3RkaW4KYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4gLS1qc29uCiAgICBlZGl0IGFuZGRyZXNzIDxlbmNvZGVkLXY1LUFuZGRyZXNzPiA8Y29udGVudD4KYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4gLS1qc29uCiAgICBlZGl0IGFuZGRyZXNzIDxlbmNvZGVkLXY1LUFuZGRyZXNzPiAtLXN0ZGluCmJ3IFstLXdvcmtzcGFjZSBBQlNPTFVURV9QQVRIXSBbLS1hZG1pdCBMT0dJQ0FMX1BBVEhdLi4uIHNoZWxsCmBgYAoKVmVyc2lvbiBhbmQgVXBkYXRlIGRvIG5vdCBjYWxsIEJhY2t3cml0ZXIgQ29yZSBvciBSdW50aW1lIGFuZCBjcmVhdGUgbm8gQ29yZQp3aXJlIG9yIGNhcGFiaWxpdHkgd29ya2Zsb3cuCgpDb250ZW4=\",\"text\":\"bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\\n    edit anddress <encoded-v5-Anddress> --stdin\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\\n    edit anddress <encoded-v5-Anddress> <content>\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\\n    edit anddress <encoded-v5-Anddress> --stdin\\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\\n```\\n\\nVersion and Update do not call Backwriter Core or Runtime and create no Core\\nwire or capability workflow.\\n\\nConten\"}],\"more\":true,\"active\":[]}\n"}
{"id":6,"bytes":5229,"sha256":"da713a1a6d7e3046ad53076f4ee4716e78ae1e530f4dc9436990c5881a05a816","content":"{\"id\":6,\"events\":[{\"sequence\":21,\"ns\":\"166630188087973\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"dCBTZWFyY2ggcHJlc2VydmVzIENvcmUgbGl0ZXJhbCB2YWxpZGF0aW9uLCBzY29wZSwgcHJvamVjdGlvbiwgYW5kCmRldGVybWluaXN0aWMgcmVzdWx0IG9yZGVyLiBUaGUgZGlzdGluY3QgYHNlYXJjaCAvZmlsZWAgZm9ybSB2YWxpZGF0ZXMgb25lCmxvZ2ljYWwgcGF0aCBhbmQgcmV0dXJucyB0aGUgY3VycmVudCBGaWxlIEFuZGRyZXNzIGZvciBhbiBhZG1pdHRlZCByZWd1bGFyClVURi04LCBOVUwtZnJlZSBzb3VyY2UgcmVnYXJkbGVzcyBvZiB3aGV0aGVyIGl0IGlzIGVtcHR5IG9yIGNvbnRhaW5zIG1hdGNoaW5nCnRleHQuIE1pc3NpbmcgcGF0aHMgYW5kIGRpcmVjdG9yaWVzIHJldHVybiBFbXB0eTsgdGhlIGZvcm0gaGFzIG5vIHNjb3BlCnNlbGVjdG9ycyBvciBzeW50aGV0aWMgY29udGVudCBxdWVyeS4gVmlldyBkZWNvZGVzIHY1IEFuZGRyZXNzZXMgYW5kIHByb2plY3RzCmVhY2ggdG8gaXRzZWxmIG9yIG9uZSBhbmNlc3RvciBiZWZvcmUgcmVhZGluZyBpdHMgZXhhY3QgY29udGVudC4gQSA=\",\"text\":\"t Search preserves Core literal validation, scope, projection, and\\ndeterministic result order. The distinct `search /file` form validates one\\nlogical path and returns the current File Anddress for an admitted regular\\nUTF-8, NUL-free source regardless of whether it is empty or contains matching\\ntext. Missing paths and directories return Empty; the form has no scope\\nselectors or synthetic content query. View decodes v5 Anddresses and projects\\neach to itself or one ancestor before reading its exact content. A \"},{\"sequence\":22,\"ns\":\"166630188094253\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"c2luZ2xlIGlucHV0CmRlZmF1bHRzIHRvIHNlbGYgcHJvamVjdGlvbi4gYC0tYXNgIGNob29zZXMgb25lIHRhcmdldCBraW5kOyBtdWx0aXBsZSBpbnB1dHMKcmVxdWlyZSBib3RoIGAtLWpzb25gIGFuZCBgLS1hc2AuIENoZWNrIGRlY29kZXMgZXZlcnkgdjUgb3BlcmFuZCBiZWZvcmUgUnVudGltZQphY2Nlc3MuIE9uZSBodW1hbiBpbnB1dCB3cml0ZXMgYEN1cnJlbnRgLCBgTm90Q3VycmVudGAsIG9yIGBVbmF2YWlsYWJsZWA7IGEKYmF0Y2ggcmVxdWlyZXMgYC0tanNvbmAgYW5kIHByZXNlcnZlcyBvbmUgb3V0Y29tZSBwZXIgaW5wdXQuIFNlYXJjaCwgVmlldywKQ2hlY2ssIGFuZCBFZGl0IGAtLWpzb25gCndyaXRlIGNvbXBhY3QgQWRhcHRlciBvYmplY3RzIHdpdGggZXhhY3QgZW1iZWRkZWQgdjUgQW5kZHJlc3Mgb2JqZWN0cyB3aGVyZQphcHBsaWNhYmxlOyBlYWNoIGlzIGFuIEFkYXB0ZXIgc2NoZW1hLCBub3QgYSBzZWNvbmQgQ29yZSB3aXJlLiBWaWV3IHVzZXMgdGhlCmhhcmQtY3V0IGBidy4=\",\"text\":\"single input\\ndefaults to self projection. `--as` chooses one target kind; multiple inputs\\nrequire both `--json` and `--as`. Check decodes every v5 operand before Runtime\\naccess. One human input writes `Current`, `NotCurrent`, or `Unavailable`; a\\nbatch requires `--json` and preserves one outcome per input. Search, View,\\nCheck, and Edit `--json`\\nwrite compact Adapter objects with exact embedded v5 Anddress objects where\\napplicable; each is an Adapter schema, not a second Core wire. View uses the\\nhard-cut `bw.\"},{\"sequence\":23,\"ns\":\"166630188100867\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"Y2xpLnZpZXcudjJgIG91dGNvbWVzIGFycmF5IGZvciBib3RoIHNpbmdsZSBhbmQgYmF0Y2ggcmVzdWx0czsKQ2hlY2sgdXNlcyB0aGUgaGFyZC1jdXQgYGJ3LmNsaS5jaGVjay52MmAgb3JkZXJlZCBvdXRjb21lcyBhcnJheS4KUmF3IFZpZXcgaXMgYW4gZXhwbGljaXQgQWRhcHRlciBleGFjdC10ZXh0IG1vZGUgdGhhdCByZXVzZXMgdGhlIG9yZGluYXJ5IFZpZXcKcHJvamVjdGlvbiB3aXRob3V0IGEgQ29yZSB3aXJlIG9yIGNoYW5nZWQgVmlldyBtZWFuaW5nLgpIdW1hbiBTZWFyY2gsIFZpZXcsIGFuZCBDaGVjayBrZWVwIHRoZWlyIGV4aXN0aW5nIHByb2plY3Rpb25zOyBodW1hbiBFZGl0CnJlY2VpcHRzIGludGVudGlvbmFsbHkgcmV0dXJuIHRoZSBleGFjdCBjdXJyZW50IHY1IG9iamVjdCB3aGVuIG9uZSBleGlzdHMuClRoZSBTZXNzaW9uIGhvbGRzIG9uZSBSdW50aW1lIHVudGlsIEVPRgpvciBgZXhpdGAgYW5kIGhhcyBleHBsaWNpdCBsb2NhbCBTZWFyY2gsIFBpY2ssIEFuZGRyZXNzLCBFZGk=\",\"text\":\"cli.view.v2` outcomes array for both single and batch results;\\nCheck uses the hard-cut `bw.cli.check.v2` ordered outcomes array.\\nRaw View is an explicit Adapter exact-text mode that reuses the ordinary View\\nprojection without a Core wire or changed View meaning.\\nHuman Search, View, and Check keep their existing projections; human Edit\\nreceipts intentionally return the exact current v5 object when one exists.\\nThe Session holds one Runtime until EOF\\nor `exit` and has explicit local Search, Pick, Anddress, Edi\"},{\"sequence\":24,\"ns\":\"166630188107445\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"dCwgVmlldywgYW5kIENoZWNrCmJpbmRpbmdzIHBsdXMgbm9uLWFsaWFzaW5nIEFuY2hlZHJlc3MgaGFuZGxlcy4gSXQgb3ducyBvbmUgZXhwbGljaXQgYERhdGFTdG9yZWAKZm9yIHRoZSBTZXNzaW9uIG9ubHk7IG5hbWVzIGFyZSB0eXBlZCBhbmQgbmV2ZXIgcGVyc2lzdCBwYXN0IEVPRiBvciBgZXhpdGAuClNlc3Npb24gUGljawpwYXNzZXMgYSBuYW1lZCBTZWFyY2ggb3IgUGljayBjb2xsZWN0aW9uIGFuZCBhbiBBZGFwdGVyLXBhcnNlZCBwcmVkaWNhdGUgZGlyZWN0bHkKdG8gQ29yZTsgU2Vzc2lvbiBiYXRjaCBDaGVjayBwYXNzZXMgYSBuYW1lZCBtYXRjaGluZyBvdXRjb21lIGRpcmVjdGx5IHRvIGl0cwpSdW50aW1lIGJhdGNoIHNlYW0gYW5kIHByaW50cyBvbmx5IHJlcG9ydCBjb3VudHMuIFNlc3Npb24gQW5jaG9yIGNyZWF0ZXMgYSBsaXZlCmhhbmRsZSBvbmx5IHRocm91Z2ggYGxldCA8bmFtZT4gPSBhbmNob3IgY3JlYXRlIDxhbmRkcmVzcy1yZWY+YCwgdmlld3MgaXQgdGhyb3VnaApgdmk=\",\"text\":\"t, View, and Check\\nbindings plus non-aliasing Anchedress handles. It owns one explicit `DataStore`\\nfor the Session only; names are typed and never persist past EOF or `exit`.\\nSession Pick\\npasses a named Search or Pick collection and an Adapter-parsed predicate directly\\nto Core; Session batch Check passes a named matching outcome directly to its\\nRuntime batch seam and prints only report counts. Session Anchor creates a live\\nhandle only through `let <name> = anchor create <anddress-ref>`, views it through\\n`vi\"}],\"more\":true,\"active\":[]}\n"}
{"id":7,"bytes":5246,"sha256":"44aeae284e1488150259bb6e7c31ddbe249417fa3a0787fb1dac2853ede18ff0","content":"{\"id\":7,\"events\":[{\"sequence\":25,\"ns\":\"166630188113925\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"ZXcgYW5jaG9yZWQgQDxuYW1lPmAsIGFuZCBjYW4gaW52YWxpZGF0ZSBpdHMgbG9naWNhbCBzb3VyY2Ugd2l0aCBgYW5jaG9yCmludmFsaWRhdGUtc291cmNlIDxsb2dpY2FsLXBhdGg+YC4gT25lLXNob3QgRGF0YSBhbmQgQW5jaG9yIGFyZSBpbnRlbnRpb25hbGx5CnVuc3VwcG9ydGVkIGJlY2F1c2UgdGhlaXIgRGF0YVN0b3JlIGFuZCBsaXZlLWhhbmRsZSBjb250cmFjdHMgcmVxdWlyZSBTZXNzaW9uCmxpZmV0aW1lLiBPbmUtc2hvdCBQaWNrLCByYXcgRWRpdC1vYmplY3QgdHJhbnNwb3J0LCBhbmQgYSBzZXBhcmF0ZQpBcHBseSB0cmFuc3BvcnQgYXdhaXQgY29sbGVjdGlvbiBvciBFZGl0IHRyYW5zcG9ydCBzY2hlbWEgYXV0aG9yaXR5LiBUaGUKZGlzdGluY3QgQW5kZHJlc3MtZmlyc3Qgb25lLXNob3QgRWRpdCBhYm92ZSBpcyBpbXBsZW1lbnRlZC4gUmF3IG91dHB1dCBvdGhlcgp0aGFuIG9uZS1zaG90IFZpZXcgYW5kIGZ1cnRoZXIgU2Vzc2lvbiBiZWhhdmlvciByZW1haW4gZGVmZXJyZWQuCgo=\",\"text\":\"ew anchored @<name>`, and can invalidate its logical source with `anchor\\ninvalidate-source <logical-path>`. One-shot Data and Anchor are intentionally\\nunsupported because their DataStore and live-handle contracts require Session\\nlifetime. One-shot Pick, raw Edit-object transport, and a separate\\nApply transport await collection or Edit transport schema authority. The\\ndistinct Anddress-first one-shot Edit above is implemented. Raw output other\\nthan one-shot View and further Session behavior remain deferred.\\n\\n\"},{\"sequence\":26,\"ns\":\"166630188119589\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"IyMgU2NvcGUKCkJhY2t3cml0ZXIgaXMgbm90IEdpdCwgYSBmaWxlIHdhdGNoZXIsIGRhZW1vbiwgcGVyc2lzdGVudCBpbmRleCwgb3IgZWRpdG9yIFVJLgpJdCByZWFkcyBhZG1pdHRlZCBjdXJyZW50IFdvcmtzcGFjZSBTb3VyY2UgdGhyb3VnaCBSdW50aW1lJ3Mgc2FmZSBuby1mb2xsb3cKYWNjZXNzLiBJdCBkb2VzIG5vdCBtb2RlbCBicmFuY2hlcywgbWVyZ2VzLCBoaXN0b3J5LCBhdXRvbWF0aWMgcmUtZXZhbHVhdGlvbiwKb3IgZWRpdG9yIGJ1ZmZlcnMuCgpBcHBseSB1c2VzIGl0cyBhY2NlcHRlZCBjdXJyZW50IG9ic2VydmF0aW9uIGFuZCBkb2VzIG5vdCBjb29yZGluYXRlIGNvbmN1cnJlbnQKd3JpdGVycy4gV3JpdGVycyBtYXkgcmFjZSBhbmQgb25lIHB1YmxpY2F0aW9uIG1heSBvdmVyd3JpdGUgYW5vdGhlcgpzb3VyY2UtdmlzaWJsZSBjaGFuZ2U7IGhvc3RzIHJlcXVpcmluZyBhIHN0cm9uZ2VyIGd1YXJhbnRlZSBjb29yZGluYXRlIG91dHNpZGUKQmFja3dyaXRlci4KCiMjIEJ1aWxkIGY=\",\"text\":\"## Scope\\n\\nBackwriter is not Git, a file watcher, daemon, persistent index, or editor UI.\\nIt reads admitted current Workspace Source through Runtime's safe no-follow\\naccess. It does not model branches, merges, history, automatic re-evaluation,\\nor editor buffers.\\n\\nApply uses its accepted current observation and does not coordinate concurrent\\nwriters. Writers may race and one publication may overwrite another\\nsource-visible change; hosts requiring a stronger guarantee coordinate outside\\nBackwriter.\\n\\n## Build f\"},{\"sequence\":27,\"ns\":\"166630188126723\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"cm9tIHNvdXJjZSBhbmQgdGVzdAoKYGBgc2gKY2FyZ28gYnVpbGQgLS1vZmZsaW5lIC0tbG9ja2VkIC0tcmVsZWFzZQouL3RhcmdldC9yZWxlYXNlL2J3IHNlYXJjaCBsaW5lICJuZWVkbGUiCi4vdGFyZ2V0L3JlbGVhc2UvYncgLS13b3Jrc3BhY2UgL3BhdGgvcHJvamVjdCBzZWFyY2ggcGFyYWdyYXBoICJuZWVkbGUiCmNhcmdvIHRlc3QgLS1vZmZsaW5lIC0tbG9ja2VkCmBgYAoKIyMgT2ZmaWNpYWwgZGVza3RvcCBkaXN0cmlidXRpb24KClRoZSBvZmZpY2lhbCBkaXN0cmlidXRpb24gYXV0aG9yaXR5IGlzCltodHRwczovL2JhY2t3cml0ZXIucGVudGFncmF0aW9uLmNvbV0oaHR0cHM6Ly9iYWNrd3JpdGVyLnBlbnRhZ3JhdGlvbi5jb20pLgpJdCBwdWJsaXNoZXMgdGhlIGNsb3NlZCBCYWNrd3JpdGVyIGAwLjIuNmAgcmVsZWFzZSBmb3IgTGludXgvV1NMIHg4Nl82NCwKbWFjT1MgYXJtNjQsIG1hY09TIHg4Nl82NCwgYW5kIFdpbmRvd3MgeDg2XzY0IGZyb20gU291cmNlIEF1dGhvcml0eSByZXZpc2k=\",\"text\":\"rom source and test\\n\\n```sh\\ncargo build --offline --locked --release\\n./target/release/bw search line \\\"needle\\\"\\n./target/release/bw --workspace /path/project search paragraph \\\"needle\\\"\\ncargo test --offline --locked\\n```\\n\\n## Official desktop distribution\\n\\nThe official distribution authority is\\n[https://backwriter.pentagration.com](https://backwriter.pentagration.com).\\nIt publishes the closed Backwriter `0.2.6` release for Linux/WSL x86_64,\\nmacOS arm64, macOS x86_64, and Windows x86_64 from Source Authority revisi\"},{\"sequence\":28,\"ns\":\"166630188134222\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"b24KYDA5YmI2YzQyNDA4MTU5NGJkODZhOTVmMDQzNDViNzg2ZWY5YjQ2YjZgLiBMaW51eCB1c2VzIGNhbm9uaWNhbCB0YXJnZXQKYHg4Nl82NC11bmtub3duLWxpbnV4LW11c2xgOwpgeDg2XzY0LXVua25vd24tbGludXgtZ251YCByZW1haW5zIHRoZSBsb2NhbCBkZXZlbG9wbWVudC90ZXN0LWhvc3QgdGFyZ2V0LgptYWNPUyB1c2VzIGBhYXJjaDY0LWFwcGxlLWRhcndpbmAgd2l0aCBtaW5pbXVtIG1hY09TIDExLjAgYW5kCmB4ODZfNjQtYXBwbGUtZGFyd2luYCB3aXRoIG1pbmltdW0gbWFjT1MgMTAuMTIuIFRoZSBtYWNPUyBhcnRpZmFjdHMgcmVjZWl2ZQpzdGF0aWMgY3Jvc3MtYnVpbGQgdmVyaWZpY2F0aW9uIGJ1dCBhcmUgbm90IGNsYWltZWQgdG8gaGF2ZSBiZWVuIGV4ZWN1dGVkIG9uIGEKbmF0aXZlIE1hYyBiZWZvcmUgcHVibGljYXRpb24uIFdpbmRvd3MgdXNlcyBgeDg2XzY0LXBjLXdpbmRvd3MtZ251YCBhbmQgdGhlCmNhbm9uaWNhbCBleGVjdXRhYmxlIGBidy5leGVgOyBpdHMgc3RhdGk=\",\"text\":\"on\\n`09bb6c424081594bd86a95f04345b786ef9b46b6`. Linux uses canonical target\\n`x86_64-unknown-linux-musl`;\\n`x86_64-unknown-linux-gnu` remains the local development/test-host target.\\nmacOS uses `aarch64-apple-darwin` with minimum macOS 11.0 and\\n`x86_64-apple-darwin` with minimum macOS 10.12. The macOS artifacts receive\\nstatic cross-build verification but are not claimed to have been executed on a\\nnative Mac before publication. Windows uses `x86_64-pc-windows-gnu` and the\\ncanonical executable `bw.exe`; its stati\"}],\"more\":true,\"active\":[]}\n"}
{"id":8,"bytes":5235,"sha256":"2b74619f5deb0103c6d9fa2775926ab6b56eaf9611bacaba5d8c1600d4b8b864","content":"{\"id\":8,\"events\":[{\"sequence\":29,\"ns\":\"166630188144500\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"YyBjcm9zcy1idWlsZCB2ZXJpZmljYXRpb24gZG9lcyBub3QKY2xhaW0gbmF0aXZlIFdpbmRvd3MsIFBvd2VyU2hlbGwsIG9yIENNRCBleGVjdXRpb24uIExpbnV4IGFybTY0IGlzIG5vdCBjdXJyZW50bHkKcHJvdmlkZWQsIGFuZCBubyB1bml2ZXJzYWwgaG9zdC1jb21wYXRpYmlsaXR5IGNsYWltIGlzIG1hZGUuCgpgaW5zdGFsbC5zaGAgcmVhZHMgdGhlIGNhbm9uaWNhbCBtYW5pZmVzdCwgdmVyaWZpZXMgdGhlIGRvd25sb2FkZWQgYXJ0aWZhY3QKYWdhaW5zdCB0aGUgbWFuaWZlc3QgU0hBLTI1NiwgYW5kIGluc3RhbGxzIHRoZSB2ZXJpZmllZCBiaW5hcnkgYXQKYCRIT01FLy5sb2NhbC9iaW4vYndgIHdpdGggYSBzYW1lLWRpcmVjdG9yeSByZW5hbWUuIENvbmN1cnJlbnQgc2FtZS11c2VyCkhPTUUgbXV0YXRpb24gaXMgY2FsbGVyLW93bmVkLiBUaGUgcHVibGlzaGVkIGAuc2hhMjU2YCBzaWRlY2FyIGlzIGZvciBtYW51YWwKdmVyaWZpY2F0aW9uIGFuZCBpcyBub3QgaW5zdGFsbGVyIGF1dGhvcmk=\",\"text\":\"c cross-build verification does not\\nclaim native Windows, PowerShell, or CMD execution. Linux arm64 is not currently\\nprovided, and no universal host-compatibility claim is made.\\n\\n`install.sh` reads the canonical manifest, verifies the downloaded artifact\\nagainst the manifest SHA-256, and installs the verified binary at\\n`$HOME/.local/bin/bw` with a same-directory rename. Concurrent same-user\\nHOME mutation is caller-owned. The published `.sha256` sidecar is for manual\\nverification and is not installer authori\"},{\"sequence\":30,\"ns\":\"166630188167972\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"dHkuIFdpbmRvd3MgUG93ZXJTaGVsbCB2ZXJpZmllcyB0aGUKc2FtZSBtYW5pZmVzdCBhdXRob3JpdHkgYW5kIGV4YWN0IFpJUCwgYW5kIGluc3RhbGxzIHRvCmAkSE9NRVwubG9jYWxcYmluXGJ3LmV4ZWAgd2l0aG91dCBlZGl0aW5nIFBBVEggb3IgdGhlIFBvd2VyU2hlbGwgcHJvZmlsZS4gVGhlCnRocmVlIGNhbm9uaWNhbCBpbnN0YWxsIGNvbW1hbmRzIGFuZCB0aGUgY2Fub25pY2FsIGBidyB1cGRhdGVgIGNvbW1hbmQgYXJlIGtlcHQKdG9nZXRoZXIgaW4gW1F1aWNrIHN0YXJ0XSgjcXVpY2stc3RhcnQpLgoKVGhlIENNRCBjb21tYW5kIHdyaXRlcyBgaW5zdGFsbC5jbWRgIGluIHRoZSBjdXJyZW50IGRpcmVjdG9yeSBhbmQgcmVtb3ZlcyBpdAphZnRlciBhIHN1Y2Nlc3NmdWwgaW5zdGFsbGF0aW9uLiBBbiBleGlzdGluZyBmaWxlIHdpdGggdGhhdCBuYW1lIGlzIHJlcGxhY2VkLgoKVGhlIENSTEYgYGluc3RhbGwuY21kYCBjaGVja3MgYGN1cmwuZXhlYCBhbmQgYHBvd2Vyc2hlbGwuZXhlYCwgZG8=\",\"text\":\"ty. Windows PowerShell verifies the\\nsame manifest authority and exact ZIP, and installs to\\n`$HOME\\\\.local\\\\bin\\\\bw.exe` without editing PATH or the PowerShell profile. The\\nthree canonical install commands and the canonical `bw update` command are kept\\ntogether in [Quick start](#quick-start).\\n\\nThe CMD command writes `install.cmd` in the current directory and removes it\\nafter a successful installation. An existing file with that name is replaced.\\n\\nThe CRLF `install.cmd` checks `curl.exe` and `powershell.exe`, do\"},{\"sequence\":31,\"ns\":\"166630188175616\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"d25sb2FkcyBleGFjdGx5CnRoZSBjYW5vbmljYWwgYGluc3RhbGwucHMxYCBvdmVyIEhUVFBTLW9ubHkgVExTIDEuMi1vci1uZXdlciB0cmFuc3BvcnQgaW50byBhCmNvbGxpc2lvbi1mYWlsaW5nIGAlVEVNUCVgIHRhc2sgZGlyZWN0b3J5LCBkZWxlZ2F0ZXMgYWxsIGluc3RhbGxhdGlvbiBtZWFuaW5nLApjbGVhbnMgdGhlIGRpcmVjdG9yeSwgYW5kIHByZXNlcnZlcyB0aGUgY2hpbGQgZXhpdCBjb2RlLiBJdCBvd25zIG5vIG1hbmlmZXN0LApTSEEtMjU2LCBaSVAsIG9yIGluc3RhbGxhdGlvbiBsb2dpYy4gVGhlIExpbnV4LWhvc3RlZCBDTUQgcmVncmVzc2lvbiBpcyBzdGF0aWM7Cm5vIG5hdGl2ZSBDTUQgZXhlY3V0aW9uIGlzIGNsYWltZWQuClRoZSBkaXN0cmlidXRpb24gcHJvdmlkZXMgbm8KcHVibGlzaGVyLWF1dGhlbnRpY2l0eSBzaWduYXR1cmUgb3IgdHJ1c3RlZCBzaWduaW5nIGlkZW50aXR5LCBiYWNrZ3JvdW5kIG9yCmF1dG9tYXRpYyB1cGRhdGUsIHRlbGVtZXRyeSwgYHN1ZG9gIGV4ZWM=\",\"text\":\"wnloads exactly\\nthe canonical `install.ps1` over HTTPS-only TLS 1.2-or-newer transport into a\\ncollision-failing `%TEMP%` task directory, delegates all installation meaning,\\ncleans the directory, and preserves the child exit code. It owns no manifest,\\nSHA-256, ZIP, or installation logic. The Linux-hosted CMD regression is static;\\nno native CMD execution is claimed.\\nThe distribution provides no\\npublisher-authenticity signature or trusted signing identity, background or\\nautomatic update, telemetry, `sudo` exec\"},{\"sequence\":32,\"ns\":\"166630188181700\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"dXRpb24sIG9yIGF1dG9tYXRpYyBgUEFUSGAgb3IKc2hlbGwtc3RhcnR1cC1maWxlIGNoYW5nZS4KR2l0SHViIGlzIGEgcHVibGljIHNvdXJjZSBhbmQgZG9jdW1lbnRhdGlvbiBtaXJyb3IsIG5vdCB0aGUgZGlzdHJpYnV0aW9uCmF1dGhvcml0eS4gVGhlIGNvbXBsZXRlIGJldGEuMSwgYmV0YS4yLCBiZXRhLjMsIHN0YWJsZSBgMC4xLjBgLCBgMC4yLjBgLCBhbmQKYDAuMi4xYCBhbmQgYDAuMi4yYCB2ZXJzaW9uIGRpcmVjdG9yaWVzIHJlbWFpbiB1bmNoYW5nZWQgYW5kIGltbXV0YWJsZS4gVGhlCmNvbXBsZXRlIGAwLjIuM2AsIGAwLjIuNGAsIGAwLjIuNWAsIGFuZCBgMC4yLjZgIHZlcnNpb24gZGlyZWN0b3JpZXMgYXJlIGltbXV0YWJsZTsKdGhlIGAwLjIuNmAgcmVsZWFzZSBpcyBjbG9zZWQuIFRoZSBhY3RpdmUgcHVibGljIHRyZWUgaGFzIGV4YWN0bHkgNzYgZmlsZXMsCmFuZCB0aGUgY3VycmVudCBpbnN0YWxsZXJzIGFuZCBtYW5pZmVzdCBzZWxlY3QgYDAuMi42YC4gQW55IGxhdGVyIHBsYXQ=\",\"text\":\"ution, or automatic `PATH` or\\nshell-startup-file change.\\nGitHub is a public source and documentation mirror, not the distribution\\nauthority. The complete beta.1, beta.2, beta.3, stable `0.1.0`, `0.2.0`, and\\n`0.2.1` and `0.2.2` version directories remain unchanged and immutable. The\\ncomplete `0.2.3`, `0.2.4`, `0.2.5`, and `0.2.6` version directories are immutable;\\nthe `0.2.6` release is closed. The active public tree has exactly 76 files,\\nand the current installers and manifest select `0.2.6`. Any later plat\"}],\"more\":true,\"active\":[]}\n"}
{"id":9,"bytes":3011,"sha256":"79da1bf96ed214538bd389f2a6434dd161b46a32b291cccfe886477fe3edd603","content":"{\"id\":9,\"events\":[{\"sequence\":33,\"ns\":\"166630188190193\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"Zm9ybSBvcgp2ZXJzaW9uIHJlcXVpcmVzIHNlcGFyYXRlIE93bmVyIGF1dGhvcml0eS4gTGludXggYXJtNjQsIHRhZ3MsIEdpdEh1YiBSZWxlYXNlcywKY3JhdGVzLmlvIHB1YmxpY2F0aW9uLCBhbmQgYmFja2dyb3VuZCBvciBhdXRvbWF0aWMgdXBkYXRlIHJlbWFpbiBvdXRzaWRlIHRoZQpjb21wbGV0ZWQgcHVibGljYXRpb24uIFRoZSBlYXJsaWVyIGAwLjIuNGAgcHVibGljYXRpb24gYWRkZWQgaXRzIGVpZ2h0CnZlcnNpb25lZCBmaWxlcyBhbmQgcmVwbGFjZWQgb25seSB0aGUgdHdvIGluc3RhbGxlcnMgYW5kIG1hbmlmZXN0IHBvaW50ZXJzCndpdGhvdXQgcmVwbGFjaW5nIGFueSBvZiB0aGUgNDggcHJpb3IgdmVyc2lvbmVkIGZpbGVzIG9yIHRoZSBDTUQgQWRhcHRlci4gSXRzCnNlY29uZCBwdWJsaXNoZXIgcnVuIHJldXNlZCBhbGwgNjAgZmlsZXMgd2l0aG91dCBtZXRhZGF0YSBjaGFuZ2UuIFRoZSBgMC4yLjVgCnB1YmxpY2F0aW9uIHRoZW4gYWRkZWQgaXRzIGVpZ2h0IHZlcnNpb25lZCBmaWw=\",\"text\":\"form or\\nversion requires separate Owner authority. Linux arm64, tags, GitHub Releases,\\ncrates.io publication, and background or automatic update remain outside the\\ncompleted publication. The earlier `0.2.4` publication added its eight\\nversioned files and replaced only the two installers and manifest pointers\\nwithout replacing any of the 48 prior versioned files or the CMD Adapter. Its\\nsecond publisher run reused all 60 files without metadata change. The `0.2.5`\\npublication then added its eight versioned fil\"},{\"sequence\":34,\"ns\":\"166630188196286\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"ZXMsIHJlcGxhY2VkIHRoZSB0d28gaW5zdGFsbGVycwphbmQgbWFuaWZlc3QgcG9pbnRlcnMsIGFuZCByZXVzZWQgYWxsIDY4IGZpbGVzIG9uIGl0cyBzZWNvbmQgcnVuLiBSMyBhZGRzIHRoZQplaWdodCBgMC4yLjZgIGZpbGVzIGFuZCByZXBsYWNlcyB0aGUgdHdvIGluc3RhbGxlcnMgYW5kIG1hbmlmZXN0IGxhc3Q7IGl0cwpzZWNvbmQgcnVuIHJldXNlcyBhbGwgNzYgZmlsZXMgd2l0aG91dCBieXRlIG9yIG1ldGFkYXRhIGNoYW5nZS4KCiMjIEFyY2hpdGVjdHVyZQoKLSBbQ3VycmVudCBzdGF0ZV0oZG9jcy9jdXJyZW50L25vdy5tZCkKLSBbQmFja3dyaXRlciBwcm90b2NvbF0oZG9jcy9hcmNoaXRlY3R1cmUvYmFja3dyaXRlci10ZXh0LWNvb3JkaW5hdGlvbi1wcm90b2NvbC5tZCkKLSBbQW5kZHJlc3MgYW5kIGV4YWN0IExpbmUgbW9kZWxdKGRvY3MvYXJjaGl0ZWN0dXJlL3JlYnVpbGRhYmxlLXN0cnVjdHVyYWwtYWRkcmVzc2luZy5tZCkKLSBbQ0xJIFYxIGF1dGhvcml0eV0oZG9jcy9hcmNoaXQ=\",\"text\":\"es, replaced the two installers\\nand manifest pointers, and reused all 68 files on its second run. R3 adds the\\neight `0.2.6` files and replaces the two installers and manifest last; its\\nsecond run reuses all 76 files without byte or metadata change.\\n\\n## Architecture\\n\\n- [Current state](docs/current/now.md)\\n- [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)\\n- [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)\\n- [CLI V1 authority](docs/archit\"},{\"sequence\":35,\"ns\":\"166630188203516\",\"kind\":\"guide\",\"processId\":null,\"base64\":\"ZWN0dXJlL2JhY2t3cml0ZXItY2xpLXYxLm1kKQotIFtWZXJpZmljYXRpb25dKGRvY3MvZGV2ZWxvcG1lbnQvdmVyaWZpY2F0aW9uLm1kKQoKIyMgTGljZW5zZQoKW01JVCBMaWNlbnNlXShMSUNFTlNFKQo=\",\"text\":\"ecture/backwriter-cli-v1.md)\\n- [Verification](docs/development/verification.md)\\n\\n## License\\n\\n[MIT License](LICENSE)\\n\"}],\"more\":false,\"active\":[]}\n"}
{"id":10,"bytes":5605,"sha256":"d6d82b6b2795647f4b7811b4803ea14c1864753d3e2f8e724b2163abef55a885","content":"{\"id\":10,\"events\":[{\"sequence\":36,\"ns\":\"166745584717025\",\"kind\":\"start\",\"processId\":1,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw\",\"help\",\"shell\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh/genie-new/workspace\"},{\"sequence\":37,\"ns\":\"166745585787165\",\"kind\":\"stdin-eof\",\"processId\":1},{\"sequence\":38,\"ns\":\"166745585841608\",\"kind\":\"spawn\",\"processId\":1,\"pid\":2729447},{\"sequence\":39,\"ns\":\"166745586139412\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"TkFNRQogIGJ3IHNoZWxsIC0gcmV1c2Ugc2hvcnQgcmVmZXJlbmNlcyBhY3Jvc3Mgc2VhcmNoLCB2aWV3LCByZXBsYWNlLCBhbmQgY2hlY2sKClVTQUdFCiAgYncgWy0td29ya3NwYWNlIEFCU09MVVRFX1BBVEhdIFstLWFkbWl0IExPR0lDQUxfUEFUSF0uLi4gc2hlbGwKCkNPTU1BTkRTCiAgc2VhcmNoIDxsaW5lfHBhcmFncmFwaHxmaWxlPiA8cXVlcnk+IFstLXNvdXJjZSBQQVRIIHwgLS1zdWJ0cmVlIFBBVEhdLi4uCiAgc2VhcmNoIC9maWxlIDxsb2dpY2FsLXBhdGg+CiAgdmlldyA8UkVGPi4uLiBbLS1hcyA8bGluZXxwYXJhZ3JhcGh8ZmlsZT5dCiAgcmVwbGFjZSA8UkVGPiA8Q09OVEVOVD4KICBjaGVjayA8UkVGPi4uLgogIGxldCA8TkFNRT4gPSA8UkVGPgogIGV4aXQKClJFRkVSRU5DRVMgQU5EIENPTlRFTlQKICBATiBpcyBhIHplcm8tYmFzZWQsIGFwcGVuZC1vbmx5IHJlZmVyZW5jZSBmcm9tIHRoaXMgcHJvY2Vzcy4gSXQgZW5kcyBhdCBFT0Ygb3IgZXhpdC4KICBAbmFtZSBpcyBhIG5hbWU=\",\"text\":\"NAME\\n  bw shell - reuse short references across search, view, replace, and check\\n\\nUSAGE\\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\\n\\nCOMMANDS\\n  search <line|paragraph|file> <query> [--source PATH | --subtree PATH]...\\n  search /file <logical-path>\\n  view <REF>... [--as <line|paragraph|file>]\\n  replace <REF> <CONTENT>\\n  check <REF>...\\n  let <NAME> = <REF>\\n  exit\\n\\nREFERENCES AND CONTENT\\n  @N is a zero-based, append-only reference from this process. It ends at EOF or exit.\\n  @name is a name\"},{\"sequence\":40,\"ns\":\"166745586163080\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"ZCBBbmRkcmVzczsgQGhpdHNbMF0gc2VsZWN0cyBhbiBpbmRleGVkIFNlYXJjaC9QaWNrIGJpbmRpbmcuCiAgbGV0IG5hbWUgPSBATiBtYWtlcyBhIG5hbWVkIGFsaWFzLiBOYW1lcyBjYW5ub3QgYmUgcmVkZWZpbmVkLgogIERvdWJsZS1xdW90ZSBhcmd1bWVudHMgY29udGFpbmluZyBzcGFjZXMuIFF1b3RlZCBlc2NhcGVzIGFyZSBcXCwgXCIsIFxuLCBcciwgYW5kIFx0LgogIE9uZSBwaHlzaWNhbCBpbnB1dCBsaW5lIGlzIG9uZSBjb21tYW5kOyBubyBzaW5nbGUgcXVvdGVzLCBwaXBlcywgb3IgRU9GIENvbnRlbnQgc2VsZWN0b3IuCiAgTGluZSBSZXBsYWNlIGFjY2VwdHMgYm9keSBvbmx5OiBOVUwvQ1IvTEYgYXJlIHJlamVjdGVkOyB0aGUgZXhpc3RpbmcgdGVybWluYXRvciBpcyBwcmVzZXJ2ZWQuCiAgRmlsZS9QYXJhZ3JhcGggUmVwbGFjZSB1c2VzIGV4YWN0IENvbnRlbnQuIENoYW5nZWQgc291cmNlIGJ5dGVzIHN0YWxlIG9sZCBzYW1lLXNvdXJjZSByZWZzLAogIGluY2x1ZGluZyBvdGhlciA=\",\"text\":\"d Anddress; @hits[0] selects an indexed Search/Pick binding.\\n  let name = @N makes a named alias. Names cannot be redefined.\\n  Double-quote arguments containing spaces. Quoted escapes are \\\\\\\\, \\\\\\\", \\\\n, \\\\r, and \\\\t.\\n  One physical input line is one command; no single quotes, pipes, or EOF Content selector.\\n  Line Replace accepts body only: NUL/CR/LF are rejected; the existing terminator is preserved.\\n  File/Paragraph Replace uses exact Content. Changed source bytes stale old same-source refs,\\n  including other \"},{\"sequence\":41,\"ns\":\"166745586197454\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"bG9jYXRpb25zLiBVc2UgdGhlIGZyZXNoIHJlY2VpcHQgb3IgZXhwbGljaXRseSBzZWFyY2ggYWdhaW47IG5ldmVyIGF1dG8tcmV0cnkuCgpPVVRQVVQKICBTZWFyY2ggYXBwZW5kcyBvbmUgQE4gcGVyIHJlc3VsdDsgRW1wdHkgd3JpdGVzIG5vdGhpbmcuCiAgVmlldyByZXR1cm5zIGVhY2ggaW5wdXQgcmVmLCBhIGZyZXNoIHJlZiwga2luZC9sb2NhdGlvbiwgYW5kIGNvbXBsZXRlIENvbnRlbnQgaW4gaW5wdXQgb3JkZXIuCiAgVmlldzxUQUI+UkVGPFRBQj5ieXRlcz1OIHN0YXJ0cyBhIHJlY29yZCwgZm9sbG93ZWQgYnkgZnJlc2gtcmVmIG1ldGFkYXRhIGFuZCBOIGV4YWN0IGJ5dGVzLgogIFRoZSBmb2xsb3dpbmcgTEYgYW5kIEVuZFZpZXcgbGluZSBhcmUgZGlzcGxheSBmcmFtaW5nLCBub3Qgc291cmNlIENvbnRlbnQuCiAgUmVsYXRpb25BYnNlbnQgaXMgc2hvd24gYXQgaXRzIGlucHV0IHBvc2l0aW9uIHdpdGhvdXQgQ29udGVudCBvciBhIGZyZXNoIHNsb3QuCiAgUmVwbGFjZSB3cml0ZXMgVW4=\",\"text\":\"locations. Use the fresh receipt or explicitly search again; never auto-retry.\\n\\nOUTPUT\\n  Search appends one @N per result; Empty writes nothing.\\n  View returns each input ref, a fresh ref, kind/location, and complete Content in input order.\\n  View<TAB>REF<TAB>bytes=N starts a record, followed by fresh-ref metadata and N exact bytes.\\n  The following LF and EndView line are display framing, not source Content.\\n  RelationAbsent is shown at its input position without Content or a fresh slot.\\n  Replace writes Un\"},{\"sequence\":42,\"ns\":\"166745586206117\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"Y2hhbmdlZC9DaGFuZ2VkIHdpdGggYSBmcmVzaCByZWYsIG9yIENoYW5nZWQ8VEFCPk5vbmUgd2l0aG91dCBvbmUuCiAgQ2hlY2sgd3JpdGVzIG9uZSBzdGF0dXMgcGVyIGlucHV0OyBvbmx5IEN1cnJlbnQgYXBwZW5kcyBhIGZyZXNoIHJlZi4gQ2hlY2sgaXMgb3B0aW9uYWwuCgpFWEFNUExFUwogIFN0YXJ0IGJ3IHNoZWxsIGluIGEgd29ya3NwYWNlIHdpdGggb25seSBub3RlLnR4dCBjb250YWluaW5nIG5lZWRsZSBwbHVzIENSTEYsIHRoZW4gZW50ZXI6CiAgc2VhcmNoIGxpbmUgbmVlZGxlIC0tc291cmNlIG5vdGUudHh0CiAgdmlldyBAMCAtLWFzIHBhcmFncmFwaAogIGxldCBzZWxlY3RlZCA9IEAwCiAgcmVwbGFjZSBAc2VsZWN0ZWQgIm5ldyB2YWx1ZSIKICBjaGVjayBAMCBAMgogIHZpZXcgQDMKICBleGl0CgpBRFZBTkNFRAogIGxldCBoaXRzID0gc2VhcmNoIGxpbmUgbmVlZGxlCiAgdmlldyBhbmRkcmVzcyBAaGl0c1swXQogIFJhdyBuYW1lZCBWaWV3IHdyaXRlcyBleGFjdCBDb250ZW50IHc=\",\"text\":\"changed/Changed with a fresh ref, or Changed<TAB>None without one.\\n  Check writes one status per input; only Current appends a fresh ref. Check is optional.\\n\\nEXAMPLES\\n  Start bw shell in a workspace with only note.txt containing needle plus CRLF, then enter:\\n  search line needle --source note.txt\\n  view @0 --as paragraph\\n  let selected = @0\\n  replace @selected \\\"new value\\\"\\n  check @0 @2\\n  view @3\\n  exit\\n\\nADVANCED\\n  let hits = search line needle\\n  view anddress @hits[0]\\n  Raw named View writes exact Content w\"}],\"more\":true,\"active\":[]}\n"}
{"id":11,"bytes":1342,"sha256":"6680d90aecfe1847242ecc0c7ba188934a1da6ea39d6bd71f723e782c0dda63a","content":"{\"id\":11,\"events\":[{\"sequence\":43,\"ns\":\"166745586217277\",\"kind\":\"stdout\",\"processId\":1,\"base64\":\"aXRob3V0IGRpcmVjdC1WaWV3IGZyYW1pbmcuCiAgU2VlIGJ3IGhlbHAgcGljaywgYncgaGVscCBhbmNob3IsIGJ3IGhlbHAgYXBwbHksIGFuZCBidyBoZWxwIGRhdGEuCgpGQUlMVVJFUwogIFVzYWdlIGVycm9ycyByZWNvcmQgZXhpdCAyOyBSdW50aW1lL3NvdXJjZSBlcnJvcnMgcmVjb3JkIGV4aXQgMTsgbGF0ZXIgY29tbWFuZHMgc3RpbGwgcnVuLgogIFN0ZGluL3N0ZG91dCBmYWlsdXJlIGVuZHMgdGhlIHNoZWxsIHdpdGggZXhpdCAxLiBQYXJ0aWFsIG91dHB1dCBjYW5ub3QgcHJvdmUgZGVsaXZlcnkgb3IKICB1bmRvIHB1YmxpY2F0aW9uLiAtLWpzb24vLS1yYXcgYW5kIGNvbW1hbmQtbGluZSBvcGVyYW5kcyBhcmUgdW5hdmFpbGFibGUuCgpTRUUgQUxTTwogIGJ3IGhlbHAgc2VhcmNoCiAgYncgaGVscCBlZGl0Cg==\",\"text\":\"ithout direct-View framing.\\n  See bw help pick, bw help anchor, bw help apply, and bw help data.\\n\\nFAILURES\\n  Usage errors record exit 2; Runtime/source errors record exit 1; later commands still run.\\n  Stdin/stdout failure ends the shell with exit 1. Partial output cannot prove delivery or\\n  undo publication. --json/--raw and command-line operands are unavailable.\\n\\nSEE ALSO\\n  bw help search\\n  bw help edit\\n\"},{\"sequence\":44,\"ns\":\"166745586256655\",\"kind\":\"stdout-eof\",\"processId\":1},{\"sequence\":45,\"ns\":\"166745586291591\",\"kind\":\"stderr-eof\",\"processId\":1},{\"sequence\":46,\"ns\":\"166745586327880\",\"kind\":\"close\",\"processId\":1,\"code\":0,\"signal\":null}],\"more\":false,\"active\":[]}\n"}
{"id":12,"bytes":324,"sha256":"4385ba3121226421b48463f23ce8cf167575b1b885e36fca18d40d3e04e9941b","content":"{\"id\":12,\"events\":[{\"sequence\":47,\"ns\":\"166771478373617\",\"kind\":\"start\",\"processId\":2,\"argv\":[\"/tmp/backwriter-gate5r.kUMPDh/n-target/release/bw\",\"shell\"],\"cwd\":\"/tmp/backwriter-gate5r.kUMPDh/genie-new/workspace\"},{\"sequence\":48,\"ns\":\"166771479442427\",\"kind\":\"spawn\",\"processId\":2,\"pid\":2729910}],\"more\":false,\"active\":[2]}\n"}
{"id":13,"bytes":742,"sha256":"b30128efacbeab49ce92e89aaa4bac2e030b30bc51e3da9b4555d729ffdfcb56","content":"{\"id\":13,\"events\":[{\"sequence\":49,\"ns\":\"166785338359676\",\"kind\":\"stdin\",\"processId\":2,\"base64\":\"c2VhcmNoIGxpbmUgInJldHJ5X2J1ZGdldCA9IDMiCg==\",\"text\":\"search line \\\"retry_budget = 3\\\"\\n\"},{\"sequence\":50,\"ns\":\"166785338541237\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDAJTGluZQl1bml0LTAxLnR4dDoyCkAxCUxpbmUJdW5pdC0wMS50eHQ6NQpAMglMaW5lCXVuaXQtMDIudHh0OjIKQDMJTGluZQl1bml0LTAyLnR4dDo1CkA0CUxpbmUJdW5pdC0wMy50eHQ6MgpANQlMaW5lCXVuaXQtMDMudHh0OjUKQDYJTGluZQl1bml0LTA0LnR4dDoyCkA3CUxpbmUJdW5pdC0wNC50eHQ6NQo=\",\"text\":\"@0\\tLine\\tunit-01.txt:2\\n@1\\tLine\\tunit-01.txt:5\\n@2\\tLine\\tunit-02.txt:2\\n@3\\tLine\\tunit-02.txt:5\\n@4\\tLine\\tunit-03.txt:2\\n@5\\tLine\\tunit-03.txt:5\\n@6\\tLine\\tunit-04.txt:2\\n@7\\tLine\\tunit-04.txt:5\\n\"}],\"more\":false,\"active\":[2]}\n"}
{"id":14,"bytes":2172,"sha256":"25e7194790a2b9f0cc7c245af8824ac59aabb5d14ce6e6918ce901254163047a","content":"{\"id\":14,\"events\":[{\"sequence\":51,\"ns\":\"166800373354012\",\"kind\":\"stdin\",\"processId\":2,\"base64\":\"dmlldyBAMCBAMSBAMiBAMyBANCBANSBANiBANyAtLWFzIHBhcmFncmFwaAo=\",\"text\":\"view @0 @1 @2 @3 @4 @5 @6 @7 --as paragraph\\n\"},{\"sequence\":52,\"ns\":\"166800373549754\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"VmlldwlAMAlieXRlcz0zMgpAOAlQYXJhZ3JhcGgJdW5pdC0wMS50eHQ6MS0yCnJvbGUgPSBwcmltYXJ5CnJldHJ5X2J1ZGdldCA9IDMKCkVuZFZpZXcKVmlldwlAMQlieXRlcz0zNApAOQlQYXJhZ3JhcGgJdW5pdC0wMS50eHQ6NC01CnJvbGUgPSBzZWNvbmRhcnkKcmV0cnlfYnVkZ2V0ID0gMwoKRW5kVmlldwpWaWV3CUAyCWJ5dGVzPTM0CkAxMAlQYXJhZ3JhcGgJdW5pdC0wMi50eHQ6MS0yCnJvbGUgPSBzZWNvbmRhcnkNcmV0cnlfYnVkZ2V0ID0gMw0KRW5kVmlldwpWaWV3CUAzCWJ5dGVzPTMyCkAxMQlQYXJhZ3JhcGgJdW5pdC0wMi50eHQ6NC01CnJvbGUgPSBwcmltYXJ5DXJldHJ5X2J1ZGdldCA9IDMNCkVuZFZpZXcKVmlldwlANAlieXRlcz0zNApAMTIJUGFyYWdyYXBoCXVuaXQtMDMudHh0OjEtMgpyb2xlID0gcHJpbWFyeQ0KcmV0cnlfYnVkZ2V0ID0gMw0KCkVuZFZpZXcKVmlldwlANQlieXRlcz0zNgpAMTMJUGFyYWdyYXBoCXVuaXQtMDMudHh0OjQtNQpyb2xlID0gc2Vjb25kYXJ5DQpyZXQ=\",\"text\":\"View\\t@0\\tbytes=32\\n@8\\tParagraph\\tunit-01.txt:1-2\\nrole = primary\\nretry_budget = 3\\n\\nEndView\\nView\\t@1\\tbytes=34\\n@9\\tParagraph\\tunit-01.txt:4-5\\nrole = secondary\\nretry_budget = 3\\n\\nEndView\\nView\\t@2\\tbytes=34\\n@10\\tParagraph\\tunit-02.txt:1-2\\nrole = secondary\\rretry_budget = 3\\r\\nEndView\\nView\\t@3\\tbytes=32\\n@11\\tParagraph\\tunit-02.txt:4-5\\nrole = primary\\rretry_budget = 3\\r\\nEndView\\nView\\t@4\\tbytes=34\\n@12\\tParagraph\\tunit-03.txt:1-2\\nrole = primary\\r\\nretry_budget = 3\\r\\n\\nEndView\\nView\\t@5\\tbytes=36\\n@13\\tParagraph\\tunit-03.txt:4-5\\nrole = secondary\\r\\nret\"},{\"sequence\":53,\"ns\":\"166800373583675\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"cnlfYnVkZ2V0ID0gMw0KCkVuZFZpZXcKVmlldwlANglieXRlcz0zNApAMTQJUGFyYWdyYXBoCXVuaXQtMDQudHh0OjEtMgpyb2xlID0gc2Vjb25kYXJ5CnJldHJ5X2J1ZGdldCA9IDMKCkVuZFZpZXcKVmlldwlANwlieXRlcz0zMQpAMTUJUGFyYWdyYXBoCXVuaXQtMDQudHh0OjQtNQpyb2xlID0gcHJpbWFyeQpyZXRyeV9idWRnZXQgPSAzCkVuZFZpZXcK\",\"text\":\"ry_budget = 3\\r\\n\\nEndView\\nView\\t@6\\tbytes=34\\n@14\\tParagraph\\tunit-04.txt:1-2\\nrole = secondary\\nretry_budget = 3\\n\\nEndView\\nView\\t@7\\tbytes=31\\n@15\\tParagraph\\tunit-04.txt:4-5\\nrole = primary\\nretry_budget = 3\\nEndView\\n\"}],\"more\":false,\"active\":[2]}\n"}
{"id":15,"bytes":1014,"sha256":"5cdbf527be65fb048c7fc97feaebe9ad57f3e5eab34142d121dfeb007a5519a7","content":"{\"id\":15,\"events\":[{\"sequence\":54,\"ns\":\"166815717537333\",\"kind\":\"stdin\",\"processId\":2,\"base64\":\"cmVwbGFjZSBAMCAicmV0cnlfYnVkZ2V0ID0gNSIKcmVwbGFjZSBAMyAicmV0cnlfYnVkZ2V0ID0gNSIKcmVwbGFjZSBANCAicmV0cnlfYnVkZ2V0ID0gNSIKcmVwbGFjZSBANyAicmV0cnlfYnVkZ2V0ID0gNSIK\",\"text\":\"replace @0 \\\"retry_budget = 5\\\"\\nreplace @3 \\\"retry_budget = 5\\\"\\nreplace @4 \\\"retry_budget = 5\\\"\\nreplace @7 \\\"retry_budget = 5\\\"\\n\"},{\"sequence\":55,\"ns\":\"166815717677135\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDE2CUNoYW5nZWQJTGluZQl1bml0LTAxLnR4dDoyCg==\",\"text\":\"@16\\tChanged\\tLine\\tunit-01.txt:2\\n\"},{\"sequence\":56,\"ns\":\"166815717713165\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDE3CUNoYW5nZWQJTGluZQl1bml0LTAyLnR4dDo1CkAxOAlDaGFuZ2VkCUxpbmUJdW5pdC0wMy50eHQ6Mgo=\",\"text\":\"@17\\tChanged\\tLine\\tunit-02.txt:5\\n@18\\tChanged\\tLine\\tunit-03.txt:2\\n\"},{\"sequence\":57,\"ns\":\"166815717725059\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDE5CUNoYW5nZWQJTGluZQl1bml0LTA0LnR4dDo1Cg==\",\"text\":\"@19\\tChanged\\tLine\\tunit-04.txt:5\\n\"}],\"more\":false,\"active\":[2]}\n"}
{"id":16,"bytes":1894,"sha256":"c02e82cba2e75d42da20c7e66fb5044c3a8ae774a34eb5acd226647e5a9ecac9","content":"{\"id\":16,\"events\":[{\"sequence\":58,\"ns\":\"166830242886528\",\"kind\":\"stdin\",\"processId\":2,\"base64\":\"Y2hlY2sgQDE2IEAxNyBAMTggQDE5CnZpZXcgQDE2IEAxNyBAMTggQDE5IC0tYXMgZmlsZQo=\",\"text\":\"check @16 @17 @18 @19\\nview @16 @17 @18 @19 --as file\\n\"},{\"sequence\":59,\"ns\":\"166830242965346\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"QDIwCUN1cnJlbnQJTGluZQl1bml0LTAxLnR4dDoyCkAyMQlDdXJyZW50CUxpbmUJdW5pdC0wMi50eHQ6NQpAMjIJQ3VycmVudAlMaW5lCXVuaXQtMDMudHh0OjIKQDIzCUN1cnJlbnQJTGluZQl1bml0LTA0LnR4dDo1Cg==\",\"text\":\"@20\\tCurrent\\tLine\\tunit-01.txt:2\\n@21\\tCurrent\\tLine\\tunit-02.txt:5\\n@22\\tCurrent\\tLine\\tunit-03.txt:2\\n@23\\tCurrent\\tLine\\tunit-04.txt:5\\n\"},{\"sequence\":60,\"ns\":\"166830242985911\",\"kind\":\"stdout\",\"processId\":2,\"base64\":\"VmlldwlAMTYJYnl0ZXM9NjcKQDI0CUZpbGUJdW5pdC0wMS50eHQKcm9sZSA9IHByaW1hcnkKcmV0cnlfYnVkZ2V0ID0gNQoKcm9sZSA9IHNlY29uZGFyeQpyZXRyeV9idWRnZXQgPSAzCgpFbmRWaWV3ClZpZXcJQDE3CWJ5dGVzPTY3CkAyNQlGaWxlCXVuaXQtMDIudHh0CnJvbGUgPSBzZWNvbmRhcnkNcmV0cnlfYnVkZ2V0ID0gMw0Ncm9sZSA9IHByaW1hcnkNcmV0cnlfYnVkZ2V0ID0gNQ0KRW5kVmlldwpWaWV3CUAxOAlieXRlcz03MgpAMjYJRmlsZQl1bml0LTAzLnR4dApyb2xlID0gcHJpbWFyeQ0KcmV0cnlfYnVkZ2V0ID0gNQ0KDQpyb2xlID0gc2Vjb25kYXJ5DQpyZXRyeV9idWRnZXQgPSAzDQoKRW5kVmlldwpWaWV3CUAxOQlieXRlcz02NgpAMjcJRmlsZQl1bml0LTA0LnR4dApyb2xlID0gc2Vjb25kYXJ5CnJldHJ5X2J1ZGdldCA9IDMKCnJvbGUgPSBwcmltYXJ5CnJldHJ5X2J1ZGdldCA9IDUKRW5kVmlldwo=\",\"text\":\"View\\t@16\\tbytes=67\\n@24\\tFile\\tunit-01.txt\\nrole = primary\\nretry_budget = 5\\n\\nrole = secondary\\nretry_budget = 3\\n\\nEndView\\nView\\t@17\\tbytes=67\\n@25\\tFile\\tunit-02.txt\\nrole = secondary\\rretry_budget = 3\\r\\rrole = primary\\rretry_budget = 5\\r\\nEndView\\nView\\t@18\\tbytes=72\\n@26\\tFile\\tunit-03.txt\\nrole = primary\\r\\nretry_budget = 5\\r\\n\\r\\nrole = secondary\\r\\nretry_budget = 3\\r\\n\\nEndView\\nView\\t@19\\tbytes=66\\n@27\\tFile\\tunit-04.txt\\nrole = secondary\\nretry_budget = 3\\n\\nrole = primary\\nretry_budget = 5\\nEndView\\n\"}],\"more\":false,\"active\":[2]}\n"}
{"id":17,"bytes":459,"sha256":"5dd2f2486f8fb736b5c2da0d3fc2bbc2dc89b63b4dde9dc45158d9bfb5236038","content":"{\"id\":17,\"events\":[{\"sequence\":61,\"ns\":\"166844205492255\",\"kind\":\"stdin\",\"processId\":2,\"base64\":\"ZXhpdAo=\",\"text\":\"exit\\n\"},{\"sequence\":62,\"ns\":\"166844205527507\",\"kind\":\"stdin-eof\",\"processId\":2},{\"sequence\":63,\"ns\":\"166844205680239\",\"kind\":\"stderr-eof\",\"processId\":2},{\"sequence\":64,\"ns\":\"166844205730681\",\"kind\":\"stdout-eof\",\"processId\":2},{\"sequence\":65,\"ns\":\"166844205804526\",\"kind\":\"close\",\"processId\":2,\"code\":0,\"signal\":null}],\"more\":false,\"active\":[]}\n"}
{"id":18,"bytes":197,"sha256":"3e7fee78eaacb0dac888035d1bb6f36b7246673af85a06ecceb5cf880db57d79","content":"{\"id\":18,\"events\":[{\"sequence\":66,\"ns\":\"166856844793511\",\"kind\":\"finished\",\"firstNs\":\"166630187804206\",\"lastProductCloseNs\":\"166844205804526\",\"elapsedNs\":\"214018000320\"}],\"more\":false,\"active\":[]}\n"}
```


#### Archived file: verify-archive.mjs

Bytes: 3844; SHA-256: `2bcaf6dc326d7a59a2740fac235782cd26861836efd0efd8b79312c1e82201a6`.

```javascript
import fs from 'node:fs';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
const root = '/tmp/backwriter-gate5r.kUMPDh';
const tracker = '/home/NOEEZ/backwriter/docs/tasks/2026-09-05-backwriter-0.3.0-independent-namespace-complete-view.md';
const text = fs.readFileSync(tracker, 'utf8');
const sha = bytes => createHash('sha256').update(bytes).digest('hex');
const parseLines = text => text.split('\n').filter(Boolean).map(JSON.parse);
const files = new Map();
const pattern = /^#### Archived file: (.+)\n\nBytes: (\d+); SHA-256: `([a-f0-9]{64})`\.\n\n```\w+\n([\s\S]*?)^```$/gm;
for (const match of text.matchAll(pattern)) {
  const [, name, length, digest, content] = match;
  assert(!files.has(name));
  assert.equal(Buffer.byteLength(content), Number(length), name);
  assert.equal(sha(content), digest, name);
  if (fs.existsSync(`${root}/${name}`)) assert.equal(content, fs.readFileSync(`${root}/${name}`, 'utf8'), name);
  files.set(name, content);
}
assert.equal(files.size, 21);
const fixture = JSON.parse(files.get('fixture.json'));
const audits = JSON.parse(files.get('all-arms-audit.json'));
let deliveredCount = 0;
let deliveredBytes = 0;
let finalFiles = 0;
for (const arm of ['preflight', 'help-n', 'help-old', 'dummy-old', 'dummy-new', 'genie-old', 'genie-new']) {
  const delivered = parseLines(files.get(`records/${arm}/delivered.jsonl`));
  const requests = parseLines(files.get(`records/${arm}/requests.jsonl`));
  assert.equal(delivered.length, requests.length, arm);
  const events = delivered.flatMap(entry => {
    assert.equal(entry.bytes, Buffer.byteLength(entry.content));
    assert.equal(entry.sha256, sha(entry.content));
    return JSON.parse(entry.content).events;
  });
  const originalEvents = `${root}/records/${arm}/events.jsonl`;
  const reconstructed = events.map(event => JSON.stringify(event) + '\n').join('');
  if (fs.existsSync(originalEvents)) assert.equal(reconstructed, fs.readFileSync(originalEvents, 'utf8'), arm);
  const audit = audits.find(audit => audit.arm === arm);
  if (audit) {
    const finish = events.at(-1);
    assert.equal(finish.kind, 'finished');
    assert.equal(finish.firstNs, requests[0].ns);
    assert.equal(finish.elapsedNs, audit.wholeArmElapsedNs);
    const close = events.filter(event => event.kind === 'close').at(-1);
    assert.equal(finish.elapsedNs, (BigInt(close.ns) - BigInt(requests[0].ns)).toString());
    assert.equal(delivered.length, audit.visibleResponses);
    assert.equal(delivered.reduce((sum, entry) => sum + entry.bytes, 0), audit.visibleBytes);
    for (const [kind, expected] of [['stdout', audit.stdoutBytes], ['stderr', audit.stderrBytes], ['guide', audit.guideBytes]]) {
      assert.equal(events.filter(event => event.kind === kind).reduce((sum, event) => sum + Buffer.from(event.base64, 'base64').length, 0), expected);
    }
    for (const [name, expected] of Object.entries(fixture.expected)) {
      assert.equal(audit.finalFiles[name].content, expected);
      assert.equal(audit.finalFiles[name].sha256, sha(expected));
      const actual = `${root}/${arm}/workspace/${name}`;
      if (fs.existsSync(actual)) assert(Buffer.from(expected).equals(fs.readFileSync(actual)));
      finalFiles++;
    }
    deliveredCount += delivered.length;
    deliveredBytes += audit.visibleBytes;
  }
}
for (const name of ['recorder.mjs', 'preflight-child.mjs', 'verify-preflight.mjs']) {
  if (fs.existsSync(`${root}/${name}`)) {
    const original = fs.readFileSync(`${root}/${name}`, 'utf8');
    assert(text.includes('```javascript\n' + original + '```'), name);
    assert(text.includes(sha(original)), name);
  }
}
process.stdout.write(JSON.stringify({ archivedFiles: files.size, rawEventsReconstructed: true, finalFiles, evaluationResponses: deliveredCount, evaluationDeliveredBytes: deliveredBytes }) + '\n');
```

## Gate 5R capture preflight — evaluation not yet started

Recorded 2026-09-05 before any new evaluator. Entry main and origin/main are
`a340e9ff37f9f78f76e7e380f115cefd2006f85d`, with an initially empty index
and clean worktree. Product/test/build inputs compare byte-identical to
`4c3038505b8f87d01483b60abd3ec7494d0bc94d`. Gate 5's GNU/musl
293/293 and 52 release smokes are reused evidence, not newly executed tests.
No musl rebuild, full suite or old benchmark is repeated.

N is exported from that Gate 4 object and rebuilt only with
`cargo build --offline --locked --release` using a task-local target directory.
Its SHA is `72ef3e058061ca97eb290ddfc5031bdfae2d7c92c2a5be1e6310482ef2da63d0`.
The existing local GNU binary matches the pinned N-1 SHA
`5abd626ad17bb2847f65047bb744cdcb1f28f3b6a2a58691242a39d548f77559`
and is copied into the task root, not rebuilt or installed. Both print
`Backwriter 0.2.6\n`. N-1 public inputs are the clean
`09bb6c424081594bd86a95f04345b786ef9b46b6` Git export; N public inputs
are the Gate 4 export, never the current NO-GO README.

The old Genie N raw events 6–9 show all eight Paragraph Content records first,
then a four-File baseline View, four Replace commands and fresh Check/final
File View. Therefore the extra baseline View is **optional preservation
verification**, not missing-Content recovery. The actual nine capability
commands and original NO-GO/raw transcripts below are preserved. BOX 25's
eight-command reference is not BOX 26's acceptance ceiling. Stream/time/
model-visible evidence gaps are not excused by this classification.

### Predeclared measurement contract

A task-local Node recorder reuses ordinary child-process pipes, without a
PTY, product hook, target-selection logic or retry. It receives explicit
argv and UTF-8 stdin from the evaluator. Raw stdout/stderr/stdin are preserved
as base64 byte segments in recorder-observed sequence order; splitting a
read into at most 512-byte transport segments changes neither bytes nor order.
Original kernel write boundaries or simultaneous cross-pipe kernel ordering
are not claimed. EOF, spawn, close/code/signal, cwd and argv are separate
events. Persistent shell commands do not receive invented OS exit codes.

The recorder uses one process.hrtime.bigint clock. Whole-arm start is receipt
of the first request, before Help/README is read. End is the final product
close event, not transcript/report completion. Gaps between requests, including
model thinking and transport/approval delays, remain included. Build, fixture,
preflight and post-run evaluation/report are outside that interval.

Each visible response is exactly compact JSON plus LF with ordered keys
`id, events, more, active`. JSON escaping, metadata keys, base64 and readable
text duplication all count toward **model-delivered text-content UTF-8 bytes**.
Invalid UTF-8 segments retain base64 with text=null. A response contains at
most 6,000 bytes of serialized events; remaining events queue for explicit
polls, never disappear through truncation. Empty responses, failures, repeated
Help/guide reads and every split response are included.

The evaluator forwards the execution result's output string unchanged through
`text(result.output)`. Before forwarding, an ack request passes that exact
string back to the recorder, which compares it byte-for-byte to the saved
response and stores its UTF-8 length/SHA/content. Only the original response
is emitted to the model; nested transport and ack tool outputs are not emitted.
A successful ack is not product delivery/authenticity semantics. This records
the actual text-content argument delivered by the tool orchestration API.
The platform-added tool-result header/envelope, hidden tokenization and UI
rendering bytes are **unobservable and excluded**, not represented as zero.
Product stdout/stderr counts are computed independently and never substituted
for these delivered-content counts. One evaluator functions.exec call per
visible response is the model tool-turn unit; transport/ack subprocess counts
are separate from bw processes and capability commands.

Arms receive only their pinned executable/public guide and isolated fixture.
The recorder is transport, not an answer sequence. Each evaluator must poll
while more=true, close all product processes and drain final output; no
failure may be discarded. The parent checks the full-byte oracle independently.

### Actual preflight results

The first unprivileged Unix-socket listen failed EPERM before any child or
evaluation. The same task-local recorder was started with approved execution
permission; no TCP listener/service or dependency was added. New AI arms have
not started at this checkpoint.

The test child emitted separate stdout/stderr, Unicode, CRLF/LF, quotes,
backslash and tab, accepted exact stdin, then exited 7 after EOF. Normal bw
Help exited 0 with empty stderr. N shell Help required two visible responses
with more=true followed by the remaining bytes. Direct Help output was
compared to concatenated recorded raw bytes, including terminal LF.
All saved responses equal their actual forwarded strings and SHA/length;
all visible events reconstruct the original event stream. An explicit empty
poll is recorded. Monotonic event order was checked.

| Preflight input | Events | Delivered responses | Delivered text bytes | stdout bytes | stderr bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| Unicode/EOF/nonzero child plus empty poll | 11 | 2 | 1,325 | 43 | 29 |
| N top and shell Help | 19 | 3 | 10,043 | 3,446 | 0 |
| N-1 top Help | 8 | 1 | 3,052 | 963 | 0 |

The verifier returned passed=true. These are preflight counts, not AI-arm
measurements. Full reconstructed recorder inputs and raw records are retained
until final archival; evaluation remains pending.

### Capture source, preserved before evaluation

Task root: `/tmp/backwriter-gate5r.kUMPDh`. Paths are task-local and must be
substituted consistently during reconstruction, not installed in real HOME.
Node is the existing `/usr/bin/node` v26.8.1. Rust/Cargo match Gate 5's recorded
1.95.0 toolchain. No product source or build flags are changed.

Recorder SHA-256:
`b51d9d1cc9d389cd3a1948d27560f486e067721cf83cc74e71001112d78453b8`.

```javascript
import fs from 'node:fs';
import net from 'node:net';
import path from 'node:path';
import { spawn } from 'node:child_process';
import { createHash } from 'node:crypto';

const [mode, root, arg] = process.argv.slice(2);
const socketPath = path.join(root, 'capture.sock');
const sha = bytes => createHash('sha256').update(bytes).digest('hex');
if (mode === 'client') {
  const socket = net.createConnection(socketPath);
  socket.on('connect', () => socket.end(arg + '\n'));
  socket.on('data', bytes => process.stdout.write(bytes));
  socket.on('error', error => { process.stderr.write(String(error) + '\n'); process.exitCode = 1; });
} else if (mode === 'server') {
  const configs = JSON.parse(fs.readFileSync(path.join(root, 'capture-config.json')));
  const arms = new Map();
  const now = () => process.hrtime.bigint().toString();
  const append = (file, value) => fs.appendFileSync(file, JSON.stringify(value) + '\n');
  function armFor(name) {
    if (!Object.hasOwn(configs, name)) throw Error('Unknown arm');
    if (!arms.has(name)) {
      const directory = path.join(root, 'records', name);
      fs.mkdirSync(directory, { recursive: true });
      arms.set(name, { name, directory, queue: [], processes: new Map(), sequence: 0, response: 0, first: null, lastClose: null, responses: new Map(), finished: false });
    }
    return arms.get(name);
  }
  function event(arm, value, visible = true) {
    const record = { sequence: ++arm.sequence, ns: now(), ...value };
    append(path.join(arm.directory, 'events.jsonl'), record);
    if (visible) arm.queue.push(record);
    return record;
  }
  function bytesEvent(arm, kind, bytes, processId = null) {
    for (let offset = 0; offset < bytes.length; offset += 512) {
      const part = bytes.subarray(offset, offset + 512);
      const text = part.toString('utf8');
      event(arm, { kind, processId, base64: part.toString('base64'), text: Buffer.from(text).equals(part) ? text : null });
    }
  }
  async function handle(request) {
    const arm = armFor(request.arm);
    if (request.op === 'ack') {
      const saved = arm.responses.get(request.id);
      if (saved !== request.content) throw Error('Delivered content differs from saved response');
      append(path.join(arm.directory, 'delivered.jsonl'), { id: request.id, bytes: Buffer.byteLength(request.content), sha256: sha(request.content), content: request.content });
      return 'OK\n';
    }
    if (arm.finished) throw Error('Arm already finished');
    const received = now();
    if (arm.first === null) arm.first = received;
    append(path.join(arm.directory, 'requests.jsonl'), { ns: received, request });
    const config = configs[request.arm];
    if (request.op === 'start') {
      if (!Array.isArray(request.args) || request.args.some(x => typeof x !== 'string')) throw Error('args must be string array');
      const processId = arm.processes.size + 1;
      event(arm, { kind: 'start', processId, argv: [config.binary, ...request.args], cwd: config.cwd });
      const child = spawn(config.binary, request.args, { cwd: config.cwd, stdio: ['pipe', 'pipe', 'pipe'] });
      const state = { child, closed: false, eof: false };
      arm.processes.set(processId, state);
      child.on('spawn', () => event(arm, { kind: 'spawn', processId, pid: child.pid }));
      child.on('error', error => event(arm, { kind: 'spawn-error', processId, message: String(error) }));
      child.stdout.on('data', bytes => bytesEvent(arm, 'stdout', bytes, processId));
      child.stderr.on('data', bytes => bytesEvent(arm, 'stderr', bytes, processId));
      child.stdout.on('end', () => event(arm, { kind: 'stdout-eof', processId }));
      child.stderr.on('end', () => event(arm, { kind: 'stderr-eof', processId }));
      child.stdin.on('error', error => event(arm, { kind: 'stdin-error', processId, message: String(error) }));
      child.on('close', (code, signal) => {
        state.closed = true;
        arm.lastClose = event(arm, { kind: 'close', processId, code, signal }).ns;
      });
      if (Object.hasOwn(request, 'stdin')) {
        const bytes = Buffer.from(request.stdin, 'utf8');
        bytesEvent(arm, 'stdin', bytes, processId);
        child.stdin.write(bytes);
      }
      if (request.eof) { state.eof = true; event(arm, { kind: 'stdin-eof', processId }); child.stdin.end(); }
    } else if (request.op === 'send') {
      const state = arm.processes.get(request.processId);
      if (!state || state.closed || state.eof) throw Error('No writable process');
      if (Object.hasOwn(request, 'stdin')) {
        const bytes = Buffer.from(request.stdin, 'utf8');
        bytesEvent(arm, 'stdin', bytes, request.processId);
        state.child.stdin.write(bytes);
      }
      if (request.eof) { state.eof = true; event(arm, { kind: 'stdin-eof', processId: request.processId }); state.child.stdin.end(); }
    } else if (request.op === 'guide') {
      if (!config.guide) throw Error('No public guide for this arm');
      bytesEvent(arm, 'guide', fs.readFileSync(config.guide));
    } else if (request.op === 'finish') {
      if (arm.queue.length || [...arm.processes.values()].some(x => !x.closed)) throw Error('Drain output and close processes first');
      event(arm, { kind: 'finished', firstNs: arm.first, lastProductCloseNs: arm.lastClose, elapsedNs: arm.lastClose === null ? null : (BigInt(arm.lastClose) - BigInt(arm.first)).toString() });
      arm.finished = true;
    } else if (request.op !== 'poll') throw Error('Unknown operation');
    await new Promise(resolve => setTimeout(resolve, 100));
    const events = [];
    let size = 0;
    while (arm.queue.length && size + Buffer.byteLength(JSON.stringify(arm.queue[0])) < 6000) {
      const next = arm.queue.shift();
      events.push(next);
      size += Buffer.byteLength(JSON.stringify(next));
    }
    const id = ++arm.response;
    const response = JSON.stringify({ id, events, more: arm.queue.length > 0, active: [...arm.processes.entries()].filter(([, value]) => !value.closed).map(([key]) => key) }) + '\n';
    fs.writeFileSync(path.join(arm.directory, `response-${id}.json`), response);
    arm.responses.set(id, response);
    return response;
  }
  let chain = Promise.resolve();
  const server = net.createServer({ allowHalfOpen: true }, socket => {
    const chunks = [];
    socket.on('data', chunk => chunks.push(chunk));
    socket.on('end', () => {
      chain = chain.then(async () => {
        try { socket.end(await handle(JSON.parse(Buffer.concat(chunks).toString('utf8')))); }
        catch (error) { socket.end(JSON.stringify({ recorderError: String(error) }) + '\n'); }
      });
    });
  });
  server.listen(socketPath, () => process.stdout.write('Recorder ready\n'));
} else throw Error('Expected server or client mode');
```

Preflight child SHA-256:
`32af6b01af2fcc2338a6f2ff0a1f80fc067f2590009f93c4826211da68d5de20`.

```javascript
process.stdout.write('첫 줄\r\nquote=" slash=\\ tab=\t\n');
process.stderr.write('정상 stderr\n');
let input = Buffer.alloc(0);
process.stdin.on('data', bytes => { input = Buffer.concat([input, bytes]); });
process.stdin.on('end', () => {
  setTimeout(() => {
    process.stdout.write(input);
    process.stderr.write('실패 stderr\r\n');
    process.exitCode = 7;
  }, 30);
});
```

Preflight verifier SHA-256:
`c95368895c22b626e9b74b8e02e8b850b6689a3497a11462adee7a71ea6c07ca`.

```javascript
import fs from 'node:fs';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
const root = '/tmp/backwriter-gate5r.kUMPDh';
const read = (arm, name) => fs.readFileSync(`${root}/records/${arm}/${name}`, 'utf8');
const jsonl = value => value.split('\n').filter(Boolean).map(line => JSON.parse(line));
const bytes = (events, kind, id) => Buffer.concat(events.filter(x => x.kind === kind && x.processId === id).map(x => Buffer.from(x.base64, 'base64')));
const sha = value => createHash('sha256').update(value).digest('hex');
const results = [];
for (const arm of ['preflight', 'help-n', 'help-old']) {
  const events = jsonl(read(arm, 'events.jsonl'));
  const delivered = jsonl(read(arm, 'delivered.jsonl'));
  const responses = fs.readdirSync(`${root}/records/${arm}`).filter(x => /^response-[0-9]+\.json$/.test(x));
  assert.equal(delivered.length, responses.length);
  for (const entry of delivered) {
    assert.equal(entry.content, read(arm, `response-${entry.id}.json`));
    assert.equal(entry.bytes, Buffer.byteLength(entry.content));
    assert.equal(entry.sha256, sha(entry.content));
  }
  const visibleEvents = delivered.flatMap(x => JSON.parse(x.content).events);
  assert.deepEqual(visibleEvents, events);
  assert(events.every((x, i) => i === 0 || BigInt(x.ns) >= BigInt(events[i - 1].ns)));
  if (arm === 'preflight') {
    assert.equal(bytes(events, 'stdout', 1).toString(), '첫 줄\r\nquote=" slash=\\ tab=\t\n입력\r\n끝\n');
    assert.equal(bytes(events, 'stderr', 1).toString(), '정상 stderr\n실패 stderr\r\n');
    assert.equal(bytes(events, 'stdin', 1).toString(), '입력\r\n끝\n');
    assert.equal(events.find(x => x.kind === 'close').code, 7);
    for (const kind of ['stdin-eof', 'stdout-eof', 'stderr-eof']) assert(events.some(x => x.kind === kind));
    assert(delivered.some(x => JSON.parse(x.content).events.length === 0));
  } else {
    for (const start of events.filter(x => x.kind === 'start')) {
      const direct = spawnSync(start.argv[0], start.argv.slice(1), { cwd: start.cwd, input: Buffer.alloc(0) });
      assert.equal(direct.error, undefined);
      assert.equal(direct.status, 0);
      assert(bytes(events, 'stdout', start.processId).equals(direct.stdout));
      assert(bytes(events, 'stderr', start.processId).equals(direct.stderr));
      assert.equal(direct.stderr.length, 0);
      assert.equal(events.find(x => x.kind === 'close' && x.processId === start.processId).code, 0);
    }
  }
  results.push({ arm, events: events.length, deliveredResponses: delivered.length, deliveredBytes: delivered.reduce((sum, x) => sum + x.bytes, 0), stdoutBytes: events.filter(x => x.kind === 'stdout').reduce((sum, x) => sum + Buffer.from(x.base64, 'base64').length, 0), stderrBytes: events.filter(x => x.kind === 'stderr').reduce((sum, x) => sum + Buffer.from(x.base64, 'base64').length, 0) });
}
assert(jsonl(read('help-n', 'delivered.jsonl')).some(x => JSON.parse(x.content).more));
process.stdout.write(JSON.stringify({ passed: true, results }) + '\n');
```



## Preserved Gate 5 record

The following original status, decisions and raw evidence are historical.
Gate 5R's current result and scoped measurement contract are above. The old
record is retained verbatim, including its incomplete-capture NO-GO.

## Status — Gates 1–4 complete; Gate 5 evaluated, readiness NO-GO

Recorded 2026-09-05. Gate 1 closes authority, audited discrepancies, open
decisions and acceptance. Gate 2 implements the existing namespace predicate
and focused regressions only. Gate 3 implements Owner-approved D1/D2 after
the preserved proposal below; its focused verification is recorded separately.
Gate 4 completes Help, private CLI/test modules and evidence contraction.
Gate 5 runs the final 293/293 GNU/musl suites and all four independent AI arms.
Its NO-GO and exact evidence gaps are recorded below; this is not a source-ready
or release claim, and no production correction is made in this gate.
No version bump, readiness SHA,
artifact, installer, publication or operational change is made. Cargo/CLI and
official distribution remain published and closed `0.2.6`.

The [source note](2026-09-05-backwriter-0.3.0-independent-namespace-complete-view-source.md)
and [grounded roadmap](2026-09-05-backwriter-0.3.0-independent-namespace-complete-view-roadmap.md)
are preserved planning evidence. BOX references identify the source note, not
another semantic authority. Active [Protocol](../architecture/backwriter-text-coordination-protocol.md),
[address model](../architecture/rebuildable-structural-addressing.md),
[CLI](../architecture/backwriter-cli-v1.md), [principles](../principles/backwriter-core-principles.md)
and [verification](../development/verification.md) own the reconciled target.

| Input | Pinned evidence |
| --- | --- |
| Backwriter entry | `main`, HEAD = origin/main = `4cb15c23f978dbf0dc3d4fbe884170df181b399e`; tracked worktree/index clean; only the two intended planning notes untracked |
| N-1 Source Authority | `09bb6c424081594bd86a95f04345b786ef9b46b6`, closed `0.2.6` |
| Server read-only control | `main`, HEAD = origin/main = `04bd84a5a386638e4bf7e4d2e60d092bda3c3d35`, clean |
| Public control, inherited R3 evidence | `0.2.6`, 76 files/11 directories, manifest 876 bytes, SHA-256 `47001acd4831954a5106a3aac5b9fdfe0b36791144f355f52523cd0d0eb7d5f1`; no new live probe |
| Source-note file SHA-256 | `c0c26b95b762bccb951e9b2b4fa02333a2bb4ddf230931a102b2a3d03eb9b377` (includes the planning pass's terminal LF) |
| Roadmap file SHA-256 | `6bedb46814df6442976a9a46c23488ecad672dd92d5f65f6ef95116a51866821` |

Both planning files are committed byte-for-byte as received. Their references
to a future Gate 1 remain historical planning language, not current status.

## Gate 1 consumer audit and contraction decisions

The audit used `rg` definitions, direct callers and related tests, not a global
`artext` replacement. It does not infer absence of external Rust users or read
real historical state. The storage assertion in BOX 3–4/23 is reconciled with
the following actual production evidence.

| Responsibility | Current production consumer/evidence | Decision and target |
| --- | --- | --- |
| Root/base resolution | `src/runtime.rs::open_with_authority` accepts an absolute ordinary workspace, canonicalizes/opens it and creates RAM anchors/proofs | Keep accepted Runtime workspace root and no-follow policy; no private directory creator |
| Private source exclusion | `runtime.rs::is_backwriter_spill`, both platform branches | Reuse one shared policy for exact root `.bw` plus legacy `.artext/bw`; Windows ASCII-case policy, otherwise case-sensitive; Gate 2 only |
| Search content and exact File | `runtime/search.rs::execute_exact_file`, executor preflight, traversal and selected-source checks call that predicate | Keep caller-specific Empty/skip/error behavior and validation order; no matcher, tier, traversal algorithm, order, duplicate or writer change |
| View single/batch | `runtime/view.rs::validate_runtime_input` before accepted source access | Reuse guard and existing Unavailable boundary; no new observer |
| Check | `runtime/check.rs::classify_group` before proof/source access | Preserve private-path NotCurrent and hash/length/Line-count classification |
| Apply | `runtime/apply.rs` shared executor preflight checks the same predicate before staging | Preserve errors, prospective result, publication and proof/Anchor reflection |
| Anchor/invalidation | `runtime/anchor.rs::observe_current` and `invalidate_source` share the guard | Preserve fail-closed observation and path-exact invalidation |
| Private root creation | No creator in Runtime/CLI; `open_with_authority` opens only the workspace | N/A; no eager `.bw`, `.bw/bw`, HOME relocation or new environment precedence |
| Private stored-state read/write | No reader/writer/schema consumer found; DataStore, refs and Host proof are RAM only | N/A; no store, old reader, fallback, migration or deletion |
| Generic spill and stored-state cleanup | Protocol reserves future spill for a host-provided system root; no implementation | N/A; do not introduce spill, source retention or cleanup to satisfy a checklist |
| Update bootstrap | `src/bin/bw.rs::UpdateTemporary::{create,installer,cleanup,handoff}` and Drop | Keep `env::temp_dir()/backwriter-update-<nonce>`, ordinary-directory checks, Unix cleanup and Windows handoff; unrelated to workspace state |
| Apply temporaries | `runtime/apply.rs::Temporary`, `edit_temporary_name`, `publish` | Keep admitted same-parent create-new/no-follow staging/readback/removal/rename; `.env.artext-apply-edit-` and `artext.backwriter-apply-edit-v1-temporary` unchanged |
| Installers | Read-only server `backwriter/install.sh`, `install.ps1`, `install.cmd`; no `.artext`/`.bw` consumer | Keep HOME `.local/bin/bw[.exe]`, existing task temporaries and staging; no installer/Update execution or change |
| Existing path regression | `tests/search.rs` exact File and private/sibling/nested cases; `tests/view.rs::view_rejects_private_path_before_access_and_allows_other_artext_children`; Runtime Windows predicate test | Reuse unique boundaries and extend focused coverage in Gate 2, without actual old-state access |

No non-reconstructible stored-data consumer was found. Discovery of one later
stops only the dependent cutover for an Owner decision; it does not authorize
opening, migrating or deleting its real contents. `.bw` is a reservation and
source-exclusion target, not a claim that a store already exists. The retained
legacy guard protects leftovers without supporting old state.

| Shell responsibility | Actual consumer / regression | Keep, remove or pending |
| --- | --- | --- |
| Dispatch and refs | `execute_session_command` -> `execute_session_view` -> `execute_session_ref_view`; `parse_session_ref_view`, `resolve_session_ref` | Keep handwritten parsing, numeric/named syntax, validation and one initial resolution |
| Runtime result acquisition | Explicit `--as` uses `view_batch`; omitted `--as` loops through `run_view` | Reuse native results; mixed-kind plural self issue remains D1 below, not a hidden wrapper |
| Lost Content | `execute_session_ref_view` destructures `Projected { anddress, .. }` | Remove discarded Content in Gate 3; write the owned result without cloning a display collection |
| Lost successful peers | `outcomes.iter().any(RelationAbsent)` returns `write_session_relation_absent`, which writes only absent entries | Remove this whole-command presentation branch; preserve each normal outcome in place |
| Ref allocation and output | `reserve_session_refs`, `write_session_ref_line`, `BufWriter`, existing stream errors | Reuse capacity/error/location machinery; absent consumes no slot, duplicates each consume one; exact framing/stream-slot KATs are D2 |
| Retained raw/JSON output | `run_view` and existing human/raw/JSON writers, raw named Session result bindings | Keep byte-exact output and actual advanced consumers; do not use them as a Content-fetch workaround |
| Behavioral ref evidence | `shell_local_references_start_at_zero_append_in_order_and_keep_named_raw_aliases`; `shell_local_view_relation_absent_and_search_failure_do_not_consume_reference_slots`; malformed-ref and Replace/Check slot tests in `tests/cli.rs` | Existing all-projected and all-absent KATs do not prove mixed-peer Content; Gate 3 must fill that gap |
| Help | `write_command_help` and shared constants; `command_local_help_kats_are_exact_and_skip_runtime_opening`, usage and example tests | Keep one canonical usage source. Replace unsupported advanced-topic help with real raw Session help in Gate 4, not one-shot execution |
| CLI physical structure | `src/bin/bw.rs`, `tests/cli.rs`, structural `include_str!` consumers | Relocate existing responsibilities only in Gate 4; conditional test modules within one CLI integration crate, retain distinct structural evidence |

Source lookup confirms `view_batch` takes one common `AnddressTarget`. Current
single-input explicit `--as` also takes that batch branch. Current plural
without `--as` is not a one-batch implementation. No tests or production code
are changed to conceal those facts. The existing body-only Replace and raw
Session exact-extent regressions remain distinct consumers, not duplicates.

## Closed target boundaries and explicit prerequisites

The Protocol fixes exact Runtime-root-relative `.bw` and legacy `.artext/bw`
components plus descendants. `.bw-notes`, `.artext/bw2`, other `.artext`
children and nested `x/.bw` / `x/.artext/bw` remain ordinary sources subject
to admission/no-follow policy. No whole `.artext` reservation, path alias,
storage reader or eager creation is authorized.

The CLI target requires input ref -> fresh ref, kind/location and exact returned
Content for each projected occurrence. Order and duplicates survive; absent is
shown at its own position without a slot or fabricated Content. Runtime failure
returns no partial success. Count/reserve projected slots before append/output,
consume existing outcomes, and perform no display-only Search/View/re-resolution.
Single input requires one single View; batch requires one batch View. No second
executor or Content-copy collection is allowed.

| Decision | Status / required closure |
| --- | --- |
| D1: mixed-kind plural self-View | Owner approved; Gate 3 implemented and focused tests passed. Replace common-kind argument with Option: None=self, Some=common upward. Single View unchanged; source-breaking caller migration without facade or second executor |
| D2: exact framing and stream-failure slot reporting | Owner approved; Gate 3 implemented and exact KAT/failure tests passed. Byte-length Content framing, input/fresh refs, reserve-before-output, append-before-record, no absent slot, terminal Stream and no Drop retry; no delivery, cancellation or rollback claim |
| D3: original four-file fixture | Locate before Gate 5. Repository docs/tests search found the older three-file control, not the exact independent four-file bytes/oracle. If unavailable, explicitly define one common spec-conformant replacement, independent full-byte oracle and new digest for all four arms; no lost-fixture identity claim |

D1/D2 are closed by Gate 3; D3 still blocks its Gate 5 claim. Exact framing
comes from the approved proposal, not the illustrative arrow. No private storage base
decision remains open: the implemented exclusion base is the Runtime root and
there is no store to relocate. Actual future IO needs a new consumer audit.

Keep all v5 fields/wire/KATs/hash domains/workspace coordinate, source identity
and `sourceLineCount` currentness, including false-count `NotCurrent`. Keep
Search matching/traversal algorithm/tiers/order/multiplicity/schema, one-shot
raw/JSON View, exact File/Paragraph Content, Line body-only replacement with
None/LF/CR/CRLF preservation, raw Session, one-shot stdin, Apply/Anchor/Host
proof semantics. Shell stdin remains command input, not EOF Content transport.
No persistence/history/relocation/rebinding, rollback/CAS/lock, new parser,
public crate, generic framework, compatibility reader or performance project.
Multiple Replace commands are still separate publications, not a transaction.

## Five gates and acceptance

| Gate | State | Required evidence before completion |
| --- | --- | --- |
| 1 — authority | Complete | Consumer/N/A inventory; active target versus closed release; D1–D3; unchanged planning notes; input equality, metadata, document/Git hygiene |
| 2 — namespace | Complete | BOX 23 focused tests using existing filter; absent/new/old-only/both roots; task-local old sentinel byte equality; new reserved file/symlink never exposed; exact components/case and ordinary sibling/nested paths; no help/version creation; existing no-follow/admission/direct-access errors; nonexistent store IO stays N/A |
| 3 — complete shell View | Complete, approved D1/D2 | Single self and Line-to-Paragraph/File; one single/batch call; ordered duplicate input/ref/Content mapping; mixed projected/absent peers; empty/terminator/framing KATs; reserve/Runtime/write/flush failures; zero Content-only extra observation; unchanged raw/JSON/Replace; recoverable reserve overflow is not allocator-exhaustion proof |
| 4 — help/modules/verification | Complete | Executable direct/named/quoted examples including ref producers, fresh Current slots and same-source staleness; real advanced Pick/Anchor/Apply/Data help; no new one-shot/parser/crate; private code relocation; tests split only for useful navigation within one CLI integration crate; current rules extracted before duplicate history links, unique evidence preserved verbatim |
| 5 — integration/readiness | Evaluated, NO-GO | 293/293 full GNU/musl tests and 52 release smoke invocations pass; four fresh AI arms match the independent oracle, but required capture evidence is incomplete and Genie N uses nine rather than eight capability commands; no version promotion or release |

Gate 2 sentinels belong only to a task-local fixture, never the real old path.
An ignored subtree need not be opened to validate its contents. BOX 23's absent
new-root and both-root storage expectations mean noncreation/nonconsumption in
this store-less implementation. No test may introduce IO just to obtain PASS.

Gate 4 must not remove unique tests to meet file-size targets or replace
existing safety checks with forwarding wrappers. Ten help sections are the
old KAT, not a requirement for empty Version sections in the target. Preserve
current common verification rules before moving history; duplicate evidence may
be linked, unique metrics/environments/path spellings/SHAs need a preserved
location first. A minimal historical index is only a later bounded decision,
not a Gate 1 file. Do not rewrite past release facts or claim speed from moves.

Gate 5's BOX 25–26 fixture has eight duplicate Lines in four files, one primary
edit per file, four untouched secondary Lines and LF/CR/CRLF/None. Run four
independent arms: Dummy N-1, Dummy N, Genie N-1, Genie N. Dummy may freely use
public help and select one-shot; that choice is not failure. Genie follows
Search 1 + context Paragraph batch View 1 + Replace 4 + batch Check 1 + final
File batch View 1 = eight capability commands, not eight processes/tool turns.
Candidate context View must supply primary-selection Content with zero
Content-fetch self-View, named-binding-copy or raw-View workaround, zero
terminator mistakes/Wrong Apply and exact independent oracle. N-1's missing
Content is measured honestly, not repaired by modifying the comparator.

Record process count, actual bw commands, model tool turns, unexpected CLI
failures, extra Content-only commands, stdout/stderr bytes, model-visible bytes
and elapsed separately. Help/start/exit do not inflate the eight capability
commands. Increased useful Content bytes are not automatically a regression.
Each n=1 arm gives observations, not a mandatory ratio or broad speed claim.
Retain stale Safe Reject controls; one Search is not generalized to multiple
edits within a file. Do not execute <=0.2.5 comparisons, external-tool arms or
the user-installed 0.2.2 bw, and do not update that binary.

## Gate 1 executed and reused verification

- Read AGENTS/active authority, both planning notes and the identified production
  definitions/callers/tests. Read server AGENTS and installer source only; no
  server write, installer, publisher, endpoint or service command was run.
- Compared all 38 tracked non-Markdown Backwriter files byte-for-byte with
  pinned N-1. This includes production, tests and embedded fixtures, Cargo/lock,
  toolchain and release profile. There is no tracked build script or Cargo
  config. No fixture/test/build input is added or changed; the two new planning
  files are Markdown only and are not compiler/test inputs.
- `cargo metadata --offline --locked --format-version 1` succeeds: package
  `0.2.6`, no package feature definitions, existing library/bin/test targets.
  No build/test flags, target or profile are changed. Reuse the recorded GNU
  `x86_64-unknown-linux-gnu` and musl `x86_64-unknown-linux-musl` default-feature
  suites under pinned Rust/Cargo 1.95.0, the same Cargo test profile and recorded
  flags: **285 passed each**. This is retained evidence, not a fresh suite run
  or proof of an untested custom feature/flag combination.
- Run local Markdown-link, fence, conflict-marker and diff checks; inspect
  exact allowed paths and empty pre-stage index; confirm `.artext` is absent
  and untracked and no task-local output was created. Stage only approved
  documents after those checks and recheck the cached diff before commit.

This Gate has no suite, benchmark, artifact build, installed bw execution,
HOME change, publication, Actions, gh, tag, service or DNS action. README,
Rust/tests/Cargo/lock/toolchain, server and live state remain untouched. Full
final candidate verification belongs to Gate 5, not this input-equivalent
documentation change. Native macOS/Windows/PowerShell/CMD gaps and absent
lock/rollback/fsync/crash-durability guarantees remain explicit limitations.

At the Gate 1 boundary, next was Gate 2 only. Gates 3–5 require their preceding evidence and unresolved
decisions; separate future artifact/installer/publication authority is not
granted here.

## Gate 2 namespace implementation and focused evidence

Entry: clean `main = origin/main = f0379b0059a1c51be511742fd9f17cb21b61ac23`.
Only the existing `is_backwriter_spill` non-Windows and Windows branches change
production behavior. Exact root `.bw` joins legacy `.artext/bw`; descendants
require a slash component boundary. Windows compares only these components
ASCII-case-insensitively; non-Windows remains case-sensitive. There is no new
filter, constructor, registry, IO, store, fallback, migration or cleanup path.
Store creation/read/write/spill/cleanup remain N/A.

The Gate 1 consumer table was rechecked against each definition and caller.
All nine production call sites stay byte-identical: four Search, one View,
one Check, one Apply, two Anchor. Search scope preflight still precedes selected
private skipping; exact private File lookup is Empty. View relation validation
still precedes Unavailable. Check returns NotCurrent before proof/source access.
Edit validation precedes private Unavailable, with neither unit Apply nor
receipt Apply publishing. Anchor and invalidation retain existing errors and
leave the unrelated live Anchor/current proof intact. Update bootstrap and
Apply same-parent staging, names, domains and cleanup are unchanged.

| Focused GNU group | Passed | Direct evidence |
| --- | ---: | --- |
| Runtime | 1 | Existing Windows-only predicate test expanded to both platform case policies and exact components |
| Search | 6 | Existing range/private regression extended with absent/new-only/old-only/both sentinels, invalid UTF-8/NUL exclusion, ordinary siblings/nested roots, narrowed scope/admission, private file/symlink and direct View/Check/Anchor/Apply rejection; exact File and admission/no-follow controls |
| View | 5 | Private single/batch Unavailable, projection InvalidInput priority, all-or-none, symlink/nonregular safety, alternate admission |
| Check | 3 | Private NotCurrent in ordered duplicate batches under Untrusted and Host; ordinary Current/NotCurrent/Unavailable controls |
| Apply | 3 | Private unit/receipt rejection, NUL validation priority, unchanged bytes, live Anchor and staging contents; symlink and late-invalid-source controls |
| Anchor | 2 | Private Anchor and both invalidation seams reject without consuming sentinels or changing the ordinary live Anchor/proof; invalid source fail-close control |
| CLI | 5 | Exact help/version, four namespace noncreation/sentinel cases, workspace/admission, Update-help no-download and View error priority |

Total: **25 distinct focused GNU tests passed**. Existing functions/fixtures are
extended, not replaced with a new harness or crate; `tests/support.rs` is
unchanged. Verifier reads of task-local sentinels are not Runtime store IO.
The sole predicate unit now runs on GNU as well as Windows; no native Windows
or Windows-only test execution is claimed. Full GNU/musl suites remain Gate 5;
the earlier 285/285 counts are baseline only, not post-change test results.

Executed offline/locked metadata (full dependency graph), tree, fmt check,
GNU all-target check, clippy `-D warnings`, and release build with Rust/Cargo
1.95.0. Clippy initially found two unnecessary test clones; they were removed
and the final check passes. Default features, existing test/release profiles
and flags are unchanged, with `CARGO_TARGET_DIR` isolated under the task-local
temporary root. No toolchain was installed. Six asserted release smoke commands
cover ordered Search v2/v5 output, both private exact File Empty results,
`--admit x` retaining nested private-looking names, Help and exact
`Backwriter 0.2.6\n`; five fixture files remain byte-identical with no additions.
No user-installed `bw`, Update, benchmark, older comparator or artifact runs.

Input audit: of 38 tracked non-Markdown files, the seven allowed Rust/test
files change; the other 31 are byte-identical to entry. Core/v5 and its KAT
definitions, CLI, remaining Runtime, Cargo/lock/toolchain, support fixtures,
target definitions, features and profiles are unchanged. There is no tracked
build script or Cargo configuration. Changed inputs are tested above, not
covered by blanket reuse of an old suite. The Runtime file delta, including
its test, is **+654 bytes / +27 lines**; the six integration files total
**+12,438 bytes / +312 lines**. This is coverage expansion, not a performance
or size-contraction claim.

Both planning-note SHA-256 values above remain exact; README is unchanged.
Server stays clean at its pinned SHA with no Gate 2 server/live operation.
Document links/fences, allowed paths, empty pre-stage index, untracked output
and repository `.artext`/`.bw` absence are audited. Only this task's temporary
target and CLI fixture are removed before the approved commit/push.

Next: Gate 3 after D1/D2 closure; D3 remains required before Gate 5. Gate 2
does not implement complete shell View, module/help contraction, readiness,
new storage or a release. Official distribution remains closed `0.2.6`.

## Gate 3 D1/D2 proposals — awaiting Owner approval

Proposal only, inspected at clean `main = origin/main =
3f06c8b2ee37c094fef887d8e33bc7304cd27ef7`. Neither decision below is approved
API/format authority, implementation, a Gate 3 GO, or source readiness.
Gates 1–2 remain complete; Gates 3–5 and D3 remain pending. Cargo/CLI/public
remain `0.2.6`. The existing evidence and both planning notes are unchanged.

### D1 recommendation: extend the existing batch projection argument

Recommend replacing, not supplementing, the current batch signature with:

```rust
pub fn view_batch(
    &self,
    anddresses: &[Anddress],
    projection: Option<AnddressTarget>,
) -> Result<Vec<ViewOutcome>, ViewError>
```

Proposed meaning: `None` projects each input to its own `input.target()`;
`Some(kind)` retains the existing common upward projection. Empty input remains
`Ok(Vec::new())` for either form. The existing single `view(&Anddress,
AnddressTarget)` stays unchanged. This is a Rust source-breaking change:
existing callers must replace `view_batch(inputs, kind)` with
`view_batch(inputs, Some(kind))`, including function-pointer expectations.
There is no implicit conversion, compatibility overload/alias, wrapper, new
request type, second executor or claim that external Rust callers do not exist.
v5 wire, ViewOutcome and error variants do not change.

Evidence: `src/bin/bw.rs::execute_session_ref_view` currently uses batch only
with `--as`; omitted `--as` loops over `run_view`. The Runtime batch already
projects every input before source validation and groups the resulting targets
by source, not by target kind. `RangeCapture` consumes each projected range;
`finish_batch` restores input positions. No mixed-kind observation engine is
needed. Proposed flow:

1. Parse and resolve all references once, retaining borrowed input spellings.
2. Exactly one input: one `run_view`/single Runtime View, using explicit kind
   or that input's self kind, including the explicit `--as` case.
3. Multiple inputs: exactly one `view_batch(&inputs, projection)`. Delete the
   self-View loop; do not split by kind, reject mixed kinds or change projection.
4. Pass the optional argument through `runtime.rs` to the existing
   `runtime/view.rs::execute_batch` and `project_inputs`. Only at each
   `project_request` choose `projection.unwrap_or_else(|| input.target())`.
5. Reuse `validate_runtime_input`, source-key ordering, `batch_group_end`,
   `execute_batch_group`, direct/trusted capture, and `finish_batch` unchanged.

All projections are preflighted before admission/private checks and IO, so a
later downward InvalidInput still wins over an earlier unavailable source.
Absent projections remain indexed normal outcomes and do not cause source IO;
this does not add currentness checks to an absent relation. Preserve each
source's one direct observation in Untrusted/proof-miss mode, one retained
handle with requested range reads in matching Host mode, hash/length/Line-count
checks, proof invalidation rules, order, duplicate multiplicity and all-or-none
failure. A stale projected peer still fails the whole batch.

Direct production callers are only one-shot batch View and direct shell View
in `src/bin/bw.rs`; one-shot supplies `Some(kind)` and keeps its existing
required `--as`/JSON grammar and output. Raw named Session single View keeps
`run_view`. Known Rust test callers are `tests/view.rs`, the native batch
oracle inside `tests/cli.rs`, and a resource regression in
`src/runtime/view.rs`; existing explicit kinds become `Some(kind)`.
Approval would therefore minimally permit `src/runtime.rs`,
`src/runtime/view.rs`, `src/bin/bw.rs`, `tests/view.rs`, `tests/cli.rs`, and
targeted active-authority/tracker updates. No other production consumer was
found by repository-wide `rg view_batch`; external source compatibility remains
an explicit Owner cost, not a reason to add a parallel API.

### D2 recommendation: length-delimited Content with existing ref metadata

Recommend this one direct-shell-only presentation, with ASCII TAB/LF as shown:

```text
View<TAB><input REF><TAB>bytes=<N><LF>
<existing write_session_ref_line output for fresh REF, kind and location>
<exact N UTF-8 Content bytes><LF>EndView<LF>
```

`N = content.len()` in bytes, never Unicode scalar/character count. There is
no blank line between the existing metadata line's LF and the first Content
byte. Exactly one LF followed by `EndView` and one LF is display framing after
the N bytes; none of these bytes belongs to Content. Preserve all original
None/LF/CR/CRLF terminators. Do not scan Content for `EndView`, normalize it,
append a source terminator or build another encoded Content value. This is a
human display contract, not a new Core wire, JSON/Content schema or parser.

An absent item is exactly `View<TAB><input REF><TAB>RelationAbsent<LF>` at its
input position, with no fresh ref, kind, fabricated Content or end record.
Echo the once-resolved input token (`@N`, `@name`, `@name[index]`) without its
shell quoting. The existing valid reference grammar is ASCII and contains no
TAB/LF. Duplicate inputs each produce their own projected slot and record.

Path audit corrects a possible premise: `src/source.rs::validate_logical_path`
already rejects every `char::is_control`, including TAB, CR, LF and NUL, as
well as colon/backslash. There are no currently admitted control-character
paths to escape. Spaces inside components, quotes, Unicode and delimiter-like
names stay verbatim in the existing TAB-separated metadata line; they cannot
introduce a structural TAB/LF or location colon. Do not tighten or extend the
path grammar. Byte framing does not promise safe visual rendering of every
Unicode glyph or terminal control contained in Content. Any future allowance
of control-character paths would require a separate presentation review.

Proposed exact stdout KATs below use Rust string escape notation, not literal
backslashes on stdout. Each independent projected example starts with refs
length 3, input `@0` and fresh `@3`; named paths are illustrative existing
valid-v5 fixtures to use after approval, not new fixtures created in this pass.

| Content / case | Exact expected stdout |
| --- | --- |
| Empty File | `"View\t@0\tbytes=0\n@3\tFile\tnote.txt\n\nEndView\n"` |
| Line `x`, None | `"View\t@0\tbytes=1\n@3\tLine\tnote.txt:1\nx\nEndView\n"` |
| Line `x\n`, LF | `"View\t@0\tbytes=2\n@3\tLine\tnote.txt:1\nx\n\nEndView\n"` |
| Line `x\r`, CR | `"View\t@0\tbytes=2\n@3\tLine\tnote.txt:1\nx\r\nEndView\n"` |
| Line `x\r\n`, CRLF | `"View\t@0\tbytes=3\n@3\tLine\tnote.txt:1\nx\r\n\nEndView\n"` |
| Unicode Line `β\r\n` | `"View\t@0\tbytes=4\n@3\tLine\tnote.txt:1\nβ\r\n\nEndView\n"` |
| File `EndView\n` | `"View\t@0\tbytes=8\n@3\tFile\tnote.txt\nEndView\n\nEndView\n"` |
| File `x`, path with space | `"View\t@0\tbytes=1\n@3\tFile\tdir/a b.txt\nx\nEndView\n"` |
| Absent input `@1` | `"View\t@1\tRelationAbsent\n"` |

For mixed input `@0 @1 @0 --as paragraph`, where `@0` has Paragraph Content
`x\n` at `note.txt:1-1` and `@1` is a separator Line, initial refs length 3:

```text
"View\t@0\tbytes=2\n@3\tParagraph\tnote.txt:1-1\nx\n\nEndView\nView\t@1\tRelationAbsent\nView\t@0\tbytes=2\n@4\tParagraph\tnote.txt:1-1\nx\n\nEndView\n"
```

Proposed append/failure rule: acquire all Runtime outcomes first, count only
Projected items P, then call the existing `reserve_session_refs(refs, P)`
before any append/output. Keep a single outcomes collection; for single View
its one-element vector is fallibly reserved before the call. Consume outcomes
zipped with borrowed input spellings in input order. For each Projected item,
append its owned Anddress immediately BEFORE the first write of that record,
then write header, existing `write_session_ref_line(..., None, &refs[slot])`,
the borrowed bytes of its owned Content, and the end framing. Drop Content
after use. Direct indexing of the just-appended slot is not ref re-resolution.
An absent record appends nothing. Flush once after the last record. No Content
clone, second result/display collection, extra resolution/Search/View/read is
needed. Preserve append-only slots; do not undo or reuse appended slots.

Let L be entry refs length, P the total projected count and k the number of
projected records whose processing has begun (append already performed).
Lengths below describe RAM just before returning/error unwind; terminal shell
exit drops all refs, not persistent state. Stdout means bytes from this command.

| Boundary | Ref length | Possible stdout | Error / subsequent commands |
| --- | --- | --- | --- |
| Malformed or unresolved ref | L | Empty | Existing Usage, status 2 recorded; shell continues |
| Runtime failure, including stale/downward | L | Empty | Existing Execution mapping, status 1 recorded; shell continues |
| Input/outcome/ref reserve failure before output | L | Empty | Execution, status 1 recorded; shell continues |
| First logical write fails on first Projected record | L + 1 | Empty or a prefix accepted by the sink; no complete-delivery claim | Stream; immediate shell exit 1 |
| First logical write fails on first absent record | L | Empty or prefix | Stream; immediate shell exit 1 |
| Later/partial header, metadata, Content or end write fails | L + k, including current Projected item but not a current absent item | Prefix through the failing write; possibly earlier complete records | Stream; immediate shell exit 1; unvisited items are not appended |
| Final flush fails | L + P | Empty, partial, or even all bytes; delivery still unconfirmed | Stream; immediate shell exit 1 |
| Writes and explicit flush succeed | L + P | All ordered records | Success; shell continues |

BufWriter may defer the first underlying write until several records have been
appended; that sink failure belongs to the L+k row, not necessarily L+1.
To avoid an implicit retry by BufWriter Drop after a Stream error, the proposed
writer consumes it with `into_parts` and discards its unflushed buffer on error.
Already written bytes cannot be recalled. This is not rollback; no explicit or
Drop-driven retry is added, no previous Apply publication is cancelled, and
partial stdout never establishes successful delivery. The existing
`execute_shell` Stream arm already terminates rather than processing later
commands. Non-Stream command errors retain its highest-error accounting.

### Minimal approval-dependent implementation and focused acceptance

Remove the direct View any-RelationAbsent early return, Content discard and
plural single-View loop. `write_session_relation_absent` has only that direct
View caller; replace its narrow body/name with the complete per-item View
writer, not a retained obsolete writer plus a facade. Keep
`write_session_refs` for Search, `write_session_replace` for Replace and
`write_session_ref_line` unchanged for all actual callers. Reuse BufWriter and
CliError::Stream, with one narrow writer taking ordinary `Write` for tests;
no output trait/framework, production failure hook, module split or new crate.
Single and batch acquisition feed this same writer. One-shot/raw/JSON View,
Search, Replace and Check output bytes remain unchanged.

| Existing regression / location | Preserve and extend after approval |
| --- | --- |
| `tests/cli.rs::shell_local_references_start_at_zero_append_in_order_and_keep_named_raw_aliases` | Exact new direct-View bytes, input/fresh refs, duplicates, named alias and single/mixed-kind self; retain raw named View bytes; structural single-vs-batch call and no re-resolution/Content clone audit |
| `shell_local_view_relation_absent_and_search_failure_do_not_consume_reference_slots` | Replace old absent-only KAT with input-associated absent record; add mixed projected/absent/duplicate ordering and later-slot checks, not suppression of peers |
| `shell_local_references_reject_malformed_numeric_forms_before_runtime_access`, `shell_local_replace_failure_does_not_consume_a_fresh_slot` | Retain malformed/stale rejection; assert failed View adds no slots and later valid command still runs |
| Existing CLI Line terminator and one-shot/raw/JSON View regressions | Keep their bytes unchanged; integrate direct View empty, Unicode, None/LF/CR/CRLF, delimiter-like Content and space-path table cases without another fixture engine |
| `tests/view.rs::view_batch_preserves_empty_single_duplicate_and_mixed_source_order` | Add None with mixed File/Paragraph/Line self targets across same/different sources; explicit Some keeps old expected results |
| `view_batch_preserves_relations_terminators_unicode_and_raw_ranges`, `view_batch_preflights_relations_and_fails_all_for_unavailable_members` | Retain Some upward/absent/downward priorities, mixed absent/current and late failure; extend None identity/duplicate ranges |
| `host_view_batch_reuses_and_invalidates_proof_per_source_group` | Add mixed-kind self to matching/missing/mismatched Host proofs; preserve per-source invalidation and ordinary equivalence |
| `src/runtime/view.rs` existing OneByteReader, provisional-output, structural one-observation and resource regressions | Reuse for mixed targets and Some migration; no new observer or runtime counter |

The CLI integration suite currently has no deterministic in-process ref-vector
allocation/write/flush failure seam. Do not call a successful process KAT proof
of those cases. In the existing binary's test compilation, directly exercise
`reserve_session_refs` with a capacity-overflow request and the actual narrow
View writer with a test-only `Write` sink failing at header/body/end/flush
boundaries (including buffered first-sink-write and an absent first item).
Assert refs length, captured prefix and no flush-on-drop retry. Use existing
valid-v5 decoding and outcomes, not a copied writer or production injection flag.
Capacity-overflow covers the existing recoverable reserve error mapping;
structural ordering proves that real P reservation precedes output. It is not
process allocator-exhaustion proof. Retain the existing terminal Stream arm
and add an integration broken-pipe case only for terminal behavior, not as a
deterministic substitute for byte-boundary tests. A small `#[cfg(test)]` binary
test module is within the proposed `src/bin/bw.rs` change, not a new harness or
integration crate. No such code/tests/fixtures are added by this proposal.

After approval and implementation, run impacted GNU focused CLI/View/Host/
namespace regressions plus fmt and static checks. Full GNU/musl is Gate 5.
Gate 2's 25 focused tests and six smoke commands, and N-1's 285/285 baseline,
are historical evidence only, not fresh D1/D2 or full candidate passes.
This proposal runs document/link/fence/conflict/diff/path and unchanged-input
audits only: no product tests, build, benchmark, operational probe or installed
bw. Only this section is appended; index stays empty, with no commit/push.

Owner decisions requested separately:

1. Approve D1's source-breaking Option batch signature and the bounded caller/
   regression migration above, preserving the existing single seam.
2. Approve D2's exact direct-shell framing and per-record append-before-write,
   reserve-before-output, terminal Stream/no-retry contract and focused tests.

Until those approvals, D1/D2 remain awaiting Owner approval and Gate 3 remains
unimplemented, without a GO claim. D3 and Gates 4–5 are unaffected.

## Gate 3 implementation and focused verification — complete

Entry: `main`, HEAD = origin/main =
`3f06c8b2ee37c094fef887d8e33bc7304cd27ef7`, empty index, with only the
224-line D1/D2 proposal appended above. That proposal is preserved verbatim as
pre-approval evidence. The Owner subsequently explicitly approved D1/D2's
source-breaking API/CLI implementation and allowed-path commit/non-force push.
Approval preceded production mutation; this section records implementation and
verification, not a retroactive implementation claim for the proposal.

| Consumer | Gate 3 decision / evidence |
| --- | --- |
| Runtime batch API and `project_inputs` | Option selects self or common upward kind at the sole `project_request` call; explicit callers migrate to Some. No new type, error, overload or executor; external Rust source compatibility intentionally breaks |
| Direct shell single/plural acquisition | Resolve each input once; reserve singleton result before one single View, otherwise invoke one batch with optional projection. Delete repeated self loop; retain grouping, capture, finish and currentness/Host proof |
| Direct shell presentation | Replace absent-only writer and any-absent suppression with one consuming View writer; count/reserve P before append/output; append owned Anddress before each projected record; write exact Content and discard it |
| Retained writers | Search consumes `write_session_refs`, Replace consumes `write_session_replace`, and all three use the unchanged `write_session_ref_line`. One-shot human/raw/JSON and named Session writers remain their distinct consumers |
| Output failure | Real Write sink exercises every byte boundary with unbuffered and buffered writes, absent-first, final flush, prefixes and L+k/L+P. `into_parts` prevents Drop retry; broken pipe exits before a later Replace |
| Observation evidence | Existing OneByteReader covers mixed File/Paragraph/Line in one forward observation and all-provisional late failure. Structural call audits retain single=1/batch=1, one Untrusted observation or matching Host handle, no Content-only Search/View/re-resolution |

Exactly **116 distinct focused GNU tests passed** after the final Rust edits:
3 binary writer/reservation tests, 74 CLI integration tests, 21 View integration
tests, 13 Runtime View unit tests, and 5 namespace controls (Runtime predicate,
Search private/sibling fixture, Host Check private/admission, Apply unavailable
path, Anchor path-exact invalidation). The existing CLI crate is tested as one
impacted component, not a full GNU product suite. No test/helper file is added;
binary cfg(test) reuses unchanged `tests/support.rs` valid-v5 fixtures. Four
CLI tests and three binary tests are new; existing batch/Host/KAT tests extend
their original evidence.

Covered: mixed-kind None including empty/single/duplicates and multiple sources;
Some upward/absent/downward priority; Host match/mismatch/miss and failure
invalidation; late source failure without output or slots; exact empty/Unicode,
LF/CR/CRLF/None, delimiter-like Content, spaces/quotes in safe metadata; named
and indexed refs; absent peers and subsequent slot numbering; stale and malformed
refs with continuation; raw/JSON/one-shot/Replace/Check bytes. Ref reservation
overflow proves recoverable Execution mapping and length preservation, not
process allocator exhaustion. Production ordering proves P is reserved before
output. No production hook, copied writer, new observer or allocation claim.

Verification corrected new test fixture coordinates and obsolete direct-View
expectations, moved the binary test module to satisfy clippy, and enlarged the
broken-pipe fixture beyond pipe buffering after a concurrent child descriptor
window exposed the initial small-output test's weakness. Final focused tests
pass; no behavior was relaxed to suppress those failures.

Rust/Cargo 1.95.0, default features and existing profiles: full offline/locked
metadata and dependency tree, fmt, GNU all-target check, clippy `-D warnings`,
and release build pass. Nine asserted task-local release commands cover raw
Session Apply fixture setup, Help/Version, Search v2/v5, raw View, Check,
mixed projected/absent/duplicate direct View, mixed-kind self View, and unchanged
named raw View. Each has exact expected stdout, empty stderr and exit 0. The
final fixture is exact `x\r\n \t\n`, SHA-256
`1f55a163eac1472bc23893cf93791df0d0e5434017aebed04d3f15803b29a7cf`.

No full GNU/musl, AI arm, benchmark, historical comparator or native Windows
run is claimed. The 285/285 baseline remains historical. Cargo/CLI/public
stay `0.2.6`; Gate 4 help/private modules/verification contraction and Gate 5
integration/readiness, including D3, remain pending. Existing help constants
are intentionally unchanged until Gate 4. No server, installed bw, actual HOME,
public tree, service or Cloudflare operation occurs. Planning notes, README,
Cargo/lock/toolchain/dependencies and all non-allowlisted tracked inputs remain
byte-identical. Document/Git hygiene and exact-path temporary cleanup precede
the approved commit and non-force push; no artifact or release is produced.

## Gate 4 Help, private modules and verification contraction — complete

Entry: clean `main = origin/main =
3d35f14338d2374777acd485d0bce49387800fbc`. The source note, grounded roadmap
and 224-line pre-approval D1/D2 proposal remain byte-identical. This gate changes
Help and physical organization only; it does not reopen D1/D2, v5, Core/Runtime,
schemas, namespace, version, readiness or operational authority.

### Consumer and deletion decisions

| Responsibility / actual consumer | Decision |
| --- | --- |
| Top dispatch, one-shot argument validation, shared Search/Replace preparation, Runtime calls, Update bootstrap | Keep in `src/bin/bw.rs`; no second executor/parser, Update or HOME behavior change |
| `write_command_help`, constants and `canonical_usage` consumed by dispatch/error reporting | Move to private `bw/help.rs`; reuse usage extraction, add only four raw Session Help topics; remove empty Version sections and the uniform ten-section assertion |
| CLI error construction, promotion, stream/exit reporting consumed by all command paths | Move existing bodies to private `bw/error.rs`; retain error codes/priority and status meanings |
| Direct and raw Session parsing, refs, bindings, Pick composition and Data lifetime | Move to private `bw/shell.rs`; preserve actual advanced consumers and shared one-shot helpers |
| Human/JSON/raw and direct-ref output, Data display, batch reports | Move to private `bw/output.rs`; keep distinct native-result consumers, one canonical address encoder and Gate 3 consuming writer/failure tests |
| Test organization | Keep one CLI integration crate and its shared fixture/support; move only responsibility groups into child modules; no helper/framework or integration binary added |

All 117 top-level production function definitions survive, including platform
alternatives. Excluding signature visibility/wrapping, 116 bodies are exact;
only `write_command_help` changes to accept advanced topics. No non-Help
executor/writer body changes. Module imports and sibling visibility are private
and explicit. Error methods retain their bodies. Production byte equality with
N-1 or the Gate 3 parent is not claimed; module moves do not imply speed gains.
README's inherited 0.2.5 Gate 6 reference is corrected to the actual 0.2.6
Gate 6 `c78e07f242035230e8b071d583491ac633f58d29`: its `src/**` diff against
published Source Authority `09bb6c424081594bd86a95f04345b786ef9b46b6` is empty.

Top Help now introduces shell as ordinary short-reference work. Shell Help
covers all direct commands, numeric/named/indexed refs, quoting/escapes,
Content framing and fresh refs, body-only Line Replace, Current-only Check
slots, same-source staleness and terminal stream failure. Four advanced topics
derive their operands/bindings/output/failures from the unchanged raw parsers.
Their one-shot execution, including `<capability> --help`, stays rejected;
only `bw help <topic>` is added. Existing one-shot Help forms are unchanged.

### Test inventory and focused evidence

| Location after move | Existing test functions |
| --- | ---: |
| `tests/cli.rs` (shared fixtures plus Search/global/Update controls) | 12 |
| `tests/cli/help.rs` | 5 |
| `tests/cli/edit.rs` | 10 |
| `tests/cli/view.rs` | 9 |
| `tests/cli/check.rs` | 6 |
| `tests/cli/shell.rs` | 32 |
| `src/bin/bw/output.rs::view_output_tests` | 3 |

CLI inventory is exactly the same 74 names before and after movement; module
qualification alone changes their displayed paths. Help cases and five complete
direct/advanced example rows extend existing functions. The removed section-order
helper asserted a retired presentation constraint, not a failure boundary.
Independent test-owned Help KATs do not include production constants.
Structural source assertions now read the actual entry/output/shell locations.

Final focused GNU evidence is **116 distinct passed**: CLI 74, binary 3,
View integration 21, Runtime View 13, namespace controls 5. The latter are the
existing spill-boundary unit, Search private/sibling fixture, Host Check
private/admission, Apply unavailable path, and Anchor path-exact invalidation.
Gate 3's reserve/append/framing, every byte-boundary/flush failure, no Drop retry,
single=1/batch=1 and terminal broken pipe all pass. No new observer, counter,
hook, Content clone, parser or result collection is introduced.

Full offline/locked metadata/tree, fmt, GNU all-target check, clippy
`-D warnings` and release build pass with Rust/Cargo 1.95.0, default features
and existing profiles in task-local targets. Initial relocation validation
found one stale include path and unnecessary imports; both were corrected
without weakening their assertions. Final tests/checks pass. This is not a
whole GNU product-suite pass; full GNU/musl, AI arms, D3 and benchmarks remain
Gate 5 work. Previous 285/285 and Gate 3 116/9 are baseline evidence, not fresh
full-suite claims.

### Release smoke and unchanged execution

The clean Gate 3 parent and current candidate are built offline/locked in
separate task-local targets. A common workspace is reset to exact
`needle\r\n` before each comparison; neither installed bw nor actual HOME is
used. **33 process invocations** pass: 12 candidate Help topics, one Version,
five parent/candidate shell example pairs and five one-shot command pairs.
All examples include their ref producers. There is no timing comparison.

The direct example obtains @0 from Search, @1 from Paragraph View, aliases @0,
replaces through that alias to @2, checks old @0 as NotCurrent and @2 as
Current/@3, then reads @3 as @4. Exact final Content is `new value\r\n`.
Pick, Anchor, raw Apply and Data examples match documented commands and
byte-exact expected output. Parent/candidate stdout, stderr, exit and final
source bytes are equal for all five flows.

The one-shot pair uses original Search-v2/v5 embedded object bytes without
reencoding, raw View, Current Check, JSON receipt Edit and stale nonpublication.
Both end with `replacement\r\n`; stale Apply exits 1 with empty stdout and
exact `error: current source is unavailable\n`. Help/Version create no private
state. Usage Help text intentionally changes while codes/status meanings stay
fixed. Release binary hashes are identity evidence, not artifact authority:

- Parent: `f0ecce7326d745b9286c4c6ce86dcae1241dba5116b3f7e1f48cf6bf213e295d`.
- Candidate: `72ef3e058061ca97eb290ddfc5031bdfae2d7c92c2a5be1e6310482ef2da63d0`.
- Task-local smoke driver: `9e1569362ac9234f8160dfcd697cf3ff44d073c5ae1f8b7658ebc921c6deea47`.

### Verification evidence preservation

Current safety/target/reuse/hygiene rules were extracted before removal:
v5/count currentness, no-follow/text/resource fail-close, observation and
all-or-none controls, receipt/proof/Anchor/publication boundaries, exact output
and streaming, full stable-candidate target commands and bounded N/N-1 comparison.
Repeated release/gate narratives and detailed measurements link to their existing
trackers via the [history index](../history/index.md), not a second full archive.

The original verification file is pinned above at 96,993 bytes/1,487 lines,
SHA-256 `7bd2851b1add8756590ab4f0888b2d6e4c0e5f78eaacaffd7afd0a60c2bb9666`.
The [verbatim excerpts](../history/2026-09-05-verification-before-0.3.0.md)
preserve only evidence units with additional details, plus their interpreting
conditions. Each original line span equals the stored excerpt bytes exactly:

| Original verification lines | Preserved additional evidence |
| --- | --- |
| 155–161 | 0.2.5 endpoint/install closure detail with gate context |
| 519–537 | 0.2.4 raw Session rchar and paired terminator evidence |
| 592–611, 632–645, 647–666 | Patch Box scratch-boundary and intermediate 247-test details |
| 740–749 | Exact unrounded Patch Box elapsed values and conditions |
| 908–915 | 0.2.2 endpoint/error/cache and isolated install details |
| 1059–1080 | Phase 7A exact baseline values and follow-up conditions |
| 1116–1210, 1239–1257 | Detailed historical raw/streaming/capability regression inventory |
| 1284–1386 | Beta.3/stable/0.2.0/0.2.1 release verification details |
| 1404–1471 | Historical CLI/Session regression inventory |

All 12 excerpt units pass byte identity. All 273 distinct original long
digests/revisions and multi-decimal measurement values remain present in the
linked/preserved documents. Existing task evidence is not rewritten or rerun;
old v3/v4 paths and contracts are explicitly historical, not current authority.
Link/anchor/fence/conflict-marker audits pass. Current verification keeps only
policy, candidate/N-1, required matrix, this gate's evidence and three evidence
links; no readiness SHA or final suite count is invented.

### Exact sizes and boundary

| Scope | Before lines / bytes | After lines / bytes | Delta |
| --- | ---: | ---: | ---: |
| CLI entry file | 3,437 / 132,293 | 962 / 33,746 | -2,475 / -98,547 |
| CLI entry plus four private modules | 3,437 / 132,293 | 3,917 / 142,109 | +480 / +9,816 |
| CLI test entry | 3,905 / 140,960 | 1,139 / 38,240 | -2,766 / -102,720 |
| CLI test crate including five child modules | 3,905 / 140,960 | 4,129 / 151,117 | +224 / +10,157 |
| Active verification | 1,487 / 96,993 | 141 / 9,790 | -1,346 / -87,203 |
| Unique historical excerpts | absent | 455 / 29,650 | preservation, not active growth |
| Historical index | absent | 18 / 1,413 | existing evidence discovery |

Help is now readable multiline Rust text, not line-golfed escaped constants.
Added bytes document actual commands and independent KATs; reduced entry files
reflect navigation, not total-code or performance contraction.

The 49 tracked paths outside the approved set remain byte-identical to entry,
including Core/Runtime/v5, Cargo/lock/toolchain and both planning notes. The
proposal remains exact. No server, live/public/service/Cloudflare/credential,
installed executable or actual HOME access/change is part of this gate.
Only inspected task-owned exports, targets, scripts and fixtures are removed.
Exact allowed-path staging and cached review precede one non-force push.
Gate 5/D3, native-platform gaps and separately authorized release remain open.

## Gate 5 D3 and final readiness — NO-GO

Recorded 2026-09-05 against clean `main = origin/main =
4c3038505b8f87d01483b60abd3ec7494d0bc94d`. This evaluation changes only
documentation. Production `src/**`, tests, Cargo/lock, build/toolchain inputs
and the version KAT remain byte-identical to that candidate. The verdict is
**NO-GO**, not a claim that native product semantics failed.

The final native matrix passes, and all sixteen final arm files match the
independent oracle. Mandatory comparison evidence is nevertheless incomplete:
three PTY arms merged stdout/stderr, the first Dummy N-1 Help also lacks
separate streams, complete whole-run elapsed/model-visible byte counts were
not captured, and Genie N added one pre-edit File View (nine rather than the
eight reference capability commands). These gaps are not zero values. No
coaching, best-of rerun, production fix, weakened criterion or version promotion
is substituted. Cargo/CLI remain `0.2.6`; public distribution and Update remain
closed `0.2.6`. Update was not run. A separate decision is needed before any
new evaluation or release work; this record creates no release authority.

### Contraction and evidence origin

Reuse: the existing native suites, shell, direct View writer, receipt path,
source-grouped View/Check and release executable. No test/helper/framework or
production code was added to the repository. Task-local exports, four isolated
workspaces and an independent smoke script served only this evaluation.
Store creation/read/write/spill/cleanup are N/A because no production consumer
exists. Retained raw Session remains the exact-range/lifetime consumer, not a
fallback hidden in the candidate's direct View writer.

Repository `rg` searches for four-file, unit-01, primary/secondary, oracle and
fixture references found the older three-file control in the 0.2.6 tracker
and the unresolved BOX 25 reference, but no original four-file bytes/oracle.
`git ls-tree` of docs/tests also found no corresponding fixture. This is a
declared **replacement fixture**, not proof of identity with the missing one.
No host-local historical fixture was inspected. N-1 is solely
`09bb6c424081594bd86a95f04345b786ef9b46b6`; no older comparison was executed.

### Pre-authored independent full-byte oracle

Before any AI run, the following two literal maps were fixed independently of
candidate output. Each arm received only the initial map in its own workspace.
File names are ordered unit-01 through unit-04. JSON string escapes below
represent exact UTF-8 file bytes; no implicit final LF is added to a value.

```json
{"initial":{"unit-01.txt":"role = primary\nretry_budget = 3\n\nrole = secondary\nretry_budget = 3\n","unit-02.txt":"role = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 3\r","unit-03.txt":"role = primary\r\nretry_budget = 3\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n","unit-04.txt":"role = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 3"},"expected":{"unit-01.txt":"role = primary\nretry_budget = 5\n\nrole = secondary\nretry_budget = 3\n","unit-02.txt":"role = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 5\r","unit-03.txt":"role = primary\r\nretry_budget = 5\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n","unit-04.txt":"role = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 5"}}
```

Serialization of each map is compact `JSON.stringify(map) + "\\n"`, in that
insertion order, with no BOM. Initial and expected serializations are each
366 bytes. Initial SHA-256:
`f7fa969215979bb27ae3a983c9c44f9673bc2f9573a2bda39839639cb6da8f75`.
Expected SHA-256:
`e1a729c3b3a8c73a730f92289f8b6d34e3681bdc018cacb4229503b747249bc2`.
The full initial/expected fixture JSON above, compact plus LF, is 755 bytes;
SHA-256 `26e2853962e5611c640c4d5c15f25ca68988a112600bce69f350d58497075998`.

| File | Bytes, initial = final | Initial SHA-256 | Expected/final SHA-256 in all four arms |
| --- | ---: | --- | --- |
| unit-01.txt | 67 | d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa | bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0 |
| unit-02.txt | 67 | f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389 | 6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a |
| unit-03.txt | 72 | 0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6 | 8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0 |
| unit-04.txt | 66 | 4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6 | 72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe |

After every arm exited, the parent compared exact bytes and the complete
workspace name set against the pre-authored expected map. All sixteen files
passed, with no extra workspace entry. Four primary changes, four secondary
preservations, newline mistakes 0 and Wrong Apply 0 hold in each arm. This
parent oracle check is not an external-tool comparison arm or an AI body read.
The N first Paragraph View supplied all decision-making Content. Neither N
arm used self-View, named-copy or raw-View to recover missing Content.

### Independent arm inputs and conditions

All four agents were spawned with `fork_turns="none"`, identical inherited
model/reasoning settings (no override) and the same enabled tool surface.
No implementation conversation, source, private tracker, oracle or sibling
result was supplied. The tools do not expose an independently pinned model
build identifier in these records; inheritance is the recorded condition,
not a claim of an externally reproducible model seed. With three child slots,
Dummy N-1, Dummy N and Genie N-1 started first; Genie N started once a slot
freed. Each was one actual AI run, not a scripted target-selection arm.

Common task instruction: discover the eight exact `retry_budget = 3` Lines
in four files; read surrounding Paragraph Content to select the four whose
Paragraph says `role = primary`; replace only those bodies with
`retry_budget = 5`; preserve secondary Paragraphs and every other
byte/terminator; confirm currentness and inspect final Files with bw.

Common restrictions: no source/AGENTS/private docs/oracle/sibling arm access;
all workspace body reads and edits through the supplied bw only, not filesystem
readers/writers, grep/cat/sed/Git; no installed bw, Update or unrelated directory
work. Existing shell/Node could invoke bw, preserve/parse output and interact
with stdin, but not decide targets. Record actual argv/stdin/output/exit,
process/command/tool counts, extra Content queries and elapsed, preserving
every failure without reset or best-of restart. Save an English transcript
with apply_patch. PTY echo/ONLCR disabling was supplied as transport advice,
with explicit warning to record merged-stream limitations or separately
redirect stderr; no answer command sequence was supplied.

| Arm | Public discovery input / instruction | Binary and isolated workspace under task root |
| --- | --- | --- |
| Dummy N-1 | Public Help only; choose own one-shot or shell route | nminus-target/release/bw; dummy-old/workspace |
| Dummy N | Public Help only; choose own one-shot or shell route | gnu-target/release/bw; dummy-new/workspace |
| Genie N-1 | Public Help and exact pinned N-1 README copy; derive the documented shell route, no supplied answer sequence | nminus-target/release/bw; genie-old/workspace |
| Genie N | Public Help and exact candidate README copy; derive the documented shell route, no supplied answer sequence | gnu-target/release/bw; genie-new/workspace |

Task root was `/tmp/backwriter-gate5.74tHth`. The Genie README copies were
named `public-guide.md` in each arm root, outside its workspace. Their
historical release wording was not rewritten for the comparison. No raw
8-command answer was injected; the reference is assessed after the runs.

### Actual comparison, without hidden normalization

| Measure | Dummy N-1 | Dummy N | Genie N-1 | Genie N |
| --- | ---: | ---: | ---: | ---: |
| bw processes | 13 | 3 | 3 | 3 |
| Help commands | 5 | 2 | 2 | 2 |
| Search | 1 | 1 | 2 | 1 |
| View commands | 2 | 2 | 14 | 3 |
| Edit/Replace commands | 4 | 4 | 4 | 4 |
| Check commands | 1 | 1 | 2 | 1 |
| Capability commands total | 8 | 8 | 22 | 9 |
| Explicit raw Apply | 0 | 0 | 0 | 0 |
| Named-copy let / exit | 0 / 0 | 0 / 1 | 12 / 1 | 0 / 1 |
| Extra Content-only calls beyond context + final inspection | 0 | 0 | 12 | 1 |
| Model functions.exec turns, including transcript save | 7 | 8 | 11 | 10 |
| Unexpected CLI failure / reset / retry | 0 / 0 / 0 | 0 / 0 / 0 | 0 / 0 / 0 | 0 / 0 / 0 |
| Final file oracle | exact | exact | exact | exact |
| Reported partial elapsed, not whole run | >=35.744 s | 26.162 s interactive | 40.459 s interactive | about 43 s after initial docs |

The Dummy N transcript calls its two required Views “extra”; the normalized
extra-call row above counts only work beyond required context/final inspection,
so it is zero. Genie N's additional baseline File View is an actual extra call,
not removed to fit eight. Genie N-1's projected direct View omitted Content,
so it made twelve named-copy/raw-View pairs. Its old secondary refs became
NotCurrent after same-source edits, an expected currentness contract rather
than a product error; the agent's surprise, fresh Search and extra Check remain
in the transcript. Dummy N-1 legitimately chose one-shot JSON and needed no
extra Content fetch. The shell lifecycle and Help are not capability work.
Internal Runtime invocation counts were not instrumented by these AI arms.

Decoded archived output payload counts (UTF-8, no JSON-escape overhead):

| Capture | Dummy N-1 | Dummy N | Genie N-1 | Genie N |
| --- | ---: | ---: | ---: | ---: |
| Product output payload bytes, including Help | 20,437 | 5,047 | 3,745 | 5,507 |
| All archived output payload bytes, including repeated guide reads | 20,437 | 5,047 | 37,393 | 40,555 |
| Independently captured stdout bytes | 19,474 after first Help | unavailable | unavailable | unavailable |
| Independently captured stderr bytes | 0 after first Help | unavailable | unavailable | unavailable |

The first Dummy N-1 Help's 963 bytes are merged, despite its raw transcript's
stdout/stderr field names; its captureNote controls that interpretation.
PTY arms preserve merged bytes, not a fabricated zero-stderr measurement.
All product processes finally exited 0; interactive records do not expose an
OS exit per shell command. The complete model-visible tool-envelope bytes and
full-run times are unavailable. Archived payload totals are reproducible
lower-scope counts, **not** replacements for those missing measurements.
Initial Genie guide output was reconstructed by the agent from its exact
second public-guide read; both reads are counted. These limitations themselves
prevent GO. No n=1 latency ratio, benchmark score or broad speed claim is made.

### Final native verification, executed on N

Rust `1.95.0 (59807616e 2026-04-14)`, Cargo
`1.95.0 (f2d3ce0bd 2026-03-21)`, rustc LLVM `22.1.2`; GNU host
`x86_64-unknown-linux-gnu`, installed `x86_64-unknown-linux-musl`
with unix/crt-static cfg. Default features (no feature override), checked-in
test/dev profiles; release panic=abort, strip=true, lto=true,
codegen-units=1. No new compiler-option override or dependency. All candidate
target output went into separate task-local GNU/musl directories.

Executed offline/locked metadata and dependency tree, fmt --check, all-target
check, full tests, all-target clippy with `-D warnings`, and release build
for GNU and musl. Initial full test output was terminal-truncated; both full
test commands were repeated to retain complete logs, not counted as distinct
test cases. Final per-target counts: library 72, binary 3, Anchor 31, Apply 35,
Check 13, CLI 74, Data 11, Edit 6, Pick 7, Search 20, support 0, View 21,
doctests 0: **293 passed, 0 failed, 0 ignored** on each target.

Existing regressions directly cover v5 KAT/strict errors/no older decoder,
namespace/safety, ordered Search/View/Check and duplicates, D1 mixed-kind
source grouping, D2 every writer byte boundary/flush/terminal Stream/no Drop
retry and slot reservation overflow, broken stdout, EOF Content/terminators,
three receipts/no-op, raw five Edit/four Position, Host hit/miss/invalidation
and Anchor same-after reflection. Existing duplicate drift asserts
Correct 1 / Safe Reject 6 / Wrong Apply 0 for both Untrusted and guarded Host;
duplicate Paragraph and stale prepublication controls also pass. No historical
performance result or earlier focused run is relabeled as this full suite.

The task-local release smoke ran **26 invocations per target, 52 total**:
top/named Help equality and all eleven topics, exact Version
`Backwriter 0.2.6\\n`, raw Session Insert/Apply, mixed File/Line/named/indexed
self View, Paragraph absent/duplicate framing, ordered Check, Search v2/v5,
raw View, JSON batch View, stdin JSON receipt followed by fresh View, stale
nonpublishing rejection, namespace exact-File Empty, and exact final CRLF
bytes. Output streams were separately captured in this parent smoke, unlike
the AI PTYs. This script was not supplied to agents. Node child spawning hit
sandbox EPERM and was rerun with approved execution permission; a later
read-only Node-to-Git hash audit also hit EPERM and used a plain Git pipe.
Neither is a CLI failure or concealed agent retry.

| Identity / raw evidence | SHA-256 or Git object |
| --- | --- |
| Gate 4 src Git tree | 8427811aaae0763aac22e774f01244c246c9d4ee |
| src framed SHA-256, 27 paths | 07cac94f1651db40a6bb6b7e64029740471f18e3c4ab626e5db3b46c7709e386 |
| N GNU release bw | 72ef3e058061ca97eb290ddfc5031bdfae2d7c92c2a5be1e6310482ef2da63d0 |
| N musl release bw | 1ea57f0c1d4c8a8e6e080b6a38bd7840c870e32aa038623660f2b03cfdd3d997 |
| N-1 GNU release bw | 5abd626ad17bb2847f65047bb744cdcb1f28f3b6a2a58691242a39d548f77559 |
| Full GNU test log | 608d9987a195b84282d872baa4588cfc1f2e880dbe0455570614d84c54d8fac1 |
| Full musl test log | e60fdb42d701b62fba8fba9dd15487b48688bace8e84bca81873233c871be6ed |
| Parent smoke script | 0296f2f1f25b055d618d376668e2bcff8a0f62c1c9d5245392cb36645cd0a9e0 |
| Parent exact smoke result JSON | 60a661343f135da62d24665b8a73564f397dbfefe16d20890a880d96ec914049 |

Source hash framing is each sorted `git ls-files -z src` path, NUL, exact
file bytes, NUL, concatenated then SHA-256. N-1 was built from its clean Git
export; its entire old suite was not rerun. N was tested from the clean pinned
checkout. No version change occurred, so there is no new-version binary/KAT
reuse claim. Documentation-only final edits preserve every test/build input.
Native macOS, Windows, PowerShell and CMD were not executed. No installer,
publisher, service, server, public endpoint, real HOME or installed bw was
accessed by this evaluation. No release or updater operation was performed.

### Unedited arm transcripts

The following fenced contents preserve the original agent transcripts,
including imprecise terminology and capture limitations; the normalized
assessment above does not silently rewrite them. Each SHA hashes the exact
UTF-8 transcript including its original terminal LF(s), excluding the fence.
Tool-wrapper metadata not captured there remains unavailable. Task-local
paths in these records are historical evidence and are removed after archival.

| Transcript | Bytes | SHA-256 |
| --- | ---: | --- |
| dummy-old | 55,824 | 470f0da66b7badbb3f4193041bdda4ffef17a0b2d6f535130786758183489da4 |
| dummy-new | 8,878 | 1f592454ded40d32b38a8d64c50c95a619de5ee99be7c9166c29667537786855 |
| genie-old | 43,638 | 8ee69ce4ccee59944fbe63b6956963fc743e97ffddb08cf1bb2a4509a2e62ac5 |
| genie-new | 48,806 | f62b653c26ed21d79a118883dea7abae247234109cb05fa308185eed9c8141ff |

#### D3 dummy-old transcript

````text
# Independent public-Help-only CLI run: dummy-old

## Result and decisions

One independent run, no restart. Public Help chose the one-shot route. Search returned eight Lines in four files. A batch Paragraph View showed primary roles at unit-01.txt line 2, unit-02.txt line 5, unit-03.txt line 2, and unit-04.txt line 5. Only after reading that Content, I explicitly selected search result positions 0, 3, 4, 7. Scripts only invoked bw, preserved and parsed its outputs; they did not decide targets. Four Line-body replacements returned changed. The four fresh receipt Anddresses checked current. Final batch File View showed four primary budgets of 5 and four secondary budgets of 3, with LF, CR, CRLF, and final missing terminator represented as expected. No fixture filesystem reads/writes outside bw were performed. No oracle, repository, sibling arm, installed bw, or updater was inspected or invoked.

## Metrics

- bw processes: 13 (5 Help; Search 1; View 2; Edit 4; Check 1; Shell 0).
- Capability-bearing CLI processes: 8. View operands: 8 Paragraph projections plus 4 File projections; Check operands: 4; Edit targets: 4.
- Extra Content-only queries beyond required discovery context and final inspection: 0. Content-bearing View commands total: 2 (12 operands).
- Failures: 0; retries: 0; restarts: 0.
- Execution tool calls: 6 exec_command calls in 6 functions.exec calls. Transcript save adds 1 apply_patch in 1 functions.exec call (7 functions.exec calls total through save).
- Helper Node processes: 5. Six command submissions use the execution service's zsh launcher; launcher exec replacement/internal descendants were not instrumented. Count of explicitly launched workload processes: 18 = 13 bw + 5 Node.
- Measured elapsed: 35744 ms, from just after initial --help capture to final Check/File View capture. Initial help and agent setup time were not clocked, so full-run wall time is unavailable; this value is a lower bound, not falsely precise full elapsed.
- Output strings below are exact JSON-escaped UTF-8 captured strings including final newlines. Initial direct Help capture has the documented stream-attribution limitation; all remaining stdout/stderr were captured separately. No PTY was used. All bw stdin inputs are empty; replacement Content was passed as an argv operand.

## Shell submissions

All submissions used cwd `/tmp/backwriter-gate5.74tHth/dummy-old/workspace`. Default execution shell: zsh. Every shell submission exited 0.

### Submission 1

```json
"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw --help"
```

### Submission 2

```json
"node -e 'const {spawnSync}=require(\"node:child_process\");for(const c of [\"search\",\"view\",\"edit\",\"check\"]){const argv=[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"help\",c];const r=spawnSync(argv[0],argv.slice(1),{encoding:\"utf8\"});console.log(JSON.stringify({argv,stdin:\"\",stdout:r.stdout,stderr:r.stderr,exit:r.status}));}'"
```

### Submission 3

```json
"node -e 'const {spawnSync}=require(\"node:child_process\");const argv=[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"search\",\"line\",\"retry_budget = 3\"];const r=spawnSync(argv[0],argv.slice(1),{encoding:\"utf8\"});console.log(JSON.stringify({argv,stdin:\"\",stdout:r.stdout,stderr:r.stderr,exit:r.status}));'"
```

### Submission 4

```json
"node -e 'const {spawnSync}=require(\"node:child_process\");const argv=[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"view\",\"anddress\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-01.txt\\\",\\\"sourceStateHash\\\":\\\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"15\\\",\\\"byteEnd\\\":\\\"32\\\",\\\"terminator\\\":\\\"lf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"32\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-01.txt\\\",\\\"sourceStateHash\\\":\\\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"67\\\",\\\"terminator\\\":\\\"lf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"33\\\",\\\"parentByteEnd\\\":\\\"67\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-02.txt\\\",\\\"sourceStateHash\\\":\\\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"17\\\",\\\"byteEnd\\\":\\\"34\\\",\\\"terminator\\\":\\\"cr\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"34\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-02.txt\\\",\\\"sourceStateHash\\\":\\\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"67\\\",\\\"terminator\\\":\\\"cr\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"67\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-03.txt\\\",\\\"sourceStateHash\\\":\\\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\\\",\\\"sourceByteLength\\\":\\\"72\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"16\\\",\\\"byteEnd\\\":\\\"34\\\",\\\"terminator\\\":\\\"crlf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"34\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-03.txt\\\",\\\"sourceStateHash\\\":\\\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\\\",\\\"sourceByteLength\\\":\\\"72\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"54\\\",\\\"byteEnd\\\":\\\"72\\\",\\\"terminator\\\":\\\"crlf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"36\\\",\\\"parentByteEnd\\\":\\\"72\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-04.txt\\\",\\\"sourceStateHash\\\":\\\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\\\",\\\"sourceByteLength\\\":\\\"66\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"17\\\",\\\"byteEnd\\\":\\\"34\\\",\\\"terminator\\\":\\\"lf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"34\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-04.txt\\\",\\\"sourceStateHash\\\":\\\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\\\",\\\"sourceByteLength\\\":\\\"66\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"66\\\",\\\"terminator\\\":\\\"none\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"66\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"--as\",\"paragraph\"];const r=spawnSync(argv[0],argv.slice(1),{encoding:\"utf8\"});console.log(JSON.stringify({argv,stdin:\"\",stdout:r.stdout,stderr:r.stderr,exit:r.status}));'"
```

### Submission 5

```json
"node -e 'const {spawnSync}=require(\"node:child_process\");for(const argv of [[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"edit\",\"anddress\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-01.txt\\\",\\\"sourceStateHash\\\":\\\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"15\\\",\\\"byteEnd\\\":\\\"32\\\",\\\"terminator\\\":\\\"lf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"32\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"retry_budget = 5\"],[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"edit\",\"anddress\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-02.txt\\\",\\\"sourceStateHash\\\":\\\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"67\\\",\\\"terminator\\\":\\\"cr\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"67\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"retry_budget = 5\"],[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"edit\",\"anddress\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-03.txt\\\",\\\"sourceStateHash\\\":\\\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\\\",\\\"sourceByteLength\\\":\\\"72\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"16\\\",\\\"byteEnd\\\":\\\"34\\\",\\\"terminator\\\":\\\"crlf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"34\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"retry_budget = 5\"],[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"edit\",\"anddress\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-04.txt\\\",\\\"sourceStateHash\\\":\\\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\\\",\\\"sourceByteLength\\\":\\\"66\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"66\\\",\\\"terminator\\\":\\\"none\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"66\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"retry_budget = 5\"]]){const r=spawnSync(argv[0],argv.slice(1),{encoding:\"utf8\"});console.log(JSON.stringify({argv,stdin:\"\",stdout:r.stdout,stderr:r.stderr,exit:r.status}));}'"
```

### Submission 6

```json
"node -e 'const {spawnSync}=require(\"node:child_process\");for(const argv of [[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"check\",\"anddress\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-01.txt\\\",\\\"sourceStateHash\\\":\\\"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"15\\\",\\\"byteEnd\\\":\\\"32\\\",\\\"terminator\\\":\\\"lf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"32\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-02.txt\\\",\\\"sourceStateHash\\\":\\\"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"67\\\",\\\"terminator\\\":\\\"cr\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"67\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-03.txt\\\",\\\"sourceStateHash\\\":\\\"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0\\\",\\\"sourceByteLength\\\":\\\"72\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"16\\\",\\\"byteEnd\\\":\\\"34\\\",\\\"terminator\\\":\\\"crlf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"34\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-04.txt\\\",\\\"sourceStateHash\\\":\\\"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe\\\",\\\"sourceByteLength\\\":\\\"66\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"66\\\",\\\"terminator\\\":\\\"none\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"66\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\"],[\"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw\",\"--json\",\"view\",\"anddress\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-01.txt\\\",\\\"sourceStateHash\\\":\\\"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"15\\\",\\\"byteEnd\\\":\\\"32\\\",\\\"terminator\\\":\\\"lf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"32\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-02.txt\\\",\\\"sourceStateHash\\\":\\\"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a\\\",\\\"sourceByteLength\\\":\\\"67\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"67\\\",\\\"terminator\\\":\\\"cr\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"67\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-03.txt\\\",\\\"sourceStateHash\\\":\\\"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0\\\",\\\"sourceByteLength\\\":\\\"72\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"16\\\",\\\"byteEnd\\\":\\\"34\\\",\\\"terminator\\\":\\\"crlf\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"0\\\",\\\"parentByteEnd\\\":\\\"34\\\",\\\"parentFileLineOffset\\\":\\\"0\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"{\\\"version\\\":\\\"artext.backwriter-anddress.v5\\\",\\\"workspaceCoordinate\\\":\\\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\\\",\\\"logicalPath\\\":\\\"unit-04.txt\\\",\\\"sourceStateHash\\\":\\\"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe\\\",\\\"sourceByteLength\\\":\\\"66\\\",\\\"sourceLineCount\\\":\\\"5\\\",\\\"kind\\\":\\\"line\\\",\\\"byteStart\\\":\\\"50\\\",\\\"byteEnd\\\":\\\"66\\\",\\\"terminator\\\":\\\"none\\\",\\\"lineOffsetInParent\\\":\\\"1\\\",\\\"parentKind\\\":\\\"paragraph\\\",\\\"parentByteStart\\\":\\\"35\\\",\\\"parentByteEnd\\\":\\\"66\\\",\\\"parentFileLineOffset\\\":\\\"3\\\",\\\"parentLineCount\\\":\\\"2\\\"}\",\"--as\",\"file\"]]){const r=spawnSync(argv[0],argv.slice(1),{encoding:\"utf8\"});console.log(JSON.stringify({argv,stdin:\"\",stdout:r.stdout,stderr:r.stderr,exit:r.status}));}'"
```

## Exact bw argv, inputs, outputs, and exits

### bw process 1

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--help"
  ],
  "exit": 0,
  "stderr": null,
  "stdout": "USAGE\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\n  bw help [<command>]\n\nGLOBAL OPTIONS\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\n  --json                     Select JSON output where the command supports it.\n  --raw                      Select raw View output only.\n\nCAPABILITIES\n  search   Discover current File, Paragraph, or Line Anddresses.\n  view     Read one or more current Anddresses.\n  edit     Replace one current Anddress.\n  check    Check one or more current Anddresses.\n  shell    Run advanced raw Session commands.\n  version  Print the Backwriter version.\n  update   Run the installed-platform updater.\n\nPick, Anchor, Apply, and Data have no one-shot command; use bw shell.\n\nADDITIONAL HELP\n  bw help <command>\n\nGlobal options precede the command. Canonical output options are documented only in that position.\n",
  "stdin": "",
  "captureNote": "Initial exec tool exposes a combined output field; recorded stdout is that field. Separate stderr bytes are unavailable for this first help invocation. Subsequent invocations capture separate stdout/stderr pipes through Node."
}
```

### bw process 2

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "help",
    "search"
  ],
  "stdin": "",
  "stdout": "NAME\n  bw search - discover current Anddresses by exact literal Line content or logical File path\n\nUSAGE\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] search <line|paragraph|file> <query> [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] search /file <logical-path>\n\nDESCRIPTION\n  Searches admitted Workspace Source. Literal queries are case-sensitive and match exact Line content without normalization.\n\nARGUMENTS\n  <line|paragraph|file>  Returned target kind.\n  <query>                Nonempty literal query.\n  /file <logical-path>   Exact logical File lookup.\n\nOPTIONS\n  --workspace, --admit, and --json must precede search.\n  --source LOGICAL_PATH and --subtree LOGICAL_PATH narrow a literal search scope.\n\nWHAT HAPPENS\n  Opens the Runtime, scans admitted source once per selected source, and returns all-or-nothing current results.\n\nOUTPUT\n  Human output lists matches. --json writes the fixed bw.cli.search.v2 envelope.\n\nEXAMPLES\n  bw search line needle --source note.txt\n  bw --json search paragraph needle\n  bw search /file note.txt\n\nFAILURES\n  Invalid request or scope is a usage failure. Unavailable source or Runtime failure exits 1.\n\nSEE ALSO\n  bw help view\n  bw help shell\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 3

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "help",
    "view"
  ],
  "stdin": "",
  "stdout": "NAME\n  bw view - project current content from one or more v5 Anddresses\n\nUSAGE\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json|--raw] view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json view anddress <encoded-v5-Anddress>... --as <line|paragraph|file>\n\nDESCRIPTION\n  Validates current source state and projects the requested target relation from caller-provided v5 Anddresses.\n\nARGUMENTS\n  anddress                  Required input form.\n  <encoded-v5-Anddress>     One or more canonical v5 objects.\n\nOPTIONS\n  --workspace, --admit, --json, and --raw must precede view.\n  --as selects line, paragraph, or file and must be last. Batch View requires --json and --as.\n\nWHAT HAPPENS\n  Opens the Runtime after input validation and returns the requested current projection.\n\nOUTPUT\n  One human or raw View writes content. JSON writes the fixed bw.cli.view.v2 envelope.\n\nEXAMPLES\n  bw view anddress '<v5-Anddress>'\n  bw --raw view anddress '<v5-Line-Anddress>'\n  bw --json view anddress '<v5-Anddress>' --as paragraph\n\nFAILURES\n  Invalid input or unsupported output form is a usage failure. Unavailable or stale source exits 1.\n\nSEE ALSO\n  bw help search\n  bw help check\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 4

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "help",
    "edit"
  ],
  "stdin": "",
  "stdout": "NAME\n  bw edit - replace one current v5 Anddress\n\nUSAGE\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] edit anddress <encoded-v5-Anddress> <content>\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] edit anddress <encoded-v5-Anddress> --stdin\n\nDESCRIPTION\n  Replaces exactly one current File, Paragraph, or Line target through the Runtime Replace seam.\n\nARGUMENTS\n  anddress                  Required input form.\n  <encoded-v5-Anddress>     One canonical v5 object.\n  <content>                  One positional replacement string.\n  --stdin                    Read replacement Content from standard input through EOF.\n\nOPTIONS\n  --workspace, --admit, and --json must precede edit.\n  --stdin is the exclusive Content selector; use standard input to pass literal --stdin Content.\n\nWHAT HAPPENS\n  Validates the Anddress, reads selected standard input before Runtime access, preserves an existing Line terminator automatically, then applies one Replace.\n\nOUTPUT\n  Human output writes the receipt outcome and fresh Anddress when present. --json writes bw.cli.edit.v1.\n\nEXAMPLES\n  bw edit anddress '<v5-Anddress>' 'replacement'\n  printf '%s' 'replacement' | bw edit anddress '<v5-Anddress>' --stdin\n\nFAILURES\n  Invalid input is a usage failure. Standard-input, stale, unavailable, or publication failure exits 1.\n\nSEE ALSO\n  bw help view\n  bw help check\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 5

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "help",
    "check"
  ],
  "stdin": "",
  "stdout": "NAME\n  bw check - check one or more current v5 Anddresses\n\nUSAGE\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... check anddress <encoded-v5-Anddress>\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json check anddress <encoded-v5-Anddress>...\n\nDESCRIPTION\n  Checks the current state of caller-provided v5 Anddresses in input order.\n\nARGUMENTS\n  anddress                  Required input form.\n  <encoded-v5-Anddress>     One or more canonical v5 objects.\n\nOPTIONS\n  --workspace, --admit, and --json must precede check.\n  Multiple inputs require --json. No command-local options are available.\n\nWHAT HAPPENS\n  Validates every input before opening the Runtime, then reports one currentness state per input.\n\nOUTPUT\n  One human input writes one state. --json writes the fixed bw.cli.check.v2 envelope.\n\nEXAMPLES\n  bw check anddress '<v5-Anddress>'\n  bw --json check anddress '<v5-Anddress>' '<v5-Anddress>'\n\nFAILURES\n  Invalid input or a non-JSON batch is a usage failure. Runtime failure exits 1.\n\nSEE ALSO\n  bw help search\n  bw help shell\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 6

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "search",
    "line",
    "retry_budget = 3"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.search.v2\",\"outcome\":\"found\",\"occurrences\":[{\"logicalPath\":\"unit-01.txt\",\"kind\":\"line\",\"line\":\"2\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"15\",\"byteEnd\":\"32\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"32\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"unit-01.txt\",\"kind\":\"line\",\"line\":\"5\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"33\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"unit-02.txt\",\"kind\":\"line\",\"line\":\"2\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"17\",\"byteEnd\":\"34\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"unit-02.txt\",\"kind\":\"line\",\"line\":\"5\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"unit-03.txt\",\"kind\":\"line\",\"line\":\"2\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"16\",\"byteEnd\":\"34\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"unit-03.txt\",\"kind\":\"line\",\"line\":\"5\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"54\",\"byteEnd\":\"72\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"36\",\"parentByteEnd\":\"72\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"unit-04.txt\",\"kind\":\"line\",\"line\":\"2\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"17\",\"byteEnd\":\"34\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}},{\"logicalPath\":\"unit-04.txt\",\"kind\":\"line\",\"line\":\"5\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"66\",\"terminator\":\"none\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"66\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}}]}\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 7

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "view",
    "anddress",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"15\",\"byteEnd\":\"32\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"32\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"33\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"17\",\"byteEnd\":\"34\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"16\",\"byteEnd\":\"34\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"54\",\"byteEnd\":\"72\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"36\",\"parentByteEnd\":\"72\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"17\",\"byteEnd\":\"34\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"66\",\"terminator\":\"none\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"66\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "--as",
    "paragraph"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.view.v2\",\"outcomes\":[{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"0\",\"byteEnd\":\"32\",\"fileLineOffset\":\"0\",\"lineCount\":\"2\"},\"content\":\"role = primary\\nretry_budget = 3\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"33\",\"byteEnd\":\"67\",\"fileLineOffset\":\"3\",\"lineCount\":\"2\"},\"content\":\"role = secondary\\nretry_budget = 3\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"0\",\"byteEnd\":\"34\",\"fileLineOffset\":\"0\",\"lineCount\":\"2\"},\"content\":\"role = secondary\\rretry_budget = 3\\r\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"35\",\"byteEnd\":\"67\",\"fileLineOffset\":\"3\",\"lineCount\":\"2\"},\"content\":\"role = primary\\rretry_budget = 3\\r\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"0\",\"byteEnd\":\"34\",\"fileLineOffset\":\"0\",\"lineCount\":\"2\"},\"content\":\"role = primary\\r\\nretry_budget = 3\\r\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"36\",\"byteEnd\":\"72\",\"fileLineOffset\":\"3\",\"lineCount\":\"2\"},\"content\":\"role = secondary\\r\\nretry_budget = 3\\r\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"0\",\"byteEnd\":\"34\",\"fileLineOffset\":\"0\",\"lineCount\":\"2\"},\"content\":\"role = secondary\\nretry_budget = 3\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"paragraph\",\"byteStart\":\"35\",\"byteEnd\":\"66\",\"fileLineOffset\":\"3\",\"lineCount\":\"2\"},\"content\":\"role = primary\\nretry_budget = 3\"}]}\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 8

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "edit",
    "anddress",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"d5030da326915b93ad162f2fb12e1c9b65468cae66ad397afe69327045aa2dfa\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"15\",\"byteEnd\":\"32\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"32\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "retry_budget = 5"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.edit.v1\",\"outcome\":\"changed\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"15\",\"byteEnd\":\"32\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"32\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}}\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 9

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "edit",
    "anddress",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"f1773b77b3cc23541179b39a19080dde2df5805cc9f9d1f3805ecedc4d825389\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "retry_budget = 5"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.edit.v1\",\"outcome\":\"changed\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}}\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 10

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "edit",
    "anddress",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"0897791dfdd076d281c39f54956b93d8547517adfd3ca654d17f3977eae80cd6\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"16\",\"byteEnd\":\"34\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "retry_budget = 5"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.edit.v1\",\"outcome\":\"changed\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"16\",\"byteEnd\":\"34\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}}\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 11

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "edit",
    "anddress",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"4fcbbfc38bd1d7f8b18ede63c8bb45ce4d0a67b3fc3dcf47785b437366bfafd6\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"66\",\"terminator\":\"none\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"66\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "retry_budget = 5"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.edit.v1\",\"outcome\":\"changed\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"66\",\"terminator\":\"none\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"66\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}}\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 12

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "check",
    "anddress",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"15\",\"byteEnd\":\"32\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"32\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"16\",\"byteEnd\":\"34\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"66\",\"terminator\":\"none\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"66\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.check.v2\",\"outcomes\":[{\"status\":\"current\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"15\",\"byteEnd\":\"32\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"32\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}},{\"status\":\"current\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}},{\"status\":\"current\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"16\",\"byteEnd\":\"34\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}},{\"status\":\"current\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"66\",\"terminator\":\"none\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"66\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}}]}\n",
  "stderr": "",
  "exit": 0
}
```

### bw process 13

```json
{
  "argv": [
    "/tmp/backwriter-gate5.74tHth/nminus-target/release/bw",
    "--json",
    "view",
    "anddress",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"15\",\"byteEnd\":\"32\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"32\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"67\",\"terminator\":\"cr\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"67\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"16\",\"byteEnd\":\"34\",\"terminator\":\"crlf\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"34\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"2\"}",
    "{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"line\",\"byteStart\":\"50\",\"byteEnd\":\"66\",\"terminator\":\"none\",\"lineOffsetInParent\":\"1\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"35\",\"parentByteEnd\":\"66\",\"parentFileLineOffset\":\"3\",\"parentLineCount\":\"2\"}",
    "--as",
    "file"
  ],
  "stdin": "",
  "stdout": "{\"schema\":\"bw.cli.view.v2\",\"outcomes\":[{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-01.txt\",\"sourceStateHash\":\"bc8cc52d2e785a22bd9e128abd5fd9c11c31a0acd1e779993fcf19932ffb86d0\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"file\"},\"content\":\"role = primary\\nretry_budget = 5\\n\\nrole = secondary\\nretry_budget = 3\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-02.txt\",\"sourceStateHash\":\"6848e9494b59c5bff10181e4e5927a7307894008a1e71097dd3f3d47e39a275a\",\"sourceByteLength\":\"67\",\"sourceLineCount\":\"5\",\"kind\":\"file\"},\"content\":\"role = secondary\\rretry_budget = 3\\r\\rrole = primary\\rretry_budget = 5\\r\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-03.txt\",\"sourceStateHash\":\"8fd3263bf6045e2f2b49137746bf868b4892a3aabed56b44ed24d3d8860528c0\",\"sourceByteLength\":\"72\",\"sourceLineCount\":\"5\",\"kind\":\"file\"},\"content\":\"role = primary\\r\\nretry_budget = 5\\r\\n\\r\\nrole = secondary\\r\\nretry_budget = 3\\r\\n\"},{\"outcome\":\"projected\",\"anddress\":{\"version\":\"artext.backwriter-anddress.v5\",\"workspaceCoordinate\":\"4308cb0cb0caa9782ab4b31dd8ebc0cb4ce8ed1924f9861dcc068cafcdcfa596\",\"logicalPath\":\"unit-04.txt\",\"sourceStateHash\":\"72c7b2e7d2c421aba67f2db7d3c1819e84da9349208c12293e189f9a5b8030fe\",\"sourceByteLength\":\"66\",\"sourceLineCount\":\"5\",\"kind\":\"file\"},\"content\":\"role = secondary\\nretry_budget = 3\\n\\nrole = primary\\nretry_budget = 5\"}]}\n",
  "stderr": "",
  "exit": 0
}
```

````

#### D3 dummy-new transcript

````text
# Independent dummy-new public-Help-only evaluation

## Outcome

Discovered eight matching Lines. Read all eight surrounding Paragraphs before choosing targets. Replaced only primary Lines @0, @3, @4, @7, one per source. Four fresh receipts checked Current. Viewed all four final Files. Secondary values remain 3. No failed commands, retries, resets, or other fixture reads/writes.

## Measurement and capture limits

- Product processes: 3 (two Help processes and one persistent shell).
- Tool calls through task completion: 7 functions.exec calls, each containing one terminal call. Transcript persistence adds one functions.exec/apply_patch call, total 8.
- Shell commands: 9 including exit.
- Capability commands: Search 1; View 2 (8 Paragraph inputs then 4 File inputs); Replace 4; Check 1 (4 inputs). No other capability commands.
- Extra Content-only queries: 2 View commands / 12 inputs; zero additional Search queries.
- Measured interactive phase: 26162 ms from returned shell startup to returned shell completion; initial startup and two Help calls excluded. Whole-run wall time was not independently instrumented, so no exact whole-run elapsed is claimed.
- All outputs below are exact JSON-escaped strings returned by the terminal tool. Help commands used plain pipes, whose API returned merged output without separate stdout/stderr. Shell used a PTY with echo and ONLCR disabled: stdout/stderr are merged and cannot be separately attributed. Thus exact merged output is preserved, not separately captured streams. No diagnostic text appeared.
- cwd for every product process: /tmp/backwriter-gate5.74tHth/dummy-new/workspace.
- Terminal launch shell: zsh default login semantics. Explicit wrapper shell inputs and product argv are recorded below. No additional product processes were invoked.

## Exact transcript

### Call 1

Shell input: "/tmp/backwriter-gate5.74tHth/gnu-target/release/bw --help"

argv: ["/tmp/backwriter-gate5.74tHth/gnu-target/release/bw","--help"]

Merged output: "USAGE\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\n  bw help [<command>]\n\nGLOBAL OPTIONS\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\n  --json                     Select JSON output where the command supports it.\n  --raw                      Select raw View output only.\n\nCOMMANDS\n  shell    Reuse short references across search, view, replace, and check.\n  search   Discover current File, Paragraph, or Line Anddresses.\n  view     Read one or more current Anddresses.\n  edit     Replace one current Anddress.\n  check    Check one or more current Anddresses.\n  version  Print the Backwriter version.\n  update   Run the installed-platform updater.\n\nADDITIONAL HELP\n  bw help <command>\n  Advanced topics: pick, anchor, apply, data (raw Session only; no one-shot execution).\n\nGlobal options precede the command. Use bw help shell for ordinary short-ref work.\n"

Exit: 0.

### Call 2

Shell input: "/tmp/backwriter-gate5.74tHth/gnu-target/release/bw help shell"

argv: ["/tmp/backwriter-gate5.74tHth/gnu-target/release/bw","help","shell"]

Merged output: "NAME\n  bw shell - reuse short references across search, view, replace, and check\n\nUSAGE\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n\nCOMMANDS\n  search <line|paragraph|file> <query> [--source PATH | --subtree PATH]...\n  search /file <logical-path>\n  view <REF>... [--as <line|paragraph|file>]\n  replace <REF> <CONTENT>\n  check <REF>...\n  let <NAME> = <REF>\n  exit\n\nREFERENCES AND CONTENT\n  @N is a zero-based, append-only reference from this process. It ends at EOF or exit.\n  @name is a named Anddress; @hits[0] selects an indexed Search/Pick binding.\n  let name = @N makes a named alias. Names cannot be redefined.\n  Double-quote arguments containing spaces. Quoted escapes are \\\\, \\\", \\n, \\r, and \\t.\n  One physical input line is one command; no single quotes, pipes, or EOF Content selector.\n  Line Replace accepts body only: NUL/CR/LF are rejected; the existing terminator is preserved.\n  File/Paragraph Replace uses exact Content. Changed source bytes stale old same-source refs,\n  including other locations. Use the fresh receipt or explicitly search again; never auto-retry.\n\nOUTPUT\n  Search appends one @N per result; Empty writes nothing.\n  View returns each input ref, a fresh ref, kind/location, and complete Content in input order.\n  View<TAB>REF<TAB>bytes=N starts a record, followed by fresh-ref metadata and N exact bytes.\n  The following LF and EndView line are display framing, not source Content.\n  RelationAbsent is shown at its input position without Content or a fresh slot.\n  Replace writes Unchanged/Changed with a fresh ref, or Changed<TAB>None without one.\n  Check writes one status per input; only Current appends a fresh ref. Check is optional.\n\nEXAMPLES\n  Start bw shell in a workspace with only note.txt containing needle plus CRLF, then enter:\n  search line needle --source note.txt\n  view @0 --as paragraph\n  let selected = @0\n  replace @selected \"new value\"\n  check @0 @2\n  view @3\n  exit\n\nADVANCED\n  let hits = search line needle\n  view anddress @hits[0]\n  Raw named View writes exact Content without direct-View framing.\n  See bw help pick, bw help anchor, bw help apply, and bw help data.\n\nFAILURES\n  Usage errors record exit 2; Runtime/source errors record exit 1; later commands still run.\n  Stdin/stdout failure ends the shell with exit 1. Partial output cannot prove delivery or\n  undo publication. --json/--raw and command-line operands are unavailable.\n\nSEE ALSO\n  bw help search\n  bw help edit\n"

Exit: 0.

### Call 3

Shell input: "stty -echo -onlcr; exec /tmp/backwriter-gate5.74tHth/gnu-target/release/bw shell"

argv: ["stty","-echo","-onlcr"] followed by exec ["/tmp/backwriter-gate5.74tHth/gnu-target/release/bw","shell"]

Merged output: ""

Process remains running (session 8842).

### Call 4

Exact shell stdin: "search line \"retry_budget = 3\"\n"

Merged output: "@0\tLine\tunit-01.txt:2\n@1\tLine\tunit-01.txt:5\n@2\tLine\tunit-02.txt:2\n@3\tLine\tunit-02.txt:5\n@4\tLine\tunit-03.txt:2\n@5\tLine\tunit-03.txt:5\n@6\tLine\tunit-04.txt:2\n@7\tLine\tunit-04.txt:5\n"

Process remains running.

### Call 5

Exact shell stdin: "view @0 @1 @2 @3 @4 @5 @6 @7 --as paragraph\n"

Merged output: "View\t@0\tbytes=32\n@8\tParagraph\tunit-01.txt:1-2\nrole = primary\nretry_budget = 3\n\nEndView\nView\t@1\tbytes=34\n@9\tParagraph\tunit-01.txt:4-5\nrole = secondary\nretry_budget = 3\n\nEndView\nView\t@2\tbytes=34\n@10\tParagraph\tunit-02.txt:1-2\nrole = secondary\rretry_budget = 3\r\nEndView\nView\t@3\tbytes=32\n@11\tParagraph\tunit-02.txt:4-5\nrole = primary\rretry_budget = 3\r\nEndView\nView\t@4\tbytes=34\n@12\tParagraph\tunit-03.txt:1-2\nrole = primary\r\nretry_budget = 3\r\n\nEndView\nView\t@5\tbytes=36\n@13\tParagraph\tunit-03.txt:4-5\nrole = secondary\r\nretry_budget = 3\r\n\nEndView\nView\t@6\tbytes=34\n@14\tParagraph\tunit-04.txt:1-2\nrole = secondary\nretry_budget = 3\n\nEndView\nView\t@7\tbytes=31\n@15\tParagraph\tunit-04.txt:4-5\nrole = primary\nretry_budget = 3\nEndView\n"

Process remains running.

### Call 6

Exact shell stdin: "replace @0 \"retry_budget = 5\"\nreplace @3 \"retry_budget = 5\"\nreplace @4 \"retry_budget = 5\"\nreplace @7 \"retry_budget = 5\"\n"

Merged output: "@16\tChanged\tLine\tunit-01.txt:2\n@17\tChanged\tLine\tunit-02.txt:5\n@18\tChanged\tLine\tunit-03.txt:2\n@19\tChanged\tLine\tunit-04.txt:5\n"

Process remains running.

### Call 7

Exact shell stdin: "check @16 @17 @18 @19\nview @20 @21 @22 @23 --as file\nexit\n"

Merged output: "@20\tCurrent\tLine\tunit-01.txt:2\n@21\tCurrent\tLine\tunit-02.txt:5\n@22\tCurrent\tLine\tunit-03.txt:2\n@23\tCurrent\tLine\tunit-04.txt:5\nView\t@20\tbytes=67\n@24\tFile\tunit-01.txt\nrole = primary\nretry_budget = 5\n\nrole = secondary\nretry_budget = 3\n\nEndView\nView\t@21\tbytes=67\n@25\tFile\tunit-02.txt\nrole = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 5\r\nEndView\nView\t@22\tbytes=72\n@26\tFile\tunit-03.txt\nrole = primary\r\nretry_budget = 5\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n\nEndView\nView\t@23\tbytes=66\n@27\tFile\tunit-04.txt\nrole = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 5\nEndView\n"

Exit: 0.

## Human target decision

Paragraph Content showed primary at unit-01.txt:2, unit-02.txt:5, unit-03.txt:2, unit-04.txt:5. No program selected these targets. Line Replace's documented terminator preservation was used. Final Content shows LF, CR, CRLF, and an unterminated last Line respectively; no full pre-edit File view was taken, so whole-file byte preservation outside viewed Paragraphs relies on the documented Line Replace contract, not an independent byte comparison.

````

#### D3 genie-old transcript

````text
# Independent documented-shell evaluation: N-minus

Only supplied public README and public Help were used. Fixture Content reads and edits were exclusively through the supplied bw, in this arm's workspace. No source, oracle, other arm, installed bw, update, or filesystem fixture Content access occurred.

## Result and metrics

Four primary Line bodies changed: unit-01.txt:2, unit-02.txt:5, unit-03.txt:2, unit-04.txt:5. Eight initial literal matches were inspected via their Paragraph Content before deciding. Final bw File Content shows primary values 5 and secondary values 3 with LF, CR, CRLF, and absent-final-terminator cases preserved by the documented body-only Replace contract. No independent whole-file before/after byte comparison was performed.

All four replacement receipt references checked Current. All four pre-edit secondary references unexpectedly checked NotCurrent; this evidence is retained, not retried away. A new Search found the four remaining secondary Lines and all four fresh references checked Current. No syntax/runtime failures or resets occurred; final shell exit was 0.

Shell elapsed wall clock (from shell-start tool return to exit tool return): 40459 ms. Documentation discovery occurred before that timer; full-run elapsed was not instrumented and must not be inferred from this value.

Tool counts through transcript creation: 11 functions orchestration calls; 12 nested calls (5 exec_command, 6 write_stdin, 1 apply_patch). Product processes: 3 bw processes (--help, help shell, shell). Other explicitly launched processes: 5 zsh wrappers, 2 cat README reads, 1 stty; expected total 11 OS processes, excluding infrastructure and any implicit login-shell startup subprocesses. No process monitor was used.

Shell commands: 35 (Search 2; View 14; Replace 4; Check 2; let 12; exit 1). View commands comprise 2 reference-only projected batches (8 Paragraph and 4 File inputs) plus 12 extra Content-only raw queries (8 Paragraph, 4 File). Check has 12 inputs, 8 Current and 4 NotCurrent. Replace invokes 4 Edit/Apply workflows by documented semantics; no explicit raw Edit/Apply, Pick, Anchor, or Data commands. Internal capability invocation counts were not instrumented. Query literals: two `retry_budget = 3` searches. No extra search for role metadata.

## Capture limitations

All outputs below are exact tool-returned strings represented as JSON escaped UTF-8 text. The tool does not expose separate stdout/stderr for ordinary exec calls. The shell used a PTY, `stty -echo -onlcr`, and therefore its output is explicitly a merged stdout/stderr stream, not independently attributed streams. Input is the exact string sent, including LF. No PTY echo was included. The README was read a second time solely to archive its exact returned text; the initial read's output was the same complete public copy and is reproduced below. This duplication is included in tool/process counts. Output chunks can span multiple shell commands and are not assigned per-command exit status; only final process exit is available.

## Invocation transcript

```json
{"cmd":"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw --help","argv":["/tmp/backwriter-gate5.74tHth/nminus-target/release/bw","--help"],"exit":0,"merged_output":"USAGE\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\n  bw help [<command>]\n\nGLOBAL OPTIONS\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\n  --json                     Select JSON output where the command supports it.\n  --raw                      Select raw View output only.\n\nCAPABILITIES\n  search   Discover current File, Paragraph, or Line Anddresses.\n  view     Read one or more current Anddresses.\n  edit     Replace one current Anddress.\n  check    Check one or more current Anddresses.\n  shell    Run advanced raw Session commands.\n  version  Print the Backwriter version.\n  update   Run the installed-platform updater.\n\nPick, Anchor, Apply, and Data have no one-shot command; use bw shell.\n\nADDITIONAL HELP\n  bw help <command>\n\nGlobal options precede the command. Canonical output options are documented only in that position.\n"}
{"cmd":"cat /tmp/backwriter-gate5.74tHth/genie-old/public-guide.md","argv":["cat","/tmp/backwriter-gate5.74tHth/genie-old/public-guide.md"],"exit":0,"merged_output":"# Backwriter\n\nBackwriter is a Rust Core and Runtime for current, structural text work over\nadmitted Workspace Source. It provides target-local File, Paragraph, and Line\naddresses without turning source history or editor state into Core identity.\n\nThe Core capability inventory is Search, View, Pick, Anchor, Check, Edit,\nApply, and Data. The repository currently provides their Rust Core/Runtime\nsurfaces and the canonical `bw` executable's one-shot human and JSON\nSearch/View/Check/Edit, raw View, Anddress-first one-shot Edit, Session Pick, batch\nCheck, Anchor, Edit, Apply, result-binding, explicit Data modes, and\nAdapter-owned Version and Update.\n\n## Quick start\n\nInstall the closed official `0.2.5` release with the command for your\nplatform.\n\nLinux, macOS, or WSL:\n\n```sh\ncurl -fsSL https://backwriter.pentagration.com/install.sh | sh\n```\n\nWindows PowerShell:\n\n```powershell\nirm https://backwriter.pentagration.com/install.ps1 | iex\n```\n\nWindows CMD:\n\n```cmd\ncurl.exe -fsSL https://backwriter.pentagration.com/install.cmd -o install.cmd && call install.cmd && del install.cmd\n```\n\nThe POSIX installer places `bw` at `$HOME/.local/bin/bw`; the PowerShell and\nCMD paths place `bw.exe` at `$HOME\\.local\\bin\\bw.exe`. They do not change\n`PATH`, a shell startup file, the PowerShell profile, or the registry. A fresh\ninstall prints `Installed Backwriter: <version>`; replacing an existing\ndestination prints `Updated Backwriter: <version>`. The executable path and\n`PATH` guidance are printed separately only when the installation directory is\nnot already on `PATH`.\n\n### Version\n\n```sh\nbw version\n```\n\nThe closed official `0.2.5` build prints exactly:\n\n```text\nBackwriter 0.2.5\n```\n\n### Update\n\n```sh\nbw update\n```\n\n`bw update` downloads and delegates to the current official installer. The\ncurrent manifest selects the closed `0.2.5` distribution. The installer\nreads that manifest, verifies the selected artifact, and installs or reinstalls\nthat manifest version only after validation succeeds. It does not run a\nbackground updater or compare release versions. The installer accepts only the\nexact immutable `0.2.4` manifest and current `0.2.5` manifest; `0.2.3`, `0.2.2`, `0.2.1`, `0.2.0`, stable\n`0.1.0`, and beta.3 acceptance is retired. The command still performs no\nversion comparison and adds no version guard.\n\nThe product is Backwriter. The source Cargo package and library crate are\n`backwriter` at source-ready unpublished `0.2.6`; a source-built `bw version`\nprints `Backwriter 0.2.6`. The sole canonical executable and external Adapter\ncommand are `bw`. There is no `backwriter` binary, alias, or wrapper. The\nofficial installer remains separate and selects the closed public `0.2.5`\ndistribution.\n\nProduction `src/**` remains byte-identical to Gate 6 candidate\n`22e6df23755cdc80b299b77be313d307b67bc37f`. Official artifacts, installers,\nmanifest, Update target, and the exact 68-file public distribution remain\npublished and closed `0.2.5` from Source Authority\n`e4022fc073e9df3928e1c3817b266ce92121a03c`. Update performs no version\ncomparison, so source-built `0.2.6` still installs or reinstalls that official\n`0.2.5` release until a separately authorized release closes the gap.\n\n## Anddress-first editing\n\nThe default source-checkout replacement flow is:\n\n1. Run `bw --json search ...`.\n2. Select one occurrence from `occurrences` and pass its exact embedded v5\n   `anddress` object unchanged as one argv value.\n3. Run `bw edit anddress '<opaque-v5-object>' '<new-content>'`.\n\nHuman Search rows are not encoded Anddress values and cannot be Edit input.\nTreat the selected JSON object as opaque: do not interpret or rewrite its hash,\nrange, length, or other fields. File and Paragraph Content is the exact\nreplacement. Line Content is body-only, rejects NUL, CR, and LF, and preserves\nthe None, LF, CR, or CRLF terminator carried by the exact v5 Line. A Line NUL\nis `edit.content_contains_nul`; a Line CR or LF is\n`edit.line_body_contains_terminator`. The latter explains that Backwriter adds\nthe current terminator and that advanced raw Session Edit/Apply owns exact\nextent replacement. Apply alone confirms that source state before publication.\nView or Pick may help a caller select a target; neither View nor Check is\nrequired.\n\nHuman success writes one exact LF-terminated receipt row. `Unchanged` is\nfollowed by the still-current input v5 object; `Changed` is followed by the\nfresh v5 object when the resulting File, Line, or unique Paragraph has one,\nand otherwise by `None`. With leading `--json`, the same result is the compact\nAdapter-only `bw.cli.edit.v1` object with `schema`, `outcome`, and `anddress`\nkeys in that order; `anddress` is the exact v5 object or JSON `null`. Reuse only\nthe address returned by the receipt. A changed Paragraph with `None` requires\nan explicit Search before later target work. Exit `1` is neither a stale-only\nclassification nor proof that source bytes are unchanged, so it must not\ntrigger automatic retry.\n\nOne-shot Content is either one UTF-8 argv value or the exclusive `--stdin`\nselector in that position; stdin is read to EOF after v5 address validation and\nbefore Runtime access. Both forms have the same target-specific Content rules.\nFile/Paragraph CR and LF are exact Content, while Line body Content never\nsilently strips a trailing newline. Literal `--json` and `--raw` remain exact\nContent; a literal `--stdin` Content value is supplied through standard input.\n\nRaw Session is the advanced composition surface for Insert/Delete/Move/Copy,\nPosition, Anchor/Data lifetime, explicit bindings, and separate Apply. It is\nnot a prerequisite or alias for ordinary Replace. Its existing raw\n`edit replace` accepts caller-provided exact range Content, including an\nexplicit terminator or multiline replacement, and `apply @edit` publishes it\nseparately. The caller owns the binding, index, quoting, terminator, and\npublication boundary. General replacement should use one-shot body Content\nfirst; no exact one-shot flag or alternate executor exists.\n\n### Shell-local references\n\n`bw shell` also provides a short-lived interactive flow without adding a Core\nwire or persistent identity:\n\n```text\nsearch line needle\nview @0\nreplace @1 replacement\ncheck @2 @3\nexit\n```\n\nQuote one direct-shell query or replacement argument when it contains\nwhitespace; the quotes are shell syntax and are not part of the exact literal\nor replacement Content:\n\n```text\nsearch line \"duplicate = one\"\nreplace @1 \"duplicate = two\"\n```\n\nSuccessful direct `search` and projected direct `view` append `@N` references\nin output order, including duplicates. Direct `check <REF>...` resolves every\nreference before Runtime access, writes one Current/NotCurrent/Unavailable\nstate per input, and appends a fresh slot only for Current. `replace @N <content>` uses the same\ntarget-aware Content rules as one-shot Edit and appends a fresh reference for\n`Unchanged` or `Changed` when the receipt has an Anddress; `Changed\\tNone`\nadds none. Slots are zero-based canonical unsigned decimals, append-only, and\ndiscarded at `exit` or EOF. `@name` and `@name[index]` remain the advanced raw\nSession forms; `let name = @N` explicitly clones a numeric slot into that\nexisting named Anddress binding. Raw `edit replace` and separate `apply @edit`\nremain the advanced exact-range path.\n\nThe default workspace is the process current working directory. An explicit\n`--workspace` must be absolute and is checked by Runtime. Search admits `.` by\ndefault; repeat `--admit LOGICAL_PATH` before `search` to narrow admission.\nAfter the query, repeat `--source LOGICAL_PATH` or `--subtree LOGICAL_PATH` to\nnarrow a Search scope. Without a scope selector, Search covers all admitted\nsources.\n\n## Current CLI scope\n\n`bw` currently implements Adapter-owned one-shot Version and Update, one-shot\nhuman or JSON Search, View, Check, and Edit, raw View, Anddress-first one-shot Edit,\nplus Session Pick, batch Check, Anchor, Edit, Apply, and Data:\n\n```text\nbw version\nbw update\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    view anddress <encoded-v5-Anddress>... [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    check anddress <encoded-v5-Anddress>...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --raw\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    check anddress <encoded-v5-Anddress>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n```\n\nVersion and Update do not call Backwriter Core or Runtime and create no Core\nwire or capability workflow.\n\nContent Search preserves Core literal validation, scope, projection, and\ndeterministic result order. The distinct `search /file` form validates one\nlogical path and returns the current File Anddress for an admitted regular\nUTF-8, NUL-free source regardless of whether it is empty or contains matching\ntext. Missing paths and directories return Empty; the form has no scope\nselectors or synthetic content query. View decodes v5 Anddresses and projects\neach to itself or one ancestor before reading its exact content. A single input\ndefaults to self projection. `--as` chooses one target kind; multiple inputs\nrequire both `--json` and `--as`. Check decodes every v5 operand before Runtime\naccess. One human input writes `Current`, `NotCurrent`, or `Unavailable`; a\nbatch requires `--json` and preserves one outcome per input. Search, View,\nCheck, and Edit `--json`\nwrite compact Adapter objects with exact embedded v5 Anddress objects where\napplicable; each is an Adapter schema, not a second Core wire. View uses the\nhard-cut `bw.cli.view.v2` outcomes array for both single and batch results;\nCheck uses the hard-cut `bw.cli.check.v2` ordered outcomes array.\nRaw View is an explicit Adapter exact-text mode that reuses the ordinary View\nprojection without a Core wire or changed View meaning.\nHuman Search, View, and Check keep their existing projections; human Edit\nreceipts intentionally return the exact current v5 object when one exists.\nThe Session holds one Runtime until EOF\nor `exit` and has explicit local Search, Pick, Anddress, Edit, View, and Check\nbindings plus non-aliasing Anchedress handles. It owns one explicit `DataStore`\nfor the Session only; names are typed and never persist past EOF or `exit`.\nSession Pick\npasses a named Search or Pick collection and an Adapter-parsed predicate directly\nto Core; Session batch Check passes a named matching outcome directly to its\nRuntime batch seam and prints only report counts. Session Anchor creates a live\nhandle only through `let <name> = anchor create <anddress-ref>`, views it through\n`view anchored @<name>`, and can invalidate its logical source with `anchor\ninvalidate-source <logical-path>`. One-shot Data and Anchor are intentionally\nunsupported because their DataStore and live-handle contracts require Session\nlifetime. One-shot Pick, raw Edit-object transport, and a separate\nApply transport await collection or Edit transport schema authority. The\ndistinct Anddress-first one-shot Edit above is implemented. Raw output other\nthan one-shot View and further Session behavior remain deferred.\n\n## Scope\n\nBackwriter is not Git, a file watcher, daemon, persistent index, or editor UI.\nIt reads admitted current Workspace Source through Runtime's safe no-follow\naccess. It does not model branches, merges, history, automatic re-evaluation,\nor editor buffers.\n\nApply uses its accepted current observation and does not coordinate concurrent\nwriters. Writers may race and one publication may overwrite another\nsource-visible change; hosts requiring a stronger guarantee coordinate outside\nBackwriter.\n\n## Build from source and test\n\n```sh\ncargo build --offline --locked --release\n./target/release/bw search line \"needle\"\n./target/release/bw --workspace /path/project search paragraph \"needle\"\ncargo test --offline --locked\n```\n\n## Official desktop distribution\n\nThe official distribution authority is\n[https://backwriter.pentagration.com](https://backwriter.pentagration.com).\nIt publishes the closed Backwriter `0.2.5` release for Linux/WSL x86_64,\nmacOS arm64, macOS x86_64, and Windows x86_64 from Source Authority revision\n`e4022fc073e9df3928e1c3817b266ce92121a03c`. Linux uses canonical target\n`x86_64-unknown-linux-musl`;\n`x86_64-unknown-linux-gnu` remains the local development/test-host target.\nmacOS uses `aarch64-apple-darwin` with minimum macOS 11.0 and\n`x86_64-apple-darwin` with minimum macOS 10.12. The macOS artifacts receive\nstatic cross-build verification but are not claimed to have been executed on a\nnative Mac before publication. Windows uses `x86_64-pc-windows-gnu` and the\ncanonical executable `bw.exe`; its static cross-build verification does not\nclaim native Windows, PowerShell, or CMD execution. Linux arm64 is not currently\nprovided, and no universal host-compatibility claim is made.\n\n`install.sh` reads the canonical manifest, verifies the downloaded artifact\nagainst the manifest SHA-256, and installs the verified binary at\n`$HOME/.local/bin/bw` with a same-directory rename. Concurrent same-user\nHOME mutation is caller-owned. The published `.sha256` sidecar is for manual\nverification and is not installer authority. Windows PowerShell verifies the\nsame manifest authority and exact ZIP, and installs to\n`$HOME\\.local\\bin\\bw.exe` without editing PATH or the PowerShell profile. The\nthree canonical install commands and the canonical `bw update` command are kept\ntogether in [Quick start](#quick-start).\n\nThe CMD command writes `install.cmd` in the current directory and removes it\nafter a successful installation. An existing file with that name is replaced.\n\nThe CRLF `install.cmd` checks `curl.exe` and `powershell.exe`, downloads exactly\nthe canonical `install.ps1` over HTTPS-only TLS 1.2-or-newer transport into a\ncollision-failing `%TEMP%` task directory, delegates all installation meaning,\ncleans the directory, and preserves the child exit code. It owns no manifest,\nSHA-256, ZIP, or installation logic. The Linux-hosted CMD regression is static;\nno native CMD execution is claimed.\nThe distribution provides no\npublisher-authenticity signature or trusted signing identity, background or\nautomatic update, telemetry, `sudo` execution, or automatic `PATH` or\nshell-startup-file change.\nGitHub is a public source and documentation mirror, not the distribution\nauthority. The complete beta.1, beta.2, beta.3, stable `0.1.0`, `0.2.0`, and\n`0.2.1` and `0.2.2` version directories remain unchanged and immutable. The\ncomplete `0.2.3`, `0.2.4`, and `0.2.5` version directories are immutable; the\n`0.2.5` release is closed. The active public tree has exactly 68 files, and\nthe current installers and manifest select `0.2.5`. Any later platform or\nversion requires separate Owner authority. Linux arm64, tags, GitHub Releases,\ncrates.io publication, and background or automatic update remain outside the\ncompleted publication. The earlier `0.2.4` publication added its eight\nversioned files and replaced only the two installers and manifest pointers\nwithout replacing any of the 48 prior versioned files or the CMD Adapter. Its\nsecond publisher run reused all 60 files without metadata change. The `0.2.5`\npublication then added its eight versioned files, replaced the two installers\nand manifest pointers, and reused all 68 files on its second run.\n\n## Architecture\n\n- [Current state](docs/current/now.md)\n- [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)\n- [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)\n- [CLI V1 authority](docs/architecture/backwriter-cli-v1.md)\n- [Verification](docs/development/verification.md)\n\n## License\n\n[MIT License](LICENSE)\n"}
{"cmd":"/tmp/backwriter-gate5.74tHth/nminus-target/release/bw help shell","argv":["/tmp/backwriter-gate5.74tHth/nminus-target/release/bw","help","shell"],"exit":0,"merged_output":"NAME\n  bw shell - run one local reference session and advanced raw Session commands\n\nUSAGE\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n\nDESCRIPTION\n  Reads commands from standard input until exit. Direct search, view, replace, and check use session-local numeric Anddress references. Raw bindings and raw capability composition remain the advanced surface.\n\nARGUMENTS\n  None.\n\nOPTIONS\n  --workspace and --admit must precede shell.\n  --json and --raw are unavailable.\n\nWHAT HAPPENS\n  A successful direct search, view, or current check emits append-only @N references. Direct replace uses one reference and emits a fresh reference when one exists. References end with this shell process. Raw let, Pick, View, Check, Anchor, Edit, Apply, and Data retain their existing grammar.\n\nOUTPUT\n  Direct references write @N, target kind, and location. Direct Check writes one state per input. Raw commands write their existing human result.\n\nEXAMPLES\n  bw shell\n  search line needle\n  view @0\n  replace @1 replacement\n  check @2 @3\n  let hits = search line needle\n  view anddress @hits[0]\n  exit\n\nFAILURES\n  Invalid shell grammar is a usage failure. Runtime and source failures exit 1.\n\nSEE ALSO\n  bw help search\n  bw help edit\n"}
{"cmd":"stty -echo -onlcr; exec /tmp/backwriter-gate5.74tHth/nminus-target/release/bw shell","argvs":[["stty","-echo","-onlcr"],["/tmp/backwriter-gate5.74tHth/nminus-target/release/bw","shell"]],"cwd":"/tmp/backwriter-gate5.74tHth/genie-old/workspace","tty":true}
{"input":null,"merged_output":"","exit":null,"session_running":true}
{"input":"search line \"retry_budget = 3\"\n","merged_output":"@0\tLine\tunit-01.txt:2\n@1\tLine\tunit-01.txt:5\n@2\tLine\tunit-02.txt:2\n@3\tLine\tunit-02.txt:5\n@4\tLine\tunit-03.txt:2\n@5\tLine\tunit-03.txt:5\n@6\tLine\tunit-04.txt:2\n@7\tLine\tunit-04.txt:5\n","exit":null,"session_running":true}
{"input":"view @0 @1 @2 @3 @4 @5 @6 @7 --as paragraph\n","merged_output":"@8\tParagraph\tunit-01.txt:1-2\n@9\tParagraph\tunit-01.txt:4-5\n@10\tParagraph\tunit-02.txt:1-2\n@11\tParagraph\tunit-02.txt:4-5\n@12\tParagraph\tunit-03.txt:1-2\n@13\tParagraph\tunit-03.txt:4-5\n@14\tParagraph\tunit-04.txt:1-2\n@15\tParagraph\tunit-04.txt:4-5\n","exit":null,"session_running":true}
{"input":"let p0 = @8\nview anddress @p0\n","merged_output":"role = primary\nretry_budget = 3\n","exit":null,"session_running":true}
{"input":"let p1 = @9\nview anddress @p1\nlet p2 = @10\nview anddress @p2\nlet p3 = @11\nview anddress @p3\nlet p4 = @12\nview anddress @p4\nlet p5 = @13\nview anddress @p5\nlet p6 = @14\nview anddress @p6\nlet p7 = @15\nview anddress @p7\n","merged_output":"role = secondary\nretry_budget = 3\nrole = secondary\rretry_budget = 3\rrole = primary\rretry_budget = 3\rrole = primary\r\nretry_budget = 3\r\nrole = secondary\r\nretry_budget = 3\r\nrole = secondary\nretry_budget = 3\nrole = primary\nretry_budget = 3","exit":null,"session_running":true}
{"input":"replace @0 \"retry_budget = 5\"\nreplace @3 \"retry_budget = 5\"\nreplace @4 \"retry_budget = 5\"\nreplace @7 \"retry_budget = 5\"\ncheck @16 @17 @18 @19 @1 @2 @5 @6\n","merged_output":"@16\tChanged\tLine\tunit-01.txt:2\n@17\tChanged\tLine\tunit-02.txt:5\n@18\tChanged\tLine\tunit-03.txt:2\n@19\tChanged\tLine\tunit-04.txt:5\n@20\tCurrent\tLine\tunit-01.txt:2\n@21\tCurrent\tLine\tunit-02.txt:5\n@22\tCurrent\tLine\tunit-03.txt:2\n@23\tCurrent\tLine\tunit-04.txt:5\nNotCurrent\nNotCurrent\nNotCurrent\nNotCurrent\n","exit":null,"session_running":true}
{"input":"view @16 @17 @18 @19 --as file\nlet f0 = @24\nview anddress @f0\nlet f1 = @25\nview anddress @f1\nlet f2 = @26\nview anddress @f2\nlet f3 = @27\nview anddress @f3\nsearch line \"retry_budget = 3\"\ncheck @28 @29 @30 @31\nexit\n","merged_output":"@24\tFile\tunit-01.txt\n@25\tFile\tunit-02.txt\n@26\tFile\tunit-03.txt\n@27\tFile\tunit-04.txt\nrole = primary\nretry_budget = 5\n\nrole = secondary\nretry_budget = 3\nrole = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 5\rrole = primary\r\nretry_budget = 5\r\n\r\nrole = secondary\r\nretry_budget = 3\r\nrole = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 5@28\tLine\tunit-01.txt:5\n@29\tLine\tunit-02.txt:2\n@30\tLine\tunit-03.txt:5\n@31\tLine\tunit-04.txt:2\n@32\tCurrent\tLine\tunit-01.txt:5\n@33\tCurrent\tLine\tunit-02.txt:2\n@34\tCurrent\tLine\tunit-03.txt:5\n@35\tCurrent\tLine\tunit-04.txt:2\n","exit":0,"session_running":false}
{"cmd":"cat /tmp/backwriter-gate5.74tHth/genie-old/public-guide.md","argv":["cat","/tmp/backwriter-gate5.74tHth/genie-old/public-guide.md"],"purpose":"exact public-guide archival reread","exit":0,"merged_output":"# Backwriter\n\nBackwriter is a Rust Core and Runtime for current, structural text work over\nadmitted Workspace Source. It provides target-local File, Paragraph, and Line\naddresses without turning source history or editor state into Core identity.\n\nThe Core capability inventory is Search, View, Pick, Anchor, Check, Edit,\nApply, and Data. The repository currently provides their Rust Core/Runtime\nsurfaces and the canonical `bw` executable's one-shot human and JSON\nSearch/View/Check/Edit, raw View, Anddress-first one-shot Edit, Session Pick, batch\nCheck, Anchor, Edit, Apply, result-binding, explicit Data modes, and\nAdapter-owned Version and Update.\n\n## Quick start\n\nInstall the closed official `0.2.5` release with the command for your\nplatform.\n\nLinux, macOS, or WSL:\n\n```sh\ncurl -fsSL https://backwriter.pentagration.com/install.sh | sh\n```\n\nWindows PowerShell:\n\n```powershell\nirm https://backwriter.pentagration.com/install.ps1 | iex\n```\n\nWindows CMD:\n\n```cmd\ncurl.exe -fsSL https://backwriter.pentagration.com/install.cmd -o install.cmd && call install.cmd && del install.cmd\n```\n\nThe POSIX installer places `bw` at `$HOME/.local/bin/bw`; the PowerShell and\nCMD paths place `bw.exe` at `$HOME\\.local\\bin\\bw.exe`. They do not change\n`PATH`, a shell startup file, the PowerShell profile, or the registry. A fresh\ninstall prints `Installed Backwriter: <version>`; replacing an existing\ndestination prints `Updated Backwriter: <version>`. The executable path and\n`PATH` guidance are printed separately only when the installation directory is\nnot already on `PATH`.\n\n### Version\n\n```sh\nbw version\n```\n\nThe closed official `0.2.5` build prints exactly:\n\n```text\nBackwriter 0.2.5\n```\n\n### Update\n\n```sh\nbw update\n```\n\n`bw update` downloads and delegates to the current official installer. The\ncurrent manifest selects the closed `0.2.5` distribution. The installer\nreads that manifest, verifies the selected artifact, and installs or reinstalls\nthat manifest version only after validation succeeds. It does not run a\nbackground updater or compare release versions. The installer accepts only the\nexact immutable `0.2.4` manifest and current `0.2.5` manifest; `0.2.3`, `0.2.2`, `0.2.1`, `0.2.0`, stable\n`0.1.0`, and beta.3 acceptance is retired. The command still performs no\nversion comparison and adds no version guard.\n\nThe product is Backwriter. The source Cargo package and library crate are\n`backwriter` at source-ready unpublished `0.2.6`; a source-built `bw version`\nprints `Backwriter 0.2.6`. The sole canonical executable and external Adapter\ncommand are `bw`. There is no `backwriter` binary, alias, or wrapper. The\nofficial installer remains separate and selects the closed public `0.2.5`\ndistribution.\n\nProduction `src/**` remains byte-identical to Gate 6 candidate\n`22e6df23755cdc80b299b77be313d307b67bc37f`. Official artifacts, installers,\nmanifest, Update target, and the exact 68-file public distribution remain\npublished and closed `0.2.5` from Source Authority\n`e4022fc073e9df3928e1c3817b266ce92121a03c`. Update performs no version\ncomparison, so source-built `0.2.6` still installs or reinstalls that official\n`0.2.5` release until a separately authorized release closes the gap.\n\n## Anddress-first editing\n\nThe default source-checkout replacement flow is:\n\n1. Run `bw --json search ...`.\n2. Select one occurrence from `occurrences` and pass its exact embedded v5\n   `anddress` object unchanged as one argv value.\n3. Run `bw edit anddress '<opaque-v5-object>' '<new-content>'`.\n\nHuman Search rows are not encoded Anddress values and cannot be Edit input.\nTreat the selected JSON object as opaque: do not interpret or rewrite its hash,\nrange, length, or other fields. File and Paragraph Content is the exact\nreplacement. Line Content is body-only, rejects NUL, CR, and LF, and preserves\nthe None, LF, CR, or CRLF terminator carried by the exact v5 Line. A Line NUL\nis `edit.content_contains_nul`; a Line CR or LF is\n`edit.line_body_contains_terminator`. The latter explains that Backwriter adds\nthe current terminator and that advanced raw Session Edit/Apply owns exact\nextent replacement. Apply alone confirms that source state before publication.\nView or Pick may help a caller select a target; neither View nor Check is\nrequired.\n\nHuman success writes one exact LF-terminated receipt row. `Unchanged` is\nfollowed by the still-current input v5 object; `Changed` is followed by the\nfresh v5 object when the resulting File, Line, or unique Paragraph has one,\nand otherwise by `None`. With leading `--json`, the same result is the compact\nAdapter-only `bw.cli.edit.v1` object with `schema`, `outcome`, and `anddress`\nkeys in that order; `anddress` is the exact v5 object or JSON `null`. Reuse only\nthe address returned by the receipt. A changed Paragraph with `None` requires\nan explicit Search before later target work. Exit `1` is neither a stale-only\nclassification nor proof that source bytes are unchanged, so it must not\ntrigger automatic retry.\n\nOne-shot Content is either one UTF-8 argv value or the exclusive `--stdin`\nselector in that position; stdin is read to EOF after v5 address validation and\nbefore Runtime access. Both forms have the same target-specific Content rules.\nFile/Paragraph CR and LF are exact Content, while Line body Content never\nsilently strips a trailing newline. Literal `--json` and `--raw` remain exact\nContent; a literal `--stdin` Content value is supplied through standard input.\n\nRaw Session is the advanced composition surface for Insert/Delete/Move/Copy,\nPosition, Anchor/Data lifetime, explicit bindings, and separate Apply. It is\nnot a prerequisite or alias for ordinary Replace. Its existing raw\n`edit replace` accepts caller-provided exact range Content, including an\nexplicit terminator or multiline replacement, and `apply @edit` publishes it\nseparately. The caller owns the binding, index, quoting, terminator, and\npublication boundary. General replacement should use one-shot body Content\nfirst; no exact one-shot flag or alternate executor exists.\n\n### Shell-local references\n\n`bw shell` also provides a short-lived interactive flow without adding a Core\nwire or persistent identity:\n\n```text\nsearch line needle\nview @0\nreplace @1 replacement\ncheck @2 @3\nexit\n```\n\nQuote one direct-shell query or replacement argument when it contains\nwhitespace; the quotes are shell syntax and are not part of the exact literal\nor replacement Content:\n\n```text\nsearch line \"duplicate = one\"\nreplace @1 \"duplicate = two\"\n```\n\nSuccessful direct `search` and projected direct `view` append `@N` references\nin output order, including duplicates. Direct `check <REF>...` resolves every\nreference before Runtime access, writes one Current/NotCurrent/Unavailable\nstate per input, and appends a fresh slot only for Current. `replace @N <content>` uses the same\ntarget-aware Content rules as one-shot Edit and appends a fresh reference for\n`Unchanged` or `Changed` when the receipt has an Anddress; `Changed\\tNone`\nadds none. Slots are zero-based canonical unsigned decimals, append-only, and\ndiscarded at `exit` or EOF. `@name` and `@name[index]` remain the advanced raw\nSession forms; `let name = @N` explicitly clones a numeric slot into that\nexisting named Anddress binding. Raw `edit replace` and separate `apply @edit`\nremain the advanced exact-range path.\n\nThe default workspace is the process current working directory. An explicit\n`--workspace` must be absolute and is checked by Runtime. Search admits `.` by\ndefault; repeat `--admit LOGICAL_PATH` before `search` to narrow admission.\nAfter the query, repeat `--source LOGICAL_PATH` or `--subtree LOGICAL_PATH` to\nnarrow a Search scope. Without a scope selector, Search covers all admitted\nsources.\n\n## Current CLI scope\n\n`bw` currently implements Adapter-owned one-shot Version and Update, one-shot\nhuman or JSON Search, View, Check, and Edit, raw View, Anddress-first one-shot Edit,\nplus Session Pick, batch Check, Anchor, Edit, Apply, and Data:\n\n```text\nbw version\nbw update\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    view anddress <encoded-v5-Anddress>... [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    check anddress <encoded-v5-Anddress>...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --raw\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    check anddress <encoded-v5-Anddress>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n```\n\nVersion and Update do not call Backwriter Core or Runtime and create no Core\nwire or capability workflow.\n\nContent Search preserves Core literal validation, scope, projection, and\ndeterministic result order. The distinct `search /file` form validates one\nlogical path and returns the current File Anddress for an admitted regular\nUTF-8, NUL-free source regardless of whether it is empty or contains matching\ntext. Missing paths and directories return Empty; the form has no scope\nselectors or synthetic content query. View decodes v5 Anddresses and projects\neach to itself or one ancestor before reading its exact content. A single input\ndefaults to self projection. `--as` chooses one target kind; multiple inputs\nrequire both `--json` and `--as`. Check decodes every v5 operand before Runtime\naccess. One human input writes `Current`, `NotCurrent`, or `Unavailable`; a\nbatch requires `--json` and preserves one outcome per input. Search, View,\nCheck, and Edit `--json`\nwrite compact Adapter objects with exact embedded v5 Anddress objects where\napplicable; each is an Adapter schema, not a second Core wire. View uses the\nhard-cut `bw.cli.view.v2` outcomes array for both single and batch results;\nCheck uses the hard-cut `bw.cli.check.v2` ordered outcomes array.\nRaw View is an explicit Adapter exact-text mode that reuses the ordinary View\nprojection without a Core wire or changed View meaning.\nHuman Search, View, and Check keep their existing projections; human Edit\nreceipts intentionally return the exact current v5 object when one exists.\nThe Session holds one Runtime until EOF\nor `exit` and has explicit local Search, Pick, Anddress, Edit, View, and Check\nbindings plus non-aliasing Anchedress handles. It owns one explicit `DataStore`\nfor the Session only; names are typed and never persist past EOF or `exit`.\nSession Pick\npasses a named Search or Pick collection and an Adapter-parsed predicate directly\nto Core; Session batch Check passes a named matching outcome directly to its\nRuntime batch seam and prints only report counts. Session Anchor creates a live\nhandle only through `let <name> = anchor create <anddress-ref>`, views it through\n`view anchored @<name>`, and can invalidate its logical source with `anchor\ninvalidate-source <logical-path>`. One-shot Data and Anchor are intentionally\nunsupported because their DataStore and live-handle contracts require Session\nlifetime. One-shot Pick, raw Edit-object transport, and a separate\nApply transport await collection or Edit transport schema authority. The\ndistinct Anddress-first one-shot Edit above is implemented. Raw output other\nthan one-shot View and further Session behavior remain deferred.\n\n## Scope\n\nBackwriter is not Git, a file watcher, daemon, persistent index, or editor UI.\nIt reads admitted current Workspace Source through Runtime's safe no-follow\naccess. It does not model branches, merges, history, automatic re-evaluation,\nor editor buffers.\n\nApply uses its accepted current observation and does not coordinate concurrent\nwriters. Writers may race and one publication may overwrite another\nsource-visible change; hosts requiring a stronger guarantee coordinate outside\nBackwriter.\n\n## Build from source and test\n\n```sh\ncargo build --offline --locked --release\n./target/release/bw search line \"needle\"\n./target/release/bw --workspace /path/project search paragraph \"needle\"\ncargo test --offline --locked\n```\n\n## Official desktop distribution\n\nThe official distribution authority is\n[https://backwriter.pentagration.com](https://backwriter.pentagration.com).\nIt publishes the closed Backwriter `0.2.5` release for Linux/WSL x86_64,\nmacOS arm64, macOS x86_64, and Windows x86_64 from Source Authority revision\n`e4022fc073e9df3928e1c3817b266ce92121a03c`. Linux uses canonical target\n`x86_64-unknown-linux-musl`;\n`x86_64-unknown-linux-gnu` remains the local development/test-host target.\nmacOS uses `aarch64-apple-darwin` with minimum macOS 11.0 and\n`x86_64-apple-darwin` with minimum macOS 10.12. The macOS artifacts receive\nstatic cross-build verification but are not claimed to have been executed on a\nnative Mac before publication. Windows uses `x86_64-pc-windows-gnu` and the\ncanonical executable `bw.exe`; its static cross-build verification does not\nclaim native Windows, PowerShell, or CMD execution. Linux arm64 is not currently\nprovided, and no universal host-compatibility claim is made.\n\n`install.sh` reads the canonical manifest, verifies the downloaded artifact\nagainst the manifest SHA-256, and installs the verified binary at\n`$HOME/.local/bin/bw` with a same-directory rename. Concurrent same-user\nHOME mutation is caller-owned. The published `.sha256` sidecar is for manual\nverification and is not installer authority. Windows PowerShell verifies the\nsame manifest authority and exact ZIP, and installs to\n`$HOME\\.local\\bin\\bw.exe` without editing PATH or the PowerShell profile. The\nthree canonical install commands and the canonical `bw update` command are kept\ntogether in [Quick start](#quick-start).\n\nThe CMD command writes `install.cmd` in the current directory and removes it\nafter a successful installation. An existing file with that name is replaced.\n\nThe CRLF `install.cmd` checks `curl.exe` and `powershell.exe`, downloads exactly\nthe canonical `install.ps1` over HTTPS-only TLS 1.2-or-newer transport into a\ncollision-failing `%TEMP%` task directory, delegates all installation meaning,\ncleans the directory, and preserves the child exit code. It owns no manifest,\nSHA-256, ZIP, or installation logic. The Linux-hosted CMD regression is static;\nno native CMD execution is claimed.\nThe distribution provides no\npublisher-authenticity signature or trusted signing identity, background or\nautomatic update, telemetry, `sudo` execution, or automatic `PATH` or\nshell-startup-file change.\nGitHub is a public source and documentation mirror, not the distribution\nauthority. The complete beta.1, beta.2, beta.3, stable `0.1.0`, `0.2.0`, and\n`0.2.1` and `0.2.2` version directories remain unchanged and immutable. The\ncomplete `0.2.3`, `0.2.4`, and `0.2.5` version directories are immutable; the\n`0.2.5` release is closed. The active public tree has exactly 68 files, and\nthe current installers and manifest select `0.2.5`. Any later platform or\nversion requires separate Owner authority. Linux arm64, tags, GitHub Releases,\ncrates.io publication, and background or automatic update remain outside the\ncompleted publication. The earlier `0.2.4` publication added its eight\nversioned files and replaced only the two installers and manifest pointers\nwithout replacing any of the 48 prior versioned files or the CMD Adapter. Its\nsecond publisher run reused all 60 files without metadata change. The `0.2.5`\npublication then added its eight versioned files, replaced the two installers\nand manifest pointers, and reused all 68 files on its second run.\n\n## Architecture\n\n- [Current state](docs/current/now.md)\n- [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)\n- [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)\n- [CLI V1 authority](docs/architecture/backwriter-cli-v1.md)\n- [Verification](docs/development/verification.md)\n\n## License\n\n[MIT License](LICENSE)\n"}
```

````

#### D3 genie-new transcript

````text
# Independent documented-shell evaluation: new CLI

One uninterrupted run. All fixture body reads and edits used the specified bw binary. No oracle, repository source, private documents, or sibling arms inspected. Commands derived from the supplied public guide and Help. No failure or reset occurred.

## Result and decisions

Discovery returned eight Lines across four files. Paragraph View established primary targets @0, @3, @4, @7; the other four Paragraphs said secondary. Full-file View was taken before edits solely to verify byte preservation. Four body-only replacements returned fresh @20–@23; Check reported all Current. Final file Views confirm exactly one ASCII `3` to `5` substitution per file, no length or terminator changes, and unchanged secondary Content. LF, CR, CRLF, and final unterminated Line were preserved.

Exact byte differences, computed solely from bw View output, with zero-based offsets:

```json
[
  {
    "path": "unit-01.txt",
    "bytesBefore": 67,
    "bytesAfter": 67,
    "differences": [
      {
        "offset": 30,
        "before": "3",
        "after": "5"
      }
    ]
  },
  {
    "path": "unit-02.txt",
    "bytesBefore": 67,
    "bytesAfter": 67,
    "differences": [
      {
        "offset": 65,
        "before": "3",
        "after": "5"
      }
    ]
  },
  {
    "path": "unit-03.txt",
    "bytesBefore": 72,
    "bytesAfter": 72,
    "differences": [
      {
        "offset": 31,
        "before": "3",
        "after": "5"
      }
    ]
  },
  {
    "path": "unit-04.txt",
    "bytesBefore": 66,
    "bytesAfter": 66,
    "differences": [
      {
        "offset": 65,
        "before": "3",
        "after": "5"
      }
    ]
  }
]
```

## Metrics

- Product processes: 3 (top-level Help, shell Help, one interactive shell).
- Additional explicitly invoked utility processes: 3 (two public-guide head reads, one stty). Five command-launch calls each use the configured host shell; the interactive launch exec-replaced that host shell with bw. No task-decision script.
- Public bw command counts: Help 2; Search 1; View 3 (8 Paragraph inputs + 4 baseline File inputs + 4 final File inputs); Replace 4; Check 1 (4 inputs); exit 1.
- Capability work: Search 1, View 16 target reads, Check 4 target inputs; 4 successful body-only replacements (Edit/Apply path). No explicit raw Apply/Pick/Anchor/Data command. Internal capability invocation counts were not instrumented.
- Extra Content-only Search queries: 0. One Content query was the required discovery. Extra baseline preservation inspection: one View command / four File inputs.
- Failures: 0; interactive shell final exit 0.
- Elapsed task interaction: approximately 43 seconds (06:52:07 UTC immediately after initial documentation read through 06:52:50 UTC after final verification/exit). Initial Help/guide tool calls preceded the starting timestamp by under one displayed second; reporting/transcript work is excluded.
- Through transcript creation: 10 functions.exec wrapper calls, containing 5 exec_command, 5 write_stdin, 2 clock reads, and 1 apply_patch = 13 nested calls. No further fixture operation is planned.

## Capture limitations

All output strings below are exact JSON-escaped strings returned by execution tooling, preserving CR/LF. Initial Help was copied verbatim from its tool return; initial public-guide output was reconstructed from an identical second public-guide read, recorded below. No body was read by that utility. Execution tooling exposes a combined output field rather than independent stdout/stderr. Interactive execution used a PTY with `stty -echo -onlcr`, suppressing input echo and LF-to-CRLF output translation; stdout and stderr therefore remain merged, and separate exact stderr attribution cannot be proved. No error text appeared. PTY launch and intermediate shell inputs have no per-command OS exit status; the retained process finally exited 0. No hidden command output is omitted.

Working directory for every process: `/tmp/backwriter-gate5.74tHth/genie-new/workspace`.
Interactive launch actual utility argv: `["stty","-echo","-onlcr"]`; product argv: `["/tmp/backwriter-gate5.74tHth/gnu-target/release/bw","shell"]`. Shell Help argv: `["/tmp/backwriter-gate5.74tHth/gnu-target/release/bw","help","shell"]`. Other launch strings map literally to their whitespace-separated argv; all interactive input bytes are recorded below.

## Exact chronological transcript

### Event 1

```json
{
  "cmd": "/tmp/backwriter-gate5.74tHth/gnu-target/release/bw --help",
  "argv": [
    "/tmp/backwriter-gate5.74tHth/gnu-target/release/bw",
    "--help"
  ],
  "result": {
    "exit_code": 0,
    "output": "USAGE\n  bw [GLOBAL OPTIONS] <command> [command options and operands]\n  bw help [<command>]\n\nGLOBAL OPTIONS\n  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.\n  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.\n  --json                     Select JSON output where the command supports it.\n  --raw                      Select raw View output only.\n\nCOMMANDS\n  shell    Reuse short references across search, view, replace, and check.\n  search   Discover current File, Paragraph, or Line Anddresses.\n  view     Read one or more current Anddresses.\n  edit     Replace one current Anddress.\n  check    Check one or more current Anddresses.\n  version  Print the Backwriter version.\n  update   Run the installed-platform updater.\n\nADDITIONAL HELP\n  bw help <command>\n  Advanced topics: pick, anchor, apply, data (raw Session only; no one-shot execution).\n\nGlobal options precede the command. Use bw help shell for ordinary short-ref work.\n"
  }
}
```

### Event 2

```json
{
  "cmd": "head -c 40000 /tmp/backwriter-gate5.74tHth/genie-new/public-guide.md",
  "argv": [
    "head",
    "-c",
    "40000",
    "/tmp/backwriter-gate5.74tHth/genie-new/public-guide.md"
  ],
  "result": {
    "exit_code": 0,
    "output": "# Backwriter\n\nBackwriter is a Rust Core and Runtime for current, structural text work over\nadmitted Workspace Source. It provides target-local File, Paragraph, and Line\naddresses without turning source history or editor state into Core identity.\n\nThe Core capability inventory is Search, View, Pick, Anchor, Check, Edit,\nApply, and Data. The repository currently provides their Rust Core/Runtime\nsurfaces and the canonical `bw` executable's one-shot human and JSON\nSearch/View/Check/Edit, raw View, Anddress-first one-shot Edit, Session Pick, batch\nCheck, Anchor, Edit, Apply, result-binding, explicit Data modes, and\nAdapter-owned Version and Update.\n\n## Quick start\n\nInstall the closed official `0.2.6` release with the command for your\nplatform.\n\nLinux, macOS, or WSL:\n\n```sh\ncurl -fsSL https://backwriter.pentagration.com/install.sh | sh\n```\n\nWindows PowerShell:\n\n```powershell\nirm https://backwriter.pentagration.com/install.ps1 | iex\n```\n\nWindows CMD:\n\n```cmd\ncurl.exe -fsSL https://backwriter.pentagration.com/install.cmd -o install.cmd && call install.cmd && del install.cmd\n```\n\nThe POSIX installer places `bw` at `$HOME/.local/bin/bw`; the PowerShell and\nCMD paths place `bw.exe` at `$HOME\\.local\\bin\\bw.exe`. They do not change\n`PATH`, a shell startup file, the PowerShell profile, or the registry. A fresh\ninstall prints `Installed Backwriter: <version>`; replacing an existing\ndestination prints `Updated Backwriter: <version>`. The executable path and\n`PATH` guidance are printed separately only when the installation directory is\nnot already on `PATH`.\n\n### Version\n\n```sh\nbw version\n```\n\nThe closed official `0.2.6` build prints exactly:\n\n```text\nBackwriter 0.2.6\n```\n\n### Update\n\n```sh\nbw update\n```\n\n`bw update` downloads and delegates to the current official installer. The\ncurrent manifest selects the closed `0.2.6` distribution. The installer\nreads that manifest, verifies the selected artifact, and installs or reinstalls\nthat manifest version only after validation succeeds. It does not run a\nbackground updater or compare release versions. The installer accepts only the\nexact immutable `0.2.5` manifest and current `0.2.6` manifest; `0.2.4`, `0.2.3`, `0.2.2`, `0.2.1`, `0.2.0`, stable\n`0.1.0`, and beta.3 acceptance is retired. The command still performs no\nversion comparison and adds no version guard.\n\nThe product is Backwriter. The source Cargo package and library crate are\n`backwriter` at published and closed `0.2.6`; a source-built `bw version`\nprints `Backwriter 0.2.6`. The sole canonical executable and external Adapter\ncommand are `bw`. There is no `backwriter` binary, alias, or wrapper. The\nofficial installer selects the closed public `0.2.6`\ndistribution.\n\nThe published `0.2.6` Source Authority's `src/**` is byte-identical to its Gate 6\ncandidate `c78e07f242035230e8b071d583491ac633f58d29`; that claim does not cover\nthis later checkout. Unpublished `0.3.0` Gates 1–4 add the private namespace\nboundary, complete direct shell View, discoverable Help and private CLI modules.\nCargo and `bw version` still report `0.2.6`; readiness and release remain pending.\nUse `bw help shell` for short-reference Search/View/Replace/Check and\n`bw help pick`, `bw help anchor`, `bw help apply`, or `bw help data` for advanced\nraw Session topics. These Help topics add no one-shot capability execution.\nR3 release closure is complete: Source Authority\n`09bb6c424081594bd86a95f04345b786ef9b46b6` supplies published and closed\n`0.2.6`. Official artifacts, installers, manifest, and Update now select\n`0.2.6` in the exact 76-file tree. Installers accept only exact `0.2.5` and\n`0.2.6` manifests. Update still performs no version comparison. Production\nRust, Cargo, tests, toolchain, and v5 wire are unchanged by release closure.\n\n## Anddress-first editing\n\nThe default replacement flow is:\n\n1. Run `bw --json search ...`.\n2. Select one occurrence from `occurrences` and pass its exact embedded v5\n   `anddress` object unchanged as one argv value.\n3. Run `bw edit anddress '<opaque-v5-object>' '<new-content>'`.\n\nHuman Search rows are not encoded Anddress values and cannot be Edit input.\nTreat the selected JSON object as opaque: do not interpret or rewrite its hash,\nrange, length, or other fields. File and Paragraph Content is the exact\nreplacement. Line Content is body-only, rejects NUL, CR, and LF, and preserves\nthe None, LF, CR, or CRLF terminator carried by the exact v5 Line. A Line NUL\nis `edit.content_contains_nul`; a Line CR or LF is\n`edit.line_body_contains_terminator`. The latter explains that Backwriter adds\nthe current terminator and that advanced raw Session Edit/Apply owns exact\nextent replacement. Apply alone confirms that source state before publication.\nView or Pick may help a caller select a target; neither View nor Check is\nrequired.\n\nHuman success writes one exact LF-terminated receipt row. `Unchanged` is\nfollowed by the still-current input v5 object; `Changed` is followed by the\nfresh v5 object when the resulting File, Line, or unique Paragraph has one,\nand otherwise by `None`. With leading `--json`, the same result is the compact\nAdapter-only `bw.cli.edit.v1` object with `schema`, `outcome`, and `anddress`\nkeys in that order; `anddress` is the exact v5 object or JSON `null`. Reuse only\nthe address returned by the receipt. A changed Paragraph with `None` requires\nan explicit Search before later target work. Exit `1` is neither a stale-only\nclassification nor proof that source bytes are unchanged, so it must not\ntrigger automatic retry.\n\nOne-shot Content is either one UTF-8 argv value or the exclusive `--stdin`\nselector in that position; stdin is read to EOF after v5 address validation and\nbefore Runtime access. Both forms have the same target-specific Content rules.\nFile/Paragraph CR and LF are exact Content, while Line body Content never\nsilently strips a trailing newline. Literal `--json` and `--raw` remain exact\nContent; a literal `--stdin` Content value is supplied through standard input.\n\nRaw Session is the advanced composition surface for Insert/Delete/Move/Copy,\nPosition, Anchor/Data lifetime, explicit bindings, and separate Apply. It is\nnot a prerequisite or alias for ordinary Replace. Its existing raw\n`edit replace` accepts caller-provided exact range Content, including an\nexplicit terminator or multiline replacement, and `apply @edit` publishes it\nseparately. The caller owns the binding, index, quoting, terminator, and\npublication boundary. General replacement should use one-shot body Content\nfirst; no exact one-shot flag or alternate executor exists.\n\n### Shell-local references\n\n`bw shell` also provides a short-lived interactive flow without adding a Core\nwire or persistent identity:\n\n```text\nsearch line needle\nview @0\nreplace @1 replacement\ncheck @2 @3\nexit\n```\n\nQuote one direct-shell query or replacement argument when it contains\nwhitespace; the quotes are shell syntax and are not part of the exact literal\nor replacement Content:\n\n```text\nsearch line \"duplicate = one\"\nreplace @1 \"duplicate = two\"\n```\n\nSuccessful direct `search` and projected direct `view` append `@N` references\nin output order, including duplicates. Direct `check <REF>...` resolves every\nreference before Runtime access, writes one Current/NotCurrent/Unavailable\nstate per input, and appends a fresh slot only for Current. `replace @N <content>` uses the same\ntarget-aware Content rules as one-shot Edit and appends a fresh reference for\n`Unchanged` or `Changed` when the receipt has an Anddress; `Changed\\tNone`\nadds none. Slots are zero-based canonical unsigned decimals, append-only, and\ndiscarded at `exit` or EOF. `@name` and `@name[index]` remain the advanced raw\nSession forms; `let name = @N` explicitly clones a numeric slot into that\nexisting named Anddress binding. Raw `edit replace` and separate `apply @edit`\nremain the advanced exact-range path.\n\nThe default workspace is the process current working directory. An explicit\n`--workspace` must be absolute and is checked by Runtime. Search admits `.` by\ndefault; repeat `--admit LOGICAL_PATH` before `search` to narrow admission.\nAfter the query, repeat `--source LOGICAL_PATH` or `--subtree LOGICAL_PATH` to\nnarrow a Search scope. Without a scope selector, Search covers all admitted\nsources.\n\n## Current CLI scope\n\n`bw` currently implements Adapter-owned one-shot Version and Update, one-shot\nhuman or JSON Search, View, Check, and Edit, raw View, Anddress-first one-shot Edit,\nplus Session Pick, batch Check, Anchor, Edit, Apply, and Data:\n\n```text\nbw version\nbw update\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    view anddress <encoded-v5-Anddress>... [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    check anddress <encoded-v5-Anddress>...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --raw\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    check anddress <encoded-v5-Anddress>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n```\n\nVersion and Update do not call Backwriter Core or Runtime and create no Core\nwire or capability workflow.\n\nContent Search preserves Core literal validation, scope, projection, and\ndeterministic result order. The distinct `search /file` form validates one\nlogical path and returns the current File Anddress for an admitted regular\nUTF-8, NUL-free source regardless of whether it is empty or contains matching\ntext. Missing paths and directories return Empty; the form has no scope\nselectors or synthetic content query. View decodes v5 Anddresses and projects\neach to itself or one ancestor before reading its exact content. A single input\ndefaults to self projection. `--as` chooses one target kind; multiple inputs\nrequire both `--json` and `--as`. Check decodes every v5 operand before Runtime\naccess. One human input writes `Current`, `NotCurrent`, or `Unavailable`; a\nbatch requires `--json` and preserves one outcome per input. Search, View,\nCheck, and Edit `--json`\nwrite compact Adapter objects with exact embedded v5 Anddress objects where\napplicable; each is an Adapter schema, not a second Core wire. View uses the\nhard-cut `bw.cli.view.v2` outcomes array for both single and batch results;\nCheck uses the hard-cut `bw.cli.check.v2` ordered outcomes array.\nRaw View is an explicit Adapter exact-text mode that reuses the ordinary View\nprojection without a Core wire or changed View meaning.\nHuman Search, View, and Check keep their existing projections; human Edit\nreceipts intentionally return the exact current v5 object when one exists.\nThe Session holds one Runtime until EOF\nor `exit` and has explicit local Search, Pick, Anddress, Edit, View, and Check\nbindings plus non-aliasing Anchedress handles. It owns one explicit `DataStore`\nfor the Session only; names are typed and never persist past EOF or `exit`.\nSession Pick\npasses a named Search or Pick collection and an Adapter-parsed predicate directly\nto Core; Session batch Check passes a named matching outcome directly to its\nRuntime batch seam and prints only report counts. Session Anchor creates a live\nhandle only through `let <name> = anchor create <anddress-ref>`, views it through\n`view anchored @<name>`, and can invalidate its logical source with `anchor\ninvalidate-source <logical-path>`. One-shot Data and Anchor are intentionally\nunsupported because their DataStore and live-handle contracts require Session\nlifetime. One-shot Pick, raw Edit-object transport, and a separate\nApply transport await collection or Edit transport schema authority. The\ndistinct Anddress-first one-shot Edit above is implemented. Raw output other\nthan one-shot View and further Session behavior remain deferred.\n\n## Scope\n\nBackwriter is not Git, a file watcher, daemon, persistent index, or editor UI.\nIt reads admitted current Workspace Source through Runtime's safe no-follow\naccess. It does not model branches, merges, history, automatic re-evaluation,\nor editor buffers.\n\nApply uses its accepted current observation and does not coordinate concurrent\nwriters. Writers may race and one publication may overwrite another\nsource-visible change; hosts requiring a stronger guarantee coordinate outside\nBackwriter.\n\n## Build from source and test\n\n```sh\ncargo build --offline --locked --release\n./target/release/bw search line \"needle\"\n./target/release/bw --workspace /path/project search paragraph \"needle\"\ncargo test --offline --locked\n```\n\n## Official desktop distribution\n\nThe official distribution authority is\n[https://backwriter.pentagration.com](https://backwriter.pentagration.com).\nIt publishes the closed Backwriter `0.2.6` release for Linux/WSL x86_64,\nmacOS arm64, macOS x86_64, and Windows x86_64 from Source Authority revision\n`09bb6c424081594bd86a95f04345b786ef9b46b6`. Linux uses canonical target\n`x86_64-unknown-linux-musl`;\n`x86_64-unknown-linux-gnu` remains the local development/test-host target.\nmacOS uses `aarch64-apple-darwin` with minimum macOS 11.0 and\n`x86_64-apple-darwin` with minimum macOS 10.12. The macOS artifacts receive\nstatic cross-build verification but are not claimed to have been executed on a\nnative Mac before publication. Windows uses `x86_64-pc-windows-gnu` and the\ncanonical executable `bw.exe`; its static cross-build verification does not\nclaim native Windows, PowerShell, or CMD execution. Linux arm64 is not currently\nprovided, and no universal host-compatibility claim is made.\n\n`install.sh` reads the canonical manifest, verifies the downloaded artifact\nagainst the manifest SHA-256, and installs the verified binary at\n`$HOME/.local/bin/bw` with a same-directory rename. Concurrent same-user\nHOME mutation is caller-owned. The published `.sha256` sidecar is for manual\nverification and is not installer authority. Windows PowerShell verifies the\nsame manifest authority and exact ZIP, and installs to\n`$HOME\\.local\\bin\\bw.exe` without editing PATH or the PowerShell profile. The\nthree canonical install commands and the canonical `bw update` command are kept\ntogether in [Quick start](#quick-start).\n\nThe CMD command writes `install.cmd` in the current directory and removes it\nafter a successful installation. An existing file with that name is replaced.\n\nThe CRLF `install.cmd` checks `curl.exe` and `powershell.exe`, downloads exactly\nthe canonical `install.ps1` over HTTPS-only TLS 1.2-or-newer transport into a\ncollision-failing `%TEMP%` task directory, delegates all installation meaning,\ncleans the directory, and preserves the child exit code. It owns no manifest,\nSHA-256, ZIP, or installation logic. The Linux-hosted CMD regression is static;\nno native CMD execution is claimed.\nThe distribution provides no\npublisher-authenticity signature or trusted signing identity, background or\nautomatic update, telemetry, `sudo` execution, or automatic `PATH` or\nshell-startup-file change.\nGitHub is a public source and documentation mirror, not the distribution\nauthority. The complete beta.1, beta.2, beta.3, stable `0.1.0`, `0.2.0`, and\n`0.2.1` and `0.2.2` version directories remain unchanged and immutable. The\ncomplete `0.2.3`, `0.2.4`, `0.2.5`, and `0.2.6` version directories are immutable;\nthe `0.2.6` release is closed. The active public tree has exactly 76 files,\nand the current installers and manifest select `0.2.6`. Any later platform or\nversion requires separate Owner authority. Linux arm64, tags, GitHub Releases,\ncrates.io publication, and background or automatic update remain outside the\ncompleted publication. The earlier `0.2.4` publication added its eight\nversioned files and replaced only the two installers and manifest pointers\nwithout replacing any of the 48 prior versioned files or the CMD Adapter. Its\nsecond publisher run reused all 60 files without metadata change. The `0.2.5`\npublication then added its eight versioned files, replaced the two installers\nand manifest pointers, and reused all 68 files on its second run. R3 adds the\neight `0.2.6` files and replaces the two installers and manifest last; its\nsecond run reuses all 76 files without byte or metadata change.\n\n## Architecture\n\n- [Current state](docs/current/now.md)\n- [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)\n- [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)\n- [CLI V1 authority](docs/architecture/backwriter-cli-v1.md)\n- [Verification](docs/development/verification.md)\n\n## License\n\n[MIT License](LICENSE)\n"
  }
}
```

### Event 3

```json
{
  "cmd": "/tmp/backwriter-gate5.74tHth/gnu-target/release/bw help shell",
  "result": {
    "chunk_id": "4ae29b",
    "wall_time_seconds": 0.000003669,
    "exit_code": 0,
    "original_token_count": 615,
    "output": "NAME\n  bw shell - reuse short references across search, view, replace, and check\n\nUSAGE\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n\nCOMMANDS\n  search <line|paragraph|file> <query> [--source PATH | --subtree PATH]...\n  search /file <logical-path>\n  view <REF>... [--as <line|paragraph|file>]\n  replace <REF> <CONTENT>\n  check <REF>...\n  let <NAME> = <REF>\n  exit\n\nREFERENCES AND CONTENT\n  @N is a zero-based, append-only reference from this process. It ends at EOF or exit.\n  @name is a named Anddress; @hits[0] selects an indexed Search/Pick binding.\n  let name = @N makes a named alias. Names cannot be redefined.\n  Double-quote arguments containing spaces. Quoted escapes are \\\\, \\\", \\n, \\r, and \\t.\n  One physical input line is one command; no single quotes, pipes, or EOF Content selector.\n  Line Replace accepts body only: NUL/CR/LF are rejected; the existing terminator is preserved.\n  File/Paragraph Replace uses exact Content. Changed source bytes stale old same-source refs,\n  including other locations. Use the fresh receipt or explicitly search again; never auto-retry.\n\nOUTPUT\n  Search appends one @N per result; Empty writes nothing.\n  View returns each input ref, a fresh ref, kind/location, and complete Content in input order.\n  View<TAB>REF<TAB>bytes=N starts a record, followed by fresh-ref metadata and N exact bytes.\n  The following LF and EndView line are display framing, not source Content.\n  RelationAbsent is shown at its input position without Content or a fresh slot.\n  Replace writes Unchanged/Changed with a fresh ref, or Changed<TAB>None without one.\n  Check writes one status per input; only Current appends a fresh ref. Check is optional.\n\nEXAMPLES\n  Start bw shell in a workspace with only note.txt containing needle plus CRLF, then enter:\n  search line needle --source note.txt\n  view @0 --as paragraph\n  let selected = @0\n  replace @selected \"new value\"\n  check @0 @2\n  view @3\n  exit\n\nADVANCED\n  let hits = search line needle\n  view anddress @hits[0]\n  Raw named View writes exact Content without direct-View framing.\n  See bw help pick, bw help anchor, bw help apply, and bw help data.\n\nFAILURES\n  Usage errors record exit 2; Runtime/source errors record exit 1; later commands still run.\n  Stdin/stdout failure ends the shell with exit 1. Partial output cannot prove delivery or\n  undo publication. --json/--raw and command-line operands are unavailable.\n\nSEE ALSO\n  bw help search\n  bw help edit\n"
  }
}
```

### Event 4

```json
{
  "cmd": "stty -echo -onlcr; exec /tmp/backwriter-gate5.74tHth/gnu-target/release/bw shell",
  "result": {
    "chunk_id": "8944b6",
    "wall_time_seconds": 1.00114906,
    "session_id": 20127,
    "original_token_count": 0,
    "output": ""
  }
}
```

### Event 5

```json
{
  "stdin": "search line \"retry_budget = 3\"\n",
  "result": {
    "chunk_id": "1f64a3",
    "wall_time_seconds": 1.000797233,
    "session_id": 20127,
    "original_token_count": 44,
    "output": "@0\tLine\tunit-01.txt:2\n@1\tLine\tunit-01.txt:5\n@2\tLine\tunit-02.txt:2\n@3\tLine\tunit-02.txt:5\n@4\tLine\tunit-03.txt:2\n@5\tLine\tunit-03.txt:5\n@6\tLine\tunit-04.txt:2\n@7\tLine\tunit-04.txt:5\n"
  }
}
```

### Event 6

```json
{
  "stdin": "view @0 @1 @2 @3 @4 @5 @6 @7 --as paragraph\n",
  "result": {
    "chunk_id": "0f889f",
    "wall_time_seconds": 1.000483504,
    "session_id": 20127,
    "original_token_count": 179,
    "output": "View\t@0\tbytes=32\n@8\tParagraph\tunit-01.txt:1-2\nrole = primary\nretry_budget = 3\n\nEndView\nView\t@1\tbytes=34\n@9\tParagraph\tunit-01.txt:4-5\nrole = secondary\nretry_budget = 3\n\nEndView\nView\t@2\tbytes=34\n@10\tParagraph\tunit-02.txt:1-2\nrole = secondary\rretry_budget = 3\r\nEndView\nView\t@3\tbytes=32\n@11\tParagraph\tunit-02.txt:4-5\nrole = primary\rretry_budget = 3\r\nEndView\nView\t@4\tbytes=34\n@12\tParagraph\tunit-03.txt:1-2\nrole = primary\r\nretry_budget = 3\r\n\nEndView\nView\t@5\tbytes=36\n@13\tParagraph\tunit-03.txt:4-5\nrole = secondary\r\nretry_budget = 3\r\n\nEndView\nView\t@6\tbytes=34\n@14\tParagraph\tunit-04.txt:1-2\nrole = secondary\nretry_budget = 3\n\nEndView\nView\t@7\tbytes=31\n@15\tParagraph\tunit-04.txt:4-5\nrole = primary\nretry_budget = 3\nEndView\n"
  }
}
```

### Event 7

```json
{
  "stdin": "view @0 @2 @4 @6 --as file\n",
  "result": {
    "chunk_id": "f805a1",
    "wall_time_seconds": 1.001154642,
    "session_id": 20127,
    "original_token_count": 115,
    "output": "View\t@0\tbytes=67\n@16\tFile\tunit-01.txt\nrole = primary\nretry_budget = 3\n\nrole = secondary\nretry_budget = 3\n\nEndView\nView\t@2\tbytes=67\n@17\tFile\tunit-02.txt\nrole = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 3\r\nEndView\nView\t@4\tbytes=72\n@18\tFile\tunit-03.txt\nrole = primary\r\nretry_budget = 3\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n\nEndView\nView\t@6\tbytes=66\n@19\tFile\tunit-04.txt\nrole = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 3\nEndView\n"
  }
}
```

### Event 8

```json
{
  "stdin": "replace @0 \"retry_budget = 5\"\nreplace @3 \"retry_budget = 5\"\nreplace @4 \"retry_budget = 5\"\nreplace @7 \"retry_budget = 5\"\n",
  "result": {
    "chunk_id": "9db1ae",
    "wall_time_seconds": 1.000935374,
    "session_id": 20127,
    "original_token_count": 31,
    "output": "@20\tChanged\tLine\tunit-01.txt:2\n@21\tChanged\tLine\tunit-02.txt:5\n@22\tChanged\tLine\tunit-03.txt:2\n@23\tChanged\tLine\tunit-04.txt:5\n"
  }
}
```

### Event 9

```json
{
  "stdin": "check @20 @21 @22 @23\nview @20 @21 @22 @23 --as file\nexit\n",
  "result": {
    "chunk_id": "4e2aac",
    "wall_time_seconds": 0.000003668,
    "exit_code": 0,
    "original_token_count": 147,
    "output": "@24\tCurrent\tLine\tunit-01.txt:2\n@25\tCurrent\tLine\tunit-02.txt:5\n@26\tCurrent\tLine\tunit-03.txt:2\n@27\tCurrent\tLine\tunit-04.txt:5\nView\t@20\tbytes=67\n@28\tFile\tunit-01.txt\nrole = primary\nretry_budget = 5\n\nrole = secondary\nretry_budget = 3\n\nEndView\nView\t@21\tbytes=67\n@29\tFile\tunit-02.txt\nrole = secondary\rretry_budget = 3\r\rrole = primary\rretry_budget = 5\r\nEndView\nView\t@22\tbytes=72\n@30\tFile\tunit-03.txt\nrole = primary\r\nretry_budget = 5\r\n\r\nrole = secondary\r\nretry_budget = 3\r\n\nEndView\nView\t@23\tbytes=66\n@31\tFile\tunit-04.txt\nrole = secondary\nretry_budget = 3\n\nrole = primary\nretry_budget = 5\nEndView\n"
  }
}
```

### Event 10

```json
{
  "cmd": "head -c 40000 /tmp/backwriter-gate5.74tHth/genie-new/public-guide.md",
  "note": "Re-read public guide only to preserve the identical initial documentation stdout in transcript; no fixture read.",
  "result": {
    "chunk_id": "d67c14",
    "wall_time_seconds": 0.000005018,
    "exit_code": 0,
    "original_token_count": 4381,
    "output": "# Backwriter\n\nBackwriter is a Rust Core and Runtime for current, structural text work over\nadmitted Workspace Source. It provides target-local File, Paragraph, and Line\naddresses without turning source history or editor state into Core identity.\n\nThe Core capability inventory is Search, View, Pick, Anchor, Check, Edit,\nApply, and Data. The repository currently provides their Rust Core/Runtime\nsurfaces and the canonical `bw` executable's one-shot human and JSON\nSearch/View/Check/Edit, raw View, Anddress-first one-shot Edit, Session Pick, batch\nCheck, Anchor, Edit, Apply, result-binding, explicit Data modes, and\nAdapter-owned Version and Update.\n\n## Quick start\n\nInstall the closed official `0.2.6` release with the command for your\nplatform.\n\nLinux, macOS, or WSL:\n\n```sh\ncurl -fsSL https://backwriter.pentagration.com/install.sh | sh\n```\n\nWindows PowerShell:\n\n```powershell\nirm https://backwriter.pentagration.com/install.ps1 | iex\n```\n\nWindows CMD:\n\n```cmd\ncurl.exe -fsSL https://backwriter.pentagration.com/install.cmd -o install.cmd && call install.cmd && del install.cmd\n```\n\nThe POSIX installer places `bw` at `$HOME/.local/bin/bw`; the PowerShell and\nCMD paths place `bw.exe` at `$HOME\\.local\\bin\\bw.exe`. They do not change\n`PATH`, a shell startup file, the PowerShell profile, or the registry. A fresh\ninstall prints `Installed Backwriter: <version>`; replacing an existing\ndestination prints `Updated Backwriter: <version>`. The executable path and\n`PATH` guidance are printed separately only when the installation directory is\nnot already on `PATH`.\n\n### Version\n\n```sh\nbw version\n```\n\nThe closed official `0.2.6` build prints exactly:\n\n```text\nBackwriter 0.2.6\n```\n\n### Update\n\n```sh\nbw update\n```\n\n`bw update` downloads and delegates to the current official installer. The\ncurrent manifest selects the closed `0.2.6` distribution. The installer\nreads that manifest, verifies the selected artifact, and installs or reinstalls\nthat manifest version only after validation succeeds. It does not run a\nbackground updater or compare release versions. The installer accepts only the\nexact immutable `0.2.5` manifest and current `0.2.6` manifest; `0.2.4`, `0.2.3`, `0.2.2`, `0.2.1`, `0.2.0`, stable\n`0.1.0`, and beta.3 acceptance is retired. The command still performs no\nversion comparison and adds no version guard.\n\nThe product is Backwriter. The source Cargo package and library crate are\n`backwriter` at published and closed `0.2.6`; a source-built `bw version`\nprints `Backwriter 0.2.6`. The sole canonical executable and external Adapter\ncommand are `bw`. There is no `backwriter` binary, alias, or wrapper. The\nofficial installer selects the closed public `0.2.6`\ndistribution.\n\nThe published `0.2.6` Source Authority's `src/**` is byte-identical to its Gate 6\ncandidate `c78e07f242035230e8b071d583491ac633f58d29`; that claim does not cover\nthis later checkout. Unpublished `0.3.0` Gates 1–4 add the private namespace\nboundary, complete direct shell View, discoverable Help and private CLI modules.\nCargo and `bw version` still report `0.2.6`; readiness and release remain pending.\nUse `bw help shell` for short-reference Search/View/Replace/Check and\n`bw help pick`, `bw help anchor`, `bw help apply`, or `bw help data` for advanced\nraw Session topics. These Help topics add no one-shot capability execution.\nR3 release closure is complete: Source Authority\n`09bb6c424081594bd86a95f04345b786ef9b46b6` supplies published and closed\n`0.2.6`. Official artifacts, installers, manifest, and Update now select\n`0.2.6` in the exact 76-file tree. Installers accept only exact `0.2.5` and\n`0.2.6` manifests. Update still performs no version comparison. Production\nRust, Cargo, tests, toolchain, and v5 wire are unchanged by release closure.\n\n## Anddress-first editing\n\nThe default replacement flow is:\n\n1. Run `bw --json search ...`.\n2. Select one occurrence from `occurrences` and pass its exact embedded v5\n   `anddress` object unchanged as one argv value.\n3. Run `bw edit anddress '<opaque-v5-object>' '<new-content>'`.\n\nHuman Search rows are not encoded Anddress values and cannot be Edit input.\nTreat the selected JSON object as opaque: do not interpret or rewrite its hash,\nrange, length, or other fields. File and Paragraph Content is the exact\nreplacement. Line Content is body-only, rejects NUL, CR, and LF, and preserves\nthe None, LF, CR, or CRLF terminator carried by the exact v5 Line. A Line NUL\nis `edit.content_contains_nul`; a Line CR or LF is\n`edit.line_body_contains_terminator`. The latter explains that Backwriter adds\nthe current terminator and that advanced raw Session Edit/Apply owns exact\nextent replacement. Apply alone confirms that source state before publication.\nView or Pick may help a caller select a target; neither View nor Check is\nrequired.\n\nHuman success writes one exact LF-terminated receipt row. `Unchanged` is\nfollowed by the still-current input v5 object; `Changed` is followed by the\nfresh v5 object when the resulting File, Line, or unique Paragraph has one,\nand otherwise by `None`. With leading `--json`, the same result is the compact\nAdapter-only `bw.cli.edit.v1` object with `schema`, `outcome`, and `anddress`\nkeys in that order; `anddress` is the exact v5 object or JSON `null`. Reuse only\nthe address returned by the receipt. A changed Paragraph with `None` requires\nan explicit Search before later target work. Exit `1` is neither a stale-only\nclassification nor proof that source bytes are unchanged, so it must not\ntrigger automatic retry.\n\nOne-shot Content is either one UTF-8 argv value or the exclusive `--stdin`\nselector in that position; stdin is read to EOF after v5 address validation and\nbefore Runtime access. Both forms have the same target-specific Content rules.\nFile/Paragraph CR and LF are exact Content, while Line body Content never\nsilently strips a trailing newline. Literal `--json` and `--raw` remain exact\nContent; a literal `--stdin` Content value is supplied through standard input.\n\nRaw Session is the advanced composition surface for Insert/Delete/Move/Copy,\nPosition, Anchor/Data lifetime, explicit bindings, and separate Apply. It is\nnot a prerequisite or alias for ordinary Replace. Its existing raw\n`edit replace` accepts caller-provided exact range Content, including an\nexplicit terminator or multiline replacement, and `apply @edit` publishes it\nseparately. The caller owns the binding, index, quoting, terminator, and\npublication boundary. General replacement should use one-shot body Content\nfirst; no exact one-shot flag or alternate executor exists.\n\n### Shell-local references\n\n`bw shell` also provides a short-lived interactive flow without adding a Core\nwire or persistent identity:\n\n```text\nsearch line needle\nview @0\nreplace @1 replacement\ncheck @2 @3\nexit\n```\n\nQuote one direct-shell query or replacement argument when it contains\nwhitespace; the quotes are shell syntax and are not part of the exact literal\nor replacement Content:\n\n```text\nsearch line \"duplicate = one\"\nreplace @1 \"duplicate = two\"\n```\n\nSuccessful direct `search` and projected direct `view` append `@N` references\nin output order, including duplicates. Direct `check <REF>...` resolves every\nreference before Runtime access, writes one Current/NotCurrent/Unavailable\nstate per input, and appends a fresh slot only for Current. `replace @N <content>` uses the same\ntarget-aware Content rules as one-shot Edit and appends a fresh reference for\n`Unchanged` or `Changed` when the receipt has an Anddress; `Changed\\tNone`\nadds none. Slots are zero-based canonical unsigned decimals, append-only, and\ndiscarded at `exit` or EOF. `@name` and `@name[index]` remain the advanced raw\nSession forms; `let name = @N` explicitly clones a numeric slot into that\nexisting named Anddress binding. Raw `edit replace` and separate `apply @edit`\nremain the advanced exact-range path.\n\nThe default workspace is the process current working directory. An explicit\n`--workspace` must be absolute and is checked by Runtime. Search admits `.` by\ndefault; repeat `--admit LOGICAL_PATH` before `search` to narrow admission.\nAfter the query, repeat `--source LOGICAL_PATH` or `--subtree LOGICAL_PATH` to\nnarrow a Search scope. Without a scope selector, Search covers all admitted\nsources.\n\n## Current CLI scope\n\n`bw` currently implements Adapter-owned one-shot Version and Update, one-shot\nhuman or JSON Search, View, Check, and Edit, raw View, Anddress-first one-shot Edit,\nplus Session Pick, batch Check, Anchor, Edit, Apply, and Data:\n\n```text\nbw version\nbw update\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search <line|paragraph|file> <query>\n    [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    search /file <logical-path>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    view anddress <encoded-v5-Anddress>... [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    check anddress <encoded-v5-Anddress>...\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --raw\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    check anddress <encoded-v5-Anddress>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]...\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> <content>\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json\n    edit anddress <encoded-v5-Anddress> --stdin\nbw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n```\n\nVersion and Update do not call Backwriter Core or Runtime and create no Core\nwire or capability workflow.\n\nContent Search preserves Core literal validation, scope, projection, and\ndeterministic result order. The distinct `search /file` form validates one\nlogical path and returns the current File Anddress for an admitted regular\nUTF-8, NUL-free source regardless of whether it is empty or contains matching\ntext. Missing paths and directories return Empty; the form has no scope\nselectors or synthetic content query. View decodes v5 Anddresses and projects\neach to itself or one ancestor before reading its exact content. A single input\ndefaults to self projection. `--as` chooses one target kind; multiple inputs\nrequire both `--json` and `--as`. Check decodes every v5 operand before Runtime\naccess. One human input writes `Current`, `NotCurrent`, or `Unavailable`; a\nbatch requires `--json` and preserves one outcome per input. Search, View,\nCheck, and Edit `--json`\nwrite compact Adapter objects with exact embedded v5 Anddress objects where\napplicable; each is an Adapter schema, not a second Core wire. View uses the\nhard-cut `bw.cli.view.v2` outcomes array for both single and batch results;\nCheck uses the hard-cut `bw.cli.check.v2` ordered outcomes array.\nRaw View is an explicit Adapter exact-text mode that reuses the ordinary View\nprojection without a Core wire or changed View meaning.\nHuman Search, View, and Check keep their existing projections; human Edit\nreceipts intentionally return the exact current v5 object when one exists.\nThe Session holds one Runtime until EOF\nor `exit` and has explicit local Search, Pick, Anddress, Edit, View, and Check\nbindings plus non-aliasing Anchedress handles. It owns one explicit `DataStore`\nfor the Session only; names are typed and never persist past EOF or `exit`.\nSession Pick\npasses a named Search or Pick collection and an Adapter-parsed predicate directly\nto Core; Session batch Check passes a named matching outcome directly to its\nRuntime batch seam and prints only report counts. Session Anchor creates a live\nhandle only through `let <name> = anchor create <anddress-ref>`, views it through\n`view anchored @<name>`, and can invalidate its logical source with `anchor\ninvalidate-source <logical-path>`. One-shot Data and Anchor are intentionally\nunsupported because their DataStore and live-handle contracts require Session\nlifetime. One-shot Pick, raw Edit-object transport, and a separate\nApply transport await collection or Edit transport schema authority. The\ndistinct Anddress-first one-shot Edit above is implemented. Raw output other\nthan one-shot View and further Session behavior remain deferred.\n\n## Scope\n\nBackwriter is not Git, a file watcher, daemon, persistent index, or editor UI.\nIt reads admitted current Workspace Source through Runtime's safe no-follow\naccess. It does not model branches, merges, history, automatic re-evaluation,\nor editor buffers.\n\nApply uses its accepted current observation and does not coordinate concurrent\nwriters. Writers may race and one publication may overwrite another\nsource-visible change; hosts requiring a stronger guarantee coordinate outside\nBackwriter.\n\n## Build from source and test\n\n```sh\ncargo build --offline --locked --release\n./target/release/bw search line \"needle\"\n./target/release/bw --workspace /path/project search paragraph \"needle\"\ncargo test --offline --locked\n```\n\n## Official desktop distribution\n\nThe official distribution authority is\n[https://backwriter.pentagration.com](https://backwriter.pentagration.com).\nIt publishes the closed Backwriter `0.2.6` release for Linux/WSL x86_64,\nmacOS arm64, macOS x86_64, and Windows x86_64 from Source Authority revision\n`09bb6c424081594bd86a95f04345b786ef9b46b6`. Linux uses canonical target\n`x86_64-unknown-linux-musl`;\n`x86_64-unknown-linux-gnu` remains the local development/test-host target.\nmacOS uses `aarch64-apple-darwin` with minimum macOS 11.0 and\n`x86_64-apple-darwin` with minimum macOS 10.12. The macOS artifacts receive\nstatic cross-build verification but are not claimed to have been executed on a\nnative Mac before publication. Windows uses `x86_64-pc-windows-gnu` and the\ncanonical executable `bw.exe`; its static cross-build verification does not\nclaim native Windows, PowerShell, or CMD execution. Linux arm64 is not currently\nprovided, and no universal host-compatibility claim is made.\n\n`install.sh` reads the canonical manifest, verifies the downloaded artifact\nagainst the manifest SHA-256, and installs the verified binary at\n`$HOME/.local/bin/bw` with a same-directory rename. Concurrent same-user\nHOME mutation is caller-owned. The published `.sha256` sidecar is for manual\nverification and is not installer authority. Windows PowerShell verifies the\nsame manifest authority and exact ZIP, and installs to\n`$HOME\\.local\\bin\\bw.exe` without editing PATH or the PowerShell profile. The\nthree canonical install commands and the canonical `bw update` command are kept\ntogether in [Quick start](#quick-start).\n\nThe CMD command writes `install.cmd` in the current directory and removes it\nafter a successful installation. An existing file with that name is replaced.\n\nThe CRLF `install.cmd` checks `curl.exe` and `powershell.exe`, downloads exactly\nthe canonical `install.ps1` over HTTPS-only TLS 1.2-or-newer transport into a\ncollision-failing `%TEMP%` task directory, delegates all installation meaning,\ncleans the directory, and preserves the child exit code. It owns no manifest,\nSHA-256, ZIP, or installation logic. The Linux-hosted CMD regression is static;\nno native CMD execution is claimed.\nThe distribution provides no\npublisher-authenticity signature or trusted signing identity, background or\nautomatic update, telemetry, `sudo` execution, or automatic `PATH` or\nshell-startup-file change.\nGitHub is a public source and documentation mirror, not the distribution\nauthority. The complete beta.1, beta.2, beta.3, stable `0.1.0`, `0.2.0`, and\n`0.2.1` and `0.2.2` version directories remain unchanged and immutable. The\ncomplete `0.2.3`, `0.2.4`, `0.2.5`, and `0.2.6` version directories are immutable;\nthe `0.2.6` release is closed. The active public tree has exactly 76 files,\nand the current installers and manifest select `0.2.6`. Any later platform or\nversion requires separate Owner authority. Linux arm64, tags, GitHub Releases,\ncrates.io publication, and background or automatic update remain outside the\ncompleted publication. The earlier `0.2.4` publication added its eight\nversioned files and replaced only the two installers and manifest pointers\nwithout replacing any of the 48 prior versioned files or the CMD Adapter. Its\nsecond publisher run reused all 60 files without metadata change. The `0.2.5`\npublication then added its eight versioned files, replaced the two installers\nand manifest pointers, and reused all 68 files on its second run. R3 adds the\neight `0.2.6` files and replaces the two installers and manifest last; its\nsecond run reuses all 76 files without byte or metadata change.\n\n## Architecture\n\n- [Current state](docs/current/now.md)\n- [Backwriter protocol](docs/architecture/backwriter-text-coordination-protocol.md)\n- [Anddress and exact Line model](docs/architecture/rebuildable-structural-addressing.md)\n- [CLI V1 authority](docs/architecture/backwriter-cli-v1.md)\n- [Verification](docs/development/verification.md)\n\n## License\n\n[MIT License](LICENSE)\n"
  }
}
```


````
