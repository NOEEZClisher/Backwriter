use super::*;

#[test]
fn session_reuses_search_projection_view_and_check_with_exact_bindings() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\nneedle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle --source note.txt\nlet copied_hits = @hits\nlet second = @copied_hits[1]\nlet copied_second = @second\nview anddress @copied_second\ncheck anddress @hits[1]\nexit\n",
    );
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"Found 2\n0\tLine\tnote.txt:1\n1\tLine\tnote.txt:2\nneedle\nCurrent\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn shell_local_references_start_at_zero_append_in_order_and_keep_named_raw_aliases() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\nneedle\n");

    let output = run_shell(
        root.path(),
        "search line absent\nsearch line needle\nlet primary = @1\nview anddress @primary\nview @0 @0\nview @0 @1 --as paragraph\nexit\n",
    );
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"@0\tLine\tnote.txt:1\n@1\tLine\tnote.txt:2\nneedle\nView\t@0\tbytes=7\n@2\tLine\tnote.txt:1\nneedle\n\nEndView\nView\t@0\tbytes=7\n@3\tLine\tnote.txt:1\nneedle\n\nEndView\nView\t@0\tbytes=14\n@4\tParagraph\tnote.txt:1-2\nneedle\nneedle\n\nEndView\nView\t@1\tbytes=14\n@5\tParagraph\tnote.txt:1-2\nneedle\nneedle\n\nEndView\n"
    );
    assert!(output.stderr.is_empty());

    let source = include_str!("../../src/bin/bw/shell.rs");
    assert_eq!(source.matches("fn reserve_session_refs").count(), 1);
    let search = source
        .split_once("fn execute_session_search")
        .unwrap()
        .1
        .split_once("fn execute_session_replace")
        .unwrap()
        .0;
    assert!(
        search
            .find("reserve_session_refs(refs, anddresses.len())")
            .unwrap()
            < search.find("refs.extend(anddresses)").unwrap()
    );
    let replace = source
        .split_once("fn execute_session_replace")
        .unwrap()
        .1
        .split_once("fn execute_session_ref_view")
        .unwrap()
        .0;
    assert!(replace.contains("prepare_replace_content(&target, tokens[2].clone())"));
    assert!(
        replace.find("reserve_session_refs(refs, 1)").unwrap()
            < replace.find(".apply_replace(&edit)").unwrap()
    );
    let view = source
        .split_once("fn execute_session_ref_view")
        .unwrap()
        .1
        .split_once("fn parse_session_ref_view")
        .unwrap()
        .0;
    assert_eq!(view.matches("resolve_session_ref(").count(), 1);
    assert_eq!(view.matches("run_view(").count(), 1);
    assert_eq!(view.matches(".view_batch(").count(), 1);
    assert!(view.contains("inputs.len() == 1"));
    assert!(view.find("try_reserve_exact(1)").unwrap() < view.find("run_view(").unwrap());
    assert!(!view.contains("for input in"));
    assert!(!view.contains(".clone()"));
    assert!(!source.contains("fn write_session_relation_absent"));
}

#[test]
fn shell_local_references_reject_malformed_numeric_forms_before_runtime_access() {
    let root = tempfile::tempdir().unwrap();

    let output = run_shell(
        root.path(),
        "view @\nview @00\nview @+1\nview @-1\nview @999999999999999999999999999999999999999999999999999999\nview @1\nexit\n",
    );
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let stderr = text(output.stderr);
    assert!(stderr.contains("numeric reference is empty"));
    assert!(stderr.contains("numeric reference must be canonical"));
    assert!(stderr.contains("numeric reference must be an unsigned decimal"));
    assert!(stderr.contains("numeric reference is out of range"));
}

#[test]
fn shell_local_view_preserves_mixed_kinds_named_inputs_and_absent_peers() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "x\r\n \t\n");
    let output = run_shell(
        root.path(),
        "search line x\nsearch line \" \"\nsearch /file note.txt\nlet named = @0\nlet hits = search line x\nview @2 @named @hits[0]\nview @0 @1 @0 --as paragraph\ncheck @7\nexit\n",
    );
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert_eq!(output.stdout, b"@0\tLine\tnote.txt:1\n@1\tLine\tnote.txt:2\n@2\tFile\tnote.txt\nFound 1\n0\tLine\tnote.txt:1\nView\t@2\tbytes=6\n@3\tFile\tnote.txt\nx\r\n \t\n\nEndView\nView\t@named\tbytes=3\n@4\tLine\tnote.txt:1\nx\r\n\nEndView\nView\t@hits[0]\tbytes=3\n@5\tLine\tnote.txt:1\nx\r\n\nEndView\nView\t@0\tbytes=3\n@6\tParagraph\tnote.txt:1-1\nx\r\n\nEndView\nView\t@1\tRelationAbsent\nView\t@0\tbytes=3\n@7\tParagraph\tnote.txt:1-1\nx\r\n\nEndView\n@8\tCurrent\tParagraph\tnote.txt:1-1\n");
    for (kind, expected) in [
        ("paragraph", b"@0\tLine\tnote.txt:1\nView\t@0\tbytes=3\n@1\tParagraph\tnote.txt:1-1\nx\r\n\nEndView\n".as_slice()),
        ("file", b"@0\tLine\tnote.txt:1\nView\t@0\tbytes=6\n@1\tFile\tnote.txt\nx\r\n \t\n\nEndView\n".as_slice()),
    ] {
        let single = run_shell(root.path(), &format!("search line x\nview @0 --as {kind}\nexit\n"));
        assert!(single.status.success());
        assert!(single.stderr.is_empty());
        assert_eq!(single.stdout, expected);
    }
}

