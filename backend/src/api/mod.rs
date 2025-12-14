//! API routes

mod admin;
mod auth;
mod health;
mod keys;
mod register;
mod servers;

use crate::services::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        // Health
        .route("/health", get(health::health_check))
        // Server info (public)
        .route("/server/info", get(admin::get_server_info))
        // Admin
        .route("/admin/login", post(admin::admin_login))
        .route("/admin/dashboard", get(admin::admin_dashboard))
        // Auth
        .route("/auth/join", post(auth::join))
        .route("/auth/verify", post(auth::verify))
        .route("/auth/logout", post(auth::logout))
        // Key exchange (legacy)
        .route("/keys/public", get(keys::get_public_key))
        .route("/keys/exchange", post(keys::key_exchange))
        .route("/keys/send", post(keys::send_encrypted))
        // Registration (per-client keypairs)
        .route("/register/init", post(register::register_init))
        .route("/register/complete", post(register::register_complete))
        .route("/register/clients", get(register::list_clients))
        .route("/register/keys", get(register::list_server_keys))
        // Server federation
        .route("/servers/public", get(servers::list_public_servers))
        .route("/servers/register", post(servers::register_server))
        .route("/servers/sync", post(servers::sync_servers))
        .route("/servers/stats", get(servers::server_stats))
        .route("/servers/all", get(servers::list_all_servers))
}
