//! Client storage using individual YAML files in clients.d/
//!
//! Each client gets its own config file for isolation and easier management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

const CLIENTS_DIR: &str = "data/clients.d";

/// A registered client entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    /// Unique client identifier
    pub client_id: String,
    /// Client's public key (hex-encoded)
    pub public_key: String,
    /// Server key ID used for this client
    pub server_key_id: String,
    /// Server's public key for this client (hex-encoded)
    pub server_public_key: String,
    /// Server's secret key for this client (hex-encoded, encrypted in production)
    pub server_secret_key: String,
    /// When the client registered
    pub registered_at: String,
    /// Last time client was seen
    pub last_seen: Option<String>,
    /// Client metadata
    pub metadata: Option<ClientMetadata>,
}

/// Optional client metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClientMetadata {
    /// Human-readable name
    pub name: Option<String>,
    /// Device type
    pub device_type: Option<String>,
    /// Client version
    pub version: Option<String>,
    /// Custom tags
    pub tags: Vec<String>,
}

impl ClientConfig {
    /// Create a new client config with generated server keypair
    pub fn new(client_id: &str, client_public_key: &str) -> Self {
        use x25519_dalek::{PublicKey, StaticSecret};

        let secret = StaticSecret::random_from_rng(rand::thread_rng());
        let public = PublicKey::from(&secret);

        Self {
            client_id: client_id.to_string(),
            public_key: client_public_key.to_string(),
            server_key_id: client_id.to_string(),
            server_public_key: hex::encode(public.to_bytes()),
            server_secret_key: hex::encode(secret.to_bytes()),
            registered_at: chrono::Utc::now().to_rfc3339(),
            last_seen: Some(chrono::Utc::now().to_rfc3339()),
            metadata: None,
        }
    }

    /// Get file path for this client's config
    fn file_path(&self) -> PathBuf {
        PathBuf::from(CLIENTS_DIR).join(format!("{}.yaml", self.client_id))
    }

    /// Save this client config to its own file
    pub fn save(&self) -> std::io::Result<()> {
        let path = self.file_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let yaml = serde_yaml::to_string(self).map_err(std::io::Error::other)?;
        fs::write(path, yaml)
    }

    /// Load a client config from file
    pub fn load(client_id: &str) -> Option<Self> {
        let path = PathBuf::from(CLIENTS_DIR).join(format!("{}.yaml", client_id));
        if path.exists() {
            let content = fs::read_to_string(path).ok()?;
            serde_yaml::from_str(&content).ok()
        } else {
            None
        }
    }

    /// Delete this client's config file
    pub fn delete(&self) -> std::io::Result<()> {
        let path = self.file_path();
        if path.exists() {
            fs::remove_file(path)
        } else {
            Ok(())
        }
    }

    /// Get the server's secret key for deriving shared secret
    pub fn get_server_secret(&self) -> Option<x25519_dalek::StaticSecret> {
        let bytes: [u8; 32] = hex::decode(&self.server_secret_key).ok()?.try_into().ok()?;
        Some(x25519_dalek::StaticSecret::from(bytes))
    }

    /// Derive shared secret with this client
    pub fn derive_shared_secret(&self) -> Option<[u8; 32]> {
        let secret = self.get_server_secret()?;
        let client_bytes: [u8; 32] = hex::decode(&self.public_key).ok()?.try_into().ok()?;
        let client_public = x25519_dalek::PublicKey::from(client_bytes);
        Some(secret.diffie_hellman(&client_public).to_bytes())
    }
}

/// Client store manager
#[derive(Clone)]
pub struct ClientStore {
    clients: Arc<RwLock<HashMap<String, ClientConfig>>>,
}

