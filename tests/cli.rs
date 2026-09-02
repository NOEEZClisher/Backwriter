mod support;

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
        anddress::{Anddress, AnddressTarget as PublicAnddressTarget, LineTerminator},
        check::CheckOutcome,
        search::{
            SearchOccurrence, SearchOutcome, SearchPosition, SearchQuery, SearchRequest,
            SearchScope, SearchTarget,
        },
        view::ViewOutcome,
    },
    runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime},
};
use serde_json::Value;

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
    let SearchOutcome::Found { occurrences } = workspace.search(&request).unwrap() else {
        panic!("coordinate source");
    };
    let coordinate = occurrences[0].anddress().workspace_coordinate();
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
    assert!(text(output.stderr).contains("Usage:"));
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

fn expected_check_json(outcome: &CheckOutcome<Option<Anddress>>) -> Vec<u8> {
    let mut output = b"{\"schema\":\"bw.cli.check.v1\",\"status\":\"".to_vec();
    output.extend_from_slice(raw_check_status(outcome).as_bytes());
    output.extend_from_slice(b"\",\"filtered\":");
    if let Some(filtered) = &outcome.filtered {
        output.extend_from_slice(&filtered.encode().unwrap());
    } else {
        output.extend_from_slice(b"null");
    }
    output.extend_from_slice(b"}\n");
    output
}