#[test]
fn shell_local_view_empty_unicode_delimiters_and_safe_metadata_remain_exact() {
    for (path, content) in [
        ("empty.txt", ""),
        ("dir/a b.txt", "β\r\n"),
        ("EndView.txt", "View\t@0\nEndView\n"),
        ("quote\"β.txt", "x\r"),
    ] {
        let root = tempfile::tempdir().unwrap();
        write(root.path(), path, content);
        let path_token = serde_json::to_string(path).unwrap();
        let output = run_shell(
            root.path(),
            &format!("search /file {path_token}\nview @0\nexit\n"),
        );
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        assert_eq!(
            output.stdout,
            format!(
                "@0\tFile\t{path}\nView\t@0\tbytes={}\n@1\tFile\t{path}\n{content}\nEndView\n",
                content.len()
            )
            .as_bytes()
        );
        assert_eq!(
            fs::read(root.path().join(path)).unwrap(),
            content.as_bytes()
        );
    }
}

#[test]
fn shell_local_view_late_runtime_failure_has_no_output_or_slots_and_continues() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "a.txt", "x\n");
    write(root.path(), "b.txt", "x\n");
    let output = run_shell_after_initial_output(
        root.path(),
        "search line x\n",
        2,
        || fs::write(root.path().join("b.txt"), b"changed\n").unwrap(),
        "view @0 @1\nview @0\nexit\n",
    );
    assert_eq!(output.status.code(), Some(1));
    assert!(text(output.stderr).contains("unavailable"));
    assert_eq!(output.stdout, b"@0\tLine\ta.txt:1\n@1\tLine\tb.txt:1\nView\t@0\tbytes=2\n@2\tLine\ta.txt:1\nx\n\nEndView\n");
}

#[cfg(unix)]
#[test]
fn shell_local_view_broken_pipe_exits_before_later_publication() {
    let root = tempfile::tempdir().unwrap();
    // Exceed pipe buffering even if a concurrently spawned child briefly owns
    // an inherited read descriptor before exec closes it.
    let before = "before\n".repeat(262_144);
    write(root.path(), "note.txt", &before);
    let mut child = Command::new(binary())
        .current_dir(root.path())
        .arg("shell")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(b"search /file note.txt\n").unwrap();
    let mut reader = BufReader::new(child.stdout.take().unwrap());
    let mut first = String::new();
    reader.read_line(&mut first).unwrap();
    assert_eq!(first, "@0\tFile\tnote.txt\n");
    drop(reader);
    stdin
        .write_all(b"view @0\nreplace @0 after\nexit\n")
        .unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(!output.stderr.is_empty());
    assert_eq!(
        fs::read(root.path().join("note.txt")).unwrap(),
        before.as_bytes()
    );
}

#[test]
fn shell_local_view_relation_absent_and_search_failure_do_not_consume_reference_slots() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", " \t\nneedle\n");

    let output = run_shell(
        root.path(),
        "search line \" \"\nview @0 --as paragraph\nreplace @0 body\nview @1\nexit\n",
    );
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"@0\tLine\tnote.txt:1\nView\t@0\tRelationAbsent\n@1\tChanged\tLine\tnote.txt:1\nView\t@1\tbytes=5\n@2\tLine\tnote.txt:1\nbody\n\nEndView\n"
    );
    assert!(output.stderr.is_empty());
    assert_eq!(
        fs::read(root.path().join("note.txt")).unwrap(),
        b"body\nneedle\n"
    );

    let unavailable = tempfile::tempdir().unwrap();
    write(unavailable.path(), "note.txt", "needle\n");
    fs::write(unavailable.path().join("broken.txt"), b"needle\0").unwrap();
    let output = run_shell(unavailable.path(), "search line needle\nview @0\nexit\n");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let stderr = text(output.stderr);
    assert!(stderr.contains("unavailable"));
    assert!(stderr.contains("numeric reference is out of range: 0"));
}

#[test]
fn shell_local_replace_preserves_line_terminators_and_issues_fresh_references() {
    for (before, expected) in [
        ("old", "new"),
        ("old\n", "new\n"),
        ("old\r", "new\r"),
        ("old\r\n", "new\r\n"),
    ] {
        let root = tempfile::tempdir().unwrap();
        write(root.path(), "note.txt", before);

        let output = run_shell(
            root.path(),
            "search line old\nreplace @0 new\nview @0\nview @1\nreplace @1 new\nexit\n",
        );
        assert_eq!(output.status.code(), Some(1));
        assert_eq!(
            output.stdout,
            format!("@0\tLine\tnote.txt:1\n@1\tChanged\tLine\tnote.txt:1\nView\t@1\tbytes={}\n@2\tLine\tnote.txt:1\n{expected}\nEndView\n@3\tUnchanged\tLine\tnote.txt:1\n", expected.len()).as_bytes()
        );
        assert!(text(output.stderr).contains("unavailable"));
        assert_eq!(
            fs::read(root.path().join("note.txt")).unwrap(),
            expected.as_bytes()
        );
    }
}

