//! Runtime-owned Search execution over current-only forward source observations.

use std::cmp::Ordering;

use crate::backwriter::anddress::{
    Anddress, AnddressTarget, LineBodyClass, construct_anddress, construct_source_identity,
};
use crate::backwriter::search::{
    LiteralMatcher, MatchTier, PreparedLiteral, SearchError, SearchOutcome, SearchRequest,
    SearchRequestKind, SearchScope, SearchScopeEntry, SearchTarget,
};
use crate::safe_path::{
    ClassifiedChild, classify_child, directory_names, open_directory, open_regular,
};
use crate::source::validate_logical_path;

use super::{
    DirectoryAccessError, WorkspaceRuntime, is_backwriter_spill, path_is_within_root,
    source_scan::{SourceScanError, observe_source},
};

struct SearchDirectory {
    path: String,
    directory: cap_std::fs::Dir,
    names: Vec<std::ffi::OsString>,
    cursor: usize,
}

pub(super) fn execute(
    runtime: &WorkspaceRuntime,
    request: &SearchRequest,
) -> Result<SearchOutcome, SearchError> {
    match request.kind() {
        SearchRequestKind::Content {
            query,
            scope,
            target,
        } => execute_content(runtime, query, scope, *target),
        SearchRequestKind::ExactFile { logical_path } => execute_exact_file(runtime, logical_path),
    }
}

fn execute_content(
    runtime: &WorkspaceRuntime,
    query: &crate::backwriter::search::SearchQuery,
    scope: &SearchScope,
    target: SearchTarget,
) -> Result<SearchOutcome, SearchError> {
    let literal = PreparedLiteral::new(query)?;
    let mut executor = SearchExecutor {
        runtime,
        scope,
        target,
        literal,
        full_line_results: Vec::new(),
        substring_results: Vec::new(),
    };
    executor.preflight()?;
    executor.execute()?;
    executor.full_line_results.sort_unstable_by(compare_bucket);
    executor.substring_results.sort_unstable_by(compare_bucket);
    let anddresses = join_result_buckets(executor.full_line_results, executor.substring_results)?;
    Ok(if anddresses.is_empty() {
        SearchOutcome::Empty
    } else {
        SearchOutcome::Found { anddresses }
    })
}

fn execute_exact_file(
    runtime: &WorkspaceRuntime,
    logical_path: &str,
) -> Result<SearchOutcome, SearchError> {
    if is_backwriter_spill(logical_path) {
        return Ok(SearchOutcome::Empty);
    }
    let mut file = match runtime.open_admitted_source(logical_path) {
        Ok(file) => file,
        Err(DirectoryAccessError::Unadmitted) => return Err(SearchError::InvalidScope),
        Err(DirectoryAccessError::NotCurrent) => return Ok(SearchOutcome::Empty),
        Err(DirectoryAccessError::Unavailable) => return Err(SearchError::Unavailable),
    };
    let state = observe_source(&mut file, |_, _| Ok(())).map_err(|_| SearchError::Unavailable)?;
    let source = construct_source_identity(
        &runtime.workspace_coordinate,
        logical_path,
        &state.hash,
        state.byte_length,
    )
    .map_err(|_| SearchError::Unavailable)?;
    let anddress = construct_anddress(&source, AnddressTarget::File, 0, state.byte_length)
        .map_err(|_| SearchError::Unavailable)?;
    let mut anddresses = Vec::new();
    anddresses
        .try_reserve_exact(1)
        .map_err(|_| SearchError::Unavailable)?;
    anddresses.push(anddress);
    Ok(SearchOutcome::Found { anddresses })
}

struct SearchExecutor<'a> {
    runtime: &'a WorkspaceRuntime,
    scope: &'a SearchScope,
    target: SearchTarget,
    literal: PreparedLiteral<'a>,
    full_line_results: Vec<Anddress>,
    substring_results: Vec<Anddress>,
}

