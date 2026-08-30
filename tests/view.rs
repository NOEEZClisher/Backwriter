mod support;

use std::fs;

use backwriter::backwriter::anddress::{Anddress, AnddressTarget, LineTerminator};
use backwriter::backwriter::search::{
    SearchOutcome, SearchQuery, SearchRequest, SearchScope, SearchTarget,
};
use backwriter::backwriter::view::{ViewError, ViewOutcome};
use backwriter::runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime};
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
    let request = SearchRequest::new(
        SearchQuery::new("one").unwrap(),
        SearchScope::all_admitted(),
        SearchTarget::File,
    );
    match workspace.search(&request).unwrap() {
        SearchOutcome::Found { anddresses } => anddresses[0].workspace_coordinate().to_owned(),
        SearchOutcome::Empty => panic!("coordinate source"),
    }
}

fn address(
    coordinate: String,
    path: &str,
    source: &[u8],
    target: AnddressTarget,
    byte_start: usize,
    byte_end: usize,
) -> Anddress {
    support::address(&coordinate, path, source, target, byte_start, byte_end)
}

#[test]
fn view_checks_exact_source_and_range_and_returns_related_addresses() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = b"one\n\ntwo\r\n";
    fs::write(root.join("note.txt"), source).unwrap();
    let workspace = runtime(&root);
    let input = address(
        coordinate(&workspace),
        "note.txt",
        source,
        AnddressTarget::Line,
        5,
        10,
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
            input.workspace_coordinate().to_owned(),
            "note.txt",
            source,
            AnddressTarget::File,
            0,
            source.len(),
        )
    );
    assert_eq!(
        paragraph.unwrap(),
        address(
            input.workspace_coordinate().to_owned(),
            "note.txt",
            source,
            AnddressTarget::Paragraph,
            5,
            10,
        )
    );

    fs::write(root.join("note.txt"), b"one\n\ntwo\n").unwrap();
    assert_eq!(workspace.view(&input), Err(ViewError::Unavailable));
}

#[test]
fn view_rejects_private_path_before_access_and_allows_other_artext_children() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir_all(root.join(".artext/bw")).unwrap();
    fs::create_dir_all(root.join(".artext/other")).unwrap();
    fs::write(root.join("coordinate.txt"), b"one").unwrap();
    fs::write(root.join(".artext/bw/file"), b"x").unwrap();
    fs::write(root.join(".artext/other/file"), b"x").unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);

    assert_eq!(
        workspace.view(&address(
            coordinate.clone(),
            ".artext/bw/file",
            b"x",
            AnddressTarget::File,
            0,
            1,
        )),
        Err(ViewError::Unavailable)
    );
    assert!(matches!(
        workspace.view(&address(
            coordinate,
            ".artext/other/file",
            b"x",
            AnddressTarget::File,
            0,
            1,
        )),
        Ok(ViewOutcome::File { text }) if text == "x"
    ));
}

