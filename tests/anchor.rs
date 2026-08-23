use std::fs;

use artext::{
    backwriter::{
        anchor::{AnchorError, AnchorOutcome},
        anddress::{ANDDRESS_VERSION, Anddress, AnddressTarget, Natural},
        apply::ApplyError,
        edit::{Edit, Position},
        view::{ViewError, ViewOutcome},
    },
    runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime},
};
use tempfile::tempdir;

fn runtime(root: &std::path::Path) -> WorkspaceRuntime {
    WorkspaceRuntime::open(
        root,
        WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
    )
    .unwrap()
}

fn address(workspace: &WorkspaceRuntime, target: AnddressTarget) -> Anddress {
    Anddress {
        version: ANDDRESS_VERSION.to_owned(),
        workspace_coordinate: workspace
            .workspace_root()
            .to_string_lossy()
            .len()
            .to_string(),
        logical_path: "note.txt".to_owned(),
        target,
    }
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
        _ => panic!("source"),
    }
}

fn current(workspace: &WorkspaceRuntime, target: AnddressTarget) -> Anddress {
    current_at(workspace, "note.txt", target)
}

fn current_at(
    workspace: &WorkspaceRuntime,
    logical_path: &str,
    target: AnddressTarget,
) -> Anddress {
    let mut value = address(workspace, target);
    value.workspace_coordinate = coordinate(workspace);
    value.logical_path = logical_path.to_owned();
    value
}

#[test]
fn anchor_is_runtime_local_and_drop_allows_a_fresh_handle() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let input = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let handle = match workspace.anchor(&input).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("first handle"),
    };
    assert!(matches!(
        workspace.anchor(&input),
        Ok(AnchorOutcome::AlreadyLive)
    ));
    assert!(matches!(
        workspace.view_anchored(&handle),
        Ok(ViewOutcome::Line { .. })
    ));
    drop(handle);
    assert!(matches!(
        workspace.anchor(&input),
        Ok(AnchorOutcome::Anchored(_))
    ));
}

#[test]
fn raw_anchor_tracks_large_file_and_paragraph_without_view_capture() {
    let fixture = tempdir().unwrap();
    let file_tail = "f".repeat(20_000);
    let paragraph_tail = "p".repeat(20_000);
    fs::write(
        fixture.path().join("note.txt"),
        format!("one{file_tail}\n\nparagraph{paragraph_tail}\n"),
    )
    .unwrap();
    let mut workspace = runtime(fixture.path());
    let file = current(&workspace, AnddressTarget::File);
    let paragraph = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::one(),
        },
    );

    let file_handle = match workspace.anchor(&file).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("file"),
    };
    let paragraph_handle = match workspace.anchor(&paragraph).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("paragraph"),
    };
    assert!(matches!(
        workspace.anchor(&file),
        Ok(AnchorOutcome::AlreadyLive)
    ));
    assert!(
        matches!(workspace.view_anchored(&file_handle), Ok(ViewOutcome::File { text }) if text.len() == 40_015)
    );
    assert!(
        matches!(workspace.view_anchored(&paragraph_handle), Ok(ViewOutcome::Paragraph { text, .. }) if text.len() == 20_010)
    );
}

#[test]
fn raw_anchor_uses_tracker_only_observation() {
    let source = include_str!("../src/runtime/anchor.rs");
    let raw_anchor = source
        .split_once("pub(super) fn anchor")
        .and_then(|(_, source)| source.split_once("pub(super) fn view_anchored"))
        .map(|(raw_anchor, _)| raw_anchor)
        .expect("raw Anchor section");

    assert!(raw_anchor.contains("&inputs, None"));
    assert!(!raw_anchor.contains("ViewCapture"));
    assert!(!raw_anchor.contains("ViewOutcome"));
}

#[test]
fn invalidation_is_path_exact_and_does_not_read_the_source() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\n").unwrap();
    let mut workspace = runtime(fixture.path());
    assert_eq!(
        workspace.invalidate_anchored_source("."),
        Err(AnchorError::InvalidInput)
    );
    assert_eq!(workspace.invalidate_anchored_source("missing.txt"), Ok(()));
    assert_eq!(
        workspace.invalidate_anchored_source(".artext/bw/x"),
        Err(AnchorError::Unavailable)
    );
}

