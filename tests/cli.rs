use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};

use artext::{
    backwriter::{
        anddress::{ANDDRESS_VERSION, Anddress, AnddressTarget, Natural},
        search::{SearchOutcome, SearchQuery, SearchRequest, SearchScope, SearchTarget},
    },
    runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime},
};

fn binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_backwriter"))
}

fn run(root: &Path, arguments: &[&str]) -> Output {
    Command::new(binary())
        .current_dir(root)
        .args(arguments)
        .output()
        .unwrap()
}

fn run_shell(root: &Path, input: &str) -> Output {
    let mut child = Command::new(binary())
        .current_dir(root)
        .arg("shell")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

fn run_shell_after_initial_output(
    root: &Path,
    initial: &str,
    initial_line_count: usize,
    mutate: impl FnOnce(),
    remaining: &str,
) -> Output {
    let mut child = Command::new(binary())
        .current_dir(root)
        .arg("shell")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(initial.as_bytes()).unwrap();
    stdin.flush().unwrap();

    let mut stdout_reader = BufReader::new(child.stdout.take().unwrap());
    let mut stdout = Vec::new();
    for _ in 0..initial_line_count {
        stdout_reader.read_until(b'\n', &mut stdout).unwrap();
    }
    mutate();
    stdin.write_all(remaining.as_bytes()).unwrap();
    drop(stdin);
    stdout_reader.read_to_end(&mut stdout).unwrap();
    let status = child.wait().unwrap();
    let mut stderr = Vec::new();
    child
        .stderr
        .take()
        .unwrap()
        .read_to_end(&mut stderr)
        .unwrap();
    Output {
        status,
        stdout,
        stderr,
    }
}

fn text(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).unwrap()
}

fn write(root: &Path, path: &str, content: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn view_operand(root: &Path, path: &str, target: AnddressTarget) -> String {
    let workspace = WorkspaceRuntime::open(
        root,
        WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
    )
    .unwrap();
    let request = SearchRequest::new(
        SearchQuery::new("coordinate").unwrap(),
        SearchScope::all_admitted(),
        SearchTarget::File,
    );
    let SearchOutcome::Found { anddresses } = workspace.search(&request).unwrap() else {
        panic!("coordinate source");
    };
    String::from_utf8(
        Anddress {
            version: ANDDRESS_VERSION.to_owned(),
            workspace_coordinate: anddresses[0].workspace_coordinate.clone(),
            logical_path: path.to_owned(),
            target,
        }
        .encode()
        .unwrap(),
    )
    .unwrap()
}

fn assert_usage(output: Output) {
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(text(output.stderr).contains("Usage:"));
}

fn assert_execution_error(output: Output) {
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(text(output.stderr).starts_with("error: "));
}

fn assert_check_status(output: Output, status: &str) {
    assert!(output.status.success());
    assert_eq!(output.stdout, format!("{status}\n").as_bytes());
    assert!(output.stderr.is_empty());
}

#[test]
fn canonical_binary_help_and_default_workspace_search() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let output = run(root.path(), &["search", "line", "needle"]);
    assert!(output.status.success());
    assert_eq!(text(output.stdout), "Found 1\n0\tLine\tnote.txt:0\n");
    assert!(output.stderr.is_empty());
    assert!(!binary().with_file_name("bw").exists());

    let help = run(root.path(), &["--help"]);
    assert!(help.status.success());
    assert!(text(help.stdout).starts_with("Usage:\n  backwriter "));
    assert!(help.stderr.is_empty());
}

