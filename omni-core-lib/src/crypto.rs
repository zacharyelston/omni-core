//! Cryptographic services for secure communication.
//!
//! Implements X25519 key exchange and ChaCha20Poly1305 encryption.

use base64::Engine;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use x25519_dalek::{PublicKey, StaticSecret};

/// Server keypair for X25519 key exchange
#[derive(Clone)]
pub struct ServerKeyPair {
    secret: StaticSecret,
    public: PublicKey,
}

impl ServerKeyPair {
    /// Generate a new random keypair
    pub fn generate() -> Self {
        let secret = StaticSecret::random_from_rng(rand::thread_rng());
        let public = PublicKey::from(&secret);
        Self { secret, public }
    }

    /// Restore from hex-encoded secret key
    pub fn from_secret_hex(secret_hex: &str) -> Result<Self, CryptoError> {
        let bytes = hex::decode(secret_hex).map_err(|_| CryptoError::InvalidSecretKey)?;
        let arr: [u8; 32] = bytes
            .try_into()
            .map_err(|_| CryptoError::InvalidSecretKey)?;
        let secret = StaticSecret::from(arr);
        let public = PublicKey::from(&secret);
        Ok(Self { secret, public })
    }

    /// Get public key as bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.public.to_bytes()
    }

    /// Get public key as hex string
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public.to_bytes())
    }

    /// Get secret key as hex string (for secure storage)
    pub fn secret_key_hex(&self) -> String {
        hex::encode(self.secret.as_bytes())
    }

    /// Derive shared secret from a client's public key
    pub fn derive_shared_secret(&self, client_public: &[u8; 32]) -> [u8; 32] {
        let client_public = PublicKey::from(*client_public);
        self.secret.diffie_hellman(&client_public).to_bytes()
    }

    /// Derive shared secret from hex-encoded public key
    pub fn derive_shared_secret_hex(&self, public_hex: &str) -> Result<[u8; 32], CryptoError> {
        let bytes = parse_public_key(public_hex)?;
        Ok(self.derive_shared_secret(&bytes))
    }
}

/// Client keypair for X25519 key exchange
#[derive(Clone)]
pub struct ClientKeyPair {
    secret: StaticSecret,
    public: PublicKey,
}

impl ClientKeyPair {
    /// Generate a new random keypair
    pub fn generate() -> Self {
        let secret = StaticSecret::random_from_rng(rand::thread_rng());
        let public = PublicKey::from(&secret);
        Self { secret, public }
    }

    /// Get public key as hex string
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public.to_bytes())
    }

    /// Derive shared secret from server's public key
    pub fn derive_shared_secret(&self, server_public: &[u8; 32]) -> [u8; 32] {
        let server_public = PublicKey::from(*server_public);
        self.secret.diffie_hellman(&server_public).to_bytes()
    }
}

/// Encrypted message with nonce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    /// Base64-encoded nonce (12 bytes)
    pub nonce: String,
    /// Base64-encoded ciphertext
    pub ciphertext: String,
}

impl EncryptedMessage {
    /// Encrypt plaintext using the shared secret
    pub fn encrypt(plaintext: &[u8], shared_secret: &[u8; 32]) -> Result<Self, CryptoError> {
        let cipher =
            ChaCha20Poly1305::new_from_slice(shared_secret).map_err(|_| CryptoError::InvalidKey)?;

        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|_| CryptoError::EncryptionFailed)?;

        let b64 = base64::engine::general_purpose::STANDARD;
        Ok(Self {
            nonce: b64.encode(nonce_bytes),
            ciphertext: b64.encode(ciphertext),
        })
    }

    /// Decrypt ciphertext using the shared secret
    pub fn decrypt(&self, shared_secret: &[u8; 32]) -> Result<Vec<u8>, CryptoError> {
        let b64 = base64::engine::general_purpose::STANDARD;

        let nonce_bytes: [u8; 12] = b64
            .decode(&self.nonce)
            .map_err(|_| CryptoError::InvalidNonce)?
            .try_into()
            .map_err(|_| CryptoError::InvalidNonce)?;

        let ciphertext = b64
            .decode(&self.ciphertext)
            .map_err(|_| CryptoError::InvalidCiphertext)?;

        let cipher =
            ChaCha20Poly1305::new_from_slice(shared_secret).map_err(|_| CryptoError::InvalidKey)?;

        let nonce = Nonce::from_slice(&nonce_bytes);

        cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|_| CryptoError::DecryptionFailed)
    }
}

/// Cryptographic errors
#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Invalid key")]
    InvalidKey,
    #[error("Invalid secret key")]
    InvalidSecretKey,
    #[error("Invalid public key")]
    InvalidPublicKey,
    #[error("Invalid nonce")]
    InvalidNonce,
    #[error("Invalid ciphertext")]
    InvalidCiphertext,
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
}

/// Parse hex-encoded public key
pub fn parse_public_key(hex_key: &str) -> Result<[u8; 32], CryptoError> {
    let bytes = hex::decode(hex_key).map_err(|_| CryptoError::InvalidPublicKey)?;
    bytes.try_into().map_err(|_| CryptoError::InvalidPublicKey)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_exchange() {
        let server = ServerKeyPair::generate();
        let client = ClientKeyPair::generate();

        let server_shared = server
            .derive_shared_secret_hex(&client.public_key_hex())
            .unwrap();
        let client_shared = client.derive_shared_secret(&server.public_key_bytes());

        assert_eq!(server_shared, client_shared);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let server = ServerKeyPair::generate();
        let client = ClientKeyPair::generate();

        let shared = server
            .derive_shared_secret_hex(&client.public_key_hex())
            .unwrap();

        let plaintext = b"Hello, World!";
        let encrypted = EncryptedMessage::encrypt(plaintext, &shared).unwrap();
        let decrypted = encrypted.decrypt(&shared).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_keypair_serialization() {
        let original = ServerKeyPair::generate();
        let secret_hex = original.secret_key_hex();
        let public_hex = original.public_key_hex();

        let restored = ServerKeyPair::from_secret_hex(&secret_hex).unwrap();
        assert_eq!(restored.public_key_hex(), public_hex);
    }
}
