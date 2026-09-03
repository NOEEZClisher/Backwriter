use std::error::Error;
use std::fs;

use backwriter::backwriter::anddress::Anddress;
use backwriter::backwriter::check::CheckOutcome;
use backwriter::backwriter::data::{
    DataError, DataKind, DataName, DataNameError, DataStore, StoreError,
};
use backwriter::backwriter::pick::PickOutcome;
use backwriter::backwriter::search::{
    SearchOutcome, SearchQuery, SearchRequest, SearchScope, SearchTarget,
};
use backwriter::backwriter::view::ViewOutcome;
use backwriter::runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime};
use tempfile::tempdir;

mod support;

fn name(value: &str) -> DataName {
    DataName::new(value.to_owned()).unwrap()
}

fn address(path: &str) -> Anddress {
    support::file(&"a".repeat(64), path, b"")
}

fn occurrence(value: Anddress) -> Anddress {
    value
}

fn check_outcomes() -> (
    CheckOutcome<Option<Anddress>>,
    CheckOutcome<SearchOutcome>,
    CheckOutcome<PickOutcome>,
) {
    let fixture = tempdir().unwrap();
    let root = fixture.path().join("workspace");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("seed.txt"), "seed\n").unwrap();
    let workspace = WorkspaceRuntime::open(
        &root,
        WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
    )
    .unwrap();
    let request = SearchRequest::new(
        SearchQuery::new("seed").unwrap(),
        SearchScope::all_admitted(),
        SearchTarget::File,
    );
    let SearchOutcome::Found { mut anddresses } = workspace.search(&request).unwrap() else {
        panic!("fixture supplies one source");
    };
    let current = anddresses.remove(0);
    let raw = workspace.check(current.clone()).unwrap();
    let search = workspace
        .check_search(SearchOutcome::Found {
            anddresses: vec![current.clone()],
        })
        .unwrap();
    let pick = workspace
        .check_pick(PickOutcome::Selected {
            anddresses: vec![current],
        })
        .unwrap();
    (raw, search, pick)
}

#[test]
fn data_names_accept_native_utf8_without_normalization() {
    assert_eq!(DataName::new(String::new()), Err(DataNameError::Empty));

    let unicode = name("한글");
    let whitespace = name(" \t ");
    let nul = name("nul\0name");
    assert_eq!(unicode.as_str(), "한글");
    assert_eq!(whitespace.as_str(), " \t ");
    assert_eq!(nul.as_str(), "nul\0name");

    let upper = name("Name");
    let lower = name("name");
    let composed = name("é");
    let decomposed = name("e\u{301}");
    assert_ne!(upper, lower);
    assert_ne!(composed, decomposed);
}

#[test]
fn typed_store_get_pairs_accept_every_native_payload() {
    let mut store = DataStore::new();
    let anddress_name = name("anddress");
    let search_name = name("search");
    let pick_name = name("pick");
    let view_name = name("view");
    let check_anddress_name = name("check-anddress");
    let check_search_name = name("check-search");
    let check_pick_name = name("check-pick");
    let address = address("a.txt");
    let search = SearchOutcome::Found {
        anddresses: vec![occurrence(address.clone())],
    };
    let pick = PickOutcome::Selected {
        anddresses: vec![address.clone()],
    };
    let view = ViewOutcome::File {
        anddress: address.clone(),
        text: "snapshot".to_owned(),
    };
    let (check_anddress, check_search, check_pick) = check_outcomes();

    store
        .store_anddress(&anddress_name, address.clone())
        .unwrap();
    store.store_search(&search_name, search.clone()).unwrap();
    store.store_pick(&pick_name, pick.clone()).unwrap();
    store.store_view(&view_name, view.clone()).unwrap();
    store
        .store_check_anddress(&check_anddress_name, check_anddress.clone())
        .unwrap();
    store
        .store_check_search(&check_search_name, check_search.clone())
        .unwrap();
    store
        .store_check_pick(&check_pick_name, check_pick.clone())
        .unwrap();

    assert_eq!(store.get_anddress(&anddress_name), Some(&address));
    assert_eq!(store.get_search(&search_name), Some(&search));
    assert_eq!(store.get_pick(&pick_name), Some(&pick));
    assert_eq!(store.get_view(&view_name), Some(&view));
    assert_eq!(
        store.get_check_anddress(&check_anddress_name),
        Some(&check_anddress)
    );
    assert_eq!(
        store.get_check_search(&check_search_name),
        Some(&check_search)
    );
    assert_eq!(store.get_check_pick(&check_pick_name), Some(&check_pick));
    assert_eq!(store.get_anddress(&name("missing")), None);
}