fn assert_check_json(output: Output, expected: &CheckOutcome<Option<Anddress>>, input: &Anddress) {
    assert!(output.status.success());
    assert_eq!(output.stdout, expected_check_json(expected));
    assert!(output.stderr.is_empty());

    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(document["schema"], "bw.cli.check.v1");
    assert_eq!(document["status"], raw_check_status(expected));
    match &expected.filtered {
        Some(filtered) => {
            let encoded = serde_json::to_vec(&document["filtered"]).unwrap();
            assert_eq!(Anddress::decode(&encoded).unwrap(), *filtered);
            assert_eq!(filtered, input);
        }
        None => assert!(document["filtered"].is_null()),
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
        SearchOutcome::Found { occurrences } => {
            output.extend_from_slice(b"found\",\"occurrences\":[");
            for (index, occurrence) in occurrences.iter().enumerate() {
                if index != 0 {
                    output.push(b',');
                }
                let anddress = occurrence.anddress();
                output.extend_from_slice(b"{\"logicalPath\":");
                serde_json::to_writer(&mut output, anddress.logical_path()).unwrap();
                match occurrence.position() {
                    None => output.extend_from_slice(b",\"kind\":\"file\""),
                    Some(SearchPosition::Line { line }) => {
                        write!(output, ",\"kind\":\"line\",\"line\":\"{line}\"").unwrap();
                    }
                    Some(SearchPosition::Paragraph {
                        start_line,
                        end_line,
                    }) => write!(
                        output,
                        ",\"kind\":\"paragraph\",\"lineStart\":\"{start_line}\",\"lineEnd\":\"{end_line}\""
                    )
                    .unwrap(),
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
        SearchOutcome::Empty => &[] as &[SearchOccurrence],
        SearchOutcome::Found { occurrences } => occurrences,
    };
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        let anddress = expected.anddress();
        assert_eq!(actual["logicalPath"], anddress.logical_path());
        match expected.position() {
            None => {
                assert_eq!(actual["kind"], "file");
                assert!(actual.get("line").is_none());
                assert!(actual.get("lineStart").is_none());
                assert!(actual.get("lineEnd").is_none());
            }
            Some(SearchPosition::Line { line }) => {
                assert_eq!(actual["kind"], "line");
                assert_eq!(actual["line"], line.to_string());
            }
            Some(SearchPosition::Paragraph {
                start_line,
                end_line,
            }) => {
                assert_eq!(actual["kind"], "paragraph");
                assert_eq!(actual["lineStart"], start_line.to_string());
                assert_eq!(actual["lineEnd"], end_line.to_string());
            }
        }
        let encoded = serde_json::to_vec(&actual["anddress"]).unwrap();
        assert_eq!(Anddress::decode(&encoded).unwrap(), *anddress);
    }
}

fn expected_view_json(outcome: &ViewOutcome) -> Vec<u8> {
    let mut output = b"{\"schema\":\"bw.cli.view.v1\",\"kind\":".to_vec();
    match outcome {
        ViewOutcome::File { text, .. } => {
            output.extend_from_slice(b"\"file\",\"text\":");
            serde_json::to_writer(&mut output, text).unwrap();
        }
        ViewOutcome::Paragraph { text, file, .. } => {
            output.extend_from_slice(b"\"paragraph\",\"text\":");
            serde_json::to_writer(&mut output, text).unwrap();
            output.extend_from_slice(b",\"file\":");
            output.extend_from_slice(&file.encode().unwrap());
        }
        ViewOutcome::Line {
            content,
            terminator,
            file,
            paragraph,
            ..
        } => {
            output.extend_from_slice(b"\"line\",\"content\":");
            serde_json::to_writer(&mut output, content).unwrap();
            output.extend_from_slice(match terminator {
                LineTerminator::None => b",\"terminator\":\"none\",\"file\":",
                LineTerminator::Lf => b",\"terminator\":\"lf\",\"file\":",
                LineTerminator::Cr => b",\"terminator\":\"cr\",\"file\":",
                LineTerminator::Crlf => b",\"terminator\":\"crlf\",\"file\":",
            });
            output.extend_from_slice(&file.encode().unwrap());
            output.extend_from_slice(b",\"paragraph\":");
            if let Some(paragraph) = paragraph {
                output.extend_from_slice(&paragraph.encode().unwrap());
            } else {
                output.extend_from_slice(b"null");
            }
        }
        ViewOutcome::RelationAbsent => panic!("CLI self-View relation exists"),
    }
    output.extend_from_slice(b"}\n");
    output
}

fn expected_human_view(outcome: &ViewOutcome) -> Vec<u8> {
    match outcome {
        ViewOutcome::File { text, .. } | ViewOutcome::Paragraph { text, .. } => {
            text.as_bytes().to_vec()
        }
        ViewOutcome::Line {
            content,
            terminator,
            ..
        } => {
            let mut output = content.as_bytes().to_vec();
            output.extend_from_slice(match terminator {
                LineTerminator::None => b"",
                LineTerminator::Lf => b"\n",
                LineTerminator::Cr => b"\r",
                LineTerminator::Crlf => b"\r\n",
            });
            output
        }
        ViewOutcome::RelationAbsent => panic!("CLI self-View relation exists"),
    }
}

fn assert_view_json(output: Output, expected: &ViewOutcome) {
    assert!(output.status.success());
    assert_eq!(output.stdout, expected_view_json(expected));
    assert!(output.stderr.is_empty());

    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(document["schema"], "bw.cli.view.v1");
    match expected {
        ViewOutcome::File { text, .. } => {
            assert_eq!(document["kind"], "file");
            assert_eq!(document["text"], text.as_str());
        }
        ViewOutcome::Paragraph { text, file, .. } => {
            assert_eq!(document["kind"], "paragraph");
            assert_eq!(document["text"], text.as_str());
            let encoded = serde_json::to_vec(&document["file"]).unwrap();
            assert_eq!(Anddress::decode(&encoded).unwrap(), *file);
        }
        ViewOutcome::Line {
            content,
            terminator,
            file,
            paragraph,
            ..
        } => {
            assert_eq!(document["kind"], "line");
            assert_eq!(document["content"], content.as_str());
            assert_eq!(
                document["terminator"],
                match terminator {
                    LineTerminator::None => "none",
                    LineTerminator::Lf => "lf",
                    LineTerminator::Cr => "cr",
                    LineTerminator::Crlf => "crlf",
                }
            );
            let encoded = serde_json::to_vec(&document["file"]).unwrap();
            assert_eq!(Anddress::decode(&encoded).unwrap(), *file);
            match paragraph {
                Some(paragraph) => {
                    let encoded = serde_json::to_vec(&document["paragraph"]).unwrap();
                    assert_eq!(Anddress::decode(&encoded).unwrap(), *paragraph);
                }
                None => assert!(document["paragraph"].is_null()),
            }
        }
        ViewOutcome::RelationAbsent => panic!("CLI self-View relation exists"),
    }
}

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
    let help_stdout = text(help.stdout);
    assert!(help_stdout.starts_with("Usage:\n  bw "));
    assert!(help_stdout.contains("[--json] search"));
    assert!(help_stdout.contains("[--json] check"));
    assert!(help_stdout.contains("[--json|--raw] view"));
    assert!(help_stdout.contains("edit anddress <encoded-v4-Anddress> <content>"));
    assert!(help_stdout.contains("  bw version\n"));
    assert!(help_stdout.contains("  bw update\n"));
    assert!(help.stderr.is_empty());

    let version = run(root.path(), &["version"]);
    assert!(version.status.success());
    assert_eq!(version.stdout, b"Backwriter 0.2.2\n");
    assert!(version.stderr.is_empty());
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
fn one_shot_search_json_streams_exact_v4_objects_for_every_target() {
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
        assert_eq!(anddress["version"], "artext.backwriter-anddress.v4");
        assert!(anddress.get("sourceStateHash").is_some());
        assert!(anddress.get("sourceByteLength").is_some());
        assert!(anddress.get("byteStart").is_some());
        assert!(anddress.get("byteEnd").is_some());
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
    let source = include_str!("../src/bin/bw.rs");
    let writer = source
        .split("fn write_search_json")
        .nth(1)
        .unwrap()
        .split("fn write_pick")
        .next()
        .unwrap();
    assert!(writer.contains("for (index, occurrence) in occurrences.iter().enumerate()"));
    assert!(writer.contains("anddress\n                    .encode()"));
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
fn one_shot_view_json_streams_exact_v4_objects_and_preserves_human_output() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    write(
        root.path(),
        "note.txt",
        "quote \" and slash \\ control \u{1}\n\nparagraph λ\r\nline cr\rline lf\n \t\nnone",
    );

    for target in [
        AnddressTarget::File,
        AnddressTarget::Paragraph {
            ordinal: Natural::one(),
        },
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "quote \" and slash \\ control \u{1}\n".to_owned(),
        },
        AnddressTarget::Line {
            ordinal: Natural::parse("2").unwrap(),
            exact_extent: "paragraph λ\r\n".to_owned(),
        },
        AnddressTarget::Line {
            ordinal: Natural::parse("3").unwrap(),
            exact_extent: "line cr\r".to_owned(),
        },
        AnddressTarget::Line {
            ordinal: Natural::parse("5").unwrap(),
            exact_extent: " \t\n".to_owned(),
        },
        AnddressTarget::Line {
            ordinal: Natural::parse("6").unwrap(),
            exact_extent: "none".to_owned(),
        },
    ] {
        let operand = view_operand(root.path(), "note.txt", target);
        let workspace = WorkspaceRuntime::open(
            root.path(),
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap();
        let input = Anddress::decode(operand.as_bytes()).unwrap();
        let expected = workspace.view(&input, input.target()).unwrap();

        assert_view_json(
            run(root.path(), &["--json", "view", "anddress", &operand]),
            &expected,
        );
        let human = run(root.path(), &["view", "anddress", &operand]);
        assert!(human.status.success());
        assert_eq!(human.stdout, expected_human_view(&expected));
        assert!(human.stderr.is_empty());
        let raw = run(root.path(), &["--raw", "view", "anddress", &operand]);
        assert!(raw.status.success());
        assert_eq!(raw.stdout, expected_human_view(&expected));
        assert!(raw.stderr.is_empty());
    }

    let escaped = run(
        root.path(),
        &[
            "--json",
            "view",
            "anddress",
            &view_operand(
                root.path(),
                "note.txt",
                AnddressTarget::Line {
                    ordinal: Natural::zero(),
                    exact_extent: "quote \" and slash \\ control \u{1}\n".to_owned(),
                },
            ),
        ],
    );
    assert!(
        escaped
            .stdout
            .windows(b"\\u0001".len())
            .any(|window| window == b"\\u0001")
    );
    assert!(
        escaped
            .stdout
            .windows(b"\\\"".len())
            .any(|window| window == b"\\\"")
    );
    assert!(
        escaped
            .stdout
            .windows(b"\\\\".len())
            .any(|window| window == b"\\\\")
    );
}

#[test]
fn one_shot_view_json_rejects_invalid_forms_and_keeps_errors_off_stdout() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    write(root.path(), "note.txt", "actual\n");
    let operand = view_operand(root.path(), "note.txt", AnddressTarget::File);

    assert_usage(run(
        root.path(),
        &["--json", "--json", "view", "anddress", &operand],
    ));
    assert_usage(run(root.path(), &["view", "anddress", &operand, "--json"]));
    assert_usage(run(root.path(), &["--json", "view", "anchored", "handle"]));
    assert_usage(run(
        root.path(),
        &["--json", "view", "anddress", &operand, "extra"],
    ));
    assert_usage(run(root.path(), &["view", "anddress", &operand, "--raw"]));

    let stale = view_operand(
        root.path(),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "actual\n".to_owned(),
        },
    );
    write(root.path(), "note.txt", "changed\n");
    assert_execution_error(run(root.path(), &["--json", "view", "anddress", &stale]));
    assert_execution_error(run(root.path(), &["--raw", "view", "anddress", &stale]));

    write(root.path(), "unadmitted.txt", "unadmitted\n");
    let unadmitted = view_operand(root.path(), "unadmitted.txt", AnddressTarget::File);
    assert_execution_error(run(
        root.path(),
        &[
            "--admit",
            "coordinate.txt",
            "--json",
            "view",
            "anddress",
            &unadmitted,
        ],
    ));
    assert_execution_error(run(
        root.path(),
        &[
            "--raw",
            "--admit",
            "coordinate.txt",
            "view",
            "anddress",
            &unadmitted,
        ],
    ));
}

