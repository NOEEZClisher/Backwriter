mod support;

#[path = "cli/check.rs"]
mod check;
#[path = "cli/edit.rs"]
mod edit;
#[path = "cli/help.rs"]
mod help;
#[path = "cli/shell.rs"]
mod shell;
#[path = "cli/view.rs"]
mod view;
use help::*;

use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};

#[cfg(unix)]
use std::{
    env,
    os::unix::fs::{MetadataExt, PermissionsExt},
};

use backwriter::{
    backwriter::{
        anddress::{Anddress, AnddressTarget as PublicAnddressTarget},
        check::{CheckOutcome, CheckStatus},
        search::{SearchOutcome, SearchQuery, SearchRequest, SearchScope, SearchTarget},
        view::ViewOutcome,
    },
    runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime},
};
use serde_json::Value;

const EDIT_CONTENT_NUL_CAUSE: &str = "Edit Content must not contain NUL.";
const EDIT_LINE_BODY_CAUSE: &str = "Line Edit accepts body Content only. Backwriter preserves the existing Line terminator automatically. Exact extent replacement is available through advanced raw Session Edit/Apply.";

fn binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_bw"))
}

fn run(root: &Path, arguments: &[&str]) -> Output {
    Command::new(binary())
        .current_dir(root)
        .args(arguments)
        .output()
        .unwrap()
}

fn run_with_stdin(root: &Path, arguments: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(binary())
        .current_dir(root)
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.as_mut().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
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

#[cfg(unix)]
fn write_executable(root: &Path, path: &str, content: &str) {
    write(root, path, content);
    fs::set_permissions(root.join(path), fs::Permissions::from_mode(0o755)).unwrap();
}

#[derive(Clone)]
struct Natural(usize);

impl Natural {
    fn zero() -> Self {
        Self(0)
    }

    fn one() -> Self {
        Self(1)
    }

    fn parse(value: &str) -> Result<Self, std::num::ParseIntError> {
        value.parse().map(Self)
    }
}

#[derive(Clone)]
enum AnddressTarget {
    File,
    Paragraph {
        ordinal: Natural,
    },
    Line {
        ordinal: Natural,
        exact_extent: String,
    },
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
    let coordinate = anddresses[0].workspace_coordinate();
    let source = fs::read(root.join(path)).unwrap_or_default();
    let address = match target {
        AnddressTarget::File => support::file(coordinate, path, &source),
        AnddressTarget::Paragraph { ordinal } => {
            let paragraphs = paragraph_ranges(&source);
            let (start, end) = paragraphs.get(ordinal.0).copied().unwrap_or((0, 0));
            support::address(
                coordinate,
                path,
                &source,
                PublicAnddressTarget::Paragraph,
                start,
                end,
            )
        }
        AnddressTarget::Line {
            ordinal,
            exact_extent,
        } => {
            let spans = support::line_spans(&source);
            if let Some((start, end)) = spans.get(ordinal.0).copied()
                && source[start..end] == *exact_extent.as_bytes()
            {
                support::address(
                    coordinate,
                    path,
                    &source,
                    PublicAnddressTarget::Line,
                    start,
                    end,
                )
            } else {
                let mut stale_source = exact_extent.into_bytes();
                stale_source.push(b'!');
                support::address(
                    coordinate,
                    path,
                    &stale_source,
                    PublicAnddressTarget::Line,
                    0,
                    stale_source.len() - 1,
                )
            }
        }
    };
    String::from_utf8(address.encode().unwrap()).unwrap()
}

fn paragraph_ranges(source: &[u8]) -> Vec<(usize, usize)> {
    let mut paragraphs = Vec::new();
    let mut paragraph_start = None;
    let mut paragraph_end = 0;
    for (start, end) in support::line_spans(source) {
        let mut body_end = end;
        if source[..body_end].ends_with(b"\r\n") {
            body_end -= 2;
        } else if source[..body_end].ends_with(b"\r") || source[..body_end].ends_with(b"\n") {
            body_end -= 1;
        }
        let text = source[start..body_end]
            .iter()
            .any(|byte| !matches!(byte, b' ' | b'\t'));
        if text {
            paragraph_start.get_or_insert(start);
            paragraph_end = end;
        } else if let Some(paragraph_start) = paragraph_start.take() {
            paragraphs.push((paragraph_start, paragraph_end));
        }
    }
    if let Some(paragraph_start) = paragraph_start {
        paragraphs.push((paragraph_start, paragraph_end));
    }
    paragraphs
}

fn assert_usage(output: Output) {
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let stderr = text(output.stderr);
    assert!(stderr.contains("\nusage:\n") || stderr.contains("\n\nUSAGE\n"));
}

fn usage_from_help(help: &str) -> &str {
    let usage = help.split_once("USAGE\n").unwrap().1;
    usage.split_once("\n\n").map_or(usage, |(usage, _)| usage)
}

fn assert_actionable_usage(output: Output, code: &str, cause: &str, help: &str, hint: &str) {
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(
        text(output.stderr),
        format!(
            "error[{code}]:\n{cause}\n\nusage:\n{}\n\nhint:\nrun `{hint}`\n",
            usage_from_help(help)
        )
    );
}

fn assert_execution_error(output: Output) {
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(text(output.stderr).starts_with("error: "));
}

fn assert_unavailable(output: Output) {
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(text(output.stderr).contains("unavailable"));
}

fn assert_check_status(output: Output, status: &str) {
    assert!(output.status.success());
    assert_eq!(output.stdout, format!("{status}\n").as_bytes());
    assert!(output.stderr.is_empty());
}

fn raw_check_status(outcome: &CheckOutcome<Option<Anddress>>) -> &'static str {
    match (
        outcome.filtered.is_some(),
        outcome.report.current_count(),
        outcome.report.removed_count(),
        outcome.report.unavailable_count(),
        outcome.report.checked_count(),
    ) {
        (true, 1, 0, 0, 1) => "current",
        (false, 0, 1, 0, 1) => "not-current",
        (true, 0, 0, 1, 1) => "unavailable",
        _ => panic!("inconsistent raw Check report"),
    }
}

