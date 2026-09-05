//! Runtime binding and exact View execution.

use std::io::{Read, Seek, SeekFrom};

use crate::backwriter::anddress::{Anddress, AnddressTarget};
use crate::backwriter::view::{ViewError, ViewOutcome, project_request};

use super::{
    CurrentProofMatch, WorkspaceRuntime, is_backwriter_spill,
    source_scan::{SourceScanError, TargetProjection, observe_source, observe_structural},
    structural_cursor::{LineSpan, StructuralSink},
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
    capture_focus: Option<(usize, Anddress)>,
) -> Result<AnchoredObservation, ObservationError> {
    let focus = capture_focus.as_ref().map(|(index, _)| *index);
    let capture = capture_focus
        .map(|(_, projected)| RangeCapture::new(projected))
        .transpose()
        .map_err(map_scan_error)?;
    let targets = TargetProjection::new(inputs).map_err(map_scan_error)?;
    let mut observation = AnchoredSink { targets, capture };
    let state = observe_structural(reader, &mut observation).map_err(map_scan_error)?;
    observation.targets.finish(&state);
    let current = observation.targets.into_current();
    let outcome = if focus.is_some_and(|index| current[index]) {
        Some(
            observation
                .capture
                .take()
                .expect("capture focus creates a range capture")
                .finish()
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
    projection: AnddressTarget,
) -> Result<ViewOutcome, ViewError> {
    let Some(projected) = project_request(input, projection)? else {
        return Ok(ViewOutcome::RelationAbsent);
    };
    validate_runtime_input(runtime, &projected)?;
    match runtime.match_current_proof(&projected) {
        CurrentProofMatch::Missing => {
            let mut file = runtime
                .open_admitted_source(projected.logical_path())
                .map_err(|_| ViewError::Unavailable)?;
            observe_direct(&mut file, projected).map_err(|_| ViewError::Unavailable)
        }
        CurrentProofMatch::Mismatched => Err(ViewError::Unavailable),
        CurrentProofMatch::Matching => execute_trusted_projected(runtime, projected),
    }
}

pub(super) fn execute_batch(
    runtime: &WorkspaceRuntime,
    inputs: &[Anddress],
    projection: Option<AnddressTarget>,
) -> Result<Vec<ViewOutcome>, ViewError> {
    let projected = project_inputs(inputs, projection)?;
    for anddress in projected.iter().flatten() {
        validate_runtime_input(runtime, anddress)?;
    }

    let mut outcomes = Vec::new();
    outcomes
        .try_reserve_exact(inputs.len())
        .map_err(|_| ViewError::Unavailable)?;
    outcomes.resize_with(inputs.len(), || None);

    let mut order = Vec::new();
    order
        .try_reserve_exact(inputs.len())
        .map_err(|_| ViewError::Unavailable)?;
    for (index, value) in projected.iter().enumerate() {
        if value.is_some() {
            order.push(index);
        } else {
            outcomes[index] = Some(ViewOutcome::RelationAbsent);
        }
    }
    order.sort_unstable_by(|left, right| {
        super::compare_source_keys(
            projected[*left]
                .as_ref()
                .expect("ordered projection exists"),
            projected[*right]
                .as_ref()
                .expect("ordered projection exists"),
        )
        .then_with(|| left.cmp(right))
    });

    let mut start = 0;
    while start < order.len() {
        let end = batch_group_end(&projected, &order, start);
        execute_batch_group(runtime, &projected, &order[start..end], &mut outcomes)?;
        start = end;
    }
    finish_batch(outcomes)
}

fn project_inputs(
    inputs: &[Anddress],
    projection: Option<AnddressTarget>,
) -> Result<Vec<Option<Anddress>>, ViewError> {
    let mut projected = Vec::new();
    projected
        .try_reserve_exact(inputs.len())
        .map_err(|_| ViewError::Unavailable)?;
    for input in inputs {
        projected.push(project_request(
            input,
            projection.unwrap_or_else(|| input.target()),
        )?);
    }
    Ok(projected)
}

fn validate_runtime_input(runtime: &WorkspaceRuntime, input: &Anddress) -> Result<(), ViewError> {
    if is_backwriter_spill(input.logical_path())
        || input.workspace_coordinate() != runtime.workspace_coordinate
        || runtime.selected_root(input.logical_path()).is_err()
    {
        return Err(ViewError::Unavailable);
    }
    Ok(())
}

fn batch_group_end(projected: &[Option<Anddress>], order: &[usize], start: usize) -> usize {
    let first = projected[order[start]]
        .as_ref()
        .expect("grouped projection exists");
    let mut end = start + 1;
    while end < order.len()
        && first.same_source(
            projected[order[end]]
                .as_ref()
                .expect("grouped projection exists"),
        )
    {
        end += 1;
    }
    end
}

fn execute_batch_group(
    runtime: &WorkspaceRuntime,
    projected: &[Option<Anddress>],
    group: &[usize],
    outcomes: &mut [Option<ViewOutcome>],
) -> Result<(), ViewError> {
    let path = projected[group[0]]
        .as_ref()
        .expect("grouped projection exists")
        .logical_path();
    match runtime.select_current_proof(path) {
        Some(proof) => {
            if group.iter().any(|&index| {
                let input = projected[index]
                    .as_ref()
                    .expect("grouped projection exists");
                !super::source_state_matches(
                    &proof.hash,
                    proof.byte_length,
                    proof.line_count,
                    input,
                )
            }) {
                return Err(ViewError::Unavailable);
            }
            execute_trusted_batch(runtime, projected, group, outcomes)
        }
        None => {
            let mut file = runtime
                .open_admitted_source(path)
                .map_err(|_| ViewError::Unavailable)?;
            observe_direct_batch(&mut file, projected, group, outcomes)
                .map_err(|_| ViewError::Unavailable)
        }
    }
}

fn observe_direct_batch(
    reader: &mut impl Read,
    projected: &[Option<Anddress>],
    group: &[usize],
    outcomes: &mut [Option<ViewOutcome>],
) -> Result<(), SourceScanError> {
    let mut captures = Vec::new();
    captures
        .try_reserve_exact(group.len())
        .map_err(|_| SourceScanError::Resource)?;
    for &index in group {
        captures.push((
            index,
            RangeCapture::new(
                projected[index]
                    .as_ref()
                    .expect("grouped projection exists")
                    .clone(),
            )?,
        ));
    }
    let state = observe_source(reader, |bytes, byte_start| {
        for (_, capture) in &mut captures {
            capture.source(bytes, byte_start)?;
        }
        Ok(())
    })?;
    if group.iter().any(|&index| {
        let input = projected[index]
            .as_ref()
            .expect("grouped projection exists");
        !super::source_state_matches(
            state.hash.as_bytes(),
            state.byte_length,
            state.line_count,
            input,
        )
    }) {
        return Err(SourceScanError::InvalidSource);
    }

    let mut finished = Vec::new();
    finished
        .try_reserve_exact(captures.len())
        .map_err(|_| SourceScanError::Resource)?;
    for (index, capture) in captures {
        finished.push((index, capture.finish()?));
    }
    for (index, outcome) in finished {
        outcomes[index] = Some(outcome);
    }
    Ok(())
}

fn execute_trusted_batch(
    runtime: &WorkspaceRuntime,
    projected: &[Option<Anddress>],
    group: &[usize],
    outcomes: &mut [Option<ViewOutcome>],
) -> Result<(), ViewError> {
    let path = projected[group[0]]
        .as_ref()
        .expect("grouped projection exists")
        .logical_path();
    let result = runtime
        .open_admitted_source(path)
        .map_err(|_| TrustedViewError::Source)
        .and_then(|mut file| {
            let mut finished = Vec::new();
            finished
                .try_reserve_exact(group.len())
                .map_err(|_| TrustedViewError::Resource)?;
            for &index in group {
                finished.push((
                    index,
                    read_projected(
                        &mut file,
                        projected[index]
                            .as_ref()
                            .expect("grouped projection exists"),
                    )?,
                ));
            }
            for (index, outcome) in finished {
                outcomes[index] = Some(outcome);
            }
            Ok(())
        });
    if matches!(
        result,
        Err(TrustedViewError::Source | TrustedViewError::Resource)
    ) {
        runtime.invalidate_current_proof(path);
    }
    result.map_err(|_| ViewError::Unavailable)
}

fn finish_batch(outcomes: Vec<Option<ViewOutcome>>) -> Result<Vec<ViewOutcome>, ViewError> {
    let mut finished = Vec::new();
    finished
        .try_reserve_exact(outcomes.len())
        .map_err(|_| ViewError::Unavailable)?;
    for outcome in outcomes {
        finished.push(outcome.ok_or(ViewError::Unavailable)?);
    }
    Ok(finished)
}

pub(super) fn execute_trusted_projected(
    runtime: &WorkspaceRuntime,
    projected: Anddress,
) -> Result<ViewOutcome, ViewError> {
    let outcome = runtime
        .open_admitted_source(projected.logical_path())
        .map_err(|_| TrustedViewError::Source)
        .and_then(|mut file| read_projected(&mut file, &projected));
    if matches!(
        outcome,
        Err(TrustedViewError::Source | TrustedViewError::Resource)
    ) {
        runtime.invalidate_current_proof(projected.logical_path());
    }
    outcome.map_err(|_| ViewError::Unavailable)
}

pub(super) fn observe_direct(
    reader: &mut impl Read,
    projected: Anddress,
) -> Result<ViewOutcome, SourceScanError> {
    let mut capture = RangeCapture::new(projected)?;
    let state = observe_source(reader, |bytes, byte_start| {
        capture.source(bytes, byte_start)
    })?;
    if !super::source_state_matches(
        state.hash.as_bytes(),
        state.byte_length,
        state.line_count,
        &capture.anddress,
    ) {
        return Err(SourceScanError::InvalidSource);
    }
    capture.finish()
}

struct AnchoredSink<'a> {
    targets: TargetProjection<'a>,
    capture: Option<RangeCapture>,
}

impl StructuralSink for AnchoredSink<'_> {
    fn source(&mut self, bytes: &[u8], byte_start: usize) -> Result<(), SourceScanError> {
        if let Some(capture) = self.capture.as_mut() {
            capture.source(bytes, byte_start)?;
        }
        Ok(())
    }

    fn line(&mut self, line: LineSpan) -> Result<(), SourceScanError> {
        self.targets.line(line)
    }

    fn paragraph(
        &mut self,
        paragraph: crate::backwriter::anddress::ParagraphGeometry,
    ) -> Result<(), SourceScanError> {
        self.targets.paragraph(paragraph)
    }
}

