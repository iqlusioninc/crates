//! Data independent (i.e. constant time-ish) Base32 encoding and decoding

//  Copyright (c) 2016 - 2018 Paragon Initiative Enterprises.
//  Copyright (c) 2014 Steve "Sc00bz" Thomas (steve at tobtu dot com)
//
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
//
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
//
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.

use super::Error;

macro_rules! add {
    ($a:expr, $b:expr) => {
        $a.checked_add($b).expect("overflow")
    };
}

macro_rules! sub {
    ($a:expr, $b:expr) => {
        $a.checked_sub($b).expect("underflow")
    };
}

macro_rules! shr {
    ($a:expr, $b:expr) => {
        $a.checked_shr($b).expect("overflow")
    };
}

macro_rules! shl {
    ($a:expr, $b:expr) => {
        $a.checked_shl($b).expect("overflow")
    };
}

/// Encode into Base32
pub(crate) fn encode(src: &[u8]) -> Vec<u8> {
    // TODO: estimate capacity
    let mut dest = vec![];
    let src_len = src.len();

    // Main loop (no padding):
    let mut i = 0usize;

    while add!(i, 5) <= src_len {
        let chunk = &src[i..add!(i, 5)];
        let b0 = chunk[0];
        let b1 = chunk[1];
        let b2 = chunk[2];
        let b3 = chunk[3];
        let b4 = chunk[4];

        dest.push(encode5_bits(shr!(b0, 3) & 31));
        dest.push(encode5_bits((shl!(b0, 2) | shr!(b1, 6)) & 31));
        dest.push(encode5_bits(shr!(b1, 1) & 31));
        dest.push(encode5_bits((shl!(b1, 4) | shr!(b2, 4)) & 31));
        dest.push(encode5_bits((shl!(b2, 1) | shr!(b3, 7)) & 31));
        dest.push(encode5_bits(shr!(b3, 2) & 31));
        dest.push(encode5_bits((shl!(b3, 3) | shr!(b4, 5)) & 31));
        dest.push(encode5_bits(b4 & 31));

        i = add!(i, 5);
    }

    // The last chunk, which may have padding:
    if i < src_len {
        let chunk = &src[i..];
        let b0 = chunk[0];

        if add!(i, 3) < src_len {
            let b1 = chunk[1];
            let b2 = chunk[2];
            let b3 = chunk[3];

            dest.push(encode5_bits(shr!(b0, 3) & 31));
            dest.push(encode5_bits((shl!(b0, 2) | shr!(b1, 6)) & 31));
            dest.push(encode5_bits(shr!(b1, 1) & 31));
            dest.push(encode5_bits((shl!(b1, 4) | shr!(b2, 4)) & 31));
            dest.push(encode5_bits((shl!(b2, 1) | shr!(b3, 7)) & 31));
            dest.push(encode5_bits(shr!(b3, 2) & 31));
            dest.push(encode5_bits(shl!(b3, 3) & 31));
        } else if add!(i, 2) < src_len {
            let b1 = chunk[1];
            let b2 = chunk[2];

            dest.push(encode5_bits(shr!(b0, 3) & 31));
            dest.push(encode5_bits((shl!(b0, 2) | shr!(b1, 6)) & 31));
            dest.push(encode5_bits(shr!(b1, 1) & 31));
            dest.push(encode5_bits((shl!(b1, 4) | shr!(b2, 4)) & 31));
            dest.push(encode5_bits(shl!(b2, 1) & 31));
        } else if add!(i, 1) < src_len {
            let b1 = chunk[1];

            dest.push(encode5_bits(shr!(b0, 3) & 31));
            dest.push(encode5_bits((shl!(b0, 2) | shr!(b1, 6)) & 31));
            dest.push(encode5_bits(shr!(b1, 1) & 31));
            dest.push(encode5_bits(shl!(b1, 4) & 31));
        } else {
            dest.push(encode5_bits(shr!(b0, 3) & 31));
            dest.push(encode5_bits(shl!(b0, 2) & 31));
        }
    }

    dest
}

