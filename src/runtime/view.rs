//! Runtime binding and one-forward-read execution for exact View.

use std::io::Read;

use crate::backwriter::anddress::{Anddress, AnddressTarget, LineTerminator, construct_anddress};
use crate::backwriter::view::{ViewError, ViewOutcome, validate_input};

use super::{
    WorkspaceRuntime, is_backwriter_spill,
    source_scan::{SourceScanError, TargetProjection, observe_source},
};

#[derive(Clone, Copy, Debug)]
pub(super) enum ObservationError {
    Read,
    InvalidSource,
    Resource,
}

pub(super) struct AnchoredObservation {
    pub(super) current: Vec<bool>,
    pub(super) outcome: Option<ViewOutcome>,
}

pub(super) fn observe_anchored(
    reader: &mut impl Read,
    inputs: &[Anddress],
    capture_focus: Option<usize>,
) -> Result<AnchoredObservation, ObservationError> {
    let indexes = indices(inputs.len())?;
    let mut targets = TargetProjection::new(inputs, &indexes).map_err(map_scan_error)?;
    let mut capture = capture_focus.map(|focus| DirectViewProjection::new(&inputs[focus]));
    let state = observe_source(reader, |bytes, chunk_start| {
        if let Some(capture) = capture.as_mut() {
            capture.push(bytes, chunk_start)?;
        }
        targets.push(bytes, chunk_start)
    })
    .map_err(map_scan_error)?;
    targets.finish(&state);
    let current = targets.into_current();
    let outcome = if capture_focus.is_some_and(|focus| current[focus]) {
        Some(
            capture
                .expect("capture focus creates a View capture")
                .finish(state.byte_length)
                .map_err(map_scan_error)?,
        )
    } else {
        None
    };
    Ok(AnchoredObservation { current, outcome })
}

pub(super) fn execute(
    runtime: &WorkspaceRuntime,
    input: &Anddress,
) -> Result<ViewOutcome, ViewError> {
    validate_input(input)?;
    if is_backwriter_spill(input.logical_path())
        || input.workspace_coordinate() != runtime.workspace_coordinate
    {
        return Err(ViewError::Unavailable);
    }
    let mut file = runtime
        .open_admitted_source(input.logical_path())
        .map_err(|_| ViewError::Unavailable)?;
    observe_direct(&mut file, input)
        .map_err(|_| ViewError::Unavailable)?
        .ok_or(ViewError::Unavailable)
}

pub(super) fn observe_direct(
    reader: &mut impl Read,
    input: &Anddress,
) -> Result<Option<ViewOutcome>, SourceScanError> {
    let mut projection = DirectViewProjection::new(input);
    let state = observe_source(reader, |bytes, chunk_start| {
        projection.push(bytes, chunk_start)
    })?;
    if input.source_byte_length() != state.byte_length || input.source_state_hash() != state.hash {
        return Ok(None);
    }
    projection.finish(state.byte_length).map(Some)
}

struct DirectViewProjection<'a> {
    input: &'a Anddress,
    target: Vec<u8>,
    line_relation: Option<LineRelation>,
}

impl<'a> DirectViewProjection<'a> {
    fn new(input: &'a Anddress) -> Self {
        Self {
            input,
            target: Vec::new(),
            line_relation: (input.target() == AnddressTarget::Line)
                .then(|| LineRelation::new(input.byte_start(), input.byte_end())),
        }
    }

    fn push(&mut self, bytes: &[u8], chunk_start: usize) -> Result<(), SourceScanError> {
        if self.input.target() == AnddressTarget::File {
            append(&mut self.target, bytes)?;
        } else {
            append_overlap(
                &mut self.target,
                bytes,
                chunk_start,
                self.input.byte_start(),
                self.input.byte_end(),
            )?;
        }
        if let Some(relation) = self.line_relation.as_mut() {
            relation.push(bytes, chunk_start)?;
        }
        Ok(())
    }

