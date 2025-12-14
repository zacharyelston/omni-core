//! Server registry for server-to-server federation
//!
//! Servers can register with each other and share their known server lists.
//! This creates a DNS-like discovery mechanism for the Omni Core network.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

const SERVERS_DIR: &str = "data/servers.d";

/// A known server entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerEntry {
    /// Unique server identifier (public key hash or UUID)
    pub server_id: String,
    /// Human-readable server name
    pub name: String,
    /// Server description
    pub description: Option<String>,
    /// Public URL for the server
    pub public_url: String,
    /// Server's public key (hex-encoded)
    pub public_key: String,
    /// Whether this server is publicly listed
    pub is_public: bool,
    /// Whether we are authenticated with this server
    pub is_authenticated: bool,
    /// When we first discovered this server
    pub discovered_at: String,
    /// Last time we successfully contacted this server
    pub last_seen: Option<String>,
    /// Last time we synced server list from this server
    pub last_sync: Option<String>,
    /// Server version
    pub version: Option<String>,
    /// Trust level (0-100, higher = more trusted)
    pub trust_level: u8,
}

impl ServerEntry {
    pub fn new(server_id: &str, name: &str, public_url: &str, public_key: &str) -> Self {
        Self {
            server_id: server_id.to_string(),
            name: name.to_string(),
            description: None,
            public_url: public_url.to_string(),
            public_key: public_key.to_string(),
            is_public: true,
            is_authenticated: false,
            discovered_at: chrono::Utc::now().to_rfc3339(),
            last_seen: None,
            last_sync: None,
            version: None,
            trust_level: 50,
        }
    }

    /// Get the file path for this server's config
    fn file_path(&self) -> PathBuf {
        PathBuf::from(SERVERS_DIR).join(format!("{}.yaml", self.server_id))
    }

    /// Save this server entry to its own file
    pub fn save(&self) -> std::io::Result<()> {
        let path = self.file_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let yaml = serde_yaml::to_string(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(path, yaml)
    }

    /// Load a server entry from file
    pub fn load(server_id: &str) -> Option<Self> {
        let path = PathBuf::from(SERVERS_DIR).join(format!("{}.yaml", server_id));
        if path.exists() {
            let content = fs::read_to_string(path).ok()?;
            serde_yaml::from_str(&content).ok()
        } else {
            None
        }
    }

    /// Delete this server's config file
    pub fn delete(&self) -> std::io::Result<()> {
        let path = self.file_path();
        if path.exists() {
            fs::remove_file(path)
        } else {
            Ok(())
        }
    }
}

/// Server registry manager
#[derive(Clone)]
pub struct ServerRegistry {
    servers: Arc<RwLock<HashMap<String, ServerEntry>>>,
}

impl ServerRegistry {
    pub fn new() -> Self {
        let registry = Self {
            servers: Arc::new(RwLock::new(HashMap::new())),
        };
        registry.load_all();
        registry
    }