#[test]
fn one_shot_view_json_writer_has_no_value_clone_or_collection_path() {
    let source = include_str!("../src/bin/bw.rs");
    let writer = source
        .split("fn write_view_json")
        .nth(1)
        .unwrap()
        .split("fn write_check")
        .next()
        .unwrap();
    assert!(writer.contains("serde_json::to_writer"));
    assert!(!writer.contains("Value"));
    assert!(!writer.contains(".clone()"));
    assert!(!writer.contains("collect("));
    assert!(!writer.contains("Vec<ViewOutcome>"));
}

#[test]
fn one_shot_raw_view_accepts_global_order_and_rejects_every_other_output_form() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    write(root.path(), "note.txt", "raw\r\n");
    let operand = view_operand(root.path(), "note.txt", AnddressTarget::File);

    let ordered = run(
        root.path(),
        &[
            "--workspace",
            root.path().to_str().unwrap(),
            "--raw",
            "--admit",
            ".",
            "view",
            "anddress",
            &operand,
        ],
    );
    assert!(ordered.status.success());
    assert_eq!(ordered.stdout, b"raw\r\n");
    assert!(ordered.stderr.is_empty());

    for arguments in [
        vec!["--raw", "--raw", "view", "anddress", &operand],
        vec!["--json", "--raw", "view", "anddress", &operand],
        vec!["--raw", "--json", "view", "anddress", &operand],
        vec!["--raw", "search", "line", "raw"],
        vec!["--raw", "check", "anddress", &operand],
        vec!["--raw", "shell"],
        vec!["--raw", "data"],
        vec!["--raw", "pick"],
        vec!["--raw", "view", "anchored", "handle"],
    ] {
        assert_usage(run(root.path(), &arguments));
    }

    let source = include_str!("../src/bin/bw.rs");
    assert!(source.contains("enum OutputMode"));
    assert!(!source.contains("let mut json"));
    assert!(!source.contains("write_view_raw"));
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
    let raw_line_output = run(root.path(), &["--raw", "view", "anddress", &line]);
    assert!(raw_line_output.status.success());
    assert_eq!(raw_line_output.stdout, large_line.as_bytes());
    assert!(raw_line_output.stderr.is_empty());
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
fn one_shot_edit_replaces_file_and_paragraph_with_exact_content() {
    for (before, kind, content, expected, paragraph_receipt) in [
        ("old\n", "file", "", "", false),
        (
            "old\n",
            "file",
            "한글\r\nsecond\n",
            "한글\r\nsecond\n",
            false,
        ),
        ("old\n", "file", "--json", "--json", false),
        ("old\n", "file", "--raw", "--raw", false),
        ("old\n", "file", "--stdin", "--stdin", false),
        ("first\nline\n\nkeep\n", "paragraph", "", "\nkeep\n", false),
        (
            "first\nline\n\nkeep\n",
            "paragraph",
            "새 문단\r\n둘\n",
            "새 문단\r\n둘\n\nkeep\n",
            true,
        ),
        (
            "first\nline\n\nkeep\n",
            "paragraph",
            "--json",
            "--json\nkeep\n",
            false,
        ),
        (
            "first\nline\n\nkeep\n",
            "paragraph",
            "--raw",
            "--raw\nkeep\n",
            false,
        ),
        (
            "first\nline\n\nkeep\n",
            "paragraph",
            "--stdin",
            "--stdin\nkeep\n",
            false,
        ),
        (
            "first\nline\n\nkeep\n",
            "paragraph",
            "left\n\nright\n",
            "left\n\nright\n\nkeep\n",
            false,
        ),
    ] {
        for json in [false, true] {
            let workspace = tempfile::tempdir().unwrap();
            let caller = tempfile::tempdir().unwrap();
            write(workspace.path(), "admitted/coordinate.txt", "coordinate\n");
            write(workspace.path(), "admitted/note.txt", before);
            let target = if kind == "file" {
                AnddressTarget::File
            } else {
                AnddressTarget::Paragraph {
                    ordinal: Natural::zero(),
                }
            };
            let operand = view_operand(workspace.path(), "admitted/note.txt", target);
            let input = Anddress::decode(operand.as_bytes()).unwrap();
            let mut arguments = vec![
                "--admit",
                "admitted",
                "--workspace",
                workspace.path().to_str().unwrap(),
            ];
            if json {
                arguments.push("--json");
            }
            arguments.extend(["edit", "anddress", &operand, content]);
            let output = run(caller.path(), &arguments);
            let expected_anddress = if kind == "file" {
                Some(support::file(
                    input.workspace_coordinate(),
                    "admitted/note.txt",
                    expected.as_bytes(),
                ))
            } else if paragraph_receipt {
                Some(support::paragraph(
                    input.workspace_coordinate(),
                    "admitted/note.txt",
                    expected.as_bytes(),
                    0,
                ))
            } else {
                None
            };
            assert_edit_output(output, "changed", expected_anddress.as_ref(), json);
            assert_eq!(
                fs::read(workspace.path().join("admitted/note.txt")).unwrap(),
                expected.as_bytes()
            );
        }
    }
}

