//! Caller-owned typed RAM storage for native Backwriter values.

use std::error::Error as StdError;
use std::fmt;

use thiserror::Error;

use crate::backwriter::anddress::Anddress;
use crate::backwriter::check::CheckOutcome;
use crate::backwriter::pick::PickOutcome;
use crate::backwriter::search::SearchOutcome;
use crate::backwriter::view::ViewOutcome;

#[derive(Debug, Eq, PartialEq)]
pub struct DataName(String);

impl DataName {
    pub fn new(value: String) -> Result<Self, DataNameError> {
        if value.is_empty() {
            return Err(DataNameError::Empty);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum DataNameError {
    #[error("Data name is empty")]
    Empty,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataKind {
    Anddress,
    Search,
    Pick,
    View,
    CheckAnddress,
    CheckSearch,
    CheckPick,
}

pub struct DataStore {
    anddresses: Vec<Entry<Anddress>>,
    searches: Vec<Entry<SearchOutcome>>,
    picks: Vec<Entry<PickOutcome>>,
    views: Vec<Entry<ViewOutcome>>,
    check_anddresses: Vec<Entry<CheckOutcome<Option<Anddress>>>>,
    check_searches: Vec<Entry<CheckOutcome<SearchOutcome>>>,
    check_picks: Vec<Entry<CheckOutcome<PickOutcome>>>,
}

pub enum StoreError<T> {
    AlreadyExists { value: T },
    Resource { value: T },
}

impl<T> fmt::Debug for StoreError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists { .. } => formatter.write_str("AlreadyExists { .. }"),
            Self::Resource { .. } => formatter.write_str("Resource { .. }"),
        }
    }
}

impl<T> PartialEq for StoreError<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::AlreadyExists { value: left }, Self::AlreadyExists { value: right })
            | (Self::Resource { value: left }, Self::Resource { value: right }) => left == right,
            _ => false,
        }
    }
}

impl<T> Eq for StoreError<T> where T: Eq {}

impl<T> fmt::Display for StoreError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists { .. } => formatter.write_str("Data entry already exists"),
            Self::Resource { .. } => formatter.write_str("Data resource allocation failed"),
        }
    }
}

impl<T: fmt::Debug> StdError for StoreError<T> {}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum DataError {
    #[error("Data entry was not found")]
    NotFound,
    #[error("Data entry already exists")]
    AlreadyExists,
    #[error("Data resource allocation failed")]
    Resource,
}

struct Entry<T> {
    name: DataName,
    value: T,
}

#[allow(clippy::new_without_default, clippy::result_large_err)]
impl DataStore {
    pub fn new() -> Self {
        Self {
            anddresses: Vec::new(),
            searches: Vec::new(),
            picks: Vec::new(),
            views: Vec::new(),
            check_anddresses: Vec::new(),
            check_searches: Vec::new(),
            check_picks: Vec::new(),
        }
    }

    pub fn store_anddress(
        &mut self,
        name: &DataName,
        value: Anddress,
    ) -> Result<(), StoreError<Anddress>> {
        store(&mut self.anddresses, name, value)
    }

    pub fn get_anddress(&self, name: &DataName) -> Option<&Anddress> {
        get(&self.anddresses, name)
    }

    pub fn store_search(
        &mut self,
        name: &DataName,
        value: SearchOutcome,
    ) -> Result<(), StoreError<SearchOutcome>> {
        store(&mut self.searches, name, value)
    }

    pub fn get_search(&self, name: &DataName) -> Option<&SearchOutcome> {
        get(&self.searches, name)
    }

    pub fn store_pick(
        &mut self,
        name: &DataName,
        value: PickOutcome,
    ) -> Result<(), StoreError<PickOutcome>> {
        store(&mut self.picks, name, value)
    }

    pub fn get_pick(&self, name: &DataName) -> Option<&PickOutcome> {
        get(&self.picks, name)
    }

    pub fn store_view(
        &mut self,
        name: &DataName,
        value: ViewOutcome,
    ) -> Result<(), StoreError<ViewOutcome>> {
        store(&mut self.views, name, value)
    }

    pub fn get_view(&self, name: &DataName) -> Option<&ViewOutcome> {
        get(&self.views, name)
    }

    pub fn store_check_anddress(
        &mut self,
        name: &DataName,
        value: CheckOutcome<Option<Anddress>>,
    ) -> Result<(), StoreError<CheckOutcome<Option<Anddress>>>> {
        store(&mut self.check_anddresses, name, value)
    }

    pub fn get_check_anddress(&self, name: &DataName) -> Option<&CheckOutcome<Option<Anddress>>> {
        get(&self.check_anddresses, name)
    }

