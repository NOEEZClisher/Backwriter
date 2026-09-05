use super::*;

pub(super) const TOP_LEVEL_HELP_KAT: &str = r#"USAGE
  bw [GLOBAL OPTIONS] <command> [command options and operands]
  bw help [<command>]

GLOBAL OPTIONS
  --workspace ABSOLUTE_PATH  Select an absolute workspace before the command.
  --admit LOGICAL_PATH       Admit a logical root before the command; repeatable.
  --json                     Select JSON output where the command supports it.
  --raw                      Select raw View output only.

COMMANDS
  shell    Reuse short references across search, view, replace, and check.
  search   Discover current File, Paragraph, or Line Anddresses.
  view     Read one or more current Anddresses.
  edit     Replace one current Anddress.
  check    Check one or more current Anddresses.
  version  Print the Backwriter version.
  update   Run the installed-platform updater.

ADDITIONAL HELP
  bw help <command>
  Advanced topics: pick, anchor, apply, data (raw Session only; no one-shot execution).

Global options precede the command. Use bw help shell for ordinary short-ref work.
"#;

pub(super) const SEARCH_HELP_KAT: &str = r#"NAME
  bw search - discover current Anddresses by exact literal Line content or logical File path

USAGE
  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] search <line|paragraph|file> <query> [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...
  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] search /file <logical-path>

DESCRIPTION
  Searches admitted Workspace Source. Literal queries are case-sensitive and match exact Line content without normalization.

ARGUMENTS
  <line|paragraph|file>  Returned target kind.
  <query>                Nonempty literal query.
  /file <logical-path>   Exact logical File lookup.

OPTIONS
  --workspace, --admit, and --json must precede search.
  --source LOGICAL_PATH and --subtree LOGICAL_PATH narrow a literal search scope.

WHAT HAPPENS
  Opens the Runtime, scans admitted source once per selected source, and returns all-or-nothing current results.

OUTPUT
  Human output lists matches. --json writes the fixed bw.cli.search.v2 envelope.

EXAMPLES
  bw search line needle --source note.txt
  bw --json search paragraph needle
  bw search /file note.txt

FAILURES
  Invalid request or scope is a usage failure. Unavailable source or Runtime failure exits 1.

SEE ALSO
  bw help view
  bw help shell
"#;

pub(super) const VIEW_HELP_KAT: &str = r#"NAME
  bw view - project current content from one or more v5 Anddresses

USAGE
  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json|--raw] view anddress <encoded-v5-Anddress> [--as <line|paragraph|file>]
  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json view anddress <encoded-v5-Anddress>... --as <line|paragraph|file>

DESCRIPTION
  Validates current source state and projects the requested target relation from caller-provided v5 Anddresses.

ARGUMENTS
  anddress                  Required input form.
  <encoded-v5-Anddress>     One or more canonical v5 objects.

OPTIONS
  --workspace, --admit, --json, and --raw must precede view.
  --as selects line, paragraph, or file and must be last. Batch View requires --json and --as.

WHAT HAPPENS
  Opens the Runtime after input validation and returns the requested current projection.

OUTPUT
  One human or raw View writes content. JSON writes the fixed bw.cli.view.v2 envelope.

EXAMPLES
  bw view anddress '<v5-Anddress>'
  bw --raw view anddress '<v5-Line-Anddress>'
  bw --json view anddress '<v5-Anddress>' --as paragraph

FAILURES
  Invalid input or unsupported output form is a usage failure. Unavailable or stale source exits 1.

SEE ALSO
  bw help search
  bw help check
"#;

pub(super) const EDIT_HELP_KAT: &str = r#"NAME
  bw edit - replace one current v5 Anddress

USAGE
  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] edit anddress <encoded-v5-Anddress> <content>
  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] edit anddress <encoded-v5-Anddress> --stdin

DESCRIPTION
  Replaces exactly one current File, Paragraph, or Line target through the Runtime Replace seam.

ARGUMENTS
  anddress                  Required input form.
  <encoded-v5-Anddress>     One canonical v5 object.
  <content>                  One positional replacement string.
  --stdin                    Read replacement Content from standard input through EOF.

OPTIONS
  --workspace, --admit, and --json must precede edit.
  --stdin is the exclusive Content selector; use standard input to pass literal --stdin Content.

WHAT HAPPENS
  Validates the Anddress, reads selected standard input before Runtime access, preserves an existing Line terminator automatically, then applies one Replace.