impl SearchExecutor<'_> {
    fn preflight(&self) -> Result<(), SearchError> {
        if let Some(entries) = self.scope.entries() {
            for entry in entries {
                self.validate_scope_entry(entry)?;
            }
        }
        Ok(())
    }

    fn execute(&mut self) -> Result<(), SearchError> {
        if let Some(entries) = self.scope.entries() {
            for entry in entries {
                if entry.is_subtree() {
                    self.search_subtree(entry.path())?;
                } else {
                    self.search_source(entry.path())?;
                }
            }
        } else {
            for root in self.runtime.admission().roots() {
                self.search_subtree(root.as_str())?;
            }
        }
        Ok(())
    }

    fn validate_scope_entry(&self, entry: &SearchScopeEntry) -> Result<(), SearchError> {
        let root = self
            .runtime
            .selected_root(entry.path())
            .map_err(|_| SearchError::InvalidScope)?;
        let allowed = if entry.is_subtree() {
            entry.path() == root.as_str() || path_is_within_root(entry.path(), root.as_str())
        } else {
            entry.path() != root.as_str() && path_is_within_root(entry.path(), root.as_str())
        };
        allowed.then_some(()).ok_or(SearchError::InvalidScope)
    }

    fn search_subtree(&mut self, path: &str) -> Result<(), SearchError> {
        if is_backwriter_spill(path) {
            return Ok(());
        }
        let directory = self.open_logical_directory(path)?;
        self.walk_directory(path.to_owned(), directory)
    }

    fn open_logical_directory(&self, path: &str) -> Result<cap_std::fs::Dir, SearchError> {
        self.runtime
            .open_admitted_directory(path)
            .map_err(|error| match error {
                DirectoryAccessError::Unadmitted => SearchError::InvalidScope,
                DirectoryAccessError::NotCurrent | DirectoryAccessError::Unavailable => {
                    SearchError::Unavailable
                }
            })
    }

    fn walk_directory(
        &mut self,
        path: String,
        directory: cap_std::fs::Dir,
    ) -> Result<(), SearchError> {
        let names = directory_names(&directory).map_err(|_| SearchError::Unavailable)?;
        let mut stack = Vec::new();
        stack.try_reserve(1).map_err(|_| SearchError::Unavailable)?;
        stack.push(SearchDirectory {
            path,
            directory,
            names,
            cursor: 0,
        });

        while !stack.is_empty() {
            let next = {
                let current = stack.last_mut().expect("nonempty traversal stack");
                if current.cursor == current.names.len() {
                    Next::Finished
                } else {
                    let name = current.names[current.cursor]
                        .to_str()
                        .filter(|name| validate_logical_path(name).is_ok());
                    current.cursor += 1;
                    match name {
                        None => Next::Ignored,
                        Some(name) => {
                            let path = child_path(&current.path, name)?;
                            if is_backwriter_spill(&path) {
                                Next::Ignored
                            } else {
                                match classify_child(&current.directory, name)
                                    .map_err(|_| SearchError::Unavailable)?
                                {
                                    ClassifiedChild::Directory => Next::Directory {
                                        path,
                                        directory: open_directory(
                                            &current.directory,
                                            name,
                                            ClassifiedChild::Directory,
                                        )
                                        .map_err(|_| SearchError::Unavailable)?,
                                    },
                                    ClassifiedChild::Regular => Next::Source {
                                        path,
                                        file: open_regular(
                                            &current.directory,
                                            name,
                                            ClassifiedChild::Regular,
                                        )
                                        .map_err(|_| SearchError::Unavailable)?,
                                    },
                                    ClassifiedChild::Excluded => Next::Ignored,
                                }
                            }
                        }
                    }
                }
            };
            match next {
                Next::Finished => {
                    stack.pop();
                }
                Next::Ignored => {}
                Next::Directory { path, directory } => {
                    let names =
                        directory_names(&directory).map_err(|_| SearchError::Unavailable)?;
                    stack.try_reserve(1).map_err(|_| SearchError::Unavailable)?;
                    stack.push(SearchDirectory {
                        path,
                        directory,
                        names,
                        cursor: 0,
                    });
                }
                Next::Source { path, file } => self.search_open_source(&path, file)?,
            }
        }
        Ok(())
    }

    fn search_source(&mut self, path: &str) -> Result<(), SearchError> {
        if is_backwriter_spill(path) {
            return Ok(());
        }
        let file = self
            .runtime
            .open_admitted_source(path)
            .map_err(|error| match error {
                DirectoryAccessError::Unadmitted => SearchError::InvalidScope,
                DirectoryAccessError::NotCurrent | DirectoryAccessError::Unavailable => {
                    SearchError::Unavailable
                }
            })?;
        self.search_open_source(path, file)
    }

    fn search_open_source(
        &mut self,
        path: &str,
        mut file: cap_std::fs::File,
    ) -> Result<(), SearchError> {
        scan_open_source(
            &mut file,
            &self.runtime.workspace_coordinate,
            path,
            &self.literal,
            self.target,
            &mut self.full_line_results,
            &mut self.substring_results,
        )?;
        Ok(())
    }
}

