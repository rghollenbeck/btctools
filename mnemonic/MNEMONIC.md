//! # btctools::mnemonic
//!
//! `btctools::mnemonic` is a Rust library designed to handle operations related to the BIP-39 mnemonic standard.
//! It is part of the broader `btctools` crate, a toolbox for Bitcoin developers and enthusiasts.
//!
//! This library provides utilities for working with BIP-39 mnemonic wordlists, including:
//! - Loading the BIP-39 wordlist from a JSON file.
//! - Converting between binary, decimal, and mnemonic words.
//! - Validating BIP-39 word inputs.
//!
//! ## Features
//! - **Binary to Word Conversion**:
//!   Convert 11-bit binary strings to their corresponding mnemonic words.
//!
//! - **Decimal to Word Conversion**:
//!   Convert integers (0-2047) to their corresponding mnemonic words.
//!
//! - **Word to Binary Conversion**:
//!   Find the binary value associated with a specific mnemonic word.
//!
//! - **Word to Decimal Conversion**:
//!   Retrieve the decimal value associated with a mnemonic word.
//!
//! ## Usage
//! To use this library, add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! btctools = { path = "../btctools" }
//! ```
//!
//! ### Example
//! ```rust
//! use btctools::mnemonic::{load_wordlist, decimal_to_word, word_to_decimal};
//!
//! fn main() {
//!     let wordlist_path = "data/bip39list.json";
//!     let wordlist = load_wordlist(wordlist_path).expect("Failed to load wordlist");
//!
//!     // Convert decimal to word
//!     let word = decimal_to_word(&wordlist, 0).expect("Failed to convert");
//!     println!("Word for decimal 0: {}", word);
//!
//!     // Convert word to decimal
//!     let decimal = word_to_decimal(&wordlist, "abandon").expect("Failed to convert");
//!     println!("Decimal for word 'abandon': {}", decimal);
//! }
//! ```
//!
//! ## File Format
//! The library expects the BIP-39 wordlist to be in JSON format, structured as follows:
//! ```json
//! {
//!     "wordlist": [
//!         {
//!             "binary": "00000000000",
//!             "word": "abandon"
//!         },
//!         {
//!             "binary": "00000000001",
//!             "word": "ability"
//!         }
//!     ]
//! }
//! ```
//!
//! ## Contribution
//! Contributions are welcome! If you encounter issues or have feature requests, feel free to open an issue on the [GitHub repository](https://github.com/rghollenbeck/btctools).
//!
//! ## License
//! This project is licensed under the MIT License. See the `LICENSE` file for details.

