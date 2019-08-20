use error::ErrorKind;
use failure::Error;
use std::fmt;

const ENTROPY_OFFSET: usize = 8;

/// Determines the number of words that will be present in a [`Mnemonic`][Mnemonic] phrase
///
/// Also directly affects the amount of entropy that will be used to create a [`Mnemonic`][Mnemonic],
/// and therefore the cryptographic strength of the HD wallet keys/addresses that can be derived from
/// it using the [`Seed`][Seed].
///
/// For example, a 12 word mnemonic phrase is essentially a friendly representation of a 128-bit key,
/// while a 24 word mnemonic phrase is essentially a 256-bit key.
///
/// If you know you want a specific phrase length, you can use the enum variant directly, for example
/// `MnemonicType::Words12`.
///
/// You can also get a `MnemonicType` that corresponds to one of the standard BIP39 key sizes by
/// passing arbitrary `usize` values:
///
/// ```
/// use bip39::{MnemonicType};
///
/// let mnemonic_type = MnemonicType::for_key_size(128).unwrap();
/// ```
///
/// [MnemonicType]: ../mnemonic_type/struct.MnemonicType.html
/// [Mnemonic]: ../mnemonic/struct.Mnemonic.html
/// [Seed]: ../seed/struct.Seed.html
///
#[derive(Debug, Copy, Clone)]
pub enum MnemonicType {
    //  ... = (entropy_bits << ...)   | checksum_bits
    Words12 = (128 << ENTROPY_OFFSET) | 4,
    Words15 = (160 << ENTROPY_OFFSET) | 5,
    Words18 = (192 << ENTROPY_OFFSET) | 6,
    Words21 = (224 << ENTROPY_OFFSET) | 7,
    Words24 = (256 << ENTROPY_OFFSET) | 8,
}

impl MnemonicType {
    /// Get a `MnemonicType` for a mnemonic phrase with a specific number of words
    ///
    /// Specifying a word count not provided for by the BIP39 standard will return an `Error`
    /// of kind `ErrorKind::InvalidWordLength`.
    ///
    /// # Example
    /// ```
    /// use bip39::{MnemonicType};
    ///
    /// let mnemonic_type = MnemonicType::for_word_count(12).unwrap();
    /// ```
    pub fn for_word_count(size: usize) -> Result<MnemonicType, Error> {
        let mnemonic_type = match size {
            12 => MnemonicType::Words12,
            15 => MnemonicType::Words15,
            18 => MnemonicType::Words18,
            21 => MnemonicType::Words21,
            24 => MnemonicType::Words24,
            _ => Err(ErrorKind::InvalidWordLength(size))?,
        };

        Ok(mnemonic_type)
    }

    /// Get a `MnemonicType` for a mnemonic phrase representing the given key size as bits
    ///
    /// Specifying a key size not provided for by the BIP39 standard will return an `Error`
    /// of kind `ErrorKind::InvalidKeysize`.
    ///
    /// # Example
    /// ```
    /// use bip39::{MnemonicType};
    ///
    /// let mnemonic_type = MnemonicType::for_key_size(128).unwrap();
    /// ```
    pub fn for_key_size(size: usize) -> Result<MnemonicType, Error> {
        let mnemonic_type = match size {
            128 => MnemonicType::Words12,
            160 => MnemonicType::Words15,
            192 => MnemonicType::Words18,
            224 => MnemonicType::Words21,
            256 => MnemonicType::Words24,
            _ => Err(ErrorKind::InvalidKeysize(size))?,
        };

        Ok(mnemonic_type)
    }

