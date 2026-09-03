# Backwriter 0.2.5 Structural Specialization and Performance Recovery

Status: Gates 1 through 7 complete; source readiness is GO. Bulk literal
matching, raw/structural observation, canonical encoding reuse, dense pending
memory, consumer contraction, and integrated evidence are complete. Cargo and
`bw version` are source-ready, unpublished `0.2.5`; artifacts, installers,
Update, and the public distribution remain published and closed `0.2.4`.

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
- **F** is the committed Gate 5 revision
  `5fbd6886758533ec887a2783de80361cebb60e31`.
- **G** is F plus only the Gate 6 production contraction.

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

### Gate 6 — consumer reaudit and contraction — complete

The reachability audit removes only carriers or branches with no remaining
consumer. Raw and structural observation constructors and Apply output
construction are infallible, so their false `Result` layers disappear.
`ObservationBuilder::push` and `StructuralCursor::push` no longer return
duplicate offsets; the raw callback still receives its checked byte start, and
structural finish returns only the Line count that the raw observation checks.
Anchor target projection always consumed every input, so its duplicate
all-index vector and allocation are removed.

`Anddress::same_source` directly serves Check/View grouping and Apply
same-source validation. The one Runtime source-state comparator serves
observation and Host-proof checks, while Apply's delegating state/proof wrappers
are removed. File and Paragraph Search cannot execute in one projection, so
one best-tier slot replaces two mutually exclusive fields. Crate-private
Issuer construction delegates source validation and `Arc` creation to the
same owned-source path used by strict decode.

The audit retains Check's owned-input status grouping and View's projected
optional-outcome grouping because they have distinct resource errors and
result restoration consumers. Apply's raw/structural output enum remains
demanded by File-only versus receipt/live-Anchor after-state work.
`AfterProjector`, staging/publication/provenance, Host proof, Anchor
reflection, Gate 2 matcher, raw Line counter, Gate 5 16,384-entry chunks, final
Search sort, result buckets, and the canonical writer each retain a direct
behavioral or output consumer.

GNU and musl each pass 268 tests. V5 KAT/no-v4, every terminator and
8,191/8,192/8,193 boundary, Search tier/order/duplicates, View single/batch
all-or-none, false-Line-count `NotCurrent`, receipts, publication, Host proof,
Anchor, and Correct 1 / Safe Reject 6 / Wrong Apply 0 remain green.
Production G is 304,431 bytes/9,213 lines. That is -1,727/-48 from F and
+7,162/+259 (2.41%/2.89%) from B, below the fixed
306,187-byte/9,222-line ceiling. B remains the target; the retained difference
has the consumers above and creates no renewed allowance.

Gate 6 changes source-observation plumbing, so focused current-G measurement
reuses the Gate 3 256 MiB sparse and 134,217,728-short-Line fixtures, their
fixed SHA-256 values, CPU 0, `powersave`, `/tmp` tmpfs, one warm-up, and seven
fresh processes. It does not repeat the full Gate 7 matrix.

| Focused G cell | G median/p95 ms | G/D median/p95 |
| --- | ---: | ---: |
| Host Check | 0.001/0.002 | 1.0000/1.0000 |
| Untrusted Check | 169.623/170.436 | 1.0375/1.0363 |
| unit raw-after Apply | 233.491/235.467 | 1.0425/1.0321 |
| receipt Apply | 234.161/235.633 | 1.0476/1.0453 |
| live-Anchor Apply | 233.390/237.392 | 1.0413/1.0360 |
| short-Line Check | 171.756/175.116 | 1.0448/1.0618 |
| CRLF one-shot Edit | 0.954/1.100 | 0.4164/0.4643 |

Every sparse/dense fixture hash, Apply final bytes, and CRLF fixed output hash
matches. Host Check retains zero capability open/read/hash/cursor work by
production structure. One focused 200,000 one-byte File run returns exact
ordered Search results from `d000/f000.txt` through `d199/f999.txt`; batch and
sequential View both equal all 200,000 addresses and contents. Search/batch/
sequential inner times are 556.213/711.319/675.253 ms and process HWM is
159,552 KiB. The task-local harness source/binary SHA-256 values are
`b8b4114095a9a99f3aa9046b43794e3d31a019e80ea8807e74f5a0831ce04d94`/
`44d915394ddf2737598bb281975421dec4906c9910365d6e27f242a2700b47e6`.
All focused fixtures, raw evidence, binaries, and harness source are removed.

