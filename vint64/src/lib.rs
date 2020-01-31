//! `vint64`: simple and efficient variable-length integer encoding
//!
//! # About
//!
//! This crate implements a variable-length encoding for 64-bit little endian
//! integers with a number of properties which make it superior in almost every
//! way to other variable-length integer encodings like [LEB128], SQLite "Varuints",
//! or CBOR:
//!
//! - Capable of expressing the full 64-bit integer range with a maximum of 9-bytes
//! - Total length of a `vint64` can be determined via the first byte alone
//! - Provides the most compact encoding possible for every value in range
//! - No loops involved in decoding: just (unaligned) loads, masks, and shifts
//! - No complex branch-heavy logic: decoding is CTZ + shifts and sanity checks
//!
//! Integers serialized as unsigned `vint64` are (up to) 64-bit unsigned little
//! endian integers, with the `[0, (2⁶⁴)−1]` range supported.
//!
//! They have serialized lengths from 1-byte to 9-bytes depending on what value
//! they're representing. The number of remaining bytes is stored in the leading
//! byte, indicated by the number of trailing zeroes in that byte.
//!
//! Below is an example of how prefix bits signal the length of the integer value
//! which follows:
//!
//! | Prefix     | Precision | Total Bytes |
//! |------------|-----------|-------------|
//! | `xxxxxxx1` | 7 bits    | 1 byte      |
//! | `xxxxxx10` | 14 bits   | 2 bytes     |
//! | `xxxxx100` | 21 bits   | 3 bytes     |
//! | `xxxx1000` | 28 bits   | 4 bytes     |
//! | `xxx10000` | 35 bits   | 5 bytes     |
//! | `xx100000` | 42 bits   | 6 bytes     |
//! | `x1000000` | 49 bits   | 7 bytes     |
//! | `10000000` | 56 bits   | 8 bytes     |
//! | `00000000` | 64 bits   | 9 bytes     |
//!
//! All arithmetic needed to serialize and deserialize `vint64` can be performed
//! using only 64-bit integers. The case of the prefix byte being all-zero is
//! a special case, and any remaining arithmetic is performed on the remaining
//! bytes.
//!
//! Some precedent for this sort of encoding can be found in the
//! [Extensible Binary Meta Language] (used by e.g. the [Matroska]
//! media container format), however note that the specific type of "vint"
//! used by that format still requires a loop to decode.
//!
//! # Usage
//!
//! ```
//! // Encode a 64-bit integer as a vint64
//! let encoded = vint64::encode(42);
//! assert_eq!(encoded.as_ref(), &[0x55]);
//!
//! // Get the length of a `vint64` from its first byte.
//! // NOTE: this is inclusive of the first byte itself.
//! let encoded_len = vint64::length_hint(encoded.as_ref()[0]);
//!
//! // Decode an encoded vint64 with trailing data
//! let mut slice: &[u8] = &[0x55, 0xde, 0xad, 0xbe, 0xef];
//! let decoded = vint64::decode(&mut slice).unwrap();
//! assert_eq!(decoded, 42);
//! assert_eq!(slice, &[0xde, 0xad, 0xbe, 0xef]);
//!
//! // Zigzag encoding can be used to encode signed vint64s.
//! // Decode with `vint64::decode_signed`.
//! let signed = vint64::encode_signed(-42);
//! assert_eq!(signed.as_ref(), &[0xa7]);
//! ```
//!
//! [LEB128]: https://cr.yp.to/libtai/vint.html
//! [Extensible Binary Meta Language]: https://en.wikipedia.org/wiki/Extensible_Binary_Meta_Language
//! [Matroska]: https://www.matroska.org/

#![no_std]
#![doc(html_root_url = "https://docs.rs/vint64/0.1.2")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use core::{
    convert::{TryFrom, TryInto},
    fmt::{self, Debug},
};

/// Maximum length of a `vint64` in bytes
pub const MAX_BYTES: usize = 9;

