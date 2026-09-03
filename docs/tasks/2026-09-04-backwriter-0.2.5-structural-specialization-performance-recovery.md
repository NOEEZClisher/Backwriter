# Backwriter 0.2.5 Structural Specialization and Performance Recovery

Status: Gates 1 through 4 complete. Bulk literal matching, raw/structural
observation, and canonical encoding reuse are implemented and verified. Cargo,
`bw version`, artifacts, installers, Update, and the public distribution
remain published and closed `0.2.4`.

This tracker resolves the planning questions preserved in the companion
[source note](2026-09-04-backwriter-0.2.5-structural-specialization-performance-recovery-source.md)
and [grounded roadmap](2026-09-04-backwriter-0.2.5-structural-specialization-performance-recovery-roadmap.md).
Those two documents remain input evidence. This tracker records the approved
authority where an input alternative conflicts with the closed `0.2.4`
contract.

## Governing rule

> Semantics stay unified. Execution becomes specialized again.

The shorter release definition is **unified authority, specialized hot
paths**. The target preserves v5 structure, wire, output, ordering,
multiplicity, errors, and failure boundaries while removing work that a
specific capability does not consume. It is a performance-recovery target,
not a structural redesign.

## Fixed evidence labels

- **A** is the published v4 comparison revision
  `195aaa37068122097ecc04d2644642b6afcc6765`.
- **B** is the closed `0.2.4` production revision
  `8b20987893ea5ac454c4c0a50d0c470e26b5e650`.
- The closed `0.2.4` release Source Authority is
  `0ee4dcce14da93f925c27a04d0e79051c83fd124`.
- **C** is the Gate 2 candidate built from
  `0b7fbbd9d06c0f2417374d428089232704c49b8b` plus the exact Gate 2 Search
  diff. A future comparison must not relabel A or B.
- **D** is the committed Gate 3 revision
  `042cc9e7f6dfe6faf23937367ec02446693a1d2d`.
- **E** is D plus the exact sampled Gate 4 production delta, SHA-256
  `e2dbdcf529f14009b9a4c6caefc88ace414feae10eaf2c0769b2d8ca471b162c`.

The B production baseline is 297,269 bytes and 8,954 lines. GNU and musl each
have a closed 258-test `0.2.4` result.

## Gate 1 decisions

### Source Line count remains currentness evidence

`sourceLineCount` remains part of v5 `SourceIdentity`, ordinary address
equality, Runtime `CurrentObservation`, Host proof, and View, Check, and Apply
source-state comparison. A same-hash, same-length typed value with a false Line
count remains `NotCurrent`; the existing manual-v5-mutation Safe Reject is not
narrowed.

This is the conservative choice where the source note proposed removing Line
count from proof/currentness. SHA-256 plus exact byte length identifies the
source bytes, but the active v5 contract also requires the address's claimed
derived Line count to agree with the accepted observation or trusted proof.
Gate 3 may remove Paragraph and parent-geometry work from raw consumers only by
counting Lines in the same forward read with a minimal raw accumulator that
does not own or invoke `StructuralCursor`. It may not stop deriving Line count,
weaken `NotCurrent`, or change v5 fields.

### Typed addresses are valid by construction

Safe Rust can construct an `Anddress` only through strict v5 decode or the sole
crate-private `AnddressIssuer`; private fields prevent caller mutation. Decode
and Issuer validation remain strict, and public `Anddress::validate()` remains
available and strict. Unsupported-version, encoding, invalid-geometry, and
resource classifications do not change.

Gate 4 removes repeated `validate()` only where production reachability proves
that the hot path accepts an already typed `Anddress` created by those
boundaries. It does not weaken wire decode, public explicit validation, Edit
validation, or any source-less error priority, and introduces no second
validator or unchecked wire-to-value path.

### One reusable canonical encoder is public

Gate 4 adds exactly this narrow library surface:

```rust
pub fn encode_into(&self, output: &mut Vec<u8>) -> Result<(), AnddressError>
```

The method clears `output` on entry, computes the complete required length with
checked arithmetic, and uses fallible reserve before appending canonical bytes.
An arithmetic or reserve failure returns `AnddressError::Resource` with
`output.len() == 0`; existing capacity may remain available to the caller.
After successful reserve, writing the already validated typed value is
infallible. Success replaces any previous contents with exactly one canonical
v5 object and no trailing bytes.

Existing `Anddress::encode()` remains public, creates one empty `Vec<u8>`,
delegates to `encode_into`, and returns that vector. The four exact v5 KAT byte
sequences, JSON escaping, field order, canonical decimals, and existing error
type remain unchanged. The `bw` binary may reuse one scratch vector across
results; it must not duplicate the canonical writer or retain a second result
collection.

## Audited current consumers

- `LiteralMatcher` and `SearchProjection` are retained because content Search
  consumes exact literal tiering, Line boundaries, ordering, and duplicates.
  Gate 2 replaces the per-byte caller loop with one equivalent segment path;
  it does not add a second matcher.
- `StructuralCursor` is retained as the sole complete Line/Paragraph framer for
  content Search and prospective output that actually needs receipt or Anchor
  geometry. Gate 3 separates raw byte-state observation without creating a
  second structural parser.
