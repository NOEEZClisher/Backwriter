use std::fs;

use backwriter::backwriter::anddress::{
    ANDDRESS_VERSION, Anddress, AnddressTarget, LineTerminator,
};
use backwriter::backwriter::search::{
    SearchError, SearchInputError, SearchOutcome, SearchQuery, SearchRequest, SearchScope,
    SearchScopeEntry, SearchTarget,
};
use backwriter::runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime};
use tempfile::tempdir;

mod support;

fn runtime(root: &std::path::Path) -> WorkspaceRuntime {
    WorkspaceRuntime::open(
        root,
        WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
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

fn found(runtime: &WorkspaceRuntime, query: &str, target: SearchTarget) -> Vec<Anddress> {
    match runtime
        .search(&SearchRequest::new(
            SearchQuery::new(query).unwrap(),
            SearchScope::all_admitted(),
            target,
        ))
        .unwrap()
    {
        SearchOutcome::Found { anddresses } => anddresses,
        SearchOutcome::Empty => Vec::new(),
    }
}

fn request(query: &str, scope: SearchScope, target: SearchTarget) -> SearchRequest {
    SearchRequest::new(SearchQuery::new(query).unwrap(), scope, target)
}

fn exact_file(runtime: &WorkspaceRuntime, logical_path: &str) -> SearchOutcome {
    runtime
        .search(&SearchRequest::exact_file(logical_path).unwrap())
        .unwrap()
}

fn exact_file_address(runtime: &WorkspaceRuntime, logical_path: &str) -> Anddress {
    let SearchOutcome::Found { mut anddresses } = exact_file(runtime, logical_path) else {
        panic!("exact File")
    };
    assert_eq!(anddresses.len(), 1);
    anddresses.pop().unwrap()
}

#[test]
fn public_results_preserve_anddress_ownership_and_geometry() {
    let workspace = "a".repeat(64);
    let address = |target| support::address(&workspace, "note.txt", b"text", target, 0, 4);
    let file = address(AnddressTarget::File);
    let paragraph = address(AnddressTarget::Paragraph);
    let line = address(AnddressTarget::Line);

    let outcome = SearchOutcome::Found {
        anddresses: vec![file.clone(), paragraph.clone(), line.clone()],
    };
    assert_eq!(outcome.clone(), outcome);
    let SearchOutcome::Found { anddresses } = outcome else {
        unreachable!()
    };
    assert_eq!(anddresses, vec![file, paragraph.clone(), line.clone()]);
    assert_eq!(paragraph.line_range(), 0..1);
    assert_eq!(line.line_number(), Some(1));
    assert_eq!(line.parent(), Some(paragraph));
}

#[test]
fn host_proofs_are_path_runtime_admission_mode_and_drop_isolated() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir_all(root.join("docs")).unwrap();
    fs::write(root.join("a.txt"), b"same\n").unwrap();
    fs::write(root.join("b.txt"), b"same\n").unwrap();
    fs::write(root.join("docs/note.txt"), b"same\n").unwrap();

    let mut paths = host_runtime(&root, ".");
    let a = exact_file_address(&paths, "a.txt");
    let b = exact_file_address(&paths, "b.txt");
    assert_eq!(a.source_state_hash(), b.source_state_hash());
    paths.invalidate_source("a.txt").unwrap();
    fs::remove_file(root.join("a.txt")).unwrap();
    fs::remove_file(root.join("b.txt")).unwrap();
    assert_eq!(paths.check(a).unwrap().filtered, None);
    assert_eq!(paths.check(b.clone()).unwrap().filtered, Some(b));

    let host = host_runtime(&root, ".");
    let current = exact_file_address(&host, "docs/note.txt");
    let named = host_runtime(&root, "docs");
    let untrusted = runtime(&root);
    fs::remove_file(root.join("docs/note.txt")).unwrap();

    assert_eq!(
        host.check(current.clone()).unwrap().filtered,
        Some(current.clone())
    );
    assert_eq!(named.check(current.clone()).unwrap().filtered, None);
    assert_eq!(untrusted.check(current.clone()).unwrap().filtered, None);
    drop(host);
    assert_eq!(
        host_runtime(&root, ".").check(current).unwrap().filtered,
        None
    );
}

#[test]
fn search_projects_exact_source_state_and_byte_ranges() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), "needle\r\nneedle x\n\nneedle").unwrap();
    let workspace = runtime(&root);
    let lines = found(&workspace, "needle", SearchTarget::Line);
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0].logical_path(), "note.txt");
    assert_eq!(lines[0].target(), AnddressTarget::Line);
    assert_eq!((lines[0].byte_start(), lines[0].byte_end()), (0, 8));
    assert_eq!((lines[1].byte_start(), lines[1].byte_end()), (18, 24));
    assert_eq!((lines[2].byte_start(), lines[2].byte_end()), (8, 17));
    assert!(lines.iter().all(|value| value.source_byte_length() == 24));
    assert!(lines.iter().all(|value| value.source_line_count() == 4));
    assert_eq!(
        lines.iter().map(Anddress::line_number).collect::<Vec<_>>(),
        vec![Some(1), Some(4), Some(2)]
    );
    assert_eq!(
        lines.iter().map(Anddress::terminator).collect::<Vec<_>>(),
        vec![
            Some(LineTerminator::Crlf),
            Some(LineTerminator::None),
            Some(LineTerminator::Lf),
        ]
    );
    assert!(
        lines
            .windows(2)
            .all(|pair| { pair[0].source_state_hash() == pair[1].source_state_hash() })
    );
    let paragraphs = found(&workspace, "needle", SearchTarget::Paragraph);
    assert_eq!(
        paragraphs
            .iter()
            .map(|value| (
                value.target(),
                value.range(),
                value.line_range(),
                value.line_count(),
            ))
            .collect::<Vec<_>>(),
        vec![
            (AnddressTarget::Paragraph, 0..17, 0..2, 2),
            (AnddressTarget::Paragraph, 18..24, 3..4, 1),
        ]
    );
    for (line, expected_parent) in lines.iter().zip([
        paragraphs[0].clone(),
        paragraphs[1].clone(),
        paragraphs[0].clone(),
    ]) {
        assert_eq!(line.parent(), Some(expected_parent));
    }
    let files = found(&workspace, "needle", SearchTarget::File);
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].range(), 0..24);
    assert_eq!(files[0].line_range(), 0..4);
}