/// Decode a Base32-encoded string into raw binary
pub(crate) fn decode(src: &[u8]) -> Result<Vec<u8>, Error> {
    // Remove padding
    let mut src_len = src.len();

    if src_len == 0 {
        return Ok(vec![]);
    }

    if src_len & 7 == 0 {
        for _ in 0..7 {
            if src[sub!(src_len, 1)] == b'=' {
                src_len = sub!(src_len, 1);
            } else {
                break;
            }
        }
    }

    if (src_len & 7) == 1 {
        return Err(Error::PaddingInvalid);
    }

    // TODO: estimate capacity
    let mut dest = vec![];
    let mut err = 0u8;

    // Main loop (no padding):
    let mut i = 0usize;

    while add!(i, 8) <= src_len {
        let chunk = &src[i..add!(i, 8)];
        let c0 = decode5_bits(chunk[0]);
        let c1 = decode5_bits(chunk[1]);
        let c2 = decode5_bits(chunk[2]);
        let c3 = decode5_bits(chunk[3]);
        let c4 = decode5_bits(chunk[4]);
        let c5 = decode5_bits(chunk[5]);
        let c6 = decode5_bits(chunk[6]);
        let c7 = decode5_bits(chunk[7]);

        dest.push((shl!(c0, 3) | shr!(c1, 2)) & 0xff);
        dest.push((shl!(c1, 6) | shl!(c2, 1) | shr!(c3, 4)) & 0xff);
        dest.push((shl!(c3, 4) | shr!(c4, 1)) & 0xff);
        dest.push((shl!(c4, 7) | shl!(c5, 2) | shr!(c6, 3)) & 0xff);
        dest.push((shl!(c6, 5) | (c7)) & 0xff);

        err |= shr!((c0 | c1 | c2 | c3 | c4 | c5 | c6 | c7), 8);
        i = add!(i, 8);
    }

    // The last chunk, which may have padding:
    if i < src_len {
        let chunk = &src[i..];
        let c0 = decode5_bits(chunk[0]);

        if add!(i, 6) < src_len {
            let c1 = decode5_bits(chunk[1]);
            let c2 = decode5_bits(chunk[2]);
            let c3 = decode5_bits(chunk[3]);
            let c4 = decode5_bits(chunk[4]);
            let c5 = decode5_bits(chunk[5]);
            let c6 = decode5_bits(chunk[6]);

            dest.push((shl!(c0, 3) | shr!(c1, 2)) & 0xff);
            dest.push((shl!(c1, 6) | shl!(c2, 1) | shr!(c3, 4)) & 0xff);
            dest.push((shl!(c3, 4) | shr!(c4, 1)) & 0xff);
            dest.push((shl!(c4, 7) | shl!(c5, 2) | shr!(c6, 3)) & 0xff);

            err |= shr!(c0 | c1 | c2 | c3 | c4 | c5 | c6, 8);
        } else if add!(i, 5) < src_len {
            let c1 = decode5_bits(chunk[1]);
            let c2 = decode5_bits(chunk[2]);
            let c3 = decode5_bits(chunk[3]);
            let c4 = decode5_bits(chunk[4]);
            let c5 = decode5_bits(chunk[5]);

            dest.push((shl!(c0, 3) | shr!(c1, 2)) & 0xff);
            dest.push((shl!(c1, 6) | shl!(c2, 1) | shr!(c3, 4)) & 0xff);
            dest.push((shl!(c3, 4) | shr!(c4, 1)) & 0xff);
            dest.push((shl!(c4, 7) | shl!(c5, 2)) & 0xff);

            err |= shr!(c0 | c1 | c2 | c3 | c4 | c5, 8);
        } else if add!(i, 4) < src_len {
            let c1 = decode5_bits(chunk[1]);
            let c2 = decode5_bits(chunk[2]);
            let c3 = decode5_bits(chunk[3]);
            let c4 = decode5_bits(chunk[4]);

            dest.push((shl!(c0, 3) | shr!(c1, 2)) & 0xff);
            dest.push((shl!(c1, 6) | shl!(c2, 1) | shr!(c3, 4)) & 0xff);
            dest.push((shl!(c3, 4) | shr!(c4, 1)) & 0xff);

            err |= shr!(c0 | c1 | c2 | c3 | c4, 8);
        } else if add!(i, 3) < src_len {
            let c1 = decode5_bits(chunk[1]);
            let c2 = decode5_bits(chunk[2]);
            let c3 = decode5_bits(chunk[3]);

            dest.push((shl!(c0, 3) | shr!(c1, 2)) & 0xff);
            dest.push((shl!(c1, 6) | shl!(c2, 1) | shr!(c3, 4)) & 0xff);

            err |= shr!(c0 | c1 | c2 | c3, 8);
        } else if add!(i, 2) < src_len {
            let c1 = decode5_bits(chunk[1]);
            let c2 = decode5_bits(chunk[2]);

            dest.push((shl!(c0, 3) | shr!(c1, 2)) & 0xff);
            dest.push((shl!(c1, 6) | shl!(c2, 1)) & 0xff);

            err |= shr!(c0 | c1 | c2, 8);
        } else if add!(i, 1) < src_len {
            let c1 = decode5_bits(chunk[1]);
            dest.push((shl!(c0, 3) | shr!(c1, 2)) & 0xff);
            err |= shr!(c0 | c1, 8);
        } else {
            dest.push(shl!(c0, 3) & 0xff);
            err |= shr!(c0, 8);
        }
    }

    if err == 0 {
        Ok(dest)
    } else {
        // TODO: detect out-of-range byte
        Err(Error::EncodingInvalid)
    }
}

