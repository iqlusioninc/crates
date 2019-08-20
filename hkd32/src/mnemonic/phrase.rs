//! BIP39 mnemonic phrases

use super::{
    bits::{BitWriter, IterExt},
    language::Language,
    seed::{Seed, SEED_SIZE},
};
use crate::{Error, KeyMaterial, Path, KEY_SIZE};
use alloc::string::String;
use core::convert::TryInto;
use hmac::Hmac;
use sha2::{Digest, Sha256, Sha512};
use zeroize::{Zeroize, Zeroizing};

/// Number of PBKDF2 rounds to perform when deriving the seed
const PBKDF2_ROUNDS: usize = 2048;

/// Source entropy for a BIP39 mnemonic phrase
pub type Entropy = [u8; KEY_SIZE];

/// BIP39 mnemonic phrases: sequences of words representing cryptographic keys.
#[derive(Clone)]
pub struct Phrase {
    /// Language
    language: Language,

    /// Source entropy for this phrase
    entropy: Entropy,

    /// Mnemonic phrase
    phrase: String,
}

impl Phrase {
    /// Create a random BIP39 mnemonic phrase.
    pub fn random(language: Language) -> Self {
        let mut entropy = Entropy::default();
        getrandom::getrandom(&mut entropy).expect("RNG failure!");
        Self::from_entropy(entropy, language)
    }

    /// Create a new BIP39 mnemonic phrase from the given entropy
    pub fn from_entropy(entropy: Entropy, language: Language) -> Self {
        let wordlist = language.wordlist();
        let checksum_byte = Sha256::digest(entropy.as_ref()).as_ref()[0];

        // First, create a byte iterator for the given entropy and the first byte of the
        // hash of the entropy that will serve as the checksum (up to 8 bits for biggest
        // entropy source).
        //
        // Then we transform that into a bits iterator that returns 11 bits at a
        // time (as u16), which we can map to the words on the `wordlist`.
        //
        // Given the entropy is of correct size, this ought to give us the correct word
        // count.
        let phrase = entropy
            .iter()
            .chain(Some(&checksum_byte))
            .bits()
            .map(|bits| wordlist.get_word(bits))
            .join(" ");

        Phrase {
            phrase,
            language,
            entropy,
        }
    }

    /// Create a new BIP39 mnemonic phrase from the given string.
    ///
    /// The phrase supplied will be checked for word length and validated
    /// according to the checksum specified in BIP0039.
    pub fn new<S>(phrase: S, language: Language) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        let phrase = phrase.as_ref();
        let wordmap = language.wordmap();

        // Preallocate enough space for the longest possible word list
        let mut bits = BitWriter::with_capacity(264);

        for word in phrase.split(" ") {
            bits.push(wordmap.get_bits(&word)?);
        }

        let mut entropy = Zeroizing::new(bits.into_bytes());

        if entropy.len() != KEY_SIZE + 1 {
            return Err(Error);
        }

        let actual_checksum = entropy[KEY_SIZE];

        // Truncate to get rid of the byte containing the checksum
        entropy.truncate(KEY_SIZE);

        let expected_checksum = Sha256::digest(&entropy).as_ref()[0];

        if actual_checksum != expected_checksum {
            Err(Error)?;
        }

        Ok(Self::from_entropy(
            entropy.as_slice().try_into().unwrap(),
            language,
        ))
    }

    /// Get source entropy for this phrase.
    pub fn entropy(&self) -> &Entropy {
        &self.entropy
    }

    /// Get the mnemonic phrase as a string reference.
    pub fn phrase(&self) -> &str {
        &self.phrase
    }

    /// Language this phrase's wordlist is for
    pub fn language(&self) -> Language {
        self.language
    }

    /// Convert this mnemonic phrase's entropy directly into key material.
    /// If you are looking for the shortest path between a mnemonic phrase
    /// and a key derivation hierarchy, this is it.
    ///
    /// Note: that this does not follow the normal BIP39 derivation, which
    /// first applies PBKDF2 along with a secondary password. Use `to_seed`
    /// if you are interested in BIP39 compatibility.
    /// Derive a BIP32 subkey from this seed
    pub fn derive_subkey(self, path: impl AsRef<Path>) -> KeyMaterial {
        KeyMaterial::from(self).derive_subkey(path)
    }

    /// Convert this mnemonic phrase into the BIP39 seed value
    pub fn to_seed(&self, password: &str) -> Seed {
        let salt = Zeroizing::new(format!("mnemonic{}", password));
        let mut seed = [0u8; SEED_SIZE];
        pbkdf2::pbkdf2::<Hmac<Sha512>>(
            &self.phrase.as_bytes(),
            salt.as_bytes(),
            PBKDF2_ROUNDS,
            &mut seed,
        );
        Seed(seed)
    }
}

impl From<Phrase> for KeyMaterial {
    /// Convert to `KeyMaterial` using an empty password
    fn from(phrase: Phrase) -> KeyMaterial {
        KeyMaterial::from_bytes(&phrase.entropy).unwrap()
    }
}

impl Drop for Phrase {
    fn drop(&mut self) {
        self.phrase.zeroize();
        self.entropy.zeroize();
    }
}