/// Error type: indicates decoding failure
#[derive(Copy, Clone, Debug)]
pub struct Error;

/// `vint64`: serialized variable-width 64-bit integers
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vint64 {
    /// Serialized variable-width integer
    bytes: [u8; MAX_BYTES],

    /// Encoded length in bytes
    length: usize,
}

impl AsRef<[u8]> for Vint64 {
    fn as_ref(&self) -> &[u8] {
        &self.bytes[..self.length]
    }
}

impl Debug for Vint64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bytes_ref = self.as_ref();
        write!(f, "Vint64({})", decode(&mut bytes_ref).unwrap())
    }
}

impl From<u64> for Vint64 {
    fn from(value: u64) -> Vint64 {
        let mut length = 1;
        let mut result = (value << 1) | 1;
        let mut max = 1 << 7;
        let mut bytes = [0u8; MAX_BYTES];

        while value >= max {
            // 9-byte special case
            if length == 8 {
                bytes[1..].copy_from_slice(&value.to_le_bytes());
                return Self { bytes, length: 9 };
            }

            result <<= 1;
            max <<= 7;
            length += 1;
        }

        bytes[..8].copy_from_slice(&result.to_le_bytes());
        Self { bytes, length }
    }
}

impl From<i64> for Vint64 {
    fn from(value: i64) -> Vint64 {
        (((value << 1) ^ (value >> 63)) as u64).into()
    }
}

impl TryFrom<&[u8]> for Vint64 {
    type Error = Error;

    fn try_from(slice: &[u8]) -> Result<Self, Error> {
        let mut slice_ref = slice;
        decode(&mut slice_ref).map(Vint64::from)
    }
}

/// Get the length of a `vint64` from the first byte.
///
/// NOTE: The returned value is inclusive of the first byte itself.
pub fn length_hint(byte: u8) -> usize {
    byte.trailing_zeros() as usize + 1
}

/// Encode an unsigned 64-bit integer as `vint64`
pub fn encode(value: u64) -> Vint64 {
    value.into()
}

/// Decode a `vint64`-encoded unsigned 64-bit integer.
///
/// Accepts a mutable reference to a slice containing the `vint64`.
/// Upon success, the reference is updated to begin at the byte immediately
/// after the encoded `vint64`.
pub fn decode(input: &mut &[u8]) -> Result<u64, Error> {
    let bytes = *input;
    let length = length_hint(*bytes.first().ok_or_else(|| Error)?);

    if length == 9 {
        if bytes.len() < 9 {
            return Err(Error);
        }

        let result = u64::from_le_bytes(bytes[1..9].try_into().unwrap());

        // Ensure there are no superfluous trailing zeros
        if result < (1 << 56) {
            return Err(Error);
        }

        *input = &bytes[9..];
        return Ok(result);
    }

    if bytes.len() < length {
        return Err(Error);
    }

    let mut encoded = [0u8; 8];
    encoded[..length].copy_from_slice(&bytes[..length]);
    let result = u64::from_le_bytes(encoded) >> length;

    // Ensure there are no superfluous trailing zeros
    if length > 1 && result < (1 << (7 * (length - 1))) {
        return Err(Error);
    }

    *input = &bytes[length..];
    Ok(result)
}

/// Encode a signed integer as a zigzag-encoded `vint64`
pub fn encode_signed(value: i64) -> Vint64 {
    value.into()
}

/// Decode a zigzag-encoded `vint64` as a signed integer
pub fn decode_signed(input: &mut &[u8]) -> Result<i64, Error> {
    let decoded = decode(input)?;
    Ok((decoded >> 1) as i64 ^ -((decoded & 1) as i64))
}

#[cfg(test)]
mod tests {
    use super::{decode, decode_signed, encode, encode_signed};

    #[test]
    fn encode_zero() {
        assert_eq!(encode(0).as_ref(), &[1]);
    }

