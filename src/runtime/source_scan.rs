//! Private one-pass source observation and exact target projection.

use std::io::Read;

use crate::backwriter::anddress::{Anddress, AnddressTarget};
use crate::hash::Sha256;

use super::structural_cursor::{LineSpan, StructuralCursor, StructuralSink};

pub(crate) const READ_BUFFER_SIZE: usize = 8_192;
const BYTE_LOW_BITS: u64 = 0x7f7f_7f7f_7f7f_7f7f;
const BYTE_ONE_BITS: u64 = 0x0101_0101_0101_0101;
const BYTE_HIGH_BITS: u64 = 0x8080_8080_8080_8080;

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

/// Incremental source-policy and exact-state accumulator shared by every raw
/// source observation and generated Apply output.
pub(crate) struct ObservationBuilder {
    utf8: Utf8Validator,
    hash: Sha256,
    byte_length: usize,
    line_count: usize,
    trailing_cr: bool,
    ends_with_terminator: bool,
}

impl ObservationBuilder {
    pub(crate) fn new() -> Result<Self, SourceScanError> {
        Ok(Self {
            utf8: Utf8Validator::default(),
            hash: Sha256::new(),
            byte_length: 0,
            line_count: 0,
            trailing_cr: false,
            ends_with_terminator: false,
        })
    }

    pub(crate) fn byte_offset(&self) -> usize {
        self.byte_length
    }

    pub(crate) fn push(
        &mut self,
        bytes: &[u8],
        mut on_chunk: impl FnMut(&[u8], usize) -> Result<(), SourceScanError>,
    ) -> Result<usize, SourceScanError> {
        let chunk_start = self.byte_length;
        let byte_length = self
            .byte_length
            .checked_add(bytes.len())
            .ok_or(SourceScanError::Resource)?;
        let text = scan_text_chunk(bytes, self.trailing_cr);
        let line_count = self
            .line_count
            .checked_add(text.line_breaks)
            .ok_or(SourceScanError::Resource)?;
        let valid = if text.ascii_without_nul && self.utf8.length == 0 {
            true
        } else {
            self.utf8.push(bytes)
        };
        if !valid {
            return Err(SourceScanError::InvalidSource);
        }
        self.hash.update(bytes);
        on_chunk(bytes, chunk_start)?;
        self.byte_length = byte_length;
        self.line_count = line_count;
        if let Some(&last) = bytes.last() {
            self.trailing_cr = last == b'\r';
            self.ends_with_terminator = matches!(last, b'\r' | b'\n');
        }
        Ok(chunk_start)
    }

    pub(crate) fn finish(self) -> Result<CurrentObservation, SourceScanError> {
        if !self.utf8.finish() {
            return Err(SourceScanError::InvalidSource);
        }
        let line_count = if self.byte_length != 0 && !self.ends_with_terminator {
            self.line_count
                .checked_add(1)
                .ok_or(SourceScanError::Resource)?
        } else {
            self.line_count
        };
        Ok(CurrentObservation {
            hash: self.hash.finish().to_hex(),
            byte_length: self.byte_length,
            line_count,
        })
    }
}

struct TextChunk {
    line_breaks: usize,
    ascii_without_nul: bool,
}

fn scan_text_chunk(bytes: &[u8], preceding_cr: bool) -> TextChunk {
    let mut blocks = bytes.chunks_exact(128);
    let mut line_breaks = 0;
    let mut cr = 0_u64;
    let mut non_ascii = 0_u64;
    let mut nul = 0_u64;
    for block in &mut blocks {
        let mut terminator_lanes = 0_u64;
        for chunk in block.chunks_exact(8) {
            let word = u64::from_le_bytes(chunk.try_into().expect("fixed word chunk"));
            let compared_cr = word ^ u64::from(b'\r').wrapping_mul(BYTE_ONE_BITS);
            cr |= compared_cr.wrapping_sub(BYTE_ONE_BITS) & !compared_cr & BYTE_HIGH_BITS;
            terminator_lanes += matching_byte_mask(word, b'\n') >> 7;
            non_ascii |= word & BYTE_HIGH_BITS;
            nul |= word.wrapping_sub(BYTE_ONE_BITS) & !word & BYTE_HIGH_BITS;
        }
        line_breaks += ((terminator_lanes.wrapping_mul(BYTE_ONE_BITS)) >> 56) as usize;
    }
    for &byte in blocks.remainder() {
        line_breaks += usize::from(byte == b'\n');
        cr |= u64::from(byte == b'\r');
        non_ascii |= u64::from(byte & 0x80);
        nul |= u64::from(byte == 0);
    }
    line_breaks = if cr == 0 {
        line_breaks - usize::from(preceding_cr && bytes.first() == Some(&b'\n'))
    } else {
        line_break_count_with_cr(bytes, preceding_cr)
    };
    TextChunk {
        line_breaks,
        ascii_without_nul: non_ascii == 0 && nul == 0,
    }
}