#[test]
fn explicit_workspace_and_repeated_admission_use_runtime_validation() {
    let workspace = tempfile::tempdir().unwrap();
    let caller = tempfile::tempdir().unwrap();
    write(workspace.path(), "src/a.txt", "needle\n");
    write(workspace.path(), "tests/b.txt", "needle\n");
    write(workspace.path(), "ignored.txt", "needle\n");

    let output = Command::new(binary())
        .current_dir(caller.path())
        .arg("--workspace")
        .arg(workspace.path())
        .args([
            "--admit", "src", "--admit", "tests", "search", "file", "needle",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        text(output.stdout),
        "Found 2\n0\tFile\tsrc/a.txt\n1\tFile\ttests/b.txt\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn line_paragraph_file_and_scope_output_keep_core_order() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "a.txt", "needle\nxneedle\n\n");
    write(root.path(), "b.txt", "needle\n");
    write(root.path(), "dir/c.txt", "needle\n");
    write(root.path(), "top.txt", "needle\n");

    let line = run(root.path(), &["search", "line", "needle"]);
    assert!(line.status.success());
    assert_eq!(
        text(line.stdout),
        "Found 5\n0\tLine\ta.txt:0\n1\tLine\tb.txt:0\n2\tLine\tdir/c.txt:0\n3\tLine\ttop.txt:0\n4\tLine\ta.txt:1\n"
    );
    let paragraph = run(root.path(), &["search", "paragraph", "needle"]);
    assert!(paragraph.status.success());
    assert_eq!(
        text(paragraph.stdout),
        "Found 4\n0\tParagraph\ta.txt:0\n1\tParagraph\tb.txt:0\n2\tParagraph\tdir/c.txt:0\n3\tParagraph\ttop.txt:0\n"
    );
    let file = run(root.path(), &["search", "file", "needle"]);
    assert!(file.status.success());
    assert_eq!(
        text(file.stdout),
        "Found 4\n0\tFile\ta.txt\n1\tFile\tb.txt\n2\tFile\tdir/c.txt\n3\tFile\ttop.txt\n"
    );

    let scoped = run(
        root.path(),
        &[
            "search",
            "line",
            "needle",
            "--source",
            "top.txt",
            "--subtree",
            "dir",
        ],
    );
    assert!(scoped.status.success());
    assert_eq!(
        text(scoped.stdout),
        "Found 2\n0\tLine\tdir/c.txt:0\n1\tLine\ttop.txt:0\n"
    );
    assert_usage(run(
        root.path(),
        &[
            "search",
            "line",
            "needle",
            "--source",
            "dir/c.txt",
            "--subtree",
            "dir",
        ],
    ));
}

#[test]
fn human_output_hides_raw_anddress_and_preserves_space_query() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "spaced file.txt", "contains a space\n");

    let found = run(root.path(), &["search", "line", "a space"]);
    assert!(found.status.success());
    let found_stdout = text(found.stdout);
    assert_eq!(found_stdout, "Found 1\n0\tLine\tspaced file.txt:0\n");
    assert!(!found_stdout.contains("artext.backwriter-anddress"));
    assert!(!found_stdout.contains(&root.path().display().to_string()));
    assert!(found.stderr.is_empty());

    let empty = run(root.path(), &["search", "line", "absent"]);
    assert!(empty.status.success());
    assert_eq!(text(empty.stdout), "Found 0\n");
    assert!(empty.stderr.is_empty());
}

#[test]
fn syntax_and_unimplemented_forms_are_usage_errors() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    for arguments in [
        vec!["search", "word", "needle"],
        vec!["search", "line", ""],
        vec!["--workspace", "relative", "search", "line", "needle"],
        vec!["--unknown", "search", "line", "needle"],
        vec!["shell", "extra"],
        vec!["pick"],
        vec!["view"],
        vec!["search", "line", "needle", "--json"],
        vec!["search", "line", "needle", "--raw"],
    ] {
        assert_usage(run(root.path(), &arguments));
    }
}

#[test]
fn unavailable_workspace_and_source_are_execution_errors() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");
    let unavailable_workspace = root.path().join("missing-workspace");

    let workspace = Command::new(binary())
        .current_dir(root.path())
        .arg("--workspace")
        .arg(unavailable_workspace)
        .args(["search", "line", "needle"])
        .output()
        .unwrap();
    assert_execution_error(workspace);
    assert_execution_error(run(
        root.path(),
        &["search", "line", "needle", "--source", "missing.txt"],
    ));
}

