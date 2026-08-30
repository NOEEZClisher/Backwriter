//! Compact exact-source-state v4 structural addresses.

use std::{fmt, sync::Arc};

use serde::de::{IgnoredAny, MapAccess, Visitor};
use thiserror::Error;

use crate::hash::is_lower_hex_sha256;
use crate::source::validate_logical_path;

pub const ANDDRESS_VERSION: &str = "artext.backwriter-anddress.v4";

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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Anddress {
    source: Arc<SourceIdentity>,
    target: AnddressTarget,
    byte_start: usize,
    byte_end: usize,
}

impl Anddress {
    pub fn new(
        workspace_coordinate: &str,
        logical_path: &str,
        source_state_hash: &str,
        source_byte_length: usize,
        target: AnddressTarget,
        byte_start: usize,
        byte_end: usize,
    ) -> Result<Self, AnddressError> {
        let source = construct_source_identity(
            workspace_coordinate,
            logical_path,
            source_state_hash,
            source_byte_length,
        )?;
        construct_anddress(&source, target, byte_start, byte_end)
    }

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

    pub fn target(&self) -> AnddressTarget {
        self.target
    }

    pub fn byte_start(&self) -> usize {
        self.byte_start
    }

    pub fn byte_end(&self) -> usize {
        self.byte_end
    }

    pub fn encode(&self) -> Result<Vec<u8>, AnddressError> {
        self.validate()?;
        let mut output = String::new();
        output
            .try_reserve(
                self.workspace_coordinate()
                    .len()
                    .checked_add(self.logical_path().len())
                    .and_then(|length| length.checked_add(self.source_state_hash().len()))
                    .and_then(|length| length.checked_add(192))
                    .ok_or(AnddressError::Resource)?,
            )
            .map_err(|_| AnddressError::Resource)?;
        output.push('{');
        append_field(&mut output, "version", ANDDRESS_VERSION)?;
        output.push(',');
        append_field(
            &mut output,
            "workspaceCoordinate",
            self.workspace_coordinate(),
        )?;
        output.push(',');
        append_field(&mut output, "logicalPath", self.logical_path())?;
        output.push(',');
        append_field(&mut output, "sourceStateHash", self.source_state_hash())?;
        output.push(',');
        append_decimal_field(&mut output, "sourceByteLength", self.source_byte_length())?;
        output.push(',');
        append_field(&mut output, "kind", self.target.as_str())?;
        output.push(',');
        append_decimal_field(&mut output, "byteStart", self.byte_start)?;
        output.push(',');
        append_decimal_field(&mut output, "byteEnd", self.byte_end)?;
        output.push('}');
        Ok(output.into_bytes())
    }

    pub fn decode(encoded: &[u8]) -> Result<Self, AnddressError> {
        let wire: Wire = serde_json::from_slice(encoded).map_err(|_| AnddressError::Encoding)?;
        wire.into_anddress()
    }

    pub fn validate(&self) -> Result<(), AnddressError> {
        validate_source(
            self.workspace_coordinate(),
            self.logical_path(),
            self.source_state_hash(),
        )?;
        validate_range(
            self.source_byte_length(),
            self.target,
            self.byte_start,
            self.byte_end,
        )
    }

