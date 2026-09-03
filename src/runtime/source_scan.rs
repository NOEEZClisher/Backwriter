//! Private one-pass source observation and exact target projection.

use std::io::Read;

use crate::backwriter::anddress::{Anddress, AnddressTarget};
use crate::hash::Sha256;

use super::structural_cursor::{LineSpan, StructuralCursor, StructuralSink};

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
    pub(crate) line_count: usize,
}

/// Tracks exact current target evidence without retaining source bytes.
pub(crate) struct TargetProjection<'a> {
    inputs: &'a [Anddress],
    indexes: &'a [usize],
    current: Vec<bool>,
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
        })
    }

    pub(crate) fn finish(&mut self, state: &CurrentObservation) {
        for &index in self.indexes {
            let input = &self.inputs[index];
            let source_matches = input.source_byte_length() == state.byte_length
                && input.source_line_count() == state.line_count
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
}

impl StructuralSink for TargetProjection<'_> {
    fn line(&mut self, line: LineSpan) -> Result<(), SourceScanError> {
        for &index in self.indexes {
            let input = &self.inputs[index];
            if input.target() == AnddressTarget::Line
                && input.byte_start() == line.byte_start
                && input.byte_end() == line.byte_end
            {
                self.current[index] = true;
            }
        }
        Ok(())
    }

    fn paragraph(
        &mut self,
        paragraph: crate::backwriter::anddress::ParagraphGeometry,
    ) -> Result<(), SourceScanError> {
        for &index in self.indexes {
            let input = &self.inputs[index];
            if input.target() == AnddressTarget::Paragraph
                && input.byte_start() == paragraph.byte_start
                && input.byte_end() == paragraph.byte_end
            {
                self.current[index] = true;
            }
        }
        Ok(())
    }
}

#[derive(Default)]
struct Utf8Validator {
    incomplete: [u8; 4],
    length: usize,
}

/// Incremental source-policy and exact-state accumulator shared by retained
/// source observation and generated Apply output.
pub(crate) struct ObservationBuilder {
    utf8: Utf8Validator,
    cursor: StructuralCursor,
    hash: Sha256,
}

impl ObservationBuilder {
    pub(crate) fn new() -> Result<Self, SourceScanError> {
        Ok(Self {
            utf8: Utf8Validator::default(),
            cursor: StructuralCursor::default(),
            hash: Sha256::new(),
        })
    }

    pub(crate) fn byte_offset(&self) -> usize {
        self.cursor.byte_offset()
    }

    pub(crate) fn push_structural(
        &mut self,
        bytes: &[u8],
        sink: &mut impl StructuralSink,
    ) -> Result<usize, SourceScanError> {
        if !self.utf8.push(bytes) {
            return Err(SourceScanError::InvalidSource);
        }
        let chunk_start = self.cursor.byte_offset();
        self.hash.update(bytes);
        sink.source(bytes, chunk_start)?;
        self.cursor.push(bytes, sink)?;
        Ok(chunk_start)
    }

    pub(crate) fn finish_structural(
        self,
        sink: &mut impl StructuralSink,
    ) -> Result<CurrentObservation, SourceScanError> {
        if !self.utf8.finish() {
            return Err(SourceScanError::InvalidSource);
        }
        let (byte_length, line_count) = self.cursor.finish(sink)?;
        Ok(CurrentObservation {
            hash: self.hash.finish().to_hex(),
            byte_length,
            line_count,
        })
    }
}

struct EmptySink;

impl StructuralSink for EmptySink {}

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
    on_chunk: impl FnMut(&[u8], usize) -> Result<(), SourceScanError>,
) -> Result<CurrentObservation, SourceScanError> {
    let mut sink = ChunkSink(on_chunk);
    observe_structural(reader, &mut sink)
}

pub(crate) fn observe_structural(
    reader: &mut impl Read,
    sink: &mut impl StructuralSink,
) -> Result<CurrentObservation, SourceScanError> {
    let mut observation = ObservationBuilder::new()?;
    let mut scratch = [0_u8; READ_BUFFER_SIZE];
    loop {
        let count = reader
            .read(&mut scratch)
            .map_err(|_| SourceScanError::Read)?;
        if count == 0 {
            return observation.finish_structural(sink);
        }
        let bytes = &scratch[..count];
        observation.push_structural(bytes, sink)?;
    }
}

struct ChunkSink<F>(F);

impl<F> StructuralSink for ChunkSink<F>
where
    F: FnMut(&[u8], usize) -> Result<(), SourceScanError>,
{
    fn source(&mut self, bytes: &[u8], byte_start: usize) -> Result<(), SourceScanError> {
        (self.0)(bytes, byte_start)
    }
}

/// Validates and consumes exactly one trusted source length without hashing.
/// One final byte is requested only to reject growth beyond the proof.
pub(crate) fn validate_source_exact(
    reader: &mut impl Read,
    expected_length: usize,
    mut on_chunk: impl FnMut(&[u8], usize) -> Result<(), SourceScanError>,
) -> Result<(), SourceScanError> {
    let mut utf8 = Utf8Validator::default();
    let mut cursor = StructuralCursor::default();
    let mut sink = EmptySink;
    let mut scratch = [0_u8; READ_BUFFER_SIZE];
    while cursor.byte_offset() < expected_length {
        let remaining = expected_length
            .checked_sub(cursor.byte_offset())
            .ok_or(SourceScanError::Resource)?;
        let capacity = remaining.min(scratch.len());
        let count = reader
            .read(&mut scratch[..capacity])
            .map_err(|_| SourceScanError::Read)?;
        if count == 0 {
            return Err(SourceScanError::InvalidSource);
        }
        let bytes = &scratch[..count];
        if !utf8.push(bytes) {
            return Err(SourceScanError::InvalidSource);
        }
        let chunk_start = cursor.push(bytes, &mut sink)?;
        on_chunk(bytes, chunk_start)?;
    }
    let mut extra = [0_u8; 1];
    if reader.read(&mut extra).map_err(|_| SourceScanError::Read)? != 0 {
        return Err(SourceScanError::InvalidSource);
    }
    if !utf8.finish() {
        return Err(SourceScanError::InvalidSource);
    }
    let (actual, _) = cursor.finish(&mut sink)?;
    (actual == expected_length)
        .then_some(())
        .ok_or(SourceScanError::InvalidSource)
}

#[cfg(test)]
mod tests {
    use std::io::{self, Read};

    use super::{
        EmptySink, ObservationBuilder, READ_BUFFER_SIZE, SourceScanError, validate_source_exact,
    };

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

    #[test]
    fn observation_counts_cr_lf_crlf_and_no_eol_across_chunks() {
        for (chunks, expected_length, expected_lines) in [
            (vec![b"".as_slice()], 0, 0),
            (vec![b"\n".as_slice()], 1, 1),
            (vec![b"\r".as_slice(), b"\n".as_slice()], 2, 1),
            (
                vec![
                    b"one\r".as_slice(),
                    b"\n\n \t\r".as_slice(),
                    b"last".as_slice(),
                ],
                13,
                4,
            ),
        ] {
            let mut observation = ObservationBuilder::new().unwrap();
            let mut sink = EmptySink;
            for chunk in chunks {
                observation.push_structural(chunk, &mut sink).unwrap();
            }
            let state = observation.finish_structural(&mut sink).unwrap();
            assert_eq!(state.byte_length, expected_length);
            assert_eq!(state.line_count, expected_lines);
        }
    }
}