#[test]
fn exact_file_lookup_is_content_independent_and_integrates_with_check() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("empty.txt"), "").unwrap();
    fs::write(root.join("nonempty.txt"), "unrelated content\n").unwrap();
    fs::create_dir(root.join("directory")).unwrap();
    fs::create_dir_all(root.join(".artext/bw")).unwrap();
    fs::write(root.join(".artext/bw/hidden.txt"), "ordinary").unwrap();
    let workspace = runtime(&root);

    let empty = exact_file(&workspace, "empty.txt");
    let SearchOutcome::Found { anddresses } = &empty else {
        panic!("empty File lookup")
    };
    assert_eq!(anddresses.len(), 1);
    assert_eq!(anddresses[0].logical_path(), "empty.txt");
    assert_eq!(anddresses[0].target(), AnddressTarget::File);
    assert_eq!(
        (anddresses[0].byte_start(), anddresses[0].byte_end()),
        (0, 0)
    );

    let nonempty = exact_file(&workspace, "nonempty.txt");
    let SearchOutcome::Found { anddresses } = &nonempty else {
        panic!("nonempty File lookup")
    };
    assert_eq!(anddresses.len(), 1);
    assert_eq!(anddresses[0].logical_path(), "nonempty.txt");
    assert_eq!(anddresses[0].target(), AnddressTarget::File);

    assert_eq!(exact_file(&workspace, "missing.txt"), SearchOutcome::Empty);
    assert_eq!(exact_file(&workspace, "directory"), SearchOutcome::Empty);
    assert_eq!(
        exact_file(&workspace, ".artext/bw/hidden.txt"),
        SearchOutcome::Empty
    );

    let checked = workspace.check_search(empty).unwrap();
    assert_eq!(checked.report.current_count(), 1);
    assert_eq!(checked.report.checked_count(), 1);
    assert!(matches!(
        checked.filtered,
        SearchOutcome::Found { ref anddresses }
            if anddresses.len() == 1
                && anddresses[0].logical_path() == "empty.txt"
    ));
}

