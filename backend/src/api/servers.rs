//! Server federation API endpoints
//!
//! Endpoints for server-to-server communication and discovery.

use crate::services::{AppState, ServerEntry};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

/// Public server info for discovery
#[derive(Serialize)]
pub struct PublicServerInfo {
    pub server_id: String,
    pub name: String,
    pub description: Option<String>,
    pub public_url: String,
    pub public_key: String,
    pub version: String,
}

/// Server list response
#[derive(Serialize)]
pub struct ServerListResponse {
    pub servers: Vec<PublicServerInfo>,
    pub total: usize,
}

/// Register server request
#[derive(Deserialize)]
pub struct RegisterServerRequest {
    pub server_id: String,
    pub name: String,
    pub description: Option<String>,
    pub public_url: String,
    pub public_key: String,
    pub is_public: bool,
}

/// Register server response
#[derive(Serialize)]
pub struct RegisterServerResponse {
    pub success: bool,
    pub message: String,
    pub our_server_id: String,
    pub our_public_key: String,
}

/// Sync request - request to get server list
#[derive(Deserialize)]
pub struct SyncRequest {
    pub requesting_server_id: String,
    pub requesting_server_key: String,
}

/// Get list of public servers (no auth required)
pub async fn list_public_servers(State(state): State<AppState>) -> Json<ServerListResponse> {
    let servers = state.server_registry.list_public();
    let public_servers: Vec<PublicServerInfo> = servers
        .into_iter()
        .map(|s| PublicServerInfo {
            server_id: s.server_id,
            name: s.name,
            description: s.description,
            public_url: s.public_url,
            public_key: s.public_key,
            version: s.version.unwrap_or_default(),
        })
        .collect();

    let total = public_servers.len();
    Json(ServerListResponse {
        servers: public_servers,
        total,
    })
}

/// Register a new server (server-to-server)
pub async fn register_server(
    State(state): State<AppState>,
    Json(req): Json<RegisterServerRequest>,
) -> Result<Json<RegisterServerResponse>, (StatusCode, String)> {
    // Create server entry
    let entry = ServerEntry::new(&req.server_id, &req.name, &req.public_url, &req.public_key);

    // Update with additional fields
    let mut entry = entry;
    entry.description = req.description;
    entry.is_public = req.is_public;

    // Register the server
    state
        .server_registry
        .register(entry)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(RegisterServerResponse {
        success: true,
        message: "Server registered successfully".to_string(),
        our_server_id: state.admin.get_server_id(),
        our_public_key: state.admin.get_server_public_key(),
    }))
}

/// Sync server list from authenticated server
pub async fn sync_servers(
    State(state): State<AppState>,
    Json(req): Json<SyncRequest>,
) -> Result<Json<ServerListResponse>, (StatusCode, String)> {
    // Verify the requesting server is authenticated
    let server = state
        .server_registry
        .get(&req.requesting_server_id)
        .ok_or((StatusCode::UNAUTHORIZED, "Unknown server".to_string()))?;

    if !server.is_authenticated {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Server not authenticated".to_string(),
        ));
    }

    // Return our known public servers
    let servers = state.server_registry.list_public();
    let public_servers: Vec<PublicServerInfo> = servers
        .into_iter()
        .map(|s| PublicServerInfo {
            server_id: s.server_id,
            name: s.name,
            description: s.description,
            public_url: s.public_url,
            public_key: s.public_key,
            version: s.version.unwrap_or_default(),
        })
        .collect();

    // Mark that we synced with this server
    let _ = state.server_registry.mark_synced(&req.requesting_server_id);

    let total = public_servers.len();
    Ok(Json(ServerListResponse {
        servers: public_servers,
        total,
    }))
}

/// Get server statistics (admin only)
#[derive(Serialize)]
pub struct ServerStats {
    pub total_servers: usize,
    pub public_servers: usize,
    pub authenticated_servers: usize,
}

pub async fn server_stats(State(state): State<AppState>) -> Json<ServerStats> {
    Json(ServerStats {
        total_servers: state.server_registry.count(),
        public_servers: state.server_registry.count_public(),
        authenticated_servers: state.server_registry.count_authenticated(),
    })
}

/// List all known servers (admin dashboard)
pub async fn list_all_servers(State(state): State<AppState>) -> Json<Vec<ServerEntry>> {
    Json(state.server_registry.list_all())
}