#[test]
fn stale_anchor_input_preserves_current_binding_and_foreign_handle_is_unavailable() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\ntwo\n").unwrap();
    let mut first_runtime = runtime(fixture.path());
    let mut second_runtime = runtime(fixture.path());
    let live = current(
        &first_runtime,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let stale = current(
        &first_runtime,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "stale\n".to_owned(),
        },
    );
    let handle = match first_runtime.anchor(&live).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("live handle"),
    };
    assert!(matches!(
        first_runtime.anchor(&stale),
        Err(AnchorError::Unavailable)
    ));
    assert!(
        matches!(first_runtime.view_anchored(&handle), Ok(ViewOutcome::Line { content, .. }) if content == "one")
    );
    assert_eq!(
        second_runtime.view_anchored(&handle),
        Err(ViewError::Unavailable)
    );
}

#[test]
fn current_anchor_input_fail_closes_a_stale_same_path_binding() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\ntwo\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let stale = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let handle = match workspace.anchor(&stale).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("first handle"),
    };
    fs::write(fixture.path().join("note.txt"), "changed\ntwo\n").unwrap();
    let current_input = address(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "two\n".to_owned(),
        },
    );
    let mut current_input = current_input;
    current_input.workspace_coordinate = stale.workspace_coordinate.clone();

    assert!(matches!(
        workspace.anchor(&current_input),
        Err(AnchorError::Unavailable)
    ));
    assert_eq!(
        workspace.view_anchored(&handle),
        Err(ViewError::Unavailable)
    );
}

#[test]
fn apply_reflects_moved_contained_bindings_without_anchoring_copies() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\ntwo\n\nthree\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let paragraph = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let three = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::one(),
        },
    );
    let one = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let contained = match workspace.anchor(&one).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("first handle"),
    };
    workspace
        .apply(&Edit::Move {
            target: paragraph,
            position: Position::After(three),
        })
        .unwrap();
    assert!(
        matches!(workspace.view_anchored(&contained), Ok(ViewOutcome::Line { content, .. }) if content == "one")
    );
    drop(contained);

    let original = Anddress {
        version: ANDDRESS_VERSION.to_owned(),
        workspace_coordinate: one.workspace_coordinate.clone(),
        logical_path: "note.txt".to_owned(),
        target: AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    };
    fs::write(fixture.path().join("note.txt"), "a\nb\nc\n").unwrap();
    let original_handle = match workspace.anchor(&original).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("original handle"),
    };
    let destination = Anddress {
        version: ANDDRESS_VERSION.to_owned(),
        workspace_coordinate: one.workspace_coordinate.clone(),
        logical_path: "note.txt".to_owned(),
        target: AnddressTarget::Line {
            ordinal: Natural::parse("2").unwrap(),
            exact_extent: "c\n".to_owned(),
        },
    };
    workspace
        .apply(&Edit::Copy {
            target: original,
            position: Position::After(destination),
        })
        .unwrap();
    assert!(
        matches!(workspace.view_anchored(&original_handle), Ok(ViewOutcome::Line { content, .. }) if content == "b")
    );
    let copied = Anddress {
        version: ANDDRESS_VERSION.to_owned(),
        workspace_coordinate: one.workspace_coordinate.clone(),
        logical_path: "note.txt".to_owned(),
        target: AnddressTarget::Line {
            ordinal: Natural::parse("3").unwrap(),
            exact_extent: "b\n".to_owned(),
        },
    };
    assert!(matches!(
        workspace.anchor(&copied),
        Ok(AnchorOutcome::Anchored(_))
    ));
}

