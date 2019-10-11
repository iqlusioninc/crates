//! HTTP response handling

#[cfg(feature = "alloc")]
mod body;
#[cfg(feature = "std")]
mod reader;

#[cfg(feature = "alloc")]
pub use self::body::Body;
#[cfg(feature = "std")]
pub use self::reader::Reader;
