//! Pure Core selection over caller-provided Anddress values.

use thiserror::Error;

use crate::backwriter::anddress::{Anddress, AnddressTarget};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PickTargetKind {
    File,
    Paragraph,
    Line,
}

pub struct PickPredicate(PredicateRepr);

#[allow(clippy::large_enum_variant)]
enum PredicateRepr {
    Atom(Atom),
    Program(Program),
}

#[allow(clippy::large_enum_variant)]
enum Atom {
    All,
    TargetKind(PickTargetKind),
    OneOf(Vec<Anddress>),
    SameFile(Anddress),
}

struct Program {
    opcodes: Vec<Opcode>,
    atoms: Vec<Atom>,
    max_stack: usize,
}

enum Opcode {
    Atom,
    Not,
    AllOf(usize),
    AnyOf(usize),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PickOutcome {
    Empty,
    Selected { anddresses: Vec<Anddress> },
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum PickError {
    #[error("Pick resource allocation failed")]
    Resource,
}

impl PickPredicate {
    pub fn all() -> Self {
        Self(PredicateRepr::Atom(Atom::All))
    }

    pub fn target_kind(kind: PickTargetKind) -> Self {
        Self(PredicateRepr::Atom(Atom::TargetKind(kind)))
    }

    pub fn one_of(members: Vec<Anddress>) -> Self {
        Self(PredicateRepr::Atom(Atom::OneOf(members)))
    }

    pub fn same_file(reference: Anddress) -> Self {
        Self(PredicateRepr::Atom(Atom::SameFile(reference)))
    }

    pub fn negate(inner: Self) -> Result<Self, PickError> {
        let opcode_count = inner
            .opcode_count()
            .checked_add(1)
            .ok_or(PickError::Resource)?;
        let atom_count = inner.atom_count();
        let mut program = Program::from_first(inner, opcode_count, atom_count)?;
        program.opcodes.push(Opcode::Not);
        Ok(Self(PredicateRepr::Program(program)))
    }

    pub fn all_of(first: Self, rest: Vec<Self>) -> Result<Self, PickError> {
        Self::combine(first, rest, true)
    }

    pub fn any_of(first: Self, rest: Vec<Self>) -> Result<Self, PickError> {
        Self::combine(first, rest, false)
    }

    fn combine(first: Self, rest: Vec<Self>, all: bool) -> Result<Self, PickError> {
        if rest.is_empty() {
            return Ok(first);
        }
        let arity = rest.len().checked_add(1).ok_or(PickError::Resource)?;
        let mut opcode_count = 0_usize;
        let mut atom_count = 0_usize;
        let mut completed = 0_usize;
        let mut max_stack = 0_usize;
        for predicate in std::iter::once(&first).chain(rest.iter()) {
            max_stack = max_stack.max(
                completed
                    .checked_add(predicate.max_stack())
                    .ok_or(PickError::Resource)?,
            );
            completed = completed.checked_add(1).ok_or(PickError::Resource)?;
            opcode_count = opcode_count
                .checked_add(predicate.opcode_count())
                .ok_or(PickError::Resource)?;
            atom_count = atom_count
                .checked_add(predicate.atom_count())
                .ok_or(PickError::Resource)?;
        }
        opcode_count = opcode_count.checked_add(1).ok_or(PickError::Resource)?;

        let mut program = Program::from_first(first, opcode_count, atom_count)?;
        for predicate in rest {
            program.append(predicate);
        }
        program.opcodes.push(if all {
            Opcode::AllOf(arity)
        } else {
            Opcode::AnyOf(arity)
        });
        program.max_stack = max_stack;
        Ok(Self(PredicateRepr::Program(program)))
    }

    fn opcode_count(&self) -> usize {
        match &self.0 {
            PredicateRepr::Atom(_) => 1,
            PredicateRepr::Program(program) => program.opcodes.len(),
        }
    }

    fn atom_count(&self) -> usize {
        match &self.0 {
            PredicateRepr::Atom(_) => 1,
            PredicateRepr::Program(program) => program.atoms.len(),
        }
    }

    fn max_stack(&self) -> usize {
        match &self.0 {
            PredicateRepr::Atom(_) => 1,
            PredicateRepr::Program(program) => program.max_stack,
        }
    }
}

impl Program {
    fn from_first(
        first: PickPredicate,
        opcode_count: usize,
        atom_count: usize,
    ) -> Result<Self, PickError> {
        match first.0 {
            PredicateRepr::Atom(atom) => {
                let mut opcodes = Vec::new();
                opcodes
                    .try_reserve(opcode_count)
                    .map_err(|_| PickError::Resource)?;
                let mut atoms = Vec::new();
                atoms
                    .try_reserve(atom_count)
                    .map_err(|_| PickError::Resource)?;
                opcodes.push(Opcode::Atom);
                atoms.push(atom);
                Ok(Self {
                    opcodes,
                    atoms,
                    max_stack: 1,
                })
            }
            PredicateRepr::Program(mut program) => {
                program
                    .opcodes
                    .try_reserve(opcode_count - program.opcodes.len())
                    .map_err(|_| PickError::Resource)?;
                program
                    .atoms
                    .try_reserve(atom_count - program.atoms.len())
                    .map_err(|_| PickError::Resource)?;
                Ok(program)
            }
        }
    }

    fn append(&mut self, predicate: PickPredicate) {
        match predicate.0 {
            PredicateRepr::Atom(atom) => {
                self.opcodes.push(Opcode::Atom);
                self.atoms.push(atom);
            }
            PredicateRepr::Program(mut program) => {
                self.opcodes.append(&mut program.opcodes);
                self.atoms.append(&mut program.atoms);
            }
        }
    }

    fn matches(&self, candidate: &Anddress, stack: &mut Vec<bool>) -> bool {
        stack.clear();
        let mut atoms = self.atoms.iter();
        for opcode in &self.opcodes {
            match opcode {
                Opcode::Atom => stack.push(matches_atom(
                    atoms.next().expect("Pick program atoms match opcodes"),
                    candidate,
                )),
                Opcode::Not => {
                    let value = stack.last_mut().expect("valid Pick program");
                    *value = !*value;
                }
                Opcode::AllOf(arity) => reduce(stack, *arity, true),
                Opcode::AnyOf(arity) => reduce(stack, *arity, false),
            }
        }
        debug_assert!(atoms.next().is_none());
        stack.pop().expect("valid Pick program")
    }
}

pub fn pick(
    mut candidates: Vec<Anddress>,
    predicate: &PickPredicate,
) -> Result<PickOutcome, PickError> {
    if candidates.is_empty() {
        return Ok(PickOutcome::Empty);
    }
    match &predicate.0 {
        PredicateRepr::Atom(atom) => candidates.retain(|candidate| matches_atom(atom, candidate)),
        PredicateRepr::Program(program) => {
            let mut stack = Vec::new();
            stack
                .try_reserve(program.max_stack)
                .map_err(|_| PickError::Resource)?;
            candidates.retain(|candidate| program.matches(candidate, &mut stack));
        }
    }
    Ok(if candidates.is_empty() {
        PickOutcome::Empty
    } else {
        PickOutcome::Selected {
            anddresses: candidates,
        }
    })
}

fn matches_atom(atom: &Atom, candidate: &Anddress) -> bool {
    match atom {
        Atom::All => true,
        Atom::TargetKind(kind) => target_kind(candidate) == *kind,
        Atom::OneOf(members) => members.iter().any(|member| member == candidate),
        Atom::SameFile(reference) => {
            candidate.workspace_coordinate() == reference.workspace_coordinate()
                && candidate.logical_path() == reference.logical_path()
        }
    }
}

fn reduce(stack: &mut Vec<bool>, arity: usize, all: bool) {
    let start = stack.len().checked_sub(arity).expect("valid Pick program");
    let value = if all {
        stack[start..].iter().all(|value| *value)
    } else {
        stack[start..].iter().any(|value| *value)
    };
    stack.truncate(start);
    stack.push(value);
}

fn target_kind(anddress: &Anddress) -> PickTargetKind {
    match anddress.target() {
        AnddressTarget::File => PickTargetKind::File,
        AnddressTarget::Paragraph => PickTargetKind::Paragraph,
        AnddressTarget::Line => PickTargetKind::Line,
    }
}