struct ParagraphState {
    best_tier: Option<MatchTier>,
    byte_start: usize,
    byte_end: usize,
}

struct ProvisionalTarget {
    tier: MatchTier,
    target: AnddressTarget,
    byte_start: usize,
    byte_end: usize,
}

struct FileProjection<'a> {
    matcher: LiteralMatcher<'a>,
    best_tier: Option<MatchTier>,
    line_started: bool,
    pending_cr: bool,
}

struct LineProjection<'a> {
    matcher: LiteralMatcher<'a>,
    line_start: usize,
    line_started: bool,
    pending_cr: bool,
    provisional: Vec<ProvisionalTarget>,
}

struct ParagraphProjection<'a> {
    matcher: LiteralMatcher<'a>,
    line_start: usize,
    line_started: bool,
    pending_cr: bool,
    body_class: LineBodyClass,
    paragraph: Option<ParagraphState>,
    provisional: Vec<ProvisionalTarget>,
}

enum ProjectionOutcome {
    File(Option<MatchTier>),
    Targets(Vec<ProvisionalTarget>),
}

fn scan_open_source(
    reader: &mut impl std::io::Read,
    workspace_coordinate: &str,
    logical_path: &str,
    literal: &PreparedLiteral<'_>,
    target: SearchTarget,
    full_line_results: &mut Vec<Anddress>,
    substring_results: &mut Vec<Anddress>,
) -> Result<(), SearchError> {
    let (state, outcome) = match target {
        SearchTarget::File => {
            let mut projection = FileProjection::new(literal);
            let state = observe_source(reader, |bytes, _| projection.push(bytes))
                .map_err(|_| SearchError::Unavailable)?;
            (state, ProjectionOutcome::File(projection.finish()))
        }
        SearchTarget::Paragraph => {
            let mut projection = ParagraphProjection::new(literal);
            let state = observe_source(reader, |bytes, chunk_start| {
                projection.push(bytes, chunk_start)
            })
            .map_err(|_| SearchError::Unavailable)?;
            projection
                .finish(state.byte_length)
                .map_err(|_| SearchError::Unavailable)?;
            (state, ProjectionOutcome::Targets(projection.provisional))
        }
        SearchTarget::Line => {
            let mut projection = LineProjection::new(literal);
            let state = observe_source(reader, |bytes, chunk_start| {
                projection.push(bytes, chunk_start)
            })
            .map_err(|_| SearchError::Unavailable)?;
            projection
                .finish(state.byte_length)
                .map_err(|_| SearchError::Unavailable)?;
            (state, ProjectionOutcome::Targets(projection.provisional))
        }
    };
    let source = construct_source_identity(
        workspace_coordinate,
        logical_path,
        &state.hash,
        state.byte_length,
    )
    .map_err(|_| SearchError::Unavailable)?;
    match outcome {
        ProjectionOutcome::File(Some(tier)) => push_result(
            full_line_results,
            substring_results,
            tier,
            construct_anddress(&source, AnddressTarget::File, 0, state.byte_length)
                .map_err(|_| SearchError::Unavailable)?,
        )?,
        ProjectionOutcome::File(None) => {}
        ProjectionOutcome::Targets(provisional) => {
            for provisional in provisional {
                push_result(
                    full_line_results,
                    substring_results,
                    provisional.tier,
                    construct_anddress(
                        &source,
                        provisional.target,
                        provisional.byte_start,
                        provisional.byte_end,
                    )
                    .map_err(|_| SearchError::Unavailable)?,
                )?;
            }
        }
    }
    Ok(())
}

