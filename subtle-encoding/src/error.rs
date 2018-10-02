/// Error type
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum Error {
    /// Data is not encoded correctly
    #[fail(display = "bad encoding")]
    EncodingInvalid,

    /// Input or output buffer is an incorrect length
    #[fail(display = "invalid length")]
    LengthInvalid,
}

/// Assert that the provided condition is true, or else return the given error
macro_rules! ensure {
    ($condition:expr, $err:ident) => {
        if !($condition) {
            Err($err)?;
        }
    };
}
