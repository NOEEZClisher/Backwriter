use std::{error::Error, fmt::Debug};

use backwriter::backwriter::{
    anddress::{Anddress, AnddressTarget},
    edit::{Edit, EditError, Position},
};

fn address(target: AnddressTarget) -> Anddress {
    Anddress::new(
        &"a".repeat(64),
        "note.txt",
        &"b".repeat(64),
        4,
        target,
        0,
        4,
    )
    .unwrap()
}

fn file() -> Anddress {
    address(AnddressTarget::File)
}

fn paragraph() -> Anddress {
    address(AnddressTarget::Paragraph)
}

fn line() -> Anddress {
    address(AnddressTarget::Line)
}

#[test]
fn public_values_have_only_the_required_traits() {
    fn value_traits<T: Clone + Debug + Eq + PartialEq>() {}
    fn error_traits<T: Clone + Copy + Debug + Eq + PartialEq + Error>() {}

    value_traits::<Position>();
    value_traits::<Edit>();
    error_traits::<EditError>();

    let source = include_str!("../src/backwriter/edit.rs");
    for forbidden in [
        "Default",
        "serde",
        "Hash",
        "Request",
        "Outcome",
        "Builder",
        "trait ",
        "WorkspaceRuntime",
    ] {
        assert!(
            !source.contains(forbidden),
            "unexpected {forbidden} surface"
        );
    }
}

#[test]
fn positions_accept_only_their_contractual_target_kinds() {
    for position in [
        Position::Before(paragraph()),
        Position::Before(line()),
        Position::After(paragraph()),
        Position::After(line()),
        Position::StartOf(file()),
        Position::EndOf(file()),
    ] {
        assert_eq!(
            Edit::Insert {
                position,
                content: String::new(),
            }
            .validate(),
            Ok(())
        );
    }

    for position in [
        Position::Before(file()),
        Position::After(file()),
        Position::StartOf(paragraph()),
        Position::StartOf(line()),
        Position::EndOf(paragraph()),
        Position::EndOf(line()),
    ] {
        assert_eq!(
            Edit::Insert {
                position,
                content: String::new(),
            }
            .validate(),
            Err(EditError::InvalidInput)
        );
    }
}

#[test]
fn operations_accept_exactly_their_contractual_targets() {
    for edit in [
        Edit::Replace {
            target: file(),
            content: String::new(),
        },
        Edit::Replace {
            target: paragraph(),
            content: String::new(),
        },
        Edit::Replace {
            target: line(),
            content: String::new(),
        },
        Edit::Delete {
            target: paragraph(),
        },
        Edit::Delete { target: line() },
        Edit::Move {
            target: paragraph(),
            position: Position::Before(line()),
        },
        Edit::Move {
            target: line(),
            position: Position::EndOf(file()),
        },
        Edit::Copy {
            target: paragraph(),
            position: Position::After(line()),
        },
        Edit::Copy {
            target: line(),
            position: Position::StartOf(file()),
        },
    ] {
        assert_eq!(edit.validate(), Ok(()));
    }

    for edit in [
        Edit::Delete { target: file() },
        Edit::Move {
            target: file(),
            position: Position::StartOf(file()),
        },
        Edit::Copy {
            target: file(),
            position: Position::EndOf(file()),
        },
    ] {
        assert_eq!(edit.validate(), Err(EditError::InvalidInput));
    }
}

#[test]
fn public_construction_keeps_edit_operands_source_less_and_valid() {
    for edit in [
        Edit::Insert {
            position: Position::StartOf(file()),
            content: String::new(),
        },
        Edit::Replace {
            target: paragraph(),
            content: String::new(),
        },
        Edit::Move {
            target: line(),
            position: Position::EndOf(file()),
        },
    ] {
        assert_eq!(edit.validate(), Ok(()));
    }
}

#[test]
fn content_is_exact_and_only_rejects_nul() {
    for content in ["", "\r", "\n", "\r\n", "한글\t punctuation !?"] {
        let insert = Edit::Insert {
            position: Position::Before(paragraph()),
            content: content.to_owned(),
        };
        let replace = Edit::Replace {
            target: file(),
            content: content.to_owned(),
        };
        assert_eq!(insert.validate(), Ok(()));
        assert_eq!(replace.validate(), Ok(()));
        let Edit::Insert {
            content: inserted, ..
        } = insert
        else {
            unreachable!();
        };
        let Edit::Replace {
            content: replaced, ..
        } = replace
        else {
            unreachable!();
        };
        assert_eq!(inserted, content);
        assert_eq!(replaced, content);
    }

    for edit in [
        Edit::Insert {
            position: Position::After(line()),
            content: "before\0after".to_owned(),
        },
        Edit::Replace {
            target: paragraph(),
            content: "\0".to_owned(),
        },
    ] {
        assert_eq!(edit.validate(), Err(EditError::InvalidInput));
    }
}

#[test]
fn move_and_copy_do_not_add_relational_or_size_constraints() {
    let other_source = Anddress::new(
        &"a".repeat(64),
        "other.txt",
        &"b".repeat(64),
        4,
        AnddressTarget::Line,
        0,
        4,
    )
    .unwrap();
    for edit in [
        Edit::Move {
            target: paragraph(),
            position: Position::Before(other_source),
        },
        Edit::Copy {
            target: line(),
            position: Position::After(line()),
        },
        Edit::Insert {
            position: Position::EndOf(file()),
            content: "x".repeat(4098),
        },
    ] {
        assert_eq!(edit.validate(), Ok(()));
    }
}
