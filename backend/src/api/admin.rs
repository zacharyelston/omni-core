//! Admin authentication endpoints

use crate::services::AppState;
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

/// Server info response (public, no auth required)
#[derive(Serialize)]
pub struct ServerInfoResponse {
    pub server_public_key: String,
    pub server_name: String,
    pub version: String,
}

/// Admin login request
#[derive(Deserialize)]
pub struct AdminLoginRequest {
    pub admin_key: String,
}

/// Admin login response
#[derive(Serialize)]
pub struct AdminLoginResponse {
    pub authenticated: bool,
    pub message: String,
}

/// Admin dashboard data (requires auth)
#[derive(Serialize)]
pub struct AdminDashboardResponse {
    pub total_clients: usize,
    pub total_server_keys: usize,
    pub server_public_key: String,
}

/// Get server public info (for QR code display)
pub async fn get_server_info(State(state): State<AppState>) -> Json<ServerInfoResponse> {
    Json(ServerInfoResponse {
        server_public_key: state.admin.get_server_public_key(),
        server_name: "Omni Core Server".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Admin login
pub async fn admin_login(
    State(state): State<AppState>,
    Json(req): Json<AdminLoginRequest>,
) -> Result<Json<AdminLoginResponse>, (StatusCode, String)> {
    if state.admin.verify(&req.admin_key) {
        // Create admin session
        let session = state.sessions.create(state.config.session_ttl_secs * 24); // 24x longer for admin

        Ok(Json(AdminLoginResponse {
            authenticated: true,
            message: format!("Admin session created. API key: {}", session.api_key),
        }))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid admin key".to_string()))
    }
}

/// Get admin dashboard (requires valid admin session)
pub async fn admin_dashboard(State(state): State<AppState>) -> Json<AdminDashboardResponse> {
    let clients = state.keystore.list_clients();
    let keys = state.keystore.list_server_keys();

    Json(AdminDashboardResponse {
        total_clients: clients.len(),
        total_server_keys: keys.len(),
        server_public_key: state.admin.get_server_public_key(),
    })
}