#[test]
fn apply_replacement_uses_only_the_source_target_for_containing_provenance() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("coordinate.txt"), "one\n").unwrap();
    fs::write(fixture.path().join("note.txt"), "a\nb\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let paragraph = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let b = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let paragraph_handle = match workspace.anchor(&paragraph).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("paragraph"),
    };
    workspace
        .apply(&Edit::Replace {
            target: b,
            content: "B\n".to_owned(),
        })
        .unwrap();
    assert!(matches!(
        workspace.view_anchored(&paragraph_handle),
        Ok(ViewOutcome::Paragraph { text, .. }) if text == "a\nB\n"
    ));

    let split = Anddress {
        version: ANDDRESS_VERSION.to_owned(),
        workspace_coordinate: paragraph.workspace_coordinate.clone(),
        logical_path: "note.txt".to_owned(),
        target: AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "B\n".to_owned(),
        },
    };
    workspace
        .apply(&Edit::Replace {
            target: split,
            content: "\nx\n".to_owned(),
        })
        .unwrap();
    assert_eq!(
        workspace.view_anchored(&paragraph_handle),
        Err(ViewError::Unavailable)
    );

    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "old\n\n").unwrap();
    fs::write(fixture.path().join("coordinate.txt"), "one\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let paragraph = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let old = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "old\n".to_owned(),
        },
    );
    let handle = match workspace.anchor(&paragraph).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("paragraph"),
    };
    workspace
        .apply(&Edit::Replace {
            target: old,
            content: "\nnew\n".to_owned(),
        })
        .unwrap();
    assert!(matches!(
        workspace.view_anchored(&handle),
        Ok(ViewOutcome::Paragraph { text, .. }) if text == "new\n"
    ));
}

#[test]
fn paragraph_source_membership_keeps_outside_lines_independent() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("coordinate.txt"), "one\n").unwrap();
    fs::write(fixture.path().join("note.txt"), "a\n\nb\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let paragraph = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let selected = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "a\n".to_owned(),
        },
    );
    let outside = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::parse("2").unwrap(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let selected_handle = match workspace.anchor(&selected).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("selected line"),
    };
    let outside_handle = match workspace.anchor(&outside).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("outside line"),
    };

    workspace
        .apply(&Edit::Replace {
            target: paragraph,
            content: "A\n".to_owned(),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
        "A\n\nb\n"
    );
    assert_eq!(
        workspace.view_anchored(&selected_handle),
        Err(ViewError::Unavailable)
    );
    assert!(matches!(
        workspace.view_anchored(&outside_handle),
        Ok(ViewOutcome::Line { content, .. }) if content == "b"
    ));
    assert!(matches!(
        workspace.anchor(&outside),
        Ok(AnchorOutcome::AlreadyLive)
    ));

    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("coordinate.txt"), "one\n").unwrap();
    fs::write(fixture.path().join("note.txt"), "a\n\nb\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let paragraph = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let outside = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::parse("2").unwrap(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let outside_handle = match workspace.anchor(&outside).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("outside line"),
    };
    workspace
        .apply(&Edit::Delete { target: paragraph })
        .unwrap();
    assert_eq!(
        fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
        "\nb\n"
    );
    assert!(matches!(
        workspace.view_anchored(&outside_handle),
        Ok(ViewOutcome::Line { content, .. }) if content == "b"
    ));
    assert!(matches!(
        workspace.anchor(&current(
            &workspace,
            AnddressTarget::Line {
                ordinal: Natural::one(),
                exact_extent: "b\n".to_owned(),
            },
        )),
        Ok(AnchorOutcome::AlreadyLive)
    ));
}

#[test]
fn copy_source_member_rebinds_a_joined_terminal_line_exactly() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("coordinate.txt"), "one\n").unwrap();
    fs::write(fixture.path().join("note.txt"), "a\nc").unwrap();
    let mut workspace = runtime(fixture.path());
    let source = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "c".to_owned(),
        },
    );
    let handle = match workspace.anchor(&source).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("source line"),
    };

    workspace
        .apply(&Edit::Copy {
            target: source.clone(),
            position: Position::Before(source),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
        "a\ncc"
    );
    assert!(matches!(
        workspace.view_anchored(&handle),
        Ok(ViewOutcome::Line { content, terminator, .. })
            if content == "cc" && terminator == artext::backwriter::anddress::LineTerminator::None
    ));
    assert!(matches!(
        workspace.anchor(&current(
            &workspace,
            AnddressTarget::Line {
                ordinal: Natural::one(),
                exact_extent: "cc".to_owned(),
            },
        )),
        Ok(AnchorOutcome::AlreadyLive)
    ));
    assert!(fs::read_dir(fixture.path()).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".env.artext-apply-")
    }));
}