fn expected_check_json(inputs: &[Anddress], statuses: &[CheckStatus]) -> Vec<u8> {
    assert_eq!(inputs.len(), statuses.len());
    let mut output = b"{\"schema\":\"bw.cli.check.v2\",\"outcomes\":[".to_vec();
    for (index, (input, status)) in inputs.iter().zip(statuses).enumerate() {
        if index != 0 {
            output.push(b',');
        }
        let label = match status {
            CheckStatus::Current => "current",
            CheckStatus::NotCurrent => "not-current",
            CheckStatus::Unavailable => "unavailable",
        };
        output.extend_from_slice(b"{\"status\":\"");
        output.extend_from_slice(label.as_bytes());
        output.extend_from_slice(b"\",\"anddress\":");
        if *status == CheckStatus::NotCurrent {
            output.extend_from_slice(b"null");
        } else {
            output.extend_from_slice(&input.encode().unwrap());
        }
        output.push(b'}');
    }
    output.extend_from_slice(b"]}\n");
    output
}

fn assert_check_json(output: Output, inputs: &[Anddress], statuses: &[CheckStatus]) {
    assert!(output.status.success());
    assert_eq!(output.stdout, expected_check_json(inputs, statuses));
    assert!(output.stderr.is_empty());

    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(document["schema"], "bw.cli.check.v2");
    let outcomes = document["outcomes"].as_array().unwrap();
    assert_eq!(outcomes.len(), inputs.len());
    for ((input, status), outcome) in inputs.iter().zip(statuses).zip(outcomes) {
        let label = match status {
            CheckStatus::Current => "current",
            CheckStatus::NotCurrent => "not-current",
            CheckStatus::Unavailable => "unavailable",
        };
        assert_eq!(outcome["status"], label);
        if *status == CheckStatus::NotCurrent {
            assert!(outcome["anddress"].is_null());
        } else {
            let encoded = serde_json::to_vec(&outcome["anddress"]).unwrap();
            assert_eq!(Anddress::decode(&encoded).unwrap(), *input);
        }
    }
}

