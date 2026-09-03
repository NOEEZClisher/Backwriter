mod support;

use std::fs;
#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, PermissionsExt};

use backwriter::backwriter::anchor::AnchorOutcome;
use backwriter::backwriter::anddress::{Anddress, AnddressTarget as PublicAnddressTarget};
use backwriter::backwriter::apply::{ApplyError, EditReceipt};
use backwriter::backwriter::edit::{Edit, Position};
use backwriter::backwriter::search::{
    SearchOutcome, SearchQuery, SearchRequest, SearchScope, SearchTarget,
};
use backwriter::backwriter::view::ViewOutcome;
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

fn exact_file(runtime: &WorkspaceRuntime, path: &str) -> Anddress {
    match runtime
        .search(&SearchRequest::exact_file(path).unwrap())
        .unwrap()
    {
        SearchOutcome::Found { occurrences } => {
            occurrences.into_iter().next().unwrap().into_anddress()
        }
        SearchOutcome::Empty => panic!("exact File"),
    }
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
        SearchOutcome::Found { occurrences } => TestCoordinate {
            value: occurrences[0].anddress().workspace_coordinate().to_owned(),
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
    let production = apply.split("#[cfg(test)]").next().unwrap();

    assert_eq!(runtime.matches("pub fn apply(").count(), 1);
    assert_eq!(runtime.matches("pub fn apply_replace(").count(), 1);
    assert!(runtime.contains("edit: &Edit"));
    assert!(!runtime.contains("apply_edit"));
    assert!(!runtime.contains("apply_anchored"));
    assert_eq!(production.matches(".open_admitted_source(").count(), 1);
    assert_eq!(production.matches("pub(super) fn execute(").count(), 1);
    assert_eq!(production.matches("AnddressIssuer::new(").count(), 1);
    assert_eq!(production.matches("observe_source(source").count(), 1);
    assert_eq!(production.matches("stage_source(&mut source").count(), 1);
    let trusted = apply
        .split_once("fn stage_source_trusted")
        .map(|(_, trusted)| trusted)
        .unwrap()
        .split_once("pub(super) fn execute")
        .map(|(trusted, _)| trusted)
        .unwrap();
    assert!(trusted.contains("validate_source_exact"));
    assert!(!trusted.contains("observe_source"));
    assert!(!trusted.contains("ObservationBuilder"));
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
        .expect("Apply execution")
        .split_once("fn map_edit_error")
        .map(|(execute, _)| execute)
        .expect("Apply execution end");
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
    let validation = execute.find("edit.validate()").unwrap();
    let proof_selection = execute.find("select_current_proof").unwrap();
    let source_open = execute.find("open_admitted_source").unwrap();
    let prepared_proof = execute.find("prepare_current_proof_installation").unwrap();
    let receipt = execute.find("changed_receipt(").unwrap();
    let reflection = execute.find("reflection_plan(").unwrap();
    let publication = execute.rfind("publish(").unwrap();
    assert!(validation < proof_selection);
    assert!(proof_selection < source_open);
    assert!(receipt < reflection);
    assert!(reflection < prepared_proof);
    assert!(prepared_proof < publication);
    assert!(publication < execute.rfind("Ok(receipt)").unwrap());
    assert!(!execute[..validation].contains("invalidate_current_proof"));
    assert!(!execute.contains("current_proofs.lock"));
    assert!(!execute.contains("search("));

    let source_scan = include_str!("../src/runtime/source_scan.rs");
    let exact_validation = source_scan
        .split_once("pub(crate) fn validate_source_exact")
        .map(|(_, exact)| exact)
        .unwrap();
    assert!(!exact_validation.contains("Sha256"));
}

#[test]
fn host_apply_reuses_and_replaces_current_proof_across_publications() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), b"one\n").unwrap();
    fs::write(root.join("other.txt"), b"other\n").unwrap();
    let mut workspace = host_runtime(&root);
    let first = exact_file(&workspace, "note.txt");
    let other = exact_file(&workspace, "other.txt");
    let handle = match workspace.anchor(&first).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("File Anchor"),
    };

    let second = support::file(first.workspace_coordinate(), "note.txt", b"two\n");
    assert_eq!(
        workspace.apply_replace(&Edit::Replace {
            target: first.clone(),
            content: "two\n".to_owned(),
        }),
        Ok(EditReceipt::Changed {
            anddress: Some(second.clone())
        })
    );
    assert_eq!(workspace.check(first.clone()).unwrap().filtered, None);
    assert_eq!(
        workspace.check(second.clone()).unwrap().filtered,
        Some(second.clone())
    );
    assert!(matches!(
        workspace.view(&second, second.target()),
        Ok(ViewOutcome::File { text, .. }) if text == "two\n"
    ));
    let parked_other = root.join("parked-other");
    fs::rename(root.join("other.txt"), &parked_other).unwrap();
    assert_eq!(
        workspace.check(other.clone()).unwrap().filtered,
        Some(other)
    );
    fs::rename(&parked_other, root.join("other.txt")).unwrap();

    let parked = root.join("parked");
    fs::rename(root.join("note.txt"), &parked).unwrap();
    assert_eq!(
        workspace.check(second.clone()).unwrap().filtered,
        Some(second.clone())
    );
    fs::rename(&parked, root.join("note.txt")).unwrap();

    let third = support::file(first.workspace_coordinate(), "note.txt", b"three\n");
    assert_eq!(
        workspace.apply_replace(&Edit::Replace {
            target: second.clone(),
            content: "three\n".to_owned(),
        }),
        Ok(EditReceipt::Changed {
            anddress: Some(third.clone())
        })
    );
    assert_eq!(fs::read(root.join("note.txt")).unwrap(), b"three\n");
    assert_eq!(workspace.check(second.clone()).unwrap().filtered, None);
    assert_eq!(
        workspace.check(third.clone()).unwrap().filtered,
        Some(third)
    );
    assert_eq!(
        workspace.apply(&Edit::Replace {
            target: first,
            content: "wrong\n".to_owned(),
        }),
        Err(ApplyError::Unavailable)
    );
    assert!(matches!(
        workspace.view_anchored(&handle, PublicAnddressTarget::File),
        Ok(ViewOutcome::File { text, .. }) if text == "three\n"
    ));
    assert_no_apply_temp(&root);
}

