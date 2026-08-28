//! Runtime-owned Search execution over current-only forward source observations.

use std::cmp::Ordering;

use crate::backwriter::anddress::{Anddress, AnddressTarget, LineBodyClass, construct_anddress};
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
    source_scan::{DecimalOrdinal, SourceEvent, SourceScanError, scan_source},
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
    scan_source(&mut file, |_| Ok(())).map_err(|_| SearchError::Unavailable)?;
    let anddress = construct_anddress(
        &runtime.workspace_coordinate,
        logical_path,
        AnddressTarget::File,
    )
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
}

struct StreamProjection<'a> {
    workspace_coordinate: &'a str,
    logical_path: &'a str,
    target: SearchTarget,
    matcher: LiteralMatcher<'a>,
    line_extent: Vec<u8>,
    paragraph: Option<ParagraphState>,
    paragraph_ordinal: DecimalOrdinal,
    file_best: Option<MatchTier>,
    full_line_results: &'a mut Vec<Anddress>,
    substring_results: &'a mut Vec<Anddress>,
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
    let mut projection = StreamProjection {
        workspace_coordinate,
        logical_path,
        target,
        matcher: literal.matcher(),
        line_extent: Vec::new(),
        paragraph: None,
        paragraph_ordinal: DecimalOrdinal::zero().map_err(|_| SearchError::Unavailable)?,
        file_best: None,
        full_line_results,
        substring_results,
    };
    scan_source(reader, |event| projection.handle(event)).map_err(|_| SearchError::Unavailable)?;
    projection.finish()
}

