use std::{
    fs,
    io::Write,
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
