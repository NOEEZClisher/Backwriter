//! Compact, source-less v3 structural addresses.

use std::{cmp::Ordering, fmt};

use serde::de::{IgnoredAny, MapAccess, Visitor};
use thiserror::Error;

use crate::hash::is_lower_hex_sha256;
use crate::source::validate_logical_path;

pub const ANDDRESS_VERSION: &str = "artext.backwriter-anddress.v3";

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

/// An opaque, canonical, arbitrary-size unsigned natural number.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Natural(String);

impl Natural {
    pub fn parse(value: &str) -> Result<Self, AnddressError> {
        let mut owned = String::new();
        owned
            .try_reserve(value.len())
            .map_err(|_| AnddressError::Resource)?;
        owned.push_str(value);
        Self::from_wire(owned)
    }

    pub fn zero() -> Self {
        Self("0".to_owned())
    }
    pub fn one() -> Self {
        Self("1".to_owned())
    }
    pub fn is_zero(&self) -> bool {
        self.0 == "0"
    }

    fn from_wire(value: String) -> Result<Self, AnddressError> {
        if value.is_empty()
            || !value.bytes().all(|byte| byte.is_ascii_digit())
            || (value.len() > 1 && value.starts_with('0'))
        {
            return Err(AnddressError::Invalid);
        }
        Ok(Self(value))
    }

    pub(crate) fn cmp_canonical_decimal_bytes(&self, other: &[u8]) -> Ordering {
        self.0
            .len()
            .cmp(&other.len())
            .then_with(|| self.0.as_bytes().cmp(other))
    }
}

impl Ord for Natural {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_canonical_decimal_bytes(other.0.as_bytes())
    }
}
impl PartialOrd for Natural {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl fmt::Display for Natural {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Anddress {
    pub version: String,
    pub workspace_coordinate: String,
    pub logical_path: String,
    pub target: AnddressTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnddressTarget {
    File,
    Paragraph {
        ordinal: Natural,
    },
    Line {
        ordinal: Natural,
        exact_extent: String,
    },
}

impl Anddress {
    pub fn encode(&self) -> Result<Vec<u8>, AnddressError> {
        self.validate()?;
        let mut output = String::new();
        output
            .try_reserve(
                self.workspace_coordinate
                    .len()
                    .checked_add(self.logical_path.len())
                    .ok_or(AnddressError::Resource)?,
            )
            .map_err(|_| AnddressError::Resource)?;
        output.push('{');
        append_field(&mut output, "version", &self.version)?;
        output.push(',');
        append_field(
            &mut output,
            "workspaceCoordinate",
            &self.workspace_coordinate,
        )?;
        output.push(',');
        append_field(&mut output, "logicalPath", &self.logical_path)?;
        output.push(',');
        append_field(&mut output, "kind", self.target.kind())?;
        match &self.target {
            AnddressTarget::File => {}
            AnddressTarget::Paragraph { ordinal } => {
                output.push(',');
                append_field(&mut output, "ordinal", &ordinal.to_string())?;
            }
            AnddressTarget::Line {
                ordinal,
                exact_extent,
            } => {
                output.push(',');
                append_field(&mut output, "ordinal", &ordinal.to_string())?;
                output.push(',');
                append_field(&mut output, "exactExtent", exact_extent)?;
            }
        }
        output.push('}');
        Ok(output.into_bytes())
    }

    pub fn decode(encoded: &[u8]) -> Result<Self, AnddressError> {
        let wire: Wire = serde_json::from_slice(encoded).map_err(|_| AnddressError::Encoding)?;
        wire.into_anddress()
    }