#[test]
fn store_preserves_kinds_and_returns_the_exact_duplicate_input() {
    let mut store = DataStore::new();
    let shared = name("shared");
    let first = address("first.txt");
    let duplicate = address("duplicate.txt");
    let search = SearchOutcome::Found {
        anddresses: vec![occurrence(address("search.txt"))],
    };

    store.store_anddress(&shared, first.clone()).unwrap();
    store.store_search(&shared, search.clone()).unwrap();
    assert_eq!(
        store.store_anddress(&shared, duplicate.clone()),
        Err(StoreError::AlreadyExists {
            value: duplicate.clone(),
        })
    );
    assert_eq!(store.get_anddress(&shared), Some(&first));
    assert_eq!(store.get_search(&shared), Some(&search));
}

#[test]
fn list_yields_only_borrowed_kind_name_pairs_without_order_authority() {
    let mut store = DataStore::new();
    let one = name("one");
    let two = name("two");
    let three = name("three");
    let four = name("four");
    let five = name("five");
    let six = name("six");
    let seven = name("seven");
    let (check_anddress, check_search, check_pick) = check_outcomes();

    store.store_anddress(&one, address("one.txt")).unwrap();
    store.store_search(&two, SearchOutcome::Empty).unwrap();
    store.store_pick(&three, PickOutcome::Empty).unwrap();
    store
        .store_view(
            &four,
            ViewOutcome::File {
                anddress: address("four.txt"),
                text: String::new(),
            },
        )
        .unwrap();
    store.store_check_anddress(&five, check_anddress).unwrap();
    store.store_check_search(&six, check_search).unwrap();
    store.store_check_pick(&seven, check_pick).unwrap();

    let entries: Vec<_> = store
        .list()
        .map(|(kind, entry_name)| (kind, entry_name.as_str()))
        .collect();
    assert_eq!(entries.len(), 7);
    assert!(entries.contains(&(DataKind::Anddress, "one")));
    assert!(entries.contains(&(DataKind::Search, "two")));
    assert!(entries.contains(&(DataKind::Pick, "three")));
    assert!(entries.contains(&(DataKind::View, "four")));
    assert!(entries.contains(&(DataKind::CheckAnddress, "five")));
    assert!(entries.contains(&(DataKind::CheckSearch, "six")));
    assert!(entries.contains(&(DataKind::CheckPick, "seven")));
}

#[test]
fn rename_has_documented_priority_and_preserves_the_payload_kind() {
    let mut store = DataStore::new();
    let missing = name("missing");
    let old = name("old");
    let destination = name("destination");
    let renamed = name("renamed");
    let payload = PickOutcome::Selected {
        anddresses: vec![address("payload.txt")],
    };

    store.store_pick(&old, payload.clone()).unwrap();
    store.store_pick(&destination, PickOutcome::Empty).unwrap();
    assert_eq!(
        store.rename(DataKind::Pick, &missing, &destination),
        Err(DataError::NotFound)
    );
    assert_eq!(
        store.rename(DataKind::Pick, &old, &destination),
        Err(DataError::AlreadyExists)
    );
    assert_eq!(
        store.rename(DataKind::Pick, &old, &old),
        Err(DataError::AlreadyExists)
    );
    assert_eq!(store.rename(DataKind::Pick, &old, &renamed), Ok(()));
    assert_eq!(store.get_pick(&old), None);
    assert_eq!(store.get_pick(&renamed), Some(&payload));
    assert_eq!(store.get_pick(&destination), Some(&PickOutcome::Empty));
}

#[test]
fn remove_drops_only_the_selected_binding() {
    let mut store = DataStore::new();
    let removed = name("removed");
    let retained = name("retained");
    let missing = name("missing");
    let retained_value = ViewOutcome::File {
        anddress: address("retained.txt"),
        text: "keep".to_owned(),
    };

    store
        .store_view(
            &removed,
            ViewOutcome::File {
                anddress: address("removed.txt"),
                text: "drop".to_owned(),
            },
        )
        .unwrap();
    store.store_view(&retained, retained_value.clone()).unwrap();
    assert_eq!(
        store.remove(DataKind::View, &missing),
        Err(DataError::NotFound)
    );
    assert_eq!(store.remove(DataKind::View, &removed), Ok(()));
    assert_eq!(store.get_view(&removed), None);
    assert_eq!(store.get_view(&retained), Some(&retained_value));
}