#[test]
fn shell_local_replace_failure_does_not_consume_a_fresh_slot() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "old\n");

    let output = run_shell(
        root.path(),
        "search line old\nreplace @0 \"bad\\n\"\nreplace @0 new\nexit\n",
    );
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        output.stdout,
        b"@0\tLine\tnote.txt:1\n@1\tChanged\tLine\tnote.txt:1\n"
    );
    assert!(text(output.stderr).contains(EDIT_LINE_BODY_CAUSE));
    assert_eq!(fs::read(root.path().join("note.txt")).unwrap(), b"new\n");
}

#[test]
fn shell_local_replace_covers_file_and_paragraph_receipts() {
    let file = tempfile::tempdir().unwrap();
    write(file.path(), "note.txt", "old\r\n");
    let file_output = run_shell(file.path(), "search file old\nreplace @0 new\nexit\n");
    assert!(file_output.status.success());
    assert_eq!(
        file_output.stdout,
        b"@0\tFile\tnote.txt\n@1\tChanged\tFile\tnote.txt\n"
    );
    assert_eq!(fs::read(file.path().join("note.txt")).unwrap(), b"new");

    let paragraph = tempfile::tempdir().unwrap();
    write(paragraph.path(), "note.txt", "old\n\nsecond\n");
    let paragraph_output = run_shell(
        paragraph.path(),
        "search paragraph old\nreplace @0 \"new\\n\"\nexit\n",
    );
    assert!(paragraph_output.status.success());
    assert_eq!(
        paragraph_output.stdout,
        b"@0\tParagraph\tnote.txt:1-1\n@1\tChanged\tParagraph\tnote.txt:1-1\n"
    );
    assert_eq!(
        fs::read(paragraph.path().join("note.txt")).unwrap(),
        b"new\n\nsecond\n"
    );

    let literal_stdin = tempfile::tempdir().unwrap();
    write(literal_stdin.path(), "note.txt", "old");
    let literal_output = run_shell(
        literal_stdin.path(),
        "search file old\nreplace @0 --stdin\nexit\n",
    );
    assert!(literal_output.status.success());
    assert_eq!(
        fs::read(literal_stdin.path().join("note.txt")).unwrap(),
        b"--stdin"
    );

    let absent = tempfile::tempdir().unwrap();
    write(absent.path(), "note.txt", "old\n");
    let absent_output = run_shell(
        absent.path(),
        "search paragraph old\nreplace @0 \"\\n\"\nview @1\nexit\n",
    );
    assert_eq!(absent_output.status.code(), Some(2));
    assert_eq!(
        absent_output.stdout,
        b"@0\tParagraph\tnote.txt:1-1\nChanged\tNone\n"
    );
    assert!(text(absent_output.stderr).contains("numeric reference is out of range: 1"));
    assert_eq!(fs::read(absent.path().join("note.txt")).unwrap(), b"\n");
}

#[test]
fn session_pick_all_and_target_kind_project_the_existing_core_order() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "first\n\nsecond\n");

    let output = run_shell(
        root.path(),
        "let files = search file first\npick @files all\nlet paragraphs = search paragraph first\npick @paragraphs target-kind paragraph\nlet lines = search line first\npick @lines target-kind line\nexit\n",
    );
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"Found 1\n0\tFile\tnote.txt\nSelected 1\n0\tFile\tnote.txt\nFound 1\n0\tParagraph\tnote.txt:1-1\nSelected 1\n0\tParagraph\tnote.txt:0-6\nFound 1\n0\tLine\tnote.txt:1\nSelected 1\n0\tLine\tnote.txt:0-6\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn session_pick_same_file_and_one_of_preserve_candidate_order() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "a.txt", "needle\nneedle\n");
    write(root.path(), "b.txt", "needle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle\nlet same = pick @hits same-file @hits[0]\nlet selected = pick @hits all-of(one-of @hits[2] @hits[0])\npick @same all\nexit\n",
    );
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"Found 3\n0\tLine\ta.txt:1\n1\tLine\ta.txt:2\n2\tLine\tb.txt:1\nSelected 2\n0\tLine\ta.txt:0-7\n1\tLine\ta.txt:7-14\nSelected 2\n0\tLine\ta.txt:0-7\n1\tLine\tb.txt:0-7\nSelected 2\n0\tLine\ta.txt:0-7\n1\tLine\ta.txt:7-14\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn session_pick_composition_is_iterative_and_pick_bindings_feed_view_and_check() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\nneedle\nneedle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle\nlet selected = pick @hits all-of (target-kind line) (not (one-of @hits[1]))\npick @selected any-of(all) (not(not(target-kind line)))\nview anddress @selected[1]\ncheck anddress @selected[1]\nexit\n",
    );
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"Found 3\n0\tLine\tnote.txt:1\n1\tLine\tnote.txt:2\n2\tLine\tnote.txt:3\nSelected 2\n0\tLine\tnote.txt:0-7\n1\tLine\tnote.txt:14-21\nSelected 2\n0\tLine\tnote.txt:0-7\n1\tLine\tnote.txt:14-21\nneedle\nCurrent\n"
    );
    assert!(output.stderr.is_empty());

    let nesting = 4_096;
    let input = format!(
        "let hits = search line needle\npick @hits {}all{}\nexit\n",
        "not(".repeat(nesting),
        ")".repeat(nesting)
    );
    let deep = run_shell(root.path(), &input);
    assert!(deep.status.success());
    assert_eq!(
        deep.stdout,
        b"Found 3\n0\tLine\tnote.txt:1\n1\tLine\tnote.txt:2\n2\tLine\tnote.txt:3\nSelected 3\n0\tLine\tnote.txt:0-7\n1\tLine\tnote.txt:7-14\n2\tLine\tnote.txt:14-21\n"
    );
    assert!(deep.stderr.is_empty());
}

