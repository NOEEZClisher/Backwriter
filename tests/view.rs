use std::fs;

use artext::backwriter::anddress::{
    ANDDRESS_VERSION, Anddress, AnddressTarget, LineTerminator, Natural,
};
use artext::backwriter::view::{ViewError, ViewOutcome};
use artext::runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime};
use tempfile::tempdir;

fn runtime(root: &std::path::Path) -> WorkspaceRuntime {
    runtime_with_admission(root, ".")
}
fn runtime_with_admission(root: &std::path::Path, admission: &str) -> WorkspaceRuntime {
    WorkspaceRuntime::open(
        root,
        WorkspaceAdmission::new([AdmissionRoot::new(admission).unwrap()]).unwrap(),
    )
    .unwrap()
}
fn coordinate(workspace: &WorkspaceRuntime) -> String {
    let request = artext::backwriter::search::SearchRequest::new(
        artext::backwriter::search::SearchQuery::new("one").unwrap(),
        artext::backwriter::search::SearchScope::all_admitted(),
        artext::backwriter::search::SearchTarget::File,
    );
    match workspace.search(&request).unwrap() {
        artext::backwriter::search::SearchOutcome::Found { anddresses } => {
            anddresses[0].workspace_coordinate.clone()
        }
        _ => panic!("coordinate source"),
    }
}
fn address(coordinate: String, path: &str, target: AnddressTarget) -> Anddress {
    Anddress {
        version: ANDDRESS_VERSION.to_owned(),
        workspace_coordinate: coordinate,
        logical_path: path.to_owned(),
        target,
    }
}

#[test]
fn view_checks_current_target_locator_and_returns_related_addresses() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), "one\n\ntwo\r\n").unwrap();
    let workspace = runtime(&root);
    let input = address(
        coordinate(&workspace),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::parse("2").unwrap(),
            exact_extent: "two\r\n".to_owned(),
        },
    );
    let ViewOutcome::Line {
        content,
        terminator,
        file,
        paragraph,
    } = workspace.view(&input).unwrap()
    else {
        panic!("line")
    };
    assert_eq!(content, "two");
    assert_eq!(terminator, LineTerminator::Crlf);
    assert_eq!(
        file,
        address(
            input.workspace_coordinate.clone(),
            "note.txt",
            AnddressTarget::File
        )
    );
    assert_eq!(
        paragraph.unwrap(),
        address(
            input.workspace_coordinate.clone(),
            "note.txt",
            AnddressTarget::Paragraph {
                ordinal: Natural::one()
            }
        )
    );
    fs::write(root.join("note.txt"), "one\n\ntwo\n").unwrap();
    assert_eq!(workspace.view(&input), Err(ViewError::Unavailable));
}

#[test]
fn view_rejects_invalid_or_private_inputs_before_access() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "one").unwrap();
    fs::create_dir_all(root.join(".artext/bw")).unwrap();
    fs::write(root.join(".artext/bw/file"), "x").unwrap();
    let workspace = runtime(&root);
    let input = address(
        coordinate(&workspace),
        ".artext/bw/file",
        AnddressTarget::File,
    );
    assert_eq!(workspace.view(&input), Err(ViewError::Unavailable));
    let invalid = Anddress {
        version: ANDDRESS_VERSION.to_owned(),
        workspace_coordinate: "x".to_owned(),
        logical_path: "missing".to_owned(),
        target: AnddressTarget::File,
    };
    assert_eq!(workspace.view(&invalid), Err(ViewError::InvalidInput));
}