#[test]
fn copy_source_member_rebinds_across_after_planner_source_batch_boundaries() {
    for length in [8_191_usize, 8_192, 8_193] {
        let fixture = tempdir().unwrap();
        let body = format!("one{}", "x".repeat(length - 3));
        fs::write(fixture.path().join("note.txt"), &body).unwrap();
        let mut workspace = runtime(fixture.path());
        let source = current(
            &workspace,
            AnddressTarget::Line {
                ordinal: Natural::zero(),
                exact_extent: body.clone(),
            },
        );
        let handle = match workspace.anchor(&source).unwrap() {
            AnchorOutcome::Anchored(handle) => handle,
            AnchorOutcome::AlreadyLive => panic!("source line"),
        };

        workspace
            .apply(&Edit::Copy {
                target: source.clone(),
                position: Position::Before(source),
            })
            .unwrap();

        let expected = format!("{body}{body}");
        assert_eq!(
            fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
            expected
        );
        assert!(matches!(
            workspace.view_anchored(&handle),
            Ok(ViewOutcome::Line { content, terminator, .. })
                if content == expected && terminator == artext::backwriter::anddress::LineTerminator::None
        ));
        assert!(fs::read_dir(fixture.path()).unwrap().all(|entry| {
            !entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with(".env.artext-apply-")
        }));
    }
}

#[test]
fn apply_file_anchor_replace_preserves_the_handle_without_a_relation_pass() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let file = current(&workspace, AnddressTarget::File);
    let handle = match workspace.anchor(&file).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("file anchor"),
    };

    workspace
        .apply(&Edit::Replace {
            target: file,
            content: "after\n".to_owned(),
        })
        .unwrap();

    assert_eq!(
        fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
        "after\n"
    );
    assert!(matches!(
        workspace.view_anchored(&handle),
        Ok(ViewOutcome::File { text }) if text == "after\n"
    ));
    assert!(fs::read_dir(fixture.path()).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".env.artext-apply-")
    }));
}

#[test]
fn file_replace_preserves_only_the_file_anchor_without_relation_scan() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\ntwo\n\nthree\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let file = current(&workspace, AnddressTarget::File);
    let paragraph = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let line = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "two\n".to_owned(),
        },
    );
    let file_handle = match workspace.anchor(&file).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("file anchor"),
    };
    let paragraph_handle = match workspace.anchor(&paragraph).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("paragraph anchor"),
    };
    let line_handle = match workspace.anchor(&line).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("line anchor"),
    };

    workspace
        .apply(&Edit::Replace {
            target: file,
            content: "one\nthree\n".to_owned(),
        })
        .unwrap();

    assert_eq!(
        fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
        "one\nthree\n"
    );
    assert!(matches!(
        workspace.view_anchored(&file_handle),
        Ok(ViewOutcome::File { text }) if text == "one\nthree\n"
    ));
    assert_eq!(
        workspace.view_anchored(&paragraph_handle),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view_anchored(&line_handle),
        Err(ViewError::Unavailable)
    );
    assert!(fs::read_dir(fixture.path()).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".env.artext-apply-")
    }));
}