    fn finish(mut self, source_byte_length: usize) -> Result<ViewOutcome, SourceScanError> {
        match self.input.target() {
            AnddressTarget::File => Ok(ViewOutcome::File {
                text: String::from_utf8(self.target).map_err(|_| SourceScanError::InvalidSource)?,
            }),
            AnddressTarget::Paragraph => Ok(ViewOutcome::Paragraph {
                text: String::from_utf8(self.target).map_err(|_| SourceScanError::InvalidSource)?,
                file: file_address(self.input)?,
            }),
            AnddressTarget::Line => {
                let (content, terminator) = line_parts(self.target)?;
                let mut relation = self
                    .line_relation
                    .take()
                    .expect("Line projection has relation state");
                relation.finish(source_byte_length);
                Ok(ViewOutcome::Line {
                    content,
                    terminator,
                    file: file_address(self.input)?,
                    paragraph: relation
                        .related
                        .map(|(start, end)| paragraph_address(self.input, start, end))
                        .transpose()?,
                })
            }
        }
    }
}

struct LineRelation {
    target_start: usize,
    target_end: usize,
    line_start: usize,
    line_started: bool,
    pending_cr: bool,
    line_has_text: bool,
    paragraph_start: usize,
    paragraph_end: usize,
    selected_in_paragraph: bool,
    in_paragraph: bool,
    related: Option<(usize, usize)>,
}

impl LineRelation {
    fn new(target_start: usize, target_end: usize) -> Self {
        Self {
            target_start,
            target_end,
            line_start: 0,
            line_started: false,
            pending_cr: false,
            line_has_text: false,
            paragraph_start: 0,
            paragraph_end: 0,
            selected_in_paragraph: false,
            in_paragraph: false,
            related: None,
        }
    }

    fn push(&mut self, bytes: &[u8], chunk_start: usize) -> Result<(), SourceScanError> {
        for (index, &byte) in bytes.iter().enumerate() {
            let byte_start = chunk_start
                .checked_add(index)
                .ok_or(SourceScanError::Resource)?;
            if self.pending_cr {
                if byte == b'\n' {
                    self.finish_line(byte_start.checked_add(1).ok_or(SourceScanError::Resource)?);
                    continue;
                }
                self.finish_line(byte_start);
            }
            self.begin_line(byte_start);
            match byte {
                b'\r' => self.pending_cr = true,
                b'\n' => {
                    self.finish_line(byte_start.checked_add(1).ok_or(SourceScanError::Resource)?)
                }
                _ => self.line_has_text |= !matches!(byte, b' ' | b'\t'),
            }
        }
        Ok(())
    }

    fn finish(&mut self, source_byte_length: usize) {
        if self.line_started {
            self.finish_line(source_byte_length);
        }
        self.finish_paragraph();
    }

    fn begin_line(&mut self, byte_start: usize) {
        if !self.line_started {
            self.line_started = true;
            self.line_start = byte_start;
        }
    }

    fn finish_line(&mut self, byte_end: usize) {
        let selected = self.line_start == self.target_start && byte_end == self.target_end;
        if self.line_has_text {
            if !self.in_paragraph {
                self.in_paragraph = true;
                self.paragraph_start = self.line_start;
            }
            self.paragraph_end = byte_end;
            self.selected_in_paragraph |= selected;
        } else {
            self.finish_paragraph();
        }
        self.line_started = false;
        self.pending_cr = false;
        self.line_has_text = false;
    }

    fn finish_paragraph(&mut self) {
        if !self.in_paragraph {
            return;
        }
        if self.selected_in_paragraph {
            self.related = Some((self.paragraph_start, self.paragraph_end));
        }
        self.in_paragraph = false;
        self.selected_in_paragraph = false;
    }
}

fn append_overlap(
    output: &mut Vec<u8>,
    bytes: &[u8],
    chunk_start: usize,
    target_start: usize,
    target_end: usize,
) -> Result<(), SourceScanError> {
    let chunk_end = chunk_start
        .checked_add(bytes.len())
        .ok_or(SourceScanError::Resource)?;
    let overlap_start = target_start.max(chunk_start);
    let overlap_end = target_end.min(chunk_end);
    if overlap_start < overlap_end {
        append(
            output,
            &bytes[overlap_start - chunk_start..overlap_end - chunk_start],
        )?;
    }
    Ok(())
}

fn indices(length: usize) -> Result<Vec<usize>, ObservationError> {
    let mut indexes = Vec::new();
    indexes
        .try_reserve_exact(length)
        .map_err(|_| ObservationError::Resource)?;
    for index in 0..length {
        indexes.push(index);
    }
    Ok(indexes)
}

fn map_scan_error(error: SourceScanError) -> ObservationError {
    match error {
        SourceScanError::Read => ObservationError::Read,
        SourceScanError::InvalidSource => ObservationError::InvalidSource,
        SourceScanError::Resource => ObservationError::Resource,
    }
}

