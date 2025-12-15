//! Background sync service for server federation.

use crate::registry::{PublicServerInfo, ServerRegistry};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info, warn};

/// Sync service configuration
#[derive(Debug, Clone)]
pub struct SyncConfig {
    /// Interval between sync attempts
    pub interval_secs: u64,
    /// Maximum servers to fetch per sync
    pub max_servers: usize,
    /// Request timeout
    pub timeout_secs: u64,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            interval_secs: 3600, // 1 hour
            max_servers: 100,
            timeout_secs: 30,
        }
    }
}

/// Sync request sent to other servers
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRequest {
    pub requesting_server_id: String,
    pub requesting_server_key: String,
}

/// Sync response from other servers
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResponse {
    pub servers: Vec<PublicServerInfo>,
    pub total: usize,
}

/// Sync service for background server discovery
pub struct SyncService {
    registry: ServerRegistry,
    config: SyncConfig,
    server_id: String,
    server_public_key: String,
    client: reqwest::Client,
}

impl SyncService {
    /// Create a new sync service
    pub fn new(
        registry: ServerRegistry,
        server_id: String,
        server_public_key: String,
        config: Option<SyncConfig>,
    ) -> Self {
        let config = config.unwrap_or_default();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            registry,
            config,
            server_id,
            server_public_key,
            client,
        }
    }

    /// Run the sync loop
    pub async fn run(&self) {
        let mut interval = time::interval(Duration::from_secs(self.config.interval_secs));

        loop {
            interval.tick().await;
            self.sync_once().await;
        }
    }

    /// Perform a single sync
    pub async fn sync_once(&self) {
        info!("Starting server sync");

        let authenticated_servers = self.registry.list_authenticated();
        if authenticated_servers.is_empty() {
            debug!("No authenticated servers to sync with");
            return;
        }

        for server in authenticated_servers {
            match self.sync_with_server(&server.public_url).await {
                Ok(count) => {
                    info!("Synced {} servers from {}", count, server.name);
                    self.registry.update_last_seen(&server.server_id);
                }
                Err(e) => {
                    warn!("Failed to sync with {}: {}", server.name, e);
                }
            }
        }
    }

    /// Sync with a specific server
    async fn sync_with_server(&self, base_url: &str) -> Result<usize, SyncError> {
        let url = format!("{}/api/v1/servers/sync", base_url);

        let request = SyncRequest {
            requesting_server_id: self.server_id.clone(),
            requesting_server_key: self.server_public_key.clone(),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| SyncError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(SyncError::ServerError(response.status().to_string()));
        }

        let sync_response: SyncResponse = response
            .json()
            .await
            .map_err(|e| SyncError::ParseError(e.to_string()))?;

        // Merge new servers into registry
        self.registry.merge_from(sync_response.servers.clone());

        Ok(sync_response.servers.len())
    }
}

/// Spawn the sync service as a background task
pub fn spawn_sync_service(
    registry: ServerRegistry,
    server_id: String,
    server_public_key: String,
    interval_secs: Option<u64>,
) {
    let config = interval_secs.map(|secs| SyncConfig {
        interval_secs: secs,
        ..Default::default()
    });

    let service = SyncService::new(registry, server_id, server_public_key, config);

    tokio::spawn(async move {
        service.run().await;
    });
}

/// Sync errors
#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Server error: {0}")]
    ServerError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}
