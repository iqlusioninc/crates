extern crate bip39;

use bip39::{Language, Mnemonic, MnemonicType, Seed};

fn test_word_count(expected_word_count: usize) {
    let mnemonic_type = MnemonicType::for_word_count(expected_word_count).unwrap();

    let mnemonic = Mnemonic::new(mnemonic_type, Language::English);
    let actual_word_count = mnemonic.phrase().split(" ").count();

    assert_eq!(actual_word_count, expected_word_count);
    assert_eq!(mnemonic_type.word_count(), expected_word_count);

    let seed = Seed::new(&mnemonic, "");
    let seed_bytes: &[u8] = seed.as_bytes();

    assert!(seed_bytes.len() == 64);
}

#[test]
fn generate_12_english() {
    test_word_count(12);
}

#[test]
fn generate_15_english() {
    test_word_count(15);
}

#[test]
fn generate_18_english() {
    test_word_count(18);
}

#[test]
fn generate_21_english() {
    test_word_count(21);
}

#[test]
fn generate_24_english() {
    test_word_count(24);
}

#[test]
fn generate_from_invalid_entropy() {
    // 15 bytes
    let entropy = &[
        0x33, 0xE4, 0x6B, 0xB1, 0x3A, 0x74, 0x6E, 0xA4, 0x1C, 0xDD, 0xE4, 0x5C, 0x90, 0x84, 0x6A,
    ];

    assert!(Mnemonic::from_entropy(entropy, Language::English).is_err());
}