#[test]
fn one_shot_edit_replaces_line_body_and_preserves_every_terminator() {
    for (before, content, expected) in [
        ("old", "한글", "한글"),
        ("old\n", "", "\n"),
        ("old\r", "β", "β\r"),
        ("old\r\n", "새 줄", "새 줄\r\n"),
        ("old\n", "--json", "--json\n"),
        ("old\r\n", "--raw", "--raw\r\n"),
        ("old", "--stdin", "--stdin"),
    ] {
        for json in [false, true] {
            let root = tempfile::tempdir().unwrap();
            write(root.path(), "coordinate.txt", "coordinate\n");
            write(root.path(), "note.txt", before);
            let operand = view_operand(
                root.path(),
                "note.txt",
                AnddressTarget::Line {
                    ordinal: Natural::zero(),
                    exact_extent: before.to_owned(),
                },
            );
            let input = Anddress::decode(operand.as_bytes()).unwrap();
            let arguments = if json {
                vec!["--json", "edit", "anddress", &operand, content]
            } else {
                vec!["edit", "anddress", &operand, content]
            };
            let output = run(root.path(), &arguments);
            let expected_anddress = support::line(
                input.workspace_coordinate(),
                "note.txt",
                expected.as_bytes(),
                0,
            );
            assert_edit_output(output, "changed", Some(&expected_anddress), json);
            assert_eq!(
                fs::read(root.path().join("note.txt")).unwrap(),
                expected.as_bytes()
            );
        }
    }
}