#[test]
fn exact_file_lookup_reuses_path_admission_and_source_safety() {
    for path in [
        "",
        ".",
        "../escape.txt",
        "/absolute.txt",
        "a/../b.txt",
        "a\\b.txt",
        ".git/config",
    ] {
        assert_eq!(
            SearchRequest::exact_file(path),
            Err(SearchInputError::InvalidFile)
        );
    }

    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::create_dir(root.join("a")).unwrap();
    fs::create_dir(root.join("b")).unwrap();
    fs::write(root.join("a/same.txt"), "left").unwrap();
    fs::write(root.join("b/same.txt"), "right").unwrap();
    fs::write(root.join("outside.txt"), "outside").unwrap();
    fs::write(root.join("a/invalid.txt"), b"\xff").unwrap();
    fs::write(root.join("a/zero.txt"), b"text\0").unwrap();
    let workspace = WorkspaceRuntime::open(
        &root,
        WorkspaceAdmission::new([
            AdmissionRoot::new("a").unwrap(),
            AdmissionRoot::new("b").unwrap(),
        ])
        .unwrap(),
    )
    .unwrap();

    let left = exact_file(&workspace, "a/same.txt");
    let right = exact_file(&workspace, "b/same.txt");
    let (
        SearchOutcome::Found {
            anddresses: left_occurrences,
        },
        SearchOutcome::Found {
            anddresses: right_occurrences,
        },
    ) = (&left, &right)
    else {
        panic!("named admission File lookup")
    };
    assert_eq!(left_occurrences.len(), 1);
    assert_eq!(right_occurrences.len(), 1);
    assert_eq!(left_occurrences[0].logical_path(), "a/same.txt");
    assert_eq!(right_occurrences[0].logical_path(), "b/same.txt");
    assert_eq!(
        left_occurrences[0].workspace_coordinate(),
        right_occurrences[0].workspace_coordinate()
    );
    assert_eq!(
        workspace.search(&SearchRequest::exact_file("outside.txt").unwrap()),
        Err(SearchError::InvalidScope)
    );
    assert_eq!(
        workspace.search(&SearchRequest::exact_file("a/invalid.txt").unwrap()),
        Err(SearchError::Unavailable)
    );
    assert_eq!(
        workspace.search(&SearchRequest::exact_file("a/zero.txt").unwrap()),
        Err(SearchError::Unavailable)
    );
}

