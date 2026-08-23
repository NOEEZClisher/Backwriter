use std::fmt;

use sha2::Digest as _;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct Digest([u8; 32]);

impl Digest {
    pub(crate) fn to_hex(self) -> String {
        let mut output = String::with_capacity(64);
        for byte in self.0 {
            use fmt::Write as _;
            write!(output, "{byte:02x}").expect("writing to String cannot fail");
        }
        output
    }
}

impl fmt::Debug for Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_hex())
    }
}

pub(crate) struct Sha256(sha2::Sha256);

impl Sha256 {
    pub(crate) fn new() -> Self {
        Self(sha2::Sha256::new())
    }

    pub(crate) fn update(&mut self, input: &[u8]) {
        self.0.update(input);
    }

    pub(crate) fn finish(self) -> Digest {
        Digest(self.0.finalize().into())
    }
}

pub(crate) fn is_lower_hex_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

const ESCAPE_SCRATCH_BYTES: usize = 256;

pub(crate) struct Transcript(Sha256);

pub(crate) struct OpenTranscriptField {
    hash: Sha256,
    scratch: [u8; ESCAPE_SCRATCH_BYTES],
    used: usize,
}

impl Transcript {
    pub(crate) fn new(domain: &str) -> Self {
        OpenTranscriptField {
            hash: Sha256::new(),
            scratch: [0; ESCAPE_SCRATCH_BYTES],
            used: 0,
        }
        .write(domain.as_bytes())
        .finish_field()
    }

    pub(crate) fn open_field(self) -> OpenTranscriptField {
        OpenTranscriptField {
            hash: self.0,
            scratch: [0; ESCAPE_SCRATCH_BYTES],
            used: 0,
        }
    }

    pub(crate) fn finish(self) -> Digest {
        self.0.finish()
    }
}

impl OpenTranscriptField {
    pub(crate) fn write_chunk(&mut self, value: &[u8]) {
        for &byte in value {
            if byte == 0 {
                if self.scratch.len() - self.used < 2 {
                    self.hash.update(&self.scratch[..self.used]);
                    self.used = 0;
                }
                self.scratch[self.used] = 0;
                self.scratch[self.used + 1] = 0xff;
                self.used += 2;
            } else {
                if self.used == self.scratch.len() {
                    self.hash.update(&self.scratch);
                    self.used = 0;
                }
                self.scratch[self.used] = byte;
                self.used += 1;
            }
        }
    }

    pub(crate) fn write(mut self, value: &[u8]) -> Self {
        self.write_chunk(value);
        self
    }

    pub(crate) fn finish_field(mut self) -> Transcript {
        if self.used != 0 {
            self.hash.update(&self.scratch[..self.used]);
        }
        self.hash.update(&[0, 0]);
        Transcript(self.hash)
    }
}

pub(crate) fn transcript_hex(
    domain: &str,
    fields: impl IntoIterator<Item = impl AsRef<[u8]>>,
) -> String {
    let mut transcript = Transcript::new(domain);
    for field in fields {
        transcript = transcript.open_field().write(field.as_ref()).finish_field();
    }
    transcript.finish().to_hex()
}

#[cfg(test)]
mod tests {
    use super::{Sha256, Transcript, is_lower_hex_sha256, transcript_hex};

    #[test]
    fn sha256_matches_fips_vectors() {
        assert_eq!(
            sha256(b"").to_hex(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256(b"abc").to_hex(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        let mut incremental = Sha256::new();
        incremental.update(b"a");
        incremental.update(b"bc");
        assert_eq!(incremental.finish(), sha256(b"abc"));
    }

    #[test]
    fn escaped_transcript_is_domain_separated_and_chunk_invariant() {
        let one = transcript_hex("domain", [b"a\0b".as_slice(), b"".as_slice()]);
        assert_eq!(
            one,
            "6169816b2ffc6d8445f14007e96a62d7b38346b7c68a260f08b1e6967c530128"
        );
        let other = transcript_hex("other", [b"a\0b".as_slice(), b"".as_slice()]);
        assert_ne!(one, other);
        let split = Transcript::new("domain")
            .open_field()
            .write(b"a")
            .write(b"\0")
            .write(b"b")
            .finish_field()
            .open_field()
            .write(b"")
            .finish_field();
        assert_eq!(split.finish().to_hex(), one);
        for split_at in 0..=3 {
            let irregular = Transcript::new("domain")
                .open_field()
                .write(&b"a\0b"[..split_at])
                .write(&b"a\0b"[split_at..])
                .finish_field()
                .open_field()
                .write(b"")
                .finish_field();
            assert_eq!(irregular.finish().to_hex(), one);
        }

        let edge_bytes = b"\0a\0\0b\0";
        let edge_one = transcript_hex("domain", [edge_bytes.as_slice(), b"".as_slice()]);
        for split_at in 0..=edge_bytes.len() {
            let split = Transcript::new("domain")
                .open_field()
                .write(&edge_bytes[..split_at])
                .write(&edge_bytes[split_at..])
                .finish_field()
                .open_field()
                .write(b"")
                .finish_field();
            assert_eq!(split.finish().to_hex(), edge_one);
        }
        assert!(is_lower_hex_sha256(&one));
        assert!(!is_lower_hex_sha256(&"A".repeat(64)));
        assert!(!is_lower_hex_sha256(&"a".repeat(63)));
    }

    fn sha256(input: &[u8]) -> super::Digest {
        let mut hasher = Sha256::new();
        hasher.update(input);
        hasher.finish()
    }
}
