//! HMAC-based Hierarchical Key Derivation: deterministically derive a
//! hierarchy of symmetric keys from initial keying material through
//! repeated applications of the Hash-based Message Authentication Code
//! (HMAC) construction.
//!
//! This library implements a fully symmetric construction inspired by
//! [BIP-0032: Hierarchical Deterministic Wallets][bip32].
//!
//! [bip32]: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki

#![no_std]
#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/hkd32/0.0.0")]

use hmac::{Hmac, Mac};
use sha2::Sha512;
use zeroize::Zeroize;

/// Size of input key material and derived keys
pub const KEY_SIZE: usize = 32;

/// Derive an output key from the given input key material and erivation path
pub fn derive(input_key_material: &[u8; KEY_SIZE], path: &[&[u8]]) -> [u8; KEY_SIZE] {
    path.iter()
        .enumerate()
        .fold(*input_key_material, |mut parent_key, (i, elem)| {
            let mut hmac = Hmac::<Sha512>::new_varkey(&parent_key).unwrap();
            hmac.input(elem);

            let mut hmac_result = hmac.result().code();
            parent_key.zeroize();

            let (secret_key, chain_code) = hmac_result.split_at_mut(KEY_SIZE);
            let mut child_key = [0u8; KEY_SIZE];

            if i < path.len() - 1 {
                // Use chain code for all but the last element
                child_key.copy_from_slice(chain_code);
            } else {
                // Use secret key for the last element
                child_key.copy_from_slice(secret_key);
            }

            secret_key.zeroize();
            chain_code.zeroize();
            child_key
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_VECTOR_KEY: [u8; KEY_SIZE] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];

    /// Empty path outputs the original IKM
    #[test]
    fn test_vector_0_empty_path() {
        let output_key = derive(&TEST_VECTOR_KEY, &[]);
        assert_eq!(
            output_key,
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31
            ]
        );
    }

    #[test]
    fn test_vector_1() {
        let output_key = derive(&TEST_VECTOR_KEY, &[b"1"]);
        assert_eq!(
            output_key,
            [
                132, 75, 58, 18, 91, 107, 10, 110, 128, 162, 98, 177, 192, 212, 50, 101, 136, 46,
                46, 83, 179, 150, 64, 68, 250, 57, 101, 1, 227, 159, 148, 20
            ]
        );
    }

    #[test]
    fn test_vector_2() {
        let output_key = derive(&TEST_VECTOR_KEY, &[b"1", b"2"]);
        assert_eq!(
            output_key,
            [
                110, 41, 196, 37, 188, 239, 92, 14, 14, 8, 176, 199, 3, 232, 46, 214, 237, 183, 11,
                238, 110, 19, 100, 64, 191, 71, 221, 96, 0, 165, 202, 6
            ]
        );
    }

    #[test]
    fn test_vector_3() {
        let output_key = derive(&TEST_VECTOR_KEY, &[b"1", b"2", b"3"]);
        assert_eq!(
            output_key,
            [
                17, 67, 145, 251, 66, 229, 67, 213, 30, 37, 15, 106, 223, 215, 34, 87, 221, 46,
                192, 225, 50, 153, 127, 65, 168, 152, 14, 237, 100, 231, 142, 3
            ]
        );
    }
}
