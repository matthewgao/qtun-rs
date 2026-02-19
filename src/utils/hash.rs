//! Hash utility functions

use md5::{Md5, Digest as Md5Digest};
use sha1::{Sha1, Digest as Sha1Digest};
use sha2::{Sha256, Digest as Sha256Digest};
use base64::{Engine as _, engine::general_purpose};

/// Compute SHA256 hash
pub fn sha256(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

/// Compute SHA1 hash
pub fn sha1(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

/// Compute MD5 hash
pub fn md5(input: &[u8]) -> Vec<u8> {
    let mut hasher = Md5::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

/// Convert bytes to hex string
pub fn to_hex(input: &[u8]) -> String {
    hex::encode(input)
}

/// Convert bytes to base64 string (raw, no padding)
pub fn to_b64(input: &[u8]) -> String {
    general_purpose::STANDARD_NO_PAD.encode(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        let result = md5(b"hello");
        assert_eq!(result.len(), 16);
    }

    #[test]
    fn test_sha256() {
        let result = sha256(b"hello");
        assert_eq!(result.len(), 32);
    }
}