impl<'a> FileProjection<'a> {
    fn new(literal: &'a PreparedLiteral<'a>) -> Self {
        Self {
            matcher: literal.matcher(),
            best_tier: None,
            line_started: false,
            pending_cr: false,
        }
    }

    fn push(&mut self, bytes: &[u8]) -> Result<(), SourceScanError> {
        if self.best_tier == Some(MatchTier::FullLine) {
            return Ok(());
        }
        for &byte in bytes {
            if self.pending_cr {
                if byte == b'\n' {
                    self.finish_line();
                    if self.best_tier == Some(MatchTier::FullLine) {
                        return Ok(());
                    }
                    continue;
                }
                self.finish_line();
                if self.best_tier == Some(MatchTier::FullLine) {
                    return Ok(());
                }
            }
            self.begin_line();
            match byte {
                b'\r' => self.pending_cr = true,
                b'\n' => self.finish_line(),
                _ => self.matcher.push(byte),
            }
            if self.best_tier == Some(MatchTier::FullLine) {
                return Ok(());
            }
        }
        Ok(())
    }

    fn finish(mut self) -> Option<MatchTier> {
        if self.line_started && self.best_tier != Some(MatchTier::FullLine) {
            self.finish_line();
        }
        self.best_tier
    }

    fn begin_line(&mut self) {
        if !self.line_started {
            self.line_started = true;
            self.matcher.reset();
        }
    }

    fn finish_line(&mut self) {
        let tier = self.matcher.finish();
        if let Some(tier) = tier {
            prefer_tier(&mut self.best_tier, tier);
        }
        self.line_started = false;
        self.pending_cr = false;
    }
}

impl<'a> LineProjection<'a> {
    fn new(literal: &'a PreparedLiteral<'a>) -> Self {
        Self {
            matcher: literal.matcher(),
            line_start: 0,
            line_started: false,
            pending_cr: false,
            provisional: Vec::new(),
        }
    }

    fn push(&mut self, bytes: &[u8], chunk_start: usize) -> Result<(), SourceScanError> {
        for (index, &byte) in bytes.iter().enumerate() {
            let byte_start = chunk_start
                .checked_add(index)
                .ok_or(SourceScanError::Resource)?;
            if self.pending_cr {
                if byte == b'\n' {
                    self.finish_line(byte_start.checked_add(1).ok_or(SourceScanError::Resource)?)?;
                    continue;
                }
                self.finish_line(byte_start)?;
            }
            self.begin_line(byte_start);
            match byte {
                b'\r' => self.pending_cr = true,
                b'\n' => {
                    self.finish_line(byte_start.checked_add(1).ok_or(SourceScanError::Resource)?)?
                }
                _ => self.matcher.push(byte),
            }
        }
        Ok(())
    }

    fn finish(&mut self, source_byte_length: usize) -> Result<(), SourceScanError> {
        if self.line_started {
            self.finish_line(source_byte_length)?;
        }
        Ok(())
    }

    fn begin_line(&mut self, byte_start: usize) {
        if !self.line_started {
            self.line_started = true;
            self.line_start = byte_start;
            self.matcher.reset();
        }
    }

    fn finish_line(&mut self, byte_end: usize) -> Result<(), SourceScanError> {
        if let Some(tier) = self.matcher.finish() {
            push_provisional(
                &mut self.provisional,
                tier,
                AnddressTarget::Line,
                self.line_start,
                byte_end,
            )?;
        }
        self.line_started = false;
        self.pending_cr = false;
        Ok(())
    }
}