- `CurrentObservation`, `CurrentProof`, and `SourceProofEvidence` are retained
  by Untrusted and Host View, Check, Apply, Search proof installation, and
  invalidation. Their Line-count comparison remains authoritative.
- `AnddressIssuer` is retained as the sole ordinary-address construction
  boundary for Search, View projection, prospective Apply receipts, and Anchor
  reflection. Gate 4 validates its shared source once and each target geometry
  once; no capability-local constructor is permitted.
- Search's tier buckets and final sort are retained for deterministic global
  ordering. Gate 5 may replace only the monolithic provisional geometry store
  with storage that demonstrably releases consumed capacity.
- View batch source grouping is retained because it provides ordered,
  duplicate-preserving, all-or-nothing one-observation-per-source execution.
- Apply staging, prospective provenance, publication, Host-proof installation,
  and Anchor reflection are retained because unit Apply, Replace receipts, and
  live continuity consume their distinct failure and publication boundaries.
- Search and Apply currently duplicate Line-to-Paragraph attachment arithmetic.
  Gate 5 or 6 may move only that arithmetic behind one geometry-owned helper.
- CLI Search, View, Check, and Edit writers retain their Adapter schemas and
  exact bytes. Gate 4 replaces per-object allocation in Search and batch View
  with the one reusable encoder buffer and adds no JSON model or writer.

## Ordered gates

### Gate 1 — authority — complete

Close the Line-count, typed-validation, and reusable-encoder decisions; pin
baselines, thresholds, exclusions, and release separation. No production,
version, test, README, server, or public-state change occurs.

### Gate 2 — bulk literal matching — complete

Add one segment operation to the existing matcher and delete its per-byte
caller loop. Preserve literal semantics, tiering, all-or-nothing failure,
ordering, duplicates, UTF-8/NUL policy, and chunk/overflow boundaries. Measure
the fixed 256 MiB and 1 GiB sparse cells and the 1,048,576-hit native cell.

One `LiteralMatcher::push_segment` now owns checked bulk Line-length accounting,
zero-state first-byte skipping, carried KMP partial state, and stop-after-match.
The old `push`/`push_content_byte` pair and Runtime byte loop are deleted.
`SearchProjection` alone decides File/Paragraph tier saturation; the existing
cursor, source observation, provisional store, Issuer, tier buckets, and final
sort remain because their structural, failure, ownership, and ordering
consumers are unchanged. No `StructuralDemand`, cursor mode, parser, result
collection, or error type is added.

Exhaustive binary-alphabet tests compare byte-at-a-time, every possible segment
partition, and whole-segment matching. Focused controls cover absent/terminal
first bytes, one-byte and overlapping queries, `abab`/`ababa`, prefix/suffix
substrings, Unicode splits, cross-chunk partials, a 65,536-byte matched suffix,
all terminators, every target, 8,191/8,192/8,193-byte edges, checked overflow,
and late invalid source disposal. GNU and musl each pass 258 tests plus
all-target check, clippy `-D warnings`, and release build.

#### Fixed native evidence

Clean Git-object exports use A and B above. C uses the same base plus only
`src/backwriter/search.rs` and `src/runtime/search.rs` from the candidate. The
fixed fixture recipes are one Line of `x` followed by `needle\n` at exactly
256 MiB and 1 GiB, and exactly 1,048,576 copies of `needle\n`. Their SHA-256
values are respectively
`641f7442659ee50a6c5e183fd0a95963deaa21490ac2884215639ea704614d9e`,
`904c75499d4dc222f3df76ad0c2dcc397e0a163b56ed5c65692f65de7d67a162`,
and `913515a8747b7f1bf66a0e60d4f7d62aee87266faeffbf1aa60509d478c86b8c`.

The A and B/C native harness source SHA-256 values are
`84faa44b5a1605c0078c760c514d2df654cf43a92036ba2f45da2a246c7cf2a8`
and `65b796a3c7c63d0b70a78c02cccdf860c5c69ddaee2c9919c74d98f8fda64e7d`.
They differ only at the incompatible A occurrence carrier versus B/C direct
Anddress collection. The fixture generator, measurement orchestrator, C
`CLOCK_MONOTONIC_RAW` runner source, and compiled runner SHA-256 values are
`2769b13a75c07e92208ba9d2ad78a36509c294adf31e66b07651827b06e22868`,
`0902b126661704bf048eebd5c3b4dfc16c348a36fdb52b754face63bd1d6dee7`,
`46ca9b3191485898ff8936806c70bbcbd867305695c977ab1377b233f5f1a4e5`,
and `7b0b8e2f25cdd2883034694114a3b403233678358737236e736595d5eacb8b2a`.
A/B/C harness binary SHA-256 values are
`39cde5c87ca6c13b726b6d13fb6f198e845c151a3c7bec945aec7fe3d468fd8f`,
`8391816ad24bd6f2d0a4318a02e0f864b574f650f1737c6549c39f56711c461d`,
and `0d93d7ff5ff3eaf36555607d3493e1f6ef7d29df6d2dd0b9e8edcfe0ff811f2c`.