#[test]
fn apply_position_is_geometry_and_does_not_grant_provenance() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("coordinate.txt"), "one\n").unwrap();
    fs::write(fixture.path().join("note.txt"), "a\nb\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let paragraph = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let b = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let paragraph_handle = match workspace.anchor(&paragraph).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("paragraph"),
    };
    let b_handle = match workspace.anchor(&b).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("line"),
    };
    workspace
        .apply(&Edit::Insert {
            position: Position::Before(b),
            content: "X\n".to_owned(),
        })
        .unwrap();
    assert_eq!(
        workspace.view_anchored(&paragraph_handle),
        Err(ViewError::Unavailable)
    );
    assert!(matches!(
        workspace.view_anchored(&b_handle),
        Ok(ViewOutcome::Line { content, .. }) if content == "b"
    ));

    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("coordinate.txt"), "one\n").unwrap();
    fs::write(fixture.path().join("note.txt"), "a\nb\nc").unwrap();
    let mut workspace = runtime(fixture.path());
    let b = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let c = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::parse("2").unwrap(),
            exact_extent: "c".to_owned(),
        },
    );
    let b_handle = match workspace.anchor(&b).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("destination"),
    };
    let c_handle = match workspace.anchor(&c).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("source"),
    };
    workspace
        .apply(&Edit::Copy {
            target: c,
            position: Position::Before(b),
        })
        .unwrap();
    assert_eq!(
        workspace.view_anchored(&b_handle),
        Err(ViewError::Unavailable)
    );
    assert!(matches!(
        workspace.view_anchored(&c_handle),
        Ok(ViewOutcome::Line { content, .. }) if content == "c"
    ));
}

#[test]
fn apply_move_and_copy_keep_only_the_normalized_source_relations() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("coordinate.txt"), "one\n").unwrap();
    fs::write(fixture.path().join("note.txt"), "a\nb\nc\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let b = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let c = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::parse("2").unwrap(),
            exact_extent: "c\n".to_owned(),
        },
    );
    let b_handle = match workspace.anchor(&b).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("destination"),
    };
    workspace
        .apply(&Edit::Copy {
            target: c,
            position: Position::Before(b),
        })
        .unwrap();
    assert!(matches!(
        workspace.view_anchored(&b_handle),
        Ok(ViewOutcome::Line { content, .. }) if content == "b"
    ));

    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("coordinate.txt"), "one\n").unwrap();
    fs::write(fixture.path().join("note.txt"), "\na\nb\n\nc\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let destination = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "\n".to_owned(),
        },
    );
    let source = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::one(),
        },
    );
    let c = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::parse("4").unwrap(),
            exact_extent: "c\n".to_owned(),
        },
    );
    let paragraph_handle = match workspace.anchor(&source).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("source paragraph"),
    };
    let line_handle = match workspace.anchor(&c).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("source line"),
    };
    workspace
        .apply(&Edit::Move {
            target: source,
            position: Position::Before(destination),
        })
        .unwrap();
    assert!(
        matches!(workspace.view_anchored(&paragraph_handle), Ok(ViewOutcome::Paragraph { text, .. }) if text == "c\n")
    );
    assert!(matches!(
        workspace.view_anchored(&line_handle),
        Ok(ViewOutcome::Line { content, .. }) if content == "c"
    ));
}

#[test]
fn same_kind_line_relations_preserve_source_and_outside_anchors() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("coordinate.txt"), "one\n").unwrap();
    fs::write(fixture.path().join("note.txt"), "a\nb\nc\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let file = current(&workspace, AnddressTarget::File);
    let a = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "a\n".to_owned(),
        },
    );
    let source = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let outside = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::parse("2").unwrap(),
            exact_extent: "c\n".to_owned(),
        },
    );
    let file_handle = match workspace.anchor(&file).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("file"),
    };
    let source_handle = match workspace.anchor(&source).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("source"),
    };
    let outside_handle = match workspace.anchor(&outside).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("outside"),
    };

    workspace
        .apply(&Edit::Copy {
            target: source,
            position: Position::Before(a),
        })
        .unwrap();

    assert_eq!(
        fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
        "b\na\nb\nc\n"
    );
    assert!(matches!(
        workspace.view_anchored(&file_handle),
        Ok(ViewOutcome::File { text }) if text == "b\na\nb\nc\n"
    ));
    assert!(matches!(
        workspace.view_anchored(&source_handle),
        Ok(ViewOutcome::Line { content, .. }) if content == "b"
    ));
    assert!(matches!(
        workspace.view_anchored(&outside_handle),
        Ok(ViewOutcome::Line { content, .. }) if content == "c"
    ));
    let copied = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "b\n".to_owned(),
        },
    );
    assert!(matches!(
        workspace.anchor(&copied),
        Ok(AnchorOutcome::Anchored(_))
    ));
    assert!(fs::read_dir(fixture.path()).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".env.artext-apply-")
    }));
}

