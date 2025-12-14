//! Server synchronization service
//!
//! Background task that periodically syncs public server lists from known authenticated servers.

use crate::services::{ServerEntry, ServerRegistry};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{error, info, warn};

/// Configuration for the sync service
#[derive(Clone)]
pub struct SyncConfig {
    /// How often to sync (in seconds)
    pub interval_secs: u64,
    /// Maximum servers to fetch per sync
    pub max_servers_per_sync: usize,
    /// Request timeout in seconds
    pub timeout_secs: u64,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            interval_secs: 3600, // 1 hour
            max_servers_per_sync: 100,
            timeout_secs: 30,
        }
    }
}

/// Server sync service
pub struct SyncService {
    registry: ServerRegistry,
    config: SyncConfig,
    our_server_id: String,
    our_public_key: String,
}

impl SyncService {
    pub fn new(registry: ServerRegistry, our_server_id: String, our_public_key: String) -> Self {
        Self {
            registry,
            config: SyncConfig::default(),
            our_server_id,
            our_public_key,
        }
    }

    pub fn with_config(mut self, config: SyncConfig) -> Self {
        self.config = config;
        self
    }

    /// Start the background sync task
    pub fn start(self: Arc<Self>) {
        let service = self.clone();
        tokio::spawn(async move {
            service.run_sync_loop().await;
        });
    }

    /// Run the sync loop
    async fn run_sync_loop(&self) {
        let mut ticker = interval(Duration::from_secs(self.config.interval_secs));

        info!(
            "Starting server sync service (interval: {}s)",
            self.config.interval_secs
        );

        loop {
            ticker.tick().await;

            if let Err(e) = self.sync_all_servers().await {
                error!("Sync failed: {}", e);
            }
        }
    }

    /// Sync with all authenticated servers
    async fn sync_all_servers(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let authenticated_servers = self.registry.list_authenticated();

        if authenticated_servers.is_empty() {
            info!("No authenticated servers to sync with");
            return Ok(());
        }

        info!(
            "Starting sync with {} authenticated servers",
            authenticated_servers.len()
        );

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(self.config.timeout_secs))
            .build()?;

        let mut total_added = 0;

        for server in authenticated_servers {
            match self.sync_with_server(&client, &server).await {
                Ok(added) => {
                    total_added += added;
                    info!(
                        "Synced with {} ({}): {} new servers",
                        server.name, server.public_url, added
                    );
                }
                Err(e) => {
                    warn!(
                        "Failed to sync with {} ({}): {}",
                        server.name, server.public_url, e
                    );
                }
            }
        }

        info!("Sync complete: {} new servers discovered", total_added);
        Ok(())
    }

    /// Sync with a single server
    async fn sync_with_server(
        &self,
        client: &reqwest::Client,
        server: &ServerEntry,
    ) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/api/v1/servers/sync", server.public_url);

        let response = client
            .post(&url)
            .json(&serde_json::json!({
                "requesting_server_id": self.our_server_id,
                "requesting_server_key": self.our_public_key,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Server returned {}", response.status()).into());
        }

        let data: SyncResponse = response.json().await?;

        // Merge the servers we received
        let added = self.registry.merge_from(data.servers);

        // Mark that we synced with this server
        let _ = self.registry.mark_synced(&server.server_id);

        Ok(added)
    }
}

#[derive(serde::Deserialize)]
struct SyncResponse {
    servers: Vec<ServerEntry>,
    #[allow(dead_code)]
    total: usize,
}

/// Spawn the sync service as a background task
pub fn spawn_sync_service(
    registry: ServerRegistry,
    our_server_id: String,
    our_public_key: String,
    interval_secs: Option<u64>,
) {
    let mut config = SyncConfig::default();
    if let Some(secs) = interval_secs {
        config.interval_secs = secs;
    }

    let service =
        Arc::new(SyncService::new(registry, our_server_id, our_public_key).with_config(config));

    service.start();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_config_default() {
        let config = SyncConfig::default();
        assert_eq!(config.interval_secs, 3600);
        assert_eq!(config.max_servers_per_sync, 100);
        assert_eq!(config.timeout_secs, 30);
    }
}
