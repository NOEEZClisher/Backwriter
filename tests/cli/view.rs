use super::*;

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
fn one_shot_view_json_streams_exact_v5_objects_and_preserves_human_output() {
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
fn one_shot_view_projection_and_batch_share_the_v2_item_schema() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    let source = "one\n \t\nlast\r\n";
    write(root.path(), "note.txt", source);
    let line = view_operand(
        root.path(),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let separator = view_operand(
        root.path(),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: " \t\n".to_owned(),
        },
    );

    let human = run(
        root.path(),
        &["view", "anddress", &line, "--as", "paragraph"],
    );
    assert!(human.status.success());
    assert_eq!(human.stdout, b"one\n");
    assert!(human.stderr.is_empty());

    let inputs = [
        Anddress::decode(line.as_bytes()).unwrap(),
        Anddress::decode(separator.as_bytes()).unwrap(),
        Anddress::decode(line.as_bytes()).unwrap(),
    ];
    let runtime = WorkspaceRuntime::open(
        root.path(),
        WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
    )
    .unwrap();
    let expected = runtime
        .view_batch(&inputs, Some(PublicAnddressTarget::Paragraph))
        .unwrap();
    assert!(matches!(expected[1], ViewOutcome::RelationAbsent));
    assert_eq!(expected[0], expected[2]);

    let output = run(
        root.path(),
        &[
            "--json",
            "view",
            "anddress",
            &line,
            &separator,
            &line,
            "--as",
            "paragraph",
        ],
    );
    assert!(output.status.success());
    assert_eq!(output.stdout, expected_view_json_many(&expected));
    assert!(output.stderr.is_empty());

    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(document["schema"], "bw.cli.view.v2");
    let outcomes = document["outcomes"].as_array().unwrap();
    assert_eq!(outcomes.len(), 3);
    assert_eq!(outcomes[1]["outcome"], "relation-absent");
    for (actual, expected) in outcomes.iter().zip(&expected) {
        if let ViewOutcome::Projected { anddress, content } = expected {
            let encoded = serde_json::to_vec(&actual["anddress"]).unwrap();
            assert_eq!(Anddress::decode(&encoded).unwrap(), *anddress);
            assert_eq!(actual["content"], content.as_str());
        }
    }
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
    for arguments in [
        vec!["--json", "view", "anddress", &operand, "--as"],
        vec!["--json", "view", "anddress", &operand, "--as", "block"],
        vec![
            "--json", "view", "anddress", &operand, "--as", "file", "extra",
        ],
        vec!["view", "anddress", &operand, &operand, "--as", "file"],
        vec!["--json", "view", "anddress", &operand, &operand],
    ] {
        assert_usage(run(root.path(), &arguments));
    }
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
    let source = include_str!("../../src/bin/bw/output.rs");
    let writer = source
        .split_once("fn write_view_json(")
        .unwrap()
        .1
        .split("fn raw_check_status")
        .next()
        .unwrap();
    assert!(writer.contains("serde_json::to_writer"));
    assert_eq!(writer.matches("let mut encoded = Vec::new();").count(), 1);
    assert!(writer.contains(".encode_into(encoded)"));
    assert!(!writer.contains(".encode()"));
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

    let source = include_str!("../../src/bin/bw.rs");
    assert!(source.contains("enum OutputMode"));
    assert!(!source.contains("let mut json"));
    assert!(!source.contains("write_view_raw"));
    assert!(!include_str!("../../src/bin/bw/output.rs").contains("write_view_raw"));
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
