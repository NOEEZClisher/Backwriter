use std::fs;

use artext::backwriter::anchor::AnchorOutcome;
use artext::backwriter::anddress::{ANDDRESS_VERSION, Anddress, AnddressTarget, Natural};
use artext::backwriter::check::CheckError;
use artext::backwriter::pick::PickOutcome;
use artext::backwriter::search::SearchOutcome;
use artext::runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime};
use tempfile::tempdir;

fn runtime(root: &std::path::Path, admission: &str) -> WorkspaceRuntime {
    WorkspaceRuntime::open(
        root,
        WorkspaceAdmission::new([AdmissionRoot::new(admission).unwrap()]).unwrap(),
    )
    .unwrap()
}

fn coordinate(runtime: &WorkspaceRuntime) -> String {
    let request = artext::backwriter::search::SearchRequest::new(
        artext::backwriter::search::SearchQuery::new("seed").unwrap(),
        artext::backwriter::search::SearchScope::all_admitted(),
        artext::backwriter::search::SearchTarget::File,
    );
    let SearchOutcome::Found { anddresses } = runtime.search(&request).unwrap() else {
        panic!("source fixture supplies a coordinate");
    };
    anddresses[0].workspace_coordinate.clone()
}

fn address(coordinate: &str, path: &str, target: AnddressTarget) -> Anddress {
    Anddress {
        version: ANDDRESS_VERSION.to_owned(),
        workspace_coordinate: coordinate.to_owned(),
        logical_path: path.to_owned(),
        target,
    }
}

#[test]
fn check_reports_current_removed_and_unavailable_raw_addresses() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), "seed\n").unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    fs::write(root.join("invalid.txt"), b"\xff").unwrap();
    fs::write(root.join("zero.txt"), b"seed\0").unwrap();

    let current = address(&coordinate, "seed.txt", AnddressTarget::File);
    let result = workspace.check(current.clone()).unwrap();
    assert_eq!(result.filtered, Some(current));
    assert_eq!(result.report.current_count(), 1);
    assert_eq!(result.report.checked_count(), 1);

    let stale = address(
        &coordinate,
        "seed.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "other\n".to_owned(),
        },
    );
    let result = workspace.check(stale.clone()).unwrap();
    assert_eq!(result.filtered, None);
    assert_eq!(result.report.removed(), &[stale]);

    let unavailable = address(&coordinate, "invalid.txt", AnddressTarget::File);
    let result = workspace.check(unavailable.clone()).unwrap();
    assert_eq!(result.filtered, Some(unavailable.clone()));
    assert_eq!(result.report.unavailable(), &[unavailable]);

    let unavailable = address(&coordinate, "zero.txt", AnddressTarget::File);
    let result = workspace.check(unavailable.clone()).unwrap();
    assert_eq!(result.filtered, Some(unavailable.clone()));
    assert_eq!(result.report.unavailable(), &[unavailable]);
}

#[test]
fn check_search_and_pick_preserve_order_multiplicity_and_canonical_empty() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), "seed\n").unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    fs::write(root.join("invalid.txt"), b"\xff").unwrap();
    let current = address(&coordinate, "seed.txt", AnddressTarget::File);
    let stale = address(
        &coordinate,
        "seed.txt",
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "missing".to_owned(),
        },
    );
    let unavailable = address(&coordinate, "invalid.txt", AnddressTarget::File);
    let candidates = vec![
        current.clone(),
        stale.clone(),
        unavailable.clone(),
        current.clone(),
    ];

    let checked = workspace
        .check_search(SearchOutcome::Found {
            anddresses: candidates.clone(),
        })
        .unwrap();
    assert_eq!(
        checked.filtered,
        SearchOutcome::Found {
            anddresses: vec![current.clone(), unavailable.clone(), current.clone()]
        }
    );
    assert_eq!(checked.report.current_count(), 2);
    assert_eq!(checked.report.removed(), std::slice::from_ref(&stale));
    assert_eq!(
        checked.report.unavailable(),
        std::slice::from_ref(&unavailable)
    );

    let checked = workspace
        .check_pick(PickOutcome::Selected {
            anddresses: candidates,
        })
        .unwrap();
    assert_eq!(
        checked.filtered,
        PickOutcome::Selected {
            anddresses: vec![current.clone(), unavailable, current]
        }
    );
    assert_eq!(checked.report.removed(), &[stale]);
    let empty = workspace
        .check_search(SearchOutcome::Found {
            anddresses: Vec::new(),
        })
        .unwrap();
    assert_eq!(empty.filtered, SearchOutcome::Empty);
    assert_eq!(empty.report.checked_count(), 0);
    let empty = workspace
        .check_pick(PickOutcome::Selected {
            anddresses: Vec::new(),
        })
        .unwrap();
    assert_eq!(empty.filtered, PickOutcome::Empty);
    assert_eq!(empty.report.checked_count(), 0);
}