The host is Linux `7.2.2-arch1-1` on an Intel Core i7-12700K with Rust/Cargo
1.95.0, LLVM 22.1.2, CPU 0, the existing `powersave` governor, and `/tmp`
tmpfs. Each variant receives one warm-up and seven fresh-process samples in
crossed orders `ABC/CBA/BCA/ACB/CAB/BAC/ABC`. Inner time covers native
`WorkspaceRuntime::search`; the retained result is then traversed once without
a second collection to compute semantic and canonical-wire digests. HWM and
process I/O include that verification traversal. Nearest-rank p95 over seven
samples is the maximum.

| Fixture | Results | A median/p95 ms | B median/p95 ms | C median/p95 ms | A/B/C peak HWM KiB | C/A median/p95 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 256 MiB sparse | 1 | 265.839/284.172 | 379.335/380.655 | 302.766/304.684 | 2,324/2,308/2,316 | 1.1389/1.0722 |
| 1 GiB sparse | 1 | 1,046.983/1,057.311 | 1,495.575/1,505.156 | 1,192.754/1,196.308 | 2,360/2,360/2,332 | 1.1392/1.1315 |
| 1,048,576 Lines | 1,048,576 | 43.362/44.197 | 127.498/142.103 | 132.171/134.278 | 108,988/166,332/166,136 | 3.0480/3.0382 |

Raw inner milliseconds and HWM KiB, in each variant's execution-round order:

| Fixture / variant | Raw inner ms | Raw HWM KiB | `rchar` / `wchar` |
| --- | --- | --- | --- |
| 256 MiB A | 265.839, 266.722, 265.096, 265.368, 265.208, 284.172, 265.913 | 2040, 2180, 2244, 2324, 2168, 2160, 2160 | 268442042 / 188 |
| 256 MiB B | 380.655, 379.675, 376.356, 379.277, 378.473, 379.448, 379.335 | 2152, 2220, 2144, 2308, 2296, 2200, 2056 | 268441993 / 188 |
| 256 MiB C | 301.954, 301.651, 304.381, 304.608, 300.892, 304.684, 302.766 | 2068, 2316, 2172, 2300, 2172, 2312, 2248 | 268441993 / 188 |
| 1 GiB A | 1047.804, 1047.751, 1040.596, 1057.311, 1045.615, 1046.983, 1044.883 | 2324, 2360, 2252, 2152, 2308, 2316, 2160 | 1073748410 / 189 |
| 1 GiB B | 1496.897, 1491.814, 1505.156, 1495.575, 1495.139, 1494.941, 1498.056 | 2320, 2360, 2328, 2100, 2160, 2216, 2172 | 1073748361 / 189 |
| 1 GiB C | 1196.308, 1192.754, 1186.904, 1194.166, 1194.676, 1188.533, 1191.303 | 2072, 2300, 2332, 2332, 2120, 2172, 2152 | 1073748361 / 189 |
| Dense A | 44.197, 42.992, 43.303, 43.362, 43.556, 43.565, 43.142 | 108756, 108900, 108720, 108904, 108988, 108976, 108852 | 7346618 / 199 |
| Dense B | 126.119, 127.498, 127.711, 125.339, 127.909, 125.707, 142.103 | 166256, 166132, 166108, 166248, 166128, 166264, 166332 | 7346569 / 200 |
| Dense C | 132.171, 131.479, 130.224, 132.932, 134.278, 131.517, 133.520 | 166104, 166004, 166124, 166136, 165968, 166136, 166136 | 7346569 / 200 |

All A/B/C semantic order digests match per fixture:
`7c1543a3dd75740c7e69fc7dd3ea3687894a843cb3e22aa6d1cce4aa54c92e43`,
`38d02b0d6f9556df1f1ade5a30b6cb70cc69736740bc30fd3ef6a257b30fde9d`,
and `9aa0320348d16abc85e47b9533a2e59480b3d6ca3e3bf7a52e3ba00c0caac690`.
B/C canonical-wire digests match at
`55c2cb1d7b2bbe23b5d2b05f71452060bad8ac8c691a1058bc120981e4af8639`,
`fef9fa3fc86aa6d18bf9a34f1aa84fa8589f09df0074b3cef67a0186ff143a96`,
and `b547d3fbd6a59fc54789e25d41de6eb12fa5823a68f6818035417718f3044063`.
The three raw CSV SHA-256 values are
`33976cbfe73e87f97020b83332d881e8cd3d104e1b506f078179efeb425179e9`,
`ee544b2434621a409289b5949b59252d5d2e3988f24d203b5171987cde7644c0`,
and `27959f35dbaa58af979c500e4467efbcc0284a11b87536a754b66d5757c0c84f`.

The sparse median target of 1.10 is not met, but both cells pass the fixed 1.15
ceiling and every sparse p95 ratio also passes. Gate 2 is therefore **GO**
without conditional cursor specialization. Dense memory remains deliberately
unchanged and exceeds the later Gate 5 hard threshold; it is recorded as Gate
5 input rather than misclassified as a Gate 2 regression. Production is
298,222 bytes/8,981 lines, +953 bytes/+27 lines (0.32%/0.30%) over B. The growth
is one matcher operation and target saturation, below the allowed 3 percent
with direct semantic and measured evidence; Gate 6 must still contract the
final target to its fixed bound.

### Gate 3 — raw and structural observation