fn line_break_count_with_cr(bytes: &[u8], preceding_cr: bool) -> usize {
    let mut line_breaks = 0;
    let mut previous_was_cr = preceding_cr;
    for &byte in bytes {
        match byte {
            b'\r' => {
                line_breaks += 1;
                previous_was_cr = true;
            }
            b'\n' => {
                line_breaks += usize::from(!previous_was_cr);
                previous_was_cr = false;
            }
            _ => previous_was_cr = false,
        }
    }
    line_breaks
}

fn matching_byte_mask(word: u64, byte: u8) -> u64 {
    let compared = word ^ u64::from(byte).wrapping_mul(BYTE_ONE_BITS);
    !(((compared & BYTE_LOW_BITS) + BYTE_LOW_BITS) | compared | BYTE_LOW_BITS)
}

/// Composes raw observation with the sole structural cursor for callers that
/// consume Line or Paragraph geometry.
pub(crate) struct StructuralObservationBuilder {
    observation: ObservationBuilder,
    cursor: StructuralCursor,
}

impl StructuralObservationBuilder {
    pub(crate) fn new() -> Result<Self, SourceScanError> {
        Ok(Self {
            observation: ObservationBuilder::new()?,
            cursor: StructuralCursor::default(),
        })
    }

    pub(crate) fn byte_offset(&self) -> usize {
        debug_assert_eq!(self.observation.byte_offset(), self.cursor.byte_offset());
        self.observation.byte_offset()
    }

    pub(crate) fn push(
        &mut self,
        bytes: &[u8],
        sink: &mut impl StructuralSink,
    ) -> Result<usize, SourceScanError> {
        let chunk_start = self
            .observation
            .push(bytes, |bytes, byte_start| sink.source(bytes, byte_start))?;
        let cursor_start = self.cursor.push(bytes, sink)?;
        if cursor_start != chunk_start {
            return Err(SourceScanError::InvalidSource);
        }
        Ok(chunk_start)
    }

    pub(crate) fn finish(
        self,
        sink: &mut impl StructuralSink,
    ) -> Result<CurrentObservation, SourceScanError> {
        let state = self.observation.finish()?;
        let (byte_length, line_count) = self.cursor.finish(sink)?;
        if byte_length != state.byte_length || line_count != state.line_count {
            return Err(SourceScanError::InvalidSource);
        }
        Ok(state)
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
        observation.push(&scratch[..count], &mut on_chunk)?;
    }
}

