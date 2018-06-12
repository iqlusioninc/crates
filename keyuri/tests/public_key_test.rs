extern crate keyuri;

// Ed25519 public key test
//
// Uses public key from RFC 8032 test vector: "TEST 1" secret key
// https://tools.ietf.org/html/rfc8032#section-7.1
mod ed25519 {
    use keyuri::public_key::Ed25519PublicKey;
    use keyuri::{Encodable, KeyURI};

    const EXAMPLE_URI: &str =
        "public.key:ed25519;6adfsqvzky9t042tlmfujeq88g8wzuhnm2nzxfd0qgdx3ac82ydqcr0f3h";

    const EXAMPLE_DASHERIZED: &str =
        "public-key-ed25519.6adfsqvzky9t042tlmfujeq88g8wzuhnm2nzxfd0qgdx3ac82ydqpk359s";

    const EXAMPLE_BYTES: &[u8] = &[
        215, 90, 152, 1, 130, 177, 10, 183, 213, 75, 254, 211, 201, 100, 7, 58, 14, 225, 114, 243,
        218, 166, 35, 37, 175, 2, 26, 104, 247, 7, 81, 26,
    ];

    #[test]
    fn parse_uri() {
        let key = KeyURI::parse_uri(EXAMPLE_URI).unwrap();
        assert_eq!(
            key.public_key().unwrap().ed25519_key().unwrap().as_ref(),
            EXAMPLE_BYTES
        );
    }

    #[test]
    fn parse_dasherized() {
        let key = KeyURI::parse_dasherized(EXAMPLE_DASHERIZED).unwrap();
        assert_eq!(
            key.public_key().unwrap().ed25519_key().unwrap().as_ref(),
            EXAMPLE_BYTES
        );
    }

    #[test]
    fn serialize_uri() {
        let key = Ed25519PublicKey::new(EXAMPLE_BYTES).unwrap();
        assert_eq!(&key.to_uri_string(), EXAMPLE_URI);
    }

    #[test]
    fn serialize_dasherized() {
        let key = Ed25519PublicKey::new(EXAMPLE_BYTES).unwrap();
        assert_eq!(&key.to_dasherized_string(), EXAMPLE_DASHERIZED);
    }
}
