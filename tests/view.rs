mod support;

use std::fs;

use backwriter::backwriter::anchor::AnchorOutcome;
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

fn host_runtime(root: &std::path::Path) -> WorkspaceRuntime {
    WorkspaceRuntime::open_host_authoritative(
        root,
        WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
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

fn projected(anddress: Anddress, content: &[u8]) -> ViewOutcome {
    ViewOutcome::Projected {
        anddress,
        content: String::from_utf8(content.to_vec()).unwrap(),
    }
}

#[test]
fn view_checks_exact_source_range_and_projected_address() {
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

    let ViewOutcome::Projected { anddress, content } =
        workspace.view(&input, input.target()).unwrap()
    else {
        panic!("line")
    };
    assert_eq!(anddress, input);
    assert_eq!(content, "two\r\n");
    assert_eq!(anddress.terminator(), Some(LineTerminator::Crlf));
    assert_eq!(
        anddress.project(AnddressTarget::File).unwrap().unwrap(),
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
        anddress
            .project(AnddressTarget::Paragraph)
            .unwrap()
            .unwrap(),
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
    assert_eq!(
        workspace.view(&input, input.target()),
        Err(ViewError::Unavailable)
    );
}

#[test]
fn view_projects_all_six_upward_relations_from_one_current_observation() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = b"one\r\ntwo\n \t\r\nlast\r";
    fs::write(root.join("note.txt"), source).unwrap();
    let mut workspace = runtime(&root);
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
        coordinate.clone(),
        "note.txt",
        source,
        AnddressTarget::Paragraph,
        0,
        9,
    );
    let line = address(coordinate, "note.txt", source, AnddressTarget::Line, 5, 9);

    assert_eq!(
        workspace.view(&file, AnddressTarget::File),
        Ok(projected(file.clone(), source))
    );
    assert_eq!(
        workspace.view(&paragraph, AnddressTarget::Paragraph),
        Ok(projected(paragraph.clone(), b"one\r\ntwo\n"))
    );
    assert_eq!(
        workspace.view(&paragraph, AnddressTarget::File),
        Ok(projected(file.clone(), source))
    );
    assert_eq!(
        workspace.view(&line, AnddressTarget::Line),
        Ok(projected(line.clone(), b"two\n"))
    );
    assert_eq!(
        workspace.view(&line, AnddressTarget::Paragraph),
        Ok(projected(paragraph.clone(), b"one\r\ntwo\n"))
    );
    assert_eq!(
        workspace.view(&line, AnddressTarget::File),
        Ok(projected(file.clone(), source))
    );

    let handle = match workspace.anchor(&line).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("Line Anchor"),
    };
    assert_eq!(
        workspace.view_anchored(&handle, AnddressTarget::Paragraph),
        Ok(projected(paragraph, b"one\r\ntwo\n"))
    );
    assert_eq!(
        workspace.view_anchored(&handle, AnddressTarget::File),
        Ok(projected(file, source))
    );
}

#[test]
fn view_batch_preserves_empty_single_duplicate_and_mixed_source_order() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source_a = b"one\r\ntwo\n \t\r\nlast\r";
    let source_b = "β\none".as_bytes();
    fs::write(root.join("a.txt"), source_a).unwrap();
    fs::write(root.join("b.txt"), source_b).unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let file_a = address(
        coordinate.clone(),
        "a.txt",
        source_a,
        AnddressTarget::File,
        0,
        source_a.len(),
    );
    let paragraph_a = address(
        coordinate.clone(),
        "a.txt",
        source_a,
        AnddressTarget::Paragraph,
        0,
        9,
    );
    let line_a = address(
        coordinate.clone(),
        "a.txt",
        source_a,
        AnddressTarget::Line,
        5,
        9,
    );
    let file_b = address(
        coordinate.clone(),
        "b.txt",
        source_b,
        AnddressTarget::File,
        0,
        source_b.len(),
    );
    let line_b = address(coordinate, "b.txt", source_b, AnddressTarget::Line, 0, 3);

    assert_eq!(
        workspace.view_batch(&[], AnddressTarget::File),
        Ok(Vec::new())
    );
    assert_eq!(
        workspace.view_batch(std::slice::from_ref(&line_a), AnddressTarget::Line),
        Ok(vec![projected(line_a.clone(), b"two\n")])
    );

    let output_a = projected(file_a.clone(), source_a);
    let output_b = projected(file_b, source_b);
    assert_eq!(
        workspace.view_batch(
            &[line_a.clone(), line_b, line_a, paragraph_a, file_a,],
            AnddressTarget::File,
        ),
        Ok(vec![
            output_a.clone(),
            output_b,
            output_a.clone(),
            output_a.clone(),
            output_a,
        ])
    );
}

