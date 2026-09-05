# Verification

## 0.3.0 verification policy — Gate 1 only

The [five-gate tracker](../tasks/2026-09-05-backwriter-0.3.0-independent-namespace-complete-view.md)
owns gate evidence and pending acceptance, not competing semantic authority.
Use focused namespace, shell View, help and module-wiring tests during Gates
2–4. Run the complete GNU/musl matrix at the final stable Gate 5 candidate,
including offline/locked metadata/tree, fmt, all-target check, tests, clippy
with warnings denied, release builds and release Help/Version/capability smoke.
Do not default to a full two-target rerun at every small development gate.
This policy governs `0.3.0` over the older blanket run-after-change guidance.

Reuse requires equality of production, tests and fixtures, build scripts,
Cargo/lock, toolchain, target, features, profile and relevant flags. Record run,
reused and unexecuted evidence separately. Version/build-metadata changes need
binary identity and Version KAT checks, not repetition of historical benchmarks.
G1 verifies documentation hygiene, offline/locked metadata and unchanged inputs;
it reuses the existing 285-test GNU and 285-test musl results, not a new run.

Gate 2 uses task-local absent/new/old-only/both-root fixtures and unchanged old
sentinels, private file/symlink cases, exact component and Windows case
boundaries, ordinary siblings/nested paths, no-follow failures and help/version
noncreation. Store creation/read/write/spill/cleanup are N/A unless real
consumers are found; never add IO to satisfy BOX 23. Do not inspect actual old
state. Gate 3 freezes exact framing/ref/Content KATs, empty/Unicode and every
terminator, delimiter-like Content, ordered duplicates, mixed projected/absent
results, allocation/Runtime/stdout failures, and one single or batch call with
zero Content-only Search/View/re-resolution. Mixed-kind plural self-View needs
its explicit prerequisite decision before that acceptance can be claimed.

Gate 4 preserves still-current rules before replacing duplicated history with
links; unique numbers, environments, path spellings and source revisions must
have a preserved evidence location before removal. It adds no historical index
at Gate 1. Conditional test modules retain one CLI integration crate and all
distinct structural/behavioral coverage. No speed claim follows from relocation.

Gate 5 compares only pinned N-1 `0.2.6` Source Authority
`09bb6c424081594bd86a95f04345b786ef9b46b6` and the actual N candidate. Locate
the original BOX 25–26 independent four-file fixture first; if unavailable,
declare a new shared fixture digest and independently defined full-byte oracle.
It must have eight duplicate Lines, four primary edits (one per file), four
untouched secondary Lines and LF/CR/CRLF/None. Run Dummy N-1/N and Genie N-1/N
as four arms. Dummy chooses from public help freely; one-shot is not failure.
Genie's reference is Search 1 + Paragraph batch View 1 + Replace 4 + Check
batch 1 + File batch View 1: eight capability commands, excluding help and
shell lifecycle. Candidate first context View must supply decision-making
Content, with zero Content-only self-View/named-copy/raw-View workaround,
zero terminator mistakes/Wrong Apply and exact oracle. Record N-1 deficiencies
honestly rather than changing the candidate acceptance or extending this
one-edit-per-file flow to repeated edits in the same source.

Keep process count, actual commands, model tool turns, unexpected CLI failures,
Content-only extra commands, stdout/stderr bytes, model-visible bytes and elapsed
separate. More returned Content is not itself regression. Each n=1 arm supplies
observations, not a mandatory timing ratio or broad performance claim. No
`0.2.5` or older execution, external-tool comparison arm, user `bw 0.2.2`
execution/update, suite/benchmark repetition at G1, or release authority follows.
Existing native-platform gaps and lock/rollback/fsync/crash-durability limits
remain. G1 does not alter the historical evidence below.

## 0.2.6 operational Adapter and verification-contraction authority

Gates 1–6 retain one test authority for exact Help/usage/JSON KATs, stdin and
Line terminators, shell references/Replace, ordered Check, raw Session, and the
inherited v5/Search/currentness/publication/Host-proof/Anchor boundaries. GNU
and musl each pass 285 tests; `Correct 1 / Safe Reject 6 / Wrong Apply 0` is
unchanged. Gate 8 reruns the complete GNU and musl suites at 285 tests each and
is GO: only root Cargo/lock version, the exact Version KAT, and active status
advance to source-ready unpublished `0.2.6`. Production Rust remains
byte-identical to Gate 6; at that source gate official distribution remained
`0.2.5`. R3 below closes the official `0.2.6` release.

Gate 7 executes candidate `c78e07f242035230e8b071d583491ac633f58d29` only
against clean exported N-1 `a9b47b06e0c4ac4c3058332f85a2885f47edd53a`.
Blind Dummy, public-only Genie, and external `grep`/`cat`/`sed`/Git controls
close the duplicate/Paragraph, Line-body, fresh-reference, ordered-Check, and
stale-precondition evidence. The final Genie flow uses Search once, batch View,
Replace, immediate fresh-ref Check, and final View with no raw-v5 or raw
Edit/Apply operation. Its final fixture SHA-256 is
`084d54d2f243db7d40c11e841f57e00bcf862e41bc5e4af1ef474dedc30c5adc`.

This active section now records only current controls and Gate 7 results.
Detailed task evidence remains in the tracker and older task records; N-2 and
older releases are not executed without an explicit recovery or migration need.

## 0.2.5 performance-recovery gates

Gate 1 is documentation-only. Gates 2 through 7 replace the sole literal
matcher's per-byte Runtime caller loop with one checked segment operation and
move raw consumers without structural geometry onto a cursor-free observation
path, consolidate canonical Anddress encoding, and release dense pending storage.
Gates 2–6 leave Cargo, lockfile, README, toolchain, v5 wire, and Adapter output
unchanged; Gate 7 advances only source version authority after GO. The
[tracker](../tasks/2026-09-04-backwriter-0.2.5-structural-specialization-performance-recovery.md)
fixes evidence labels, gate order, thresholds, exclusions, and the three
authority decisions before implementation. Gate 2 passed 258 tests per target;
Gate 3 passed 261, Gate 4 passed 263, and Gate 5 passes 268 per target.
All-target check,
clippy with warnings denied, release build, and offline/locked metadata and
tree plus rustfmt also pass. Gates 6 and 7 retain the 268-test count per target.

Line count remains v5 identity and currentness evidence. Gate 3 retains the
same-hash/same-length/false-Line-count `NotCurrent` control while proving that
raw consumers use one minimal same-read Line counter and zero
`StructuralCursor` work. Strict decode, public `validate()`, and error
priority remain. The sole Issuer validates shared source once and each target
geometry once; typed View, Check, and Anchor do not repeat that source-less
validation. Gate 4 proves that `encode_into` clears its buffer, reserves
fallibly before output, leaves length zero on error, reuses capacity, and emits
the exact existing File, Paragraph, Paragraph-parent Line, and File-parent Line
KAT bytes. Existing `encode()` delegates and remains byte-exact. Search and
batch View structurally own one reusable operation-local scratch, while
single-address Edit and Check retain their distinct one-address encoder use.

Gate-specific evidence is cumulative:

- Gate 2: exhaustive byte/segment matcher parity plus fixed 256 MiB, 1 GiB,
  and 1,048,576-hit native Search cells.
- Gate 3: raw/structural parity and failure boundaries; Host/Untrusted Check,
  View, Range Apply, CRLF Edit, and 134-million-short-Line cells.
- Gate 4: one and million-address allocation/latency/output measurements,
  writer equality, million-result CLI Search, and 200,000-file Search/View.
- Gate 5: exact dense count/order/digest, all-or-none cleanup, one huge
  Paragraph, many one-Line Paragraphs, and actual release of pending capacity.
- Gate 6: one cursor, one Issuer, no retired carrier/relation/private View, no
  duplicate parser/validator/writer, and code-size contraction.
- Gate 7: complete GNU/musl semantics and fixed A/B/G performance, memory, I/O,
  output, drift, and code-size evidence; source-readiness is GO.
- Gate 8: artifacts, installers, manifest-last publication, endpoints, update,
  idempotent reuse, and release closure — complete. At that closure the live root had 68
  regular files; loopback and public HTTPS each passed 68 exact GET/HEAD body,
  length, MIME, cache-policy, and zero-HEAD-body checks. Isolated fresh install,
  public `0.2.4` update, and `0.2.5` reinstall selected the exact Linux member.

Fixed gates are G/A sparse target at most 1.10 and ceiling 1.15; 1,048,576-hit
RSS target at most 130 MiB, soft at most 140 MiB, and hard NO-GO above 145 MiB;
CRLF Edit G/A target at most 1.20 and hard ceiling 1.25; Host Check zero I/O;
Untrusted Check, View, and Range Apply boundaries recorded in the tracker;
exact KAT/output/order and Correct 1 / Safe Reject 6 / Wrong Apply 0; and final
production no larger than 297,269 bytes/8,954 lines unless growth up to 3
percent has direct evidence. Any duplicate parser, validator, or writer is a
hard NO-GO. Native and CLI Search measurements remain separate.

Gate 7 runs on Linux 7.2.2-arch1-1 x86_64, Intel i7-12700K CPU 0 with the
existing `powersave` governor, `/tmp` tmpfs, `CLOCK_MONOTONIC_RAW`, Rust/Cargo
1.95.0, one warm-up, and seven balanced crossed A/B/G fresh processes. The
256 MiB and 1 GiB sparse G/A median/p95 ratios are 1.0983/1.1163 and
1.0959/1.0984; the first p95 misses the 1.10 target but passes the 1.15 hard
ceiling. The two 1,048,576-result G shapes peak at 87,924/87,992 KiB with exact
B/G v5 output. CRLF Edit is 1.0062/1.0324; Untrusted Check is
1.0804/1.0689. An independent full Apply confirmation puts unit, receipt, and
live-Anchor G/A median/p95 at 1.0456/1.0425, 1.0481/1.0416, and
1.0506/1.0376 after one initial receipt p95 outlier.

Reusable G encoding records zero loop allocations for a repeated Line,
1,048,576 Lines, and 1,000,000 Files and matches B's canonical v5 digests.
The 200,000-file Search order and batch/sequential View digest are exact. Host
Check retains zero capability I/O and the approximately one-microsecond class;
the 134,217,728-Line Untrusted Check G/A result is 1.0733/1.0710. The fixed AI
flow is Search 1, batch Line-to-Paragraph View 1, receipt Edit 2, final View 1,
with post-Edit Search, mandatory Check, history, relocation, and retry all
zero. GNU and musl each pass 268 tests; drift remains Correct 1 / Safe Reject
6 / Wrong Apply 0. Production stays byte-identical to G at 304,431 bytes and
9,213 lines. Exact fixtures, commands, samples, digests, I/O, HWM, and
harness/raw-evidence hashes are recorded in the tracker.