pub(crate) fn observe_structural(
    reader: &mut impl Read,
    sink: &mut impl StructuralSink,
) -> Result<CurrentObservation, SourceScanError> {
    let mut observation = StructuralObservationBuilder::new()?;
    let mut scratch = [0_u8; READ_BUFFER_SIZE];
    loop {
        let count = reader
            .read(&mut scratch)
            .map_err(|_| SourceScanError::Read)?;
        if count == 0 {
            return observation.finish(sink);
        }
        let bytes = &scratch[..count];
        observation.push(bytes, sink)?;
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
    let mut byte_offset = 0_usize;
    let mut scratch = [0_u8; READ_BUFFER_SIZE];
    while byte_offset < expected_length {
        let remaining = expected_length
            .checked_sub(byte_offset)
            .ok_or(SourceScanError::Resource)?;
        let capacity = remaining.min(scratch.len());
        let count = reader
            .read(&mut scratch[..capacity])
            .map_err(|_| SourceScanError::Read)?;
        if count == 0 {
            return Err(SourceScanError::InvalidSource);
        }
        let bytes = &scratch[..count];
        let next_offset = byte_offset
            .checked_add(count)
            .ok_or(SourceScanError::Resource)?;
        if !utf8.push(bytes) {
            return Err(SourceScanError::InvalidSource);
        }
        on_chunk(bytes, byte_offset)?;
        byte_offset = next_offset;
    }
    let mut extra = [0_u8; 1];
    if reader.read(&mut extra).map_err(|_| SourceScanError::Read)? != 0 {
        return Err(SourceScanError::InvalidSource);
    }
    if !utf8.finish() {
        return Err(SourceScanError::InvalidSource);
    }
    (byte_offset == expected_length)
        .then_some(())
        .ok_or(SourceScanError::InvalidSource)
}

#[cfg(test)]
mod tests {
    use std::io::{self, Cursor, Read};

    use crate::runtime::structural_cursor::{LineSpan, StructuralSink};

    use super::{
        ObservationBuilder, READ_BUFFER_SIZE, SourceScanError, StructuralObservationBuilder,
        observe_source, observe_structural, scan_text_chunk, validate_source_exact,
    };

    struct CountingReader {
        bytes: Vec<u8>,
        position: usize,
        requested: usize,
        returned: usize,
        fail_at: Option<usize>,
        max_chunk: usize,
    }

    impl CountingReader {
        fn new(bytes: Vec<u8>) -> Self {
            Self {
                bytes,
                position: 0,
                requested: 0,
                returned: 0,
                fail_at: None,
                max_chunk: usize::MAX,
            }
        }

        fn with_max_chunk(mut self, max_chunk: usize) -> Self {
            self.max_chunk = max_chunk;
            self
        }
    }

    impl Read for CountingReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            self.requested += buffer.len();
            if self.fail_at == Some(self.position) {
                return Err(io::Error::other("injected read failure"));
            }
            let count = buffer
                .len()
                .min(self.max_chunk)
                .min(self.bytes.len() - self.position);
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
            for chunk in chunks {
                observation.push(chunk, |_, _| Ok(())).unwrap();
            }
            let state = observation.finish().unwrap();
            assert_eq!(state.byte_length, expected_length);
            assert_eq!(state.line_count, expected_lines);
        }

        for length in 0_u32..=10 {
            for mut encoded in 0..3_u32.pow(length) {
                let mut bytes = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    bytes.push([b'x', b'\r', b'\n'][(encoded % 3) as usize]);
                    encoded /= 3;
                }
                for preceding_cr in [false, true] {
                    let mut expected = 0;
                    let mut previous_was_cr = preceding_cr;
                    for &byte in &bytes {
                        match byte {
                            b'\r' => {
                                expected += 1;
                                previous_was_cr = true;
                            }
                            b'\n' => {
                                expected += usize::from(!previous_was_cr);
                                previous_was_cr = false;
                            }
                            _ => previous_was_cr = false,
                        }
                    }
                    assert_eq!(scan_text_chunk(&bytes, preceding_cr).line_breaks, expected);
                }
            }
        }
    }

    #[derive(Default)]
    struct StructuralCount {
        source_chunks: usize,
        lines: usize,
        paragraphs: usize,
    }

    impl StructuralSink for StructuralCount {
        fn source(&mut self, _bytes: &[u8], _byte_start: usize) -> Result<(), SourceScanError> {
            self.source_chunks += 1;
            Ok(())
        }

        fn line(&mut self, _line: LineSpan) -> Result<(), SourceScanError> {
            self.lines += 1;
            Ok(())
        }

        fn paragraph(
            &mut self,
            _paragraph: crate::backwriter::anddress::ParagraphGeometry,
        ) -> Result<(), SourceScanError> {
            self.paragraphs += 1;
            Ok(())
        }
    }

    #[test]
    fn raw_and_structural_observations_have_exact_state_parity() {
        let mut edge = vec![b'x'; READ_BUFFER_SIZE - 1];
        edge.extend_from_slice("é".as_bytes());
        edge.extend_from_slice(b"\r\n\t \rfinal");
        let mut cases = vec![
            Vec::new(),
            b"\n".to_vec(),
            b"\r".to_vec(),
            b"\r\n".to_vec(),
            b"one\rtwo\nthree\r\nfour".to_vec(),
            edge,
        ];
        for length in [READ_BUFFER_SIZE - 1, READ_BUFFER_SIZE, READ_BUFFER_SIZE + 1] {
            let mut bytes = vec![b'x'; length];
            bytes[length - 2..].copy_from_slice("é".as_bytes());
            cases.push(bytes);
        }
        for bytes in cases {
            for max_chunk in [
                1,
                READ_BUFFER_SIZE - 1,
                READ_BUFFER_SIZE,
                READ_BUFFER_SIZE + 1,
            ] {
                let mut raw_reader = CountingReader::new(bytes.clone()).with_max_chunk(max_chunk);
                let mut raw_chunks = 0;
                let raw = observe_source(&mut raw_reader, |_, _| {
                    raw_chunks += 1;
                    Ok(())
                })
                .unwrap();

                let mut structural_reader =
                    CountingReader::new(bytes.clone()).with_max_chunk(max_chunk);
                let mut structural = StructuralCount::default();
                let state = observe_structural(&mut structural_reader, &mut structural).unwrap();
                assert_eq!(raw, state);
                assert_eq!(structural.lines, state.line_count);
                assert_eq!(structural.source_chunks, raw_chunks);
            }
        }
    }

    #[test]
    fn raw_and_structural_observations_fail_closed_on_invalid_or_late_input() {
        for bytes in [b"valid\nlate\0".to_vec(), b"valid\nlate\xff".to_vec()] {
            let mut raw = CountingReader::new(bytes.clone()).with_max_chunk(1);
            let mut raw_chunks = 0;
            assert_eq!(
                observe_source(&mut raw, |_, _| {
                    raw_chunks += 1;
                    Ok(())
                }),
                Err(SourceScanError::InvalidSource)
            );
            assert!(raw_chunks > 0);

            let mut structural = CountingReader::new(bytes).with_max_chunk(1);
            let mut sink = StructuralCount::default();
            assert_eq!(
                observe_structural(&mut structural, &mut sink),
                Err(SourceScanError::InvalidSource)
            );
        }

        let mut raw = CountingReader::new(b"valid late failure".to_vec()).with_max_chunk(1);
        raw.fail_at = Some(6);
        assert_eq!(
            observe_source(&mut raw, |_, _| Ok(())),
            Err(SourceScanError::Read)
        );
        let mut structural = CountingReader::new(b"valid late failure".to_vec()).with_max_chunk(1);
        structural.fail_at = Some(6);
        assert_eq!(
            observe_structural(&mut structural, &mut StructuralCount::default()),
            Err(SourceScanError::Read)
        );

        let mut rejected = CountingReader::new(vec![b'x'; READ_BUFFER_SIZE + 1]);
        let mut accepted_chunks = 0;
        assert_eq!(
            observe_source(&mut rejected, |_, start| {
                if start == 0 {
                    accepted_chunks += 1;
                    Ok(())
                } else {
                    Err(SourceScanError::Resource)
                }
            }),
            Err(SourceScanError::Resource)
        );
        assert_eq!(accepted_chunks, 1);
    }

    #[test]
    fn raw_checked_state_fails_before_callback_and_structural_cursor_is_singular() {
        let mut callback_count = 0;
        let mut length_overflow = ObservationBuilder::new().unwrap();
        length_overflow.byte_length = usize::MAX;
        assert_eq!(
            length_overflow.push(b"x", |_, _| {
                callback_count += 1;
                Ok(())
            }),
            Err(SourceScanError::Resource)
        );
        assert_eq!(callback_count, 0);

        let mut finish_overflow = ObservationBuilder::new().unwrap();
        finish_overflow.line_count = usize::MAX;
        finish_overflow
            .push(b"x", |_, _| Ok(()))
            .expect("the final unterminated Line owns the overflow");
        assert_eq!(finish_overflow.finish(), Err(SourceScanError::Resource));

        let mut count_overflow = ObservationBuilder::new().unwrap();
        count_overflow.line_count = usize::MAX;
        assert_eq!(
            count_overflow.push(b"\n", |_, _| {
                callback_count += 1;
                Ok(())
            }),
            Err(SourceScanError::Resource)
        );
        assert_eq!(callback_count, 0);

        let production = include_str!("source_scan.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        let raw_builder = production
            .split("pub(crate) struct ObservationBuilder")
            .nth(1)
            .unwrap()
            .split("pub(crate) struct StructuralObservationBuilder")
            .next()
            .unwrap();
        assert!(!raw_builder.contains("StructuralCursor"));
        let raw_observer = production
            .split("pub(crate) fn observe_source")
            .nth(1)
            .unwrap()
            .split("pub(crate) fn observe_structural")
            .next()
            .unwrap();
        assert!(!raw_observer.contains("observe_structural"));
        assert!(!raw_observer.contains("StructuralCursor"));
        let exact = production
            .split("pub(crate) fn validate_source_exact")
            .nth(1)
            .unwrap();
        assert!(!exact.contains("StructuralCursor"));
        assert_eq!(production.matches("cursor: StructuralCursor,").count(), 1);
        assert_eq!(production.matches("StructuralCursor::default()").count(), 1);

        let mut empty = Cursor::new(Vec::<u8>::new());
        assert_eq!(
            observe_source(&mut empty, |_, _| Ok(()))
                .unwrap()
                .line_count,
            0
        );
        assert!(StructuralObservationBuilder::new().is_ok());
    }
}
