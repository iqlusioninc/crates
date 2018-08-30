/// Error types for bech32 encoding / decoding
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum Error {
    /// Checksum for the bech32 string does not match expected value
    #[fail(display = "checksum mismatch")]
    ChecksumInvalid,

    /// Data is not valid
    #[fail(display = "bad encoding")]
    EncodingInvalid,

    /// String is too short or long
    #[fail(display = "invalid length (min 8, max 90)")]
    LengthInvalid,

    /// Padding missing/invalid
    #[fail(display = "padding invalid")]
    PaddingInvalid,

    /// Human readable part is invalid
    #[fail(display = "prefix (a.k.a. human-readable part) is empty")]
    PrefixInvalid,

    /// Missing the sub-delimiter character
    #[fail(display = "missing sub-delimiter")]
    SeparatorMissing,
}