Gate 2 exhaustively compares every binary query through length four and content
through length six across byte-at-a-time, every segment partition, and whole
segments. Explicit cases cover missing and terminal first bytes, one-byte and
overlapping literals, carried KMP partials, Unicode splits, exact/substring
tiers, long suffixes, CR/LF/CRLF/no-EOL, and 8,191/8,192/8,193-byte boundaries
for File, Paragraph, and Line. Checked length overflow fails closed. Structural
tests confirm one matcher segment call and no per-byte caller loop; File and
Paragraph `FullLine` saturation does not weaken late UTF-8/NUL/read failure.

The fixed native harness uses A=`195aaa37068122097ecc04d2644642b6afcc6765`,
B=`8b20987893ea5ac454c4c0a50d0c470e26b5e650`, and the Gate 2 candidate from
base `0b7fbbd9d06c0f2417374d428089232704c49b8b`. On CPU 0 with the existing
`powersave` governor, tmpfs fixtures, `CLOCK_MONOTONIC_RAW`, one warm-up, and
seven crossed runs, 256 MiB C/A median/p95 ratios are 1.1389/1.0722 and 1 GiB
ratios are 1.1392/1.1315. Both pass the 1.15 ceiling but miss the 1.10 target,
so no cursor specialization is activated. All three cells have exact count and
semantic order equality; B/C canonical v5 digests are byte-identical. Dense C
peak is 166,136 KiB and remains Gate 5 input, not a Gate 2 memory claim.

Gate 3 uses the same A and B revisions, committed Gate 2
C=`05c50802b7393a213147b8a2b52b2616b4b06bee`, and D as C plus the Gate 3
candidate delta. The same CPU 0, `powersave`, tmpfs, raw clock, one warm-up,
seven crossed samples, and nearest-rank p95 rules apply. The 256 MiB sparse
source SHA-256 remains
`641f7442659ee50a6c5e183fd0a95963deaa21490ac2884215639ea704614d9e`;
the explicit density recipe is 134,217,728 copies of `x\n` with SHA-256
`a3978b948296b92171d4b9ae213daf796b3d79e6bc40ccc6f5d3dfc03f66c2e4`.

| Gate 3 cell | A/B/C/D median ms | D/A median | D/A p95 | D peak HWM KiB |
| --- | --- | ---: | ---: | ---: |
| Host Check | 0.001/0.001/0.001/0.001 | 0.9345 | 1.3056 | 2,600 |
| Untrusted Check | 151.208/238.431/251.025/163.487 | 1.0812 | 1.0712 | 2,600 |
| Host self-Line View | 154.857/68.237/68.364/69.525 | 0.4490 | 0.4465 | 264,676 |
| Untrusted self-Line View | 531.818/270.417/267.883/199.133 | 0.3744 | 0.3939 | 264,680 |
| unit raw-after Apply | 211.653/301.910/298.813/223.974 | 1.0582 | 1.0047 | 2,600 |
| receipt Apply | 212.856/300.970/297.458/223.528 | 1.0501 | 1.0528 | 2,600 |
| live-Anchor Apply | 213.208/304.552/297.168/224.142 | 1.0513 | 1.0568 | 2,600 |
| short-Line Check | 150.146/492.255/502.830/164.386 | 1.0948 | 1.0820 | 2,592 |
| CRLF one-shot Edit | 2.227/1.718/2.268/2.291 | 1.0287 | 1.0039 | 2,736 |

Host Check has zero capability open/read/hash/cursor work by structural audit;
the recorded process I/O includes the untimed proof-installing Search. Raw
Check and View each use one accepted observation, while Apply before-state has
zero cursor and after-state has at most one cursor only for a non-File receipt
or live non-File Anchor. All semantic/source digests match; CRLF final bytes are
SHA-256 `cc326fa86d3e5924c488283058e530b9413d6acec0f4f78a954882f85f92edbf`.
The native and Edit CSV SHA-256 values are
`d16c8eaf2992e6dff787bc844da7e3cdb6bb7e00813ed355018f3669b1c0b5a8`
and `bd1446554c9c9e09c2dfbb7bc87440c988b42f86f3404d0ba2de1192829575da`.

Gate 4 compares committed Gate 3
D=`042cc9e7f6dfe6faf23937367ec02446693a1d2d` with E as D plus the exact
sampled production delta, SHA-256
`e2dbdcf529f14009b9a4c6caefc88ace414feae10eaf2c0769b2d8ca471b162c`.
The fixed host uses CPU 0, `powersave`, tmpfs, Rust/Cargo 1.95.0, one warm-up,
seven crossed samples, and nearest-rank p95.

| Gate 4 cell | D median/p95 | E median/p95 | D/E allocations per result | D/E peak HWM KiB |
| --- | ---: | ---: | ---: | ---: |
| one Line encode | 4,824/6,408 ns | 3,283/4,771 ns | 30/0 | n/a |
| 1,048,576 Line encode | 698.879/701.466 ns | 256.217/256.510 ns | 30/0 | n/a |
| 1,048,576 File encode | 496.660/500.301 ns | 166.140/170.774 ns | 17/0 | n/a |
| 1,048,576-result CLI Search | 0.85/0.87 s | 0.21/0.22 s | not claimed | 166,544/166,544 |
| native 200,000-File Search | 545.077/549.280 ms | 522.292/531.186 ms | not claimed | 126,588/126,716 |
| batch 200,000-File View | 669.651/673.911 ms | 659.558/666.399 ms | not claimed | 162,816/163,244 |

Every D/E output is exact. The million-result CLI object is 630,800,294 bytes
with SHA-256
`fcaceecf33c02bc382a25cce862dff97145f4c1941f04b0c52a269068672890a`;
native Search and batch View result digests are
`289912c04cc30f3a11126683a421cf8431f7d386ca89821749567887c019082e`
and `bcb333381293bb186f643565298d36783f0bd1913aaf3f8c737bd5cc5e02f958`.
The roughly 630 MB CLI stream is Adapter output, not native engine memory, and
the zero allocation evidence applies only to repeated canonical address
encoding. Production grows by 12 bytes/31 lines from Gate 3 and remains
2.42%/2.71% over B, inside but not renewing the existing direct-evidence
allowance.

Gate 5 compares committed Gate 4 E=`caa17fefa7394553a7fe4edfccea03b64245dd61`
with F as the exact pending-chunk and attachment candidate. The fixed host uses
CPU 0, `powersave`, tmpfs, Rust/Cargo 1.95.0, one warm-up, seven crossed fresh
processes, `perf` duration, `/usr/bin/time` HWM, and nearest-rank p95. The huge
Paragraph fixture is 1,048,576 copies of `needle\n`, 7,340,032 bytes, SHA-256
`913515a8747b7f1bf66a0e60d4f7d62aee87266faeffbf1aa60509d478c86b8c`.
The one-Line Paragraph fixture is 1,048,576 copies of `needle\n\n`, 8,388,608
bytes, SHA-256
`7e0d3b4cb91c4ed44f5a43986c70dca6b2ad8e1b33a214fb0c4dd6f311674464`.

| Gate 5 native Search cell | E median/p95 ms | F median/p95 ms | E/F p95 HWM KiB | E/F I/O |
| --- | ---: | ---: | ---: | ---: |
| one huge Paragraph | 77.892/79.547 | 89.234/95.662 | 166,192/87,816 | 0/0 |
| 1,048,576 one-Line Paragraphs | 81.106/86.370 | 92.087/93.502 | 166,200/87,864 | 0/0 |

Both cells return 1,048,576 exact ordered results. Their E/F newline-separated
canonical v5 transcript SHA-256 values are respectively
`8a0757469aaca90c84fb6807037b2d269c8fe277fbf7fe2023f6e4b6cb4ed0a3`
and `22d7ef161bad25c4d0d86c53b526a5f3a7809bbb4df648336fe8bcb6801c12b9`.
The chosen chunk holds 16,384 pending targets; the largest fixture uses 64
chunks, and ordered issuance drops each consumed chunk. Both peaks are below
86 MiB and the 130 MiB target, so the conditional shared Paragraph `Arc` is not
implemented. The harness source and raw measurement evidence SHA-256 values are
`764635b11096c5692bd6f1ad1df9fe62096d9730ba214790d2b0d8c7e0d1938b`
and `0abe220fb0e8be1a856f9e408770186049a88e872caafc4099968b1f97f5f245`.

Gate 6 verifies that raw and structural builder construction is infallible,
the source callback still receives the checked byte start, and cursor/result
Line-count parity remains exact without returning duplicate offsets. Anchor's
structural target projection no longer allocates an all-input index vector.
Runtime grouping uses `Anddress::same_source`; source-state comparison remains
one hash/length/Line-count authority. File and Paragraph Search share one
mutually exclusive tier slot. Issuer construction uses one owned-source
validation/`Arc` path. Apply retains its demanded raw-versus-structural output
mode, but neither output layer returns an impossible construction error.

GNU and musl each pass all 268 tests. Existing tests retain v5 KAT/no-v4,
8,191/8,192/8,193 raw/structural parity, exact Search tiers/order/duplicates,
View single/batch all-or-none, false-Line-count `NotCurrent`, receipts,
publication, Host proof, Anchor, and Correct 1 / Safe Reject 6 / Wrong Apply 0.
Production G is 304,431 bytes/9,213 lines: -1,727/-48 from F and
+7,162/+259 (2.41%/2.89%) from B, below the 306,187-byte/9,222-line ceiling.
B remains the target; this gate records no renewed growth allowance.

Because Gate 6 changes source observation plumbing, a focused current-G run
reuses the Gate 3 256 MiB sparse and short-Line fixtures, exact fixture SHA-256
values, CPU 0, `powersave`, `/tmp` tmpfs, one warm-up, and seven fresh
processes. Median/p95 milliseconds are 0.001/0.002 for Host Check,
169.623/170.436 for Untrusted Check, 233.491/235.467 for unit Apply,
234.161/235.633 for receipt Apply, 233.390/237.392 for live-Anchor Apply, and
171.756/175.116 for dense short-Line Check. Relative to D, the nontrivial
median/p95 ratios are respectively 1.0375/1.0363, 1.0425/1.0321,
1.0476/1.0453, 1.0413/1.0360, and 1.0448/1.0618. Host Check remains one
microsecond-class and has zero capability open/read/hash/cursor work by the
same production structure. Seven fresh CRLF one-shot Edit processes preserve
the fixed output SHA and record 0.954/1.100 ms median/p95.

The focused 200,000 one-byte File run returns exact paths from
`d000/f000.txt` through `d199/f999.txt`; Search, batch View, and sequential
View take 556.213, 711.319, and 675.253 ms with 159,552 KiB process HWM. Batch
and sequential results equal all 200,000 Search Anddresses and contents in
order. The task-local harness source and binary SHA-256 values are
`b8b4114095a9a99f3aa9046b43794e3d31a019e80ea8807e74f5a0831ce04d94`
and `44d915394ddf2737598bb281975421dec4906c9910365d6e27f242a2700b47e6`;
all fixtures, evidence files, and harness outputs are removed after the run.

