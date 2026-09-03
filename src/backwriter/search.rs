//! Validated Search input and incremental literal matching.

use thiserror::Error;

use crate::backwriter::anddress::Anddress;
use crate::source::validate_logical_path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchQuery(String);
impl SearchQuery {
    pub fn new(value: impl AsRef<str>) -> Result<Self, SearchInputError> {
        let value = value.as_ref();
        if value.is_empty() || value.contains(['\0', '\r', '\n']) {
            return Err(SearchInputError::InvalidQuery);
        }
        Ok(Self(value.to_owned()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchScopeEntry {
    path: String,
    kind: ScopeEntryKind,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScopeEntryKind {
    Subtree,
    Source,
}
impl SearchScopeEntry {
    pub fn subtree(path: impl AsRef<str>) -> Result<Self, SearchInputError> {
        Self::new(path.as_ref(), ScopeEntryKind::Subtree)
    }
    pub fn source(path: impl AsRef<str>) -> Result<Self, SearchInputError> {
        Self::new(path.as_ref(), ScopeEntryKind::Source)
    }
    fn new(path: &str, kind: ScopeEntryKind) -> Result<Self, SearchInputError> {
        if path == "." || validate_logical_path(path).is_err() {
            return Err(SearchInputError::InvalidScope);
        }
        Ok(Self {
            path: path.to_owned(),
            kind,
        })
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub(crate) fn is_subtree(&self) -> bool {
        self.kind == ScopeEntryKind::Subtree
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchScope(ScopeKind);
#[derive(Clone, Debug, Eq, PartialEq)]
enum ScopeKind {
    AllAdmitted,
    Only(Vec<SearchScopeEntry>),
}
impl SearchScope {
    pub fn all_admitted() -> Self {
        Self(ScopeKind::AllAdmitted)
    }
    pub fn only(
        entries: impl IntoIterator<Item = SearchScopeEntry>,
    ) -> Result<Self, SearchInputError> {
        let mut collected: Vec<_> = entries.into_iter().collect();
        if collected.is_empty() {
            return Err(SearchInputError::InvalidScope);
        }
        collected.sort_unstable_by(|left, right| left.path.as_bytes().cmp(right.path.as_bytes()));
        if collected
            .windows(2)
            .any(|pair| pair[0].path.as_bytes() == pair[1].path.as_bytes())
        {
            return Err(SearchInputError::InvalidScope);
        }
        for entry in &collected {
            for (index, _) in entry.path.match_indices('/') {
                if collected
                    .binary_search_by(|candidate| {
                        candidate
                            .path
                            .as_bytes()
                            .cmp(&entry.path.as_bytes()[..index])
                    })
                    .is_ok()
                {
                    return Err(SearchInputError::InvalidScope);
                }
            }
        }
        Ok(Self(ScopeKind::Only(collected)))
    }
    pub(crate) fn entries(&self) -> Option<&[SearchScopeEntry]> {
        match &self.0 {
            ScopeKind::AllAdmitted => None,
            ScopeKind::Only(entries) => Some(entries),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SearchTarget {
    Line,
    Paragraph,
    File,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchRequest {
    kind: SearchRequestKind,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum SearchRequestKind {
    Content {
        query: SearchQuery,
        scope: SearchScope,
        target: SearchTarget,
    },
    ExactFile {
        logical_path: String,
    },
}
impl SearchRequest {
    pub fn new(query: SearchQuery, scope: SearchScope, target: SearchTarget) -> Self {
        Self {
            kind: SearchRequestKind::Content {
                query,
                scope,
                target,
            },
        }
    }

    pub fn exact_file(logical_path: impl AsRef<str>) -> Result<Self, SearchInputError> {
        let logical_path = logical_path.as_ref();
        if validate_logical_path(logical_path).is_err() {
            return Err(SearchInputError::InvalidFile);
        }
        Ok(Self {
            kind: SearchRequestKind::ExactFile {
                logical_path: logical_path.to_owned(),
            },
        })
    }

    pub(crate) fn kind(&self) -> &SearchRequestKind {
        &self.kind
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SearchOutcome {
    Empty,
    Found { anddresses: Vec<Anddress> },
}
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum SearchInputError {
    #[error("search query is invalid")]
    InvalidQuery,
    #[error("search scope is invalid")]
    InvalidScope,
    #[error("search file path is invalid")]
    InvalidFile,
}
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum SearchError {
    #[error("search scope is not admitted")]
    InvalidScope,
    #[error("workspace source is unavailable")]
    Unavailable,
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum MatchTier {
    FullLine,
    Substring,
}

pub(crate) struct PreparedLiteral<'a> {
    query: &'a [u8],
    failure: Vec<usize>,
}
impl<'a> PreparedLiteral<'a> {
    pub(crate) fn new(query: &'a SearchQuery) -> Result<Self, SearchError> {
        let query = query.0.as_bytes();
        let mut failure = Vec::new();
        failure
            .try_reserve_exact(query.len())
            .map_err(|_| SearchError::Unavailable)?;
        failure.resize(query.len(), 0);
        let mut matched = 0;
        for index in 1..query.len() {
            while matched > 0 && query[index] != query[matched] {
                matched = failure[matched - 1];
            }
            if query[index] == query[matched] {
                matched += 1;
            }
            failure[index] = matched;
        }
        Ok(Self { query, failure })
    }
    pub(crate) fn matcher(&self) -> LiteralMatcher<'_> {
        LiteralMatcher {
            literal: self,
            matched: 0,
            full_line_length: Some(0),
            found: false,
        }
    }
}

pub(crate) struct LiteralMatcher<'a> {
    literal: &'a PreparedLiteral<'a>,
    matched: usize,
    full_line_length: Option<usize>,
    found: bool,
}

impl LiteralMatcher<'_> {
    pub(crate) fn reset(&mut self) {
        self.matched = 0;
        self.full_line_length = Some(0);
        self.found = false;
    }

    pub(crate) fn push_segment(&mut self, bytes: &[u8]) -> Result<(), SearchError> {
        if bytes.is_empty() {
            return Ok(());
        }
        if let Some(length) = self.full_line_length {
            let length = length
                .checked_add(bytes.len())
                .ok_or(SearchError::Unavailable)?;
            self.full_line_length = (length <= self.literal.query.len()).then_some(length);
        }
        if self.found {
            return Ok(());
        }

        let mut cursor = 0;
        while cursor < bytes.len() {
            if self.matched == 0 {
                let Some(next) = bytes[cursor..]
                    .iter()
                    .position(|byte| *byte == self.literal.query[0])
                else {
                    return Ok(());
                };
                cursor += next;
            }

            let byte = bytes[cursor];
            cursor += 1;
            while self.matched > 0 && byte != self.literal.query[self.matched] {
                self.matched = self.literal.failure[self.matched - 1];
            }
            if byte == self.literal.query[self.matched] {
                self.matched += 1;
                if self.matched == self.literal.query.len() {
                    self.found = true;
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    pub(crate) fn finish(&self) -> Option<MatchTier> {
        self.found
            .then_some(if self.full_line_length == Some(self.literal.query.len()) {
                MatchTier::FullLine
            } else {
                MatchTier::Substring
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        MatchTier, PreparedLiteral, SearchError, SearchInputError, SearchQuery, SearchScope,
        SearchScopeEntry,
    };

    fn finish_with_splits(
        literal: &PreparedLiteral<'_>,
        content: &[u8],
        split_mask: usize,
    ) -> Result<Option<MatchTier>, SearchError> {
        let mut matcher = literal.matcher();
        if content.is_empty() {
            matcher.push_segment(content)?;
        } else {
            let mut start = 0;
            for end in 1..=content.len() {
                if end == content.len() || split_mask & (1 << (end - 1)) != 0 {
                    matcher.push_segment(&content[start..end])?;
                    start = end;
                }
            }
        }
        Ok(matcher.finish())
    }

    #[test]
    fn literal_and_scope_validation_is_unbounded_and_canonical() {
        assert!(SearchQuery::new("needle").is_ok());
        assert_eq!(SearchQuery::new(""), Err(SearchInputError::InvalidQuery));
        assert!(SearchScope::only([SearchScopeEntry::subtree("docs").unwrap()]).is_ok());
        assert!(SearchScope::only([]).is_err());
    }

    #[test]
    fn segment_matching_matches_every_byte_partition_and_fails_closed_on_overflow() {
        for query_length in 1..=4 {
            for query_bits in 0..1 << query_length {
                let query = (0..query_length)
                    .map(|index| {
                        if query_bits & (1 << index) == 0 {
                            'a'
                        } else {
                            'b'
                        }
                    })
                    .collect::<String>();
                let query = SearchQuery::new(query).unwrap();
                let literal = PreparedLiteral::new(&query).unwrap();
                for content_length in 0..=6 {
                    for content_bits in 0..1 << content_length {
                        let content = (0..content_length)
                            .map(|index| {
                                if content_bits & (1 << index) == 0 {
                                    b'a'
                                } else {
                                    b'b'
                                }
                            })
                            .collect::<Vec<_>>();
                        let byte_at_a_time = if content.is_empty() {
                            0
                        } else {
                            (1 << (content.len() - 1)) - 1
                        };
                        let expected = finish_with_splits(&literal, &content, byte_at_a_time);
                        assert_eq!(finish_with_splits(&literal, &content, 0), expected);
                        for split_mask in 0..1 << content.len().saturating_sub(1) {
                            assert_eq!(
                                finish_with_splits(&literal, &content, split_mask),
                                expected
                            );
                        }
                    }
                }
            }
        }

        for (content, query, expected) in [
            ("zzzz", "needle", None),
            ("zzzn", "n", Some(MatchTier::Substring)),
            ("x", "x", Some(MatchTier::FullLine)),
            ("abab", "abab", Some(MatchTier::FullLine)),
            ("ababa", "abab", Some(MatchTier::Substring)),
            ("prefix needle", "needle", Some(MatchTier::Substring)),
            ("needle suffix", "needle", Some(MatchTier::Substring)),
        ] {
            let query = SearchQuery::new(query).unwrap();
            let literal = PreparedLiteral::new(&query).unwrap();
            for split in 0..=content.len() {
                let mut matcher = literal.matcher();
                matcher.push_segment(&content.as_bytes()[..split]).unwrap();
                matcher.push_segment(&content.as_bytes()[split..]).unwrap();
                assert_eq!(matcher.finish(), expected);
            }
        }

        let content = "prefix é🦀 needle suffix";
        let query = SearchQuery::new("é🦀").unwrap();
        let literal = PreparedLiteral::new(&query).unwrap();
        for split in 0..=content.len() {
            let mut matcher = literal.matcher();
            matcher.push_segment(&content.as_bytes()[..split]).unwrap();
            matcher.push_segment(&content.as_bytes()[split..]).unwrap();
            assert_eq!(matcher.finish(), Some(MatchTier::Substring));
        }

        let query = SearchQuery::new("needle").unwrap();
        let literal = PreparedLiteral::new(&query).unwrap();
        let mut matcher = literal.matcher();
        matcher.push_segment(b"nee").unwrap();
        matcher.push_segment(b"dle").unwrap();
        matcher.push_segment(&vec![b'x'; 65_536]).unwrap();
        assert_eq!(matcher.finish(), Some(MatchTier::Substring));

        let mut overflow = literal.matcher();
        overflow.full_line_length = Some(usize::MAX);
        assert_eq!(overflow.push_segment(b"x"), Err(SearchError::Unavailable));
        assert_eq!(overflow.finish(), None);
    }
}
