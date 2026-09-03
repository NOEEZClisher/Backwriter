mod support;

use std::fs;

use backwriter::backwriter::anddress::{ANDDRESS_VERSION, Anddress, AnddressTarget};
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

fn host_runtime(root: &std::path::Path, admission: &str) -> WorkspaceRuntime {
    WorkspaceRuntime::open_host_authoritative(
        root,
        WorkspaceAdmission::new([AdmissionRoot::new(admission).unwrap()]).unwrap(),
    )
    .unwrap()
}

fn exact_file(runtime: &WorkspaceRuntime, path: &str) -> Anddress {
    let SearchOutcome::Found { mut anddresses } = runtime
        .search(&SearchRequest::exact_file(path).unwrap())
        .unwrap()
    else {
        panic!("exact File")
    };
    assert_eq!(anddresses.len(), 1);
    anddresses.pop().unwrap()
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
fn check_reports_current_removed_and_unavailable_v5_addresses() {
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

    let search_candidates = vec![
        current.clone(),
        stale.clone(),
        unavailable.clone(),
        current.clone(),
    ];
    let expected_search = SearchOutcome::Found {
        anddresses: vec![
            search_candidates[0].clone(),
            search_candidates[2].clone(),
            search_candidates[3].clone(),
        ],
    };
    let checked = workspace
        .check_search(SearchOutcome::Found {
            anddresses: search_candidates,
        })
        .unwrap();
    assert_eq!(checked.filtered, expected_search);
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
fn check_outputs_only_canonical_v5_addresses() {
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

#[test]
fn host_search_proof_drives_every_check_form_and_preserves_a_large_mixed_group() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let source = b"seed\none\n";
    fs::write(root.join("note.txt"), source).unwrap();
    let host = host_runtime(&root, ".");
    let current_file = exact_file(&host, "note.txt");
    let coordinate = current_file.workspace_coordinate().to_owned();
    let current_line =
        support::address(&coordinate, "note.txt", source, AnddressTarget::Line, 1, 1);
    let current_paragraph = support::address(
        &coordinate,
        "note.txt",
        source,
        AnddressTarget::Paragraph,
        2,
        8,
    );
    let stale_hash = support::address(
        &coordinate,
        "note.txt",
        b"seed\ntwo\n",
        AnddressTarget::Line,
        0,
        1,
    );
    let stale_length = support::file(&coordinate, "note.txt", b"x");

    // Deliberately violating the Host mutation guard makes absence an I/O
    // tripwire only; the asserted currentness relies exclusively on the proof.
    fs::remove_file(root.join("note.txt")).unwrap();

    let one = host.check(current_file.clone()).unwrap();
    assert_eq!(one.filtered, Some(current_file.clone()));
    assert_eq!(one.report.current_count(), 1);

    let search_inputs = vec![
        stale_hash.clone(),
        current_line.clone(),
        stale_length.clone(),
        current_file.clone(),
    ];
    let expected_search = SearchOutcome::Found {
        anddresses: vec![search_inputs[1].clone(), search_inputs[3].clone()],
    };
    let checked = host
        .check_search(SearchOutcome::Found {
            anddresses: search_inputs,
        })
        .unwrap();
    assert_eq!(checked.filtered, expected_search);
    assert_eq!(
        checked.report.removed(),
        &[stale_hash.clone(), stale_length.clone()]
    );

    let pattern = [
        current_file.clone(),
        stale_hash.clone(),
        current_line.clone(),
        stale_length.clone(),
        current_paragraph.clone(),
    ];
    let mut candidates = Vec::with_capacity(10_000);
    let mut expected_filtered = Vec::with_capacity(6_000);
    let mut expected_removed = Vec::with_capacity(4_000);
    for index in 0..10_000 {
        let candidate = pattern[index % pattern.len()].clone();
        if matches!(index % pattern.len(), 1 | 3) {
            expected_removed.push(candidate.clone());
        } else {
            expected_filtered.push(candidate.clone());
        }
        candidates.push(candidate);
    }
    let checked = host
        .check_pick(PickOutcome::Selected {
            anddresses: candidates,
        })
        .unwrap();
    assert_eq!(
        checked.filtered,
        PickOutcome::Selected {
            anddresses: expected_filtered
        }
    );
    assert_eq!(checked.report.current_count(), 6_000);
    assert_eq!(checked.report.removed_count(), 4_000);
    assert_eq!(checked.report.checked_count(), 10_000);
    assert_eq!(checked.report.removed(), expected_removed);
    assert!(checked.report.unavailable().is_empty());

    // Mismatches neither remove nor replace the source proof.
    assert_eq!(
        host.check(current_file.clone()).unwrap().filtered,
        Some(current_file)
    );
}

#[test]
fn host_check_observes_each_proof_miss_group_and_never_installs_from_fallback() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("hit.txt"), b"hit\n").unwrap();
    fs::write(root.join("miss.txt"), b"miss\n").unwrap();
    let host = host_runtime(&root, ".");
    let hit = exact_file(&host, "hit.txt");
    let miss = support::file(hit.workspace_coordinate(), "miss.txt", b"miss\n");

    // The removed hit is an I/O tripwire; miss.txt remains the ordinary
    // observation path and its duplicate occurrences share one source group.
    fs::remove_file(root.join("hit.txt")).unwrap();
    let inputs = vec![
        miss.clone(),
        hit.clone(),
        miss.clone(),
        hit.clone(),
        miss.clone(),
    ];
    let checked = host
        .check_pick(PickOutcome::Selected {
            anddresses: inputs.clone(),
        })
        .unwrap();
    assert_eq!(
        checked.filtered,
        PickOutcome::Selected { anddresses: inputs }
    );
    assert_eq!(checked.report.current_count(), 5);

    fs::remove_file(root.join("miss.txt")).unwrap();
    let checked = host.check(miss.clone()).unwrap();
    assert_eq!(checked.filtered, None);
    assert_eq!(checked.report.removed(), &[miss]);
    assert_eq!(host.check(hit.clone()).unwrap().filtered, Some(hit));
}

