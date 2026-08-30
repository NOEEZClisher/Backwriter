//! Runtime-owned Check V1 currentness classification.

use super::{
    DirectoryAccessError, WorkspaceRuntime, is_backwriter_spill,
    source_scan::{SourceScanError, observe_source},
};
use crate::backwriter::anddress::{Anddress, AnddressError};
use crate::backwriter::check::{CheckError, CheckOutcome, CheckReport};
use crate::backwriter::pick::PickOutcome;
use crate::backwriter::search::SearchOutcome;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Currentness {
    Current,
    NotCurrent,
    Unavailable,
}

pub(super) fn check_one(
    runtime: &WorkspaceRuntime,
    input: Anddress,
) -> Result<CheckOutcome<Option<Anddress>>, CheckError> {
    validate_one(&input)?;
    let mut inputs = Vec::new();
    inputs
        .try_reserve_exact(1)
        .map_err(|_| CheckError::Resource)?;
    inputs.push(input);
    let CheckOutcome {
        mut filtered,
        report,
    } = execute_prevalidated_batch(runtime, inputs)?;
    Ok(CheckOutcome {
        filtered: filtered.pop(),
        report,
    })
}

pub(super) fn check_search(
    runtime: &WorkspaceRuntime,
    input: SearchOutcome,
) -> Result<CheckOutcome<SearchOutcome>, CheckError> {
    let inputs = match input {
        SearchOutcome::Empty => Vec::new(),
        SearchOutcome::Found { anddresses } => anddresses,
    };
    validate_all(&inputs)?;
    let CheckOutcome { filtered, report } = execute_prevalidated_batch(runtime, inputs)?;
    Ok(CheckOutcome {
        filtered: search_outcome(filtered),
        report,
    })
}

pub(super) fn check_pick(
    runtime: &WorkspaceRuntime,
    input: PickOutcome,
) -> Result<CheckOutcome<PickOutcome>, CheckError> {
    let inputs = match input {
        PickOutcome::Empty => Vec::new(),
        PickOutcome::Selected { anddresses } => anddresses,
    };
    validate_all(&inputs)?;
    let CheckOutcome { filtered, report } = execute_prevalidated_batch(runtime, inputs)?;
    Ok(CheckOutcome {
        filtered: pick_outcome(filtered),
        report,
    })
}

fn execute_prevalidated_batch(
    runtime: &WorkspaceRuntime,
    inputs: Vec<Anddress>,
) -> Result<CheckOutcome<Vec<Anddress>>, CheckError> {
    let mut order = indices(inputs.len())?;
    order.sort_unstable_by(|left, right| compare_groups(&inputs[*left], &inputs[*right]));
    let mut statuses = Vec::new();
    statuses
        .try_reserve_exact(inputs.len())
        .map_err(|_| CheckError::Resource)?;
    statuses.resize(inputs.len(), Currentness::NotCurrent);

    let mut start = 0;
    while start < order.len() {
        let end = group_end(&inputs, &order, start);
        classify_group(runtime, &inputs, &order[start..end], &mut statuses)?;
        start = end;
    }
    finish(inputs, statuses)
}

fn validate_all(inputs: &[Anddress]) -> Result<(), CheckError> {
    for input in inputs {
        validate_one(input)?;
    }
    Ok(())
}

fn validate_one(input: &Anddress) -> Result<(), CheckError> {
    input.validate().map_err(|error| match error {
        AnddressError::UnsupportedVersion => CheckError::UnsupportedVersion,
        AnddressError::Invalid | AnddressError::Encoding => CheckError::InvalidInput,
        AnddressError::Resource => CheckError::Resource,
    })
}

fn indices(length: usize) -> Result<Vec<usize>, CheckError> {
    let mut result = Vec::new();
    result
        .try_reserve_exact(length)
        .map_err(|_| CheckError::Resource)?;
    for index in 0..length {
        result.push(index);
    }
    Ok(result)
}

fn compare_groups(left: &Anddress, right: &Anddress) -> std::cmp::Ordering {
    left.workspace_coordinate()
        .as_bytes()
        .cmp(right.workspace_coordinate().as_bytes())
        .then_with(|| {
            left.logical_path()
                .as_bytes()
                .cmp(right.logical_path().as_bytes())
        })
}

fn same_group(left: &Anddress, right: &Anddress) -> bool {
    left.workspace_coordinate() == right.workspace_coordinate()
        && left.logical_path() == right.logical_path()
}

fn group_end(inputs: &[Anddress], order: &[usize], start: usize) -> usize {
    let first = &inputs[order[start]];
    let mut end = start + 1;
    while end < order.len() && same_group(first, &inputs[order[end]]) {
        end += 1;
    }
    end
}