## 0.2.4 structural-authority gates

Gate 1 records the consumer baseline. Gates 2–7 hard-cut current source to v5,
install the sole Issuer and complete-source structural cursor, and contract
Search results to direct Anddresses and View to geometry-driven exact-range
projection, remove one-shot Edit's private View, and close source-state-only
Check plus direct v5 consumer contraction, then close integrated source
readiness. Gate 8 publishes and closes Cargo, `bw version`, the four-target
distribution, installers, manifest, and Update target at `0.2.4`; the closed
`0.2.3` distribution remains immutable v4 evidence. Exact File, Paragraph, text-Line, and
File-child-Line KATs; strict v4/v3 rejection; source/geometry mutation and
overflow fail-closure; algebra; shared source identity; every terminator and
body class; source Line counting; Runtime Search geometry; Apply receipt/Anchor
projection; and existing semantic regressions are executable tests. Search
matching and Apply publication paths are unchanged. View has no relation or
Paragraph scan and returns only `Projected { anddress, content }` or
`RelationAbsent`.

Gates 6 and 7 pass the complete suite on both GNU and musl. Both targets pass offline/locked
all-target check, clippy with warnings denied, and release build; the GNU run
also records offline/locked metadata and dependency tree plus rustfmt. Gate 7
advances only the root package, root lock entry, and version KAT to `0.2.4`;
toolchain, dependencies, production `src/**`, and Search/View/Check/Edit
Adapter envelopes are unchanged. View alone hard-cuts to
`bw.cli.view.v2` for shared single/batch outcome items.

Check validates every input before I/O and groups by workspace coordinate and
logical path. Matching Host proof classifies from SHA-256, byte length, and Line
count without an open/read/hash, while mismatch is I/O-free `NotCurrent`;
miss, invalidation, poison, or unusable proof falls back to one source
observation and installs no proof. Reports preserve
input order and multiplicity, keep Current and Unavailable inputs, remove only
NotCurrent inputs, and canonicalize empty Search/Pick results. Structural tests
fix one production `StructuralCursor`, one ordinary-address Issuer, no target
geometry branch in Check, and no proof or Anchor mutation. Data, Pick, Session,
and external Rust regressions consume direct v5 values without an adapter.

Every later implementation gate must reject unsupported/invalid structure
before source mutation, discard provisional results on failure, leave no
parallel v4 Runtime path, and preserve the capability-specific error and
all-or-none contracts. Gate evidence is cumulative:

- Gate 2: exact v5 algebra, source/target geometry, canonical wire KAT, strict
  no-v4 decode, projection validation, and one construction authority.
- Gate 3: one `StructuralCursor` for CR/LF/CRLF/no-EOL and Paragraph framing,
  Search order/multiplicity, sparse large-source Search, one million hits, and
  absence of positional result duplication.
- Gate 4: exact self/ancestor View range projection, ordered duplicate
  single/batch results, one observation per source group, and 200,000-file
  source grouping without a relation scan.
- Gate 5: File/Paragraph/Line Replace receipts, every Line terminator,
  no-op/publication/proof/Anchor behavior, and no private Edit View.
- Gate 6: Check currentness, Data/Pick/Session consumers, no capability-owned
  constructor, one structural parser, and measured production-code contraction.
- Gate 7: complete GNU/musl semantics, blind-duplicate Correct 1 / Safe Reject
  6 / Wrong Apply 0, AI workflow evidence, sparse/one-million-hit/200,000-file
  memory measurements, and an explicit source-readiness GO/NO-GO.
- Gate 8: pinned artifacts, installers, Update, manifest-last publication,
  idempotent reuse, endpoint/install smoke, and release closure — complete.

Wall-clock results are evidence, not a semantic gate. No gate may infer
history, relocation, registry, watcher, retry, merge, rollback, or publication
authority from structural geometry.

Gate 3 fixes complete-source cursor behavior at 8,191/8,192/8,193-byte scratch
edges and checked offset overflow before input consumption. Parent/candidate
streaming JSON is byte-identical for 256 MiB and 1 GiB one-hit sources and a
1,048,576-hit source. Candidate peak RSS is 2,640 KiB, 2,504 KiB, and 166,404
KiB respectively; the parent million-hit peak is 215,660 KiB. These are
bounded-memory observations, not performance or arbitrary-input promises.

Gate 4's complete GNU suite passes 258 tests. The six allowed projections and
three downward rejections, File-child-Line `RelationAbsent`, direct/trusted/
anchored range equality, exact CR/LF/CRLF/no-EOL and Unicode Content, A/B/A
grouping, duplicates, fail-all behavior, Data retention, and Anchor/Apply
consumer continuity are executable. Matching Host proof tests read exactly the
requested range on one handle; proof-miss groups use one complete observation.
CLI tests parse and re-decode the hard-cut `bw.cli.view.v2` single and batch
items, preserve order and duplicates, and cover `--as` grammar and
`RelationAbsent`. The retired relation-scanner-specific tests are removed with
their production scanner; no compatibility assertion remains.

A task-local native harness creates exactly 200,000 one-byte admitted Files,
obtains 200,000 ordered File Search results from `d000/f000.txt` through
`d199/f999.txt`, and passes that collection to one File `view_batch` call. It
returns exactly 200,000 ordered `Projected` outcomes whose Anddresses equal the
inputs and whose Content is `x`. The harness, its generated workspace, and its
build output are removed after verification.

Gate 5 keeps the 258-test count while strengthening existing Apply and CLI
regressions. One-shot Edit now proves strict v5 decode, Line terminator lookup,
Content validation, and `Edit::Replace` validation before Runtime open, followed
by exactly one `apply_replace` call and zero View/Search/Check calls. An invalid
Line body remains a usage error even when the source is missing, proving that
the rejected command performs no source I/O.

Apply retains one public unit seam, one Replace receipt seam, and one internal
executor. Its prospective output uses one `finish_structural` pass and one
`AnddressIssuer`; receipt and same-path Anchor candidates enter that same
`AfterProjector`. A separate relation vector, receipt-target clone, and local
containment/overlap functions are removed in favor of v5 `contains` and
`overlaps`. Existing File/Paragraph/Line, every terminator, Unicode, empty and
large no-EOL, scratch-boundary, no-op, currentness, Host proof, Anchor,
publication failure, output failure, and five-Edit/four-Position raw Session
regressions remain the behavioral evidence.

Gate 7 exports published v4 A
`195aaa37068122097ecc04d2644642b6afcc6765`, first-v5 V5
`f93f44b785961695402eaaffa521cd4de5071bc2`, and candidate B
`8b20987893ea5ac454c4c0a50d0c470e26b5e650` from clean Git objects under the
fixed task root `/tmp/backwriter-gate7.8VIlme`. The same locked toolchain,
fixtures, CPU 0, `CLOCK_MONOTONIC_RAW`, one warm-up, and seven crossed samples
(`AB/BA/AB/BA/AB/BA/AB`) apply throughout. Binary SHA-256 is
`bd4aee49b531a525cc1375509d3d068e32538c061e84828f797f62101dc64a6e`
for A, `6c875cdcf2e1ae60c25b46e34b9840dab40480fbedf451fb35012d9e8feb14ad`
for V5, and
`68fba45ddee9d481213f5555d77ffa2b2a309e21a1ebc2c12ac45a6f29f2b105`
for B. Fixture-generator and Search-runner SHA-256 values are
`61df1572529a92dc06d319efbc3cd1617e984daed61a610775bb1faa03ca8d6f`
and `6d942cdb894eefa413592c88fd2d4c2e32b25aba1428d0d2975a4b1556437df7`.
Input SHA-256 values are
`641f7442659ee50a6c5e183fd0a95963deaa21490ac2884215639ea704614d9e`
for the 256 MiB sparse source,
`904c75499d4dc222f3df76ad0c2dcc397e0a163b56ed5c65692f65de7d67a162`
for the 1 GiB sparse source,
`913515a8747b7f1bf66a0e60d4f7d62aee87266faeffbf1aa60509d478c86b8c`
for the dense source, and
`08edfc37b84fc8a5e960bb2f9437590cc521c487b9aa098d3841d839e06ebf61`
for the ordered path/NUL/content contract of the 200,000-file fixture.

Search results are exact and complete:

| Fixture | Results | A median/p95 ms, maxRSS KiB, rchar, output bytes | B median/p95 ms, maxRSS KiB, rchar, output bytes |
| --- | ---: | --- | --- |
| 256 MiB sparse | 1 | 277.688/278.691, 10,844, 268,441,917, 453 | 394.048/396.072, 10,856, 268,441,917, 642 |
| 1 GiB sparse | 1 | 1,083.990/1,086.879, 10,956, 1,073,748,285, 455 | 1,551.261/1,553.824, 10,968, 1,073,748,285, 645 |
| 1,048,576 dense Lines | 1,048,576 | 541.525/568.415, 109,172, 7,346,493, 414,856,172 | 1,017.803/1,066.878, 166,488, 7,346,493, 628,703,142 |
| 200,000 one-byte Files | 200,000 | 614.027/623.230, 109,648, 206,461, 74,400,064 | 648.221/657.925, 114,984, 206,461, 72,800,064 |

`rchar` is identical within every A/B row. Sparse maxRSS is source-size
independent; the dense and many-file differences are explained by the larger
self-contained v5 geometry, not a second result collection. V5 and B output
are byte-identical at SHA-256
`56c62059fb5c0de9e5189bcc72808a280f9d5d5da00be425945b2a8fc5af89d3`,
`01d9f612a5d5c2220d173bf6e9369cf2f278dd8392787543b334b01954fc5fc6`,
`b740ea98080fc731b9a11a75190474c5c3487be5fe411007d420bc58a6bb44aa`,
and `3b8edb97992c30c45720ff87f153cae878be8dece1293e3f78512452a9f610ef`
for those four fixed-root fixtures. The earlier Gate 3 raw JSON SHA values
remain preserved evidence for its deleted task root; because workspace
coordinate is part of every Anddress, Gate 7 records new root-bound values
rather than misrepresenting a different root as the same byte stream. Raw
Search evidence SHA-256 is
`14e6f0137a02456398de9644950967952868dbb782cd583993a38b9e041db293`.

