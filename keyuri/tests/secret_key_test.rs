extern crate keyuri;

macro_rules! secret_key_test {
    ($name:ident, $keytype:ident, $uri:expr, $dasherized:expr, $bytes:expr) => {
        mod $name {
            use keyuri::secret_key::$keytype;
            use keyuri::{AsSecretSlice, Encodable, KeyURI};

            #[test]
            fn parse_uri() {
                let key = KeyURI::parse_uri($uri).unwrap();
                assert_eq!(
                    key.secret_key().unwrap().$name().unwrap().as_secret_slice(),
                    $bytes
                );
            }

            #[test]
            fn parse_dasherized() {
                let key = KeyURI::parse_dasherized($dasherized).unwrap();
                assert_eq!(
                    key.secret_key().unwrap().$name().unwrap().as_secret_slice(),
                    $bytes
                );
            }

            #[test]
            fn serialize_uri() {
                let key = $keytype::new($bytes).unwrap();
                assert_eq!(&key.to_uri_string(), $uri);
            }

            #[test]
            fn serialize_dasherized() {
                let key = $keytype::new($bytes).unwrap();
                assert_eq!(&key.to_dasherized_string(), $dasherized);
            }
        }
    };
}

// AES-128-GCM secret key test
//
// Uses key from NIST AES-GCM test vector: gcmEncryptExtIV128.rsp (Count = 0)
// http://csrc.nist.gov/groups/STM/cavp/documents/mac/gcmtestvectors.zip
secret_key_test!(
    aes128gcm_key,
    Aes128GcmKey,
    "secret.key:aes128gcm;z965e4e2ascfhaf0w6rjzt5f2utyxnpq",
    "secret-key-aes128gcm.z965e4e2ascfhaf0w6rjzt5f2uvphxev",
    &[17, 117, 76, 215, 42, 236, 48, 155, 245, 47, 118, 135, 33, 46, 137, 87]
);

// AES-256-GCM secret key test
//
// Uses key from NIST AES-GCM test vector: gcmEncryptExtIV256.rsp (Count = 0)
// http://csrc.nist.gov/groups/STM/cavp/documents/mac/gcmtestvectors.zip
secret_key_test!(
    aes256gcm_key,
    Aes256GcmKey,
    "secret.key:aes256gcm;k5k9qk3h678d5hwnfusvyf2qagd4393ulrjmlrl6shulyjf9qk6qh2uawf",
    "secret-key-aes256gcm.k5k9qk3h678d5hwnfusvyf2qagd4393ulrjmlrl6shulyjf9qk6qyuu6uk",
    &[
        181, 44, 80, 90, 55, 215, 142, 218, 93, 211, 79, 32, 194, 37, 64, 234, 27, 88, 150, 60,
        248, 229, 191, 143, 250, 133, 249, 242, 73, 37, 5, 180
    ]
);

// Ed25519 secret key test
//
// Uses secret scalar from RFC 8032 test vector: "TEST 1" secret key
// https://tools.ietf.org/html/rfc8032#section-7.1
secret_key_test!(
    ed25519_key,
    Ed25519SecretKey,
    "secret.key:ed25519;n4smr800l4dxpw5yft6f9mpvc3zyn3tf0vexjxts8wkqx89w0asqj90p24",
    "secret-key-ed25519.n4smr800l4dxpw5yft6f9mpvc3zyn3tf0vexjxts8wkqx89w0asqts3u7j",
    &[
        157, 97, 177, 157, 239, 253, 90, 96, 186, 132, 74, 244, 146, 236, 44, 196, 68, 73, 197,
        105, 123, 50, 105, 25, 112, 59, 172, 3, 28, 174, 127, 96
    ]
);
