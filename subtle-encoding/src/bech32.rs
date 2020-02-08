//! Bech32 (BIP-173) checksummed Base32 data encoding (WARNING: preview!)
//!
//! NOTE: This implementation is not yet constant time, but we intend to make
//! it such. It is provided as a preview of an upcoming feature, and is
//! not enabled by default.
//!
//! To enable it, add the following cargo feature: `bech32-preview`

use zeroize::Zeroize;

mod base32;
mod checksum;

use self::checksum::{Checksum, CHECKSUM_SIZE};
use crate::error::Error;
use alloc::{string::String, vec::Vec};

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

/// Encode the given data as lower-case Bech32, returning a `String`
pub fn encode<S, D>(hrp: S, data: D) -> String
where
    S: AsRef<str>,
    D: AsRef<[u8]>,
{
    Bech32::lower_case().encode(hrp, data)
}

/// Decode the given data from lower-case Bech32, returning a 2-tuple of the
/// "human readable part" of the message as a `String` and a `Vec<u8>` of data,
/// or an `Error` if decoding failed.
pub fn decode<S>(encoded: S) -> Result<(String, Vec<u8>), Error>
where
    S: AsRef<str>,
{
    Bech32::lower_case().decode(encoded)
}

/// Encode the given data as upper-case Bech32, returning a `Vec<u8>`
pub fn encode_upper<S, D>(hrp: S, data: D) -> String
where
    S: AsRef<str>,
    D: AsRef<[u8]>,
{
    Bech32::upper_case().encode(hrp, data)
}

/// Decode the given data from upper-case Bech32, returning a 2-tuple of the
/// "human readable part" of the message as a `String` and a `Vec<u8>` of data,
/// or an `Error` if decoding failed.
pub fn decode_upper<S>(encoded: S) -> Result<(String, Vec<u8>), Error>
where
    S: AsRef<str>,
{
    Bech32::upper_case().decode(encoded)
}

/// Bech32 encoder/decoder
pub struct Bech32 {
    /// Encoding character set
    charset: [char; 32],

    /// Inverse alphabet used to decode
    charset_inverse: [Option<u8>; 128],

    /// Separator between the human readable and base32-encoded parts of a Bech32 string
    separator: char,
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

