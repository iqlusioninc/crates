//! Support for BIP39 mnemonics.
//!
//! These enable deriving `hkd32::KeyMaterial` from a 24-word BIP39 phrase.

use crate::{Error, KeyMaterial};
use alloc::string::String;
use zeroize::Zeroize;

/// Number of words required for an HKD32-compatible mnemonic phrase
pub const WORD_COUNT: usize = 24;

/// Supported languages.
///
/// Presently only English is specified by the BIP39 standard
#[derive(Copy, Clone, Debug)]
pub enum Language {
    /// English is presently the only supported language
    English,
}

impl Default for Language {
    fn default() -> Language {
        Language::English
    }
}

impl From<Language> for bip39::Language {
    fn from(language: Language) -> bip39::Language {
        match language {
            Language::English => bip39::Language::English,
        }
    }
}

/// 24-word BIP39 Mnemonic phrase
pub struct Phrase {
    /// String value containing the phrase
    string: String,

    /// Associated language for this phrase
    language: Language,
}

impl Phrase {
    /// Create a random BIP39 mnemonic phrase.
    pub fn random(language: Language) -> Self {
        Self::from_key_material(&KeyMaterial::random(), language)
    }

    /// Create a new BIP39 mnemonic phrase from the given string.
    ///
    /// Must be a valid 24-word BIP39 mnemonic.
    pub fn new<S>(phrase: S, language: Language) -> Result<Phrase, Error>
    where
        S: AsRef<str>,
    {
        let phrase = phrase.as_ref();

        if bip39::Mnemonic::validate(phrase, language.into()).is_err() {
            return Err(Error);
        }

        if phrase.split(' ').count() != WORD_COUNT {
            return Err(Error);
        }

        Ok(Phrase {
            string: phrase.into(),
            language,
        })
    }

    /// Create a new BIP39 mnemonic phrase from the given `KeyMaterial`
    pub(crate) fn from_key_material(key_material: &KeyMaterial, language: Language) -> Self {
        let mnemonic =
            bip39::Mnemonic::from_entropy(key_material.as_bytes(), language.into()).unwrap();

        Phrase {
            string: mnemonic.into_phrase(),
            language,
        }
    }

    /// Borrow this mnemonic phrase as a string.
    pub fn as_str(&self) -> &str {
        self.string.as_ref()
    }

    /// Language this phrase's wordlist is for
    pub fn language(&self) -> Language {
        self.language
    }
}

impl Drop for Phrase {
    fn drop(&mut self) {
        self.string.zeroize();
    }
}

impl From<Phrase> for KeyMaterial {
    fn from(phrase: Phrase) -> KeyMaterial {
        let mnemonic =
            bip39::Mnemonic::from_phrase(&phrase.string, phrase.language.into()).unwrap();
        Self::from_bytes(mnemonic.entropy()).unwrap()
    }
}
