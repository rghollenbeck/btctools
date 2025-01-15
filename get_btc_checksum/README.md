# get_btc_checksum

**get_btc_checksum** is a lightweight Rust library for verifying and generating Bitcoin seed phrase checksums. It ensures that a mnemonic phrase adheres to the BIP-39 standard by validating the word count, wordlist compatibility, and checksum correctness.

## Features
- Validate a BIP-39 seed phrase for:
  - Proper word count (12, 15, 18, 21, or 24 words).
  - Validity of individual words against the BIP-39 wordlist.
  - Correct checksum.
- Simple integration into other projects.

## Usage

### Example Code
Below is an example of how to use `get_btc_checksum` in your Rust project:

```rust
use get_btc_checksum::{validate_seed_phrase, SeedPhraseError};

fn main() {
    let seed_phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    match validate_seed_phrase(seed_phrase) {
        Ok(()) => println!("Seed phrase is valid!"),
        Err(SeedPhraseError::InvalidWord(word)) => println!("Invalid word in seed phrase: {}", word),
        Err(SeedPhraseError::InvalidChecksum) => println!("Invalid checksum for seed phrase."),
        Err(SeedPhraseError::InvalidWordCount) => println!("Seed phrase must contain 12, 15, 18, 21, or 24 words."),
    }
}
```

### Installation

1. Add `get_btc_checksum` as a dependency in your `Cargo.toml`:
   ```toml
   [dependencies]
   get_btc_checksum = "0.1.0"
   ```

2. Import `get_btc_checksum` into your project:
   ```rust
   use get_btc_checksum::*;
   ```

## License
This project is licensed under the [MIT License](LICENSE).

## Contributions
Contributions are welcome! Feel free to submit pull requests or open issues on [GitHub](https://github.com/rghollenbeck/btctools).

## Keywords
- Bitcoin
- BIP-39
- Mnemonic
- Wallet
- Checksum

## Categories
- Cryptography
- Blockchain
- Bitcoin
- Wallets

