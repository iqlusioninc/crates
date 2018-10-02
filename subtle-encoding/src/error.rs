/// Error type
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum Error {
    /// Data is not encoded correctly
    #[fail(display = "bad encoding")]
    EncodingInvalid,
}