#[test]
fn host_check_invalidation_restores_changed_missing_and_invalid_source_boundaries() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), b"one\n").unwrap();
    let mut host = host_runtime(&root, ".");

    let first = exact_file(&host, "note.txt");
    host.invalidate_source("note.txt").unwrap();
    fs::write(root.join("note.txt"), b"two\n").unwrap();
    let checked = host.check(first.clone()).unwrap();
    assert_eq!(checked.filtered, None);
    assert_eq!(checked.report.removed(), &[first]);

    let second = exact_file(&host, "note.txt");
    host.invalidate_source("note.txt").unwrap();
    fs::remove_file(root.join("note.txt")).unwrap();
    let checked = host.check(second.clone()).unwrap();
    assert_eq!(checked.filtered, None);
    assert_eq!(checked.report.removed(), &[second]);

    fs::write(root.join("note.txt"), b"three\n").unwrap();
    let third = exact_file(&host, "note.txt");
    host.invalidate_source("note.txt").unwrap();
    fs::write(root.join("note.txt"), b"invalid\xff").unwrap();
    let checked = host.check(third.clone()).unwrap();
    assert_eq!(checked.filtered, Some(third.clone()));
    assert_eq!(checked.report.unavailable(), &[third]);
}

#[test]
fn untrusted_search_followed_by_check_keeps_the_live_observation_path() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), b"current\n").unwrap();
    let untrusted = runtime(&root, ".");
    let current = exact_file(&untrusted, "note.txt");

    fs::write(root.join("note.txt"), b"invalid\xff").unwrap();
    let checked = untrusted.check(current.clone()).unwrap();
    assert_eq!(checked.filtered, Some(current.clone()));
    assert_eq!(checked.report.current_count(), 0);
    assert_eq!(checked.report.unavailable(), &[current]);
}

#[test]
fn host_check_keeps_workspace_private_admission_and_empty_boundaries() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir_all(root.join("admitted")).unwrap();
    fs::create_dir_all(root.join(".artext/bw")).unwrap();
    fs::write(root.join("admitted/note.txt"), b"note\n").unwrap();
    fs::write(root.join("other.txt"), b"other\n").unwrap();
    fs::write(root.join(".artext/bw/private.txt"), b"private\n").unwrap();
    let host = host_runtime(&root, ".");
    let current = exact_file(&host, "admitted/note.txt");
    let coordinate = current.workspace_coordinate().to_owned();
    let wrong_workspace = support::file(&"0".repeat(64), "admitted/note.txt", b"note\n");
    let private = support::file(&coordinate, ".artext/bw/private.txt", b"private\n");

    for input in [wrong_workspace, private] {
        let checked = host.check(input.clone()).unwrap();
        assert_eq!(checked.filtered, None);
        assert_eq!(checked.report.removed(), &[input]);
    }

    let admitted_only = host_runtime(&root, "admitted");
    let unadmitted = support::file(&coordinate, "other.txt", b"other\n");
    let checked = admitted_only.check(unadmitted.clone()).unwrap();
    assert_eq!(checked.filtered, None);
    assert_eq!(checked.report.removed(), &[unadmitted]);

    let empty = host.check_pick(PickOutcome::Empty).unwrap();
    assert_eq!(empty.filtered, PickOutcome::Empty);
    assert_eq!(empty.report.current_count(), 0);
    assert_eq!(empty.report.removed_count(), 0);
    assert_eq!(empty.report.unavailable_count(), 0);
    assert_eq!(empty.report.checked_count(), 0);
}
