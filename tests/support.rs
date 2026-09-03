#![allow(dead_code)]

use backwriter::backwriter::anddress::{Anddress, AnddressTarget};
use sha2::{Digest, Sha256};

pub fn source_hash(bytes: &[u8]) -> String {
    use std::fmt::Write as _;

    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(64);
    for byte in digest {
        write!(output, "{byte:02x}").unwrap();
    }
    output
}

pub fn file(workspace_coordinate: &str, logical_path: &str, source: &[u8]) -> Anddress {
    address(
        workspace_coordinate,
        logical_path,
        source,
        AnddressTarget::File,
        0,
        source.len(),
    )
}

pub fn line(
    workspace_coordinate: &str,
    logical_path: &str,
    source: &[u8],
    ordinal: usize,
) -> Anddress {
    let (start, end) = line_spans(source)[ordinal];
    address(
        workspace_coordinate,
        logical_path,
        source,
        AnddressTarget::Line,
        start,
        end,
    )
}

pub fn paragraph(
    workspace_coordinate: &str,
    logical_path: &str,
    source: &[u8],
    ordinal: usize,
) -> Anddress {
    let lines = line_spans(source);
    let mut paragraphs = Vec::new();
    let mut start = None;
    let mut end = 0;
    for (line_start, line_end) in lines {
        let body = line_body(&source[line_start..line_end]);
        let text = body.iter().any(|byte| !matches!(byte, b' ' | b'\t'));
        if text {
            start.get_or_insert(line_start);
            end = line_end;
        } else if let Some(paragraph_start) = start.take() {
            paragraphs.push((paragraph_start, end));
        }
    }
    if let Some(paragraph_start) = start {
        paragraphs.push((paragraph_start, end));
    }
    let (start, end) = paragraphs[ordinal];
    address(
        workspace_coordinate,
        logical_path,
        source,
        AnddressTarget::Paragraph,
        start,
        end,
    )
}

pub fn line_spans(source: &[u8]) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut start = 0;
    let mut index = 0;
    while index < source.len() {
        match source[index] {
            b'\r' if source.get(index + 1) == Some(&b'\n') => {
                index += 2;
                spans.push((start, index));
                start = index;
            }
            b'\r' | b'\n' => {
                index += 1;
                spans.push((start, index));
                start = index;
            }
            _ => index += 1,
        }
    }
    if start < source.len() {
        spans.push((start, source.len()));
    }
    spans
}

fn line_body(line: &[u8]) -> &[u8] {
    if line.ends_with(b"\r\n") {
        &line[..line.len() - 2]
    } else if line.ends_with(b"\r") || line.ends_with(b"\n") {
        &line[..line.len() - 1]
    } else {
        line
    }
}

pub fn address(
    workspace_coordinate: &str,
    logical_path: &str,
    source: &[u8],
    target: AnddressTarget,
    byte_start: usize,
    byte_end: usize,
) -> Anddress {
    let lines = line_spans(source);
    let source_fields = format!(
        r#""version":"artext.backwriter-anddress.v5","workspaceCoordinate":{},"logicalPath":{},"sourceStateHash":"{}","sourceByteLength":"{}","sourceLineCount":"{}","kind":"{}""#,
        serde_json::to_string(workspace_coordinate).unwrap(),
        serde_json::to_string(logical_path).unwrap(),
        source_hash(source),
        source.len(),
        lines.len(),
        match target {
            AnddressTarget::File => "file",
            AnddressTarget::Paragraph => "paragraph",
            AnddressTarget::Line => "line",
        },
    );
    let geometry = match target {
        AnddressTarget::File => String::new(),
        AnddressTarget::Paragraph => {
            let (file_line_offset, line_count) = paragraph_geometry(source, byte_start, byte_end);
            format!(
                r#","byteStart":"{byte_start}","byteEnd":"{byte_end}","fileLineOffset":"{file_line_offset}","lineCount":"{line_count}""#
            )
        }
        AnddressTarget::Line => {
            let exact = lines
                .iter()
                .position(|&(start, end)| start == byte_start && end == byte_end);
            let file_line_offset = exact.unwrap_or_else(|| {
                lines
                    .iter()
                    .position(|&(_, end)| byte_start < end)
                    .unwrap_or_else(|| lines.len().saturating_sub(1))
            });
            let terminator = exact
                .map(|_| line_terminator(&source[byte_start..byte_end]))
                .unwrap_or("none");
            let paragraph = exact.and_then(|line_index| {
                line_body(&source[byte_start..byte_end])
                    .iter()
                    .any(|byte| !matches!(byte, b' ' | b'\t'))
                    .then(|| containing_paragraph(source, line_index))
            });
            match paragraph {
                Some((parent_start, parent_end, parent_offset, parent_count)) => format!(
                    r#","byteStart":"{byte_start}","byteEnd":"{byte_end}","terminator":"{terminator}","lineOffsetInParent":"{}","parentKind":"paragraph","parentByteStart":"{parent_start}","parentByteEnd":"{parent_end}","parentFileLineOffset":"{parent_offset}","parentLineCount":"{parent_count}""#,
                    file_line_offset - parent_offset,
                ),
                None => format!(
                    r#","byteStart":"{byte_start}","byteEnd":"{byte_end}","terminator":"{terminator}","lineOffsetInParent":"{file_line_offset}","parentKind":"file""#
                ),
            }
        }
    };
    Anddress::decode(format!("{{{source_fields}{geometry}}}").as_bytes()).unwrap()
}

fn paragraph_geometry(source: &[u8], start: usize, end: usize) -> (usize, usize) {
    let lines = line_spans(source);
    let first = lines
        .iter()
        .position(|&(line_start, _)| line_start == start)
        .unwrap_or(0);
    let count = lines[first..]
        .iter()
        .take_while(|&&(_, line_end)| line_end <= end)
        .count()
        .max(1);
    (first, count)
}

fn containing_paragraph(source: &[u8], line_index: usize) -> (usize, usize, usize, usize) {
    let lines = line_spans(source);
    let is_text = |index: usize| {
        let (start, end) = lines[index];
        line_body(&source[start..end])
            .iter()
            .any(|byte| !matches!(byte, b' ' | b'\t'))
    };
    let mut first = line_index;
    while first != 0 && is_text(first - 1) {
        first -= 1;
    }
    let mut last = line_index + 1;
    while last < lines.len() && is_text(last) {
        last += 1;
    }
    (lines[first].0, lines[last - 1].1, first, last - first)
}

fn line_terminator(line: &[u8]) -> &'static str {
    if line.ends_with(b"\r\n") {
        "crlf"
    } else if line.ends_with(b"\r") {
        "cr"
    } else if line.ends_with(b"\n") {
        "lf"
    } else {
        "none"
    }
}
