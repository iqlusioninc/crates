//! bech32k: KeyURI-specific bech32 encoding/decoding support

mod base32;
mod checksum;
mod error;

use self::checksum::Checksum;
pub use self::error::Error;

/// Minimum length of a bech32k string
pub const MIN_LENGTH: usize = 8;

/// Maximum length of a bech32k string
pub const MAX_LENGTH: usize = 90;

/// bech32k encoding character set (same as bech32)
const CHARSET: [char; 32] = [
    'q', 'p', 'z', 'r', 'y', '9', 'x', '8', 'g', 'f', '2', 't', 'v', 'd', 'w', '0', 's', '3', 'j',
    'n', '5', '4', 'k', 'h', 'c', 'e', '6', 'm', 'u', 'a', '7', 'l',
];

/// Inverse mapping from character codes to CHARSET indexes
const CHARSET_INVERSE: [i8; 128] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    15, -1, 10, 17, 21, 20, 26, 30, 7, 5, -1, -1, -1, -1, -1, -1, -1, 29, -1, 24, 13, 25, 9, 8, 23,
    -1, 18, 22, 31, 27, 19, -1, 1, 0, 3, 16, 11, 28, 12, 14, 6, 4, 2, -1, -1, -1, -1, -1, -1, 29,
    -1, 24, 13, 25, 9, 8, 23, -1, 18, 22, 31, 27, 19, -1, 1, 0, 3, 16, 11, 28, 12, 14, 6, 4, 2, -1,
    -1, -1, -1, -1,
];

/// bech32k-specific separator between URI prefix and encoded data
///
/// Note this differs from bech32's `1` character
const SEPARATOR: char = ';';

/// Encode a bech32k string from a string prefix and binary data
pub fn encode(prefix: &str, data: &[u8]) -> String {
    let base32_data = base32::encode(data).unwrap();
    let checksum = Checksum::new(prefix.as_bytes(), &base32_data);
    let data_with_checksum: String = base32_data
        .iter()
        .chain(checksum.as_ref().iter())
        .map(|byte| CHARSET[*byte as usize])
        .collect();

    format!("{}{}{}", prefix, SEPARATOR, data_with_checksum)
}

/// Decode a bech32k string to a prefix string and binary data
pub fn decode(encoded: &str) -> Result<(String, Vec<u8>), Error> {
    let len: usize = encoded.len();

    if encoded.find(SEPARATOR).is_none() {
        return Err(Error::SeparatorMissing);
    }

    match len {
        MIN_LENGTH...MAX_LENGTH => (),
        _ => return Err(Error::LengthInvalid),
    }

    let parts: Vec<&str> = encoded.splitn(2, SEPARATOR).collect();

    let prefix = parts[0];
    if prefix.is_empty() {
        return Err(Error::LengthInvalid);
    }

    let data = parts[1];
    if data.len() < 6 {
        return Err(Error::LengthInvalid);
    }

    let mut has_lower: bool = false;
    let mut has_upper: bool = false;
    let mut prefix_bytes = vec![];

    for mut byte in prefix.bytes() {
        match byte {
            33...126 => (),
            _ => return Err(Error::CharInvalid { byte }),
        }

        match byte {
            b'A'...b'Z' => {
                has_upper = true;
                byte += b'a' - b'A'
            }
            b'a'...b'z' => {
                has_lower = true;
            }
            _ => (),
        }

        prefix_bytes.push(byte);
    }

    let mut data_bytes = vec![];

    for mut byte in data.bytes() {
        // Check character validity
        match byte {
            b'0'...b'9' | b'A'...b'Z' | b'a'...b'z' => match byte {
                // These characters are not valid
                b'1' | b'B' | b'I' | b'O' | b'b' | b'i' | b'o' => {
                    return Err(Error::CharInvalid { byte })
                }
                _ => (),
            },
            _ => return Err(Error::CharInvalid { byte }),
        }

        // Check for mixed case (otherwise converting upper case to lower case)
        match byte {
            b'A'...b'Z' => {
                has_upper = true;
                byte += b'a' - b'A';
            }
            b'a'...b'z' => {
                has_lower = true;
            }
            _ => (),
        }

        data_bytes.push(CHARSET_INVERSE[byte as usize] as u8);
    }

    if has_lower && has_upper {
        return Err(Error::CaseInvalid);
    }

    Checksum::verify(&prefix_bytes, &data_bytes)?;

    let data_bytes_len = data_bytes.len();
    data_bytes.truncate(data_bytes_len - 6);

    Ok((
        String::from_utf8(prefix_bytes).unwrap(),
        base32::decode(&data_bytes)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};

    const EXAMPLE_PREFIX: &str = "example.prefix";
    const EXAMPLE_DATA: &[u8] = &[0, 255, 1, 2, 3, 42, 101];
    const EXAMPLE_ENCODED: &str = "example.prefix;qrlszqsr9fjsjhjw53";

    #[test]
    fn test_beck32k_roundtrip() {
        let encoded = encode(EXAMPLE_PREFIX, EXAMPLE_DATA);
        assert_eq!(encoded, EXAMPLE_ENCODED);

        let (prefix, data) = decode(&encoded).unwrap();
        assert_eq!(prefix, EXAMPLE_PREFIX);
        assert_eq!(data, EXAMPLE_DATA);
    }
}
