mod support;

use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use backwriter::backwriter::anddress::{Anddress, AnddressTarget as PublicAnddressTarget};
use backwriter::backwriter::apply::ApplyError;
use backwriter::backwriter::edit::{Edit, Position};
use backwriter::backwriter::search::{
    SearchOutcome, SearchQuery, SearchRequest, SearchScope, SearchTarget,
};
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

#[derive(Clone)]
struct TestCoordinate {
    value: String,
    root: std::path::PathBuf,
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

fn coordinate(workspace: &WorkspaceRuntime) -> TestCoordinate {
    let request = SearchRequest::new(
        SearchQuery::new("coordinate").unwrap(),
        SearchScope::all_admitted(),
        SearchTarget::File,
    );
    match workspace.search(&request).unwrap() {
        SearchOutcome::Found { anddresses } => TestCoordinate {
            value: anddresses[0].workspace_coordinate().to_owned(),
            root: workspace.workspace_root().to_owned(),
        },
        SearchOutcome::Empty => panic!("coordinate source"),
    }
}

fn address(coordinate: TestCoordinate, path: &str, target: AnddressTarget) -> Anddress {
    let source = fs::read(coordinate.root.join(path)).unwrap_or_default();
    match target {
        AnddressTarget::File => support::file(&coordinate.value, path, &source),
        AnddressTarget::Paragraph { ordinal } => {
            let paragraphs = paragraph_ranges(&source);
            let (start, end) = paragraphs.get(ordinal.0).copied().unwrap_or((0, 0));
            support::address(
                &coordinate.value,
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
                    &coordinate.value,
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
                    &coordinate.value,
                    path,
                    &stale_source,
                    PublicAnddressTarget::Line,
                    0,
                    stale_source.len() - 1,
                )
            }
        }
    }
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

fn assert_no_apply_temp(directory: &std::path::Path) {
    assert!(fs::read_dir(directory).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".env.artext-apply-")
    }));
}

#[test]
fn apply_has_one_edit_seam_and_one_source_observation() {
    let runtime = include_str!("../src/runtime.rs");
    let apply = include_str!("../src/runtime/apply.rs");

    assert_eq!(runtime.matches("pub fn apply(").count(), 1);
    assert!(runtime.contains("edit: &Edit"));
    assert!(!runtime.contains("apply_edit"));
    assert!(!runtime.contains("apply_anchored"));
    assert_eq!(apply.matches(".open_admitted_source(").count(), 1);
    assert_eq!(apply.matches("observe_source(source").count(), 1);
    assert_eq!(apply.matches("stage_source(&mut source").count(), 1);
    assert!(apply.contains("staging.write(bytes)"));
    assert!(apply.contains("staging.open_read()?"));
    for forbidden in [
        "ApplyStream",
        "move_identity",
        "LegacyResolver",
        "DecimalOrdinal",
        "ExactTargetTracker",
        "SourceEvent",
        "SourceFramer",
        "scan_source(",
    ] {
        assert!(!apply.contains(forbidden), "retired {forbidden}");
    }
    let execute = apply
        .split_once("pub(super) fn execute")
        .map(|(_, execute)| execute)
        .expect("Apply execution");
    let staging_close = execute.find("staging.close()?;").expect("staging close");
    let empty_insert = execute
        .find("if geometry.direct_noop(edit)")
        .expect("direct no-op branch");
    let comparison = execute.find("let comparison =").expect("comparison");
    let after = execute
        .find("let temporary = Temporary::create(")
        .expect("after temporary");
    assert!(staging_close < empty_insert);
    assert!(empty_insert < comparison);
    assert!(empty_insert < after);
}

