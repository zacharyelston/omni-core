//! API routes

mod auth;
mod health;
mod keys;

use axum::{routing::{get, post}, Router};
use crate::services::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        // Health
        .route("/health", get(health::health_check))
        // Auth
        .route("/auth/join", post(auth::join))
        .route("/auth/verify", post(auth::verify))
        .route("/auth/logout", post(auth::logout))
        // Key exchange
        .route("/keys/public", get(keys::get_public_key))
        .route("/keys/exchange", post(keys::key_exchange))
        .route("/keys/send", post(keys::send_encrypted))
}