The 200,000-file native View harness confirms ordered `d000/f000.txt` through
`d199/f999.txt`, exact `x` Content, and batch/sequential equality at semantic
digest `47142f33ec75709312a40aa34b4b9f9f85ff15df50d76e644b99c29bb289451b`.
A/B `rchar` is identical at 606,529; median/p95 is
1,976.203/1,983.647 ms versus 2,018.248/2,064.369 ms, and maxRSS is
147,620 versus 158,564 KiB. The small View harness proves late Line self,
Paragraph, File, separator `RelationAbsent`, and duplicate batch equality;
A/B semantic SHA-256 is
`237e926aeaca9f6bcdfa779e633eaabe1ab9a31f0b55af30103d65932a2a44d9`
and V5/B output is byte-identical. Raw View evidence SHA-256 values are
`ccb80ec6a8d8381d5ed940ce9989b9f1d08fe5e9cfe53aa8af185f51b08d58cf`
and `2eb8f6e677c629625db52daaf444533c7f08bc01754b918b8b57e4438e2c5262`.
The many-file runner SHA-256 is
`f061fd9947b3fc739b6260dca4298d4f8b9502a2778d07bc29dcfe021eb8a97c`;
its A/V5 and B harness SHA-256 values are
`3776bac25a81548191201674d97496eb805ed200a87e678994a4e01d3dcf8d86`
and `3944f839c0facf3c54ef78015415959fa48f16e86a1aa6a9c6ad3d5673ee26f6`.
The small-View runner SHA-256 is
`4d15e2e3bb88f9dfd77fe2e4f7e089f7e1e976c1c605ae64b4c518b6719b7751`;
its A, V5, and B harness SHA-256 values are
`ef290c86c116bafe43eba8c2baaadce65191a28a02ffe488c18345edfda3edb8`,
`821d91c905248521fde068fc7d2561d4aa6b52f4c4f4070bc8468a099cf50874`,
and `75eca9a722549de1c25a25a3108f6938b56dfab400e702413b98ee8a2b8f37e1`.
The View input SHA-256 is
`70b89d947ded1c114ef109f8f45e4cd7d5e16497515cc38c6e4e5f1545a6ab78`.

None/LF/CR/CRLF one-shot Edit preserves each exact terminator and gives
byte-identical V5/B receipts. CRLF A/B median/p95 is 1.187/2.533 ms versus
1.798/2.771 ms, maxRSS is 11,036 versus 11,040 KiB, and final SHA-256 is
`cc326fa86d3e5924c488283058e530b9413d6acec0f4f78a954882f85f92edbf`.
The raw Session Replace/Apply control is byte-identical across A/V5/B and has
the same final SHA; A/B `rchar` is 6,632 and output SHA-256 is
`fa25fff17c951881bb024d75d2129bcf8681c13cc54a113cafb5e7c4f74c37e8`.
Edit runner/evidence SHA-256 values are
`dbff910d4fa4159bd2b1b56834b44bcc7549028d4e0a3c09d5ad427569c80645`
and `1c564949ff842ca36ddc6a447c3dd3c596bf5b3d9a498e0fdc0c87128980ac7c`.
The original input SHA-256 values are
`7c9bc58081262feba4a5609d4c9f0ae1353edd0d31ae48f92d6a68b1089fe090`
for None,
`d35fe0ba542f0e6402a4b323b465c7a13484d702f698adad65945e39d0c50c6f`
for LF,
`54a2d1515d7157eca2f57a655558197db1223ee863a46c681b41c0d02e7d3234`
for CR, and
`ec43c761c4cd9b113208d4f99b2e912697e414e739e4c60324f4b5ae2c72a3fb`
for CRLF.

The AI workflow uses five processes and five Adapter/Runtime calls: one Search,
one two-input Line-to-Paragraph `view_batch`, two `apply_replace` calls, and
one View of the second fresh receipt. Both Search objects are sliced once from
the original JSON, the duplicate Paragraph projection is preserved, the first
fresh receipt drives the second Edit, and the second receipt drives View. It
uses zero redundant JSON indexing, repeated individual View, post-Edit Search,
mandatory Check, history, relocation, and retry. A separate old-address Edit
exits 1 with only `error: current source is unavailable` and publishes no
bytes. Exact final bytes are `first final\r\nsecond needle\r\n`, SHA-256
`275b708709f7fbdcbfd6f150a430003dc636ea9a09884a34a110546633e3e5f0`.
Workflow runner/evidence SHA-256 values are
`cddb3f9db977569073ee2b80fe72f1c97bca779f224012d715898dd2dec8256b`
and `4a3ceb5ef477f62f42cecb6e925a159513c19fe8d9f75a87698243fe9dac35b8`.
The post-version GNU release binary SHA-256 is
`3a5988d74606ea5307083d5de7f469d4e318f72ac7295c80c3bb6c9687f83e3e`;
rerunning the same workflow with that binary produces evidence SHA-256
`03619d573711c8557bc1e19b7930ec4504e34e23ce76996ea75b139b28321f9c`
and the same final source SHA. Its raw Session control retains output SHA-256
`fa25fff17c951881bb024d75d2129bcf8681c13cc54a113cafb5e7c4f74c37e8`.

Before and after the version-only change, GNU and musl each pass all 258 tests,
all-target check, clippy with warnings denied, and release build; metadata,
dependency tree, and formatting also pass offline/locked. The blind duplicate
matrix remains Correct 1 / Safe Reject 6 / Wrong Apply 0. Production `src/**`
is byte-identical to Gate 6, so its recorded 297,269-byte/8,954-line measure
remains below Gate 1's 302,614 bytes/9,155 lines. Gate 7 is therefore GO and
advances only the package/root lock version, version KAT, and active status to
source-ready `0.2.4`.

Gate 8 reconstructs the exact Linux x86_64, macOS arm64/x86_64, and Windows
x86_64 artifacts from Source Authority
`0ee4dcce14da93f925c27a04d0e79051c83fd124`; the canonical 876-byte manifest
has SHA-256 `64db11f3851b9d490c1135877fc975e841bbe231153073b7e5397fc008cfde6e`.
The exact publisher installs eight versioned files, replaces `install.sh` and
`install.ps1`, and replaces the manifest last. Its second live execution reuses
all 60 files without changing bytes, inode, mode, owner, size, mtime, or ctime;
the prior 48 versioned files and `install.cmd` remain unchanged. Loopback and
public HTTPS each pass 60 GET and 60 HEAD checks plus root/unknown 404 checks
with exact bytes, length, MIME, cache policy, and zero HEAD bodies. Isolated
fresh install, public `0.2.3` Update, and `0.2.4` reinstall select the exact
Linux archive member. GNU and musl each pass 258 tests; installer, publisher,
CMD static, and Origin regressions pass 41, 56, 12, and 13 cases. Static
macOS/Windows verification is retained without a native runtime, PowerShell,
or CMD execution claim. Origin and cloudflared process identity, restart count,
listener, unit, YAML, DNS, tunnel, connector, and credential metadata remain
unchanged.

## 0.2.3 Patch Box Gates 1–8

Gate 1 records the direct Search, View, Edit, Check, Data, Session, Pick, and
writer consumers. Gate 2 implements only the Search observation carrier and
its two Adapter projections:

- `SearchOutcome::Found { occurrences: Vec<SearchOccurrence> }` owns one exact
  v4 Anddress and target-coherent optional `SearchPosition` per result. Public
  validation covers File/None, nonzero Line, ordered nonzero Paragraph bounds,
  Clone/Eq, borrowing, and ownership transfer.
- Runtime increments checked one-based Line state inside the existing Line and
  Paragraph framing projections. CR, LF, CRLF, bare CR, no-EOL, empty and
  separator Lines, Unicode, no synthetic EOF Line, and 8,191/8,192/8,193-byte
  scratch boundaries are exercised. No additional source open, read, hash pass,
  whole-source retention, observation object, or parallel result vector exists.
- Check filters complete occurrences while keeping report evidence as raw
  Anddresses. Data and Session store the carrier; Session indexing extracts the
  contained Anddress; Pick alone receives an explicit raw-Anddress collection
  and retains its established outcome and byte-range display.
- One-shot and Session human Search share `write_search`: File is path-only,
  Line is `path:line`, and Paragraph is `path:start-end`. The streaming JSON
  writer emits exact `bw.cli.search.v2` occurrence items and directly embeds
  `Anddress::encode()` output without a JSON value tree, result clone, or second
  collection. Empty/Found, all target kinds, position fields, key order,
  duplicate/order retention, escaping, large results, and Search-to-Edit v4
  extraction are byte-exact regressions.

Late selected-source failure continues to discard every provisional occurrence
and position. Existing literal tiers, ordering, multiplicity, v4 KAT,
currentness, and single-observation controls remain unchanged. Production has
no v1 writer or compatibility branch; `bw.cli.search.v1` remains only immutable
published `0.2.2` evidence. Cargo, CLI version, README, toolchain, server, the
official 44-file public tree, and services remain unchanged.

The complete offline/locked GNU-host suite passes 245 tests: the 243 inherited
controls plus one Runtime position/framing boundary test and one public
occurrence validation/ownership test.

Gate 3 changes the native single View seams to accept one existing
`AnddressTarget` projection. Public regressions cover all six allowed
Line-to-Line/Paragraph/File, Paragraph-to-Paragraph/File, and File-to-File
relations; projected v4 kind/range/hash/length and exact Content; related File
and optional Paragraph addresses; and `RelationAbsent` for separator and
raw-valid nonstructural Line-to-Paragraph requests. File-to-Paragraph/Line and
Paragraph-to-Line return `InvalidInput` before a missing source can be opened.

The direct one-forward-observation path, matching Host trusted range path, and
anchored path produce equal results. Scalar-aligned raw ranges exercise every
allowed projection through direct/trusted parity. CR, LF, CRLF, bare CR,
no-EOL, Unicode, separator relations, and 8,191/8,192/8,193-byte boundaries
cover exact upward Content. Existing stale/hash/length/range, workspace/path,
admission, no-follow, UTF-8/NUL, resource, proof invalidation, and Anchor
fail-closure controls remain. CLI one-shot/Session/anchored View and one-shot
Edit pass self projection, while Data retains the extended `ViewOutcome`; all
existing human/raw/JSON bytes and grammar remain unchanged. Structural checks
retain one `finish_outcome`, no `view_projected` facade, no request DTO, and no
second View parser or executor.

The complete offline/locked GNU-host suite now passes 247 tests: the 245 Gate 2
controls plus two public Gate 3 projection and pre-I/O validation regressions.

Gate 4 adds the public ordered batch seam without changing single View,
anchored View, or Adapter execution. Empty, single, duplicate, mixed A/B/A,
same-source distinct and overlapping ranges, all six allowed projections, the
three downward relations, `RelationAbsent`, Unicode, separators, every Line
terminator, no-EOL, raw ranges, and the 8,191/8,192/8,193-byte boundaries are
covered. Returned outcomes preserve exact input order and multiplicity.

Every source-less input and requested relation is validated in input order
before complete coordinate, spill, and admission preflight or source I/O.
Stale state, foreign coordinate, spill, unadmitted and missing paths, symlinks,
invalid UTF-8, NUL, late read failure, and Resource failure return no partial
vector. Structural regression fixes one source-key group pass, one open and
one direct observer per group, no public single-View loop, and fallible
allocation for indices, captures, provisional slots, and final output.