OUTPUT
  Human output writes the receipt outcome and fresh Anddress when present. --json writes bw.cli.edit.v1.

EXAMPLES
  bw edit anddress '<v5-Anddress>' 'replacement'
  printf '%s' 'replacement' | bw edit anddress '<v5-Anddress>' --stdin

FAILURES
  Invalid input is a usage failure. Standard-input, stale, unavailable, or publication failure exits 1.

SEE ALSO
  bw help view
  bw help check
"#;

pub(super) const CHECK_HELP_KAT: &str = r#"NAME
  bw check - check one or more current v5 Anddresses

USAGE
  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... check anddress <encoded-v5-Anddress>
  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... --json check anddress <encoded-v5-Anddress>...

DESCRIPTION
  Checks the current state of caller-provided v5 Anddresses in input order.

ARGUMENTS
  anddress                  Required input form.
  <encoded-v5-Anddress>     One or more canonical v5 objects.

OPTIONS
  --workspace, --admit, and --json must precede check.
  Multiple inputs require --json. No command-local options are available.

WHAT HAPPENS
  Validates every input before opening the Runtime, then reports one currentness state per input.

OUTPUT
  One human input writes one state. --json writes the fixed bw.cli.check.v2 envelope.

EXAMPLES
  bw check anddress '<v5-Anddress>'
  bw --json check anddress '<v5-Anddress>' '<v5-Anddress>'

FAILURES
  Invalid input or a non-JSON batch is a usage failure. Runtime failure exits 1.

SEE ALSO
  bw help search
  bw help shell
"#;

pub(super) const SHELL_HELP_KAT: &str = r#"NAME
  bw shell - reuse short references across search, view, replace, and check

USAGE
  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell

COMMANDS
  search <line|paragraph|file> <query> [--source PATH | --subtree PATH]...
  search /file <logical-path>
  view <REF>... [--as <line|paragraph|file>]
  replace <REF> <CONTENT>
  check <REF>...
  let <NAME> = <REF>
  exit

REFERENCES AND CONTENT
  @N is a zero-based, append-only reference from this process. It ends at EOF or exit.
  @name is a named Anddress; @hits[0] selects an indexed Search/Pick binding.
  let name = @N makes a named alias. Names cannot be redefined.
  Double-quote arguments containing spaces. Quoted escapes are \\, \", \n, \r, and \t.
  One physical input line is one command; no single quotes, pipes, or EOF Content selector.
  Line Replace accepts body only: NUL/CR/LF are rejected; the existing terminator is preserved.
  File/Paragraph Replace uses exact Content. Changed source bytes stale old same-source refs,
  including other locations. Use the fresh receipt or explicitly search again; never auto-retry.

OUTPUT
  Search appends one @N per result; Empty writes nothing.
  View returns each input ref, a fresh ref, kind/location, and complete Content in input order.
  View<TAB>REF<TAB>bytes=N starts a record, followed by fresh-ref metadata and N exact bytes.
  The following LF and EndView line are display framing, not source Content.
  RelationAbsent is shown at its input position without Content or a fresh slot.
  Replace writes Unchanged/Changed with a fresh ref, or Changed<TAB>None without one.
  Check writes one status per input; only Current appends a fresh ref. Check is optional.

EXAMPLES
  Start bw shell in a workspace with only note.txt containing needle plus CRLF, then enter:
  search line needle --source note.txt
  view @0 --as paragraph
  let selected = @0
  replace @selected "new value"
  check @0 @2
  view @3
  exit

ADVANCED
  let hits = search line needle
  view anddress @hits[0]
  Raw named View writes exact Content without direct-View framing.
  See bw help pick, bw help anchor, bw help apply, and bw help data.

FAILURES
  Usage errors record exit 2; Runtime/source errors record exit 1; later commands still run.
  Stdin/stdout failure ends the shell with exit 1. Partial output cannot prove delivery or
  undo publication. --json/--raw and command-line operands are unavailable.

SEE ALSO
  bw help search
  bw help edit
"#;

pub(super) const UPDATE_HELP_KAT: &str = r#"NAME
  bw update - run the installed-platform updater

USAGE
  bw update

DESCRIPTION
  Downloads and hands off to the canonical installer for the current platform.

ARGUMENTS
  None.

OPTIONS
  None.

WHAT HAPPENS
  Performs the existing update download and installer handoff.

OUTPUT
  The installer owns its output.

EXAMPLES
  bw update

FAILURES
  Any option or operand is a usage failure. Download, installer, or platform failure exits 1.

