//! YAML-based key storage for server and client keys

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};
use x25519_dalek::{PublicKey, StaticSecret};

const SERVER_KEYS_FILE: &str = "data/server_keys.yaml";
const CLIENT_CONFIG_FILE: &str = "data/client_config.yaml";

/// A server keypair for a specific client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerKeyEntry {
    pub client_id: String,
    pub public_key: String,
    #[serde(skip_serializing, skip_deserializing)]
    #[serde(default)]
    secret_key_bytes: Option<[u8; 32]>,
    /// Hex-encoded secret key (stored encrypted in production)
    pub secret_key: String,
    pub created_at: String,
}

impl ServerKeyEntry {
    pub fn generate(client_id: &str) -> Self {
        let secret = StaticSecret::random_from_rng(rand::thread_rng());
        let public = PublicKey::from(&secret);
        
        Self {
            client_id: client_id.to_string(),
            public_key: hex::encode(public.to_bytes()),
            secret_key_bytes: Some(secret.to_bytes()),
            secret_key: hex::encode(secret.to_bytes()),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn get_secret(&self) -> Option<StaticSecret> {
        if let Some(bytes) = self.secret_key_bytes {
            Some(StaticSecret::from(bytes))
        } else {
            // Parse from hex string
            let bytes = hex::decode(&self.secret_key).ok()?;
            let arr: [u8; 32] = bytes.try_into().ok()?;
            Some(StaticSecret::from(arr))
        }
    }

    pub fn derive_shared_secret(&self, client_public_hex: &str) -> Option<[u8; 32]> {
        let secret = self.get_secret()?;
        let client_bytes: [u8; 32] = hex::decode(client_public_hex).ok()?.try_into().ok()?;
        let client_public = PublicKey::from(client_bytes);
        Some(secret.diffie_hellman(&client_public).to_bytes())
    }
}

/// Client configuration entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientEntry {
    pub client_id: String,
    pub client_public_key: String,
    pub server_key_id: String,
    pub registered_at: String,
    pub last_seen: Option<String>,
}

/// Server keys storage (server_keys.yaml)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerKeysStore {
    pub keys: HashMap<String, ServerKeyEntry>,
}

impl ServerKeysStore {
    pub fn load() -> Self {
        Self::load_from(SERVER_KEYS_FILE)
    }

    pub fn load_from(path: &str) -> Self {
        if Path::new(path).exists() {
            let content = fs::read_to_string(path).unwrap_or_default();
            serde_yaml::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        self.save_to(SERVER_KEYS_FILE)
    }

    pub fn save_to(&self, path: &str) -> std::io::Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        let yaml = serde_yaml::to_string(self).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;
        fs::write(path, yaml)
    }

    pub fn add_key(&mut self, entry: ServerKeyEntry) {
        self.keys.insert(entry.client_id.clone(), entry);
    }

    pub fn get_key(&self, client_id: &str) -> Option<&ServerKeyEntry> {
        self.keys.get(client_id)
    }
}

/// Client configuration storage (client_config.yaml)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientConfigStore {
    pub clients: HashMap<String, ClientEntry>,
}

impl ClientConfigStore {
    pub fn load() -> Self {
        Self::load_from(CLIENT_CONFIG_FILE)
    }

    pub fn load_from(path: &str) -> Self {
        if Path::new(path).exists() {
            let content = fs::read_to_string(path).unwrap_or_default();
            serde_yaml::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        self.save_to(CLIENT_CONFIG_FILE)
    }

    pub fn save_to(&self, path: &str) -> std::io::Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        let yaml = serde_yaml::to_string(self).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;
        fs::write(path, yaml)
    }

    pub fn add_client(&mut self, entry: ClientEntry) {
        self.clients.insert(entry.client_id.clone(), entry);
    }

    pub fn get_client(&self, client_id: &str) -> Option<&ClientEntry> {
        self.clients.get(client_id)
    }
}

/// Thread-safe key store manager
#[derive(Clone)]
pub struct KeyStoreManager {
    server_keys: Arc<RwLock<ServerKeysStore>>,
    client_config: Arc<RwLock<ClientConfigStore>>,
}

impl KeyStoreManager {
    pub fn new() -> Self {
        Self {
            server_keys: Arc::new(RwLock::new(ServerKeysStore::load())),
            client_config: Arc::new(RwLock::new(ClientConfigStore::load())),
        }
    }

    /// Generate a new server keypair for a client
    pub fn generate_server_key_for_client(&self, client_id: &str) -> ServerKeyEntry {
        let entry = ServerKeyEntry::generate(client_id);
        {
            let mut store = self.server_keys.write().unwrap();
            store.add_key(entry.clone());
            let _ = store.save();
        }
        entry
    }

    /// Get server key for a client
    pub fn get_server_key(&self, client_id: &str) -> Option<ServerKeyEntry> {
        let store = self.server_keys.read().unwrap();
        store.get_key(client_id).cloned()
    }

    /// Register a client with their public key
    pub fn register_client(&self, client_id: &str, client_public_key: &str) -> Option<ClientEntry> {
        // Ensure server key exists for this client
        let server_key = self.get_server_key(client_id)?;
        
        let entry = ClientEntry {
            client_id: client_id.to_string(),
            client_public_key: client_public_key.to_string(),
            server_key_id: client_id.to_string(),
            registered_at: chrono::Utc::now().to_rfc3339(),
            last_seen: Some(chrono::Utc::now().to_rfc3339()),
        };

        {
            let mut store = self.client_config.write().unwrap();
            store.add_client(entry.clone());
            let _ = store.save();
        }

        Some(entry)
    }

    /// Get client configuration
    pub fn get_client(&self, client_id: &str) -> Option<ClientEntry> {
        let store = self.client_config.read().unwrap();
        store.get_client(client_id).cloned()
    }

    /// Derive shared secret for a client
    pub fn derive_shared_secret(&self, client_id: &str) -> Option<[u8; 32]> {
        let server_key = self.get_server_key(client_id)?;
        let client = self.get_client(client_id)?;
        server_key.derive_shared_secret(&client.client_public_key)
    }

    /// List all registered clients
    pub fn list_clients(&self) -> Vec<ClientEntry> {
        let store = self.client_config.read().unwrap();
        store.clients.values().cloned().collect()
    }

    /// List all server keys
    pub fn list_server_keys(&self) -> Vec<(String, String)> {
        let store = self.server_keys.read().unwrap();
        store.keys.iter()
            .map(|(id, k)| (id.clone(), k.public_key.clone()))
            .collect()
    }
}

impl Default for KeyStoreManager {
    fn default() -> Self {
        Self::new()
    }
}
