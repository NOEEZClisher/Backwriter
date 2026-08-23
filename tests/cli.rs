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
fn session_anchor_creates_views_and_invalidates_only_the_selected_source() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "left.txt", "needle\n");
    write(root.path(), "right.txt", "needle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle\nlet left = anchor create @hits[0]\nlet duplicate = anchor create @hits[0]\nlet right = anchor create @hits[1]\nview anchored @left\nanchor invalidate-source left.txt\nview anchored @right\nexit\n",
    );
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"Found 2\n0\tLine\tleft.txt:0\n1\tLine\tright.txt:0\nAnchored\nAlreadyLive\nAnchored\nneedle\nOK\nneedle\n"
    );
    assert!(output.stderr.is_empty());

    let invalidated = run_shell(
        root.path(),
        "let hits = search line needle\nlet handle = anchor create @hits[0]\nanchor invalidate-source left.txt\nview anchored @handle\nsearch line needle\nexit\n",
    );
    assert_eq!(invalidated.status.code(), Some(1));
    assert_eq!(
        invalidated.stdout,
        b"Found 2\n0\tLine\tleft.txt:0\n1\tLine\tright.txt:0\nAnchored\nOK\nFound 2\n0\tLine\tleft.txt:0\n1\tLine\tright.txt:0\n"
    );
    assert!(text(invalidated.stderr).contains("unavailable"));

    let invalid = run_shell(
        root.path(),
        "let hits = search line needle\nlet handle = anchor create @hits[0]\nlet alias = @handle\nview anchored @handle[0]\nanchor create @hits[0]\nanchor invalidate-source left.txt extra\nview anchored @missing\nexit\n",
    );
    assert_eq!(invalid.status.code(), Some(2));
    let stderr = text(invalid.stderr);
    assert!(stderr.contains("Anchedress binding cannot be cloned"));
    assert!(stderr.contains("Anchedress bindings cannot be indexed"));
    assert!(stderr.contains("anchor create is available only"));
    assert!(stderr.contains("invalidate-source accepts exactly"));
    assert!(stderr.contains("unknown binding: missing"));
}

#[test]
fn session_anchor_preserves_file_paragraph_and_line_views() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n\nsecond needle\n");

    let output = run_shell(
        root.path(),
        "let files = search file needle\nlet paragraphs = search paragraph needle\nlet lines = search line needle\nlet file = anchor create @files[0]\nlet paragraph = anchor create @paragraphs[0]\nlet line = anchor create @lines[0]\nview anchored @file\nview anchored @paragraph\nview anchored @line\nexit\n",
    );
    assert!(output.status.success());
    let stdout = text(output.stdout);
    assert_eq!(stdout.matches("Anchored\n").count(), 3);
    assert!(stdout.ends_with("needle\n\nsecond needle\nneedle\nneedle\n"));
    assert!(output.stderr.is_empty());

    let direct = run(root.path(), &["anchor", "create", "not-an-address"]);
    assert_eq!(direct.status.code(), Some(2));
}

#[test]
fn session_edit_apply_builds_each_core_edit_and_preserves_bindings() {
    let cases = [
        (
            "one\n",
            "let lines = search line one\nlet edit = edit insert before @lines[0] \"zero\\n\"\nlet copy = @edit\napply @edit\nexit\n",
            "zero\none\n",
        ),
        (
            "one\n",
            "let lines = search line one\nlet edit = edit replace @lines[0] \"two\\r\\n\"\napply @edit\nexit\n",
            "two\r\n",
        ),
        (
            "one\n",
            "let lines = search line one\nlet edit = edit delete @lines[0]\napply @edit\nexit\n",
            "",
        ),
        (
            "a\nb\n",
            "let lines = search line a\nlet files = search file a\nlet edit = edit move @lines[0] end-of @files[0]\napply @edit\nexit\n",
            "b\na\n",
        ),
        (
            "a\n",
            "let lines = search line a\nlet files = search file a\nlet edit = edit copy @lines[0] end-of @files[0]\napply @edit\nexit\n",
            "a\na\n",
        ),
    ];
    for (before, input, after) in cases {
        let root = tempfile::tempdir().unwrap();
        write(root.path(), "note.txt", before);
        let output = run_shell(root.path(), input);
        assert!(output.status.success(), "{}", text(output.stderr));
        assert_eq!(
            fs::read_to_string(root.path().join("note.txt")).unwrap(),
            after
        );
        assert!(text(output.stdout).contains("OK\n"));
    }
}

#[test]
fn session_edit_apply_rejects_invalid_forms_without_stopping_later_commands() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "one\n");
    let output = run_shell(
        root.path(),
        "edit delete @missing\nlet lines = search line one\nlet wrong = @lines\nlet bad = edit insert start-of @lines[0] x\nlet edit = edit insert before @lines[0] \"\\t\\\"\\\\\\r\\n\"\napply @edit[0]\napply @wrong\napply @edit extra\napply @edit\nexit\n",
    );
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        fs::read(root.path().join("note.txt")).unwrap(),
        b"\t\"\\\r\none\n"
    );
    let stderr = text(output.stderr);
    assert!(stderr.contains("unsupported Session command: edit"));
    assert!(stderr.contains("binding is not an Edit: wrong"));
    assert!(stderr.contains("Edit bindings cannot be indexed"));
    assert!(stderr.contains("binding is not an Edit: wrong"));
    assert!(stderr.contains("apply accepts exactly one Edit binding"));
}