    #[test]
    fn encode_bit_pattern_examples() {
        assert_eq!(encode(0x0f0f).as_ref(), &[0x3e, 0x3c]);
        assert_eq!(encode(0x0f0f_f0f0).as_ref(), &[0x08, 0x0f, 0xff, 0xf0]);
        assert_eq!(
            encode(0x0f0f_f0f0_0f0f).as_ref(),
            &[0xc0, 0x87, 0x07, 0x78, 0xf8, 0x87, 0x07]
        );
        assert_eq!(
            encode(0x0f0f_f0f0_0f0f_f0f0).as_ref(),
            &[0x00, 0xf0, 0xf0, 0x0f, 0x0f, 0xf0, 0xf0, 0x0f, 0x0f]
        );
    }

    #[test]
    fn encode_maxint() {
        assert_eq!(
            encode(core::u64::MAX).as_ref(),
            &[0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );
    }

    #[test]
    fn encode_signed_values() {
        assert_eq!(
            encode_signed(0x0f0f_f0f0).as_ref(),
            &[0x10, 0x3c, 0xfc, 0xc3, 0x03]
        );

        assert_eq!(
            encode_signed(-0x0f0f_f0f0).as_ref(),
            &[0xf0, 0x3b, 0xfc, 0xc3, 0x03]
        );
    }

    #[test]
    fn decode_zero() {
        let mut slice = [1].as_ref();
        assert_eq!(decode(&mut slice).unwrap(), 0);
    }

    #[test]
    fn decode_bit_pattern_examples() {
        let mut slice = [0x3e, 0x3c].as_ref();
        assert_eq!(decode(&mut slice).unwrap(), 0x0f0f);
        assert!(slice.is_empty());

        let mut slice = [0x08, 0x0f, 0xff, 0xf0].as_ref();
        assert_eq!(decode(&mut slice).unwrap(), 0x0f0f_f0f0);
        assert!(slice.is_empty());

        let mut slice = [0xc0, 0x87, 0x07, 0x78, 0xf8, 0x87, 0x07].as_ref();
        assert_eq!(decode(&mut slice).unwrap(), 0x0f0f_f0f0_0f0f);
        assert!(slice.is_empty());

        let mut slice = [0x00, 0xf0, 0xf0, 0x0f, 0x0f, 0xf0, 0xf0, 0x0f, 0x0f].as_ref();
        assert_eq!(decode(&mut slice).unwrap(), 0x0f0f_f0f0_0f0f_f0f0);
        assert!(slice.is_empty());
    }

    #[test]
    fn decode_maxint() {
        let mut slice = [0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff].as_ref();
        assert_eq!(decode(&mut slice).unwrap(), core::u64::MAX);
        assert!(slice.is_empty());
    }

    #[test]
    fn decode_with_trailing_data() {
        let mut slice = [0x3e, 0x3c, 0xde, 0xad, 0xbe, 0xef].as_ref();
        assert_eq!(decode(&mut slice).unwrap(), 0x0f0f);
        assert_eq!(slice, &[0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn decode_truncated() {
        let mut slice = [0].as_ref();
        assert!(decode(&mut slice).is_err());

        let mut slice = [0x08, 0x0f, 0xff].as_ref();
        assert!(decode(&mut slice).is_err());
    }

    #[test]
    fn decode_trailing_zeroes() {
        let mut slice = [0x08, 0x00, 0x00, 0x00].as_ref();
        assert!(decode(&mut slice).is_err());
    }

    #[test]
    fn decode_signed_values() {
        let mut slice = [0x10, 0x3c, 0xfc, 0xc3, 0x03].as_ref();
        assert_eq!(decode_signed(&mut slice).unwrap(), 0x0f0f_f0f0);

        let mut slice = [0xf0, 0x3b, 0xfc, 0xc3, 0x03].as_ref();
        assert_eq!(decode_signed(&mut slice).unwrap(), -0x0f0f_f0f0);
    }
}