#[cfg(unix)]
#[test]
fn host_apply_direct_and_identical_noops_preserve_proof_anchor_inode_and_bytes() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source_path = root.join("note.txt");
    fs::write(&source_path, b"same\n").unwrap();
    let mut workspace = host_runtime(&root);
    let file = exact_file(&workspace, "note.txt");
    let handle = match workspace.anchor(&file).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("File Anchor"),
    };
    let inode = fs::metadata(&source_path).unwrap().ino();

    let empty_range = support::address(
        file.workspace_coordinate(),
        "note.txt",
        b"same\n",
        PublicAnddressTarget::Line,
        0,
        0,
    );
    assert_eq!(
        workspace.apply_replace(&Edit::Replace {
            target: empty_range.clone(),
            content: String::new(),
        }),
        Ok(EditReceipt::Unchanged {
            anddress: empty_range
        })
    );

    workspace
        .apply(&Edit::Insert {
            position: Position::StartOf(file.clone()),
            content: String::new(),
        })
        .unwrap();
    assert_eq!(
        workspace.apply_replace(&Edit::Replace {
            target: file.clone(),
            content: "same\n".to_owned(),
        }),
        Ok(EditReceipt::Unchanged {
            anddress: file.clone()
        })
    );

    assert_eq!(fs::read(&source_path).unwrap(), b"same\n");
    assert_eq!(fs::metadata(&source_path).unwrap().ino(), inode);
    assert!(matches!(
        workspace.view_anchored(&handle, PublicAnddressTarget::File),
        Ok(ViewOutcome::File { text, .. }) if text == "same\n"
    ));
    let parked = root.join("parked");
    fs::rename(&source_path, &parked).unwrap();
    assert_eq!(workspace.check(file.clone()).unwrap().filtered, Some(file));
    fs::rename(&parked, &source_path).unwrap();
    assert_no_apply_temp(&root);
}

