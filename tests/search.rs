use std::fs;

use artext::backwriter::anddress::{ANDDRESS_VERSION, Anddress, AnddressTarget, Natural};
use artext::backwriter::search::{
    SearchError, SearchInputError, SearchOutcome, SearchQuery, SearchRequest, SearchScope,
    SearchScopeEntry, SearchTarget,
};
use artext::runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime};
use tempfile::tempdir;

fn runtime(root: &std::path::Path) -> WorkspaceRuntime {
    WorkspaceRuntime::open(
        root,
        WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
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

#[test]
fn search_projects_exact_line_extent_and_paragraph_ordinal() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), "needle\r\nneedle x\n\nneedle").unwrap();
    let workspace = runtime(&root);
    let lines = found(&workspace, "needle", SearchTarget::Line);
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0].logical_path, "note.txt");
    assert_eq!(
        lines[0].target,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "needle\r\n".to_owned()
        }
    );
    assert_eq!(
        lines[1].target,
        AnddressTarget::Line {
            ordinal: Natural::parse("3").unwrap(),
            exact_extent: "needle".to_owned()
        }
    );
    assert_eq!(
        lines[2].target,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "needle x\n".to_owned()
        }
    );
    let paragraphs = found(&workspace, "needle", SearchTarget::Paragraph);
    assert_eq!(
        paragraphs.into_iter().map(|a| a.target).collect::<Vec<_>>(),
        vec![
            AnddressTarget::Paragraph {
                ordinal: Natural::zero()
            },
            AnddressTarget::Paragraph {
                ordinal: Natural::one()
            }
        ]
    );
    assert_eq!(found(&workspace, "needle", SearchTarget::File).len(), 1);
}