#[test]
fn apply_inserts_at_each_exact_position_without_normalization() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    fs::write(root.join("note.txt"), "one\r\ntwo\n\nthree").unwrap();
    let line = |ordinal: &str, extent: &str| {
        address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Line {
                ordinal: Natural::parse(ordinal).unwrap(),
                exact_extent: extent.to_owned(),
            },
        )
    };
    let paragraph = |ordinal: &str| {
        address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Paragraph {
                ordinal: Natural::parse(ordinal).unwrap(),
            },
        )
    };

    for (position, expected) in [
        (
            Position::Before(line("0", "one\r\n")),
            "Xone\r\ntwo\n\nthree",
        ),
        (
            Position::After(line("0", "one\r\n")),
            "one\r\nXtwo\n\nthree",
        ),
        (Position::Before(paragraph("1")), "one\r\ntwo\n\nXthree"),
        (Position::After(paragraph("0")), "one\r\ntwo\nX\nthree"),
    ] {
        fs::write(root.join("note.txt"), "one\r\ntwo\n\nthree").unwrap();
        workspace
            .apply(&Edit::Insert {
                position,
                content: "X".to_owned(),
            })
            .unwrap();
        assert_eq!(fs::read_to_string(root.join("note.txt")).unwrap(), expected);
    }
    fs::write(root.join("note.txt"), "one\rtwo").unwrap();
    workspace
        .apply(&Edit::Insert {
            position: Position::After(line("0", "one\r")),
            content: "X".to_owned(),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "one\rXtwo"
    );
    assert_no_apply_temp(&root);
}

#[test]
fn exact_file_lookup_enables_start_and_end_insert_into_empty_files() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("start.txt"), "").unwrap();
    fs::write(root.join("end.txt"), "").unwrap();
    let mut workspace = runtime(&root);

    for (path, position) in [
        ("start.txt", Position::StartOf as fn(Anddress) -> Position),
        ("end.txt", Position::EndOf as fn(Anddress) -> Position),
    ] {
        let SearchOutcome::Found { mut anddresses } = workspace
            .search(&SearchRequest::exact_file(path).unwrap())
            .unwrap()
        else {
            panic!("empty File lookup")
        };
        assert_eq!(anddresses.len(), 1);
        workspace
            .apply(&Edit::Insert {
                position: position(anddresses.pop().unwrap()),
                content: "hello".to_owned(),
            })
            .unwrap();
        assert_eq!(fs::read(root.join(path)).unwrap(), b"hello");
    }
}

#[test]
fn v4_duplicate_line_drift_fails_without_wrong_publication() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let original = b"header\nneedle\nneedle\nfooter\n";
    fs::write(root.join("note.txt"), original).unwrap();
    let mut workspace = runtime(&root);
    let request = SearchRequest::new(
        SearchQuery::new("needle").unwrap(),
        SearchScope::all_admitted(),
        SearchTarget::Line,
    );
    let SearchOutcome::Found { anddresses } = workspace.search(&request).unwrap() else {
        panic!("duplicate lines")
    };
    let selected = anddresses[1].clone();
    let changed = b"needle\nheader\nneedle\nneedle\nfooter\n";
    fs::write(root.join("note.txt"), changed).unwrap();

    assert_eq!(
        workspace.apply(&Edit::Replace {
            target: selected,
            content: "TARGET\n".to_owned(),
        }),
        Err(ApplyError::Unavailable)
    );
    assert_eq!(fs::read(root.join("note.txt")).unwrap(), changed);
    assert_no_apply_temp(&root);
}

#[test]
fn v4_duplicate_paragraph_drift_fails_without_wrong_publication() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let original = b"header\n\nneedle\n\nneedle\n\nfooter\n";
    fs::write(root.join("note.txt"), original).unwrap();
    let mut workspace = runtime(&root);
    let request = SearchRequest::new(
        SearchQuery::new("needle").unwrap(),
        SearchScope::all_admitted(),
        SearchTarget::Paragraph,
    );
    let SearchOutcome::Found { anddresses } = workspace.search(&request).unwrap() else {
        panic!("duplicate paragraphs")
    };
    let selected = anddresses[1].clone();
    let changed = b"needle\n\nheader\n\nneedle\n\nneedle\n\nfooter\n";
    fs::write(root.join("note.txt"), changed).unwrap();

    assert_eq!(
        workspace.apply(&Edit::Replace {
            target: selected,
            content: "TARGET\n".to_owned(),
        }),
        Err(ApplyError::Unavailable)
    );
    assert_eq!(fs::read(root.join("note.txt")).unwrap(), changed);
    assert_no_apply_temp(&root);
}