    pub(crate) fn source_identity(&self) -> &Arc<SourceIdentity> {
        &self.source
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

pub(crate) fn construct_source_identity(
    workspace_coordinate: &str,
    logical_path: &str,
    source_state_hash: &str,
    source_byte_length: usize,
) -> Result<Arc<SourceIdentity>, AnddressError> {
    validate_source(workspace_coordinate, logical_path, source_state_hash)?;
    Ok(Arc::new(SourceIdentity {
        workspace_coordinate: fallible_copy(workspace_coordinate)?,
        logical_path: fallible_copy(logical_path)?,
        source_state_hash: fallible_copy(source_state_hash)?,
        source_byte_length,
    }))
}

pub(crate) fn construct_anddress(
    source: &Arc<SourceIdentity>,
    target: AnddressTarget,
    byte_start: usize,
    byte_end: usize,
) -> Result<Anddress, AnddressError> {
    validate_range(source.source_byte_length, target, byte_start, byte_end)?;
    Ok(Anddress {
        source: Arc::clone(source),
        target,
        byte_start,
        byte_end,
    })
}

fn validate_source(
    workspace_coordinate: &str,
    logical_path: &str,
    source_state_hash: &str,
) -> Result<(), AnddressError> {
    if !is_lower_hex_sha256(workspace_coordinate)
        || validate_logical_path(logical_path).is_err()
        || !is_lower_hex_sha256(source_state_hash)
    {
        return Err(AnddressError::Invalid);
    }
    Ok(())
}

fn validate_range(
    source_byte_length: usize,
    target: AnddressTarget,
    byte_start: usize,
    byte_end: usize,
) -> Result<(), AnddressError> {
    if byte_start > byte_end || byte_end > source_byte_length {
        return Err(AnddressError::Invalid);
    }
    if target == AnddressTarget::File && (byte_start != 0 || byte_end != source_byte_length) {
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

fn append_field(output: &mut String, name: &str, value: &str) -> Result<(), AnddressError> {
    let name = serde_json::to_string(name).map_err(|_| AnddressError::Encoding)?;
    let value = serde_json::to_string(value).map_err(|_| AnddressError::Encoding)?;
    output
        .try_reserve(
            name.len()
                .checked_add(value.len())
                .and_then(|size| size.checked_add(1))
                .ok_or(AnddressError::Resource)?,
        )
        .map_err(|_| AnddressError::Resource)?;
    output.push_str(&name);
    output.push(':');
    output.push_str(&value);
    Ok(())
}

fn append_decimal_field(
    output: &mut String,
    name: &str,
    value: usize,
) -> Result<(), AnddressError> {
    append_field(output, name, &value.to_string())
}

#[derive(Default)]
struct Wire {
    version: Option<WireString>,
    workspace_coordinate: Option<WireString>,
    logical_path: Option<WireString>,
    source_state_hash: Option<WireString>,
    source_byte_length: Option<WireString>,
    kind: Option<WireString>,
    byte_start: Option<WireString>,
    byte_end: Option<WireString>,
    invalid: bool,
    duplicate_version: bool,
}

impl Wire {
    fn into_anddress(self) -> Result<Anddress, AnddressError> {
        if self.duplicate_version {
            return Err(AnddressError::Encoding);
        }
        let version = wire_string(self.version)?;
        if version != ANDDRESS_VERSION {
            return Err(AnddressError::UnsupportedVersion);
        }
        if self.invalid {
            return Err(AnddressError::Encoding);
        }
        let workspace_coordinate = wire_string(self.workspace_coordinate)?;
        let logical_path = wire_string(self.logical_path)?;
        let source_state_hash = wire_string(self.source_state_hash)?;
        let source_byte_length = wire_usize(self.source_byte_length)?;
        let target = match wire_string(self.kind)?.as_str() {
            "file" => AnddressTarget::File,
            "paragraph" => AnddressTarget::Paragraph,
            "line" => AnddressTarget::Line,
            _ => return Err(AnddressError::Encoding),
        };
        let byte_start = wire_usize(self.byte_start)?;
        let byte_end = wire_usize(self.byte_end)?;
        Anddress::new(
            &workspace_coordinate,
            &logical_path,
            &source_state_hash,
            source_byte_length,
            target,
            byte_start,
            byte_end,
        )
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
                formatter.write_str("a v4 Anddress object")
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
                        "kind" => {
                            read_wire_value(&mut map, &mut wire.kind, &mut wire.invalid)?;
                        }
                        "byteStart" => {
                            read_wire_value(&mut map, &mut wire.byte_start, &mut wire.invalid)?;
                        }
                        "byteEnd" => {
                            read_wire_value(&mut map, &mut wire.byte_end, &mut wire.invalid)?;
                        }
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

    #[test]
    fn exact_v4_kat_and_shared_source_identity() {
        let source = construct_source_identity(ZERO_HASH, "source.txt", ZERO_HASH, 4).unwrap();
        let file = construct_anddress(&source, AnddressTarget::File, 0, 4).unwrap();
        let line = construct_anddress(&source, AnddressTarget::Line, 0, 4).unwrap();
        let paragraph = construct_anddress(&source, AnddressTarget::Paragraph, 0, 4).unwrap();
        assert!(Arc::ptr_eq(file.source_identity(), line.source_identity()));
        assert!(Arc::ptr_eq(
            line.source_identity(),
            paragraph.source_identity()
        ));
        assert_eq!(
            file.encode().unwrap(),
            br#"{"version":"artext.backwriter-anddress.v4","workspaceCoordinate":"0000000000000000000000000000000000000000000000000000000000000000","logicalPath":"source.txt","sourceStateHash":"0000000000000000000000000000000000000000000000000000000000000000","sourceByteLength":"4","kind":"file","byteStart":"0","byteEnd":"4"}"#
        );
        assert_eq!(
            paragraph.encode().unwrap(),
            br#"{"version":"artext.backwriter-anddress.v4","workspaceCoordinate":"0000000000000000000000000000000000000000000000000000000000000000","logicalPath":"source.txt","sourceStateHash":"0000000000000000000000000000000000000000000000000000000000000000","sourceByteLength":"4","kind":"paragraph","byteStart":"0","byteEnd":"4"}"#
        );
        assert_eq!(
            line.encode().unwrap(),
            br#"{"version":"artext.backwriter-anddress.v4","workspaceCoordinate":"0000000000000000000000000000000000000000000000000000000000000000","logicalPath":"source.txt","sourceStateHash":"0000000000000000000000000000000000000000000000000000000000000000","sourceByteLength":"4","kind":"line","byteStart":"0","byteEnd":"4"}"#
        );
        assert_eq!(Anddress::decode(&line.encode().unwrap()).unwrap(), line);
    }

    #[test]
    fn equality_includes_source_hash_length_kind_and_range() {
        let base = Anddress::new(
            ZERO_HASH,
            "source.txt",
            ZERO_HASH,
            4,
            AnddressTarget::Line,
            0,
            4,
        )
        .unwrap();
        for different in [
            Anddress::new(
                ZERO_HASH,
                "source.txt",
                &"1".repeat(64),
                4,
                AnddressTarget::Line,
                0,
                4,
            )
            .unwrap(),
            Anddress::new(
                ZERO_HASH,
                "source.txt",
                ZERO_HASH,
                5,
                AnddressTarget::Line,
                0,
                4,
            )
            .unwrap(),
            Anddress::new(
                ZERO_HASH,
                "source.txt",
                ZERO_HASH,
                4,
                AnddressTarget::Paragraph,
                0,
                4,
            )
            .unwrap(),
            Anddress::new(
                ZERO_HASH,
                "source.txt",
                ZERO_HASH,
                4,
                AnddressTarget::Line,
                1,
                4,
            )
            .unwrap(),
        ] {
            assert_ne!(base, different);
        }
    }

    #[test]
    fn ranges_and_file_geometry_are_checked() {
        assert!(
            Anddress::new(
                ZERO_HASH,
                "empty.txt",
                ZERO_HASH,
                0,
                AnddressTarget::File,
                0,
                0,
            )
            .is_ok()
        );
        for (kind, start, end, length) in [
            (AnddressTarget::Line, 2, 1, 3),
            (AnddressTarget::Paragraph, 0, 4, 3),
            (AnddressTarget::File, 1, 3, 3),
            (AnddressTarget::File, 0, 2, 3),
        ] {
            assert_eq!(
                Anddress::new(ZERO_HASH, "source.txt", ZERO_HASH, length, kind, start, end,),
                Err(AnddressError::Invalid)
            );
        }
        let maximum = usize::MAX;
        let address = Anddress::new(
            ZERO_HASH,
            "source.txt",
            ZERO_HASH,
            maximum,
            AnddressTarget::Line,
            maximum,
            maximum,
        )
        .unwrap();
        assert_eq!(address.byte_start(), maximum);
        assert_eq!(
            Anddress::decode(&address.encode().unwrap()).unwrap(),
            address
        );
    }

    #[test]
    fn wire_rejects_noncanonical_or_unrepresentable_decimals() {
        let base = |length: &str, start: &str, end: &str| {
            format!(
                r#"{{"version":"{ANDDRESS_VERSION}","workspaceCoordinate":"{ZERO_HASH}","logicalPath":"source.txt","sourceStateHash":"{ZERO_HASH}","sourceByteLength":"{length}","kind":"line","byteStart":"{start}","byteEnd":"{end}"}}"#
            )
        };
        for encoded in [
            base("01", "0", "1"),
            base("1", "+0", "1"),
            base("1", "", "1"),
            base("184467440737095516160", "0", "0"),
        ] {
            assert_eq!(
                Anddress::decode(encoded.as_bytes()),
                Err(AnddressError::Encoding)
            );
        }
    }

    #[test]
    fn readable_v3_has_unsupported_version_priority() {
        let v3 = br#"{"version":"artext.backwriter-anddress.v3","workspaceCoordinate":0,"unknown":true}"#;
        assert_eq!(Anddress::decode(v3), Err(AnddressError::UnsupportedVersion));
        let duplicate = br#"{"version":"artext.backwriter-anddress.v3","version":"artext.backwriter-anddress.v3"}"#;
        assert_eq!(Anddress::decode(duplicate), Err(AnddressError::Encoding));
    }
}
