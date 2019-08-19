//! Base64 encoding with (almost) data-independent constant time(-ish) operation.
//!
//! Adapted from this C++ implementation:
//!
//! <https://github.com/Sc00bz/ConstTimeEncoding/blob/master/base64.cpp>
//!
//! Copyright (c) 2014 Steve "Sc00bz" Thomas (steve at tobtu dot com)
//! Derived code is dual licensed MIT + Apache 2 (with permission)

use super::{
    Encoding,
    Error::{self, *},
};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use zeroize::Zeroize;

/// Base64 `Encoding` (traditional non-URL-safe RFC 4648 version)
///
/// Character set: `[A-Z]`, `[a-z]`, `[0-9]`, `+`, `/`
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Base64 {}

/// Return a `Base64` encoder
#[inline]
pub fn encoder() -> Base64 {
    Base64::default()
}

/// Encode the given data as Base64, returning a `Vec<u8>`
#[cfg(feature = "alloc")]
pub fn encode<B: AsRef<[u8]>>(bytes: B) -> Vec<u8> {
    encoder().encode(bytes)
}

/// Decode the given data from Base64, returning a `Vec<u8>`
#[cfg(feature = "alloc")]
pub fn decode<B: AsRef<[u8]>>(encoded_bytes: B) -> Result<Vec<u8>, Error> {
    encoder().decode(encoded_bytes)
}

impl Encoding for Base64 {
    fn encode_to_slice(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
        if self.encoded_len(src) > dst.len() {
            return Err(LengthInvalid);
        }

        let mut src_offset: usize = 0;
        let mut dst_offset: usize = 0;
        let mut src_length: usize = src.len();

        while src_length >= 3 {
            encode_3bytes(
                &src[src_offset..(src_offset + 3)],
                &mut dst[dst_offset..(dst_offset + 4)],
            );

            src_offset += 3;
            dst_offset += 4;
            src_length -= 3;
        }

        if src_length > 0 {
            let mut tmp = [0u8; 3];
            tmp[..src_length].copy_from_slice(&src[src_offset..(src_offset + src_length)]);
            encode_3bytes(&tmp, &mut dst[dst_offset..]);
            tmp.zeroize();

            dst[dst_offset + 3] = b'=';

            if src_length == 1 {
                dst[dst_offset + 2] = b'=';
            }

            dst_offset += 4;
        }

        Ok(dst_offset)
    }

    fn encoded_len(&self, bytes: &[u8]) -> usize {
        (((bytes.len() * 4) / 3) + 3) & !3
    }

    fn decode_to_slice(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
        // TODO: constant-time whitespace tolerance
        if !src.is_empty() && char::from(src[src.len() - 1]).is_whitespace() {
            return Err(TrailingWhitespace);
        }

        ensure!(self.decoded_len(src)? <= dst.len(), LengthInvalid);

        let mut src_offset: usize = 0;
        let mut dst_offset: usize = 0;
        let mut src_length: usize = src.len();
        let mut err: isize = 0;

        while src_length > 4 {
            err |= decode_3bytes(
                &src[src_offset..(src_offset + 4)],
                &mut dst[dst_offset..(dst_offset + 3)],
            );
            src_offset += 4;
            dst_offset += 3;
            src_length -= 4;
        }

        if src_length > 0 {
            let mut i = 0;
            let mut tmp_out = [0u8; 3];
            let mut tmp_in = [b'A'; 4];

            while i < src_length && src[src_offset + i] != b'=' {
                tmp_in[i] = src[src_offset + i];
                i += 1;
            }

            if i < 2 {
                err = 1;
            }

            src_length = i - 1;
            err |= decode_3bytes(&tmp_in, &mut tmp_out);
            tmp_in.zeroize();

            dst[dst_offset..(dst_offset + src_length)].copy_from_slice(&tmp_out[..src_length]);
            tmp_out.zeroize();

            dst_offset += i - 1;
        }

        if err == 0 {
            Ok(dst_offset)
        } else {
            Err(EncodingInvalid)
        }
    }

    fn decoded_len(&self, bytes: &[u8]) -> Result<usize, Error> {
        if bytes.is_empty() {
            return Ok(0);
        }

        let mut i = bytes.len() - 1;
        let mut pad_count: usize = 0;

        while i > 0 && bytes[i] == b'=' {
            pad_count += 1;
            i -= 1;
        }

        Ok(((bytes.len() - pad_count) * 3) / 4)
    }
}

// Base64 character set:
// [A-Z]      [a-z]      [0-9]      +     /
// 0x41-0x5a, 0x61-0x7a, 0x30-0x39, 0x2b, 0x2f