#[cfg(unix)]
#[test]
fn changed_apply_preserves_the_source_basic_mode() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let mut workspace = runtime(&root);

    for (mode, content) in [(0o600, "private"), (0o755, "executable")] {
        let source = root.join("note.txt");
        fs::write(&source, "before").unwrap();
        let file = address(coordinate(&workspace), "note.txt", AnddressTarget::File);
        fs::set_permissions(&source, fs::Permissions::from_mode(mode)).unwrap();
        assert_eq!(
            fs::metadata(&source).unwrap().permissions().mode() & 0o777,
            mode
        );

        workspace
            .apply(&Edit::Replace {
                target: file.clone(),
                content: content.to_owned(),
            })
            .unwrap();

        assert_eq!(fs::read_to_string(&source).unwrap(), content);
        assert_eq!(
            fs::metadata(&source).unwrap().permissions().mode() & 0o777,
            mode
        );
        assert_no_apply_temp(&root);
    }
}

#[test]
fn apply_replaces_deletes_moves_and_copies_from_one_logical_source() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let file = || address(coordinate.clone(), "note.txt", AnddressTarget::File);
    let paragraph = |ordinal: &str| {
        address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Paragraph {
                ordinal: Natural::parse(ordinal).unwrap(),
            },
        )
    };
    let line = |ordinal: &str, extent: &str| {
        address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Line {
                ordinal: Natural::parse(ordinal).unwrap(),
                exact_extent: extent.to_owned(),
            },
        )
    };

    fs::write(root.join("note.txt"), "old").unwrap();
    workspace
        .apply(&Edit::Replace {
            target: file(),
            content: "new\n".to_owned(),
        })
        .unwrap();
    assert_eq!(fs::read_to_string(root.join("note.txt")).unwrap(), "new\n");

    fs::write(root.join("note.txt"), "one\ntwo\n\nthree\n").unwrap();
    workspace
        .apply(&Edit::Replace {
            target: paragraph("0"),
            content: "alpha\n".to_owned(),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "alpha\n\nthree\n"
    );

    fs::write(root.join("note.txt"), "one\ntwo\nthree\n").unwrap();
    workspace
        .apply(&Edit::Delete {
            target: line("1", "two\n"),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "one\nthree\n"
    );

    fs::write(root.join("note.txt"), "a\nb\nc\n").unwrap();
    workspace
        .apply(&Edit::Move {
            target: line("1", "b\n"),
            position: Position::After(line("2", "c\n")),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "a\nc\nb\n"
    );

    fs::write(root.join("note.txt"), "a\nb\nc\n").unwrap();
    workspace
        .apply(&Edit::Copy {
            target: line("2", "c\n"),
            position: Position::Before(line("0", "a\n")),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "c\na\nb\nc\n"
    );
    assert_no_apply_temp(&root);
}

#[test]
fn apply_replace_preserves_exact_terminators_and_scratch_boundaries() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);

    for (ordinal, extent, content, expected) in [
        ("0", "a\n", "A", "Ab\rc\r\nd"),
        ("1", "b\r", "", "a\nc\r\nd"),
        ("2", "c\r\n", "C\n", "a\nb\rC\nd"),
        ("3", "d", "끝", "a\nb\rc\r\n끝"),
    ] {
        fs::write(root.join("note.txt"), "a\nb\rc\r\nd").unwrap();
        workspace
            .apply(&Edit::Replace {
                target: address(
                    coordinate.clone(),
                    "note.txt",
                    AnddressTarget::Line {
                        ordinal: Natural::parse(ordinal).unwrap(),
                        exact_extent: extent.to_owned(),
                    },
                ),
                content: content.to_owned(),
            })
            .unwrap();
        assert_eq!(fs::read_to_string(root.join("note.txt")).unwrap(), expected);
    }

    for boundary in [8_191, 8_192, 8_193] {
        let prefix = format!("{}한", "x".repeat(boundary));
        let extent = "old\r\n".to_owned();
        fs::write(root.join("note.txt"), format!("{prefix}\n{extent}tail\n")).unwrap();
        workspace
            .apply(&Edit::Replace {
                target: address(
                    coordinate.clone(),
                    "note.txt",
                    AnddressTarget::Line {
                        ordinal: Natural::one(),
                        exact_extent: extent,
                    },
                ),
                content: "new\r\n".to_owned(),
            })
            .unwrap();
        assert_eq!(
            fs::read_to_string(root.join("note.txt")).unwrap(),
            format!("{prefix}\nnew\r\ntail\n")
        );
    }
    assert_no_apply_temp(&root);
}

#[test]
fn apply_rejects_late_invalid_source_before_publication_or_empty_insert() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let file = address(coordinate.clone(), "note.txt", AnddressTarget::File);

    for invalid_tail in [b"\xff".as_slice(), b"\xe2".as_slice(), b"\0".as_slice()] {
        let mut source = vec![b'a'; 8_192];
        source.extend_from_slice(invalid_tail);
        fs::write(root.join("note.txt"), &source).unwrap();
        assert_eq!(
            workspace.apply(&Edit::Replace {
                target: file.clone(),
                content: "published".to_owned(),
            }),
            Err(ApplyError::Unavailable)
        );
        assert_eq!(
            workspace.apply(&Edit::Insert {
                position: Position::EndOf(file.clone()),
                content: String::new(),
            }),
            Err(ApplyError::Unavailable)
        );
        assert_eq!(fs::read(root.join("note.txt")).unwrap(), source);
        assert_no_apply_temp(&root);
    }
}

