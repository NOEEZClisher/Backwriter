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

    pub(crate) fn push(&mut self, byte: u8) {
        if let Some(length) = self.full_line_length {
            self.full_line_length = (length != self.literal.query.len()).then_some(length + 1);
        }
        self.push_content_byte(byte);
    }

    pub(crate) fn push_content_byte(&mut self, byte: u8) {
        while self.matched > 0 && byte != self.literal.query[self.matched] {
            self.matched = self.literal.failure[self.matched - 1];
        }
        if byte == self.literal.query[self.matched] {
            self.matched += 1;
            if self.matched == self.literal.query.len() {
                self.found = true;
                self.matched = self.literal.failure[self.matched - 1];
            }
        }
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
        MatchTier, PreparedLiteral, SearchInputError, SearchQuery, SearchScope, SearchScopeEntry,
    };
    #[test]
    fn literal_and_scope_validation_is_unbounded_and_canonical() {
        assert!(SearchQuery::new("needle").is_ok());
        assert_eq!(SearchQuery::new(""), Err(SearchInputError::InvalidQuery));
        assert!(SearchScope::only([SearchScopeEntry::subtree("docs").unwrap()]).is_ok());
        assert!(SearchScope::only([]).is_err());
    }

    #[test]
    fn full_line_eligibility_stops_at_the_query_length() {
        let query = SearchQuery::new("needle").unwrap();
        let literal = PreparedLiteral::new(&query).unwrap();
        let mut matcher = literal.matcher();
        for byte in b"needle longer" {
            matcher.push(*byte);
        }
        assert_eq!(matcher.finish(), Some(MatchTier::Substring));
    }
}