impl ClientStore {
    pub fn new() -> Self {
        let store = Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        };
        store.load_all();
        store
    }

    /// Load all client configs from clients.d/
    fn load_all(&self) {
        let dir = Path::new(CLIENTS_DIR);
        if !dir.exists() {
            let _ = fs::create_dir_all(dir);
            return;
        }

        if let Ok(entries) = fs::read_dir(dir) {
            let mut clients = self.clients.write().unwrap();
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "yaml") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(client) = serde_yaml::from_str::<ClientConfig>(&content) {
                            clients.insert(client.client_id.clone(), client);
                        }
                    }
                }
            }
        }
    }

    /// Register a new client (generates server keypair)
    pub fn register(
        &self,
        client_id: &str,
        client_public_key: &str,
    ) -> std::io::Result<ClientConfig> {
        let config = ClientConfig::new(client_id, client_public_key);
        config.save()?;

        let mut clients = self.clients.write().unwrap();
        clients.insert(client_id.to_string(), config.clone());

        Ok(config)
    }

    /// Initialize registration (server generates keypair, client hasn't sent public key yet)
    pub fn init_registration(&self, client_id: &str) -> std::io::Result<ClientConfig> {
        // Create with empty client public key - will be filled in on complete
        let config = ClientConfig::new(client_id, "");
        config.save()?;

        let mut clients = self.clients.write().unwrap();
        clients.insert(client_id.to_string(), config.clone());

        Ok(config)
    }

    /// Complete registration by setting client's public key
    pub fn complete_registration(
        &self,
        client_id: &str,
        client_public_key: &str,
    ) -> std::io::Result<Option<ClientConfig>> {
        let mut clients = self.clients.write().unwrap();

        if let Some(config) = clients.get_mut(client_id) {
            config.public_key = client_public_key.to_string();
            config.last_seen = Some(chrono::Utc::now().to_rfc3339());
            config.save()?;
            Ok(Some(config.clone()))
        } else {
            Ok(None)
        }
    }

    /// Get a client by ID
    pub fn get(&self, client_id: &str) -> Option<ClientConfig> {
        let clients = self.clients.read().unwrap();
        clients.get(client_id).cloned()
    }

    /// Check if a client exists
    pub fn exists(&self, client_id: &str) -> bool {
        let clients = self.clients.read().unwrap();
        clients.contains_key(client_id)
    }

    /// Remove a client
    pub fn remove(&self, client_id: &str) -> std::io::Result<()> {
        let mut clients = self.clients.write().unwrap();
        if let Some(config) = clients.remove(client_id) {
            config.delete()?;
        }
        Ok(())
    }

    /// Update last seen timestamp
    pub fn touch(&self, client_id: &str) -> std::io::Result<()> {
        let mut clients = self.clients.write().unwrap();
        if let Some(config) = clients.get_mut(client_id) {
            config.last_seen = Some(chrono::Utc::now().to_rfc3339());
            config.save()?;
        }
        Ok(())
    }

    /// List all clients
    pub fn list_all(&self) -> Vec<ClientConfig> {
        let clients = self.clients.read().unwrap();
        clients.values().cloned().collect()
    }

    /// Get client count
    pub fn count(&self) -> usize {
        let clients = self.clients.read().unwrap();
        clients.len()
    }

    /// Derive shared secret for a client
    pub fn derive_shared_secret(&self, client_id: &str) -> Option<[u8; 32]> {
        let clients = self.clients.read().unwrap();
        clients.get(client_id)?.derive_shared_secret()
    }
}

impl Default for ClientStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config_creation() {
        let config = ClientConfig::new("test-client", "abcd1234");
        assert_eq!(config.client_id, "test-client");
        assert_eq!(config.public_key, "abcd1234");
        assert!(!config.server_public_key.is_empty());
        assert!(!config.server_secret_key.is_empty());
    }

    #[test]
    fn test_client_store_in_memory() {
        let store = ClientStore {
            clients: Arc::new(RwLock::new(HashMap::new())),
        };

        // Add directly to in-memory store for testing
        let config = ClientConfig::new("client-1", "pubkey123");
        {
            let mut clients = store.clients.write().unwrap();
            clients.insert(config.client_id.clone(), config);
        }

        assert_eq!(store.count(), 1);
        assert!(store.exists("client-1"));

        let retrieved = store.get("client-1").unwrap();
        assert_eq!(retrieved.public_key, "pubkey123");
    }
}