#[test]
fn check_matches_digit_boundary_line_and_paragraph_ordinals_without_reordering() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), "seed\n").unwrap();
    fs::write(root.join("note.txt"), "x\n\n".repeat(101)).unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    let line_99 = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::parse("99").unwrap(),
            exact_extent: "\n".to_owned(),
        },
    );
    let line_100 = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::parse("100").unwrap(),
            exact_extent: "x\n".to_owned(),
        },
    );
    let paragraph_99 = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Paragraph {
            ordinal: Natural::parse("99").unwrap(),
        },
    );
    let paragraph_100 = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Paragraph {
            ordinal: Natural::parse("100").unwrap(),
        },
    );
    let wrong = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::parse("100").unwrap(),
            exact_extent: "wrong\n".to_owned(),
        },
    );
    let checked = workspace
        .check_search(SearchOutcome::Found {
            anddresses: vec![
                paragraph_100.clone(),
                line_100.clone(),
                wrong.clone(),
                paragraph_99.clone(),
                line_99.clone(),
                line_100.clone(),
            ],
        })
        .unwrap();
    assert_eq!(
        checked.filtered,
        SearchOutcome::Found {
            anddresses: vec![
                paragraph_100,
                line_100.clone(),
                paragraph_99,
                line_99,
                line_100,
            ]
        }
    );
    assert_eq!(checked.report.removed(), &[wrong]);
}

#[test]
fn check_validates_every_occurrence_before_runtime_access() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), "seed\n").unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    let unsupported = Anddress {
        version: "other".to_owned(),
        workspace_coordinate: coordinate.clone(),
        logical_path: "missing.txt".to_owned(),
        target: AnddressTarget::File,
    };
    assert_eq!(
        workspace.check(unsupported),
        Err(CheckError::UnsupportedVersion)
    );
    let invalid = Anddress {
        version: ANDDRESS_VERSION.to_owned(),
        workspace_coordinate: coordinate.clone(),
        logical_path: "missing\0.txt".to_owned(),
        target: AnddressTarget::File,
    };
    assert_eq!(workspace.check(invalid), Err(CheckError::InvalidInput));
    let invalid = Anddress {
        version: ANDDRESS_VERSION.to_owned(),
        workspace_coordinate: coordinate.clone(),
        logical_path: "missing\0.txt".to_owned(),
        target: AnddressTarget::File,
    };
    let unsupported = Anddress {
        version: "other".to_owned(),
        workspace_coordinate: coordinate.clone(),
        logical_path: "missing.txt".to_owned(),
        target: AnddressTarget::File,
    };
    for (anddresses, error) in [
        (
            vec![invalid.clone(), unsupported.clone()],
            CheckError::InvalidInput,
        ),
        (vec![unsupported, invalid], CheckError::UnsupportedVersion),
    ] {
        assert_eq!(
            workspace.check_search(SearchOutcome::Found { anddresses }),
            Err(error)
        );
    }
}