Host regressions cover A/B/A proof groups, trusted/direct output parity,
proof-miss fallback, mismatch before I/O with proof preservation, one matching
group handle, and existing source/resource invalidation. The complete
offline/locked GNU-host suite passes 253 tests: the 247 Gate 3 controls plus
two private observer/all-or-nothing tests and four public batch regressions.

Gate 5 adds `EditReceipt::{Unchanged, Changed}` and the Replace-only
`WorkspaceRuntime::apply_replace` seam while keeping the unit-returning public
`apply` and raw Session unchanged. Structural regression fixes one internal
executor, one admitted source open and observation, one prospective-after
source identity, receipt preparation before the existing Anchor plan and Host
proof, and successful receipt return only after confirmed publication. It
excludes Search, source reopen, second observation, request DTO, parallel
executor, binding, Data kind, JSON, and stdin.

Changed File coverage checks exact prospective hash, length, full range,
immediate View and Check, old-address rejection, and a following Replace with
the returned address. Line coverage checks exact None/LF/CR/CRLF results,
Unicode, empty body, empty no-EOL output, and current zero/nonzero ranges.
Paragraph coverage fixes zero, one, and multiple resulting Paragraphs as
`Changed(None)`, `Changed(Some)`, and `Changed(None)` without restricting
Content. Direct zero-range and assembled byte-identical no-op return the exact
input while preserving bytes, inode, Host proof, and live Anchor. Receipt and
Anchor regressions compare the same installed after source identity.

Non-Replace input rejects before filesystem access. Existing stale, invalid,
unadmitted, open/read/resource, staging collision, rename uncertainty, cleanup,
proof invalidation, and Anchor fail-closure tests continue through the shared
executor and return no successful receipt on error. Existing raw Apply covers
all five operations and four positions. The Gate 5 baseline had 255 tests: the
253 Gate 4 controls plus two public receipt regressions.

Gate 6 removes only the one-shot Adapter's discarded-receipt and shared `OK`
writer call. One direct writer emits exact human `Unchanged`/`Changed` rows or
the fixed-order `bw.cli.edit.v1` object. It calls `Anddress::encode()` once
before creating the stdout writer and directly reuses those bytes; structural
checks reject a JSON `Value`, reserialization, clone, result collection,
post-Apply Search, reopen, Check, stdin reader, parallel writer, or second
schema. Raw Session Apply continues to use the independent status writer and
emits exact `OK` plus LF.

CLI regressions cover human and JSON `Unchanged`, changed File/Line/unique
Paragraph addresses, and changed zero/multiple-Paragraph `None`/`null`; exact
schema/key order/final LF; embedded canonical-v4 decode and byte equality;
every Line terminator, empty and Unicode Content; and a Search-v2 object passed
unchanged to Edit followed by View and another Edit using only the fresh
receipt address. Leading `--json`, rejected `--raw`, duplicate output choice,
extra operands, and literal positional `--json`, `--raw`, and `--stdin` are
covered. Existing stale, missing, unadmitted, Runtime resource/read, staging,
rename-uncertain, and publication failure controls plus writer-after-Apply
ordering establish zero success output and no receipt on Apply failure. A
Linux `/dev/full` control proves a post-publication flush failure exits `1`
while leaving the confirmed source publication intact and without retry.

At that historical `0.2.2` Gate 6, argv was the only Content transport. Direct
empty/Unicode, File/Paragraph-newline, and Line-body coverage existed; argument
length, shell/newline behavior, and process-list/history exposure supplied no
reproduced consumer failure, measured payload need, or concrete security
requirement. The later `0.2.6` Gate 3 adds the one exclusive `--stdin` EOF
selector; neither gate adds a generic content source, file transport, or
placeholder. The complete offline/locked GNU-host suite passed 256 tests: the
255 Gate 5 controls plus one CLI stream-failure regression.

Gate 7 builds the published `0.2.2` Source Authority
`04b36d9ca9cc725bedeb17231339c67b5f0590ea` and the integrated Patch Box parent
`d3e2b2e65112e9f0f018cd29050652928e4ef412` from clean Git-object exports. On
the exact `retry_budget = 3\r\n` fixture, A performs Search/Edit/Search/View/Edit
and B performs Search/JSON Edit/View/JSON Edit. Both finish with exact bytes
`retry_budget = 7\r\n` and SHA-256
`798ba02ce45d505e56b0112210695a52931a40797aa9eb6f68d608d9c9b6173e`.
A/B process and Adapter-command counts are `5/4`, Search `2/1`, repeated
post-Edit Search `1/0`, JSON array indexing `2/1`, explicit View `1/1`, Edit
internal View `2/2`, Apply `2/2`, and total Runtime capability calls `7/6`.
Caller-visible raw Apply, mandatory Check, Wrong Apply, history, relocation,
retry, and newline mistakes are all `0` in both flows. B uses each fresh
receipt address directly for the following View and Edit without post-Edit
Search.

One untimed warm-up per variant precedes five order-crossed `AB`, `BA`, `AB`,
`BA`, `AB` samples. The monotonic interval spans the first `bw` spawn through
the final source read and excludes fixture reset. A median/p95-nearest-rank is
`25.168625`/`25.812745` ms; B is `19.793078`/`20.982414` ms. These elapsed
samples are diagnostic evidence, not a performance gate or general speed
claim. The task-local driver SHA-256 is
`18ec103d2815f52957d29e3be986f3b9e8027d3442a14bd252f60145033e410d` and its
raw JSON evidence SHA-256 is
`8c48cb6192621f0d7c92a6be76d432beb2c11bd134c00be008349ca01dd8243a`;
both are removed after verification.

Before and after the source-version change, offline/locked GNU and musl runs
each pass the complete 256-test suite, all-target check, and release build.
GNU also passes formatting and clippy with warnings denied. Release Help,
Version, JSON Search-to-receipt-to-fresh-View/Edit, representative raw Session,
v4 KAT/no-v3, Search order and duplicates, single/batch View all-or-none and
one-observation behavior, all receipt states and writer failure, raw Apply,
Host proof hit/miss/mismatch/invalidation, Anchor reflection/fail-closure, and
the duplicate Line/Paragraph drift matrices remain green. Gate 7 therefore
advances only the root package, root lock entry, version KAT, and active status
documents to source-ready, unpublished `0.2.3`; production `src/**`, Core,
Runtime, v4 wire, toolchain, and dependencies remain byte-identical to the
Gate 6 parent. Official artifacts, installers, manifest, public root, service,
and `bw update` remain closed `0.2.2` until Gate 8.

Gate 8 reconstructs the four canonical `0.2.3` artifacts from Source Authority
revision `195aaa37068122097ecc04d2644642b6afcc6765`, their sidecars, and the
exact 876-byte manifest with SHA-256
`876ce79316663faa06cbcd2d374edcc7874e8374b1838f823e52fc623466ea73`.
Before live publication, GNU and musl each pass all 256 Rust tests and their
offline/locked all-target check and release build; GNU also passes formatting
and clippy with warnings denied. The installer, `0.2.3` publisher, CMD, and
Origin suites pass 38, 55, 12, and 13 regressions respectively.

The exact publisher adds only the eight `releases/0.2.3` files, then replaces
`install.sh`, `install.ps1`, and the manifest last. All 40 earlier versioned
files and `install.cmd` retain bytes, inode, mode, owner, size, and timestamps.
A second publisher execution reuses all 52 files without metadata change.
Every one of the 52 files passes loopback and public HTTPS GET and HEAD checks
for exact body or empty HEAD body, length, media type, and cache policy; root
and unknown paths remain empty no-store 404 responses.

An isolated public fresh install prints exact `Installed Backwriter: 0.2.3`.
The public `0.2.2` binary Update and a `0.2.3` reinstall print exact `Updated
Backwriter: 0.2.3`; the installed binary equals the canonical Linux archive
member. Installed Help, Version, Search v2 position, original embedded v4
object to JSON Edit receipt to fresh View and next Edit with zero post-Edit
Search, CRLF preservation, old-address no-publication rejection, Check, raw
Session Apply, and duplicate-drift Safe Reject probes pass. The actual user
HOME, PATH, and shell startup files remain unchanged.

Origin and cloudflared PID, InvocationID, restart count, the loopback listener,
unit and ingress YAML bytes, credential metadata, tunnel UUID and connector,
and DNS remain unchanged. macOS and Windows receive static cross-build and
archive verification only; no native macOS, Windows, PowerShell, or CMD
execution is claimed. No tag, GitHub Release, crates.io publication, cache
purge, service, tunnel, DNS, route, or credential mutation closes this release.

## 0.2.2 Anddress-first editing Gates 1–6

Gate 1 closes authority and Gate 2 implements only the one-shot Adapter
composition tracked in the
[0.2.2 tracker](../tasks/2026-09-01-backwriter-0.2.2-anddress-first-editing.md).
CLI regressions prove exact File and Paragraph replacement; Line body
replacement with None, LF, CR, and CRLF preservation; empty and Unicode
Content; CR/LF Line rejection before Apply; strict v4 decode; stale, missing,
and unadmitted source rejection; byte-identical no-op and Unix inode
preservation; exact `OK` plus LF success; stderr-only exit `1`/`2` failures;
literal `--json`/`--raw` File, Paragraph, and terminator-preserving Line Content;
and leading global output-option plus trailing extra-operand rejection without
source mutation.

Structural evidence fixes one ordinary Runtime, one private View, only
`Edit::Replace`, the original decoded Anddress as Apply target, existing Edit
validation, Runtime Apply, and the existing status writer in that order. It
excludes Search, Check, a new Runtime seam, engine, state machine, retained
observation, relocation, retry, fallback, v4 schema, error alias, or
compatibility layer. Existing Apply/Anchor regressions continue to own
View-to-Apply mutation, uncertain publication, Anchor reflection, Host proof,
and invalidation evidence. Raw Core Edit/Position/Apply, Session Edit and Apply,
Cargo `0.2.1`, `Backwriter 0.2.1`, Core, Runtime, and the public distribution
remain unchanged controls. The complete offline/locked GNU-host suite passes
242 tests: 236 existing controls plus six Gate 2 CLI regressions.

Gate 3's source and consumer audit closes without an additional Content
transport, machine-output schema, parser, writer, type, dependency, or Runtime
seam. Existing argv covers accepted Content, Search JSON carries exact v4
Anddress objects, and the existing status/error path distinguishes success,
usage, and execution outcomes without claiming that exit `1` preserves source
bytes or permits retry. Documented residual constraints are OS argument limits,
shell quoting/newline portability, and process-list/history exposure.

