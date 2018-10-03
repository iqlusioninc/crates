//! The `Encoding` trait: common operations across all encoders

use super::Error;
use prelude::*;

/// All encoding types in this crate implement this trait
pub trait Encoding: Send + Sync {
    /// Encode the given slice into the destination buffer.
    ///
    /// Returns the size of the encoded output, or `Error` if the destination
    /// buffer was too small to hold the encoded output.
    fn encode_to_slice(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, Error>;

    /// Calculate the length of the given data after encoding.
    fn encoded_len(&self, bytes: &[u8]) -> usize;

    /// Encode the given buffer, returning a `Vec<u8>`
    #[cfg(feature = "alloc")]
    fn encode<B: AsRef<[u8]>>(&self, bytes: B) -> Vec<u8> {
        let length = self.encoded_len(bytes.as_ref());
        let mut result = vec![0u8; length];

        debug_assert_eq!(
            self.encode_to_slice(bytes.as_ref(), &mut result),
            Ok(length)
        );

        result
    }

    /// Decode hexadecimal (upper or lower case) with branchless / secret-independent logic
    fn decode_to_slice(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, Error>;

    /// Calculate the length of the given data after decoding.
    fn decoded_len(&self, encoded_bytes: &[u8]) -> Result<usize, Error>;

    /// Decode the given buffer, returning a `Vec<u8>`
    #[cfg(feature = "alloc")]
    fn decode<B: AsRef<[u8]>>(&self, encoded_bytes: B) -> Result<Vec<u8>, Error> {
        let length = self.decoded_len(encoded_bytes.as_ref())?;
        let mut result = vec![0u8; length];

        debug_assert_eq!(
            self.decode_to_slice(encoded_bytes.as_ref(), &mut result),
            Ok(length)
        );

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test input for encoding/decoding cases
    const TEST_DATA: &[u8] = b"Testing 1, 2, 3...";

    /// Dummy encoding we use to test `Encoding` methods
    struct TestEncoding {}

    impl Default for TestEncoding {
        fn default() -> Self {
            Self {}
        }
    }

    impl Encoding for TestEncoding {
        fn encode_to_slice(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
            let length = self.encoded_len(src);
            assert_eq!(dst.len(), length);
            Ok(length)
        }

        fn encoded_len(&self, bytes: &[u8]) -> usize {
            bytes.len() * 4 / 3
        }

        fn decode_to_slice(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
            let length = self.decoded_len(src)?;
            assert_eq!(dst.len(), length);
            Ok(length)
        }

        fn decoded_len(&self, bytes: &[u8]) -> Result<usize, Error> {
            Ok(bytes.len() * 3 / 4)
        }
    }

    /// Make sure `encode()` doesn't panic
    #[test]
    fn test_encode() {
        TestEncoding::default().encode(TEST_DATA);
    }

    /// Make sure `decode()` doesn't panic
    #[test]
    fn test_decode() {
        let encoding = TestEncoding::default();
        let encoded = encoding.encode(TEST_DATA);
        encoding.decode(&encoded).unwrap();
    }
}