    /// Get a `MnemonicType` for an existing mnemonic phrase
    ///
    /// This can be used when you need information about a mnemonic phrase based on the number of
    /// words, for example you can get the entropy value using [`MnemonicType::entropy_bits`][MnemonicType::entropy_bits()].
    ///
    /// Specifying a phrase that does not match one of the standard BIP39 phrase lengths will return
    /// an `Error` of kind `ErrorKind::InvalidWordLength`. The phrase will not be validated in any
    /// other way.
    ///
    /// # Example
    /// ```
    /// use bip39::{MnemonicType};
    ///
    /// let test_mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let mnemonic_type = MnemonicType::for_phrase(test_mnemonic).unwrap();
    ///
    /// let entropy_bits = mnemonic_type.entropy_bits();
    /// ```
    ///
    /// [MnemonicType::entropy_bits()]: ./enum.MnemonicType.html#method.entropy_bits
    pub fn for_phrase(phrase: &str) -> Result<MnemonicType, Error> {
        let word_count = phrase.split(" ").count();

        Self::for_word_count(word_count)
    }

    /// Return the number of entropy+checksum bits
    ///
    ///
    /// # Example
    /// ```
    /// use bip39::{MnemonicType};
    ///
    /// let test_mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let mnemonic_type = MnemonicType::for_phrase(test_mnemonic).unwrap();
    ///
    /// let total_bits = mnemonic_type.total_bits();
    /// ```
    pub fn total_bits(&self) -> usize {
        self.entropy_bits() + self.checksum_bits() as usize
    }

    /// Return the number of entropy bits
    ///
    ///
    /// # Example
    /// ```
    /// use bip39::{MnemonicType};
    ///
    /// let test_mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let mnemonic_type = MnemonicType::for_phrase(test_mnemonic).unwrap();
    ///
    /// let entropy_bits = mnemonic_type.entropy_bits();
    /// ```
    pub fn entropy_bits(&self) -> usize {
        (*self as usize) >> ENTROPY_OFFSET
    }

    /// Return the number of checksum bits
    ///
    ///
    /// # Example
    /// ```
    /// use bip39::{MnemonicType};
    ///
    /// let test_mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let mnemonic_type = MnemonicType::for_phrase(test_mnemonic).unwrap();
    ///
    /// let checksum_bits = mnemonic_type.checksum_bits();
    /// ```
    pub fn checksum_bits(&self) -> u8 {
        (*self as usize) as u8
    }

    /// Return the number of words
    ///
    ///
    /// # Example
    /// ```
    /// use bip39::{MnemonicType};
    ///
    /// let mnemonic_type = MnemonicType::Words12;
    ///
    /// let word_count = mnemonic_type.word_count();
    /// ```
    pub fn word_count(&self) -> usize {
        self.total_bits() / 11
    }
}

impl Default for MnemonicType {
    fn default() -> MnemonicType {
        MnemonicType::Words12
    }
}

impl fmt::Display for MnemonicType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} words ({}bits)",
            self.word_count(),
            self.entropy_bits()
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn word_count() {
        assert_eq!(MnemonicType::Words12.word_count(), 12);
        assert_eq!(MnemonicType::Words15.word_count(), 15);
        assert_eq!(MnemonicType::Words18.word_count(), 18);
        assert_eq!(MnemonicType::Words21.word_count(), 21);
        assert_eq!(MnemonicType::Words24.word_count(), 24);
    }

    #[test]
    fn entropy_bits() {
        assert_eq!(MnemonicType::Words12.entropy_bits(), 128);
        assert_eq!(MnemonicType::Words15.entropy_bits(), 160);
        assert_eq!(MnemonicType::Words18.entropy_bits(), 192);
        assert_eq!(MnemonicType::Words21.entropy_bits(), 224);
        assert_eq!(MnemonicType::Words24.entropy_bits(), 256);
    }

    #[test]
    fn checksum_bits() {
        assert_eq!(MnemonicType::Words12.checksum_bits(), 4);
        assert_eq!(MnemonicType::Words15.checksum_bits(), 5);
        assert_eq!(MnemonicType::Words18.checksum_bits(), 6);
        assert_eq!(MnemonicType::Words21.checksum_bits(), 7);
        assert_eq!(MnemonicType::Words24.checksum_bits(), 8);
    }
}