fn line_parts(mut line_bytes: Vec<u8>) -> Result<(String, LineTerminator), SourceScanError> {
    let (length, terminator) = if line_bytes.ends_with(b"\r\n") {
        (line_bytes.len() - 2, LineTerminator::Crlf)
    } else if line_bytes.ends_with(b"\r") {
        (line_bytes.len() - 1, LineTerminator::Cr)
    } else if line_bytes.ends_with(b"\n") {
        (line_bytes.len() - 1, LineTerminator::Lf)
    } else {
        (line_bytes.len(), LineTerminator::None)
    };
    line_bytes.truncate(length);
    String::from_utf8(line_bytes)
        .map(|content| (content, terminator))
        .map_err(|_| SourceScanError::InvalidSource)
}

fn append(output: &mut Vec<u8>, bytes: &[u8]) -> Result<(), SourceScanError> {
    output
        .try_reserve(bytes.len())
        .map_err(|_| SourceScanError::Resource)?;
    output.extend_from_slice(bytes);
    Ok(())
}

fn file_address(input: &Anddress) -> Result<Anddress, SourceScanError> {
    construct_anddress(
        input.source_identity(),
        AnddressTarget::File,
        0,
        input.source_byte_length(),
    )
    .map_err(|_| SourceScanError::Resource)
}

fn paragraph_address(
    input: &Anddress,
    byte_start: usize,
    byte_end: usize,
) -> Result<Anddress, SourceScanError> {
    construct_anddress(
        input.source_identity(),
        AnddressTarget::Paragraph,
        byte_start,
        byte_end,
    )
    .map_err(|_| SourceScanError::Resource)
}

#[cfg(test)]
mod tests {
    use std::io::{self, Read};

    use crate::hash::Sha256;
    use crate::runtime::source_scan::READ_BUFFER_SIZE;

    use super::*;

