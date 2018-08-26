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
#![doc(html_root_url = "https://docs.rs/iq-bech32/0.0.1")]

extern crate clear_on_drop;
#[macro_use]
extern crate failure;

use clear_on_drop::clear::Clear;

mod base32;
mod charset;
mod checksum;
mod error;

use charset::{CHARSET, CHARSET_INVERSE};
use checksum::{Checksum, CHECKSUM_SIZE};

pub use charset::SEPARATOR;
pub use error::Error;

/// Maximum length of a bech32 string
pub const MAX_LENGTH: usize = 90;

/// Bech32 encoder/decoder
pub struct Bech32 {
    /// Separator between the human readable and base32-encoded parts of a Bech32 string
    separator: char,
}

impl Default for Bech32 {
    fn default() -> Self {
        Self {
            separator: SEPARATOR,
        }
    }
}

impl Bech32 {
    /// Create a Bech32 instance with the given separator character
    ///
    /// Panics if the separator character is invalid
    pub fn new(separator: char) -> Self {
        // Check separator validity
        match separator {
            '1' | 'B' | 'I' | 'O' | 'b' | 'i' | 'o' => (),
            '0'...'9' | 'A'...'Z' | 'a'...'z' => panic!("invalid separator: {:?}", separator),
            _ => (),
        }

        Self { separator }
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
            result.push(CHARSET[*byte as usize]);
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

        check_character_validity(encoded_str)?;

        let pos = encoded_str
            .rfind(self.separator)
            .ok_or_else(|| Error::SeparatorMissing)?;

        let hrp = encoded_str[..pos].to_lowercase();

        if hrp.is_empty() {
            return Err(Error::PrefixInvalid);
        }

        let encoded_data = &encoded_str[(pos + 1)..];

        if encoded_data.len() < CHECKSUM_SIZE {
            return Err(Error::LengthInvalid);
        }

        let mut base32_data = Vec::with_capacity(encoded_data.len());

        for encoded_byte in encoded_data.bytes() {
            let decoded_byte = CHARSET_INVERSE[encoded_byte as usize]
                .ok_or_else(|| Error::DataInvalid { byte: encoded_byte })?;

            base32_data.push(decoded_byte);
        }

        // TODO: use catch here?
        if let Err(e) = Checksum::verify(hrp.as_bytes(), &base32_data) {
            // Clear any secrets that might be in base32_data
            base32_data.as_mut_slice().clear();
            return Err(e);
        }

        let base32_data_len = base32_data.len() - CHECKSUM_SIZE;
        let decode_result = base32::decode(&base32_data[..base32_data_len]);

        // Clear any secrets that might be in data_bytes
        base32_data.as_mut_slice().clear();
        decode_result.map(|decoded| (hrp, decoded))
    }
}

/// Ensure the characters in the string have uniform case and are in range
fn check_character_validity(encoded: &str) -> Result<(), Error> {
    let mut has_lower: bool = false;
    let mut has_upper: bool = false;

    for char in encoded.chars() {
        match char {
            '!'...'@' => (),
            'A'...'Z' => has_upper = true,
            '['...'`' => (),
            'a'...'z' => has_lower = true,
            '{'...'~' => (),
            _ => return Err(Error::CharInvalid { char }),
        }
    }

    if has_lower && has_upper {
        return Err(Error::CaseInvalid);
    }

    Ok(())
}
