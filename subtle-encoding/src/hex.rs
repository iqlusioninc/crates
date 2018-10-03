//! Adapted from this C++ implementation:
//!
//! <https://github.com/Sc00bz/ConstTimeEncoding/blob/master/hex.cpp>
//!
//! Copyright (c) 2014 Steve "Sc00bz" Thomas (steve at tobtu dot com)
//! Derived code is dual licensed MIT + Apache 2 (with permission from @Sc00bz)

use super::{
    Encoding,
    Error::{self, EncodingInvalid, LengthInvalid},
};
#[cfg(feature = "alloc")]
use prelude::*;

/// Hexadecimal `Encoding` (a.k.a. Base16)
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Hex {}

/// Return a default `Hex` encoder
#[inline]
pub fn encoder() -> Hex {
    Hex::default()
}

/// Encode the given data as hexadecimal, returning a `Vec<u8>`
#[cfg(feature = "alloc")]
pub fn encode<B: AsRef<[u8]>>(bytes: B) -> Vec<u8> {
    encoder().encode(bytes)
}

/// Decode the given data from hexadecimal, returning a `Vec<u8>`
#[cfg(feature = "alloc")]
pub fn decode<B: AsRef<[u8]>>(encoded_bytes: B) -> Result<Vec<u8>, Error> {
    encoder().decode(encoded_bytes)
}

impl Encoding for Hex {
    fn encode_to_slice(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
        for (i, src_byte) in src.iter().enumerate() {
            let offset = mul!(i, 2);
            dst[offset] = encode_nibble(shr!(src_byte, 4));
            dst[add!(offset, 1)] = encode_nibble(src_byte & 0x0f);
        }

        Ok(mul!(src.len(), 2))
    }

    fn encoded_len(&self, bytes: &[u8]) -> usize {
        mul!(bytes.len(), 2)
    }

    fn decode_to_slice(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
        let dst_length = self.decoded_len(src)?;
        ensure!(dst_length <= dst.len(), LengthInvalid);

        let mut err: usize = 0;
        for (i, dst_byte) in dst.iter_mut().enumerate().take(dst_length) {
            let src_offset = mul!(i, 2);
            let byte =
                shl!(decode_nibble(src[src_offset]), 4) | decode_nibble(src[add!(src_offset, 1)]);
            err |= shr!(byte, 8);
            *dst_byte = byte as u8;
        }

        if err == 0 {
            Ok(dst_length)
        } else {
            Err(EncodingInvalid)
        }
    }

    fn decoded_len(&self, bytes: &[u8]) -> Result<usize, Error> {
        let encoded_length = bytes.len();

        if encoded_length == 0 {
            return Ok(0);
        } else {
            ensure!(encoded_length & 1 == 0, LengthInvalid);
        }

        Ok(shr!(encoded_length, 1))
    }
}

/// Decode a single nibble of hex
#[inline]
fn decode_nibble(src: u8) -> usize {
    // 0-9  0x30-0x39
    // A-F  0x41-0x46 or a-f  0x61-0x66
    let mut byte = src as isize;
    let mut ret: isize = -1;

    // if (byte > 0x2f && byte < 0x3a) ret += byte - 0x30 + 1; // -47
    ret = add!(
        ret,
        shr!((sub!(0x2fisize, byte) & sub!(byte, 0x3a)), 8) & sub!(byte, 47)
    );

    // case insensitive decode
    byte |= 0x20;

    // if (byte > 0x60 && byte < 0x67) ret += byte - 0x61 + 10 + 1; // -86
    add!(
        ret,
        shr!(sub!(0x60isize, byte) & sub!(byte, 0x67), 8) & sub!(byte, 86)
    ) as usize
}

/// Encode a single nibble of hex
#[inline]
fn encode_nibble(src: u8) -> u8 {
    let mut ret: isize = src as isize;

    // 0-9  0x30-0x39
    // a-f  0x61-0x66
    ret = add!(ret, 0x30);

    // if (in > 0x39) in += 0x61 - 0x3a;
    add!(ret, shr!(sub!(0x39isize, ret), 8) & sub!(0x61isize, 0x3a)) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use error::Error::*;

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
            let out_len = encoder().encode_to_slice(vector.raw, &mut out).unwrap();

            assert_eq!(vector.hex, &out[..out_len]);
        }
    }

    #[test]
    fn decode_test_vectors() {
        for vector in HEX_TEST_VECTORS {
            // 5 is the size of the largest decoded test vector
            let mut out = [0u8; 5];
            let out_len = encoder().decode_to_slice(vector.hex, &mut out).unwrap();

            assert_eq!(vector.raw, &out[..out_len]);
        }
    }

    #[test]
    fn reject_odd_size_input() {
        let mut out = [0u8; 3];
        assert_eq!(
            LengthInvalid,
            encoder().decode_to_slice(b"12345", &mut out).err().unwrap(),
        )
    }

    #[test]
    fn encode_and_decode_various_lengths() {
        let data = [b'X'; 64];

        for i in 0..data.len() {
            let encoded = encoder().encode(&data[..i]);

            // Make sure it round trips
            let decoded = encoder().decode(encoded).unwrap();

            assert_eq!(decoded.as_slice(), &data[..i]);
        }
    }
}
