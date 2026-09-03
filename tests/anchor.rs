mod support;

use std::fs;

use backwriter::{
    backwriter::{
        anchor::{AnchorError, AnchorOutcome},
        anddress::{Anddress, AnddressTarget as PublicAnddressTarget, LineTerminator},
        apply::{ApplyError, EditReceipt},
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

fn host_runtime(root: &std::path::Path) -> WorkspaceRuntime {
    WorkspaceRuntime::open_host_authoritative(
        root,
        WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
    )
    .unwrap()
}

fn line_body<'a>(anddress: &Anddress, content: &'a str) -> &'a str {
    match anddress.terminator().expect("projected Line terminator") {
        LineTerminator::None => content,
        LineTerminator::Lf | LineTerminator::Cr => &content[..content.len() - 1],
        LineTerminator::Crlf => &content[..content.len() - 2],
    }
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

fn build_address(
    workspace: &WorkspaceRuntime,
    logical_path: &str,
    target: AnddressTarget,
    coordinate: String,
) -> Anddress {
    let source = fs::read(workspace.workspace_root().join(logical_path)).unwrap_or_default();
    match target {
        AnddressTarget::File => support::file(&coordinate, logical_path, &source),
        AnddressTarget::Paragraph { ordinal } => {
            let paragraphs = paragraph_ranges(&source);
            let (start, end) = paragraphs.get(ordinal.0).copied().unwrap_or((0, 0));
            support::address(
                &coordinate,
                logical_path,
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
                    &coordinate,
                    logical_path,
                    &source,
                    PublicAnddressTarget::Line,
                    start,
                    end,
                )
            } else {
                let mut stale_source = exact_extent.into_bytes();
                stale_source.push(b'!');
                support::address(
                    &coordinate,
                    logical_path,
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

fn coordinate(workspace: &WorkspaceRuntime) -> String {
    let request = backwriter::backwriter::search::SearchRequest::exact_file("note.txt").unwrap();
    match workspace.search(&request).unwrap() {
        backwriter::backwriter::search::SearchOutcome::Found { anddresses } => {
            anddresses[0].workspace_coordinate().to_owned()
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
    build_address(workspace, logical_path, target, coordinate(workspace))
}

#[test]
fn host_apply_reflects_file_paragraph_and_line_from_the_installed_after_identity() {
    let fixture = tempdir().unwrap();
    let before = b"one\n\ntwo\n";
    fs::write(fixture.path().join("note.txt"), before).unwrap();
    let mut workspace = host_runtime(fixture.path());
    let file = current(&workspace, AnddressTarget::File);
    let paragraph = support::address(
        file.workspace_coordinate(),
        "note.txt",
        before,
        PublicAnddressTarget::Paragraph,
        0,
        4,
    );
    let line = support::address(
        file.workspace_coordinate(),
        "note.txt",
        before,
        PublicAnddressTarget::Line,
        0,
        4,
    );
    let file_handle = match workspace.anchor(&file).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("File Anchor"),
    };
    let paragraph_handle = match workspace.anchor(&paragraph).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("Paragraph Anchor"),
    };
    let line_handle = match workspace.anchor(&line).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("Line Anchor"),
    };

    workspace
        .apply(&Edit::Insert {
            position: Position::StartOf(file),
            content: "prefix\n\n".to_owned(),
        })
        .unwrap();

    let after = b"prefix\n\none\n\ntwo\n";
    let after_file = support::file(paragraph.workspace_coordinate(), "note.txt", after);
    let after_paragraph = support::address(
        paragraph.workspace_coordinate(),
        "note.txt",
        after,
        PublicAnddressTarget::Paragraph,
        8,
        12,
    );
    assert_eq!(fs::read(fixture.path().join("note.txt")).unwrap(), after);
    assert_eq!(
        workspace.check(after_file.clone()).unwrap().filtered,
        Some(after_file.clone())
    );
    assert!(matches!(
        workspace.view_anchored(&file_handle, PublicAnddressTarget::File),
        Ok(ViewOutcome::Projected { content, .. }) if content.as_bytes() == after
    ));
    assert!(matches!(
        workspace.view_anchored(&paragraph_handle, PublicAnddressTarget::Paragraph),
        Ok(ViewOutcome::Projected { anddress, content })
            if content == "one\n"
                && anddress.project(PublicAnddressTarget::File).unwrap().as_ref()
                    == Some(&after_file)
    ));
    assert!(matches!(
        workspace.view_anchored(&line_handle, PublicAnddressTarget::Line),
        Ok(ViewOutcome::Projected { anddress, content })
            if line_body(&anddress, &content) == "one"
                && anddress.project(PublicAnddressTarget::File).unwrap().as_ref()
                    == Some(&after_file)
                && anddress.project(PublicAnddressTarget::Paragraph).unwrap().as_ref()
                    == Some(&after_paragraph)
    ));
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
        workspace.view_anchored(&handle, PublicAnddressTarget::Line),
        Ok(ViewOutcome::Projected { .. })
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
        matches!(workspace.view_anchored(&file_handle, PublicAnddressTarget::File), Ok(ViewOutcome::Projected { content, .. }) if content.len() == 40_015)
    );
    assert!(
        matches!(workspace.view_anchored(&paragraph_handle, PublicAnddressTarget::Paragraph), Ok(ViewOutcome::Projected { content, .. }) if content.len() == 20_010)
    );
}

#[test]
fn anchor_uses_direct_target_projection_without_view_capture() {
    let source = include_str!("../src/runtime/anchor.rs");
    let raw_anchor = source
        .split_once("pub(super) fn anchor")
        .and_then(|(_, source)| source.split_once("pub(super) fn view_anchored"))
        .map(|(raw_anchor, _)| raw_anchor)
        .expect("raw Anchor section");

    assert!(raw_anchor.contains("&inputs, None"));
    assert!(!raw_anchor.contains("ViewCapture"));
    assert!(!raw_anchor.contains("ViewOutcome"));

    let anchored = source
        .split_once("pub(super) fn view_anchored")
        .and_then(|(_, source)| source.split_once("pub(super) fn invalidate_source"))
        .map(|(anchored, _)| anchored)
        .expect("anchored View section");
    assert!(anchored.contains("match_current_proof"));
    assert!(anchored.contains("CurrentProofMatch::Matching"));
    assert!(anchored.contains("execute_trusted"));
    assert!(anchored.contains("CurrentProofMatch::Missing"));
    assert!(anchored.contains("observe_current"));
}

#[test]
fn invalidation_is_path_exact_and_does_not_read_the_source() {
    assert_eq!(
        include_str!("../src/runtime.rs")
            .matches("anchor::invalidate_source(self, path)")
            .count(),
        2
    );
    let invalidator = include_str!("../src/runtime/anchor.rs")
        .split_once("pub(super) fn invalidate_source")
        .and_then(|(_, invalidator)| invalidator.split_once("fn validate("))
        .map(|(invalidator, _)| invalidator)
        .unwrap();
    assert!(invalidator.contains("invalidate_source_state(path)"));
    assert!(!invalidator.contains("open_admitted"));
    assert!(!invalidator.contains("observe_"));

    let fixture = tempdir().unwrap();
    fs::write(fixture.path().join("note.txt"), "one\n").unwrap();
    let mut workspace = host_runtime(fixture.path());
    let note = current(&workspace, AnddressTarget::File);
    let note_handle = match workspace.anchor(&note).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("note File Anchor"),
    };
    assert_eq!(
        workspace.invalidate_source("."),
        Err(AnchorError::InvalidInput)
    );
    assert_eq!(workspace.invalidate_source("missing.txt"), Ok(()));
    assert_eq!(
        workspace.invalidate_source(".artext/bw/x"),
        Err(AnchorError::Unavailable)
    );
    let parked_note = fixture.path().join("parked-note");
    fs::rename(fixture.path().join("note.txt"), &parked_note).unwrap();
    assert_eq!(
        workspace.check(note.clone()).unwrap().filtered,
        Some(note.clone())
    );
    fs::rename(&parked_note, fixture.path().join("note.txt")).unwrap();
    assert!(matches!(
        workspace.view_anchored(&note_handle, PublicAnddressTarget::File),
        Ok(ViewOutcome::Projected { content, .. }) if content == "one\n"
    ));
    assert_eq!(workspace.invalidate_anchored_source("note.txt"), Ok(()));
    assert_eq!(
        workspace.view_anchored(&note_handle, PublicAnddressTarget::File),
        Err(ViewError::Unavailable)
    );

    fs::create_dir(fixture.path().join("admitted")).unwrap();
    fs::write(fixture.path().join("admitted/source.txt"), "admitted\n").unwrap();
    let mut named = WorkspaceRuntime::open_host_authoritative(
        fixture.path(),
        WorkspaceAdmission::new([AdmissionRoot::new("admitted").unwrap()]).unwrap(),
    )
    .unwrap();
    let backwriter::backwriter::search::SearchOutcome::Found { mut anddresses } = named
        .search(
            &backwriter::backwriter::search::SearchRequest::exact_file("admitted/source.txt")
                .unwrap(),
        )
        .unwrap()
    else {
        panic!("admitted File")
    };
    let admitted = anddresses.pop().unwrap();
    let admitted_handle = match named.anchor(&admitted).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("admitted File Anchor"),
    };
    assert_eq!(named.invalidate_source("admitted/missing.txt"), Ok(()));
    assert_eq!(
        named.invalidate_source("note.txt"),
        Err(AnchorError::Unavailable)
    );
    let parked_admitted = fixture.path().join("parked-admitted");
    fs::rename(fixture.path().join("admitted/source.txt"), &parked_admitted).unwrap();
    assert_eq!(
        named.check(admitted.clone()).unwrap().filtered,
        Some(admitted.clone())
    );
    fs::rename(&parked_admitted, fixture.path().join("admitted/source.txt")).unwrap();
    assert!(matches!(
        named.view_anchored(&admitted_handle, PublicAnddressTarget::File),
        Ok(ViewOutcome::Projected { content, .. }) if content == "admitted\n"
    ));
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
        matches!(first_runtime.view_anchored(&handle, PublicAnddressTarget::Line), Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "one")
    );
    assert_eq!(
        second_runtime.view_anchored(&handle, PublicAnddressTarget::Line),
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
    let current_input = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "two\n".to_owned(),
        },
    );

    assert!(matches!(
        workspace.anchor(&current_input),
        Err(AnchorError::Unavailable)
    ));
    assert_eq!(
        workspace.view_anchored(&handle, PublicAnddressTarget::Line),
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
        matches!(workspace.view_anchored(&contained, PublicAnddressTarget::Line), Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "one")
    );
    drop(contained);

    fs::write(fixture.path().join("note.txt"), "a\nb\nc\n").unwrap();
    let original = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let original_handle = match workspace.anchor(&original).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("original handle"),
    };
    let destination = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::parse("2").unwrap(),
            exact_extent: "c\n".to_owned(),
        },
    );
    workspace
        .apply(&Edit::Copy {
            target: original,
            position: Position::After(destination),
        })
        .unwrap();
    assert!(
        matches!(workspace.view_anchored(&original_handle, PublicAnddressTarget::Line), Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "b")
    );
    let copied = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::parse("3").unwrap(),
            exact_extent: "b\n".to_owned(),
        },
    );
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
    let fresh_b = support::address(
        paragraph.workspace_coordinate(),
        "note.txt",
        b"a\nB\n",
        PublicAnddressTarget::Line,
        2,
        4,
    );
    assert_eq!(
        workspace.apply_replace(&Edit::Replace {
            target: b,
            content: "B\n".to_owned(),
        }),
        Ok(EditReceipt::Changed {
            anddress: Some(fresh_b.clone())
        })
    );
    assert!(matches!(
        workspace.view_anchored(&paragraph_handle, PublicAnddressTarget::Paragraph),
        Ok(ViewOutcome::Projected { anddress, content })
            if content == "a\nB\n"
                && anddress.source_state_hash() == fresh_b.source_state_hash()
                && anddress.source_byte_length() == fresh_b.source_byte_length()
    ));

    let split = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "B\n".to_owned(),
        },
    );
    workspace
        .apply(&Edit::Replace {
            target: split,
            content: "\nx\n".to_owned(),
        })
        .unwrap();
    assert_eq!(
        workspace.view_anchored(&paragraph_handle, PublicAnddressTarget::Paragraph),
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
        workspace.view_anchored(&handle, PublicAnddressTarget::Paragraph),
        Ok(ViewOutcome::Projected { content, .. }) if content == "new\n"
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
        workspace.view_anchored(&selected_handle, PublicAnddressTarget::Line),
        Err(ViewError::Unavailable)
    );
    assert!(matches!(
        workspace.view_anchored(&outside_handle, PublicAnddressTarget::Line),
        Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "b"
    ));
    assert!(matches!(
        workspace.anchor(&outside),
        Err(AnchorError::Unavailable)
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
        workspace.view_anchored(&outside_handle, PublicAnddressTarget::Line),
        Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "b"
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
        workspace.view_anchored(&handle, PublicAnddressTarget::Line),
        Ok(ViewOutcome::Projected { anddress, content })
            if content == "cc" && anddress.terminator() == Some(LineTerminator::None)
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
fn copy_source_member_rebinds_across_after_projector_chunk_boundaries() {
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
            workspace.view_anchored(&handle, PublicAnddressTarget::Line),
            Ok(ViewOutcome::Projected { anddress, content })
                if content == expected && anddress.terminator() == Some(LineTerminator::None)
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
        workspace.view_anchored(&handle, PublicAnddressTarget::File),
        Ok(ViewOutcome::Projected { content, .. }) if content == "after\n"
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
        workspace.view_anchored(&file_handle, PublicAnddressTarget::File),
        Ok(ViewOutcome::Projected { content, .. }) if content == "one\nthree\n"
    ));
    assert_eq!(
        workspace.view_anchored(&paragraph_handle, PublicAnddressTarget::Paragraph),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view_anchored(&line_handle, PublicAnddressTarget::Line),
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
        workspace.view_anchored(&paragraph_handle, PublicAnddressTarget::Paragraph),
        Err(ViewError::Unavailable)
    );
    assert!(matches!(
        workspace.view_anchored(&b_handle, PublicAnddressTarget::Line),
        Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "b"
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
        workspace.view_anchored(&b_handle, PublicAnddressTarget::Line),
        Err(ViewError::Unavailable)
    );
    assert!(matches!(
        workspace.view_anchored(&c_handle, PublicAnddressTarget::Line),
        Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "c"
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
        workspace.view_anchored(&b_handle, PublicAnddressTarget::Line),
        Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "b"
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
        matches!(workspace.view_anchored(&paragraph_handle, PublicAnddressTarget::Paragraph), Ok(ViewOutcome::Projected { content, .. }) if content == "c\n")
    );
    assert!(matches!(
        workspace.view_anchored(&line_handle, PublicAnddressTarget::Line),
        Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "c"
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
        workspace.view_anchored(&file_handle, PublicAnddressTarget::File),
        Ok(ViewOutcome::Projected { content, .. }) if content == "b\na\nb\nc\n"
    ));
    assert!(matches!(
        workspace.view_anchored(&source_handle, PublicAnddressTarget::Line),
        Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "b"
    ));
    assert!(matches!(
        workspace.view_anchored(&outside_handle, PublicAnddressTarget::Line),
        Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "c"
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
        workspace.view_anchored(&source_handle, PublicAnddressTarget::Paragraph),
        Ok(ViewOutcome::Projected { content, .. }) if content == "a\n"
    ));
    assert!(matches!(
        workspace.view_anchored(&outside_handle, PublicAnddressTarget::Paragraph),
        Ok(ViewOutcome::Projected { content, .. }) if content == "c\n"
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
        workspace.view_anchored(&target_handle, PublicAnddressTarget::Paragraph),
        Err(ViewError::Unavailable)
    );
    assert!(matches!(
        workspace.view_anchored(&outside_handle, PublicAnddressTarget::Paragraph),
        Ok(ViewOutcome::Projected { content, .. }) if content == "b\n"
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
        matches!(workspace.view_anchored(&outside_handle, PublicAnddressTarget::Line), Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "c")
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
        workspace.view_anchored(&nested_handle, PublicAnddressTarget::Line),
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
        workspace.view_anchored(&container_handle, PublicAnddressTarget::Paragraph),
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
        matches!(workspace.view_anchored(&handle, PublicAnddressTarget::Line), Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "b")
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
        matches!(workspace.view_anchored(&file_handle, PublicAnddressTarget::File), Ok(ViewOutcome::Projected { content, .. }) if content == source)
    );
    assert!(
        matches!(workspace.view_anchored(&first_handle, PublicAnddressTarget::Line), Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content).len() == 8_191)
    );
    assert!(
        matches!(workspace.view_anchored(&second_handle, PublicAnddressTarget::Line), Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content).len() == 8_191)
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
fn zero_range_apply_noops_preserve_live_anchor_continuity() {
    let fixture = tempdir().unwrap();
    let source = b"one\nb\n";
    fs::write(fixture.path().join("note.txt"), source).unwrap();
    let mut workspace = runtime(fixture.path());
    let live = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "b\n".to_owned(),
        },
    );
    let handle = match workspace.anchor(&live).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("anchor"),
    };
    let raw = support::address(
        &coordinate(&workspace),
        "note.txt",
        source,
        PublicAnddressTarget::Line,
        2,
        2,
    );

    for edit in [
        Edit::Delete {
            target: raw.clone(),
        },
        Edit::Copy {
            target: raw.clone(),
            position: Position::Before(live.clone()),
        },
        Edit::Move {
            target: raw.clone(),
            position: Position::After(live.clone()),
        },
        Edit::Replace {
            target: raw,
            content: String::new(),
        },
    ] {
        workspace.apply(&edit).unwrap();
        assert_eq!(fs::read(fixture.path().join("note.txt")).unwrap(), source);
        assert!(
            matches!(workspace.view_anchored(&handle, PublicAnddressTarget::Line), Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "b")
        );
    }
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
        workspace.view_anchored(&file_handle, PublicAnddressTarget::File),
        Ok(ViewOutcome::Projected { content, .. }) if content == "one\nb\n"
    ));
    assert!(matches!(
        workspace.view_anchored(&line_handle, PublicAnddressTarget::Line),
        Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "b"
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
    assert_eq!(
        operand_workspace.view_anchored(&live_handle, PublicAnddressTarget::Line),
        Err(ViewError::Unavailable)
    );

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
        binding_workspace.view_anchored(&file_handle, PublicAnddressTarget::File),
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
        matches!(workspace.view_anchored(&handle, PublicAnddressTarget::Paragraph), Ok(ViewOutcome::Projected { content, .. }) if content == "b\n")
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
        workspace.view_anchored(&first_handle, PublicAnddressTarget::Line),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view_anchored(&second_handle, PublicAnddressTarget::Line),
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
    let stale = current(
        &workspace,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "missing\n".to_owned(),
        },
    );

    assert_eq!(
        workspace.view(&stale, stale.target()),
        Err(ViewError::Unavailable)
    );
    assert!(matches!(
        workspace.view_anchored(&handle, PublicAnddressTarget::Line),
        Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "one"
    ));
}

