//! Compact exact-source-state v5 structural addresses.

use std::{fmt, ops::Range, sync::Arc};

use serde::de::{IgnoredAny, MapAccess, Visitor};
use thiserror::Error;

use crate::hash::is_lower_hex_sha256;
use crate::source::validate_logical_path;

pub const ANDDRESS_VERSION: &str = "artext.backwriter-anddress.v5";

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum AnddressError {
    #[error("Anddress input is invalid")]
    Invalid,
    #[error("Anddress encoding is invalid")]
    Encoding,
    #[error("Anddress version is unsupported")]
    UnsupportedVersion,
    #[error("Anddress resource allocation failed")]
    Resource,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineTerminator {
    None,
    Lf,
    Cr,
    Crlf,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LineBodyClass {
    Empty,
    HorizontalWhitespace,
    Text,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnddressTarget {
    File,
    Paragraph,
    Line,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct SourceIdentity {
    workspace_coordinate: String,
    logical_path: String,
    source_state_hash: String,
    source_byte_length: usize,
    source_line_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ParagraphGeometry {
    pub(crate) byte_start: usize,
    pub(crate) byte_end: usize,
    pub(crate) file_line_offset: usize,
    pub(crate) line_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ParentGeometry {
    File,
    Paragraph(ParagraphGeometry),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TargetGeometry {
    File,
    Paragraph(ParagraphGeometry),
    Line {
        byte_start: usize,
        byte_end: usize,
        terminator: LineTerminator,
        line_offset_in_parent: usize,
        parent: ParentGeometry,
    },
}

pub(crate) fn attach_line_to_paragraph(
    geometry: &mut TargetGeometry,
    paragraph: ParagraphGeometry,
) -> Result<bool, AnddressError> {
    let TargetGeometry::Line {
        byte_start,
        byte_end,
        line_offset_in_parent,
        parent,
        ..
    } = *geometry
    else {
        return Ok(false);
    };
    if byte_start < paragraph.byte_start || byte_end > paragraph.byte_end {
        return Ok(false);
    }
    let Some(parent_offset) = line_offset_in_parent.checked_sub(paragraph.file_line_offset) else {
        return Err(AnddressError::Invalid);
    };
    if byte_start > byte_end
        || paragraph.byte_start >= paragraph.byte_end
        || paragraph.line_count == 0
        || paragraph
            .file_line_offset
            .checked_add(paragraph.line_count)
            .is_none()
        || parent_offset >= paragraph.line_count
        || parent != ParentGeometry::File
    {
        return Err(AnddressError::Invalid);
    }
    let TargetGeometry::Line {
        line_offset_in_parent,
        parent,
        ..
    } = geometry
    else {
        unreachable!("validated Line geometry changed before attachment")
    };
    *line_offset_in_parent = parent_offset;
    *parent = ParentGeometry::Paragraph(paragraph);
    Ok(true)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Anddress {
    source: Arc<SourceIdentity>,
    geometry: TargetGeometry,
}

pub(crate) struct AnddressIssuer {
    source: Arc<SourceIdentity>,
}

impl AnddressIssuer {
    pub(crate) fn new(
        workspace_coordinate: &str,
        logical_path: &str,
        source_state_hash: &str,
        source_byte_length: usize,
        source_line_count: usize,
    ) -> Result<Self, AnddressError> {
        Self::from_owned_source(SourceIdentity {
            workspace_coordinate: fallible_copy(workspace_coordinate)?,
            logical_path: fallible_copy(logical_path)?,
            source_state_hash: fallible_copy(source_state_hash)?,
            source_byte_length,
            source_line_count,
        })
    }

    fn from_owned_source(source: SourceIdentity) -> Result<Self, AnddressError> {
        validate_source(&source)?;
        Ok(Self {
            source: Arc::new(source),
        })
    }

    pub(crate) fn from_source(source: &Arc<SourceIdentity>) -> Self {
        Self {
            source: Arc::clone(source),
        }
    }

    pub(crate) fn issue(&self, geometry: TargetGeometry) -> Result<Anddress, AnddressError> {
        validate_geometry(&self.source, geometry)?;
        Ok(Anddress {
            source: Arc::clone(&self.source),
            geometry,
        })
    }
}

impl Anddress {
    pub fn version(&self) -> &'static str {
        ANDDRESS_VERSION
    }

    pub fn workspace_coordinate(&self) -> &str {
        &self.source.workspace_coordinate
    }

    pub fn logical_path(&self) -> &str {
        &self.source.logical_path
    }

    pub fn source_state_hash(&self) -> &str {
        &self.source.source_state_hash
    }

    pub fn source_byte_length(&self) -> usize {
        self.source.source_byte_length
    }

    pub fn source_line_count(&self) -> usize {
        self.source.source_line_count
    }

    pub fn target(&self) -> AnddressTarget {
        match self.geometry {
            TargetGeometry::File => AnddressTarget::File,
            TargetGeometry::Paragraph(_) => AnddressTarget::Paragraph,
            TargetGeometry::Line { .. } => AnddressTarget::Line,
        }
    }

    pub fn byte_start(&self) -> usize {
        match self.geometry {
            TargetGeometry::File => 0,
            TargetGeometry::Paragraph(paragraph) => paragraph.byte_start,
            TargetGeometry::Line { byte_start, .. } => byte_start,
        }
    }

    pub fn byte_end(&self) -> usize {
        match self.geometry {
            TargetGeometry::File => self.source_byte_length(),
            TargetGeometry::Paragraph(paragraph) => paragraph.byte_end,
            TargetGeometry::Line { byte_end, .. } => byte_end,
        }
    }

    pub fn range(&self) -> Range<usize> {
        self.byte_start()..self.byte_end()
    }

    pub fn line_count(&self) -> usize {
        match self.geometry {
            TargetGeometry::File => self.source_line_count(),
            TargetGeometry::Paragraph(paragraph) => paragraph.line_count,
            TargetGeometry::Line { .. } => 1,
        }
    }

    pub fn line_range(&self) -> Range<usize> {
        let start = match self.geometry {
            TargetGeometry::File => 0,
            TargetGeometry::Paragraph(paragraph) => paragraph.file_line_offset,
            TargetGeometry::Line {
                line_offset_in_parent,
                parent,
                ..
            } => match parent {
                ParentGeometry::File => line_offset_in_parent,
                ParentGeometry::Paragraph(paragraph) => {
                    paragraph.file_line_offset + line_offset_in_parent
                }
            },
        };
        start..start + self.line_count()
    }

    pub fn line_number(&self) -> Option<usize> {
        (self.target() == AnddressTarget::Line).then(|| self.line_range().start + 1)
    }

    pub fn terminator(&self) -> Option<LineTerminator> {
        match self.geometry {
            TargetGeometry::Line { terminator, .. } => Some(terminator),
            _ => None,
        }
    }

    pub fn same_source(&self, other: &Self) -> bool {
        self.workspace_coordinate() == other.workspace_coordinate()
            && self.logical_path() == other.logical_path()
    }

    pub fn same_state(&self, other: &Self) -> bool {
        self.source == other.source
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.same_state(other)
            && self.byte_start() <= other.byte_start()
            && other.byte_end() <= self.byte_end()
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.same_state(other)
            && self.byte_start() < other.byte_end()
            && other.byte_start() < self.byte_end()
    }

    pub fn parent(&self) -> Option<Self> {
        let geometry = match self.geometry {
            TargetGeometry::File => return None,
            TargetGeometry::Paragraph(_) => TargetGeometry::File,
            TargetGeometry::Line { parent, .. } => match parent {
                ParentGeometry::File => TargetGeometry::File,
                ParentGeometry::Paragraph(paragraph) => TargetGeometry::Paragraph(paragraph),
            },
        };
        Some(
            AnddressIssuer::from_source(&self.source)
                .issue(geometry)
                .expect("validated parent geometry remains valid"),
        )
    }

    pub fn projection_valid(&self, target: AnddressTarget) -> bool {
        matches!(
            (self.target(), target),
            (AnddressTarget::File, AnddressTarget::File)
                | (
                    AnddressTarget::Paragraph,
                    AnddressTarget::Paragraph | AnddressTarget::File
                )
                | (
                    AnddressTarget::Line,
                    AnddressTarget::Line | AnddressTarget::Paragraph | AnddressTarget::File
                )
        )
    }

    pub fn project(&self, target: AnddressTarget) -> Result<Option<Self>, AnddressError> {
        if !self.projection_valid(target) {
            return Err(AnddressError::Invalid);
        }
        if target == self.target() {
            return Ok(Some(self.clone()));
        }
        if target == AnddressTarget::File {
            return Ok(self.ancestor(AnddressTarget::File));
        }
        match self.geometry {
            TargetGeometry::Line {
                parent: ParentGeometry::File,
                ..
            } => Ok(None),
            TargetGeometry::Line {
                parent: ParentGeometry::Paragraph(_),
                ..
            } => Ok(self.parent()),
            _ => Err(AnddressError::Invalid),
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, AnddressError> {
        let mut output = Vec::new();
        self.encode_into(&mut output)?;
        Ok(output)
    }

    /// Replaces `output` with this address's canonical v5 JSON object.
    pub fn encode_into(&self, output: &mut Vec<u8>) -> Result<(), AnddressError> {
        output.clear();
        let mut length = EncodedLength(0);
        encode_to(self, &mut length)?;
        output
            .try_reserve(length.0)
            .map_err(|_| AnddressError::Resource)?;
        encode_to(self, output)?;
        debug_assert_eq!(output.len(), length.0);
        Ok(())
    }

    pub fn decode(encoded: &[u8]) -> Result<Self, AnddressError> {
        let wire: Wire = serde_json::from_slice(encoded).map_err(|_| AnddressError::Encoding)?;
        wire.into_anddress()
    }

    pub fn validate(&self) -> Result<(), AnddressError> {
        validate_address(&self.source, self.geometry)
    }

    #[cfg(test)]
    pub(crate) fn source_identity(&self) -> &Arc<SourceIdentity> {
        &self.source
    }

    #[cfg(test)]
    pub(crate) fn geometry(&self) -> TargetGeometry {
        self.geometry
    }

    fn ancestor(&self, target: AnddressTarget) -> Option<Self> {
        let geometry = match target {
            AnddressTarget::File => TargetGeometry::File,
            AnddressTarget::Paragraph => match self.geometry {
                TargetGeometry::Paragraph(paragraph)
                | TargetGeometry::Line {
                    parent: ParentGeometry::Paragraph(paragraph),
                    ..
                } => TargetGeometry::Paragraph(paragraph),
                _ => return None,
            },
            AnddressTarget::Line => return None,
        };
        Some(
            AnddressIssuer::from_source(&self.source)
                .issue(geometry)
                .expect("validated ancestor geometry remains valid"),
        )
    }
}

impl AnddressTarget {
    fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Paragraph => "paragraph",
            Self::Line => "line",
        }
    }
}

impl LineTerminator {
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Lf => "lf",
            Self::Cr => "cr",
            Self::Crlf => "crlf",
        }
    }

    fn byte_length(self) -> usize {
        match self {
            Self::None => 0,
            Self::Lf | Self::Cr => 1,
            Self::Crlf => 2,
        }
    }
}

fn validate_source(source: &SourceIdentity) -> Result<(), AnddressError> {
    if !is_lower_hex_sha256(&source.workspace_coordinate)
        || validate_logical_path(&source.logical_path).is_err()
        || !is_lower_hex_sha256(&source.source_state_hash)
        || (source.source_byte_length == 0 && source.source_line_count != 0)
        || (source.source_byte_length != 0
            && (source.source_line_count == 0
                || source.source_line_count > source.source_byte_length))
    {
        return Err(AnddressError::Invalid);
    }
    Ok(())
}

fn validate_address(
    source: &SourceIdentity,
    geometry: TargetGeometry,
) -> Result<(), AnddressError> {
    validate_source(source)?;
    validate_geometry(source, geometry)
}

fn validate_geometry(
    source: &SourceIdentity,
    geometry: TargetGeometry,
) -> Result<(), AnddressError> {
    match geometry {
        TargetGeometry::File => Ok(()),
        TargetGeometry::Paragraph(paragraph) => validate_paragraph(source, paragraph),
        TargetGeometry::Line {
            byte_start,
            byte_end,
            terminator,
            line_offset_in_parent,
            parent,
        } => {
            if byte_start > byte_end
                || byte_end > source.source_byte_length
                || terminator.byte_length() > byte_end - byte_start
                || (byte_start == byte_end && terminator != LineTerminator::None)
            {
                return Err(AnddressError::Invalid);
            }
            match parent {
                ParentGeometry::File => {
                    if line_offset_in_parent >= source.source_line_count {
                        return Err(AnddressError::Invalid);
                    }
                }
                ParentGeometry::Paragraph(paragraph) => {
                    validate_paragraph(source, paragraph)?;
                    if line_offset_in_parent >= paragraph.line_count
                        || byte_start < paragraph.byte_start
                        || byte_end > paragraph.byte_end
                    {
                        return Err(AnddressError::Invalid);
                    }
                    paragraph
                        .file_line_offset
                        .checked_add(line_offset_in_parent)
                        .ok_or(AnddressError::Invalid)?;
                }
            }
            Ok(())
        }
    }
}

fn validate_paragraph(
    source: &SourceIdentity,
    paragraph: ParagraphGeometry,
) -> Result<(), AnddressError> {
    if paragraph.byte_start >= paragraph.byte_end
        || paragraph.byte_end > source.source_byte_length
        || paragraph.line_count == 0
        || paragraph.byte_end - paragraph.byte_start < paragraph.line_count
        || paragraph
            .file_line_offset
            .checked_add(paragraph.line_count)
            .is_none_or(|end| end > source.source_line_count)
    {
        return Err(AnddressError::Invalid);
    }
    Ok(())
}

fn fallible_copy(value: &str) -> Result<String, AnddressError> {
    let mut copy = String::new();
    copy.try_reserve(value.len())
        .map_err(|_| AnddressError::Resource)?;
    copy.push_str(value);
    Ok(copy)
}

trait EncodeSink {
    fn push(&mut self, bytes: &[u8]) -> Result<(), AnddressError>;
}

struct EncodedLength(usize);

impl EncodeSink for EncodedLength {
    fn push(&mut self, bytes: &[u8]) -> Result<(), AnddressError> {
        self.0 = self
            .0
            .checked_add(bytes.len())
            .ok_or(AnddressError::Resource)?;
        Ok(())
    }
}

impl EncodeSink for Vec<u8> {
    fn push(&mut self, bytes: &[u8]) -> Result<(), AnddressError> {
        self.extend_from_slice(bytes);
        Ok(())
    }
}

fn encode_to(anddress: &Anddress, output: &mut impl EncodeSink) -> Result<(), AnddressError> {
    output.push(br#"{"version":"artext.backwriter-anddress.v5","workspaceCoordinate":""#)?;
    output.push(anddress.workspace_coordinate().as_bytes())?;
    output.push(br#"","logicalPath":""#)?;
    append_json_path(output, anddress.logical_path())?;
    output.push(br#"","sourceStateHash":""#)?;
    output.push(anddress.source_state_hash().as_bytes())?;
    append_decimal_field(
        output,
        br#"","sourceByteLength":""#,
        anddress.source_byte_length(),
    )?;
    append_decimal_field(
        output,
        br#","sourceLineCount":""#,
        anddress.source_line_count(),
    )?;
    output.push(br#","kind":""#)?;
    output.push(anddress.target().as_str().as_bytes())?;
    output.push(b"\"")?;
    match anddress.geometry {
        TargetGeometry::File => {}
        TargetGeometry::Paragraph(paragraph) => append_paragraph(output, false, paragraph)?,
        TargetGeometry::Line {
            byte_start,
            byte_end,
            terminator,
            line_offset_in_parent,
            parent,
        } => {
            append_decimal_field(output, br#","byteStart":""#, byte_start)?;
            append_decimal_field(output, br#","byteEnd":""#, byte_end)?;
            append_field(output, br#","terminator":""#, terminator.as_str())?;
            append_decimal_field(output, br#","lineOffsetInParent":""#, line_offset_in_parent)?;
            match parent {
                ParentGeometry::File => append_field(output, br#","parentKind":""#, "file")?,
                ParentGeometry::Paragraph(paragraph) => {
                    append_field(output, br#","parentKind":""#, "paragraph")?;
                    append_paragraph(output, true, paragraph)?;
                }
            }
        }
    }
    output.push(b"}")
}

fn append_json_path(output: &mut impl EncodeSink, path: &str) -> Result<(), AnddressError> {
    let mut start = 0;
    for (index, _) in path.match_indices('"') {
        output.push(&path.as_bytes()[start..index])?;
        output.push(br#"\""#)?;
        start = index + 1;
    }
    output.push(&path.as_bytes()[start..])
}

fn append_decimal(output: &mut impl EncodeSink, mut value: usize) -> Result<(), AnddressError> {
    let mut digits = [0_u8; 3 * std::mem::size_of::<usize>()];
    let mut start = digits.len();
    loop {
        start -= 1;
        digits[start] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    output.push(&digits[start..])
}

fn append_field(
    output: &mut impl EncodeSink,
    prefix: &[u8],
    value: &str,
) -> Result<(), AnddressError> {
    output.push(prefix)?;
    output.push(value.as_bytes())?;
    output.push(b"\"")
}

fn append_decimal_field(
    output: &mut impl EncodeSink,
    prefix: &[u8],
    value: usize,
) -> Result<(), AnddressError> {
    output.push(prefix)?;
    append_decimal(output, value)?;
    output.push(b"\"")
}

fn append_paragraph(
    output: &mut impl EncodeSink,
    parent: bool,
    paragraph: ParagraphGeometry,
) -> Result<(), AnddressError> {
    let prefixes = if parent {
        [
            br#","parentByteStart":""# as &[u8],
            br#","parentByteEnd":""#,
            br#","parentFileLineOffset":""#,
            br#","parentLineCount":""#,
        ]
    } else {
        [
            br#","byteStart":""# as &[u8],
            br#","byteEnd":""#,
            br#","fileLineOffset":""#,
            br#","lineCount":""#,
        ]
    };
    for (prefix, value) in prefixes.into_iter().zip([
        paragraph.byte_start,
        paragraph.byte_end,
        paragraph.file_line_offset,
        paragraph.line_count,
    ]) {
        append_decimal_field(output, prefix, value)?;
    }
    Ok(())
}

#[derive(Default)]
struct Wire {
    version: Option<WireString>,
    workspace_coordinate: Option<WireString>,
    logical_path: Option<WireString>,
    source_state_hash: Option<WireString>,
    source_byte_length: Option<WireString>,
    source_line_count: Option<WireString>,
    kind: Option<WireString>,
    byte_start: Option<WireString>,
    byte_end: Option<WireString>,
    file_line_offset: Option<WireString>,
    line_count: Option<WireString>,
    terminator: Option<WireString>,
    line_offset_in_parent: Option<WireString>,
    parent_kind: Option<WireString>,
    parent_byte_start: Option<WireString>,
    parent_byte_end: Option<WireString>,
    parent_file_line_offset: Option<WireString>,
    parent_line_count: Option<WireString>,
    invalid: bool,
    duplicate_version: bool,
}

impl Wire {
    fn into_anddress(mut self) -> Result<Anddress, AnddressError> {
        if self.duplicate_version {
            return Err(AnddressError::Encoding);
        }
        let version = wire_string(self.version.take())?;
        if version != ANDDRESS_VERSION {
            return Err(AnddressError::UnsupportedVersion);
        }
        if self.invalid {
            return Err(AnddressError::Encoding);
        }
        let source = SourceIdentity {
            workspace_coordinate: wire_string(self.workspace_coordinate.take())?,
            logical_path: wire_string(self.logical_path.take())?,
            source_state_hash: wire_string(self.source_state_hash.take())?,
            source_byte_length: wire_usize(self.source_byte_length.take())?,
            source_line_count: wire_usize(self.source_line_count.take())?,
        };
        let geometry = match wire_string(self.kind.take())?.as_str() {
            "file" => {
                self.require_absent()?;
                TargetGeometry::File
            }
            "paragraph" => {
                let paragraph = self.take_paragraph(false)?;
                self.require_absent()?;
                TargetGeometry::Paragraph(paragraph)
            }
            "line" => {
                let byte_start = wire_usize(self.byte_start.take())?;
                let byte_end = wire_usize(self.byte_end.take())?;
                let terminator = match wire_string(self.terminator.take())?.as_str() {
                    "none" => LineTerminator::None,
                    "lf" => LineTerminator::Lf,
                    "cr" => LineTerminator::Cr,
                    "crlf" => LineTerminator::Crlf,
                    _ => return Err(AnddressError::Encoding),
                };
                let line_offset_in_parent = wire_usize(self.line_offset_in_parent.take())?;
                let parent = match wire_string(self.parent_kind.take())?.as_str() {
                    "file" => ParentGeometry::File,
                    "paragraph" => ParentGeometry::Paragraph(self.take_paragraph(true)?),
                    _ => return Err(AnddressError::Encoding),
                };
                self.require_absent()?;
                TargetGeometry::Line {
                    byte_start,
                    byte_end,
                    terminator,
                    line_offset_in_parent,
                    parent,
                }
            }
            _ => return Err(AnddressError::Encoding),
        };
        AnddressIssuer::from_owned_source(source)?.issue(geometry)
    }

    fn take_paragraph(&mut self, parent: bool) -> Result<ParagraphGeometry, AnddressError> {
        let (byte_start, byte_end, file_line_offset, line_count) = if parent {
            (
                &mut self.parent_byte_start,
                &mut self.parent_byte_end,
                &mut self.parent_file_line_offset,
                &mut self.parent_line_count,
            )
        } else {
            (
                &mut self.byte_start,
                &mut self.byte_end,
                &mut self.file_line_offset,
                &mut self.line_count,
            )
        };
        Ok(ParagraphGeometry {
            byte_start: wire_usize(byte_start.take())?,
            byte_end: wire_usize(byte_end.take())?,
            file_line_offset: wire_usize(file_line_offset.take())?,
            line_count: wire_usize(line_count.take())?,
        })
    }

    fn require_absent(&self) -> Result<(), AnddressError> {
        [
            &self.byte_start,
            &self.byte_end,
            &self.file_line_offset,
            &self.line_count,
            &self.terminator,
            &self.line_offset_in_parent,
            &self.parent_kind,
            &self.parent_byte_start,
            &self.parent_byte_end,
            &self.parent_file_line_offset,
            &self.parent_line_count,
        ]
        .iter()
        .all(|field| field.is_none())
        .then_some(())
        .ok_or(AnddressError::Encoding)
    }
}

fn wire_string(value: Option<WireString>) -> Result<String, AnddressError> {
    match value {
        Some(WireString::String(value)) => Ok(value),
        _ => Err(AnddressError::Encoding),
    }
}
fn wire_usize(value: Option<WireString>) -> Result<usize, AnddressError> {
    let value = wire_string(value)?;
    if value.is_empty()
        || !value.bytes().all(|byte| byte.is_ascii_digit())
        || (value.len() > 1 && value.starts_with('0'))
    {
        return Err(AnddressError::Encoding);
    }
    value.bytes().try_fold(0_usize, |result, byte| {
        result
            .checked_mul(10)
            .and_then(|result| result.checked_add(usize::from(byte - b'0')))
            .ok_or(AnddressError::Encoding)
    })
}

enum WireString {
    String(String),
    NonString,
}

impl<'de> serde::Deserialize<'de> for WireString {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct WireStringVisitor;
        impl<'de> Visitor<'de> for WireStringVisitor {
            type Value = WireString;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a JSON string or another JSON value")
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<WireString, E> {
                Ok(WireString::String(value.to_owned()))
            }

            fn visit_string<E: serde::de::Error>(self, value: String) -> Result<WireString, E> {
                Ok(WireString::String(value))
            }

            fn visit_bool<E: serde::de::Error>(self, _value: bool) -> Result<WireString, E> {
                Ok(WireString::NonString)
            }

            fn visit_i64<E: serde::de::Error>(self, _value: i64) -> Result<WireString, E> {
                Ok(WireString::NonString)
            }

            fn visit_u64<E: serde::de::Error>(self, _value: u64) -> Result<WireString, E> {
                Ok(WireString::NonString)
            }

            fn visit_f64<E: serde::de::Error>(self, _value: f64) -> Result<WireString, E> {
                Ok(WireString::NonString)
            }

            fn visit_unit<E: serde::de::Error>(self) -> Result<WireString, E> {
                Ok(WireString::NonString)
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut sequence: A,
            ) -> Result<WireString, A::Error> {
                while sequence.next_element::<IgnoredAny>()?.is_some() {}
                Ok(WireString::NonString)
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<WireString, A::Error> {
                while map.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
                Ok(WireString::NonString)
            }
        }
        deserializer.deserialize_any(WireStringVisitor)
    }
}

fn read_wire_value<'de, M: MapAccess<'de>>(
    map: &mut M,
    slot: &mut Option<WireString>,
    duplicate: &mut bool,
) -> Result<(), M::Error> {
    if slot.is_some() {
        *duplicate = true;
        map.next_value::<IgnoredAny>()?;
    } else {
        *slot = Some(map.next_value::<WireString>()?);
    }
    Ok(())
}

impl<'de> serde::Deserialize<'de> for Wire {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct WireVisitor;
        impl<'de> Visitor<'de> for WireVisitor {
            type Value = Wire;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a v5 Anddress object")
            }
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Wire, M::Error> {
                let mut wire = Wire::default();
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "version" => read_wire_value(
                            &mut map,
                            &mut wire.version,
                            &mut wire.duplicate_version,
                        )?,
                        "workspaceCoordinate" => read_wire_value(
                            &mut map,
                            &mut wire.workspace_coordinate,
                            &mut wire.invalid,
                        )?,
                        "logicalPath" => {
                            read_wire_value(&mut map, &mut wire.logical_path, &mut wire.invalid)?;
                        }
                        "sourceStateHash" => read_wire_value(
                            &mut map,
                            &mut wire.source_state_hash,
                            &mut wire.invalid,
                        )?,
                        "sourceByteLength" => read_wire_value(
                            &mut map,
                            &mut wire.source_byte_length,
                            &mut wire.invalid,
                        )?,
                        "sourceLineCount" => read_wire_value(
                            &mut map,
                            &mut wire.source_line_count,
                            &mut wire.invalid,
                        )?,
                        "kind" => {
                            read_wire_value(&mut map, &mut wire.kind, &mut wire.invalid)?;
                        }
                        "byteStart" => {
                            read_wire_value(&mut map, &mut wire.byte_start, &mut wire.invalid)?;
                        }
                        "byteEnd" => {
                            read_wire_value(&mut map, &mut wire.byte_end, &mut wire.invalid)?;
                        }
                        "fileLineOffset" => read_wire_value(
                            &mut map,
                            &mut wire.file_line_offset,
                            &mut wire.invalid,
                        )?,
                        "lineCount" => {
                            read_wire_value(&mut map, &mut wire.line_count, &mut wire.invalid)?;
                        }
                        "terminator" => {
                            read_wire_value(&mut map, &mut wire.terminator, &mut wire.invalid)?;
                        }
                        "lineOffsetInParent" => read_wire_value(
                            &mut map,
                            &mut wire.line_offset_in_parent,
                            &mut wire.invalid,
                        )?,
                        "parentKind" => {
                            read_wire_value(&mut map, &mut wire.parent_kind, &mut wire.invalid)?;
                        }
                        "parentByteStart" => read_wire_value(
                            &mut map,
                            &mut wire.parent_byte_start,
                            &mut wire.invalid,
                        )?,
                        "parentByteEnd" => {
                            read_wire_value(&mut map, &mut wire.parent_byte_end, &mut wire.invalid)?
                        }
                        "parentFileLineOffset" => read_wire_value(
                            &mut map,
                            &mut wire.parent_file_line_offset,
                            &mut wire.invalid,
                        )?,
                        "parentLineCount" => read_wire_value(
                            &mut map,
                            &mut wire.parent_line_count,
                            &mut wire.invalid,
                        )?,
                        _ => {
                            wire.invalid = true;
                            map.next_value::<IgnoredAny>()?;
                        }
                    }
                }
                Ok(wire)
            }
        }
        deserializer.deserialize_map(WireVisitor)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    const ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

    fn issuer(length: usize, lines: usize) -> AnddressIssuer {
        AnddressIssuer::new(ZERO_HASH, "source.txt", ZERO_HASH, length, lines).unwrap()
    }

    #[test]
    fn exact_v5_kats_and_shared_source_identity() {
        let issuer = issuer(14, 3);
        let paragraph = ParagraphGeometry {
            byte_start: 0,
            byte_end: 12,
            file_line_offset: 0,
            line_count: 2,
        };
        let file = issuer.issue(TargetGeometry::File).unwrap();
        let paragraph_address = issuer.issue(TargetGeometry::Paragraph(paragraph)).unwrap();
        let text_line = issuer
            .issue(TargetGeometry::Line {
                byte_start: 6,
                byte_end: 12,
                terminator: LineTerminator::Crlf,
                line_offset_in_parent: 1,
                parent: ParentGeometry::Paragraph(paragraph),
            })
            .unwrap();
        let file_line = issuer
            .issue(TargetGeometry::Line {
                byte_start: 12,
                byte_end: 14,
                terminator: LineTerminator::Lf,
                line_offset_in_parent: 2,
                parent: ParentGeometry::File,
            })
            .unwrap();
        assert!(Arc::ptr_eq(
            file.source_identity(),
            text_line.source_identity()
        ));
        assert!(Arc::ptr_eq(
            text_line.source_identity(),
            paragraph_address.source_identity()
        ));
        assert_eq!(
            file.encode().unwrap(),
            br#"{"version":"artext.backwriter-anddress.v5","workspaceCoordinate":"0000000000000000000000000000000000000000000000000000000000000000","logicalPath":"source.txt","sourceStateHash":"0000000000000000000000000000000000000000000000000000000000000000","sourceByteLength":"14","sourceLineCount":"3","kind":"file"}"#
        );
        assert_eq!(
            paragraph_address.encode().unwrap(),
            br#"{"version":"artext.backwriter-anddress.v5","workspaceCoordinate":"0000000000000000000000000000000000000000000000000000000000000000","logicalPath":"source.txt","sourceStateHash":"0000000000000000000000000000000000000000000000000000000000000000","sourceByteLength":"14","sourceLineCount":"3","kind":"paragraph","byteStart":"0","byteEnd":"12","fileLineOffset":"0","lineCount":"2"}"#
        );
        assert_eq!(
            text_line.encode().unwrap(),
            br#"{"version":"artext.backwriter-anddress.v5","workspaceCoordinate":"0000000000000000000000000000000000000000000000000000000000000000","logicalPath":"source.txt","sourceStateHash":"0000000000000000000000000000000000000000000000000000000000000000","sourceByteLength":"14","sourceLineCount":"3","kind":"line","byteStart":"6","byteEnd":"12","terminator":"crlf","lineOffsetInParent":"1","parentKind":"paragraph","parentByteStart":"0","parentByteEnd":"12","parentFileLineOffset":"0","parentLineCount":"2"}"#
        );
        assert_eq!(
            file_line.encode().unwrap(),
            br#"{"version":"artext.backwriter-anddress.v5","workspaceCoordinate":"0000000000000000000000000000000000000000000000000000000000000000","logicalPath":"source.txt","sourceStateHash":"0000000000000000000000000000000000000000000000000000000000000000","sourceByteLength":"14","sourceLineCount":"3","kind":"line","byteStart":"12","byteEnd":"14","terminator":"lf","lineOffsetInParent":"2","parentKind":"file"}"#
        );
        let addresses = [file, paragraph_address, text_line, file_line];
        let mut output = b"replaced".to_vec();
        addresses[2].encode_into(&mut output).unwrap();
        let capacity = output.capacity();
        for address in &addresses {
            let expected = address.encode().unwrap();
            address.encode_into(&mut output).unwrap();
            assert_eq!(output, expected);
            assert_eq!(output.capacity(), capacity);
            assert_eq!(Anddress::decode(&output).unwrap(), *address);
        }
    }

    #[test]
    fn encoder_escapes_only_valid_path_content_and_writes_decimal_bounds() {
        let address =
            AnddressIssuer::new(ZERO_HASH, "문\"서.txt", ZERO_HASH, usize::MAX, usize::MAX)
                .unwrap()
                .issue(TargetGeometry::File)
                .unwrap();
        let mut output = Vec::new();
        address.encode_into(&mut output).unwrap();
        assert_eq!(Anddress::decode(&output), Ok(address));
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains(r#""logicalPath":"문\"서.txt""#));
        assert!(output.contains(&format!(
            r#""sourceByteLength":"{}","sourceLineCount":"{}""#,
            usize::MAX,
            usize::MAX
        )));

        for path in ["a\\b.txt", "a\nb.txt", "a\u{1f}b.txt"] {
            assert!(matches!(
                AnddressIssuer::new(ZERO_HASH, path, ZERO_HASH, 1, 1),
                Err(AnddressError::Invalid)
            ));
        }

        let mut length = EncodedLength(usize::MAX);
        assert_eq!(length.push(b"x"), Err(AnddressError::Resource));
    }

    #[test]
    fn typed_construction_and_encoding_have_single_validation_and_writer_paths() {
        let production = include_str!("anddress.rs")
            .split("#[cfg(test)]\nmod tests")
            .next()
            .unwrap();
        let issuer = production.split("impl AnddressIssuer").nth(1).unwrap();
        let constructor = issuer
            .split("pub(crate) fn new")
            .nth(1)
            .unwrap()
            .split("fn from_owned_source")
            .next()
            .unwrap();
        assert_eq!(constructor.matches("from_owned_source").count(), 1);
        assert!(!constructor.contains("validate_source"));
        let issue = issuer
            .split("pub(crate) fn issue")
            .nth(1)
            .unwrap()
            .split("impl Anddress")
            .next()
            .unwrap();
        assert!(issue.contains("validate_geometry"));
        assert!(!issue.contains("validate_source"));
        assert_eq!(production.matches("fn validate_source(").count(), 1);
        assert_eq!(production.matches("fn validate_geometry(").count(), 1);
        assert_eq!(production.matches("fn encode_to(").count(), 1);
        let encode = production
            .split("pub fn encode(&self)")
            .nth(1)
            .unwrap()
            .split("pub fn decode")
            .next()
            .unwrap();
        assert!(encode.contains("self.encode_into(&mut output)?"));
        assert!(!encode.contains("self.validate()"));
        let encode_into = encode.split("pub fn encode_into").nth(1).unwrap();
        assert!(
            encode_into.find("output.clear();")
                < encode_into.find("let mut length = EncodedLength(0);")
        );
        assert!(
            encode_into.find("encode_to(self, &mut length)?;")
                < encode_into.find(".try_reserve(length.0)")
        );
        assert!(
            encode_into.find(".try_reserve(length.0)")
                < encode_into.rfind("encode_to(self, output)?")
        );
        assert!(!production.contains("serde_json::to_string"));
        assert!(!production.contains("value.to_string()"));
    }

    #[test]
    fn algebra_projects_only_self_or_ancestors() {
        let issuer = issuer(14, 3);
        let paragraph = ParagraphGeometry {
            byte_start: 0,
            byte_end: 12,
            file_line_offset: 0,
            line_count: 2,
        };
        let line = issuer
            .issue(TargetGeometry::Line {
                byte_start: 6,
                byte_end: 12,
                terminator: LineTerminator::Crlf,
                line_offset_in_parent: 1,
                parent: ParentGeometry::Paragraph(paragraph),
            })
            .unwrap();
        let parent = line.parent().unwrap();
        let file = parent.parent().unwrap();
        assert_eq!(line.range(), 6..12);
        assert_eq!(line.line_count(), 1);
        assert_eq!(line.line_range(), 1..2);
        assert_eq!(line.line_number(), Some(2));
        assert_eq!(line.terminator(), Some(LineTerminator::Crlf));
        assert!(file.same_source(&line));
        assert!(file.same_state(&line));
        assert_eq!(
            line.project(AnddressTarget::Paragraph),
            Ok(Some(parent.clone()))
        );
        assert_eq!(line.project(AnddressTarget::File), Ok(Some(file.clone())));
        assert!(file.contains(&line));
        assert!(parent.contains(&line));
        assert!(parent.overlaps(&line));
        assert_eq!(
            file.project(AnddressTarget::Line),
            Err(AnddressError::Invalid)
        );

        let separator = issuer
            .issue(TargetGeometry::Line {
                byte_start: 12,
                byte_end: 14,
                terminator: LineTerminator::Lf,
                line_offset_in_parent: 2,
                parent: ParentGeometry::File,
            })
            .unwrap();
        assert_eq!(separator.parent(), Some(file));
        assert_eq!(separator.project(AnddressTarget::Paragraph), Ok(None));
    }

    #[test]
    fn equality_clone_and_source_state_relations_include_complete_geometry() {
        let issuer = issuer(4, 1);
        let line = issuer
            .issue(TargetGeometry::Line {
                byte_start: 0,
                byte_end: 4,
                terminator: LineTerminator::Lf,
                line_offset_in_parent: 0,
                parent: ParentGeometry::File,
            })
            .unwrap();
        assert_eq!(line, line.clone());
        let other_state = AnddressIssuer::new(ZERO_HASH, "source.txt", &"1".repeat(64), 4, 1)
            .unwrap()
            .issue(line.geometry())
            .unwrap();
        assert!(line.same_source(&other_state));
        assert!(!line.same_state(&other_state));
        assert!(!line.contains(&other_state));
        assert!(!line.overlaps(&other_state));
        let other_source = AnddressIssuer::new(ZERO_HASH, "other.txt", ZERO_HASH, 4, 1)
            .unwrap()
            .issue(line.geometry())
            .unwrap();
        assert!(!line.same_source(&other_source));
        assert!(!line.same_state(&other_source));
        assert!(!line.contains(&other_source));
        assert!(!line.overlaps(&other_source));
    }

    #[test]
    fn paragraph_attachment_is_exact_and_never_partially_mutates_one_geometry() {
        let paragraph = ParagraphGeometry {
            byte_start: 2,
            byte_end: 8,
            file_line_offset: 1,
            line_count: 3,
        };
        let mut line = TargetGeometry::Line {
            byte_start: 4,
            byte_end: 6,
            terminator: LineTerminator::Lf,
            line_offset_in_parent: 2,
            parent: ParentGeometry::File,
        };
        assert_eq!(attach_line_to_paragraph(&mut line, paragraph), Ok(true));
        assert!(matches!(
            line,
            TargetGeometry::Line {
                line_offset_in_parent: 1,
                parent: ParentGeometry::Paragraph(parent),
                ..
            } if parent == paragraph
        ));

        let cases = [
            (
                TargetGeometry::Line {
                    byte_start: 8,
                    byte_end: 9,
                    terminator: LineTerminator::None,
                    line_offset_in_parent: 4,
                    parent: ParentGeometry::File,
                },
                paragraph,
                Ok(false),
            ),
            (TargetGeometry::File, paragraph, Ok(false)),
            (
                TargetGeometry::Line {
                    byte_start: 2,
                    byte_end: 4,
                    terminator: LineTerminator::Lf,
                    line_offset_in_parent: 0,
                    parent: ParentGeometry::File,
                },
                paragraph,
                Err(AnddressError::Invalid),
            ),
            (
                TargetGeometry::Line {
                    byte_start: 0,
                    byte_end: 1,
                    terminator: LineTerminator::None,
                    line_offset_in_parent: usize::MAX,
                    parent: ParentGeometry::File,
                },
                ParagraphGeometry {
                    byte_start: 0,
                    byte_end: 1,
                    file_line_offset: usize::MAX,
                    line_count: 1,
                },
                Err(AnddressError::Invalid),
            ),
        ];
        for (mut geometry, parent, expected) in cases {
            let before = geometry;
            assert_eq!(attach_line_to_paragraph(&mut geometry, parent), expected);
            assert_eq!(geometry, before);
        }
    }

    #[test]
    fn geometry_mutation_and_overflow_fail_closed() {
        let issuer = issuer(4, 2);
        for geometry in [
            TargetGeometry::Paragraph(ParagraphGeometry {
                byte_start: 0,
                byte_end: 4,
                file_line_offset: usize::MAX,
                line_count: 1,
            }),
            TargetGeometry::Paragraph(ParagraphGeometry {
                byte_start: 1,
                byte_end: 1,
                file_line_offset: 0,
                line_count: 1,
            }),
            TargetGeometry::Line {
                byte_start: 0,
                byte_end: 1,
                terminator: LineTerminator::Crlf,
                line_offset_in_parent: 0,
                parent: ParentGeometry::File,
            },
            TargetGeometry::Line {
                byte_start: 0,
                byte_end: 4,
                terminator: LineTerminator::None,
                line_offset_in_parent: 2,
                parent: ParentGeometry::File,
            },
        ] {
            assert_eq!(issuer.issue(geometry), Err(AnddressError::Invalid));
        }
        assert!(
            AnddressIssuer::new(ZERO_HASH, "empty.txt", ZERO_HASH, 0, 0)
                .unwrap()
                .issue(TargetGeometry::File)
                .is_ok()
        );
        assert_eq!(
            Anddress::decode(
                format!(
                    r#"{{"version":"{ANDDRESS_VERSION}","workspaceCoordinate":"{ZERO_HASH}","logicalPath":"source.txt","sourceStateHash":"{ZERO_HASH}","sourceByteLength":"4","sourceLineCount":"2","kind":"line","byteStart":"0","byteEnd":"2","terminator":"lf","lineOffsetInParent":"2","parentKind":"paragraph","parentByteStart":"0","parentByteEnd":"4","parentFileLineOffset":"0","parentLineCount":"2"}}"#
                )
                .as_bytes()
            ),
            Err(AnddressError::Invalid)
        );
        assert!(matches!(
            AnddressIssuer::new(ZERO_HASH, "source.txt", ZERO_HASH, 1, 2),
            Err(AnddressError::Invalid)
        ));
    }

    #[test]
    fn wire_rejects_shape_errors_and_noncanonical_decimals() {
        let base = |length: &str| {
            format!(
                r#"{{"version":"{ANDDRESS_VERSION}","workspaceCoordinate":"{ZERO_HASH}","logicalPath":"source.txt","sourceStateHash":"{ZERO_HASH}","sourceByteLength":"{length}","sourceLineCount":"1","kind":"file"}}"#
            )
        };
        let overflow = format!("{}0", usize::MAX);
        for encoded in [base("01"), base("+1"), base(""), base(&overflow)] {
            assert_eq!(
                Anddress::decode(encoded.as_bytes()),
                Err(AnddressError::Encoding)
            );
        }
        let duplicate =
            format!(r#"{{"version":"{ANDDRESS_VERSION}","version":"{ANDDRESS_VERSION}"}}"#);
        assert_eq!(
            Anddress::decode(duplicate.as_bytes()),
            Err(AnddressError::Encoding)
        );
        let unknown = format!(
            r#"{{"version":"{ANDDRESS_VERSION}","workspaceCoordinate":"{ZERO_HASH}","logicalPath":"source.txt","sourceStateHash":"{ZERO_HASH}","sourceByteLength":"1","sourceLineCount":"1","kind":"file","unknown":"0"}}"#
        );
        assert_eq!(
            Anddress::decode(unknown.as_bytes()),
            Err(AnddressError::Encoding)
        );
    }
}
