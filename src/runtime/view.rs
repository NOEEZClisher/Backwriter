//! Runtime binding and one-forward-read execution for exact View.

use std::io::Read;

use crate::backwriter::anddress::{
    Anddress, AnddressTarget, LineBodyClass, LineTerminator, Natural, construct_anddress,
};
use crate::backwriter::view::{ViewError, ViewOutcome, validate_input};

use super::{
    WorkspaceRuntime, is_backwriter_spill,
    source_scan::{DecimalOrdinal, ExactTargetTracker, SourceEvent, SourceScanError, scan_source},
};

#[derive(Clone, Copy, Debug)]
pub(super) enum ObservationError {
    Read,
    InvalidSource,
    Resource,
}

pub(super) struct Observation {
    pub(super) current: Vec<bool>,
    pub(super) outcome: Option<ViewOutcome>,
}

pub(super) fn observe(
    reader: &mut impl Read,
    inputs: &[Anddress],
    capture_focus: Option<usize>,
) -> Result<Observation, ObservationError> {
    let indexes = indices(inputs.len())?;
    let mut tracker = ExactTargetTracker::new(inputs, &indexes).map_err(map_scan_error)?;
    let mut capture = capture_focus
        .map(|focus| ViewCapture::new(&inputs[focus]))
        .transpose()
        .map_err(map_scan_error)?;
    scan_source(reader, |event| {
        if let Some(capture) = capture.as_mut() {
            capture.consume(event)?;
        }
        tracker.consume(event)
    })
    .map_err(map_scan_error)?;
    tracker.finish();
    let current = tracker.into_current();
    let outcome = if capture_focus.is_some_and(|focus| current[focus]) {
        Some(
            capture
                .expect("capture focus creates a View capture")
                .finish()
                .map_err(map_scan_error)?,
        )
    } else {
        None
    };
    Ok(Observation { current, outcome })
}

pub(super) fn execute(
    runtime: &WorkspaceRuntime,
    input: &Anddress,
) -> Result<ViewOutcome, ViewError> {
    validate_input(input)?;
    if is_backwriter_spill(&input.logical_path)
        || input.workspace_coordinate != runtime.workspace_coordinate
    {
        return Err(ViewError::Unavailable);
    }
    let mut file = runtime
        .open_admitted_source(&input.logical_path)
        .map_err(|_| ViewError::Unavailable)?;
    let observed = observe(&mut file, std::slice::from_ref(input), Some(0))
        .map_err(|_| ViewError::Unavailable)?;
    observed.outcome.ok_or(ViewError::Unavailable)
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

struct ViewCapture<'a> {
    input: &'a Anddress,
    file: Vec<u8>,
    paragraph: Vec<u8>,
    current_line: Vec<u8>,
    line_selected: bool,
    paragraph_candidate: bool,
    line_paragraph: Option<Natural>,
    paragraph_ordinal: DecimalOrdinal,
    in_paragraph: bool,
}

impl<'a> ViewCapture<'a> {
    fn new(input: &'a Anddress) -> Result<Self, SourceScanError> {
        Ok(Self {
            input,
            file: Vec::new(),
            paragraph: Vec::new(),
            current_line: Vec::new(),
            line_selected: false,
            paragraph_candidate: false,
            line_paragraph: None,
            paragraph_ordinal: DecimalOrdinal::zero()?,
            in_paragraph: false,
        })
    }

    fn consume(&mut self, event: SourceEvent<'_>) -> Result<(), SourceScanError> {
        match event {
            SourceEvent::StartLine { ordinal } => self.start_line(ordinal),
            SourceEvent::Byte { byte, content } => self.push_byte(byte, content),
            SourceEvent::EndLine { body_class, .. } => self.finish_line(body_class),
        }
    }

    fn start_line(&mut self, ordinal: &DecimalOrdinal) -> Result<(), SourceScanError> {
        self.current_line.clear();
        self.paragraph_candidate = matches!(
            &self.input.target,
            AnddressTarget::Paragraph {
                ordinal: target_ordinal,
            } if *target_ordinal == self.paragraph_ordinal.as_natural()?
        );
        self.line_selected = matches!(
            &self.input.target,
            AnddressTarget::Line {
                ordinal: target_ordinal,
                ..
            } if *target_ordinal == ordinal.as_natural()?
        );
        if self.line_selected {
            self.line_paragraph = None;
        }
        Ok(())
    }

    fn push_byte(&mut self, byte: u8, _content: bool) -> Result<(), SourceScanError> {
        if matches!(self.input.target, AnddressTarget::File) {
            push_byte(&mut self.file, byte)?;
        }
        if self.paragraph_candidate {
            push_byte(&mut self.current_line, byte)?;
        }
        Ok(())
    }

    fn finish_line(&mut self, body_class: LineBodyClass) -> Result<(), SourceScanError> {
        if body_class == LineBodyClass::Text {
            if self.paragraph_candidate {
                append(&mut self.paragraph, &self.current_line)?;
            }
            if self.line_selected {
                self.line_paragraph = Some(self.paragraph_ordinal.as_natural()?);
            }
            self.in_paragraph = true;
        } else if self.in_paragraph {
            self.paragraph_ordinal.increment()?;
            self.in_paragraph = false;
        }
        Ok(())
    }