#[test]
fn wire_is_flat_strict_and_version_priority_is_explicit() {
    let coordinate = "a".repeat(64);
    let hash = support::source_hash(b"b\n");
    let address = support::line(&coordinate, "note.txt", b"b\n", 0);
    assert_eq!(
        String::from_utf8(address.encode().unwrap()).unwrap(),
        format!(
            "{{\"version\":\"{}\",\"workspaceCoordinate\":\"{}\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"{}\",\"sourceByteLength\":\"2\",\"sourceLineCount\":\"1\",\"kind\":\"line\",\"byteStart\":\"0\",\"byteEnd\":\"2\",\"terminator\":\"lf\",\"lineOffsetInParent\":\"0\",\"parentKind\":\"paragraph\",\"parentByteStart\":\"0\",\"parentByteEnd\":\"2\",\"parentFileLineOffset\":\"0\",\"parentLineCount\":\"1\"}}",
            ANDDRESS_VERSION, coordinate, hash,
        )
    );
    let file = support::file(&coordinate, "note.txt", b"b\n");
    assert_eq!(
        Anddress::decode(
            format!(
                r#"{{ "kind" : "file", "logicalPath" : "note.txt", "workspaceCoordinate" : "{coordinate}", "version" : "{ANDDRESS_VERSION}", "sourceStateHash" : "{hash}", "sourceLineCount" : "1", "sourceByteLength" : "2" }}"#
            )
            .as_bytes(),
        )
        .unwrap(),
        file
    );
    assert_eq!(
        Anddress::decode(br#"{"version":"old","kind":null}"#),
        Err(backwriter::backwriter::anddress::AnddressError::UnsupportedVersion)
    );
    assert_eq!(Anddress::decode(br#"{"version":"artext.backwriter-anddress.v4","workspaceCoordinate":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","logicalPath":"note.txt","kind":"file","version":"artext.backwriter-anddress.v4"}"#), Err(backwriter::backwriter::anddress::AnddressError::Encoding));
    for version in [
        "artext.backwriter-anddress.v4",
        "artext.backwriter-anddress.v3",
    ] {
        assert_eq!(
            Anddress::decode(format!(r#"{{"version":"{version}"}}"#).as_bytes()),
            Err(backwriter::backwriter::anddress::AnddressError::UnsupportedVersion)
        );
    }
}

#[test]
fn wire_rejects_missing_unknown_wrong_type_and_noncanonical_ranges() {
    use backwriter::backwriter::anddress::AnddressError;
    let base = format!(
        "{{\"version\":\"{ANDDRESS_VERSION}\",\"workspaceCoordinate\":\"{}\",\"logicalPath\":\"note.txt\",\"sourceStateHash\":\"{}\",\"sourceByteLength\":\"2\",\"sourceLineCount\":\"1\",",
        "a".repeat(64),
        "b".repeat(64)
    );
    assert_eq!(
        Anddress::decode(format!("{base}\"kind\":null}}").as_bytes()),
        Err(AnddressError::Encoding)
    );
    assert_eq!(
        Anddress::decode(format!("{base}\"kind\":\"file\",\"unknown\":\"x\"}}").as_bytes()),
        Err(AnddressError::Encoding)
    );
    for encoded in [
        format!("{base}\"kind\":\"paragraph\",\"byteStart\":\"0\"}}"),
        format!("{base}\"kind\":0}}"),
        format!("{base}\"kind\":\"file\",\"kind\":\"file\"}}"),
    ] {
        assert_eq!(
            Anddress::decode(encoded.as_bytes()),
            Err(AnddressError::Encoding)
        );
    }
    for (start, end, expected) in [
        ("01", "2", AnddressError::Encoding),
        ("-1", "2", AnddressError::Encoding),
        ("2", "1", AnddressError::Invalid),
        ("0", "3", AnddressError::Invalid),
    ] {
        assert_eq!(
            Anddress::decode(
                format!(
                    "{base}\"kind\":\"line\",\"byteStart\":\"{start}\",\"byteEnd\":\"{end}\",\"terminator\":\"none\",\"lineOffsetInParent\":\"0\",\"parentKind\":\"file\"}}"
                )
                .as_bytes()
            ),
            Err(expected)
        );
    }
}

#[test]
fn wire_ignores_large_invalid_values_without_materializing_them() {
    use backwriter::backwriter::anddress::AnddressError;

    let large = "x".repeat(65_536);
    let valid = format!(
        r#"{{"version":"{}","workspaceCoordinate":"{}","logicalPath":"note.txt","sourceStateHash":"{}","sourceByteLength":"0","sourceLineCount":"0","kind":"file""#,
        ANDDRESS_VERSION,
        "a".repeat(64),
        "b".repeat(64),
    );

    for encoded in [
        format!(r#"{valid},"unknown":"{large}"}}"#),
        format!(r#"{valid},"unknown":["{large}",{{"nested":"{large}"}}]}}"#),
        format!(r#"{valid},"unknown":{{"nested":["{large}"]}}}}"#),
        format!(r#"{valid},"workspaceCoordinate":{{"nested":"{large}"}}}}"#),
    ] {
        assert_eq!(
            Anddress::decode(encoded.as_bytes()),
            Err(AnddressError::Encoding)
        );
    }

    assert_eq!(
        Anddress::decode(
            format!(r#"{{"version":"old","kind":{{"nested":"{large}"}}}}"#).as_bytes()
        ),
        Err(AnddressError::UnsupportedVersion)
    );
    assert_eq!(
        Anddress::decode(
            format!(r#"{{"version":"old","version":{{"nested":"{large}"}}}}"#).as_bytes()
        ),
        Err(AnddressError::Encoding)
    );

    let production = include_str!("../src/backwriter/anddress.rs");
    assert_eq!(production.matches("serde_json::Value").count(), 0);
}

#[test]
fn search_handles_separators_repeated_occurrences_ordering_and_fail_all() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::create_dir(root.join("docs")).unwrap();
    fs::write(root.join("z.txt"), "needle needle\nneedle\n\nneedle").unwrap();
    fs::write(root.join("docs/a.txt"), "needle\n").unwrap();
    let workspace = runtime(&root);
    let lines = found(&workspace, "needle", SearchTarget::Line);
    assert_eq!(
        lines
            .iter()
            .map(|value| value.logical_path())
            .collect::<Vec<_>>(),
        vec!["docs/a.txt", "z.txt", "z.txt", "z.txt"]
    );
    assert_eq!(lines[3].target(), AnddressTarget::Line);
    assert_eq!((lines[3].byte_start(), lines[3].byte_end()), (0, 14));
    assert_eq!(
        found(&workspace, "needle", SearchTarget::Paragraph).len(),
        3
    );
    assert_eq!(found(&workspace, "needle", SearchTarget::File).len(), 2);
    fs::write(root.join("invalid.txt"), b"needle\xff").unwrap();
    assert_eq!(
        workspace.search(&SearchRequest::new(
            SearchQuery::new("needle").unwrap(),
            SearchScope::all_admitted(),
            SearchTarget::Line
        )),
        Err(SearchError::Unavailable)
    );
}

#[test]
fn search_projects_ranges_after_many_lines_and_ignores_only_backwriter_spill() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let mut source = String::new();
    for _ in 0..4097 {
        source.push_str("skip\n");
    }
    source.push_str("needle");
    fs::write(root.join("many.txt"), source).unwrap();
    fs::create_dir_all(root.join(".artext/bw")).unwrap();
    fs::write(root.join(".artext/bw/hidden.txt"), "needle").unwrap();
    fs::create_dir_all(root.join(".artext/other")).unwrap();
    fs::write(root.join(".artext/other/visible.txt"), "needle").unwrap();
    fs::create_dir_all(root.join(".artext/bw2")).unwrap();
    fs::write(root.join(".artext/bw2/visible.txt"), "needle").unwrap();
    fs::create_dir_all(root.join("nested/.artext/bw")).unwrap();
    fs::write(root.join("nested/.artext/bw/visible.txt"), "needle").unwrap();
    let values = found(&runtime(&root), "needle", SearchTarget::Line);
    assert_eq!(values.len(), 4);
    assert_eq!(values[0].logical_path(), ".artext/bw2/visible.txt");
    assert_eq!(values[1].logical_path(), ".artext/other/visible.txt");
    assert_eq!(values[2].logical_path(), "many.txt");
    assert_eq!(values[2].target(), AnddressTarget::Line);
    assert_eq!(
        (values[2].byte_start(), values[2].byte_end()),
        (4097 * 5, 4097 * 5 + 6)
    );
    assert_eq!(values[3].logical_path(), "nested/.artext/bw/visible.txt");
}

#[test]
fn search_matches_separator_as_line_and_file_but_not_paragraph() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), "text\n \t\rnext").unwrap();
    let workspace = runtime(&root);
    let lines = found(&workspace, " \t", SearchTarget::Line);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].target(), AnddressTarget::Line);
    assert_eq!((lines[0].byte_start(), lines[0].byte_end()), (5, 8));
    assert_eq!(lines[0].terminator(), Some(LineTerminator::Cr));
    assert_eq!(lines[0].line_number(), Some(2));
    assert_eq!(lines[0].parent().unwrap().target(), AnddressTarget::File);
    assert_eq!(lines[0].project(AnddressTarget::Paragraph).unwrap(), None);
    assert!(found(&workspace, " \t", SearchTarget::Paragraph).is_empty());
    assert_eq!(found(&workspace, " \t", SearchTarget::File).len(), 1);
}

#[test]
fn search_promotes_parents_to_full_line_tier() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("a.txt"), "prefix needle\n").unwrap();
    fs::write(root.join("z.txt"), "prefix needle\nneedle\n").unwrap();
    let workspace = runtime(&root);
    for target in [SearchTarget::Paragraph, SearchTarget::File] {
        let values = found(&workspace, "needle", target);
        assert_eq!(
            values
                .iter()
                .map(|value| value.logical_path())
                .collect::<Vec<_>>(),
            vec!["z.txt", "a.txt"]
        );
    }
}

#[test]
fn search_keeps_identical_extents_distinct_by_range_and_bare_cr() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), "needle\rneedle\r").unwrap();
    let values = found(&runtime(&root), "needle", SearchTarget::Line);
    assert_eq!(values.len(), 2);
    assert_eq!(values[0].target(), AnddressTarget::Line);
    assert_eq!((values[0].byte_start(), values[0].byte_end()), (0, 7));
    assert_eq!((values[1].byte_start(), values[1].byte_end()), (7, 14));
}

