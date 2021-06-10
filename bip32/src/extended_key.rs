//! Parser for extended key types (i.e. `xprv` and `xpub`)

use crate::{ChainCode, Depth, Error, Prefix, Result, Version, KEY_SIZE};
use core::{convert::TryInto, str::FromStr};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

/// Serialized extended key (e.g. `xprv` and `xpub`).
pub struct ExtendedKey {
    /// [Prefix] (a.k.a. "version") of the key (e.g. `xprv`, `xpub`)
    pub prefix: Prefix,

    /// Depth in the key derivation hierarchy.
    pub depth: Depth,

    /// Parent fingerprint.
    pub parent_fingerprint: u32,

    /// Child number.
    pub child_number: u32,

    /// Chain code.
    pub chain_code: ChainCode,

    /// Key material.
    pub key_bytes: [u8; KEY_SIZE],
}

impl ExtendedKey {
    /// Size of an extended key when deserialized into bytes from Base58.
    pub const BYTE_SIZE: usize = 78;

    /// Maximum size of a Base58Check-encoded extended key in bytes.
    ///
    /// Note that extended keys can also be 111-bytes.
    pub const MAX_BASE58_SIZE: usize = 112;
}

impl FromStr for ExtendedKey {
    type Err = Error;

    fn from_str(base58: &str) -> Result<Self> {
        let mut bytes = [0u8; Self::BYTE_SIZE + 4]; // with 4-byte checksum
        let decoded_len = bs58::decode(base58).with_check(None).into(&mut bytes)?;

        if decoded_len != Self::BYTE_SIZE {
            return Err(Error::Decode);
        }

        let prefix = base58.get(..4).ok_or(Error::Decode).and_then(|chars| {
            Prefix::validate_str(chars)?;
            let version = Version::from_be_bytes(bytes[..4].try_into()?);
            Ok(Prefix::from_parts_unchecked(chars, version))
        })?;

        let depth = bytes[4];
        let parent_fingerprint = u32::from_be_bytes(bytes[5..9].try_into()?);
        let child_number = u32::from_be_bytes(bytes[9..13].try_into()?);
        let chain_code = bytes[13..45].try_into()?;
        let key_bytes = bytes[46..78].try_into()?;

        #[cfg(feature = "zeroize")]
        bytes.zeroize();

        Ok(ExtendedKey {
            prefix,
            depth,
            parent_fingerprint,
            child_number,
            chain_code,
            key_bytes,
        })
    }
}

#[cfg(feature = "zeroize")]
impl Zeroize for ExtendedKey {
    fn zeroize(&mut self) {
        // TODO(tarcieri): prefix?
        self.depth.zeroize();
        self.parent_fingerprint.zeroize();
        self.child_number.zeroize();
        self.chain_code.zeroize();
        self.key_bytes.zeroize();
    }
}

#[cfg(feature = "zeroize")]
impl Drop for ExtendedKey {
    fn drop(&mut self) {
        self.zeroize();
    }
}

// TODO(tarcieri): consolidate test vectors
#[cfg(test)]
mod tests {
    use super::ExtendedKey;
    use hex_literal::hex;

    #[test]
    fn bip32_test_vector_1() {
        let xprv: ExtendedKey = "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPP\
             qjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi"
            .parse()
            .unwrap();

        assert_eq!(xprv.prefix.as_str(), "xprv");
        assert_eq!(xprv.depth, 0);
        assert_eq!(xprv.parent_fingerprint, 0);
        assert_eq!(xprv.child_number, 0);
        assert_eq!(
            xprv.chain_code,
            hex!("873DFF81C02F525623FD1FE5167EAC3A55A049DE3D314BB42EE227FFED37D508")
        );
        assert_eq!(
            xprv.key_bytes,
            hex!("E8F32E723DECF4051AEFAC8E2C93C9C5B214313817CDB01A1494B917C8436B35")
        );

        let xpub: ExtendedKey = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhe\
             PY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8"
            .parse()
            .unwrap();

        assert_eq!(xpub.prefix.as_str(), "xpub");
        assert_eq!(xpub.depth, 0);
        assert_eq!(xpub.parent_fingerprint, 0);
        assert_eq!(xpub.child_number, 0);
        assert_eq!(
            xpub.chain_code,
            hex!("873DFF81C02F525623FD1FE5167EAC3A55A049DE3D314BB42EE227FFED37D508")
        );
        assert_eq!(
            xpub.key_bytes,
            hex!("39A36013301597DAEF41FBE593A02CC513D0B55527EC2DF1050E2E8FF49C85C2")
        );
    }
}