#[test]
fn view_file_paragraph_and_line_preserve_exact_human_bytes() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    let source = "file\n\nparagraph\nline\n";
    write(root.path(), "note.txt", source);

    let file = view_operand(root.path(), "note.txt", AnddressTarget::File);
    let file_output = run(root.path(), &["view", "anddress", &file]);
    assert!(file_output.status.success());
    assert_eq!(file_output.stdout, source.as_bytes());
    assert!(file_output.stderr.is_empty());

    let paragraph = view_operand(
        root.path(),
        "note.txt",
        AnddressTarget::Paragraph {
            ordinal: Natural::one(),
        },
    );
    let paragraph_output = run(root.path(), &["view", "anddress", &paragraph]);
    assert!(paragraph_output.status.success());
    assert_eq!(paragraph_output.stdout, b"paragraph\nline\n");
    assert!(paragraph_output.stderr.is_empty());

    let line = view_operand(
        root.path(),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::parse("3").unwrap(),
            exact_extent: "line\n".to_owned(),
        },
    );
    let line_output = run(root.path(), &["view", "anddress", &line]);
    assert!(line_output.status.success());
    assert_eq!(line_output.stdout, b"line\n");
    assert!(!text(line_output.stdout).contains("workspaceCoordinate"));
    assert!(line_output.stderr.is_empty());
}

#[test]
fn view_line_terminators_and_large_no_eol_are_exact() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    write(root.path(), "terminators.txt", "lf\ncr\rcrlf\r\nnone");

    for (ordinal, exact_extent) in [
        ("0", "lf\n"),
        ("1", "cr\r"),
        ("2", "crlf\r\n"),
        ("3", "none"),
    ] {
        let operand = view_operand(
            root.path(),
            "terminators.txt",
            AnddressTarget::Line {
                ordinal: Natural::parse(ordinal).unwrap(),
                exact_extent: exact_extent.to_owned(),
            },
        );
        let output = run(root.path(), &["view", "anddress", &operand]);
        assert!(output.status.success());
        assert_eq!(output.stdout, exact_extent.as_bytes());
        assert!(output.stderr.is_empty());
    }

    let large_line = format!("large-{}-tail", "x".repeat(20_000));
    let large_source = format!("coordinate\n\n{large_line}");
    write(root.path(), "large.txt", &large_source);
    let file = view_operand(root.path(), "large.txt", AnddressTarget::File);
    let file_output = run(root.path(), &["view", "anddress", &file]);
    assert!(file_output.status.success());
    assert_eq!(file_output.stdout, large_source.as_bytes());
    let line = view_operand(
        root.path(),
        "large.txt",
        AnddressTarget::Line {
            ordinal: Natural::parse("2").unwrap(),
            exact_extent: large_line.clone(),
        },
    );
    let line_output = run(root.path(), &["view", "anddress", &line]);
    assert!(line_output.status.success());
    assert_eq!(line_output.stdout, large_line.as_bytes());
    assert!(line_output.stderr.is_empty());
}

#[test]
fn view_rejects_invalid_and_unavailable_inputs_at_the_right_exit() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    write(root.path(), "note.txt", "actual\n");

    assert_usage(run(root.path(), &["view", "anddress", "{"]));
    assert_usage(run(
        root.path(),
        &["view", "anddress", r#"{"version":"old","kind":null}"#],
    ));
    let wrong_extent = view_operand(
        root.path(),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "wrong\n".to_owned(),
        },
    );
    assert_execution_error(run(root.path(), &["view", "anddress", &wrong_extent]));
    let stale = view_operand(
        root.path(),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "actual\n".to_owned(),
        },
    );
    write(root.path(), "note.txt", "changed\n");
    assert_execution_error(run(root.path(), &["view", "anddress", &stale]));

    let admitted_root = tempfile::tempdir().unwrap();
    write(
        admitted_root.path(),
        "admitted/coordinate.txt",
        "coordinate\n",
    );
    write(admitted_root.path(), "other.txt", "other\n");
    let unadmitted = view_operand(admitted_root.path(), "other.txt", AnddressTarget::File);
    assert_execution_error(run(
        admitted_root.path(),
        &["--admit", "admitted", "view", "anddress", &unadmitted],
    ));
}