#[test]
fn same_kind_paragraph_relations_keep_copy_and_delete_dispositions() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("coordinate.txt"), "one\n").unwrap();
    fs::write(fixture.path().join("note.txt"), "a\n\nb\n\nc\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let source = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let middle = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::one(),
        },
    );
    let outside = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::parse("2").unwrap(),
        },
    );
    let source_handle = match workspace.anchor(&source).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("source"),
    };
    let outside_handle = match workspace.anchor(&outside).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("outside"),
    };

    workspace
        .apply(&Edit::Copy {
            target: source,
            position: Position::After(middle),
        })
        .unwrap();

    assert_eq!(
        fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
        "a\n\nb\na\n\nc\n"
    );
    assert!(matches!(
        workspace.view_anchored(&source_handle),
        Ok(ViewOutcome::Paragraph { text, .. }) if text == "a\n"
    ));
    assert!(matches!(
        workspace.view_anchored(&outside_handle),
        Ok(ViewOutcome::Paragraph { text, .. }) if text == "c\n"
    ));

    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("coordinate.txt"), "one\n").unwrap();
    fs::write(fixture.path().join("note.txt"), "a\n\nb\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let target = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let outside = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::one(),
        },
    );
    let target_handle = match workspace.anchor(&target).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("target"),
    };
    let outside_handle = match workspace.anchor(&outside).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("outside"),
    };

    workspace.apply(&Edit::Delete { target }).unwrap();

    assert_eq!(
        fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
        "\nb\n"
    );
    assert_eq!(
        workspace.view_anchored(&target_handle),
        Err(ViewError::Unavailable)
    );
    assert!(matches!(
        workspace.view_anchored(&outside_handle),
        Ok(ViewOutcome::Paragraph { text, .. }) if text == "b\n"
    ));
    assert!(fs::read_dir(fixture.path()).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".env.artext-apply-")
    }));
}

#[test]
fn apply_rebinds_outside_targets_and_removes_absorbed_or_split_candidates() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\nb\nc\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let middle = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let outside = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::parse("2").unwrap(),
            exact_extent: "c\n".to_owned(),
        },
    );
    let outside_handle = match workspace.anchor(&outside).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("outside handle"),
    };
    workspace
        .apply(&Edit::Move {
            target: middle,
            position: Position::After(outside),
        })
        .unwrap();
    assert!(
        matches!(workspace.view_anchored(&outside_handle), Ok(ViewOutcome::Line { content, .. }) if content == "c")
    );

    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\nb\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let paragraph = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let nested = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let nested_handle = match workspace.anchor(&nested).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("nested handle"),
    };
    workspace
        .apply(&Edit::Replace {
            target: paragraph,
            content: String::new(),
        })
        .unwrap();
    assert_eq!(
        workspace.view_anchored(&nested_handle),
        Err(ViewError::Unavailable)
    );

    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\nb\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let container = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let target = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let container_handle = match workspace.anchor(&container).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("container handle"),
    };
    workspace
        .apply(&Edit::Replace {
            target,
            content: "x\n\ny\n".to_owned(),
        })
        .unwrap();
    assert_eq!(
        workspace.view_anchored(&container_handle),
        Err(ViewError::Unavailable)
    );
}