#[test]
fn view_batch_preserves_relations_terminators_unicode_and_raw_ranges() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = "α\nβ\rγ\r\n \t\nlast".as_bytes();
    fs::write(root.join("coordinate.txt"), b"one").unwrap();
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
        coordinate.clone(),
        "note.txt",
        source,
        AnddressTarget::Paragraph,
        0,
        10,
    );
    let last_paragraph = address(
        coordinate.clone(),
        "note.txt",
        source,
        AnddressTarget::Paragraph,
        13,
        17,
    );
    let lines = [
        address(
            coordinate.clone(),
            "note.txt",
            source,
            AnddressTarget::Line,
            0,
            3,
        ),
        address(
            coordinate.clone(),
            "note.txt",
            source,
            AnddressTarget::Line,
            3,
            6,
        ),
        address(
            coordinate.clone(),
            "note.txt",
            source,
            AnddressTarget::Line,
            6,
            10,
        ),
        address(
            coordinate.clone(),
            "note.txt",
            source,
            AnddressTarget::Line,
            10,
            13,
        ),
        address(
            coordinate.clone(),
            "note.txt",
            source,
            AnddressTarget::Line,
            13,
            17,
        ),
    ];
    let raw = address(coordinate, "note.txt", source, AnddressTarget::Line, 3, 8);

    let line_outcomes = workspace.view_batch(&lines, AnddressTarget::Line).unwrap();
    assert_eq!(line_outcomes.len(), 5);
    for (outcome, content, terminator, related) in [
        (
            &line_outcomes[0],
            "α\n",
            LineTerminator::Lf,
            Some(&paragraph),
        ),
        (
            &line_outcomes[1],
            "β\r",
            LineTerminator::Cr,
            Some(&paragraph),
        ),
        (
            &line_outcomes[2],
            "γ\r\n",
            LineTerminator::Crlf,
            Some(&paragraph),
        ),
        (&line_outcomes[3], " \t\n", LineTerminator::Lf, None),
        (
            &line_outcomes[4],
            "last",
            LineTerminator::None,
            Some(&last_paragraph),
        ),
    ] {
        assert!(matches!(
            outcome,
            ViewOutcome::Projected {
                anddress,
                content: actual,
            } if actual == content
                && anddress.terminator() == Some(terminator)
                && anddress.project(AnddressTarget::File).unwrap().as_ref() == Some(&file)
                && anddress.project(AnddressTarget::Paragraph).unwrap().as_ref() == related
        ));
    }

    let paragraph_output = projected(paragraph.clone(), "α\nβ\rγ\r\n".as_bytes());
    assert_eq!(
        workspace.view_batch(
            &[
                paragraph.clone(),
                lines[0].clone(),
                lines[3].clone(),
                raw,
                lines[4].clone(),
                lines[0].clone(),
            ],
            AnddressTarget::Paragraph,
        ),
        Ok(vec![
            paragraph_output.clone(),
            paragraph_output.clone(),
            ViewOutcome::RelationAbsent,
            ViewOutcome::RelationAbsent,
            projected(last_paragraph, b"last"),
            paragraph_output,
        ])
    );
}