fn expected_edit_output(outcome: &str, anddress: Option<&Anddress>, json: bool) -> Vec<u8> {
    let mut output = if json {
        format!("{{\"schema\":\"bw.cli.edit.v1\",\"outcome\":\"{outcome}\",\"anddress\":")
            .into_bytes()
    } else {
        let label = if outcome == "unchanged" {
            "Unchanged"
        } else {
            "Changed"
        };
        format!("{label}\t").into_bytes()
    };
    match anddress {
        Some(anddress) => output.extend_from_slice(&anddress.encode().unwrap()),
        None if json => output.extend_from_slice(b"null"),
        None => output.extend_from_slice(b"None"),
    }
    if json {
        output.push(b'}');
    }
    output.push(b'\n');
    output
}

fn assert_edit_output(output: Output, outcome: &str, expected: Option<&Anddress>, json: bool) {
    assert!(output.status.success(), "{}", text(output.stderr.clone()));
    assert_eq!(output.stdout, expected_edit_output(outcome, expected, json));
    assert!(output.stderr.is_empty());

    if json {
        let document: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(document["schema"], "bw.cli.edit.v1");
        assert_eq!(document["outcome"], outcome);
        match expected {
            Some(expected) => {
                let prefix = format!(
                    "{{\"schema\":\"bw.cli.edit.v1\",\"outcome\":\"{outcome}\",\"anddress\":"
                );
                let encoded = output
                    .stdout
                    .strip_prefix(prefix.as_bytes())
                    .and_then(|value| value.strip_suffix(b"}\n"))
                    .unwrap();
                assert_eq!(Anddress::decode(encoded).unwrap(), *expected);
                assert_eq!(encoded, expected.encode().unwrap());
            }
            None => assert!(document["anddress"].is_null()),
        }
    }
}

fn expected_search_json(outcome: &SearchOutcome) -> Vec<u8> {
    let mut output = b"{\"schema\":\"bw.cli.search.v2\",\"outcome\":\"".to_vec();
    match outcome {
        SearchOutcome::Empty => output.extend_from_slice(b"empty\",\"occurrences\":[]}"),
        SearchOutcome::Found { anddresses } => {
            output.extend_from_slice(b"found\",\"occurrences\":[");
            for (index, anddress) in anddresses.iter().enumerate() {
                if index != 0 {
                    output.push(b',');
                }
                output.extend_from_slice(b"{\"logicalPath\":");
                serde_json::to_writer(&mut output, anddress.logical_path()).unwrap();
                match anddress.target() {
                    PublicAnddressTarget::File => output.extend_from_slice(b",\"kind\":\"file\""),
                    PublicAnddressTarget::Line => {
                        let line = anddress.line_number().unwrap();
                        write!(output, ",\"kind\":\"line\",\"line\":\"{line}\"").unwrap();
                    }
                    PublicAnddressTarget::Paragraph => {
                        let lines = anddress.line_range();
                        let start_line = lines.start + 1;
                        let end_line = lines.end;
                        write!(
                            output,
                            ",\"kind\":\"paragraph\",\"lineStart\":\"{start_line}\",\"lineEnd\":\"{end_line}\""
                        )
                        .unwrap();
                    }
                }
                output.extend_from_slice(b",\"anddress\":");
                output.extend_from_slice(&anddress.encode().unwrap());
                output.push(b'}');
            }
            output.extend_from_slice(b"]}");
        }
    }
    output.push(b'\n');
    output
}

