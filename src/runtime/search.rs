//! Runtime-owned Search execution over current-only forward source observations.

use std::cmp::Ordering;

use crate::backwriter::anddress::{
    AnddressIssuer, LineBodyClass, ParagraphGeometry, TargetGeometry, attach_line_to_paragraph,
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
    CurrentProof, DirectoryAccessError, WorkspaceRuntime, is_backwriter_spill, path_is_within_root,
    source_scan::{CurrentObservation, SourceScanError, observe_source, observe_structural},
    structural_cursor::{LineSpan, StructuralSink},
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
        proofs: Vec::new(),
    };
    executor.preflight()?;
    executor.execute()?;
    executor.full_line_results.sort_unstable_by(compare_bucket);
    executor.substring_results.sort_unstable_by(compare_bucket);
    let anddresses = join_result_buckets(executor.full_line_results, executor.substring_results)?;
    let outcome = if anddresses.is_empty() {
        SearchOutcome::Empty
    } else {
        SearchOutcome::Found { anddresses }
    };
    runtime.install_search_proofs(executor.proofs)?;
    Ok(outcome)
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
        Err(DirectoryAccessError::NotCurrent) => {
            runtime.invalidate_current_proof(logical_path);
            return Ok(SearchOutcome::Empty);
        }
        Err(DirectoryAccessError::Unavailable) => {
            runtime.invalidate_current_proof(logical_path);
            return Err(SearchError::Unavailable);
        }
    };
    let state = match observe_source(&mut file, |_, _| Ok(())) {
        Ok(state) => state,
        Err(_) => {
            runtime.invalidate_current_proof(logical_path);
            return Err(SearchError::Unavailable);
        }
    };
    let issuer = AnddressIssuer::new(
        &runtime.workspace_coordinate,
        logical_path,
        &state.hash,
        state.byte_length,
        state.line_count,
    )
    .map_err(|_| SearchError::Unavailable)?;
    let anddress = issuer
        .issue(TargetGeometry::File)
        .map_err(|_| SearchError::Unavailable)?;
    let mut anddresses = Vec::new();
    anddresses
        .try_reserve_exact(1)
        .map_err(|_| SearchError::Unavailable)?;
    anddresses.push(anddress);
    let proof = CurrentProof::new(
        logical_path,
        state.hash,
        state.byte_length,
        state.line_count,
    )?;
    let mut proofs = Vec::new();
    proofs
        .try_reserve_exact(1)
        .map_err(|_| SearchError::Unavailable)?;
    proofs.push(proof);
    runtime.install_search_proofs(proofs)?;
    Ok(SearchOutcome::Found { anddresses })
}

struct SearchExecutor<'a> {
    runtime: &'a WorkspaceRuntime,
    scope: &'a SearchScope,
    target: SearchTarget,
    literal: PreparedLiteral<'a>,
    full_line_results: Vec<crate::backwriter::anddress::Anddress>,
    substring_results: Vec<crate::backwriter::anddress::Anddress>,
    proofs: Vec<CurrentProof>,
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
        let directory = match self.open_logical_directory(path) {
            Ok(directory) => directory,
            Err(error) => {
                self.runtime.invalidate_current_proof(path);
                return Err(error);
            }
        };
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
                                let classified = match classify_child(&current.directory, name) {
                                    Ok(classified) => classified,
                                    Err(_) => {
                                        self.runtime.invalidate_current_proof(&path);
                                        return Err(SearchError::Unavailable);
                                    }
                                };
                                match classified {
                                    ClassifiedChild::Directory => {
                                        self.runtime.invalidate_current_proof(&path);
                                        let directory = match open_directory(
                                            &current.directory,
                                            name,
                                            ClassifiedChild::Directory,
                                        ) {
                                            Ok(directory) => directory,
                                            Err(_) => return Err(SearchError::Unavailable),
                                        };
                                        Next::Directory { path, directory }
                                    }
                                    ClassifiedChild::Regular => {
                                        let file = match open_regular(
                                            &current.directory,
                                            name,
                                            ClassifiedChild::Regular,
                                        ) {
                                            Ok(file) => file,
                                            Err(_) => {
                                                self.runtime.invalidate_current_proof(&path);
                                                return Err(SearchError::Unavailable);
                                            }
                                        };
                                        Next::Source { path, file }
                                    }
                                    ClassifiedChild::Excluded => {
                                        self.runtime.invalidate_current_proof(&path);
                                        Next::Ignored
                                    }
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
        let file = match self.runtime.open_admitted_source(path) {
            Ok(file) => file,
            Err(DirectoryAccessError::Unadmitted) => return Err(SearchError::InvalidScope),
            Err(DirectoryAccessError::NotCurrent | DirectoryAccessError::Unavailable) => {
                self.runtime.invalidate_current_proof(path);
                return Err(SearchError::Unavailable);
            }
        };
        self.search_open_source(path, file)
    }

    fn search_open_source(
        &mut self,
        path: &str,
        mut file: cap_std::fs::File,
    ) -> Result<(), SearchError> {
        let state = match scan_open_source(
            &mut file,
            &self.runtime.workspace_coordinate,
            path,
            &self.literal,
            self.target,
            &mut self.full_line_results,
            &mut self.substring_results,
        ) {
            Ok(state) => state,
            Err(error) => {
                self.runtime.invalidate_current_proof(path);
                return Err(error);
            }
        };
        let proof = CurrentProof::new(path, state.hash, state.byte_length, state.line_count)?;
        self.proofs
            .try_reserve(1)
            .map_err(|_| SearchError::Unavailable)?;
        self.proofs.push(proof);
        Ok(())
    }
}

