//! Identity `Encoding`: output is identical to input

use super::{
    Encoding,
    Error::{self, LengthInvalid},
};

/// `Encoding` which does not transform data and returns the original input.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Identity {}

/// Constant `Identity` encoding that can be used in lieu of calling `default()`
pub const IDENTITY: &Identity = &Identity {};

impl Encoding for Identity {
    fn encode_to_slice(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
        ensure!(self.encoded_len(src) == dst.len(), LengthInvalid);
        dst.copy_from_slice(src);
        Ok(src.len())
    }

    fn encoded_len(&self, bytes: &[u8]) -> usize {
        bytes.len()
    }

    fn decode_to_slice(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
        ensure!(self.decoded_len(src)? == dst.len(), LengthInvalid);
        dst.copy_from_slice(src);
        Ok(src.len())
    }

    fn decoded_len(&self, bytes: &[u8]) -> Result<usize, Error> {
        Ok(bytes.len())
    }
}

// TODO(tarcieri): `no_std` tests
#[cfg(feature = "alloc")]
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &[u8] = b"Testing 1, 2, 3...";

    #[test]
    fn test_encode() {
        assert_eq!(IDENTITY.encode(TEST_DATA), TEST_DATA);
    }

    #[test]
    fn test_decode() {
        assert_eq!(IDENTITY.decode(TEST_DATA).unwrap(), TEST_DATA);
    }
}