#[test]
fn apply_replace_returns_exact_line_and_paragraph_results() {
    for (before, content, after) in [
        ("old", "new", "new"),
        ("old", "", ""),
        ("old\n", "\n", "\n"),
        ("old\r", "β\r", "β\r"),
        ("old\r\n", "새 줄\r\n", "새 줄\r\n"),
    ] {
        let fixture = tempdir().unwrap();
        let root = fixture.path();
        let prefix = "prefix\n";
        let before_source = format!("{prefix}{before}");
        let after_source = format!("{prefix}{after}");
        fs::write(root.join("note.txt"), &before_source).unwrap();
        let mut workspace = runtime(root);
        let file = exact_file(&workspace, "note.txt");
        let target = support::address(
            file.workspace_coordinate(),
            "note.txt",
            before_source.as_bytes(),
            PublicAnddressTarget::Line,
            prefix.len(),
            prefix.len() + before.len(),
        );
        let fresh = (!after.is_empty()).then(|| {
            support::address(
                file.workspace_coordinate(),
                "note.txt",
                after_source.as_bytes(),
                PublicAnddressTarget::Line,
                prefix.len(),
                prefix.len() + after.len(),
            )
        });
        assert_eq!(
            workspace.apply_replace(&Edit::Replace {
                target,
                content: content.to_owned(),
            }),
            Ok(EditReceipt::Changed {
                anddress: fresh.clone()
            })
        );
        assert_eq!(
            fs::read(root.join("note.txt")).unwrap(),
            after_source.as_bytes()
        );
        if let Some(fresh) = fresh {
            assert_eq!(
                workspace.check(fresh.clone()).unwrap().filtered,
                Some(fresh.clone())
            );
            assert!(matches!(
                workspace.view(&fresh, PublicAnddressTarget::Line),
                Ok(ViewOutcome::Line { anddress, .. }) if anddress == fresh
            ));
        }
        assert_no_apply_temp(root);
    }

    for (content, after, expected_range) in [
        ("\n", "\n\nkeep\n", None),
        ("new\n", "new\n\nkeep\n", Some((0, 4))),
        ("a\n\nb\n", "a\n\nb\n\nkeep\n", None),
    ] {
        let fixture = tempdir().unwrap();
        let root = fixture.path();
        let before = b"old\n\nkeep\n";
        fs::write(root.join("note.txt"), before).unwrap();
        let mut workspace = runtime(root);
        let file = exact_file(&workspace, "note.txt");
        let target = support::address(
            file.workspace_coordinate(),
            "note.txt",
            before,
            PublicAnddressTarget::Paragraph,
            0,
            4,
        );
        let expected = expected_range.map(|(start, end)| {
            support::address(
                file.workspace_coordinate(),
                "note.txt",
                after.as_bytes(),
                PublicAnddressTarget::Paragraph,
                start,
                end,
            )
        });
        assert_eq!(
            workspace.apply_replace(&Edit::Replace {
                target,
                content: content.to_owned(),
            }),
            Ok(EditReceipt::Changed { anddress: expected })
        );
        assert_eq!(fs::read(root.join("note.txt")).unwrap(), after.as_bytes());
        assert_no_apply_temp(root);
    }
}