#[test]
fn apply_replace_preserves_logical_path_and_rejects_unavailable_sources() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir_all(root.join(".artext/bw")).unwrap();
    fs::create_dir(root.join("directory")).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    fs::write(root.join("a.txt"), "linked").unwrap();
    fs::hard_link(root.join("a.txt"), root.join("b.txt")).unwrap();
    fs::write(root.join(".artext/bw/private.txt"), "private").unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);

    workspace
        .apply(&Edit::Replace {
            target: address(coordinate.clone(), "a.txt", AnddressTarget::File),
            content: "replaced".to_owned(),
        })
        .unwrap();
    assert_eq!(fs::read_to_string(root.join("a.txt")).unwrap(), "replaced");
    assert_eq!(fs::read_to_string(root.join("b.txt")).unwrap(), "linked");

    for path in [".artext/bw/private.txt", "missing.txt", "directory"] {
        assert_eq!(
            workspace.apply(&Edit::Replace {
                target: address(coordinate.clone(), path, AnddressTarget::File),
                content: "new".to_owned(),
            }),
            Err(ApplyError::Unavailable)
        );
    }
    assert_no_apply_temp(&root);
}

#[cfg(unix)]
#[test]
fn apply_replace_rejects_a_replaced_symlink_without_publication() {
    use std::os::unix::fs::symlink;

    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let source = root.join("note.txt");
    fs::write(&source, "one").unwrap();
    let mut workspace = runtime(&root);
    let input = address(coordinate(&workspace), "note.txt", AnddressTarget::File);
    let outside = fixture.path().join("outside.txt");
    fs::write(&outside, "outside").unwrap();
    fs::remove_file(&source).unwrap();
    symlink(&outside, &source).unwrap();

    assert_eq!(
        workspace.apply(&Edit::Replace {
            target: input,
            content: "new".to_owned(),
        }),
        Err(ApplyError::Unavailable)
    );
    assert_eq!(fs::read_to_string(&outside).unwrap(), "outside");
}