#[test]
fn apply_noop_leaves_live_anchor_continuity_unchanged() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\nb\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let target = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let handle = match workspace.anchor(&target).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("anchor"),
    };

    workspace
        .apply(&Edit::Move {
            target: target.clone(),
            position: Position::Before(target),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
        "one\nb\n"
    );
    assert!(
        matches!(workspace.view_anchored(&handle), Ok(ViewOutcome::Line { content, .. }) if content == "b")
    );

    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("coordinate.txt"), "one\n").unwrap();
    let extent = format!("{}\r\n", "x".repeat(8_191));
    let source = format!("{extent}{extent}");
    fs::write(fixture.path().join("note.txt"), &source).unwrap();
    let mut workspace = runtime(fixture.path());
    let file = current(&workspace, AnddressTarget::File);
    let first = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: extent.clone(),
        },
    );
    let second = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: extent,
        },
    );
    let file_handle = match workspace.anchor(&file).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("file"),
    };
    let first_handle = match workspace.anchor(&first).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("first"),
    };
    let second_handle = match workspace.anchor(&second).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("second"),
    };

    workspace
        .apply(&Edit::Move {
            target: first,
            position: Position::After(second),
        })
        .unwrap();

    assert_eq!(
        fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
        source
    );
    assert!(
        matches!(workspace.view_anchored(&file_handle), Ok(ViewOutcome::File { text }) if text == source)
    );
    assert!(
        matches!(workspace.view_anchored(&first_handle), Ok(ViewOutcome::Line { content, .. }) if content.len() == 8_191)
    );
    assert!(
        matches!(workspace.view_anchored(&second_handle), Ok(ViewOutcome::Line { content, .. }) if content.len() == 8_191)
    );
    assert!(fs::read_dir(fixture.path()).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".env.artext-apply-")
    }));
}

#[test]
fn empty_insert_preserves_live_file_and_line_anchors_after_validation() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\nb\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let file = current(&workspace, AnddressTarget::File);
    let line = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let file_handle = match workspace.anchor(&file).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("file anchor"),
    };
    let line_handle = match workspace.anchor(&line).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("line anchor"),
    };

    workspace
        .apply(&Edit::Insert {
            position: Position::EndOf(file),
            content: String::new(),
        })
        .unwrap();

    assert_eq!(
        fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
        "one\nb\n"
    );
    assert!(matches!(
        workspace.view_anchored(&file_handle),
        Ok(ViewOutcome::File { text }) if text == "one\nb\n"
    ));
    assert!(matches!(
        workspace.view_anchored(&line_handle),
        Ok(ViewOutcome::Line { content, .. }) if content == "b"
    ));
    assert!(fs::read_dir(fixture.path()).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".env.artext-apply-")
    }));
}

#[test]
fn empty_insert_keeps_stale_operand_and_binding_fail_closure() {
    let operand_fixture = tempdir().unwrap();
    fs::write(operand_fixture.path().join("note.txt"), "one\nb\n").unwrap();
    let mut operand_workspace = runtime(operand_fixture.path());
    let stale_operand = current(
        &operand_workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let live = current(
        &operand_workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let live_handle = match operand_workspace.anchor(&live).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("live anchor"),
    };
    fs::write(operand_fixture.path().join("note.txt"), "changed\nb\n").unwrap();
    assert_eq!(
        operand_workspace.apply(&Edit::Insert {
            position: Position::Before(stale_operand),
            content: String::new(),
        }),
        Err(ApplyError::Unavailable)
    );
    assert!(matches!(
        operand_workspace.view_anchored(&live_handle),
        Ok(ViewOutcome::Line { content, .. }) if content == "b"
    ));

    let binding_fixture = tempdir().unwrap();
    fs::write(binding_fixture.path().join("note.txt"), "one\nb\n").unwrap();
    let mut binding_workspace = runtime(binding_fixture.path());
    let file = current(&binding_workspace, AnddressTarget::File);
    let stale = current(
        &binding_workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let file_handle = match binding_workspace.anchor(&file).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("file anchor"),
    };
    let _stale_handle = match binding_workspace.anchor(&stale).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("stale anchor"),
    };
    fs::write(binding_fixture.path().join("note.txt"), "changed\nb\n").unwrap();
    assert_eq!(
        binding_workspace.apply(&Edit::Insert {
            position: Position::EndOf(file),
            content: String::new(),
        }),
        Err(ApplyError::Unavailable)
    );
    assert_eq!(
        binding_workspace.view_anchored(&file_handle),
        Err(ViewError::Unavailable)
    );
}