struct ProvisionalTarget {
    tier: MatchTier,
    geometry: TargetGeometry,
}

const PENDING_CHUNK_CAPACITY: usize = 16_384;

#[derive(Default)]
struct PendingTargets {
    chunks: Vec<Vec<ProvisionalTarget>>,
    len: usize,
}

impl PendingTargets {
    fn push(&mut self, tier: MatchTier, geometry: TargetGeometry) -> Result<(), SourceScanError> {
        let next_len = self.len.checked_add(1).ok_or(SourceScanError::Resource)?;
        if self
            .chunks
            .last()
            .is_none_or(|chunk| chunk.len() == PENDING_CHUNK_CAPACITY)
        {
            self.chunks
                .try_reserve(1)
                .map_err(|_| SourceScanError::Resource)?;
            let mut chunk = Vec::new();
            chunk
                .try_reserve_exact(PENDING_CHUNK_CAPACITY)
                .map_err(|_| SourceScanError::Resource)?;
            self.chunks.push(chunk);
        }
        self.chunks
            .last_mut()
            .expect("pending chunk was created")
            .push(ProvisionalTarget { tier, geometry });
        self.len = next_len;
        Ok(())
    }

    fn attach_paragraph(
        &mut self,
        start: usize,
        end: usize,
        paragraph: ParagraphGeometry,
    ) -> Result<(), SourceScanError> {
        if start > end || end > self.len {
            return Err(SourceScanError::InvalidSource);
        }
        if start == end {
            return Ok(());
        }
        let first_chunk = start / PENDING_CHUNK_CAPACITY;
        let last_chunk = (end - 1) / PENDING_CHUNK_CAPACITY;
        for chunk_index in first_chunk..=last_chunk {
            let base = chunk_index * PENDING_CHUNK_CAPACITY;
            let chunk = self
                .chunks
                .get_mut(chunk_index)
                .ok_or(SourceScanError::InvalidSource)?;
            let local_start = start.saturating_sub(base);
            let local_end = (end - base).min(chunk.len());
            for target in &mut chunk[local_start..local_end] {
                if !attach_line_to_paragraph(&mut target.geometry, paragraph)
                    .map_err(|_| SourceScanError::InvalidSource)?
                {
                    return Err(SourceScanError::InvalidSource);
                }
            }
        }
        Ok(())
    }
}

struct SearchProjection<'a> {
    target: SearchTarget,
    matcher: LiteralMatcher<'a>,
    file_tier: Option<MatchTier>,
    paragraph_tier: Option<MatchTier>,
    paragraph_result_start: Option<usize>,
    paragraph_result_end: usize,
    pending: PendingTargets,
}

fn scan_open_source(
    reader: &mut impl std::io::Read,
    workspace_coordinate: &str,
    logical_path: &str,
    literal: &PreparedLiteral<'_>,
    target: SearchTarget,
    full_line_results: &mut Vec<crate::backwriter::anddress::Anddress>,
    substring_results: &mut Vec<crate::backwriter::anddress::Anddress>,
) -> Result<CurrentObservation, SearchError> {
    let mut projection = SearchProjection::new(literal, target);
    let state =
        observe_structural(reader, &mut projection).map_err(|_| SearchError::Unavailable)?;
    let issuer = AnddressIssuer::new(
        workspace_coordinate,
        logical_path,
        &state.hash,
        state.byte_length,
        state.line_count,
    )
    .map_err(|_| SearchError::Unavailable)?;
    if target == SearchTarget::File {
        if let Some(tier) = projection.file_tier {
            let anddress = issuer
                .issue(TargetGeometry::File)
                .map_err(|_| SearchError::Unavailable)?;
            push_result(full_line_results, substring_results, tier, anddress)?;
        }
    } else {
        for chunk in projection.pending.chunks {
            for provisional in chunk {
                let anddress = issuer
                    .issue(provisional.geometry)
                    .map_err(|_| SearchError::Unavailable)?;
                push_result(
                    full_line_results,
                    substring_results,
                    provisional.tier,
                    anddress,
                )?;
            }
        }
    }
    Ok(state)
}

