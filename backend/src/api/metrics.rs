//! Metrics API endpoint

use axum::{extract::State, response::IntoResponse};
use metrics_exporter_prometheus::PrometheusHandle;

/// GET /metrics - Prometheus metrics endpoint
pub async fn get_metrics(State(handle): State<PrometheusHandle>) -> impl IntoResponse {
    handle.render()
}