impl<'a> ParagraphProjection<'a> {
    fn new(literal: &'a PreparedLiteral<'a>) -> Self {
        Self {
            matcher: literal.matcher(),
            line_start: 0,
            line_started: false,
            pending_cr: false,
            body_class: LineBodyClass::Empty,
            paragraph: None,
            provisional: Vec::new(),
        }
    }

    fn push(&mut self, bytes: &[u8], chunk_start: usize) -> Result<(), SourceScanError> {
        for (index, &byte) in bytes.iter().enumerate() {
            let byte_start = chunk_start
                .checked_add(index)
                .ok_or(SourceScanError::Resource)?;
            if self.pending_cr {
                if byte == b'\n' {
                    self.finish_line(byte_start.checked_add(1).ok_or(SourceScanError::Resource)?)?;
                    continue;
                }
                self.finish_line(byte_start)?;
            }
            self.begin_line(byte_start);
            match byte {
                b'\r' => self.pending_cr = true,
                b'\n' => {
                    self.finish_line(byte_start.checked_add(1).ok_or(SourceScanError::Resource)?)?
                }
                _ => {
                    self.matcher.push(byte);
                    if !matches!(byte, b' ' | b'\t') {
                        self.body_class = LineBodyClass::Text;
                    } else if self.body_class == LineBodyClass::Empty {
                        self.body_class = LineBodyClass::HorizontalWhitespace;
                    }
                }
            }
        }
        Ok(())
    }

    fn finish(&mut self, source_byte_length: usize) -> Result<(), SourceScanError> {
        if self.line_started {
            self.finish_line(source_byte_length)?;
        }
        self.close_paragraph()
    }

    fn begin_line(&mut self, byte_start: usize) {
        if !self.line_started {
            self.line_started = true;
            self.line_start = byte_start;
            self.matcher.reset();
        }
    }

    fn finish_line(&mut self, byte_end: usize) -> Result<(), SourceScanError> {
        let tier = self.matcher.finish();
        if self.body_class == LineBodyClass::Text {
            let paragraph = self.paragraph.get_or_insert(ParagraphState {
                best_tier: None,
                byte_start: self.line_start,
                byte_end,
            });
            paragraph.byte_end = byte_end;
            if let Some(tier) = tier {
                prefer_tier(&mut paragraph.best_tier, tier);
            }
        } else {
            self.close_paragraph()?;
        }
        self.line_started = false;
        self.pending_cr = false;
        self.body_class = LineBodyClass::Empty;
        Ok(())
    }

    fn close_paragraph(&mut self) -> Result<(), SourceScanError> {
        if let Some(paragraph) = self.paragraph.take()
            && let Some(tier) = paragraph.best_tier
        {
            push_provisional(
                &mut self.provisional,
                tier,
                AnddressTarget::Paragraph,
                paragraph.byte_start,
                paragraph.byte_end,
            )?;
        }
        Ok(())
    }
}

fn prefer_tier(best: &mut Option<MatchTier>, tier: MatchTier) {
    if best.is_none_or(|current| tier < current) {
        *best = Some(tier);
    }
}

fn push_provisional(
    provisional: &mut Vec<ProvisionalTarget>,
    tier: MatchTier,
    target: AnddressTarget,
    byte_start: usize,
    byte_end: usize,
) -> Result<(), SourceScanError> {
    provisional
        .try_reserve(1)
        .map_err(|_| SourceScanError::Resource)?;
    provisional.push(ProvisionalTarget {
        tier,
        target,
        byte_start,
        byte_end,
    });
    Ok(())
}

fn push_result(
    full_line_results: &mut Vec<Anddress>,
    substring_results: &mut Vec<Anddress>,
    tier: MatchTier,
    anddress: Anddress,
) -> Result<(), SearchError> {
    let bucket = match tier {
        MatchTier::FullLine => full_line_results,
        MatchTier::Substring => substring_results,
    };
    bucket
        .try_reserve(1)
        .map_err(|_| SearchError::Unavailable)?;
    bucket.push(anddress);
    Ok(())
}