fn assert_search_json(output: Output, expected: &SearchOutcome) {
    assert!(output.status.success());
    assert_eq!(output.stdout, expected_search_json(expected));
    assert!(output.stderr.is_empty());

    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(document["schema"], "bw.cli.search.v2");
    assert_eq!(
        document["outcome"],
        match expected {
            SearchOutcome::Empty => "empty",
            SearchOutcome::Found { .. } => "found",
        }
    );
    let actual = document["occurrences"].as_array().unwrap();
    let expected = match expected {
        SearchOutcome::Empty => &[] as &[Anddress],
        SearchOutcome::Found { anddresses } => anddresses,
    };
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        assert_eq!(actual["logicalPath"], expected.logical_path());
        match expected.target() {
            PublicAnddressTarget::File => {
                assert_eq!(actual["kind"], "file");
                assert!(actual.get("line").is_none());
                assert!(actual.get("lineStart").is_none());
                assert!(actual.get("lineEnd").is_none());
            }
            PublicAnddressTarget::Line => {
                let line = expected.line_number().unwrap();
                assert_eq!(actual["kind"], "line");
                assert_eq!(actual["line"], line.to_string());
            }
            PublicAnddressTarget::Paragraph => {
                let lines = expected.line_range();
                let start_line = lines.start + 1;
                let end_line = lines.end;
                assert_eq!(actual["kind"], "paragraph");
                assert_eq!(actual["lineStart"], start_line.to_string());
                assert_eq!(actual["lineEnd"], end_line.to_string());
            }
        }
        let encoded = serde_json::to_vec(&actual["anddress"]).unwrap();
        assert_eq!(Anddress::decode(&encoded).unwrap(), *expected);
    }
}

fn expected_view_json(outcome: &ViewOutcome) -> Vec<u8> {
    expected_view_json_many(std::slice::from_ref(outcome))
}

fn expected_view_json_many(outcomes: &[ViewOutcome]) -> Vec<u8> {
    let mut output = b"{\"schema\":\"bw.cli.view.v2\",\"outcomes\":[".to_vec();
    for (index, outcome) in outcomes.iter().enumerate() {
        if index != 0 {
            output.push(b',');
        }
        match outcome {
            ViewOutcome::Projected { anddress, content } => {
                output.extend_from_slice(b"{\"outcome\":\"projected\",\"anddress\":");
                output.extend_from_slice(&anddress.encode().unwrap());
                output.extend_from_slice(b",\"content\":");
                serde_json::to_writer(&mut output, content).unwrap();
                output.extend_from_slice(b"}");
            }
            ViewOutcome::RelationAbsent => {
                output.extend_from_slice(b"{\"outcome\":\"relation-absent\"}");
            }
        }
    }
    output.extend_from_slice(b"]}\n");
    output
}

fn expected_human_view(outcome: &ViewOutcome) -> Vec<u8> {
    match outcome {
        ViewOutcome::Projected { content, .. } => content.as_bytes().to_vec(),
        ViewOutcome::RelationAbsent => panic!("CLI self-View relation exists"),
    }
}

fn assert_view_json(output: Output, expected: &ViewOutcome) {
    assert!(output.status.success());
    assert_eq!(output.stdout, expected_view_json(expected));
    assert!(output.stderr.is_empty());

    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(document["schema"], "bw.cli.view.v2");
    let items = document["outcomes"].as_array().unwrap();
    assert_eq!(items.len(), 1);
    let document = &items[0];
    match expected {
        ViewOutcome::Projected { anddress, content } => {
            assert_eq!(document["outcome"], "projected");
            assert_eq!(document["content"], content.as_str());
            let encoded = serde_json::to_vec(&document["anddress"]).unwrap();
            assert_eq!(Anddress::decode(&encoded).unwrap(), *anddress);
        }
        ViewOutcome::RelationAbsent => assert_eq!(document["outcome"], "relation-absent"),
    }
}

