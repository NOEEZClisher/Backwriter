use super::*;

#[test]
fn one_shot_check_json_v2_preserves_ordered_v5_statuses() {
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
        let status = match raw_check_status(&expected) {
            "current" => CheckStatus::Current,
            "not-current" => CheckStatus::NotCurrent,
            "unavailable" => CheckStatus::Unavailable,
            _ => unreachable!(),
        };

        assert_check_json(
            run(root.path(), &["--json", "check", "anddress", &operand]),
            std::slice::from_ref(&input),
            std::slice::from_ref(&status),
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
fn one_shot_check_json_v2_preserves_mixed_order_duplicates_and_all_input_validation() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "coordinate.txt", "coordinate\n");
    write(root.path(), "current.txt", "current\n");
    write(root.path(), "stale.txt", "stale\n");
    write(root.path(), "broken.txt", "broken\n");
    let current =
        Anddress::decode(view_operand(root.path(), "current.txt", AnddressTarget::File).as_bytes())
            .unwrap();
    let stale =
        Anddress::decode(view_operand(root.path(), "stale.txt", AnddressTarget::File).as_bytes())
            .unwrap();
    let unavailable =
        Anddress::decode(view_operand(root.path(), "broken.txt", AnddressTarget::File).as_bytes())
            .unwrap();
    write(root.path(), "stale.txt", "changed\n");
    fs::write(root.path().join("broken.txt"), b"broken\0").unwrap();
    let inputs = vec![
        current.clone(),
        stale.clone(),
        unavailable.clone(),
        current.clone(),
    ];
    let operands: Vec<String> = inputs
        .iter()
        .map(|input| String::from_utf8(input.encode().unwrap()).unwrap())
        .collect();
    let workspace = WorkspaceRuntime::open(
        root.path(),
        WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
    )
    .unwrap();
    let statuses = workspace.check_batch(&inputs).unwrap();
    assert_eq!(
        statuses,
        vec![
            CheckStatus::Current,
            CheckStatus::NotCurrent,
            CheckStatus::Unavailable,
            CheckStatus::Current,
        ]
    );
    assert_check_json(
        run(
            root.path(),
            &[
                "--json",
                "check",
                "anddress",
                &operands[0],
                &operands[1],
                &operands[2],
                &operands[3],
            ],
        ),
        &inputs,
        &statuses,
    );

    let missing_root = root.path().join("missing-root");
    let output = Command::new(binary())
        .args([
            "--workspace",
            missing_root.to_str().unwrap(),
            "--json",
            "check",
            "anddress",
            &operands[0],
            "{",
        ])
        .output()
        .unwrap();
    assert_usage(output);
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
    assert_usage(run(root.path(), &["check", "anddress", &operand, &operand]));
    assert_usage(run(
        root.path(),
        &["--json", "check", "anddress", &operand, "{"],
    ));
    assert_usage(run(root.path(), &["--json", "check", "anddress", "{"]));

    let source = include_str!("../../src/bin/bw/output.rs");
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
        .split("fn write_session_refs")
        .next()
        .unwrap();
    assert!(writer.contains("bw.cli.check.v2"));
    assert!(writer.contains("encode_into(&mut scratch)"));
    assert!(!writer.contains("Value"));
    assert!(!writer.contains(".clone()"));
    assert!(!writer.contains("bw.cli.check.v1"));
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