#[test]
fn one_shot_edit_rejects_line_break_content_without_touching_source() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    write(root.path(), "note.txt", "old\r\n");
    let operand = view_operand(
        root.path(),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "old\r\n".to_owned(),
        },
    );

    for content in ["bad\nbody", "bad\rbody", "bad\r\nbody"] {
        let output = run(root.path(), &["edit", "anddress", &operand, content]);
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(text(output.stderr).contains("Edit input is invalid"));
        assert_eq!(fs::read(root.path().join("note.txt")).unwrap(), b"old\r\n");
    }
}

#[test]
fn one_shot_edit_rejects_invalid_forms_flags_and_addresses_before_publication() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    write(root.path(), "note.txt", "old\n");
    let operand = view_operand(root.path(), "note.txt", AnddressTarget::File);
    let noncanonical = operand.replacen(
        "\"sourceByteLength\":\"4\"",
        "\"sourceByteLength\":\"04\"",
        1,
    );
    assert_ne!(noncanonical, operand);
    let v3 = r#"{"version":"artext.backwriter-anddress.v3","workspaceCoordinate":"x","logicalPath":"note.txt","kind":"file"}"#;

    for arguments in [
        vec!["edit"],
        vec!["edit", "wrong"],
        vec!["edit", "anddress"],
        vec!["edit", "anddress", &operand],
        vec!["edit", "anddress", &operand, "new", "extra"],
        vec!["edit", "anddress", "{", "new"],
        vec!["edit", "anddress", v3, "new"],
        vec!["edit", "anddress", &noncanonical, "new"],
        vec!["--json", "--json", "edit", "anddress", &operand, "new"],
        vec!["--raw", "edit", "anddress", &operand, "new"],
        vec!["edit", "anddress", &operand, "new", "--json"],
        vec!["edit", "anddress", &operand, "new", "--raw"],
        vec!["edit", "anddress", &operand, "new", "--stdin"],
    ] {
        assert_usage(run(root.path(), &arguments));
        assert_eq!(fs::read(root.path().join("note.txt")).unwrap(), b"old\n");
    }
}

#[test]
fn one_shot_edit_maps_stale_missing_and_unadmitted_sources_to_execution_failure() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    write(root.path(), "stale.txt", "old\n");
    let stale = view_operand(root.path(), "stale.txt", AnddressTarget::File);
    write(root.path(), "stale.txt", "external\n");
    assert_unavailable(run(
        root.path(),
        &["edit", "anddress", &stale, "replacement"],
    ));
    assert_eq!(
        fs::read(root.path().join("stale.txt")).unwrap(),
        b"external\n"
    );

    let missing = view_operand(root.path(), "missing.txt", AnddressTarget::File);
    assert_unavailable(run(
        root.path(),
        &["edit", "anddress", &missing, "replacement"],
    ));
    assert!(!root.path().join("missing.txt").exists());

    write(root.path(), "other.txt", "other\n");
    let unadmitted = view_operand(root.path(), "other.txt", AnddressTarget::File);
    assert_unavailable(run(
        root.path(),
        &[
            "--admit",
            "coordinate.txt",
            "edit",
            "anddress",
            &unadmitted,
            "replacement",
        ],
    ));
    assert_eq!(fs::read(root.path().join("other.txt")).unwrap(), b"other\n");
}