Gate 4 uses one task-local workspace whose sole source is the exact Line
`retry_budget = 3` plus CRLF. JSON Search returned one exact 311-byte v4 object;
passing those object bytes unchanged to one-shot Edit with body
`retry_budget = 5` exited `0`, wrote exactly `OK` plus LF, preserved CRLF, and
used two processes and two one-shot Adapter commands. The Edit command itself
privately invoked View and Apply.
A separate stale reuse control exited `1` with the existing Unavailable error
and preserved the already-edited bytes. The raw comparison used one Session
process with Search binding, optional View, indexed Replace Edit binding,
separate Apply, and `exit`: four work expressions plus one control expression.
It produced byte-identical final source while keeping binding, index, escape,
terminator, and publication responsibility with the raw caller. No elapsed-time
claim is made.

The JSON extractor existed only in the task-local fixture and established exact
substring transfer rather than parse-and-reserialize behavior. It is not a
product dependency. The fixture was removed after verification. Rust, tests,
Cargo metadata inputs, lockfile, and toolchain remain byte-identical to the
Gate 3 parent, so the existing complete 242-test result is reused rather than
misrepresented as a new code-test run. Direct source-release `bw --help`,
`bw version`, Search, Edit, stale rejection, and raw Session smoke evidence was
rerun for this documentation gate.

Gate 5's tracker consumer matrix binds each retained surface to its production
caller and behavioral regression. Public Rust Edit/Position validation,
`WorkspaceRuntime::apply`, Runtime geometry/publication, and Anchor reflection
remain covered by external-crate-style `edit`, `apply`, and `anchor`
integration tests. Raw Session remains covered by its parser, explicit binding
and clone/reuse paths, borrowed unindexed Apply, exact source results, and Data
rejection. Canonical one-shot Edit remains covered by the six existing
File/Paragraph/Line, terminator, invalid-input, unavailable-source, no-op, and
structural-composition regressions.

The existing Session operations case now proves all five Edit variants and all
four Position forms by adding valid `After(Line)` and `StartOf(File)` inserts.
Its unused clone is removed because the separate clone-and-both-Apply regression
is the unique reuse evidence. The invalid-form case now asserts the actual
`Edit input is invalid` stderr from `StartOf(Line)` and removes one duplicate
wrong-binding assertion. No test function, helper, fixture, production Rust,
Cargo input, CLI behavior, or public contract is added. Core NUL validation,
public Apply revalidation, exact source assertions, borrowed Apply structure,
and direct Apply/Anchor regressions remain distinct controls. The complete
offline/locked GNU-host suite remains 242 tests. Automated JSON
Search-to-one-shot Edit end-to-end coverage is intentionally Gate 6 input.

Gate 6 adds that one independent regression without a helper, parser, or second
wire path. It verifies the exact single-found `bw.cli.search.v1` prefix and
suffix, removes only those fixed bytes, decodes the remaining original object
as v4, passes its unchanged UTF-8 bytes as one Edit argv, and proves exact
`retry_budget = 3\r\n` to `retry_budget = 5\r\n` replacement with exit `0`,
empty stderr, and `OK` plus LF. Existing no-op, stale reuse, and terminator
regressions remain the unique controls for those meanings.

The complete GNU and musl suites each pass 243 tests. V4 KATs,
Search/View/Check/Apply semantics, Correct `1`/Safe Reject `6`/Wrong Apply `0`,
raw Session's five Edit variants and four Positions, binding/index,
clone/reuse, separate Apply, every one-shot target/terminator, and exact
`0`/`1`/`2` output boundaries remain intact. Compared with Source Authority
`4a1b06fb375bfd906a6f27de4de15a8febfe08ec`, Core, Runtime, Anddress v4,
toolchain, and dependency inputs are byte-identical, and the Adapter and Runtime
retain one Edit executor each. Source Cargo and `bw version` advance to
`0.2.2`; official `0.2.1` artifacts, installers, manifest, public tree, and
service remain unchanged. A source-built `0.2.2` Update may install official
`0.2.1` because the command has no version comparison; Gate 7 owns publication.

## 0.2.2 release closure

Gate 7 reconstructs the four canonical artifacts and sidecars from Source
Authority revision `04b36d9ca9cc725bedeb17231339c67b5f0590ea` and reproduces
the exact 876-byte manifest with SHA-256
`c2e55c9617db5a30fc5320d00e70d547ed9720bacbeac7e0a3cbec33b2fb079d`.
The publisher adds the eight `releases/0.2.2` files, replaces `install.sh`,
replaces `install.ps1`, and publishes the manifest last. It preserves bytes,
inode, mode, owner, size, and mtime for all 32 earlier versioned files and
`install.cmd`; an idempotent rerun reuses all 44 files and their metadata.

All 44 loopback and public HTTPS files pass GET and HEAD with exact bodies,
SHA-256, lengths, content type, zero HEAD downloads, and cache policy. Root and
unknown-path GET/HEAD remain 404/no-store. A task-local canonical `curl | sh`
fresh install and an actual public `0.2.1` binary's explicit update install
byte-identical `0.2.2` Linux binaries and print the exact Installed and Updated
outcomes. The installed binary passes Help, Version, JSON Search-to-exact-v4
one-shot Edit with CRLF preservation, View, Check, raw Session Apply, stale
reuse, and duplicate-drift Safe Reject probes.

Closure includes the completed 243-test GNU and 243-test musl source matrices and
their offline/locked metadata, tree, formatting, all-target checking, clippy
with warnings denied, and release builds. Origin 13, installer 37, publisher
53, and CMD 12 regressions pass with their standard checks and build. Origin
and cloudflared PID, InvocationID, restart count, listener, unit/YAML,
credential metadata, tunnel, DNS, actual user HOME, process PATH, and shell
startup files remain unchanged. macOS and Windows artifacts receive static
cross-build verification only; no native macOS, Windows, PowerShell, or CMD
execution is claimed. No tag, GitHub Release, crates.io publication, cache
purge, service, tunnel, DNS, route, or credential change occurs.

## 0.2.1 observation-reuse and release closure

The `0.2.1` target is published and closed. Phase 2 adds the Host Runtime
constructor, source invalidation kernel, private proof state, and
successful-Search proof installation. Phase 3 adds bounded ordinary View proof
consumption. Phase 4 adds Check current-proof group classification;
Phase 5 adds Apply proof precondition reuse, exact no-op preservation,
prospective-after proof installation, and coupled Anchor reflection;
Phase 6 closes path-exact invalidation, guarded mutation sequencing,
authority isolation, matching anchored View reuse, failure transitions, and the
both-mode drift matrix;
Phase 7 records the historical fixed A/B source-readiness NO-GO; Phase 7A
closes that run's sole failed related-Paragraph performance gate; Phase 7B
remeasures the complete matrix and closes source readiness with every gate PASS.
V4 wire and default Untrusted behavior remain unchanged; the development phases
preserved the then-current public `0.2.0` release until the separate `0.2.1`
publication closure. The
Protocol owns default Untrusted Mode and explicit Host-authoritative Mode; the
[phase tracker](../tasks/2026-08-30-backwriter-0.2.1-current-observation-reuse.md)
owns the execution audit, fixed `0.2.0` comparison inputs, complete raw Phase 7,
7A, and 7B samples, checksums, and gate evidence.

Phase 2 regressions prove that Untrusted Search installs no proof; Host exact
File and content Search install exact hash/length; re-search replaces one path;
workspaces and logical paths remain isolated; multi-source success installs all
fully observed sources while late failure installs none; and invalidation,
unavailable source, Anchor fail-closure, Apply entry, and uncertain publication
remove only affected proof. Structural checks exclude retained bytes, results,
ranges, history, public getters, CLI Host activation, and Debug disclosure. At
Phase 2 closure, Check and Apply still used the full `0.2.0` observation path.
Search remains the only target finder; Check and Apply must not relocate or
context-match.

Phase 3 regressions prove ordinary View proof hit, miss, mismatch, explicit
invalidation fallback, and unchanged Untrusted behavior. They cover
File/Paragraph/Line output, every terminator, Unicode, whitespace and
raw-valid nonstructural Lines, UTF-8 range cuts, short reads, matching-proof
removal, fixed-scratch boundaries, and long adjacent Lines. Structural and
counting-reader evidence proves the trusted path computes no SHA-256 or
complete observation, reads no source-size-proportional prefix merely to find a
Line relation, releases proof state before I/O, and retains only the returned
target plus fixed scratch. The Phase 3 development suite passes 215 GNU-host
Rust tests.

Phase 4 regressions prove Host Search to raw, Search-outcome, and Pick-outcome
Check hits; 10,000 mixed matching/stale occurrences with duplicates and exact
order; hash and length mismatches without fallback or proof mutation; and
raw-valid nonstructural Paragraph and Line currentness. They cover mixed
proof-hit/proof-miss sources, one observation per miss group, no Check proof
installation, Untrusted/miss/poison/unusable fallback, explicit invalidation
followed by changed, missing, and invalid source, and unchanged workspace,
private-path, admission, empty-report, and Resource boundaries. Structural
evidence fixes the proof lookup before filesystem open, one fallback observer,
owned fixed-size digest evidence, and lock release before I/O, hashing, and
report assembly. The Phase 4 development suite passes 220 GNU-host Rust tests.

Phase 5 regressions prove Host Search-to-Apply proof hit without a before hash,
exact changed publication, prospective-after proof installation, a second Apply
using that proof, and old-address Safe Reject. Direct and assembled identical
no-op preserve proof, live Anchor, inode, source bytes, and temporary state;
proof-mismatched operands reject before source access while preserving proof
and Anchor. They cover Host proof miss, Untrusted fallback through the complete
existing suite, post-Apply View and Check reuse, File/Paragraph/Line Anchor
reflection from the same after identity, unrelated-path isolation, short/grown/
invalid trusted source fail-closure, temporary collision, source read/resource
classification, publication uncertainty, and the existing duplicate-drift
matrix. Structural evidence fixes exact-length-plus-one fixed-scratch staging,
zero SHA-256 on the trusted before path, prospective proof preparation before
publication, no proof lock across I/O/hash/emission/publication, and no retained
source, previous proof, or history. The Phase 5 development suite passes 228
GNU-host Rust tests.

Phase 6 regressions prove both public invalidation methods delegate to one
I/O-free path-exact proof-plus-Anchor operation; invalid syntax, private paths,
and unadmitted paths preserve unrelated state; and same-hash paths, workspaces,
admissions, Runtimes, Host versus Untrusted mode, and Runtime lifetimes remain
isolated. After correct pre-mutation invalidation, same-length and
different-length replacement, deletion, invalid UTF-8, and NUL make stale View,
Check, and Apply safe-reject without publication. A confirmed Apply followed by
invalidation and external mutation likewise rejects its old after address.