#[test]
fn anchor_rejects_a_raw_valid_nonstructural_range() {
    let fixture = tempdir().unwrap();
    let source = b"one\n";
    fs::write(fixture.path().join("note.txt"), source).unwrap();
    let mut workspace = runtime(fixture.path());
    let raw = support::address(
        &coordinate(&workspace),
        "note.txt",
        source,
        PublicAnddressTarget::Line,
        1,
        2,
    );

    assert!(matches!(
        workspace.anchor(&raw),
        Err(AnchorError::Unavailable)
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

    assert_eq!(
        workspace.view_anchored(&second_handle, PublicAnddressTarget::Line),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view_anchored(&first_handle, PublicAnddressTarget::Line),
        Err(ViewError::Unavailable)
    );
    assert_eq!(
        workspace.view_anchored(&second_handle, PublicAnddressTarget::Line),
        Err(ViewError::Unavailable)
    );
}

#[test]
fn host_anchored_proof_mismatch_fail_closes_before_source_access() {
    let fixture = tempdir().unwrap();
    let source_path = fixture.path().join("note.txt");
    fs::write(&source_path, "old\n").unwrap();
    let mut workspace = host_runtime(fixture.path());
    let old = current(&workspace, AnddressTarget::File);
    let handle = match workspace.anchor(&old).unwrap() {
        AnchorOutcome::Anchored(handle) => handle,
        AnchorOutcome::AlreadyLive => panic!("old File Anchor"),
    };

    // This deliberately violates the Host guard only to create mismatched
    // proof evidence and use source absence as an I/O tripwire.
    fs::write(&source_path, "new\n").unwrap();
    let new = current(&workspace, AnddressTarget::File);
    let parked = fixture.path().join("parked-note");
    fs::rename(&source_path, &parked).unwrap();

    assert_eq!(
        workspace.view_anchored(&handle, PublicAnddressTarget::File),
        Err(ViewError::Unavailable)
    );
    assert_eq!(workspace.check(new.clone()).unwrap().filtered, None);

    fs::rename(&parked, &source_path).unwrap();
    assert_eq!(
        workspace.view_anchored(&handle, PublicAnddressTarget::File),
        Err(ViewError::Unavailable)
    );
    assert_eq!(workspace.check(new.clone()).unwrap().filtered, Some(new));
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
        workspace.view_anchored(&handle, PublicAnddressTarget::Line),
        Err(ViewError::Unavailable)
    );
    fs::write(fixture.path().join("note.txt"), "one\n").unwrap();
    assert_eq!(
        workspace.view_anchored(&handle, PublicAnddressTarget::Line),
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
        workspace.view_anchored(&note_handle, PublicAnddressTarget::File),
        Err(ViewError::Unavailable)
    );
    assert!(
        matches!(workspace.view_anchored(&linked_handle, PublicAnddressTarget::Line), Ok(ViewOutcome::Projected { anddress, content }) if line_body(&anddress, &content) == "one")
    );

    let mut reopened = runtime(fixture.path());
    assert_eq!(
        reopened.view_anchored(&linked_handle, PublicAnddressTarget::Line),
        Err(ViewError::Unavailable)
    );
}