Gate 3 is complete. One raw `ObservationBuilder` owns UTF-8/NUL validation,
SHA-256, checked byte length, exact Line counting, and chunk delivery. Its safe
word-at-a-time ASCII path counts LF while detecting CR, NUL, and non-ASCII;
CR-containing chunks use the exact scalar CR/LF/CRLF rule, and non-ASCII or
split UTF-8 uses the existing incremental validator. The builder owns no
Paragraph, parent geometry, or `StructuralCursor`. One
`StructuralObservationBuilder` composes the raw state with the sole existing
cursor in the same source read and fail-closes if their length or Line count
differs.

`observe_source` is direct raw execution and no longer delegates to structural
observation. `validate_source_exact` uses only incremental UTF-8/NUL state,
checked offsets, the expected length, and one growth byte. Check proof misses,
ordinary/batch View, Apply before-state, and staging are raw. Content Search and
proof-miss Anchor retain structural framing; exact File Search is raw. Changed
Apply output is raw for unit Apply, File receipt, and File-only Anchor state,
and composes one cursor only when a non-File receipt or live non-File Anchor
needs geometry. Staging, publication, proof installation, receipts, and Anchor
reflection otherwise keep the existing executor and order.

Executable parity covers empty source, every terminator and no-EOL, all
three-symbol `x`/CR/LF sequences through length 10, split Unicode,
8,191/8,192/8,193-byte sources, one-byte reads, invalid UTF-8/NUL, callback and
late read failure, and forced length/Line-count overflow. Structural audits fix
zero cursors in the raw builder, raw observer, and exact validator, with one
cursor field and construction in the structural builder. Existing Check
order/duplicates, View all-or-none grouping, Apply no-op/publication/proof/
Anchor, stale/foreign/missing/unadmitted/symlink, and blind-drift controls stay
green. GNU and musl each pass 261 tests.

#### Fixed Gate 3 evidence

A is `195aaa37068122097ecc04d2644642b6afcc6765`, B is
`8b20987893ea5ac454c4c0a50d0c470e26b5e650`, C is committed Gate 2
`05c50802b7393a213147b8a2b52b2616b4b06bee`, and D is C plus only the Gate 3
candidate. The host and crossed orders remain CPU 0, Linux `7.2.2-arch1-1`,
Rust/Cargo 1.95.0, LLVM 22.1.2, `powersave`, `/tmp` tmpfs, one warm-up, seven
samples, and `ABCD/DCBA/BCDA/CADB/DABC/ACBD/BADC`; p95 is nearest-rank maximum.
The sparse fixture is exactly one Line of `x` followed by `needle\n` at 256
MiB, SHA-256
`641f7442659ee50a6c5e183fd0a95963deaa21490ac2884215639ea704614d9e`.
With no earlier density recipe, Gate 3 fixes the authorized assumption as
exactly 134,217,728 copies of `x\n` (256 MiB), SHA-256
`a3978b948296b92171d4b9ae213daf796b3d79e6bc40ccc6f5d3dfc03f66c2e4`.

| Cell | A/B/C/D median ms | A/B/C/D p95 ms | D/A median/p95 | A/B/C/D peak HWM KiB |
| --- | --- | --- | --- | --- |
| Host Check | 0.001/0.001/0.001/0.001 | 0.002/0.002/0.002/0.002 | 0.9345/1.3056 | 2,592/2,596/2,588/2,600 |
| Untrusted Check | 151.208/238.431/251.025/163.487 | 153.535/242.097/253.234/164.473 | 1.0812/1.0712 | 2,544/2,604/2,592/2,600 |
| Host self-Line View | 154.857/68.237/68.364/69.525 | 156.995/69.902/75.710/70.097 | 0.4490/0.4465 | 264,684/264,684/264,684/264,676 |
| Untrusted self-Line View | 531.818/270.417/267.883/199.133 | 534.426/274.056/269.846/210.527 | 0.3744/0.3939 | 264,676/264,684/264,684/264,680 |
| unit raw-after Apply | 211.653/301.910/298.813/223.974 | 227.078/303.846/313.524/228.144 | 1.0582/1.0047 | 2,608/2,604/2,604/2,600 |
| receipt Apply | 212.856/300.970/297.458/223.528 | 214.117/304.085/301.059/225.422 | 1.0501/1.0528 | 2,576/2,600/2,604/2,600 |
| live-Anchor Apply | 213.208/304.552/297.168/224.142 | 216.822/306.817/299.680/229.144 | 1.0513/1.0568 | 2,592/2,604/2,604/2,600 |
| short-Line Check | 150.146/492.255/502.830/164.386 | 152.425/511.928/507.370/164.929 | 1.0948/1.0820 | 2,608/2,596/2,596/2,592 |
| CRLF one-shot Edit | 2.227/1.718/2.268/2.291 | 2.360/2.341/2.362/2.369 | 1.0287/1.0039 | 2,680/2,664/2,756/2,736 |

Raw elapsed samples in variant order A/B/C/D are:

| Cell | A raw | B raw | C raw | D raw |
| --- | --- | --- | --- | --- |
| Host Check ns | 1420/1548/1481/1174/1198/1610/1227 | 1198/1409/2134/1463/1366/1694/1754 | 1229/2001/1239/1720/1189/1777/1294 | 1326/1385/2102/1327/1337/1259/1271 |
| Untrusted Check ms | 149.133/148.427/153.535/151.208/152.275/150.345/151.499 | 235.323/238.431/238.142/239.101/238.861/242.097/235.431 | 249.864/249.916/251.025/253.234/253.086/248.841/252.534 | 160.946/162.650/164.223/164.473/163.487/161.804/163.832 |
| Host View ms | 154.206/155.216/153.822/155.221/153.754/154.857/156.995 | 68.032/68.237/68.206/68.197/69.902/68.726/68.942 | 67.364/69.573/68.300/67.603/68.364/70.672/75.710 | 69.979/70.097/68.593/68.790/69.776/69.083/69.525 |
| Untrusted View ms | 533.011/534.426/528.009/530.053/530.068/534.399/531.818 | 270.417/274.056/270.746/266.278/268.977/271.883/270.035 | 264.497/263.549/267.883/268.280/269.846/264.509/268.547 | 199.133/210.527/197.511/199.509/196.551/195.961/199.457 |
| unit Apply ms | 212.488/211.355/213.682/211.653/211.114/210.712/227.078 | 301.948/301.910/300.990/297.515/303.846/299.227/303.122 | 313.524/298.815/295.272/295.952/297.543/298.813/300.808 | 223.974/222.356/223.991/223.322/222.730/224.366/228.144 |
| receipt Apply ms | 212.856/212.213/212.861/214.117/212.061/213.032/211.369 | 304.085/301.619/299.462/300.390/300.288/304.053/300.970 | 299.289/297.458/301.059/297.119/298.994/295.606/296.656 | 223.483/222.787/222.583/223.528/224.913/225.422/223.713 |
| Anchor Apply ms | 212.439/213.953/216.822/213.766/212.747/213.149/213.208 | 300.908/304.552/306.817/300.606/299.094/305.028/305.747 | 295.463/296.801/299.680/297.168/296.909/299.621/298.328 | 224.142/222.932/229.144/224.975/223.747/224.616/222.268 |
| density Check ms | 151.410/152.130/150.146/148.828/149.697/149.819/152.425 | 490.440/490.750/511.928/492.255/511.458/493.689/491.956 | 507.370/502.830/505.822/498.845/502.075/506.049/499.543 | 162.678/163.896/164.640/163.896/164.929/164.450/164.386 |
| CRLF Edit ms | 0.790/2.360/2.302/2.175/2.227/0.446/2.266 | 0.932/2.288/2.341/0.444/2.335/0.440/1.718 | 0.975/2.362/2.332/2.257/2.289/0.428/2.268 | 2.369/2.291/2.319/0.431/0.774/0.510/2.311 |

Process `rchar`/`wchar` ranges are 268,449,450–268,449,456/67 for Host
Check; 536,884,910–536,884,911/67 for Untrusted Check and both View cells;
536,884,926–536,884,928/268,435,531 for unit/receipt Apply;
805,320,390–805,320,395/268,435,531 for live-Anchor Apply;
536,884,914–536,884,915/67 for density; and A 6,547/356 versus B/C/D
6,529/540 for Edit. Host Check's process reads include the untimed Search that
installs proof; the measured Check itself has zero open, I/O, hash, and cursor
work by production structure. Every native semantic digest matches. All Apply
cells end at SHA-256
`7f8b1dfc466b6249f06cbe55c9174df2578e7754da793fded244ef5cba2a38f1`;
View matches the sparse fixture digest. CRLF Edit exits 0, keeps stderr empty,
preserves exact final SHA-256
`cc326fa86d3e5924c488283058e530b9413d6acec0f4f78a954882f85f92edbf`,
and retains each revision's exact receipt output.

Fixture-generator source/binary SHA-256 values are
`f355d73a5d5b896fc1e60bf072d03b302e4a1f63ede71b4ea147d9df90ed9aba`/
`5dc2b1351d79ce61b0dd0bdad927ea331b412e75508bcf323c4ccdb71c5d75d0`.
The A and B/C/D native harness source hashes are
`495b94185052e7f11c2501cfe8929e571b25f3241ed370841a4c5916cb07afb1` and
`803d799e93436e416402a0e1395c6893b2567ea6689b12b829ffeb8631eece7a`;
their binaries are respectively
`5696888d19c4f59b317338c713ef4b80a8ad0cff1aa726b2b9efed527cfb70e3`,
`6e46ab2dba3be990dcaac8bd366cde053f9a65687d20cf65dd1d6e2b0aa1be1c`,
`33a74ff3d761e36acb2ce7abf022041f57883e0298878e340f223e737662681d`,
and `2cd579038f34a8683918443c6d422c2da1bf47a24ebe9e79c87961f557cf7e96`.
The C raw-clock runner source/binary, orchestrator, and analyzer hashes are
`7ee8e59f551b19ad715efc7781e136cd1492beade0f50e1e4d456b818620d2c6`,
`ffa2752db8ad0670e1593b66a719ba577321046944f603fe057005c622854ac0`,
`fc5d4b81b9ef20f558c5f84b63c7d79927fb05fb2e15e608dc8a9f18a7aa0db1`,
and `aa48c1dc8b714154f8c9b5f2f63f45860ca5cb8cb1f698f4e8b10d71843ea7fa`.
Native/Edit CSV hashes are
`d16c8eaf2992e6dff787bc844da7e3cdb6bb7e00813ed355018f3669b1c0b5a8`
and `bd1446554c9c9e09c2dfbb7bc87440c988b42f86f3404d0ba2de1192829575da`.