#[test]
fn view_batch_preflights_relations_and_fails_all_for_unavailable_members() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("a.txt"), b"one\n").unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let current = address(
        coordinate.clone(),
        "a.txt",
        b"one\n",
        AnddressTarget::File,
        0,
        4,
    );
    let paragraph = address(
        coordinate.clone(),
        "a.txt",
        b"one\n",
        AnddressTarget::Paragraph,
        0,
        4,
    );
    let line = address(
        coordinate.clone(),
        "a.txt",
        b"one\n",
        AnddressTarget::Line,
        0,
        4,
    );

    fs::write(root.join("b.txt"), b"new\n").unwrap();
    fs::write(root.join("invalid.txt"), b"\xff").unwrap();
    fs::write(root.join("zero.txt"), b"x\0").unwrap();
    let unavailable = [
        address(
            coordinate.clone(),
            "b.txt",
            b"old\n",
            AnddressTarget::File,
            0,
            4,
        ),
        address(
            coordinate.clone(),
            "invalid.txt",
            b"\xff",
            AnddressTarget::File,
            0,
            1,
        ),
        address(
            coordinate.clone(),
            "zero.txt",
            b"x\0",
            AnddressTarget::File,
            0,
            2,
        ),
        address(
            coordinate.clone(),
            "missing.txt",
            b"missing",
            AnddressTarget::File,
            0,
            7,
        ),
        address(
            "b".repeat(64),
            "foreign.txt",
            b"x",
            AnddressTarget::File,
            0,
            1,
        ),
        address(
            coordinate.clone(),
            ".artext/bw/private.txt",
            b"x",
            AnddressTarget::File,
            0,
            1,
        ),
    ];
    for bad in unavailable {
        assert_eq!(
            workspace.view_batch(&[current.clone(), bad], AnddressTarget::File),
            Err(ViewError::Unavailable)
        );
    }

    fs::create_dir_all(root.join("docs")).unwrap();
    fs::write(root.join("docs/note.txt"), b"one\n").unwrap();
    fs::write(root.join("outside.txt"), b"one\n").unwrap();
    let admitted = runtime_with_admission(&root, "docs");
    let admitted_file = address(
        coordinate.clone(),
        "docs/note.txt",
        b"one\n",
        AnddressTarget::File,
        0,
        4,
    );
    let unadmitted_file = address(
        coordinate.clone(),
        "outside.txt",
        b"one\n",
        AnddressTarget::File,
        0,
        4,
    );
    assert_eq!(
        admitted.view_batch(&[admitted_file, unadmitted_file], AnddressTarget::File),
        Err(ViewError::Unavailable)
    );

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        let outside = fixture.path().join("outside.txt");
        fs::write(&outside, b"one\n").unwrap();
        symlink(&outside, root.join("linked.txt")).unwrap();
        let linked = address(
            coordinate.clone(),
            "linked.txt",
            b"one\n",
            AnddressTarget::File,
            0,
            4,
        );
        assert_eq!(
            workspace.view_batch(&[current.clone(), linked], AnddressTarget::File),
            Err(ViewError::Unavailable)
        );
    }

    fs::remove_file(root.join("a.txt")).unwrap();
    for (inputs, projection) in [
        (
            vec![paragraph.clone(), current.clone()],
            AnddressTarget::Paragraph,
        ),
        (vec![line.clone(), current], AnddressTarget::Line),
        (vec![line, paragraph], AnddressTarget::Line),
    ] {
        assert_eq!(
            workspace.view_batch(&inputs, projection),
            Err(ViewError::InvalidInput)
        );
    }
}

