//! Omni Core Backend Server

use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env if present
    let _ = dotenvy::dotenv();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load config
    let config = config::Config::from_env()?;
    let port = config.port;

    // Create app state
    let state = services::AppState::new(config);

    // Start background sync service (hourly by default)
    let sync_interval = std::env::var("SYNC_INTERVAL_SECS")
        .ok()
        .and_then(|s| s.parse().ok());
    services::spawn_sync_service(
        state.server_registry.clone(),
        state.admin.get_server_id(),
        state.admin.get_server_public_key(),
        sync_interval,
    );

    // Build router
    let app = Router::new()
        .nest("/api/v1", api::routes())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 Omni Core server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