fn classify_group(
    runtime: &WorkspaceRuntime,
    inputs: &[Anddress],
    group: &[usize],
    statuses: &mut [Currentness],
) -> Result<(), CheckError> {
    let exemplar = &inputs[group[0]];
    if exemplar.workspace_coordinate() != runtime.workspace_coordinate
        || is_backwriter_spill(exemplar.logical_path())
    {
        set_group(statuses, group, Currentness::NotCurrent);
        return Ok(());
    }
    if runtime.selected_root(exemplar.logical_path()).is_err() {
        set_group(statuses, group, Currentness::NotCurrent);
        return Ok(());
    }
    if let Some(proof) = runtime.select_current_proof(exemplar.logical_path()) {
        classify_source_state(&proof.hash, proof.byte_length, inputs, group, statuses);
        return Ok(());
    }
    let mut file = match runtime.open_admitted_source(exemplar.logical_path()) {
        Ok(file) => file,
        Err(DirectoryAccessError::Unadmitted | DirectoryAccessError::NotCurrent) => {
            set_group(statuses, group, Currentness::NotCurrent);
            return Ok(());
        }
        Err(DirectoryAccessError::Unavailable) => {
            set_group(statuses, group, Currentness::Unavailable);
            return Ok(());
        }
    };
    classify_observed_source(&mut file, inputs, group, statuses)
}

fn classify_observed_source(
    reader: &mut impl std::io::Read,
    inputs: &[Anddress],
    group: &[usize],
    statuses: &mut [Currentness],
) -> Result<(), CheckError> {
    match observe_source(reader, |_, _| Ok(())) {
        Ok(state) => {
            classify_source_state(
                state.hash.as_bytes(),
                state.byte_length,
                inputs,
                group,
                statuses,
            );
            Ok(())
        }
        Err(SourceScanError::Read | SourceScanError::InvalidSource) => {
            set_group(statuses, group, Currentness::Unavailable);
            Ok(())
        }
        Err(SourceScanError::Resource) => Err(CheckError::Resource),
    }
}

fn classify_source_state(
    hash: &[u8],
    byte_length: usize,
    inputs: &[Anddress],
    group: &[usize],
    statuses: &mut [Currentness],
) {
    for &index in group {
        let input = &inputs[index];
        statuses[index] = if super::source_state_matches(hash, byte_length, input) {
            Currentness::Current
        } else {
            Currentness::NotCurrent
        };
    }
}

fn set_group(statuses: &mut [Currentness], group: &[usize], status: Currentness) {
    for &index in group {
        statuses[index] = status;
    }
}

fn finish(
    inputs: Vec<Anddress>,
    statuses: Vec<Currentness>,
) -> Result<CheckOutcome<Vec<Anddress>>, CheckError> {
    let (current_count, removed_count, unavailable_count) = statuses.iter().fold(
        (0_usize, 0_usize, 0_usize),
        |(current_count, removed_count, unavailable_count), status| match status {
            Currentness::Current => (current_count + 1, removed_count, unavailable_count),
            Currentness::NotCurrent => (current_count, removed_count + 1, unavailable_count),
            Currentness::Unavailable => (current_count, removed_count, unavailable_count + 1),
        },
    );
    let mut filtered = Vec::new();
    let mut removed = Vec::new();
    let mut unavailable = Vec::new();
    filtered
        .try_reserve_exact(current_count + unavailable_count)
        .map_err(|_| CheckError::Resource)?;
    removed
        .try_reserve_exact(removed_count)
        .map_err(|_| CheckError::Resource)?;
    unavailable
        .try_reserve_exact(unavailable_count)
        .map_err(|_| CheckError::Resource)?;
    for (input, status) in inputs.into_iter().zip(statuses) {
        match status {
            Currentness::Current => {
                filtered.push(input);
            }
            Currentness::NotCurrent => removed.push(input),
            Currentness::Unavailable => {
                unavailable.push(input.clone());
                filtered.push(input);
            }
        }
    }
    Ok(CheckOutcome {
        filtered,
        report: CheckReport::from_parts(current_count, removed, unavailable),
    })
}

fn search_outcome(anddresses: Vec<Anddress>) -> SearchOutcome {
    if anddresses.is_empty() {
        SearchOutcome::Empty
    } else {
        SearchOutcome::Found { anddresses }
    }
}

