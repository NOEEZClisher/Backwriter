mod support;

use std::fs;

use backwriter::backwriter::anddress::{ANDDRESS_VERSION, AnddressTarget};
use backwriter::backwriter::pick::PickOutcome;
use backwriter::backwriter::search::{
    SearchOutcome, SearchQuery, SearchRequest, SearchScope, SearchTarget,
};
use backwriter::runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime};
use tempfile::tempdir;

fn runtime(root: &std::path::Path, admission: &str) -> WorkspaceRuntime {
    WorkspaceRuntime::open(
        root,
        WorkspaceAdmission::new([AdmissionRoot::new(admission).unwrap()]).unwrap(),
    )
    .unwrap()
}

fn coordinate(runtime: &WorkspaceRuntime) -> String {
    let request = SearchRequest::new(
        SearchQuery::new("seed").unwrap(),
        SearchScope::all_admitted(),
        SearchTarget::File,
    );
    let SearchOutcome::Found { anddresses } = runtime.search(&request).unwrap() else {
        panic!("coordinate source")
    };
    anddresses[0].workspace_coordinate().to_owned()
}

#[test]
fn check_reports_current_removed_and_unavailable_v4_addresses() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), b"seed\n").unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    fs::write(root.join("invalid.txt"), b"\xff").unwrap();
    fs::write(root.join("zero.txt"), b"seed\0").unwrap();

    let current = support::file(&coordinate, "seed.txt", b"seed\n");
    let result = workspace.check(current.clone()).unwrap();
    assert_eq!(result.filtered, Some(current));
    assert_eq!(result.report.current_count(), 1);
    assert_eq!(result.report.checked_count(), 1);

    let stale = support::line(&coordinate, "seed.txt", b"other\n", 0);
    let result = workspace.check(stale.clone()).unwrap();
    assert_eq!(result.filtered, None);
    assert_eq!(result.report.removed(), &[stale]);

    for (path, bytes) in [("invalid.txt", b"\xff".as_slice()), ("zero.txt", b"seed\0")] {
        let unavailable = support::file(&coordinate, path, bytes);
        let result = workspace.check(unavailable.clone()).unwrap();
        assert_eq!(result.filtered, Some(unavailable.clone()));
        assert_eq!(result.report.unavailable(), &[unavailable]);
    }
}

#[test]
fn check_search_and_pick_preserve_order_multiplicity_and_canonical_empty() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), b"seed\n").unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    fs::write(root.join("invalid.txt"), b"\xff").unwrap();
    let current = support::file(&coordinate, "seed.txt", b"seed\n");
    let stale = support::line(&coordinate, "seed.txt", b"other\n", 0);
    let unavailable = support::file(&coordinate, "invalid.txt", b"\xff");
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
    assert_eq!(
        workspace
            .check_search(SearchOutcome::Empty)
            .unwrap()
            .filtered,
        SearchOutcome::Empty
    );
    assert_eq!(
        workspace.check_pick(PickOutcome::Empty).unwrap().filtered,
        PickOutcome::Empty
    );
}

#[test]
fn check_matches_ranges_across_decimal_digit_boundaries_without_reordering() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), b"seed\n").unwrap();
    let source = "x\n\n".repeat(101);
    fs::write(root.join("note.txt"), &source).unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    let line_99 = support::line(&coordinate, "note.txt", source.as_bytes(), 99);
    let line_100 = support::line(&coordinate, "note.txt", source.as_bytes(), 100);
    let paragraph_49 = support::paragraph(&coordinate, "note.txt", source.as_bytes(), 49);
    let paragraph_50 = support::paragraph(&coordinate, "note.txt", source.as_bytes(), 50);
    let candidates = vec![
        line_100.clone(),
        paragraph_49.clone(),
        line_99.clone(),
        paragraph_50.clone(),
    ];

    let checked = workspace
        .check_pick(PickOutcome::Selected {
            anddresses: candidates.clone(),
        })
        .unwrap();
    assert_eq!(
        checked.filtered,
        PickOutcome::Selected {
            anddresses: candidates
        }
    );
    assert_eq!(checked.report.current_count(), 4);
}

#[test]
fn check_treats_raw_nonstructural_ranges_as_current_source_sentinels() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = b"seed\none\r\ntwo";
    fs::write(root.join("note.txt"), source).unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    let raw_paragraph = support::address(
        &coordinate,
        "note.txt",
        source,
        AnddressTarget::Paragraph,
        2,
        9,
    );
    let raw_line = support::address(&coordinate, "note.txt", source, AnddressTarget::Line, 1, 1);

    for input in [raw_paragraph, raw_line] {
        let checked = workspace.check(input.clone()).unwrap();
        assert_eq!(checked.filtered, Some(input));
        assert_eq!(checked.report.current_count(), 1);
        assert!(checked.report.removed().is_empty());
    }
}

#[test]
fn check_groups_mixed_hash_kind_range_and_duplicates_without_reordering() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = b"seed\none\n";
    fs::write(root.join("note.txt"), source).unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    let current_file = support::file(&coordinate, "note.txt", source);
    let current_raw = support::address(&coordinate, "note.txt", source, AnddressTarget::Line, 1, 7);
    let stale = support::address(
        &coordinate,
        "note.txt",
        b"seed\ntwo\n",
        AnddressTarget::Paragraph,
        0,
        5,
    );
    let candidates = vec![
        current_raw.clone(),
        stale.clone(),
        current_file.clone(),
        current_raw.clone(),
    ];

    let checked = workspace
        .check_pick(PickOutcome::Selected {
            anddresses: candidates,
        })
        .unwrap();

    assert_eq!(
        checked.filtered,
        PickOutcome::Selected {
            anddresses: vec![current_raw.clone(), current_file, current_raw]
        }
    );
    assert_eq!(checked.report.current_count(), 3);
    assert_eq!(checked.report.removed(), &[stale]);
    assert!(checked.report.unavailable().is_empty());
}

#[test]
fn check_marks_every_target_removed_after_any_source_state_change() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = b"seed\none\n";
    fs::write(root.join("note.txt"), source).unwrap();
    let workspace = runtime(&root, ".");
    let coordinate = coordinate(&workspace);
    let candidates = vec![
        support::file(&coordinate, "note.txt", source),
        support::paragraph(&coordinate, "note.txt", source, 0),
        support::line(&coordinate, "note.txt", source, 1),
    ];

    fs::write(root.join("note.txt"), b"seed\ntwo\n").unwrap();
    let checked = workspace
        .check_pick(PickOutcome::Selected {
            anddresses: candidates.clone(),
        })
        .unwrap();
    assert_eq!(checked.filtered, PickOutcome::Empty);
    assert_eq!(checked.report.removed(), candidates);
}

#[test]
fn check_outputs_only_canonical_v4_addresses() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), b"seed\n").unwrap();
    let workspace = runtime(&root, ".");
    let current = support::file(&coordinate(&workspace), "seed.txt", b"seed\n");
    let result = workspace.check(current).unwrap();
    let output = result.filtered.unwrap();
    assert_eq!(output.version(), ANDDRESS_VERSION);
    assert_eq!(output.target(), AnddressTarget::File);
    assert_eq!(output.byte_start(), 0);
    assert_eq!(output.byte_end(), output.source_byte_length());
}