#[test]
fn session_pick_rejects_malformed_references_and_preserves_existing_bindings() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle\nlet address = @hits[0]\nlet empty = search line absent\npick @empty all\npick @hits all trailing\npick @hits not(all\npick @hits all-of()\npick @hits one-of\npick @hits same-file @hits\npick @hits target-kind section\npick @hits unknown\npick @missing all\npick @hits[0] all\npick @address all\nlet selected = pick @hits all\nview anddress @selected\nview anddress @selected[1]\nsearch line needle --source note.txt\nexit\n",
    );
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        output.stdout,
        b"Found 1\n0\tLine\tnote.txt:1\nFound 0\nSelected 0\nSelected 1\n0\tLine\tnote.txt:0-7\n@0\tLine\tnote.txt:1\n"
    );
    let stderr = text(output.stderr);
    assert!(stderr.contains("Pick predicate has trailing input"));
    assert!(stderr.contains("unclosed Pick predicate parenthesis"));
    assert!(stderr.contains("Pick composition requires at least one predicate"));
    assert!(stderr.contains("one-of requires at least one Anddress reference"));
    assert!(stderr.contains("Search binding requires an index: hits"));
    assert!(stderr.contains("invalid Pick target kind: section"));
    assert!(stderr.contains("invalid Pick predicate: unknown"));
    assert!(stderr.contains("unknown binding: missing"));
    assert!(stderr.contains("Pick candidates require a Search or Pick binding without an index"));
    assert!(stderr.contains("Pick candidates require a Search or Pick binding: address"));
    assert!(stderr.contains("Pick binding requires an index: selected"));
    assert!(stderr.contains("binding index is out of range: selected"));
}

#[test]
fn session_batch_check_reports_search_and_pick_counts_without_changing_bindings() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "current.txt", "needle\n");
    write(root.path(), "removed.txt", "needle\n");
    write(root.path(), "unavailable.txt", "needle\n");

    let output = run_shell_after_initial_output(
        root.path(),
        "let hits = search line needle\n",
        4,
        || {
            write(root.path(), "removed.txt", "changed\n");
            fs::write(root.path().join("unavailable.txt"), b"needle\0").unwrap();
        },
        "check search @hits\nlet selected = pick @hits all\ncheck pick @selected\npick @selected all\nexit\n",
    );
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"Found 3\n0\tLine\tcurrent.txt:1\n1\tLine\tremoved.txt:1\n2\tLine\tunavailable.txt:1\nChecked 3\nCurrent 1\nNotCurrent 1\nUnavailable 1\nSelected 3\n0\tLine\tcurrent.txt:0-7\n1\tLine\tremoved.txt:0-7\n2\tLine\tunavailable.txt:0-7\nChecked 3\nCurrent 1\nNotCurrent 1\nUnavailable 1\nSelected 3\n0\tLine\tcurrent.txt:0-7\n1\tLine\tremoved.txt:0-7\n2\tLine\tunavailable.txt:0-7\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn shell_direct_check_preserves_input_statuses_and_only_issues_current_references() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "current.txt", "needle\n");
    write(root.path(), "stale.txt", "needle\n");
    write(root.path(), "unavailable.txt", "needle\n");

    let output = run_shell_after_initial_output(
        root.path(),
        "search line needle\n",
        3,
        || {
            write(root.path(), "stale.txt", "changed\n");
            fs::write(root.path().join("unavailable.txt"), b"needle\0").unwrap();
        },
        "check @0 @1 @2 @0\ncheck @00 @0\nview @3 @4 @5\nview @3 @4\nexit\n",
    );
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        output.stdout,
        b"@0\tLine\tcurrent.txt:1\n@1\tLine\tstale.txt:1\n@2\tLine\tunavailable.txt:1\n@3\tCurrent\tLine\tcurrent.txt:1\nNotCurrent\nUnavailable\n@4\tCurrent\tLine\tcurrent.txt:1\nView\t@3\tbytes=7\n@5\tLine\tcurrent.txt:1\nneedle\n\nEndView\nView\t@4\tbytes=7\n@6\tLine\tcurrent.txt:1\nneedle\n\nEndView\n"
    );
    let stderr = text(output.stderr);
    assert!(stderr.contains("numeric reference must be canonical"));
}