#[test]
fn view_reads_unicode_and_all_line_terminators() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), b"one").unwrap();
    let source = "한글\nβ\rγ".as_bytes();
    fs::write(root.join("note.txt"), source).unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);

    for (start, end, content, terminator) in [
        (0, 7, "한글", LineTerminator::Lf),
        (7, 10, "β", LineTerminator::Cr),
        (10, 12, "γ", LineTerminator::None),
    ] {
        assert!(matches!(
            workspace.view(&address(
                coordinate.clone(),
                "note.txt",
                source,
                AnddressTarget::Line,
                start,
                end,
            )),
            Ok(ViewOutcome::Line {
                content: actual,
                terminator: actual_terminator,
                ..
            }) if actual == content && actual_terminator == terminator
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
    let source_path = root.join("note.txt");
    fs::write(&source_path, b"one").unwrap();
    let workspace = runtime(&root);
    let input = address(
        coordinate(&workspace),
        "note.txt",
        b"one",
        AnddressTarget::File,
        0,
        3,
    );
    let outside = fixture.path().join("outside.txt");
    fs::write(&outside, b"one").unwrap();
    fs::remove_file(&source_path).unwrap();
    symlink(&outside, source_path).unwrap();
    assert_eq!(workspace.view(&input), Err(ViewError::Unavailable));
}

#[test]
fn every_target_is_bound_to_the_complete_source_state() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = b"one\ntwo\n\nthree";
    fs::write(root.join("note.txt"), source).unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let file = address(
        coordinate.clone(),
        "note.txt",
        source,
        AnddressTarget::File,
        0,
        source.len(),
    );
    let paragraph = address(
        coordinate,
        "note.txt",
        source,
        AnddressTarget::Paragraph,
        0,
        8,
    );
    assert!(matches!(
        workspace.view(&file),
        Ok(ViewOutcome::File { .. })
    ));
    assert!(matches!(
        workspace.view(&paragraph),
        Ok(ViewOutcome::Paragraph { ref text, .. }) if text == "one\ntwo\n"
    ));

    fs::write(root.join("note.txt"), b"one\ntwo\n\nother").unwrap();
    assert_eq!(workspace.view(&file), Err(ViewError::Unavailable));
    assert_eq!(workspace.view(&paragraph), Err(ViewError::Unavailable));
}

#[test]
fn view_discards_same_length_inside_and_outside_mutations_truncation_and_growth() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = b"one\ntwo\nthree";
    fs::write(root.join("note.txt"), source).unwrap();
    let workspace = runtime(&root);
    let input = address(
        coordinate(&workspace),
        "note.txt",
        source,
        AnddressTarget::Line,
        4,
        8,
    );

    for changed in [
        b"one\nTWO\nthree".as_slice(),
        b"ONE\ntwo\nthree".as_slice(),
        b"one\ntwo".as_slice(),
        b"one\ntwo\nthree!".as_slice(),
    ] {
        fs::write(root.join("note.txt"), changed).unwrap();
        assert_eq!(workspace.view(&input), Err(ViewError::Unavailable));
    }

    fs::write(root.join("note.txt"), source).unwrap();
    assert!(matches!(
        workspace.view(&input),
        Ok(ViewOutcome::Line { content, .. }) if content == "two"
    ));
}

#[test]
fn view_accepts_raw_empty_range_and_rejects_invalid_text_wrong_coordinate_and_nonregular_sources() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), b"one").unwrap();
    fs::create_dir(root.join("directory")).unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    assert!(matches!(
        workspace.view(&address(
            coordinate.clone(),
            "note.txt",
            b"one",
            AnddressTarget::Line,
            1,
            1,
        )),
        Ok(ViewOutcome::Line {
            content,
            terminator: LineTerminator::None,
            paragraph: None,
            ..
        }) if content.is_empty()
    ));
    assert_eq!(
        workspace.view(&address(
            "b".repeat(64),
            "note.txt",
            b"one",
            AnddressTarget::File,
            0,
            3,
        )),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view(&address(
            coordinate.clone(),
            "missing.txt",
            b"one",
            AnddressTarget::File,
            0,
            3,
        )),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view(&address(
            coordinate.clone(),
            "directory",
            b"one",
            AnddressTarget::File,
            0,
            3,
        )),
        Err(ViewError::Unavailable)
    );

    fs::write(root.join("note.txt"), b"one\xff").unwrap();
    assert_eq!(
        workspace.view(&address(
            coordinate.clone(),
            "note.txt",
            b"one\xff",
            AnddressTarget::File,
            0,
            4,
        )),
        Err(ViewError::Unavailable)
    );
    fs::write(root.join("note.txt"), b"one\0").unwrap();
    assert_eq!(
        workspace.view(&address(
            coordinate,
            "note.txt",
            b"one\0",
            AnddressTarget::File,
            0,
            4,
        )),
        Err(ViewError::Unavailable)
    );
}

#[test]
fn separator_line_has_no_paragraph_and_raw_paragraph_range_is_exact() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = b"one\n \t\ntwo";
    fs::write(root.join("note.txt"), source).unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);

    assert!(matches!(
        workspace.view(&address(
            coordinate.clone(),
            "note.txt",
            source,
            AnddressTarget::Paragraph,
            1,
            1,
        )),
        Ok(ViewOutcome::Paragraph { text, .. }) if text.is_empty()
    ));
    let ViewOutcome::Line { paragraph, .. } = workspace
        .view(&address(
            coordinate,
            "note.txt",
            source,
            AnddressTarget::Line,
            4,
            7,
        ))
        .unwrap()
    else {
        panic!("separator")
    };
    assert!(paragraph.is_none());
}

