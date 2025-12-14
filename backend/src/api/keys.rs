//! Key exchange endpoints

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::services::{parse_public_key, AppState, EncryptedMessage};

/// Response with server's public key
#[derive(Serialize)]
pub struct PublicKeyResponse {
    pub public_key: String,
}

/// Request to establish encrypted session
#[derive(Deserialize)]
pub struct KeyExchangeRequest {
    /// Client's X25519 public key (hex-encoded)
    pub client_public_key: String,
}

/// Response with session details after key exchange
#[derive(Serialize)]
pub struct KeyExchangeResponse {
    pub session_id: String,
    pub api_key: String,
    pub expires_at: String,
    pub server_public_key: String,
}

/// Request to send encrypted message
#[derive(Deserialize)]
pub struct EncryptedRequest {
    /// Client's public key for this message
    pub client_public_key: String,
    /// Encrypted payload
    pub payload: EncryptedMessage,
}

/// Response with encrypted data
#[derive(Serialize)]
pub struct EncryptedResponse {
    pub payload: EncryptedMessage,
}

/// Get server's public key for key exchange
pub async fn get_public_key(
    State(state): State<AppState>,
) -> Json<PublicKeyResponse> {
    Json(PublicKeyResponse {
        public_key: state.server_keypair.public_key_hex(),
    })
}

/// Perform key exchange and create encrypted session
pub async fn key_exchange(
    State(state): State<AppState>,
    Json(req): Json<KeyExchangeRequest>,
) -> Result<Json<KeyExchangeResponse>, (StatusCode, String)> {
    // Parse client's public key
    let client_public = parse_public_key(&req.client_public_key)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Derive shared secret (not returned, used for encryption)
    let _shared_secret = state.server_keypair.derive_shared_secret(&client_public);

    // Create session
    let session = state.sessions.create(state.config.session_ttl_secs);

    Ok(Json(KeyExchangeResponse {
        session_id: session.id.to_string(),
        api_key: session.api_key,
        expires_at: session.expires_at.to_rfc3339(),
        server_public_key: state.server_keypair.public_key_hex(),
    }))
}

/// Send encrypted message to server
pub async fn send_encrypted(
    State(state): State<AppState>,
    Json(req): Json<EncryptedRequest>,
) -> Result<Json<EncryptedResponse>, (StatusCode, String)> {
    // Parse client's public key
    let client_public = parse_public_key(&req.client_public_key)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Derive shared secret
    let shared_secret = state.server_keypair.derive_shared_secret(&client_public);

    // Decrypt the incoming message
    let plaintext = req.payload.decrypt(&shared_secret)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Process the message (echo back for now)
    let response_text = format!("Received: {}", String::from_utf8_lossy(&plaintext));

    // Encrypt the response
    let encrypted_response = EncryptedMessage::encrypt(response_text.as_bytes(), &shared_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(EncryptedResponse {
        payload: encrypted_response,
    }))
}