enum Next {
    Finished,
    Ignored,
    Directory {
        path: String,
        directory: cap_std::fs::Dir,
    },
    Source {
        path: String,
        file: cap_std::fs::File,
    },
}

fn child_path(parent: &str, name: &str) -> Result<String, SearchError> {
    let length = if parent == "." {
        name.len()
    } else {
        parent
            .len()
            .checked_add(1)
            .and_then(|length| length.checked_add(name.len()))
            .ok_or(SearchError::Unavailable)?
    };
    let mut path = String::new();
    path.try_reserve_exact(length)
        .map_err(|_| SearchError::Unavailable)?;
    if parent != "." {
        path.push_str(parent);
        path.push('/');
    }
    path.push_str(name);
    Ok(path)
}

fn join_result_buckets(
    mut full_line_results: Vec<Anddress>,
    mut substring_results: Vec<Anddress>,
) -> Result<Vec<Anddress>, SearchError> {
    if full_line_results.is_empty() {
        return Ok(substring_results);
    }
    if substring_results.is_empty() {
        return Ok(full_line_results);
    }
    full_line_results
        .try_reserve(substring_results.len())
        .map_err(|_| SearchError::Unavailable)?;
    full_line_results.append(&mut substring_results);
    Ok(full_line_results)
}

fn compare_bucket(left: &Anddress, right: &Anddress) -> Ordering {
    left.logical_path()
        .as_bytes()
        .cmp(right.logical_path().as_bytes())
        .then_with(|| left.byte_start().cmp(&right.byte_start()))
        .then_with(|| left.byte_end().cmp(&right.byte_end()))
}

#[cfg(test)]
mod tests {
    use std::io::{self, Read};

    use crate::backwriter::anddress::AnddressTarget;
    use crate::backwriter::search::{SearchQuery, SearchTarget};
    use crate::runtime::source_scan::READ_BUFFER_SIZE;

    use super::*;