#[test]
fn session_batch_check_accepts_empty_outcomes_and_rejects_invalid_binding_forms() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let empty = run_shell(
        root.path(),
        "let empty_search = search line absent\ncheck search @empty_search\nlet empty_pick = pick @empty_search all\ncheck pick @empty_pick\nexit\n",
    );
    assert!(empty.status.success());
    assert_eq!(
        empty.stdout,
        b"Found 0\nChecked 0\nCurrent 0\nNotCurrent 0\nUnavailable 0\nSelected 0\nChecked 0\nCurrent 0\nNotCurrent 0\nUnavailable 0\n"
    );
    assert!(empty.stderr.is_empty());

    let invalid = run_shell(
        root.path(),
        "let hits = search line needle\nlet selected = pick @hits all\nlet address = @hits[0]\ncheck search @selected\ncheck pick @hits\ncheck search @address\ncheck pick @address\ncheck search @hits[0]\ncheck pick @selected[0]\ncheck search @missing\ncheck search @hits extra\ncheck pick @selected extra\ncheck anddress @hits[0]\nexit\n",
    );
    assert_eq!(invalid.status.code(), Some(2));
    assert_eq!(
        invalid.stdout,
        b"Found 1\n0\tLine\tnote.txt:1\nSelected 1\n0\tLine\tnote.txt:0-7\nCurrent\n"
    );
    let stderr = text(invalid.stderr);
    assert!(stderr.contains("check search requires a Search binding"));
    assert!(stderr.contains("check pick requires a Pick binding"));
    assert!(stderr.contains("indexed binding references select an Anddress"));
    assert!(stderr.contains("unknown binding: missing"));
    assert!(stderr.contains("check search accepts exactly one binding"));
    assert!(stderr.contains("check pick accepts exactly one binding"));
}

#[test]
fn session_anchor_creates_views_and_invalidates_only_the_selected_source() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "left.txt", "needle\n");
    write(root.path(), "right.txt", "needle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle\nlet left = anchor create @hits[0]\nlet duplicate = anchor create @hits[0]\nlet right = anchor create @hits[1]\nview anchored @left\nanchor invalidate-source left.txt\nview anchored @right\nexit\n",
    );
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"Found 2\n0\tLine\tleft.txt:1\n1\tLine\tright.txt:1\nAnchored\nAlreadyLive\nAnchored\nneedle\nOK\nneedle\n"
    );
    assert!(output.stderr.is_empty());

    let invalidated = run_shell(
        root.path(),
        "let hits = search line needle\nlet handle = anchor create @hits[0]\nanchor invalidate-source left.txt\nview anchored @handle\nsearch line needle\nexit\n",
    );
    assert_eq!(invalidated.status.code(), Some(1));
    assert_eq!(
        invalidated.stdout,
        b"Found 2\n0\tLine\tleft.txt:1\n1\tLine\tright.txt:1\nAnchored\nOK\n@0\tLine\tleft.txt:1\n@1\tLine\tright.txt:1\n"
    );
    assert!(text(invalidated.stderr).contains("unavailable"));

    let invalid = run_shell(
        root.path(),
        "let hits = search line needle\nlet handle = anchor create @hits[0]\nlet alias = @handle\nview anchored @handle[0]\nanchor create @hits[0]\nanchor invalidate-source left.txt extra\nview anchored @missing\nexit\n",
    );
    assert_eq!(invalid.status.code(), Some(2));
    let stderr = text(invalid.stderr);
    assert!(stderr.contains("Anchedress binding cannot be cloned"));
    assert!(stderr.contains("Anchedress bindings cannot be indexed"));
    assert!(stderr.contains("anchor create is available only"));
    assert!(stderr.contains("invalidate-source accepts exactly"));
    assert!(stderr.contains("unknown binding: missing"));
}

#[test]
fn session_anchor_preserves_file_paragraph_and_line_views() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n\nsecond needle\n");

    let output = run_shell(
        root.path(),
        "let files = search file needle\nlet paragraphs = search paragraph needle\nlet lines = search line needle\nlet file = anchor create @files[0]\nlet paragraph = anchor create @paragraphs[0]\nlet line = anchor create @lines[0]\nview anchored @file\nview anchored @paragraph\nview anchored @line\nexit\n",
    );
    assert!(output.status.success());
    let stdout = text(output.stdout);
    assert_eq!(stdout.matches("Anchored\n").count(), 3);
    assert!(stdout.ends_with("needle\n\nsecond needle\nneedle\nneedle\n"));
    assert!(output.stderr.is_empty());

    let direct = run(root.path(), &["anchor", "create", "not-an-address"]);
    assert_eq!(direct.status.code(), Some(2));
}

