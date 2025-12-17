//! Cryptographic services for key exchange and encryption

use base64::Engine;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};

/// Server keypair for X25519 key exchange
#[derive(Clone)]
pub struct ServerKeyPair {
    secret: StaticSecret,
    public: PublicKey,
}

impl ServerKeyPair {
    pub fn generate() -> Self {
        let secret = StaticSecret::random_from_rng(rand::thread_rng());
        let public = PublicKey::from(&secret);
        Self { secret, public }
    }

    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.public.to_bytes()
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public.to_bytes())
    }

    /// Derive shared secret from client's public key
    pub fn derive_shared_secret(&self, client_public: &[u8; 32]) -> [u8; 32] {
        let client_public = PublicKey::from(*client_public);
        self.secret.diffie_hellman(&client_public).to_bytes()
    }
}

/// Client keypair (ephemeral, generated per session)
pub struct ClientKeyPair {
    secret: EphemeralSecret,
    public: PublicKey,
}

impl ClientKeyPair {
    pub fn generate() -> Self {
        let secret = EphemeralSecret::random_from_rng(rand::thread_rng());
        let public = PublicKey::from(&secret);
        Self { secret, public }
    }

    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.public.to_bytes()
    }

    /// Derive shared secret from server's public key
    pub fn derive_shared_secret(self, server_public: &[u8; 32]) -> [u8; 32] {
        let server_public = PublicKey::from(*server_public);
        self.secret.diffie_hellman(&server_public).to_bytes()
    }
}

/// Encrypted message envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    /// Base64-encoded nonce (12 bytes)
    pub nonce: String,
    /// Base64-encoded ciphertext
    pub ciphertext: String,
}

impl EncryptedMessage {
    /// Encrypt plaintext using shared secret
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

    /// Decrypt ciphertext using shared secret
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

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Invalid key")]
    InvalidKey,
    #[error("Invalid nonce")]
    InvalidNonce,
    #[error("Invalid ciphertext")]
    InvalidCiphertext,
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Invalid public key")]
    InvalidPublicKey,
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
    fn test_key_exchange_and_encryption() {
        // Server generates keypair
        let server = ServerKeyPair::generate();
        let server_public = server.public_key_bytes();

        // Client generates ephemeral keypair
        let client = ClientKeyPair::generate();
        let client_public = client.public_key_bytes();

        // Both derive the same shared secret
        let server_shared = server.derive_shared_secret(&client_public);
        let client_shared = client.derive_shared_secret(&server_public);

        assert_eq!(server_shared, client_shared);

        // Encrypt with server's shared secret
        let plaintext = b"Hello, encrypted world!";
        let encrypted = EncryptedMessage::encrypt(plaintext, &server_shared).unwrap();

        // Decrypt with client's shared secret (same as server's)
        let decrypted = encrypted.decrypt(&client_shared).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }
}
