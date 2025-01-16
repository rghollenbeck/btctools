// src/lib.rs in mnemonic

// new lib.rs
use serde::Deserialize;
use std::fs;

pub mod mnemonic; // Expose the bip39 module

#[derive(Deserialize, Clone)] // Add Clone derivation
pub struct WordList {
    pub wordlist: Vec<WordEntry>,
}

#[derive(Deserialize, Clone)] // Add Clone derivation
pub struct WordEntry {
    pub binary: String,
    pub word: String,
}

/// Converts a binary to its associated BIP39 word.
pub fn binary_to_word<'a>(word_list: &'a WordList, binary: &str) -> Option<&'a str> {
    word_list.wordlist.iter().find(|w| w.binary == binary).map(|w| w.word.as_str())
}

/// Converts a BIP39 word to its associated binary.
pub fn word_to_binary<'a>(word_list: &'a WordList, word: &str) -> Option<&'a str> {
    word_list.wordlist.iter().find(|w| w.word == word).map(|w| w.binary.as_str())
}

/// Loads the BIP39 word list from the JSON file.
pub fn load_wordlist(path: &str) -> Result<WordList, Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string(path)?;
    let wordlist: WordList = serde_json::from_str(&json_data)?;
    Ok(wordlist)
}

/// Converts a decimal index value to its BIP39 word.
pub fn decimal_to_word(word_list: &WordList, decimal: u32) -> Option<&str> {
    let binary = format!("{:011b}", decimal);
    binary_to_word(word_list, &binary)
}

/// Converts a BIP39 word to its decimal index value.
pub fn word_to_decimal(word_list: &WordList, word: &str) -> Option<u32> {
    word_to_binary(word_list, word).and_then(|binary| u32::from_str_radix(binary, 2).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_to_word() {
        let word_list = load_wordlist("data/bip39list.json").unwrap();
        assert_eq!(binary_to_word(&word_list, "00000000000"), Some("abandon"));
    }

    #[test]
    fn test_decimal_to_word() {
        let word_list = load_wordlist("data/bip39list.json").unwrap();
        assert_eq!(decimal_to_word(&word_list, 0), Some("abandon"));
    }
}


/* old lib.rs
use serde::Deserialize;
use std::fs;
pub mod mnemonic; // Expose the bip39 module

#[derive(Deserialize)]
pub struct WordList {
    pub wordlist: Vec<WordEntry>,
}

/// Converts a binary to its associated BIP39 word.
pub fn binary_to_word<'a>(word_list: &'a WordList, binary: &str) -> Option<&'a str> {
    word_list.wordlist.iter().find(|w| w.binary == binary).map(|w| w.word.as_str())
}	

/// Converts a BIP39 word to its associated binary.
pub fn word_to_binary<'a>(word_list: &'a WordList, word: &str) -> Option<&'a str> {
    word_list.wordlist.iter().find(|w| w.word == word).map(|w| w.binary.as_str())
}

#[derive(Deserialize)]
pub struct WordEntry {
    pub binary: String,
    pub word: String,
}

/// Loads the BIP39 word list from the json file.
pub fn load_wordlist(path: &str) -> Result<WordList, Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string(path)?;
    let wordlist: WordList = serde_json::from_str(&json_data)?;
    Ok(wordlist)
}

/// Converts a decimal index value to its BIP39 word 
pub fn decimal_to_word(word_list: &WordList, decimal: u32) -> Option<&str> {
    let binary = format!("{:011b}", decimal);
    binary_to_word(word_list, &binary)
}


/// Converts a BIP39 word to its decimal index value
pub fn word_to_decimal(word_list: &WordList, word: &str) -> Option<u32> {
    word_to_binary(word_list, word).and_then(|binary| u32::from_str_radix(binary, 2).ok())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_to_word() {
        let word_list = load_wordlist("data/bip39list.json").unwrap(); //path_to_bip39list.json
        assert_eq!(binary_to_word(&word_list, "00000000000"), Some("abandon"));
    }

    #[test]
    fn test_decimal_to_word() {
        let word_list = load_wordlist("data/bip39list.json").unwrap(); //path_to_bip39list.json
        assert_eq!(decimal_to_word(&word_list, 0), Some("abandon"));
    }
}*/
