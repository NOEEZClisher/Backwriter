use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
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
        vec!["shell"],
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
