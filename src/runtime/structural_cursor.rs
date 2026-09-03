//! Allocation-bounded framing for one forward source observation.

use crate::backwriter::anddress::{
    LineBodyClass, LineTerminator, ParagraphGeometry, ParentGeometry, TargetGeometry,
};

use super::source_scan::SourceScanError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LineSpan {
    pub(crate) byte_start: usize,
    pub(crate) byte_end: usize,
    pub(crate) terminator: LineTerminator,
    pub(crate) body_class: LineBodyClass,
    pub(crate) file_line_offset: usize,
}

impl LineSpan {
    pub(crate) fn file_geometry(self) -> TargetGeometry {
        TargetGeometry::Line {
            byte_start: self.byte_start,
            byte_end: self.byte_end,
            terminator: self.terminator,
            line_offset_in_parent: self.file_line_offset,
            parent: ParentGeometry::File,
        }
    }
}

pub(crate) trait StructuralSink {
    fn source(&mut self, _bytes: &[u8], _byte_start: usize) -> Result<(), SourceScanError> {
        Ok(())
    }

    fn begin_line(
        &mut self,
        _byte_start: usize,
        _file_line_offset: usize,
    ) -> Result<(), SourceScanError> {
        Ok(())
    }

    fn segment(
        &mut self,
        _bytes: &[u8],
        _byte_start: usize,
        _is_content: bool,
    ) -> Result<(), SourceScanError> {
        Ok(())
    }

    fn line(&mut self, _line: LineSpan) -> Result<(), SourceScanError> {
        Ok(())
    }

    fn paragraph(&mut self, _paragraph: ParagraphGeometry) -> Result<(), SourceScanError> {
        Ok(())
    }
}

pub(crate) struct StructuralCursor {
    byte_offset: usize,
    line_start: usize,
    line_count: usize,
    line_started: bool,
    pending_cr: bool,
    body_class: LineBodyClass,
    paragraph_start: usize,
    paragraph_end: usize,
    paragraph_file_line_offset: usize,
    paragraph_line_count: usize,
    in_paragraph: bool,
}

impl Default for StructuralCursor {
    fn default() -> Self {
        Self {
            byte_offset: 0,
            line_start: 0,
            line_count: 0,
            line_started: false,
            pending_cr: false,
            body_class: LineBodyClass::Empty,
            paragraph_start: 0,
            paragraph_end: 0,
            paragraph_file_line_offset: 0,
            paragraph_line_count: 0,
            in_paragraph: false,
        }
    }
}

impl StructuralCursor {
    pub(crate) fn byte_offset(&self) -> usize {
        self.byte_offset
    }

    pub(crate) fn push(
        &mut self,
        bytes: &[u8],
        sink: &mut impl StructuralSink,
    ) -> Result<usize, SourceScanError> {
        let chunk_start = self.byte_offset;
        self.byte_offset
            .checked_add(bytes.len())
            .ok_or(SourceScanError::Resource)?;
        let mut cursor = 0;
        while cursor < bytes.len() {
            if self.pending_cr {
                if bytes[cursor] == b'\n' {
                    self.push_segment(&bytes[cursor..cursor + 1], false, sink)?;
                    cursor += 1;
                    self.finish_line(LineTerminator::Crlf, sink)?;
                    continue;
                }
                self.finish_line(LineTerminator::Cr, sink)?;
            }

            self.begin_line(sink)?;
            let remaining = &bytes[cursor..];
            let content_length = remaining
                .iter()
                .position(|byte| matches!(byte, b'\r' | b'\n'))
                .unwrap_or(remaining.len());
            if content_length != 0 {
                let content = &remaining[..content_length];
                self.include_body(content);
                self.push_segment(content, true, sink)?;
                cursor += content_length;
            }
            if cursor == bytes.len() {
                break;
            }

            let delimiter = bytes[cursor];
            self.push_segment(&bytes[cursor..cursor + 1], false, sink)?;
            cursor += 1;
            if delimiter == b'\r' {
                self.pending_cr = true;
            } else {
                self.finish_line(LineTerminator::Lf, sink)?;
            }
        }
        Ok(chunk_start)
    }

