/*

This file mainly has utility functions to help with encryption and decryption of data.

*/

use aes_gcm_siv::aead::{generic_array::GenericArray, Aead};
use aes_gcm_siv::{Aes256GcmSiv, KeyInit};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::error::Error;
use base64::prelude::*;
use sha2::{Digest, Sha256};

/// Encrypts the given data using the given key and a random nonce.
/// The nonce length is 12 bytes.
/// The output is a base64 encoded string
pub fn encrypt<T: Serialize>(
    data: T,
    key: &[u8; 32],
) -> Result<String, Box<dyn Error>> {
    // The plaintext is the JSON rep of the data
    let plaintext = serde_json::to_string(&data)?;

    // Create the nonce using a cryptographically secure random number generator
    let nonce = rand::thread_rng().gen::<[u8; 12]>();

    // Create the cipher
    let key = GenericArray::from_slice(key);
    let cipher = Aes256GcmSiv::new(key);
    let ciphertext = cipher
        .encrypt(GenericArray::from_slice(&nonce), plaintext.as_bytes())
        .map_err(|e| e.to_string())?;

    // Combine the nonce and the ciphertext
    let mut encrypted_data = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphertext);

    // Return the base64 encoded string
    let encoded_data = BASE64_URL_SAFE.encode(&encrypted_data);

    Ok(encoded_data)
}

/// Decrypts the given base64 encoded string using the given key.
/// The nonce length is 12 bytes.
/// The input is a base64 encoded string
pub fn decrypt<T: for<'a> Deserialize<'a>>(
    data: &str,
    key: &[u8; 32],
) -> Result<T, Box<dyn Error>> {
    // Decode the base64 encoded string
    let encrypted_data = BASE64_URL_SAFE.decode(data.as_bytes())?;

    // Split the nonce and the ciphertext
    let nonce = GenericArray::from_slice(&encrypted_data[..12]);
    let ciphertext = &encrypted_data[12..];

    // Create the cipher
    let key = GenericArray::from_slice(key);
    let cipher = Aes256GcmSiv::new(key);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| e.to_string())?;

    // Convert from vector of bytes to string
    let plaintext = String::from_utf8(plaintext)?;

    // The plaintext is the JSON rep of the data
    let data: T = serde_json::from_str(&plaintext)?;

    Ok(data)
}

pub fn hash(data: &str) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    let result = format!("{:x}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pet;

    #[test]
    fn test_encrypt_decrypt() {
        let key = rand::thread_rng().gen::<[u8; 32]>();
        let data = "Hello, World!";
        let encrypted_data = encrypt(data, &key).unwrap();
        let decrypted_data: String = decrypt(&encrypted_data, &key).unwrap();
        assert_eq!(data, decrypted_data);
    }

    #[test]
    fn test_encrypt_decrypt_struct() {
        let key = rand::thread_rng().gen::<[u8; 32]>();
        let data = Pet::new("Test".to_string(), "Test".to_string());
        let encrypted_data = encrypt(&data, &key).unwrap();
        let decrypted_data: Pet = decrypt(&encrypted_data, &key).unwrap();
        assert_eq!(data, decrypted_data);
    }

    #[test]
    fn test_encrypt_decrypt_struct_with_option() {
        let key = rand::thread_rng().gen::<[u8; 32]>();
        let mut data = Pet::new("Test".to_string(), "Test".to_string());
        data.set_pet_yard("Test".to_string());
        let encrypted_data = encrypt(&data, &key).unwrap();
        let decrypted_data: Pet = decrypt(&encrypted_data, &key).unwrap();
        assert_eq!(data, decrypted_data);
    }

    #[test]
    fn test_encrypt_decrypt_struct_with_option_none() {
        let key = rand::thread_rng().gen::<[u8; 32]>();
        let data = Pet::new("Test".to_string(), "Test".to_string());
        let encrypted_data = encrypt(&data, &key).unwrap();
        let decrypted_data: Pet = decrypt(&encrypted_data, &key).unwrap();
        assert_eq!(data, decrypted_data);
    }

    #[test]
    fn test_encrypt_decrypt_not_equal() {
        let key = rand::thread_rng().gen::<[u8; 32]>();
        let data_1 = "Hello, World!";
        let data_2 = "Hello, World";

        let encrypted_data_1 = encrypt(data_1, &key).unwrap();
        let encrypted_data_2 = encrypt(data_2, &key).unwrap();
        let decrypted_data_1: String = decrypt(&encrypted_data_1, &key).unwrap();
        let decrypted_data_2: String = decrypt(&encrypted_data_2, &key).unwrap();

        assert_ne!(data_1, data_2);
        assert_ne!(encrypted_data_1, encrypted_data_2);
        assert_ne!(decrypted_data_1, decrypted_data_2);
    }

    #[test]
    fn test_hash() {
        let data = "Hello, World!";
        let hashed_data = hash(data);
        assert_eq!(hashed_data, "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f");
    }
}