impl<'a> SearchProjection<'a> {
    fn new(literal: &'a PreparedLiteral<'a>, target: SearchTarget) -> Self {
        Self {
            target,
            matcher: literal.matcher(),
            file_tier: None,
            paragraph_tier: None,
            paragraph_result_start: None,
            paragraph_result_end: 0,
            pending: PendingTargets::default(),
        }
    }
}

impl StructuralSink for SearchProjection<'_> {
    fn begin_line(
        &mut self,
        _byte_start: usize,
        _file_line_offset: usize,
    ) -> Result<(), SourceScanError> {
        self.matcher.reset();
        Ok(())
    }

    fn segment(
        &mut self,
        bytes: &[u8],
        _byte_start: usize,
        is_content: bool,
    ) -> Result<(), SourceScanError> {
        let saturated = match self.target {
            SearchTarget::File => self.file_tier == Some(MatchTier::FullLine),
            SearchTarget::Paragraph => self.paragraph_tier == Some(MatchTier::FullLine),
            SearchTarget::Line => false,
        };
        if is_content && !saturated {
            self.matcher
                .push_segment(bytes)
                .map_err(|_| SourceScanError::Resource)?;
        }
        Ok(())
    }

    fn line(&mut self, line: LineSpan) -> Result<(), SourceScanError> {
        let tier = self.matcher.finish();
        match self.target {
            SearchTarget::File => {
                if let Some(tier) = tier {
                    prefer_tier(&mut self.file_tier, tier);
                }
            }
            SearchTarget::Paragraph => {
                if line.body_class == LineBodyClass::Text
                    && let Some(tier) = tier
                {
                    prefer_tier(&mut self.paragraph_tier, tier);
                }
            }
            SearchTarget::Line => {
                if line.body_class == LineBodyClass::Text && self.paragraph_result_start.is_none() {
                    self.paragraph_result_start = Some(self.pending.len);
                }
                if let Some(tier) = tier {
                    self.pending.push(tier, line.file_geometry())?;
                }
                if line.body_class == LineBodyClass::Text {
                    self.paragraph_result_end = self.pending.len;
                }
            }
        }
        Ok(())
    }

    fn paragraph(&mut self, paragraph: ParagraphGeometry) -> Result<(), SourceScanError> {
        match self.target {
            SearchTarget::File => {}
            SearchTarget::Paragraph => {
                if let Some(tier) = self.paragraph_tier.take() {
                    self.pending
                        .push(tier, TargetGeometry::Paragraph(paragraph))?;
                }
            }
            SearchTarget::Line => {
                if let Some(start) = self.paragraph_result_start.take() {
                    self.pending
                        .attach_paragraph(start, self.paragraph_result_end, paragraph)?;
                }
            }
        }
        self.paragraph_result_end = self.pending.len;
        Ok(())
    }
}

fn prefer_tier(best: &mut Option<MatchTier>, tier: MatchTier) {
    if best.is_none_or(|current| tier < current) {
        *best = Some(tier);
    }
}