#[test]
fn view_reads_unicode_and_each_exact_line_terminator_from_ordinary_artext_source() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir_all(root.join(".artext/other")).unwrap();
    fs::write(root.join("coordinate.txt"), "one").unwrap();
    fs::write(root.join(".artext/other/note.txt"), "한글\nβ\rγ").unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    for (ordinal, extent, content, terminator) in [
        ("0", "한글\n", "한글", LineTerminator::Lf),
        ("1", "β\r", "β", LineTerminator::Cr),
        ("2", "γ", "γ", LineTerminator::None),
    ] {
        assert!(matches!(
            workspace.view(&address(
                coordinate.clone(),
                ".artext/other/note.txt",
                AnddressTarget::Line {
                    ordinal: Natural::parse(ordinal).unwrap(),
                    exact_extent: extent.to_owned(),
                },
            )),
            Ok(ViewOutcome::Line { content: actual, terminator: actual_terminator, .. })
                if actual == content && actual_terminator == terminator
        ));
    }
}

#[cfg(unix)]
#[test]
fn view_maps_replaced_source_symlink_to_unavailable() {
    use std::os::unix::fs::symlink;

    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = root.join("note.txt");
    fs::write(&source, "one").unwrap();
    let workspace = runtime(&root);
    let input = address(coordinate(&workspace), "note.txt", AnddressTarget::File);
    let outside = fixture.path().join("outside.txt");
    fs::write(&outside, "one").unwrap();
    fs::remove_file(&source).unwrap();
    symlink(&outside, source).unwrap();
    assert_eq!(workspace.view(&input), Err(ViewError::Unavailable));
}

#[test]
fn view_file_and_paragraph_currentness_are_target_specific() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), "one\ntwo\n\nthree").unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let file = address(coordinate.clone(), "note.txt", AnddressTarget::File);
    let paragraph = address(
        coordinate.clone(),
        "note.txt",
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    assert!(matches!(
        workspace.view(&file),
        Ok(ViewOutcome::File { .. })
    ));
    let ViewOutcome::Paragraph {
        text,
        file: related_file,
    } = workspace.view(&paragraph).unwrap()
    else {
        panic!("paragraph")
    };
    assert_eq!(text, "one\ntwo\n");
    assert_eq!(related_file, file);
    fs::write(root.join("note.txt"), "changed\n\nthree").unwrap();
    assert!(
        matches!(workspace.view(&file), Ok(ViewOutcome::File { text }) if text.starts_with("changed"))
    );
    assert!(
        matches!(workspace.view(&paragraph), Ok(ViewOutcome::Paragraph { text, .. }) if text == "changed\n")
    );
}

#[test]
fn view_maps_huge_missing_ordinal_and_invalid_text_to_unavailable() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), "one").unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let huge = address(
        coordinate.clone(),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::parse(&format!("1{}", "0".repeat(4097))).unwrap(),
            exact_extent: "one".to_owned(),
        },
    );
    assert_eq!(workspace.view(&huge), Err(ViewError::Unavailable));
    fs::write(root.join("note.txt"), b"one\xff").unwrap();
    assert_eq!(
        workspace.view(&address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::File
        )),
        Err(ViewError::Unavailable)
    );
    fs::write(root.join("note.txt"), b"one\0").unwrap();
    assert_eq!(
        workspace.view(&address(coordinate, "note.txt", AnddressTarget::File)),
        Err(ViewError::Unavailable)
    );
}

#[test]
fn view_accepts_same_coordinate_under_a_different_admission() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir_all(root.join("docs")).unwrap();
    fs::write(root.join("docs/note.txt"), "one").unwrap();
    let all = runtime(&root);
    let input = address(coordinate(&all), "docs/note.txt", AnddressTarget::File);
    assert!(matches!(
        runtime_with_admission(&root, "docs").view(&input),
        Ok(ViewOutcome::File { text }) if text == "one"
    ));
}

#[test]
fn view_closes_unsupported_coordinate_missing_and_nonregular_inputs() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), "one").unwrap();
    fs::create_dir(root.join("directory")).unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let unsupported = Anddress {
        version: "other".to_owned(),
        workspace_coordinate: "not-a-coordinate".to_owned(),
        logical_path: "\0".to_owned(),
        target: AnddressTarget::File,
    };
    assert_eq!(
        workspace.view(&unsupported),
        Err(ViewError::UnsupportedVersion)
    );
    assert_eq!(
        workspace.view(&address("b".repeat(64), "note.txt", AnddressTarget::File)),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view(&address(
            coordinate.clone(),
            "missing.txt",
            AnddressTarget::File
        )),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view(&address(coordinate, "directory", AnddressTarget::File)),
        Err(ViewError::Unavailable)
    );
}