#[test]
fn check_reports_interleaved_duplicate_occurrences_in_input_order() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), "seed\n").unwrap();
    fs::write(root.join("a-current.txt"), "current a\n").unwrap();
    fs::write(root.join("b-removed.txt"), "removed b\n").unwrap();
    fs::write(root.join("d-current.txt"), "current d\n").unwrap();
    fs::write(root.join("e-removed.txt"), "removed e\n").unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    fs::write(root.join("c-unavailable.txt"), b"\xff").unwrap();
    fs::write(root.join("f-unavailable.txt"), b"\0").unwrap();
    let current_a = address(&coordinate, "a-current.txt", AnddressTarget::File);
    let current_d = address(&coordinate, "d-current.txt", AnddressTarget::File);
    let removed_b = address(
        &coordinate,
        "b-removed.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "stale b\n".to_owned(),
        },
    );
    let removed_e = address(
        &coordinate,
        "e-removed.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "stale e\n".to_owned(),
        },
    );
    let unavailable_c = address(&coordinate, "c-unavailable.txt", AnddressTarget::File);
    let unavailable_f = address(&coordinate, "f-unavailable.txt", AnddressTarget::File);
    let inputs = vec![
        unavailable_f.clone(),
        removed_e.clone(),
        current_d.clone(),
        unavailable_c.clone(),
        removed_b.clone(),
        current_a.clone(),
        unavailable_f.clone(),
        removed_b.clone(),
    ];
    let checked = workspace
        .check_search(SearchOutcome::Found { anddresses: inputs })
        .unwrap();
    assert_eq!(
        checked.filtered,
        SearchOutcome::Found {
            anddresses: vec![
                unavailable_f.clone(),
                current_d.clone(),
                unavailable_c.clone(),
                current_a.clone(),
                unavailable_f.clone(),
            ]
        }
    );
    assert_eq!(checked.report.current_count(), 2);
    assert_eq!(checked.report.removed_count(), 3);
    assert_eq!(checked.report.unavailable_count(), 3);
    assert_eq!(checked.report.checked_count(), 8);
    assert_eq!(
        checked.report.removed(),
        &[removed_e, removed_b.clone(), removed_b]
    );
    assert_eq!(
        checked.report.unavailable(),
        &[unavailable_f.clone(), unavailable_c, unavailable_f]
    );

    let empty_search = workspace
        .check_search(SearchOutcome::Found {
            anddresses: vec![address(&coordinate, "missing.txt", AnddressTarget::File)],
        })
        .unwrap();
    assert_eq!(empty_search.filtered, SearchOutcome::Empty);
    let empty_pick = workspace
        .check_pick(PickOutcome::Selected {
            anddresses: vec![address(&coordinate, "missing.txt", AnddressTarget::File)],
        })
        .unwrap();
    assert_eq!(empty_pick.filtered, PickOutcome::Empty);
}

#[test]
fn check_tracks_unsorted_structural_duplicates_without_changing_input_order() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), "seed\n").unwrap();
    fs::write(
        root.join("note.txt"),
        "\n \t\r\n\nzero\r\none\rtwo\n\n\nthree",
    )
    .unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    let file = address(&coordinate, "note.txt", AnddressTarget::File);
    let paragraph_zero = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let paragraph_one = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Paragraph {
            ordinal: Natural::one(),
        },
    );
    let missing_paragraph = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Paragraph {
            ordinal: Natural::parse("2").unwrap(),
        },
    );
    let zero = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::parse("3").unwrap(),
            exact_extent: "zero\r\n".to_owned(),
        },
    );
    let one = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::parse("4").unwrap(),
            exact_extent: "one\r".to_owned(),
        },
    );
    let wrong_one = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::parse("4").unwrap(),
            exact_extent: "wrong\r".to_owned(),
        },
    );
    let two = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::parse("5").unwrap(),
            exact_extent: "two\n".to_owned(),
        },
    );
    let separator = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "\n".to_owned(),
        },
    );
    let missing_line = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::parse(&format!("1{}", "0".repeat(4097))).unwrap(),
            exact_extent: "three".to_owned(),
        },
    );
    let inputs = vec![
        missing_paragraph.clone(),
        one.clone(),
        file.clone(),
        wrong_one.clone(),
        paragraph_one.clone(),
        zero.clone(),
        one.clone(),
        paragraph_zero.clone(),
        missing_line.clone(),
        paragraph_zero.clone(),
        two.clone(),
        separator.clone(),
    ];

    let checked = workspace
        .check_search(SearchOutcome::Found { anddresses: inputs })
        .unwrap();

    assert_eq!(
        checked.filtered,
        SearchOutcome::Found {
            anddresses: vec![
                one.clone(),
                file,
                paragraph_one,
                zero,
                one,
                paragraph_zero.clone(),
                paragraph_zero,
                two,
                separator,
            ]
        }
    );
    assert_eq!(
        checked.report.removed(),
        &[missing_paragraph, wrong_one, missing_line]
    );
    assert_eq!(checked.report.current_count(), 9);
    assert_eq!(checked.report.removed_count(), 3);
    assert_eq!(checked.report.unavailable_count(), 0);
}

