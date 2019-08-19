//! Base32 encoding support

use crate::error::Error;
use alloc::vec::Vec;

/// Encode binary data as base32
pub fn encode(data: &[u8]) -> Vec<u8> {
    convert(data, 8, 5).unwrap()
}

/// Decode data from base32
pub fn decode(data: &[u8]) -> Result<Vec<u8>, Error> {
    convert(data, 5, 8)
}

fn convert(data: &[u8], src_base: u32, dst_base: u32) -> Result<Vec<u8>, Error> {
    let mut acc = 0u32;
    let mut bits = 0u32;
    let mut result = vec![]; // TODO: calculate size and use with_capacity
    let max = (1u32 << dst_base) - 1;

    for value in data {
        let v = u32::from(*value);
        ensure!(v >> src_base == 0, Error::EncodingInvalid);

        acc = (acc << src_base) | v;
        bits += src_base;

        while bits >= dst_base {
            bits -= dst_base;
            result.push(((acc >> bits) & max) as u8);
        }
    }

    if src_base > dst_base {
        if bits > 0 {
            result.push(((acc << (dst_base - bits)) & max) as u8);
        }
    } else if bits >= src_base || ((acc << (dst_base - bits)) & max) != 0 {
        return Err(Error::PaddingInvalid);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};
    use crate::error::Error;

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
            decode(&EXAMPLE_ENCODED[..encoded_len - 1])
        );
    }

    #[test]
    fn decode_range_error() {
        assert_eq!(
            Err(Error::EncodingInvalid),
            decode(EXAMPLE_DECODED) // decode the already decoded data
        );
    }
}
