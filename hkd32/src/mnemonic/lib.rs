//!
//! This is a Rust implementation of the [bip39][bip39-standard] standard for Bitcoin HD wallet
//! mnemonic phrases.
//!
//!
//! [bip39-standard]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
//!
//! ## Quickstart
//!
//! ```rust
//! use bip39::{Mnemonic, MnemonicType, Language, Seed};
//!
//! /// create a new randomly generated mnemonic phrase
//! let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
//!
//! /// get the phrase
//! let phrase: &str = mnemonic.phrase();
//! println!("phrase: {}", phrase);
//!
//! /// get the HD wallet seed
//! let seed = Seed::new(&mnemonic, "");
//!
//! // get the HD wallet seed as raw bytes
//! let seed_bytes: &[u8] = seed.as_bytes();
//!
//! // print the HD wallet seed as a hex string
//! println!("{:X}", seed);
//! ```
//!
#[macro_use] extern crate failure;
#[macro_use] extern crate once_cell;
extern crate pbkdf2;
extern crate hashbrown;
extern crate sha2;
extern crate hmac;

mod mnemonic;
mod error;
mod mnemonic_type;
mod language;
mod util;
mod seed;

mod crypto;

pub use language::Language;
pub use mnemonic::Mnemonic;
pub use mnemonic_type::MnemonicType;
pub use seed::Seed;
pub use error::ErrorKind;