fn pick_outcome(anddresses: Vec<Anddress>) -> PickOutcome {
    if anddresses.is_empty() {
        PickOutcome::Empty
    } else {
        PickOutcome::Selected { anddresses }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{self, Read},
        panic::{AssertUnwindSafe, catch_unwind},
    };

    use crate::backwriter::anddress::AnddressTarget;
    use crate::hash::Sha256;
    use crate::runtime::{
        AdmissionRoot, CurrentProof, WorkspaceAdmission, source_scan::READ_BUFFER_SIZE,
    };

    use super::*;

    struct FixtureReader<'a> {
        bytes: &'a [u8],
        cursor: usize,
        chunk_size: usize,
        fail_after: Option<usize>,
        failed: bool,
        returned_eof: bool,
    }

    impl Read for FixtureReader<'_> {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            assert!(!self.failed, "source was read again after an error");
            assert!(!self.returned_eof, "source was read again after EOF");
            if self.fail_after.is_some_and(|offset| self.cursor >= offset) {
                self.failed = true;
                return Err(io::Error::other("scripted failure"));
            }
            if self.cursor == self.bytes.len() {
                self.returned_eof = true;
                return Ok(0);
            }

            let failure_boundary = self.fail_after.unwrap_or(self.bytes.len());
            let count = self
                .chunk_size
                .min(buffer.len())
                .min(failure_boundary - self.cursor)
                .min(self.bytes.len() - self.cursor);
            assert_ne!(count, 0, "fixture made no forward progress");
            let end = self.cursor + count;
            buffer[..count].copy_from_slice(&self.bytes[self.cursor..end]);
            self.cursor = end;
            Ok(count)
        }
    }

    fn reader(bytes: &[u8], fail_after: Option<usize>) -> FixtureReader<'_> {
        FixtureReader {
            bytes,
            cursor: 0,
            chunk_size: 1,
            fail_after,
            failed: false,
            returned_eof: false,
        }
    }

    fn address(bytes: &[u8], target: AnddressTarget, start: usize, end: usize) -> Anddress {
        let mut hash = Sha256::new();
        hash.update(bytes);
        Anddress::new(
            &"0".repeat(64),
            "source.txt",
            &hash.finish().to_hex(),
            bytes.len(),
            target,
            start,
            end,
        )
        .unwrap()
    }

    fn all_target_kinds() -> Vec<Anddress> {
        let bytes = b"one\nstale\n";
        vec![
            address(bytes, AnddressTarget::File, 0, bytes.len()),
            address(bytes, AnddressTarget::Paragraph, 0, bytes.len()),
            address(bytes, AnddressTarget::Line, 0, 4),
            address(bytes, AnddressTarget::Line, 4, bytes.len()),
        ]
    }

    fn runtime_address(runtime: &WorkspaceRuntime, bytes: &[u8]) -> Anddress {
        let mut hash = Sha256::new();
        hash.update(bytes);
        Anddress::new(
            &runtime.workspace_coordinate,
            "source.txt",
            &hash.finish().to_hex(),
            bytes.len(),
            AnddressTarget::File,
            0,
            bytes.len(),
        )
        .unwrap()
    }

    #[test]
    fn one_byte_reads_compare_only_source_hash_and_length() {
        let bytes = "é€🦀\r\nnext\rthird".as_bytes();
        let inputs = vec![
            address(bytes, AnddressTarget::File, 0, bytes.len()),
            address(bytes, AnddressTarget::Paragraph, 0, bytes.len()),
            address(bytes, AnddressTarget::Line, 0, 11),
            address(bytes, AnddressTarget::Line, 11, 16),
            address(bytes, AnddressTarget::Line, 16, 21),
        ];
        let group = [0, 1, 2, 3, 4];
        let mut statuses = vec![Currentness::NotCurrent; inputs.len()];
        let mut source = reader(bytes, None);

        classify_observed_source(&mut source, &inputs, &group, &mut statuses).unwrap();

        assert_eq!(statuses, vec![Currentness::Current; inputs.len()]);
        assert!(source.returned_eof);
    }

    #[test]
    fn late_invalid_or_failed_source_overrides_every_provisional_status() {
        for (bytes, failure) in [
            (b"one\n\xff".as_slice(), None),
            (b"one\n\xe2".as_slice(), None),
            (b"one\n\0".as_slice(), None),
            (b"one\nlate".as_slice(), Some(4)),
        ] {
            let inputs = all_target_kinds();
            let group = [0, 1, 2, 3];
            let mut statuses = vec![Currentness::NotCurrent; inputs.len()];
            let mut source = reader(bytes, failure);

            classify_observed_source(&mut source, &inputs, &group, &mut statuses).unwrap();

            assert_eq!(statuses, vec![Currentness::Unavailable; inputs.len()]);
        }
    }

    #[test]
    fn file_only_reads_through_a_late_failure() {
        let inputs = vec![address(
            b"valid prefix then failure",
            AnddressTarget::File,
            0,
            25,
        )];
        let group = [0];
        let mut statuses = vec![Currentness::NotCurrent];
        let mut source = reader(b"valid prefix then failure", Some(12));

        classify_observed_source(&mut source, &inputs, &group, &mut statuses).unwrap();

        assert_eq!(statuses, vec![Currentness::Unavailable]);
    }

    #[test]
    fn mixed_geometry_is_current_and_hash_mismatch_is_not_current() {
        let source = format!("{}\n", "x".repeat(READ_BUFFER_SIZE * 3));
        let mut mismatched = source.clone();
        mismatched.pop();
        mismatched.push('!');
        let raw = address(
            source.as_bytes(),
            AnddressTarget::Paragraph,
            1,
            source.len() - 1,
        );
        let inputs = vec![
            address(source.as_bytes(), AnddressTarget::File, 0, source.len()),
            raw.clone(),
            address(source.as_bytes(), AnddressTarget::Line, 7, 7),
            address(
                mismatched.as_bytes(),
                AnddressTarget::Line,
                0,
                mismatched.len(),
            ),
            raw,
        ];
        let group = [0, 1, 2, 3, 4];
        let mut statuses = vec![Currentness::NotCurrent; inputs.len()];
        let mut observed = FixtureReader {
            bytes: source.as_bytes(),
            cursor: 0,
            chunk_size: READ_BUFFER_SIZE,
            fail_after: None,
            failed: false,
            returned_eof: false,
        };

        classify_observed_source(&mut observed, &inputs, &group, &mut statuses).unwrap();

        assert_eq!(
            statuses,
            [
                Currentness::Current,
                Currentness::Current,
                Currentness::Current,
                Currentness::NotCurrent,
                Currentness::Current,
            ]
        );
        assert!(observed.returned_eof);
    }

    #[test]
    fn proof_miss_poison_and_unusable_state_fall_back_without_check_installation() {
        let fixture = tempfile::tempdir().unwrap();
        let bytes = b"current\n";
        fs::write(fixture.path().join("source.txt"), bytes).unwrap();
        let admission = || WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap();

        let missing =
            WorkspaceRuntime::open_host_authoritative(fixture.path(), admission()).unwrap();
        let input = runtime_address(&missing, bytes);
        assert_eq!(missing.check(input.clone()).unwrap().filtered, Some(input));
        assert!(missing.current_proofs.lock().unwrap().is_empty());

        let unusable =
            WorkspaceRuntime::open_host_authoritative(fixture.path(), admission()).unwrap();
        unusable
            .install_search_proofs(vec![
                CurrentProof::new("source.txt", "short".to_owned(), bytes.len()).unwrap(),
            ])
            .unwrap();
        let input = runtime_address(&unusable, bytes);
        assert_eq!(unusable.check(input.clone()).unwrap().filtered, Some(input));
        let proofs = unusable.current_proofs.lock().unwrap();
        assert_eq!(proofs.len(), 1);
        assert_eq!(proofs[0].hash, "short");
        drop(proofs);

        let poisoned =
            WorkspaceRuntime::open_host_authoritative(fixture.path(), admission()).unwrap();
        let input = runtime_address(&poisoned, bytes);
        poisoned
            .install_search_proofs(vec![
                CurrentProof::new(
                    "source.txt",
                    input.source_state_hash().to_owned(),
                    input.source_byte_length(),
                )
                .unwrap(),
            ])
            .unwrap();
        assert!(
            catch_unwind(AssertUnwindSafe(|| {
                let _proofs = poisoned.current_proofs.lock().unwrap();
                panic!("poison current proof lock");
            }))
            .is_err()
        );
        assert_eq!(poisoned.check(input.clone()).unwrap().filtered, Some(input));
    }

    #[test]
    fn check_production_has_no_structural_scanner_or_target_branch() {
        let production = include_str!("check.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        for forbidden in [
            "ExactTargetTracker",
            "SourceEvent",
            "scan_source(",
            "AnddressTarget",
        ] {
            assert!(!production.contains(forbidden));
        }
        assert_eq!(production.matches("observe_source(").count(), 1);
        assert_eq!(production.matches("select_current_proof(").count(), 1);
        assert!(!production.contains("install_search_proofs"));
        assert!(!production.contains("invalidate_current_proof"));
        let group = production
            .split("fn classify_group")
            .nth(1)
            .unwrap()
            .split("fn classify_observed_source")
            .next()
            .unwrap();
        assert!(group.find("select_current_proof") < group.find("open_admitted_source"));

        let source_scan = include_str!("source_scan.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        assert!(!source_scan.contains("fn is_current"));
        let apply = include_str!("apply.rs");
        assert_eq!(apply.matches("observe_source(source").count(), 1);
        assert_eq!(apply.matches("stage_source(&mut source").count(), 1);
        assert!(!apply.contains("scan_source("));
        assert!(include_str!("anchor.rs").contains("observe_anchored"));

        let runtime = include_str!("../runtime.rs");
        assert!(runtime.contains("#[derive(Clone, Copy)]\nstruct SourceProofEvidence"));
        assert!(
            runtime.contains(
                "fn select_current_proof(&self, path: &str) -> Option<SourceProofEvidence>"
            )
        );
    }
}
