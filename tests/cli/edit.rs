use super::*;

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
fn one_shot_edit_stdin_matches_argv_content_and_preserves_exact_boundaries() {
    for (before, target, content, expected) in [
        ("old", AnddressTarget::File, "한글\nnext", "한글\nnext"),
        (
            "first\nline\n\nkeep\n",
            AnddressTarget::Paragraph {
                ordinal: Natural::zero(),
            },
            "new\nparagraph\n",
            "new\nparagraph\n\nkeep\n",
        ),
        (
            "old",
            AnddressTarget::Line {
                ordinal: Natural::zero(),
                exact_extent: "old".to_owned(),
            },
            "",
            "",
        ),
        (
            "old\n",
            AnddressTarget::Line {
                ordinal: Natural::zero(),
                exact_extent: "old\n".to_owned(),
            },
            "replace",
            "replace\n",
        ),
        (
            "old\r",
            AnddressTarget::Line {
                ordinal: Natural::zero(),
                exact_extent: "old\r".to_owned(),
            },
            "replace",
            "replace\r",
        ),
        (
            "old\r\n",
            AnddressTarget::Line {
                ordinal: Natural::zero(),
                exact_extent: "old\r\n".to_owned(),
            },
            "replace",
            "replace\r\n",
        ),
    ] {
        let root = tempfile::tempdir().unwrap();
        write(root.path(), "coordinate.txt", "coordinate\n");
        write(root.path(), "note.txt", before);
        let argv_operand = view_operand(root.path(), "note.txt", target.clone());
        let argv = run(root.path(), &["edit", "anddress", &argv_operand, content]);
        assert_eq!(
            fs::read(root.path().join("note.txt")).unwrap(),
            expected.as_bytes()
        );
        write(root.path(), "note.txt", before);
        let stdin_operand = view_operand(root.path(), "note.txt", target);
        let stdin = run_with_stdin(
            root.path(),
            &["edit", "anddress", &stdin_operand, "--stdin"],
            content.as_bytes(),
        );
        assert!(argv.status.success());
        assert_eq!(stdin.status.code(), Some(0));
        assert_eq!(argv.stdout, stdin.stdout);
        assert!(stdin.stderr.is_empty());
        assert_eq!(
            fs::read(root.path().join("note.txt")).unwrap(),
            expected.as_bytes()
        );
    }
}

#[test]
fn one_shot_edit_stdin_rejects_invalid_content_without_publication() {
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

    #[cfg(unix)]
    let inode = fs::metadata(root.path().join("note.txt")).unwrap().ino();
    for (input, code, cause) in [
        (
            b"bad\0body".as_slice(),
            "edit.content_contains_nul",
            EDIT_CONTENT_NUL_CAUSE,
        ),
        (
            b"bad\nbody",
            "edit.line_body_contains_terminator",
            EDIT_LINE_BODY_CAUSE,
        ),
        (
            b"bad\rbody",
            "edit.line_body_contains_terminator",
            EDIT_LINE_BODY_CAUSE,
        ),
        (
            b"bad\r\nbody",
            "edit.line_body_contains_terminator",
            EDIT_LINE_BODY_CAUSE,
        ),
    ] {
        let output = run_with_stdin(
            root.path(),
            &["edit", "anddress", &operand, "--stdin"],
            input,
        );
        assert_actionable_usage(output, code, cause, EDIT_HELP_KAT, "bw help edit");
        assert_eq!(fs::read(root.path().join("note.txt")).unwrap(), b"old\r\n");
        #[cfg(unix)]
        assert_eq!(
            fs::metadata(root.path().join("note.txt")).unwrap().ino(),
            inode
        );
    }

    let output = run_with_stdin(
        root.path(),
        &["edit", "anddress", &operand, "--stdin"],
        b"\xff",
    );
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(fs::read(root.path().join("note.txt")).unwrap(), b"old\r\n");

    for (before, target) in [
        ("old\r\n", AnddressTarget::File),
        (
            "first\r\nline\r\n\r\nkeep\r\n",
            AnddressTarget::Paragraph {
                ordinal: Natural::zero(),
            },
        ),
    ] {
        write(root.path(), "note.txt", before);
        let operand = view_operand(root.path(), "note.txt", target);
        #[cfg(unix)]
        let inode = fs::metadata(root.path().join("note.txt")).unwrap().ino();
        assert_actionable_usage(
            run_with_stdin(
                root.path(),
                &["edit", "anddress", &operand, "--stdin"],
                b"bad\0content",
            ),
            "edit.content_contains_nul",
            EDIT_CONTENT_NUL_CAUSE,
            EDIT_HELP_KAT,
            "bw help edit",
        );
        assert_eq!(
            fs::read(root.path().join("note.txt")).unwrap(),
            before.as_bytes()
        );
        #[cfg(unix)]
        assert_eq!(
            fs::metadata(root.path().join("note.txt")).unwrap().ino(),
            inode
        );
    }

    fs::remove_file(root.path().join("note.txt")).unwrap();
    assert_actionable_usage(
        run_with_stdin(
            root.path(),
            &["edit", "anddress", &operand, "--stdin"],
            b"still\ninvalid",
        ),
        "edit.line_body_contains_terminator",
        EDIT_LINE_BODY_CAUSE,
        EDIT_HELP_KAT,
        "bw help edit",
    );
    assert!(!root.path().join("note.txt").exists());

    assert_actionable_usage(
        run(
            root.path(),
            &["edit", "anddress", &operand, "--stdin", "extra"],
        ),
        "edit.extra_operand",
        "edit anddress accepts exactly one anddress and Content selector",
        EDIT_HELP_KAT,
        "bw help edit",
    );
}

