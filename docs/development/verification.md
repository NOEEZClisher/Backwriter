# Verification

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
same-parent staging cleanup, deterministic temporary-name collision
preservation, logical-path independence for hard links, unavailable and
no-follow sources, and large whitespace Lines without unnecessary Paragraph
state.
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

Before handoff, verify the diff and index, confirm repository-root `.artext` is
absent and untracked, preserve historical task/history files, and leave the
index empty.

CLI process regressions cover the canonical `backwriter` binary without a `bw`
binary, `--help`, default-current-directory and explicit absolute workspaces,
default and repeated admission, Line/Paragraph/File Search, repeated source and
subtree scope selectors, Core scope rejection, deterministic human output,
space-preserving query argv, raw-Anddress/workspace-coordinate omission, Empty,
usage versus Runtime execution exits, unsupported deferred forms, and strict
stdout/stderr separation. View regressions cover v3 decode, File/Paragraph/Line
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
Session Edit/Apply regressions cover all five Edit variants, all four Position
forms, exact source bytes, quoted content escapes, binding cloning, invalid
forms, and continued execution after errors without CLI recovery.
