//! Server configuration service
//!
//! Manages server settings stored in config.d/server-config.yaml

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};

const CONFIG_FILE: &str = "data/config.d/server-config.yaml";

/// Server identity settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: "Omni Core Server".to_string(),
            description: "An Omni Core authentication server".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Network settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    pub host: String,
    pub port: u16,
    pub public_url: String,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            public_url: String::new(),
        }
    }
}

/// Authentication settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSettings {
    pub session_ttl_secs: u64,
    pub admin_session_multiplier: u32,
}

impl Default for AuthSettings {
    fn default() -> Self {
        Self {
            session_ttl_secs: 3600,
            admin_session_multiplier: 24,
        }
    }
}

/// Federation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationSettings {
    pub enabled: bool,
    pub public: bool,
    pub sync_interval_secs: u64,
    pub max_known_servers: usize,
}

impl Default for FederationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            public: true,
            sync_interval_secs: 3600,
            max_known_servers: 1000,
        }
    }
}

/// Complete server configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerConfig {
    pub server: ServerSettings,
    pub network: NetworkSettings,
    pub auth: AuthSettings,
    pub federation: FederationSettings,
}

impl ServerConfig {
    /// Load config from file or create default
    pub fn load() -> Self {
        if Path::new(CONFIG_FILE).exists() {
            if let Ok(content) = fs::read_to_string(CONFIG_FILE) {
                if let Ok(config) = serde_yaml::from_str(&content) {
                    return config;
                }
            }
        }
        let config = Self::default();
        let _ = config.save();
        config
    }

    /// Save config to file
    pub fn save(&self) -> std::io::Result<()> {
        if let Some(parent) = Path::new(CONFIG_FILE).parent() {
            fs::create_dir_all(parent)?;
        }
        let yaml = serde_yaml::to_string(self).map_err(std::io::Error::other)?;
        fs::write(CONFIG_FILE, yaml)
    }
}

/// Thread-safe config manager
#[derive(Clone)]
pub struct ConfigManager {
    config: Arc<RwLock<ServerConfig>>,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(ServerConfig::load())),
        }
    }

    /// Get current config
    pub fn get(&self) -> ServerConfig {
        self.config.read().unwrap().clone()
    }

    /// Get server settings
    pub fn get_server(&self) -> ServerSettings {
        self.config.read().unwrap().server.clone()
    }

    /// Get network settings
    pub fn get_network(&self) -> NetworkSettings {
        self.config.read().unwrap().network.clone()
    }

    /// Get auth settings
    pub fn get_auth(&self) -> AuthSettings {
        self.config.read().unwrap().auth.clone()
    }

    /// Get federation settings
    pub fn get_federation(&self) -> FederationSettings {
        self.config.read().unwrap().federation.clone()
    }

    /// Update server settings
    pub fn update_server(&self, settings: ServerSettings) -> std::io::Result<()> {
        let mut config = self.config.write().unwrap();
        config.server = settings;
        config.save()
    }

    /// Update network settings
    pub fn update_network(&self, settings: NetworkSettings) -> std::io::Result<()> {
        let mut config = self.config.write().unwrap();
        config.network = settings;
        config.save()
    }

    /// Update auth settings
    pub fn update_auth(&self, settings: AuthSettings) -> std::io::Result<()> {
        let mut config = self.config.write().unwrap();
        config.auth = settings;
        config.save()
    }

    /// Update federation settings
    pub fn update_federation(&self, settings: FederationSettings) -> std::io::Result<()> {
        let mut config = self.config.write().unwrap();
        config.federation = settings;
        config.save()
    }

    /// Update entire config
    pub fn update(&self, new_config: ServerConfig) -> std::io::Result<()> {
        let mut config = self.config.write().unwrap();
        *config = new_config;
        config.save()
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ServerConfig::default();
        assert_eq!(config.server.name, "Omni Core Server");
        assert_eq!(config.network.port, 8080);
        assert_eq!(config.auth.session_ttl_secs, 3600);
        assert!(config.federation.enabled);
    }
}