/// Uses bitwise operators instead of table-lookups to turn 8-bit integers
/// into 5-bit integers.
fn encode5_bits(src: u8) -> u8 {
    let mut diff: isize = 0x61;

    // if (src > 25) ret -= 72;
    diff = sub!(diff, shr!(sub!(25isize, src as isize), 8) & 73);

    add!(src as isize, diff) as u8
}

/// Uses bitwise operators instead of table-lookups to turn 5-bit integers
/// into 8-bit integers.
fn decode5_bits(src: u8) -> u8 {
    let mut ret: isize = -1;

    // if (src > 96 && src < 123) ret += src - 97 + 1; // -64
    ret = add!(
        ret,
        shr!(
            sub!(0x60isize, src as isize) & sub!(src as isize, 0x7bisize),
            8
        ) & sub!(src as isize, 96isize)
    );

    // if (src > 0x31 && src < 0x38) ret += src - 24 + 1; // -23
    add!(
        ret,
        shr!(
            sub!(0x31isize, src as isize) & sub!(src as isize, 0x38isize),
            8
        ) & sub!(src as isize, 23isize)
    ) as u8
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};
    use error::Error;

    const EXAMPLE_ENCODED: &[u8] = &[0, 4, 1, 0, 6, 1, 8, 9, 2, 4, 16, 20, 3, 0, 8];
    const EXAMPLE_DECODED: &[u8] = &[1, 2, 3, 5, 9, 17, 33, 65, 129];

    #[test]
    fn encode_base32() {
        assert_eq!(EXAMPLE_ENCODED, encode(EXAMPLE_DECODED).as_slice());
    }

    #[test]
    fn decode_valid_base32() {
        assert_eq!(EXAMPLE_DECODED, decode(EXAMPLE_ENCODED).unwrap().as_slice());
    }

    #[test]
    fn decode_padding_error() {
        let encoded_len = EXAMPLE_ENCODED.len();
        assert_eq!(
            Err(Error::PaddingInvalid),
            decode(&EXAMPLE_ENCODED[..sub!(encoded_len, 1)])
        );
    }

    #[test]
    fn decode_range_error() {
        assert_eq!(
            Err(Error::EncodingInvalid),
            // Decode the decoded example, which is not valid Bech32
            decode(EXAMPLE_DECODED)
        );
    }
}