#[test]
fn apply_rejects_cross_source_move_and_preserves_strict_interior_move() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    fs::write(root.join("note.txt"), "one\ntwo\n\n").unwrap();
    fs::write(root.join("other.txt"), "other\n").unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let paragraph = address(
        coordinate.clone(),
        "note.txt",
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let first_line = address(
        coordinate.clone(),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let other = address(
        coordinate,
        "other.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "other\n".to_owned(),
        },
    );
    assert_eq!(
        workspace.apply(&Edit::Move {
            target: paragraph.clone(),
            position: Position::After(first_line),
        }),
        Err(ApplyError::InvalidInput)
    );
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "one\ntwo\n\n"
    );
    assert_eq!(
        workspace.apply(&Edit::Move {
            target: paragraph,
            position: Position::Before(other),
        }),
        Err(ApplyError::InvalidInput)
    );
    assert_no_apply_temp(&root);
}

#[test]
fn apply_keeps_noops_unpublished_and_allows_copy_at_its_own_boundary() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    fs::write(root.join("note.txt"), "a\nb\nc\n").unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let file = address(coordinate.clone(), "note.txt", AnddressTarget::File);
    let line = |ordinal: &str, extent: &str| {
        address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Line {
                ordinal: Natural::parse(ordinal).unwrap(),
                exact_extent: extent.to_owned(),
            },
        )
    };
    let paragraph = |ordinal: &str| {
        address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Paragraph {
                ordinal: Natural::parse(ordinal).unwrap(),
            },
        )
    };

    workspace
        .apply(&Edit::Insert {
            position: Position::EndOf(file),
            content: String::new(),
        })
        .unwrap();
    workspace
        .apply(&Edit::Move {
            target: line("1", "b\n"),
            position: Position::Before(line("1", "b\n")),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "a\nb\nc\n"
    );
    assert_no_apply_temp(&root);

    for position in [
        Position::After(line("1", "b\n")),
        Position::After(line("0", "a\n")),
        Position::Before(line("2", "c\n")),
    ] {
        workspace
            .apply(&Edit::Move {
                target: line("1", "b\n"),
                position,
            })
            .unwrap();
    }
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "a\nb\nc\n"
    );
    assert_no_apply_temp(&root);

    fs::write(root.join("note.txt"), "a\nb\n\nc\nd\n").unwrap();
    for position in [
        Position::Before(paragraph("0")),
        Position::After(paragraph("0")),
    ] {
        workspace
            .apply(&Edit::Move {
                target: paragraph("0"),
                position,
            })
            .unwrap();
    }
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "a\nb\n\nc\nd\n"
    );
    assert_no_apply_temp(&root);

    fs::write(root.join("note.txt"), "a\nb\nc\n").unwrap();
    workspace
        .apply(&Edit::Copy {
            target: line("1", "b\n"),
            position: Position::After(line("1", "b\n")),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "a\nb\nb\nc\n"
    );
    assert_no_apply_temp(&root);
}

#[test]
fn apply_exact_file_replace_skips_publication_after_the_scratch_boundary() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let exact = format!("{}끝\r\n", "x".repeat(8_193));
    fs::write(root.join("note.txt"), &exact).unwrap();
    fs::hard_link(root.join("note.txt"), root.join("linked.txt")).unwrap();
    let mut workspace = runtime(&root);
    let target = address(coordinate(&workspace), "note.txt", AnddressTarget::File);

    workspace
        .apply(&Edit::Replace {
            target,
            content: exact.clone(),
        })
        .unwrap();

    assert_eq!(fs::read(root.join("note.txt")).unwrap(), exact.as_bytes());
    fs::write(root.join("linked.txt"), "still-linked").unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "still-linked"
    );
    assert_no_apply_temp(&root);
}

