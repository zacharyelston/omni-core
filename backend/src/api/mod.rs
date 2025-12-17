//! API routes

mod admin;
mod auth;
mod health;
mod keys;
pub mod metrics;
mod register;
mod servers;
mod settings;

use crate::services::AppState;
use axum::{
    routing::{get, post, put},
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
        // Settings
        .route("/settings", get(settings::get_settings))
        .route("/settings", put(settings::update_all_settings))
        .route("/settings/server", get(settings::get_server_settings))
        .route("/settings/server", put(settings::update_server_settings))
        .route("/settings/network", get(settings::get_network_settings))
        .route("/settings/network", put(settings::update_network_settings))
        .route("/settings/auth", get(settings::get_auth_settings))
        .route("/settings/auth", put(settings::update_auth_settings))
        .route(
            "/settings/federation",
            get(settings::get_federation_settings),
        )
        .route(
            "/settings/federation",
            put(settings::update_federation_settings),
        )
}