#[test]
fn one_shot_edit_exact_noop_preserves_bytes_and_inode_and_reuses_existing_seams() {
    for json in [false, true] {
        let root = tempfile::tempdir().unwrap();
        write(root.path(), "coordinate.txt", "coordinate\n");
        write(root.path(), "note.txt", "same\r\n");
        let operand = view_operand(root.path(), "note.txt", AnddressTarget::File);
        let input = Anddress::decode(operand.as_bytes()).unwrap();
        #[cfg(unix)]
        let inode = fs::metadata(root.path().join("note.txt")).unwrap().ino();

        let arguments = if json {
            vec!["--json", "edit", "anddress", &operand, "same\r\n"]
        } else {
            vec!["edit", "anddress", &operand, "same\r\n"]
        };
        let output = run(root.path(), &arguments);
        assert_edit_output(output, "unchanged", Some(&input), json);
        assert_eq!(fs::read(root.path().join("note.txt")).unwrap(), b"same\r\n");
        #[cfg(unix)]
        assert_eq!(
            fs::metadata(root.path().join("note.txt")).unwrap().ino(),
            inode
        );
    }

    let source = include_str!("../src/bin/bw.rs");
    let edit = source
        .split_once("fn execute_edit")
        .unwrap()
        .1
        .split_once("fn execute_view")
        .unwrap()
        .0;
    let decode = edit.find("decode_anddress(encoded)").unwrap();
    let open = edit.find("open_runtime(workspace, admissions)").unwrap();
    let view = edit.find("run_view(&runtime, &anddress)").unwrap();
    let construct = edit.find("Edit::Replace").unwrap();
    let validate = edit.find("edit.validate()").unwrap();
    let apply = edit.find(".apply_replace(&edit)").unwrap();
    let write = edit.find("write_edit(receipt, output)").unwrap();
    assert!(decode < open);
    assert!(open < view);
    assert!(view < construct);
    assert!(construct < validate);
    assert!(validate < apply);
    assert!(apply < write);
    assert!(!edit.contains("run_search"));
    assert!(!edit.contains("run_check"));
    assert_eq!(edit.matches("open_runtime").count(), 1);
    assert!(!edit.contains("write_session_status"));
    assert!(!edit.contains("stdin"));
    assert!(!edit.contains("output options must precede the capability"));
    assert_eq!(source.matches("fn execute_edit").count(), 1);

    let writer = source
        .split_once("fn write_edit")
        .unwrap()
        .1
        .split_once("fn execute_view")
        .unwrap()
        .0;
    assert_eq!(writer.matches(".encode").count(), 1);
    assert!(writer.find(".encode").unwrap() < writer.find("BufWriter::new").unwrap());
    assert!(!writer.contains("serde_json"));
    assert!(!writer.contains("Value"));
    assert!(!writer.contains("collect("));
    assert!(!writer.contains(".clone()"));
    assert!(!writer.contains("stdin"));
    assert!(!writer.contains("thread::"));
    assert!(!writer.contains("spawn("));
    assert_eq!(writer.matches("bw.cli.edit.v1").count(), 1);

    let core_edit = include_str!("../src/backwriter/edit.rs");
    assert!(core_edit.contains("!content.contains('\\0')"));
}

#[cfg(target_os = "linux")]
#[test]
fn one_shot_edit_output_failure_reports_exit_one_after_publication_without_retry() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    write(root.path(), "note.txt", "before\n");
    let operand = view_operand(root.path(), "note.txt", AnddressTarget::File);
    let full = fs::OpenOptions::new()
        .write(true)
        .open("/dev/full")
        .unwrap();

    let output = Command::new(binary())
        .current_dir(root.path())
        .args(["edit", "anddress", &operand, "after\n"])
        .stdout(Stdio::from(full))
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(text(output.stderr).starts_with("error: "));
    assert_eq!(fs::read(root.path().join("note.txt")).unwrap(), b"after\n");
}

#[test]
fn one_shot_check_json_preserves_raw_status_and_filtered_v4_values() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    write(root.path(), "note.txt", "file\n\nparagraph\nline\n");

    let cases = [
        view_operand(root.path(), "note.txt", AnddressTarget::File),
        view_operand(
            root.path(),
            "note.txt",
            AnddressTarget::Paragraph {
                ordinal: Natural::one(),
            },
        ),
        view_operand(
            root.path(),
            "note.txt",
            AnddressTarget::Line {
                ordinal: Natural::parse("3").unwrap(),
                exact_extent: "line\n".to_owned(),
            },
        ),
        view_operand(root.path(), "missing.txt", AnddressTarget::File),
        view_operand(root.path(), "broken.txt", AnddressTarget::File),
    ];
    write(root.path(), "broken.txt", "broken\0");

    for operand in cases {
        let input = Anddress::decode(operand.as_bytes()).unwrap();
        let workspace = WorkspaceRuntime::open(
            root.path(),
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap();
        let expected = workspace.check(input.clone()).unwrap();

        assert_check_json(
            run(root.path(), &["--json", "check", "anddress", &operand]),
            &expected,
            &input,
        );
        let human_status = match raw_check_status(&expected) {
            "current" => "Current",
            "not-current" => "NotCurrent",
            "unavailable" => "Unavailable",
            _ => unreachable!(),
        };
        assert_check_status(
            run(root.path(), &["check", "anddress", &operand]),
            human_status,
        );
    }
}

