//! Wordlist support
//!
//! NOTE: This implementation is not constant time and may leak information
//! via timing side-channels!
//!
//! Adapted from the `bip39` crate

use super::bits::{Bits, Bits11};
use crate::Error;
use alloc::{collections::BTreeMap, vec::Vec};

/// Supported languages.
///
/// Presently only English is specified by the BIP39 standard
#[derive(Copy, Clone, Debug)]
pub enum Language {
    /// English is presently the only supported language
    English,
}

impl Language {
    /// Get the word list for this language
    pub(crate) fn wordlist(&self) -> &'static WordList {
        match *self {
            Language::English => &lazy::WORDLIST_ENGLISH,
        }
    }

    /// Get a wordmap that allows word -> index lookups in the word list
    pub(crate) fn wordmap(&self) -> &'static WordMap {
        match *self {
            Language::English => &lazy::WORDMAP_ENGLISH,
        }
    }
}

impl Default for Language {
    fn default() -> Language {
        Language::English
    }
}

pub(crate) struct WordMap {
    inner: BTreeMap<&'static str, Bits11>,
}

pub(crate) struct WordList {
    inner: Vec<&'static str>,
}

impl WordMap {
    pub fn get_bits(&self, word: &str) -> Result<Bits11, Error> {
        self.inner.get(word).cloned().ok_or_else(|| Error)
    }
}

impl WordList {
    pub fn get_word(&self, bits: Bits11) -> &'static str {
        self.inner[bits.bits() as usize]
    }
}

mod lazy {
    use super::{Bits11, WordList, WordMap};
    use alloc::vec::Vec;
    use lazy_static::lazy_static;

    /// lazy generation of the word list
    fn gen_wordlist(lang_words: &'static str) -> WordList {
        let inner: Vec<_> = lang_words.split_whitespace().collect();

        debug_assert!(inner.len() == 2048, "Invalid wordlist length");

        WordList { inner }
    }

    /// lazy generation of the word map
    fn gen_wordmap(wordlist: &WordList) -> WordMap {
        let inner = wordlist
            .inner
            .iter()
            .enumerate()
            .map(|(i, item)| (*item, Bits11::from(i as u16)))
            .collect();

        WordMap { inner }
    }

    lazy_static! {
        pub(crate) static ref WORDLIST_ENGLISH: WordList =
            gen_wordlist(include_str!("langs/english.txt"));
        pub(crate) static ref WORDMAP_ENGLISH: WordMap = gen_wordmap(&WORDLIST_ENGLISH);
    }
}