#[test]
fn session_edit_apply_builds_each_core_edit_and_every_position() {
    let cases = [
        (
            "one\n",
            "let lines = search line one\nlet edit = edit insert before @lines[0] \"zero\\n\"\napply @edit\nexit\n",
            "zero\none\n",
        ),
        (
            "one\n",
            "let lines = search line one\nlet edit = edit insert after @lines[0] \"zero\\n\"\napply @edit\nexit\n",
            "one\nzero\n",
        ),
        (
            "one\n",
            "let files = search file one\nlet edit = edit insert start-of @files[0] \"zero\\n\"\napply @edit\nexit\n",
            "zero\none\n",
        ),
        (
            "one\n",
            "let lines = search line one\nlet edit = edit replace @lines[0] \"two\\r\\n\"\napply @edit\nexit\n",
            "two\r\n",
        ),
        (
            "one\n",
            "let lines = search line one\nlet edit = edit delete @lines[0]\napply @edit\nexit\n",
            "",
        ),
        (
            "a\nb\n",
            "let lines = search line a\nlet files = search file a\nlet edit = edit move @lines[0] end-of @files[0]\napply @edit\nexit\n",
            "b\na\n",
        ),
        (
            "a\n",
            "let lines = search line a\nlet files = search file a\nlet edit = edit copy @lines[0] end-of @files[0]\napply @edit\nexit\n",
            "a\na\n",
        ),
    ];
    for (before, input, after) in cases {
        let root = tempfile::tempdir().unwrap();
        write(root.path(), "note.txt", before);
        let output = run_shell(root.path(), input);
        assert!(output.status.success(), "{}", text(output.stderr));
        assert_eq!(
            fs::read_to_string(root.path().join("note.txt")).unwrap(),
            after
        );
        assert!(text(output.stdout).contains("OK\n"));
    }
}

#[test]
fn session_raw_replace_is_the_advanced_exact_extent_surface() {
    for (before, replacement, expected) in [
        ("old", "one\\ntwo", "one\ntwo"),
        ("old\n", "one\\r", "one\r"),
        ("old\r", "one\\r\\n", "one\r\n"),
        ("old\r\n", "one", "one"),
    ] {
        let root = tempfile::tempdir().unwrap();
        write(root.path(), "note.txt", before);
        let output = run_shell(
            root.path(),
            &format!(
                "let lines = search line old\nlet edit = edit replace @lines[0] \"{replacement}\"\napply @edit\nexit\n"
            ),
        );
        assert!(output.status.success(), "{}", text(output.stderr));
        assert!(output.stderr.is_empty());
        assert!(output.stdout.ends_with(b"OK\n"));
        assert_eq!(
            fs::read(root.path().join("note.txt")).unwrap(),
            expected.as_bytes()
        );
    }
}

#[test]
fn session_exact_file_lookup_inserts_into_an_empty_file_end_to_end() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "empty.txt", "");

    let output = run_shell(
        root.path(),
        "let files = search /file empty.txt\nlet edit = edit insert end-of @files[0] hello\napply @edit\ncheck anddress @files[0]\nexit\n",
    );
    assert!(output.status.success(), "{}", text(output.stderr));
    assert_eq!(
        output.stdout,
        b"Found 1\n0\tFile\tempty.txt\nOK\nNotCurrent\n"
    );
    assert!(output.stderr.is_empty());
    assert_eq!(fs::read(root.path().join("empty.txt")).unwrap(), b"hello");
}

#[test]
fn session_edit_apply_rejects_invalid_forms_without_stopping_later_commands() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "one\n");
    let output = run_shell(
        root.path(),
        "edit delete @missing\nlet lines = search line one\nlet wrong = @lines\nlet bad = edit insert start-of @lines[0] x\nlet edit = edit insert before @lines[0] \"\\t\\\"\\\\\\r\\n\"\napply @edit[0]\napply @wrong\napply @edit extra\napply @edit\nexit\n",
    );
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        fs::read(root.path().join("note.txt")).unwrap(),
        b"\t\"\\\r\none\n"
    );
    let stderr = text(output.stderr);
    assert!(stderr.contains("unsupported Session command: edit"));
    assert!(stderr.contains("Edit input is invalid"));
    assert!(stderr.contains("binding is not an Edit: wrong"));
    assert!(stderr.contains("Edit bindings cannot be indexed"));
    assert!(stderr.contains("apply accepts exactly one Edit binding"));
}

#[test]
fn session_view_and_check_result_bindings_keep_direct_output_and_clone_only_results() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");
    let output = run_shell(
        root.path(),
        "let lines = search line needle\nlet view = view anddress @lines[0]\nlet view_copy = @view\nlet raw_check = check anddress @lines[0]\nlet raw_copy = @raw_check\nlet search_check = check search @lines\nlet search_copy = @search_check\nlet picked = pick @lines all\nlet pick_check = check pick @picked\nlet pick_copy = @pick_check\nlet handle = anchor create @lines[0]\nlet anchored = view anchored @handle\nview anchored @handle\nview anddress @view\ncheck search @search_check\napply @raw_check\nexit\n",
    );
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        output.stdout,
        b"Found 1\n0\tLine\tnote.txt:1\nneedle\nCurrent\nChecked 1\nCurrent 1\nNotCurrent 0\nUnavailable 0\nSelected 1\n0\tLine\tnote.txt:0-7\nChecked 1\nCurrent 1\nNotCurrent 0\nUnavailable 0\nAnchored\nneedle\nneedle\n"
    );
    let stderr = text(output.stderr);
    assert!(stderr.contains("check search requires a Search binding"));
    assert!(stderr.contains("binding is not an Edit: raw_check"));
}