### Gate 7 — fixed evidence and source readiness — complete, GO

Gate 7 changes no production `src/**`. Clean Git exports compare published v4
A=`195aaa37068122097ecc04d2644642b6afcc6765`, closed `0.2.4`
B=`8b20987893ea5ac454c4c0a50d0c470e26b5e650`, and contracted candidate
G=`22e6df23755cdc80b299b77be313d307b67bc37f`. Their release `bw` binaries are
780,016/800,760/795,312 bytes with SHA-256
`bd4aee49b531a525cc1375509d3d068e32538c061e84828f797f62101dc64a6e`,
`68fba45ddee9d481213f5555d77ffa2b2a309e21a1ebc2c12ac45a6f29f2b105`,
and `90445f1a7f271327deb84bcafdcc87010bd5f1da024bafdac01a636c04a9bd35`.
A/B lockfiles hash to
`71462aff768f45fea9d4e730f7ec9c1fca389dde132882049c7eae63acd9fac9`;
G hashes to
`8fa6e2baf598162f1173e35ba1a1df455bc4bfff0cd762339f0983576d7fac9d`.

The fixed host is Linux 7.2.2-arch1-1 x86_64 on an Intel i7-12700K, CPU 0,
existing `powersave` governor, `/tmp` tmpfs, Rust 1.95.0
`59807616e1fa2540724bfbac14d7976d7e4a3860`, Cargo 1.95.0, and LLVM 22.1.2.
Harnesses use `CLOCK_MONOTONIC_RAW`; every performance cell has one warm-up
then seven fresh processes in orders `ABG/GBA/BGA/AGB/GAB/BAG/ABG`. Median is
sample four after sorting and nearest-rank p95 is sample seven. Native Search,
CLI Search output, capability inner work, and process HWM remain separate.

The task-local fixture generator source/output SHA-256 values are
`43ed7c350727c8009e2d07bbc60f3d62e51f94dd6501b20aa131ba249ce04e56`/
`6e59376fe2c6a364e18ca3b4ed1a8dd919dc108abaf78205e9429db050da9a00`.
The fixed fixtures are:

| Fixture | Exact bytes or entries | SHA-256 |
| --- | ---: | --- |
| sparse 256 MiB | 268,435,456 | `641f7442659ee50a6c5e183fd0a95963deaa21490ac2884215639ea704614d9e` |
| sparse 1 GiB | 1,073,741,824 | `904c75499d4dc222f3df76ad0c2dcc397e0a163b56ed5c65692f65de7d67a162` |
| one huge Paragraph | 7,340,032 | `913515a8747b7f1bf66a0e60d4f7d62aee87266faeffbf1aa60509d478c86b8c` |
| 1,048,576 one-Line Paragraphs | 8,388,608 | `7e0d3b4cb91c4ed44f5a43986c70dca6b2ad8e1b33a214fb0c4dd6f311674464` |
| 134,217,728 `x\n` Lines | 268,435,456 | `a3978b948296b92171d4b9ae213daf796b3d79e6bc40ccc6f5d3dfc03f66c2e4` |
| 200,000 one-byte Files | `d000/f000.txt` through `d199/f999.txt` | path transcript `a89a87e469e7226b8ef6e66aa29541e43e0cb8081eeea7c6c0cac3e03b64961b` |

The native Search source/runner/raw hashes are
`c2a251b4272ce1c509979ad86a00002244214b1cdaebb2865ce79e62de21205f`/
`a4e8bdb40b225236eeabdad2fbe2b93f6b07cbc8a6b763db64a269e6c1660d20`/
`41dddcc34c1a87251589ac0fe586746c4c16971de1dda856d8ad7ae4b2266002`;
its A/B/G binaries hash to
`9256b41ab063390c10785bd3f26b65291e2660d810ca9231d4c39888849c3577`,
`168c216ef334694b6c1768435a92e1fac61817131d39f7ac30528001b9e3b052`,
and `fb968426532db0e1c72f36566d120bf2e1e35d4c1c2f7ac3a183dc2ed7f1fbf0`.
The exact invocation is `taskset -c 0 <harness> <mode> <absolute-fixture>`.