#[test]
fn search_selected_source_failure_discards_provisional_results() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("a.txt"), "needle").unwrap();
    let scope = SearchScope::only([
        SearchScopeEntry::source("a.txt").unwrap(),
        SearchScopeEntry::source("missing.txt").unwrap(),
    ])
    .unwrap();
    assert_eq!(
        runtime(&root).search(&request("needle", scope, SearchTarget::Line)),
        Err(SearchError::Unavailable)
    );
}

#[test]
fn search_scope_overlap_preflight_is_component_aware() {
    assert_eq!(
        SearchScope::only([
            SearchScopeEntry::source("docs/a.txt").unwrap(),
            SearchScopeEntry::subtree("docs").unwrap(),
        ]),
        Err(SearchInputError::InvalidScope)
    );
    assert!(
        SearchScope::only([
            SearchScopeEntry::source("docs-a.txt").unwrap(),
            SearchScopeEntry::subtree("docs").unwrap(),
        ])
        .is_ok()
    );
}

#[test]
fn search_rejects_invalid_query_and_scope_before_access_and_keeps_canonical_scope_order() {
    for query in ["needle\0", "needle\r", "needle\n"] {
        assert_eq!(SearchQuery::new(query), Err(SearchInputError::InvalidQuery));
    }
    assert_eq!(
        SearchScopeEntry::source("."),
        Err(SearchInputError::InvalidScope)
    );
    let a = SearchScopeEntry::source("a.txt").unwrap();
    assert_eq!(
        SearchScope::only([a.clone(), a]),
        Err(SearchInputError::InvalidScope)
    );
    assert_eq!(
        SearchScope::only([
            SearchScopeEntry::source("a.txt").unwrap(),
            SearchScopeEntry::subtree("a.txt").unwrap(),
        ]),
        Err(SearchInputError::InvalidScope)
    );

    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("a.txt"), "needle").unwrap();
    fs::write(root.join("z.txt"), "needle").unwrap();
    fs::write(root.join("invalid.txt"), b"needle\xff").unwrap();
    let sorted_scope = SearchScope::only([
        SearchScopeEntry::source("a.txt").unwrap(),
        SearchScopeEntry::source("z.txt").unwrap(),
    ])
    .unwrap();
    let scope = SearchScope::only([
        SearchScopeEntry::source("z.txt").unwrap(),
        SearchScopeEntry::source("a.txt").unwrap(),
    ])
    .unwrap();
    assert_eq!(scope, sorted_scope);
    let SearchOutcome::Found { anddresses } = runtime(&root)
        .search(&request("needle", scope, SearchTarget::Line))
        .unwrap()
    else {
        panic!("canonical selected results")
    };
    assert_eq!(
        anddresses
            .iter()
            .map(|value| value.logical_path())
            .collect::<Vec<_>>(),
        vec!["a.txt", "z.txt"]
    );
}