SEE ALSO
  bw help version
"#;

pub(super) const VERSION_HELP_KAT: &str = r#"NAME
  bw version - print the Backwriter version

USAGE
  bw version

OUTPUT
  One compiled Backwriter version line, followed by LF. No Runtime or Workspace Source access.

EXAMPLES
  bw version

FAILURES
  Any option or operand is a usage failure. Standard-output failure exits 1.

SEE ALSO
  bw help update
"#;

pub(super) const PICK_HELP_KAT: &str = r#"NAME
  Pick - advanced raw Session selection; no one-shot command

USAGE
  pick @<search-or-pick-binding> <predicate>
  let <name> = pick @<search-or-pick-binding> <predicate>

OPERANDS
  Candidates require one unindexed named Search/Pick binding, not a numeric ref.
  Predicates: all; target-kind <file|paragraph|line>; one-of <anddress-ref>...;
  same-file <anddress-ref>; not (<predicate>); all-of (<predicate>)...; any-of (<predicate>)...
  Address operands are @name or @hits[0]. At least one member/group is required.
  Pick preserves input order and duplicates; it does not read source or prove currentness.

OUTPUT
  Selected N followed by indexed kind/path/byte-range rows. Empty writes Selected 0.
  let retains the result after output; direct Pick retains no binding or numeric refs.

EXAMPLES
  Start bw shell with note.txt containing needle plus CRLF, then enter:
  let hits = search line needle --source note.txt
  let chosen = pick @hits target-kind line
  view anddress @chosen[0]
  exit

FAILURES
  Invalid predicates, binding kinds, indices, or duplicate names are usage errors.
  Resource/stdout failures are execution errors. No implicit selection or binding update.

SEE ALSO
  bw help shell
  bw help data
"#;

pub(super) const ANCHOR_HELP_KAT: &str = r#"NAME
  Anchor - advanced raw Session live continuity; no one-shot command

USAGE
  let <name> = anchor create <anddress-ref>
  view anchored @<handle>
  anchor invalidate-source <logical-path>

OPERANDS
  create is only a let right-hand side. Use @name or @hits[0], not a numeric ref.
  A handle belongs to this Runtime and cannot be cloned, indexed, or used as an Anddress.
  invalidate-source takes exactly one logical path; it ends that source's live continuity.

OUTPUT
  Anchored creates the handle binding. AlreadyLive creates no alias or binding.
  Anchored View writes exact Content; successful invalidation writes OK plus LF.

EXAMPLES
  Start bw shell with note.txt containing needle plus CRLF, then enter:
  let hits = search line needle --source note.txt
  let live = anchor create @hits[0]
  view anchored @live
  anchor invalidate-source note.txt
  exit

FAILURES
  Invalid forms, addresses, and duplicate names are usage errors.
  Unavailable source/continuity and View failures are execution errors.
  Handles end with this shell; there is no persistence, adoption, or re-identification.

SEE ALSO
  bw help shell
  bw help apply
"#;

pub(super) const APPLY_HELP_KAT: &str = r#"NAME
  Apply - advanced raw Session exact-extent editing; no one-shot command

USAGE
  let <name> = edit insert <position> <anddress-ref> <content>
  let <name> = edit replace <anddress-ref> <content>
  let <name> = edit delete <anddress-ref>
  let <name> = edit <move|copy> <anddress-ref> <position> <anddress-ref>
  apply @<edit-binding>

OPERANDS
  Positions are before/after (Line/Paragraph) or start-of/end-of (File).
  Addresses are @name or @hits[0], not numeric refs. Apply takes exactly one unindexed Edit.
  Raw Content is the exact extent, including desired terminators; unlike direct replace,
  raw Replace does not preserve a Line terminator for you. NUL is invalid.
  let copy = @edit explicitly clones an Edit; Apply borrows it without consuming the binding.
  An Edit cannot be indexed or stored in Data. Existing currentness still governs reuse.

OUTPUT
  Edit binding construction writes nothing and does not publish. Successful Apply writes OK
  plus LF, not a receipt or fresh ref. Obtain later ordinary addresses by explicit Search.

EXAMPLES
  Start bw shell with note.txt containing needle plus CRLF, then enter:
  let hits = search line needle --source note.txt
  let change = edit replace @hits[0] "replacement\r\n"
  apply @change
  exit

FAILURES
  Invalid Edit, position, binding, index, or NUL is a usage error.
  Apply failures are execution errors. Exit 1 or partial output is not evidence of unchanged
  source; no automatic retry, rollback, transaction, or recovery is provided.