fn push_result(
    full_line_results: &mut Vec<crate::backwriter::anddress::Anddress>,
    substring_results: &mut Vec<crate::backwriter::anddress::Anddress>,
    tier: MatchTier,
    anddress: crate::backwriter::anddress::Anddress,
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
    mut full_line_results: Vec<crate::backwriter::anddress::Anddress>,
    mut substring_results: Vec<crate::backwriter::anddress::Anddress>,
) -> Result<Vec<crate::backwriter::anddress::Anddress>, SearchError> {
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

fn compare_bucket(
    left: &crate::backwriter::anddress::Anddress,
    right: &crate::backwriter::anddress::Anddress,
) -> Ordering {
    left.logical_path()
        .as_bytes()
        .cmp(right.logical_path().as_bytes())
        .then_with(|| left.byte_start().cmp(&right.byte_start()))
        .then_with(|| left.byte_end().cmp(&right.byte_end()))
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{self, Read},
    };

    use crate::{
        backwriter::{
            anchor::AnchorOutcome,
            anddress::{Anddress, AnddressTarget},
            edit::{Edit, Position},
            search::{SearchQuery, SearchRequest, SearchScope, SearchScopeEntry, SearchTarget},
            view::ViewOutcome,
        },
        hash::Sha256,
        runtime::{
            AdmissionRoot, WorkspaceAdmission, source_scan::READ_BUFFER_SIZE,
            structural_cursor::StructuralCursor,
        },
    };

    use super::*;

    fn admission(root: &str) -> WorkspaceAdmission {
        WorkspaceAdmission::new([AdmissionRoot::new(root).unwrap()]).unwrap()
    }

    fn host_runtime(root: &std::path::Path) -> WorkspaceRuntime {
        WorkspaceRuntime::open_host_authoritative(root, admission(".")).unwrap()
    }

    fn proofs(runtime: &WorkspaceRuntime) -> Vec<(String, String, usize)> {
        runtime
            .current_proofs
            .lock()
            .unwrap()
            .iter()
            .map(|proof| {
                (
                    proof.logical_path.clone(),
                    proof.hash.clone(),
                    proof.byte_length,
                )
            })
            .collect()
    }

    fn source_hash(bytes: &[u8]) -> String {
        let mut hash = Sha256::new();
        hash.update(bytes);
        hash.finish().to_hex()
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
        project_occurrences_chunked(bytes, query, target, fail_after, max_chunk)
    }

    fn project_occurrences_chunked(
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

    fn pending_line(index: usize) -> TargetGeometry {
        TargetGeometry::Line {
            byte_start: index,
            byte_end: index + 1,
            terminator: crate::backwriter::anddress::LineTerminator::None,
            line_offset_in_parent: index,
            parent: crate::backwriter::anddress::ParentGeometry::File,
        }
    }

    #[test]
    fn pending_chunks_keep_global_ranges_order_and_paragraph_boundaries() {
        for count in [
            1,
            PENDING_CHUNK_CAPACITY - 1,
            PENDING_CHUNK_CAPACITY,
            PENDING_CHUNK_CAPACITY + 1,
        ] {
            let mut pending = PendingTargets::default();
            for index in 0..count {
                pending
                    .push(
                        if index % 2 == 0 {
                            MatchTier::FullLine
                        } else {
                            MatchTier::Substring
                        },
                        pending_line(index),
                    )
                    .unwrap();
            }
            assert_eq!(pending.len, count);
            assert_eq!(pending.chunks.len(), count.div_ceil(PENDING_CHUNK_CAPACITY));
            assert!(
                pending
                    .chunks
                    .iter()
                    .take(pending.chunks.len().saturating_sub(1))
                    .all(|chunk| chunk.len() == PENDING_CHUNK_CAPACITY)
            );
            assert_eq!(
                pending
                    .chunks
                    .iter()
                    .flatten()
                    .map(|target| target.geometry)
                    .collect::<Vec<_>>(),
                (0..count).map(pending_line).collect::<Vec<_>>()
            );
        }

        let text_lines = PENDING_CHUNK_CAPACITY + 1;
        let mut pending = PendingTargets::default();
        for index in 0..text_lines + 1 {
            pending
                .push(MatchTier::FullLine, pending_line(index))
                .unwrap();
        }
        let paragraph = ParagraphGeometry {
            byte_start: 0,
            byte_end: text_lines,
            file_line_offset: 0,
            line_count: text_lines,
        };
        pending.attach_paragraph(0, text_lines, paragraph).unwrap();
        let flattened = pending.chunks.iter().flatten().collect::<Vec<_>>();
        for (index, target) in flattened[..text_lines].iter().enumerate() {
            assert!(matches!(
                target.geometry,
                TargetGeometry::Line {
                    line_offset_in_parent,
                    parent: crate::backwriter::anddress::ParentGeometry::Paragraph(parent),
                    ..
                } if line_offset_in_parent == index && parent == paragraph
            ));
        }
        assert_eq!(flattened[text_lines].geometry, pending_line(text_lines));
    }

    #[test]
    fn pending_chunks_attach_many_one_line_paragraphs_without_replaying_prior_chunks() {
        let count = PENDING_CHUNK_CAPACITY + 1;
        let mut pending = PendingTargets::default();
        for index in 0..count {
            pending
                .push(MatchTier::Substring, pending_line(index))
                .unwrap();
            pending
                .attach_paragraph(
                    index,
                    index + 1,
                    ParagraphGeometry {
                        byte_start: index,
                        byte_end: index + 1,
                        file_line_offset: index,
                        line_count: 1,
                    },
                )
                .unwrap();
        }
        assert_eq!(pending.chunks.len(), 2);
        for (index, target) in pending.chunks.iter().flatten().enumerate() {
            assert!(matches!(
                target.geometry,
                TargetGeometry::Line {
                    line_offset_in_parent: 0,
                    parent: crate::backwriter::anddress::ParentGeometry::Paragraph(parent),
                    ..
                } if parent.file_line_offset == index && parent.line_count == 1
            ));
        }
    }

    #[test]
    fn pending_resource_boundary_fails_without_adding_or_issuing_a_target() {
        let mut pending = PendingTargets {
            chunks: Vec::new(),
            len: usize::MAX,
        };
        assert_eq!(
            pending.push(MatchTier::FullLine, pending_line(0)),
            Err(SourceScanError::Resource)
        );
        assert!(pending.chunks.is_empty());
    }

    #[test]
    fn cross_chunk_full_line_and_substring_tiers_keep_global_order() {
        let mut source = Vec::new();
        source
            .try_reserve_exact((PENDING_CHUNK_CAPACITY + 1) * 9)
            .unwrap();
        for index in 0..PENDING_CHUNK_CAPACITY + 1 {
            source.extend_from_slice(if index % 2 == 0 {
                b"needle\n"
            } else {
                b"xneedle\n"
            });
        }
        let (full, substring) = project_chunked(
            &source,
            "needle",
            SearchTarget::Line,
            None,
            READ_BUFFER_SIZE,
        )
        .unwrap();
        assert_eq!(full.len(), PENDING_CHUNK_CAPACITY / 2 + 1);
        assert_eq!(substring.len(), PENDING_CHUNK_CAPACITY / 2);
        assert!(
            full.windows(2)
                .chain(substring.windows(2))
                .all(|pair| pair[0].byte_start() < pair[1].byte_start())
        );
        assert!(
            full.iter()
                .all(|anddress| anddress.line_number().unwrap() % 2 == 1)
        );
        assert!(
            substring
                .iter()
                .all(|anddress| anddress.line_number().unwrap() % 2 == 0)
        );
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
    fn bulk_literal_matching_preserves_tiers_fallback_and_dense_candidates() {
        let (full, substring) =
            project_chunked(b"x\nax\n", "x", SearchTarget::Line, None, 1).unwrap();
        assert_eq!(full.len(), 1);
        assert_eq!((full[0].byte_start(), full[0].byte_end()), (0, 2));
        assert_eq!(substring.len(), 1);
        assert_eq!((substring[0].byte_start(), substring[0].byte_end()), (2, 5));

        let (full, substring) = project_chunked(
            b"need\nneedle\nneedlex\n",
            "needle",
            SearchTarget::Line,
            None,
            READ_BUFFER_SIZE,
        )
        .unwrap();
        assert_eq!(full.len(), 1);
        assert_eq!((full[0].byte_start(), full[0].byte_end()), (5, 12));
        assert_eq!(substring.len(), 1);
        assert_eq!(
            (substring[0].byte_start(), substring[0].byte_end()),
            (12, 20)
        );

        for (source, query) in [("aaaaaaaaab\n", "aaaaab"), ("abababaca\n", "ababaca")] {
            let (full, substring) =
                project_chunked(source.as_bytes(), query, SearchTarget::Line, None, 2).unwrap();
            assert!(full.is_empty());
            assert_eq!(substring.len(), 1);
        }

        let dense_no_hit = format!("{}\n", "a".repeat(READ_BUFFER_SIZE * 2));
        let (full, substring) = project_chunked(
            dense_no_hit.as_bytes(),
            "aaaaab",
            SearchTarget::Line,
            None,
            READ_BUFFER_SIZE,
        )
        .unwrap();
        assert!(full.is_empty() && substring.is_empty());

        let (full, substring) =
            project_chunked(b"abab\naca", "ababaca", SearchTarget::Line, None, 2).unwrap();
        assert!(full.is_empty() && substring.is_empty());
    }

    #[test]
    fn bulk_literal_matching_preserves_terminators_and_long_query_carry() {
        let source = b"needle\rneedle\nneedle\r\nneedle";
        let expected = project_chunked(source, "needle", SearchTarget::Line, None, 1).unwrap();
        for max_chunk in [
            2,
            7,
            READ_BUFFER_SIZE - 1,
            READ_BUFFER_SIZE,
            READ_BUFFER_SIZE + 1,
            source.len(),
        ] {
            assert_eq!(
                project_chunked(source, "needle", SearchTarget::Line, None, max_chunk).unwrap(),
                expected
            );
        }
        let (full, substring) = expected;
        assert!(substring.is_empty());
        assert_eq!(full.len(), 4);
        assert_eq!(
            full.iter()
                .map(|anddress| (anddress.byte_start(), anddress.byte_end()))
                .collect::<Vec<_>>(),
            vec![(0, 7), (7, 14), (14, 22), (22, 28)]
        );

        let query = format!("{}é", "z".repeat(READ_BUFFER_SIZE * 2));
        let source = format!("prefix{query}\n");
        let (full, substring) = project_chunked(
            source.as_bytes(),
            &query,
            SearchTarget::Line,
            None,
            READ_BUFFER_SIZE,
        )
        .unwrap();
        assert!(full.is_empty());
        assert_eq!(substring.len(), 1);
        assert_eq!(
            (substring[0].byte_start(), substring[0].byte_end()),
            (0, source.len())
        );
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
                let expected = project_chunked(&source, "needle", target, None, 1).unwrap();
                for max_chunk in [
                    2,
                    7,
                    READ_BUFFER_SIZE - 1,
                    READ_BUFFER_SIZE,
                    READ_BUFFER_SIZE + 1,
                    source.len(),
                ] {
                    assert_eq!(
                        project_chunked(&source, "needle", target, None, max_chunk).unwrap(),
                        expected
                    );
                }
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
    fn anddress_geometry_supplies_positions_across_scratch_boundaries() {
        let source = "\nα\rneedle\r\n \t\rneedle";
        let (lines, substring) =
            project_occurrences_chunked(source.as_bytes(), "needle", SearchTarget::Line, None, 1)
                .unwrap();
        assert!(substring.is_empty());
        assert_eq!(
            lines.iter().map(Anddress::line_number).collect::<Vec<_>>(),
            vec![Some(3), Some(5)]
        );

        let (paragraphs, substring) = project_occurrences_chunked(
            source.as_bytes(),
            "needle",
            SearchTarget::Paragraph,
            None,
            1,
        )
        .unwrap();
        assert!(substring.is_empty());
        assert_eq!(
            paragraphs
                .iter()
                .map(Anddress::line_range)
                .collect::<Vec<_>>(),
            vec![1..3, 4..5]
        );

        let (separator, substring) = project_occurrences_chunked(
            source.as_bytes(),
            " \t",
            SearchTarget::Line,
            None,
            READ_BUFFER_SIZE,
        )
        .unwrap();
        assert!(substring.is_empty());
        assert_eq!(separator[0].line_number(), Some(4));

        let (terminal, substring) =
            project_occurrences_chunked(b"needle\r\n", "needle", SearchTarget::Line, None, 1)
                .unwrap();
        assert!(substring.is_empty());
        assert_eq!(terminal.len(), 1);
        assert_eq!(terminal[0].line_number(), Some(1));

        for byte_start in [READ_BUFFER_SIZE - 1, READ_BUFFER_SIZE, READ_BUFFER_SIZE + 1] {
            let mut boundary = vec![b'x'; byte_start - 1];
            boundary.extend_from_slice(b"\nneedle");
            let (lines, substring) = project_occurrences_chunked(
                &boundary,
                "needle",
                SearchTarget::Line,
                None,
                READ_BUFFER_SIZE,
            )
            .unwrap();
            assert!(substring.is_empty());
            assert_eq!(lines[0].byte_start(), byte_start);
            assert_eq!(lines[0].line_number(), Some(2));

            let (paragraphs, substring) = project_occurrences_chunked(
                &boundary,
                "needle",
                SearchTarget::Paragraph,
                None,
                READ_BUFFER_SIZE,
            )
            .unwrap();
            assert!(substring.is_empty());
            assert_eq!(paragraphs[0].line_range(), 0..2);
        }
    }

    #[test]
    fn exact_file_uses_one_observation_without_literal_projection() {
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
        let source_scan = include_str!("source_scan.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        let raw = source_scan
            .split("pub(crate) fn observe_source")
            .nth(1)
            .unwrap()
            .split("pub(crate) fn observe_structural")
            .next()
            .unwrap();
        assert!(!raw.contains("observe_structural"));
        assert!(!raw.contains("StructuralCursor"));
    }

    #[test]
    fn file_full_line_stops_projection_but_not_observation_validation() {
        let query = SearchQuery::new("needle").unwrap();
        let literal = PreparedLiteral::new(&query).unwrap();
        let mut projection = SearchProjection::new(&literal, SearchTarget::File);
        let mut cursor = StructuralCursor::default();
        cursor.push(b"needle\n", &mut projection).unwrap();
        assert_eq!(projection.file_tier, Some(MatchTier::FullLine));
        cursor
            .push(b"ignored\rstill ignored\n", &mut projection)
            .unwrap();
        assert_eq!(projection.file_tier, Some(MatchTier::FullLine));
        cursor.finish(&mut projection).unwrap();

        let (full, substring) = project_chunked(
            b"needle\nignored\n\nneedle x\n",
            "needle",
            SearchTarget::Paragraph,
            None,
            1,
        )
        .unwrap();
        assert_eq!(full.len(), 1);
        assert_eq!(substring.len(), 1);

        let production = include_str!("search.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        let projection = production
            .split("impl StructuralSink for SearchProjection")
            .nth(1)
            .unwrap();
        assert_eq!(projection.matches(".push_segment(bytes)").count(), 1);
        assert!(!projection.contains("for &byte in bytes"));

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
        assert_eq!(production.matches("observe_source(").count(), 1);
        assert_eq!(production.matches("observe_structural(").count(), 1);
    }

    #[test]
    fn issuer_runs_only_after_the_structural_observation_succeeds() {
        let production = include_str!("search.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        let scan = production
            .split("fn scan_open_source")
            .nth(1)
            .unwrap()
            .split("impl<'a> SearchProjection")
            .next()
            .unwrap();
        assert!(
            scan.find("let issuer = AnddressIssuer::new")
                > scan.find("let state = observe_structural")
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

    #[test]
    fn only_host_search_installs_exact_content_and_file_proofs() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("exact.txt"), b"exact\n").unwrap();
        fs::write(fixture.path().join("content.txt"), b"needle\n").unwrap();

        let untrusted = WorkspaceRuntime::open(fixture.path(), admission(".")).unwrap();
        exact_file(&untrusted, "exact.txt");
        assert!(proofs(&untrusted).is_empty());

        let host = host_runtime(fixture.path());
        exact_file(&host, "exact.txt");
        host.search(&SearchRequest::new(
            SearchQuery::new("needle").unwrap(),
            SearchScope::only([SearchScopeEntry::source("content.txt").unwrap()]).unwrap(),
            SearchTarget::Line,
        ))
        .unwrap();
        assert_eq!(
            proofs(&host),
            vec![
                ("content.txt".to_owned(), source_hash(b"needle\n"), 7,),
                ("exact.txt".to_owned(), source_hash(b"exact\n"), 6),
            ]
        );
    }

    #[test]
    fn host_research_replaces_one_path_without_retaining_the_old_state() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("note.txt"), b"old\n").unwrap();
        let mut host = host_runtime(fixture.path());

        exact_file(&host, "note.txt");
        exact_file(&host, "note.txt");
        assert_eq!(proofs(&host).len(), 1);
        let old_hash = source_hash(b"old\n");
        assert_eq!(proofs(&host)[0].1, old_hash);

        host.invalidate_source("note.txt").unwrap();
        fs::write(fixture.path().join("note.txt"), b"new state\n").unwrap();
        exact_file(&host, "note.txt");
        let current = proofs(&host);
        assert_eq!(current.len(), 1);
        assert_eq!(current[0].1, source_hash(b"new state\n"));
        assert_eq!(current[0].2, 10);
        assert_ne!(current[0].1, old_hash);

        fs::write(fixture.path().join("note.txt"), b"invalid\0source").unwrap();
        assert_eq!(
            host.search(&SearchRequest::exact_file("note.txt").unwrap()),
            Err(SearchError::Unavailable)
        );
        assert!(proofs(&host).is_empty());

        fs::remove_file(fixture.path().join("note.txt")).unwrap();
        assert_eq!(
            host.search(&SearchRequest::exact_file("note.txt").unwrap()),
            Ok(SearchOutcome::Empty)
        );
        assert!(proofs(&host).is_empty());
    }

    #[test]
    fn host_multi_source_success_installs_independent_proofs_and_failure_installs_none() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("a.txt"), b"needle\n").unwrap();
        fs::write(fixture.path().join("b.txt"), b"other\n").unwrap();
        let host = host_runtime(fixture.path());
        host.search(&SearchRequest::new(
            SearchQuery::new("needle").unwrap(),
            SearchScope::all_admitted(),
            SearchTarget::Line,
        ))
        .unwrap();
        assert_eq!(
            proofs(&host)
                .iter()
                .map(|proof| proof.0.as_str())
                .collect::<Vec<_>>(),
            vec!["a.txt", "b.txt"]
        );

        fs::write(fixture.path().join("z.txt"), b"late\xff").unwrap();
        let failed = host_runtime(fixture.path());
        assert_eq!(
            failed.search(&SearchRequest::new(
                SearchQuery::new("needle").unwrap(),
                SearchScope::all_admitted(),
                SearchTarget::Line,
            )),
            Err(SearchError::Unavailable)
        );
        assert!(proofs(&failed).is_empty());
    }

    #[test]
    fn host_proofs_are_runtime_path_exact_and_share_anchor_apply_invalidation() {
        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("note.txt"), b"one\n").unwrap();
        fs::write(fixture.path().join("other.txt"), b"two\n").unwrap();
        let mut host = host_runtime(fixture.path());
        let note = exact_file(&host, "note.txt");
        let other = exact_file(&host, "other.txt");
        let note_handle = match host.anchor(&note).unwrap() {
            AnchorOutcome::Anchored(handle) => handle,
            AnchorOutcome::AlreadyLive => panic!("note Anchor"),
        };
        let other_handle = match host.anchor(&other).unwrap() {
            AnchorOutcome::Anchored(handle) => handle,
            AnchorOutcome::AlreadyLive => panic!("other Anchor"),
        };

        host.invalidate_source("note.txt").unwrap();
        assert_eq!(
            proofs(&host)
                .iter()
                .map(|proof| proof.0.as_str())
                .collect::<Vec<_>>(),
            vec!["other.txt"]
        );
        assert!(
            host.view_anchored(&note_handle, AnddressTarget::File)
                .is_err()
        );
        assert!(matches!(
            host.view_anchored(&other_handle, AnddressTarget::File),
            Ok(ViewOutcome::Projected { content, .. }) if content == "two\n"
        ));

        exact_file(&host, "note.txt");
        let current_other = exact_file(&host, "other.txt");
        host.apply(&Edit::Insert {
            position: Position::StartOf(current_other),
            content: String::new(),
        })
        .unwrap();
        assert_eq!(
            proofs(&host)
                .iter()
                .map(|proof| proof.0.as_str())
                .collect::<Vec<_>>(),
            vec!["note.txt", "other.txt"]
        );

        let isolated = host_runtime(fixture.path());
        exact_file(&isolated, "other.txt");
        assert_eq!(proofs(&isolated).len(), 1);
        assert_eq!(proofs(&host).len(), 2);

        let other_workspace = tempfile::tempdir().unwrap();
        fs::write(other_workspace.path().join("note.txt"), b"different\n").unwrap();
        let distinct = host_runtime(other_workspace.path());
        exact_file(&distinct, "note.txt");
        assert_ne!(proofs(&host)[0].1, proofs(&distinct)[0].1);
    }

    #[test]
    fn proof_shape_debug_and_cli_keep_the_closed_private_boundary() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<WorkspaceRuntime>();

        let fixture = tempfile::tempdir().unwrap();
        fs::write(fixture.path().join("secret-path.txt"), b"secret bytes\n").unwrap();
        let host = host_runtime(fixture.path());
        exact_file(&host, "secret-path.txt");
        let current = proofs(&host);
        let debug = format!("{host:?}");
        assert!(!debug.contains("secret-path.txt"));
        assert!(!debug.contains(&current[0].1));

        let runtime_source = include_str!("../runtime.rs");
        let proof_shape = runtime_source
            .split("struct CurrentProof")
            .nth(1)
            .unwrap()
            .split("impl CurrentProof")
            .next()
            .unwrap();
        for forbidden in [
            "Vec<u8>",
            "Anddress",
            "SearchOutcome",
            "byte_start",
            "byte_end",
            "previous",
            "history",
        ] {
            assert!(!proof_shape.contains(forbidden), "retained {forbidden}");
        }
        let cli = include_str!("../bin/bw.rs");
        assert!(cli.contains("WorkspaceRuntime::open(&workspace, admission)"));
        assert!(!cli.contains("open_host_authoritative"));

        let apply = include_str!("apply.rs");
        let production_apply = apply.split("#[cfg(test)]").next().unwrap();
        let execute = production_apply
            .split("pub(super) fn execute")
            .nth(1)
            .unwrap()
            .split("fn map_edit_error")
            .next()
            .unwrap();
        let validation = execute.find("edit.validate()").unwrap();
        let selection = execute.find("select_current_proof").unwrap();
        assert!(validation < selection);
        assert!(!execute[..validation].contains("invalidate_current_proof"));
        assert!(execute.contains("prepare_current_proof_installation"));
        assert!(!production_apply.contains("install_search_proofs"));
    }
}