#[test]
fn rename_and_remove_dispatch_each_data_kind_without_cross_kind_aliasing() {
    let mut store = DataStore::new();
    let old = name("shared");
    let anddress_name = name("anddress-new");
    let search_name = name("search-new");
    let pick_name = name("pick-new");
    let view_name = name("view-new");
    let check_anddress_name = name("check-anddress-new");
    let check_search_name = name("check-search-new");
    let check_pick_name = name("check-pick-new");
    let anddress = address("anddress.txt");
    let search = SearchOutcome::Found {
        anddresses: vec![occurrence(address("search.txt"))],
    };
    let pick = PickOutcome::Selected {
        anddresses: vec![address("pick.txt")],
    };
    let view = ViewOutcome::File {
        anddress: address("view.txt"),
        text: "view".to_owned(),
    };
    let (check_anddress, check_search, check_pick) = check_outcomes();

    store.store_anddress(&old, anddress.clone()).unwrap();
    store.store_search(&old, search.clone()).unwrap();
    store.store_pick(&old, pick.clone()).unwrap();
    store.store_view(&old, view.clone()).unwrap();
    store
        .store_check_anddress(&old, check_anddress.clone())
        .unwrap();
    store
        .store_check_search(&old, check_search.clone())
        .unwrap();
    store.store_check_pick(&old, check_pick.clone()).unwrap();

    assert_eq!(
        store.rename(DataKind::Anddress, &old, &anddress_name),
        Ok(())
    );
    assert_eq!(store.get_anddress(&old), None);
    assert_eq!(store.get_anddress(&anddress_name), Some(&anddress));
    assert_eq!(store.get_search(&old), Some(&search));
    assert_eq!(store.get_pick(&old), Some(&pick));
    assert_eq!(store.get_view(&old), Some(&view));
    assert_eq!(store.get_check_anddress(&old), Some(&check_anddress));
    assert_eq!(store.get_check_search(&old), Some(&check_search));
    assert_eq!(store.get_check_pick(&old), Some(&check_pick));

    assert_eq!(store.rename(DataKind::Search, &old, &search_name), Ok(()));
    assert_eq!(store.get_search(&old), None);
    assert_eq!(store.get_search(&search_name), Some(&search));
    assert_eq!(store.get_pick(&old), Some(&pick));
    assert_eq!(store.get_view(&old), Some(&view));
    assert_eq!(store.get_check_anddress(&old), Some(&check_anddress));
    assert_eq!(store.get_check_search(&old), Some(&check_search));
    assert_eq!(store.get_check_pick(&old), Some(&check_pick));

    assert_eq!(store.rename(DataKind::Pick, &old, &pick_name), Ok(()));
    assert_eq!(store.get_pick(&old), None);
    assert_eq!(store.get_pick(&pick_name), Some(&pick));
    assert_eq!(store.get_view(&old), Some(&view));
    assert_eq!(store.get_check_anddress(&old), Some(&check_anddress));
    assert_eq!(store.get_check_search(&old), Some(&check_search));
    assert_eq!(store.get_check_pick(&old), Some(&check_pick));

    assert_eq!(store.rename(DataKind::View, &old, &view_name), Ok(()));
    assert_eq!(store.get_view(&old), None);
    assert_eq!(store.get_view(&view_name), Some(&view));
    assert_eq!(store.get_check_anddress(&old), Some(&check_anddress));
    assert_eq!(store.get_check_search(&old), Some(&check_search));
    assert_eq!(store.get_check_pick(&old), Some(&check_pick));

    assert_eq!(
        store.rename(DataKind::CheckAnddress, &old, &check_anddress_name),
        Ok(())
    );
    assert_eq!(store.get_check_anddress(&old), None);
    assert_eq!(
        store.get_check_anddress(&check_anddress_name),
        Some(&check_anddress)
    );
    assert_eq!(store.get_check_search(&old), Some(&check_search));
    assert_eq!(store.get_check_pick(&old), Some(&check_pick));

    assert_eq!(
        store.rename(DataKind::CheckSearch, &old, &check_search_name),
        Ok(())
    );
    assert_eq!(store.get_check_search(&old), None);
    assert_eq!(
        store.get_check_search(&check_search_name),
        Some(&check_search)
    );
    assert_eq!(store.get_check_pick(&old), Some(&check_pick));

    assert_eq!(
        store.rename(DataKind::CheckPick, &old, &check_pick_name),
        Ok(())
    );
    assert_eq!(store.get_check_pick(&old), None);
    assert_eq!(store.get_check_pick(&check_pick_name), Some(&check_pick));

    assert_eq!(store.remove(DataKind::Anddress, &anddress_name), Ok(()));
    assert_eq!(store.get_anddress(&anddress_name), None);
    assert_eq!(store.get_search(&search_name), Some(&search));
    assert_eq!(store.remove(DataKind::Search, &search_name), Ok(()));
    assert_eq!(store.get_search(&search_name), None);
    assert_eq!(store.get_pick(&pick_name), Some(&pick));
    assert_eq!(store.remove(DataKind::Pick, &pick_name), Ok(()));
    assert_eq!(store.get_pick(&pick_name), None);
    assert_eq!(store.get_view(&view_name), Some(&view));
    assert_eq!(store.remove(DataKind::View, &view_name), Ok(()));
    assert_eq!(store.get_view(&view_name), None);
    assert_eq!(
        store.get_check_anddress(&check_anddress_name),
        Some(&check_anddress)
    );
    assert_eq!(
        store.remove(DataKind::CheckAnddress, &check_anddress_name),
        Ok(())
    );
    assert_eq!(store.get_check_anddress(&check_anddress_name), None);
    assert_eq!(
        store.get_check_search(&check_search_name),
        Some(&check_search)
    );
    assert_eq!(
        store.remove(DataKind::CheckSearch, &check_search_name),
        Ok(())
    );
    assert_eq!(store.get_check_search(&check_search_name), None);
    assert_eq!(store.get_check_pick(&check_pick_name), Some(&check_pick));
    assert_eq!(store.remove(DataKind::CheckPick, &check_pick_name), Ok(()));
    assert_eq!(store.get_check_pick(&check_pick_name), None);
    assert_eq!(store.list().count(), 0);
}

