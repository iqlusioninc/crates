/// Error types for bech32 encoding / decoding
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum Error {
    /// Mixed-case string
    #[fail(display = "string contains mixed-case")]
    CaseInvalid,

    /// Checksum for the bech32 string does not match expected value
    #[fail(display = "checksum mismatch")]
    ChecksumInvalid,

    /// Character is not valid
    #[fail(display = "character invalid ({})'", char)]
    CharInvalid {
        /// Invalid character
        char: char,
    },

    /// Data is not valid
    #[fail(display = "data invalid ({})", byte)]
    DataInvalid {
        /// Invalid byte
        byte: u8,
    },

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