impl StreamProjection<'_> {
    fn handle(&mut self, event: SourceEvent<'_>) -> Result<(), SourceScanError> {
        match event {
            SourceEvent::StartLine { .. } => {
                self.matcher.reset();
                if self.target == SearchTarget::Line {
                    self.line_extent.clear();
                }
            }
            SourceEvent::Byte { byte, content } => {
                if self.target == SearchTarget::Line {
                    self.line_extent
                        .try_reserve(1)
                        .map_err(|_| SourceScanError::Resource)?;
                    self.line_extent.push(byte);
                }
                if content {
                    self.matcher.push(byte);
                }
            }
            SourceEvent::EndLine {
                ordinal,
                body_class,
            } => self.finish_line(ordinal, body_class)?,
        }
        Ok(())
    }

    fn finish_line(
        &mut self,
        ordinal: &DecimalOrdinal,
        body_class: LineBodyClass,
    ) -> Result<(), SourceScanError> {
        let tier = self.matcher.finish();
        if self.target == SearchTarget::File
            && tier.is_some_and(|tier| self.file_best.is_none_or(|best| tier < best))
        {
            self.file_best = tier;
        }
        if body_class == LineBodyClass::Text {
            let paragraph = self
                .paragraph
                .get_or_insert(ParagraphState { best_tier: None });
            if let Some(tier) = tier {
                match self.target {
                    SearchTarget::Line => self.push_line(tier, ordinal)?,
                    SearchTarget::Paragraph
                        if paragraph.best_tier.is_none_or(|best| tier < best) =>
                    {
                        paragraph.best_tier = Some(tier)
                    }
                    _ => {}
                }
            }
        } else {
            self.close_paragraph()?;
            if self.target == SearchTarget::Line
                && let Some(tier) = tier
            {
                self.push_line(tier, ordinal)?;
            }
        }
        Ok(())
    }

    fn close_paragraph(&mut self) -> Result<(), SourceScanError> {
        let Some(paragraph) = self.paragraph.take() else {
            return Ok(());
        };
        if self.target == SearchTarget::Paragraph
            && let Some(tier) = paragraph.best_tier
        {
            self.push(
                tier,
                AnddressTarget::Paragraph {
                    ordinal: self.paragraph_ordinal.as_natural()?,
                },
            )?;
        }
        self.paragraph_ordinal.increment()?;
        Ok(())
    }

    fn push_line(
        &mut self,
        tier: MatchTier,
        ordinal: &DecimalOrdinal,
    ) -> Result<(), SourceScanError> {
        let bytes =
            std::str::from_utf8(&self.line_extent).map_err(|_| SourceScanError::InvalidSource)?;
        let mut extent = String::new();
        extent
            .try_reserve_exact(bytes.len())
            .map_err(|_| SourceScanError::Resource)?;
        extent.push_str(bytes);
        self.push(
            tier,
            AnddressTarget::Line {
                ordinal: ordinal.as_natural()?,
                exact_extent: extent,
            },
        )
    }

    fn push(&mut self, tier: MatchTier, target: AnddressTarget) -> Result<(), SourceScanError> {
        let bucket = match tier {
            MatchTier::FullLine => &mut self.full_line_results,
            MatchTier::Substring => &mut self.substring_results,
        };
        bucket
            .try_reserve(1)
            .map_err(|_| SourceScanError::Resource)?;
        bucket.push(
            construct_anddress(self.workspace_coordinate, self.logical_path, target)
                .map_err(|_| SourceScanError::Resource)?,
        );
        Ok(())
    }

    fn finish(&mut self) -> Result<(), SearchError> {
        self.close_paragraph()
            .map_err(|_| SearchError::Unavailable)?;
        if self.target == SearchTarget::File
            && let Some(tier) = self.file_best
        {
            self.push(tier, AnddressTarget::File)
                .map_err(|_| SearchError::Unavailable)?;
        }
        Ok(())
    }
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
    left.logical_path
        .as_bytes()
        .cmp(right.logical_path.as_bytes())
        .then_with(|| match (&left.target, &right.target) {
            (
                AnddressTarget::Line { ordinal: left, .. },
                AnddressTarget::Line { ordinal: right, .. },
            ) => left.cmp(right),
            (
                AnddressTarget::Paragraph { ordinal: left },
                AnddressTarget::Paragraph { ordinal: right },
            ) => left.cmp(right),
            (AnddressTarget::File, AnddressTarget::File) => Ordering::Equal,
            _ => Ordering::Equal,
        })
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
            let count = 1_usize
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
        let query = SearchQuery::new(query).unwrap();
        let literal = PreparedLiteral::new(&query)?;
        let mut reader = FixtureReader {
            bytes,
            cursor: 0,
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
        assert!(matches!(
            &full[0].target,
            AnddressTarget::Line { exact_extent, .. } if exact_extent == "needle\r"
        ));
        assert!(matches!(
            &full[1].target,
            AnddressTarget::Line { exact_extent, .. } if exact_extent == "needle"
        ));
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
        assert!(matches!(
            &substring[0].target,
            AnddressTarget::Line { exact_extent, .. } if exact_extent == &line
        ));
    }

    #[test]
    fn matched_line_copies_exact_extent_without_taking_reusable_scratch() {
        let query = SearchQuery::new("needle").unwrap();
        let literal = PreparedLiteral::new(&query).unwrap();
        let coordinate = "0".repeat(64);
        let mut full = Vec::new();
        let mut substring = Vec::new();
        let mut scratch = Vec::new();
        scratch.try_reserve_exact(READ_BUFFER_SIZE).unwrap();
        scratch.extend_from_slice(b"needle\n");
        let pointer = scratch.as_ptr();
        let capacity = scratch.capacity();
        let mut projection = StreamProjection {
            workspace_coordinate: &coordinate,
            logical_path: "source.txt",
            target: SearchTarget::Line,
            matcher: literal.matcher(),
            line_extent: scratch,
            paragraph: None,
            paragraph_ordinal: DecimalOrdinal::zero().unwrap(),
            file_best: None,
            full_line_results: &mut full,
            substring_results: &mut substring,
        };

        projection
            .push_line(MatchTier::FullLine, &DecimalOrdinal::zero().unwrap())
            .unwrap();

        assert_eq!(projection.line_extent.as_ptr(), pointer);
        assert_eq!(projection.line_extent.capacity(), capacity);
        assert!(matches!(
            &full[0].target,
            AnddressTarget::Line { exact_extent, .. }
                if exact_extent == "needle\n" && exact_extent.as_ptr() != pointer
        ));
    }
}
