//! Bech32 (BIP-173) checksummed binary data encoding

#![crate_name = "iq_bech32"]
#![crate_type = "rlib"]
#![allow(unknown_lints, suspicious_arithmetic_impl)]
#![deny(
    warnings,
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
#![doc(html_root_url = "https://docs.rs/iq-bech32/0.1.0")]

extern crate clear_on_drop;
#[macro_use]
extern crate failure;

use clear_on_drop::clear::Clear;

mod base32;
mod checksum;
mod error;

use checksum::{Checksum, CHECKSUM_SIZE};
pub use error::Error;

/// Default separator character
pub const DEFAULT_SEPARATOR: char = '1';

/// Bech32 default alphabet (lower case)
pub const DEFAULT_CHARSET: [char; 32] = [
    'q', 'p', 'z', 'r', 'y', '9', 'x', '8', 'g', 'f', '2', 't', 'v', 'd', 'w', '0', 's', '3', 'j',
    'n', '5', '4', 'k', 'h', 'c', 'e', '6', 'm', 'u', 'a', '7', 'l',
];

/// Bech32 default alphabet (upper case)
pub const DEFAULT_CHARSET_UPCASE: [char; 32] = [
    'Q', 'P', 'Z', 'R', 'Y', '9', 'X', '8', 'G', 'F', '2', 'T', 'V', 'D', 'W', '0', 'S', '3', 'J',
    'N', '5', '4', 'K', 'H', 'C', 'E', '6', 'M', 'U', 'A', '7', 'L',
];

/// Maximum length of a bech32 string
pub const MAX_LENGTH: usize = 90;

/// Bech32 encoder/decoder
pub struct Bech32 {
    /// Encoding character set
    pub charset: [char; 32],

    /// Inverse alphabet used to decode
    pub charset_inverse: [Option<u8>; 128],

    /// Separator between the human readable and base32-encoded parts of a Bech32 string
    pub separator: char,
}

impl Default for Bech32 {
    fn default() -> Self {
        Bech32::lower_case()
    }
}

impl Bech32 {
    /// Decode lower case Bech32 strings
    pub fn lower_case() -> Self {
        Self::new(DEFAULT_CHARSET, DEFAULT_SEPARATOR)
    }

    /// Decode upper case Bech32 strings
    pub fn upper_case() -> Self {
        Self::new(DEFAULT_CHARSET_UPCASE, DEFAULT_SEPARATOR)
    }

    /// Create a Bech32 instance with the given separator character
    ///
    /// Panics if the separator character is invalid
    pub fn new(charset: [char; 32], separator: char) -> Self {
        // Check separator validity
        match separator {
            '1' | 'B' | 'I' | 'O' | 'b' | 'i' | 'o' => (),
            '0'...'9' | 'A'...'Z' | 'a'...'z' => panic!("invalid separator: {:?}", separator),
            _ => (),
        }

        let mut charset_inverse = [None; 128];

        for (i, char) in charset.iter().enumerate() {
            let mut byte = [0u8];
            char.encode_utf8(byte.as_mut());
            charset_inverse[byte[0] as usize] = Some(i as u8);
        }

        Self {
            charset,
            charset_inverse,
            separator,
        }
    }

    /// Return the separator character currently in use
    pub fn separator(&self) -> char {
        self.separator
    }

    /// Encode a bech32 string from a human-readable part (hrp) and binary data
    pub fn encode<S: AsRef<str>, D: AsRef<[u8]>>(&self, hrp: S, data: D) -> String {
        let mut base32_data = base32::encode(data.as_ref());
        let mut result =
            String::with_capacity(hrp.as_ref().len() + 1 + base32_data.len() + CHECKSUM_SIZE);

        result.push_str(hrp.as_ref());
        result.push(self.separator);

        let checksum = Checksum::new(hrp.as_ref().as_bytes(), &base32_data);
        for byte in base32_data.iter().chain(checksum.as_ref().iter()) {
            let c = self
                .charset
                .get(*byte as usize)
                .expect("out of range character for alphabet");

            result.push(*c);
        }

        // Clear any potential secrets
        base32_data.as_mut_slice().clear();

        result
    }

    /// Decode a bech32 string to a human-readable part (HRP) and binary data
    pub fn decode<S: AsRef<str>>(&self, encoded: S) -> Result<(String, Vec<u8>), Error> {
        let encoded_str = encoded.as_ref();
        let encoded_len: usize = encoded_str.len();

        // TODO: support for longer strings
        if encoded_len > MAX_LENGTH {
            return Err(Error::LengthInvalid);
        }

        let pos = encoded_str
            .rfind(self.separator)
            .ok_or_else(|| Error::SeparatorMissing)?;

        if pos == encoded_str.len() {
            return Err(Error::SeparatorMissing);
        }

        let hrp = encoded_str[..pos].to_lowercase();

        if hrp.is_empty() {
            return Err(Error::PrefixInvalid);
        }

        // Ensure all characters in the human readable part are in a valid range
        for c in hrp.chars() {
            match c {
                '!'...'@' | 'A'...'Z' | '['...'`' | 'a'...'z' | '{'...'~' => (),
                _ => return Err(Error::EncodingInvalid),
            }
        }

        let encoded_data = &encoded_str[(pos.checked_add(1).unwrap())..];

        if encoded_data.len() < CHECKSUM_SIZE {
            return Err(Error::LengthInvalid);
        }

        let mut base32_data = Vec::with_capacity(encoded_data.len());

        for encoded_byte in encoded_data.bytes() {
            let decoded_byte = self
                .charset_inverse
                .get(encoded_byte as usize)
                .and_then(|byte| *byte)
                .ok_or_else(|| Error::EncodingInvalid)?;

            base32_data.push(decoded_byte);
        }

        // TODO: use catch here?
        if let Err(e) = Checksum::verify(hrp.as_bytes(), &base32_data) {
            // Clear any secrets that might be in base32_data
            base32_data.as_mut_slice().clear();
            return Err(e);
        }

        let base32_data_len = base32_data.len().checked_sub(CHECKSUM_SIZE).unwrap();
        let decode_result = base32::decode(&base32_data[..base32_data_len]);

        // Clear any secrets that might be in data_bytes
        base32_data.as_mut_slice().clear();
        decode_result.map(|decoded| (hrp, decoded))
    }
}
