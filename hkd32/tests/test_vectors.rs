//! Temporary test vectors (known to match a previous implementation)
// TODO(tarcieri): switch to BIP32 test vectors

use hkd32::*;

fn parse_path(path: &str) -> PathBuf {
    path.parse()
        .unwrap_or_else(|_| panic!("couldn't parse path: {:?}", path))
}

fn test_key() -> KeyMaterial {
    KeyMaterial::new([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ])
}

#[cfg(feature = "mnemonic")]
fn test_mnemonic() -> mnemonic::Phrase {
    // This phrase is the BIP39 equipvalent of `test_key()` above
    let bip39_phrase: &str =
        "abandon amount liar amount expire adjust cage candy arch gather drum bullet \
         absurd math era live bid rhythm alien crouch range attend journey unaware";

    mnemonic::Phrase::new(bip39_phrase, Default::default()).unwrap()
}

/// Root path outputs the original IKM
#[test]
fn test_vector_0_empty_path() {
    let output_key = test_key().derive_subkey(parse_path("/"));

    assert_eq!(
        output_key.as_bytes(),
        [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31
        ]
    );
}

#[test]
fn test_vector_1() {
    let output_key = test_key().derive_subkey(parse_path("/1"));

    assert_eq!(
        output_key.as_bytes(),
        [
            132, 75, 58, 18, 91, 107, 10, 110, 128, 162, 98, 177, 192, 212, 50, 101, 136, 46, 46,
            83, 179, 150, 64, 68, 250, 57, 101, 1, 227, 159, 148, 20
        ]
    );
}

#[test]
fn test_vector_2() {
    let output_key = test_key().derive_subkey(parse_path("/1/2"));

    assert_eq!(
        output_key.as_bytes(),
        [
            110, 41, 196, 37, 188, 239, 92, 14, 14, 8, 176, 199, 3, 232, 46, 214, 237, 183, 11,
            238, 110, 19, 100, 64, 191, 71, 221, 96, 0, 165, 202, 6
        ]
    );
}

#[test]
fn test_vector_3() {
    let output_key = test_key().derive_subkey(parse_path("/1/2/3"));

    assert_eq!(
        output_key.as_bytes(),
        [
            17, 67, 145, 251, 66, 229, 67, 213, 30, 37, 15, 106, 223, 215, 34, 87, 221, 46, 192,
            225, 50, 153, 127, 65, 168, 152, 14, 237, 100, 231, 142, 3
        ]
    );
}

#[cfg(feature = "mnemonic")]
#[test]
fn test_mnemonic_derivation() {
    let mnemonic = test_mnemonic();
    assert_eq!(test_key().as_bytes(), mnemonic.entropy());

    let output_key = mnemonic.derive_subkey(parse_path("/1/2/3"));

    assert_eq!(
        output_key.as_bytes(),
        [
            17, 67, 145, 251, 66, 229, 67, 213, 30, 37, 15, 106, 223, 215, 34, 87, 221, 46, 192,
            225, 50, 153, 127, 65, 168, 152, 14, 237, 100, 231, 142, 3
        ]
    );
}