#[test]
fn session_view_and_check_result_bindings_keep_direct_output_and_clone_only_results() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");
    let output = run_shell(
        root.path(),
        "let lines = search line needle\nlet view = view anddress @lines[0]\nlet view_copy = @view\nlet raw_check = check anddress @lines[0]\nlet raw_copy = @raw_check\nlet search_check = check search @lines\nlet search_copy = @search_check\nlet picked = pick @lines all\nlet pick_check = check pick @picked\nlet pick_copy = @pick_check\nlet handle = anchor create @lines[0]\nlet anchored = view anchored @handle\nview anchored @handle\nview anddress @view\ncheck search @search_check\napply @raw_check\nexit\n",
    );
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        output.stdout,
        b"Found 1\n0\tLine\tnote.txt:0\nneedle\nCurrent\nChecked 1\nCurrent 1\nNotCurrent 0\nUnavailable 0\nSelected 1\n0\tLine\tnote.txt:0\nChecked 1\nCurrent 1\nNotCurrent 0\nUnavailable 0\nAnchored\nneedle\nneedle\n"
    );
    let stderr = text(output.stderr);
    assert!(stderr.contains("check search requires a Search binding"));
    assert!(stderr.contains("binding is not an Edit: raw_check"));
}

#[test]
fn session_data_stores_gets_and_binds_all_native_value_kinds() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle\nlet address = @hits[0]\nlet picked = pick @hits all\nlet viewed = view anddress @address\nlet checked_anddress = check anddress @address\nlet checked_search = check search @hits\nlet checked_pick = check pick @picked\ndata store anddress \"quoted\\\"slash\\\\\" @hits[0]\ndata store search shared @hits\ndata store pick shared @picked\ndata store view shared @viewed\ndata store check-anddress shared @checked_anddress\ndata store check-search shared @checked_search\ndata store check-pick shared @checked_pick\ndata list\ndata get anddress \"quoted\\\"slash\\\\\"\ndata get search shared\ndata get pick shared\ndata get view shared\ndata get check-anddress shared\ndata get check-search shared\ndata get check-pick shared\nlet restored_address = data get anddress \"quoted\\\"slash\\\\\"\nlet restored_search = data get search shared\nlet restored_pick = data get pick shared\nlet restored_view = data get view shared\nlet restored_check_anddress = data get check-anddress shared\nlet restored_check_search = data get check-search shared\nlet restored_check_pick = data get check-pick shared\nview anddress @restored_address\npick @restored_search all\ncheck pick @restored_pick\nexit\n",
    );

    assert!(output.status.success(), "{}", text(output.stderr));
    let stdout = text(output.stdout);
    assert_eq!(stdout.matches("OK\n").count(), 7);
    assert!(stdout.contains(
        "anddress\t\"quoted\\\"slash\\\\\"\nsearch\t\"shared\"\npick\t\"shared\"\nview\t\"shared\"\ncheck-anddress\t\"shared\"\ncheck-search\t\"shared\"\ncheck-pick\t\"shared\"\n"
    ));
    assert_eq!(stdout.matches("Anddress\tLine\tnote.txt:0\n").count(), 2);
    assert!(stdout.matches("Found 1\n0\tLine\tnote.txt:0\n").count() >= 3);
    assert!(stdout.matches("Selected 1\n0\tLine\tnote.txt:0\n").count() >= 3);
    assert!(stdout.matches("needle\n").count() >= 3);
    assert!(stdout.matches("Current\n").count() >= 3);
    assert!(
        stdout
            .matches("Checked 1\nCurrent 1\nNotCurrent 0\nUnavailable 0\n")
            .count()
            >= 5
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn session_data_rejects_wrong_values_preserves_entries_and_drops_at_eof() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let invalid = run_shell(
        root.path(),
        "let hits = search line needle\nlet edit = edit delete @hits[0]\nlet handle = anchor create @hits[0]\ndata store search stored @hits\ndata store search stored @hits\ndata store pick stored @hits\ndata store anddress empty @hits[0]\ndata store anddress \"\" @hits[0]\ndata store anddress bad-edit @edit\ndata store search bad-anchor @handle\ndata rename search stored stored\ndata rename search stored renamed\ndata get search stored\ndata get search renamed\ndata remove search renamed\ndata get search renamed\nlet restored = data get anddress empty\ndata store search invalid @restored\ndata store view invalid @hits[0]\ndata store search indexed @hits[0]\ndata list extra\nexit\n",
    );
    assert_eq!(invalid.status.code(), Some(2));
    assert_eq!(
        invalid.stdout,
        b"Found 1\n0\tLine\tnote.txt:0\nAnchored\nOK\nOK\nOK\nFound 1\n0\tLine\tnote.txt:0\nOK\nAnddress\tLine\tnote.txt:0\n"
    );
    let stderr = text(invalid.stderr);
    assert!(stderr.contains("Data entry already exists"));
    assert!(stderr.contains("Data kind does not match binding"));
    assert!(stderr.contains("Data name is empty"));
    assert!(stderr.contains("Edit binding cannot be used as an Anddress: edit"));
    assert!(stderr.contains("Anchedress binding cannot be cloned: handle"));
    assert!(stderr.contains("Data entry was not found"));
    assert!(stderr.contains("indexed binding references select an Anddress"));
    assert!(stderr.contains("unsupported data command"));

    let next_session = run_shell(root.path(), "data list\ndata get anddress empty\nexit\n");
    assert_eq!(next_session.status.code(), Some(2));
    assert!(next_session.stdout.is_empty());
    assert!(text(next_session.stderr).contains("Data entry was not found"));
    assert_usage(run(root.path(), &["data", "list"]));
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