SEE ALSO
  bw help shell
  bw help edit
"#;

pub(super) const DATA_HELP_KAT: &str = r#"NAME
  Data - advanced raw Session typed value storage; no one-shot command

USAGE
  data store <kind> <name> <value-ref>
  data get <kind> <name>
  let <binding> = data get <kind> <name>
  data rename <kind> <old-name> <new-name>
  data remove <kind> <name>
  data list

OPERANDS
  Kinds: anddress, search, pick, view, check-anddress, check-search, check-pick.
  anddress Store takes @name or @hits[0]; other kinds take an unindexed matching binding.
  Numeric refs require a named alias first. Edit and Anchor handles cannot be stored.
  Names are nonempty strings; quote spaces. Equal names in different kinds are independent.
  The one DataStore lasts only until this shell ends. Store is explicit, never automatic.

OUTPUT
  Store/Rename/Remove write OK plus LF. Get uses the value's existing human output.
  let Get writes once before retaining its binding. List writes kind and escaped quoted name
  per entry in Core order; an empty List writes nothing. Failures preserve stored entries.

EXAMPLES
  Start bw shell with note.txt containing needle plus CRLF, then enter:
  let hits = search line needle --source note.txt
  data store search saved @hits
  let restored = data get search saved
  view anddress @restored[0]
  data rename search saved renamed
  data list
  data remove search renamed
  exit

FAILURES
  Missing/duplicate names, unknown kinds, wrong binding types, or malformed refs are usage
  errors. Resource/stdout failures are execution errors. No disk storage or session recovery.

SEE ALSO
  bw help shell
  bw help pick
"#;

#[test]
fn canonical_binary_help_and_default_workspace_search() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let output = run(root.path(), &["search", "line", "needle"]);
    assert!(output.status.success());
    assert_eq!(text(output.stdout), "Found 1\n0\tLine\tnote.txt:1\n");
    assert!(output.stderr.is_empty());
    assert_eq!(binary().file_name().unwrap(), "bw");
    assert!(
        !Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/bin/backwriter.rs")
            .exists()
    );

    let help = run(root.path(), &["--help"]);
    assert!(help.status.success());
    assert_eq!(help.stdout, TOP_LEVEL_HELP_KAT.as_bytes());
    assert!(help.stderr.is_empty());

    let named_help = run(root.path(), &["help"]);
    assert!(named_help.status.success());
    assert_eq!(named_help.stdout, TOP_LEVEL_HELP_KAT.as_bytes());
    assert!(named_help.stderr.is_empty());

    let version = run(root.path(), &["version"]);
    assert!(version.status.success());
    assert_eq!(version.stdout, b"Backwriter 0.3.0\n");
    assert!(version.stderr.is_empty());
}