#[test]
fn apply_keeps_horizontal_whitespace_outside_paragraphs() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let line = |ordinal: &str, extent: &str| {
        address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Line {
                ordinal: Natural::parse(ordinal).unwrap(),
                exact_extent: extent.to_owned(),
            },
        )
    };
    let paragraph = |ordinal: &str| {
        address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Paragraph {
                ordinal: Natural::parse(ordinal).unwrap(),
            },
        )
    };

    fs::write(root.join("note.txt"), "a\n \t\nb\n").unwrap();
    workspace
        .apply(&Edit::Insert {
            position: Position::Before(paragraph("1")),
            content: "X".to_owned(),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "a\n \t\nXb\n"
    );

    fs::write(root.join("note.txt"), "a\n \t\nb\n").unwrap();
    workspace
        .apply(&Edit::Replace {
            target: paragraph("0"),
            content: "X\n".to_owned(),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "X\n \t\nb\n"
    );

    fs::write(root.join("note.txt"), "a\n \t\nb\n").unwrap();
    workspace
        .apply(&Edit::Delete {
            target: paragraph("0"),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        " \t\nb\n"
    );

    fs::write(root.join("note.txt"), "a\n \t\nb\n").unwrap();
    workspace
        .apply(&Edit::Move {
            target: line("0", "a\n"),
            position: Position::Before(paragraph("1")),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        " \t\na\nb\n"
    );
    assert_no_apply_temp(&root);
}

#[test]
fn apply_batches_long_whitespace_outside_paragraph_candidates() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let file = || address(coordinate.clone(), "note.txt", AnddressTarget::File);
    let paragraph = |ordinal: &str| {
        address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Paragraph {
                ordinal: Natural::parse(ordinal).unwrap(),
            },
        )
    };
    let line = |ordinal: &str, extent: String| {
        address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Line {
                ordinal: Natural::parse(ordinal).unwrap(),
                exact_extent: extent,
            },
        )
    };
    let whitespace = " ".repeat(8_192 * 3 + 1);

    fs::write(root.join("note.txt"), &whitespace).unwrap();
    workspace
        .apply(&Edit::Insert {
            position: Position::EndOf(file()),
            content: String::new(),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        whitespace
    );
    assert_no_apply_temp(&root);

    fs::write(root.join("note.txt"), &whitespace).unwrap();
    workspace
        .apply(&Edit::Replace {
            target: file(),
            content: "replacement".to_owned(),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "replacement"
    );
    assert_no_apply_temp(&root);

    let source = format!("a\n{whitespace}\nb\n");
    let separator = format!("{whitespace}\n");
    fs::write(root.join("note.txt"), &source).unwrap();
    workspace
        .apply(&Edit::Insert {
            position: Position::Before(paragraph("1")),
            content: "X".to_owned(),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        format!("a\n{separator}Xb\n")
    );

    fs::write(root.join("note.txt"), &source).unwrap();
    workspace
        .apply(&Edit::Move {
            target: paragraph("0"),
            position: Position::Before(paragraph("1")),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        format!("{separator}a\nb\n")
    );

    fs::write(root.join("note.txt"), &source).unwrap();
    workspace
        .apply(&Edit::Copy {
            target: paragraph("0"),
            position: Position::Before(paragraph("1")),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        format!("a\n{separator}a\nb\n")
    );

    fs::write(root.join("note.txt"), &source).unwrap();
    let separator_line = line("1", separator.clone());
    let first_line = line("0", "a\n".to_owned());
    workspace
        .apply(&Edit::Move {
            target: separator_line.clone(),
            position: Position::Before(first_line.clone()),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        format!("{separator}a\nb\n")
    );

    fs::write(root.join("note.txt"), &source).unwrap();
    workspace
        .apply(&Edit::Copy {
            target: separator_line,
            position: Position::Before(first_line),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        format!("{separator}a\n{separator}b\n")
    );
    assert_no_apply_temp(&root);
}

#[test]
fn apply_scans_the_complete_source_once_before_any_publication() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    fs::write(root.join("note.txt"), "first\nsecond").unwrap();
    let mut workspace = runtime(&root);
    let input = address(
        coordinate(&workspace),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "first\n".to_owned(),
        },
    );
    fs::write(root.join("note.txt"), "first\nsecond\0").unwrap();

    assert_eq!(
        workspace.apply(&Edit::Delete { target: input }),
        Err(ApplyError::Unavailable)
    );
    assert_eq!(fs::read(root.join("note.txt")).unwrap(), b"first\nsecond\0");
    assert_no_apply_temp(&root);
}

#[test]
fn apply_splices_large_single_lines_with_fixed_staging_scratch() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let mut source = "x".repeat(8_192 * 3 + 17);
    source.push('\n');
    fs::write(root.join("note.txt"), &source).unwrap();
    let mut workspace = runtime(&root);
    let input = address(
        coordinate(&workspace),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: source.clone(),
        },
    );

    workspace
        .apply(&Edit::Insert {
            position: Position::Before(input),
            content: "prefix".to_owned(),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        format!("prefix{source}")
    );
    assert_no_apply_temp(&root);
}

#[test]
fn apply_covers_line_and_paragraph_replace_delete_move_and_copy_directions() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let line = |ordinal: &str, extent: &str| {
        address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Line {
                ordinal: Natural::parse(ordinal).unwrap(),
                exact_extent: extent.to_owned(),
            },
        )
    };
    let paragraph = |ordinal: &str| {
        address(
            coordinate.clone(),
            "note.txt",
            AnddressTarget::Paragraph {
                ordinal: Natural::parse(ordinal).unwrap(),
            },
        )
    };

    fs::write(root.join("note.txt"), "a\nb\n\nc\nd\n").unwrap();
    workspace
        .apply(&Edit::Replace {
            target: line("1", "b\n"),
            content: "B\n".to_owned(),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "a\nB\n\nc\nd\n"
    );

    fs::write(root.join("note.txt"), "a\nb\n\nc\nd\n").unwrap();
    workspace
        .apply(&Edit::Delete {
            target: paragraph("0"),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "\nc\nd\n"
    );

    fs::write(root.join("note.txt"), "a\nb\nc\n").unwrap();
    workspace
        .apply(&Edit::Move {
            target: line("2", "c\n"),
            position: Position::Before(line("0", "a\n")),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "c\na\nb\n"
    );

    fs::write(root.join("note.txt"), "a\nb\nc\n").unwrap();
    workspace
        .apply(&Edit::Copy {
            target: line("1", "b\n"),
            position: Position::After(line("2", "c\n")),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "a\nb\nc\nb\n"
    );

    fs::write(root.join("note.txt"), "a\n\nb\n").unwrap();
    workspace
        .apply(&Edit::Move {
            target: paragraph("1"),
            position: Position::Before(paragraph("0")),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "b\na\n\n"
    );

    fs::write(root.join("note.txt"), "a\n\nb\n").unwrap();
    workspace
        .apply(&Edit::Copy {
            target: paragraph("0"),
            position: Position::After(paragraph("1")),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "a\n\nb\na\n"
    );

    fs::write(root.join("note.txt"), "a\n\nb\n").unwrap();
    workspace
        .apply(&Edit::Copy {
            target: paragraph("1"),
            position: Position::Before(paragraph("0")),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(root.join("note.txt")).unwrap(),
        "b\na\n\nb\n"
    );
    assert_no_apply_temp(&root);
}

#[test]
fn apply_rejects_hard_linked_logical_paths_and_invalid_late_utf8() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    fs::write(root.join("a.txt"), "a\n").unwrap();
    fs::hard_link(root.join("a.txt"), root.join("b.txt")).unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let source = |path: &str| {
        address(
            coordinate.clone(),
            path,
            AnddressTarget::Line {
                ordinal: Natural::zero(),
                exact_extent: "a\n".to_owned(),
            },
        )
    };
    assert_eq!(
        workspace.apply(&Edit::Copy {
            target: source("a.txt"),
            position: Position::Before(source("b.txt")),
        }),
        Err(ApplyError::InvalidInput)
    );
    assert_eq!(fs::read_to_string(root.join("a.txt")).unwrap(), "a\n");

    fs::write(root.join("note.txt"), "first\nsecond").unwrap();
    let input = address(
        coordinate.clone(),
        "note.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "first\n".to_owned(),
        },
    );
    fs::write(root.join("note.txt"), b"first\nsecond\xff").unwrap();
    assert_eq!(
        workspace.apply(&Edit::Delete { target: input }),
        Err(ApplyError::Unavailable)
    );
    assert_eq!(
        fs::read(root.join("note.txt")).unwrap(),
        b"first\nsecond\xff"
    );
    assert_no_apply_temp(&root);
}

#[test]
fn v3_apply_operand_is_rejected_before_runtime_access() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let _workspace = runtime(&root);
    assert_eq!(
        Anddress::decode(
            br#"{"version":"artext.backwriter-anddress.v3","workspaceCoordinate":"0","logicalPath":"missing.txt","kind":"line","ordinal":"0","exactExtent":"missing\\n"}"#,
        ),
        Err(backwriter::backwriter::anddress::AnddressError::UnsupportedVersion)
    );
}

#[test]
fn apply_uses_raw_v4_ranges_without_structural_relocation() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let source = "αβ\n".as_bytes();
    fs::write(root.join("note.txt"), source).unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let raw = support::address(
        &coordinate.value,
        "note.txt",
        source,
        PublicAnddressTarget::Line,
        2,
        4,
    );

    workspace
        .apply(&Edit::Replace {
            target: raw,
            content: "γ".to_owned(),
        })
        .unwrap();

    assert_eq!(fs::read(root.join("note.txt")).unwrap(), "αγ\n".as_bytes());
    assert_no_apply_temp(&root);
}

#[test]
fn apply_zero_ranges_are_noops_except_nonempty_replace_insertion() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let source = b"abc";
    fs::write(root.join("note.txt"), source).unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let raw = || {
        support::address(
            &coordinate.value,
            "note.txt",
            source,
            PublicAnddressTarget::Line,
            1,
            1,
        )
    };
    let position = || Position::Before(raw());

    for edit in [
        Edit::Delete { target: raw() },
        Edit::Copy {
            target: raw(),
            position: position(),
        },
        Edit::Move {
            target: raw(),
            position: position(),
        },
        Edit::Replace {
            target: raw(),
            content: String::new(),
        },
    ] {
        workspace.apply(&edit).unwrap();
        assert_eq!(fs::read(root.join("note.txt")).unwrap(), source);
    }

    workspace
        .apply(&Edit::Replace {
            target: raw(),
            content: "X".to_owned(),
        })
        .unwrap();
    assert_eq!(fs::read(root.join("note.txt")).unwrap(), b"aXbc");
    assert_no_apply_temp(&root);
}

#[test]
fn apply_same_path_state_mismatch_is_unavailable_without_publication() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let source = b"abc\n";
    fs::write(root.join("note.txt"), source).unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let current = support::address(
        &coordinate.value,
        "note.txt",
        source,
        PublicAnddressTarget::Line,
        0,
        1,
    );
    let stale_bytes = b"axc\n";
    let stale = support::address(
        &coordinate.value,
        "note.txt",
        stale_bytes,
        PublicAnddressTarget::Line,
        2,
        3,
    );

    assert_eq!(
        workspace.apply(&Edit::Copy {
            target: current,
            position: Position::Before(stale),
        }),
        Err(ApplyError::Unavailable)
    );
    assert_eq!(fs::read(root.join("note.txt")).unwrap(), source);
    assert_no_apply_temp(&root);
}

#[test]
fn apply_rejects_a_raw_range_that_would_split_utf8() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("coordinate.txt"), "coordinate").unwrap();
    let source = "é\n".as_bytes();
    fs::write(root.join("note.txt"), source).unwrap();
    let mut workspace = runtime(&root);
    let coordinate = coordinate(&workspace);
    let raw = support::address(
        &coordinate.value,
        "note.txt",
        source,
        PublicAnddressTarget::Line,
        1,
        2,
    );

    assert_eq!(
        workspace.apply(&Edit::Delete { target: raw }),
        Err(ApplyError::Unavailable)
    );
    assert_eq!(fs::read(root.join("note.txt")).unwrap(), source);
    assert_no_apply_temp(&root);
}
