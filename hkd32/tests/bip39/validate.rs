extern crate bip39;

use ::bip39::{Mnemonic, MnemonicType, Language};

fn validate_language(lang: Language) {
    let types = &[
        MnemonicType::Words12,
        MnemonicType::Words15,
        MnemonicType::Words18,
        MnemonicType::Words21,
        MnemonicType::Words24,
    ];

    for mtype in types {
        for _ in 0..1000 {
            let m1 = Mnemonic::new(*mtype, lang);
            let m2 = Mnemonic::from_phrase(m1.phrase(), lang).expect("Can create a Mnemonic");

            assert_eq!(m1.entropy(), m2.entropy());
        }
    }
}

#[test]
fn validate_12_english() {
    let phrase = "park remain person kitchen mule spell knee armed position rail grid ankle";

    let _ = Mnemonic::from_phrase(phrase, Language::English).expect("Can create a Mnemonic");
}

#[test]
fn validate_15_english() {
    let phrase = "any paddle cabbage armor atom satoshi fiction night wisdom nasty they midnight chicken play phone";

    let _ = Mnemonic::from_phrase(phrase, Language::English).expect("Can create a Mnemonic");
}

#[test]
fn validate_18_english() {
    let phrase = "soda oak spy claim best oppose gun ghost school use sign shock sign pipe vote follow category filter";

    let _ = Mnemonic::from_phrase(phrase, Language::English).expect("Can create a Mnemonic");
}


#[test]
fn validate_21_english() {
    let phrase = "quality useless orient offer pole host amazing title only clog sight wild anxiety gloom market rescue fan language entry fan oyster";

    let _ = Mnemonic::from_phrase(phrase, Language::English).expect("Can create a Mnemonic");
}


#[test]
fn validate_24_english() {
    let phrase = "always guess retreat devote warm poem giraffe thought prize ready maple daughter girl feel clay silent lemon bracket abstract basket toe tiny sword world";

    let _ = Mnemonic::from_phrase(phrase, Language::English).expect("Can create a Mnemonic");
}


#[test]
fn validate_12_english_uppercase() {
    let invalid_phrase = "Park remain person kitchen mule spell knee armed position rail grid ankle";

    assert!(Mnemonic::from_phrase(invalid_phrase, Language::English).is_err());
}

#[test]
fn validate_english() {
    validate_language(Language::English);
}

#[test]
fn validate_chinese_simplified() {
    validate_language(Language::ChineseSimplified);
}

#[test]
fn validate_chinese_traditional() {
    validate_language(Language::ChineseTraditional);
}

#[test]
fn validate_french() {
    validate_language(Language::French);
}

#[test]
fn validate_italian() {
    validate_language(Language::Italian);
}

#[test]
fn validate_japanese() {
    validate_language(Language::Japanese);
}

#[test]
fn validate_korean() {
    validate_language(Language::Korean);
}

#[test]
fn validate_spanish() {
    validate_language(Language::Spanish);
}