#[test]
fn view_rejects_anchored_and_extra_operands() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    let operand = view_operand(root.path(), "coordinate.txt", AnddressTarget::File);

    assert_usage(run(root.path(), &["view", "anchored", "handle"]));
    assert_usage(run(root.path(), &["view", "anddress", &operand, "extra"]));
}

#[test]
fn check_reports_current_for_each_target_kind_without_address_output() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    write(root.path(), "note.txt", "file\n\nparagraph\nline\n");

    for target in [
        AnddressTarget::File,
        AnddressTarget::Paragraph {
            ordinal: Natural::one(),
        },
        AnddressTarget::Line {
            ordinal: Natural::parse("3").unwrap(),
            exact_extent: "line\n".to_owned(),
        },
    ] {
        let operand = view_operand(root.path(), "note.txt", target);
        let output = run(root.path(), &["check", "anddress", &operand]);
        assert_check_status(output, "Current");
    }
}

#[test]
fn check_reports_not_current_and_unavailable_from_the_runtime_report() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    write(root.path(), "note.txt", "actual\n");

    let stale = view_operand(
        root.path(),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "actual\n".to_owned(),
        },
    );
    assert_check_status(
        run(root.path(), &["check", "anddress", &stale]),
        "NotCurrent",
    );

    let wrong_extent = view_operand(
        root.path(),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "wrong\n".to_owned(),
        },
    );
    assert_check_status(
        run(root.path(), &["check", "anddress", &wrong_extent]),
        "NotCurrent",
    );

    let missing = view_operand(root.path(), "missing.txt", AnddressTarget::File);
    assert_check_status(
        run(root.path(), &["check", "anddress", &missing]),
        "NotCurrent",
    );

    let unavailable = view_operand(root.path(), "broken.txt", AnddressTarget::File);
    write(root.path(), "broken.txt", "broken\0");
    assert_check_status(
        run(root.path(), &["check", "anddress", &unavailable]),
        "Unavailable",
    );
}

#[test]
fn check_rejects_invalid_forms_and_extra_operands() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    let operand = view_operand(root.path(), "coordinate.txt", AnddressTarget::File);

    assert_usage(run(root.path(), &["check", "anddress", "{"]));
    assert_usage(run(
        root.path(),
        &["check", "anddress", r#"{"version":"old","kind":null}"#],
    ));
    assert_usage(run(
        root.path(),
        &[
            "check",
            "anddress",
            r#"{"version":"artext.backwriter-anddress.v3","workspaceCoordinate":"x","logicalPath":"note.txt","kind":"file"}"#,
        ],
    ));
    assert_usage(run(root.path(), &["check", "search", "value"]));
    assert_usage(run(root.path(), &["check", "pick", "value"]));
    assert_usage(run(root.path(), &["check", "anddress", &operand, "extra"]));

    let unavailable_workspace = root.path().join("missing-workspace");
    let workspace = Command::new(binary())
        .current_dir(root.path())
        .arg("--workspace")
        .arg(unavailable_workspace)
        .args(["check", "anddress", &operand])
        .output()
        .unwrap();
    assert_execution_error(workspace);
}