#[test]
fn view_distinguishes_missing_paragraph_line_mismatch_and_separator_parent() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), "one\n \t\ntwo").unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    assert_eq!(
        workspace.view(&address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Paragraph {
                ordinal: Natural::parse("2").unwrap(),
            },
        )),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view(&address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Line {
                ordinal: Natural::parse("9").unwrap(),
                exact_extent: "one\n".to_owned(),
            },
        )),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view(&address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Line {
                ordinal: Natural::zero(),
                exact_extent: "changed\n".to_owned(),
            },
        )),
        Err(ViewError::Unavailable)
    );
    let ViewOutcome::Line { paragraph, .. } = workspace
        .view(&address(
            coordinate,
            "note.txt",
            AnddressTarget::Line {
                ordinal: Natural::one(),
                exact_extent: " \t\n".to_owned(),
            },
        ))
        .unwrap()
    else {
        panic!("separator")
    };
    assert!(paragraph.is_none());
}

#[test]
fn view_finds_a_real_4097th_line_and_tuple_reestablishment_is_only_current_lookup() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), "one\n".repeat(4097)).unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let input = address(
        coordinate,
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::parse("4096").unwrap(),
            exact_extent: "one\n".to_owned(),
        },
    );
    assert!(matches!(
        workspace.view(&input),
        Ok(ViewOutcome::Line { .. })
    ));
    fs::write(root.join("note.txt"), "two\n").unwrap();
    assert_eq!(workspace.view(&input), Err(ViewError::Unavailable));
    fs::write(root.join("note.txt"), "one\n".repeat(4097)).unwrap();
    assert!(matches!(
        workspace.view(&input),
        Ok(ViewOutcome::Line { .. })
    ));
}

#[test]
fn view_returns_complete_large_file_paragraph_and_line_outputs() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = format!("one-first{}last", "x".repeat(24_000));
    fs::write(root.join("note.txt"), &source).unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);

    let ViewOutcome::File { text } = workspace
        .view(&address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::File,
        ))
        .unwrap()
    else {
        panic!("file")
    };
    assert_eq!(text.len(), source.len());
    assert!(text.starts_with("one-first") && text.ends_with("last"));

    let ViewOutcome::Paragraph { text, .. } = workspace
        .view(&address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Paragraph {
                ordinal: Natural::zero(),
            },
        ))
        .unwrap()
    else {
        panic!("paragraph")
    };
    assert_eq!(text.len(), source.len());
    assert!(text.starts_with("one-first") && text.ends_with("last"));

    let ViewOutcome::Line { content, .. } = workspace
        .view(&address(
            coordinate,
            "note.txt",
            AnddressTarget::Line {
                ordinal: Natural::zero(),
                exact_extent: source.clone(),
            },
        ))
        .unwrap()
    else {
        panic!("line")
    };
    assert_eq!(content.len(), source.len());
    assert!(content.starts_with("one-first") && content.ends_with("last"));
}

#[test]
fn view_paragraph_ignores_unrelated_chunk_spanning_lines() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = format!(
        "{}\n\none-target\n\n{}\n",
        "before".repeat(2_000),
        "after".repeat(2_000)
    );
    fs::write(root.join("note.txt"), source).unwrap();
    let workspace = runtime(&root);
    let input = address(
        coordinate(&workspace),
        "note.txt",
        AnddressTarget::Paragraph {
            ordinal: Natural::one(),
        },
    );

    assert!(matches!(
        workspace.view(&input),
        Ok(ViewOutcome::Paragraph { text, .. }) if text == "one-target\n"
    ));
}
