# badlib

**badlib** (derived from BIP-39 Answers Desk) is a lightweight Rust library for interacting with the BIP-39 wordlist. It provides functions to convert between binary, decimal, and BIP-39 words efficiently.  Calculating a BIP-39 mnemonic word phrase and checking for a valid checksum can be a hassle.  Badlib modularizes the process.  Other processes in the wallet under construction can query this library and get quick lookup of a mnemonic word's decimal index or its binary counterpart and match it to a BIP-39 mnemonic word, and also the reverse.

**Note:** badlib is now part of the [btctools](https://github.com/rghollenbeck/btctools) suite, a toolbox of libraries and utilities for Bitcoin developers and enthusiasts.

## Features
- Convert **binary to word**.
- Convert **decimal to word**.
- Convert **word to binary**.
- Convert **word to decimal**.

## License
This project is licensed under the [MIT License](../LICENSE).

## Usage

### Example Code
Below is an example of how to use `badlib` in your Rust project:

```rust
use badlib::{load_wordlist, binary_to_word, decimal_to_word, word_to_binary, word_to_decimal};

fn main() {
    let word_list = load_wordlist("data/bip39list.json").expect("Failed to load wordlist");

    // Binary to Word
    assert_eq!(binary_to_word(&word_list, "00000000000"), Some("abandon"));

    // Decimal to Word
    assert_eq!(decimal_to_word(&word_list, 5), Some("absent"));

    // Word to Binary
    assert_eq!(word_to_binary(&word_list, "abandon"), Some("00000000000"));

    // Word to Decimal
    assert_eq!(word_to_decimal(&word_list, "abandon"), Some(0));
}
```

### Installation
1. Add `badlib` as a dependency in your `Cargo.toml`:
   ```toml
   [dependencies]
   badlib = "0.1.0"
   ```

2. Import `badlib` into your project:
   ```rust
   use badlib::*;
   ```

## License
This project is licensed under the [MIT License](LICENSE).

## Contributions
Contributions are welcome! Feel free to submit pull requests or open issues on [GitHub](https://github.com/rghollenbeck/badlib).
