//! Tests for crypto module

#[cfg(test)]
mod tests {
    use crate::services::crypto::*;

    #[test]
    fn test_server_keypair_generation() {
        let keypair = ServerKeyPair::generate();
        let public_hex = keypair.public_key_hex();
        
        // Public key should be 64 hex chars (32 bytes)
        assert_eq!(public_hex.len(), 64);
        assert!(hex::decode(&public_hex).is_ok());
    }

    #[test]
    fn test_key_exchange_produces_same_secret() {
        let server = ServerKeyPair::generate();
        let server_public = server.public_key_bytes();

        // Simulate client keypair
        use x25519_dalek::{EphemeralSecret, PublicKey};
        let client_secret = EphemeralSecret::random_from_rng(rand::thread_rng());
        let client_public = PublicKey::from(&client_secret);
        let client_public_bytes = client_public.to_bytes();

        // Server derives shared secret
        let server_shared = server.derive_shared_secret(&client_public_bytes);

        // Client derives shared secret
        let server_public_key = PublicKey::from(server_public);
        let client_shared = client_secret.diffie_hellman(&server_public_key).to_bytes();

        // Both should be identical
        assert_eq!(server_shared, client_shared);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let shared_secret = [42u8; 32];
        let plaintext = b"Hello, encrypted world!";

        let encrypted = EncryptedMessage::encrypt(plaintext, &shared_secret).unwrap();
        let decrypted = encrypted.decrypt(&shared_secret).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_decrypt_with_wrong_key_fails() {
        let shared_secret = [42u8; 32];
        let wrong_secret = [99u8; 32];
        let plaintext = b"Secret message";

        let encrypted = EncryptedMessage::encrypt(plaintext, &shared_secret).unwrap();
        let result = encrypted.decrypt(&wrong_secret);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_public_key_valid() {
        let valid_hex = "a".repeat(64);
        let result = parse_public_key(&valid_hex);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_public_key_invalid_length() {
        let short_hex = "abc123";
        let result = parse_public_key(short_hex);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_public_key_invalid_hex() {
        let invalid_hex = "g".repeat(64); // 'g' is not valid hex
        let result = parse_public_key(&invalid_hex);
        assert!(result.is_err());
    }

    #[test]
    fn test_encrypted_message_different_nonces() {
        let shared_secret = [42u8; 32];
        let plaintext = b"Same message";

        let encrypted1 = EncryptedMessage::encrypt(plaintext, &shared_secret).unwrap();
        let encrypted2 = EncryptedMessage::encrypt(plaintext, &shared_secret).unwrap();

        // Nonces should be different (random)
        assert_ne!(encrypted1.nonce, encrypted2.nonce);
        // Ciphertexts should be different due to different nonces
        assert_ne!(encrypted1.ciphertext, encrypted2.ciphertext);
    }
}