#[cfg(unix)]
#[test]
fn one_shot_update_delegates_to_the_https_installer_and_propagates_status() {
    let root = tempfile::tempdir().unwrap();
    let fake_bin = root.path().join("bin");
    let update_temp = root.path().join("temporary");
    fs::create_dir(&fake_bin).unwrap();
    fs::create_dir(&update_temp).unwrap();
    let curl_log = root.path().join("curl.log");
    write_executable(
        root.path(),
        "bin/curl",
        r#"#!/bin/sh
set -eu
: "${BW_FAKE_CURL_LOG:?}"
: > "$BW_FAKE_CURL_LOG"
for argument in "$@"; do
    printf '%s\n' "$argument" >> "$BW_FAKE_CURL_LOG"
done
exit_code=${BW_FAKE_CURL_EXIT-0}
if [ "$exit_code" -ne 0 ]; then
    exit "$exit_code"
fi
output=''
while [ "$#" -gt 0 ]; do
    if [ "$1" = '--output' ]; then
        shift
        output=${1-}
    fi
    shift
done
[ -n "$output" ]
printf '%s' "${BW_FAKE_INSTALLER:?}" > "$output"
"#,
    );

    let mut paths = vec![fake_bin.clone()];
    paths.extend(env::split_paths(&env::var_os("PATH").unwrap_or_default()));
    let path = env::join_paths(paths).unwrap();
    let invoke = |installer: &str, curl_exit: Option<&str>| {
        let mut command = Command::new(binary());
        command
            .current_dir(root.path())
            .arg("update")
            .env("PATH", &path)
            .env("TMPDIR", &update_temp)
            .env("BW_FAKE_CURL_LOG", &curl_log)
            .env("BW_FAKE_INSTALLER", installer);
        if let Some(exit) = curl_exit {
            command.env("BW_FAKE_CURL_EXIT", exit);
        }
        command.output().unwrap()
    };

    let success = invoke("#!/bin/sh\nprintf 'Backwriter delegated\\n'\n", None);
    assert!(success.status.success(), "{}", text(success.stderr));
    assert_eq!(success.stdout, b"Backwriter delegated\n");
    assert!(success.stderr.is_empty());
    let curl_arguments = fs::read_to_string(&curl_log).unwrap();
    let curl_arguments = curl_arguments.lines().collect::<Vec<_>>();
    assert_eq!(curl_arguments.len(), 12);
    assert_eq!(
        &curl_arguments[..10],
        [
            "--fail",
            "--show-error",
            "--silent",
            "--location",
            "--proto",
            "=https",
            "--proto-redir",
            "=https",
            "--tlsv1.2",
            "--output",
        ]
    );
    let installer_path = Path::new(curl_arguments[10]);
    assert_eq!(installer_path.file_name().unwrap(), "install.sh");
    let temporary_leaf = installer_path.parent().unwrap().file_name().unwrap();
    let temporary_leaf = temporary_leaf.to_str().unwrap();
    let nonce = temporary_leaf.strip_prefix("backwriter-update-").unwrap();
    assert_eq!(nonce.len(), 32);
    assert!(
        nonce
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    );
    assert_eq!(
        curl_arguments[11],
        "https://backwriter.pentagration.com/install.sh"
    );
    assert!(!installer_path.exists());
    assert!(fs::read_dir(&update_temp).unwrap().next().is_none());

    let installer_failure = invoke(
        "#!/bin/sh\nprintf 'installer failed\\n' >&2\nexit 7\n",
        None,
    );
    assert_eq!(installer_failure.status.code(), Some(7));
    assert!(installer_failure.stdout.is_empty());
    assert_eq!(installer_failure.stderr, b"installer failed\n");
    assert!(fs::read_dir(&update_temp).unwrap().next().is_none());

    let download_failure = invoke("#!/bin/sh\nexit 0\n", Some("22"));
    assert_eq!(download_failure.status.code(), Some(1));
    assert!(download_failure.stdout.is_empty());
    assert!(text(download_failure.stderr).contains("could not download update installer"));
    assert!(fs::read_dir(&update_temp).unwrap().next().is_none());

    let source = include_str!("../src/bin/bw.rs");
    assert!(source.contains("backwriter-update-{nonce:032x}"));
    assert!(source.contains(".arg(\"-WaitForProcessId\")"));
    assert!(source.contains(".arg(std::process::id().to_string())"));
    assert!(source.contains(".arg(\"-BootstrapRoot\")"));
    assert!(source.contains(".arg(&temporary.root)"));
    assert!(source.contains("temporary.handoff()"));
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
        "Found 5\n0\tLine\ta.txt:1\n1\tLine\tb.txt:1\n2\tLine\tdir/c.txt:1\n3\tLine\ttop.txt:1\n4\tLine\ta.txt:2\n"
    );
    let paragraph = run(root.path(), &["search", "paragraph", "needle"]);
    assert!(paragraph.status.success());
    assert_eq!(
        text(paragraph.stdout),
        "Found 4\n0\tParagraph\ta.txt:1-2\n1\tParagraph\tb.txt:1-1\n2\tParagraph\tdir/c.txt:1-1\n3\tParagraph\ttop.txt:1-1\n"
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
        "Found 2\n0\tLine\tdir/c.txt:1\n1\tLine\ttop.txt:1\n"
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
    assert_eq!(found_stdout, "Found 1\n0\tLine\tspaced file.txt:1\n");
    assert!(!found_stdout.contains("artext.backwriter-anddress"));
    assert!(!found_stdout.contains(&root.path().display().to_string()));
    assert!(found.stderr.is_empty());

    let empty = run(root.path(), &["search", "line", "absent"]);
    assert!(empty.status.success());
    assert_eq!(text(empty.stdout), "Found 0\n");
    assert!(empty.stderr.is_empty());
}