    /// Load all server configs from servers.d/
    fn load_all(&self) {
        let dir = Path::new(SERVERS_DIR);
        if !dir.exists() {
            let _ = fs::create_dir_all(dir);
            return;
        }

        if let Ok(entries) = fs::read_dir(dir) {
            let mut servers = self.servers.write().unwrap();
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "yaml") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(server) = serde_yaml::from_str::<ServerEntry>(&content) {
                            servers.insert(server.server_id.clone(), server);
                        }
                    }
                }
            }
        }
    }

    /// Register a new server
    pub fn register(&self, entry: ServerEntry) -> std::io::Result<()> {
        entry.save()?;
        let mut servers = self.servers.write().unwrap();
        servers.insert(entry.server_id.clone(), entry);
        Ok(())
    }

    /// Update an existing server
    pub fn update(&self, entry: ServerEntry) -> std::io::Result<()> {
        self.register(entry)
    }

    /// Get a server by ID
    pub fn get(&self, server_id: &str) -> Option<ServerEntry> {
        let servers = self.servers.read().unwrap();
        servers.get(server_id).cloned()
    }

    /// Remove a server
    pub fn remove(&self, server_id: &str) -> std::io::Result<()> {
        let mut servers = self.servers.write().unwrap();
        if let Some(entry) = servers.remove(server_id) {
            entry.delete()?;
        }
        Ok(())
    }

    /// List all known servers
    pub fn list_all(&self) -> Vec<ServerEntry> {
        let servers = self.servers.read().unwrap();
        servers.values().cloned().collect()
    }

    /// List only public servers (for sharing with other servers)
    pub fn list_public(&self) -> Vec<ServerEntry> {
        let servers = self.servers.read().unwrap();
        servers.values().filter(|s| s.is_public).cloned().collect()
    }

    /// List authenticated servers
    pub fn list_authenticated(&self) -> Vec<ServerEntry> {
        let servers = self.servers.read().unwrap();
        servers
            .values()
            .filter(|s| s.is_authenticated)
            .cloned()
            .collect()
    }

    /// Mark a server as authenticated
    pub fn set_authenticated(&self, server_id: &str, authenticated: bool) -> std::io::Result<()> {
        let mut servers = self.servers.write().unwrap();
        if let Some(entry) = servers.get_mut(server_id) {
            entry.is_authenticated = authenticated;
            entry.last_seen = Some(chrono::Utc::now().to_rfc3339());
            entry.save()?;
        }
        Ok(())
    }

    /// Update last seen timestamp
    pub fn touch(&self, server_id: &str) -> std::io::Result<()> {
        let mut servers = self.servers.write().unwrap();
        if let Some(entry) = servers.get_mut(server_id) {
            entry.last_seen = Some(chrono::Utc::now().to_rfc3339());
            entry.save()?;
        }
        Ok(())
    }

    /// Update last sync timestamp
    pub fn mark_synced(&self, server_id: &str) -> std::io::Result<()> {
        let mut servers = self.servers.write().unwrap();
        if let Some(entry) = servers.get_mut(server_id) {
            entry.last_sync = Some(chrono::Utc::now().to_rfc3339());
            entry.save()?;
        }
        Ok(())
    }

    /// Merge servers from another server's list
    /// Only adds new servers, doesn't overwrite existing ones
    pub fn merge_from(&self, servers: Vec<ServerEntry>) -> usize {
        let mut added = 0;
        let mut current = self.servers.write().unwrap();

        for server in servers {
            if !current.contains_key(&server.server_id) {
                if server.save().is_ok() {
                    current.insert(server.server_id.clone(), server);
                    added += 1;
                }
            }
        }

        added
    }

    /// Get server count
    pub fn count(&self) -> usize {
        let servers = self.servers.read().unwrap();
        servers.len()
    }

    /// Get public server count
    pub fn count_public(&self) -> usize {
        let servers = self.servers.read().unwrap();
        servers.values().filter(|s| s.is_public).count()
    }

    /// Get authenticated server count
    pub fn count_authenticated(&self) -> usize {
        let servers = self.servers.read().unwrap();
        servers.values().filter(|s| s.is_authenticated).count()
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

    fn setup_test_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn test_server_entry_save_load() {
        let temp = setup_test_dir();
        let path = temp.path().join("test-server.yaml");

        let entry = ServerEntry::new(
            "test-server-123",
            "Test Server",
            "https://test.example.com",
            "abc123pubkey",
        );

        let yaml = serde_yaml::to_string(&entry).unwrap();
        fs::write(&path, &yaml).unwrap();

        let loaded: ServerEntry =
            serde_yaml::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(loaded.server_id, "test-server-123");
        assert_eq!(loaded.name, "Test Server");
        assert_eq!(loaded.public_url, "https://test.example.com");
    }

    #[test]
    fn test_server_registry_operations() {
        let registry = ServerRegistry {
            servers: Arc::new(RwLock::new(HashMap::new())),
        };

        let entry = ServerEntry::new(
            "server-1",
            "Server One",
            "https://one.example.com",
            "pubkey1",
        );

        // Add to in-memory only for test
        {
            let mut servers = registry.servers.write().unwrap();
            servers.insert(entry.server_id.clone(), entry.clone());
        }

        assert_eq!(registry.count(), 1);

        let retrieved = registry.get("server-1").unwrap();
        assert_eq!(retrieved.name, "Server One");
    }
}