    pub(crate) fn finish(
        mut self,
        sink: &mut impl StructuralSink,
    ) -> Result<(usize, usize), SourceScanError> {
        if self.line_started {
            let terminator = if self.pending_cr {
                LineTerminator::Cr
            } else {
                LineTerminator::None
            };
            self.finish_line(terminator, sink)?;
        }
        self.finish_paragraph(sink)?;
        Ok((self.byte_offset, self.line_count))
    }

    fn begin_line(&mut self, sink: &mut impl StructuralSink) -> Result<(), SourceScanError> {
        if self.line_started {
            return Ok(());
        }
        self.line_started = true;
        self.line_start = self.byte_offset;
        self.body_class = LineBodyClass::Empty;
        sink.begin_line(self.line_start, self.line_count)
    }

    fn push_segment(
        &mut self,
        bytes: &[u8],
        is_content: bool,
        sink: &mut impl StructuralSink,
    ) -> Result<(), SourceScanError> {
        let byte_start = self.byte_offset;
        self.byte_offset = self
            .byte_offset
            .checked_add(bytes.len())
            .ok_or(SourceScanError::Resource)?;
        sink.segment(bytes, byte_start, is_content)
    }

    fn include_body(&mut self, bytes: &[u8]) {
        if bytes.iter().any(|byte| !matches!(byte, b' ' | b'\t')) {
            self.body_class = LineBodyClass::Text;
        } else if !bytes.is_empty() && self.body_class == LineBodyClass::Empty {
            self.body_class = LineBodyClass::HorizontalWhitespace;
        }
    }

    fn finish_line(
        &mut self,
        terminator: LineTerminator,
        sink: &mut impl StructuralSink,
    ) -> Result<(), SourceScanError> {
        let terminator_length = match terminator {
            LineTerminator::None => 0,
            LineTerminator::Lf | LineTerminator::Cr => 1,
            LineTerminator::Crlf => 2,
        };
        self.byte_offset
            .checked_sub(terminator_length)
            .ok_or(SourceScanError::InvalidSource)?;
        let line = LineSpan {
            byte_start: self.line_start,
            byte_end: self.byte_offset,
            terminator,
            body_class: self.body_class,
            file_line_offset: self.line_count,
        };
        sink.line(line)?;

        if self.body_class == LineBodyClass::Text {
            if !self.in_paragraph {
                self.in_paragraph = true;
                self.paragraph_start = self.line_start;
                self.paragraph_file_line_offset = self.line_count;
                self.paragraph_line_count = 0;
            }
            self.paragraph_end = self.byte_offset;
            self.paragraph_line_count = self
                .paragraph_line_count
                .checked_add(1)
                .ok_or(SourceScanError::Resource)?;
        } else {
            self.finish_paragraph(sink)?;
        }

        self.line_count = self
            .line_count
            .checked_add(1)
            .ok_or(SourceScanError::Resource)?;
        self.line_started = false;
        self.pending_cr = false;
        self.body_class = LineBodyClass::Empty;
        Ok(())
    }