Proof mismatches use filesystem-absence tripwires to prove zero source access
and state preservation for ordinary View and Apply, while Check remains
`NotCurrent` without mutation. Matching anchored View structurally shares the
ordinary trusted View helper; an anchored proof mismatch removes same-path
proof and continuity before source access. Existing private failure seams and
integration regressions fix trusted View open/seek/read/short/resource proof
removal, Apply open/read/resource and definite-prepublication preservation
boundaries,
invalid/length-drift fail-closure, no-op preservation, confirmed after
installation/reflection, and path-exact uncertain-publication invalidation.
Failed Search installs no provisional proof, Check fallback installs none, and
Runtime drop retains none.

The exact seven-cell duplicate-Line drift matrix passes in both Untrusted and
correctly guarded Host modes with one Correct Apply, six Safe Rejects, and zero
Wrong Applies; duplicate Paragraph drift rejects in both modes. Structural
evidence continues to exclude proof locks across I/O, hashing, emission, and
publication, plus whole-source retention, prior-proof chains, history, public
hooks, or persistent cache. The Phase 6 development suite passes 234 GNU-host
Rust tests. No Phase 6 result is a benchmark or release-readiness claim.

Phase 7 uses immutable A=`2fad6e4` and B=`a24ff5e`, independent offline/locked
release targets, fixed historical fixture SHA-256 values, the same task-local
harness, CPU 2 P-core, `powersave`, and one warm-up plus seven order-rotated
samples per A/BU/BH cell. Search passes at `268.333` ms BU and `268.379` ms BH
against `313.929`. Host Check hit passes with exact zero `rchar`/`wchar` and the
unchanged zero-read/hash/target-search structural evidence. One-million-result
peak HWM passes at `58.5078` and `58.5156` bytes/hit against `61.4383`; low-hit
128-to-256 MiB HWM is flat, whole-source retention remains absent, and Wrong
Apply remains zero.

Host Search-to-late-Line View fails: median `1,079.943` ms and p95 `1,096.362`
ms exceed the formal 400 ms ceiling and 350 ms recommendation. Its exact
`536,870,913` `rchar` consists of Search's complete source read plus the
trusted Line relation's complete no-separator Paragraph boundary read. Result
SHA-256, source SHA-256, count, v4 wire, order, multiplicity, Untrusted
fallback, and all Phase 6 transitions remain exact. The decision is therefore
NO-GO; production Rust and Cargo version remain unchanged. The Phase 7 suite
uses no new benchmark framework or repository instrumentation, and its
complete raw evidence and reproduction hashes are in the tracker.

Phase 7A removes the related-Paragraph path's private byte-at-a-time reverse and
forward cursors and reuses the same two fixed 8,192-byte scratch arrays. Direct
chunk scans preserve CR, LF, CRLF split across scratch boundaries, EOF bare CR,
no-EOL, Unicode scalar ranges, long Lines, surrounding space/tab separators,
BOF/EOF Paragraphs, source/error boundaries, and exact related addresses. A
word candidate filter is exhaustively checked across every byte alignment and
scratch-edge lengths; trusted and direct projection remain equal for every
scalar-aligned target range. The GNU-host suite passes 236 tests.

The exact Phase 7 256 MiB fixture
`d4edd123621cf230590d7812e64bec69460789eba3e0c7136b88a3f26c88f5e5`
is measured against baseline `0f1cc6b` and the candidate on CPU 2 P-core with
`powersave`, one warm-up, and seven order-crossed samples. Host
Search-to-late-Line View moves from `1,035.274` / `1,041.995` ms median/p95 to
`331.527` / `332.547` ms and therefore passes both 400 ms and recommended 350
ms gates. Baseline and candidate stdout SHA-256 are both
`1813f9bf4a219f21de1c4a539a5057676ed37835cc405b78501ea894566212b3`;
both record HWM below 2.7 MiB, exact `536,870,913` `rchar`, and zero residual
`wchar`. Search-only stays `270.734` versus `270.534` ms; Host Check remains
zero-I/O; the one-million-result candidate peaks at 60,080 KiB, 58.672
bytes/hit and below the 61.4383 bound. Separator, forward, File/Paragraph,
ordinary/anchored, and Untrusted controls have byte-identical outputs and no
unexplained material regression. Apply code is untouched and the full suite
retains the existing one-Correct/six-Safe-Reject/zero-Wrong matrix. This closes
the performance gate only. Phase 7B reuses the fixed large fixtures and one
task-local A/BU/BH source across 17 cells. BU/BH 256 MiB Search medians are
`267.397`/`267.273` ms, Host Search-to-late-Line View is `324.254` ms, Host
Check proof hit is exact zero I/O, and one-million-hit HWM is `58.609`
bytes/hit. All cell payloads agree and the drift regression retains one
Correct, six Safe Rejects, and zero Wrong Applies. Source version is `0.2.1`;
artifact and publication authority is unchanged.

Phase 7A source verification passed 236 GNU-host tests. Phase 7B post-decision
verification passes offline/locked metadata and dependency tree, format,
all-target check, all 236 GNU-host tests, clippy with warnings denied, release
build, and release capability probes; version output is exactly
`Backwriter 0.2.1`.

## 0.2.0 Phase 7 and Search recommendation closure

The closed public `0.1.0` release remains immutable v3 evidence. Current
published `0.2.0` Rust, Cargo, CLI, and tests use only Anddress v4. Phase 3
implements the SHA-256 source-state/value/wire kernel and hard-cutover decoding.
Phase 4 implements one-read target-specific Search observation. Phase 5 makes
ordinary View and Check direct consumers of the same hash/length observer.
Phase 6 makes Apply and Anchor direct v4 range/provenance consumers and removes
the last legacy locator/parser and generic event-framer path. Phase 7 completes
integrated correctness and A/B measurement without adding a benchmark framework
or changing public meaning. Their phase gates, the historical v3
drift reproduction, fixtures, raw samples, and profile results are tracked in
[Backwriter 0.2.0 Anddress fast path](../tasks/2026-08-30-backwriter-0.2.0-anddress-fast-path.md).
That task tracks progress only; the Protocol, address model, and principles own
semantics.

Phase 3 verification proves exact eight-field KATs for every kind; valid-only
public construction; canonical decimal and checked machine-range boundaries;
hash, length, kind, and range equality; strict malformed/duplicate/missing/
unknown/wrong-type handling; well-formed v3 `UnsupportedVersion`; shared private
source identity across same-source results; one-read hash/range Search; v4
human/JSON/Session round trips; and full existing capability regressions. The
exact seven-cell duplicate-Line drift matrix produces one Correct Apply, six
Safe Rejects, and zero Wrong Applies. Each reject preserves exact source bytes,
a current same-path File Anchor, and temporary/publication state. The duplicate
Paragraph regression separately preserves the same fail-closed result. Phase 6
preserves those results without an Apply/Anchor consumer indirection.

Current regressions cover SHA-256 transcript and platform-coordinate KATs,
strict v4 flat-wire encoding/decoding and error priority, zero/nonzero/maximum
machine ranges, exact source identity sharing, Search no-limit traversal, query
and scope preflight, canonical range ordering, selected-source fail-all,
NUL/invalid UTF-8 source handling, spill boundaries, no-follow symlinks and hard
links, View exact-source direct range projection and related v4 addresses,
terminator and Unicode reconstruction, and Pick stable order, multiplicity, target kinds,
complete-v4-value OneOf, and deep iterative boolean composition. Apply regressions cover Edit validation
priority, exact File/Paragraph/Line and raw-valid range splice geometry, all
line terminators, Unicode and scratch boundaries, cross-source rejection,
zero-range and byte-identical no-op publication avoidance, and late
invalid/incomplete/NUL/read/write closure. They also cover
same-parent staging cleanup, failed-publication prospective-after cleanup,
deterministic temporary-name collision preservation, logical-path independence
for hard links, Unix basic-mode preservation across changed publication,
unavailable and no-follow sources, and large whitespace Lines without
unnecessary Paragraph state.
Anchor regressions cover Runtime-local opaque handles, direct structural
projection and nonstructural-range rejection, duplicate anchoring,
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
source-state/range mismatches, UTF-8/NUL observations, spill and admission safety, hard-link path
independence, stateless recovery, and Anchor non-mutation. Data regressions
cover native UTF-8 names, all seven typed Store/Get pairs, duplicate input
return including the exact owned View allocation, borrowed kind/name listing,
all-kind Rename/Remove dispatch, rename and remove priority, all three
CheckOutcome payloads, and no fixed entry or name-length cap.

Exact File Search regressions cover source-less logical-path validation,
empty/nonempty regular sources without content matching, missing and directory
Empty outcomes, named admission and unadmitted paths, private spill, symlink and
hard-link boundaries, invalid UTF-8/NUL closure, one ordinary v4 File result,
Check integration, and empty-File Apply at both `StartOf` and `EndOf`.

Check streaming regressions cover chunk-boundary UTF-8 and NUL validation; late
invalid, incomplete, NUL, and read failure; forward-only access; exact
hash/length equality; batch order and multiplicity; mixed hash/kind/range
inputs; duplicate occurrences; raw-valid nonstructural ranges; and Anchor
non-mutation.
Search streaming regressions cover target-specific File, Paragraph, and Line
projection; 8,191/8,192/8,193 scratch boundaries; split multibyte UTF-8, CRLF,
and literals; Line-scoped KMP; complete matching ranges; one-read incremental
SHA-256; shared same-source identity; File `FullLine` projection stop with
continued validation; and late text/read failure discard. The Line-only slice
fast path additionally covers one-byte queries and chunks, shorter/equal/longer
Lines, `aaaaab` and `ababaca` fallback/overlap, dense first-byte no-hit input,
cross-Line rejection, chunk-ending CR and every terminator, no-EOL, Unicode,
and a multi-scratch query. Exact File structure
audits prove one common observation and no generic Line framer. Ordinary View
regressions cover File/Paragraph/Line exact bytes, terminators, Unicode,
8,191/8,192/8,193 scratch boundaries, raw-valid nonstructural ranges, UTF-8
scalar-cut failure, every source-state mutation class, exact-state
reappearance, and late failure/resource discard. Its minimal Line relation
state preserves the optional Paragraph only for an exact current text Line.
Check regressions prove exact hash/length-only comparison while preserving
unsorted mixed geometry and duplicates. Anchor streaming regressions cover
direct File/Paragraph/Line projection without a View outcome, raw-valid
nonstructural rejection, selected anchored-binding success beside a stale
sibling, and exact-path fail-close on the selected mismatch. Anchored View uses
the ordinary direct target capture rather than a second parser.

