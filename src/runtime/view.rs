//! Runtime binding and exact View execution.

use std::io::{Read, Seek, SeekFrom};

use crate::backwriter::anddress::{Anddress, AnddressTarget, LineTerminator, construct_anddress};
use crate::backwriter::view::{ViewError, ViewOutcome, validate_input};

use super::{
    CurrentProofMatch, WorkspaceRuntime, is_backwriter_spill,
    source_scan::{READ_BUFFER_SIZE, SourceScanError, TargetProjection, observe_source},
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
    match runtime.match_current_proof(input) {
        CurrentProofMatch::Missing => {
            let mut file = runtime
                .open_admitted_source(input.logical_path())
                .map_err(|_| ViewError::Unavailable)?;
            observe_direct(&mut file, input)
                .map_err(|_| ViewError::Unavailable)?
                .ok_or(ViewError::Unavailable)
        }
        CurrentProofMatch::Mismatched => Err(ViewError::Unavailable),
        CurrentProofMatch::Matching => {
            let outcome = runtime
                .open_admitted_source(input.logical_path())
                .map_err(|_| TrustedViewError::Source)
                .and_then(|mut file| observe_trusted(&mut file, input));
            if matches!(
                outcome,
                Err(TrustedViewError::Source | TrustedViewError::Resource)
            ) {
                runtime.invalidate_current_proof(input.logical_path());
            }
            outcome.map_err(|_| ViewError::Unavailable)
        }
    }
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
        let related = self.line_relation.take().and_then(|mut relation| {
            relation.finish(source_byte_length);
            relation.related
        });
        finish_outcome(self.input, self.target, related)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TrustedViewError {
    Source,
    InvalidRange,
    Resource,
}

fn observe_trusted(
    reader: &mut (impl Read + Seek),
    input: &Anddress,
) -> Result<ViewOutcome, TrustedViewError> {
    let target = read_range(reader, input.byte_start(), input.byte_end())?;
    let related = if input.target() == AnddressTarget::Line {
        related_paragraph_range(reader, input, &target)?
    } else {
        None
    };
    finish_outcome(input, target, related).map_err(map_trusted_scan_error)
}

fn read_range(
    reader: &mut (impl Read + Seek),
    start: usize,
    end: usize,
) -> Result<Vec<u8>, TrustedViewError> {
    let length = end.checked_sub(start).ok_or(TrustedViewError::Resource)?;
    let mut output = Vec::new();
    output
        .try_reserve_exact(length)
        .map_err(|_| TrustedViewError::Resource)?;
    output.resize(length, 0);
    if length == 0 {
        return Ok(output);
    }
    seek_to(reader, start)?;
    read_fully(reader, &mut output)?;
    Ok(output)
}

fn related_paragraph_range(
    reader: &mut (impl Read + Seek),
    input: &Anddress,
    target: &[u8],
) -> Result<Option<(usize, usize)>, TrustedViewError> {
    if !is_exact_text_line(reader, input, target)? {
        return Ok(None);
    }
    let start = scan_paragraph_start(reader, input.byte_start())?;
    let end = scan_paragraph_end(reader, input.byte_end(), input.source_byte_length())?;
    Ok(Some((start, end)))
}

fn is_exact_text_line(
    reader: &mut (impl Read + Seek),
    input: &Anddress,
    target: &[u8],
) -> Result<bool, TrustedViewError> {
    if target.is_empty() {
        return Ok(false);
    }
    if input.byte_start() != 0 {
        match read_byte_at(reader, input.byte_start() - 1)? {
            b'\n' => {}
            b'\r' if target.first() != Some(&b'\n') => {}
            _ => return Ok(false),
        }
    }

    let first_terminator = target.iter().position(|byte| matches!(byte, b'\r' | b'\n'));
    let body_end = match first_terminator {
        None if input.byte_end() == input.source_byte_length() => target.len(),
        None => return Ok(false),
        Some(index) if target[index] == b'\n' => {
            if index + 1 != target.len() {
                return Ok(false);
            }
            index
        }
        Some(index) => {
            let terminator_length = if target.get(index + 1) == Some(&b'\n') {
                2
            } else {
                1
            };
            if index + terminator_length != target.len() {
                return Ok(false);
            }
            if terminator_length == 1
                && input.byte_end() < input.source_byte_length()
                && read_byte_at(reader, input.byte_end())? == b'\n'
            {
                return Ok(false);
            }
            index
        }
    };
    Ok(target[..body_end]
        .iter()
        .any(|byte| !matches!(byte, b' ' | b'\t')))
}

fn scan_paragraph_start(
    reader: &mut (impl Read + Seek),
    target_start: usize,
) -> Result<usize, TrustedViewError> {
    let mut cursor = ReverseBytes::new(reader, target_start);
    let mut paragraph_start = target_start;
    while cursor.position() != 0 {
        match cursor.next()? {
            Some(b'\n') => {
                if cursor.peek()? == Some(b'\r') {
                    cursor.next()?;
                }
            }
            Some(b'\r') => {}
            Some(_) | None => return Err(TrustedViewError::Source),
        }
        let mut has_text = false;
        while let Some(byte) = cursor.peek()? {
            if matches!(byte, b'\r' | b'\n') {
                break;
            }
            cursor.next()?;
            has_text |= !matches!(byte, b' ' | b'\t');
        }
        if !has_text {
            break;
        }
        paragraph_start = cursor.position();
    }
    Ok(paragraph_start)
}

fn scan_paragraph_end(
    reader: &mut (impl Read + Seek),
    target_end: usize,
    source_end: usize,
) -> Result<usize, TrustedViewError> {
    let mut cursor = ForwardBytes::new(reader, target_end, source_end)?;
    let mut paragraph_end = target_end;
    while cursor.position() < source_end {
        let mut has_text = false;
        loop {
            match cursor.next()? {
                Some(b'\r') => {
                    if cursor.peek()? == Some(b'\n') {
                        cursor.next()?;
                    }
                    break;
                }
                Some(b'\n') | None => break,
                Some(byte) => has_text |= !matches!(byte, b' ' | b'\t'),
            }
        }
        if !has_text {
            break;
        }
        paragraph_end = cursor.position();
    }
    Ok(paragraph_end)
}

struct ReverseBytes<'a, R> {
    reader: &'a mut R,
    scratch: [u8; READ_BUFFER_SIZE],
    buffer_start: usize,
    buffer_end: usize,
    position: usize,
}