#[test]
fn host_view_batch_reuses_and_invalidates_proof_per_source_group() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source_a = b"one\n";
    let source_b = b"one\r\n";
    fs::write(root.join("a.txt"), source_a).unwrap();
    fs::write(root.join("b.txt"), source_b).unwrap();
    let coordinate = coordinate(&runtime(&root));
    let line_a = address(
        coordinate.clone(),
        "a.txt",
        source_a,
        AnddressTarget::Line,
        0,
        source_a.len(),
    );
    let line_b = address(
        coordinate.clone(),
        "b.txt",
        source_b,
        AnddressTarget::Line,
        0,
        source_b.len(),
    );
    let inputs = [line_a.clone(), line_b.clone(), line_a.clone()];
    let mut host = host_runtime(&root);
    let request = SearchRequest::new(
        SearchQuery::new("one").unwrap(),
        SearchScope::all_admitted(),
        SearchTarget::Line,
    );
    assert!(matches!(
        host.search(&request),
        Ok(SearchOutcome::Found { anddresses }) if anddresses.len() == 2
    ));

    let trusted = host.view_batch(&inputs, AnddressTarget::Line).unwrap();
    let direct = runtime(&root)
        .view_batch(&inputs, AnddressTarget::Line)
        .unwrap();
    assert_eq!(trusted, direct);
    assert_eq!(trusted[0], trusted[2]);

    let stale_a = address(coordinate, "a.txt", b"two\n", AnddressTarget::Line, 0, 4);
    let parked_a = root.join("parked-a");
    fs::rename(root.join("a.txt"), &parked_a).unwrap();
    assert_eq!(
        host.view_batch(&[line_a.clone(), stale_a], AnddressTarget::Line),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        host.check(line_a.clone()).unwrap().filtered,
        Some(line_a.clone())
    );
    fs::rename(&parked_a, root.join("a.txt")).unwrap();

    let parked_b = root.join("parked-b");
    fs::rename(root.join("b.txt"), &parked_b).unwrap();
    assert_eq!(
        host.view_batch(&[line_b.clone(), line_b.clone()], AnddressTarget::Line,),
        Err(ViewError::Unavailable)
    );
    assert_eq!(host.check(line_b.clone()).unwrap().filtered, None);
    fs::rename(&parked_b, root.join("b.txt")).unwrap();
    assert_eq!(
        host.check(line_a.clone()).unwrap().filtered,
        Some(line_a.clone())
    );

    host.invalidate_source("a.txt").unwrap();
    assert_eq!(
        host.view_batch(&[line_a.clone(), line_a.clone()], AnddressTarget::Line,),
        runtime(&root).view_batch(&[line_a.clone(), line_a], AnddressTarget::Line)
    );
}

#[test]
fn view_rejects_downward_requests_before_io_and_returns_relation_absent() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = b"one\r\ntwo\n \t\r\nlast\r";
    let source_path = root.join("note.txt");
    fs::write(&source_path, source).unwrap();
    let mut workspace = runtime(&root);
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
        coordinate.clone(),
        "note.txt",
        source,
        AnddressTarget::Paragraph,
        0,
        9,
    );
    let separator = address(
        coordinate.clone(),
        "note.txt",
        source,
        AnddressTarget::Line,
        9,
        13,
    );
    let raw = address(coordinate, "note.txt", source, AnddressTarget::Line, 1, 7);

    assert_eq!(
        workspace.view(&separator, AnddressTarget::Paragraph),
        Ok(ViewOutcome::RelationAbsent)
    );
    assert_eq!(
        workspace.view(&raw, AnddressTarget::Paragraph),
        Ok(ViewOutcome::RelationAbsent)
    );
    let handle = match workspace.anchor(&separator).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("separator Line Anchor"),
    };
    let file_handle = match workspace.anchor(&file).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("File Anchor"),
    };
    assert_eq!(
        workspace.view_anchored(&handle, AnddressTarget::Paragraph),
        Ok(ViewOutcome::RelationAbsent)
    );

    fs::remove_file(source_path).unwrap();
    for (input, projection) in [
        (&file, AnddressTarget::Paragraph),
        (&file, AnddressTarget::Line),
        (&paragraph, AnddressTarget::Line),
    ] {
        assert_eq!(
            workspace.view(input, projection),
            Err(ViewError::InvalidInput)
        );
    }
    assert_eq!(
        workspace.view_anchored(&file_handle, AnddressTarget::Paragraph),
        Err(ViewError::InvalidInput)
    );
    assert_eq!(
        workspace.view_anchored(&handle, AnddressTarget::Line),
        Err(ViewError::Unavailable)
    );
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
        workspace.view(
            &address(
                coordinate.clone(),
                ".artext/bw/file",
                b"x",
                AnddressTarget::File,
                0,
                1,
            ),
            AnddressTarget::File
        ),
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
        ), AnddressTarget::File),
        Ok(ViewOutcome::Projected { content, .. }) if content == "x"
    ));
}