    struct FixtureReader<'a> {
        bytes: &'a [u8],
        cursor: usize,
        max_chunk: usize,
        fail_after: Option<usize>,
        returned_eof: bool,
        failed: bool,
    }

    impl Read for FixtureReader<'_> {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            assert!(!self.returned_eof, "source was read again after EOF");
            assert!(!self.failed, "source was read again after an error");
            if self.fail_after.is_some_and(|offset| self.cursor >= offset) {
                self.failed = true;
                return Err(io::Error::other("scripted failure"));
            }
            if self.cursor == self.bytes.len() {
                self.returned_eof = true;
                return Ok(0);
            }
            let boundary = self.fail_after.unwrap_or(self.bytes.len());
            let count = self
                .max_chunk
                .min(buffer.len())
                .min(boundary - self.cursor)
                .min(self.bytes.len() - self.cursor);
            assert_ne!(count, 0, "fixture made no forward progress");
            buffer[..count].copy_from_slice(&self.bytes[self.cursor..self.cursor + count]);
            self.cursor += count;
            Ok(count)
        }
    }

    fn project(
        bytes: &[u8],
        query: &str,
        target: SearchTarget,
        fail_after: Option<usize>,
    ) -> Result<(Vec<Anddress>, Vec<Anddress>), SearchError> {
        project_chunked(bytes, query, target, fail_after, 1)
    }

    fn project_chunked(
        bytes: &[u8],
        query: &str,
        target: SearchTarget,
        fail_after: Option<usize>,
        max_chunk: usize,
    ) -> Result<(Vec<Anddress>, Vec<Anddress>), SearchError> {
        let query = SearchQuery::new(query).unwrap();
        let literal = PreparedLiteral::new(&query)?;
        let mut reader = FixtureReader {
            bytes,
            cursor: 0,
            max_chunk,
            fail_after,
            returned_eof: false,
            failed: false,
        };
        let mut full = Vec::new();
        let mut substring = Vec::new();
        scan_open_source(
            &mut reader,
            &"0".repeat(64),
            "source.txt",
            &literal,
            target,
            &mut full,
            &mut substring,
        )?;
        Ok((full, substring))
    }

    fn only_result(bytes: &[u8], query: &str, target: SearchTarget) -> (MatchTier, Anddress) {
        let (mut full, mut substring) =
            project_chunked(bytes, query, target, None, READ_BUFFER_SIZE).unwrap();
        match (full.pop(), substring.pop()) {
            (Some(anddress), None) => (MatchTier::FullLine, anddress),
            (None, Some(anddress)) => (MatchTier::Substring, anddress),
            _ => panic!("expected exactly one projected result"),
        }
    }

    #[test]
    fn one_byte_search_preserves_utf8_framing_and_line_scoped_kmp() {
        let (full, substring) = project(
            "é€🦀\r\nneedle\rneedle".as_bytes(),
            "needle",
            SearchTarget::Line,
            None,
        )
        .unwrap();
        assert_eq!(substring, Vec::new());
        assert_eq!(full.len(), 2);
        assert_eq!(full[0].target(), AnddressTarget::Line);
        assert_eq!((full[0].byte_start(), full[0].byte_end()), (11, 18));
        assert_eq!(full[1].target(), AnddressTarget::Line);
        assert_eq!((full[1].byte_start(), full[1].byte_end()), (18, 24));
        let (full, substring) = project(b"nee\ndle", "needle", SearchTarget::Line, None).unwrap();
        assert!(full.is_empty() && substring.is_empty());
    }

    #[test]
    fn late_source_failure_discards_every_search_projection() {
        for (bytes, failure) in [
            (b"needle\n\xff".as_slice(), None),
            (b"needle\n\xe2".as_slice(), None),
            (b"needle\n\0".as_slice(), None),
            (b"needle\nlate".as_slice(), Some(7)),
        ] {
            for target in [
                SearchTarget::Line,
                SearchTarget::Paragraph,
                SearchTarget::File,
            ] {
                assert_eq!(
                    project(bytes, "needle", target, failure),
                    Err(SearchError::Unavailable)
                );
            }
        }
    }

    #[test]
    fn complete_extent_crosses_multiple_scratch_chunks() {
        let line = format!(
            "{}needle{}\n",
            "x".repeat(READ_BUFFER_SIZE),
            "y".repeat(READ_BUFFER_SIZE)
        );
        let (full, substring) =
            project(line.as_bytes(), "needle", SearchTarget::Line, None).unwrap();
        assert!(full.is_empty());
        assert_eq!(substring.len(), 1);
        assert_eq!(substring[0].target(), AnddressTarget::Line);
        assert_eq!(
            (substring[0].byte_start(), substring[0].byte_end()),
            (0, line.len())
        );
    }

    #[test]
    fn target_projections_preserve_scratch_and_token_boundaries() {
        for length in [READ_BUFFER_SIZE - 1, READ_BUFFER_SIZE, READ_BUFFER_SIZE + 1] {
            let mut source = b"needle".to_vec();
            source.resize(length - 1, b'x');
            source.push(b'\n');
            for target in [
                SearchTarget::File,
                SearchTarget::Paragraph,
                SearchTarget::Line,
            ] {
                let (tier, anddress) = only_result(&source, "needle", target);
                assert_eq!(tier, MatchTier::Substring);
                assert_eq!((anddress.byte_start(), anddress.byte_end()), (0, length));
            }
        }

        let utf8_split = format!("{}éneedle\r\n", "x".repeat(READ_BUFFER_SIZE - 1));
        let (_, line) = only_result(utf8_split.as_bytes(), "needle", SearchTarget::Line);
        assert_eq!((line.byte_start(), line.byte_end()), (0, utf8_split.len()));

        let crlf_split = format!("{}\r\nneedle\n", "x".repeat(READ_BUFFER_SIZE - 1));
        let (tier, line) = only_result(crlf_split.as_bytes(), "needle", SearchTarget::Line);
        assert_eq!(tier, MatchTier::FullLine);
        assert_eq!(
            (line.byte_start(), line.byte_end()),
            (READ_BUFFER_SIZE + 1, crlf_split.len())
        );

        let literal_split = format!("{}needle\n", "x".repeat(READ_BUFFER_SIZE - 3));
        let (_, line) = only_result(literal_split.as_bytes(), "needle", SearchTarget::Line);
        assert_eq!(
            (line.byte_start(), line.byte_end()),
            (0, literal_split.len())
        );
    }

    #[test]
    fn exact_file_uses_one_observation_without_search_framing() {
        for bytes in [b"".as_slice(), b"one\r\ntwo\n".as_slice()] {
            let mut reader = FixtureReader {
                bytes,
                cursor: 0,
                max_chunk: READ_BUFFER_SIZE,
                fail_after: None,
                returned_eof: false,
                failed: false,
            };
            let mut observed = 0_usize;
            let state = observe_source(&mut reader, |chunk, start| {
                assert_eq!(start, observed);
                observed = observed
                    .checked_add(chunk.len())
                    .ok_or(SourceScanError::Resource)?;
                Ok(())
            })
            .unwrap();
            let mut hash = crate::hash::Sha256::new();
            hash.update(bytes);
            assert_eq!(state.hash, hash.finish().to_hex());
            assert_eq!(state.byte_length, bytes.len());
            assert_eq!(observed, bytes.len());
            assert!(reader.returned_eof);
        }

        let production = include_str!("search.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        let exact_file = production
            .split("fn execute_exact_file")
            .nth(1)
            .unwrap()
            .split("struct SearchExecutor")
            .next()
            .unwrap();
        assert_eq!(exact_file.matches("observe_source(").count(), 1);
        assert!(!exact_file.contains("scan_source"));
        assert!(!exact_file.contains("SourceFramer"));
    }

    #[test]
    fn file_full_line_stops_projection_but_not_observation_validation() {
        let query = SearchQuery::new("needle").unwrap();
        let literal = PreparedLiteral::new(&query).unwrap();
        let mut projection = FileProjection::new(&literal);
        projection.push(b"needle\n").unwrap();
        assert_eq!(projection.best_tier, Some(MatchTier::FullLine));
        assert!(!projection.line_started);
        projection.push(b"ignored\rstill ignored\n").unwrap();
        assert_eq!(projection.best_tier, Some(MatchTier::FullLine));
        assert!(!projection.line_started);

        assert_eq!(
            project(b"needle\n\xff", "needle", SearchTarget::File, None),
            Err(SearchError::Unavailable)
        );
    }

    #[test]
    fn search_production_has_no_generic_event_scan_or_persistent_observation() {
        let production = include_str!("search.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        assert!(!production.contains("SourceEvent"));
        assert!(!production.contains("scan_source("));
        assert!(!production.contains("Sha256"));
        assert!(!include_str!("../runtime.rs").contains("CurrentObservation"));
        assert_eq!(production.matches("observe_source(").count(), 4);
    }

    #[test]
    fn projection_resource_failure_stays_local_until_observation_succeeds() {
        let query = SearchQuery::new("needle").unwrap();
        let literal = PreparedLiteral::new(&query).unwrap();
        let mut projection = LineProjection::new(&literal);
        projection.push(b"needle\n", 0).unwrap();
        assert_eq!(projection.provisional.len(), 1);
        assert_eq!(
            projection.push(b"xx", usize::MAX),
            Err(SourceScanError::Resource)
        );

        let production = include_str!("search.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        let scan = production
            .split("fn scan_open_source")
            .nth(1)
            .unwrap()
            .split("impl<'a> FileProjection")
            .next()
            .unwrap();
        assert!(
            scan.find("let source = construct_source_identity")
                > scan.find("let (state, outcome) = match target")
        );
    }

    #[test]
    fn same_source_results_share_identity_and_encode_independently() {
        let (full, substring) =
            project(b"needle\nneedle\n", "needle", SearchTarget::Line, None).unwrap();
        assert!(substring.is_empty());
        assert_eq!(full.len(), 2);
        assert!(std::sync::Arc::ptr_eq(
            full[0].source_identity(),
            full[1].source_identity()
        ));
        assert_ne!(full[0].encode().unwrap(), full[1].encode().unwrap());
    }
}