#[test]
fn command_local_help_kats_are_exact_and_skip_runtime_opening() {
    let root = tempfile::tempdir().unwrap();
    let cases = [
        ("search", SEARCH_HELP_KAT),
        ("view", VIEW_HELP_KAT),
        ("edit", EDIT_HELP_KAT),
        ("check", CHECK_HELP_KAT),
        ("shell", SHELL_HELP_KAT),
        ("update", UPDATE_HELP_KAT),
        ("version", VERSION_HELP_KAT),
    ];

    for (command, expected) in cases {
        let direct = run(root.path(), &[command, "--help"]);
        assert!(direct.status.success());
        assert_eq!(direct.stdout, expected.as_bytes());
        assert!(direct.stderr.is_empty());

        let named = run(root.path(), &["help", command]);
        assert!(named.status.success());
        assert_eq!(named.stdout, expected.as_bytes());
        assert!(named.stderr.is_empty());

        assert_usage(run(root.path(), &[command, "--help", "trailing"]));
    }

    for (command, expected) in cases[..5].iter().copied() {
        let unavailable = root.path().join("not-a-workspace");
        let output = Command::new(binary())
            .current_dir(root.path())
            .arg("--workspace")
            .arg(unavailable)
            .args([command, "--help"])
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout, expected.as_bytes());
        assert!(output.stderr.is_empty());
    }

    for (topic, expected) in [
        ("pick", PICK_HELP_KAT),
        ("anchor", ANCHOR_HELP_KAT),
        ("apply", APPLY_HELP_KAT),
        ("data", DATA_HELP_KAT),
    ] {
        let help = run(root.path(), &["help", topic]);
        assert_eq!(help.status.code(), Some(0));
        assert_eq!(help.stdout, expected.as_bytes());
        assert!(help.stderr.is_empty());
        assert_usage(run(root.path(), &[topic]));
        assert_usage(run(root.path(), &[topic, "--help"]));
    }

    assert_usage(run(root.path(), &["help", "search", "trailing"]));
    assert_usage(run(root.path(), &["help", "unknown"]));
    assert_usage(run(root.path(), &["pick"]));
    assert_usage(run(root.path(), &["anchor"]));
    assert_usage(run(root.path(), &["apply"]));
    assert_usage(run(root.path(), &["data"]));
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
    for (new, old) in [(false, false), (true, false), (false, true), (true, true)] {
        let fixture = tempfile::tempdir().unwrap();
        for (present, path) in [(new, ".bw"), (old, ".artext/bw")] {
            if present {
                fs::create_dir_all(fixture.path().join(path)).unwrap();
                fs::write(fixture.path().join(path).join("sentinel"), b"\xff\0").unwrap();
            }
        }
        for (command, expected) in [
            ("--help", TOP_LEVEL_HELP_KAT),
            ("version", "Backwriter 0.3.0\n"),
        ] {
            let output = run(fixture.path(), &[command]);
            assert!(output.status.success());
            assert_eq!(output.stdout, expected.as_bytes());
            assert!(output.stderr.is_empty());
        }
        for (present, path) in [(new, ".bw"), (old, ".artext/bw")] {
            assert_eq!(fixture.path().join(path).exists(), present);
            if present {
                assert_eq!(
                    fs::read(fixture.path().join(path).join("sentinel")).unwrap(),
                    b"\xff\0"
                );
                assert_eq!(fs::read_dir(fixture.path().join(path)).unwrap().count(), 1);
            }
        }
        assert_eq!(
            fs::read_dir(fixture.path()).unwrap().count(),
            usize::from(new) + usize::from(old)
        );
    }
}

#[test]
fn one_shot_usage_errors_have_exact_command_local_codes_usage_and_hints() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    let operand = view_operand(root.path(), "coordinate.txt", AnddressTarget::File);

    assert_actionable_usage(
        run(root.path(), &[]),
        "command.missing",
        "missing capability",
        TOP_LEVEL_HELP_KAT,
        "bw --help",
    );
    assert_actionable_usage(
        run(
            root.path(),
            &["--json", "--json", "search", "line", "coordinate"],
        ),
        "global.output_duplicate",
        "only one output option may appear",
        TOP_LEVEL_HELP_KAT,
        "bw --help",
    );
    assert_actionable_usage(
        run(root.path(), &["search", "wrong", "coordinate"]),
        "search.kind_invalid",
        "invalid search kind: wrong",
        SEARCH_HELP_KAT,
        "bw help search",
    );
    assert_actionable_usage(
        run(root.path(), &["view", "anddress", &operand, "--as"]),
        "view.target_invalid",
        "view --as requires exactly one target and must be last",
        VIEW_HELP_KAT,
        "bw help view",
    );
    assert_actionable_usage(
        run(root.path(), &["edit", "wrong", &operand, "content"]),
        "edit.form_invalid",
        "edit requires the anddress input form",
        EDIT_HELP_KAT,
        "bw help edit",
    );
    assert_actionable_usage(
        run(root.path(), &["check", "anddress", &operand, &operand]),
        "check.output_unsupported",
        "checking multiple Anddresses requires --json",
        CHECK_HELP_KAT,
        "bw help check",
    );
    assert_actionable_usage(
        run(root.path(), &["pick"]),
        "capability.one_shot_unavailable",
        "pick has no one-shot command; use bw shell",
        TOP_LEVEL_HELP_KAT,
        "bw --help",
    );
}