    pub fn validate(&self) -> Result<(), AnddressError> {
        if self.version != ANDDRESS_VERSION {
            return Err(AnddressError::UnsupportedVersion);
        }
        if !is_lower_hex_sha256(&self.workspace_coordinate)
            || validate_logical_path(&self.logical_path).is_err()
        {
            return Err(AnddressError::Invalid);
        }
        match &self.target {
            AnddressTarget::File | AnddressTarget::Paragraph { .. } => Ok(()),
            AnddressTarget::Line { exact_extent, .. } => validate_exact_extent(exact_extent),
        }
    }
}

impl AnddressTarget {
    fn kind(&self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Paragraph { .. } => "paragraph",
            Self::Line { .. } => "line",
        }
    }
}

pub(crate) fn construct_anddress(
    workspace_coordinate: &str,
    logical_path: &str,
    target: AnddressTarget,
) -> Result<Anddress, AnddressError> {
    let address = Anddress {
        version: ANDDRESS_VERSION.to_owned(),
        workspace_coordinate: fallible_copy(workspace_coordinate)?,
        logical_path: fallible_copy(logical_path)?,
        target,
    };
    address.validate()?;
    Ok(address)
}

/// Copies one already-owned address while preserving allocation failure.
pub(crate) fn fallible_copy_anddress(value: &Anddress) -> Result<Anddress, AnddressError> {
    let target = match &value.target {
        AnddressTarget::File => AnddressTarget::File,
        AnddressTarget::Paragraph { ordinal } => AnddressTarget::Paragraph {
            ordinal: Natural(fallible_copy(&ordinal.0)?),
        },
        AnddressTarget::Line {
            ordinal,
            exact_extent,
        } => AnddressTarget::Line {
            ordinal: Natural(fallible_copy(&ordinal.0)?),
            exact_extent: fallible_copy(exact_extent)?,
        },
    };
    Ok(Anddress {
        version: fallible_copy(&value.version)?,
        workspace_coordinate: fallible_copy(&value.workspace_coordinate)?,
        logical_path: fallible_copy(&value.logical_path)?,
        target,
    })
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

fn validate_exact_extent(exact_extent: &str) -> Result<(), AnddressError> {
    if exact_extent.contains('\0') {
        return Err(AnddressError::Invalid);
    }
    let bytes = exact_extent.as_bytes();
    let content_end = if bytes.ends_with(b"\r\n") {
        bytes.len() - 2
    } else if bytes.ends_with(b"\r") || bytes.ends_with(b"\n") {
        bytes.len() - 1
    } else {
        bytes.len()
    };
    if content_end == 0 && bytes.is_empty() {
        return Err(AnddressError::Invalid);
    }
    if bytes[..content_end]
        .iter()
        .any(|byte| matches!(byte, b'\r' | b'\n'))
    {
        return Err(AnddressError::Invalid);
    }
    Ok(())
}

#[derive(Default)]
struct Wire {
    version: Option<WireString>,
    workspace_coordinate: Option<WireString>,
    logical_path: Option<WireString>,
    kind: Option<WireString>,
    ordinal: Option<WireString>,
    exact_extent: Option<WireString>,
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
        let kind = wire_string(self.kind)?;
        let target = match kind.as_str() {
            "file" if self.ordinal.is_none() && self.exact_extent.is_none() => AnddressTarget::File,
            "paragraph" if self.exact_extent.is_none() => AnddressTarget::Paragraph {
                ordinal: wire_natural(self.ordinal)?,
            },
            "line" => AnddressTarget::Line {
                ordinal: wire_natural(self.ordinal)?,
                exact_extent: wire_string(self.exact_extent)?,
            },
            _ => return Err(AnddressError::Encoding),
        };
        let address = Anddress {
            version,
            workspace_coordinate,
            logical_path,
            target,
        };
        address.validate()?;
        Ok(address)
    }
}

fn wire_string(value: Option<WireString>) -> Result<String, AnddressError> {
    match value {
        Some(WireString::String(value)) => Ok(value),
        _ => Err(AnddressError::Encoding),
    }
}
fn wire_natural(value: Option<WireString>) -> Result<Natural, AnddressError> {
    Natural::from_wire(wire_string(value)?).map_err(|_| AnddressError::Encoding)
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
                formatter.write_str("a v3 Anddress object")
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
                        "kind" => {
                            read_wire_value(&mut map, &mut wire.kind, &mut wire.invalid)?;
                        }
                        "ordinal" => {
                            read_wire_value(&mut map, &mut wire.ordinal, &mut wire.invalid)?;
                        }
                        "exactExtent" => {
                            read_wire_value(&mut map, &mut wire.exact_extent, &mut wire.invalid)?;
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
    use super::*;
    #[test]
    fn natural_is_canonical_and_arbitrary() {
        let long = Natural::parse(&format!("1{}", "0".repeat(1000))).unwrap();
        assert!(Natural::parse("18446744073709551616").unwrap() < long);
        assert!(Natural::parse("01").is_err());
    }
    #[test]
    fn canonical_decimal_byte_comparison_matches_natural_order() {
        for (left, right, expected) in [
            ("0", "0", Ordering::Equal),
            ("9", "10", Ordering::Less),
            ("10", "9", Ordering::Greater),
            ("99", "100", Ordering::Less),
            ("100", "99", Ordering::Greater),
            ("123", "122", Ordering::Greater),
            ("123", "123", Ordering::Equal),
            ("123", "124", Ordering::Less),
        ] {
            let value = Natural::parse(left).unwrap();
            assert_eq!(
                value.cmp_canonical_decimal_bytes(right.as_bytes()),
                expected
            );
        }
        let long = format!("1{}", "0".repeat(4097));
        assert_eq!(
            Natural::parse(&long)
                .unwrap()
                .cmp_canonical_decimal_bytes(long.as_bytes()),
            Ordering::Equal
        );
    }
    #[test]
    fn exact_extent_is_one_physical_line() {
        for value in ["x", "x\n", "x\r", "x\r\n", "\n"] {
            assert!(validate_exact_extent(value).is_ok());
        }
        for value in ["", "x\ny", "x\0"] {
            assert!(validate_exact_extent(value).is_err());
        }
    }
}
