//! Admin authentication service

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};

const ADMIN_CONFIG_FILE: &str = "data/config.d/admin.yaml";

/// Admin configuration with generated key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfig {
    /// Unique server identifier
    pub server_id: String,
    /// Admin API key (generated on first run)
    pub admin_key: String,
    /// When the key was generated
    pub created_at: String,
    /// Server's default public key for display
    pub server_public_key: String,
}

impl AdminConfig {
    /// Generate new admin config with random key
    pub fn generate(server_public_key: &str) -> Self {
        use base64::Engine;
        use rand::RngCore;

        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        let admin_key = format!(
            "admin_{}",
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
        );

        // Generate unique server ID from public key hash
        let server_id = format!("srv_{}", &server_public_key[..16]);

        Self {
            server_id,
            admin_key,
            created_at: chrono::Utc::now().to_rfc3339(),
            server_public_key: server_public_key.to_string(),
        }
    }

    /// Load from file or generate new
    pub fn load_or_generate(server_public_key: &str) -> Self {
        if Path::new(ADMIN_CONFIG_FILE).exists() {
            let content = fs::read_to_string(ADMIN_CONFIG_FILE).unwrap_or_default();
            if let Ok(mut config) = serde_yaml::from_str::<AdminConfig>(&content) {
                // Update server public key if changed
                config.server_public_key = server_public_key.to_string();
                return config;
            }
        }

        // Generate new config
        let config = Self::generate(server_public_key);
        let _ = config.save();

        // Log the admin key on first generation
        tracing::warn!("==============================================");
        tracing::warn!("ADMIN KEY GENERATED (save this securely!):");
        tracing::warn!("{}", config.admin_key);
        tracing::warn!("==============================================");

        config
    }

    /// Save to file
    pub fn save(&self) -> std::io::Result<()> {
        if let Some(parent) = Path::new(ADMIN_CONFIG_FILE).parent() {
            fs::create_dir_all(parent)?;
        }
        let yaml = serde_yaml::to_string(self).map_err(std::io::Error::other)?;
        fs::write(ADMIN_CONFIG_FILE, yaml)
    }
}

/// Admin authentication manager
#[derive(Clone)]
pub struct AdminAuth {
    config: Arc<RwLock<AdminConfig>>,
}

impl AdminAuth {
    pub fn new(server_public_key: &str) -> Self {
        let config = AdminConfig::load_or_generate(server_public_key);
        Self {
            config: Arc::new(RwLock::new(config)),
        }
    }

    /// Verify admin key
    pub fn verify(&self, key: &str) -> bool {
        let config = self.config.read().unwrap();
        config.admin_key == key
    }

    /// Get server public key for display
    pub fn get_server_public_key(&self) -> String {
        let config = self.config.read().unwrap();
        config.server_public_key.clone()
    }

    /// Get server ID
    pub fn get_server_id(&self) -> String {
        let config = self.config.read().unwrap();
        config.server_id.clone()
    }

    /// Check if admin key exists (for UI display logic)
    pub fn has_admin_key(&self) -> bool {
        let config = self.config.read().unwrap();
        !config.admin_key.is_empty()
    }
}