    fn finish_paragraph(&mut self, sink: &mut impl StructuralSink) -> Result<(), SourceScanError> {
        if !self.in_paragraph {
            return Ok(());
        }
        sink.paragraph(ParagraphGeometry {
            byte_start: self.paragraph_start,
            byte_end: self.paragraph_end,
            file_line_offset: self.paragraph_file_line_offset,
            line_count: self.paragraph_line_count,
        })?;
        self.in_paragraph = false;
        self.paragraph_line_count = 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Capture {
        segments: usize,
        lines: Vec<LineSpan>,
        paragraphs: Vec<ParagraphGeometry>,
    }

    impl StructuralSink for Capture {
        fn segment(
            &mut self,
            bytes: &[u8],
            _byte_start: usize,
            _is_content: bool,
        ) -> Result<(), SourceScanError> {
            self.segments += bytes.len();
            Ok(())
        }

        fn line(&mut self, line: LineSpan) -> Result<(), SourceScanError> {
            self.lines.push(line);
            Ok(())
        }

        fn paragraph(&mut self, paragraph: ParagraphGeometry) -> Result<(), SourceScanError> {
            self.paragraphs.push(paragraph);
            Ok(())
        }
    }

    #[test]
    fn frames_every_body_and_terminator_across_chunk_boundaries() {
        let mut cursor = StructuralCursor::default();
        let mut capture = Capture::default();
        for chunk in [
            b"\n ".as_slice(),
            b"\t\rtext\r".as_slice(),
            b"\nlast".as_slice(),
        ] {
            cursor.push(chunk, &mut capture).unwrap();
        }
        assert_eq!(cursor.finish(&mut capture).unwrap(), (14, 4));
        assert_eq!(
            capture.lines,
            [
                LineSpan {
                    byte_start: 0,
                    byte_end: 1,
                    terminator: LineTerminator::Lf,
                    body_class: LineBodyClass::Empty,
                    file_line_offset: 0,
                },
                LineSpan {
                    byte_start: 1,
                    byte_end: 4,
                    terminator: LineTerminator::Cr,
                    body_class: LineBodyClass::HorizontalWhitespace,
                    file_line_offset: 1,
                },
                LineSpan {
                    byte_start: 4,
                    byte_end: 10,
                    terminator: LineTerminator::Crlf,
                    body_class: LineBodyClass::Text,
                    file_line_offset: 2,
                },
                LineSpan {
                    byte_start: 10,
                    byte_end: 14,
                    terminator: LineTerminator::None,
                    body_class: LineBodyClass::Text,
                    file_line_offset: 3,
                },
            ]
        );
        assert_eq!(
            capture.paragraphs,
            [ParagraphGeometry {
                byte_start: 4,
                byte_end: 14,
                file_line_offset: 2,
                line_count: 2,
            }]
        );
    }

    #[test]
    fn framing_is_identical_at_each_fixed_scratch_edge() {
        for text_length in [8_191, 8_192, 8_193] {
            let mut source = vec![b'x'; text_length];
            source.extend_from_slice(b"\r\n \t\ny");
            let mut cursor = StructuralCursor::default();
            let mut capture = Capture::default();
            for chunk in source.chunks(8_192) {
                cursor.push(chunk, &mut capture).unwrap();
            }
            assert_eq!(cursor.finish(&mut capture).unwrap(), (source.len(), 3));
            assert_eq!(capture.lines[0].byte_end, text_length + 2);
            assert_eq!(capture.lines[0].terminator, LineTerminator::Crlf);
            assert_eq!(
                capture.lines[1].body_class,
                LineBodyClass::HorizontalWhitespace
            );
            assert_eq!(capture.lines[2].terminator, LineTerminator::None);
            assert_eq!(
                capture.paragraphs,
                [
                    ParagraphGeometry {
                        byte_start: 0,
                        byte_end: text_length + 2,
                        file_line_offset: 0,
                        line_count: 1,
                    },
                    ParagraphGeometry {
                        byte_start: text_length + 5,
                        byte_end: text_length + 6,
                        file_line_offset: 2,
                        line_count: 1,
                    },
                ]
            );
        }
    }

    #[test]
    fn checked_offsets_fail_before_consuming_an_overflowing_chunk() {
        let mut cursor = StructuralCursor {
            byte_offset: usize::MAX,
            ..StructuralCursor::default()
        };
        let mut capture = Capture::default();
        assert_eq!(
            cursor.push(b"x", &mut capture),
            Err(SourceScanError::Resource)
        );
        assert_eq!(capture.segments, 0);
        assert!(capture.lines.is_empty());
        assert!(capture.paragraphs.is_empty());
    }
}