struct RangeCapture {
    anddress: Anddress,
    bytes: Vec<u8>,
}

impl RangeCapture {
    fn new(anddress: Anddress) -> Result<Self, SourceScanError> {
        let length = anddress
            .byte_end()
            .checked_sub(anddress.byte_start())
            .ok_or(SourceScanError::Resource)?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(length)
            .map_err(|_| SourceScanError::Resource)?;
        Ok(Self { anddress, bytes })
    }

    fn source(&mut self, bytes: &[u8], chunk_start: usize) -> Result<(), SourceScanError> {
        append_overlap(
            &mut self.bytes,
            bytes,
            chunk_start,
            self.anddress.byte_start(),
            self.anddress.byte_end(),
        )
    }

    fn finish(self) -> Result<ViewOutcome, SourceScanError> {
        let expected = self
            .anddress
            .byte_end()
            .checked_sub(self.anddress.byte_start())
            .ok_or(SourceScanError::Resource)?;
        if self.bytes.len() != expected {
            return Err(SourceScanError::InvalidSource);
        }
        let content = String::from_utf8(self.bytes).map_err(|_| SourceScanError::InvalidSource)?;
        Ok(ViewOutcome::Projected {
            anddress: self.anddress,
            content,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TrustedViewError {
    Source,
    InvalidRange,
    Resource,
}

fn read_projected(
    reader: &mut (impl Read + Seek),
    projected: &Anddress,
) -> Result<ViewOutcome, TrustedViewError> {
    let bytes = read_range(reader, projected.byte_start(), projected.byte_end())?;
    let content = String::from_utf8(bytes).map_err(|_| TrustedViewError::InvalidRange)?;
    Ok(ViewOutcome::Projected {
        anddress: projected.clone(),
        content,
    })
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
        let overlap = &bytes[overlap_start - chunk_start..overlap_end - chunk_start];
        output
            .try_reserve(overlap.len())
            .map_err(|_| SourceScanError::Resource)?;
        output.extend_from_slice(overlap);
    }
    Ok(())
}

fn map_scan_error(error: SourceScanError) -> ObservationError {
    match error {
        SourceScanError::Read => ObservationError::Read,
        SourceScanError::InvalidSource => ObservationError::InvalidSource,
        SourceScanError::Resource => ObservationError::Resource,
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{self, Cursor, Read, Seek, SeekFrom},
        path::Path,
    };

    use crate::backwriter::anddress::LineTerminator;
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

    struct FailingTrustedReader {
        fail_seek: bool,
    }

    impl Read for FailingTrustedReader {
        fn read(&mut self, _output: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("injected trusted read failure"))
        }
    }

    impl Seek for FailingTrustedReader {
        fn seek(&mut self, _position: SeekFrom) -> io::Result<u64> {
            if self.fail_seek {
                Err(io::Error::other("injected trusted seek failure"))
            } else {
                Ok(0)
            }
        }
    }

    fn address(bytes: &[u8], target: AnddressTarget, start: usize, end: usize) -> Anddress {
        use crate::backwriter::anddress::{
            AnddressIssuer, ParagraphGeometry, ParentGeometry, TargetGeometry,
        };

        let mut hash = Sha256::new();
        hash.update(bytes);
        let spans = test_line_spans(bytes);
        let issuer = AnddressIssuer::new(
            &"0".repeat(64),
            "source.txt",
            &hash.finish().to_hex(),
            bytes.len(),
            spans.len(),
        )
        .unwrap();
        issuer
            .issue(match target {
                AnddressTarget::File => TargetGeometry::File,
                AnddressTarget::Paragraph => {
                    let first = spans
                        .iter()
                        .position(|&(line_start, _)| line_start == start)
                        .unwrap_or(0);
                    let line_count = spans[first..]
                        .iter()
                        .take_while(|&&(_, line_end)| line_end <= end)
                        .count()
                        .max(1);
                    TargetGeometry::Paragraph(ParagraphGeometry {
                        byte_start: start,
                        byte_end: end,
                        file_line_offset: first,
                        line_count,
                    })
                }
                AnddressTarget::Line => {
                    let line_index = spans
                        .iter()
                        .position(|&(line_start, line_end)| line_start == start && line_end == end)
                        .unwrap_or(0);
                    let body = test_line_body(&bytes[start..end]);
                    let parent = if body.iter().any(|byte| !matches!(byte, b' ' | b'\t')) {
                        let mut first = line_index;
                        while first != 0 && test_line_is_text(bytes, spans[first - 1]) {
                            first -= 1;
                        }
                        let mut after = line_index + 1;
                        while after < spans.len() && test_line_is_text(bytes, spans[after]) {
                            after += 1;
                        }
                        ParentGeometry::Paragraph(ParagraphGeometry {
                            byte_start: spans[first].0,
                            byte_end: spans[after - 1].1,
                            file_line_offset: first,
                            line_count: after - first,
                        })
                    } else {
                        ParentGeometry::File
                    };
                    let line_offset_in_parent = match parent {
                        ParentGeometry::File => line_index,
                        ParentGeometry::Paragraph(paragraph) => {
                            line_index - paragraph.file_line_offset
                        }
                    };
                    TargetGeometry::Line {
                        byte_start: start,
                        byte_end: end,
                        terminator: test_terminator(&bytes[start..end]),
                        line_offset_in_parent,
                        parent,
                    }
                }
            })
            .unwrap()
    }

    fn test_line_spans(bytes: &[u8]) -> Vec<(usize, usize)> {
        let mut spans = Vec::new();
        let mut start = 0;
        let mut cursor = 0;
        while cursor < bytes.len() {
            match bytes[cursor] {
                b'\r' if bytes.get(cursor + 1) == Some(&b'\n') => {
                    cursor += 2;
                    spans.push((start, cursor));
                    start = cursor;
                }
                b'\r' | b'\n' => {
                    cursor += 1;
                    spans.push((start, cursor));
                    start = cursor;
                }
                _ => cursor += 1,
            }
        }
        if start < bytes.len() {
            spans.push((start, bytes.len()));
        }
        spans
    }

    fn test_line_is_text(bytes: &[u8], span: (usize, usize)) -> bool {
        test_line_body(&bytes[span.0..span.1])
            .iter()
            .any(|byte| !matches!(byte, b' ' | b'\t'))
    }

    fn test_terminator(bytes: &[u8]) -> LineTerminator {
        if bytes.ends_with(b"\r\n") {
            LineTerminator::Crlf
        } else if bytes.ends_with(b"\r") {
            LineTerminator::Cr
        } else if bytes.ends_with(b"\n") {
            LineTerminator::Lf
        } else {
            LineTerminator::None
        }
    }

    fn test_line_body(bytes: &[u8]) -> &[u8] {
        if bytes.ends_with(b"\r\n") {
            &bytes[..bytes.len() - 2]
        } else if bytes.ends_with(b"\r") || bytes.ends_with(b"\n") {
            &bytes[..bytes.len() - 1]
        } else {
            bytes
        }
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

            let outcome = observe_direct(&mut reader, input.clone()).unwrap();

            assert_eq!(
                outcome,
                ViewOutcome::Projected {
                    anddress: input,
                    content: format!("{}é\r\n", "x".repeat(target_length)),
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
            observe_direct(&mut failed, input.clone()),
            Err(SourceScanError::Read)
        );

        for bytes in [b"two\nlate".as_slice(), b"one\nlonger".as_slice()] {
            let mut changed = OneByteReader {
                bytes,
                cursor: 0,
                fail_at: None,
                ended: false,
            };
            assert_eq!(
                observe_direct(&mut changed, input.clone()),
                Err(SourceScanError::InvalidSource)
            );
            assert!(changed.ended);
        }

        let mut projection = RangeCapture::new(input).unwrap();
        projection.source(b"one\n", 0).unwrap();
        assert_eq!(
            projection.source(b"x", usize::MAX),
            Err(SourceScanError::Resource)
        );
    }

    #[test]
    fn direct_batch_feeds_every_projection_from_one_forward_observation() {
        let bytes = "α\n \t\r\nlast".as_bytes();
        let inputs = [
            address(bytes, AnddressTarget::Line, 0, 3),
            address(bytes, AnddressTarget::Line, 3, 7),
            address(bytes, AnddressTarget::Line, 0, 3),
            address(bytes, AnddressTarget::Line, 7, 11),
            address(bytes, AnddressTarget::File, 0, 11),
            address(bytes, AnddressTarget::Paragraph, 0, 3),
        ];
        let projected = project_inputs(&inputs, None).unwrap();
        let group = [0, 1, 2, 3, 4, 5];
        let mut outcomes = Vec::new();
        outcomes.resize_with(inputs.len(), || None);
        let mut source = OneByteReader {
            bytes,
            cursor: 0,
            fail_at: None,
            ended: false,
        };

        observe_direct_batch(&mut source, &projected, &group, &mut outcomes).unwrap();

        assert!(source.ended);
        assert!(matches!(
            outcomes[0],
            Some(ViewOutcome::Projected { ref anddress, ref content })
                if content == "α\n"
                    && anddress.terminator() == Some(LineTerminator::Lf)
                    && anddress.project(AnddressTarget::Paragraph).unwrap().is_some()
        ));
        assert!(matches!(
            outcomes[1],
            Some(ViewOutcome::Projected { ref anddress, ref content })
                if content == " \t\r\n"
                    && anddress.terminator() == Some(LineTerminator::Crlf)
                    && anddress.project(AnddressTarget::Paragraph).unwrap().is_none()
        ));
        assert_eq!(outcomes[0], outcomes[2]);
        assert!(
            matches!(&outcomes[4], Some(ViewOutcome::Projected { anddress, content }) if anddress.target() == AnddressTarget::File && content.as_bytes() == bytes)
        );
        assert!(
            matches!(&outcomes[5], Some(ViewOutcome::Projected { anddress, content }) if anddress.target() == AnddressTarget::Paragraph && content == "α\n")
        );
        assert!(matches!(
            outcomes[3],
            Some(ViewOutcome::Projected { ref anddress, ref content })
                if content == "last" && anddress.terminator() == Some(LineTerminator::None)
        ));
    }

    #[test]
    fn direct_batch_keeps_every_output_provisional_through_late_failure() {
        let inputs = [
            address(b"one\nlate", AnddressTarget::Line, 0, 4),
            address(b"one\nlate", AnddressTarget::Line, 4, 8),
            address(b"one\nlate", AnddressTarget::File, 0, 8),
        ];
        let projected = project_inputs(&inputs, None).unwrap();
        let group = [0, 1, 2];
        for (bytes, fail_at, expected) in [
            (b"one\nlate".as_slice(), Some(4), SourceScanError::Read),
            (
                b"one\n\xff".as_slice(),
                None,
                SourceScanError::InvalidSource,
            ),
            (b"one\n\0x".as_slice(), None, SourceScanError::InvalidSource),
            (
                b"two\nlate".as_slice(),
                None,
                SourceScanError::InvalidSource,
            ),
        ] {
            let mut outcomes = Vec::new();
            outcomes.resize_with(inputs.len(), || None);
            let mut source = OneByteReader {
                bytes,
                cursor: 0,
                fail_at,
                ended: false,
            };

            assert_eq!(
                observe_direct_batch(&mut source, &projected, &group, &mut outcomes,),
                Err(expected)
            );
            assert!(outcomes.iter().all(Option::is_none));
        }
    }

    #[test]
    fn ordinary_view_projects_before_io_and_reuses_one_source_handle_per_group() {
        let production = include_str!("view.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        let batch = production
            .split("pub(super) fn execute_batch")
            .nth(1)
            .unwrap()
            .split("fn validate_runtime_input")
            .next()
            .unwrap();
        assert!(
            batch.find("project_inputs").unwrap() < batch.find("validate_runtime_input").unwrap()
        );
        assert!(batch.contains("execute_batch_group("));
        assert!(!batch.contains("execute(runtime"));
        let batch_group = production
            .split("fn execute_batch_group")
            .nth(1)
            .unwrap()
            .split("fn observe_direct_batch")
            .next()
            .unwrap();
        assert_eq!(batch_group.matches("select_current_proof(").count(), 1);
        assert_eq!(batch_group.matches("open_admitted_source(").count(), 1);
        let direct_batch = production
            .split("fn observe_direct_batch")
            .nth(1)
            .unwrap()
            .split("fn execute_trusted_batch")
            .next()
            .unwrap();
        assert_eq!(direct_batch.matches("observe_source(").count(), 1);
        assert!(direct_batch.contains("RangeCapture::new"));
        let trusted_batch = production
            .split("fn execute_trusted_batch")
            .nth(1)
            .unwrap()
            .split("fn finish_batch")
            .next()
            .unwrap();
        assert_eq!(trusted_batch.matches("open_admitted_source(").count(), 1);
        assert!(trusted_batch.contains("read_projected("));
        assert!(trusted_batch.contains("invalidate_current_proof"));
        let direct = production.split("fn observe_direct").nth(1).unwrap();
        for forbidden in [
            "scan_source(",
            "SourceEvent",
            "ExactTargetTracker",
            "DirectViewProjection",
            "LineRelation",
            "scan_paragraph_start",
            "scan_paragraph_end",
        ] {
            assert!(!direct.contains(forbidden));
        }
        assert_eq!(direct.matches("observe_source(").count(), 1);

        let source_scan = include_str!("source_scan.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        let target_projection = source_scan
            .split("pub(crate) struct TargetProjection")
            .nth(1)
            .unwrap()
            .split("struct Utf8Validator")
            .next()
            .unwrap();
        assert!(!target_projection.contains("indexes"));
        let raw = source_scan
            .split("pub(crate) fn observe_source")
            .nth(1)
            .unwrap()
            .split("pub(crate) fn observe_structural")
            .next()
            .unwrap();
        assert!(!raw.contains("observe_structural"));
        assert!(!raw.contains("StructuralCursor"));

        let anchored = production
            .split("fn observe_anchored")
            .nth(1)
            .unwrap()
            .split("pub(super) fn execute")
            .next()
            .unwrap();
        assert!(anchored.contains("TargetProjection"));
        assert!(anchored.contains("observe_structural("));
        assert!(anchored.contains("TargetProjection::new"));
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
        assert!(execute.contains("execute_trusted_projected("));
        let trusted_execution = production
            .split("pub(super) fn execute_trusted_projected")
            .nth(1)
            .unwrap()
            .split("pub(super) fn observe_direct")
            .next()
            .unwrap();
        assert!(trusted_execution.contains("read_projected("));
        assert!(trusted_execution.contains("invalidate_current_proof"));
        let trusted = production
            .split("fn read_projected")
            .nth(1)
            .unwrap()
            .split("fn read_range")
            .next()
            .unwrap();
        assert!(!trusted.contains("observe_source("));
        assert!(!trusted.contains("Sha256"));
        assert!(!production.contains("fn finish_outcome("));
        assert!(!production.contains("DirectViewProjection"));
        assert!(!production.contains("LineRelation"));
        assert!(!production.contains("scan_paragraph_start"));
        assert!(!production.contains("scan_paragraph_end"));
        assert!(!production.contains("ReverseBytes"));
        assert!(!production.contains("ForwardBytes"));
    }

    #[test]
    fn trusted_line_reads_only_its_projected_exact_range() {
        let mut bytes = vec![b'p'; READ_BUFFER_SIZE * 16];
        bytes.extend_from_slice(b"\n \t\r\n");
        let target_start = bytes.len();
        bytes.extend_from_slice("한글\r\n".as_bytes());
        let target_end = bytes.len();
        bytes.extend_from_slice("β\n".as_bytes());
        bytes.extend_from_slice(b"\t\n");
        bytes.extend(std::iter::repeat_n(b's', READ_BUFFER_SIZE * 16));
        let input = address(&bytes, AnddressTarget::Line, target_start, target_end);
        let mut reader = CountingCursor::new(bytes.clone());

        let outcome = read_projected(&mut reader, &input).unwrap();

        assert_eq!(
            outcome,
            ViewOutcome::Projected {
                anddress: input,
                content: "한글\r\n".to_owned(),
            }
        );
        assert_eq!(reader.bytes_read, target_end - target_start);
        assert!(reader.bytes_read < bytes.len() / 4);
        assert_eq!(reader.seeks, 1);
    }

    #[test]
    fn trusted_line_preserves_terminators_unicode_and_nonstructural_relations() {
        for (extent, terminator) in [
            ("한글\n", LineTerminator::Lf),
            ("한글\r", LineTerminator::Cr),
            ("한글\r\n", LineTerminator::Crlf),
            ("한글", LineTerminator::None),
        ] {
            let source = format!("\n{extent}");
            let start = 1;
            let end = source.len();
            let input = address(source.as_bytes(), AnddressTarget::Line, start, end);
            assert!(matches!(
                read_projected(&mut Cursor::new(source.as_bytes()), &input).unwrap(),
                ViewOutcome::Projected { anddress, content }
                    if content == extent
                        && anddress.terminator() == Some(terminator)
                        && anddress.project(AnddressTarget::Paragraph).unwrap().is_some()
            ));
        }

        let whitespace = b"one\n \t\r\ntwo";
        let whitespace_input = address(whitespace, AnddressTarget::Line, 4, 8);
        assert!(matches!(
            read_projected(&mut Cursor::new(whitespace), &whitespace_input).unwrap(),
            ViewOutcome::Projected { anddress, .. }
                if anddress.project(AnddressTarget::Paragraph).unwrap().is_none()
        ));
    }

    #[test]
    fn trusted_and_fallback_projection_match_for_exact_v5_targets() {
        let source = "α\r\n \t\nb\rc\n\n끝";
        let bytes = source.as_bytes();
        let spans = test_line_spans(bytes);
        let mut targets: Vec<_> = spans
            .iter()
            .map(|&(start, end)| address(bytes, AnddressTarget::Line, start, end))
            .collect();
        let mut paragraph_start = None;
        let mut paragraph_end = 0;
        for &span in &spans {
            if test_line_is_text(bytes, span) {
                paragraph_start.get_or_insert(span.0);
                paragraph_end = span.1;
            } else if let Some(start) = paragraph_start.take() {
                targets.push(address(
                    bytes,
                    AnddressTarget::Paragraph,
                    start,
                    paragraph_end,
                ));
            }
        }
        if let Some(start) = paragraph_start {
            targets.push(address(
                bytes,
                AnddressTarget::Paragraph,
                start,
                paragraph_end,
            ));
        }

        for input in targets {
            let projections: &[AnddressTarget] = match input.target() {
                AnddressTarget::Paragraph => &[AnddressTarget::Paragraph, AnddressTarget::File],
                AnddressTarget::Line => &[
                    AnddressTarget::Line,
                    AnddressTarget::Paragraph,
                    AnddressTarget::File,
                ],
                AnddressTarget::File => unreachable!(),
            };
            for &projection in projections {
                let Some(projected) = project_request(&input, projection).unwrap() else {
                    continue;
                };
                let expected = observe_direct(&mut Cursor::new(bytes), projected.clone()).unwrap();
                let actual = read_projected(&mut Cursor::new(bytes), &projected).unwrap();
                assert_eq!(actual, expected, "{:?}->{projection:?}", input.target());
            }
        }

        let file = address(bytes, AnddressTarget::File, 0, source.len());
        assert_eq!(
            read_projected(&mut Cursor::new(bytes), &file).unwrap(),
            observe_direct(&mut Cursor::new(bytes), file).unwrap()
        );
    }

    #[test]
    fn trusted_short_read_and_matching_open_failure_remove_only_matching_proof() {
        let input = address(b"one", AnddressTarget::File, 0, 3);
        assert_eq!(
            read_projected(&mut Cursor::new(b"on"), &input),
            Err(TrustedViewError::Source)
        );
        for fail_seek in [true, false] {
            assert_eq!(
                read_projected(&mut FailingTrustedReader { fail_seek }, &input),
                Err(TrustedViewError::Source)
            );
        }

        let fixture = tempfile::tempdir().unwrap();
        let runtime = host_runtime(fixture.path());
        let input = crate::backwriter::anddress::AnddressIssuer::new(
            &runtime.workspace_coordinate,
            "missing.txt",
            &"b".repeat(64),
            1,
            1,
        )
        .unwrap()
        .issue(crate::backwriter::anddress::TargetGeometry::File)
        .unwrap();
        runtime
            .install_search_proofs(vec![
                CurrentProof::new("missing.txt", "a".repeat(64), 1, 1).unwrap(),
            ])
            .unwrap();

        assert_eq!(
            runtime.view(&input, input.target()),
            Err(ViewError::Unavailable)
        );
        assert_eq!(runtime.current_proofs.lock().unwrap().len(), 1);

        let matching = crate::backwriter::anddress::AnddressIssuer::new(
            &runtime.workspace_coordinate,
            "missing.txt",
            &"a".repeat(64),
            1,
            1,
        )
        .unwrap()
        .issue(crate::backwriter::anddress::TargetGeometry::File)
        .unwrap();
        assert_eq!(
            runtime.view(&matching, matching.target()),
            Err(ViewError::Unavailable)
        );
        assert!(runtime.current_proofs.lock().unwrap().is_empty());

        std::fs::write(fixture.path().join("short.txt"), b"on").unwrap();
        let short = crate::backwriter::anddress::AnddressIssuer::new(
            &runtime.workspace_coordinate,
            "short.txt",
            &"c".repeat(64),
            3,
            1,
        )
        .unwrap()
        .issue(crate::backwriter::anddress::TargetGeometry::File)
        .unwrap();
        runtime
            .install_search_proofs(vec![
                CurrentProof::new("short.txt", "c".repeat(64), 3, 1).unwrap(),
            ])
            .unwrap();
        assert_eq!(
            runtime.view(&short, short.target()),
            Err(ViewError::Unavailable)
        );
        assert!(runtime.current_proofs.lock().unwrap().is_empty());

        let unicode = "aéz".as_bytes();
        std::fs::write(fixture.path().join("cut.txt"), unicode).unwrap();
        let file = address(unicode, AnddressTarget::File, 0, unicode.len());
        let cut = crate::backwriter::anddress::AnddressIssuer::new(
            &runtime.workspace_coordinate,
            "cut.txt",
            file.source_state_hash(),
            unicode.len(),
            1,
        )
        .unwrap()
        .issue(crate::backwriter::anddress::TargetGeometry::Line {
            byte_start: 2,
            byte_end: 3,
            terminator: LineTerminator::None,
            line_offset_in_parent: 0,
            parent: crate::backwriter::anddress::ParentGeometry::File,
        })
        .unwrap();
        runtime
            .install_search_proofs(vec![
                CurrentProof::new(
                    "cut.txt",
                    cut.source_state_hash().to_owned(),
                    unicode.len(),
                    1,
                )
                .unwrap(),
            ])
            .unwrap();
        assert_eq!(
            runtime.view(&cut, cut.target()),
            Err(ViewError::Unavailable)
        );
        assert_eq!(runtime.current_proofs.lock().unwrap().len(), 1);

        std::fs::write(fixture.path().join("resource.txt"), b"").unwrap();
        let resource = crate::backwriter::anddress::AnddressIssuer::new(
            &runtime.workspace_coordinate,
            "resource.txt",
            &"d".repeat(64),
            usize::MAX,
            1,
        )
        .unwrap()
        .issue(crate::backwriter::anddress::TargetGeometry::File)
        .unwrap();
        runtime
            .install_search_proofs(vec![
                CurrentProof::new("resource.txt", "d".repeat(64), usize::MAX, 1).unwrap(),
            ])
            .unwrap();
        assert_eq!(
            runtime.view(&resource, resource.target()),
            Err(ViewError::Unavailable)
        );
        runtime
            .install_search_proofs(vec![
                CurrentProof::new("resource.txt", "d".repeat(64), usize::MAX, 1).unwrap(),
            ])
            .unwrap();
        assert_eq!(
            runtime.view_batch(&[cut.clone(), resource], Some(AnddressTarget::File)),
            Err(ViewError::Unavailable)
        );
        let proofs = runtime.current_proofs.lock().unwrap();
        assert_eq!(proofs.len(), 1);
        assert_eq!(proofs[0].logical_path, "cut.txt");
    }

    fn host_runtime(root: &Path) -> WorkspaceRuntime {
        WorkspaceRuntime::open_host_authoritative(
            root,
            WorkspaceAdmission::new([AdmissionRoot::new(".").unwrap()]).unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn anchored_one_byte_observation_captures_projected_exact_content() {
        let bytes = "한글🦀\nβ\rγ".as_bytes();
        let inputs = [address(bytes, AnddressTarget::Line, 11, 14)];
        let mut reader = OneByteReader {
            bytes,
            cursor: 0,
            fail_at: None,
            ended: false,
        };

        let AnchoredObservation { current, outcome } =
            observe_anchored(&mut reader, &inputs, Some((0, inputs[0].clone()))).unwrap();

        assert_eq!(current, [true]);
        assert_eq!(
            outcome,
            Some(ViewOutcome::Projected {
                anddress: inputs[0].clone(),
                content: "β\r".to_owned(),
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
                matches!(observe_anchored(&mut reader, &inputs, Some((0, inputs[0].clone()))), Err(error) if same_error(error, expected))
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