Apply streaming regressions cover one retained no-follow source observation,
fixed-scratch staging readback, direct public v4 range geometry, incremental
prospective-after UTF-8/NUL/hash/length validation, one rename, and absence of
whole-source/after materialization. They cover 8,191/8,192/8,193 UTF-8 and
CR/LF/CRLF/no-EOL boundaries; exact File/Paragraph/Line and raw ranges; every
Position and forward/reverse Copy/Move direction; strict interior Move
rejection; zero-range, empty, boundary, and byte-identical no-ops; the seven
exact no-drift/edit-before/edit-after/adjacent/changed/equal-range/deleted Line
cells and duplicate Paragraph drift; late source read and staging write failure; temporary
collision/cleanup; Unix basic mode; and uncertain rename. Source-target
containment and overlap use only direct before ranges. The after projector
retains exact same-kind candidates and minimal original/copied/replacement
provenance, preserving unique rebind while removing split, join, absorption,
ambiguity, and collision cases. Copy follows only the original occurrence and
Position contributes geometry, never provenance.

Structural audits keep the forward observer, SHA-256, checked length, and text
policy in `source_scan.rs`; require one Apply source observation and staging as
the only readback input; and exclude `Natural`, `DecimalOrdinal`, private Apply
Anddress/Edit/Position, `LegacyResolver`, `ExactTargetTracker`, `SourceEvent`,
`SourceFramer`, `scan_source`, target extraction, source reopen/retry,
persistent observation, and source-sized complete before/after buffers from
production. Successful Anchor reflection remains allocation-free and
non-failing after all fallible planning completes.

Phase 7 benchmark verification exports v3
`399805906b352f1c8d0cc2fa0bbe6dee1a73a13c` and v4
`55999768cc7ad75ea84a08d597dbc7a7913fe6c3` directly from Git objects, builds
separate offline/locked stripped release and profiler binaries, and recreates
the exact Phase 2 fixture hashes under the fixed tmpfs task root. Each cell has
one warm-up and seven fresh processes or Sessions pinned to CPU 8. Exact
semantic output/order/count/final-source checks, deterministic output hashes,
empty stderr, wall/user/system time, HWM/RSS, `rchar`/`wchar`, throughput, and
million-hit bytes/HWM per result are required. Representative `perf stat` and
DWARF stack runs are profile evidence only and must have zero lost samples.

Formal gates require large View/Check and range Apply median CPU at most 75% of
v3, Search median wall and peak HWM at most 105%, every p95 and peak-memory
comparison below a 10% regression, bounded low-hit source memory, and no
consumer target search, relocation, second Search hash pass, whole-source
`CurrentObservation`, fixed truncation, or output/error drift. Those gates pass.
The Owner correction defines the million-hit recommendation as result-memory
peak-HWM slope, not JSON output size. Phase 7's 346.539 to 58.551 HWM bytes/hit
is an 83.10% reduction and passes the 50% recommendation; 327.470 JSON bytes/hit
is canonical Adapter payload/I/O evidence only. The remaining 2× 256 MiB Line
Search recommendation is closed by a Line-only maximal content-slice fast path.
Against the paired v3 673.898 ms median, its 272.111 ms median is a 2.476×
speedup and below the 336.949 ms target. Fixed current/candidate 128 MiB,
256 MiB, 2,048-file, and million-hit measurements preserve exact output and
all p95/peak-HWM gates. These gates supplied the source-readiness evidence for
the separately executed and now closed `0.2.0` publication.

Edit V1 semantic/public API/type/error authority and inert Rust value
implementation are complete; the single-source Apply Runtime execution
and its regressions are complete.
Value regressions cover every operation and position target-kind boundary,
source-less Anddress error mapping and field priority, exact empty/Unicode/CR/
LF/CRLF content, NUL rejection, and absence of relation or fixed-size
constraints. They use no filesystem or Runtime. Runtime regressions cover every
Edit operation and exact position boundary; same-coordinate/path validation;
source-state drift; strict Move interior rejection; raw and zero ranges; late
UTF-8/NUL/read/write failure; staging cleanup; and exact source/Anchor
preservation for direct no-ops. Changed operations compare fixed staging chunks
against generated output, validate the result incrementally, prepare direct
Line/Paragraph candidates, and publish only after the full Anchor plan exists.
They also cover source-target-only replacement and Move provenance, Copy's
original-occurrence rule, split removal, unique replacement rebinding,
Position-only geometry, no-EOL and terminator behavior, and reverse Move for
source-contained Line and Paragraph bindings.
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

Backwriter `0.2.0` release closure regenerated the four artifacts and sidecars
from source revision `2fad6e46d3a9d1da01f79f34b9ffc187447c76a8` and
reproduced the canonical 876-byte manifest with SHA-256
`b63589acd1c06606e62f08ea83dd1c2c36fbc5987665287218481f74b06a5cd4`.
The existing publisher added the eight `releases/0.2.0` files, atomically
replaced the POSIX and PowerShell installers, and replaced the manifest last.
It preserved metadata and bytes for all 16 beta.3/stable versioned files and
`install.cmd`; a complete rerun preserved metadata for all 28 files. All 28
files passed both loopback and public HTTPS GET and HEAD checks, for 56 GET and
56 HEAD responses with exact bodies, lengths, content types, and cache policy.
Root and unknown-path GET/HEAD checks remained 404/no-store. A task-local
canonical `curl | sh` fresh install and an actual public `0.1.0` binary's
explicit update installed byte-identical `0.2.0` Linux binaries. The installed
binary passed help, exact version, Search, View, Check, Session, empty-File
Apply, and duplicate-drift safe-rejection checks. Origin and cloudflared PID,
InvocationID, restart count, loopback listener, units, ingress YAML, credential
metadata, actual user HOME, and process PATH remained unchanged. macOS and
Windows artifacts received static cross-build verification only; no native
macOS, Windows, PowerShell, or CMD execution is claimed. No tag, GitHub Release,
crates.io publication, cache purge, service, tunnel, DNS, route, or credential
change occurred.

Backwriter `0.2.1` release closure regenerated the four artifacts and sidecars
from source revision `4a1b06fb375bfd906a6f27de4de15a8febfe08ec` and
reproduced the canonical 876-byte manifest with SHA-256
`04b111122f844bee17d40f68358386e6e64112ef9e3c2e7ef7547439586afc46`.
The existing publisher added the eight `releases/0.2.1` files, atomically
replaced the POSIX and PowerShell installers, and replaced the manifest last.
It preserved metadata and bytes for all 24 beta.3/stable/`0.2.0` versioned
files and `install.cmd`; a complete rerun preserved metadata for all 36 files.

All 36 files passed both loopback and public HTTPS GET and HEAD checks with
exact bodies, lengths, content types, zero HEAD bodies, and cache policy. Root
and unknown-path GET/HEAD checks remained 404/no-store. A task-local canonical
`curl | sh` fresh install and an actual public `0.2.0` binary's explicit update
installed byte-identical `0.2.1` Linux binaries and printed exact Installed and
Updated outcomes. The installed binary passed help, version, Search, View,
Check, Session, empty-File Apply, and duplicate-drift safe-rejection probes.

Closure passed 236 GNU-host tests, 236 musl tests, 13 Origin tests, 35 installer
regressions, 52 publisher regressions, 12 CMD regressions, and offline/locked
metadata, tree, formatting, all-target checking, clippy with warnings denied,
and release builds. Origin and cloudflared PID, InvocationID, restart count,
listener, units, ingress YAML, credential metadata, actual user HOME, process
PATH, and shell startup files remained unchanged. macOS and Windows artifacts
received static cross-build verification only; no native macOS, Windows,
PowerShell, or CMD execution is claimed. No tag, GitHub Release, crates.io
publication, cache purge, service, tunnel, DNS, route, or credential change
occurred.

Before staging, verify the diff and empty index, confirm repository-root
`.artext` is absent and untracked, and preserve unrelated task/history files.
Owner-authorized work then stages only the reviewed paths and repeats the
cached diff audit before commit.

The repository source package and source-built command are published and
closed `0.2.6`; the command prints exactly `Backwriter 0.2.6` plus LF. The
closed public distribution, R3-isolated installed release, installers, manifest, and
Update handoff are `0.2.6`; exact `0.2.5` is the only other accepted manifest.
Update has no version comparison and installs or reinstalls official `0.2.6`.
Prior `0.2.3`, `0.2.2`, `0.2.1`, `0.2.0`, `0.1.0`, and beta
versioned files remain immutable. The closed `0.2.1`
source suite passed 236 GNU-host and 236 musl Rust tests; the closed `0.2.0`
source suite passed 203 GNU-host Rust tests, and the historical `0.1.0` source
suite passed 193.

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
View regressions cover v4 decode, File/Paragraph/Line exact bytes,
None/LF/CR/CRLF terminators, large no-EOL output, stale source-state/range and
unadmitted source closure, plus one-shot anchored/extra-operand
rejection.
One-shot Search JSON regressions cover the exact compact v2 envelope and item
key order, Empty and Found mapping, exact v4 object embedding and re-decoding,
File/Paragraph/Line position shapes, CR/LF/CRLF/bare-CR/no-EOL framing, Unicode
and JSON escaping, result order, repeated Line content, global-option placement,
rejected duplicate/late or non-Search JSON, and a structural audit that excludes
a JSON Value or cloned result collection in the production writer.
The Search-to-Edit integration control removes only the fixed single-occurrence
v2 wrapper, validates the original embedded v4 bytes, passes them unchanged to
one-shot Edit, and verifies exact CRLF-preserving source output.
One-shot View JSON regressions cover exact compact envelope key order for File,
Paragraph, and Line; related v4 File/Paragraph object re-decoding; every Line
terminator including a separator Line's `paragraph:null`; Unicode and JSON
escaping; unchanged human projection; rejected duplicate/late/anchored/extra
forms; unavailable stdout/stderr closure; and a structural audit that excludes
a JSON Value, ViewOutcome clone, or result collection in the production writer.
One-shot Check JSON regressions cover exact compact Current, NotCurrent, and
Unavailable envelopes; direct v4 filtered-value re-decoding; File/Paragraph/Line
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
NotCurrent status, unavailable-source status, strict v4 decoding, and rejected
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

## 0.2.6 R3 release closure — GO

The [tracker's R3 evidence](../tasks/2026-09-04-backwriter-0.2.6-operational-adapter-verification-contraction.md#026-r3-release-closure--go-2026-09-05)
records pinned artifact reconstruction, the private 68-to-76-file trial,
manifest-last live publication and one idempotent rerun, exact file metadata
fingerprints, and unchanged services. Loopback/public GET/HEAD and error
boundaries pass 312 checks, including actual empty HEAD bodies. Isolated fresh
install, public `0.2.5` Update, `0.2.6` reinstall, and 17 recorded installer/CLI
commands pass exact binary, output, v5/receipt, CRLF, ordered Check, shell-ref,
stale nonpublication, and raw Session controls.

Production source and release code match the pinned passing inputs, so GNU/musl
285 tests each, installer 45, publisher 58, CMD 12, and Origin 13 are reused
prior results, not rerun suite claims. Native macOS/Windows/PowerShell/CMD and
publisher lock/rollback/fsync/crash-durability guarantees remain absent.
