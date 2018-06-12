//! Test against the BIP-173 test vectors

extern crate iq_bech32;

use iq_bech32::{Bech32, Error};

/// Bech32 test vector
struct TestVector {
    /// Bech32-encoded string
    encoded: &'static str,

    /// Human readable part
    hrp: &'static str,

    /// Binary data
    bytes: &'static [u8],
}

// BIP-173 test vectors
// https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki#Test_vectors
const VALID_TEST_VECTORS: &[TestVector] = &[
    TestVector {
        encoded: "A12UEL5L",
        hrp: "a",
        bytes: &[]
    },
    TestVector {
        encoded: "a12uel5l",
        hrp: "a",
        bytes: &[]
    },
    TestVector {
        encoded: "an83characterlonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio1tt5tgs",
        hrp: "an83characterlonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio",
        bytes: &[]
    },
    TestVector {
        hrp: "abcdef",
        bytes: &[0, 68, 50, 20, 199, 66, 84, 182, 53, 207, 132, 101, 58, 86, 215, 198, 117, 190, 119, 223],
        encoded: "abcdef1qpzry9x8gf2tvdw0s3jn54khce6mua7lmqqqxw",
    },
    TestVector {
        encoded: "11qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqc8247j",
        hrp: "1",
        bytes: &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    },
    TestVector {
        encoded: "split1checkupstagehandshakeupstreamerranterredcaperred2y9e3w",
        hrp: "split",
        bytes: &[197, 243, 139, 112, 48, 95, 81, 155, 246, 109, 133, 251, 108, 240, 48, 88, 243, 221, 228, 99, 236, 215, 145, 143, 45, 199, 67, 145, 143, 45]
    },
    TestVector {
        encoded: "?1ezyfcl",
        hrp: "?",
        bytes: &[]
    },
];

#[test]
fn encode_valid_test_vectors() {
    let bech32 = Bech32::default();
    for vector in VALID_TEST_VECTORS {
        let encoded = bech32.encode(vector.hrp, vector.bytes);
        assert_eq!(encoded, vector.encoded.to_lowercase());
    }
}

#[test]
fn decode_valid_test_vectors() {
    let bech32 = Bech32::default();
    for vector in VALID_TEST_VECTORS {
        let (hrp, data) = bech32.decode(vector.encoded).unwrap();
        assert_eq!(hrp, vector.hrp.to_lowercase());
        assert_eq!(data, vector.bytes);
    }
}

#[test]
fn hrp_character_out_of_range() {
    let bech32 = Bech32::default();
    assert_eq!(
        bech32.decode("\x201nwldj5"),
        Err(Error::CharInvalid { char: '\x20' })
    );
    assert_eq!(
        bech32.decode("\x7F1axkwrx"),
        Err(Error::CharInvalid { char: '\x7F' })
    );
}

#[test]
fn overall_max_length_exceeded() {
    let too_long: &str = "an84characterslonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio1569pvx";
    assert_eq!(
        Bech32::default().decode(too_long),
        Err(Error::LengthInvalid)
    );
}

#[test]
fn no_separator_character() {
    assert_eq!(
        Bech32::default().decode("pzry9x0s0muk"),
        Err(Error::SeparatorMissing)
    );
}

#[test]
fn empty_hrp() {
    for empty_hrp_str in &["1pzry9x0s0muk", "10a06t8", "1qzzfhee"] {
        assert_eq!(
            Bech32::default().decode(empty_hrp_str),
            Err(Error::PrefixInvalid)
        );
    }
}

#[test]
fn invalid_data_character() {
    assert_eq!(
        Bech32::default().decode("x1b4n0q5v"),
        Err(Error::DataInvalid { byte: 98 })
    );
}

#[test]
fn checksum_too_short() {
    assert_eq!(
        Bech32::default().decode("li1dgmt3"),
        Err(Error::LengthInvalid)
    );
}

#[test]
fn invalid_character_in_checksum() {
    assert_eq!(
        Bech32::default().decode("de1lg7wt\x7F"),
        Err(Error::CharInvalid { char: '\x7F' })
    );
}

#[test]
fn checksum_calculated_with_uppercase_hrp() {
    assert_eq!(
        Bech32::default().decode("A1G7SGD8"),
        Err(Error::ChecksumInvalid)
    );
}

// NOTE: not in test vectors but worth testing for anyway
#[test]
fn invalid_mixed_case() {
    assert_eq!(
        Bech32::default().decode("a12UEL5L"),
        Err(Error::CaseInvalid)
    );
}
