//! Settings API endpoints
//!
//! Provides endpoints for reading and updating server configuration.

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::services::{
    AppState, AuthSettings, FederationSettings, NetworkSettings, ServerConfig, ServerSettings,
};

/// Get all settings
pub async fn get_settings(State(state): State<AppState>) -> Json<ServerConfig> {
    Json(state.server_config.get())
}

/// Get server settings
pub async fn get_server_settings(State(state): State<AppState>) -> Json<ServerSettings> {
    Json(state.server_config.get_server())
}

/// Update server settings
pub async fn update_server_settings(
    State(state): State<AppState>,
    Json(settings): Json<ServerSettings>,
) -> Result<Json<SettingsResponse>, (StatusCode, Json<ErrorResponse>)> {
    state.server_config.update_server(settings).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to save settings: {}", e),
            }),
        )
    })?;

    Ok(Json(SettingsResponse {
        success: true,
        message: "Server settings updated".to_string(),
    }))
}

/// Get network settings
pub async fn get_network_settings(State(state): State<AppState>) -> Json<NetworkSettings> {
    Json(state.server_config.get_network())
}

/// Update network settings
pub async fn update_network_settings(
    State(state): State<AppState>,
    Json(settings): Json<NetworkSettings>,
) -> Result<Json<SettingsResponse>, (StatusCode, Json<ErrorResponse>)> {
    state.server_config.update_network(settings).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to save settings: {}", e),
            }),
        )
    })?;

    Ok(Json(SettingsResponse {
        success: true,
        message: "Network settings updated. Restart required for changes to take effect."
            .to_string(),
    }))
}

/// Get auth settings
pub async fn get_auth_settings(State(state): State<AppState>) -> Json<AuthSettings> {
    Json(state.server_config.get_auth())
}

/// Update auth settings
pub async fn update_auth_settings(
    State(state): State<AppState>,
    Json(settings): Json<AuthSettings>,
) -> Result<Json<SettingsResponse>, (StatusCode, Json<ErrorResponse>)> {
    state.server_config.update_auth(settings).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to save settings: {}", e),
            }),
        )
    })?;

    Ok(Json(SettingsResponse {
        success: true,
        message: "Auth settings updated".to_string(),
    }))
}

/// Get federation settings
pub async fn get_federation_settings(State(state): State<AppState>) -> Json<FederationSettings> {
    Json(state.server_config.get_federation())
}

/// Update federation settings
pub async fn update_federation_settings(
    State(state): State<AppState>,
    Json(settings): Json<FederationSettings>,
) -> Result<Json<SettingsResponse>, (StatusCode, Json<ErrorResponse>)> {
    state
        .server_config
        .update_federation(settings)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to save settings: {}", e),
                }),
            )
        })?;

    Ok(Json(SettingsResponse {
        success: true,
        message: "Federation settings updated".to_string(),
    }))
}

/// Update all settings at once
pub async fn update_all_settings(
    State(state): State<AppState>,
    Json(config): Json<ServerConfig>,
) -> Result<Json<SettingsResponse>, (StatusCode, Json<ErrorResponse>)> {
    state.server_config.update(config).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to save settings: {}", e),
            }),
        )
    })?;

    Ok(Json(SettingsResponse {
        success: true,
        message: "All settings updated".to_string(),
    }))
}

#[derive(Serialize)]
pub struct SettingsResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