#[test]
fn one_shot_search_json_streams_exact_v5_objects_for_every_target() {
    let root = tempfile::tempdir().unwrap();
    write(
        root.path(),
        "terms.txt",
        "needle lf\nneedle cr\rneedle crlf\r\nneedle no-eol\nλ \"quote\" \\ \u{1} needle unicode\nneedle repeated\n",
    );

    for (kind, target) in [
        ("file", SearchTarget::File),
        ("paragraph", SearchTarget::Paragraph),
        ("line", SearchTarget::Line),
    ] {
        let workspace = WorkspaceRuntime::open(
            root.path(),
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap();
        let expected = workspace
            .search(&SearchRequest::new(
                SearchQuery::new("needle").unwrap(),
                SearchScope::all_admitted(),
                target,
            ))
            .unwrap();
        let output = run(
            root.path(),
            &[
                "--admit",
                ".",
                "--json",
                "search",
                kind,
                "needle",
                "--source",
                "terms.txt",
            ],
        );
        assert_search_json(output, &expected);
    }

    let line = run(
        root.path(),
        &[
            "--json",
            "search",
            "line",
            "needle",
            "--source",
            "terms.txt",
        ],
    );
    let document: Value = serde_json::from_slice(&line.stdout).unwrap();
    for value in document["occurrences"].as_array().unwrap() {
        let anddress = &value["anddress"];
        assert_eq!(anddress["version"], "artext.backwriter-anddress.v5");
        assert!(anddress.get("sourceStateHash").is_some());
        assert!(anddress.get("sourceByteLength").is_some());
        assert!(anddress.get("sourceLineCount").is_some());
        assert!(anddress.get("byteStart").is_some());
        assert!(anddress.get("byteEnd").is_some());
        assert!(anddress.get("terminator").is_some());
        assert!(anddress.get("lineOffsetInParent").is_some());
        assert!(anddress.get("parentKind").is_some());
        assert!(anddress.get("ordinal").is_none());
        assert!(anddress.get("exactExtent").is_none());
    }
}

#[test]
fn exact_file_lookup_reuses_human_json_and_validation_boundaries() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "empty.txt", "");
    write(
        root.path(),
        "nonempty.txt",
        "no matching literal is required",
    );
    fs::create_dir(root.path().join("directory")).unwrap();

    let human = run(root.path(), &["search", "/file", "empty.txt"]);
    assert!(human.status.success());
    assert_eq!(human.stdout, b"Found 1\n0\tFile\tempty.txt\n");
    assert!(human.stderr.is_empty());

    let runtime = WorkspaceRuntime::open(
        root.path(),
        WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
    )
    .unwrap();
    let expected = runtime
        .search(&SearchRequest::exact_file("empty.txt").unwrap())
        .unwrap();
    assert_search_json(
        run(root.path(), &["--json", "search", "/file", "empty.txt"]),
        &expected,
    );

    for path in ["missing.txt", "directory"] {
        let output = run(root.path(), &["search", "/file", path]);
        assert!(output.status.success());
        assert_eq!(output.stdout, b"Found 0\n");
        assert!(output.stderr.is_empty());
    }
    for path in ["../escape.txt", "/absolute.txt", "a/../b.txt"] {
        assert_usage(run(root.path(), &["search", "/file", path]));
    }
    assert_usage(run(
        root.path(),
        &["search", "/file", "empty.txt", "--source", "empty.txt"],
    ));

    let admitted = tempfile::tempdir().unwrap();
    write(admitted.path(), "inside/empty.txt", "");
    write(admitted.path(), "outside.txt", "");
    assert_execution_error(run(
        admitted.path(),
        &["--admit", "inside", "search", "/file", "outside.txt"],
    ));
}

