//! Hex encoding/decoding with data-independent constant time(-ish) operation.
//!
//! Adapted from this C++ implementation:
//!
//! <https://github.com/Sc00bz/ConstTimeEncoding/blob/master/hex.cpp>
//!
//! Copyright (c) 2014 Steve "Sc00bz" Thomas (steve at tobtu dot com)
//! Derived code is dual licensed MIT + Apache 2.0 (with permission from @Sc00bz)
//!
//! ## Usage
//!
//! The main API acts as a drop-in replacement for the `hex` crate
//!
//! ### Encoding
//!
//! Use the [hex::encode] method:
//!
//! ```
//! use subtle_encoding::hex;
//!
//! let encoded = hex::encode([0, 1, 2, 3, 4, 5]);
//!
//! // Prints the hex encoded version: "000102030405"
//! println!("hex encoded: {}", String::from_utf8(encoded).unwrap());
//! ```
//!
//! By default hex is encoded in lower-case. To encode upper-case hex, use the
//! [hex::encode_upper] method.
//!
//! ### Decoding
//!
//! Use the [hex::decode] method, or [hex::decode_upper] for upper-case hex.
//!
//! [hex::encode]: https://docs.rs/subtle-encoding/latest/subtle_encoding/hex/fn.encode.html
//! [hex::encode_upper]: https://docs.rs/subtle-encoding/latest/subtle_encoding/hex/fn.encode_upper.html
//! [hex::decode]: https://docs.rs/subtle-encoding/latest/subtle_encoding/hex/fn.decode.html
//! [hex::decode_upper]: https://docs.rs/subtle-encoding/latest/subtle_encoding/hex/fn.decode_upper.html

use super::{
    Encoding,
    Error::{self, *},
};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Encode the given data as lower-case hexadecimal, returning a `Vec<u8>`
#[cfg(feature = "alloc")]
pub fn encode<B: AsRef<[u8]>>(bytes: B) -> Vec<u8> {
    Hex::lower_case().encode(bytes)
}

/// Decode the given data from lower-case hexadecimal, returning a `Vec<u8>`
/// of the decoded data on success, or an `Error`.
#[cfg(feature = "alloc")]
pub fn decode<B: AsRef<[u8]>>(encoded_bytes: B) -> Result<Vec<u8>, Error> {
    Hex::lower_case().decode(encoded_bytes)
}

/// Encode the given data as upper-case hexadecimal, returning a `Vec<u8>`
#[cfg(feature = "alloc")]
pub fn encode_upper<B: AsRef<[u8]>>(bytes: B) -> Vec<u8> {
    Hex::upper_case().encode(bytes)
}

/// Decode the given data from upper-case hexadecimal, returning a `Vec<u8>`
/// of the decoded data on success, or an `Error`.
#[cfg(feature = "alloc")]
pub fn decode_upper<B: AsRef<[u8]>>(encoded_bytes: B) -> Result<Vec<u8>, Error> {
    Hex::upper_case().decode(encoded_bytes)
}

/// Hexadecimal `Encoding` (a.k.a. Base16)
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Hex {
    /// Upper or lower case
    case: Case,
}

impl Hex {
    /// Lower case hex: 0-9 a-f
    pub fn lower_case() -> Hex {
        Hex { case: Case::Lower }
    }

    /// Upper case hex: 0-9 A-F
    pub fn upper_case() -> Hex {
        Hex { case: Case::Upper }
    }
}

impl Encoding for Hex {
    fn encode_to_slice(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
        if self.encoded_len(src) > dst.len() {
            return Err(LengthInvalid);
        }

        for (i, src_byte) in src.iter().enumerate() {
            let offset = i * 2;
            dst[offset] = self.case.encode_nibble(src_byte >> 4);
            dst[offset + 1] = self.case.encode_nibble(src_byte & 0x0f);
        }

        Ok(src.len() * 2)
    }

    fn encoded_len(&self, bytes: &[u8]) -> usize {
        bytes.len() * 2
    }

    fn decode_to_slice(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
        // TODO: constant-time whitespace tolerance
        if !src.is_empty() && char::from(src[src.len() - 1]).is_whitespace() {
            return Err(TrailingWhitespace);
        }

        let dst_length = self.decoded_len(src)?;
        ensure!(dst_length <= dst.len(), LengthInvalid);

        let mut err: usize = 0;

        for (i, dst_byte) in dst.iter_mut().enumerate().take(dst_length) {
            let src_offset = i * 2;
            let byte = (self.case.decode_nibble(src[src_offset]) << 4)
                | self.case.decode_nibble(src[src_offset + 1]);
            err |= byte >> 8;
            *dst_byte = byte as u8;
        }

        if err == 0 {
            Ok(dst_length)
        } else {
            Err(EncodingInvalid)
        }
    }