#[test]
fn apply_replace_rejects_every_non_replace_before_source_access() {
    let fixture = tempdir().unwrap();
    let root = fixture.path();
    fs::write(root.join("note.txt"), b"line\n").unwrap();
    let mut workspace = runtime(root);
    let file = exact_file(&workspace, "note.txt");
    let line = support::address(
        file.workspace_coordinate(),
        "note.txt",
        b"line\n",
        PublicAnddressTarget::Line,
        0,
        5,
    );
    fs::remove_file(root.join("note.txt")).unwrap();

    for edit in [
        Edit::Insert {
            position: Position::Before(line.clone()),
            content: "x".to_owned(),
        },
        Edit::Delete {
            target: line.clone(),
        },
        Edit::Move {
            target: line.clone(),
            position: Position::After(line.clone()),
        },
        Edit::Copy {
            target: line.clone(),
            position: Position::After(line.clone()),
        },
    ] {
        assert_eq!(
            workspace.apply_replace(&edit),
            Err(ApplyError::InvalidInput)
        );
    }
    assert!(!root.join("note.txt").exists());
    assert_no_apply_temp(root);
}

#[test]
fn host_apply_proof_mismatch_rejects_before_source_io_and_preserves_state() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source_path = root.join("note.txt");
    fs::write(&source_path, b"current\n").unwrap();
    let mut workspace = host_runtime(&root);
    let current = exact_file(&workspace, "note.txt");
    let stale = support::file(current.workspace_coordinate(), "note.txt", b"stale\n");
    let handle = match workspace.anchor(&current).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("File Anchor"),
    };
    let parked = root.join("parked");
    fs::rename(&source_path, &parked).unwrap();

    assert_eq!(
        workspace.apply_replace(&Edit::Replace {
            target: stale,
            content: "wrong\n".to_owned(),
        }),
        Err(ApplyError::Unavailable)
    );

    fs::rename(&parked, &source_path).unwrap();
    assert!(matches!(
        workspace.view_anchored(&handle, PublicAnddressTarget::File),
        Ok(ViewOutcome::File { text, .. }) if text == "current\n"
    ));
    fs::rename(&source_path, &parked).unwrap();
    assert_eq!(
        workspace.check(current.clone()).unwrap().filtered,
        Some(current)
    );
    fs::rename(&parked, &source_path).unwrap();
    assert_no_apply_temp(&root);
}

#[test]
fn host_apply_trusted_short_and_invalid_source_fail_close_proof_and_anchor() {
    for mutated in [
        b"short".as_slice(),
        b"bad\0text".as_slice(),
        b"current\nextra".as_slice(),
    ] {
        let fixture = tempdir().unwrap();
        let root = fixture.path().join("workspace");
        fs::create_dir(&root).unwrap();
        let source_path = root.join("note.txt");
        fs::write(&source_path, b"current\n").unwrap();
        let mut workspace = host_runtime(&root);
        let current = exact_file(&workspace, "note.txt");
        let handle = match workspace.anchor(&current).unwrap() {
            AnchorOutcome::Anchored(handle) => handle,
            AnchorOutcome::AlreadyLive => panic!("File Anchor"),
        };
        fs::write(&source_path, mutated).unwrap();

        assert_eq!(
            workspace.apply_replace(&Edit::Replace {
                target: current.clone(),
                content: "wrong\n".to_owned(),
            }),
            Err(ApplyError::Unavailable)
        );
        assert_eq!(fs::read(&source_path).unwrap(), mutated);
        assert_eq!(
            workspace.view_anchored(&handle, PublicAnddressTarget::File),
            Err(backwriter::backwriter::view::ViewError::Unavailable)
        );
        fs::remove_file(&source_path).unwrap();
        assert_eq!(workspace.check(current).unwrap().filtered, None);
        assert_no_apply_temp(&root);
    }
}

