// tests/integration_tests.rs
use badlib::*;

#[test]
fn test_word_to_binary() {
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
    let word_list = load_wordlist("data/bip39list.json").unwrap();
    assert_eq!(word_to_binary(&word_list, "zoo"), Some("11111111111"));
}

#[test]
fn test_word_to_decimal() {
	println!("Current directory: {:?}", std::env::current_dir().unwrap());
    let word_list = load_wordlist("data/bip39list.json").unwrap();
    assert_eq!(word_to_decimal(&word_list, "zoo"), Some(2047));

}

#[test]
fn test_binary_to_word() {
	println!("Current directory: {:?}", std::env::current_dir().unwrap());
    let word_list = load_wordlist("data/bip39list.json").unwrap();
    assert_eq!(binary_to_word(&word_list, "00000000000"), Some("abandon"));
}

#[test]
fn test_decimal_to_word() {
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
    let word_list = load_wordlist("data/bip39list.json").unwrap();
    assert_eq!(decimal_to_word(&word_list, 0), Some("abandon"));
}

#[test]
fn test_outlier_number_one() {
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
    let word_list = load_wordlist("data/bip39list.json").unwrap();
    assert_eq!(binary_to_word(&word_list, "101010101010"), None); // Invalid binary
}

#[test]
fn test_outlier_number_two() {
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
    let word_list = load_wordlist("data/bip39list.json").unwrap();
    assert_eq!(decimal_to_word(&word_list, 2048), None); // Out of range
}

#[test]
fn test_outlier_number_three() {
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
    let word_list = load_wordlist("data/bip39list.json").unwrap();
    assert_eq!(decimal_to_word(&word_list, u32::MAX), None); // Extreme value
}
