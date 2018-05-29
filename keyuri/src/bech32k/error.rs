/// Error types for bech32k encoding / decoding
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum Error {
    /// KeyURI is missing the ';' character
    #[fail(display = "missing separator character: \";\"")]
    SeparatorMissing,

    /// Checksum for the bech32k string does not match expected value
    #[fail(display = "checksum mismatch")]
    ChecksumInvalid,

    /// String is too short or long
    #[fail(display = "invalid length (min 8, max 90)")]
    LengthInvalid,

    /// Character is not valid
    #[fail(display = "character invalid ({})'", byte)]
    CharInvalid {
        /// Invalid byte
        byte: u8,
    },

    /// Data is not valid
    #[fail(display = "data invalid ({})", byte)]
    DataInvalid {
        /// Invalid byte
        byte: u8,
    },

    /// Padding missing/invalid
    #[fail(display = "padding invalid")]
    PaddingInvalid,

    /// Mixed-case string
    #[fail(display = "string contains mixed-case")]
    CaseInvalid,
}