#[test]
fn session_data_stores_gets_and_binds_all_native_value_kinds() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle\nlet address = @hits[0]\nlet picked = pick @hits all\nlet viewed = view anddress @address\nlet checked_anddress = check anddress @address\nlet checked_search = check search @hits\nlet checked_pick = check pick @picked\ndata store anddress \"quoted\\\"slash\\\\\" @hits[0]\ndata store search shared @hits\ndata store pick shared @picked\ndata store view shared @viewed\ndata store check-anddress shared @checked_anddress\ndata store check-search shared @checked_search\ndata store check-pick shared @checked_pick\ndata list\ndata get anddress \"quoted\\\"slash\\\\\"\ndata get search shared\ndata get pick shared\ndata get view shared\ndata get check-anddress shared\ndata get check-search shared\ndata get check-pick shared\nlet restored_address = data get anddress \"quoted\\\"slash\\\\\"\nlet restored_search = data get search shared\nlet restored_pick = data get pick shared\nlet restored_view = data get view shared\nlet restored_check_anddress = data get check-anddress shared\nlet restored_check_search = data get check-search shared\nlet restored_check_pick = data get check-pick shared\nview anddress @restored_address\npick @restored_search all\ncheck pick @restored_pick\nexit\n",
    );

    assert!(output.status.success(), "{}", text(output.stderr));
    let stdout = text(output.stdout);
    assert_eq!(stdout.matches("OK\n").count(), 7);
    assert!(stdout.contains(
        "anddress\t\"quoted\\\"slash\\\\\"\nsearch\t\"shared\"\npick\t\"shared\"\nview\t\"shared\"\ncheck-anddress\t\"shared\"\ncheck-search\t\"shared\"\ncheck-pick\t\"shared\"\n"
    ));
    assert_eq!(stdout.matches("Anddress\tLine\tnote.txt:0-7\n").count(), 2);
    assert!(stdout.matches("Found 1\n0\tLine\tnote.txt:1\n").count() >= 3);
    assert!(
        stdout
            .matches("Selected 1\n0\tLine\tnote.txt:0-7\n")
            .count()
            >= 3
    );
    assert!(stdout.matches("needle\n").count() >= 3);
    assert!(stdout.matches("Current\n").count() >= 3);
    assert!(
        stdout
            .matches("Checked 1\nCurrent 1\nNotCurrent 0\nUnavailable 0\n")
            .count()
            >= 5
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn session_data_rejects_wrong_values_preserves_entries_and_drops_at_eof() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let invalid = run_shell(
        root.path(),
        "let hits = search line needle\nlet edit = edit delete @hits[0]\nlet handle = anchor create @hits[0]\ndata store search stored @hits\ndata store search stored @hits\ndata store pick stored @hits\ndata store anddress empty @hits[0]\ndata store anddress \"\" @hits[0]\ndata store anddress bad-edit @edit\ndata store search bad-anchor @handle\ndata rename search stored stored\ndata rename search stored renamed\ndata get search stored\ndata get search renamed\ndata remove search renamed\ndata get search renamed\nlet restored = data get anddress empty\ndata store search invalid @restored\ndata store view invalid @hits[0]\ndata store search indexed @hits[0]\ndata list extra\nexit\n",
    );
    assert_eq!(invalid.status.code(), Some(2));
    assert_eq!(
        invalid.stdout,
        b"Found 1\n0\tLine\tnote.txt:1\nAnchored\nOK\nOK\nOK\nFound 1\n0\tLine\tnote.txt:1\nOK\nAnddress\tLine\tnote.txt:0-7\n"
    );
    let stderr = text(invalid.stderr);
    assert!(stderr.contains("Data entry already exists"));
    assert!(stderr.contains("Data kind does not match binding"));
    assert!(stderr.contains("Data name is empty"));
    assert!(stderr.contains("Edit binding cannot be used as an Anddress: edit"));
    assert!(stderr.contains("Anchedress binding cannot be cloned: handle"));
    assert!(stderr.contains("Data entry was not found"));
    assert!(stderr.contains("indexed binding references select an Anddress"));
    assert!(stderr.contains("unsupported data command"));

    let next_session = run_shell(root.path(), "data list\ndata get anddress empty\nexit\n");
    assert_eq!(next_session.status.code(), Some(2));
    assert!(next_session.stdout.is_empty());
    assert!(text(next_session.stderr).contains("Data entry was not found"));
    assert_usage(run(root.path(), &["data", "list"]));
}

#[test]
fn session_bindings_reject_unknown_duplicate_empty_out_of_range_and_type_mismatch() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle\nlet malformed =search line needle\nlet hits = @hits\nview anddress @hits\nlet selected = @hits[0]\ncheck anddress @selected[0]\nlet empty = search line absent\nview anddress @empty[0]\nview anddress @missing\nsearch line needle\nexit\n",
    );
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        output.stdout,
        b"Found 1\n0\tLine\tnote.txt:1\nFound 0\n@0\tLine\tnote.txt:1\n"
    );
    let stderr = text(output.stderr);
    assert!(stderr.contains("let requires a standalone = token"));
    assert!(stderr.contains("binding already exists: hits"));
    assert!(stderr.contains("Search binding requires an index: hits"));
    assert!(stderr.contains("Anddress binding cannot be indexed: selected"));
    assert!(stderr.contains("Search binding is empty: empty"));
    assert!(stderr.contains("unknown binding: missing"));
}