    fn finish(self) -> Result<ViewOutcome, SourceScanError> {
        match &self.input.target {
            AnddressTarget::File => Ok(ViewOutcome::File {
                text: String::from_utf8(self.file).map_err(|_| SourceScanError::InvalidSource)?,
            }),
            AnddressTarget::Paragraph { .. } => Ok(ViewOutcome::Paragraph {
                text: String::from_utf8(self.paragraph)
                    .map_err(|_| SourceScanError::InvalidSource)?,
                file: file_address(self.input)?,
            }),
            AnddressTarget::Line { exact_extent, .. } => {
                let (content, terminator) = line_parts(exact_extent)?;
                Ok(ViewOutcome::Line {
                    content,
                    terminator,
                    file: file_address(self.input)?,
                    paragraph: self
                        .line_paragraph
                        .map(|ordinal| paragraph_address(self.input, ordinal))
                        .transpose()?,
                })
            }
        }
    }
}

fn line_parts(exact_extent: &str) -> Result<(String, LineTerminator), SourceScanError> {
    let (length, terminator) = if exact_extent.ends_with("\r\n") {
        (exact_extent.len() - 2, LineTerminator::Crlf)
    } else if exact_extent.ends_with('\r') {
        (exact_extent.len() - 1, LineTerminator::Cr)
    } else if exact_extent.ends_with('\n') {
        (exact_extent.len() - 1, LineTerminator::Lf)
    } else {
        (exact_extent.len(), LineTerminator::None)
    };
    let content = exact_extent
        .get(..length)
        .ok_or(SourceScanError::InvalidSource)?;
    let mut result = String::new();
    result
        .try_reserve_exact(content.len())
        .map_err(|_| SourceScanError::Resource)?;
    result.push_str(content);
    Ok((result, terminator))
}

fn push_byte(output: &mut Vec<u8>, byte: u8) -> Result<(), SourceScanError> {
    output
        .try_reserve(1)
        .map_err(|_| SourceScanError::Resource)?;
    output.push(byte);
    Ok(())
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
        &input.workspace_coordinate,
        &input.logical_path,
        AnddressTarget::File,
    )
    .map_err(|_| SourceScanError::Resource)
}

fn paragraph_address(input: &Anddress, ordinal: Natural) -> Result<Anddress, SourceScanError> {
    construct_anddress(
        &input.workspace_coordinate,
        &input.logical_path,
        AnddressTarget::Paragraph { ordinal },
    )
    .map_err(|_| SourceScanError::Resource)
}

#[cfg(test)]
mod tests {
    use std::io::{self, Read};

    use crate::backwriter::anddress::ANDDRESS_VERSION;

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

    fn address(target: AnddressTarget) -> Anddress {
        Anddress {
            version: ANDDRESS_VERSION.to_owned(),
            workspace_coordinate: "0".repeat(64),
            logical_path: "source.txt".to_owned(),
            target,
        }
    }

    #[test]
    fn one_byte_observation_preserves_utf8_terminators_and_related_addresses() {
        let inputs = [address(AnddressTarget::Line {
            ordinal: Natural::one(),
            exact_extent: "β\r".to_owned(),
        })];
        let mut reader = OneByteReader {
            bytes: "한글🦀\nβ\rγ".as_bytes(),
            cursor: 0,
            fail_at: None,
            ended: false,
        };

        let Observation { current, outcome } = observe(&mut reader, &inputs, Some(0)).unwrap();

        assert_eq!(current, [true]);
        assert_eq!(
            outcome,
            Some(ViewOutcome::Line {
                content: "β".to_owned(),
                terminator: LineTerminator::Cr,
                file: address(AnddressTarget::File),
                paragraph: Some(address(AnddressTarget::Paragraph {
                    ordinal: Natural::zero(),
                })),
            })
        );
    }

    #[test]
    fn late_invalid_and_read_failure_discard_provisional_view_output() {
        let inputs = [address(AnddressTarget::File)];
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
                matches!(observe(&mut reader, &inputs, Some(0)), Err(error) if same_error(error, expected))
            );
        }
    }

    #[test]
    fn tracker_only_observation_leaves_view_outcome_absent() {
        let inputs = [
            address(AnddressTarget::File),
            address(AnddressTarget::Paragraph {
                ordinal: Natural::zero(),
            }),
        ];
        let mut reader = OneByteReader {
            bytes: b"one\n",
            cursor: 0,
            fail_at: None,
            ended: false,
        };

        let Observation { current, outcome } = observe(&mut reader, &inputs, None).unwrap();

        assert_eq!(current, [true, true]);
        assert_eq!(outcome, None);
    }

    #[test]
    fn tracker_only_observation_keeps_late_source_errors() {
        let inputs = [address(AnddressTarget::File)];
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
                matches!(observe(&mut reader, &inputs, None), Err(error) if same_error(error, expected))
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