| Native Search | Results | A median/p95 ms | B median/p95 ms | G median/p95 ms | G/A median/p95 | G peak KiB |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| sparse 256 MiB | 1 | 288.525/292.386 | 393.078/414.432 | 316.893/326.381 | 1.0983/1.1163 | 4,164 |
| sparse 1 GiB | 1 | 1,137.686/1,146.247 | 1,552.835/1,634.016 | 1,246.792/1,259.080 | 1.0959/1.0984 | 4,168 |
| one huge Paragraph | 1,048,576 | 55.563/57.546 | 138.352/149.968 | 74.340/78.340 | 1.3379/1.3614 | 87,924 |
| one-Line Paragraphs | 1,048,576 | 58.800/60.600 | 137.866/139.658 | 77.032/78.860 | 1.3101/1.3013 | 87,992 |
| 200,000 Files | 200,000 | 536.829/541.906 | 561.692/568.090 | 537.663/541.388 | 1.0016/0.9990 | 115,008 |

The sparse semantic digests are
`50255d71ba5c9cd160f2d59b5aa645362a542b26994257bda3e19e7b9122288a`
and `14ab908725d29b83707f6e384cecbcae5e5236f4cb94842f6dc1540ab56df192`;
their B/G exact v5 digests are
`8ec4c967d2b6043233247a574379c3f4327de6c8ce1e0794ada4d9e69d7ead7a`
and `bb30a83f9b3975d554eb509878fc38cb7a7747b3c9977e228c0bc409f25982df`.
The huge/many Paragraph semantic digests are
`d94e3077a8382b492c3088bf13aa60188d21de3f1785606ad9db36fd15fa3bc7`/
`e166557ba04fb29ae32422b5e6918cbf23f1a5365ce86db8209ede962fb517a6`;
B/G exact v5 digests are
`06fae2ce697682eb4ed7681166bb3f6f56290acead7649766b380f9889978460`/
`87c3ed3ad92d9dbbdc2566158def048855b3e17fe4782b7a7ff7cb81b2593460`.
All native `wchar` values are zero. Exact `rchar` is source size plus harness
accounting; G has 49 additional accounting bytes and no additional source
pass. The 256 MiB and 1 GiB sparse elapsed-ns samples are:

```text
256 A 289893739 291878472 288213212 285467091 287996067 292385512 288525289
256 B 398961990 414431999 391044185 393164810 392354115 393077677 389882050
256 G 323235683 326381468 316212422 320297235 316893292 315098600 316375448
1GiB A 1146247327 1137686203 1134400910 1131000386 1137226690 1144565794 1137914033
1GiB B 1552834928 1550409561 1550161289 1553493349 1560776669 1634015854 1550921862
1GiB G 1250520125 1245877340 1246781601 1242535198 1250922399 1259080319 1246791861
```

The CLI process runner source/binary, orchestration, and raw evidence hash to
`a997ee52260a52d8d60418acac01224b9e862e57b5fadb45c5883fd39b6bc49f`,
`da1ef7e0168d6625ecb8d44f8e18a72c1279d049512288dd593624b900cb87b9`,
`4fdf5d4922ddce7f10d7799fad02c72f786d51a02c309201d25d1847e5fc1633`,
and `3ebee7254a8da42565456e73359229dec62cedc92ee069f293db896738b49293`.
The exact child argv is `<bw> --workspace <dense-root> --json search line
needle --source note.txt`. A/B/G median/p95 times are
446.863/452.965, 898.330/914.377, and 244.151/248.670 ms; peak HWM is
109,360/166,476/88,212 KiB. A writes 414,856,172 bytes with SHA-256
`2e17aed191006dcb5e41ec04e5b1bc78d030bb51ddf005d61f5f5cc110bb3cc1`;
B/G write exact-equal 628,703,142-byte v5 output with SHA-256
`11f65c4c82a23b4ec3c827b1cdd65ce9ce80f89fd3dfc64550863a3a50999f8f`.
That output volume is not native engine memory.

