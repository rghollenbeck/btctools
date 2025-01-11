use sha2::{Digest, Sha256};
use thiserror::Error;

/// Custom error type for the entropy hasher function
#[derive(Error, Debug)]
pub enum EntropyHasherError {
    #[error("Invalid bit length. Expected one of: 128, 160, 192, 224, 256. Code: 100")]
    InvalidBitLength,
    #[error("Binary string length does not match the bit length. Code: 200")]
    LengthMismatch,
    #[error("Binary string contains invalid characters. Only '1' and '0' are allowed. Code: 300")]
    InvalidCharacters,
}

/// Function to compute the first `ENT / 32` bits of the SHA256 hash in binary
pub fn compute_entropy_checksum(bit_length: u32, binary_string: &str) -> Result<String, EntropyHasherError> {
    // Validate that the bit_length is one of the allowed values
    match bit_length {
        128 | 160 | 192 | 224 | 256 => (),
        _ => return Err(EntropyHasherError::InvalidBitLength),
    }

    // Ensure the binary string length matches the bit length
    if binary_string.len() as u32 != bit_length {
        return Err(EntropyHasherError::LengthMismatch);
    }

    // Validate that the binary string contains only '1' and '0'
    if binary_string.chars().any(|c| c != '1' && c != '0') {
        return Err(EntropyHasherError::InvalidCharacters);
    }

    // Convert the binary string to bytes
    let entropy_bytes = (0..binary_string.len())
        .step_by(8)
        .map(|i| u8::from_str_radix(&binary_string[i..i + 8], 2))
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|_| EntropyHasherError::InvalidCharacters)?;

    // Compute the SHA256 hash of the entropy
    let hash = Sha256::digest(&entropy_bytes);

    // Determine the number of checksum bits (ENT / 32)
    let checksum_bits = (bit_length / 32) as usize;

    // Extract the first `checksum_bits` from the hash and convert to binary
    let mut checksum_binary = String::new();
    for i in 0..checksum_bits {
        let bit = (hash[0] >> (7 - i)) & 1; // Extract bits from the first byte of the hash
        checksum_binary.push_str(&bit.to_string());
    }

    Ok(checksum_binary)
}