impl<'a, R: Read + Seek> ReverseBytes<'a, R> {
    fn new(reader: &'a mut R, position: usize) -> Self {
        Self {
            reader,
            scratch: [0; READ_BUFFER_SIZE],
            buffer_start: position,
            buffer_end: position,
            position,
        }
    }

    fn position(&self) -> usize {
        self.position
    }

    fn peek(&mut self) -> Result<Option<u8>, TrustedViewError> {
        if self.position == 0 {
            return Ok(None);
        }
        let byte_offset = self.position - 1;
        if byte_offset < self.buffer_start || byte_offset >= self.buffer_end {
            self.buffer_end = self.position;
            self.buffer_start = self.buffer_end.saturating_sub(READ_BUFFER_SIZE);
            seek_to(self.reader, self.buffer_start)?;
            read_fully(
                self.reader,
                &mut self.scratch[..self.buffer_end - self.buffer_start],
            )?;
        }
        Ok(Some(self.scratch[byte_offset - self.buffer_start]))
    }

    fn next(&mut self) -> Result<Option<u8>, TrustedViewError> {
        let byte = self.peek()?;
        if byte.is_some() {
            self.position -= 1;
        }
        Ok(byte)
    }
}

struct ForwardBytes<'a, R> {
    reader: &'a mut R,
    scratch: [u8; READ_BUFFER_SIZE],
    source_end: usize,
    position: usize,
    buffer_length: usize,
    buffer_cursor: usize,
}

impl<'a, R: Read + Seek> ForwardBytes<'a, R> {
    fn new(
        reader: &'a mut R,
        position: usize,
        source_end: usize,
    ) -> Result<Self, TrustedViewError> {
        seek_to(reader, position)?;
        Ok(Self {
            reader,
            scratch: [0; READ_BUFFER_SIZE],
            source_end,
            position,
            buffer_length: 0,
            buffer_cursor: 0,
        })
    }