The Check/View/Apply harness source/runner/raw evidence hashes are
`d477b9f64b5dc3496ab69f1a4e8c79425322cbac15e176fa8d9be7c6af414e8d`/
`0e1ef40413cb68293e57bb00919c038e82448bcf99dbcf0fd046b6f34112d217`/
`c2b542dd6c070d07b63273f4373a6d3bc39e128afacffe2cde4dd7b8f88bbb97`;
A/B/G binaries hash to
`269b6dd136f815dac65174e0fcb2d0804401f930d7e3526411e529927a23a144`,
`1e1c80646a721ca7eaa6607ab11e0b8d18fb0b9211e79bc47239614c032a532c`,
and `05e48c3177406322385beb18b910d5b2fecd7c1f4522232745caa203cb407d32`.

| Capability cell | A median/p95 ms | B median/p95 ms | G median/p95 ms | G/A median/p95 |
| --- | ---: | ---: | ---: | ---: |
| Host Check | 0.002/0.002 | 0.002/0.003 | 0.001/0.002 | 0.5650/0.9259 |
| Untrusted Check | 159.390/161.415 | 252.278/252.501 | 172.202/172.533 | 1.0804/1.0689 |
| Host Line View | 405.969/407.279 | 315.613/316.202 | 316.427/317.328 | 0.7794/0.7791 |
| Host Paragraph View | 472.054/478.722 | 316.369/316.987 | 316.089/322.304 | 0.6696/0.6733 |
| Host File View | 317.601/320.531 | 315.840/318.745 | 316.257/320.375 | 0.9958/0.9995 |
| Untrusted Line View | 800.104/801.539 | 547.558/553.078 | 450.259/452.597 | 0.5628/0.5647 |
| Untrusted Paragraph View | 816.323/823.000 | 547.421/554.241 | 448.866/455.379 | 0.5499/0.5533 |
| Untrusted File View | 444.817/447.473 | 549.471/551.248 | 449.371/450.839 | 1.0102/1.0075 |
| RelationAbsent | 0.007/0.009 | 0.002/0.003 | 0.002/0.004 | 0.3206/0.4232 |

Host Check production structure has zero capability open/read/hash/cursor
work; the 102 measured `rchar` bytes are the harness's two `/proc/self/io`
reads. Untrusted Check and every View read exactly 268,435,558 `rchar` bytes
except A's Host Paragraph View, whose published v4 relation scan reads twice.
Normalized self/parent/File/RelationAbsent digests agree across A/B/G; exact
B/G v5 digests also agree.

The 134,217,728-Line Check harness source/runner/raw hashes are
`3a12834f117e4c2e6d6ca9c013d21d9cb25e5dab0cd5ba1daac7575c78fa52ab`/
`843c47a3af3c90f940d2befc881c37fa0d45d009466755376340935635c24885`/
`4db40a63bdce1dfa26319b3cc6195d41cf7ba015e78d3cbc285c7f6063735971`.
Its A/B/G binaries hash to
`758142ac27b9908676e4f9017393df3601d491104124819f4915a7161540a980`,
`d2945786a334087510df3c64382b949eda85b72129e6b03bc1e31738c62c4c54`,
and `2d1c8410b05216034876f82f37d230a2637c8efbf2fd4c602ea1daaf5badeac5`.
Untrusted A/B/G median/p95 is 149.339/150.333, 478.224/482.606, and
160.286/161.012 ms; G/A is 1.0733/1.0710. G Host Check is 0.883/1.336 us.

The first 256 MiB Apply set had one G receipt p95 scheduling outlier:
G/A was 1.0432/1.1572 while unit was 1.0489/1.1138 and live Anchor
1.0481/1.0626. Thresholds were not changed. A complete independent confirmation
using the same seven orders hashes its runner/raw evidence to
`0d4ae39dc04a02184a09e4fb8fe679dd91bc3ce0be938fc24576751d77464a03`/
`6d14809c9672a8c18f5bec7c1db72e9e11e30822215d223f9eb8d6004bfc60c8`
and gives:

| Confirmed 256 MiB Apply | A median/p95 ms | B median/p95 ms | G median/p95 ms | G/A median/p95 |
| --- | ---: | ---: | ---: | ---: |
| unit raw-after | 219.980/221.287 | 307.226/318.217 | 230.002/230.688 | 1.0456/1.0425 |
| Replace receipt | 222.735/228.667 | 307.221/316.836 | 233.440/238.181 | 1.0481/1.0416 |
| live Anchor | 221.189/225.494 | 307.594/311.521 | 232.371/233.968 | 1.0506/1.0376 |

Every Apply sample has exact 268,435,573 `rchar`, 268,435,461 `wchar`, final
bytes, and publication outcome. Confirmed G receipt elapsed-ns samples are
`229681239 233440262 235537476 231645606 238181258 230363459 237909776`.
Unit Apply has no before cursor; receipt and live non-File Anchor each consume
at most one prospective after cursor.

The one-shot Edit runner/raw/sample hashes are
`5991b5bc8c291e6ff55104910e5eef2387f17fb89b1d204c4638a37ecce16bff`/
`6efcfd8f33e3b9b28d288450e8cf1cab748a2d79b865d375e47689ba6905131d`/
`8878019fe5c9cb91fb9abae8b0e1b56fa4b536056193e1b1f7e8d3e9e624d0c9`.
The measured child argv is `<bw> --workspace <fresh-root> edit anddress
<exact-search-object> 'retry_budget = 5'`; None/LF/CR/CRLF sources all preserve
their terminator and return one `Changed` receipt. CRLF A/B/G median/p95 is
2.229/2.293, 2.295/2.335, and 2.243/2.367 ms; G/A is 1.0062/1.0324 and final
SHA-256 is
`cc326fa86d3e5924c488283058e530b9413d6acec0f4f78a954882f85f92edbf`.
Production reachability and its regression retain zero private Search/View/
Check calls and exactly one `apply_replace`.

The encoder source/runner/raw hashes are
`067b24634d56cf3a94d094984c6767965ff6a6a69e453065604718565f75113a`/
`dcce93cf953417d37286bd0ff81d0d492d422920e20fe41f541789b62d4cfa61`/
`575a60419b43d5357bdddf53deb49b8ee48e102d0fa4c4e8f397c33033c0469a`.
Its A/B/G binaries hash to
`c11f6a28e3e4c648cfd6c4b7b3e35502303c8054cfe62449afdb08c91ccd612e`,
`9b3f1bed2f1df558757711af45e64dfcf5d94ea8e78b105889c831b0d394fd49`,
and `f4e355270c9263d0e6ebc217298c91bea41bdc637571778e36a3478c67c244ec`.
The reusable G loop starts with 2,048 bytes of caller capacity and records zero
allocations in all three cells:

| Encoder | A median/p95 ms; allocations | B median/p95 ms; allocations | G median/p95 ms; allocations | B/G bytes and SHA-256 |
| --- | ---: | ---: | ---: | --- |
| one Line x 1,000,000 | 579.522/606.457; 20,000,000 | 953.241/958.508; 42,000,000 | 319.200/330.074; 0 | 514,000,000; `a15db4a49ba8d35c3f07affbee7f58a0d9f6c28e0f3c96335a471c8c20b46586` |
| 1,048,576 Lines | 620.052/621.047; 20,971,520 | 1,044.970/1,060.855; 45,088,626 | 357.503/359.477; 0 | 556,413,862; `06fae2ce697682eb4ed7681166bb3f6f56290acead7649766b380f9889978460` |
| 1,000,000 Files | 569.543/599.043; 20,000,000 | 491.218/501.230; 17,000,000 | 168.184/169.546; 0 | 299,000,000; `761c0eab0c8471113b1884fb7c7262f0efd5dab31c90e0760e44f3ec6ff87087` |

