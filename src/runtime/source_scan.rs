//! Private forward framing shared by Runtime source observers.

use std::io::Read;

use crate::backwriter::anddress::{Anddress, AnddressTarget, LineBodyClass};
use crate::hash::Sha256;

pub(crate) const READ_BUFFER_SIZE: usize = 8_192;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SourceScanError {
    Read,
    InvalidSource,
    Resource,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct CurrentObservation {
    pub(crate) hash: String,
    pub(crate) byte_length: usize,
}

#[derive(Clone, Copy)]
pub(crate) enum SourceEvent {
    StartLine {
        byte_start: usize,
        line_index: usize,
    },
    Byte {
        byte: u8,
        content: bool,
    },
    EndLine {
        byte_start: usize,
        byte_end: usize,
        body_class: LineBodyClass,
    },
}

/// Tracks exact current target evidence without retaining source bytes.
pub(crate) struct ExactTargetTracker<'a> {
    inputs: &'a [Anddress],
    indexes: &'a [usize],
    current: Vec<bool>,
    paragraph_start: usize,
    paragraph_end: usize,
    in_paragraph: bool,
}

impl<'a> ExactTargetTracker<'a> {
    pub(crate) fn new(
        inputs: &'a [Anddress],
        indexes: &'a [usize],
    ) -> Result<Self, SourceScanError> {
        let mut current = Vec::new();
        current
            .try_reserve_exact(inputs.len())
            .map_err(|_| SourceScanError::Resource)?;
        current.resize(inputs.len(), false);
        Ok(Self {
            inputs,
            indexes,
            current,
            paragraph_start: 0,
            paragraph_end: 0,
            in_paragraph: false,
        })
    }

    pub(crate) fn consume(&mut self, event: SourceEvent) -> Result<(), SourceScanError> {
        match event {
            SourceEvent::StartLine { .. } | SourceEvent::Byte { .. } => Ok(()),
            SourceEvent::EndLine {
                byte_start,
                byte_end,
                body_class,
                ..
            } => {
                self.finish_line(byte_start, byte_end, body_class);
                Ok(())
            }
        }
    }

    pub(crate) fn finish(&mut self, state: &CurrentObservation) {
        self.finish_paragraph();
        for &index in self.indexes {
            let input = &self.inputs[index];
            let source_matches = input.source_byte_length() == state.byte_length
                && input.source_state_hash() == state.hash;
            if !source_matches {
                self.current[index] = false;
            } else if input.target() == AnddressTarget::File {
                self.current[index] = true;
            }
        }
    }

    pub(crate) fn into_current(self) -> Vec<bool> {
        self.current
    }

    fn finish_line(&mut self, byte_start: usize, byte_end: usize, body_class: LineBodyClass) {
        for &index in self.indexes {
            let input = &self.inputs[index];
            if input.target() == AnddressTarget::Line
                && input.byte_start() == byte_start
                && input.byte_end() == byte_end
            {
                self.current[index] = true;
            }
        }

        if body_class == LineBodyClass::Text {
            if !self.in_paragraph {
                self.in_paragraph = true;
                self.paragraph_start = byte_start;
            }
            self.paragraph_end = byte_end;
        } else if self.in_paragraph {
            self.finish_paragraph();
        }
    }

    fn finish_paragraph(&mut self) {
        if !self.in_paragraph {
            return;
        }
        for &index in self.indexes {
            let input = &self.inputs[index];
            if input.target() == AnddressTarget::Paragraph
                && input.byte_start() == self.paragraph_start
                && input.byte_end() == self.paragraph_end
            {
                self.current[index] = true;
            }
        }
        self.in_paragraph = false;
    }
}

#[derive(Default)]
struct Utf8Validator {
    incomplete: [u8; 4],
    length: usize,
}

impl Utf8Validator {
    fn push(&mut self, bytes: &[u8]) -> bool {
        if bytes.contains(&0) {
            return false;
        }
        let mut cursor = 0;
        if self.length != 0 {
            while cursor < bytes.len() && self.length < self.incomplete.len() {
                self.incomplete[self.length] = bytes[cursor];
                self.length += 1;
                cursor += 1;
                match std::str::from_utf8(&self.incomplete[..self.length]) {
                    Ok(_) => {
                        self.length = 0;
                        break;
                    }
                    Err(error) if error.error_len().is_some() => return false,
                    Err(_) => {}
                }
            }
            if self.length == self.incomplete.len() {
                return false;
            }
            if self.length != 0 {
                return true;
            }
        }
        let remaining = &bytes[cursor..];
        match std::str::from_utf8(remaining) {
            Ok(_) => true,
            Err(error) if error.error_len().is_some() => false,
            Err(error) => {
                let suffix = &remaining[error.valid_up_to()..];
                debug_assert!(!suffix.is_empty() && suffix.len() < self.incomplete.len());
                self.incomplete[..suffix.len()].copy_from_slice(suffix);
                self.length = suffix.len();
                true
            }
        }
    }

    fn finish(&self) -> bool {
        self.length == 0
    }
}

/// One incremental UTF-8/NUL validator and exact-Line framer.  Consumers may
/// feed generated bytes through the same framing contract as retained source.
pub(crate) struct SourceFramer {
    byte_offset: usize,
    line_start: usize,
    line_index: usize,
    line_started: bool,
    pending_cr: bool,
    body_class: LineBodyClass,
    utf8: Utf8Validator,
}

