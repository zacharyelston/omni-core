//! Tests for keystore module

#[cfg(test)]
mod tests {
    use crate::services::keystore::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_server_key_entry_generation() {
        let entry = ServerKeyEntry::generate("test-client");

        assert_eq!(entry.client_id, "test-client");
        assert_eq!(entry.public_key.len(), 64); // 32 bytes hex
        assert_eq!(entry.secret_key.len(), 64); // 32 bytes hex
    }

    #[test]
    fn test_server_key_entry_derive_shared_secret() {
        let entry = ServerKeyEntry::generate("test-client");

        // Create a valid client public key
        use x25519_dalek::{EphemeralSecret, PublicKey};
        let client_secret = EphemeralSecret::random_from_rng(rand::thread_rng());
        let client_public = PublicKey::from(&client_secret);
        let client_public_hex = hex::encode(client_public.to_bytes());

        let shared = entry.derive_shared_secret(&client_public_hex);
        assert!(shared.is_some());
        assert_eq!(shared.unwrap().len(), 32);
    }

    #[test]
    fn test_server_keys_store_save_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("server_keys.yaml");
        let path_str = path.to_str().unwrap();

        let mut store = ServerKeysStore::default();
        let entry = ServerKeyEntry::generate("client-1");
        store.add_key(entry.clone());

        store.save_to(path_str).unwrap();

        let loaded = ServerKeysStore::load_from(path_str);
        assert!(loaded.get_key("client-1").is_some());
        assert_eq!(
            loaded.get_key("client-1").unwrap().public_key,
            entry.public_key
        );
    }

    #[test]
    fn test_client_config_store_save_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("client_config.yaml");
        let path_str = path.to_str().unwrap();

        let mut store = ClientConfigStore::default();
        let entry = ClientEntry {
            client_id: "client-1".to_string(),
            client_public_key: "abc123".to_string(),
            server_key_id: "client-1".to_string(),
            registered_at: "2024-01-01T00:00:00Z".to_string(),
            last_seen: None,
        };
        store.add_client(entry.clone());

        store.save_to(path_str).unwrap();

        let loaded = ClientConfigStore::load_from(path_str);
        assert!(loaded.get_client("client-1").is_some());
        assert_eq!(
            loaded.get_client("client-1").unwrap().client_public_key,
            "abc123"
        );
    }

    #[test]
    fn test_key_store_manager_generate_and_register() {
        let manager = KeyStoreManager::new();

        // Generate server key
        let server_key = manager.generate_server_key_for_client("test-device");
        assert_eq!(server_key.client_id, "test-device");

        // Verify it's stored
        let retrieved = manager.get_server_key("test-device");
        assert!(retrieved.is_some());

        // Register client
        let client = manager.register_client(
            "test-device",
            "abc123def456abc123def456abc123def456abc123def456abc123def456abcd",
        );
        assert!(client.is_some());

        // Verify client is stored
        let retrieved_client = manager.get_client("test-device");
        assert!(retrieved_client.is_some());
    }

    #[test]
    fn test_key_store_manager_list_operations() {
        let manager = KeyStoreManager::new();

        manager.generate_server_key_for_client("device-1");
        manager.generate_server_key_for_client("device-2");

        let keys = manager.list_server_keys();
        assert!(keys.len() >= 2);
    }
}