#[test]
fn host_apply_miss_noop_does_not_install_but_changed_publication_does() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source_path = root.join("note.txt");
    fs::write(&source_path, b"before\n").unwrap();
    let coordinate_runtime = runtime(&root);
    let before = exact_file(&coordinate_runtime, "note.txt");
    let coordinate = before.workspace_coordinate().to_owned();
    let mut workspace = host_runtime(&root);

    workspace
        .apply(&Edit::Insert {
            position: Position::StartOf(before.clone()),
            content: String::new(),
        })
        .unwrap();
    let parked = root.join("parked");
    fs::rename(&source_path, &parked).unwrap();
    assert_eq!(workspace.check(before.clone()).unwrap().filtered, None);
    fs::rename(&parked, &source_path).unwrap();

    workspace
        .apply(&Edit::Replace {
            target: before,
            content: "after\n".to_owned(),
        })
        .unwrap();
    let after = support::file(&coordinate, "note.txt", b"after\n");
    fs::rename(&source_path, &parked).unwrap();
    assert_eq!(
        workspace.check(after.clone()).unwrap().filtered,
        Some(after)
    );
    fs::rename(&parked, &source_path).unwrap();
    assert_no_apply_temp(&root);
}

#[test]
fn host_invalidation_before_mutation_safe_rejects_every_stale_consumer() {
    let cases: [(&str, Option<&[u8]>, bool); 5] = [
        ("same length", Some(b"two\n"), false),
        ("different length", Some(b"longer\n"), false),
        ("invalid UTF-8", Some(b"bad\xff"), true),
        ("NUL", Some(b"bad\0"), true),
        ("deleted", None, false),
    ];

    for (name, mutation, unavailable) in cases {
        let fixture = tempdir().unwrap();
        let root = fixture.path().join("workspace");
        fs::create_dir(&root).unwrap();
        let source_path = root.join("note.txt");
        fs::write(&source_path, b"one\n").unwrap();
        let mut workspace = host_runtime(&root);
        let stale = exact_file(&workspace, "note.txt");
        let handle = match workspace.anchor(&stale).unwrap() {
            AnchorOutcome::Anchored(handle) => handle,
            AnchorOutcome::AlreadyLive => panic!("{name}: File Anchor"),
        };

        workspace.invalidate_source("note.txt").unwrap();
        match mutation {
            Some(bytes) => fs::write(&source_path, bytes).unwrap(),
            None => fs::remove_file(&source_path).unwrap(),
        }

        assert_eq!(
            workspace.view(&stale, stale.target()),
            Err(backwriter::backwriter::view::ViewError::Unavailable)
        );
        assert_eq!(
            workspace.view_anchored(&handle, PublicAnddressTarget::File),
            Err(backwriter::backwriter::view::ViewError::Unavailable),
            "{name}"
        );
        let checked = workspace.check(stale.clone()).unwrap();
        if unavailable {
            assert_eq!(checked.filtered, Some(stale.clone()), "{name}");
            assert_eq!(
                checked.report.unavailable(),
                std::slice::from_ref(&stale),
                "{name}"
            );
        } else {
            assert_eq!(checked.filtered, None, "{name}");
            assert_eq!(
                checked.report.removed(),
                std::slice::from_ref(&stale),
                "{name}"
            );
        }
        assert_eq!(
            workspace.apply(&Edit::Replace {
                target: stale,
                content: "WRONG\n".to_owned(),
            }),
            Err(ApplyError::Unavailable),
            "{name}"
        );
        match mutation {
            Some(bytes) => assert_eq!(fs::read(&source_path).unwrap(), bytes, "{name}"),
            None => assert!(!source_path.exists(), "{name}"),
        }
        assert_no_apply_temp(&root);
    }
}

#[test]
fn host_apply_then_guarded_invalidation_rejects_the_old_after_proof() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source_path = root.join("note.txt");
    fs::write(&source_path, b"before\n").unwrap();
    let mut workspace = host_runtime(&root);
    let before = exact_file(&workspace, "note.txt");
    let handle = match workspace.anchor(&before).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("before File Anchor"),
    };

    workspace
        .apply(&Edit::Replace {
            target: before.clone(),
            content: "after\n".to_owned(),
        })
        .unwrap();
    let after = support::file(before.workspace_coordinate(), "note.txt", b"after\n");
    assert_eq!(
        workspace.check(after.clone()).unwrap().filtered,
        Some(after.clone())
    );

    workspace.invalidate_source("note.txt").unwrap();
    fs::write(&source_path, b"external\n").unwrap();
    assert_eq!(
        workspace.apply(&Edit::Replace {
            target: after.clone(),
            content: "WRONG\n".to_owned(),
        }),
        Err(ApplyError::Unavailable)
    );
    assert_eq!(fs::read(&source_path).unwrap(), b"external\n");
    assert_eq!(workspace.check(after.clone()).unwrap().filtered, None);
    assert_eq!(
        workspace.view_anchored(&handle, PublicAnddressTarget::File),
        Err(backwriter::backwriter::view::ViewError::Unavailable)
    );
    assert_no_apply_temp(&root);
}

