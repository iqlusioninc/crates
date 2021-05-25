//! Parser for extended key types (i.e. `xprv` and `xpub`)

use crate::{ChainCode, Depth, Error, Result, Version, KEY_SIZE};
use core::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

/// Size of an extended key when deserialized into bytes from Base58.
const EXTENDED_KEY_BYTES: usize = 78;

/// Serialized extended key (e.g. `xprv` and `xpub`).
pub(crate) struct ExtendedKey {
    /// [Version] - prefix of the key interpreted as a big endian integer.
    pub version: Version,

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

impl FromStr for ExtendedKey {
    type Err = Error;

    fn from_str(base58: &str) -> Result<Self> {
        let mut bytes = [0u8; EXTENDED_KEY_BYTES + 4];

        let decoded_len = bs58::decode(base58)
            .with_check(None)
            .into(&mut bytes)
            .map_err(|_| Error)?;

        if decoded_len != EXTENDED_KEY_BYTES {
            return Err(Error);
        }

        let version = Version::try_from(&bytes[..4])?;
        let depth = bytes[4];
        let parent_fingerprint = u32::from_be_bytes(bytes[5..9].try_into()?);
        let child_number = u32::from_be_bytes(bytes[9..13].try_into()?);
        let chain_code = bytes[13..45].try_into()?;
        let key_bytes = bytes[46..78].try_into()?;

        #[cfg(feature = "zeroize")]
        bytes.zeroize();

        Ok(ExtendedKey {
            version,
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
        self.version = Version::Other(0);
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
    use super::{ExtendedKey, Version};
    use hex_literal::hex;

    #[test]
    fn bip32_test_vector_1() {
        let xprv: ExtendedKey = "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPP\
             qjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi"
            .parse()
            .unwrap();

        assert_eq!(xprv.version, Version::XPrv);
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

        assert_eq!(xpub.version, Version::XPub);
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