impl SourceFramer {
    pub(crate) fn new() -> Result<Self, SourceScanError> {
        Ok(Self {
            byte_offset: 0,
            line_start: 0,
            line_index: 0,
            line_started: false,
            pending_cr: false,
            body_class: LineBodyClass::Empty,
            utf8: Utf8Validator::default(),
        })
    }

    pub(crate) fn push(
        &mut self,
        bytes: &[u8],
        on_event: &mut impl FnMut(SourceEvent) -> Result<(), SourceScanError>,
    ) -> Result<(), SourceScanError> {
        if !self.utf8.push(bytes) {
            return Err(SourceScanError::InvalidSource);
        }
        self.push_validated(bytes, on_event)
    }

    fn push_validated(
        &mut self,
        bytes: &[u8],
        on_event: &mut impl FnMut(SourceEvent) -> Result<(), SourceScanError>,
    ) -> Result<(), SourceScanError> {
        for &byte in bytes {
            if self.pending_cr {
                if byte == b'\n' {
                    on_event(SourceEvent::Byte {
                        byte,
                        content: false,
                    })?;
                    self.advance()?;
                    self.finish_line(on_event)?;
                    continue;
                }
                self.finish_line(on_event)?;
            }
            self.begin_line(on_event)?;
            match byte {
                b'\r' => {
                    on_event(SourceEvent::Byte {
                        byte,
                        content: false,
                    })?;
                    self.advance()?;
                    self.pending_cr = true;
                }
                b'\n' => {
                    on_event(SourceEvent::Byte {
                        byte,
                        content: false,
                    })?;
                    self.advance()?;
                    self.finish_line(on_event)?;
                }
                _ => {
                    on_event(SourceEvent::Byte {
                        byte,
                        content: true,
                    })?;
                    self.advance()?;
                    if !matches!(byte, b' ' | b'\t') {
                        self.body_class = LineBodyClass::Text;
                    } else if self.body_class == LineBodyClass::Empty {
                        self.body_class = LineBodyClass::HorizontalWhitespace;
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn finish(
        &mut self,
        on_event: &mut impl FnMut(SourceEvent) -> Result<(), SourceScanError>,
    ) -> Result<(), SourceScanError> {
        if !self.utf8.finish() {
            return Err(SourceScanError::InvalidSource);
        }
        self.finish_validated(on_event)
    }

    fn finish_validated(
        &mut self,
        on_event: &mut impl FnMut(SourceEvent) -> Result<(), SourceScanError>,
    ) -> Result<(), SourceScanError> {
        if self.line_started {
            self.finish_line(on_event)?;
        }
        Ok(())
    }

    fn begin_line(
        &mut self,
        on_event: &mut impl FnMut(SourceEvent) -> Result<(), SourceScanError>,
    ) -> Result<(), SourceScanError> {
        if !self.line_started {
            self.line_started = true;
            self.line_start = self.byte_offset;
            on_event(SourceEvent::StartLine {
                byte_start: self.line_start,
                line_index: self.line_index,
            })?;
        }
        Ok(())
    }

    fn finish_line(
        &mut self,
        on_event: &mut impl FnMut(SourceEvent) -> Result<(), SourceScanError>,
    ) -> Result<(), SourceScanError> {
        on_event(SourceEvent::EndLine {
            byte_start: self.line_start,
            byte_end: self.byte_offset,
            body_class: self.body_class,
        })?;
        self.line_index = self
            .line_index
            .checked_add(1)
            .ok_or(SourceScanError::Resource)?;
        self.line_started = false;
        self.pending_cr = false;
        self.body_class = LineBodyClass::Empty;
        Ok(())
    }

    fn advance(&mut self) -> Result<(), SourceScanError> {
        self.byte_offset = self
            .byte_offset
            .checked_add(1)
            .ok_or(SourceScanError::Resource)?;
        Ok(())
    }
}

/// Reads one call-local source observation while owning its text policy,
/// incremental digest, and checked byte length. The consumer sees each chunk
/// only after it has passed UTF-8 and NUL validation.
pub(crate) fn observe_source(
    reader: &mut impl Read,
    mut on_chunk: impl FnMut(&[u8], usize) -> Result<(), SourceScanError>,
) -> Result<CurrentObservation, SourceScanError> {
    let mut utf8 = Utf8Validator::default();
    let mut hash = Sha256::new();
    let mut byte_length = 0_usize;
    let mut scratch = [0_u8; READ_BUFFER_SIZE];
    loop {
        let count = reader
            .read(&mut scratch)
            .map_err(|_| SourceScanError::Read)?;
        if count == 0 {
            if !utf8.finish() {
                return Err(SourceScanError::InvalidSource);
            }
            return Ok(CurrentObservation {
                hash: hash.finish().to_hex(),
                byte_length,
            });
        }
        let chunk_start = byte_length;
        byte_length = byte_length
            .checked_add(count)
            .ok_or(SourceScanError::Resource)?;
        let bytes = &scratch[..count];
        if !utf8.push(bytes) {
            return Err(SourceScanError::InvalidSource);
        }
        hash.update(bytes);
        on_chunk(bytes, chunk_start)?;
    }
}

pub(crate) fn scan_source(
    reader: &mut impl Read,
    mut on_event: impl FnMut(SourceEvent) -> Result<(), SourceScanError>,
) -> Result<CurrentObservation, SourceScanError> {
    let mut scanner = SourceFramer::new()?;
    let observation = observe_source(reader, |bytes, _| {
        scanner.push_validated(bytes, &mut on_event)
    })?;
    scanner.finish_validated(&mut on_event)?;
    debug_assert_eq!(scanner.byte_offset, observation.byte_length);
    Ok(observation)
}