    pub fn store_check_search(
        &mut self,
        name: &DataName,
        value: CheckOutcome<SearchOutcome>,
    ) -> Result<(), StoreError<CheckOutcome<SearchOutcome>>> {
        store(&mut self.check_searches, name, value)
    }

    pub fn get_check_search(&self, name: &DataName) -> Option<&CheckOutcome<SearchOutcome>> {
        get(&self.check_searches, name)
    }

    pub fn store_check_pick(
        &mut self,
        name: &DataName,
        value: CheckOutcome<PickOutcome>,
    ) -> Result<(), StoreError<CheckOutcome<PickOutcome>>> {
        store(&mut self.check_picks, name, value)
    }

    pub fn get_check_pick(&self, name: &DataName) -> Option<&CheckOutcome<PickOutcome>> {
        get(&self.check_picks, name)
    }

    pub fn list(&self) -> impl Iterator<Item = (DataKind, &DataName)> + '_ {
        self.anddresses
            .iter()
            .map(|entry| (DataKind::Anddress, &entry.name))
            .chain(
                self.searches
                    .iter()
                    .map(|entry| (DataKind::Search, &entry.name)),
            )
            .chain(self.picks.iter().map(|entry| (DataKind::Pick, &entry.name)))
            .chain(self.views.iter().map(|entry| (DataKind::View, &entry.name)))
            .chain(
                self.check_anddresses
                    .iter()
                    .map(|entry| (DataKind::CheckAnddress, &entry.name)),
            )
            .chain(
                self.check_searches
                    .iter()
                    .map(|entry| (DataKind::CheckSearch, &entry.name)),
            )
            .chain(
                self.check_picks
                    .iter()
                    .map(|entry| (DataKind::CheckPick, &entry.name)),
            )
    }

    pub fn rename(
        &mut self,
        kind: DataKind,
        old: &DataName,
        new: &DataName,
    ) -> Result<(), DataError> {
        match kind {
            DataKind::Anddress => rename(&mut self.anddresses, old, new),
            DataKind::Search => rename(&mut self.searches, old, new),
            DataKind::Pick => rename(&mut self.picks, old, new),
            DataKind::View => rename(&mut self.views, old, new),
            DataKind::CheckAnddress => rename(&mut self.check_anddresses, old, new),
            DataKind::CheckSearch => rename(&mut self.check_searches, old, new),
            DataKind::CheckPick => rename(&mut self.check_picks, old, new),
        }
    }

    pub fn remove(&mut self, kind: DataKind, name: &DataName) -> Result<(), DataError> {
        match kind {
            DataKind::Anddress => remove(&mut self.anddresses, name),
            DataKind::Search => remove(&mut self.searches, name),
            DataKind::Pick => remove(&mut self.picks, name),
            DataKind::View => remove(&mut self.views, name),
            DataKind::CheckAnddress => remove(&mut self.check_anddresses, name),
            DataKind::CheckSearch => remove(&mut self.check_searches, name),
            DataKind::CheckPick => remove(&mut self.check_picks, name),
        }
    }
}

fn copy_name(name: &DataName) -> Result<DataName, ()> {
    let mut value = String::new();
    value.try_reserve_exact(name.0.len()).map_err(|_| ())?;
    value.push_str(&name.0);
    Ok(DataName(value))
}

fn store<T>(entries: &mut Vec<Entry<T>>, name: &DataName, value: T) -> Result<(), StoreError<T>> {
    if entries.iter().any(|entry| entry.name == *name) {
        return Err(StoreError::AlreadyExists { value });
    }
    let name = match copy_name(name) {
        Ok(name) => name,
        Err(()) => return Err(StoreError::Resource { value }),
    };
    if entries.try_reserve(1).is_err() {
        return Err(StoreError::Resource { value });
    }
    entries.push(Entry { name, value });
    Ok(())
}

fn get<'a, T>(entries: &'a [Entry<T>], name: &DataName) -> Option<&'a T> {
    entries
        .iter()
        .find(|entry| entry.name == *name)
        .map(|entry| &entry.value)
}

fn rename<T>(entries: &mut [Entry<T>], old: &DataName, new: &DataName) -> Result<(), DataError> {
    let index = entries
        .iter()
        .position(|entry| entry.name == *old)
        .ok_or(DataError::NotFound)?;
    if entries.iter().any(|entry| entry.name == *new) {
        return Err(DataError::AlreadyExists);
    }
    let name = copy_name(new).map_err(|_| DataError::Resource)?;
    entries[index].name = name;
    Ok(())
}

fn remove<T>(entries: &mut Vec<Entry<T>>, name: &DataName) -> Result<(), DataError> {
    let index = entries
        .iter()
        .position(|entry| entry.name == *name)
        .ok_or(DataError::NotFound)?;
    let Entry { value, .. } = entries.remove(index);
    drop(value);
    Ok(())
}
