fn main() {
    let bit_length = 128;
    let binary_string = "1010101010101010101010101010101010101010101010101010101010101010";

    // Import the function from the library
    match your_crate_name::compute_entropy_checksum(bit_length, binary_string) {
        Ok(checksum) => println!("Checksum (first {} bits): {}", bit_length / 32, checksum),
        Err(e) => println!("Error: {}", e),
    }
}