#[test]
fn check_uses_exact_line_and_paragraph_structure_for_all_terminators() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), "seed\n").unwrap();
    fs::write(root.join("note.txt"), "one\r\n\t \ntwo\rthree").unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    for (ordinal, extent) in [
        ("0", "one\r\n"),
        ("1", "\t \n"),
        ("2", "two\r"),
        ("3", "three"),
    ] {
        assert!(
            workspace
                .check(address(
                    &coordinate,
                    "note.txt",
                    AnddressTarget::Line {
                        ordinal: Natural::parse(ordinal).unwrap(),
                        exact_extent: extent.to_owned(),
                    },
                ))
                .unwrap()
                .filtered
                .is_some()
        );
    }
    assert!(
        workspace
            .check(address(
                &coordinate,
                "note.txt",
                AnddressTarget::Paragraph {
                    ordinal: Natural::zero(),
                },
            ))
            .unwrap()
            .filtered
            .is_some()
    );
    assert!(
        workspace
            .check(address(
                &coordinate,
                "note.txt",
                AnddressTarget::Paragraph {
                    ordinal: Natural::one(),
                },
            ))
            .unwrap()
            .filtered
            .is_some()
    );
    assert!(
        workspace
            .check(address(
                &coordinate,
                "note.txt",
                AnddressTarget::Line {
                    ordinal: Natural::parse(&format!("1{}", "0".repeat(4097))).unwrap(),
                    exact_extent: "three".to_owned(),
                },
            ))
            .unwrap()
            .filtered
            .is_none()
    );
    fs::write(root.join("large.txt"), "seed\n".repeat(4098)).unwrap();
    assert!(
        workspace
            .check(address(
                &coordinate,
                "large.txt",
                AnddressTarget::Line {
                    ordinal: Natural::parse("4097").unwrap(),
                    exact_extent: "seed\n".to_owned(),
                },
            ))
            .unwrap()
            .filtered
            .is_some()
    );
}

#[test]
fn check_late_invalid_groups_preserve_batch_order_and_anchor_state() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), "seed\n").unwrap();
    fs::write(root.join("invalid-tail.txt"), "one\n").unwrap();
    fs::write(root.join("nul-tail.txt"), "one\n").unwrap();
    let mut workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    let anchored = address(&coordinate, "invalid-tail.txt", AnddressTarget::File);
    let AnchorOutcome::Anchored(handle) = workspace.anchor(&anchored).unwrap() else {
        panic!("new anchor");
    };

    let mut invalid_tail = b"one\n".to_vec();
    invalid_tail.extend(std::iter::repeat_n(b'x', 16_384));
    invalid_tail.push(0xff);
    fs::write(root.join("invalid-tail.txt"), invalid_tail).unwrap();
    let mut nul_tail = b"one\n".to_vec();
    nul_tail.extend(std::iter::repeat_n(b'y', 16_384));
    nul_tail.push(0);
    fs::write(root.join("nul-tail.txt"), nul_tail).unwrap();

    let invalid_file = address(&coordinate, "invalid-tail.txt", AnddressTarget::File);
    let invalid_paragraph = address(
        &coordinate,
        "invalid-tail.txt",
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let nul_file = address(&coordinate, "nul-tail.txt", AnddressTarget::File);
    let nul_line = address(
        &coordinate,
        "nul-tail.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let current = address(&coordinate, "seed.txt", AnddressTarget::File);
    let stale = address(
        &coordinate,
        "seed.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "other\n".to_owned(),
        },
    );
    let inputs = vec![
        invalid_file.clone(),
        stale.clone(),
        nul_line.clone(),
        invalid_paragraph.clone(),
        current.clone(),
        nul_file.clone(),
        invalid_file.clone(),
    ];

    let checked = workspace
        .check_pick(PickOutcome::Selected { anddresses: inputs })
        .unwrap();

    assert_eq!(
        checked.filtered,
        PickOutcome::Selected {
            anddresses: vec![
                invalid_file.clone(),
                nul_line.clone(),
                invalid_paragraph.clone(),
                current,
                nul_file.clone(),
                invalid_file.clone(),
            ]
        }
    );
    assert_eq!(checked.report.current_count(), 1);
    assert_eq!(checked.report.removed(), &[stale]);
    assert_eq!(
        checked.report.unavailable(),
        &[
            invalid_file.clone(),
            nul_line,
            invalid_paragraph,
            nul_file,
            invalid_file,
        ]
    );

    fs::write(root.join("invalid-tail.txt"), "restored\n").unwrap();
    assert!(workspace.view_anchored(&handle).is_ok());
}