    fn position(&self) -> usize {
        self.position
    }

    fn peek(&mut self) -> Result<Option<u8>, TrustedViewError> {
        if self.position == self.source_end {
            return Ok(None);
        }
        if self.buffer_cursor == self.buffer_length {
            self.buffer_length = (self.source_end - self.position).min(READ_BUFFER_SIZE);
            self.buffer_cursor = 0;
            read_fully(self.reader, &mut self.scratch[..self.buffer_length])?;
        }
        Ok(Some(self.scratch[self.buffer_cursor]))
    }

    fn next(&mut self) -> Result<Option<u8>, TrustedViewError> {
        let byte = self.peek()?;
        if byte.is_some() {
            self.buffer_cursor += 1;
            self.position += 1;
        }
        Ok(byte)
    }
}

fn read_byte_at(reader: &mut (impl Read + Seek), offset: usize) -> Result<u8, TrustedViewError> {
    seek_to(reader, offset)?;
    let mut byte = [0];
    read_fully(reader, &mut byte)?;
    Ok(byte[0])
}

fn seek_to(reader: &mut impl Seek, offset: usize) -> Result<(), TrustedViewError> {
    let offset = u64::try_from(offset).map_err(|_| TrustedViewError::Resource)?;
    reader
        .seek(SeekFrom::Start(offset))
        .map(|_| ())
        .map_err(|_| TrustedViewError::Source)
}

fn read_fully(reader: &mut impl Read, mut output: &mut [u8]) -> Result<(), TrustedViewError> {
    while !output.is_empty() {
        match reader.read(output) {
            Ok(0) => return Err(TrustedViewError::Source),
            Ok(count) => output = &mut output[count..],
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
            Err(_) => return Err(TrustedViewError::Source),
        }
    }
    Ok(())
}

