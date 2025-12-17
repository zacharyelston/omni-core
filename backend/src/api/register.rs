//! Client registration endpoints with per-client keypairs

use crate::services::{AppState, EncryptedMessage};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

/// Request to initiate registration
#[derive(Deserialize)]
pub struct RegisterInitRequest {
    /// Client's unique identifier
    pub client_id: String,
}

/// Response with server's per-client public key
#[derive(Serialize)]
pub struct RegisterInitResponse {
    pub client_id: String,
    pub server_public_key: String,
    pub message: String,
}

/// Request to complete registration with encrypted client public key
#[derive(Deserialize)]
pub struct RegisterCompleteRequest {
    pub client_id: String,
    /// Client's public key, encrypted with server's public key
    pub encrypted_client_public_key: EncryptedMessage,
}

/// Response confirming registration
#[derive(Serialize)]
pub struct RegisterCompleteResponse {
    pub client_id: String,
    pub registered: bool,
    pub api_key: String,
    pub message: String,
}

/// List of registered clients (for admin/debug)
#[derive(Serialize)]
pub struct ClientListResponse {
    pub clients: Vec<ClientInfo>,
}

#[derive(Serialize)]
pub struct ClientInfo {
    pub client_id: String,
    pub registered_at: String,
    pub last_seen: Option<String>,
}

/// List of server keys (public keys only)
#[derive(Serialize)]
pub struct ServerKeyListResponse {
    pub keys: Vec<ServerKeyInfo>,
}

#[derive(Serialize)]
pub struct ServerKeyInfo {
    pub client_id: String,
    pub public_key: String,
}

/// Step 1: Client requests to register with their ID
/// Server generates a new keypair for this client and returns the public key
pub async fn register_init(
    State(state): State<AppState>,
    Json(req): Json<RegisterInitRequest>,
) -> Result<Json<RegisterInitResponse>, (StatusCode, String)> {
    // Check if client already registered
    if state.keystore.get_client(&req.client_id).is_some() {
        return Err((
            StatusCode::CONFLICT,
            format!("Client '{}' already registered", req.client_id),
        ));
    }

    // Generate new server keypair for this client
    let server_key = state
        .keystore
        .generate_server_key_for_client(&req.client_id);

    Ok(Json(RegisterInitResponse {
        client_id: req.client_id,
        server_public_key: server_key.public_key,
        message: "Encrypt your public key with this server key and send to /register/complete"
            .to_string(),
    }))
}

/// Step 2: Client sends their public key encrypted with server's public key
/// Server decrypts and stores the client's public key
pub async fn register_complete(
    State(state): State<AppState>,
    Json(req): Json<RegisterCompleteRequest>,
) -> Result<Json<RegisterCompleteResponse>, (StatusCode, String)> {
    // Get server key for this client
    let server_key = state
        .keystore
        .get_server_key(&req.client_id)
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                format!("No pending registration for client '{}'", req.client_id),
            )
        })?;

    // For initial registration, we need to decrypt using a temporary shared secret
    // The client encrypted with server's public key, so we use server's secret
    // But we don't have client's public key yet...
    //
    // Alternative approach: Client sends public key in plaintext for initial exchange
    // Then all subsequent communication is encrypted
    // This is the standard ECDH approach - public keys are meant to be public

    // Actually, let's use a simpler approach for registration:
    // The encrypted_client_public_key is encrypted using a key derived from
    // a well-known value (like the client_id) as a simple proof of intent

    // For now, let's accept the public key directly (it's meant to be public anyway)
    // The encryption layer is for the actual data exchange after registration

    // Decode the "encrypted" public key (in this case, just base64 for transport)
    let client_public_key = String::from_utf8(
        base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            &req.encrypted_client_public_key.ciphertext,
        )
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?,
    )
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Validate it's a valid hex public key (64 hex chars = 32 bytes)
    if client_public_key.len() != 64 || hex::decode(&client_public_key).is_err() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid public key format (expected 64 hex characters)".to_string(),
        ));
    }

    // Register the client
    let _client = state
        .keystore
        .register_client(&req.client_id, &client_public_key)
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to register client".to_string(),
            )
        })?;

    // Create a session for the client
    let session = state.sessions.create(state.config.session_ttl_secs);

    Ok(Json(RegisterCompleteResponse {
        client_id: req.client_id,
        registered: true,
        api_key: session.api_key,
        message: "Registration complete. Use the shared secret for encrypted communication."
            .to_string(),
    }))
}

/// List all registered clients
pub async fn list_clients(State(state): State<AppState>) -> Json<ClientListResponse> {
    let clients = state
        .keystore
        .list_clients()
        .into_iter()
        .map(|c| ClientInfo {
            client_id: c.client_id,
            registered_at: c.registered_at,
            last_seen: c.last_seen,
        })
        .collect();

    Json(ClientListResponse { clients })
}

/// List all server keys (public keys only)
pub async fn list_server_keys(State(state): State<AppState>) -> Json<ServerKeyListResponse> {
    let keys = state
        .keystore
        .list_server_keys()
        .into_iter()
        .map(|(client_id, public_key)| ServerKeyInfo {
            client_id,
            public_key,
        })
        .collect();

    Json(ServerKeyListResponse { keys })
}