#[test]
fn session_reuses_search_projection_view_and_check_with_exact_bindings() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\nneedle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle --source note.txt\nlet copied_hits = @hits\nlet second = @copied_hits[1]\nlet copied_second = @second\nview anddress @copied_second\ncheck anddress @hits[1]\nexit\n",
    );
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"Found 2\n0\tLine\tnote.txt:0\n1\tLine\tnote.txt:1\nneedle\nCurrent\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn session_pick_all_and_target_kind_project_the_existing_core_order() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "first\n\nsecond\n");

    let output = run_shell(
        root.path(),
        "let files = search file first\npick @files all\nlet paragraphs = search paragraph first\npick @paragraphs target-kind paragraph\nlet lines = search line first\npick @lines target-kind line\nexit\n",
    );
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"Found 1\n0\tFile\tnote.txt\nSelected 1\n0\tFile\tnote.txt\nFound 1\n0\tParagraph\tnote.txt:0\nSelected 1\n0\tParagraph\tnote.txt:0\nFound 1\n0\tLine\tnote.txt:0\nSelected 1\n0\tLine\tnote.txt:0\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn session_pick_same_file_and_one_of_preserve_candidate_order() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "a.txt", "needle\nneedle\n");
    write(root.path(), "b.txt", "needle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle\nlet same = pick @hits same-file @hits[0]\nlet selected = pick @hits all-of(one-of @hits[2] @hits[0])\npick @same all\nexit\n",
    );
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"Found 3\n0\tLine\ta.txt:0\n1\tLine\ta.txt:1\n2\tLine\tb.txt:0\nSelected 2\n0\tLine\ta.txt:0\n1\tLine\ta.txt:1\nSelected 2\n0\tLine\ta.txt:0\n1\tLine\tb.txt:0\nSelected 2\n0\tLine\ta.txt:0\n1\tLine\ta.txt:1\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn session_pick_composition_is_iterative_and_pick_bindings_feed_view_and_check() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\nneedle\nneedle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle\nlet selected = pick @hits all-of (target-kind line) (not (one-of @hits[1]))\npick @selected any-of(all) (not(not(target-kind line)))\nview anddress @selected[1]\ncheck anddress @selected[1]\nexit\n",
    );
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"Found 3\n0\tLine\tnote.txt:0\n1\tLine\tnote.txt:1\n2\tLine\tnote.txt:2\nSelected 2\n0\tLine\tnote.txt:0\n1\tLine\tnote.txt:2\nSelected 2\n0\tLine\tnote.txt:0\n1\tLine\tnote.txt:2\nneedle\nCurrent\n"
    );
    assert!(output.stderr.is_empty());

    let nesting = 4_096;
    let input = format!(
        "let hits = search line needle\npick @hits {}all{}\nexit\n",
        "not(".repeat(nesting),
        ")".repeat(nesting)
    );
    let deep = run_shell(root.path(), &input);
    assert!(deep.status.success());
    assert_eq!(
        deep.stdout,
        b"Found 3\n0\tLine\tnote.txt:0\n1\tLine\tnote.txt:1\n2\tLine\tnote.txt:2\nSelected 3\n0\tLine\tnote.txt:0\n1\tLine\tnote.txt:1\n2\tLine\tnote.txt:2\n"
    );
    assert!(deep.stderr.is_empty());
}