#[test]
fn check_keeps_file_and_paragraph_current_but_removes_stale_line_and_spill() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), "seed\n").unwrap();
    fs::write(root.join("note.txt"), "before\n\nnext\n").unwrap();
    fs::create_dir_all(root.join(".artext/bw")).unwrap();
    fs::write(root.join(".artext/bw/private.txt"), "private\n").unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    let file = address(&coordinate, "note.txt", AnddressTarget::File);
    let paragraph = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let line = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "before\n".to_owned(),
        },
    );
    fs::write(root.join("note.txt"), "after\n\nnext\n").unwrap();
    for input in [file, paragraph] {
        assert!(workspace.check(input).unwrap().filtered.is_some());
    }
    assert!(workspace.check(line).unwrap().filtered.is_none());
    for input in [
        address(&"0".repeat(64), "note.txt", AnddressTarget::File),
        address(&coordinate, ".artext/bw/private.txt", AnddressTarget::File),
        address(
            &coordinate,
            "note.txt",
            AnddressTarget::Paragraph {
                ordinal: Natural::parse(&format!("1{}", "0".repeat(4097))).unwrap(),
            },
        ),
    ] {
        assert!(workspace.check(input).unwrap().filtered.is_none());
    }
}

#[test]
fn check_treats_unadmitted_missing_and_directory_sources_as_not_current() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::create_dir(root.join("admitted")).unwrap();
    fs::write(root.join("admitted/seed.txt"), "seed\n").unwrap();
    fs::create_dir(root.join("admitted/directory.txt")).unwrap();
    let workspace = runtime(&root, "admitted");
    let coordinate = coordinate(&workspace);
    for path in [
        "admitted/missing.txt",
        "other.txt",
        "admitted/directory.txt",
    ] {
        let result = workspace
            .check(address(&coordinate, path, AnddressTarget::File))
            .unwrap();
        assert!(result.filtered.is_none());
        assert_eq!(result.report.removed_count(), 1);
    }
}

#[cfg(unix)]
#[test]
fn check_treats_symlink_sources_as_not_current() {
    use std::os::unix::fs::symlink;

    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), "seed\n").unwrap();
    fs::write(root.join("outside.txt"), "seed\n").unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    symlink(root.join("outside.txt"), root.join("link.txt")).unwrap();
    let result = workspace
        .check(address(&coordinate, "link.txt", AnddressTarget::File))
        .unwrap();
    assert!(result.filtered.is_none());
}

#[test]
fn check_keeps_hard_link_paths_and_anchor_state_independent() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), "seed\n").unwrap();
    fs::write(root.join("left.txt"), "one\n").unwrap();
    fs::hard_link(root.join("left.txt"), root.join("right.txt")).unwrap();
    let mut workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    let left = address(
        &coordinate,
        "left.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let right = address(&coordinate, "right.txt", left.target.clone());
    let anchored = address(&coordinate, "left.txt", AnddressTarget::File);
    let AnchorOutcome::Anchored(handle) = workspace.anchor(&anchored).unwrap() else {
        panic!("new anchor");
    };
    fs::remove_file(root.join("left.txt")).unwrap();
    fs::write(root.join("left.txt"), "changed\n").unwrap();
    let checked = workspace
        .check_search(SearchOutcome::Found {
            anddresses: vec![left.clone(), right.clone()],
        })
        .unwrap();
    assert_eq!(
        checked.filtered,
        SearchOutcome::Found {
            anddresses: vec![right]
        }
    );
    assert_eq!(checked.report.removed(), &[left]);
    assert!(workspace.view_anchored(&handle).is_ok());
}

#[test]
fn check_is_a_stateless_current_lookup_across_source_recovery() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), "seed\n").unwrap();
    fs::write(root.join("note.txt"), "one\n").unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    let input = address(
        &coordinate,
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    assert!(workspace.check(input.clone()).unwrap().filtered.is_some());
    fs::write(root.join("note.txt"), "two\n").unwrap();
    assert!(workspace.check(input.clone()).unwrap().filtered.is_none());
    fs::write(root.join("note.txt"), "one\n").unwrap();
    assert!(workspace.check(input).unwrap().filtered.is_some());
}