#[test]
fn session_lexer_exit_and_eof_follow_the_initial_grammar() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "a space\nquote: \" and slash \\\n");

    let lexical = run_shell(
        root.path(),
        "\n\t \nlet spaced = search line \"a space\"\nlet escaped = search line \"quote: \\\" and slash \\\\\"\nview anddress @escaped[0]\nsearch line \"\"\nsearch line \"bad\\q\"\nsearch line \"unterminated\nsearch line \0\nsearch line \"a space\" | ignored\nsearch line \"a space\"\nexit extra\nsearch line \"a space\"\nexit\nsearch line \"a space\"\n",
    );
    assert_eq!(lexical.status.code(), Some(2));
    assert_eq!(
        lexical.stdout,
        b"Found 1\n0\tLine\tnote.txt:1\nFound 1\n0\tLine\tnote.txt:2\nquote: \" and slash \\\n@0\tLine\tnote.txt:1\n@1\tLine\tnote.txt:1\n"
    );
    let stderr = text(lexical.stderr);
    assert!(stderr.contains("search query is invalid"));
    assert!(stderr.contains("invalid quoted escape"));
    assert!(stderr.contains("unmatched quote"));
    assert!(stderr.contains("Session input must not contain NUL"));
    assert!(stderr.contains("invalid search option: |"));
    assert!(stderr.contains("exit accepts no operands"));

    let eof = run_shell(root.path(), "\nsearch line \"a space\"\n");
    assert!(eof.status.success());
    assert_eq!(eof.stdout, b"@0\tLine\tnote.txt:1\n");
    assert!(eof.stderr.is_empty());
}

#[test]
fn session_lexer_decodes_all_quoted_escapes_outside_edit() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let output = run_shell(
        root.path(),
        "let hits = search line needle\ndata store anddress \"slash\\\\ quote\\\" line\\ncarriage\\rtab\\t\" @hits[0]\ndata list\nexit\n",
    );
    assert!(output.status.success(), "{}", text(output.stderr));
    assert_eq!(
        output.stdout,
        b"Found 1\n0\tLine\tnote.txt:1\nOK\nanddress\t\"slash\\\\ quote\\\" line\\ncarriage\\rtab\\t\"\n"
    );
}

#[test]
fn session_apply_reuses_edit_binding_and_keeps_explicit_edit_clone() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "one\n");

    let output = run_shell(
        root.path(),
        "let lines = search line one\nlet edit = edit replace @lines[0] \"one\\n\"\nlet copy = @edit\napply @edit\napply @copy\nexit\n",
    );
    assert!(output.status.success(), "{}", text(output.stderr));
    assert_eq!(output.stdout, b"Found 1\n0\tLine\tnote.txt:1\nOK\nOK\n");
    assert_eq!(fs::read(root.path().join("note.txt")).unwrap(), b"one\n");

    let source = include_str!("../../src/bin/bw/shell.rs");
    let output = include_str!("../../src/bin/bw/output.rs");
    assert!(output.contains("fn write_view(outcome: &ViewOutcome)"));
    assert!(output.contains("fn write_batch_check(report: &CheckReport)"));
    assert!(source.contains("Result<&'a Edit, CliError>"));
    assert!(!source.contains("write_view(outcome.clone())"));
    assert!(!source.contains("write_batch_check(outcome.report.clone())"));
    assert!(!source.contains("Some(SessionValue::Edit(edit)) => Ok(edit.clone())"));
    assert!(
        source.contains("Some(SessionValue::Edit(value)) => Ok(SessionValue::Edit(value.clone()))")
    );
}

#[test]
fn session_preserves_execution_then_usage_exit_precedence_without_latest_state() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "note.txt", "needle\n");

    let execution_only = run_shell(
        root.path(),
        "search line needle --source missing.txt\nsearch line needle\n",
    );
    assert_eq!(execution_only.status.code(), Some(1));
    assert_eq!(execution_only.stdout, b"@0\tLine\tnote.txt:1\n");
    assert!(text(execution_only.stderr).contains("workspace source is unavailable"));

    let no_latest = run_shell(root.path(), "search line needle\nview anddress @latest\n");
    assert_eq!(no_latest.status.code(), Some(2));
    assert_eq!(no_latest.stdout, b"@0\tLine\tnote.txt:1\n");
    assert!(text(no_latest.stderr).contains("unknown binding: latest"));

    let execution_then_usage = run_shell(
        root.path(),
        "search line needle --source missing.txt\nunknown\nsearch line needle\n",
    );
    assert_eq!(execution_then_usage.status.code(), Some(2));
    assert_eq!(execution_then_usage.stdout, b"@0\tLine\tnote.txt:1\n");
    assert!(text(execution_then_usage.stderr).contains("unsupported Session command: unknown"));
}