#[test]
fn one_shot_edit_stdin_reads_content_beyond_multiple_reader_chunks() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    write(root.path(), "note.txt", "old\n");
    let operand = view_operand(root.path(), "note.txt", AnddressTarget::File);
    let content = format!("prefix-{}-suffix", "한".repeat(32_768));
    let output = run_with_stdin(
        root.path(),
        &["edit", "anddress", &operand, "--stdin"],
        content.as_bytes(),
    );
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert_eq!(
        fs::read(root.path().join("note.txt")).unwrap(),
        content.as_bytes()
    );
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

    #[cfg(unix)]
    let inode = fs::metadata(root.path().join("note.txt")).unwrap().ino();
    for content in ["bad\nbody", "bad\rbody", "bad\r\nbody"] {
        assert_actionable_usage(
            run(root.path(), &["edit", "anddress", &operand, content]),
            "edit.line_body_contains_terminator",
            EDIT_LINE_BODY_CAUSE,
            EDIT_HELP_KAT,
            "bw help edit",
        );
        assert_eq!(fs::read(root.path().join("note.txt")).unwrap(), b"old\r\n");
        #[cfg(unix)]
        assert_eq!(
            fs::metadata(root.path().join("note.txt")).unwrap().ino(),
            inode
        );
    }

    fs::remove_file(root.path().join("note.txt")).unwrap();
    assert_actionable_usage(
        run(
            root.path(),
            &["edit", "anddress", &operand, "still\ninvalid"],
        ),
        "edit.line_body_contains_terminator",
        EDIT_LINE_BODY_CAUSE,
        EDIT_HELP_KAT,
        "bw help edit",
    );
    assert!(!root.path().join("note.txt").exists());
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
fn one_shot_edit_exact_noop_uses_v5_geometry_and_shared_apply_without_view() {
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

    let source = include_str!("../../src/bin/bw.rs");
    let edit = source
        .split_once("fn execute_edit")
        .unwrap()
        .1
        .split_once("fn execute_view")
        .unwrap()
        .0;
    let decode = edit.find("decode_anddress_for_edit(encoded)").unwrap();
    let prepare = edit
        .find("prepare_replace_content(&anddress, content)")
        .unwrap();
    let construct = edit.find("Edit::Replace").unwrap();
    let validate = edit.find("edit.validate()").unwrap();
    let open = edit
        .find("open_runtime(workspace, admissions, Some(\"edit\"))")
        .unwrap();
    let apply = edit.find(".apply_replace(&edit)").unwrap();
    let write = edit.find("write_edit(receipt, output)").unwrap();
    assert!(decode < prepare);
    assert!(prepare < construct);
    assert!(construct < validate);
    assert!(validate < open);
    assert!(validate < apply);
    assert!(open < apply);
    assert!(apply < write);
    assert!(!edit.contains("run_view"));
    assert!(!edit.contains(".view("));
    assert!(!edit.contains("run_search"));
    assert!(!edit.contains("run_check"));
    assert!(!edit.contains("replace-exact"));
    assert!(!edit.contains("--exact"));
    assert_eq!(edit.matches("open_runtime").count(), 1);
    assert!(!edit.contains("write_session_status"));
    assert!(!edit.contains("output options must precede the capability"));
    assert_eq!(source.matches("fn execute_edit").count(), 1);
    let replace_content = source
        .split_once("fn prepare_replace_content")
        .unwrap()
        .1
        .split_once("fn decode_anddress_for_edit")
        .unwrap()
        .0;
    assert!(replace_content.contains("anddress.terminator()"));

    let raw_replace = include_str!("../../src/bin/bw/shell.rs")
        .split_once("fn parse_session_edit")
        .unwrap()
        .1
        .split_once("fn parse_session_position")
        .unwrap()
        .0;
    assert!(raw_replace.contains("Edit::Replace"));
    assert!(!raw_replace.contains("apply_replace"));

    let writer = include_str!("../../src/bin/bw/output.rs")
        .split_once("fn write_edit")
        .unwrap()
        .1
        .split_once("fn write_search")
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

    let core_edit = include_str!("../../src/backwriter/edit.rs");
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
