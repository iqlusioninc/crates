//! These are internal helper functions used when creating a new [`Mnemonic`][Mnemonic], and when turning a [`Mnemonic`][Mnemonic]
//! into a [`Seed`][Seed].
//!
//! [Mnemonic]: ../mnemonic/struct.Mnemonic.html
//! [Seed]: ../seed/struct.Seed.html
//!

extern crate rand;
use self::rand::{thread_rng, RngCore};
use hmac::Hmac;
use sha2::Digest;

const PBKDF2_ROUNDS: usize = 2048;
const PBKDF2_BYTES: usize = 64;

/// SHA256 helper function, internal to the crate
///
pub(crate) fn sha256_first_byte(input: &[u8]) -> u8 {
    sha2::Sha256::digest(input).as_ref()[0]
}

/// Random byte generator, used to create new mnemonics
///
pub(crate) fn gen_random_bytes(byte_length: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    let mut bytes = vec![0u8; byte_length];

    rng.fill_bytes(&mut bytes);

    bytes
}
/// PBKDF2 helper, used to generate [`Seed`][Seed] from [`Mnemonic`][Mnemonic]
///
/// [Mnemonic]: ../mnemonic/struct.Mnemonic.html
/// [Seed]: ../seed/struct.Seed.html
///
pub(crate) fn pbkdf2(input: &[u8], salt: &str) -> Vec<u8> {
    let mut seed = vec![0u8; PBKDF2_BYTES];

    pbkdf2::pbkdf2::<Hmac<sha2::Sha512>>(input, salt.as_bytes(), PBKDF2_ROUNDS, &mut seed);

    seed
}