#[test]
fn view_reads_unicode_and_all_line_terminators() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), b"one").unwrap();
    let source = "한글\nβ\rγ\r\nδ".as_bytes();
    fs::write(root.join("note.txt"), source).unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);

    for (start, end, terminator) in [
        (0, 7, LineTerminator::Lf),
        (7, 10, LineTerminator::Cr),
        (10, 14, LineTerminator::Crlf),
        (14, 16, LineTerminator::None),
    ] {
        assert!(matches!(
            workspace.view(&address(
                coordinate.clone(),
                "note.txt",
                source,
                AnddressTarget::Line,
                start,
                end,
            ), AnddressTarget::Line),
            Ok(ViewOutcome::Projected {
                anddress,
                content: actual,
            }) if actual.as_bytes() == &source[start..end]
                && anddress.terminator() == Some(terminator)
        ));
        assert!(matches!(
            workspace.view(
                &address(
                    coordinate.clone(),
                    "note.txt",
                    source,
                    AnddressTarget::Line,
                    start,
                    end,
                ),
                AnddressTarget::Paragraph,
            ),
            Ok(ViewOutcome::Projected { content, .. }) if content.as_bytes() == source
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
    assert_eq!(
        workspace.view(&input, input.target()),
        Err(ViewError::Unavailable)
    );
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
        workspace.view(&file, file.target()),
        Ok(ViewOutcome::Projected { .. })
    ));
    assert!(matches!(
        workspace.view(&paragraph, paragraph.target()),
        Ok(ViewOutcome::Projected { ref content, .. }) if content == "one\ntwo\n"
    ));

    fs::write(root.join("note.txt"), b"one\ntwo\n\nother").unwrap();
    assert_eq!(
        workspace.view(&file, file.target()),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view(&paragraph, paragraph.target()),
        Err(ViewError::Unavailable)
    );
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
        assert_eq!(
            workspace.view(&input, input.target()),
            Err(ViewError::Unavailable)
        );
    }

    fs::write(root.join("note.txt"), source).unwrap();
    assert!(matches!(
        workspace.view(&input, input.target()),
        Ok(ViewOutcome::Projected { content, .. }) if content == "two\n"
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
        ), AnddressTarget::Line),
        Ok(ViewOutcome::Projected {
            anddress,
            content,
        }) if content.is_empty()
            && anddress.terminator() == Some(LineTerminator::None)
            && anddress.project(AnddressTarget::Paragraph).unwrap().is_none()
    ));
    assert_eq!(
        workspace.view(
            &address(
                "b".repeat(64),
                "note.txt",
                b"one",
                AnddressTarget::File,
                0,
                3,
            ),
            AnddressTarget::File
        ),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view(
            &address(
                coordinate.clone(),
                "missing.txt",
                b"one",
                AnddressTarget::File,
                0,
                3,
            ),
            AnddressTarget::File
        ),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view(
            &address(
                coordinate.clone(),
                "directory",
                b"one",
                AnddressTarget::File,
                0,
                3,
            ),
            AnddressTarget::File
        ),
        Err(ViewError::Unavailable)
    );

    fs::write(root.join("note.txt"), b"one\xff").unwrap();
    assert_eq!(
        workspace.view(
            &address(
                coordinate.clone(),
                "note.txt",
                b"one\xff",
                AnddressTarget::File,
                0,
                4,
            ),
            AnddressTarget::File
        ),
        Err(ViewError::Unavailable)
    );
    fs::write(root.join("note.txt"), b"one\0").unwrap();
    assert_eq!(
        workspace.view(
            &address(coordinate, "note.txt", b"one\0", AnddressTarget::File, 0, 4,),
            AnddressTarget::File
        ),
        Err(ViewError::Unavailable)
    );
}