#[test]
fn session_pick_rejects_malformed_references_and_preserves_existing_bindings() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle\nlet address = @hits[0]\nlet empty = search line absent\npick @empty all\npick @hits all trailing\npick @hits not(all\npick @hits all-of()\npick @hits one-of\npick @hits same-file @hits\npick @hits target-kind section\npick @hits unknown\npick @missing all\npick @hits[0] all\npick @address all\nlet selected = pick @hits all\nview anddress @selected\nview anddress @selected[1]\nsearch line needle --source note.txt\nexit\n",
    );
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        output.stdout,
        b"Found 1\n0\tLine\tnote.txt:0\nFound 0\nSelected 0\nSelected 1\n0\tLine\tnote.txt:0\nFound 1\n0\tLine\tnote.txt:0\n"
    );
    let stderr = text(output.stderr);
    assert!(stderr.contains("Pick predicate has trailing input"));
    assert!(stderr.contains("unclosed Pick predicate parenthesis"));
    assert!(stderr.contains("Pick composition requires at least one predicate"));
    assert!(stderr.contains("one-of requires at least one Anddress reference"));
    assert!(stderr.contains("Search binding requires an index: hits"));
    assert!(stderr.contains("invalid Pick target kind: section"));
    assert!(stderr.contains("invalid Pick predicate: unknown"));
    assert!(stderr.contains("unknown binding: missing"));
    assert!(stderr.contains("Pick candidates require a Search or Pick binding without an index"));
    assert!(stderr.contains("Pick candidates require a Search or Pick binding: address"));
    assert!(stderr.contains("Pick binding requires an index: selected"));
    assert!(stderr.contains("binding index is out of range: selected"));
}

#[test]
fn session_batch_check_reports_search_and_pick_counts_without_changing_bindings() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "current.txt", "needle\n");
    write(root.path(), "removed.txt", "needle\n");
    write(root.path(), "unavailable.txt", "needle\n");

    let output = run_shell_after_initial_output(
        root.path(),
        "let hits = search line needle\n",
        4,
        || {
            write(root.path(), "removed.txt", "changed\n");
            fs::write(root.path().join("unavailable.txt"), b"needle\0").unwrap();
        },
        "check search @hits\nlet selected = pick @hits all\ncheck pick @selected\npick @selected all\nexit\n",
    );
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"Found 3\n0\tLine\tcurrent.txt:0\n1\tLine\tremoved.txt:0\n2\tLine\tunavailable.txt:0\nChecked 3\nCurrent 1\nNotCurrent 1\nUnavailable 1\nSelected 3\n0\tLine\tcurrent.txt:0\n1\tLine\tremoved.txt:0\n2\tLine\tunavailable.txt:0\nChecked 3\nCurrent 1\nNotCurrent 1\nUnavailable 1\nSelected 3\n0\tLine\tcurrent.txt:0\n1\tLine\tremoved.txt:0\n2\tLine\tunavailable.txt:0\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn session_batch_check_accepts_empty_outcomes_and_rejects_invalid_binding_forms() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let empty = run_shell(
        root.path(),
        "let empty_search = search line absent\ncheck search @empty_search\nlet empty_pick = pick @empty_search all\ncheck pick @empty_pick\nexit\n",
    );
    assert!(empty.status.success());
    assert_eq!(
        empty.stdout,
        b"Found 0\nChecked 0\nCurrent 0\nNotCurrent 0\nUnavailable 0\nSelected 0\nChecked 0\nCurrent 0\nNotCurrent 0\nUnavailable 0\n"
    );
    assert!(empty.stderr.is_empty());

    let invalid = run_shell(
        root.path(),
        "let hits = search line needle\nlet selected = pick @hits all\nlet address = @hits[0]\ncheck search @selected\ncheck pick @hits\ncheck search @address\ncheck pick @address\ncheck search @hits[0]\ncheck pick @selected[0]\ncheck search @missing\ncheck search @hits extra\ncheck pick @selected extra\ncheck anddress @hits[0]\nexit\n",
    );
    assert_eq!(invalid.status.code(), Some(2));
    assert_eq!(
        invalid.stdout,
        b"Found 1\n0\tLine\tnote.txt:0\nSelected 1\n0\tLine\tnote.txt:0\nCurrent\n"
    );
    let stderr = text(invalid.stderr);
    assert!(stderr.contains("check search requires a Search binding"));
    assert!(stderr.contains("check pick requires a Pick binding"));
    assert!(stderr.contains("indexed binding references select an Anddress"));
    assert!(stderr.contains("unknown binding: missing"));
    assert!(stderr.contains("check search accepts exactly one binding"));
    assert!(stderr.contains("check pick accepts exactly one binding"));
}