#[test]
fn duplicate_view_store_returns_the_original_owned_allocation() {
    let mut store = DataStore::new();
    let entry_name = name("view");
    let stored = ViewOutcome::File {
        anddress: address("stored.txt"),
        text: "stored".to_owned(),
    };
    store.store_view(&entry_name, stored.clone()).unwrap();

    let mut text = String::with_capacity(64);
    text.push_str("duplicate view");
    let text_pointer = text.as_ptr();
    let text_capacity = text.capacity();
    assert!(text_capacity > text.len());

    match store.store_view(
        &entry_name,
        ViewOutcome::File {
            anddress: address("duplicate.txt"),
            text,
        },
    ) {
        Err(StoreError::AlreadyExists {
            value: ViewOutcome::File { text, .. },
        }) => {
            assert_eq!(text.as_ptr(), text_pointer);
            assert_eq!(text.capacity(), text_capacity);
            assert_eq!(text, "duplicate view");
        }
        result => panic!("unexpected duplicate Store result: {result:?}"),
    }

    assert_eq!(store.get_view(&entry_name), Some(&stored));
}

#[test]
fn data_names_have_no_fixed_byte_limit() {
    let mut store = DataStore::new();
    let long_name = DataName::new("n".repeat(4_098)).unwrap();
    let value = address("long-name.txt");

    store.store_anddress(&long_name, value.clone()).unwrap();
    assert_eq!(store.get_anddress(&long_name), Some(&value));
    assert!(store.list().any(|(kind, entry_name)| {
        kind == DataKind::Anddress && entry_name.as_str() == long_name.as_str()
    }));
}

#[test]
fn data_store_has_no_fixed_entry_cap() {
    let mut store = DataStore::new();
    let value = address("many.txt");

    for index in 0..=4097 {
        let entry_name = DataName::new(format!("entry-{index}")).unwrap();
        store.store_anddress(&entry_name, value.clone()).unwrap();
    }

    let last = name("entry-4097");
    assert_eq!(store.get_anddress(&last), Some(&value));
    assert_eq!(store.list().count(), 4098);
}

#[test]
fn public_trait_and_error_surface_matches_the_contract() {
    fn assert_name_traits<T: std::fmt::Debug + Eq + PartialEq>() {}
    fn assert_kind_traits<T: Clone + Copy + std::fmt::Debug + Eq + PartialEq>() {}
    fn assert_error_traits<T: Clone + Copy + std::fmt::Debug + Eq + PartialEq + Error>() {}
    fn assert_store_error_traits<T: std::fmt::Debug + Eq + PartialEq + Error>() {}

    assert_name_traits::<DataName>();
    assert_kind_traits::<DataKind>();
    assert_error_traits::<DataNameError>();
    assert_error_traits::<DataError>();
    assert_store_error_traits::<StoreError<Anddress>>();
    assert_store_error_traits::<StoreError<SearchOutcome>>();
    assert_store_error_traits::<StoreError<PickOutcome>>();
    assert_store_error_traits::<StoreError<ViewOutcome>>();
    assert_store_error_traits::<StoreError<CheckOutcome<Option<Anddress>>>>();
    assert_store_error_traits::<StoreError<CheckOutcome<SearchOutcome>>>();
    assert_store_error_traits::<StoreError<CheckOutcome<PickOutcome>>>();
}
