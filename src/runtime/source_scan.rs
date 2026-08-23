//! Private forward framing shared by Runtime source observers.

use std::io::Read;
use std::ops::Range;

use crate::backwriter::anddress::{Anddress, AnddressTarget, LineBodyClass, Natural};

pub(crate) const READ_BUFFER_SIZE: usize = 8_192;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SourceScanError {
    Read,
    InvalidSource,
    Resource,
}

pub(crate) struct DecimalOrdinal {
    digits: Vec<u8>,
}

impl DecimalOrdinal {
    pub(crate) fn zero() -> Result<Self, SourceScanError> {
        let mut digits = Vec::new();
        digits
            .try_reserve_exact(1)
            .map_err(|_| SourceScanError::Resource)?;
        digits.push(b'0');
        Ok(Self { digits })
    }

    pub(crate) fn as_natural(&self) -> Result<Natural, SourceScanError> {
        let digits = std::str::from_utf8(&self.digits).expect("decimal digits are UTF-8");
        Natural::parse(digits).map_err(|_| SourceScanError::Resource)
    }

    pub(crate) fn increment(&mut self) -> Result<(), SourceScanError> {
        for digit in self.digits.iter_mut().rev() {
            if *digit != b'9' {
                *digit += 1;
                return Ok(());
            }
            *digit = b'0';
        }
        let length = self.digits.len();
        self.digits
            .try_reserve_exact(1)
            .map_err(|_| SourceScanError::Resource)?;
        self.digits.push(b'0');
        self.digits.copy_within(0..length, 1);
        self.digits[0] = b'1';
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub(crate) enum SourceEvent<'a> {
    StartLine {
        ordinal: &'a DecimalOrdinal,
    },
    Byte {
        byte: u8,
        content: bool,
    },
    EndLine {
        ordinal: &'a DecimalOrdinal,
        body_class: LineBodyClass,
    },
}

/// Tracks exact current target evidence without retaining source bytes.
pub(crate) struct ExactTargetTracker<'a> {
    inputs: &'a [Anddress],
    indexes: &'a [usize],
    current: Vec<bool>,
    structural_indexes: Vec<usize>,
    line_end: usize,
    line_cursor: usize,
    paragraph_cursor: usize,
    line_match: Range<usize>,
    paragraph_ordinal: DecimalOrdinal,
    in_paragraph: bool,
    extent_position: usize,
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
        let structural_count = indexes
            .iter()
            .filter(|&&index| !matches!(inputs[index].target, AnddressTarget::File))
            .count();
        let mut structural_indexes = Vec::new();
        structural_indexes
            .try_reserve_exact(structural_count)
            .map_err(|_| SourceScanError::Resource)?;
        for &index in indexes {
            if matches!(inputs[index].target, AnddressTarget::Line { .. }) {
                structural_indexes.push(index);
            }
        }
        let line_end = structural_indexes.len();
        for &index in indexes {
            if matches!(inputs[index].target, AnddressTarget::Paragraph { .. }) {
                structural_indexes.push(index);
            }
        }
        structural_indexes[..line_end].sort_unstable_by(|left, right| {
            target_ordinal(&inputs[*left].target).cmp(target_ordinal(&inputs[*right].target))
        });
        structural_indexes[line_end..].sort_unstable_by(|left, right| {
            target_ordinal(&inputs[*left].target).cmp(target_ordinal(&inputs[*right].target))
        });
        Ok(Self {
            inputs,
            indexes,
            current,
            structural_indexes,
            line_end,
            line_cursor: 0,
            paragraph_cursor: line_end,
            line_match: 0..0,
            paragraph_ordinal: DecimalOrdinal::zero()?,
            in_paragraph: false,
            extent_position: 0,
        })
    }

    pub(crate) fn consume(&mut self, event: SourceEvent<'_>) -> Result<(), SourceScanError> {
        match event {
            SourceEvent::StartLine { ordinal } => {
                self.start_line(ordinal);
                Ok(())
            }
            SourceEvent::Byte { byte, .. } => self.push_extent_byte(byte),
            SourceEvent::EndLine { body_class, .. } => self.finish_line(body_class),
        }
    }

    pub(crate) fn finish(&mut self) {
        for &index in self.indexes {
            if matches!(self.inputs[index].target, AnddressTarget::File) {
                self.current[index] = true;
            }
        }
    }

    pub(crate) fn is_current(&self, index: usize) -> bool {
        self.current[index]
    }

    pub(crate) fn into_current(self) -> Vec<bool> {
        self.current
    }

    fn start_line(&mut self, ordinal: &DecimalOrdinal) {
        self.extent_position = 0;
        while self.line_cursor < self.line_end
            && target_ordinal(&self.inputs[self.structural_indexes[self.line_cursor]].target)
                .cmp_canonical_decimal_bytes(&ordinal.digits)
                .is_lt()
        {
            self.line_cursor += 1;
        }
        let start = self.line_cursor;
        while self.line_cursor < self.line_end
            && target_ordinal(&self.inputs[self.structural_indexes[self.line_cursor]].target)
                .cmp_canonical_decimal_bytes(&ordinal.digits)
                .is_eq()
        {
            self.current[self.structural_indexes[self.line_cursor]] = true;
            self.line_cursor += 1;
        }
        self.line_match = start..self.line_cursor;
    }

    fn push_extent_byte(&mut self, byte: u8) -> Result<(), SourceScanError> {
        let offset = self.extent_position;
        let mut still_matching = false;
        for &index in &self.structural_indexes[self.line_match.clone()] {
            if !self.current[index] {
                continue;
            }
            let AnddressTarget::Line { exact_extent, .. } = &self.inputs[index].target else {
                unreachable!("matching Line index keeps its target kind");
            };
            if exact_extent.as_bytes().get(offset) == Some(&byte) {
                still_matching = true;
            } else {
                self.current[index] = false;
            }
        }
        if still_matching {
            self.extent_position = self
                .extent_position
                .checked_add(1)
                .ok_or(SourceScanError::Resource)?;
        }
        Ok(())
    }

    fn finish_line(&mut self, body_class: LineBodyClass) -> Result<(), SourceScanError> {
        for &index in &self.structural_indexes[self.line_match.clone()] {
            let AnddressTarget::Line { exact_extent, .. } = &self.inputs[index].target else {
                unreachable!("line match keeps its target kind");
            };
            if exact_extent.len() != self.extent_position {
                self.current[index] = false;
            }
        }

        if body_class == LineBodyClass::Text {
            if !self.in_paragraph {
                self.in_paragraph = true;
                while self.paragraph_cursor < self.structural_indexes.len()
                    && target_ordinal(
                        &self.inputs[self.structural_indexes[self.paragraph_cursor]].target,
                    )
                    .cmp_canonical_decimal_bytes(&self.paragraph_ordinal.digits)
                    .is_lt()
                {
                    self.paragraph_cursor += 1;
                }
                while self.paragraph_cursor < self.structural_indexes.len()
                    && target_ordinal(
                        &self.inputs[self.structural_indexes[self.paragraph_cursor]].target,
                    )
                    .cmp_canonical_decimal_bytes(&self.paragraph_ordinal.digits)
                    .is_eq()
                {
                    self.current[self.structural_indexes[self.paragraph_cursor]] = true;
                    self.paragraph_cursor += 1;
                }
            }
        } else if self.in_paragraph {
            self.paragraph_ordinal.increment()?;
            self.in_paragraph = false;
        }
        Ok(())
    }
}

