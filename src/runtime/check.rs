//! Runtime-owned Check V1 currentness classification.

use super::{
    DirectoryAccessError, WorkspaceRuntime, is_backwriter_spill,
    source_scan::{ExactTargetTracker, SourceScanError, scan_source},
};
use crate::backwriter::anddress::{Anddress, AnddressError, AnddressTarget};
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
    let needs_structure = group
        .iter()
        .any(|&index| inputs[index].target() != AnddressTarget::File);
    let mut tracker = needs_structure
        .then(|| ExactTargetTracker::new(inputs, group))
        .transpose()
        .map_err(|_| CheckError::Resource)?;
    let scanned = scan_source(reader, |event| match tracker.as_mut() {
        Some(tracker) => tracker.consume(event),
        None => Ok(()),
    });
    match scanned {
        Ok(state) => {
            match tracker {
                Some(mut tracker) => {
                    tracker.finish(&state);
                    for &index in group {
                        if tracker.is_current(index) {
                            statuses[index] = Currentness::Current;
                        }
                    }
                }
                None => {
                    for &index in group {
                        let input = &inputs[index];
                        if input.source_byte_length() == state.byte_length
                            && input.source_state_hash() == state.hash
                        {
                            statuses[index] = Currentness::Current;
                        }
                    }
                }
            }
            Ok(())
        }
        Err(SourceScanError::Read | SourceScanError::InvalidSource) => {
            set_group(statuses, group, Currentness::Unavailable);
            Ok(())
        }
        Err(SourceScanError::Resource) => Err(CheckError::Resource),
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
    use std::io::{self, Read};

    use crate::backwriter::anddress::AnddressTarget;
    use crate::hash::Sha256;
    use crate::runtime::source_scan::READ_BUFFER_SIZE;

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

    #[test]
    fn one_byte_reads_preserve_utf8_terminators_and_forward_only_access() {
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
    fn large_line_range_requires_the_complete_final_byte_and_length() {
        let source = format!("{}\n", "x".repeat(READ_BUFFER_SIZE * 3));
        let mut mismatched = source.clone();
        mismatched.pop();
        mismatched.push('!');
        let inputs = vec![
            address(source.as_bytes(), AnddressTarget::Line, 0, source.len()),
            address(
                mismatched.as_bytes(),
                AnddressTarget::Line,
                0,
                mismatched.len(),
            ),
        ];
        let group = [0, 1];
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

        assert_eq!(statuses, [Currentness::Current, Currentness::NotCurrent]);
    }
}