#[inline]
fn encode_3bytes(src: &[u8], dst: &mut [u8]) {
    let b0 = src[0] as isize;
    let b1 = src[1] as isize;
    let b2 = src[2] as isize;

    dst[0] = encode_6bits(b0 >> 2);
    dst[1] = encode_6bits(((b0 << 4) | (b1 >> 4)) & 63);
    dst[2] = encode_6bits(((b1 << 2) | (b2 >> 6)) & 63);
    dst[3] = encode_6bits(b2 & 63);
}

#[inline]
fn encode_6bits(src: isize) -> u8 {
    let mut diff = 0x41isize;

    // if (in > 25) diff += 0x61 - 0x41 - 26; // 6
    diff += ((25isize - src) >> 8) & 6;

    // if (in > 51) diff += 0x30 - 0x61 - 26; // -75
    diff -= ((51isize - src) >> 8) & 75;

    // if (in > 61) diff += 0x2b - 0x30 - 10; // -15
    diff -= ((61isize - src) >> 8) & 15;

    // if (in > 62) diff += 0x2f - 0x2b - 1; // 3
    diff += ((62isize - src) >> 8) & 3;

    (src + diff) as u8
}

#[inline]
fn decode_3bytes(src: &[u8], dst: &mut [u8]) -> isize {
    let c0 = decode_6bits(src[0]);
    let c1 = decode_6bits(src[1]);
    let c2 = decode_6bits(src[2]);
    let c3 = decode_6bits(src[3]);

    dst[0] = ((c0 << 2) | (c1 >> 4)) as u8;
    dst[1] = ((c1 << 4) | (c2 >> 2)) as u8;
    dst[2] = ((c2 << 6) | c3) as u8;

    ((c0 | c1 | c2 | c3) >> 8) & 1
}

#[inline]
fn decode_6bits(src: u8) -> isize {
    let ch = src as isize;
    let mut ret: isize = -1;

    // if (ch > 0x40 && ch < 0x5b) ret += ch - 0x41 + 1; // -64
    ret += (((64isize - ch) & (ch - 91isize)) >> 8) & (ch - 64isize);

    // if (ch > 0x60 && ch < 0x7b) ret += ch - 0x61 + 26 + 1; // -70
    ret += (((96isize - ch) & (ch - 123isize)) >> 8) & (ch - 70isize);

    // if (ch > 0x2f && ch < 0x3a) ret += ch - 0x30 + 52 + 1; // 5
    ret += (((47isize - ch) & (ch - 58isize)) >> 8) & (ch + 5isize);

    // if (ch == 0x2b) ret += 62 + 1;
    ret += (((42isize - ch) & (ch - 44isize)) >> 8) & 63;

    // if (ch == 0x2f) ret += 63 + 1;
    ret + ((((46isize - ch) & (ch - 48isize)) >> 8) & 64)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Base64 test vectors
    struct Base64Vector {
        /// Raw bytes
        raw: &'static [u8],

        /// Hex encoded
        base64: &'static [u8],
    }

    const BASE64_TEST_VECTORS: &[Base64Vector] = &[
        Base64Vector {
            raw: b"",
            base64: b"",
        },
        Base64Vector {
            raw: b"\0",
            base64: b"AA==",
        },
        Base64Vector {
            raw: b"***",
            base64: b"Kioq",
        },
        Base64Vector {
            raw: b"\x01\x02\x03\x04",
            base64: b"AQIDBA==",
        },
        Base64Vector {
            raw: b"\xAD\xAD\xAD\xAD\xAD",
            base64: b"ra2tra0=",
        },
        Base64Vector {
            raw: b"\xFF\xFF\xFF\xFF\xFF",
            base64: b"//////8=",
        },
        Base64Vector {
            raw: b"\x40\xC1\x3F\xBD\x05\x4C\x72\x2A\xA3\xC2\xF2\x11\x73\xC0\x69\xEA\
                   \x49\x7D\x35\x29\x6B\xCC\x24\x65\xF6\xF9\xD0\x41\x08\x7B\xD7\xA9",
            base64: b"QME/vQVMciqjwvIRc8Bp6kl9NSlrzCRl9vnQQQh716k=",
        },
    ];

    #[test]
    fn encode_test_vectors() {
        for vector in BASE64_TEST_VECTORS {
            let out = encoder().encode(vector.raw);
            assert_eq!(encoder().encoded_len(vector.raw), out.len());
            assert_eq!(vector.base64, &out[..]);
        }
    }

    #[test]
    fn decode_test_vectors() {
        for vector in BASE64_TEST_VECTORS {
            let out = encoder().decode(vector.base64).unwrap();
            assert_eq!(encoder().decoded_len(vector.base64).unwrap(), out.len());
            assert_eq!(vector.raw, &out[..]);
        }
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

    #[test]
    fn trailing_whitespace() {
        assert_eq!(
            encoder().decode(&b"QME/vQVMciqjwvIRc8Bp6kl9NSlrzCRl9vnQQQh716k=\n"[..]),
            Err(TrailingWhitespace)
        );
    }
}
