//! Extensible schema-driven builder, signer, and Amino serializer for
//! Cosmos SDK-formatted `StdTx` transactions, the standard transaction format
//! used by the Cosmos SDK and other Tendermint blockchains which use types
//! from the Cosmos SDK.
//!
//! Uses a TOML-based schema description language for `sdk.Msg` values which
//! should be encoded into the final `StdTx`.
//!
//! Includes a `StdTx` builder capable of constructing `sdk.Msg` values and
//! signing them using any ECDSA secp256k1 signer compatible with the
//! [`ecdsa` crate] (e.g. [`signatory-secp256k1`], [`yubihsm`]).
//!
//! # Equivalent Go code
//!
//! - [`StdTx` (godoc)](https://godoc.org/github.com/cosmos/cosmos-sdk/x/auth/types#StdTx)
//! - [`sdk.Msg` (godoc)](httpshttps://docs.rs/ecdsa://godoc.org/github.com/cosmos/cosmos-sdk/types#Msg)
//!
//! # Usage
//!
//! Below is a self-contained example of how to use [`stdtx::Builder`]
//! type to construct a signed [`StdTx`] message:
//!
//! ```
//! # #[cfg(feature = "amino")]
//! # {
//! use stdtx::{amino::Builder, error::Result};
//! use k256::ecdsa::SigningKey;
//! use rand_core::OsRng; // requires `std` feature of `rand_core`
//!
//! /// Example account number
//! const ACCOUNT_NUMBER: u64 = 946827;
//!
//! /// Example chain ID
//! const CHAIN_ID: &str = "columbus-3";
//!
//! /// Example oracle feeder for `oracle/MsgExchangeRateVote`
//! const FEEDER: &str = "terra1t9et8wjeh8d0ewf4lldchterxsmhpcgg5auy47";
//!
//! /// Example oracle validator for `oracle/MsgExchangeRateVote`
//! const VALIDATOR: &str = "terravaloper1grgelyng2v6v3t8z87wu3sxgt9m5s03x2mfyu7";
//!
//! /// Example amount of gas to include in transaction
//! const GAS_AMOUNT: u64 = 200000;
//!
//! /// Example StdTx message schema definition. See docs for the
//! /// `stdtx::Schema` type for more information:
//! /// <https://docs.rs/cosmos-stdtx/latest/stdtx/schema/index.html>
//! ///
//! /// Message types taken from Terra's oracle voter transactions:
//! /// <https://docs.terra.money/docs/dev-spec-oracle#message-types>
//! pub const TERRA_SCHEMA: &str = r#"
//!     namespace = "core/StdTx"
//!     acc_prefix = "terra"
//!     val_prefix = "terravaloper"
//!
//!     [[definition]]
//!     type_name = "oracle/MsgExchangeRatePrevote"
//!     fields = [
//!         { name = "hash",  type = "string" },
//!         { name = "denom", type = "string" },
//!         { name = "feeder", type = "sdk.AccAddress" },
//!         { name = "validator", type = "sdk.ValAddress" },
//!     ]
//!
//!     [[definition]]
//!     type_name = "oracle/MsgExchangeRateVote"
//!     fields = [
//!         { name = "exchange_rate", type = "sdk.Dec"},
//!         { name = "salt", type = "string" },
//!         { name = "denom", type = "string" },
//!         { name = "feeder", type = "sdk.AccAddress" },
//!         { name = "validator", type = "sdk.ValAddress" },
//!     ]
//!     "#;
//!
//! /// Simple builder for an `oracle/MsgExchangeRateVote` message
//! fn build_vote_msg(schema: &stdtx::amino::Schema) -> Result<stdtx::amino::Msg> {
//!     Ok(stdtx::amino::msg::Builder::new(schema, "oracle/MsgExchangeRateVote")?
//!         .decimal("exchange_rate", -1i8)?
//!         .string("salt", "XXXX")?
//!         .string("denom", "ukrw")?
//!         .acc_address_bech32("feeder", FEEDER)?
//!         .val_address_bech32("validator", VALIDATOR)?
//!         .to_msg())
//! }
//!
//! /// Parse the TOML schema for Terra `sdk.Msg` types
//! let schema = TERRA_SCHEMA.parse::<stdtx::amino::Schema>().unwrap();
//!
//! /// Create message builder, giving it an account number, chain ID, and a
//! /// boxed ECDSA secp256k1 signer
//! let builder = stdtx::amino::Builder::new(schema, CHAIN_ID, ACCOUNT_NUMBER);
//!
//! /// Create ECDSA signing key (ordinarily you wouldn't generate a random key
//! /// every time but reuse an existing one)
//! let signer = SigningKey::random(&mut OsRng);
//!
//! /// Create message to be included in the `StdTx` using the method defined above
//! let msg = build_vote_msg(builder.schema()).unwrap();
//!
//! /// Build transaction, returning serialized Amino bytes as a `Vec<u8>`
//! let sequence_number = 123456;
//! let fee = stdtx::amino::StdFee::for_gas(GAS_AMOUNT);
//! let memo = "";
//! let amino_bytes = builder
//!     .sign_amino_tx(&signer, sequence_number, fee, memo, &[msg])
//!     .unwrap();
//!
//! // `amino_bytes` is now a `Vec<u8>` containing an Amino serialized transaction
//! # }
//! ```
//!
//! [`ecdsa` crate]: https://docs.rs/ecdsa
//! [`signatory-secp256k1`]: https://docs.rs/signatory-secp256k1
//! [`yubihsm`]: https://docs.rs/yubihsm
//! [`stdtx::Builder`]: https://docs.rs/stdtx/latest/stdtx/stdtx/struct.Builder.html

#![doc(html_root_url = "https://docs.rs/stdtx/0.4.0")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

pub mod address;
pub mod amino;
pub mod decimal;
pub mod error;

pub use self::{address::Address, decimal::Decimal, error::Error};
pub use k256::ecdsa::{Signature, VerifyingKey};

/// Transaction signer for ECDSA/secp256k1 signatures
pub type Signer = dyn ecdsa::signature::Signer<Signature>;
