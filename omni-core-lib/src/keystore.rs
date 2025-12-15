//! Key storage for server and client keys.
//!
//! Provides YAML-based persistent storage for cryptographic keys.

use crate::crypto::ServerKeyPair;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};

/// Server key entry for a specific client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerKeyEntry {
    pub client_id: String,
    pub server_public_key: String,
    pub server_secret_key: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Client configuration entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientEntry {
    pub client_id: String,
    pub client_public_key: String,
    pub registered_at: chrono::DateTime<chrono::Utc>,
}

/// Key store manager for persistent key storage
#[derive(Clone)]
pub struct KeyStoreManager {
    server_keys: Arc<RwLock<HashMap<String, ServerKeyEntry>>>,
    client_keys: Arc<RwLock<HashMap<String, ClientEntry>>>,
    data_dir: String,
}

impl KeyStoreManager {
    /// Create a new key store manager
    pub fn new() -> Self {
        Self::with_data_dir("data")
    }

    /// Create with custom data directory
    pub fn with_data_dir(data_dir: &str) -> Self {
        let manager = Self {
            server_keys: Arc::new(RwLock::new(HashMap::new())),
            client_keys: Arc::new(RwLock::new(HashMap::new())),
            data_dir: data_dir.to_string(),
        };
        manager.load_from_disk();
        manager
    }

    /// Generate a new server keypair for a client
    pub fn generate_for_client(&self, client_id: &str) -> ServerKeyEntry {
        let keypair = ServerKeyPair::generate();
        let entry = ServerKeyEntry {
            client_id: client_id.to_string(),
            server_public_key: keypair.public_key_hex(),
            server_secret_key: keypair.secret_key_hex(),
            created_at: chrono::Utc::now(),
        };

        self.server_keys
            .write()
            .unwrap()
            .insert(client_id.to_string(), entry.clone());
        self.save_to_disk();

        entry
    }

    /// Get server key entry for a client
    pub fn get_server_key(&self, client_id: &str) -> Option<ServerKeyEntry> {
        self.server_keys.read().unwrap().get(client_id).cloned()
    }

    /// Get or generate server key for a client
    pub fn get_or_generate(&self, client_id: &str) -> ServerKeyEntry {
        if let Some(entry) = self.get_server_key(client_id) {
            return entry;
        }
        self.generate_for_client(client_id)
    }

    /// Register a client's public key
    pub fn register_client(&self, client_id: &str, public_key: &str) -> ClientEntry {
        let entry = ClientEntry {
            client_id: client_id.to_string(),
            client_public_key: public_key.to_string(),
            registered_at: chrono::Utc::now(),
        };

        self.client_keys
            .write()
            .unwrap()
            .insert(client_id.to_string(), entry.clone());
        self.save_to_disk();

        entry
    }

    /// Get client entry
    pub fn get_client(&self, client_id: &str) -> Option<ClientEntry> {
        self.client_keys.read().unwrap().get(client_id).cloned()
    }

    /// Derive shared secret for a client
    pub fn derive_shared_secret(&self, client_id: &str) -> Option<[u8; 32]> {
        let server_key = self.get_server_key(client_id)?;
        let client_key = self.get_client(client_id)?;

        let keypair = ServerKeyPair::from_secret_hex(&server_key.server_secret_key).ok()?;
        keypair
            .derive_shared_secret_hex(&client_key.client_public_key)
            .ok()
    }

    /// Load keys from disk
    fn load_from_disk(&self) {
        let server_keys_path = format!("{}/server_keys.yaml", self.data_dir);
        let client_keys_path = format!("{}/client_keys.yaml", self.data_dir);

        if Path::new(&server_keys_path).exists() {
            if let Ok(content) = fs::read_to_string(&server_keys_path) {
                if let Ok(keys) = serde_yaml::from_str::<HashMap<String, ServerKeyEntry>>(&content)
                {
                    *self.server_keys.write().unwrap() = keys;
                }
            }
        }

        if Path::new(&client_keys_path).exists() {
            if let Ok(content) = fs::read_to_string(&client_keys_path) {
                if let Ok(keys) = serde_yaml::from_str::<HashMap<String, ClientEntry>>(&content) {
                    *self.client_keys.write().unwrap() = keys;
                }
            }
        }
    }

    /// Save keys to disk
    fn save_to_disk(&self) {
        let _ = fs::create_dir_all(&self.data_dir);

        let server_keys_path = format!("{}/server_keys.yaml", self.data_dir);
        let client_keys_path = format!("{}/client_keys.yaml", self.data_dir);

        if let Ok(content) = serde_yaml::to_string(&*self.server_keys.read().unwrap()) {
            let _ = fs::write(&server_keys_path, content);
        }

        if let Ok(content) = serde_yaml::to_string(&*self.client_keys.read().unwrap()) {
            let _ = fs::write(&client_keys_path, content);
        }
    }
}

impl Default for KeyStoreManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_key_generation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = KeyStoreManager::with_data_dir(temp_dir.path().to_str().unwrap());

        let entry = manager.generate_for_client("test-client");
        assert_eq!(entry.client_id, "test-client");
        assert!(!entry.server_public_key.is_empty());

        let retrieved = manager.get_server_key("test-client").unwrap();
        assert_eq!(retrieved.server_public_key, entry.server_public_key);
    }

    #[test]
    fn test_client_registration() {
        let temp_dir = TempDir::new().unwrap();
        let manager = KeyStoreManager::with_data_dir(temp_dir.path().to_str().unwrap());

        let entry = manager.register_client("client-1", "abc123");
        assert_eq!(entry.client_public_key, "abc123");

        let retrieved = manager.get_client("client-1").unwrap();
        assert_eq!(retrieved.client_public_key, "abc123");
    }
}