Gate 3 is **GO**. Untrusted Check, all three Range Apply forms, density, and
CRLF Edit pass their fixed limits. Host Check remains approximately one
microsecond and I/O-free, and View exactness/HWM are recorded without an
invented latency gate.
Production is 304,463 bytes/9,166 lines: +7,194/+212 (2.42%/2.37%) over B,
inside the three-percent direct-evidence allowance. Gate 6 still owns
contraction to the final baseline target; Gate 3 adds no second parser,
validator, writer, dependency, public API, or compatibility path.

### Gate 4 — issuance and encoding — complete

The sole Issuer validates a shared source before constructing it, and validates
each issued target geometry against that source. Strict v5 decode, public
`validate()`, Edit validation, and Runtime Apply's defensive validation
remain. View, Check, and Anchor delete only repeated validation of an already
typed value whose safe Rust origins are strict decode or that Issuer.

One private canonical emitter serves public `encode_into` and delegating
`encode`. It first counts exact bytes with checked arithmetic, fallibly
reserves the caller vector, then writes through the same field-order and escape
path. The output is cleared before counting or reserve; any counting or reserve
failure leaves length zero. Valid logical paths require escaping only quote;
Unicode bytes remain exact, and decimal fields use one fixed stack buffer
without intermediate `String`, JSON `Value`, or serde model. Search and batch
View each reuse one operation-local scratch vector. Single-result Edit and
Check retain their one-address `encode()` paths because neither has a result
loop to amortize.

All four exact v5 KATs, round trips, prefilled output replacement, reusable
capacity, Unicode and quote escaping, decimal bounds, invalid path controls,
and construction/writer singularity pass. GNU passes 263 tests; the same suite
and all required target verification pass on musl. Adapter schemas, exact
bytes, order, multiplicity, ordinals, extents, failure classification, and
stdout behavior are unchanged.

The fixed Gate 4 host is Linux 7.2.2-arch1-1 x86_64 on an Intel i7-12700K,
CPU 0, `powersave`, Rust/Cargo 1.95.0, and tmpfs. After one warm-up, seven
crossed D/E samples use nearest-rank p95. The dense fixture is 1,048,576 copies
of `needle\n`, 7,340,032 bytes, SHA-256
`913515a8747b7f1bf66a0e60d4f7d62aee87266faeffbf1aa60509d478c86b8c`.
The File fixture has 200,000 one-byte sources named `000000` through
`199999`; its NUL-separated name-list SHA-256 is
`e278da996ea260823006584224bc6b9dbbbf9080eabbb70ea2712d8f3874bb66`.

| Encoder cell | D median/p95 | E median/p95 | D/E allocations per result | Exact output |
| --- | ---: | ---: | ---: | --- |
| one canonical Line | 4,824/6,408 ns | 3,283/4,771 ns | 30/0 | 394 bytes |
| 1,048,576 same-source Lines | 698.879/701.466 ns | 256.217/256.510 ns | 30/0 | 443,422,586 bytes |
| 1,048,576 distinct-source Files | 496.660/500.301 ns | 166.140/170.774 ns | 17/0 | 319,753,146 bytes |

The one, Line, and File output SHA-256 values are respectively
`eba96c306ecf6f8b0849b7377065e03a7088527041d08264f02ed2b5b704fdb9`,
`9d1d7a6c53631f68df685d84109f1b1a53b9a8d805626c4f888bd7fa2ad05644`,
and `d8bc2921edbe745e2f45980d722506991c5c874b21de7eeab5fe20498ad59ed3`
for both D and E. Peak scratch capacity falls from 520/521/525 bytes to
394/425/306 bytes. Allocation counts cover only the canonical result encoder;
they do not claim zero allocation for the CLI or Runtime.

| Consumer cell | D median/p95 | E median/p95 | D/E peak HWM KiB | Exact result |
| --- | ---: | ---: | ---: | --- |
| CLI Search, 1,048,576 JSON results | 0.85/0.87 s | 0.21/0.22 s | 166,544/166,544 | 630,800,294 bytes |
| native Search, 200,000 Files | 545.077/549.280 ms | 522.292/531.186 ms | 126,588/126,716 | 200,000 results |
| batch View, 200,000 Files | 669.651/673.911 ms | 659.558/666.399 ms | 162,816/163,244 | 200,000 results |

