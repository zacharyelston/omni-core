//! Server registry for federation.
//!
//! Manages known servers for server-to-server communication.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};

/// Server entry in the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerEntry {
    pub server_id: String,
    pub name: String,
    pub public_url: String,
    pub public_key: String,
    pub is_public: bool,
    pub is_authenticated: bool,
    pub discovered_at: chrono::DateTime<chrono::Utc>,
    pub last_seen: Option<chrono::DateTime<chrono::Utc>>,
}

/// Public server info for sync responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicServerInfo {
    pub server_id: String,
    pub name: String,
    pub public_url: String,
    pub public_key: String,
}

impl From<&ServerEntry> for PublicServerInfo {
    fn from(entry: &ServerEntry) -> Self {
        Self {
            server_id: entry.server_id.clone(),
            name: entry.name.clone(),
            public_url: entry.public_url.clone(),
            public_key: entry.public_key.clone(),
        }
    }
}

/// Server registry for managing known servers
#[derive(Clone)]
pub struct ServerRegistry {
    servers: Arc<RwLock<HashMap<String, ServerEntry>>>,
    data_dir: String,
}

impl ServerRegistry {
    /// Create a new server registry
    pub fn new() -> Self {
        Self::with_data_dir("data/servers.d")
    }

    /// Create with custom data directory
    pub fn with_data_dir(data_dir: &str) -> Self {
        let registry = Self {
            servers: Arc::new(RwLock::new(HashMap::new())),
            data_dir: data_dir.to_string(),
        };
        registry.load_from_disk();
        registry
    }

    /// Register a new server
    pub fn register(&self, entry: ServerEntry) {
        let server_id = entry.server_id.clone();
        self.servers.write().unwrap().insert(server_id, entry);
        self.save_to_disk();
    }

    /// Get a server by ID
    pub fn get(&self, server_id: &str) -> Option<ServerEntry> {
        self.servers.read().unwrap().get(server_id).cloned()
    }

    /// Get a server by public URL
    pub fn get_by_url(&self, url: &str) -> Option<ServerEntry> {
        self.servers
            .read()
            .unwrap()
            .values()
            .find(|s| s.public_url == url)
            .cloned()
    }

    /// List all servers
    pub fn list_all(&self) -> Vec<ServerEntry> {
        self.servers.read().unwrap().values().cloned().collect()
    }

    /// List public servers
    pub fn list_public(&self) -> Vec<ServerEntry> {
        self.servers
            .read()
            .unwrap()
            .values()
            .filter(|s| s.is_public)
            .cloned()
            .collect()
    }

    /// List authenticated servers
    pub fn list_authenticated(&self) -> Vec<ServerEntry> {
        self.servers
            .read()
            .unwrap()
            .values()
            .filter(|s| s.is_authenticated)
            .cloned()
            .collect()
    }

    /// Update last seen timestamp
    pub fn update_last_seen(&self, server_id: &str) {
        if let Some(entry) = self.servers.write().unwrap().get_mut(server_id) {
            entry.last_seen = Some(chrono::Utc::now());
        }
        self.save_to_disk();
    }

    /// Mark server as authenticated
    pub fn mark_authenticated(&self, server_id: &str, authenticated: bool) {
        if let Some(entry) = self.servers.write().unwrap().get_mut(server_id) {
            entry.is_authenticated = authenticated;
        }
        self.save_to_disk();
    }

    /// Remove a server
    pub fn remove(&self, server_id: &str) {
        self.servers.write().unwrap().remove(server_id);
        self.save_to_disk();
    }

    /// Merge servers from another registry (for sync)
    pub fn merge_from(&self, servers: Vec<PublicServerInfo>) {
        let mut registry = self.servers.write().unwrap();
        for info in servers {
            if !registry.contains_key(&info.server_id) {
                registry.insert(
                    info.server_id.clone(),
                    ServerEntry {
                        server_id: info.server_id,
                        name: info.name,
                        public_url: info.public_url,
                        public_key: info.public_key,
                        is_public: true,
                        is_authenticated: false,
                        discovered_at: chrono::Utc::now(),
                        last_seen: None,
                    },
                );
            }
        }
        drop(registry);
        self.save_to_disk();
    }

    /// Load from disk
    fn load_from_disk(&self) {
        if !Path::new(&self.data_dir).exists() {
            return;
        }

        let entries = fs::read_dir(&self.data_dir).ok();
        if let Some(entries) = entries {
            for entry in entries.flatten() {
                if entry
                    .path()
                    .extension()
                    .map(|e| e == "yaml")
                    .unwrap_or(false)
                {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(server) = serde_yaml::from_str::<ServerEntry>(&content) {
                            self.servers
                                .write()
                                .unwrap()
                                .insert(server.server_id.clone(), server);
                        }
                    }
                }
            }
        }
    }

    /// Save to disk
    fn save_to_disk(&self) {
        let _ = fs::create_dir_all(&self.data_dir);

        for (id, entry) in self.servers.read().unwrap().iter() {
            let path = format!("{}/{}.yaml", self.data_dir, id);
            if let Ok(content) = serde_yaml::to_string(entry) {
                let _ = fs::write(&path, content);
            }
        }
    }
}

impl Default for ServerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_register_and_get() {
        let temp_dir = TempDir::new().unwrap();
        let registry = ServerRegistry::with_data_dir(temp_dir.path().to_str().unwrap());

        let entry = ServerEntry {
            server_id: "server-1".to_string(),
            name: "Test Server".to_string(),
            public_url: "https://example.com".to_string(),
            public_key: "abc123".to_string(),
            is_public: true,
            is_authenticated: false,
            discovered_at: chrono::Utc::now(),
            last_seen: None,
        };

        registry.register(entry.clone());

        let retrieved = registry.get("server-1").unwrap();
        assert_eq!(retrieved.name, "Test Server");
    }

    #[test]
    fn test_list_public() {
        let temp_dir = TempDir::new().unwrap();
        let registry = ServerRegistry::with_data_dir(temp_dir.path().to_str().unwrap());

        registry.register(ServerEntry {
            server_id: "public-1".to_string(),
            name: "Public".to_string(),
            public_url: "https://public.com".to_string(),
            public_key: "key1".to_string(),
            is_public: true,
            is_authenticated: false,
            discovered_at: chrono::Utc::now(),
            last_seen: None,
        });

        registry.register(ServerEntry {
            server_id: "private-1".to_string(),
            name: "Private".to_string(),
            public_url: "https://private.com".to_string(),
            public_key: "key2".to_string(),
            is_public: false,
            is_authenticated: false,
            discovered_at: chrono::Utc::now(),
            last_seen: None,
        });

        let public = registry.list_public();
        assert_eq!(public.len(), 1);
        assert_eq!(public[0].server_id, "public-1");
    }
}
