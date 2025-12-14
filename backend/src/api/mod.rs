//! API routes

mod auth;
mod health;

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
}
