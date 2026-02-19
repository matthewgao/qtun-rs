//! Cryptography utilities for AES-GCM encryption

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes128Gcm, Aes256Gcm, Nonce,
};
use thiserror::Error;

use crate::utils::hash::{md5, sha256};

pub const NONCE_SIZE: usize = 12;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Invalid key")]
    InvalidKey,
    #[error("Cipher not match")]
    CipherNotMatch,
}

/// AES-128-GCM cipher wrapper
pub struct Aes128GcmCipher {
    cipher: Aes128Gcm,
    key_bytes: [u8; 16],
}

impl Clone for Aes128GcmCipher {
    fn clone(&self) -> Self {
        let cipher = Aes128Gcm::new_from_slice(&self.key_bytes)
            .expect("Key bytes should be valid");
        Self {
            cipher,
            key_bytes: self.key_bytes,
        }
    }
}

impl Aes128GcmCipher {
    /// Create a new AES-128-GCM cipher from a key string
    pub fn new(key: &str) -> Result<Self, CryptoError> {
        let key_vec = md5(key.as_bytes());
        let mut key_bytes = [0u8; 16];
        key_bytes.copy_from_slice(&key_vec);
        let cipher = Aes128Gcm::new_from_slice(&key_bytes)
            .map_err(|_| CryptoError::InvalidKey)?;
        Ok(Self { cipher, key_bytes })
    }

    /// Encrypt data with the given nonce
    pub fn encrypt(&self, nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nonce = Nonce::from_slice(nonce);
        self.cipher
            .encrypt(nonce, plaintext)
            .map_err(|_| CryptoError::EncryptionFailed)
    }

    /// Decrypt data with the given nonce
    pub fn decrypt(&self, nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nonce = Nonce::from_slice(nonce);
        self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| CryptoError::CipherNotMatch)
    }
}

/// AES-256-GCM cipher wrapper
pub struct Aes256GcmCipher {
    cipher: Aes256Gcm,
}

impl Aes256GcmCipher {
    /// Create a new AES-256-GCM cipher from a key string
    pub fn new(key: &str) -> Result<Self, CryptoError> {
        let key_bytes = sha256(key.as_bytes());
        let cipher = Aes256Gcm::new_from_slice(&key_bytes)
            .map_err(|_| CryptoError::InvalidKey)?;
        Ok(Self { cipher })
    }

    /// Encrypt data with the given nonce
    pub fn encrypt(&self, nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nonce = Nonce::from_slice(nonce);
        self.cipher
            .encrypt(nonce, plaintext)
            .map_err(|_| CryptoError::EncryptionFailed)
    }

    /// Decrypt data with the given nonce
    pub fn decrypt(&self, nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nonce = Nonce::from_slice(nonce);
        self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| CryptoError::CipherNotMatch)
    }
}

/// Generate a random nonce
pub fn generate_nonce() -> [u8; NONCE_SIZE] {
    let mut nonce = [0u8; NONCE_SIZE];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes128_encrypt_decrypt() {
        let cipher = Aes128GcmCipher::new("test-key").unwrap();
        let nonce = generate_nonce();
        let plaintext = b"Hello, World!";
        
        let ciphertext = cipher.encrypt(&nonce, plaintext).unwrap();
        let decrypted = cipher.decrypt(&nonce, &ciphertext).unwrap();
        
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_aes256_encrypt_decrypt() {
        let cipher = Aes256GcmCipher::new("test-key").unwrap();
        let nonce = generate_nonce();
        let plaintext = b"Hello, World!";
        
        let ciphertext = cipher.encrypt(&nonce, plaintext).unwrap();
        let decrypted = cipher.decrypt(&nonce, &ciphertext).unwrap();
        
        assert_eq!(plaintext.to_vec(), decrypted);
    }
}
