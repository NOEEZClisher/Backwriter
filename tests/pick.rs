use artext::backwriter::anddress::{ANDDRESS_VERSION, Anddress, AnddressTarget, Natural};
use artext::backwriter::pick::{PickOutcome, PickPredicate, PickTargetKind, pick};

fn address(path: &str, target: AnddressTarget) -> Anddress {
    address_at("a".repeat(64), path, target)
}

fn address_at(coordinate: String, path: &str, target: AnddressTarget) -> Anddress {
    Anddress {
        version: ANDDRESS_VERSION.to_owned(),
        workspace_coordinate: coordinate,
        logical_path: path.to_owned(),
        target,
    }
}

#[test]
fn pick_preserves_input_order_and_multiplicity() {
    let file = address("a.txt", AnddressTarget::File);
    let line = address(
        "a.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "x\n".to_owned(),
        },
    );
    let other = address("b.txt", AnddressTarget::File);
    assert_eq!(
        pick(
            vec![file.clone(), line.clone(), file.clone(), other],
            &PickPredicate::same_file(line.clone())
        )
        .unwrap(),
        PickOutcome::Selected {
            anddresses: vec![file.clone(), line, file]
        }
    );
}

#[test]
fn pick_boolean_vm_and_full_value_one_of_remain_pure() {
    let file = address("a.txt", AnddressTarget::File);
    let paragraph = address(
        "a.txt",
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let line = address(
        "a.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "x".to_owned(),
        },
    );
    let only_lines = PickPredicate::target_kind(PickTargetKind::Line);
    assert_eq!(
        pick(vec![file, paragraph, line.clone()], &only_lines).unwrap(),
        PickOutcome::Selected {
            anddresses: vec![line.clone()]
        }
    );
    assert_eq!(
        pick(vec![line.clone()], &PickPredicate::one_of(vec![line])).unwrap(),
        PickOutcome::Selected {
            anddresses: vec![address(
                "a.txt",
                AnddressTarget::Line {
                    ordinal: Natural::zero(),
                    exact_extent: "x".to_owned()
                }
            )]
        }
    );
}

#[test]
fn pick_all_empty_and_target_kinds_preserve_only_matching_candidates() {
    let file = address("a.txt", AnddressTarget::File);
    let paragraph = address(
        "a.txt",
        AnddressTarget::Paragraph {
            ordinal: Natural::zero(),
        },
    );
    let line = address(
        "a.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "x".to_owned(),
        },
    );
    assert_eq!(
        pick(vec![], &PickPredicate::all()).unwrap(),
        PickOutcome::Empty
    );
    assert_eq!(
        pick(
            vec![file.clone(), paragraph.clone(), line.clone()],
            &PickPredicate::all()
        )
        .unwrap(),
        PickOutcome::Selected {
            anddresses: vec![file.clone(), paragraph.clone(), line.clone()]
        }
    );
    for (kind, expected) in [
        (PickTargetKind::File, file),
        (PickTargetKind::Paragraph, paragraph),
        (PickTargetKind::Line, line),
    ] {
        assert_eq!(
            pick(
                vec![
                    address("a.txt", AnddressTarget::File),
                    address(
                        "a.txt",
                        AnddressTarget::Paragraph {
                            ordinal: Natural::zero()
                        },
                    ),
                    address(
                        "a.txt",
                        AnddressTarget::Line {
                            ordinal: Natural::zero(),
                            exact_extent: "x".to_owned(),
                        },
                    ),
                ],
                &PickPredicate::target_kind(kind),
            )
            .unwrap(),
            PickOutcome::Selected {
                anddresses: vec![expected]
            }
        );
    }
}

#[test]
fn pick_one_of_requires_the_complete_v3_value() {
    let reference = address(
        "a.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "x\n".to_owned(),
        },
    );
    let different_kind = address("a.txt", AnddressTarget::File);
    let different_ordinal = address(
        "a.txt",
        AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "x\n".to_owned(),
        },
    );
    let different_extent = address(
        "a.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "x\r".to_owned(),
        },
    );
    let different_coordinate = address_at(
        "b".repeat(64),
        "a.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "x\n".to_owned(),
        },
    );
    let different_path = address(
        "b.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "x\n".to_owned(),
        },
    );
    let different_version = Anddress {
        version: "other".to_owned(),
        workspace_coordinate: "a".repeat(64),
        logical_path: "a.txt".to_owned(),
        target: AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "x\n".to_owned(),
        },
    };
    assert_eq!(
        pick(
            vec![
                different_kind,
                different_ordinal,
                different_extent,
                different_coordinate,
                different_path,
                different_version,
                reference.clone(),
            ],
            &PickPredicate::one_of(vec![reference.clone()]),
        )
        .unwrap(),
        PickOutcome::Selected {
            anddresses: vec![reference]
        }
    );
}

#[test]
fn pick_boolean_composition_does_not_change_same_file_definition() {
    let file = address("a.txt", AnddressTarget::File);
    let line = address(
        "a.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "x".to_owned(),
        },
    );
    let elsewhere = address(
        "b.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "x".to_owned(),
        },
    );
    let selected = PickPredicate::all_of(
        PickPredicate::same_file(file.clone()),
        vec![PickPredicate::negate(PickPredicate::target_kind(PickTargetKind::File)).unwrap()],
    )
    .unwrap();
    assert_eq!(
        pick(vec![file, line.clone(), elsewhere], &selected).unwrap(),
        PickOutcome::Selected {
            anddresses: vec![line]
        }
    );
}

#[test]
fn pick_same_file_requires_coordinate_as_well_as_logical_path() {
    let reference = address_at("a".repeat(64), "same.txt", AnddressTarget::File);
    let same_path_other_coordinate = address_at(
        "b".repeat(64),
        "same.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "x".to_owned(),
        },
    );
    assert_eq!(
        pick(
            vec![reference.clone(), same_path_other_coordinate],
            &PickPredicate::same_file(reference.clone()),
        )
        .unwrap(),
        PickOutcome::Selected {
            anddresses: vec![reference]
        }
    );
}

#[test]
fn pick_any_of_and_deep_boolean_programs_remain_iterative() {
    let file = address("a.txt", AnddressTarget::File);
    let line = address(
        "a.txt",
        AnddressTarget::Line {
            ordinal: Natural::zero(),
            exact_extent: "x".to_owned(),
        },
    );
    let any = PickPredicate::any_of(
        PickPredicate::target_kind(PickTargetKind::Paragraph),
        vec![PickPredicate::one_of(vec![line.clone()])],
    )
    .unwrap();
    assert_eq!(
        pick(vec![file, line.clone()], &any).unwrap(),
        PickOutcome::Selected {
            anddresses: vec![line.clone()]
        }
    );
    let mut deep = PickPredicate::all();
    for _ in 0..4097 {
        deep = PickPredicate::negate(deep).unwrap();
    }
    assert_eq!(
        pick(vec![line.clone(), line.clone()], &deep).unwrap(),
        PickOutcome::Empty
    );
}
