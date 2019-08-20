use crypto::pbkdf2;
use mnemonic::Mnemonic;
use std::fmt;

/// The secret value used to derive HD wallet addresses from a [`Mnemonic`][Mnemonic] phrase.
///
/// Because it is not possible to create a [`Mnemonic`][Mnemonic] instance that is invalid, it is
/// therefore impossible to have a [`Seed`][Seed] instance that is invalid. This guarantees that only
/// a valid, intact mnemonic phrase can be used to derive HD wallet addresses.
///
/// To get the raw byte value use [`Seed::as_bytes()`][Seed::as_bytes()]. These can be used to derive
/// HD wallet addresses using another crate (deriving HD wallet addresses is outside the scope of this
/// crate and the BIP39 standard).
///
/// [Mnemonic]: ./mnemonic/struct.Mnemonic.html
/// [Seed]: ./seed/struct.Seed.html
/// [Seed::as_bytes()]: ./seed/struct.Seed.html#method.as_bytes

#[derive(Clone)]
pub struct Seed {
    bytes: Vec<u8>,
}

impl Seed {
    /// Generates the seed from the [`Mnemonic`][Mnemonic] and the password.
    ///
    /// [Mnemonic]: ./mnemonic/struct.Mnemonic.html
    pub fn new(mnemonic: &Mnemonic, password: &str) -> Self {
        let salt = format!("mnemonic{}", password);
        let bytes = pbkdf2(mnemonic.phrase().as_bytes(), &salt);

        Self {
            bytes,
        }
    }

    /// Get the seed value as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsRef<[u8]> for Seed {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl fmt::Debug for Seed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#X}", self)
    }
}

impl fmt::LowerHex for Seed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        for byte in &self.bytes {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

impl fmt::UpperHex for Seed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        for byte in &self.bytes {
            write!(f, "{:02X}", byte)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use language::Language;

    #[test]
    fn seed_hex_format() {
        let entropy = &[0x33, 0xE4, 0x6B, 0xB1, 0x3A, 0x74, 0x6E, 0xA4, 0x1C, 0xDD, 0xE4, 0x5C, 0x90, 0x84, 0x6A, 0x79];

        let mnemonic = Mnemonic::from_entropy(entropy, Language::English).unwrap();
        let seed = Seed::new(&mnemonic, "password");

        assert_eq!(format!("{:x}", seed), "0bde96f14c35a66235478e0c16c152fcaf6301e4d9a81d3febc50879fe7e5438e6a8dd3e39bdf3ab7b12d6b44218710e17d7a2844ee9633fab0e03d9a6c8569b");
        assert_eq!(format!("{:X}", seed), "0BDE96F14C35A66235478E0C16C152FCAF6301E4D9A81D3FEBC50879FE7E5438E6A8DD3E39BDF3AB7B12D6B44218710E17D7A2844EE9633FAB0E03D9A6C8569B");
        assert_eq!(format!("{:#x}", seed), "0x0bde96f14c35a66235478e0c16c152fcaf6301e4d9a81d3febc50879fe7e5438e6a8dd3e39bdf3ab7b12d6b44218710e17d7a2844ee9633fab0e03d9a6c8569b");
        assert_eq!(format!("{:#X}", seed), "0x0BDE96F14C35A66235478E0C16C152FCAF6301E4D9A81D3FEBC50879FE7E5438E6A8DD3E39BDF3AB7B12D6B44218710E17D7A2844EE9633FAB0E03D9A6C8569B");
    }
}
