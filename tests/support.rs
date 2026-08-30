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
    Anddress::new(
        workspace_coordinate,
        logical_path,
        &source_hash(source),
        source.len(),
        target,
        byte_start,
        byte_end,
    )
    .unwrap()
}