#[test]
fn one_shot_check_json_rejects_invalid_forms_and_keeps_fail_closed_writer() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    let operand = view_operand(root.path(), "coordinate.txt", AnddressTarget::File);

    assert_usage(run(
        root.path(),
        &["--json", "--json", "check", "anddress", &operand],
    ));
    assert_usage(run(root.path(), &["check", "anddress", &operand, "--json"]));
    assert_usage(run(root.path(), &["--json", "check", "search", "value"]));
    assert_usage(run(root.path(), &["--json", "check", "pick", "value"]));
    assert_usage(run(
        root.path(),
        &["--json", "check", "anddress", &operand, "extra"],
    ));
    assert_usage(run(root.path(), &["--json", "check", "anddress", "{"]));

    let source = include_str!("../src/bin/bw.rs");
    let status = source
        .split("fn raw_check_status")
        .nth(1)
        .unwrap()
        .split("fn write_check")
        .next()
        .unwrap();
    assert!(status.contains("inconsistent raw Check report"));
    let writer = source
        .split("fn write_check_json")
        .nth(1)
        .unwrap()
        .split("enum SessionValue")
        .next()
        .unwrap();
    assert!(writer.contains("raw_check_status(outcome)?"));
    assert!(!writer.contains("Value"));
    assert!(!writer.contains(".clone()"));
    assert!(!writer.contains("collect("));
    assert!(!writer.contains("Vec<CheckOutcome"));
    assert!(!source.contains("write_check(outcome.clone())"));
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
        b"Found 2\n0\tLine\tnote.txt:1\n1\tLine\tnote.txt:2\nneedle\nCurrent\n"
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
        b"Found 1\n0\tFile\tnote.txt\nSelected 1\n0\tFile\tnote.txt\nFound 1\n0\tParagraph\tnote.txt:1-1\nSelected 1\n0\tParagraph\tnote.txt:0-6\nFound 1\n0\tLine\tnote.txt:1\nSelected 1\n0\tLine\tnote.txt:0-6\n"
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
        b"Found 3\n0\tLine\ta.txt:1\n1\tLine\ta.txt:2\n2\tLine\tb.txt:1\nSelected 2\n0\tLine\ta.txt:0-7\n1\tLine\ta.txt:7-14\nSelected 2\n0\tLine\ta.txt:0-7\n1\tLine\tb.txt:0-7\nSelected 2\n0\tLine\ta.txt:0-7\n1\tLine\ta.txt:7-14\n"
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
        b"Found 3\n0\tLine\tnote.txt:1\n1\tLine\tnote.txt:2\n2\tLine\tnote.txt:3\nSelected 2\n0\tLine\tnote.txt:0-7\n1\tLine\tnote.txt:14-21\nSelected 2\n0\tLine\tnote.txt:0-7\n1\tLine\tnote.txt:14-21\nneedle\nCurrent\n"
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
        b"Found 3\n0\tLine\tnote.txt:1\n1\tLine\tnote.txt:2\n2\tLine\tnote.txt:3\nSelected 3\n0\tLine\tnote.txt:0-7\n1\tLine\tnote.txt:7-14\n2\tLine\tnote.txt:14-21\n"
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
        b"Found 1\n0\tLine\tnote.txt:1\nFound 0\nSelected 0\nSelected 1\n0\tLine\tnote.txt:0-7\nFound 1\n0\tLine\tnote.txt:1\n"
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
        b"Found 3\n0\tLine\tcurrent.txt:1\n1\tLine\tremoved.txt:1\n2\tLine\tunavailable.txt:1\nChecked 3\nCurrent 1\nNotCurrent 1\nUnavailable 1\nSelected 3\n0\tLine\tcurrent.txt:0-7\n1\tLine\tremoved.txt:0-7\n2\tLine\tunavailable.txt:0-7\nChecked 3\nCurrent 1\nNotCurrent 1\nUnavailable 1\nSelected 3\n0\tLine\tcurrent.txt:0-7\n1\tLine\tremoved.txt:0-7\n2\tLine\tunavailable.txt:0-7\n"
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
        b"Found 1\n0\tLine\tnote.txt:1\nSelected 1\n0\tLine\tnote.txt:0-7\nCurrent\n"
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
        b"Found 2\n0\tLine\tleft.txt:1\n1\tLine\tright.txt:1\nAnchored\nAlreadyLive\nAnchored\nneedle\nOK\nneedle\n"
    );
    assert!(output.stderr.is_empty());

    let invalidated = run_shell(
        root.path(),
        "let hits = search line needle\nlet handle = anchor create @hits[0]\nanchor invalidate-source left.txt\nview anchored @handle\nsearch line needle\nexit\n",
    );
    assert_eq!(invalidated.status.code(), Some(1));
    assert_eq!(
        invalidated.stdout,
        b"Found 2\n0\tLine\tleft.txt:1\n1\tLine\tright.txt:1\nAnchored\nOK\nFound 2\n0\tLine\tleft.txt:1\n1\tLine\tright.txt:1\n"
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
fn session_edit_apply_builds_each_core_edit_and_every_position() {
    let cases = [
        (
            "one\n",
            "let lines = search line one\nlet edit = edit insert before @lines[0] \"zero\\n\"\napply @edit\nexit\n",
            "zero\none\n",
        ),
        (
            "one\n",
            "let lines = search line one\nlet edit = edit insert after @lines[0] \"zero\\n\"\napply @edit\nexit\n",
            "one\nzero\n",
        ),
        (
            "one\n",
            "let files = search file one\nlet edit = edit insert start-of @files[0] \"zero\\n\"\napply @edit\nexit\n",
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
fn session_exact_file_lookup_inserts_into_an_empty_file_end_to_end() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "empty.txt", "");

    let output = run_shell(
        root.path(),
        "let files = search /file empty.txt\nlet edit = edit insert end-of @files[0] hello\napply @edit\ncheck anddress @files[0]\nexit\n",
    );
    assert!(output.status.success(), "{}", text(output.stderr));
    assert_eq!(
        output.stdout,
        b"Found 1\n0\tFile\tempty.txt\nOK\nNotCurrent\n"
    );
    assert!(output.stderr.is_empty());
    assert_eq!(fs::read(root.path().join("empty.txt")).unwrap(), b"hello");
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
    assert!(stderr.contains("Edit input is invalid"));
    assert!(stderr.contains("binding is not an Edit: wrong"));
    assert!(stderr.contains("Edit bindings cannot be indexed"));
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
        b"Found 1\n0\tLine\tnote.txt:1\nneedle\nCurrent\nChecked 1\nCurrent 1\nNotCurrent 0\nUnavailable 0\nSelected 1\n0\tLine\tnote.txt:0-7\nChecked 1\nCurrent 1\nNotCurrent 0\nUnavailable 0\nAnchored\nneedle\nneedle\n"
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
    assert_eq!(stdout.matches("Anddress\tLine\tnote.txt:0-7\n").count(), 2);
    assert!(stdout.matches("Found 1\n0\tLine\tnote.txt:1\n").count() >= 3);
    assert!(
        stdout
            .matches("Selected 1\n0\tLine\tnote.txt:0-7\n")
            .count()
            >= 3
    );
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
        b"Found 1\n0\tLine\tnote.txt:1\nAnchored\nOK\nOK\nOK\nFound 1\n0\tLine\tnote.txt:1\nOK\nAnddress\tLine\tnote.txt:0-7\n"
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
        b"Found 1\n0\tLine\tnote.txt:1\nFound 0\nFound 1\n0\tLine\tnote.txt:1\n"
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
        b"Found 1\n0\tLine\tnote.txt:1\nFound 1\n0\tLine\tnote.txt:2\nquote: \" and slash \\\nFound 1\n0\tLine\tnote.txt:1\nFound 1\n0\tLine\tnote.txt:1\n"
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
    assert_eq!(eof.stdout, b"Found 1\n0\tLine\tnote.txt:1\n");
    assert!(eof.stderr.is_empty());
}

#[test]
fn session_lexer_decodes_all_quoted_escapes_outside_edit() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle\ndata store anddress \"slash\\\\ quote\\\" line\\ncarriage\\rtab\\t\" @hits[0]\ndata list\nexit\n",
    );
    assert!(output.status.success(), "{}", text(output.stderr));
    assert_eq!(
        output.stdout,
        b"Found 1\n0\tLine\tnote.txt:1\nOK\nanddress\t\"slash\\\\ quote\\\" line\\ncarriage\\rtab\\t\"\n"
    );
}