    struct OneByteReader<'a> {
        bytes: &'a [u8],
        cursor: usize,
        fail_at: Option<usize>,
        ended: bool,
    }

    impl Read for OneByteReader<'_> {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            assert!(!self.ended, "observer retried after EOF or failure");
            if self.fail_at.is_some_and(|offset| self.cursor >= offset) {
                self.ended = true;
                return Err(io::Error::other("scripted failure"));
            }
            if self.cursor == self.bytes.len() {
                self.ended = true;
                return Ok(0);
            }
            buffer[0] = self.bytes[self.cursor];
            self.cursor += 1;
            Ok(1)
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

    #[test]
    fn direct_view_preserves_long_ranges_under_one_byte_reads() {
        for target_length in [READ_BUFFER_SIZE - 1, READ_BUFFER_SIZE, READ_BUFFER_SIZE + 1] {
            let mut bytes = vec![b'x'; target_length];
            bytes.extend_from_slice("é".as_bytes());
            bytes.extend_from_slice(b"\r\ntail");
            let input = address(
                &bytes,
                AnddressTarget::Line,
                0,
                target_length + "é".len() + 2,
            );
            let mut reader = OneByteReader {
                bytes: &bytes,
                cursor: 0,
                fail_at: None,
                ended: false,
            };

            let outcome = observe_direct(&mut reader, &input).unwrap().unwrap();

            assert_eq!(
                outcome,
                ViewOutcome::Line {
                    content: format!("{}é", "x".repeat(target_length)),
                    terminator: LineTerminator::Crlf,
                    file: address(&bytes, AnddressTarget::File, 0, bytes.len()),
                    paragraph: Some(address(&bytes, AnddressTarget::Paragraph, 0, bytes.len(),)),
                }
            );
            assert!(reader.ended);
        }
    }

    #[test]
    fn direct_view_discards_provisional_output_after_late_failure_or_state_mismatch() {
        let input = address(b"one\nlate", AnddressTarget::Line, 0, 4);
        let mut failed = OneByteReader {
            bytes: b"one\nlate",
            cursor: 0,
            fail_at: Some(4),
            ended: false,
        };
        assert_eq!(
            observe_direct(&mut failed, &input),
            Err(SourceScanError::Read)
        );

        for bytes in [b"two\nlate".as_slice(), b"one\nlonger".as_slice()] {
            let mut changed = OneByteReader {
                bytes,
                cursor: 0,
                fail_at: None,
                ended: false,
            };
            assert_eq!(observe_direct(&mut changed, &input).unwrap(), None);
            assert!(changed.ended);
        }

        let mut projection = DirectViewProjection::new(&input);
        projection.push(b"one\n", 0).unwrap();
        assert_eq!(
            projection.push(b"x", usize::MAX),
            Err(SourceScanError::Resource)
        );
    }

    #[test]
    fn ordinary_view_uses_only_the_direct_observation_path() {
        let production = include_str!("view.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        let direct = production.split("fn observe_direct").nth(1).unwrap();
        for forbidden in ["scan_source(", "SourceEvent", "ExactTargetTracker"] {
            assert!(!direct.contains(forbidden));
        }
        assert_eq!(direct.matches("observe_source(").count(), 1);

        let anchored = production
            .split("fn observe_anchored")
            .nth(1)
            .unwrap()
            .split("pub(super) fn execute")
            .next()
            .unwrap();
        assert!(anchored.contains("TargetProjection"));
        assert!(anchored.contains("observe_source("));
        assert!(anchored.contains("targets.push("));
        assert!(!anchored.contains("scan_source("));
    }

    #[test]
    fn anchored_one_byte_observation_preserves_utf8_terminators_and_related_addresses() {
        let bytes = "한글🦀\nβ\rγ".as_bytes();
        let inputs = [address(bytes, AnddressTarget::Line, 11, 14)];
        let mut reader = OneByteReader {
            bytes,
            cursor: 0,
            fail_at: None,
            ended: false,
        };

        let AnchoredObservation { current, outcome } =
            observe_anchored(&mut reader, &inputs, Some(0)).unwrap();

        assert_eq!(current, [true]);
        assert_eq!(
            outcome,
            Some(ViewOutcome::Line {
                content: "β".to_owned(),
                terminator: LineTerminator::Cr,
                file: address(bytes, AnddressTarget::File, 0, bytes.len()),
                paragraph: Some(address(bytes, AnddressTarget::Paragraph, 0, bytes.len())),
            })
        );
    }

    #[test]
    fn anchored_late_invalid_and_read_failure_discard_provisional_view_output() {
        let inputs = [address(b"one\n", AnddressTarget::File, 0, 4)];
        for (bytes, fail_at, expected) in [
            (
                b"one\n\xff".as_slice(),
                None,
                ObservationError::InvalidSource,
            ),
            (
                b"one\n\xe2".as_slice(),
                None,
                ObservationError::InvalidSource,
            ),
            (b"one\n\0".as_slice(), None, ObservationError::InvalidSource),
            (b"one\nlate".as_slice(), Some(4), ObservationError::Read),
        ] {
            let mut reader = OneByteReader {
                bytes,
                cursor: 0,
                fail_at,
                ended: false,
            };
            assert!(
                matches!(observe_anchored(&mut reader, &inputs, Some(0)), Err(error) if same_error(error, expected))
            );
        }
    }

    #[test]
    fn target_only_observation_leaves_view_outcome_absent() {
        let inputs = [
            address(b"one\n", AnddressTarget::File, 0, 4),
            address(b"one\n", AnddressTarget::Paragraph, 0, 4),
        ];
        let mut reader = OneByteReader {
            bytes: b"one\n",
            cursor: 0,
            fail_at: None,
            ended: false,
        };

        let AnchoredObservation { current, outcome } =
            observe_anchored(&mut reader, &inputs, None).unwrap();

        assert_eq!(current, [true, true]);
        assert_eq!(outcome, None);
    }

    #[test]
    fn target_only_observation_keeps_late_source_errors() {
        let inputs = [address(b"one\n", AnddressTarget::File, 0, 4)];
        for (bytes, fail_at, expected) in [
            (
                b"one\n\xff".as_slice(),
                None,
                ObservationError::InvalidSource,
            ),
            (b"one\n\0".as_slice(), None, ObservationError::InvalidSource),
            (b"one\nlate".as_slice(), Some(4), ObservationError::Read),
        ] {
            let mut reader = OneByteReader {
                bytes,
                cursor: 0,
                fail_at,
                ended: false,
            };
            assert!(
                matches!(observe_anchored(&mut reader, &inputs, None), Err(error) if same_error(error, expected))
            );
        }
    }

    fn same_error(left: ObservationError, right: ObservationError) -> bool {
        matches!(
            (left, right),
            (ObservationError::Read, ObservationError::Read)
                | (
                    ObservationError::InvalidSource,
                    ObservationError::InvalidSource
                )
                | (ObservationError::Resource, ObservationError::Resource)
        )
    }
}