#[test]
fn host_apply_open_failure_removes_only_proof_and_preserves_anchor() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source_path = root.join("note.txt");
    fs::write(&source_path, b"current\n").unwrap();
    let mut workspace = host_runtime(&root);
    let current = exact_file(&workspace, "note.txt");
    let handle = match workspace.anchor(&current).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("current File Anchor"),
    };
    let parked = root.join("parked-note");

    // Deliberately violate the Host guard to inject an open failure. Apply
    // must remove only the proof because no mutation evidence was observed.
    fs::rename(&source_path, &parked).unwrap();
    assert_eq!(
        workspace.apply_replace(&Edit::Replace {
            target: current.clone(),
            content: "wrong\n".to_owned(),
        }),
        Err(ApplyError::Unavailable)
    );
    assert_no_apply_temp(&root);
    fs::rename(&parked, &source_path).unwrap();
    assert!(matches!(
        workspace.view_anchored(&handle, PublicAnddressTarget::File),
        Ok(ViewOutcome::File { text, .. }) if text == "current\n"
    ));

    fs::rename(&source_path, &parked).unwrap();
    assert_eq!(workspace.check(current.clone()).unwrap().filtered, None);
    fs::rename(&parked, &source_path).unwrap();
    assert_no_apply_temp(&root);
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
        let SearchOutcome::Found { mut occurrences } = workspace
            .search(&SearchRequest::exact_file(path).unwrap())
            .unwrap()
        else {
            panic!("empty File lookup")
        };
        assert_eq!(occurrences.len(), 1);
        workspace
            .apply(&Edit::Insert {
                position: position(occurrences.pop().unwrap().into_anddress()),
                content: "hello".to_owned(),
            })
            .unwrap();
        assert_eq!(fs::read(root.join(path)).unwrap(), b"hello");
    }
}