#[test]
fn search_returns_empty_and_fails_all_for_selected_nul_source() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("empty.txt"), "ordinary").unwrap();
    assert_eq!(
        runtime(&root).search(&request(
            "needle",
            SearchScope::only([SearchScopeEntry::source("empty.txt").unwrap()]).unwrap(),
            SearchTarget::Line,
        )),
        Ok(SearchOutcome::Empty)
    );
    fs::write(root.join("binary.txt"), b"needle\0").unwrap();
    assert_eq!(
        runtime(&root).search(&request(
            "needle",
            SearchScope::only([
                SearchScopeEntry::source("empty.txt").unwrap(),
                SearchScopeEntry::source("binary.txt").unwrap(),
            ])
            .unwrap(),
            SearchTarget::Line,
        )),
        Err(SearchError::Unavailable)
    );
}

#[test]
fn search_discards_provisional_targets_after_late_invalid_source_bytes() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let mut invalid = b"needle\n".to_vec();
    invalid.extend(std::iter::repeat_n(b'x', 16_384));
    invalid.push(0xff);
    fs::write(root.join("invalid.txt"), invalid).unwrap();
    let mut nul = b"needle\n".to_vec();
    nul.extend(std::iter::repeat_n(b'y', 16_384));
    nul.push(0);
    fs::write(root.join("zero.txt"), nul).unwrap();
    let workspace = runtime(&root);

    for target in [
        SearchTarget::Line,
        SearchTarget::Paragraph,
        SearchTarget::File,
    ] {
        for path in ["invalid.txt", "zero.txt"] {
            assert_eq!(
                workspace.search(&request(
                    "needle",
                    SearchScope::only([SearchScopeEntry::source(path).unwrap()]).unwrap(),
                    target,
                )),
                Err(SearchError::Unavailable)
            );
        }
    }
}

