//! Authentication endpoints

use crate::services::AppState;
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct JoinResponse {
    pub session_id: String,
    pub api_key: String,
    pub expires_at: String,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub valid: bool,
    pub session_id: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub api_key: String,
}

#[derive(Serialize)]
pub struct LogoutResponse {
    pub success: bool,
}

/// Create a new session and return API key
pub async fn join(State(state): State<AppState>) -> Json<JoinResponse> {
    let session = state.sessions.create(state.config.session_ttl_secs);

    Json(JoinResponse {
        session_id: session.id.to_string(),
        api_key: session.api_key,
        expires_at: session.expires_at.to_rfc3339(),
    })
}

/// Verify an API key is valid
pub async fn verify(
    State(state): State<AppState>,
    Json(req): Json<AuthRequest>,
) -> Result<Json<VerifyResponse>, StatusCode> {
    match state.sessions.validate(&req.api_key) {
        Some(session) => Ok(Json(VerifyResponse {
            valid: true,
            session_id: Some(session.id.to_string()),
            expires_at: Some(session.expires_at.to_rfc3339()),
        })),
        None => Ok(Json(VerifyResponse {
            valid: false,
            session_id: None,
            expires_at: None,
        })),
    }
}

/// Logout and invalidate session
pub async fn logout(
    State(state): State<AppState>,
    Json(req): Json<AuthRequest>,
) -> Json<LogoutResponse> {
    let success = state.sessions.revoke(&req.api_key);
    Json(LogoutResponse { success })
}