#[test]
fn v5_drift_matrix_has_one_correct_apply_and_no_wrong_publication_in_both_modes() {
    const ORIGINAL: &[u8] = b"header\nneedle\nneedle\nfooter\n";
    const CORRECT: &[u8] = b"header\nneedle\nTARGET\nfooter\n";
    let cells: [(&str, &[u8], bool); 7] = [
        ("no drift", ORIGINAL, true),
        (
            "edit before target",
            b"expanded-header\nneedle\nneedle\nfooter\n",
            false,
        ),
        (
            "edit after target",
            b"header\nneedle\nneedle\nfooter changed\n",
            false,
        ),
        (
            "adjacent similar context",
            b"header\ncontext\nneedle\nfooter\n",
            false,
        ),
        (
            "target changed",
            b"header\nneedle\nchanged\nfooter\n",
            false,
        ),
        (
            "equal text inserted at another range",
            b"needle\nheader\nneedle\nneedle\nfooter\n",
            false,
        ),
        ("target deleted", b"header\nneedle\nfooter\n", false),
    ];

    for host_mode in [false, true] {
        let mode = if host_mode { "Host" } else { "Untrusted" };
        let mut correct = 0;
        let mut safe_reject = 0;
        let mut wrong = 0;
        for (name, before_apply, succeeds) in cells {
            let fixture = tempdir().unwrap();
            let root = fixture.path().join("workspace");
            fs::create_dir(&root).unwrap();
            fs::write(root.join("note.txt"), ORIGINAL).unwrap();
            let mut workspace = if host_mode {
                host_runtime(&root)
            } else {
                runtime(&root)
            };
            let request = SearchRequest::new(
                SearchQuery::new("needle").unwrap(),
                SearchScope::all_admitted(),
                SearchTarget::Line,
            );
            let SearchOutcome::Found { occurrences } = workspace.search(&request).unwrap() else {
                panic!("{mode} {name}: duplicate lines")
            };
            let selected = occurrences[1].anddress().clone();
            if !succeeds {
                if host_mode {
                    workspace.invalidate_source("note.txt").unwrap();
                }
                fs::write(root.join("note.txt"), before_apply).unwrap();
            }

            let file = support::file(selected.workspace_coordinate(), "note.txt", before_apply);
            let handle = match workspace.anchor(&file).unwrap() {
                AnchorOutcome::Anchored(handle) => handle,
                AnchorOutcome::AlreadyLive => panic!("{mode} {name}: fresh File anchor"),
            };
            let result = workspace.apply(&Edit::Replace {
                target: selected,
                content: "TARGET\n".to_owned(),
            });
            let published = fs::read(root.join("note.txt")).unwrap();

            if succeeds && result == Ok(()) && published == CORRECT {
                correct += 1;
            } else if !succeeds
                && result == Err(ApplyError::Unavailable)
                && published == before_apply
            {
                safe_reject += 1;
            } else {
                wrong += 1;
            }

            if succeeds {
                assert!(
                    matches!(workspace.view_anchored(&handle, PublicAnddressTarget::File), Ok(ViewOutcome::File { text, .. }) if text.as_bytes() == CORRECT),
                    "{mode} {name}"
                );
            } else {
                assert!(
                    matches!(workspace.view_anchored(&handle, PublicAnddressTarget::File), Ok(ViewOutcome::File { text, .. }) if text.as_bytes() == before_apply),
                    "{mode} {name}"
                );
            }
            assert_no_apply_temp(&root);
        }
        assert_eq!((correct, safe_reject, wrong), (1, 6, 0), "{mode}");
    }
}

#[test]
fn v5_duplicate_paragraph_drift_fails_without_wrong_publication_in_both_modes() {
    for host_mode in [false, true] {
        let fixture = tempdir().unwrap();
        let root = fixture.path().join("workspace");
        fs::create_dir(&root).unwrap();
        let original = b"header\n\nneedle\n\nneedle\n\nfooter\n";
        fs::write(root.join("note.txt"), original).unwrap();
        let mut workspace = if host_mode {
            host_runtime(&root)
        } else {
            runtime(&root)
        };
        let request = SearchRequest::new(
            SearchQuery::new("needle").unwrap(),
            SearchScope::all_admitted(),
            SearchTarget::Paragraph,
        );
        let SearchOutcome::Found { occurrences } = workspace.search(&request).unwrap() else {
            panic!("duplicate paragraphs")
        };
        let selected = occurrences[1].anddress().clone();
        let changed = b"needle\n\nheader\n\nneedle\n\nneedle\n\nfooter\n";
        if host_mode {
            workspace.invalidate_source("note.txt").unwrap();
        }
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
            workspace.apply_replace(&Edit::Replace {
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
        .apply_replace(&Edit::Replace {
            target: address(coordinate.clone(), "a.txt", AnddressTarget::File),
            content: "replaced".to_owned(),
        })
        .unwrap();
    assert_eq!(fs::read_to_string(root.join("a.txt")).unwrap(), "replaced");
    assert_eq!(fs::read_to_string(root.join("b.txt")).unwrap(), "linked");

    for path in [".artext/bw/private.txt", "missing.txt", "directory"] {
        assert_eq!(
            workspace.apply_replace(&Edit::Replace {
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
        workspace.apply_replace(&Edit::Replace {
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
fn apply_uses_raw_v5_ranges_without_structural_relocation() {
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