#[test]
fn one_shot_search_json_maps_empty_and_rejects_invalid_placement() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");
    let workspace = WorkspaceRuntime::open(
        root.path(),
        WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
    )
    .unwrap();
    let expected = workspace
        .search(&SearchRequest::new(
            SearchQuery::new("absent").unwrap(),
            SearchScope::all_admitted(),
            SearchTarget::Line,
        ))
        .unwrap();
    assert_search_json(
        run(
            root.path(),
            &["--json", "search", "line", "absent", "--source", "note.txt"],
        ),
        &expected,
    );
    assert_search_json(
        run(
            root.path(),
            &[
                "--json",
                "--workspace",
                root.path().to_str().unwrap(),
                "--admit",
                ".",
                "search",
                "line",
                "absent",
                "--source",
                "note.txt",
            ],
        ),
        &expected,
    );

    for arguments in [
        vec!["--json", "--json", "search", "line", "needle"],
        vec!["search", "line", "needle", "--json"],
        vec!["--json", "shell"],
        vec!["--json", "view", "anddress", "unused"],
        vec!["--json", "check", "anddress", "unused"],
        vec!["--json", "data"],
        vec!["--raw", "search", "line", "needle"],
    ] {
        assert_usage(run(root.path(), &arguments));
    }
}

#[test]
fn one_shot_search_json_writer_has_no_value_or_result_clone_path() {
    let source = include_str!("../src/bin/bw/output.rs");
    let writer = source
        .split("fn write_search_json")
        .nth(1)
        .unwrap()
        .split("fn write_pick")
        .next()
        .unwrap();
    assert!(writer.contains("for (index, anddress) in anddresses.iter().enumerate()"));
    assert_eq!(writer.matches("let mut encoded = Vec::new();").count(), 1);
    assert!(writer.contains(".encode_into(&mut encoded)"));
    assert!(!writer.contains(".encode()"));
    assert!(writer.contains("serde_json::to_writer(&mut stdout, anddress.logical_path())"));
    assert!(!writer.contains("Value"));
    assert!(!writer.contains("collect("));
    assert!(!writer.contains(".clone()"));
}

#[test]
fn one_shot_search_json_keeps_large_result_output_streamed() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "large.txt", &"needle\n".repeat(4_097));

    let output = run(
        root.path(),
        &[
            "--json",
            "search",
            "line",
            "needle",
            "--source",
            "large.txt",
        ],
    );
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(document["occurrences"].as_array().unwrap().len(), 4_097);
}

