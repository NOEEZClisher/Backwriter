//! Runtime binding and exact View execution.

use std::io::{Read, Seek, SeekFrom};

use crate::backwriter::anddress::{Anddress, AnddressTarget, LineTerminator};
use crate::backwriter::view::{ViewError, ViewOutcome, validate_request};

use super::{
    CurrentProofMatch, WorkspaceRuntime, is_backwriter_spill,
    source_scan::{READ_BUFFER_SIZE, SourceScanError, TargetProjection, observe_structural},
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
    capture_focus: Option<(usize, AnddressTarget)>,
) -> Result<AnchoredObservation, ObservationError> {
    let indexes = indices(inputs.len())?;
    let captures = capture_focus
        .into_iter()
        .map(|(focus, projection)| (focus, DirectViewProjection::new(&inputs[focus], projection)))
        .collect();
    let mut projection = DirectObservation {
        captures,
        targets: Some(TargetProjection::new(inputs, &indexes).map_err(map_scan_error)?),
    };
    let state = observe_structural(reader, &mut projection).map_err(map_scan_error)?;
    let mut targets = projection
        .targets
        .take()
        .expect("anchored observation tracks target currentness");
    targets.finish(&state);
    let current = targets.into_current();
    let outcome = if capture_focus.is_some_and(|(focus, _)| current[focus]) {
        Some(
            projection
                .captures
                .pop()
                .expect("capture focus creates a View capture")
                .1
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
    projection: AnddressTarget,
) -> Result<ViewOutcome, ViewError> {
    validate_request(input, projection)?;
    validate_runtime_input(runtime, input)?;
    match runtime.match_current_proof(input) {
        CurrentProofMatch::Missing => {
            let mut file = runtime
                .open_admitted_source(input.logical_path())
                .map_err(|_| ViewError::Unavailable)?;
            observe_direct(&mut file, input, projection).map_err(|_| ViewError::Unavailable)
        }
        CurrentProofMatch::Mismatched => Err(ViewError::Unavailable),
        CurrentProofMatch::Matching => execute_trusted(runtime, input, projection),
    }
}

pub(super) fn execute_batch(
    runtime: &WorkspaceRuntime,
    inputs: &[Anddress],
    projection: AnddressTarget,
) -> Result<Vec<ViewOutcome>, ViewError> {
    for input in inputs {
        validate_request(input, projection)?;
    }
    for input in inputs {
        validate_runtime_input(runtime, input)?;
    }
    if inputs.is_empty() {
        return Ok(Vec::new());
    }

    let mut order = indices(inputs.len()).map_err(|_| ViewError::Unavailable)?;
    order.sort_unstable_by(|left, right| {
        super::compare_source_keys(&inputs[*left], &inputs[*right]).then_with(|| left.cmp(right))
    });
    let mut outcomes = Vec::new();
    outcomes
        .try_reserve_exact(inputs.len())
        .map_err(|_| ViewError::Unavailable)?;
    outcomes.resize_with(inputs.len(), || None);

    let mut start = 0;
    while start < order.len() {
        let end = batch_group_end(inputs, &order, start);
        execute_batch_group(
            runtime,
            inputs,
            &order[start..end],
            projection,
            &mut outcomes,
        )?;
        start = end;
    }
    finish_batch(outcomes)
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

fn batch_group_end(inputs: &[Anddress], order: &[usize], start: usize) -> usize {
    let first = &inputs[order[start]];
    let mut end = start + 1;
    while end < order.len() && super::same_source_key(first, &inputs[order[end]]) {
        end += 1;
    }
    end
}

fn execute_batch_group(
    runtime: &WorkspaceRuntime,
    inputs: &[Anddress],
    group: &[usize],
    projection: AnddressTarget,
    outcomes: &mut [Option<ViewOutcome>],
) -> Result<(), ViewError> {
    let path = inputs[group[0]].logical_path();
    match runtime.select_current_proof(path) {
        Some(proof) => {
            if group.iter().any(|&index| {
                !super::source_state_matches(
                    &proof.hash,
                    proof.byte_length,
                    proof.line_count,
                    &inputs[index],
                )
            }) {
                return Err(ViewError::Unavailable);
            }
            execute_trusted_batch(runtime, inputs, group, projection, outcomes)
        }
        None => {
            let mut file = runtime
                .open_admitted_source(path)
                .map_err(|_| ViewError::Unavailable)?;
            observe_direct_batch(&mut file, inputs, group, projection, outcomes)
                .map_err(|_| ViewError::Unavailable)
        }
    }
}

fn observe_direct_batch(
    reader: &mut impl Read,
    inputs: &[Anddress],
    group: &[usize],
    projection: AnddressTarget,
    outcomes: &mut [Option<ViewOutcome>],
) -> Result<(), SourceScanError> {
    let mut captures = Vec::new();
    captures
        .try_reserve_exact(group.len())
        .map_err(|_| SourceScanError::Resource)?;
    for &index in group {
        captures.push((index, DirectViewProjection::new(&inputs[index], projection)));
    }
    let mut observation = DirectObservation {
        captures,
        targets: None,
    };
    let state = observe_structural(reader, &mut observation)?;
    if group.iter().any(|&index| {
        !super::source_state_matches(
            state.hash.as_bytes(),
            state.byte_length,
            state.line_count,
            &inputs[index],
        )
    }) {
        return Err(SourceScanError::InvalidSource);
    }
    for (index, capture) in observation.captures {
        outcomes[index] = Some(capture.finish(state.byte_length)?);
    }
    Ok(())
}

fn execute_trusted_batch(
    runtime: &WorkspaceRuntime,
    inputs: &[Anddress],
    group: &[usize],
    projection: AnddressTarget,
    outcomes: &mut [Option<ViewOutcome>],
) -> Result<(), ViewError> {
    let path = inputs[group[0]].logical_path();
    let result = runtime
        .open_admitted_source(path)
        .map_err(|_| TrustedViewError::Source)
        .and_then(|mut file| {
            for &index in group {
                outcomes[index] = Some(observe_trusted(&mut file, &inputs[index], projection)?);
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

pub(super) fn execute_trusted(
    runtime: &WorkspaceRuntime,
    input: &Anddress,
    projection: AnddressTarget,
) -> Result<ViewOutcome, ViewError> {
    let outcome = runtime
        .open_admitted_source(input.logical_path())
        .map_err(|_| TrustedViewError::Source)
        .and_then(|mut file| observe_trusted(&mut file, input, projection));
    if matches!(
        outcome,
        Err(TrustedViewError::Source | TrustedViewError::Resource)
    ) {
        runtime.invalidate_current_proof(input.logical_path());
    }
    outcome.map_err(|_| ViewError::Unavailable)
}

pub(super) fn observe_direct(
    reader: &mut impl Read,
    input: &Anddress,
    projection: AnddressTarget,
) -> Result<ViewOutcome, SourceScanError> {
    let mut observation = DirectObservation {
        captures: vec![(0, DirectViewProjection::new(input, projection))],
        targets: None,
    };
    let state = observe_structural(reader, &mut observation)?;
    if input.source_byte_length() != state.byte_length
        || input.source_line_count() != state.line_count
        || input.source_state_hash() != state.hash
    {
        return Err(SourceScanError::InvalidSource);
    }
    observation
        .captures
        .pop()
        .expect("single View capture exists")
        .1
        .finish(state.byte_length)
}

struct DirectObservation<'a> {
    captures: Vec<(usize, DirectViewProjection<'a>)>,
    targets: Option<TargetProjection<'a>>,
}

impl StructuralSink for DirectObservation<'_> {
    fn source(&mut self, bytes: &[u8], byte_start: usize) -> Result<(), SourceScanError> {
        for (_, capture) in &mut self.captures {
            capture.source(bytes, byte_start)?;
        }
        Ok(())
    }

    fn line(&mut self, line: LineSpan) -> Result<(), SourceScanError> {
        if let Some(targets) = self.targets.as_mut() {
            targets.line(line)?;
        }
        for (_, capture) in &mut self.captures {
            capture.line(line)?;
        }
        Ok(())
    }

    fn paragraph(
        &mut self,
        paragraph: crate::backwriter::anddress::ParagraphGeometry,
    ) -> Result<(), SourceScanError> {
        if let Some(targets) = self.targets.as_mut() {
            targets.paragraph(paragraph)?;
        }
        for (_, capture) in &mut self.captures {
            capture.paragraph(paragraph)?;
        }
        Ok(())
    }
}

struct DirectViewProjection<'a> {
    input: &'a Anddress,
    projection: AnddressTarget,
    target_range: Option<(usize, usize)>,
    target: Vec<u8>,
    line_relation: Option<LineRelation>,
}

impl<'a> DirectViewProjection<'a> {
    fn new(input: &'a Anddress, projection: AnddressTarget) -> Self {
        let target_range = match (input.target(), projection) {
            (_, AnddressTarget::File) => Some((0, input.source_byte_length())),
            (AnddressTarget::Paragraph, AnddressTarget::Paragraph)
            | (AnddressTarget::Line, AnddressTarget::Line) => {
                Some((input.byte_start(), input.byte_end()))
            }
            (AnddressTarget::Line, AnddressTarget::Paragraph) => input
                .project(AnddressTarget::Paragraph)
                .expect("validated Line projection")
                .map(|paragraph| (paragraph.byte_start(), paragraph.byte_end())),
            _ => None,
        };
        Self {
            input,
            projection,
            target_range,
            target: Vec::new(),
            line_relation: (input.target() == AnddressTarget::Line
                && projection != AnddressTarget::File)
                .then(|| LineRelation::new(input.byte_start(), input.byte_end())),
        }
    }

    fn source(&mut self, bytes: &[u8], chunk_start: usize) -> Result<(), SourceScanError> {
        if let Some((start, end)) = self.target_range {
            append_overlap(&mut self.target, bytes, chunk_start, start, end)?;
        }
        Ok(())
    }

    fn line(&mut self, line: LineSpan) -> Result<(), SourceScanError> {
        if let Some(relation) = self.line_relation.as_mut() {
            relation.line(line);
        }
        Ok(())
    }

    fn paragraph(
        &mut self,
        paragraph: crate::backwriter::anddress::ParagraphGeometry,
    ) -> Result<(), SourceScanError> {
        if let Some(relation) = self.line_relation.as_mut() {
            relation.paragraph(paragraph);
        }
        Ok(())
    }

    fn finish(mut self, _source_byte_length: usize) -> Result<ViewOutcome, SourceScanError> {
        let related = self
            .line_relation
            .take()
            .and_then(|relation| relation.related);
        if self.projection == AnddressTarget::Paragraph
            && self.input.target() == AnddressTarget::Line
        {
            return match related {
                Some(related) => finish_outcome(
                    self.input,
                    self.projection,
                    self.target,
                    (related.start, related.end),
                    None,
                ),
                None => Ok(ViewOutcome::RelationAbsent),
            };
        }
        let range = self
            .target_range
            .expect("every non-relational projection has a target range");
        finish_outcome(
            self.input,
            self.projection,
            self.target,
            range,
            related.map(|related| (related.start, related.end)),
        )
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
    projection: AnddressTarget,
) -> Result<ViewOutcome, TrustedViewError> {
    match (input.target(), projection) {
        (_, AnddressTarget::File) => finish_outcome(
            input,
            projection,
            read_range(reader, 0, input.source_byte_length())?,
            (0, input.source_byte_length()),
            None,
        )
        .map_err(map_trusted_scan_error),
        (AnddressTarget::Paragraph, AnddressTarget::Paragraph) => finish_outcome(
            input,
            projection,
            read_range(reader, input.byte_start(), input.byte_end())?,
            (input.byte_start(), input.byte_end()),
            None,
        )
        .map_err(map_trusted_scan_error),
        (AnddressTarget::Line, AnddressTarget::Line) => {
            let target = read_range(reader, input.byte_start(), input.byte_end())?;
            let related = related_paragraph_range(reader, input, &target)?;
            finish_outcome(
                input,
                projection,
                target,
                (input.byte_start(), input.byte_end()),
                related,
            )
            .map_err(map_trusted_scan_error)
        }
        (AnddressTarget::Line, AnddressTarget::Paragraph) => {
            let target = read_range(reader, input.byte_start(), input.byte_end())?;
            let Some((start, end)) = related_paragraph_range(reader, input, &target)? else {
                return Ok(ViewOutcome::RelationAbsent);
            };
            drop(target);
            finish_outcome(
                input,
                projection,
                read_range(reader, start, end)?,
                (start, end),
                None,
            )
            .map_err(map_trusted_scan_error)
        }
        _ => Err(TrustedViewError::InvalidRange),
    }
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
    if target_start == 0 {
        return Ok(0);
    }
    let mut scratch = [0_u8; READ_BUFFER_SIZE];
    let mut chunk_end = target_start;
    let mut paragraph_start = target_start;
    let mut expect_terminator = true;
    let mut maybe_cr = false;
    let mut has_text = false;

    while chunk_end != 0 {
        let chunk_start = chunk_end.saturating_sub(READ_BUFFER_SIZE);
        let chunk_length = chunk_end - chunk_start;
        seek_to(reader, chunk_start)?;
        read_fully(reader, &mut scratch[..chunk_length])?;
        let mut cursor = chunk_length;

        while cursor != 0 {
            if maybe_cr {
                if scratch[cursor - 1] == b'\r' {
                    cursor -= 1;
                }
                maybe_cr = false;
                expect_terminator = false;
                continue;
            }
            if expect_terminator {
                match scratch[cursor - 1] {
                    b'\n' => {
                        cursor -= 1;
                        maybe_cr = true;
                    }
                    b'\r' => {
                        cursor -= 1;
                        expect_terminator = false;
                    }
                    _ => return Err(TrustedViewError::Source),
                }
                continue;
            }

            let body = &scratch[..cursor];
            let Some(delimiter) = last_line_break(body) else {
                has_text |= body.iter().any(|byte| !matches!(byte, b' ' | b'\t'));
                cursor = 0;
                continue;
            };
            has_text |= body[delimiter + 1..]
                .iter()
                .any(|byte| !matches!(byte, b' ' | b'\t'));
            if !has_text {
                return Ok(paragraph_start);
            }
            paragraph_start = chunk_start
                .checked_add(delimiter + 1)
                .ok_or(TrustedViewError::Resource)?;
            cursor = delimiter + 1;
            expect_terminator = true;
            has_text = false;
        }

        chunk_end = chunk_start;
    }

    if maybe_cr {
        expect_terminator = false;
    }
    if expect_terminator {
        return Err(TrustedViewError::Source);
    }
    Ok(if has_text { 0 } else { paragraph_start })
}

fn scan_paragraph_end(
    reader: &mut (impl Read + Seek),
    target_end: usize,
    source_end: usize,
) -> Result<usize, TrustedViewError> {
    let mut scratch = [0_u8; READ_BUFFER_SIZE];
    let mut position = target_end;
    let mut paragraph_end = target_end;
    let mut has_text = false;
    let mut pending_cr = false;
    seek_to(reader, target_end)?;

    while position < source_end {
        let chunk_length = (source_end - position).min(READ_BUFFER_SIZE);
        read_fully(reader, &mut scratch[..chunk_length])?;
        let chunk_start = position;
        let mut cursor = 0;

        while cursor < chunk_length {
            if pending_cr {
                if scratch[cursor] == b'\n' {
                    cursor += 1;
                }
                pending_cr = false;
                if !has_text {
                    return Ok(paragraph_end);
                }
                paragraph_end = chunk_start
                    .checked_add(cursor)
                    .ok_or(TrustedViewError::Resource)?;
                has_text = false;
                continue;
            }

            let remaining = &scratch[cursor..chunk_length];
            let Some(relative) = first_line_break(remaining) else {
                has_text |= remaining.iter().any(|byte| !matches!(byte, b' ' | b'\t'));
                cursor = chunk_length;
                continue;
            };
            let delimiter = cursor + relative;
            has_text |= scratch[cursor..delimiter]
                .iter()
                .any(|byte| !matches!(byte, b' ' | b'\t'));
            let terminator = scratch[delimiter];
            cursor = delimiter + 1;
            if terminator == b'\r' {
                pending_cr = true;
            } else {
                if !has_text {
                    return Ok(paragraph_end);
                }
                paragraph_end = chunk_start
                    .checked_add(cursor)
                    .ok_or(TrustedViewError::Resource)?;
                has_text = false;
            }
        }

        position = chunk_start
            .checked_add(chunk_length)
            .ok_or(TrustedViewError::Resource)?;
    }

    if pending_cr || has_text {
        if !has_text {
            return Ok(paragraph_end);
        }
        paragraph_end = source_end;
    }
    Ok(paragraph_end)
}

fn first_line_break(bytes: &[u8]) -> Option<usize> {
    let mut words = bytes.chunks_exact(8);
    for (index, chunk) in words.by_ref().enumerate() {
        if word_has_line_break(u64::from_ne_bytes(
            chunk.try_into().expect("eight-byte chunk"),
        )) {
            return chunk
                .iter()
                .position(|byte| matches!(byte, b'\r' | b'\n'))
                .map(|offset| index * 8 + offset);
        }
    }
    let remainder_start = bytes.len() - words.remainder().len();
    words
        .remainder()
        .iter()
        .position(|byte| matches!(byte, b'\r' | b'\n'))
        .map(|offset| remainder_start + offset)
}

fn last_line_break(bytes: &[u8]) -> Option<usize> {
    let full_length = bytes.len() - bytes.len() % 8;
    if let Some(offset) = bytes[full_length..]
        .iter()
        .rposition(|byte| matches!(byte, b'\r' | b'\n'))
    {
        return Some(full_length + offset);
    }
    for (index, chunk) in bytes[..full_length].chunks_exact(8).enumerate().rev() {
        if word_has_line_break(u64::from_ne_bytes(
            chunk.try_into().expect("eight-byte chunk"),
        )) {
            return chunk
                .iter()
                .rposition(|byte| matches!(byte, b'\r' | b'\n'))
                .map(|offset| index * 8 + offset);
        }
    }
    None
}

fn word_has_line_break(word: u64) -> bool {
    const LOW_BITS: u64 = 0x0101_0101_0101_0101;
    const HIGH_BITS: u64 = 0x8080_8080_8080_8080;
    const CR: u64 = u64::from_ne_bytes([b'\r'; 8]);
    const LF: u64 = u64::from_ne_bytes([b'\n'; 8]);

    [word ^ CR, word ^ LF]
        .into_iter()
        .any(|candidate| candidate.wrapping_sub(LOW_BITS) & !candidate & HIGH_BITS != 0)
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
    selected_text_line: bool,
    done: bool,
    related: Option<RelatedParagraph>,
}

struct RelatedParagraph {
    start: usize,
    end: usize,
}

impl LineRelation {
    fn new(target_start: usize, target_end: usize) -> Self {
        Self {
            target_start,
            target_end,
            selected_text_line: false,
            done: false,
            related: None,
        }
    }

    fn line(&mut self, line: LineSpan) {
        if self.done {
            return;
        }
        if line.byte_start != self.target_start || line.byte_end != self.target_end {
            return;
        }
        if line.body_class == crate::backwriter::anddress::LineBodyClass::Text {
            self.selected_text_line = true;
        } else {
            self.done = true;
        }
    }

    fn paragraph(&mut self, paragraph: crate::backwriter::anddress::ParagraphGeometry) {
        if self.done || !self.selected_text_line {
            return;
        }
        if paragraph.byte_start <= self.target_start && self.target_end <= paragraph.byte_end {
            self.related = Some(RelatedParagraph {
                start: paragraph.byte_start,
                end: paragraph.byte_end,
            });
            self.done = true;
        }
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
    projection: AnddressTarget,
    target: Vec<u8>,
    range: (usize, usize),
    related: Option<(usize, usize)>,
) -> Result<ViewOutcome, SourceScanError> {
    let anddress = target_address(input, projection, range.0, range.1)?;
    match projection {
        AnddressTarget::File => Ok(ViewOutcome::File {
            anddress,
            text: String::from_utf8(target).map_err(|_| SourceScanError::InvalidSource)?,
        }),
        AnddressTarget::Paragraph => Ok(ViewOutcome::Paragraph {
            anddress,
            text: String::from_utf8(target).map_err(|_| SourceScanError::InvalidSource)?,
            file: file_address(input)?,
        }),
        AnddressTarget::Line => {
            let (content, terminator) = line_parts(target)?;
            Ok(ViewOutcome::Line {
                anddress,
                content,
                terminator,
                file: file_address(input)?,
                paragraph: related
                    .map(|(start, end)| {
                        target_address(input, AnddressTarget::Paragraph, start, end)
                    })
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
    target_address(input, AnddressTarget::File, 0, input.source_byte_length())
}

fn target_address(
    input: &Anddress,
    target: AnddressTarget,
    byte_start: usize,
    byte_end: usize,
) -> Result<Anddress, SourceScanError> {
    let address = input
        .project(target)
        .map_err(|_| SourceScanError::InvalidSource)?
        .ok_or(SourceScanError::InvalidSource)?;
    (address.byte_start() == byte_start && address.byte_end() == byte_end)
        .then_some(address)
        .ok_or(SourceScanError::InvalidSource)
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

            let outcome = observe_direct(&mut reader, &input, input.target()).unwrap();

            assert_eq!(
                outcome,
                ViewOutcome::Line {
                    anddress: input,
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
            observe_direct(&mut failed, &input, input.target()),
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
                observe_direct(&mut changed, &input, input.target()),
                Err(SourceScanError::InvalidSource)
            );
            assert!(changed.ended);
        }

        let mut projection = DirectViewProjection::new(&input, input.target());
        projection.source(b"one\n", 0).unwrap();
        assert_eq!(
            projection.source(b"x", usize::MAX),
            Err(SourceScanError::Resource)
        );
    }

    #[test]
    fn direct_batch_feeds_every_projection_from_one_forward_observation() {
        let bytes = "α\n \t\r\nlast".as_bytes();
        let inputs = vec![
            address(bytes, AnddressTarget::Line, 0, 3),
            address(bytes, AnddressTarget::Line, 3, 7),
            address(bytes, AnddressTarget::Line, 0, 3),
            address(bytes, AnddressTarget::Line, 7, 11),
        ];
        let group = [0, 1, 2, 3];
        let mut outcomes = Vec::new();
        outcomes.resize_with(inputs.len(), || None);
        let mut source = OneByteReader {
            bytes,
            cursor: 0,
            fail_at: None,
            ended: false,
        };

        observe_direct_batch(
            &mut source,
            &inputs,
            &group,
            AnddressTarget::Line,
            &mut outcomes,
        )
        .unwrap();

        assert!(source.ended);
        assert!(matches!(
            outcomes[0],
            Some(ViewOutcome::Line {
                ref content,
                terminator: LineTerminator::Lf,
                paragraph: Some(_),
                ..
            }) if content == "α"
        ));
        assert!(matches!(
            outcomes[1],
            Some(ViewOutcome::Line {
                ref content,
                terminator: LineTerminator::Crlf,
                paragraph: None,
                ..
            }) if content == " \t"
        ));
        assert_eq!(outcomes[0], outcomes[2]);
        assert!(matches!(
            outcomes[3],
            Some(ViewOutcome::Line {
                ref content,
                terminator: LineTerminator::None,
                paragraph: Some(_),
                ..
            }) if content == "last"
        ));
    }

    #[test]
    fn direct_batch_keeps_every_output_provisional_through_late_failure() {
        let inputs = vec![
            address(b"one\nlate", AnddressTarget::Line, 0, 4),
            address(b"one\nlate", AnddressTarget::Line, 4, 8),
        ];
        let group = [0, 1];
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
                observe_direct_batch(
                    &mut source,
                    &inputs,
                    &group,
                    AnddressTarget::Line,
                    &mut outcomes,
                ),
                Err(expected)
            );
            assert!(outcomes.iter().all(Option::is_none));
        }
    }

    #[test]
    fn ordinary_view_uses_only_the_direct_observation_path() {
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
            batch.find("validate_request").unwrap() < batch.find("validate_runtime_input").unwrap()
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
        assert_eq!(direct_batch.matches("observe_structural(").count(), 1);
        assert!(direct_batch.contains("DirectViewProjection::new"));
        let trusted_batch = production
            .split("fn execute_trusted_batch")
            .nth(1)
            .unwrap()
            .split("fn finish_batch")
            .next()
            .unwrap();
        assert_eq!(trusted_batch.matches("open_admitted_source(").count(), 1);
        assert_eq!(trusted_batch.matches("observe_trusted(").count(), 1);
        assert!(trusted_batch.contains("invalidate_current_proof"));
        let direct = production.split("fn observe_direct").nth(1).unwrap();
        for forbidden in ["scan_source(", "SourceEvent", "ExactTargetTracker"] {
            assert!(!direct.contains(forbidden));
        }
        assert_eq!(direct.matches("observe_structural(").count(), 1);

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
        assert!(execute.contains("execute_trusted("));
        let trusted_execution = production
            .split("pub(super) fn execute_trusted")
            .nth(1)
            .unwrap()
            .split("pub(super) fn observe_direct")
            .next()
            .unwrap();
        assert!(trusted_execution.contains("observe_trusted("));
        assert!(trusted_execution.contains("invalidate_current_proof"));
        let trusted = production
            .split("fn observe_trusted")
            .nth(1)
            .unwrap()
            .split("fn read_range")
            .next()
            .unwrap();
        assert!(!trusted.contains("observe_source("));
        assert!(!trusted.contains("Sha256"));
        assert_eq!(production.matches("fn finish_outcome(").count(), 1);
        assert!(!production.contains("view_projected"));
        assert!(!production.contains("ReverseBytes"));
        assert!(!production.contains("ForwardBytes"));
        let relation = production
            .split("fn scan_paragraph_start")
            .nth(1)
            .unwrap()
            .split("fn read_byte_at")
            .next()
            .unwrap();
        assert_eq!(relation.matches("[0_u8; READ_BUFFER_SIZE]").count(), 2);
        assert!(relation.contains(".rposition("));
        assert!(relation.contains(".position("));
    }

    #[test]
    fn line_break_word_filter_matches_byte_order_at_every_alignment() {
        for length in (0..=33).chain([READ_BUFFER_SIZE - 1, READ_BUFFER_SIZE, READ_BUFFER_SIZE + 1])
        {
            let mut bytes = vec![b'x'; length];
            assert_eq!(first_line_break(&bytes), None);
            assert_eq!(last_line_break(&bytes), None);
            let mut positions: Vec<_> = (0..length.min(17)).collect();
            positions.extend(length.saturating_sub(17)..length);
            positions.sort_unstable();
            positions.dedup();
            for position in positions {
                for delimiter in [b'\r', b'\n'] {
                    bytes[position] = delimiter;
                    assert_eq!(first_line_break(&bytes), Some(position));
                    assert_eq!(last_line_break(&bytes), Some(position));
                    bytes[position] = b'x';
                }
            }
        }
        assert_eq!(first_line_break(b"x\rxxx\nx"), Some(1));
        assert_eq!(last_line_break(b"x\rxxx\nx"), Some(5));
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

        let outcome = observe_trusted(&mut reader, &input, input.target()).unwrap();

        assert_eq!(
            outcome,
            ViewOutcome::Line {
                anddress: input,
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
                observe_trusted(&mut Cursor::new(source.as_bytes()), &input, input.target())
                    .unwrap(),
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
            observe_trusted(
                &mut Cursor::new(whitespace),
                &whitespace_input,
                whitespace_input.target(),
            )
            .unwrap(),
            ViewOutcome::Line {
                paragraph: None,
                ..
            }
        ));
        let raw = b"zero\none\r\ntwo";
        let raw_input = address(raw, AnddressTarget::Line, 2, 10);
        assert!(matches!(
            observe_trusted(&mut Cursor::new(raw), &raw_input, raw_input.target()).unwrap(),
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
            observe_trusted(&mut Cursor::new(&source), &input, input.target()).unwrap(),
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
    fn trusted_chunk_relation_preserves_complete_and_separator_bounded_paragraphs() {
        let mut complete = "α\r\n".as_bytes().to_vec();
        complete.extend(std::iter::repeat_n(b'b', READ_BUFFER_SIZE - 2));
        complete.push(b'\r');
        let target_start = complete.len();
        complete.extend_from_slice(b"needle\n");
        let target_end = complete.len();
        complete.extend(std::iter::repeat_n(b'c', READ_BUFFER_SIZE - 1));
        complete.extend_from_slice(b"\r\n");
        complete.extend_from_slice("끝".as_bytes());
        let input = address(&complete, AnddressTarget::Line, target_start, target_end);
        let expected = observe_direct(&mut Cursor::new(&complete), &input, input.target()).unwrap();
        let actual = observe_trusted(&mut Cursor::new(&complete), &input, input.target()).unwrap();
        assert_eq!(actual, expected);
        assert!(matches!(
            actual,
            ViewOutcome::Line {
                paragraph: Some(paragraph),
                ..
            } if paragraph.byte_start() == 0 && paragraph.byte_end() == complete.len()
        ));

        let bounded = "left\n \t\r\nneedle\r\nβ\r\t \nright";
        let target_start = bounded.find("needle").unwrap();
        let target_end = target_start + "needle\r\n".len();
        let paragraph_end = target_end + "β\r".len();
        let input = address(
            bounded.as_bytes(),
            AnddressTarget::Line,
            target_start,
            target_end,
        );
        let expected =
            observe_direct(&mut Cursor::new(bounded.as_bytes()), &input, input.target()).unwrap();
        let actual =
            observe_trusted(&mut Cursor::new(bounded.as_bytes()), &input, input.target()).unwrap();
        assert_eq!(actual, expected);
        assert!(matches!(
            actual,
            ViewOutcome::Line {
                paragraph: Some(paragraph),
                ..
            } if paragraph.byte_start() == "left\n \t\r\n".len()
                && paragraph.byte_end() == paragraph_end
        ));

        for (source, start, end) in [("\t\n끝\r", 2, 6), ("끝", 0, 3)] {
            let input = address(source.as_bytes(), AnddressTarget::Line, start, end);
            assert_eq!(
                observe_trusted(&mut Cursor::new(source.as_bytes()), &input, input.target())
                    .unwrap(),
                observe_direct(&mut Cursor::new(source.as_bytes()), &input, input.target())
                    .unwrap()
            );
        }
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
                let expected = observe_direct(&mut Cursor::new(bytes), &input, projection).unwrap();
                let actual = observe_trusted(&mut Cursor::new(bytes), &input, projection).unwrap();
                assert_eq!(actual, expected, "{:?}->{projection:?}", input.target());
            }
        }

        let file = address(bytes, AnddressTarget::File, 0, source.len());
        assert_eq!(
            observe_trusted(&mut Cursor::new(bytes), &file, AnddressTarget::File).unwrap(),
            observe_direct(&mut Cursor::new(bytes), &file, AnddressTarget::File).unwrap()
        );
    }

    #[test]
    fn trusted_short_read_and_matching_open_failure_remove_only_matching_proof() {
        let input = address(b"one", AnddressTarget::File, 0, 3);
        assert_eq!(
            observe_trusted(&mut Cursor::new(b"on"), &input, input.target()),
            Err(TrustedViewError::Source)
        );
        for fail_seek in [true, false] {
            assert_eq!(
                observe_trusted(
                    &mut FailingTrustedReader { fail_seek },
                    &input,
                    input.target(),
                ),
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
            runtime.view_batch(&[cut.clone(), resource], AnddressTarget::File),
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
            observe_anchored(&mut reader, &inputs, Some((0, inputs[0].target()))).unwrap();

        assert_eq!(current, [true]);
        assert_eq!(
            outcome,
            Some(ViewOutcome::Line {
                anddress: inputs[0].clone(),
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
                matches!(observe_anchored(&mut reader, &inputs, Some((0, inputs[0].target()))), Err(error) if same_error(error, expected))
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