fn target_ordinal(target: &AnddressTarget) -> &Natural {
    match target {
        AnddressTarget::Line { ordinal, .. } | AnddressTarget::Paragraph { ordinal } => ordinal,
        AnddressTarget::File => unreachable!("structural index excludes File targets"),
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
    line_ordinal: DecimalOrdinal,
    line_started: bool,
    pending_cr: bool,
    body_class: LineBodyClass,
    utf8: Utf8Validator,
}

impl SourceFramer {
    pub(crate) fn new() -> Result<Self, SourceScanError> {
        Ok(Self {
            line_ordinal: DecimalOrdinal::zero()?,
            line_started: false,
            pending_cr: false,
            body_class: LineBodyClass::Empty,
            utf8: Utf8Validator::default(),
        })
    }

    pub(crate) fn push(
        &mut self,
        bytes: &[u8],
        on_event: &mut impl FnMut(SourceEvent<'_>) -> Result<(), SourceScanError>,
    ) -> Result<(), SourceScanError> {
        if !self.utf8.push(bytes) {
            return Err(SourceScanError::InvalidSource);
        }
        for &byte in bytes {
            if self.pending_cr {
                if byte == b'\n' {
                    on_event(SourceEvent::Byte {
                        byte,
                        content: false,
                    })?;
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
                    self.pending_cr = true;
                }
                b'\n' => {
                    on_event(SourceEvent::Byte {
                        byte,
                        content: false,
                    })?;
                    self.finish_line(on_event)?;
                }
                _ => {
                    on_event(SourceEvent::Byte {
                        byte,
                        content: true,
                    })?;
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
        on_event: &mut impl FnMut(SourceEvent<'_>) -> Result<(), SourceScanError>,
    ) -> Result<(), SourceScanError> {
        if !self.utf8.finish() {
            return Err(SourceScanError::InvalidSource);
        }
        if self.line_started {
            self.finish_line(on_event)?;
        }
        Ok(())
    }

    fn begin_line(
        &mut self,
        on_event: &mut impl FnMut(SourceEvent<'_>) -> Result<(), SourceScanError>,
    ) -> Result<(), SourceScanError> {
        if !self.line_started {
            self.line_started = true;
            on_event(SourceEvent::StartLine {
                ordinal: &self.line_ordinal,
            })?;
        }
        Ok(())
    }

    fn finish_line(
        &mut self,
        on_event: &mut impl FnMut(SourceEvent<'_>) -> Result<(), SourceScanError>,
    ) -> Result<(), SourceScanError> {
        on_event(SourceEvent::EndLine {
            ordinal: &self.line_ordinal,
            body_class: self.body_class,
        })?;
        self.line_ordinal.increment()?;
        self.line_started = false;
        self.pending_cr = false;
        self.body_class = LineBodyClass::Empty;
        Ok(())
    }
}

pub(crate) fn scan_source(
    reader: &mut impl Read,
    mut on_event: impl FnMut(SourceEvent<'_>) -> Result<(), SourceScanError>,
) -> Result<(), SourceScanError> {
    let mut scanner = SourceFramer::new()?;
    let mut scratch = [0_u8; READ_BUFFER_SIZE];
    loop {
        let count = reader
            .read(&mut scratch)
            .map_err(|_| SourceScanError::Read)?;
        if count == 0 {
            return scanner.finish(&mut on_event);
        }
        let bytes = &scratch[..count];
        scanner.push(bytes, &mut on_event)?;
    }
}