The 200,000-file View source/runner/raw hashes are
`82d2410509766a4b73d6d590fd0c10113a1fdbcda372f6f085742587ca940647`/
`016cad5daabde26a4b41e46bbf7f45d10d1235f606ebd7c5a1e557c1c6f826d1`/
`95e3f5626c0fc1d234a0c483f6e3add484598604c2ffbb670e381c5b11b8eec9`.
Its A/B/G binaries hash to
`9fded1cbaae30d9c8a9b4bb2b8c7d7c1baca8f3f265dc79a7798b9611f70204e`,
`f641e93b5d740616e95a768eca23a29bb42b318e919f28c3f6e78120eceaf621`,
and `427cc51172327dc40cec1ad26c2da3a5301ed2760a1ca09f49761911184f1cbf`.
A/B/G batch medians are 678.278/684.928/670.192 ms; sequential medians are
653.171/649.999/629.213 ms. Every variant and mode produces exact digest
`cedf3ed78b9062b8d857ce1025542174c9b1294901e23fa72568d0643afa7e25`,
preserves order and contents, and retains one accepted observation per source
in batch. Every measured View has exactly 200,100 `rchar` bytes—one byte per
source plus 100 harness-accounting bytes—and zero `wchar`. Batch/sequential G
peak HWM is 159,116/114,860 KiB.

The fixed AI harness source/binary hashes are
`6a79f7ef85f6ca1158e628d44993aeeea678a6d713c4703d4d02029c95eebe1c`/
`5dccccaf7a8bd8cbf0f186726c126ffcbe877e75751c20c95423a4139159d78a`.
It performs Search 1 -> batch Line-to-Paragraph View 1 -> receipt Edit 2 ->
final View 1, with post-Edit Search, mandatory Check, history, relocation, and
retry all zero. Final `alpha\nbeta\n` bytes hash to
`e49c81e2d2f84e259d40e2fb8192f3bcd198b355184845d76d8f58807d0d78ee`.

GNU and musl each pass all 268 tests, all-target check, clippy with warnings
denied, and release build; offline/locked metadata/tree and rustfmt also pass.
The suite covers exact v5 KAT/no-v4, Search tiers/order/duplicates, single and
batch View all-or-none, false-Line-count `NotCurrent`, receipts and writer
failure, raw Session, Host proof hit/miss/mismatch/invalidation, Anchor
same-after reflection, stale/foreign/missing/unadmitted/UTF-8/NUL/symlink/
publication uncertainty, and opaque-v5 mutation diagnostics. Drift remains
Correct 1 / Safe Reject 6 / Wrong Apply 0. Production remains exactly 304,431
bytes/9,213 lines, +2.41%/+2.89% over B, with one parser, validator, canonical
writer, cursor, and Issuer. The 256 MiB sparse p95 misses the 1.10 target at
1.1163 but passes the fixed 1.15 hard ceiling; every other hard gate passes.
Gate 7 is therefore GO and advances only Cargo, the root lock entry, README,
version KAT, and active status to source-ready, unpublished `0.2.5`. After that
alignment, GNU and musl release binaries both print exact `Backwriter 0.2.5`
plus LF and pass matching Help, JSON Search-to-Paragraph-View-to-receipt-Edit-
to-Check/fresh-View, and raw Session Apply smokes. A source-built `0.2.5`
Update still has no version comparison and may install official `0.2.4`.

### Gate 8 — separately authorized release

Artifact reconstruction, installer allowlist, publisher, live publication,
endpoint/install/update verification, and release closure require a new exact
Owner authorization. Gate 7 source readiness does not authorize Gate 8.

## Fixed acceptance gates

- Sparse native Search uses the fixed 256 MiB and 1 GiB fixtures. G/A target is
  at most 1.10 and the allowed ceiling is 1.15. A result above 1.15 may activate
  only a measured optimization inside the sole cursor; it does not authorize a
  second parser.
- Dense Search uses exactly 1,048,576 hits. B peak RSS is 166,488 KiB; G target
  is at most 130 MiB, soft gate at most 140 MiB, and hard NO-GO above 145 MiB.
  Result count, order, multiplicity, and output digest must be exact.
- CRLF one-shot Edit G/A target is at most 1.20 and hard ceiling 1.25. It must
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