    /// Create a `Bech32` encoder with the given separator character
    ///
    /// Panics if the separator character is invalid
    pub fn new(charset: [char; 32], separator: char) -> Self {
        // Check separator validity
        match separator {
            '1' | 'B' | 'I' | 'O' | 'b' | 'i' | 'o' => (),
            '0'..='9' | 'A'..='Z' | 'a'..='z' => panic!("invalid separator: {:?}", separator),
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
    pub fn encode<S, D>(&self, hrp: S, data: D) -> String
    where
        S: AsRef<str>,
        D: AsRef<[u8]>,
    {
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
        base32_data.as_mut_slice().zeroize();

        result
    }

    /// Decode a bech32 string to a human-readable part (HRP) and binary data
    pub fn decode<S>(&self, encoded: S) -> Result<(String, Vec<u8>), Error>
    where
        S: AsRef<str>,
    {
        let encoded_str = encoded.as_ref();

        // TODO: constant-time whitespace tolerance
        if encoded_str
            .chars()
            .last()
            .map(|c| c.is_whitespace())
            .unwrap_or(false)
        {
            return Err(Error::TrailingWhitespace);
        }

        let pos = encoded_str
            .rfind(self.separator)
            .ok_or_else(|| Error::EncodingInvalid)?;

        if pos == encoded_str.len() {
            return Err(Error::EncodingInvalid);
        }

        let hrp = encoded_str[..pos].to_lowercase();

        if hrp.is_empty() {
            return Err(Error::EncodingInvalid);
        }

        // Ensure all characters in the human readable part are in a valid range
        for c in hrp.chars() {
            match c {
                '!'..='@' | 'A'..='Z' | '['..='`' | 'a'..='z' | '{'..='~' => (),
                _ => return Err(Error::EncodingInvalid),
            }
        }

        let encoded_data = &encoded_str[(pos + 1)..];

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
            base32_data.as_mut_slice().zeroize();
            return Err(e);
        }

        let base32_len = base32_data.len() - CHECKSUM_SIZE;
        let decode_result = base32::decode(&base32_data[..base32_len]);

        // Clear any secrets that might be in data_bytes
        base32_data.as_mut_slice().zeroize();
        decode_result.map(|decoded| (hrp, decoded))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Bech32 test vector
    struct TestVector {
        /// Bech32-encoded string
        encoded: &'static str,

        /// Human readable part
        hrp: &'static str,

        /// Binary data
        bytes: &'static [u8],

        /// Is the test vector upper case?
        upper_case: bool,
    }

    // BIP-173 test vectors
    // https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki#Test_vectors
    const VALID_TEST_VECTORS: &[TestVector] = &[
        TestVector {
            encoded: "A12UEL5L",
            hrp: "a",
            bytes: &[],
            upper_case: true
        },
        TestVector {
            encoded: "a12uel5l",
            hrp: "a",
            bytes: &[],
            upper_case: false
        },
        TestVector {
            encoded: "an83characterlonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio1tt5tgs",
            hrp: "an83characterlonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio",
            bytes: &[],
            upper_case: false
        },
        TestVector {
            hrp: "abcdef",
            bytes: &[0, 68, 50, 20, 199, 66, 84, 182, 53, 207, 132, 101, 58, 86, 215, 198, 117, 190, 119, 223],
            encoded: "abcdef1qpzry9x8gf2tvdw0s3jn54khce6mua7lmqqqxw",
            upper_case: false
        },
        TestVector {
            encoded: "11qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqc8247j",
            hrp: "1",
            bytes: &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            upper_case: false
        },
        TestVector {
            encoded: "split1checkupstagehandshakeupstreamerranterredcaperred2y9e3w",
            hrp: "split",
            bytes: &[197, 243, 139, 112, 48, 95, 81, 155, 246, 109, 133, 251, 108, 240, 48, 88, 243, 221, 228, 99, 236, 215, 145, 143, 45, 199, 67, 145, 143, 45],
            upper_case: false
        },
        TestVector {
            encoded: "?1ezyfcl",
            hrp: "?",
            bytes: &[],
            upper_case: false
        },
    ];

    #[test]
    fn encode_valid_test_vectors() {
        let bech32 = Bech32::default();
        for vector in VALID_TEST_VECTORS {
            let encoded = bech32.encode(vector.hrp, vector.bytes);
            assert_eq!(encoded, vector.encoded.to_lowercase());
        }
    }

    #[test]
    fn decode_valid_test_vectors() {
        for vector in VALID_TEST_VECTORS {
            let bech32 = if vector.upper_case {
                Bech32::upper_case()
            } else {
                Bech32::default()
            };

            let (hrp, data) = bech32.decode(vector.encoded).unwrap();
            assert_eq!(hrp, vector.hrp.to_lowercase());
            assert_eq!(data, vector.bytes);
        }
    }

    #[test]
    fn hrp_character_out_of_range() {
        let bech32 = Bech32::default();
        assert_eq!(bech32.decode("\x201nwldj5"), Err(Error::EncodingInvalid));
        assert_eq!(bech32.decode("\x7F1axkwrx"), Err(Error::EncodingInvalid));
    }

    #[test]
    fn no_separator_character() {
        assert_eq!(
            Bech32::default().decode("pzry9x0s0muk"),
            Err(Error::EncodingInvalid)
        );
    }

    #[test]
    fn empty_hrp() {
        for empty_hrp_str in &["1pzry9x0s0muk", "10a06t8", "1qzzfhee"] {
            assert_eq!(
                Bech32::default().decode(empty_hrp_str),
                Err(Error::EncodingInvalid)
            );
        }
    }

    #[test]
    fn invalid_data_character() {
        assert_eq!(
            Bech32::default().decode("x1b4n0q5v"),
            Err(Error::EncodingInvalid)
        );
    }

    #[test]
    fn checksum_too_short() {
        assert_eq!(
            Bech32::default().decode("li1dgmt3"),
            Err(Error::LengthInvalid)
        );
    }

    #[test]
    fn invalid_character_in_checksum() {
        assert_eq!(
            Bech32::default().decode("de1lg7wt\x7F"),
            Err(Error::EncodingInvalid)
        );
    }

    #[test]
    fn checksum_calculated_with_uppercase_hrp() {
        assert_eq!(
            Bech32::upper_case().decode("A1G7SGD8"),
            Err(Error::ChecksumInvalid)
        );
    }

    // NOTE: not in test vectors but worth testing for anyway
    #[test]
    fn invalid_mixed_case() {
        assert_eq!(
            Bech32::default().decode("a12UEL5L"),
            Err(Error::EncodingInvalid)
        );
    }
}