#[test]
fn separator_line_has_no_paragraph() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = b"one\n \t\ntwo";
    fs::write(root.join("note.txt"), source).unwrap();
    let workspace = runtime(&root);
    let coordinate = coordinate(&workspace);

    let ViewOutcome::Projected { anddress, .. } = workspace
        .view(
            &address(coordinate, "note.txt", source, AnddressTarget::Line, 4, 7),
            AnddressTarget::Line,
        )
        .unwrap()
    else {
        panic!("separator")
    };
    assert!(
        anddress
            .project(AnddressTarget::Paragraph)
            .unwrap()
            .is_none()
    );
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
        ), AnddressTarget::Paragraph),
        Ok(ViewOutcome::Projected { content, .. }) if content == "ro\none\r\n"
    ));
    assert!(matches!(
        workspace.view(&address(
            coordinate.clone(),
            "note.txt",
            source,
            AnddressTarget::Line,
            2,
            10,
        ), AnddressTarget::Line),
        Ok(ViewOutcome::Projected {
            anddress,
            content,
        }) if content == "ro\none\r\n"
            && anddress.terminator() == Some(LineTerminator::None)
            && anddress.project(AnddressTarget::Paragraph).unwrap().is_none()
    ));

    let unicode = "aéz".as_bytes();
    fs::write(root.join("note.txt"), unicode).unwrap();
    for target in [AnddressTarget::Paragraph, AnddressTarget::Line] {
        assert_eq!(
            workspace.view(
                &address(coordinate.clone(), "note.txt", unicode, target, 2, 3,),
                target
            ),
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
            workspace.view(&input, input.target()),
            Ok(ViewOutcome::Projected {
                anddress,
                content,
            }) if content == format!("{}é\r\n", "x".repeat(boundary))
                && anddress.terminator() == Some(LineTerminator::Crlf)
                && anddress.project(AnddressTarget::Paragraph).unwrap().is_some()
        ));
        assert!(matches!(
            workspace.view(&input, AnddressTarget::Paragraph),
            Ok(ViewOutcome::Projected {
                anddress,
                content,
            }) if anddress.byte_start() == 0
                && anddress.byte_end() == source.len()
                && content.as_bytes() == source
        ));
        assert!(matches!(
            workspace.view(&input, AnddressTarget::File),
            Ok(ViewOutcome::Projected { anddress, content })
                if anddress.byte_start() == 0
                    && anddress.byte_end() == source.len()
                    && content.as_bytes() == source
        ));
        for projection in [
            AnddressTarget::Line,
            AnddressTarget::Paragraph,
            AnddressTarget::File,
        ] {
            let outputs = workspace
                .view_batch(&[input.clone(), input.clone()], projection)
                .unwrap();
            assert_eq!(outputs.len(), 2);
            assert_eq!(outputs[0], outputs[1]);
        }
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
        workspace.view(&input, input.target()),
        Ok(ViewOutcome::Projected { .. })
    ));
    fs::write(root.join("note.txt"), b"two\n").unwrap();
    assert_eq!(
        workspace.view(&input, input.target()),
        Err(ViewError::Unavailable)
    );
    fs::write(root.join("note.txt"), &source).unwrap();
    assert!(matches!(
        workspace.view(&input, input.target()),
        Ok(ViewOutcome::Projected { .. })
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
            .view(
                &address(
                    coordinate.clone(),
                    "note.txt",
                    source.as_bytes(),
                    target,
                    0,
                    source.len(),
                ),
                target,
            )
            .unwrap();
        let text = match outcome {
            ViewOutcome::Projected { content, .. } => content,
            ViewOutcome::RelationAbsent => panic!("self projection exists"),
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
        runtime_with_admission(&root, "docs").view(&input, input.target()),
        Ok(ViewOutcome::Projected { content, .. }) if content == "one"
    ));
}