#[test]
fn command_help_examples_match_current_one_shot_and_raw_session_behavior() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\r\n");
    write(root.path(), "coordinate.txt", "coordinate\n");

    let search = run(
        root.path(),
        &["search", "line", "needle", "--source", "note.txt"],
    );
    assert!(search.status.success());
    assert_eq!(search.stdout, b"Found 1\n0\tLine\tnote.txt:1\n");
    assert!(search.stderr.is_empty());

    let operand = view_operand(
        root.path(),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "needle\r\n".to_owned(),
        },
    );
    let view = run(root.path(), &["view", "anddress", &operand]);
    assert!(view.status.success());
    assert_eq!(view.stdout, b"needle\r\n");
    assert!(view.stderr.is_empty());

    let check = run(root.path(), &["check", "anddress", &operand]);
    assert_check_status(check, "Current");

    let edit = run(root.path(), &["edit", "anddress", &operand, "replacement"]);
    assert!(edit.status.success());
    assert!(edit.stderr.is_empty());
    assert_eq!(
        fs::read(root.path().join("note.txt")).unwrap(),
        b"replacement\r\n"
    );

    let shell = run_shell(
        root.path(),
        "let hits = search line replacement\nview anddress @hits[0]\nexit\n",
    );
    assert!(shell.status.success());
    assert_eq!(
        shell.stdout,
        b"Found 1\n0\tLine\tnote.txt:1\nreplacement\r\n"
    );
    assert!(shell.stderr.is_empty());

    for (help, commands, expected, final_bytes) in [
        (
            SHELL_HELP_KAT,
            "search line needle --source note.txt\nview @0 --as paragraph\nlet selected = @0\nreplace @selected \"new value\"\ncheck @0 @2\nview @3\nexit\n",
            "@0\tLine\tnote.txt:1\nView\t@0\tbytes=8\n@1\tParagraph\tnote.txt:1-1\nneedle\r\n\nEndView\n@2\tChanged\tLine\tnote.txt:1\nNotCurrent\n@3\tCurrent\tLine\tnote.txt:1\nView\t@3\tbytes=11\n@4\tLine\tnote.txt:1\nnew value\r\n\nEndView\n",
            "new value\r\n",
        ),
        (
            PICK_HELP_KAT,
            "let hits = search line needle --source note.txt\nlet chosen = pick @hits target-kind line\nview anddress @chosen[0]\nexit\n",
            "Found 1\n0\tLine\tnote.txt:1\nSelected 1\n0\tLine\tnote.txt:0-8\nneedle\r\n",
            "needle\r\n",
        ),
        (
            ANCHOR_HELP_KAT,
            "let hits = search line needle --source note.txt\nlet live = anchor create @hits[0]\nview anchored @live\nanchor invalidate-source note.txt\nexit\n",
            "Found 1\n0\tLine\tnote.txt:1\nAnchored\nneedle\r\nOK\n",
            "needle\r\n",
        ),
        (
            APPLY_HELP_KAT,
            "let hits = search line needle --source note.txt\nlet change = edit replace @hits[0] \"replacement\\r\\n\"\napply @change\nexit\n",
            "Found 1\n0\tLine\tnote.txt:1\nOK\n",
            "replacement\r\n",
        ),
        (
            DATA_HELP_KAT,
            "let hits = search line needle --source note.txt\ndata store search saved @hits\nlet restored = data get search saved\nview anddress @restored[0]\ndata rename search saved renamed\ndata list\ndata remove search renamed\nexit\n",
            "Found 1\n0\tLine\tnote.txt:1\nOK\nFound 1\n0\tLine\tnote.txt:1\nneedle\r\nOK\nsearch\t\"renamed\"\nOK\n",
            "needle\r\n",
        ),
    ] {
        let fixture = tempfile::tempdir().unwrap();
        write(fixture.path(), "note.txt", "needle\r\n");
        let documented = help.split_once("then enter:\n").unwrap().1;
        let documented = documented.split_once("\n\n").unwrap().0;
        assert_eq!(
            documented
                .lines()
                .map(|line| line.strip_prefix("  ").unwrap())
                .collect::<Vec<_>>(),
            commands.lines().collect::<Vec<_>>()
        );
        let output = run_shell(fixture.path(), commands);
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(output.stdout, expected.as_bytes());
        assert!(output.stderr.is_empty());
        assert_eq!(
            fs::read(fixture.path().join("note.txt")).unwrap(),
            final_bytes.as_bytes()
        );
        assert_eq!(fs::read_dir(fixture.path()).unwrap().count(), 1);
    }
}

#[cfg(unix)]
#[test]
fn update_help_does_not_start_the_update_download() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join("bin")).unwrap();
    let called = root.path().join("curl-called");
    write_executable(
        root.path(),
        "bin/curl",
        "#!/bin/sh\n: > \"$BW_HELP_CURL_CALLED\"\nexit 99\n",
    );

    let output = Command::new(binary())
        .current_dir(root.path())
        .args(["update", "--help"])
        .env("PATH", root.path().join("bin"))
        .env("BW_HELP_CURL_CALLED", &called)
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, UPDATE_HELP_KAT.as_bytes());
    assert!(output.stderr.is_empty());
    assert!(!called.exists());
}