#[test]
fn apply_paragraph_boundary_noop_preserves_anchor_provenance() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\n \t\nb\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let target = current(
        &workspace,
        AnddressTarget::Paragraph {
            ordinal: Natural::one(),
        },
    );
    let handle = match workspace.anchor(&target).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("anchor"),
    };

    workspace
        .apply(&Edit::Move {
            target: target.clone(),
            position: Position::Before(target),
        })
        .unwrap();
    assert_eq!(
        fs::read_to_string(fixture.path().join("note.txt")).unwrap(),
        "one\n \t\nb\n"
    );
    assert!(
        matches!(workspace.view_anchored(&handle), Ok(ViewOutcome::Paragraph { text, .. }) if text == "b\n")
    );
}

#[test]
fn apply_removes_colliding_line_rebindings_after_a_join() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\nb\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let first = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let second = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let first_handle = match workspace.anchor(&first).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("first handle"),
    };
    let second_handle = match workspace.anchor(&second).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("second handle"),
    };

    workspace
        .apply(&Edit::Replace {
            target: first,
            content: "one".to_owned(),
        })
        .unwrap();
    assert_eq!(
        workspace.view_anchored(&first_handle),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view_anchored(&second_handle),
        Err(ViewError::Unavailable)
    );
}

#[test]
fn raw_view_never_changes_live_anchor_continuity() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let live = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let handle = match workspace.anchor(&live).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("first handle"),
    };
    let stale = Anddress {
        target: AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "missing\n".to_owned(),
        },
        ..live.clone()
    };

    assert_eq!(workspace.view(&stale), Err(ViewError::Unavailable));
    assert!(matches!(
        workspace.view_anchored(&handle),
        Ok(ViewOutcome::Line { content, .. }) if content == "one"
    ));
}

#[test]
fn anchored_view_checks_only_the_selected_binding_before_a_mismatch_fail_closes_the_path() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\ntwo\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let first = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let second = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "two\n".to_owned(),
        },
    );
    let first_handle = match workspace.anchor(&first).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("first handle"),
    };
    let second_handle = match workspace.anchor(&second).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("second handle"),
    };
    fs::write(fixture.path().join("note.txt"), "changed\ntwo\n").unwrap();

    assert!(matches!(
        workspace.view_anchored(&second_handle),
        Ok(ViewOutcome::Line { content, .. }) if content == "two"
    ));
    assert_eq!(
        workspace.view_anchored(&first_handle),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view_anchored(&second_handle),
        Err(ViewError::Unavailable)
    );
}

#[test]
fn anchored_view_known_invalid_source_fail_closes_the_path() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\n").unwrap();
    let mut workspace = runtime(fixture.path());
    let input = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let handle = match workspace.anchor(&input).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("first handle"),
    };
    fs::write(fixture.path().join("note.txt"), b"one\n\xff").unwrap();

    assert_eq!(
        workspace.view_anchored(&handle),
        Err(ViewError::Unavailable)
    );
    fs::write(fixture.path().join("note.txt"), "one\n").unwrap();
    assert_eq!(
        workspace.view_anchored(&handle),
        Err(ViewError::Unavailable)
    );
}

#[test]
fn explicit_invalidation_keeps_hard_link_paths_and_reopened_runtimes_separate() {
    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\n").unwrap();
    fs::hard_link(
        fixture.path().join("note.txt"),
        fixture.path().join("linked.txt"),
    )
    .unwrap();
    let mut workspace = runtime(fixture.path());
    let note = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let linked = current_at(
        &workspace,
        "linked.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "one\n".to_owned(),
        },
    );
    let note_handle = match workspace.anchor(&note).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("note"),
    };
    let linked_handle = match workspace.anchor(&linked).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("linked"),
    };
    workspace.invalidate_anchored_source("note.txt").unwrap();
    assert_eq!(
        workspace.view_anchored(&note_handle),
        Err(ViewError::Unavailable)
    );
    assert!(
        matches!(workspace.view_anchored(&linked_handle), Ok(ViewOutcome::Line { content, .. }) if content == "one")
    );

    let mut reopened = runtime(fixture.path());
    assert_eq!(
        reopened.view_anchored(&linked_handle),
        Err(ViewError::Unavailable)
    );
}