    fn decoded_len(&self, bytes: &[u8]) -> Result<usize, Error> {
        if bytes.len() & 1 == 0 {
            Ok(bytes.len() >> 1)
        } else {
            Err(LengthInvalid)
        }
    }
}

/// Lower or upper case encoders
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Case {
    Lower,
    Upper,
}

impl Case {
    /// Decode a single nibble of hex (lower or upper case)
    #[inline]
    fn decode_nibble(self, src: u8) -> usize {
        // 0-9  0x30-0x39
        // A-F  0x41-0x46 or a-f  0x61-0x66
        let byte = src as isize;
        let mut ret: isize = -1;

        // 0-9  0x30-0x39
        // if (byte > 0x2f && byte < 0x3a) ret += byte - 0x30 + 1; // -47
        ret += (((0x2fisize - byte) & (byte - 0x3a)) >> 8) & (byte - 47);

        ret += match self {
            Case::Lower => {
                // a-f  0x61-0x66
                // if (byte > 0x60 && byte < 0x67) ret += byte - 0x61 + 10 + 1; // -86
                (((0x60isize - byte) & (byte - 0x67)) >> 8) & (byte - 86)
            }
            Case::Upper => {
                // A-F  0x41-0x46
                // if (byte > 0x40 && byte < 0x47) ret += byte - 0x41 + 10 + 1; // -54
                (((0x40isize - byte) & (byte - 0x47)) >> 8) & (byte - 54)
            }
        };

        ret as usize
    }

    /// Encode a single nibble of hex
    #[inline]
    fn encode_nibble(self, src: u8) -> u8 {
        let mut ret = src as isize + 0x30;

        ret += match self {
            Case::Lower => {
                // 0-9  0x30-0x39
                // a-f  0x61-0x66
                ((0x39isize - ret) >> 8) & (0x61isize - 0x3a)
            }
            Case::Upper => {
                // 0-9  0x30-0x39
                // A-F  0x41-0x46
                ((0x39isize - ret) >> 8) & (0x41isize - 0x3a)
            }
        };

        ret as u8
    }
}

impl Default for Case {
    /// Default: lower case
    fn default() -> Case {
        Case::Lower
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error::*;

    /// Hexadecimal test vectors
    struct HexVector {
        /// Raw bytes
        raw: &'static [u8],

        /// Hex encoded
        hex: &'static [u8],
    }

    const HEX_TEST_VECTORS: &[HexVector] = &[
        HexVector { raw: b"", hex: b"" },
        HexVector {
            raw: b"\0",
            hex: b"00",
        },
        HexVector {
            raw: b"***",
            hex: b"2a2a2a",
        },
        HexVector {
            raw: b"\x01\x02\x03\x04",
            hex: b"01020304",
        },
        HexVector {
            raw: b"\xAD\xAD\xAD\xAD\xAD",
            hex: b"adadadadad",
        },
        HexVector {
            raw: b"\xFF\xFF\xFF\xFF\xFF",
            hex: b"ffffffffff",
        },
    ];

    #[test]
    fn encode_test_vectors() {
        for vector in HEX_TEST_VECTORS {
            // 10 is the size of the largest encoded test vector
            let mut out = [0u8; 10];
            let out_len = Hex::lower_case()
                .encode_to_slice(vector.raw, &mut out)
                .unwrap();

            assert_eq!(vector.hex, &out[..out_len]);
        }
    }

    #[test]
    fn decode_test_vectors() {
        for vector in HEX_TEST_VECTORS {
            // 5 is the size of the largest decoded test vector
            let mut out = [0u8; 5];
            let out_len = Hex::lower_case()
                .decode_to_slice(vector.hex, &mut out)
                .unwrap();

            assert_eq!(vector.raw, &out[..out_len]);
        }
    }

    #[test]
    fn reject_odd_size_input() {
        let mut out = [0u8; 3];
        assert_eq!(
            LengthInvalid,
            Hex::lower_case()
                .decode_to_slice(b"12345", &mut out)
                .err()
                .unwrap(),
        )
    }

    #[test]
    fn encode_and_decode_various_lengths() {
        let data = [b'X'; 64];

        for i in 0..data.len() {
            let encoded = Hex::lower_case().encode(&data[..i]);

            // Make sure it round trips
            let decoded = Hex::lower_case().decode(encoded).unwrap();

            assert_eq!(decoded.as_slice(), &data[..i]);
        }
    }
}
