//! **anomaly.rs**: Error context library with support for type-erased sources
//! and backtraces

#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]
#![doc(html_root_url = "https://docs.rs/anomaly/0.1.1")]

#[macro_use]
mod macros;

mod context;
mod message;

pub use self::{context::Context, message::Message};
#[cfg(feature = "backtrace")]
pub use backtrace;

/// Box containing a thread-safe + `'static` error suitable for use as a
/// as an `std::error::Error::source`.
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