#[test]
fn session_bindings_reject_unknown_duplicate_empty_out_of_range_and_type_mismatch() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle\nlet malformed =search line needle\nlet hits = @hits\nview anddress @hits\nlet selected = @hits[0]\ncheck anddress @selected[0]\nlet empty = search line absent\nview anddress @empty[0]\nview anddress @missing\nsearch line needle\nexit\n",
    );
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        output.stdout,
        b"Found 1\n0\tLine\tnote.txt:0\nFound 0\nFound 1\n0\tLine\tnote.txt:0\n"
    );
    let stderr = text(output.stderr);
    assert!(stderr.contains("let requires a standalone = token"));
    assert!(stderr.contains("binding already exists: hits"));
    assert!(stderr.contains("Search binding requires an index: hits"));
    assert!(stderr.contains("Anddress binding cannot be indexed: selected"));
    assert!(stderr.contains("Search binding is empty: empty"));
    assert!(stderr.contains("unknown binding: missing"));
}

#[test]
fn session_lexer_exit_and_eof_follow_the_initial_grammar() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "a space\nquote: \" and slash \\\n");

    let lexical = run_shell(
        root.path(),
        "\n\t \nlet spaced = search line \"a space\"\nlet escaped = search line \"quote: \\\" and slash \\\\\"\nview anddress @escaped[0]\nsearch line \"\"\nsearch line \"bad\\q\"\nsearch line \"unterminated\nsearch line \0\nsearch line \"a space\" | ignored\nsearch line \"a space\"\nexit extra\nsearch line \"a space\"\nexit\nsearch line \"a space\"\n",
    );
    assert_eq!(lexical.status.code(), Some(2));
    assert_eq!(
        lexical.stdout,
        b"Found 1\n0\tLine\tnote.txt:0\nFound 1\n0\tLine\tnote.txt:1\nquote: \" and slash \\\nFound 1\n0\tLine\tnote.txt:0\nFound 1\n0\tLine\tnote.txt:0\n"
    );
    let stderr = text(lexical.stderr);
    assert!(stderr.contains("search query is invalid"));
    assert!(stderr.contains("invalid quoted escape"));
    assert!(stderr.contains("unmatched quote"));
    assert!(stderr.contains("Session input must not contain NUL"));
    assert!(stderr.contains("invalid search option: |"));
    assert!(stderr.contains("exit accepts no operands"));

    let eof = run_shell(root.path(), "\nsearch line \"a space\"\n");
    assert!(eof.status.success());
    assert_eq!(eof.stdout, b"Found 1\n0\tLine\tnote.txt:0\n");
    assert!(eof.stderr.is_empty());
}

#[test]
fn session_preserves_execution_then_usage_exit_precedence_without_latest_state() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let execution_only = run_shell(
        root.path(),
        "search line needle --source missing.txt\nsearch line needle\n",
    );
    assert_eq!(execution_only.status.code(), Some(1));
    assert_eq!(execution_only.stdout, b"Found 1\n0\tLine\tnote.txt:0\n");
    assert!(text(execution_only.stderr).contains("workspace source is unavailable"));

    let no_latest = run_shell(root.path(), "search line needle\nview anddress @latest\n");
    assert_eq!(no_latest.status.code(), Some(2));
    assert_eq!(no_latest.stdout, b"Found 1\n0\tLine\tnote.txt:0\n");
    assert!(text(no_latest.stderr).contains("unknown binding: latest"));

    let execution_then_usage = run_shell(
        root.path(),
        "search line needle --source missing.txt\nunknown\nsearch line needle\n",
    );
    assert_eq!(execution_then_usage.status.code(), Some(2));
    assert_eq!(
        execution_then_usage.stdout,
        b"Found 1\n0\tLine\tnote.txt:0\n"
    );
    assert!(text(execution_then_usage.stderr).contains("unsupported Session command: unknown"));
}