#[test]
fn wire_is_flat_strict_and_version_priority_is_explicit() {
    let address = Anddress {
        version: ANDDRESS_VERSION.to_owned(),
        workspace_coordinate: "a".repeat(64),
        logical_path: "note.txt".to_owned(),
        target: AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "x\n".to_owned(),
        },
    };
    assert_eq!(
        String::from_utf8(address.encode().unwrap()).unwrap(),
        format!(
            "{{\"version\":\"{}\",\"workspaceCoordinate\":\"{}\",\"logicalPath\":\"note.txt\",\"kind\":\"line\",\"ordinal\":\"0\",\"exactExtent\":\"x\\n\"}}",
            ANDDRESS_VERSION,
            "a".repeat(64)
        )
    );
    for (address, expected) in [
        (
            Anddress {
                version: ANDDRESS_VERSION.to_owned(),
                workspace_coordinate: "a".repeat(64),
                logical_path: "note.txt".to_owned(),
                target: AnddressTarget::File,
            },
            format!(
                "{{\"version\":\"{}\",\"workspaceCoordinate\":\"{}\",\"logicalPath\":\"note.txt\",\"kind\":\"file\"}}",
                ANDDRESS_VERSION,
                "a".repeat(64)
            ),
        ),
        (
            Anddress {
                version: ANDDRESS_VERSION.to_owned(),
                workspace_coordinate: "a".repeat(64),
                logical_path: "note.txt".to_owned(),
                target: AnddressTarget::Paragraph {
                    ordinal: Natural::zero(),
                },
            },
            format!(
                "{{\"version\":\"{}\",\"workspaceCoordinate\":\"{}\",\"logicalPath\":\"note.txt\",\"kind\":\"paragraph\",\"ordinal\":\"0\"}}",
                ANDDRESS_VERSION,
                "a".repeat(64)
            ),
        ),
    ] {
        assert_eq!(
            String::from_utf8(address.encode().unwrap()).unwrap(),
            expected
        );
    }
    assert_eq!(
        Anddress::decode(
            br#"{ "kind" : "file", "logicalPath" : "note.txt", "workspaceCoordinate" : "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "version" : "artext.backwriter-anddress.v3" }"#,
        )
        .unwrap(),
        Anddress {
            version: ANDDRESS_VERSION.to_owned(),
            workspace_coordinate: "a".repeat(64),
            logical_path: "note.txt".to_owned(),
            target: AnddressTarget::File,
        }
    );
    assert_eq!(
        Anddress::decode(br#"{"version":"old","kind":null}"#),
        Err(artext::backwriter::anddress::AnddressError::UnsupportedVersion)
    );
    assert_eq!(Anddress::decode(br#"{"version":"artext.backwriter-anddress.v3","workspaceCoordinate":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","logicalPath":"note.txt","kind":"file","version":"artext.backwriter-anddress.v3"}"#), Err(artext::backwriter::anddress::AnddressError::Encoding));
    assert_eq!(Anddress::decode(br#"{"version":"artext.backwriter-anddress.v3","workspaceCoordinate":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","logicalPath":"note.txt","kind":"line","ordinal":"01","exactExtent":"x"}"#), Err(artext::backwriter::anddress::AnddressError::Encoding));
}

#[test]
fn wire_rejects_null_unknown_and_invalid_extents_without_natural_narrowing() {
    use artext::backwriter::anddress::AnddressError;
    let base = "{\"version\":\"artext.backwriter-anddress.v3\",\"workspaceCoordinate\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\",\"logicalPath\":\"note.txt\",";
    assert_eq!(
        Anddress::decode(format!("{base}\"kind\":null}}").as_bytes()),
        Err(AnddressError::Encoding)
    );
    assert_eq!(
        Anddress::decode(format!("{base}\"kind\":\"file\",\"unknown\":\"x\"}}").as_bytes()),
        Err(AnddressError::Encoding)
    );
    for encoded in [
        "{\"version\":\"artext.backwriter-anddress.v3\",\"workspaceCoordinate\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\",\"kind\":\"file\"}",
        "{\"version\":\"artext.backwriter-anddress.v3\",\"workspaceCoordinate\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\",\"logicalPath\":\"note.txt\",\"kind\":0}",
        "{\"version\":\"artext.backwriter-anddress.v3\",\"workspaceCoordinate\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\",\"logicalPath\":\"note.txt\",\"kind\":\"file\",\"kind\":\"file\"}",
    ] {
        assert_eq!(
            Anddress::decode(encoded.as_bytes()),
            Err(AnddressError::Encoding)
        );
    }
    let huge = format!("1{}", "0".repeat(4097));
    assert_eq!(
        Anddress::decode(
            format!("{base}\"kind\":\"line\",\"ordinal\":\"{huge}\",\"exactExtent\":\"x\\n\"}}")
                .as_bytes()
        )
        .unwrap()
        .target,
        AnddressTarget::Line {
            ordinal: Natural::parse(&huge).unwrap(),
            exact_extent: "x\n".to_owned()
        }
    );
    assert_eq!(
        Anddress::decode(
            format!("{base}\"kind\":\"line\",\"ordinal\":\"0\",\"exactExtent\":\"x\\ny\"}}")
                .as_bytes()
        ),
        Err(AnddressError::Invalid)
    );
    let unsupported = Anddress {
        version: "old".to_owned(),
        workspace_coordinate: "a".repeat(64),
        logical_path: "note.txt".to_owned(),
        target: AnddressTarget::File,
    };
    assert_eq!(
        unsupported.validate(),
        Err(AnddressError::UnsupportedVersion)
    );
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
            .map(|value| value.logical_path.as_str())
            .collect::<Vec<_>>(),
        vec!["docs/a.txt", "z.txt", "z.txt", "z.txt"]
    );
    assert_eq!(
        lines[3].target,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "needle needle\n".to_owned()
        }
    );
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
fn search_projects_unbounded_line_ordinals_and_ignores_only_backwriter_spill() {
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
    assert_eq!(values[0].logical_path, ".artext/bw2/visible.txt");
    assert_eq!(values[1].logical_path, ".artext/other/visible.txt");
    assert_eq!(values[2].logical_path, "many.txt");
    assert_eq!(
        values[2].target,
        AnddressTarget::Line {
            ordinal: Natural::parse("4097").unwrap(),
            exact_extent: "needle".to_owned()
        }
    );
    assert_eq!(values[3].logical_path, "nested/.artext/bw/visible.txt");
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
    assert_eq!(
        lines[0].target,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: " \t\r".to_owned(),
        }
    );
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
                .map(|value| value.logical_path.as_str())
                .collect::<Vec<_>>(),
            vec!["z.txt", "a.txt"]
        );
    }
}

#[test]
fn search_keeps_identical_extents_distinct_by_line_ordinal_and_bare_cr() {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("note.txt"), "needle\rneedle\r").unwrap();
    let values = found(&runtime(&root), "needle", SearchTarget::Line);
    assert_eq!(values.len(), 2);
    assert_eq!(
        values[0].target,
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "needle\r".to_owned(),
        }
    );
    assert_eq!(
        values[1].target,
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "needle\r".to_owned(),
        }
    );
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
            .map(|value| value.logical_path.as_str())
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
            .filter(|value| value.logical_path == logical_path)
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
            .map(|value| value.logical_path.as_str())
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
}
