//! The `Encoding` trait: common operations across all encoders

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};
#[cfg(feature = "std")]
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};
#[cfg(all(unix, feature = "std"))]
use std::{fs::OpenOptions, os::unix::fs::OpenOptionsExt};
#[cfg(feature = "std")]
use zeroize::Zeroize;

use super::Error;

/// Mode to use for newly created files
// TODO: make this configurable?
#[cfg(unix)]
pub const FILE_MODE: u32 = 0o600;

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
        let expected_length = self.encoded_len(bytes.as_ref());
        let mut encoded = vec![0u8; expected_length];

        let actual_length = self.encode_to_slice(bytes.as_ref(), &mut encoded).unwrap();
        debug_assert_eq!(expected_length, actual_length);

        encoded
    }

    /// Encode the given slice to a `String` with this `Encoding`.
    ///
    /// Returns an `Error` in the event this encoding does not produce a
    /// well-formed UTF-8 string.
    #[cfg(feature = "alloc")]
    fn encode_to_string<B: AsRef<[u8]>>(&self, bytes: B) -> Result<String, Error> {
        Ok(String::from_utf8(self.encode(bytes))?)
    }

    /// Encode the given slice with this `Encoding`, writing the result to the
    /// supplied `io::Write` type, returning the number of bytes written or a `Error`.
    #[cfg(feature = "std")]
    fn encode_to_writer<B, W>(&self, bytes: B, writer: &mut W) -> Result<usize, Error>
    where
        B: AsRef<[u8]>,
        W: Write,
    {
        let mut encoded_bytes = self.encode(bytes);
        writer.write_all(encoded_bytes.as_ref())?;
        encoded_bytes.zeroize();
        Ok(encoded_bytes.len())
    }

    /// Encode `self` and write it to a file at the given path, returning the
    /// resulting `File` or a `Error`.
    ///
    /// If the file does not exist, it will be created with a mode of
    /// `FILE_MODE` (i.e. `600`). If the file does exist, it will be erased
    /// and replaced.
    #[cfg(all(unix, feature = "std"))]
    fn encode_to_file<B, P>(&self, bytes: B, path: P) -> Result<File, Error>
    where
        B: AsRef<[u8]>,
        P: AsRef<Path>,
    {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(FILE_MODE)
            .open(path)?;

        self.encode_to_writer(bytes, &mut file)?;
        Ok(file)
    }

    /// Encode `self` and write it to a file at the given path, returning the
    /// resulting `File` or a `Error`.
    ///
    /// If the file does not exist, it will be created.
    #[cfg(all(not(unix), feature = "std"))]
    fn encode_to_file<B, P>(&self, bytes: B, path: P) -> Result<File, Error>
    where
        B: AsRef<[u8]>,
        P: AsRef<Path>,
    {
        let mut file = File::create(path.as_ref())?;
        self.encode_to_writer(bytes, &mut file)?;
        Ok(file)
    }

    /// Decode hexadecimal (upper or lower case) with branchless / secret-independent logic
    fn decode_to_slice(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, Error>;

    /// Calculate the length of the given data after decoding.
    fn decoded_len(&self, encoded_bytes: &[u8]) -> Result<usize, Error>;

    /// Decode the given buffer, returning a `Vec<u8>`
    #[cfg(feature = "alloc")]
    fn decode<B: AsRef<[u8]>>(&self, encoded_bytes: B) -> Result<Vec<u8>, Error> {
        let expected_length = self.decoded_len(encoded_bytes.as_ref())?;
        let mut decoded = vec![0u8; expected_length];

        let actual_length = self.decode_to_slice(encoded_bytes.as_ref(), &mut decoded)?;
        debug_assert_eq!(expected_length, actual_length);

        Ok(decoded)
    }

    /// Decode the given string-alike type with this `Encoding`, returning the
    /// decoded value or a `Error`.
    #[cfg(feature = "std")]
    fn decode_from_str<S: AsRef<str>>(&self, encoded: S) -> Result<Vec<u8>, Error> {
        self.decode(encoded.as_ref().as_bytes())
    }

    /// Decode the data read from the given `io::Read` type with this
    /// `Encoding`, returning the decoded value or a `Error`.
    #[cfg(feature = "std")]
    fn decode_from_reader<R: Read>(&self, reader: &mut R) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![];
        reader.read_to_end(bytes.as_mut())?;
        let result = self.decode(&bytes);
        bytes.zeroize();
        result
    }

    /// Read a file at the given path, decoding the data it contains using
    /// the provided `Encoding`, returning the decoded value or a `Error`.
    #[cfg(feature = "std")]
    fn decode_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, Error> {
        self.decode_from_reader(&mut File::open(path.as_ref())?)
    }
}

// TODO(tarcieri): `no_std` tests
#[cfg(feature = "alloc")]
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