#[test]
fn search_has_no_fixed_query_path_scope_depth_or_result_limit() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    let query = "n".repeat(4097);
    fs::write(root.join("long.txt"), &query).unwrap();
    assert_eq!(found(&runtime(&root), &query, SearchTarget::Line).len(), 1);
    assert!(SearchScopeEntry::source("p".repeat(4097)).is_ok());
    let mut entries = Vec::new();
    for index in 0..257 {
        let path = format!("scope-{index:03}.txt");
        fs::write(root.join(&path), "needle").unwrap();
        entries.push(SearchScopeEntry::source(path).unwrap());
    }
    let scope = SearchScope::only(entries).unwrap();
    let SearchOutcome::Found { anddresses } = runtime(&root)
        .search(&request("needle", scope, SearchTarget::Line))
        .unwrap()
    else {
        panic!("scoped results")
    };
    assert_eq!(anddresses.len(), 257);
    let mut nested = root.clone();
    let mut logical_path = String::new();
    for index in 0..64 {
        let component = format!("d{index}");
        fs::create_dir(nested.join(&component)).unwrap();
        nested.push(&component);
        if !logical_path.is_empty() {
            logical_path.push('/');
        }
        logical_path.push_str(&component);
    }
    fs::write(nested.join("deep.txt"), "needle").unwrap();
    logical_path.push_str("/deep.txt");
    assert_eq!(
        found(&runtime(&root), "needle", SearchTarget::Line)
            .iter()
            .filter(|value| value.logical_path() == logical_path)
            .count(),
        1
    );
    fs::write(root.join("many.txt"), "many\n".repeat(4097)).unwrap();
    let scope = SearchScope::only([SearchScopeEntry::source("many.txt").unwrap()]).unwrap();
    let SearchOutcome::Found { anddresses } = runtime(&root)
        .search(&request("many", scope, SearchTarget::Line))
        .unwrap()
    else {
        panic!("many results")
    };
    assert_eq!(anddresses.len(), 4097);
}

#[cfg(unix)]
#[test]
fn search_rejects_symlinks_and_keeps_hard_links_as_distinct_logical_paths() {
    use std::os::unix::fs::symlink;

    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("source.txt"), "needle").unwrap();
    fs::hard_link(root.join("source.txt"), root.join("hard.txt")).unwrap();
    let outside = fixture.path().join("outside.txt");
    fs::write(&outside, "needle").unwrap();
    symlink(&outside, root.join("link.txt")).unwrap();
    let values = found(&runtime(&root), "needle", SearchTarget::File);
    assert_eq!(
        values
            .iter()
            .map(|value| value.logical_path())
            .collect::<Vec<_>>(),
        vec!["hard.txt", "source.txt"]
    );
    assert_eq!(
        runtime(&root).search(&request(
            "needle",
            SearchScope::only([SearchScopeEntry::source("link.txt").unwrap()]).unwrap(),
            SearchTarget::File,
        )),
        Err(SearchError::Unavailable)
    );
    assert_eq!(
        exact_file(&runtime(&root), "link.txt"),
        SearchOutcome::Empty
    );
    for path in ["hard.txt", "source.txt"] {
        let SearchOutcome::Found { anddresses } = exact_file(&runtime(&root), path) else {
            panic!("hard-link logical File lookup")
        };
        assert_eq!(anddresses.len(), 1);
        assert_eq!(anddresses[0].logical_path(), path);
        assert_eq!(anddresses[0].target(), AnddressTarget::File);
    }
}