fn map_trusted_scan_error(error: SourceScanError) -> TrustedViewError {
    match error {
        SourceScanError::Read => TrustedViewError::Source,
        SourceScanError::InvalidSource => TrustedViewError::InvalidRange,
        SourceScanError::Resource => TrustedViewError::Resource,
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

fn finish_outcome(
    input: &Anddress,
    target: Vec<u8>,
    related: Option<(usize, usize)>,
) -> Result<ViewOutcome, SourceScanError> {
    match input.target() {
        AnddressTarget::File => Ok(ViewOutcome::File {
            text: String::from_utf8(target).map_err(|_| SourceScanError::InvalidSource)?,
        }),
        AnddressTarget::Paragraph => Ok(ViewOutcome::Paragraph {
            text: String::from_utf8(target).map_err(|_| SourceScanError::InvalidSource)?,
            file: file_address(input)?,
        }),
        AnddressTarget::Line => {
            let (content, terminator) = line_parts(target)?;
            Ok(ViewOutcome::Line {
                content,
                terminator,
                file: file_address(input)?,
                paragraph: related
                    .map(|(start, end)| paragraph_address(input, start, end))
                    .transpose()?,
            })
        }
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
    use std::{
        io::{self, Cursor, Read, Seek, SeekFrom},
        path::Path,
    };

    use crate::hash::Sha256;
    use crate::runtime::{
        AdmissionRoot, CurrentProof, WorkspaceAdmission, WorkspaceRuntime,
        source_scan::READ_BUFFER_SIZE,
    };

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

    struct CountingCursor {
        inner: Cursor<Vec<u8>>,
        bytes_read: usize,
        seeks: usize,
    }

    impl CountingCursor {
        fn new(bytes: Vec<u8>) -> Self {
            Self {
                inner: Cursor::new(bytes),
                bytes_read: 0,
                seeks: 0,
            }
        }
    }

    impl Read for CountingCursor {
        fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
            let count = self.inner.read(output)?;
            self.bytes_read += count;
            Ok(count)
        }
    }

    impl Seek for CountingCursor {
        fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
            self.seeks += 1;
            self.inner.seek(position)
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

        let execute = production
            .split("pub(super) fn execute")
            .nth(1)
            .unwrap()
            .split("pub(super) fn observe_direct")
            .next()
            .unwrap();
        assert!(execute.contains("CurrentProofMatch::Missing"));
        assert!(execute.contains("observe_direct("));
        assert!(execute.contains("CurrentProofMatch::Matching"));
        assert!(execute.contains("observe_trusted("));
        let trusted = production
            .split("fn observe_trusted")
            .nth(1)
            .unwrap()
            .split("fn read_range")
            .next()
            .unwrap();
        assert!(!trusted.contains("observe_source("));
        assert!(!trusted.contains("Sha256"));
    }

    #[test]
    fn trusted_line_reads_only_its_range_and_nearest_paragraph_boundaries() {
        let mut bytes = vec![b'p'; READ_BUFFER_SIZE * 16];
        bytes.extend_from_slice(b"\n \t\r\n");
        let target_start = bytes.len();
        bytes.extend_from_slice("한글\r\n".as_bytes());
        let target_end = bytes.len();
        bytes.extend_from_slice("β\n".as_bytes());
        let paragraph_end = bytes.len();
        bytes.extend_from_slice(b"\t\n");
        bytes.extend(std::iter::repeat_n(b's', READ_BUFFER_SIZE * 16));
        let input = address(&bytes, AnddressTarget::Line, target_start, target_end);
        let mut reader = CountingCursor::new(bytes.clone());

        let outcome = observe_trusted(&mut reader, &input).unwrap();

        assert_eq!(
            outcome,
            ViewOutcome::Line {
                content: "한글".to_owned(),
                terminator: LineTerminator::Crlf,
                file: address(&bytes, AnddressTarget::File, 0, bytes.len()),
                paragraph: Some(address(
                    &bytes,
                    AnddressTarget::Paragraph,
                    target_start,
                    paragraph_end,
                )),
            }
        );
        assert!(reader.bytes_read <= target_end - target_start + 2 * READ_BUFFER_SIZE + 2);
        assert!(reader.bytes_read < bytes.len() / 4);
        assert!(reader.seeks >= 3);
    }

    #[test]
    fn trusted_line_preserves_terminators_unicode_and_nonstructural_relations() {
        for (extent, content, terminator) in [
            ("한글\n", "한글", LineTerminator::Lf),
            ("한글\r", "한글", LineTerminator::Cr),
            ("한글\r\n", "한글", LineTerminator::Crlf),
            ("한글", "한글", LineTerminator::None),
        ] {
            let source = format!("\n{extent}");
            let start = 1;
            let end = source.len();
            let input = address(source.as_bytes(), AnddressTarget::Line, start, end);
            assert!(matches!(
                observe_trusted(&mut Cursor::new(source.as_bytes()), &input).unwrap(),
                ViewOutcome::Line {
                    content: actual,
                    terminator: actual_terminator,
                    paragraph: Some(paragraph),
                    ..
                } if actual == content
                    && actual_terminator == terminator
                    && paragraph.byte_start() == start
                    && paragraph.byte_end() == end
            ));
        }

        let whitespace = b"one\n \t\r\ntwo";
        let whitespace_input = address(whitespace, AnddressTarget::Line, 4, 8);
        assert!(matches!(
            observe_trusted(&mut Cursor::new(whitespace), &whitespace_input).unwrap(),
            ViewOutcome::Line {
                paragraph: None,
                ..
            }
        ));
        let raw = b"zero\none\r\ntwo";
        let raw_input = address(raw, AnddressTarget::Line, 2, 10);
        assert!(matches!(
            observe_trusted(&mut Cursor::new(raw), &raw_input).unwrap(),
            ViewOutcome::Line {
                paragraph: None,
                ..
            }
        ));
    }

    #[test]
    fn trusted_relation_crosses_fixed_scratch_in_both_directions() {
        let mut source = b"\n".to_vec();
        let paragraph_start = source.len();
        source.extend(std::iter::repeat_n(b'a', READ_BUFFER_SIZE + 1));
        source.extend_from_slice(b"\r\n");
        let target_start = source.len();
        source.extend_from_slice(b"needle\n");
        let target_end = source.len();
        source.extend(std::iter::repeat_n(b'b', READ_BUFFER_SIZE - 1));
        source.extend_from_slice(b"\r");
        let paragraph_end = source.len();
        source.extend_from_slice(b" \t\n");
        let input = address(&source, AnddressTarget::Line, target_start, target_end);

        assert!(matches!(
            observe_trusted(&mut Cursor::new(&source), &input).unwrap(),
            ViewOutcome::Line {
                content,
                terminator: LineTerminator::Lf,
                paragraph: Some(paragraph),
                ..
            } if content == "needle"
                && paragraph.byte_start() == paragraph_start
                && paragraph.byte_end() == paragraph_end
        ));
    }

    #[test]
    fn trusted_and_fallback_projection_match_for_every_scalar_aligned_range() {
        let source = "α\r\n \t\nb\rc\n\n끝";
        let mut boundaries: Vec<_> = source.char_indices().map(|(index, _)| index).collect();
        boundaries.push(source.len());
        for target in [AnddressTarget::Paragraph, AnddressTarget::Line] {
            for (start_index, &start) in boundaries.iter().enumerate() {
                for &end in &boundaries[start_index..] {
                    let input = address(source.as_bytes(), target, start, end);
                    let expected = observe_direct(&mut Cursor::new(source.as_bytes()), &input)
                        .unwrap()
                        .unwrap();
                    let actual =
                        observe_trusted(&mut Cursor::new(source.as_bytes()), &input).unwrap();
                    assert_eq!(actual, expected, "{target:?} {start}..{end}");
                }
            }
        }
    }

    #[test]
    fn trusted_short_read_and_matching_open_failure_remove_only_matching_proof() {
        let input = address(b"one", AnddressTarget::File, 0, 3);
        assert_eq!(
            observe_trusted(&mut Cursor::new(b"on"), &input),
            Err(TrustedViewError::Source)
        );

        let fixture = tempfile::tempdir().unwrap();
        let runtime = host_runtime(fixture.path());
        let input = Anddress::new(
            &runtime.workspace_coordinate,
            "missing.txt",
            &"b".repeat(64),
            1,
            AnddressTarget::File,
            0,
            1,
        )
        .unwrap();
        runtime
            .install_search_proofs(vec![
                CurrentProof::new("missing.txt", "a".repeat(64), 1).unwrap(),
            ])
            .unwrap();

        assert_eq!(runtime.view(&input), Err(ViewError::Unavailable));
        assert_eq!(runtime.current_proofs.lock().unwrap().len(), 1);

        let matching = Anddress::new(
            &runtime.workspace_coordinate,
            "missing.txt",
            &"a".repeat(64),
            1,
            AnddressTarget::File,
            0,
            1,
        )
        .unwrap();
        assert_eq!(runtime.view(&matching), Err(ViewError::Unavailable));
        assert!(runtime.current_proofs.lock().unwrap().is_empty());

        std::fs::write(fixture.path().join("short.txt"), b"on").unwrap();
        let short = Anddress::new(
            &runtime.workspace_coordinate,
            "short.txt",
            &"c".repeat(64),
            3,
            AnddressTarget::File,
            0,
            3,
        )
        .unwrap();
        runtime
            .install_search_proofs(vec![
                CurrentProof::new("short.txt", "c".repeat(64), 3).unwrap(),
            ])
            .unwrap();
        assert_eq!(runtime.view(&short), Err(ViewError::Unavailable));
        assert!(runtime.current_proofs.lock().unwrap().is_empty());

        let unicode = "aéz".as_bytes();
        std::fs::write(fixture.path().join("cut.txt"), unicode).unwrap();
        let file = address(unicode, AnddressTarget::File, 0, unicode.len());
        let cut = Anddress::new(
            &runtime.workspace_coordinate,
            "cut.txt",
            file.source_state_hash(),
            unicode.len(),
            AnddressTarget::Line,
            2,
            3,
        )
        .unwrap();
        runtime
            .install_search_proofs(vec![
                CurrentProof::new("cut.txt", cut.source_state_hash().to_owned(), unicode.len())
                    .unwrap(),
            ])
            .unwrap();
        assert_eq!(runtime.view(&cut), Err(ViewError::Unavailable));
        assert_eq!(runtime.current_proofs.lock().unwrap().len(), 1);
    }

    fn host_runtime(root: &Path) -> WorkspaceRuntime {
        WorkspaceRuntime::open_host_authoritative(
            root,
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap()
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