CLI output is byte-identical at SHA-256
`fcaceecf33c02bc382a25cce862dff97145f4c1941f04b0c52a269068672890a`;
its roughly 630 MB stream is Adapter output, not native engine memory. Native
Search and batch View semantic-output SHA-256 values are
`289912c04cc30f3a11126683a421cf8431f7d386ca89821749567887c019082e`
and `bcb333381293bb186f643565298d36783f0bd1913aaf3f8c737bd5cc5e02f958`.
A full sequential View pass over all 200,000 results also matches. The common,
D encoder, E encoder, and native harness source SHA-256 values are
`ff7c42c630be5b327185c7d75943233833cf90acfb826f1dc8b9c9a628de11fc`,
`81f283af66d2e1f0919de9bbc84db025a110eb5b61ca6e514945fd381bfc3b56`,
`ad09000cd2d812fd130f9121cc0a20fc04deb5f9281d0e522e8ffb28d90a8b17`,
and `021ed1e51a980c0f81b2108c1c6b7ec0dbc65d88300f02ab0768a0baac7b256c`.
The raw evidence SHA-256 is
`06868c27fe268806a927f31d907ec51bf36ecfeed8fa7508e8f6705a5f69dc2a`;
all task-local evidence is removed after verification.

Final verification removes one immediately dereferenced borrow in the CLI View
writer for clippy and adds one test-only checked-overflow assertion after E was
sampled. Neither changes an encoder, Search, native View measurement path or
any output byte.

Gate 4 production is 304,475 bytes and 9,197 lines, +12 bytes/+31 lines over
Gate 3 and +7,206/+243 (2.42%/2.71%) over B. This remains inside the existing
three-percent direct-evidence allowance and does not renew it. Gate 6 still
owns contraction to the final baseline target. Gate 4 adds no parser, validator,
parallel writer, dependency, schema, or compatibility path.

### Gate 5 — chunked pending memory — complete

Search replaces only its monolithic provisional geometry vector with a
Search-local store of fixed 16,384-entry chunks plus one global length. Push is
fallible; Paragraph result start/end remain global indexes; attachment visits
only the chunks intersecting `[start, end)`. A Paragraph spanning chunks is
promoted exactly, while a matched separator after its last content Line remains
a File child. After structural observation and source identity complete, the
sole Issuer consumes chunks in insertion order and each consumed allocation is
dropped before the next. Tier buckets, final sort, duplicates, multiplicity,
all-or-none errors, one cursor, and one Issuer remain unchanged.

One crate-private geometry helper now owns Line-kind recognition, Paragraph
containment, checked File-to-Paragraph Line offset, parent assignment, and
pre-mutation invariants. Search maps an impossible attached result to
`InvalidSource`; Apply keeps non-Line and outside candidates unattached and maps
invalid prospective geometry through its existing preparation failure. Existing
receipt and Anchor regressions prove the same prospective state and failure
boundaries. No second projector, result store, parser, or public surface is
introduced.

The fixed E/F measurement compares committed Gate 4
E=`caa17fefa7394553a7fe4edfccea03b64245dd61` with the Gate 5 candidate on
Linux 7.2.2-arch1-1 x86_64, Intel i7-12700K CPU 0, `powersave`, tmpfs, and
Rust/Cargo 1.95.0. One warm-up precedes seven crossed fresh processes; `perf`
duration and `/usr/bin/time` HWM use nearest-rank p95. The huge Paragraph is
1,048,576 copies of `needle\n`, 7,340,032 bytes, SHA-256
`913515a8747b7f1bf66a0e60d4f7d62aee87266faeffbf1aa60509d478c86b8c`.
The many-Paragraph fixture is 1,048,576 copies of `needle\n\n`, 8,388,608
bytes, SHA-256
`7e0d3b4cb91c4ed44f5a43986c70dca6b2ad8e1b33a214fb0c4dd6f311674464`.

| Native Line Search | E median/p95 ms | F median/p95 ms | E/F p95 HWM KiB | E/F I/O |
| --- | ---: | ---: | ---: | ---: |
| one huge Paragraph | 77.892/79.547 | 89.234/95.662 | 166,192/87,816 | 0/0 |
| 1,048,576 one-Line Paragraphs | 81.106/86.370 | 92.087/93.502 | 166,200/87,864 | 0/0 |

Both shapes return 1,048,576 results with exact global order and multiplicity.
Their E/F newline-separated canonical v5 transcript SHA-256 values are
`8a0757469aaca90c84fb6807037b2d269c8fe277fbf7fe2023f6e4b6cb4ed0a3`
and `22d7ef161bad25c4d0d86c53b526a5f3a7809bbb4df648336fe8bcb6801c12b9`.
The same-fixture E/F CLI Search stream is byte-identical at 628,703,142 bytes
and SHA-256
`969c02da926c3199a15b4c34506bb059d6044e21ea0c6b157e56a62490927269`;
the two-item Unicode/CRLF batch View stream is byte-identical at 1,153 bytes and
SHA-256
`dd5113b80085326b8da2564432d9b309b864d98bd475d564e77ac0caf639cb75`.
The harness source is
`764635b11096c5692bd6f1ad1df9fe62096d9730ba214790d2b0d8c7e0d1938b`;
raw performance evidence is
`0abe220fb0e8be1a856f9e408770186049a88e872caafc4099968b1f97f5f245`.
At maximum load 64 pending chunks exist before issuance; consuming one drops
16,384 provisional targets before continuing. Both p95 peaks are below 86 MiB,
well under the 130 MiB target and 140/145 MiB gates. The conditional shared
Paragraph `Arc` is therefore not tested or introduced. The candidate production
measure is 306,158 bytes/9,261 lines versus Gate 4's 304,475/9,197 and B's
297,269/8,954. The byte measure remains within the original cumulative three
percent envelope; the Line measure does not create a refreshed allowance, and
Gate 6 still owns contraction to the fixed final target.

