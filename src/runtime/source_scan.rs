//! Private one-pass source observation and exact target projection.

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

/// Tracks exact current target evidence without retaining source bytes.
pub(crate) struct TargetProjection<'a> {
    inputs: &'a [Anddress],
    indexes: &'a [usize],
    current: Vec<bool>,
    line_start: usize,
    line_started: bool,
    pending_cr: bool,
    line_has_text: bool,
    paragraph_start: usize,
    paragraph_end: usize,
    in_paragraph: bool,
}

impl<'a> TargetProjection<'a> {
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
            line_start: 0,
            line_started: false,
            pending_cr: false,
            line_has_text: false,
            paragraph_start: 0,
            paragraph_end: 0,
            in_paragraph: false,
        })
    }

    pub(crate) fn push(&mut self, bytes: &[u8], chunk_start: usize) -> Result<(), SourceScanError> {
        for (index, &byte) in bytes.iter().enumerate() {
            let byte_start = chunk_start
                .checked_add(index)
                .ok_or(SourceScanError::Resource)?;
            if self.pending_cr {
                if byte == b'\n' {
                    self.finish_direct_line(
                        byte_start.checked_add(1).ok_or(SourceScanError::Resource)?,
                    );
                    continue;
                }
                self.finish_direct_line(byte_start);
            }
            if !self.line_started {
                self.line_started = true;
                self.line_start = byte_start;
            }
            match byte {
                b'\r' => self.pending_cr = true,
                b'\n' => self.finish_direct_line(
                    byte_start.checked_add(1).ok_or(SourceScanError::Resource)?,
                ),
                _ => self.line_has_text |= !matches!(byte, b' ' | b'\t'),
            }
        }
        Ok(())
    }

    pub(crate) fn finish(&mut self, state: &CurrentObservation) {
        if self.line_started {
            self.finish_direct_line(state.byte_length);
        }
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

    fn finish_direct_line(&mut self, byte_end: usize) {
        self.finish_line(
            self.line_start,
            byte_end,
            LineBodyClass::from_has_text(self.line_has_text),
        );
        self.line_started = false;
        self.pending_cr = false;
        self.line_has_text = false;
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

impl LineBodyClass {
    fn from_has_text(has_text: bool) -> Self {
        if has_text { Self::Text } else { Self::Empty }
    }
}

#[derive(Default)]
struct Utf8Validator {
    incomplete: [u8; 4],
    length: usize,
}

#[derive(Default)]
struct SourceTextBuilder {
    utf8: Utf8Validator,
    byte_length: usize,
}

/// Incremental source-policy and exact-state accumulator shared by retained
/// source observation and generated Apply output.
pub(crate) struct ObservationBuilder {
    source: SourceTextBuilder,
    hash: Sha256,
}

impl ObservationBuilder {
    pub(crate) fn new() -> Result<Self, SourceScanError> {
        Ok(Self {
            source: SourceTextBuilder::default(),
            hash: Sha256::new(),
        })
    }

    pub(crate) fn push(&mut self, bytes: &[u8]) -> Result<usize, SourceScanError> {
        let chunk_start = self.source.push(bytes)?;
        self.hash.update(bytes);
        Ok(chunk_start)
    }

    pub(crate) fn finish(self) -> Result<CurrentObservation, SourceScanError> {
        let byte_length = self.source.finish()?;
        Ok(CurrentObservation {
            hash: self.hash.finish().to_hex(),
            byte_length,
        })
    }
}

impl SourceTextBuilder {
    fn push(&mut self, bytes: &[u8]) -> Result<usize, SourceScanError> {
        if !self.utf8.push(bytes) {
            return Err(SourceScanError::InvalidSource);
        }
        let chunk_start = self.byte_length;
        self.byte_length = self
            .byte_length
            .checked_add(bytes.len())
            .ok_or(SourceScanError::Resource)?;
        Ok(chunk_start)
    }

    fn finish(self) -> Result<usize, SourceScanError> {
        self.utf8
            .finish()
            .then_some(self.byte_length)
            .ok_or(SourceScanError::InvalidSource)
    }
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

/// Reads one call-local source observation while owning its text policy,
/// incremental digest, and checked byte length. The consumer sees each chunk
/// only after it has passed UTF-8 and NUL validation.
pub(crate) fn observe_source(
    reader: &mut impl Read,
    mut on_chunk: impl FnMut(&[u8], usize) -> Result<(), SourceScanError>,
) -> Result<CurrentObservation, SourceScanError> {
    let mut observation = ObservationBuilder::new()?;
    let mut scratch = [0_u8; READ_BUFFER_SIZE];
    loop {
        let count = reader
            .read(&mut scratch)
            .map_err(|_| SourceScanError::Read)?;
        if count == 0 {
            return observation.finish();
        }
        let bytes = &scratch[..count];
        let chunk_start = observation.push(bytes)?;
        on_chunk(bytes, chunk_start)?;
    }
}

/// Validates and consumes exactly one trusted source length without hashing.
/// One final byte is requested only to reject growth beyond the proof.
pub(crate) fn validate_source_exact(
    reader: &mut impl Read,
    expected_length: usize,
    mut on_chunk: impl FnMut(&[u8], usize) -> Result<(), SourceScanError>,
) -> Result<(), SourceScanError> {
    let mut source = SourceTextBuilder::default();
    let mut scratch = [0_u8; READ_BUFFER_SIZE];
    while source.byte_length < expected_length {
        let remaining = expected_length
            .checked_sub(source.byte_length)
            .ok_or(SourceScanError::Resource)?;
        let capacity = remaining.min(scratch.len());
        let count = reader
            .read(&mut scratch[..capacity])
            .map_err(|_| SourceScanError::Read)?;
        if count == 0 {
            return Err(SourceScanError::InvalidSource);
        }
        let bytes = &scratch[..count];
        let chunk_start = source.push(bytes)?;
        on_chunk(bytes, chunk_start)?;
    }
    let mut extra = [0_u8; 1];
    if reader.read(&mut extra).map_err(|_| SourceScanError::Read)? != 0 {
        return Err(SourceScanError::InvalidSource);
    }
    let actual = source.finish()?;
    (actual == expected_length)
        .then_some(())
        .ok_or(SourceScanError::InvalidSource)
}

#[cfg(test)]
mod tests {
    use std::io::{self, Read};

    use super::{READ_BUFFER_SIZE, SourceScanError, validate_source_exact};

    struct CountingReader {
        bytes: Vec<u8>,
        position: usize,
        requested: usize,
        returned: usize,
        fail_at: Option<usize>,
    }

    impl CountingReader {
        fn new(bytes: Vec<u8>) -> Self {
            Self {
                bytes,
                position: 0,
                requested: 0,
                returned: 0,
                fail_at: None,
            }
        }
    }

    impl Read for CountingReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            self.requested += buffer.len();
            if self.fail_at == Some(self.position) {
                return Err(io::Error::other("injected read failure"));
            }
            let count = buffer.len().min(self.bytes.len() - self.position);
            buffer[..count].copy_from_slice(&self.bytes[self.position..self.position + count]);
            self.position += count;
            self.returned += count;
            Ok(count)
        }
    }

    #[test]
    fn exact_validation_uses_fixed_chunks_and_one_growth_byte_without_hashing() {
        let expected = READ_BUFFER_SIZE + 1;
        let mut exact = CountingReader::new(vec![b'a'; expected]);
        let mut chunks = Vec::new();
        validate_source_exact(&mut exact, expected, |bytes, start| {
            chunks.push((start, bytes.len()));
            Ok(())
        })
        .unwrap();
        assert_eq!(chunks, [(0, READ_BUFFER_SIZE), (READ_BUFFER_SIZE, 1)]);
        assert_eq!(exact.returned, expected);
        assert_eq!(exact.requested, expected + 1);

        let mut grown = CountingReader::new(vec![b'a'; 1024 * 1024]);
        assert_eq!(
            validate_source_exact(&mut grown, expected, |_, _| Ok(())),
            Err(SourceScanError::InvalidSource)
        );
        assert_eq!(grown.returned, expected + 1);
        assert_eq!(grown.requested, expected + 1);
    }

    #[test]
    fn exact_validation_rejects_short_invalid_and_failed_reads() {
        let mut short = CountingReader::new(b"short".to_vec());
        assert_eq!(
            validate_source_exact(&mut short, 6, |_, _| Ok(())),
            Err(SourceScanError::InvalidSource)
        );

        for bytes in [b"nul\0".to_vec(), b"invalid\xff".to_vec()] {
            let length = bytes.len();
            let mut invalid = CountingReader::new(bytes);
            assert_eq!(
                validate_source_exact(&mut invalid, length, |_, _| Ok(())),
                Err(SourceScanError::InvalidSource)
            );
        }

        let mut failed = CountingReader::new(b"exact".to_vec());
        failed.fail_at = Some(5);
        assert_eq!(
            validate_source_exact(&mut failed, 5, |_, _| Ok(())),
            Err(SourceScanError::Read)
        );
        assert_eq!(failed.returned, 5);
    }
}