#[test]
fn raw_nonstructural_ranges_return_exact_bytes_without_asserting_relations() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = "zero\none\r\ntwo".as_bytes();
    fs::write(root.join("note.txt"), source).unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);

    assert!(matches!(
        workspace.view(&address(
            coordinate.clone(),
            "note.txt",
            source,
            AnddressTarget::Paragraph,
            2,
            10,
        )),
        Ok(ViewOutcome::Paragraph { text, .. }) if text == "ro\none\r\n"
    ));
    assert!(matches!(
        workspace.view(&address(
            coordinate.clone(),
            "note.txt",
            source,
            AnddressTarget::Line,
            2,
            10,
        )),
        Ok(ViewOutcome::Line {
            content,
            terminator: LineTerminator::Crlf,
            paragraph: None,
            ..
        }) if content == "ro\none"
    ));

    let unicode = "aéz".as_bytes();
    fs::write(root.join("note.txt"), unicode).unwrap();
    for target in [AnddressTarget::Paragraph, AnddressTarget::Line] {
        assert_eq!(
            workspace.view(&address(
                coordinate.clone(),
                "note.txt",
                unicode,
                target,
                2,
                3,
            )),
            Err(ViewError::Unavailable)
        );
    }
}

#[test]
fn view_preserves_ranges_at_every_source_scratch_boundary() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), b"one").unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);

    for boundary in [8_191, 8_192, 8_193] {
        let mut source = vec![b'x'; boundary];
        source.extend_from_slice("é".as_bytes());
        source.extend_from_slice(b"\r\ntail");
        fs::write(root.join("note.txt"), &source).unwrap();
        let end = boundary + "é".len() + 2;
        let input = address(
            coordinate.clone(),
            "note.txt",
            &source,
            AnddressTarget::Line,
            0,
            end,
        );
        assert!(matches!(
            workspace.view(&input),
            Ok(ViewOutcome::Line {
                content,
                terminator: LineTerminator::Crlf,
                paragraph: Some(_),
                ..
            }) if content == format!("{}é", "x".repeat(boundary))
        ));
    }
}

#[test]
fn view_finds_a_real_line_after_4096_lines_and_raw_state_can_reestablish() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = "one\n".repeat(4097);
    fs::write(root.join("note.txt"), &source).unwrap();
    let workspace = runtime(&root);
    let input = address(
        coordinate(&workspace),
        "note.txt",
        source.as_bytes(),
        AnddressTarget::Line,
        4096 * 4,
        4097 * 4,
    );
    assert!(matches!(
        workspace.view(&input),
        Ok(ViewOutcome::Line { .. })
    ));
    fs::write(root.join("note.txt"), b"two\n").unwrap();
    assert_eq!(workspace.view(&input), Err(ViewError::Unavailable));
    fs::write(root.join("note.txt"), &source).unwrap();
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

    for target in [
        AnddressTarget::File,
        AnddressTarget::Paragraph,
        AnddressTarget::Line,
    ] {
        let outcome = workspace
            .view(&address(
                coordinate.clone(),
                "note.txt",
                source.as_bytes(),
                target,
                0,
                source.len(),
            ))
            .unwrap();
        let text = match outcome {
            ViewOutcome::File { text } | ViewOutcome::Paragraph { text, .. } => text,
            ViewOutcome::Line { content, .. } => content,
        };
        assert_eq!(text, source);
    }
}

#[test]
fn view_accepts_same_coordinate_under_a_different_admission() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir_all(root.join("docs")).unwrap();
    fs::write(root.join("docs/note.txt"), b"one").unwrap();
    let all = runtime(&root);
    let input = address(
        coordinate(&all),
        "docs/note.txt",
        b"one",
        AnddressTarget::File,
        0,
        3,
    );
    assert!(matches!(
        runtime_with_admission(&root, "docs").view(&input),
        Ok(ViewOutcome::File { text }) if text == "one"
    ));
}