#[test]
fn host_view_reuses_matching_search_proof_and_falls_back_after_a_miss_or_invalidation() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = b"zero\n\none\r\ntwo\n\nlast";
    fs::write(root.join("note.txt"), source).unwrap();
    let coordinate = coordinate(&runtime(&root));
    let line = address(
        coordinate.clone(),
        "note.txt",
        source,
        AnddressTarget::Line,
        6,
        11,
    );
    let separator = address(
        coordinate.clone(),
        "note.txt",
        source,
        AnddressTarget::Line,
        5,
        6,
    );
    let mut host = host_runtime(&root);

    assert!(matches!(
        host.view(&line, line.target()),
        Ok(ViewOutcome::Projected {
            anddress,
            ref content,
        }) if content == "one\r\n"
            && anddress.terminator() == Some(LineTerminator::Crlf)
    ));

    let request = SearchRequest::new(
        SearchQuery::new("one").unwrap(),
        SearchScope::all_admitted(),
        SearchTarget::Line,
    );
    let SearchOutcome::Found { anddresses } = host.search(&request).unwrap() else {
        panic!("matching Line")
    };
    assert_eq!(anddresses[0], line);
    assert_eq!(anddresses.len(), 1);
    assert_eq!(
        host.view(&separator, AnddressTarget::Paragraph),
        Ok(ViewOutcome::RelationAbsent)
    );
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
        6,
        15,
    );

    assert!(matches!(
        host.view(&file, file.target()),
        Ok(ViewOutcome::Projected { ref content, .. }) if content.as_bytes() == source
    ));
    assert!(matches!(
        host.view(&paragraph, paragraph.target()),
        Ok(ViewOutcome::Projected { ref content, .. }) if content == "one\r\ntwo\n"
    ));
    assert!(matches!(
        host.view(&line, line.target()),
        Ok(ViewOutcome::Projected { anddress, .. })
            if anddress.project(AnddressTarget::Paragraph).unwrap().as_ref()
                == Some(&paragraph)
    ));
    assert_eq!(
        host.view(&line, AnddressTarget::Paragraph),
        Ok(projected(paragraph.clone(), b"one\r\ntwo\n"))
    );
    assert_eq!(
        host.view(&line, AnddressTarget::File),
        Ok(projected(file.clone(), source))
    );
    assert_eq!(
        host.view(&paragraph, AnddressTarget::File),
        Ok(projected(file.clone(), source))
    );

    let stale = support::file(file.workspace_coordinate(), "note.txt", b"stale\n");
    let parked = root.join("parked-note");
    fs::rename(root.join("note.txt"), &parked).unwrap();
    assert_eq!(
        host.view(&stale, stale.target()),
        Err(ViewError::Unavailable)
    );
    assert_eq!(host.check(file.clone()).unwrap().filtered, Some(file));
    fs::rename(&parked, root.join("note.txt")).unwrap();

    host.invalidate_source("note.txt").unwrap();
    assert!(matches!(
        host.view(&line, line.target()),
        Ok(ViewOutcome::Projected { ref content, .. }) if content == "one\r\n"
    ));

    let untrusted = runtime(&root);
    let SearchOutcome::Found { anddresses } = untrusted.search(&request).unwrap() else {
        panic!("untrusted Line")
    };
    assert!(matches!(
        untrusted.view(&anddresses[0], anddresses[0].target()),
        Ok(ViewOutcome::Projected { ref content, .. }) if content == "one\r\n"
    ));
}

#[test]
fn host_view_open_and_short_failures_remove_only_proof_and_preserve_anchor() {
    for short in [false, true] {
        let fixture = tempdir().unwrap();
        let root = fixture.path().join("workspace");
        fs::create_dir(&root).unwrap();
        let source_path = root.join("note.txt");
        fs::write(&source_path, b"current\n").unwrap();
        let mut host = host_runtime(&root);
        let SearchOutcome::Found { mut anddresses } = host
            .search(&SearchRequest::exact_file("note.txt").unwrap())
            .unwrap()
        else {
            panic!("current File")
        };
        let current = anddresses.pop().unwrap();
        let handle = match host.anchor(&current).unwrap() {
            AnchorOutcome::Anchored(handle) => handle,
            AnchorOutcome::AlreadyLive => panic!("File Anchor"),
        };
        let parked = root.join("parked-note");

        // Deliberately violate the Host guard to inject the trusted failure.
        if short {
            fs::write(&source_path, b"short").unwrap();
        } else {
            fs::rename(&source_path, &parked).unwrap();
        }
        assert_eq!(
            host.view(&current, current.target()),
            Err(ViewError::Unavailable)
        );
        if short {
            fs::write(&source_path, b"current\n").unwrap();
        } else {
            fs::rename(&parked, &source_path).unwrap();
        }
        assert!(matches!(
            host.view_anchored(&handle, AnddressTarget::File),
            Ok(ViewOutcome::Projected { content, .. }) if content == "current\n"
        ));

        fs::rename(&source_path, &parked).unwrap();
        assert_eq!(host.check(current.clone()).unwrap().filtered, None);
        fs::rename(&parked, &source_path).unwrap();
    }
}