#[test]
fn session_apply_reuses_edit_binding_and_keeps_explicit_edit_clone() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "one\n");

    let output = run_shell(
        root.path(),
        "let lines = search line one\nlet edit = edit replace @lines[0] \"one\\n\"\nlet copy = @edit\napply @edit\napply @copy\nexit\n",
    );
    assert!(output.status.success(), "{}", text(output.stderr));
    assert_eq!(output.stdout, b"Found 1\n0\tLine\tnote.txt:1\nOK\nOK\n");
    assert_eq!(fs::read(root.path().join("note.txt")).unwrap(), b"one\n");

    let source = include_str!("../src/bin/bw.rs");
    assert!(source.contains("fn write_view(outcome: &ViewOutcome)"));
    assert!(source.contains("fn write_batch_check(report: &CheckReport)"));
    assert!(source.contains("Result<&'a Edit, CliError>"));
    assert!(!source.contains("write_view(outcome.clone())"));
    assert!(!source.contains("write_batch_check(outcome.report.clone())"));
    assert!(!source.contains("Some(SessionValue::Edit(edit)) => Ok(edit.clone())"));
    assert!(
        source.contains("Some(SessionValue::Edit(value)) => Ok(SessionValue::Edit(value.clone()))")
    );
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
    assert_eq!(execution_only.stdout, b"Found 1\n0\tLine\tnote.txt:1\n");
    assert!(text(execution_only.stderr).contains("workspace source is unavailable"));

    let no_latest = run_shell(root.path(), "search line needle\nview anddress @latest\n");
    assert_eq!(no_latest.status.code(), Some(2));
    assert_eq!(no_latest.stdout, b"Found 1\n0\tLine\tnote.txt:1\n");
    assert!(text(no_latest.stderr).contains("unknown binding: latest"));

    let execution_then_usage = run_shell(
        root.path(),
        "search line needle --source missing.txt\nunknown\nsearch line needle\n",
    );
    assert_eq!(execution_then_usage.status.code(), Some(2));
    assert_eq!(
        execution_then_usage.stdout,
        b"Found 1\n0\tLine\tnote.txt:1\n"
    );
    assert!(text(execution_then_usage.stderr).contains("unsupported Session command: unknown"));
}