GNU and musl each pass all 268 tests, all-target check, clippy with warnings
denied, and release build. Exact v5 KATs, every terminator and Unicode framing,
late read/UTF-8/NUL fail-all, receipt/Anchor parity, and Correct 1 / Safe Reject
6 / Wrong Apply 0 remain green. Cargo, CLI syntax/schema, version, dependency,
server, public distribution, and release state remain `0.2.4` and unchanged.

### Gate 6 — consumer reaudit and contraction

Reaudit production reachability after Gates 2–5, remove dead validation,
observation, and writer plumbing, and move Search and Apply to one Paragraph
attachment helper. Add no feature or generic framework.

### Gate 7 — fixed evidence and source readiness

Run complete GNU/musl semantics and crossed fixed A/B/C measurement. Record
source/binary revisions, fixture and harness hashes, CPU conditions, raw
samples, medians, p95, HWM, I/O, allocations, output hashes, and code-size
delta. Only a complete GO may advance Cargo, lockfile, README, version KAT, and
`bw version` to source-ready unpublished `0.2.5`. NO-GO leaves `0.2.4` current.

### Gate 8 — separately authorized release

Artifact reconstruction, installer allowlist, publisher, live publication,
endpoint/install/update verification, and release closure require a new exact
Owner authorization. Gate 7 source readiness does not authorize Gate 8.

## Fixed acceptance gates

- Sparse native Search uses the fixed 256 MiB and 1 GiB fixtures. C/A target is
  at most 1.10 and the allowed ceiling is 1.15. A result above 1.15 may activate
  only a measured optimization inside the sole cursor; it does not authorize a
  second parser.
- Dense Search uses exactly 1,048,576 hits. B peak RSS is 166,488 KiB; C target
  is at most 130 MiB, soft gate at most 140 MiB, and hard NO-GO above 145 MiB.
  Result count, order, multiplicity, and output digest must be exact.
- CRLF one-shot Edit C/A target is at most 1.20 and hard ceiling 1.25. It must
  retain zero private View/Search/Check calls, one `apply_replace`, every Line
  terminator, fresh receipt behavior, stale-old-address rejection, and zero
  Wrong Apply.
- Host Check proof hit has zero logical I/O, open, hash, and cursor work and
  retains its approximately one-microsecond class. Untrusted Check performs one
  open, one
  forward read, one UTF-8/NUL validation, one SHA-256, one byte count, and one
  Line-count accumulator per source, with zero cursor and within 10 percent of
  `0.2.3`.
- Host View remains project plus exact-range seek/read. Untrusted View uses one
  complete raw observation and range capture with no Paragraph, relation,
  event, or address-reconstruction work.
- A 256 MiB Range Apply has no before cursor; its after cursor runs at most once
  and only for receipt or live-Anchor geometry, within 10 percent of `0.2.3`.
- Encoder measurements record allocations/result, ns/result, output bytes, and
  peak scratch capacity. The canonical writer and all exact KAT bytes remain
  singular and unchanged.
- The 200,000-file Search and View-batch cells preserve exact ordering,
  duplicates, source grouping, output, and one accepted observation per source.
- The drift matrix remains Correct 1 / Safe Reject 6 / Wrong Apply 0. Stale,
  foreign, missing, unadmitted, invalid UTF-8, NUL, symlink, publication, and
  writer failure remain fail-closed.
- Final production target is no larger than 297,269 bytes and 8,954 lines.
  Growth up to 3 percent requires direct evidence; any duplicate parser,
  validator, or writer is a hard NO-GO.

Native Search and CLI Search are measured separately. The roughly 629 MB
million-result JSON stream is Adapter output volume, not native engine memory.

## Conditional decisions

`StructuralDemand`, cursor specialization, a shared Paragraph `Arc`, and the
pending chunk size are not Gate 1 implementation choices. Cursor work is
considered only if Gate 2 exceeds the 1.15 sparse ceiling. A shared Paragraph
allocation is considered only if chunking misses the dense target and must
improve both one huge Paragraph and many one-Line Paragraphs without a material
regression. Chunk size is selected from measured behavior, not fixed at 4,096
by this authority.

## Exclusions

No gate may add v6, remove or reinterpret a v5 field, alter Search/View/Edit
schemas or bytes, restore Search position/occurrence carriers, a View relation
scanner, or private Edit View, add a capability parser, persistent source
dictionary, state, index, registry, stdin transport, CLI split, history,
relocation, watcher, merge, retry, rollback, or compatibility path.

Gate 1 changes documentation only. Server, public root, services, cloudflared,
DNS, tunnel, credentials, actual HOME, artifact, release, and deployment state
remain outside the target until separately authorized.

## Gate 6 input

Gate 6 starts from the measured 16,384-entry pending chunks and single
geometry-owned Paragraph attachment helper. It must audit production
reachability, remove remaining dead or duplicate plumbing, retain every actual
consumer and exact output, and contract toward the fixed B production target.
It does not reopen chunk size or shared Paragraph allocation and adds no feature.
