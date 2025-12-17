//! Omni Core Backend Server

use axum::{routing::get, Router};
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

    // Initialize tracing with JSON or pretty format based on env
    let use_json = std::env::var("LOG_FORMAT")
        .map(|v| v.to_lowercase() == "json")
        .unwrap_or(false);

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,tower_http=debug".into());

    if use_json {
        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    // Initialize Prometheus metrics
    let metrics_handle = services::metrics::init_metrics();

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

    // Build metrics router (separate state)
    let metrics_router = Router::new()
        .route("/metrics", get(api::metrics::get_metrics))
        .with_state(metrics_handle);

    // Build main router
    let app = Router::new()
        .nest("/api/v1", api::routes())
        .merge(metrics_router)
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