#[test]
fn one_shot_search_json_exact_object_drives_crlf_line_edit() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "retry_budget = 3\r\n");

    let search = run(
        root.path(),
        &[
            "--json",
            "search",
            "line",
            "retry_budget = 3",
            "--source",
            "note.txt",
        ],
    );
    assert_eq!(search.status.code(), Some(0));
    assert!(search.stderr.is_empty());

    let document: Value = serde_json::from_slice(&search.stdout).unwrap();
    let item = &document["occurrences"][0];
    assert_eq!(item["logicalPath"], "note.txt");
    assert_eq!(item["kind"], "line");
    assert_eq!(item["line"], "1");
    let encoded = search
        .stdout
        .strip_prefix(b"{\"schema\":\"bw.cli.search.v2\",\"outcome\":\"found\",\"occurrences\":[{\"logicalPath\":\"note.txt\",\"kind\":\"line\",\"line\":\"1\",\"anddress\":")
        .and_then(|value| value.strip_suffix(b"}]}\n"))
        .expect("exact single-found Search envelope");
    let input = Anddress::decode(encoded).unwrap();
    assert_eq!(input.target(), PublicAnddressTarget::Line);
    assert_eq!(input.logical_path(), "note.txt");
    assert_eq!(input.source_byte_length(), 18);
    assert_eq!((input.byte_start(), input.byte_end()), (0, 18));

    let encoded = std::str::from_utf8(encoded).unwrap();
    let edit = run(
        root.path(),
        &["--json", "edit", "anddress", encoded, "retry_budget = 5"],
    );
    assert_eq!(edit.status.code(), Some(0));
    assert!(edit.stderr.is_empty());
    assert_eq!(
        fs::read(root.path().join("note.txt")).unwrap(),
        b"retry_budget = 5\r\n"
    );
    let fresh_encoded = edit
        .stdout
        .strip_prefix(b"{\"schema\":\"bw.cli.edit.v1\",\"outcome\":\"changed\",\"anddress\":")
        .and_then(|value| value.strip_suffix(b"}\n"))
        .expect("exact changed Edit envelope");
    let fresh = Anddress::decode(fresh_encoded).unwrap();
    let expected = support::address(
        input.workspace_coordinate(),
        "note.txt",
        b"retry_budget = 5\r\n",
        PublicAnddressTarget::Line,
        0,
        18,
    );
    assert_eq!(fresh, expected);
    assert_eq!(fresh_encoded, expected.encode().unwrap());

    let fresh_encoded = std::str::from_utf8(fresh_encoded).unwrap();
    let viewed = run(root.path(), &["view", "anddress", fresh_encoded]);
    assert_eq!(viewed.status.code(), Some(0));
    assert_eq!(viewed.stdout, b"retry_budget = 5\r\n");
    assert!(viewed.stderr.is_empty());

    let next = run(
        root.path(),
        &["edit", "anddress", fresh_encoded, "retry_budget = 7"],
    );
    let next_expected = support::address(
        input.workspace_coordinate(),
        "note.txt",
        b"retry_budget = 7\r\n",
        PublicAnddressTarget::Line,
        0,
        18,
    );
    assert_edit_output(next, "changed", Some(&next_expected), false);
    assert_eq!(
        fs::read(root.path().join("note.txt")).unwrap(),
        b"retry_budget = 7\r\n"
    );
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
        vec!["version", "extra"],
        vec!["update", "extra"],
        vec!["--json", "version"],
        vec!["--raw", "update"],
        vec!["pick"],
        vec!["view"],
        vec!["search", "line", "needle", "--json"],
        vec!["search", "line", "needle", "--raw"],
    ] {
        assert_usage(run(root.path(), &arguments));
    }
    assert_usage(run(
        root.path(),
        &["--workspace", root.path().to_str().unwrap(), "version"],
    ));
    assert_usage(run(root.path(), &["--admit", ".", "update"]));
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
